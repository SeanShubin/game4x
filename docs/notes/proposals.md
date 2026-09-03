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

### P-198 - An HTML file is generated, never canonical

**to** sean - **status** open - **raised** 2026-09-03 - **kind** Sean's own - **shape** text -
**into** `CLAUDE.md` -> Perspectives, after *a generated file has no owner*

**Your words, and they settle a question this lane was about to put to you.** `catalog.md` is
markdown, committed, and diffable, so a rule change shows its consequences in the same commit. HTML
does not diff. Saying HTML is never canonical decides that without anyone having to weigh it again.

> **An HTML file is generated, never canonical.** Whatever it renders is the thing with an owner,
> and the HTML is one more derived form of it.

**Basis: it makes the rule above cover the reports without special-casing them.** *A generated file
has no owner* already says nobody edits one; this says which files are always in that category, so a
report that arrives as HTML cannot quietly become the place a fact lives.

**And it keeps the property that was argued for.** Because the HTML is derived, whatever it is
derived *from* is still in the repository and still diffs - so a change to a rule still shows its
effect on the data in the commit that caused it, however the reports are rendered for reading.

## Addressed to other perspectives

Items this lane has sent outward. **Nothing here waits on Sean** - the open proposals above are the
only thing that does.

### S-15 - `P-196` moved the release again, and two checks are red

**to** code - **status** open - **raised** 2026-09-03 - **source** `P-196`, promoted in `68cc893`

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

**to** code - **status** open - **raised** 2026-09-02 - **source** the quality lens, `Q-39` - **cited** `544d751`

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

## Rejected

Nothing yet. Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

## Withdrawn

| Proposal                                                                              | Why                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P-2, "twenty planet sizes are available below 500"                                    | Superseded by Sean's edit fixing the game at five named sizes.                                                                                                                                                                                                                                                                              |
| P-3, "no two territories are more than `3m` apart"                                    | Superseded by the per-size statement, and incomplete: `3m` holds for class I only, while the large planet is class III where the measured distance is 7.                                                                                                                                                                                    |
| P-4, "the twelve five-neighbour territories sit in six antipodal pairs"               | Derivable from the Goldberg choice, and no rule leans on it.                                                                                                                                                                                                                                                                                |
| P-5, "a pentagon's farthest territory is its antipodal twin"                          | Merged into P-4, then withdrawn with it.                                                                                                                                                                                                                                                                                                    |
| P-7, "the smallest planet has no six-neighbour territories"                           | Derivable from P-6 plus a line Sean had already written - the minimum is 12, **a dodecahedron**.                                                                                                                                                                                                                                            |
| P-9, "the distance between every pair is computed once and stored"                    | An implementation directive, not a rule of the game.                                                                                                                                                                                                                                                                                        |
| P-13, "the greatest distance is 3 / 5 / 6 / 7 / 9 by planet size"                     | Determined by the Goldberg choice and the size, nothing leans on it, and the numbers are **already asserted by a test**.                                                                                                                                                                                                                    |
| P-15, "Native life is a planet's own, Feral is printed life gone wild"                | **Feral is behavioural, not an origin**, and origin is not substantively relevant.                                                                                                                                                                                                                                                          |
| P-16, "every unit carries a name that persists when control changes"                  | **A unit has a type, and the type has a name.** Individual units of the same type are not distinguished.                                                                                                                                                                                                                                    |
| P-17, "depart is left unspecified so one rule covers biological and machine"          | Sean chose **starves**, committing to the biological reading for now; robots come later. Recorded in [the backlog](spec-backlog.md).                                                                                                                                                                                                        |
| P-20, "extracting one resource has no effect on extracting any other"                 | Written against the rating model and contradicted by the node model: **labor is shared**, so working a food extractor does compete with working a metal one.                                                                                                                                                                                |
| P-29, "a territory's threat level comes from what is on it"                           | Superseded by P-32. Threat is no longer a quantity a territory carries - it is one direction of **force**.                                                                                                                                                                                                                                  |
| P-36, "accidental damage is force 1, a predator is force 2"                           | Superseded on 2026-08-26: **force is inherent to the territory**, not carried by individual creatures, so there is nothing for a per-creature value to attach to.                                                                                                                                                                           |
| P-40, "the least force eats from food nodes; every species grows by the citizen rule" | Superseded on 2026-08-26. Nature has no population and **does not use nodes** - a node is intentional exploitation. The whole food chain goes with it.                                                                                                                                                                                      |
| P-43, "nothing is exterminated; coordination buys suppression"                        | Superseded on 2026-08-26. It described populations held at zero, and nature no longer has a population to hold anywhere.                                                                                                                                                                                                                    |
| P-46, "citizens and food move between adjacent territories"                           | Cut on 2026-08-26. Sean removed logistics for now so that **each territory is self-contained**; the only thing crossing a boundary is a mobile unit. Recorded in [the backlog](spec-backlog.md).                                                                                                                                            |
| P-56, "a territory satisfies its own consumption first"                               | Cut with P-46 on 2026-08-26 - it only had work to do while a remainder could reach a neighbour.                                                                                                                                                                                                                                             |
| P-124, "where a generated file lives"                                                 | Housekeeping rather than a decision, under the split Sean approved 2026-08-30. Settled by the specification lane and landed in `CLAUDE.md` -> Perspectives in the same commit.                                                                                                                                                              |
| P-144, "capacity and metal content have rules but no numbers"                         | Withdrawn on Sean's instruction, 2026-08-31. Its flat per-territory capacities are wrong under his storage rule: an extractor holds one cycle and a bin holds the rest, so a resource's capacity is the sum of the extractors and bins present, not a constant. **Its metal-content column survives as `P-146`**, which `P-145` depends on. |
| P-183, a recipe acts in one place and its results appear there                        | Withdrawn into P-190. Sean's `scope` column has a value `everywhere` - food spoils wherever it is - so *one place* would have been wrong as a general rule. The build case was already covered by `spec/logistics.md`: *whatever pays a cost must be in the territory where the thing being paid for is built*.                             |