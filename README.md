# lpv-rs

**lpv-rs is an unofficial Rust port of
[lean-package-validator](https://github.com/raphaelstolt/lean-package-validator)
(`lpv`) by Raphael Stolt.** It is not affiliated with or endorsed by the
original project. All credit for the design, the rules and the behaviour goes to
Raphael Stolt and the lpv contributors. Like the original, lpv-rs is released
under the MIT license.

lpv checks a project's `.gitattributes` `export-ignore` rules so that dist
archives (`git archive`, Composer dist installs) ship only what they should.
lpv-rs aims to be a drop-in replacement for `lean-package-validator` **6.3.2**
(the release phar): for the ported commands it uses the same subcommands, flags,
exit codes and human output, byte for byte. It also adds a `--format json`
machine report.

## At a glance

- **Parity:** 627 of 634 cases byte-identical against the lpv 6.3.2 phar
  (exit code, stdout, stderr and resulting `.gitattributes` bytes), 0 failures,
  7 known differences (listed [below](#known-differences)).
- **Speed:** `validate` is about **51x** faster (median 33.7 ms for lpv vs
  0.66 ms for lpv-rs); `validate --validate-git-archive` is about **15.5x**
  faster (38.8 ms vs 2.5 ms). Measured on an AMD Ryzen AI MAX+ 395 with PHP 8.5,
  wall-clock medians of whole process runs across 13 real packages. Full
  numbers: [bench-report.md](bench-report.md).

## Install

With a Rust toolchain (edition 2024, Rust 1.85 or newer):

```sh
cargo install --git https://github.com/jordanpartridge/lpv-rs
```

Or build from source:

```sh
git clone https://github.com/jordanpartridge/lpv-rs
cd lpv-rs
cargo build --release --locked
./target/release/lpv-rs --version
```

lpv-rs is not published on crates.io.

## Usage

```sh
# Validate the .gitattributes of a package (exit 0 when valid, 1 when not)
lpv-rs validate path/to/package

# Show what is missing or unexpected as a diff
lpv-rs validate --diff path/to/package

# Also build a git archive from HEAD and check that nothing leaks into it
cd path/to/package && lpv-rs validate --validate-git-archive

# Create or update a .gitattributes file (preview first with --dry-run)
lpv-rs create --dry-run path/to/package
lpv-rs update path/to/package

# Machine-readable report
lpv-rs validate --format json path/to/package
```

## What is ported

| lpv command | lpv-rs | Notes |
|---|---|---|
| `validate` | ported | all flags except `--report-impact` |
| `create` | ported | classic and negated flavours |
| `update` | ported | classic and negated strategies |
| `init`, `refresh`, `reformat`, `tree`, `validate-remote`, `list`, `list-skills`, `help`, `completion` | not ported | exit 1 with `lpv-rs: the "X" command ... is not ported` |

Also not ported: `validate --report-impact` (exits 1 with a message), and
Symfony's help text for `-h/--help` and `-V/--version` (lpv-rs prints clap's).

Command lookup follows Symfony Console:
- unique abbreviations work (`upd`, `creat`)
- ambiguous prefixes (`val`, `re`, `c`, `VALIDATE`) and unknown commands produce
  lpv's error blocks, including "Did you mean ...?" suggestions
- in an interactive run with a single alternative, you get lpv's
  `Do you want to run "validate" instead?` prompt

### Flags

Global: `-v/-vv/-vvv`, `-q/--quiet`, `--silent`, `-n/--no-interaction`,
`--ansi/--no-ansi`.

`validate [<directory>]`:
- `--stdin-input`, `--diff`, `--validate-git-archive`
- `--enforce-strict-order`, `--enforce-alignment`, `--report-stale-export-ignores`
- `--glob-pattern`, `--glob-pattern-file[=FILE]` (and `.lpv` in the cwd), `--preset[=NAME]`
- `--keep-license` (MIT licences are kept automatically), `--keep-readme`,
  `--keep-glob-pattern`
- `-s/--sort-from-directories-to-files`
- `--format auto|text|json`

`create [<directory>]`:
- every glob/keep option above, with `--keep-glob-pattern PATTERN`
- `-a/--align-export-ignores`, `-s`, `--enforce-strict-order`
- `--dry-run`, `--force`, `-f/--flavour classic|negated`
- `--format auto|text|json`

`update [<directory>]`:
- the same generation options as `create`, plus `--group` and `--dry-run`
- `--reformat-export-ignores` is accepted and, as in lpv 6.3.2, does nothing.

lpv's quirks are kept on purpose, so that lpv-rs is a drop-in replacement:
- The git archive is built from the process cwd, not from the directory argument.
- In `validate`, `--keep-glob-pattern` is a value-less flag and fails with lpv's
  "invalid glob pattern '1'" warning.
- The JavaScript preset can't be reached through `--preset`.
- Header comments are rewritten on overwrite.
- Line-ending detection matches lpv's.
- Negated generation `chdir()`s into the package.

## Agent mode and `--format`

Like lpv, lpv-rs switches to lpv's compact agent JSON
(`{"command","status","message",...}`) when it detects a coding agent: `AI_AGENT`,
`CURSOR_AGENT`, `CLAUDECODE` and the other variables from the `laravel/agent-detector`
version bundled in the 6.3.2 phar, or `/opt/.devin`. `--format` is an lpv-rs
extension:

- `--format auto` (default): exactly like lpv, including agent detection.
- `--format text`: always human output.
- `--format json`: a richer report on stdout. Human output is suppressed and the
  exit code is unchanged.

### `--format json` schema

`validate`:

```json
{
  "tool": "lpv-rs",
  "version": "0.1.0",
  "command": "validate",
  "directory": "/abs/pkg",
  "gitattributes_path": "/abs/pkg/.gitattributes",
  "gitattributes_present": true,
  "strategy": "classic",
  "valid": false,
  "exit_code": 1,
  "message": "...",
  "missing_export_ignores": ["tests/"],
  "unexpected_export_ignores": [],
  "stale_export_ignores": ["docs/"],
  "archive_validated": false,
  "archive_leaks": null,
  "expected_gitattributes_content": "...",
  "warnings": []
}
```

| Field | Type | Meaning |
|---|---|---|
| `tool`, `version`, `command` | string | always `"lpv-rs"`, the lpv-rs version, the subcommand |
| `directory`, `gitattributes_path` | string | absolute paths that were checked |
| `gitattributes_present` | bool | whether a `.gitattributes` file exists |
| `strategy` | string | `"classic"` or `"negated"` |
| `valid` | bool | overall result |
| `exit_code` | int | the process exit code (same as lpv's) |
| `message` | string | lpv's summary message |
| `missing_export_ignores` | string[] | entries lpv expects but the file lacks |
| `unexpected_export_ignores` | string[] | entries present but not expected |
| `stale_export_ignores` | string[] | entries pointing at paths that do not exist |
| `archive_validated` | bool | whether `--validate-git-archive` ran |
| `archive_leaks` | string[] or null | files that leaked into the archive, null when not checked |
| `expected_gitattributes_content` | string or null | the content lpv would expect, when invalid |
| `warnings` | string[] | lpv's warnings (for example leading `/` patterns) |

`create` and `update`:

```json
{
  "tool": "lpv-rs",
  "version": "0.1.0",
  "command": "create",
  "directory": "/abs/pkg",
  "gitattributes_path": "/abs/pkg/.gitattributes",
  "dry_run": true,
  "written": false,
  "exit_code": 0,
  "message": "Dry run: no file written.",
  "content": "* text=auto eol=lf\n\n.gitattributes export-ignore\n..."
}
```

## Parity and speed

- `scripts/parity.sh` builds the release binary and runs `scripts/parity.py`.
  The suite runs lpv and lpv-rs on every fixture in `tests/fixtures/` and, when
  `LPV_PACKAGES` points at a directory of package clones, on scratch copies of
  each one. Fixtures store `.gitattributes`/`.gitignore` as
  `_gitattributes`/`_gitignore`. It compares exit code, stdout, stderr and the
  resulting `.gitattributes` bytes, and writes [parity-report.md](parity-report.md).
- `scripts/bench.py` is a timing loop (hyperfine is not needed). It compares
  `validate` and `validate --validate-git-archive` across the same scratch
  copies and writes [bench-report.md](bench-report.md) with medians and speedups.

```sh
LPV_REF="php /path/to/lean-package-validator.phar" \
LPV_PACKAGES=/path/to/package-clones \
scripts/parity.sh
LPV_REF="php /path/to/lean-package-validator.phar" \
LPV_PACKAGES=/path/to/package-clones \
python3 scripts/bench.py
```

Both scripts write only below `$LPV_PARITY_WORK` (default
`target/lpv-parity-work`) and never touch the original clones. Environment
overrides: `LPV_REF` (default `lean-package-validator` on `PATH`), `LPV_RS`,
`LPV_PARITY_WORK`, `LPV_PACKAGES` (or `PACKAGES_DIR`), `LPV_REPORT`,
`LPV_BENCH_RUNS`, `LPV_BENCH_WARMUP`, `LPV_BENCH_REPORT`. Reports replace local
absolute paths with placeholders. The published reports were produced against
13 public packages from
[github.com/jordanpartridge](https://github.com/jordanpartridge).

### Known differences

The 7 cases that are not byte-identical, all expected:

1. `validate -vvv` (1 case): the phar prints Box's PHP requirements checker on
   stderr. lpv-rs has no PHP runtime to check.
2. `validate --report-impact` (1 case): not ported; lpv-rs exits 1 with a message.
3. Confirming the suggested `list` command at the "Did you mean" prompt (1 case):
   `list` is not ported.
4. Negated `.gitattributes` with a relative directory argument (4 cases:
   `update --keep-license` and `create --force --keep-license` on the `negated`
   and `negated-bin` fixtures): lpv fails to write because of its own `chdir()`.
   lpv-rs fails the same way, with the same exit code, stdout and file bytes,
   but its stderr warning is its own instead of PHP's engine warning, which
   names the phar's install path.

Also note: `--diff` output follows the sebastian/diff version **bundled in the
phar** (LCS-based `Differ`). A source checkout of v6.3.2 installed with
`composer install` pulls a newer sebastian/diff (Myers), which orders some
hunks differently, and a newer agent detector. Compare against the release phar.

## License

MIT. See [LICENSE](LICENSE). Copyright (c) 2016 - ∞ Raphael Stolt (original
lean-package-validator); copyright (c) 2026 Jordan Partridge (the Rust port).
