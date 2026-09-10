#!/usr/bin/env bash
# Runs the gap-view prototype: the world laid flat with every territory at its own shape and
# the curvature paid as gaps between them.
#
# It writes drawings rather than opening a window - `prototypes/gap-view/drawing/index.html`,
# four worlds and the numbers beside them. Open that file to see the answer.
#
# Pass a directory to write somewhere else. Other arguments pass straight through.
#
# **Its own manifest rather than the workspace's.** The crate is not a workspace member yet:
# adding one requires a row in `docs/architecture.md`, which is the specification lane's file.
# `C-76` asks for it; until then `-p gap-view` from the root would not resolve.

set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

exec cargo run --release --manifest-path "$root/prototypes/gap-view/Cargo.toml" -- "$@"
