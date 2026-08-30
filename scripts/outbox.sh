#!/usr/bin/env bash
# What is open, and addressed to whom, across every outbox in the repository.
#
#   scripts/outbox.sh                 every open item, grouped by addressee
#   scripts/outbox.sh --to code       one addressee's inbox
#   scripts/outbox.sh --check         exit 1 if anything is open and addressed
#   scripts/outbox.sh --count         the aggregate, against the limit
#
# Like pad-tables, the package sits outside the workspace, so it never appears in
# `cargo tree` or `cargo build --workspace`.

set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec cargo run --quiet --manifest-path "$root/tools/outbox/Cargo.toml" -- "$@"
