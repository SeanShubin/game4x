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
# **Both binaries, because `regression/` is four suites and this is its door.** `tests/regression.rs`
# writes `regression/scenario/` and `tests/suites.rs` writes the other three. **Until `S-209` this
# named only the first**, so deleting `regression/types/` and running this passed, wrote nothing,
# and left fifty-one files deleted - and an empty diff reads as *the cases were already current*,
# which is the one conclusion that must never be available by accident. **The workspace gate ran
# both and hid it**; only a person following this path saw nothing happen.
# **On one line, and that is not a style choice** - `hooks/pre-push` records the same bug
# costing a run of 0 of 638 tests: a continuation arrived as a literal backslash-n, reached
# `cargo test` as a positional argument, and a positional argument is the test-name filter.
cargo test --quiet --manifest-path "$root/Cargo.toml" -p game-model --test regression --test suites -- --nocapture @args
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
