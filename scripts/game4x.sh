#!/usr/bin/env bash
# Runs the application: the world as a solid you can turn in your hands.
#
# Drag to turn the world, wheel to zoom, arrow keys to turn. This is the same binary
# that is published to GitHub Pages; the only difference is that there it is compiled
# to WASM. See scripts/web.sh to run that build locally.
#
# Arguments are passed straight through.

set -euo pipefail

# Locate the workspace from this script's own path, so the script works from any
# directory. Using --manifest-path rather than changing directory keeps relative paths
# in the arguments resolving against the caller's cwd.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Release always: a debug Bevy build gives a false impression of how the thing performs.
exec cargo run --release --manifest-path "$root/Cargo.toml" -p game4x -- "$@"
