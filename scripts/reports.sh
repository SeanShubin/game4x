#!/usr/bin/env bash
# Write `reports/` - the index and the pages under it.
#
#   scripts/reports.sh     regenerates every page, and says how many moved
#
# **Sean, 2026-09-28**: *I want to re-create a structure that allows me to navigate all
# information about my tests and supporting data indexed from an html file.*
#
#   reports/index.html        the hub
#   reports/tests.*           every test, its foundation form, and whether he has read it
#   reports/ruleset.*         every rule, how many read tests fire it, and the data under them
#   reports/scenario.*        the main scenario, one case per command, by turn
#   reports/types.*           one case per relation the game declares
#   reports/primitives.*      one case per word the engine implements
#   reports/unused-rows.md    what no reviewed behaviour depends on - written by the sweep
#   reports/unused-values.md  the same for values
#
# **`R-9`'s three clauses and `tests/browsable.rs` holds them**: every reference is a link that
# resolves, every page has a markdown sibling to diff, and no page needs JavaScript to be read.
#
# **The two `unused-*` pages come from the mutation sweeps**, which take about sixteen minutes and
# run on `--ignored`, so this does not run them:
#
#   cargo test --release -p game-model -- --ignored
#
# `reports/foundation/` is written by `cargo run -p game-model --example foundation`.

set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cargo run --quiet --manifest-path "$root/Cargo.toml" -p game-model --example index -- "$@"
