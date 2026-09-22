#!/usr/bin/env bash
# Starts the review application and serves it at http://127.0.0.1:7878.
#
# Every test, whole, in the friendly form, with what the run said about it. The arrow keys move,
# `Enter` opens, `r` marks a test reviewed, `u` takes that back, and `x` files a note saying what
# needs changing. Every key press is a write to the disk that has already happened before the
# badge changes.
#
# **This is the only thing that writes `reviewed/`** - `CLAUDE.md`: a record is added and removed
# only by the review application, acting as Sean. So this is the door to it, and until now there
# was none: thirty-two scripts and none that started the tool he uses most.
#
# Ctrl-C stops it. Arguments pass straight through.

set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Debug, unlike every other launcher here: it serves one page to one reader on one machine, and a
# release build would cost a minute of waiting to save milliseconds nobody is watching for.
exec cargo run --quiet --manifest-path "$root/prototypes/thin-engine/Cargo.toml" \
  --example review-web -- "$@"
