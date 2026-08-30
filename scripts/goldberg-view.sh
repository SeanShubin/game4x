#!/usr/bin/env bash
# Runs the goldberg-view prototype: the ten smallest Goldberg solids, one at a time, in the
# abstract drawing.
#
# `[` and `]` step through them; `I` writes the ids on and off; drag, a finger or the arrows turn; wheel or pinch zooms;
# R resets. There is no game behind it, so the digits do nothing.
#
# Arguments pass straight through.

set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Release always: a debug build tessellates two hundred regions slowly enough to mislead.
exec cargo run --release --manifest-path "$root/Cargo.toml" -p goldberg-view -- "$@"
