#!/usr/bin/env bash
# Drives the application by remote control and writes what it finds: a PNG of one frame,
# and a text dump of the game and the mesh behind it.
#
# The picture is the half of spec/planet.md that no test can check, so this is how it gets
# checked at all. Arguments pass straight through to the binary; `--help` lists them.
#
#   scripts/shot.sh --shot planet.png --drawing realistic --yaw 0.6 --pitch 0.35
#   scripts/shot.sh --shot huge.png --size huge --dump huge.txt
#
# Release always: a debug build spends minutes sampling the terrain field.

set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec cargo run --release --manifest-path "$root/Cargo.toml" -p game4x -- "$@"
