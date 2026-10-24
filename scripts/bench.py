#!/usr/bin/env python3
"""Timing loop (hyperfine is not required): lean-package-validator vs lpv-rs
on `validate` and `validate --validate-git-archive` across scratch copies of
every package clone. Reports per-package medians and the speedup.

Runs only on the scratch copies prepared by parity.py ($LPV_PARITY_WORK/packages),
never on the original clones. Uses the same clean environment as the parity
suite so neither tool switches to agent JSON output.

Environment: LPV_REF, LPV_RS, LPV_PARITY_WORK, LPV_PACKAGES (see parity.py),
  LPV_BENCH_RUNS (default 15), LPV_BENCH_WARMUP (default 2),
  LPV_BENCH_REPORT (default <repo>/bench-report.md)
"""

import os
import statistics
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import parity  # noqa: E402

RUNS = int(os.environ.get("LPV_BENCH_RUNS", "15"))
WARMUP = int(os.environ.get("LPV_BENCH_WARMUP", "2"))
REPORT = Path(os.environ.get("LPV_BENCH_REPORT", parity.REPO / "bench-report.md"))
MODES = [["validate"], ["validate", "--validate-git-archive"]]


def time_once(cmd, cwd):
    t0 = time.perf_counter()
    p = subprocess.run(cmd, cwd=cwd, env=parity.clean_env(False), stdin=subprocess.DEVNULL,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return time.perf_counter() - t0, p.returncode


def bench(cmd, cwd, original):
    samples, codes = [], set()
    for i in range(WARMUP + RUNS):
        parity.restore(cwd, original)
        secs, rc = time_once(cmd, cwd)
        if i >= WARMUP:
            samples.append(secs)
            codes.add(rc)
    parity.restore(cwd, original)
    return statistics.median(samples), codes


def main() -> int:
    packages = parity.package_names()
    if not packages:
        print("no packages: set LPV_PACKAGES to a directory of git clones", file=sys.stderr)
        return 2
    rows = []
    for mode in MODES:
        for pkg in packages:
            cwd = parity.prepare_package(pkg)
            original = parity.original_gitattributes(f"pkg:{pkg}")
            ref_med, ref_rc = bench(parity.REF + mode, cwd, original)
            rs_med, rs_rc = bench(parity.RS + mode, cwd, original)
            rows.append((" ".join(mode), pkg, ref_med, rs_med, ref_rc, rs_rc))
            print(f"{' '.join(mode):34} {pkg:28} lpv {ref_med * 1000:8.1f} ms  lpv-rs {rs_med * 1000:7.2f} ms"
                  f"  x{ref_med / rs_med:6.1f}  rc {sorted(ref_rc)}/{sorted(rs_rc)}", flush=True)
    lines = ["# lpv vs lpv-rs timing", "",
             f"Reference: `{' '.join(parity.REF)}`  ", f"Candidate: `{' '.join(parity.RS)}`  ",
             f"Generated: {time.strftime('%Y-%m-%d %H:%M:%S %Z')}; {RUNS} timed runs after {WARMUP} warm-ups per cell; "
             "wall-clock medians of whole process runs.", "",
             "| Mode | Package | lpv median (ms) | lpv-rs median (ms) | Speedup | Exit codes lpv/lpv-rs |",
             "|------|---------|----------------:|-------------------:|--------:|------|"]
    for mode, pkg, ref_med, rs_med, ref_rc, rs_rc in rows:
        lines.append(f"| `{mode}` | {pkg} | {ref_med * 1000:.1f} | {rs_med * 1000:.2f} | {ref_med / rs_med:.1f}x | "
                     f"{','.join(map(str, sorted(ref_rc)))}/{','.join(map(str, sorted(rs_rc)))} |")
    for mode in MODES:
        m = " ".join(mode)
        sel = [r for r in rows if r[0] == m]
        if sel:
            ref_all = statistics.median(r[2] for r in sel)
            rs_all = statistics.median(r[3] for r in sel)
            lines.append(f"| `{m}` | **median of packages** | **{ref_all * 1000:.1f}** | **{rs_all * 1000:.2f}** | "
                         f"**{ref_all / rs_all:.1f}x** | |")
    REPORT.write_text(parity.redact("\n".join(lines) + "\n"))
    print(f"-> {REPORT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
