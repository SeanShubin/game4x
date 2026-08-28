#!/usr/bin/env pwsh
# Runs the application: the world as a solid you can turn in your hands.
#
# Turning the world is a drag, a finger, or the arrow keys; zooming is the wheel or a
# pinch; R puts the view back; 1-5 choose a planet size, before the game starts. The
# console is this terminal: type a command and press enter, /browser lists every entity,
# /game says where the game is.
#
# This is the same binary that is published to GitHub Pages. The only differences are
# that there it is compiled to WASM, and that the console and the data browser are
# elements on the page rather than this terminal - which is what spec/interface.md means
# by presentation following the platform. See scripts/web.ps1 to run that build locally.
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
