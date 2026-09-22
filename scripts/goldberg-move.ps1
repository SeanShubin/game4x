#!/usr/bin/env pwsh
# Runs the goldberg-move prototype: laying out a move across a Goldberg planet by clicking.
#
# Forty-two territories - GP(2,0), the third-smallest Goldberg solid - and four coloured disks.
# Click a disk to pick it up, and the same disk again to put it down. Click a territory and the
# move is laid out to it and the planet turns to face it; click that territory again to finish
# the move, or a different one to go further. There is no limit on how far.
#
# A destination is only clickable when one shortest route reaches it. Where several do, the
# status line says how many and you click a territory on the way first. That rule is the whole
# question this prototype asks; everything else is scenery.
#
# Drag, a finger or the arrows turn the planet; wheel or pinch zooms; R resets the view. There
# is no game behind it - no cost, no turn, no rule about what a disk is.
#
# Arguments pass straight through.

$ErrorActionPreference = 'Stop'
$manifest = Join-Path (Split-Path -Parent $PSScriptRoot) 'Cargo.toml'

# Release always, for the reason goldberg-view.ps1 gives: a debug build tessellates forty-two
# regions slowly enough to read as a stutter that is not there.
cargo run --release --manifest-path $manifest -p goldberg-move -- @args
exit $LASTEXITCODE
