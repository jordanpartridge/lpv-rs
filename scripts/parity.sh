#!/usr/bin/env bash
# Build lpv-rs (release) and run the parity suite against lean-package-validator.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release --locked
exec python3 scripts/parity.py "$@"
