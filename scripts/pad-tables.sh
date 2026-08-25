#!/usr/bin/env bash
# Align every markdown table in the repository so columns line up in a monospace editor.
# Pass paths to limit it to those files or directories.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
if [ "$#" -eq 0 ]; then set -- "$root"; fi
cargo run --quiet --manifest-path "$root/tools/pad-tables/Cargo.toml" -- "$@"
