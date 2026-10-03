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

# **The previous instance is stopped first, because otherwise this cannot start.** Windows holds a
# running executable open, so `cargo` cannot relink over it and reports
# *failed to remove file ... Access is denied*, naming a file in `target/` and saying nothing about
# the app being up. Sean hit it on `review.ps1`; this is the same gap in the same script's other
# spelling.
#
# **There is nothing to preserve.** The port is single-occupancy, so a second instance would fail to
# bind 7878 even if it built - **stopping the old one is what running this again asks for.**
#
# **Only this launcher does it.** `game4x`, `planet-view` and the rest start windowed apps with no
# fixed port, where a second window is a reasonable thing to want; this one binds an address.
if command -v taskkill >/dev/null 2>&1; then
  taskkill //IM review-web.exe //F >/dev/null 2>&1 && echo "review: stopped the instance already serving"
else
  pkill -x review-web >/dev/null 2>&1 && echo "review: stopped the instance already serving"
fi
sleep 1

# Debug, unlike every other launcher here: it serves one page to one reader on one machine, and a
# release build would cost a minute of waiting to save milliseconds nobody is watching for.
exec cargo run --quiet --manifest-path "$root/crates/game-model/Cargo.toml" \
  --example review-web -- "$@"
