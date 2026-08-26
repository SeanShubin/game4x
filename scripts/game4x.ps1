#!/usr/bin/env pwsh
# Runs the application: the world as a solid you can turn in your hands.
#
# Drag to turn the world, wheel to zoom, arrow keys to turn. This is the same binary
# that is published to GitHub Pages; the only difference is that there it is compiled
# to WASM. See scripts/web.ps1 to run that build locally.
#
# Arguments are passed straight through.

$ErrorActionPreference = 'Stop'

# Locate the workspace from this script's own path, so the script works from any
# directory. Using --manifest-path rather than changing directory keeps relative paths
# in the arguments resolving against the caller's cwd.
$manifest = Join-Path (Split-Path -Parent $PSScriptRoot) 'Cargo.toml'

# Release always: a debug Bevy build gives a false impression of how the thing performs.
cargo run --release --manifest-path $manifest -p game4x -- @args
exit $LASTEXITCODE
