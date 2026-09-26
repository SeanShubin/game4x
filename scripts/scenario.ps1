# Play the main scenario and print it turn by turn, for confirming by hand.
#
#   scripts\scenario.ps1             the turns, the commands, and the world at each turn's end
#   scripts\scenario.ps1 --detail    the same, with what every command took and made
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
