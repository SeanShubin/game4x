# Generate or check the three suites that are not about a scenario.
#
#   scripts\suites.ps1     writes any case that is missing, fails on any that has moved
#
# **`docs/process.md`**: *every type of thing that is data has a generated suite of its own, and
# there are four: the commands a scenario ran, the transformations over the things, the
# definitions of the things, and the words the engine implements.* The first is
# `scripts\regression.ps1`; these are the other three.
#
#   regression/rules/         one case per rule of `spec/data/rules.4x`
#   regression/types/         one case per relation of `spec/data/schema.4x`
#   regression/primitives/    one case per word the engine implements
#
# **A case holds the lines of the friendly source that define one thing, verbatim**, so a diff in
# a case is a diff in the file it came from and never an artifact of rendering.
#
# **Delete a file to accept what it says now.** `docs/process.md`: *absent expected data means I
# accept what it does now, so the test writes it, and what I review is the diff in version
# control.* Nothing here ever overwrites a file that is present, and a failure prints the deletion
# at each grain so it can be pasted rather than composed.
#
# `regression/` sits beside `reviewed/` because the two are siblings in meaning: one is the record
# of what has been read, the other of what has been accepted.

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
