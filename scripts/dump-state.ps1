# Write state.md: the state after commands/setup.4x and commands/play.4x, as tables.
# Every table and every column is named whether or not anything is in it, because the
# names are what it is for. See crates/game-console/src/dump.rs.

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
cargo run --quiet --manifest-path "$root/Cargo.toml" -p game-console --bin dump-state -- @args
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
