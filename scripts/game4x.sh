#!/usr/bin/env bash
# Runs the application: the world as a solid you can turn in your hands.
#
# Turning the world is a drag, a finger, or the arrow keys; zooming is the wheel or a
# pinch; R puts the view back; 1-5 start a new game on a planet of that size. The
# console is this terminal: type a command and press enter, /browser lists every entity,
# /game says where the game is.
#
# This is the same binary that is published to GitHub Pages. The only differences are
# that there it is compiled to WASM, and that the console and the data browser are
# elements on the page rather than this terminal - which is what spec/interface.md means
# by presentation following the platform. See scripts/web.sh to run that build locally.
#
# Arguments are passed straight through.

set -euo pipefail

# Locate the workspace from this script's own path, so the script works from any
# directory. Using --manifest-path rather than changing directory keeps relative paths
# in the arguments resolving against the caller's cwd.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Release always: a debug Bevy build gives a false impression of how the thing performs.
exec cargo run --release --manifest-path "$root/Cargo.toml" -p game4x -- "$@"
