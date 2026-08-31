#!/usr/bin/env bash
# Print the release's kinds and transformations, rendered from the Rust data that holds
# them. The tables it prints are the ones in releases/first-release.md, and a test keeps
# them that way; see prototypes/kinds/README.md.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cargo run --quiet --manifest-path "$root/Cargo.toml" -p kinds -- "$@"
