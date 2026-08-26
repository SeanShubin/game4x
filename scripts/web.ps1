#!/usr/bin/env pwsh
# Serves the WASM build locally, at http://localhost:8080.
#
# This is what GitHub Pages publishes, running on your own machine: same crate, same
# Trunk, same wasm32 target. Worth reaching for when something works natively and not on
# the deployed page, because that difference is the whole point of this script.
#
#   scripts/web.ps1            # serve with rebuild-on-change
#   scripts/web.ps1 --release  # as published, slower to build
#
# Needs Trunk and the wasm32 target, once per machine:
#   cargo install trunk
#   rustup target add wasm32-unknown-unknown

$ErrorActionPreference = 'Stop'

# Trunk resolves the cargo manifest from its working directory, and the workspace root is
# a virtual manifest with no root package - so it has to run from the crate itself rather
# than from the workspace root.
$crate = Join-Path (Split-Path -Parent $PSScriptRoot) 'crates/game4x'

# Trunk reads NO_COLOR as a boolean; some shells export it as "1", which it rejects
# outright rather than ignoring.
if ($env:NO_COLOR) { $env:NO_COLOR = 'true' }

Push-Location $crate
try {
    # No --public-url here: served from the root locally, unlike the /game4x/ subpath
    # that GitHub Pages puts a project site on.
    trunk serve @args
    exit $LASTEXITCODE
} finally {
    Pop-Location
}
