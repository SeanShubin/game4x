#!/usr/bin/env pwsh
# Starts the review application and serves it at http://127.0.0.1:7878.
#
# Every test, whole, in the friendly form, with what the run said about it. The arrow keys move,
# `Enter` opens, `r` marks a test reviewed, `u` takes that back, and `x` files a note saying what
# needs changing. Every key press is a write to the disk that has already happened before the
# badge changes.
#
# This is the only thing that writes `reviewed/` - CLAUDE.md: a record is added and removed only
# by the review application, acting as Sean. So this is the door to it, and until now there was
# none: thirty-two scripts and none that started the tool he uses most.
#
# Ctrl-C stops it. Arguments pass straight through.

$ErrorActionPreference = 'Stop'
$manifest = Join-Path (Split-Path -Parent $PSScriptRoot) 'crates/game-model/Cargo.toml'

# **The previous instance is stopped first, because otherwise this script cannot start.** Running
# it a second time reported:
#
#     error: failed to remove file `target\debug\examples\review-web.exe`
#     Caused by: Access is denied. (os error 5)
#
# **Windows holds an executable open while it runs**, so `cargo` cannot relink over it - and the
# message names a file in `target/`, which says nothing about the app being up. Sean hit this
# running `review.ps1` with the app already serving, and the script's whole job is to be the door.
#
# **There is nothing to preserve.** The port is single-occupancy: a second instance would fail to
# bind 7878 even if it built, so the old one has to go for the new one to serve. **Stopping it is
# what running this script again asks for.**
#
# **Named rather than matched loosely**, so nothing else is touched, and it says what it did.
$running = Get-Process -Name 'review-web' -ErrorAction SilentlyContinue
if ($running) {
  $running | ForEach-Object { Write-Host "review: stopping the instance already serving (pid $($_.Id))" }
  $running | Stop-Process -Force
  # **Waited for rather than assumed.** `Stop-Process` returns before the handle is released, and
  # `cargo` fails on exactly that gap - which is this bug with a shorter window.
  $running | Wait-Process -Timeout 10 -ErrorAction SilentlyContinue
}

# Debug, unlike every other launcher here: it serves one page to one reader on one machine, and a
# release build would cost a minute of waiting to save milliseconds nobody is watching for.
cargo run --quiet --manifest-path $manifest --example review-web -- @args
exit $LASTEXITCODE
