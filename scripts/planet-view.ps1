#!/usr/bin/env pwsh
# Runs the planet view prototype.
#
# A sphere divided into regions, fanned out flat. Drag to turn the world, wheel to
# zoom, P to fold it back into a globe, Esc to quit.
# See docs/prototypes/planet-view.md.
#
# Arguments are passed straight through:
#   scripts/planet-view.ps1 --regions 60 --seed 7
#   scripts/planet-view.ps1 --help

$ErrorActionPreference = 'Stop'

# Locate the workspace from this script's own path, so the script works from any
# directory. Using --manifest-path rather than changing directory keeps relative
# paths in the arguments, such as --capture, resolving against the caller's cwd.
$manifest = Join-Path (Split-Path -Parent $PSScriptRoot) 'Cargo.toml'

# Release always: the view rasterizes every pixel, and a debug build is unusably slow.
cargo run --release --manifest-path $manifest -p planet-view -- @args
exit $LASTEXITCODE
