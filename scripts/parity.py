#!/usr/bin/env python3
"""Parity suite: run lean-package-validator (reference) and lpv-rs on every
fixture in tests/fixtures and on scratch copies of every package clone, then
compare exit code, stdout, stderr and the resulting .gitattributes bytes.

Nothing outside the scratch work directory is ever written. Package clones are
copied once into $LPV_PARITY_WORK/packages/ and their .gitattributes is restored
from the (read-only) original after every run.

Environment:
  LPV_REF          reference command (default: lean-package-validator on PATH),
                   e.g. LPV_REF="php /path/to/lean-package-validator.phar"
  LPV_RS           lpv-rs binary (default: <repo>/target/release/lpv-rs)
  LPV_PARITY_WORK  scratch dir (default: <repo>/target/lpv-parity-work)
  LPV_PACKAGES     directory holding git clones of real packages, one per
                   subdirectory (PACKAGES_DIR is accepted too). Optional: when
                   unset, only the fixtures in tests/fixtures are compared.
  LPV_REPORT       report path (default: <repo>/parity-report.md)

Reports never contain absolute local paths: the repo path, the packages and
scratch dirs and $HOME are replaced with placeholders.
"""

import difflib
import os
import shlex
import shutil
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
HOME = Path.home()
REF = shlex.split(os.environ.get("LPV_REF", "lean-package-validator"))
RS = shlex.split(os.environ.get("LPV_RS", str(REPO / "target/release/lpv-rs")))
WORK = Path(os.environ.get("LPV_PARITY_WORK", REPO / "target/lpv-parity-work"))
_PACKAGES_ENV = os.environ.get("LPV_PACKAGES") or os.environ.get("PACKAGES_DIR")
PACKAGES = Path(_PACKAGES_ENV) if _PACKAGES_ENV else None
REPORT = Path(os.environ.get("LPV_REPORT", REPO / "parity-report.md"))
FIXTURES = REPO / "tests/fixtures"
STDIN = REPO / "tests/stdin"


def package_names() -> list[str]:
    if PACKAGES is None or not PACKAGES.is_dir():
        return []
    return sorted(p.name for p in PACKAGES.iterdir() if (p / ".git").is_dir())


def redact(text: str) -> str:
    """Replace absolute local paths with placeholders before writing a report."""
    subs = [(str(WORK), "$LPV_PARITY_WORK"), (str(REPO), "<repo>")]
    if PACKAGES is not None:
        subs.append((str(PACKAGES), "$LPV_PACKAGES"))
    subs.append((str(HOME), "~"))
    for path, placeholder in sorted(subs, key=lambda x: -len(x[0])):
        text = text.replace(path, placeholder)
    return text

# Features lpv-rs deliberately does not port (yet); mismatches here are expected.
PARENT_NEGATED_REASON = ("lpv chdir()s into the package during negated generation, so the relative "
                         "write fails with a PHP engine warning naming the phar's install path; lpv-rs "
                         "fails the same way (same exit code, stdout and file bytes) but prints its own warning")
# Known, intentional differences: marker in the args -> reason.
UNSUPPORTED_MARKERS = {
    "--report-impact": "--report-impact (distribution impact report) is not ported",
    "lst": "confirming the suggested `list` command runs a command lpv-rs does not port",
    "-vvv": "the phar's Box requirements checker prints PHP runtime details on stderr at -vvv; lpv-rs has no PHP runtime",
}

GIT_ENV = {
    "GIT_AUTHOR_NAME": "parity",
    "GIT_AUTHOR_EMAIL": "parity@example.invalid",
    "GIT_COMMITTER_NAME": "parity",
    "GIT_COMMITTER_EMAIL": "parity@example.invalid",
    "GIT_AUTHOR_DATE": "2026-01-01T00:00:00Z",
    "GIT_COMMITTER_DATE": "2026-01-01T00:00:00Z",
}


def clean_env(agent: bool) -> dict:
    env = {"HOME": str(HOME), "PATH": os.environ.get("PATH", "/usr/bin:/bin"), "LANG": "C.UTF-8"}
    if agent:
        env["AI_AGENT"] = "parity-suite"
    return env


def prepare_fixture(name: str) -> Path:
    dest = WORK / "fixtures" / name
    if dest.exists():
        shutil.rmtree(dest)
    shutil.copytree(FIXTURES / name, dest, symlinks=True)
    for src, dst in (("_gitattributes", ".gitattributes"), ("_gitignore", ".gitignore")):
        if (dest / src).exists():
            (dest / src).rename(dest / dst)
    env = {**os.environ, **GIT_ENV}
    subprocess.run(["git", "init", "-q", "-b", "main"], cwd=dest, check=True, env=env)
    subprocess.run(["git", "add", "-A"], cwd=dest, check=True, env=env)
    subprocess.run(["git", "commit", "-q", "-m", "fixture"], cwd=dest, check=True, env=env)
    return dest


def prepare_package(name: str) -> Path:
    dest = WORK / "packages" / name
    if not dest.exists():
        dest.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(["cp", "-a", str(PACKAGES / name), str(dest)], check=True)
    return dest


def original_gitattributes(scope: str) -> bytes | None:
    kind, name = scope.split(":", 1)
    src = (FIXTURES / name / "_gitattributes") if kind == "fixture" else (PACKAGES / name / ".gitattributes")
    return src.read_bytes() if src.exists() else None


def restore(target: Path, original: bytes | None) -> None:
    ga = target / ".gitattributes"
    if original is None:
        if ga.exists():
            ga.unlink()
    else:
        ga.write_bytes(original)
    stray = target / ".tar.gz"
    if stray.exists():
        stray.unlink()


def run_tool(cmd, args, cwd, agent, stdin_file):
    stdin = open(stdin_file, "rb") if stdin_file else subprocess.DEVNULL
    t0 = time.perf_counter()
    try:
        p = subprocess.run(cmd + args, cwd=cwd, env=clean_env(agent), stdin=stdin,
                           capture_output=True, timeout=300)
    finally:
        if stdin_file:
            stdin.close()
    return p.returncode, p.stdout, p.stderr, time.perf_counter() - t0


def build_cases():
    cases = []
    fixtures = sorted(p.name for p in FIXTURES.iterdir() if p.is_dir())
    common = [
        ("validate", False), ("validate --diff", False), ("validate --validate-git-archive", False),
        ("validate", True), ("validate --validate-git-archive", True),
        ("validate --report-stale-export-ignores --diff", False),
        ("create --dry-run", False), ("update --dry-run", False),
        ("update", False), ("create --force", False), ("create", False), ("update", True),
    ]
    for fx in fixtures:
        for args, agent in common:
            cases.append((f"fixture:{fx}", args, agent, "in", None))
    extra = {
        "keep-readme": ["validate --keep-readme", "update --keep-readme", "validate --keep-readme --validate-git-archive"],
        "keep-license": ["validate --keep-license", "validate --keep-license --validate-git-archive", "create --force --keep-license --dry-run"],
        "no-license-file": ["validate --keep-license --validate-git-archive", "validate --keep-license"],
        "aligned": ["validate --enforce-alignment", "update -a", "create --dry-run -a", "update --align-export-ignores --sort-from-directories-to-files"],
        "unsorted": ["validate --enforce-strict-order", "validate -s", "update -s", "validate --enforce-strict-order --stdin-input<valid.txt"],
        "python-preset": ["validate --preset python", "validate --preset=Python --diff", "validate --preset javascript",
                          "validate --preset foo", "create --dry-run --preset rust", "update --preset=go --dry-run", "validate --preset"],
        "lpv-file": ["validate -v", "validate --glob-pattern-file=missing.lpv", "validate --glob-pattern-file", "update --dry-run -v"],
        "valid": ["validate -v", "validate -vvv", "validate --glob-pattern {.*,*.md}", "validate --glob-pattern {x}",
                  "validate --glob-pattern=", "validate --keep-glob-pattern", "validate --stdin-input<valid.txt",
                  "validate --stdin-input<invalid.txt", "validate --stdin-input<garbage.txt", "validate /nonexistent/dir",
                  "validate a b", "validate --bogus", "validate --glob-pattern", "create --keep-glob-pattern {README*,tests*} --dry-run --force",
                  "update --keep-glob-pattern {x}", "update --group", "create --flavour=x", "create --flavour negated --dry-run",
                  "create --flavour negated --force -a", "create --flavour negated --force --keep-readme --keep-glob-pattern {tests*}",
                  "validate --report-impact", "frobnicate", "update --keep-readme=yes", "validate -q", "validate --no-ansi",
                  "val", "validate --stdin-input --keep-readme<invalid.txt", "VALIDATE", "re", "c", "l", "x:y",
                  "valdate", "valdate -n", "lst -q", "creat --dry-run", "valdate<yes.txt", "lst<yes.txt"],
        "crlf": ["update --group", "validate --enforce-alignment --diff"],
        "preserve-nested": ["update --group", "validate --diff -s", "update -s"],
        "negated": ["validate --report-stale-export-ignores", "validate --stdin-input<valid.txt", "update -a",
                    "create --force -a", "update --group", "update --sort-from-directories-to-files"],
        "negated-bin": ["update -a", "create --force --keep-readme", "create --force --keep-license"],
        "warnings": ["validate -v"],
        "stale-ignores": ["validate --report-stale-export-ignores", "update --group"],
        "generated-header": ["update -a"],
        "gitignore": ["validate -v", "create --force --dry-run"],
        "ai-files": ["validate -s --diff", "update -a -s"],
    }
    for fx, arglist in extra.items():
        for args in arglist:
            stdin = None
            if "<" in args:
                args, stdin = args.split("<", 1)
            cases.append((f"fixture:{fx}", args.strip(), False, "in", stdin))
    for fx in ["valid", "stale-ignores", "missing-ignores", "no-gitattributes", "lpv-file", "negated", "negated-bin"]:
        for args in ["validate", "validate --report-stale-export-ignores --diff", "validate --validate-git-archive", "create --dry-run", "update",
                     "update --keep-license", "create --force --keep-license"]:
            cases.append((f"fixture:{fx}", args, False, "parent", None))

    packages = package_names()
    pkg_args = [
        ("validate", False), ("validate --diff", False), ("validate --report-stale-export-ignores --diff", False),
        ("validate --enforce-alignment", False), ("validate -s", False), ("validate --enforce-strict-order", False),
        ("validate --keep-readme --keep-license", False), ("validate --validate-git-archive", False),
        ("validate", True), ("validate --validate-git-archive", True), ("validate -v", False),
        ("create --dry-run", False), ("update --dry-run", False), ("update", False), ("update -a", False),
        ("update --group", False), ("create --force", False), ("update", True),
    ]
    for pkg in packages:
        for args, agent in pkg_args:
            cases.append((f"pkg:{pkg}", args, agent, "in", None))
    return cases


def show(b: bytes) -> list[str]:
    return b.decode("utf-8", "replace").splitlines(keepends=True)


def main() -> int:
    if not Path(RS[0]).exists():
        print(f"lpv-rs binary not found: {RS[0]} (run cargo build --release)", file=sys.stderr)
        return 2
    cases = build_cases()
    prepared = {}
    results = []
    for idx, (scope, args, agent, cwd_mode, stdin) in enumerate(cases, 1):
        kind, name = scope.split(":", 1)
        if scope not in prepared:
            prepared[scope] = prepare_fixture(name) if kind == "fixture" else prepare_package(name)
        target = prepared[scope]
        original = original_gitattributes(scope)
        argv = shlex.split(args)
        cwd = target
        if cwd_mode == "parent":
            cwd = target.parent
            argv = argv + [name]
        stdin_file = STDIN / stdin if stdin else None
        outcome = {}
        for label, cmd in (("ref", REF), ("rs", RS)):
            restore(target, original)
            rc, out, err, secs = run_tool(cmd, argv, cwd, agent, stdin_file)
            ga = target / ".gitattributes"
            outcome[label] = (rc, out, err, ga.read_bytes() if ga.exists() else None, secs)
        restore(target, original)
        ref, rs = outcome["ref"], outcome["rs"]
        problems = []
        if ref[0] != rs[0]:
            problems.append(f"exit code {ref[0]} != {rs[0]}")
        if ref[1] != rs[1]:
            problems.append("stdout differs")
        if ref[2] != rs[2]:
            problems.append("stderr differs")
        if ref[3] != rs[3]:
            problems.append(".gitattributes bytes differ")
        case_id = f"{scope} [{cwd_mode}] {'AI_AGENT ' if agent else ''}{args}{' <' + stdin if stdin else ''}"
        unsupported = any(m in args.split() for m in UNSUPPORTED_MARKERS) or (
            cwd_mode == "parent" and name.startswith("negated") and problems == ["stderr differs"])
        status = "PASS" if not problems else ("UNSUPPORTED" if unsupported else "FAIL")
        results.append((case_id, status, problems, ref, rs))
        print(f"[{idx}/{len(cases)}] {status} {case_id}", flush=True)

    passed = sum(1 for r in results if r[1] == "PASS")
    failed = [r for r in results if r[1] == "FAIL"]
    unsup = [r for r in results if r[1] == "UNSUPPORTED"]
    lines = ["# lpv-rs parity report", "",
             f"Reference: `{' '.join(REF)}`  ", f"Candidate: `{' '.join(RS)}`  ",
             f"Generated: {time.strftime('%Y-%m-%d %H:%M:%S %Z')}", "",
             "Comparison is byte-exact on exit code, stdout, stderr and the resulting `.gitattributes` bytes.", "",
             f"**{passed} pass, {len(failed)} fail, {len(unsup)} expected-unsupported, {len(results)} total.**", "",
             "| # | Case | Result | Notes |", "|---|------|--------|-------|"]
    for i, (case_id, status, problems, _, _) in enumerate(results, 1):
        lines.append(f"| {i} | `{case_id}` | {status} | {'; '.join(problems)} |")
    for title, group in (("Failures", failed), ("Expected-unsupported differences", unsup)):
        if not group:
            continue
        lines += ["", f"## {title}", ""]
        for case_id, status, problems, ref, rs in group:
            lines += [f"### `{case_id}`", "", f"{'; '.join(problems)}", ""]
            reasons = [r for m, r in UNSUPPORTED_MARKERS.items() if m in case_id.split()]
            if "[parent]" in case_id and "negated" in case_id:
                reasons.append(PARENT_NEGATED_REASON)
            if status == "UNSUPPORTED" and reasons:
                lines += [f"Reason: {'; '.join(reasons)}.", ""]
            for what, i in (("stdout", 1), ("stderr", 2), (".gitattributes", 3)):
                a, b = ref[i] or b"", rs[i] or b""
                if a != b:
                    d = difflib.unified_diff(show(a), show(b), f"lpv {what}", f"lpv-rs {what}")
                    lines += ["```diff", "".join(d).rstrip("\n"), "```", ""]
            if ref[0] != rs[0]:
                lines += [f"exit codes: lpv={ref[0]} lpv-rs={rs[0]}", ""]
    REPORT.write_text(redact("\n".join(lines) + "\n"))
    print(f"\n{passed} pass, {len(failed)} fail, {len(unsup)} expected-unsupported, {len(results)} total -> {REPORT}")
    return 0 if not failed else 1


if __name__ == "__main__":
    sys.exit(main())
