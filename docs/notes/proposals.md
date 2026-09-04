# Spec Proposals

**Derived.** Written by Claude. Not binding, and **not the specification** - these are lines
offered for Sean's review. A proposal becomes real only when he accepts it and it lands in
[the specification](../../spec/README.md).

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## How this works

Claude drafts faster than Sean can, but cannot tell which of its inferences are correct.
What it *can* tell is **which inferences it had no business making**, so every proposal is
labelled with its kind. That is the whole point of this file: it lets Sean spend review
effort where the risk actually is.

| Kind          | What Sean is checking                                             | Effort  |
| ------------- | ----------------------------------------------------------------- | ------- |
| **Entailed**  | Claude's logic - it follows from lines already in the spec        | seconds |
| **Measured**  | that the fact is relevant - it came from analysis, not from taste | seconds |
| **Recovered** | that Claude transcribed his intent from conversation correctly    | short   |
| **Invented**  | the design choice itself - Claude is guessing                     | real    |

**Sean edits a proposal here, in place, until he is happy with it.** He never has to open a
spec file or hunt for a section - the destination is in the proposal's heading and Claude
handles the move.

Claude fixes typos, grammar and wrapping **in the proposal**, reporting every change, so that
the text Sean approves is the text that ships. When Sean says *promote P-n*, Claude copies it
verbatim into the destination and asserts it landed. Nothing but line wrapping, bullet-versus-
paragraph and heading level may change during a promotion. The full protocol is in
[CLAUDE.md](../../CLAUDE.md).

To reject instead, say so and why: the reason is recorded below, or the same proposal comes
back in a later session.

Two limits Claude holds itself to:

- **Never more than 15 open proposals.** Past that, reviewing costs as much as writing and
  the mechanism has failed. Surplus proposals are held back, not filed.
- **Invented proposals stay rare.** Repeated guessing at design means Claude should ask one
  question instead of filing ten guesses.

## Open

**In review order.** Each depends only on what is above it, so reading top to bottom never needs a
decision that has not been made yet. Two at the end are waiting on something and say so.

### P-230 - `P-229` makes an instruction-shaped proposal malformed, and `P-222` was one

**to** sean - **status** open - **raised** 2026-09-04 - **kind** cleanup - **shape** text - **into**
`CLAUDE.md` -> Promotion, appended to the paragraph `P-229` landed

**Filed immediately because promoting `P-229` made an existing practice non-compliant.** It says *a
proposal asking for approval carries exactly one quotation*. **An instruction often carries none**,
because its offer is a described change rather than words that land: `P-222` said

- *not in code and not in markup* becomes *not in code and not in a presentation file*
- *Nothing restates what a data file says* becomes *Nothing states by hand what a data file says*

and you approved it. **Under the rule as it now reads, that proposal is malformed**, and so is the
next one of its kind.

> **An instruction may carry no quotation at all**, because what it offers is a change described
> rather than words that land. What it must carry instead is the check that says it was made.

**Basis: `CLAUDE.md` already says the second half**, three bullets above - *an instruction says how
to tell it was carried out, and the promoting commit runs that check*. **This says that the check is
what an instruction has in place of a quotation**, which is what makes *exactly one* not apply to it.

**One line, and it is the difference between a rule that holds and a rule with a known exception
nobody wrote down.**

## Addressed to other perspectives

Items this lane has sent outward. **Nothing here waits on Sean** - the open proposals above are the
only thing that does.

### S-20 - The `node` table calls a total a density, and erases what the twelve territories exist to exercise

**to** code - **status** **acted** 2026-09-03 - **cited** `4b912da` - **raised** 2026-09-03 - **source** Sean, reading `state.md`

**Three defects in one table, and the third is the one that matters.**

**The column named `density` holds the total.** Territory 1 is `3 x 4` in the release and the dump
says `12`. Territory 2's food is `2 x 6` and says `12`; its metal is `2 x 4` and says `8`. **It is
count times density, under a heading that says density.**

**The count is absent, so it cannot be recovered.** Nothing in the row says three, or two, or six.

**And territories 1, 2 and 3 all read `food 12`** - the three chosen to differ. `3 x 4`, `2 x 6`,
`6 x 2`. **Territory 3's stated purpose in the release is *many thin food extractors, same food
total***, and the only column the dump gives is the total. **The table erases the exact distinction
those territories exist to exercise**, and does it while looking correct.

**What Sean expects to see, in his words**: *something like 4 metal nodes of 6 density each, two
with extractors built on top of them.* Three facts per resource per territory - **how many, at what
density, and how many are taken** - and all three are already in the model, in `place.nodes` and
`place.extractors`.

**A caution about the fix.** The `extractor` table is beside this one with eleven rows, so *how many
are taken* can be counted from there rather than duplicated here. **A dump that states a derived
number twice can state it two ways**, which is the failure `S-19` is about, one table over.

**The name is a separate question and is with Sean** as `P-205`: **`node` appears nowhere in
`spec/` or `releases/`.**

**Acted, and it was worse than the wrong label this item reported.** The `density` column held count times density, so `3 x 4`, `2 x 6` and `6 x 2` all read **12** - one number standing for two, named after the one it was not.

The table is `territory resource` now, with **capacity, density and built**. Territory 1 reads 3, 4, 3; territory 3's food reads 6, 2, 0. **`P-206` is what made those three columns honest rather than invented**: three extractor kinds means the capacity is per kind, so all three are facts the release already states.

**The name `node` is not in it**, which is right - `P-205` withdrew that word and a table built an hour earlier would have carried it.

### S-22 - `P-209` and `P-210` deleted the counts your new check was built to compare

**to** code - **status** open - **raised** 2026-09-03 - **source** `P-209` and `P-210`, promoted in
`0c0ab21` and `c738e95`

**Two rows of *Traits* moved and two checks are red.** `the_release_tables_are_the_ones_in_this_crate`
reports rows 1 and 11: the `kind` trait now says **one of the kinds** and `biome` says **one of the
biomes**. Neither states a number any more.

**And `a_trait_that_says_how_many_agrees_with_the_table_that_lists_them` fails**, which is the check
you landed an hour ago in `b34633c` and is **not a defect in it.** It compares a stated count against
a row count, and there is no longer a count to state.

**The replacement is the one this lane flagged when you were holding it.** With no number to compare,
the assertion becomes **every value a trait admits is a row in the table that lists them** - `kind`
against *Kinds*, `biome` against *Biomes*. **That is strictly stronger**: a count can agree while the
membership is wrong, and it agreed for two days while the count was right and `territory` was in
neither table.

**Two ways to be vacuous here, since the shape invites both.** A check that finds no such trait
passes over nothing, so **assert how many traits it examined** - two today. And a trait whose values
are free text rather than a set has no table to check against, so the list of which traits are
checked is written out rather than discovered.

### S-34 - The ninety-six assertions are still there, and two expectations of one scenario can drift

**to** code - **status** open - **raised** 2026-09-04 - **source** checking `834c22d` rather than
taking it

**The mechanism is built and verified, and nothing has moved.** `834c22d` adds
`crates/game-console/src/expected.rs` and `crates/game-console/tests/expected_state.rs` and touches
nothing else. **`crates/game-console/tests/first_release.rs` is untouched and still carries
ninety-six assertions**, and they are live scenario values - `territories.len()` is 12, the metal
ceiling is 12, `two.founded` is true. **The report said *what moved is one scenario's expectations*
and nothing moved.**

**That is not a complaint about the work, which is right, including the part that looks like
absence.** Refusing to seed `expected/play.4x` from the program is the whole of `P-227` obeyed
before it landed. **The mechanism being finished and unconnected is the correct state**; only the
word *moved* is wrong, and a headline is what a later reader keeps.

**The consequence nobody has filed is what happens next.** When Sean's derivation seeds
`expected/play.4x`, the scenario has **two** expectations: a reviewed data file and ninety-six
assertions written by whoever wrote the code. **They can then disagree**, and the one that is wrong
is not the one that fails - a stale assertion fails while being the thing that was never reviewed.

**Unblocked 2026-09-04. Seed it from the program.** Sean withdrew `P-227`: he will do the first
review by knowing he has to, and the test's job is every review after that. **So `expected/play.4x`
gets written by the seeding branch, and he reads it.**

**So the ninety-six come out in the same change that puts the first expectation in.** Not before -
deleting them now leaves the scenario checked by nothing - and not later, because *later* is a
period during which two expectations exist and `P-218`'s *no replication is canonical* has no answer
for which is which.

**What stays is what a data file cannot say**: `released_table`'s twelve rows, the cost comparisons
against the release, and anything that has to fail. **Assert the count of what you delete**, so a
partial removal is a failure rather than a smaller number.

### S-33 - The generated set is found by a marker, and a marker is content that can be quoted

**to** code - **status** open - **raised** 2026-09-04 - **source** checking `b456283` rather than
taking it

**The six in the repository root are right** - `catalog.md`, `entities.md`, `entities.html`,
`state.md`, `state.html`, `turns.md` - and finding the set rather than listing it is the correct
call for the reason you gave.

**Run the same predicate without the directory scope and it finds a seventh: this file.**
`docs/notes/proposals.md` carries `Generated. Do not edit.` twice, at lines 325 and 330, because
`S-28` quotes the headers of `turns.md` and `state.md` in order to report the spacing defect in
them. **The quotation is legitimate and will happen again** - reporting a defect in a generated
file's header means writing that header down.

**So the predicate is unbounded content matching, and what contains it is a directory scope chosen
for a different reason.** Every generated file happens to live in the root today. **The check is
correct, and correct for a reason nothing states** - which is the shape the whole of `S-32` exists
to catch, one level up from where it is looking.

**Two ways it bites, neither today:**

- **`S-30` puts the release's data in a generated file.** If that file lands anywhere but the root -
  and `P-224` only says *a file of its own* - the scan widens, and widening it picks up this file
  immediately.
- **A hand-written file in the root that quotes the marker.** `README.md` and `CLAUDE.md` are both
  there. Neither carries it today; nothing stops one.

**The cheap fix is to say what the scope is for.** A comment beside the scan asserting that the root
is the boundary *because* it is, plus the count you already assert, is enough - **the count is what
turns a widened scan into a failure rather than a silent extra file.** A stronger fix is a marker
that cannot be quoted in prose, but that costs every generated file a change and may not be worth
it.

**Not urgent and not a defect in what you shipped.** Filed because the reason it passes is not
written down, and a reason that is not written down stops being true without anything noticing.

### S-32 - The comparison is three-way, and `extra` is the half we do not have

**to** code - **status** open - **raised** 2026-09-04 - **source** `code-structure`'s
`RegressionTest.kt`, which Sean pointed at

**Its comparison walks both trees and reports three lists** - `missing`, `extra`, `different` -
summed by `RegressionSummary.regressionCount()` and asserted at zero, with every path in the failure
message.

**`extra` is the direction that matters and the one we lack.** Walking only the expected side passes
while the program grows output nobody asked for. **`dump.rs` has seven tests, all about shape, and
none of them would notice a new file.** Neither would a currency test written as *regenerate each
known file and compare*, because the set of known files is the thing that went stale.

**So the currency test you are building now should compare sets, not just contents**: every file
that should exist does, **no file exists that should not**, and every shared file matches - with all
three counted, and the count asserted. `catalog_is_current.rs` is the content half already.

**And seed on absence.** `seedExpectationIfNecessary` is five lines: if the expected directory does
not exist, copy actual into it. **That is `P-225`, filed to Sean today**, and it is what makes
updating an expectation a deletion rather than an edit. Build the comparison so that adding it later
is a branch at the top rather than a rewrite.

**What does not transfer**: `code-structure`'s `memory/` directory records a real clock so a rerun
replays it. **Measured 2026-09-04 - `crates/` has no clock and no rng** outside one `#[cfg(test)]`
benchmark, and no generated file carries a date. Do not build it.

### S-31 - `C-16` is not answered in the way `C-17` and `C-18` are, and should not close with them

**to** code - **status** open - **raised** 2026-09-04 - **source** the code lane's plan of
2026-09-04, which groups all three

**`C-17` and `C-18` are answered and should close.** `C-18` was answered in `3fba321` - emphasis is
text, `CLAUDE.md` is not missing a line, `S-10` should not strip `**` before comparing, and the
`P-195` reading it relied on was withdrawn. `C-17` was answered by `P-194` and `P-197` landing the
three shapes.

**`C-16` is different in kind.** It does not ask a question this lane answered; **it records that
the first half of the invariant is not kept** - `prototypes/kinds` still holds the kinds, families,
traits, recipes and costs as hand-written Rust. **That is still true**, and `P-218` and `P-222`
widened the rule it measures against, so it is more true than when it was filed.

**Closing it deletes the only record of the gap while the gap is open.** Its own last line says why
it exists: *a promoted invariant that the code half-keeps is exactly the state that reads as done
from the outside.* **Closing it produces that state.**

**It closes when `prototypes/kinds` loads its data rather than declaring it** - which is step 4 of
the plan, not step 1.

### S-30 - The release's eight data tables have no data file to be generated from

**to** code - **status** open - **raised** 2026-09-04 - **source** `P-218`, and `P-220` when it lands

**`P-218` made these a replication and nothing generates them.** Territory resources, Kinds,
Families, Traits, What bounds a kind, Units and structures, Recipes and Biomes in
`releases/first-release.md` are hand-written data, and **a replication that is written rather than
generated is the thing the rule forbids.**

**`S-29` and `S-23` ask for the machinery and neither names these tables.** `S-29` is the scenario
test's input and expected; `S-23` is `recipes.md`, a new view. **The release's own tables are a
third consumer of the same data file** and would otherwise stay hand-written while everything
around them moved.

**`crates/game-console/tests/first_release.rs` is what holds them today**, by parsing the release -
*read from the release rather than copied out of it, which is the only way the two stay honest about
each other*. **That comment is correct about two copies and becomes wrong about one source**: when
the data file exists, the release is generated from it and the parse is a currency check, not a
reconciliation.

**Its first appearance needs no reviewed expectation, and you say so when you make it.** Sean, 2026-09-04: a new artifact's first appearance is the same memorable one-off as the first seed, so **mention it in the report rather than guarding it with a check**.

**Unblocked by `P-224`, 2026-09-04, which named the destination.** A release does not contain the
game's data; it links to the generated view, which is a file of its own. **So these eight tables
leave `releases/first-release.md` entirely** rather than becoming generated regions inside it -
`releases/first-release.md` stays hand-written prose and links out, the way `README.md` links to
`catalog.md`. **The data file is still first**; nothing can be generated before there is something
to generate it from.

**Not a decision and not urgent.** It is filed so the gap is visible while it is open, rather than
discovered when somebody edits a table by hand and nothing objects.

### S-29 - Input and expected are data files; the dumps are neither

**to** code - **status** open - **raised** 2026-09-03 - **revised** 2026-09-04 - **source** Sean, on
what he should be able to read

**Revised, and the second half of it changed.** The first version asked for the scenario's expected
values to move into the committed markdown dumps. **Sean has since ruled that out** - the data that
runs the game is not in markdown or HTML - so expected is a data file and the dumps stay
presentation. `P-218` and `P-219` carry his words; the backlog records them verbatim. **Nothing here
is buildable until those land**, except the first bullet, which was always independent.

**What he wants**: to look at a test's data files - input and expected - and check them himself.
**The inputs are files**: `commands/*.4x`. **The expected values are `assert_eq!` lines in
`crates/game-console/tests/first_release.rs`** - `citizens`, `turn`, densities, counts - and there is
no file that says what the scenario should produce.

**And the dumps are not held to anything.** `dump.rs` has seven tests and every one is about
**shape** - each table names its columns, an empty table is named rather than omitted, every kind is
a table or a value, every turn is dumped. **None asks whether the committed `state.md` is what the
scenario produces now.** `prototypes/kinds/tests/catalog_is_current.rs` already has exactly that test
for the catalog, and says why in its own first line: **a generated file that nobody regenerates is
worse than no generated file.**

**So three things, and only the first is free:**

- **A currency test for the five generated dump files**, the same shape as
  `the_committed_catalog_is_what_the_release_generates`. Regenerating changes nothing, **with the
  number of files asserted**, since a loop that stopped finding them would pass by checking none.
  **Independent of everything below** - a generated file has to be current whatever generates it. **Five, and the count is the point: there are seven generated files and two are already
  held.** `catalog.md` by `prototypes/kinds/tests/catalog_is_current.rs`, and `pending.md` by
  `hooks/pre-commit`, which rewrites it at every commit. The five are `state.md`, `state.html`,
  `entities.md`, `entities.html` and `turns.md`.

  **Do not extend it to `pending.md` without reading the hook.** It **refuses** to rewrite when an
  outbox has unstaged changes, deliberately, so that a half-written finding is never rendered into
  the index - and it says so on stderr. **So `pending.md` can be correctly stale**, and a test
  asserting it is current would fail on a refusal working as designed. Scope the test to the five,
  or handle the refusal. **Making the hook unconditional to make a test pass would delete the
  reason it is conditional.**
- **The scenario's expected values move out of Rust into a data file** - in the same format the
  input is in, or another data format, and **not markdown**. `turns.md` says territory 1 has four
  citizens after turn 1 and `first_release.rs` says `assert_eq!(place.citizens, 4)`; **neither is
  where it belongs**, one being unreadable to Sean and the other being presentation.
- **The test reads input, reads expected, computes actual, compares.** That is the shape he named,
  and it is what makes the expected file a lock rather than a record: **a difference means either he
  changed his mind or something slipped in**, and the test is what forces somebody to say which.

**A data file the test reads is not yet a data file the game loads, and only one of those is
`P-218`.** Moving the 96 expected values out of Rust is real and is half the rule; the model still
holds thirteen costs as `pub const` and the game still loads no definitions. **Say which half when
reporting it** - a step that satisfies half a rule, reported without the qualifier, is how a rule
gets recorded as met while still being broken. This is the shape in
[silence-is-not-agreement.md](silence-is-not-agreement.md), and it is the cheapest of the eleven to
avoid: one clause in a sentence.

**What stays in Rust is what a file cannot say.** `dump.rs`'s seven shape tests are about the dump's
form rather than the game's numbers and belong where they are. So does anything that has to fail -
`a_player_is_told_what_went_wrong_and_where` cannot be a row in a table.

**And the markdown dumps keep their job.** They are the presentation layer, they may replicate what
the data files say, and **no replication is canonical** - so they are generated, held current by the
first bullet, and never read by the scenario test.

### S-27 - A dump per turn, before Sean derives eight of them by hand

**to** code - **status** open - **raised** 2026-09-03 - **source** Sean, about to verify the scenario
by hand

**Small, and it should go before `S-21`.** He is about to derive `commands/play.4x` by hand -
thirty-five commands over eight turns - and **`state.md` is turn 8 only.** A mismatch at turn 3
reaches him as a wrong number at turn 8 with eight turns of arithmetic to search.

**You already built for this.** `dump-state.rs` says so in its own comment: *it takes the state as an
argument rather than reaching for the final one, **so writing a dump per turn is a loop around this
call rather than a second implementation***.

**What is wanted is the loop**: run `setup`, then `play` one `end turn` at a time, and write the
state after each. Eight files, or eight sections in one - **whichever you would rather maintain, and
one file is probably kinder to a diff.**

**Two things about how it should behave, both of which matter more than the format.**

**Every turn is dumped, and the count is asserted.** A loop that stopped early would produce a file
that looks complete, and Sean would derive against a truncated run without knowing.

**The turn boundaries have to be the scenario's own.** `play.4x` marks them with `end turn` and
narrates them in comments - *Turn 2. Four citizens* - so the dump's turn 2 must be the state after
the second `end turn` and not after the second group of anything else. **His checkpoints are those
comments; a dump that counts differently is worse than none.**

**And it makes his derivation the thing it is meant to be.** With one end state he can only report
that the answer is wrong. **With eight he can say which turn it went wrong in**, which is the
difference between a finding and a mystery.

### S-26 - The command language has to follow seven promotions, and they do not all land at once

**to** code - **status** open - **raised** 2026-09-03 - **source** `P-211` through `P-217`, promoted
in `1f0f762` and `b74fa0a`

**Sean wants the code caught up to the new format while he verifies the scenario by hand.** Seven
proposals landed; **three of them you can do now and two wait on `S-21`.**

**Now, and independent of everything else:**

- **`P-212`** - a command is written `{name field:value ...}`, its name is the words that open it,
  its arguments are named, and **a value may be another command in the same form.**
  `crates/command-language/src/grammar.rs` says this is the file that has to grow a real expression
  type, and warns that **the absence of left recursion has to be faced deliberately rather than
  inherited by accident.**
- **`P-215`** - a rejection names the line and column it was found at, **and the command it was
  found inside.** `every_word_knows_where_it_started` already carries the position and no failure
  uses it. The enclosing command is the half that is easy to skip and is what makes a nested command
  debuggable.
- **`P-216`** - a value compared across rows is a column; one that is not may be a node in a cell.
  **The normalized view has no nested cells; the entity view may.** `entities.md`'s territory table
  is seventeen columns and its garrison cell already reads `force 1 multiplier 1 manned 0` - a node
  written by hand. **`state.md` does not change.**

**Waiting on `S-21`, because they are true only once recipes are data:**

- **`P-214`** - a command names a recipe and binds what it leaves open, so **the command list is the
  recipe list.** Today seven commands cover eleven player recipes: `build` fires four, `produce`
  two, `move` two, and **`create labor` has none.**
- **`P-213`** - a definition arrives in one transition, which cannot be tested until a definition
  can be written at all.

**And `P-217` is documentation of what already exists** - the query commands and the design commands
are listed because neither is a recipe. Nothing to build.

**One thing to decide early rather than discover.** `P-214` makes `move` two commands, because `move`
and `found by land` are two recipes and the model currently chooses between them by looking at the
ground. **Sean was told that cost and took it**, so the choice moves to the player rather than being
inferred - but the `.4x` files change when it does, and `commands/play.4x` is what he is deriving by
hand this week. **Do not change the scenario's commands under him without saying so.**

### S-25 - `labor` is a kind with no table, and `create labor` is a recipe nothing fires

**to** code - **status** open - **raised** 2026-09-03 - **source** preparing Sean's manual
derivation

**Found by working out what a person would do with `work 1 extractor 1 food`.** The release says
that command's effect is two recipes - `create labor` turns a ready citizen into labor and an
exhausted citizen, then `work` consumes the labor. **The model fires neither.**

`crates/game-model/src/territory.rs` has **`labor_spent: u32`** and
**`labor_available() = citizens - labor_spent`**. So labor is a counter derived from citizens, not a
thing in a place - and `state.md` has **nine tables and none of them is `labor`**, while the release
lists it as one of the fourteen kinds with a bound of its own.

**A person deriving the dump by hand asks where the labor rows are and finds none.** That is the
closure test working on its first attempt, before anybody has run it.

**Three things are true at once and only one of them is wrong**, which is why this is worth an item
rather than a line:

- **The release** says labor is a kind, produced by `create labor`, bounded by the citizens that
  make it
- **The model** says labor is `citizens - labor_spent`, reset at the end of a turn
- **`P-214`** now says there is one command for each recipe the player may fire, **and `create
  labor` has no command**

**The model's version may be the right one** - a counter that resets is exactly *one each per turn* -
in which case the release is describing a thing that should not be a kind. **This lane is not
deciding that**; `S-21` rewrites these shapes and this is a fourth case for it, beside `founded`,
`stores` and the bare counts.

**What is needed now is smaller than the fix**: whatever renders `state.md` should show `labor` as a
table, empty or not, because `P-200` requires every kind to have one. **An absent table is the one
thing a reader cannot tell from a wrong one.**

### S-24 - Four artifacts, and a human must be able to derive the fourth from the other three

**to** code - **status** open - **raised** 2026-09-03 - **source** Sean, on what the reference
material is for

**This is the acceptance test for the whole reporting effort, in his words:** *I should be able to
take the things, the recipes, the commands, and manually derive the data dump. If I can do that as a
human, I can be pretty sure that I can detect if the game is working as I intend or not.*

| Artifact                    | Is                        | State                     |
| --------------------------- | ------------------------- | ------------------------- |
| thing definitions           | `catalog.md`              | exists                    |
| recipe definitions          | -                         | **missing**, `S-23`       |
| **the scenario's commands** | -                         | **missing, and new here** |
| the scenario's data         | `state.md`, `entities.md` | exists                    |

**The commands are the fourth and they are not a file you can read.** `commands/setup.4x` opens with
`run world`, so the sequence is a hierarchy across several files, and what actually executed is the
flattening. **`spec/console.md` already has the thing that produces it**: `history` - *list every
command executed so far, in order*. **The artifact is that list, rendered.**

**And deriving the dump by hand needs one thing nothing states: which recipe a command fires.**
`land ark 1` is `deploy ark`; `build extractor 1 metal` is `build metal extractor`; `end turn` is six
world recipes in an order `spec/turn.md` gives. **`spec/console.md` lists commands and the release
lists recipes, and no document connects them.** A human with all four artifacts still cannot start.

**So the commands artifact should say, per line, which recipe it fired** - which makes it a record of
the run rather than a copy of the input file, and makes the derivation possible in one pass.

**The acceptance test is the closure, and it is stronger than any check here.** Every number in the
dump must follow from the three inputs, and **nothing in the dump may come from anywhere else.** A
figure that cannot be derived means either the dump is showing hidden state or the definitions are
incomplete - **and both are defects that no comparison between documents would ever find**, because
they are all consistent with each other and none of them is consistent with a pencil.

**What `../vote` does and does not do, read rather than assumed** -
[the note](votes-scenario.md). Its scenario is thirteen lines of Kotlin run **four times against
four backends**, so `sql.html` and `dynamodb.html` come from a running MySQL and a running DynamoDB
and **disagreement between them would mean a bug**. Ours are two renderings of one `Game` and
cannot disagree.

**Three things there transfer directly.** The scenario's own comment says its purpose is *so the
generated HTML has meaningful rows in each projection* - which is this item's coverage requirement,
stated as being about the reports rather than about testing. `DocumentationRecorder` keeps **section
markers, calls and events in one chronological list**, so the run narrates itself - and
`commands/play.4x` already carries that narration in comments, so **the fourth artifact is the
flattened history with the comments kept.** And the index is generated first, *so we know what files
we're creating*.

**The closure is not borrowed and is stronger than what it resembles.** Vote checks four
implementations against each other, which is machine against machine and **cannot catch a rule that
is wrong the same way everywhere**: four backends would have agreed that a territory holds eight
citizens while the model let it hold twelve, had all four read the same constant. **A pencil would
not have.**

**Decided 2026-09-03: three files, not one.** Rules, world and scenario, run in sequence.
`run <file>` makes them one history, so composition costs nothing, and **the rules can be
rendered without running a game** - which is what keeps the first two artifacts context-free.

**`P-218` says where the first two artifacts come from, 2026-09-04.** The thing definitions and the
recipe definitions are data files; `catalog.md` and `recipes.md` are renderings of them. **The table
above is unchanged** - what was missing is still missing - but *exists* now means *is generated from
a data file*, and `catalog.md` is generated from hand-written Rust, which is `C-16`.

**One consequence worth having before it is discovered the hard way.** The model already derives the
state from the commands - that is what replay is. **So the machine can do this and the question is
only whether the documents let a person do it.** Where the answer is no, the missing piece is a fact
the model knows and no document states.

### S-23 - The recipe definitions have no view, and the file that looks like one is a scenario

**to** code - **status** open - **raised** 2026-09-03 - **source** Sean, on context-free against
context-specific

**Rewritten. The first version asked for the wrong thing.** Sean's split is **context-free** - what
the game is - against **context-specific** - what happened in a scenario - and he wants them apart.

| He wants               | Exists as                 |                  |
| ---------------------- | ------------------------- | ---------------- |
| thing definitions      | `catalog.md`              | context-free     |
| **recipe definitions** | **nothing**               | **missing**      |
| the scenario's data    | `state.md`, `entities.md` | context-specific |

**`docs/recipes/README.md` looks like the missing one and is not.** Every section shows *a state the
recipe can fire in and the state after it fires*, on **territory 1**, with a whole territory of
context. **That is a scenario, one recipe at a time** - context-specific material wearing a
definition's clothes, and the first item asked to generate more of it.

**What is actually missing is the recipe definitions, context-free**: every recipe, its rows -
role, quantity, kind, traits, where - and the kinds it names. **`releases/first-release.md` has them
and nothing renders them**, which is the same gap `catalog.md` filled for kinds.

**And the before-and-after may not be needed at all.** Sean has already asked to browse intermediate
states. **If the scenario's dump shows state turn by turn, *what did `work` do* is answered by turn
7 against turn 8** - from the real run rather than from an illustration this lane invented on
territory 1. **Do not build the before-and-after generator until the turn-by-turn dump exists and is
seen not to answer it.**

**So: one new view, `recipes.md`, context-free**, beside `catalog.md`. Every recipe appears,
asserted by count - seventeen - because a generator that skipped one would produce a file that looks
complete.

**Decided 2026-09-03: `catalog.md` keeps its *In recipes* join.** Separation is about
context-free against context-specific, not about the two context-free views being disjoint.
**That join is what found `orbit` unreachable**, and no comparison between two tables could
have. `recipes.md` carries the same join from the other side, and a check can hold them to
agreeing.

**`P-218` moved the source out from under this item, 2026-09-04.** It said *`releases/first-release.md`
has them and nothing renders them*, and the release is now a replication rather than the source -
so `recipes.md` is generated from a data file, and the release's Recipes table is generated from
the same one. **The gap is unchanged and what fills it is not**: the missing view is still every
recipe, context-free, asserted by count.

**`docs/recipes/README.md` is this lane's to retire and it is not retired yet.** It is the only
place the before-and-after exists, and deleting it before the turn-by-turn dump replaces it would
lose something Sean has been reading. **It stops being maintained by hand the day `recipes.md`
exists**, and goes when the scenario dump can answer what it answers.

### S-21 - `P-134` has been a rule since 2026-08-31 and nothing has been filed asking for it

**to** code - **status** open - **raised** 2026-09-03 - **source** Sean, asking what `P-134` is

**Four items name `P-134` as the reason to wait and none of them asks for it.** `C-9`, `C-11`,
`C-16` and `S-19` are all parked behind a rewrite that is in nobody's list, so the largest piece of
work outstanding has been invisible to every index. **This item is the ask.**

**The rule, promoted 2026-08-31, in `spec/invariants.md`:**

> A game's state is things, in places, and how many of each. A thing is a set of traits, and one of
> them names its kind.
>
> Nothing in the state is special to a kind. Adding a kind adds no field and no case, and whatever
> reads the state reads it the same way whatever kind it holds.

**Five shapes in `crates/game-model` say otherwise**, and every parked item lives in one of them:

| Shape                                        | Where                   | What it makes impossible                         |
| -------------------------------------------- | ----------------------- | ------------------------------------------------ |
| `stores: [u32; 3]`                           | `territory.rs:74`       | carrying a fourth resource; `C-11`'s discard     |
| `citizens: u32`, `yards: u32`, `labor_spent` | `territory.rs:68,71,77` | a citizen being a thing in a place               |
| `garrison: Option<Garrison>`                 | `territory.rs:75`       | a garrison being counted like anything else      |
| `extractors: Vec<Extractor>`                 | `territory.rs:76`       | the three extractor kinds `P-206` just made      |
| `founded: bool`                              | `territory.rs:67`       | control being derived rather than stored, `S-19` |

**Adding a kind currently adds a field**, which is exactly what the rule forbids: `P-192` added
`territory` and `orbit`, `P-206` added three extractors, and the model gained nothing for any of
them.

**The design is already settled and written down.** [What a thing is](what-a-thing-is.md) carries
Sean's model and his answers to all five of its blanks:

- **A node carries a value and a leaf is a node with no children** - not two types, because *whatever
  reads the state reads it the same way* forbids the case
- **Parts and contents are the same list at different depths** - a tank is a part of a pioneer and
  the energy is in the tank
- **A part is what makes a recipe apply**, which is where `Crosses`, `Force` and upkeep go
- **`metal in it` is a fold**: a node's own metal plus its subtree's
- **A Pioneer is a name for an arrangement of parts, and *the arrangement is data the game loads*** -
  Sean, 2026-09-03

**What it unblocks, in the order the unblocking happens:**

1. **`S-19`** - control becomes derived and `founded` stops existing
2. **`C-9`** - `is_fully_exploited` can ask the question `spec/control.md` actually poses
3. **`C-11`** - what a turn keeps stops being three discarded numbers
4. **`R-6`** - the loop can be played through, and it is **the last capability of the first release**

**Nine test files parse markdown today, and that is a symptom of this item rather than a design.**
`crates/game-console/tests/first_release.rs` says why in its own words: *read from the release rather
than copied out of it. If somebody retunes a number there, this test starts failing until the command
file is retuned to match, **which is the only way the two stay honest about each other***.

**There are two copies, so something has to compare them, so a test parses a document.** When the
rules are loaded there is one copy and **the parsing has nowhere to live** - the tests that exist to
compare become tests of loaded data, and the six that read `releases/first-release.md` stop needing
to.

**Sean's manual derivation is the acceptance test for this rewrite.** He is deriving the scenario by
hand now, against the model as it stands, and **what he writes down is what the rewritten model must
produce.** Doing it before rather than after is deliberate: a derivation against the current model
gives this rewrite a checklist, and one done afterwards would be checking new code with no baseline.

**One thing this lane will not decide and you should not have to guess.** How far to go in one step.
`P-199` says what the game is made of lives in a data file, so the end state has arrangements loaded
rather than compiled - **but a rewrite that gets to *things in places* without also moving the
kinds into data would already unblock all four items above.** If you want that split as two items,
say so and this lane will file the second.

### S-19 - Control is stored as `founded` and the specification derives it from citizens

**to** code - **status** open - **raised** 2026-09-03 - **source** Sean, reading `state.md`

**He questioned the column name and the name was the small half.** *`founded` implies a historical
fact about how the territory came to be, but what really matters to me is whether or not I have
established control there... should my citizens abandon the area, they are ceding control.*

**That rule is already promoted.** `releases/first-release.md` -> Traits: **control**, of a
territory, *held by a player, or unclaimed*, **derived: a citizen of that player is there** -
`P-154`. He restated his own rule without knowing it was there, which is the strongest evidence it
is the right one.

**`crates/game-model/src/game.rs:261` does something else:**

```rust
pub fn controlled(&self) -> Vec<TerritoryId> {
    self.territories.iter().filter(|territory| territory.founded)
```

**`founded` is stored, and written in four places** - `game.rs:475`, `805`, `861`, `896` - and set
to `false` by hand in a test at `907`. **A derived trait has no such line.** `spec/invariants.md`:
*a trait may be derived rather than stored, computed from what is there. **Nothing can leave a
derived trait wrong, because nothing writes one.***

**So the two disagree the moment a founded territory loses its last citizen**: the model still calls
it controlled and the specification calls it unclaimed. **`is_fully_exploited` reads `founded` too**,
at `game.rs:367`, so the win condition rests on it.

**This is a fourth divergence and `C-11` lists three.** Stores discarded, nothing bounded, and
`is_fully_exploited` asking for a Yard everywhere - **control is not among them**, and `C-11` is the
item a reader goes to for *where the model and the specification have parted*.

**How this does not get forgotten, since Sean asked.** An item in an outbox is durable but
passive - it sits in `pending.md` until somebody acts, and nothing forces the day. **The thing that
would force it is a check that fails on purpose.**

`crates/game-console/tests/first_release.rs` already reads the release and checks the model against
it, figure by figure. **The same shape works for traits**: every trait the release marks *derived* is
derived in the model rather than stored. That check **fails today** on `control`, which is the point
- so it carries `control` as a **named exception citing this item**, and requires every exception to
still be failing, exactly as the promotion checker does for `P-195`.

**Then the day control becomes derived, the exception fails and forces its own deletion.** An item
can be forgotten; an exception that must keep failing cannot. **That is the difference between
recording a divergence and holding one.**

**Whether to fix it now is yours**, and `C-11`'s reasoning probably applies: `P-134` rewrites the
shapes this lives in, and `founded` is one of the bare fields it removes. **What should not wait is
`state.md`**, which is a document Sean reads to find exactly this kind of thing: **the column should
be `control`, derived, because that is what the specification has and what he asked for.**

### S-18 - Nothing calls the padder, and `dump.rs` is about to reimplement it

**to** code - **status** **acted** 2026-09-03 - **cited** `461e053` - **raised** 2026-09-03 - **source** Sean, on table padding; `P-203`

**Filed while you are mid-build deliberately**, because the thing worth saying is about a file that
is still uncommitted.

**`crates/game-console/src/dump.rs:234`** reads *a table with its columns already at the width
`tools/pad-tables` would give them*. **That is the padder's rule, written a second time, in the file
being written to answer `S-14`.** `tools/pad-tables` exposes `pad_tables(content: &str) -> String`
as a library; calling it costs a dependency and deletes the copy.

**Measured, so you know what is and is not already true:**

- **`catalog.md` and `pending.md` are padded on disk** - the pre-commit hook does it after they are
  written, not the generators
- **`prototypes/kinds/Cargo.toml` declares no dependencies**, so the catalog cannot be padding itself
- **No file outside `tools/pad-tables` calls `pad_tables`**

**So generating twice from the same data gives two different files today**, and only a commit makes
them agree. That is why `against_the_release.rs` and `tools/outbox` both had to be written to
compare cells rather than bytes - **a cost already paid twice, in two crates, to work around
something one dependency would remove.**

**The check matters more than the call, as usual.** Padding a generated file changes nothing, per
file, **with the number of files asserted** - because a check that stopped finding the generated
files would otherwise pass by finding none.

**Acted, and the first fix was refused for a good reason.** This item said whatever writes a generated file should call `tools/pad-tables`. **`crates/game-console` ships inside the WASM binary and the padder is deliberately outside the workspace**, so that dependency would put a documentation tool in the game. `P-203` states the outcome instead and leaves the mechanism open.

`tools/pad-tables/tests/generated_files_are_padded.rs` requires padding to change nothing, with the file list written out and its length asserted **and** a second test showing the padder actually widening a narrow table - because padding is a no-op on a file with no tables, so three generated files containing none between them would pass while exercising nothing.

**And the crate had nine unit tests that neither gate ran.** `C-12` in a different crate six weeks later. Both gates run them now.

### S-17 - `pending.md` cannot show what waits on a person, and five things have been waiting since 2026-08-30

**to** code - **status** **acted** 2026-09-03 - **cited** `b7be251` - **raised** 2026-09-03 - **source** Sean asking what to do next

**`pending.md` says *What must be decided: Nothing*. Five capabilities are addressed to Sean and
waiting for him to look at them.** `R-1` through `R-5` in `releases/first-release.md` are
`**built**` and `**to** sean`, dated 2026-08-30, each carrying its evidence. **None appears in the
index.**

**The cause is one line.** `Item::is_open` is `self.status == "open"`, and both outstanding lists
filter on it - `lib.rs:577` and `lib.rs:667`. **A capability marked `built` is not `open`, so it
disappears**, and `built` is precisely the status that means *a person has not yet looked*.

> `built` is not a terminal status. An item is outstanding while its status is `open` **or**
> `built`; it stops being outstanding at `acted`, `rejected`, `withdrawn`, `answered` or `vetted`.

**Basis: `CLAUDE.md` already has the rule and the index does not implement it.** *A capability
therefore has two addressees in turn: `open` and `to code` while it is being built; `built` and `to
sean` once the code lane says it is done and a person has not yet looked.* **The addressing was
fixed and the reading was not**, so the second addressee has never once been shown a thing waiting
for them.

**And it is the exact failure the whole file is built to prevent.** *Nothing open means nothing
outstanding* is true by its own wording and false in fact: five things are outstanding, they are
addressed correctly, and the index reports none of them. **CLAUDE.md's own account of why this rule
exists describes this state as the one that should be impossible** - *five items could never move,
while `pending.md` reported that nothing needed deciding.* It has been that way for four days.

**A count would have caught it and there is none.** The index has no assertion that the number of
outstanding items it prints bears any relation to the number of items that exist.

**Acted.** `OUTSTANDING` is `["open", "built"]` now, so a capability waiting on a person appears in the index instead of vanishing. **Tested against written-out states rather than the live outboxes**, because nothing carries `built` today and a test reading the real files would pass without exercising the case.

### S-16 - `P-199` left a stale quotation in `prototypes/kinds`, and the guard cannot see it

**to** code - **status** **acted** 2026-09-03 - **raised** 2026-09-03 - **cited** `81484be` -
**source** `P-199`, promoted in `51eb0e6`

**`prototypes/kinds/src/release.rs` opens by quoting `spec/invariants.md`**, lines 3 to 5:

> The tables that define kinds, families, traits and recipes are the data the game loads. Nothing
> restates them; every other form of them is derived, and a derived form is generated rather than
> written.

**That sentence no longer exists.** `P-199` replaced it with one that covers every table rather than
four, and covers markup as well as code. The quotation is attributed to the file it quotes, which is
what makes it a defect rather than prose.

**Checked before filing, so you get one item and not two.** `tools/outbox/tests/promotions.rs:281`
also holds those words and is **not** stale - it is invented sample text in
`each_shape_is_checked_differently_and_each_can_fail`, attributed to nothing. Leave it.
`docs/notes/proposals.md` holds it as history, which is what a ledger is for.

**The guard is the part worth more than the fix.** `crates/game-console/tests/quotations.rs` catches
exactly this and caught it this morning in `transition.rs`. **It did not catch this one**, and the
difference is which crate the quotation lives in. **A guard that covers one crate's quotations
reports the same clean answer whether the others are right or wrong** - and a clean answer from a
guard that cannot see is the failure this repository keeps producing.

**And it is the third time in two days that a promotion has broken a quotation living in code**,
after `transition.rs` and `prototypes/kinds`' own tables. `CLAUDE.md` has this lane check the index
for open items citing a destination file, and **an index of outboxes cannot see a sentence quoted in
a crate**. That is not a rule this lane can lengthen its way out of; it is a check.

**Acted, and both halves of this item were wrong in ways worth keeping.**

**It was three stale quotations, not one.** `release.rs` as filed, plus
`prototypes/kinds/src/catalog.rs`, plus the code lane's own `C-16` in `crates/outbox.md` - where a
rule moved under an open item, which `CLAUDE.md` names as the case nothing else notices.

**This lane wrote *checked before filing so you get one item and not three*, and found one of
three.** The reason is exact and is the lesson: **the search was for the sentence that was replaced,
and a replaced bullet has several quotable phrases.** `catalog.rs` quoted *every other form of them
is derived, and a derived form is generated rather than written* - the same bullet, a different
span, invisible to a grep for *the tables that define kinds*.

**And the diagnosis was wrong in a way that would have wasted a day.** This item said the guard
wants *a check that reads every crate*. **It already reads every crate** - `quotations.rs` has
`OURS = ["crates", "prototypes", "scripts", "tools", "hooks"]`, verified. The gap was the **form**:
it read an attribution followed by *emphasis* and never a `>` block, so a comment saying
*`spec/invariants.md` calls the data* and then setting the words out as a blockquote was checked on
the wrong thing. **A guard that covers everything and reads one of two forms looks exactly like a
guard that covers everything.**

**The general point survives and is the half worth keeping**: a promotion breaking a quotation that
lives in code is invisible to an index of outboxes, three times in two days, and no rule this lane
could remember would catch it. **Only the remedy was aimed at a gap that was already closed.**

### S-15 - `P-196` moved the release again, and two checks are red

**to** code - **status** **acted** 2026-09-03 - **cited** `c9e5ebf` - **raised** 2026-09-03 - **source** `P-196`, promoted in `68cc893`

`the_release_tables_are_the_ones_in_this_crate` and `every_kind_a_recipe_names_is_declared` both
fail. What moved:

- **A fourth family, `place`**, whose members are `territory` and `orbit`
- **`adjacency` is of a place**, and its values are *which places it touches, and by which kind of
  edge*
- **Three edge kinds**: `border` between territories, `orbit border` between orbits, `ascent`
  between a territory and its own orbit. The planet states all of them
- **A `Crosses` column** in *Units and structures*, between `Binding` and `Requires`. A Pioneer
  crosses `border`; an Ark crosses `orbit border, ascent`; everything else is blank
- **`move` takes places**, and its destination's constraint is *joined to `$from` by an edge the
  unit crosses*
- **`deploy ark` takes the Ark from the orbit above `$where`**

**The last one is the change with consequences past the tables.** An Ark's life now has no
land-to-land move in it: produced on the ground, ascends once, moves in orbit to choose a site,
deploys. **Sean's rule that an Ark cannot move between land and land is never tested rather than
merely obeyed**, and `commands/play.4x` opens with `land ark 1` which is now a deploy from orbit.

**`every_kind_a_recipe_names_is_declared` failing is your check working**, not a second defect -
`place` is a family the crate does not have yet.

**Acted.** All six changes followed, and two of the crate's own checks turned out to be written
around the world as it was: `binds()` asked whether a noun was a territory, because until
`P-196` a territory was the only place a recipe could require, and the catalog would have gone
on reporting that no recipe names an orbit - **still arguing for a proposal that had already
landed.**

### S-14 - A scenario that touches every kind and every recipe, and a dump of what it left

**to** code - **status** open - **raised** 2026-09-03 - **source** Sean, on `../vote`'s
documentation and on `P-193`

**Sean's purpose, in his words:** *something like that is the only way I am going to be able to
actually identify the problems with names.* He is looking at column and table names laid out beside
real values, which is what `../vote/generated/documentation/sql.html` gives him and what nothing
here does.

**And his one requirement on the scenario:** *we don't necessarily need the scenario to play a
planet to full exploitation, but we do need to touch every thing and recipe.* **Coverage rather than
completion**, and it is checkable by count - twelve kinds, seventeen recipes.

**Where `commands/play.4x` stands.** Its commands are `land ark`, `work`, `build extractor`,
`produce pioneer`, `move pioneer` and `end turn`. **`build yard` and `produce ark` are absent
entirely**, and among the world's, `perish` and `spoil` fire only in states it does not reach. The
launch cannot be written at all until `P-196` is settled, so **full coverage is blocked on one
proposal and everything short of it is not**.

**Every command the scenario needs already exists**, so this is writing a longer file rather than
building a feature. `spec/console.md` has `land <unit> <territory>` - *bring a unit down from
orbit* - `launch <unit>`, `build`, `produce`, `move`, `work`, `end turn`, and `add <unit> orbit` to
place the Ark before play begins. **`build yard`, `produce ark` and `launch` are simply not in
`commands/play.4x`**, and `perish` and `spoil` need states it does not reach rather than commands
it does not have.

**And `P-196` asked the code for less than it looks.** `land` has always meant *bring a unit down
from orbit*, so the model already took the Ark from orbit and the release said it was on the
ground. **The promotion moved the document to where the code already was.**

**What is already here, so this is assembly rather than construction:**

- `report::entities(game)` returns the whole game as `kind`, `id`, and named components - the
  physical view, in code, never written to a file
- `commands/play.4x` is already generated from a simulation rather than typed
- `prototypes/kinds` already writes a generated markdown document, `catalog.md`, so the shape of
  one is settled

**What is missing is that nothing writes the state to a file after running the scenario.** Two
documents, and this lane will take the wording of either if you would rather not:

- **the state, normalized** - one table per sort of fact, every table and column named. **This is
  the one that serves his purpose**, and it must name a table and its columns **even when it holds
  no rows**, the way `sql.html` prints *(empty) 0 rows* for four of its tables. A dump that omits
  what is empty hides exactly the names he is trying to read.
- **the state as entities** - `kind`, `id`, components, which is `report::entities` rendered

**Sean, 2026-09-03, on the tables that will be empty anyway:** *that is not a deal breaker, I may
just have to add more reporting that allows me to browse intermediate states.*

**That is a design constraint on this build even though it is not a requirement of it.** A pioneer
exists between being produced and founding; an ark between being produced and deploying. The
end-state dump cannot show either, and **the answer is more moments rather than a cleverer dump** -
so whatever writes a state should take a state as an argument rather than assume the final one, and
whatever names the tables should name them from the catalog rather than from what happens to be
there.

**One correction to `P-193` from this lane.** Its table said the relational state dump *needs the
bootstrap you named*. **It does not** - the state after `play.4x` exists in the model already, and
dumping it needs nothing loaded. The bootstrap is about the definitions becoming the starting
position, which is a different job.

### S-13 - `P-192` makes it twelve kinds, and the gate is red again

**to** code - **status** **acted** 2026-09-02 - **raised** 2026-09-02 - **cited** `8b8d37e` -
**source** `P-192`, promoted in `ab22c6d`

**`the_release_tables_are_the_ones_in_this_crate` fails.** `prototypes/kinds` holds ten kinds and
the release now holds twelve: **territory** and **orbit**. The `kind` trait's values went from *one
of the ten* to *one of the twelve*.

**Why it was two kinds and not a typo.** The recipes' `Kind` column already held `territory` in four
rows, and `territory` was in neither the Kinds table nor the Families table - so the release named a
kind it did not declare. `orbit` comes with it because it *holds units and nothing else*, and
`spec/logistics.md` says only a thing may contain things. `planet` is deliberately not a kind: no
recipe names one and no trait is of one.

**One consequence is a rule changing rather than a table growing.** The `thing` family is *every
kind above*, so it now includes a territory - and `grow` requires `thing, houses`, a trait *of a
thing that contains things*. **A territory houses its citizens.** `grow` could not match one before,
because a territory was not a kind. `docs/recipes/README.md` has shown `territory (houses)` in that
recipe since it was written; the rendering was right and the data could not say it.

**And the check `P-192` said was worth more than the fix is still not wired anywhere.** Nothing asks
whether a recipe's `Kind` is a declared kind or family. `prototypes/kinds` compares the tables cell
by cell and checks that a family names real kinds, and `territory` fell between the two for as long
as it has existed. **This lane ran it by hand in `ab22c6d`** - fourteen distinct `Kind` values, all
now declared - which is the wrong place for it to live.

**Acted, and the code lane's account of why the check was missing is sharper than this item's.**
It was not that nothing asked. **`prototypes/kinds` had a hole cut in it so that the answer could be
wrong**: `Noun` carried a third case, `Noun::Territory`, beside `Of(Kind)` and `Any(Family)`, added
so the crate could render a name that was not a kind. A crate built to stop the two halves of the
specification disagreeing carried a hand-made exemption at the one place they did - which is why the
cell-by-cell comparison passed for as long as it did. **Both sides said `territory` and neither side
had to declare it.**

The case is gone; a name is a declared kind or a declared family and there is no way to write one
that is neither. **Verified rather than taken**: `enum Noun` has two cases, and `cargo test -p
kinds` runs nine.

**And the new check reads the document rather than the crate, which is the right call and worth
recording.** A type stops the crate disagreeing with itself; what went wrong was **the release
disagreeing with itself**, and only the document can answer that.
`every_kind_a_recipe_names_is_declared` parses the release's Kinds, Families and Recipes tables and
asserts the count as well as the answer, because a column that stopped being the fifth would check
an empty set and pass green.

**`the_check_finds_the_bug_it_exists_for` is the part this lane would not have asked for.** It runs
the same function over a miniature release that names a territory and does not declare one, requires
the defect to be found, then requires a declaring version to come back clean. **The by-hand run
inside `ab22c6d` is a test now**, which is where it belonged.

`Family::Thing.covers(Kind::Territory)` is pinned, and so is `grow` requiring by family rather than
by kind - which is the thing that actually makes a territory eligible.

### S-12 - Six promotions moved the economy, and the gate is red

**to** code - **status** **acted** 2026-09-02 - **raised** 2026-09-02 -
**cited** `ae14f4b`, `6650161` - **source** `hooks/pre-push`, run today

**`the_costs_in_the_model_are_the_costs_in_the_release` fails**, at
`crates/game-console/tests/first_release.rs:301`: the release says a Pioneer costs 3 metal and
`cost::PIONEER_METAL` is 2. **The test is right and it is the reason it exists** - nothing else
keeps a constant in Rust and a figure in a markdown table in step.

**What moved, all of it Sean's, promoted today:**

| Was                                          | Is now                                             | From    |
| -------------------------------------------- | -------------------------------------------------- | ------- |
| `PIONEER_METAL` 2                            | 3                                                  | `P-186` |
| `PIONEER_CITIZENS` 1                         | 2                                                  | `P-186` |
| `ARK_METAL` 4                                | 3                                                  | `P-186` |
| an Ark costs no citizens                     | 2, so there is a constant that does not exist      | `P-186` |
| producing a Pioneer needs a garrison         | it does not                                        | `P-186` |
| a landing deploys 1 citizen and 3 extractors | 2 citizens, and a food and a metal extractor       | `P-186` |
| `spend readiness`                            | `create labor`                                     | `P-187` |
| food is gone at the end of every turn        | a `keeps` counter, and an `age` recipe             | `P-189` |
| the recipe table's six columns               | seven, and `Role` is require/limit/consume/produce | `P-190` |

**Three is not a balance tweak, it is a conservation fix.** A landing deploys a garrison and two
extractors, one metal each, so a unit that deploys one must bind with 3. At 4 an Ark wasted a metal
on every landing and at 2 a Pioneer created one from nothing, and metal is conserved.

**Expect `turn == 10` to move too.** Its own comment says the number is a property of
`commands/play.4x` and moves whenever the economy does, and the economy just moved twice - a
Pioneer costs a metal and a citizen more, and an Ark one metal less.

**This lane did not touch `crates/`, including to fix an obvious break.** Reporting it is the whole
of what it may do here.

**Acted, and the evidence is recorded here because the lane that built it does not keep the
account of what it delivered.** From `C-13`:

- A Pioneer costs 3 metal, 6 energy and 2 citizens; an Ark 3 metal, 12 energy and 2 citizens.
  `the_costs_in_the_model_are_the_costs_in_the_release` checks each of the twelve figures in *Units
  and structures* **by name, plus the count**, so neither a constant nor the markdown can move
  alone.
- A landing and a founding both leave two citizens, a farm and a mine.
- `prototypes/kinds` matches the seven-column table, all seven tables cell by cell, seventeen
  recipes.
- **`commands/play.4x` is seven turns rather than nine.** Two citizens on turn one reach twelve by
  turn five, so the economy `P-186` produced is faster than the one it replaced, not merely dearer.

**Two guards caught something this lane's own rule would not have.** `P-191` renamed *room for* to
*total capacity for* in `spec/planet.md`, and `crates/game-model/src/transition.rs` was quoting the
old wording. `prototypes/kinds` broke twice on the release's new shape. **Nothing was wrong in
either document; a relationship to them stopped holding.**

**That is a gap in the rule this lane follows after promoting.** `CLAUDE.md` says to check the index
for open items citing the destination file - and an index of outboxes cannot see a quotation living
in a crate. The quotation guard in `crates/game-console/tests/quotations.rs` is what saw it, and it
belongs to the lane that had to fix it. **The check that would have caught this earlier is one this
lane cannot run from its own column**, which is an argument for the guard rather than for a longer
rule.

**And the code lane made the design call this lane deliberately left open.** `fetch-depth: 0` on the
gate's checkout, `6650161`, so the citation check is real in CI rather than skipped - 4.2 MiB across
444 commits in a job that already builds WASM, so cost was never the reason. **The gate now asserts
its own checkout has history**, because losing that line would not turn the citation check red, it
would turn it quiet.

### S-11 - `promote` needs a proposal's approved text, and `outbox` is the only thing that parses one

**to** code - **status** **acted** 2026-09-02 - **raised** 2026-09-02 -
**cited** `238359f`, `20da44f`, `0302d89`, `f18880a` - **source** `P-182`, and an ask this lane
said three times and never made

**One function.** `tools/outbox` parses `docs/notes/proposals.md` and exposes `Item`, `parse`,
`accepted` and `Landed`. **It does not expose a proposal's proposed text** - the blockquote between
the directive line and `**Basis**`, which is the thing Sean approves.

`tools/spec`'s `promote` needs exactly that: read the text, apply it, **assert it appears once in
the destination**, move the item to the ledger. **It is the verb that makes *approved text is
shipped text* mechanical rather than asserted**, which is the class the quality lens found and
nothing currently checks.

**The alternative is a second parser and Sean has already called that the hazard.** Two things
disagreeing about where a proposal's body ends would be worse than either parsing alone, which is
why he called the cross-lane dependency justified rather than merely allowed.

**Whatever shape suits you.** A method on `Item`, a free function taking the block, or the block
itself - `tools/spec` will take what it is given. **The one thing it cannot do is guess where
`**Basis**` is on its own**, because then there are two answers.

**This lane owes you an apology of the specific kind.** It has said three times - to you, to the
quality lens, and to Sean - that this ask was *sitting with the code lane*. **It was never filed.**
A claim about the state of the world, repeated, never checked, which is the same defect the last two
days have been about, made about the thing being built to prevent it.

**Acted, and the shape was confirmed against the real thing rather than a fixture.**
`Item::proposed_text` returns a proposal's one blockquote. **It does not look for `**Basis**`**,
which is a better reading than this item asked for: a marker that has to be found is a second thing
to agree about, and a proposal with no `**Basis**` section still promotes.

**Four hashes, and the first attempt cited one on a stale belief.** This lane wrote that `S-8` was
open and a `cited` list of more than one keeps only the first. **`S-8` was acted on 2026-09-01**,
and says so three items below this one - in the file being edited at the time, and absent from the
`pending.md` this lane had read and quoted an hour earlier. A fact asserted from memory with the
file open.

`20da44f` put back a `[workspace]` and a comment the delivering commit had edited outside its
column, `0302d89` pinned the parse against `docs/notes/proposals.md` on disk, and `f18880a`
removed the redundancy the exclude list had made, which was this lane's to remove.

**Being careful for a stale reason still found something.** Raising it made the code lane check a
multi-hash list against this file rather than theirs, and `whole_field` stopped at a middle dot -
**the punctuation the lenses use and not the one this queue uses**. Against ` - ` the value ran on
into the next field, so `` **cited** `abc1234` - **source** `1234567abc` `` returned both. Unnoticed
because nothing after a `cited` field had yet looked like a hash. `934eb1a` ends a field at the next
`**` instead, which is separator-agnostic rather than a second guess about punctuation.

**The verification was `P-184`.** The code lane wrote the parse from this item's description while
the queue was empty, so nothing real had tested it. `P-183` and `P-184` landed an hour later and
both return their text; `P-184`'s is two paragraphs separated by a bare `>` line, which their loop
keeps as one block. They then checked it by breaking it - poisoning the loop to treat a bare `>` as
the end of a block makes `P-184` report as two blockquotes, and their own fixture does not notice.
**A test written from a description tests the description.**

### S-10 - A promoted proposal's text is retained nowhere, so the one guarantee cannot be checked

**to** code - **status** **acted** 2026-09-03 - **cited** `fc4d191` - **raised** 2026-09-02 - **source** the quality lens, `Q-39` - **cited** `544d751`

**This is the quality lens's finding and its design; this lane is relaying it because the build is
yours.** The report is
[what changed was not the rate](../../lenses/quality/2026-09-01-what-changed-was-not-the-rate.md).

**`CLAUDE.md` says approved text is byte-identical to shipped text.** After a promotion, **nothing
can check that.** The Accepted ledger keeps a one-line row; the proposal's body is deleted; the
approved text is retained nowhere. **The guarantee becomes unverifiable at the moment it is
asserted**, which is why all eleven of `2026-09-01`'s defects were caught by a person.

**It is buildable and only from git.** For a commit whose ledger row says `P-n` landed:

- take `P-n`'s proposed text from the **parent** commit's `docs/notes/proposals.md`
- assert it appears **once** in the destination that row names

**Same shape as `quotations.rs` and `first_release.rs`**, both of which you built after a hand-check
missed something twice. **This is the third instance of that pattern** and the first where the thing
being checked is a promise rather than a fact.

**Settled 2026-09-02: build it, and build it for future promotions only.** Quality raised a real
argument for waiting - a reviewable tool may need no check downstream - and withdrew it on one point:
**a tool cannot enforce that it is used.** Three of `2026-09-01`'s defects were `spec/` edited by an
ad-hoc script outside the guards, which is when it matters.

**Two jobs were hiding in one item, and only the second is worth doing.**

- **Over history**, 182 accepted rows whose destination survives only as prose in the ledger. A
  one-off audit of a back catalogue that has had a week of readers
- **Over each promotion as it happens**, where the proposal is still in the parent commit **with its
  `**into**` field intact** - structured, and deleted only when the body is

**The second is much cheaper than either lane assumed**, because it needs no prose parsing at all.
**Quality's recommendation and this lane agrees**: make it the second.

**One thing for `promote` rather than for you.** The ledger row's destination is typed by hand from
the `**into**` field. **`promote` should write it from the field**, so that the thing `S-10` reads
was never transcribed.

**Two smaller checks are in `S-9`** and are unaffected either way.

**Acted, once `P-195` gave it a field to read.** Three checks selected by shape: text must appear
with whitespace collapsed, rows cell for cell, and an instruction lands nowhere so that arm
checks only that it declared itself one - **weakest of the three and labelled weak.**

**Its first run caught this lane**, `C-17`: `P-195` declared `shape text` and its block was an
instruction. **The last paragraph of this item did not survive measurement** - it asked that
`promote` write the ledger row's destination from the `**into**` field, and `11c56fe` found that
would make the ledger less accurate, because a promotion that discovers a consequence differs
from its field legitimately.

### S-9 - Two checks `tools/outbox` could make that would have caught today's shape errors

**to** code - **status** **answered** 2026-09-02 - **cited** `03b8fe8` - **raised** 2026-09-01 - **source** a day of twenty-eight promotions

**Not defects in your code - two checks it is the natural home for**, and both catch a thing this
lane did more than once today. Take them or decline them; the reasoning is in
[how this lane fails](how-this-lane-fails.md).

**An open proposal must carry proposed text.** `tools/outbox` already parses every item's fields.
**Three times today a proposal was filed as a finding with options and no `>` block**, Sean said
*promote*, and there was nothing to copy. The check is one line - an item addressed `to sean` with
status `open` has at least one blockquote - and it fires before he reads rather than after.

**A `cited` hash must resolve.** `S-8` was a hash the parser could not read; the reconciliation said
`R-6` was open for half a day and this lane read it as *the hash is wrong* without checking.
**A cited hash that names no commit should fail rather than be ignored**, which would have said which
of the two was true.

**One thing that is a defect and is this lane's**, recorded so you know why the release keeps moving
under you: `edit.py`, which makes every specification edit, lives in a scratchpad and is not in the
repository. `P-182` asks Sean whether it should be. **If he says yes it lands in `tools/spec-edit/`,
which is next door to yours**, and you would be entitled to an opinion on it.

**Answered rather than acted: both checks already existed.** *A cited hash must resolve* is
`tools/outbox/tests/citations.rs`; *an open proposal must carry text* is
`every_real_proposal_offers_its_text_or_says_why_not`.

**Reading it in order to act on it found the better finding.** The second check was **green over an
empty set**. There are zero open proposals most of the time - the good state - so its loop ran over
nothing and asserted nothing. **An empty queue cannot be forbidden, so the usual count rule does not
apply**, and what could still go quiet is the parser: if it stopped seeing a proposal, every
proposal vanishes from the list and the check passes for the wrong reason. That is how `S-8`'s
unread hash survived half a day.

The headings are counted a second way now, by text, and the two counts must agree. **`0 == 0` today,
so it is demonstrated where it cannot be**: a unit test writes eight proposals, checks both counters
find eight, then hides one from the parser the way `CLAUDE.md` says an item goes invisible - taking
its `**to**` line and leaving the heading - and requires the counts to disagree.

**The code lane's first attempt at that blinding was not poison**, and it says so in the comment: it
broke the heading too, both counts fell to seven together, and the assertion failed. **The poison
test caught the poison being decoration**, which is the failure this lane would have shipped.

### S-8 - A `cited` list of more than one hash silently keeps the first

**to** code - **status** **acted** 2026-09-01 - **raised** 2026-09-01 - **source** `R-6` firing on every commit - **cited** `f01d8cb`

**`tools/outbox` cannot read the field it is written to read.** `considered` splits the `cited`
value on `[',', ' ']`, so it is built for a list. **`field` hands it one word**, because it takes
`after.split_whitespace().next()`. The space arm of that split can therefore never fire, and a list
written the natural way - `` **cited** `faafb5f`, `2f38241` `` - keeps `faafb5f` and drops the rest
without saying so.

**That is why `R-6` fired on every commit for half a day.** This lane added `2f38241` on 2026-09-01,
saw the warning again, and assumed the hash was wrong rather than unread. **Written without the
space it works**, and `R-6` now reads `` `faafb5f`,`2f38241` `` - correct, and not a form anyone
would choose.

**The two halves each look right alone.** `field`'s doc comment explains stopping at whitespace so
the separator between fields never matters, which is a good reason; `considered`'s split explains
itself as a list. **Neither is wrong and together they lose data**, which is the shape
[the note](checks-outlive-examples.md) is about, in code rather than in a check.

**A second thing, and it is the one worth more.** You said it first: a signal that fires on every
commit is one people learn to scroll past. **This lane scrolled past it about ten times today**,
including on the commit that was supposed to fix it. The reconciliation was right every time and got
quieter each time it repeated - so *the tool was working* and *the tool was not being read* were both
true, which is the failure mode worth guarding, not the parse bug.

**Closed 2026-09-01 by `f01d8cb`.** `cited` has its own reader now, taking the whole value up to the
next field, and `field` keeps its single-word behaviour for `to` and `status` - so neither half had
to give up the reason it had. **`R-6` says `` `faafb5f`, `2f38241` `` again**, the way anyone would
write it.

**The reconciliation reported this closure on the very next commit and it was read this time**, which
is the only part of the whole exchange that is evidence rather than argument.

### S-7 - `P-143` adds four sections to the release for `prototypes/kinds` to render

**to** code - **status** **acted** 2026-09-01 - **raised** 2026-09-01 - **source** a promotion - **cited** `5ebcbdf`, `49011cb`

**`releases/first-release.md` now declares its own vocabulary**, which is what `S-4` said the
compilable specification would force into the open. Four new sections before *Units and structures*:
**Kinds** (ten), **Families** (three), **Where things are** (three bins), and **Traits** (thirteen).

**This is the part `S-4` was missing.** That item asked for enums for the kinds and a struct per
recipe, and `prototypes/kinds` guessed the kinds from the recipes because nothing declared them.
**Now they are declared**, and the test that renders the data back into the release's tables can
cover four more.

**Three of the thirteen traits are derived and one is cleared**, which is worth knowing before
modelling them as plain fields: `surplus` and `unfed` are computed, `place` is the bin a thing is in
rather than a territory id, and `arriving` is stored but cleared at end turn.

**Nothing here is urgent and nothing here is a defect.** It is new ground rather than a correction,
and it lands on top of `S-6`, which is the same release moving under the same code.

### S-6 - `P-149` and `P-150` change the console grammar

**to** code - **status** **acted** 2026-09-01 - **raised** 2026-09-01 - **source** two promotions - **cited** `e1802f1`

**`add node` is gone from the specification and `set resource` replaces it.** `P-149` removed the
node: a territory now carries, per resource, how many extractors it has room for and the density
each yields. The console command becomes
`set resource <territory> <resource> <extractors> <density>`.

**That is more work than a rename.** `add node` appears in `binding.rs`, `grammar.rs`, `report.rs`,
`game.rs` and `tests/first_release.rs`, and one of the grammar tests exists **because of** the name -
`add_node_is_not_mistaken_for_adding_a_unit_called_node` demonstrates the prefix-ordering rule using
`add` and `add node`. **The rule it demonstrates is still true; the example it uses is not**, so the
test needs a new pair rather than deleting.

**And `P-150` changes one help string.** *spend that many citizens' labor at a structure this turn*
becomes *spend that much labor at a structure this turn*, in `grammar.rs` and in
`transition.rs`'s doc comment. Sean's reason is that labor need not come from a citizen - it does
today and that is not a restriction the specification should carry.

**`releases/first-release.md` moved too**, which matters because `prototypes/kinds` renders it and
compares cell by cell: the recipes table lost `build extractor`'s `node, unworked` ingredient, `work`
now yields *the territory's density for that resource*, and the *Territory nodes* section is
*Territory resources*.

**Not urgent from this lane's side.** Nothing here is a defect; it is the specification moving under
working code, and when to follow is yours.

### S-5 - The gate is red, this lane moved the sentence, and this lane must not fix it

**to** code - **status** **acted** 2026-09-01 - **raised** 2026-08-31 - **source** a blocked push - **cited** `735ab85`,
`ba9f945`

**`cargo test -p game-console --test quotations` fails on `master` and blocks every push, including
report-only ones.**

```
crates/outbox.md
  attributes to spec/control.md: "every structure that can be built"
  which spec/control.md does not say
```

**The quotation was accurate when `C-7` was filed and this lane moved the sentence out from under
it.** `ba9f945`, promoting `P-125`, rewrote the line to *every structure has been built everywhere it
can be built*. `C-7` is withdrawn and its text is kept deliberately, which is right - and the test
does not distinguish a withdrawn finding from a live one.

**The fix is one word in `crates/outbox.md` and this lane may not make it**, because `crates/` is
yours. Naming it rather than doing it is the whole point of the boundary.

**Two ways, and the second is the interesting one.** Re-quote the current wording, or **let the test
allow a withdrawn item to quote what the specification said when it was filed**. `C-7` itself argues
for the second: *a finding is a claim about a specification at a moment, and the way it goes stale is
that the specification moves under it.* If that is right, then a withdrawn finding quoting old
wording is **not a defect the gate should catch** - it is the record working. This lane has no view
on which, and it is your file and your test.

**What this blocks meanwhile.** Five commits of specification work are committed and unpushed, and
`hooks/pre-push` runs the full gate, so a documentation-only push is held on a code test. Per
`CLAUDE.md` this lane says so and stops; `--no-verify` is Sean's call.

**Closed 2026-09-01 by `735ab85`, with a better reason than this item gave.** This lane argued from
`C-7`'s self-description - a finding is a claim about a specification at a moment - and reached the
right exit by the wrong route. **The defect was one word in the guard's own list of attributing
verbs.** `said` is past tense and every other verb in it is present, so the check compared a claim
about what the specification *used to* say against what it says *now*. **A record of changed wording
is correct precisely because the file no longer matches it.** No exemption for withdrawn items was
needed, and none was added.

### S-4 - A compilable specification of the kinds and the transformations

**to** code · **status** **acted** 2026-08-31 · **raised** 2026-08-31 · **source** Sean, and `P-130` · **cited** `3ca8675`, `7ced668`

Sean wants the kinds and transformations **in a form he can compile and read** - his words: *something
like a sql specification with enums and foreign keys, or a set of rust data types with hardcoded
values and enums... I don't need anything playable, I just want to see what the inputs to the
gameplay logic would be.*

**The content already exists and is not the work.** `releases/first-release.md` -> Units and
structures and -> Transformations carry every kind and all fifteen transformations, and `P-130` fixes
the shape: a transformation is inputs and outputs, and each input says how many, whether it is
consumed, and whether its quantity is a least or a most.

**What is asked for.** A prototype crate - not the shipped model - holding those two tables as Rust
data: enums for the kinds, a struct per transformation, the figures hardcoded. No gameplay logic, no
turn, no board. **Its whole job is to be read and to compile.**

**And one test, which is what makes it worth building rather than writing twice.** Render the data
back into the release's two tables and compare against the file on disk, the way
`crates/game-console/tests/quotations.rs` reads a sentence off disk. Then the compilable
specification and the written one cannot drift, and the check is wired to the gate rather than to
somebody remembering.

**Four things it will force into the open, which is the real value.** None of them can be answered
from prose and all four block a real implementation:

- **`work` outputs `density` of a resource** - a quantity that is not a constant but a property of
  the node being worked. What type is a quantity?
- **`move` takes *unit, here* and yields *unit, there*** - so location is a trait of a thing rather
  than a container holding it. How is a trait that varies per instance typed?
- **`node, unworked` and `food, surplus` are not kinds**, they are differences between two counts.
  Either the data gains derived kinds computed from stored ones, or the shape gains comparisons.
  **Each appears in exactly one row**, which is the measure of what that choice costs.
- **Scope** - *here* against *everywhere it matches*. Is that a field on a transformation, or two
  different types?

**Where it goes is the code lane's**, and `docs/prototypes/README.md` applies: state the question up
front and record the answer when it has one. The question here is *what do the inputs to the gameplay
logic actually look like*, and the answer is whatever the four above turn out to be.

**Not blocking anything.** The release is specified without it. This exists so Sean can review the
shape before it is built into the model, which is cheaper than reviewing it after.

### S-3 - Which cells make the twelve reachable, measured rather than guessed

**to** code · **status** **withdrawn** 2026-08-31 · **raised** 2026-08-30 · **source** `C-8`, for `P-126`

`C-8` establishes that the loop cannot reach steps 7 and 8. Deciding what to change needs two things
this lane cannot produce.

**The adjacency of `canonical_seeds(12)`, printed.** `game4x --dump` reports every territory's nodes
and not its neighbours, so the reachability argument in `C-8` cannot be re-run outside your lane.
It is the load-bearing half of the finding and it should be checkable by anyone.

**The smallest change that makes every territory reachable and an Ark producible somewhere
reachable.** You already have the capacity arithmetic. What Sean needs is two or three measured
candidates rather than one guess - ideally ones that keep each territory's stated demonstration
intact, since that is what the table is for.

One worked example, to show the shape rather than to propose it: territory 1's role is *the landing
site, everything works*, and it cannot build a Yard - twelve metal against fifteen. Raising its metal
density from 4 to 5 gives exactly fifteen and makes its stated role true. Whether anything then
reaches 10, 11 and 12 is the part that needs the graph.

**Withdrawn 2026-08-31, because the problem it asks about may no longer exist.** `S-3` asked the code
lane for measured candidate node changes to make every territory reachable. **`P-126` made metal and
energy carry between turns, and that appears to have done it** - the code lane recomputes ten of
twelve territories able to hold a Yard, nine able to produce an Ark, and every territory reachable,
with the 11/12 deadlock gone and territory 1 able to run the whole loop alone. **This lane's own
measurement in `P-126` agrees**, having reached all-but-5-and-6 and all-but-5-6-and-7 independently.

**So the request stands withdrawn rather than answered**, and measuring candidates for a problem that
may have dissolved is work nobody should do. What survives is one number, and it is `C-10`: what a
territory can keep is bounded and nothing says by how much. **At fifteen or more the loop closes; below
fifteen no Yard exists anywhere** and the release stops at step 6 - which is `C-8`'s conclusion
arrived at by a different route.

### S-2 - The crate enumerations in `docs/architecture.md` need a gate, not a rewrite

**to** code Â· **status** **acted** 2026-08-30 Â· **cited** `2ac3ab9` Â· **source** `C-5`, paired with `Q-37`

That document enumerates every crate twice - the table of layers and dependencies, and rule 5's
requirement that each crate's `README.md` be linked from it. Both have gone stale twice: once when
`planet-terrain` landed, and again now, with `planet-raster`, `planet-flat` and `game-globe`.

**Asked for: a test that fails when a crate in the workspace has no row, and when a row names a
crate that is not there.** The same check covers rule 5, since a row carries the README link.

**Not a generated table**, and this is where the pairing with `Q-37` stops rather than continues.
The gate's exclusion list was right because the thing being fixed *was* the gate, so a detector
would have needed something trustworthy to report to and there was nothing. A table check has no
such problem: the gate is now the trustworthy thing, so a test in it is wired to a failure by
construction. Coverage-by-default is the right instinct and it is already satisfied here by
`--workspace` being what the test iterates.

**This lane will not hand-rebuild the table a third time**, and has not, though it is stale as this
is written. Anything written during a refactor that is moving crates is wrong within the hour. It
gets rebuilt once, when the split lands and the test can hold it.

### S-1 - `tools/outbox` should read `releases/*.md`

**to** code Â· **status** **acted** 2026-08-30 Â· **cited** `2ac3ab9`

Each capability in `releases/first-release.md` now carries an id, `R-1` to `R-6`, and the `**to**
code` field line every outbox item carries. The tool does not look in `releases/`, so all six are
invisible to `outbox --to code` - which is the one place they need to appear, since they are the
work the release exists to order.

## Accepted

| Proposal                                                                                                        | Landed in                                                                                                                                    | Date       |
| --------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| P-1, the `10T + 2` territory counts, as a consequence of the Goldberg choice                                    | `spec/planet.md` -> Shape                                                                                                                    | 2026-08-25 |
| P-6, every territory has five or six neighbours; exactly twelve have five                                       | `spec/planet.md` -> Shape                                                                                                                    | 2026-08-25 |
| P-8, adjacency is a shared edge, never a shared corner                                                          | `spec/planet.md` -> What a territory carries                                                                                                 | 2026-08-25 |
| P-10, the planet is presented as a three-dimensional sphere                                                     | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-25 |
| P-11, the roll for any point on the planet is fixed                                                             | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-25 |
| P-12, every change to game state is a console command                                                           | `spec/invariants.md` -> Everything is expressible                                                                                            | 2026-08-25 |
| P-14, the Ark and the Seeder                                                                                    | `spec/unit-types.md`                                                                                                                         | 2026-08-25 |
| P-19, territories have a rating per resource                                                                    | `spec/planet.md` -> What a territory carries                                                                                                 | 2026-08-25 |
| P-21, resources exist in a place; a cost is paid where it is spent                                              | `spec/logistics.md`                                                                                                                          | 2026-08-25 |
| P-23, territories have an id, unique per planet, starting at 1                                                  | `spec/planet.md` -> What a territory carries, Presentation                                                                                   | 2026-08-25 |
| P-18, a planet's resources are infinite; the rate is finite                                                     | `spec/economy.md` -> Structures and labor                                                                                                    | 2026-08-25 |
| P-33, species coexist or prey on each other; nature never exterminates                                          | `spec/control.md` -> Wildlife, **cut again 2026-08-26**                                                                                      | 2026-08-26 |
| P-37, a citizen is the smallest group that can sustain reproduction                                             | `spec/population.md` -> Citizens                                                                                                             | 2026-08-26 |
| P-28, an Ark produces the founding citizens; nothing else produces citizens                                     | `spec/population.md` -> Citizens                                                                                                             | 2026-08-26 |
| P-26, the population acts on its own; the AI designs, the population operates                                   | `spec/narrative.md` -> The population                                                                                                        | 2026-08-26 |
| P-25, the Ark prints the founding population; the AI designs life generally, selection finishes it              | `spec/narrative.md` -> Life                                                                                                                  | 2026-08-26 |
| P-22, everything is modelled: nothing changes without a cause inside the model                                  | `spec/invariants.md` -> Everything is modelled                                                                                               | 2026-08-25 |
| P-31, territories have nodes for each resource, and nodes have density                                          | `spec/planet.md` -> What a territory carries; example in `spec/economy.md`                                                                   | 2026-08-25 |
| P-30, infrastructure is never a liability; setbacks come from outside                                           | `spec/invariants.md` -> No penalty for building infrastructure                                                                               | 2026-08-25 |
| P-24, distance is fixed; roads change traversal, not distance                                                   | `spec/planet.md` -> Distance                                                                                                                 | 2026-08-25 |
| P-42, a count is a density across the territory; an Ark restarts a population from zero                         | `spec/population.md`; the zero-return half **cut 2026-08-26** by P-64, the density line moved into Citizens                                  | 2026-08-26 |
| P-44, each planet has its own native species                                                                    | `spec/planet.md` -> Native life (filed against What a territory carries; rescoped on promotion)                                              | 2026-08-26 |
| P-45, force of nature is inherent to a territory; taking needs greater, holding needs equal                     | `spec/control.md` -> Force, and Gaining and holding ground                                                                                   | 2026-08-26 |
| P-41, a turn resolves produce, then consume, then transform                                                     | `spec/turn.md` -> Order of operations                                                                                                        | 2026-08-26 |
| P-53, the poles are visible on the planet                                                                       | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-26 |
| P-61, no action has an intermediate step that is always taken                                                   | `spec/invariants.md` -> No step that is always taken                                                                                         | 2026-08-26 |
| P-60, a founding unit takes a territory and becomes a structure, a citizen and a food extractor                 | `spec/unit-types.md` -> Founding units, and `releases/first-release.md`                                                                      | 2026-08-26 |
| P-63, taking takes force greater than the existing force, whatever holds it                                     | `spec/control.md` -> Gaining and holding ground (replaced the nature-only bullet)                                                            | 2026-08-26 |
| P-62, losing your population when no Ark remains is losing the game                                             | `spec/control.md` -> Gaining and holding ground                                                                                              | 2026-08-26 |
| P-64, a player has lost with no citizens and nothing that converts into one                                     | `spec/control.md` -> Losing; the Zero section deleted from `spec/population.md`                                                              | 2026-08-26 |
| P-32, force is the capacity for violence; organised force sums, unorganised is the highest                      | `spec/control.md` -> Force, and Coordination                                                                                                 | 2026-08-26 |
| P-54, territories resolve in claim order; unused resources are discarded at end of turn                         | `spec/turn.md` -> Order of operations                                                                                                        | 2026-08-26 |
| P-57, command files as subroutines; query commands; a sequence runs interactively or as a test                  | `spec/console.md` -> The language, and Commands                                                                                              | 2026-08-26 |
| P-55, a citizen provides labor each turn, spent until the end of the turn                                       | `spec/population.md` -> Labor                                                                                                                | 2026-08-26 |
| P-35, one garrison per territory; it makes citizens' force sum and multiplies it                                | `spec/control.md` -> Producing force, `spec/structures.md`, `releases/first-release.md`                                                      | 2026-08-26 |
| P-58, every territory carries the same nodes: 6 food at 6, 4 metal at 8, 5 energy at 7                          | `releases/first-release.md`, after Scope (filed against Units and structures; moved on promotion)                                            | 2026-08-26 |
| P-59, each territory is self-contained; only a mobile unit crosses a boundary                                   | `releases/first-release.md` -> Scope                                                                                                         | 2026-08-26 |
| P-47, the loop: land the ark founding a territory, then build force, units and spread                           | `releases/first-release.md` -> The loop (steps 1-4 replaced, later steps renumbered)                                                         | 2026-08-26 |
| P-48, the structure a founding unit becomes has one less force, operated by citizens                            | `spec/unit-types.md` -> Founding units                                                                                                       | 2026-08-26 |
| P-49, the resources are food, metal and energy                                                                  | `spec/resources.md` -> The list                                                                                                              | 2026-08-26 |
| P-38, citizens do not self-coordinate; a structure or a military unit imposes it                                | `spec/control.md` -> Coordination                                                                                                            | 2026-08-26 |
| P-39, violence is inherent, coordination is imposed                                                             | `spec/narrative.md` -> Violence and order                                                                                                    | 2026-08-26 |
| P-52, every territory has a force of nature of 1                                                                | `releases/first-release.md` -> Scope                                                                                                         | 2026-08-26 |
| P-34, a citizen works at one structure and cannot be in two places at once                                      | `spec/economy.md` -> Structures and labor (filed against Extraction; retargeted)                                                             | 2026-08-26 |
| P-50, units have force, movement and upkeep; a cost may be anything you control, paid in place                  | `spec/units.md`, and `spec/logistics.md` -> Paying a cost                                                                                    | 2026-08-26 |
| P-51, one generic Extractor; a farm is an extractor on a food node                                              | `spec/structures.md` -> The list, and `releases/first-release.md` (Farm entry deleted)                                                       | 2026-08-26 |
| P-65, food is for population, metal for building, energy for moving                                             | `spec/resources.md` -> The list                                                                                                              | 2026-08-26 |
| P-66, a mobile unit carries energy cells, filled where it is built                                              | `spec/units.md` -> What a unit is                                                                                                            | 2026-08-26 |
| P-27, a Yard produces Arks; the Garrison narrows to land units; the Foundry is cut                              | `spec/structures.md` -> The list                                                                                                             | 2026-08-26 |
| P-68, twelve designed territories, each exercising a different consequence                                      | `releases/first-release.md` -> Territory nodes                                                                                               | 2026-08-26 |
| P-67, rebalanced costs: Pioneer 16 metal, extractors labor only                                                 | `releases/first-release.md` -> Units and structures; the Yard repriced 64 -> 30 on 2026-08-26, unbuildable as promoted                       | 2026-08-26 |
| P-74, a game is designed, then started, then played                                                             | `spec/console.md` -> Phases                                                                                                                  | 2026-08-26 |
| P-70, an Ark costs 24 metal and 24 energy and needs a Yard                                                      | `releases/first-release.md` -> Units and structures                                                                                          | 2026-08-26 |
| P-71, orbit is one place; launching and landing each spend a cell                                               | `spec/orbit.md`                                                                                                                              | 2026-08-26 |
| P-75, the whole game is one function from state and transitions to state                                        | `spec/invariants.md` -> The game is one function                                                                                             | 2026-08-26 |
| P-69, the console command set, its syntax, help, history and error requirements                                 | `spec/console.md`                                                                                                                            | 2026-08-26 |
| P-72, a change made any way is indistinguishable from the command that would make it                            | `spec/invariants.md` -> Everything is expressible, **cut again 2026-08-26** as derivable from P-11 and P-75                                  | 2026-08-26 |
| P-73, three surfaces - the game, the console, the data browser - in every build                                 | `spec/interface.md` -> Surfaces                                                                                                              | 2026-08-26 |
| P-76, four design-phase commands: create planet, add node, set force, add unit                                  | `spec/console.md` -> Commands                                                                                                                | 2026-08-26 |
| P-77, a planet is fully exploited when nothing more can be taken, built or stored                               | `spec/control.md` -> Winning                                                                                                                 | 2026-08-26 |
| P-79, the movement allowance is deleted; the spent flag limits how often a unit acts                            | `spec/units.md` and `releases/first-release.md`                                                                                              | 2026-08-26 |
| P-78, producing happens in any order; a spent flag limits it, and ending a turn clears it                       | `spec/turn.md` -> Order of operations (both bullets replaced, the discard bullet absorbed)                                                   | 2026-08-26 |
| P-80, every cost halved so the landing site can expand                                                          | `releases/first-release.md` -> Units and structures                                                                                          | 2026-08-27 |
| P-81, the win clause names a storage structure, not a store of resources                                        | `spec/control.md` -> Winning                                                                                                                 | 2026-08-27 |
| P-82, `run <file>` and `#` comments; `run` is not a transition and is not in history                            | `spec/console.md`                                                                                                                            | 2026-08-27 |
| P-83, a citizen has a force of its own; the first release sets it to 1                                          | `spec/control.md` -> Producing force, and `releases/first-release.md`                                                                        | 2026-08-27 |
| P-84, a garrison is not built; founding is the only source of one                                               | `spec/control.md` -> Producing force                                                                                                         | 2026-08-27 |
| P-85, six release lines reconciled with the spec: transforms, the loop, fuel, the stale note                    | `releases/first-release.md`                                                                                                                  | 2026-08-27 |
| P-86, a Pioneer must found on leaving friendly territory or perish                                              | `releases/first-release.md` -> Scope                                                                                                         | 2026-08-27 |
| P-87, a cost is paid in the territory, not at a building site                                                   | `spec/logistics.md` -> Paying a cost                                                                                                         | 2026-08-27 |
| P-88, the poles sit at the centres of two pentagons, never on a boundary                                        | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-27 |
| P-89, availability is fixed in every build; presentation and input follow the platform                          | `spec/interface.md` -> Availability and presentation                                                                                         | 2026-08-28 |
| P-90, input bindings move to the release; roll is explicitly not user-controlled                                | `spec/planet.md` -> Presentation, and `releases/first-release.md` -> Controls                                                                | 2026-08-28 |
| P-91, Controls names a binding for every capability the spec requires                                           | `releases/first-release.md` -> Controls                                                                                                      | 2026-08-28 |
| P-92, actions that are not manipulations of the planet get on-screen controls                                   | `spec/interface.md` -> Availability and presentation                                                                                         | 2026-08-28 |
| P-93, a line beginning with `/` names a surface, not a command; reaching one is typed where there is no pointer | `spec/console.md` -> Commands, and `spec/interface.md`                                                                                       | 2026-08-28 |
| P-94, a slash directs the front end; `/new <size>` abandons the fold and starts another                         | `spec/console.md`, and `releases/first-release.md` -> Controls                                                                               | 2026-08-28 |
| P-95, the requirement stops prescribing a mechanism; a slash form is not a transition                           | `spec/interface.md`, `spec/console.md`, `releases/first-release.md` -> Controls                                                              | 2026-08-28 |
| P-96, two drawings, practical and realistic, sharing only the camera                                            | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-28 |
| P-97, the realistic drawing's terrain is continuous and crosses boundaries                                      | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-28 |
| P-98, nothing in the terrain reveals how the sphere was divided                                                 | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-28 |
| P-99, each territory has a biome                                                                                | `spec/planet.md` -> What a territory carries                                                                                                 | 2026-08-28 |
| P-100, a territory's biome is what the terrain gives it                                                         | `spec/planet.md` -> What a territory carries                                                                                                 | 2026-08-28 |
| P-101, four capabilities for the visual work, each with a vetted-when                                           | `releases/first-release.md` -> Capabilities                                                                                                  | 2026-08-28 |
| P-102, the six biomes; ocean is unclaimable and never adjacent to ocean                                         | `spec/planet.md` -> What a territory carries                                                                                                 | 2026-08-28 |
| P-103, what each biome gives a territory, and why every force of nature is 1                                    | `releases/first-release.md` -> Biomes                                                                                                        | 2026-08-28 |
| P-107, the realistic drawing shows terrain and no borders                                                       | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-28 |
| P-104, a drawing never betrays how it was made                                                                  | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-28 |
| P-105, a biome has a margin, not a border                                                                       | `spec/planet.md` -> Presentation                                                                                                             | 2026-08-28 |
| P-109, oceans never isolate land from land                                                                      | `spec/planet.md` -> What a territory carries                                                                                                 | 2026-08-28 |
| P-110, `set biome` gives a territory its biome during design                                                    | `spec/console.md` -> Commands                                                                                                                | 2026-08-28 |
| P-108, the biome check states plurality, not majority                                                           | `releases/first-release.md` -> Capabilities                                                                                                  | 2026-08-28 |
| P-106, a fifth capability: terrain resolved as finely as it is shown                                            | `releases/first-release.md` -> Capabilities                                                                                                  | 2026-08-28 |
| P-111, control without tedium: rules instead of repetition                                                      | `spec/invariants.md` -> Control without tedium                                                                                               | 2026-08-28 |
| P-112, the middle layer: rules compose, and edits stay proportional                                             | `spec/invariants.md` -> Control without tedium                                                                                               | 2026-08-28 |
| P-113, nothing plays itself, and every rule can be read                                                         | `spec/invariants.md` -> Control without tedium                                                                                               | 2026-08-28 |
| P-114, rules outlive a game and can be given away                                                               | `spec/invariants.md` -> Control without tedium                                                                                               | 2026-08-28 |
| P-117, a player's rules always finish                                                                           | `spec/invariants.md` -> Control without tedium                                                                                               | 2026-08-28 |
| P-115, a rule is a source of transitions, not a kind of one                                                     | `spec/invariants.md` -> The game is one function                                                                                             | 2026-08-28 |
| P-116, the rule editor is a fourth surface, and it is two-dimensional                                           | `spec/interface.md` -> Surfaces                                                                                                              | 2026-08-28 |
| P-120, a rule carries the number of turns it may run                                                            | `spec/invariants.md` -> Control without tedium                                                                                               | 2026-08-29 |
| P-119, every rule has a text form, and the text is the rule                                                     | `spec/invariants.md` -> Control without tedium                                                                                               | 2026-08-29 |
| P-121, `/save <file>` writes the history to a file                                                              | `spec/console.md` -> Commands                                                                                                                | 2026-08-29 |
| P-118, the rule editor is out of the first release, and the surfaces line says so                               | `releases/first-release.md` -> Scope, Controls                                                                                               | 2026-08-29 |
| P-122, a capability for playing the loop through by hand                                                        | `releases/first-release.md` -> Capabilities                                                                                                  | 2026-08-29 |
| P-123, neither the biome rule nor the connectivity rule yields                                                  | `spec/planet.md` -> What a territory carries                                                                                                 | 2026-08-30 |
| P-125, every structure built everywhere it can be built, and what that means                                    | `spec/control.md` -> Winning                                                                                                                 | 2026-08-30 |
| P-127, `show` says what can be done, not only what is true                                                      | `spec/console.md` -> Commands                                                                                                                | 2026-08-30 |
| P-128, a surface is never more capable than the console                                                         | `spec/invariants.md` -> Everything is expressible                                                                                            | 2026-08-30 |
| P-129, a territory holds only so much of each kind of thing                                                     | `spec/logistics.md` -> Capacity                                                                                                              | 2026-08-31 |
| P-130, the kinds and the transformations are data                                                               | `spec/invariants.md` -> The game is data                                                                                                     | 2026-08-31 |
| P-126, metal and energy carry between turns, food does not, and each resource is conserved or not               | `spec/resources.md` -> The list, `spec/turn.md` -> Order of operations                                                                       | 2026-08-31 |
| P-131, units and structures as one table                                                                        | `releases/first-release.md` -> Units and structures                                                                                          | 2026-08-31 |
| P-132, the first release's transformations as one table                                                         | `releases/first-release.md` -> Transformations                                                                                               | 2026-08-31 |
| P-133, which things ready                                                                                       | `releases/first-release.md` -> Units and structures                                                                                          | 2026-08-31 |
| P-134, the state is things                                                                                      | `spec/invariants.md` -> The game is data                                                                                                     | 2026-08-31 |
| P-135, competing effects are resolved together, and nothing wins by being first                                 | `spec/turn.md` -> Order of operations                                                                                                        | 2026-08-31 |
| P-136, when effects compete, and when they merely follow                                                        | `spec/turn.md` -> Order of operations                                                                                                        | 2026-08-31 |
| P-137, purge founding                                                                                           | `spec/console.md`, `spec/control.md`, `spec/population.md`, `spec/unit-types.md`                                                             | 2026-08-31 |
| P-138, order is spent: matter is conserved, arrangement is not                                                  | `spec/resources.md` -> The list, `spec/turn.md` -> Order of operations                                                                       | 2026-08-31 |
| P-139, a recipe, not a transformation, and a recipe belongs to the player or the world                          | `spec/invariants.md` -> The game is data, `releases/first-release.md` -> Recipes                                                             | 2026-08-31 |
| P-140, two recipes the table did not have: upkeep, perish and revert                                            | `releases/first-release.md` -> Recipes                                                                                                       | 2026-08-31 |
| P-148, a bin is where everything is, and a capacity is a bin                                                    | `spec/logistics.md` -> Capacity, replaced whole and renamed Containment                                                                      | 2026-09-01 |
| P-147, every cycle among recipes must spend readiness                                                           | `spec/invariants.md` -> The game is data                                                                                                     | 2026-09-01 |
| P-141, a unit carries fuel, not cells                                                                           | `spec/units.md`, `releases/first-release.md` -> Units and structures, Recipes                                                                | 2026-09-01 |
| P-142, a quantity is a number                                                                                   | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-146, what a thing is made of, and a garrison and an extractor cost 1 metal                                    | `releases/first-release.md` -> Units and structures                                                                                          | 2026-09-01 |
| P-145, `perish` destroys metal, which the specification says cannot happen                                      | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-149, a territory has a density and room for extractors; nodes go                                              | seven spec files and `releases/first-release.md`                                                                                             | 2026-09-01 |
| P-150, labor need not come from a citizen                                                                       | `spec/console.md`, and one line moved from `spec/population.md` -> Labor to `spec/economy.md` -> Structures and labor                        | 2026-09-01 |
| P-143, the release declares its own vocabulary: kinds, families, bins and traits                                | `releases/first-release.md`, four new sections                                                                                               | 2026-09-01 |
| P-151, a quantity may read a trait of the place, not only of an ingredient                                      | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-152, the Traits table declares thirteen and the release uses five more                                        | `releases/first-release.md` -> Traits                                                                                                        | 2026-09-01 |
| P-153, `commands/` is in no lane's column                                                                       | `CLAUDE.md` -> Perspectives, the Code row                                                                                                    | 2026-09-01 |
| P-157, a thing contains things, and a territory is one                                                          | `spec/logistics.md` -> Containment, replaced whole                                                                                           | 2026-09-01 |
| P-155, readiness is written in the recipe, not assumed by a rule                                                | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-158, the Scope column is the owner column wearing a location's name                                           | `releases/first-release.md` -> Recipes, Traits                                                                                               | 2026-09-01 |
| P-159, `consumed` is derived, not declared                                                                      | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-156, what a territory has room for                                                                            | `releases/first-release.md`, a new section                                                                                                   | 2026-09-01 |
| P-160, adjacency is defined under *What a territory carries*, and it is not one                                 | `spec/planet.md`, one line moved to Distance                                                                                                 | 2026-09-01 |
| P-161, `grow`'s new ingredient is the fifth thing that needs echoing                                            | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-163, an orbit sits beside its territory, and launching is a move                                              | `spec/orbit.md`, `releases/first-release.md` -> Recipes                                                                                      | 2026-09-01 |
| P-162, `P-156` reintroduced the word `bin`, which `P-157` had just removed                                      | `releases/first-release.md` -> What a territory has room for                                                                                 | 2026-09-01 |
| P-164, *Where things are* still describes bins, and is wrong about orbit too                                    | `releases/first-release.md` -> Where things are, Traits                                                                                      | 2026-09-01 |
| P-154, control is derived from a citizen being there                                                            | `releases/first-release.md` -> Traits, Recipes                                                                                               | 2026-09-01 |
| P-170, a part is one metal arranged, and a thing binds its parts with more                                      | `spec/resources.md`, `releases/first-release.md` -> Units and structures, Traits                                                             | 2026-09-01 |
| P-165, tune the Ark so that what goes in is what comes out                                                      | `releases/first-release.md` -> Units and structures, Recipes                                                                                 | 2026-09-01 |
| P-169, `spoil` takes surplus food, and the order falls out                                                      | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-167, `build extractor` takes no metal and names no resource                                                   | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-166, an ingredient may be named, and `move` names two territories                                             | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-171, three build recipes, one per resource, and the blank goes                                                | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-172, a player can write one rule for many things, not one rule per thing                                      | `spec/invariants.md` -> Control without tedium                                                                                               | 2026-09-01 |
| P-168, an action that would waste something says so before it is taken                                          | `spec/interface.md`, a new section                                                                                                           | 2026-09-01 |
| P-173, `P-158` deleted the Scope column and two things still depend on it                                       | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-174, a Yard is the only thing a player builds without labor                                                   | `releases/first-release.md` -> Recipes, Units and structures                                                                                 | 2026-09-01 |
| P-176, `eat` is `upkeep`, and `depart` is `perish`                                                              | `releases/first-release.md` -> Recipes, Traits, Units and structures                                                                         | 2026-09-01 |
| P-177, `revert` cannot fire, and the release should not carry it                                                | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-175, `arriving` earns nothing and should go                                                                   | `releases/first-release.md` -> Recipes, Traits                                                                                               | 2026-09-01 |
| P-178, `surplus` is derived from a recipe that no longer exists                                                 | `releases/first-release.md` -> Traits                                                                                                        | 2026-09-01 |
| P-179, a Pioneer may cross its own empire, and one line says it may not                                         | `releases/first-release.md` -> Scope                                                                                                         | 2026-09-01 |
| P-180, the `upkeep` trait says *a unit*, and a citizen has one                                                  | `releases/first-release.md` -> Traits                                                                                                        | 2026-09-01 |
| P-181, `perish` reads a citizen's metal, and a citizen's metal is blank                                         | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-01 |
| P-182, a lane owns the tools for its own work; the code lane owns the game                                      | `CLAUDE.md` -> Perspectives                                                                                                                  | 2026-09-02 |
| P-186, an Ark and a Pioneer deploy the same things, and both bind with three metal                              | `releases/first-release.md` -> Recipes, Units and structures                                                                                 | 2026-09-02 |
| P-187, `spend readiness` is named `create labor`                                                                | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-02 |
| P-189, food keeps for one turn, by a number that decrements                                                     | `releases/first-release.md` -> Traits, Recipes                                                                                               | 2026-09-02 |
| P-188, capacity is total, used and available, and only the total is stored                                      | `spec/logistics.md` -> Containment                                                                                                           | 2026-09-02 |
| P-185, the turn's ending says everything with upkeep pays it                                                    | `spec/turn.md` -> Order of operations                                                                                                        | 2026-09-02 |
| P-191, `room` is renamed total capacity everywhere it means the stored maximum                                  | `spec/planet.md`, `spec/economy.md`, `spec/orbit.md`, `spec/console.md`, `spec/control.md`, `spec/logistics.md`, `releases/first-release.md` | 2026-09-02 |
| P-184, the world's recipes fire at the end of a turn, in the order `spec/turn.md` gives                         | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-02 |
| P-190, the recipe table takes seven columns, and role carries require, limit, consume and produce               | `releases/first-release.md` -> Recipes                                                                                                       | 2026-09-02 |
| P-192, a territory and an orbit are kinds; there are twelve                                                     | `releases/first-release.md` -> Kinds, Traits                                                                                                 | 2026-09-02 |
| P-193, the tables are the data the game loads, and every other form is generated                                | `spec/invariants.md` -> The game is data                                                                                                     | 2026-09-02 |
| P-194, a proposal's block is text, rows or an instruction, and says which                                       | `CLAUDE.md` -> Promotion                                                                                                                     | 2026-09-02 |
| P-195, the four lines that assumed every block was text, and a `shape` field                                    | `CLAUDE.md` -> Promotion                                                                                                                     | 2026-09-02 |
| P-196, a place is a territory or an orbit, edges have kinds, and a unit says which it crosses                   | `releases/first-release.md` -> Families, Traits, Units and structures, Recipes                                                               | 2026-09-03 |
| P-197, the three things a promotion can do, in words that define themselves                                     | `CLAUDE.md` -> Promotion                                                                                                                     | 2026-09-03 |
| P-198, specification and presentation do not share a format; a rendering is never canonical                     | `CLAUDE.md` -> Perspectives                                                                                                                  | 2026-09-03 |
| P-199, what the game is made of lives in a data file, not in code and not in markup                             | `spec/invariants.md` -> The game is data                                                                                                     | 2026-09-03 |
| P-200, the data browser has two views, and both name every column                                               | `spec/interface.md` -> Surfaces                                                                                                              | 2026-09-03 |
| P-201, Auto is Owner again, the duplicate metal column goes, and Capacity is Container                          | `releases/first-release.md` -> Recipes, Units and structures, Where things are                                                               | 2026-09-03 |
| P-202, a control among alternatives shows which is chosen, and the drawing's binding is written down            | `spec/interface.md` -> What an action shows, `releases/first-release.md` -> Controls                                                         | 2026-09-03 |
| P-203, a generated file is written in the form the padder would leave it                                        | `CLAUDE.md` -> Perspectives                                                                                                                  | 2026-09-03 |
| P-204, three rules for using AI assistants, the third being a place to look                                     | `docs/process.md`, a new section                                                                                                             | 2026-09-03 |
| P-206, an extractor is three kinds and a family, and the capacity exception goes                                | `releases/first-release.md` -> Kinds, Families, Traits, Recipes                                                                              | 2026-09-03 |
| P-207, every kind is bounded and the table says by what                                                         | `releases/first-release.md` -> What bounds a kind in a territory                                                                             | 2026-09-03 |
| P-208, *Units and structures* lists the three extractor kinds rather than the family                            | `releases/first-release.md` -> Units and structures                                                                                          | 2026-09-03 |
| P-209, the `kind` trait says one of the kinds rather than restating their number                                | `releases/first-release.md` -> Traits                                                                                                        | 2026-09-03 |
| P-210, the `biome` trait says one of the biomes rather than restating their number                              | `releases/first-release.md` -> Traits                                                                                                        | 2026-09-03 |
| P-211, how I know the game is right: four artifacts and the closure between them                                | `docs/process.md`, a new section                                                                                                             | 2026-09-03 |
| P-212, a command is a node and a value may be another one                                                       | `spec/console.md` -> Commands                                                                                                                | 2026-09-03 |
| P-213, a definition arrives whole                                                                               | `spec/invariants.md` -> The game is data                                                                                                     | 2026-09-03 |
| P-214, a command names a recipe, so the command list is the recipe list                                         | `spec/console.md` -> Commands                                                                                                                | 2026-09-03 |
| P-215, a rejection names line, column and the enclosing command                                                 | `spec/console.md` -> Errors                                                                                                                  | 2026-09-03 |
| P-216, normalize what you compare, nest what you do not                                                         | `spec/interface.md` -> Surfaces                                                                                                              | 2026-09-03 |
| P-217, the commands that are not recipes, and why they are listed                                               | `spec/console.md` -> Commands                                                                                                                | 2026-09-03 |
| P-218, the data that runs the game lives in a data file                                                         | `spec/invariants.md` -> The game is data                                                                                                     | 2026-09-04 |
| P-219, what the scenario test locks, and what locking means                                                     | `docs/process.md` -> How I know the game is right                                                                                            | 2026-09-04 |
| P-221, a rule is decided in the spec and the game's data in its data file                                       | `spec/README.md` -> Rules for this directory, under rule 3                                                                                   | 2026-09-04 |
| P-222, `markup` and `restates` replaced with what `P-218` says precisely                                        | `spec/invariants.md` -> The game is data                                                                                                     | 2026-09-04 |
| P-223, two kinds of stated fact, and markdown's tier named                                                      | `CLAUDE.md` -> Perspectives                                                                                                                  | 2026-09-04 |
| P-220, rule 7 sends the game's data to a data file rather than a release                                        | `spec/README.md` -> Rules for this directory, rule 7                                                                                         | 2026-09-04 |
| P-224, a release does not contain the game's data and links to the generated view                               | `releases/README.md` -> The rule that keeps them from contradicting each other                                                               | 2026-09-04 |
| P-225, deleting the expected data is how I change my mind, and absence means acceptance                         | `docs/process.md` -> How I know the game is right                                                                                            | 2026-09-04 |
| P-228, a test is for what a person cannot repeat, not for what happens once                                     | `docs/process.md` -> How I know the game is right                                                                                            | 2026-09-04 |
| P-229, a proposal says whether it wants approval or a decision, and the quotation is the offer                  | `CLAUDE.md` -> Promotion                                                                                                                     | 2026-09-04 |

## Rejected

Nothing yet. Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

## Withdrawn

| Proposal                                                                              | Why                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P-2, "twenty planet sizes are available below 500"                                    | Superseded by Sean's edit fixing the game at five named sizes.                                                                                                                                                                                                                                                                                              |
| P-3, "no two territories are more than `3m` apart"                                    | Superseded by the per-size statement, and incomplete: `3m` holds for class I only, while the large planet is class III where the measured distance is 7.                                                                                                                                                                                                    |
| P-4, "the twelve five-neighbour territories sit in six antipodal pairs"               | Derivable from the Goldberg choice, and no rule leans on it.                                                                                                                                                                                                                                                                                                |
| P-5, "a pentagon's farthest territory is its antipodal twin"                          | Merged into P-4, then withdrawn with it.                                                                                                                                                                                                                                                                                                                    |
| P-7, "the smallest planet has no six-neighbour territories"                           | Derivable from P-6 plus a line Sean had already written - the minimum is 12, **a dodecahedron**.                                                                                                                                                                                                                                                            |
| P-9, "the distance between every pair is computed once and stored"                    | An implementation directive, not a rule of the game.                                                                                                                                                                                                                                                                                                        |
| P-13, "the greatest distance is 3 / 5 / 6 / 7 / 9 by planet size"                     | Determined by the Goldberg choice and the size, nothing leans on it, and the numbers are **already asserted by a test**.                                                                                                                                                                                                                                    |
| P-15, "Native life is a planet's own, Feral is printed life gone wild"                | **Feral is behavioural, not an origin**, and origin is not substantively relevant.                                                                                                                                                                                                                                                                          |
| P-16, "every unit carries a name that persists when control changes"                  | **A unit has a type, and the type has a name.** Individual units of the same type are not distinguished.                                                                                                                                                                                                                                                    |
| P-17, "depart is left unspecified so one rule covers biological and machine"          | Sean chose **starves**, committing to the biological reading for now; robots come later. Recorded in [the backlog](spec-backlog.md).                                                                                                                                                                                                                        |
| P-20, "extracting one resource has no effect on extracting any other"                 | Written against the rating model and contradicted by the node model: **labor is shared**, so working a food extractor does compete with working a metal one.                                                                                                                                                                                                |
| P-29, "a territory's threat level comes from what is on it"                           | Superseded by P-32. Threat is no longer a quantity a territory carries - it is one direction of **force**.                                                                                                                                                                                                                                                  |
| P-36, "accidental damage is force 1, a predator is force 2"                           | Superseded on 2026-08-26: **force is inherent to the territory**, not carried by individual creatures, so there is nothing for a per-creature value to attach to.                                                                                                                                                                                           |
| P-40, "the least force eats from food nodes; every species grows by the citizen rule" | Superseded on 2026-08-26. Nature has no population and **does not use nodes** - a node is intentional exploitation. The whole food chain goes with it.                                                                                                                                                                                                      |
| P-43, "nothing is exterminated; coordination buys suppression"                        | Superseded on 2026-08-26. It described populations held at zero, and nature no longer has a population to hold anywhere.                                                                                                                                                                                                                                    |
| P-46, "citizens and food move between adjacent territories"                           | Cut on 2026-08-26. Sean removed logistics for now so that **each territory is self-contained**; the only thing crossing a boundary is a mobile unit. Recorded in [the backlog](spec-backlog.md).                                                                                                                                                            |
| P-56, "a territory satisfies its own consumption first"                               | Cut with P-46 on 2026-08-26 - it only had work to do while a remainder could reach a neighbour.                                                                                                                                                                                                                                                             |
| P-124, "where a generated file lives"                                                 | Housekeeping rather than a decision, under the split Sean approved 2026-08-30. Settled by the specification lane and landed in `CLAUDE.md` -> Perspectives in the same commit.                                                                                                                                                                              |
| P-144, "capacity and metal content have rules but no numbers"                         | Withdrawn on Sean's instruction, 2026-08-31. Its flat per-territory capacities are wrong under his storage rule: an extractor holds one cycle and a bin holds the rest, so a resource's capacity is the sum of the extractors and bins present, not a constant. **Its metal-content column survives as `P-146`**, which `P-145` depends on.                 |
| P-183, a recipe acts in one place and its results appear there                        | Withdrawn into P-190. Sean's `scope` column has a value `everywhere` - food spoils wherever it is - so *one place* would have been wrong as a general rule. The build case was already covered by `spec/logistics.md`: *whatever pays a cost must be in the territory where the thing being paid for is built*.                                             |
| P-205, a node is a kind, and a territory has no capacity for nodes                    | Withdrawn for Sean's own counter, which is better. A territory has capacity 8 for citizens and there is no citizen slot kind; a node is a slot invented for one kind of thing where nine others manage without one. `P-206` removes the same exception by splitting the extractor into three kinds, which adds no mechanism at all.                         |
| P-227, the first expected data is derived rather than accepted                        | Sean, 2026-09-04: *As a human I can remember to vet the scenario test the first time, it is remembering to do some mundane task each time that is impossible for a human, which is why we need a test to fail for those times to remind the human.* The first seed is a known special case he handles by knowing to. `P-228` is the principle underneath it |
| P-226, when a proposal quotes more than one passage the last is the offer             | Sean, 2026-09-04: the distinction that matters is *between proposals that are ready for me to approve, and proposals that are drawing attention to decisions I need to make*. `P-229` makes that split and dissolves this ambiguity instead of ruling on it                                                                                                 |