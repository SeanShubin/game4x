# Generate or check the per-command regression expectations in regression/scenario/.
#
#   scripts\regression.ps1     writes any that are missing, fails on any that have moved
#
# **One given/when/then per command of `scenario/main.4x`**, generated. `{given}` is what the
# command took and `{then}` is what it made.
#
# **Delete a file to accept what the scenario does now.** `docs/process.md`: *when I change my mind,
# I delete the expected data and run the scenario again. Absent expected data means I accept what it
# does now, so the test writes it, and what I review is the diff in version control.* Nothing here
# ever overwrites a file that is present.
#
# `scenario/played.md` is the whole world turn by turn; these are the detail under it.

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
cargo test --quiet --manifest-path "$root/Cargo.toml" -p game-model --test regression -- --nocapture @args
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
