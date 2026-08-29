#!/usr/bin/env pwsh
# Runs the goldberg-view prototype: the ten smallest Goldberg solids, one at a time, in the
# abstract drawing.
#
# `[` and `]` step through them; drag, a finger or the arrows turn; wheel or pinch zooms;
# R resets. There is no game behind it, so the digits do nothing.
#
# Arguments pass straight through.

$ErrorActionPreference = 'Stop'
$manifest = Join-Path (Split-Path -Parent $PSScriptRoot) 'Cargo.toml'

# Release always: a debug build tessellates two hundred regions slowly enough to mislead.
cargo run --release --manifest-path $manifest -p goldberg-view -- @args
exit $LASTEXITCODE
