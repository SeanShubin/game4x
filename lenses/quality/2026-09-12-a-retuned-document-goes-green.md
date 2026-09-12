# A retuned document goes green

**2026-09-12.** Reviewing the code lane's burst at their own floor, `b7fc6a6..464bd68` - the range
taken from the commit that created the work rather than from a clock, and re-taken twice because
the tree moved under this review while it ran.

## What was asked, and what this found

The code lane named four places and one open question: *if any other check in the repository reads
a table cell by index, that is the finding I most want.* This report answers that question, and the
answer is that the index reads are not where the hole is.

**One finding is worth acting on.** `the_costs_in_the_model_are_the_costs_in_the_release` checks ten
of the thirteen cost constants, and the three it misses are unguarded by anything else. A retuned
release and a model that disagrees with it pass the whole suite.

## The finding

**Where.** `crates/game-console/tests/first_release.rs:390`, the population list; the constants are
`crates/game-model/src/game.rs:17`, `:18` and `:46`.

**What.** The test's own docstring states its job: *Nothing keeps a constant in Rust and a figure in
a markdown table in step except this.* It reads costs by name for `citizen`, `garrison`,
`extractor`, `yard`, `ark` and `pioneer`. **`store` is not in the list**, and `MOVE_CELLS` is not
reachable by the reader at all, because `released_cost` finds a recipe by the thing it produces and
`move` produces nothing.

So of the thirteen constants in `game::cost`, **ten are checked and three are not** -
`STORE_LABOR`, `STORE_METAL` and `MOVE_CELLS`. All three have a figure stated in the Recipes table:
`build store` consumes 1 labor and 1 metal, `move` consumes 1 energy.

**The count cannot notice.** `figures` is summed over the same hand list it is meant to validate, so
`assert_eq!(figures, 10)` is ten because the list has six names in it. A seventh name would make it
twelve and the assertion would have to move. **The denominator is the thing under test** - two
counts sharing a computation, which `docs/process.md` names.

**Why.** The reader is not at fault and neither is the model. `released_cost("store")` returns
`[(1, "labor"), (1, "metal")]` correctly - nothing asks it. What is lost is the guarantee the
docstring makes, which is the one this repository has already paid for once: *When `P-80` halved
three of them, the only thing standing between a retuned document and a model that quietly
disagreed with it was a test that reads both.*

## How it was measured

**Poisoned the thing the check reads**, which here is the release rather than the constant, because
the claim is about a retuned document. Run in a clone at `464bd68`; the shared tree was not touched.

**Poisoning the constant is caught, so that direction is not the finding.** `STORE_METAL = 7` fails
a CSS-class test - the scenario can no longer afford a store - and `STORE_METAL = 0` fails
`every_committed_dump_is_what_the_scenario_produces`. Both are incidental catches under a misleading
name, and both are real.

**Poisoning the release is not caught.** `build store` retuned to consume 3 metal, the model left
alone at 1:

| Step | What went red                                         | What it asked for           |
| ---- | ----------------------------------------------------- | --------------------------- |
| 1    | `every_committed_dump_is_what_the_scenario_produces`  | run `dump-state`, commit it |
| 2    | `the_release_tables_are_the_ones_in_this_crate`       | move the prototype's copy   |
| 3    | `metal_in_it_is_its_binding_plus_its_parts`           | the store's figure is now 3 |
| 4    | `the_committed_catalog_is_what_the_release_generates` | run `kinds -- catalog`      |
| 5    | **nothing**                                           | -                           |

**`cargo test --workspace` exits 0** with `releases/first-release.md` saying three metal and
`crates/game-model/src/game.rs` charging one. Four guards fired, each naming a different file to
bring into line, and **not one of them named the model.** The same run over `move` retuned to two
energy ends the same way: prototype synced, artifacts regenerated, suite green, `MOVE_CELLS` still 1.

**The population this counted against**: thirteen `pub const` in `game::cost`, ten named in the test
by `cost_of`, three not. Not a claim of zero against nothing.

## What to do

Add `store` to the population list and let `figures` become 12, so the two store constants are
checked the way the other ten are. `MOVE_CELLS` is a different shape - the recipe is not named for a
thing - and wants either a second reader or a line saying it is deliberately out.

**Whether: worth doing now**, and it is small. The docstring is what makes it worth doing rather
than the risk: a test that says nothing else keeps these in step, while three of them are not in it,
is the kind of sentence a later reader relies on instead of re-checking.

## What was checked and is not a finding

**`readies()` - found independently, and the code lane fixed it first.** It read cell 8 of a table
`P-466` cut to seven columns and compared it with `yes`, which `P-459` had already replaced with
counts; it returned an empty list unconditionally. Probed in a clone and confirmed empty, with the
value unreachable because no recipe row says *ready*. While that was being verified, `bb7c95b`
deleted the function, replaced the dead branch with an assertion, and added
`every_recipe_row_names_a_count_rather_than_readiness` with its population counted. **Their fix is
better than the repair this lens would have proposed**, which was to fix the index.

**The three new exceptions in `tools/outbox/tests/promotions.rs`: none is covering for the check.**
The tool's own run reports *3 of 7 named exceptions were inside the window*, so all three of today's
are exercised and genuinely failing. `P-456` is an ordering artifact - a cell superseded inside its
own promoting commit - and is unique. **`P-465` and `P-466` are both the wrong `shape` label**: a
before-and-after table describes a change, and `CLAUDE.md`'s own test makes that *an instruction*
rather than *rows*. That is the third and fourth instance of `P-236`'s defect, filed as `C-19` and
now `C-107`, and in every one of them the check was right and the proposal's label was wrong. **The
remedy is upstream of this tool**, so absorbing them as exceptions is correct and the growth of the
list is evidence about the queue rather than about the check.

**Other positional reads are being closed as this was written.** `recipes::gather` and
`recipes::said` read the Recipes table by index 0 through 6, which is correct for its seven columns
today; `14c4592` and `464bd68` landed a column-order guard and moved the editor to read by name
while this report was in progress. Not filed.

**`the_costs_in_the_model_are_the_costs_in_the_release` omitting `store` is not a regression from
this burst.** At `b7fc6a6` the same six names were there with twelve figures, and store was absent
from that too. `P-466` changed the number and not the hole.
