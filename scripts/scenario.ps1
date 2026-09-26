# Play the main scenario and write scenario/played.md, for confirming by hand.
#
#   scripts\scenario.ps1     plays it, writes the file, prints nine lines saying what happened
#
# **Then read `scenario/played.md`**, which is where the playthrough is. Sean, 2026-09-26: *I see a
# terminal pop up and a bunch of commands fly by too fast for me to grok* - so nothing to grok is
# printed. `R-9` already says how this repository is read: *I can browse the reports without a
# script running*.
#
# **The file is committed**, so its diff is what says a rule moved, and
# `the_committed_playthrough_is_current` fails if it has gone stale.
#
# `spec/scenarios.md`: there is one main scenario, and it touches everything a typical game
# uses. It is the foundation, and **it is vetted by hand** - so this exists to be read rather
# than to pass. `crates/game-model/tests/scenario.rs` is the part a gate holds.
#
# The scenario is `scenario/main.4x` and the rules are `spec/data/`. Nothing here decides
# anything; see `crates/game-model/examples/scenario.rs`.

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
cargo run --quiet --manifest-path "$root/Cargo.toml" -p game-model --example scenario -- @args
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
