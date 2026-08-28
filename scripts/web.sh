#!/usr/bin/env bash
# Serves the WASM build locally, at http://localhost:8080.
#
# This is what GitHub Pages publishes, running on your own machine: same crate, same
# Trunk, same wasm32 target. Worth reaching for when something works natively and not on
# the deployed page, because that difference is the whole point of this script.
#
#   scripts/web.sh                                # serve with rebuild-on-change
#   scripts/web.sh --release                      # as published, slower to build
#   scripts/web.sh --release --address 0.0.0.0    # reachable from a phone or tablet
#
# Trunk prints the local address with a trailing dot - `http://localhost.:8080/` - which is
# the DNS root form and which Chrome refuses to resolve. Use `http://127.0.0.1:8080/`
# instead; it is the same server.
#
# Needs Trunk and the wasm32 target, once per machine:
#   cargo install trunk
#   rustup target add wasm32-unknown-unknown

set -euo pipefail

# Trunk resolves the cargo manifest from its working directory, and the workspace root is
# a virtual manifest with no root package - so it has to run from the crate itself rather
# than from the workspace root.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Trunk reads NO_COLOR as a boolean; some shells export it as "1", which it rejects
# outright rather than ignoring.
if [ -n "${NO_COLOR:-}" ]; then
    export NO_COLOR=true
fi

cd "$root/crates/game4x"
# No --public-url here: served from the root locally, unlike the /game4x/ subpath that
# GitHub Pages puts a project site on.
exec trunk serve "$@"
