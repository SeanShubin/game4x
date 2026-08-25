#!/usr/bin/env bash
# Runs the planet view prototype.
#
# A sphere divided into regions, fanned out flat. Drag to turn the world, wheel to
# zoom, P to fold it back into a globe, Esc to quit.
# See docs/prototypes/planet-view.md.
#
# Arguments are passed straight through:
#   scripts/planet-view.sh --regions 60 --seed 7
#   scripts/planet-view.sh --help

set -euo pipefail

# Locate the workspace from this script's own path, so the script works from any
# directory. Using --manifest-path rather than changing directory keeps relative paths
# in the arguments, such as --capture, resolving against the caller's cwd.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Release always: the view rasterizes every pixel, and a debug build is unusably slow.
exec cargo run --release --manifest-path "$root/Cargo.toml" -p planet-view -- "$@"
