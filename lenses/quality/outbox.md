# Quality outbox

**Derived.** The quality lens's one outbox. Every finding it has addressed to somebody, and what
became of it. Not binding - a finding is a claim about the tree, not a decision about it.

[Quality](README.md) · [Reports](README.md#reports) · [The proposal queue](../../docs/notes/proposals.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to code` - a defect or a decision in `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`,
  `hooks/` or CI. The code lane acts; Sean never has to see it.
- `to spec` - something for `spec/`, `releases/` or `docs/`. The specification lane turns it into a
  numbered proposal; **it does not decide it.**
- `to sean` - a question or a decision only Sean can make.
- **Unaddressed** research does not appear here at all. It lives in a dated report and is nobody's
  work until this file gives it a reader.

**Status** is one of `open`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` items are
outstanding.

> **The guarantee.** If nothing here is `open`, this lens knows of nothing outstanding. That is a
> promise about this file, not about the tree - it does not say the code is correct, only that
> everything this lens knows to be wrong is sitting where its reader will find it.

An item may be rejected. A producer that declines one says so in the commit that declines it, citing
the id, and this file records it. **A rejected finding is not a failure of the process** - `Q-16`
was wrong, and being refuted is the lens working.

---

## Open

> **A correction to how this lens estimates cost, 2026-08-30.** `Q-34` told the code lane that
> `tools/outbox` *"already parses `docs/notes/proposals.md` including each proposal's destination and
> date, so the flag costs almost nothing."* **It did not.** The tool read that file and parsed nothing
> from it, because the queue is a table and the tool was deliberately built not to parse tables. The
> work was table parsing plus grouping, not grouping alone.
>
> This is the third time this lens has asserted something without checking it - a specification
> requirement taken from a code comment (`Q-16`), a missing channel that existed the whole time, and
> now a capability that did not. The pattern is specific and worth naming: **claims about state get
> verified here; claims about capability and cost do not.** An estimate handed to a producer is a
> claim about code and earns the same check as any other. Verified before filing from now on, or
> filed without a number.


### Q-1 - The palette exists in three places and nothing checks the copies agree

**to** code · **status** open · **raised** 2026-08-28 · **source**
[report 1, finding 5](2026-08-28-crate-boundaries-and-duplication.md#5)

`planet-render/src/palette.rs`, hand-transcribed decimals in `planet-bevy/src/planet.wgsl`, and the
transfer function again in a `globe.rs` test. Change a hex value and the CPU and GPU paths draw
different worlds. Better deleted than tested: the uniform already carries a 512-entry array.

### Q-2 - `Biome` lives in the game, so terrain and rendering depend on the game

**to** code · **status** open · **raised** 2026-08-29 · **source**
[report 3, finding 2](2026-08-29-coupling-under-the-game.md#2)

A decision, not a defect. `planet-terrain` and `planet-render` each depend on the whole of
`game-model` for one `use` line. Architecture rule 1 says the shared part belongs beneath both;
`PlanetSize` in `planet-model` is the precedent. Blocks `Q-3` from being finished.

### Q-3 - `planet-bevy` depends on `game-front`, so a prototype links the whole game

**to** code · **status** open · **raised** 2026-08-28 · **source**
[report 3, finding 3](2026-08-29-coupling-under-the-game.md#3), folding
[report 1, findings 6 and 11](2026-08-28-crate-boundaries-and-duplication.md#6)

`GlobePlugin::detached` removed the three systems that touch the front end and not the manifest
line, so `cargo tree -p goldberg-view` still contains the command language. Splitting the crate is
the same work as report 1's finding 11, which is why the three are one item.

### Q-4 - `planet-ecs` is wired into the shipped app and does nothing there

**to** code · **status** open · **raised** 2026-08-28 · **source**
[report 1, finding 7](2026-08-28-crate-boundaries-and-duplication.md#7)

A decision, not a defect. Nothing pushes an intent, nothing reads a `Region`, and `WorldTopology`
goes stale after `/new`. Meanwhile `docs/architecture.md` rule 6 states the losing side as fact.
Blocks `Q-8`.

### Q-5 - Engine-free policy lives in `planet-bevy`, where the gate cannot test it

**to** code · **status** open · **raised** 2026-08-28 · **source**
[report 1, finding 8](2026-08-28-crate-boundaries-and-duplication.md#8)

`Orbit`, `Fingers`, `readable_on` and `summary` need no engine, and every test in `globe.rs` and
`gpu.rs` is pure. None runs before deploy, including the regression test for a bug that shipped
once.

### Q-6 - `planet_ecs::gather` is dead, and its body exists twice more

**to** code · **status** open · **raised** 2026-08-28 · **source**
[report 1, finding 9](2026-08-28-crate-boundaries-and-duplication.md#9)

Called by nothing. The confluence test covers the inlined copy, not the two others.

### Q-7 - Two independent computations of which territories touch

**to** code · **status** open · **raised** 2026-08-28 · **source**
[report 1, finding 10](2026-08-28-crate-boundaries-and-duplication.md#10)

`binding.rs` and `topology_of` agree only because `Params::default()` sets `jitter: 0.0`. A test
asserting the two graphs are the same would fail on the day jitter is turned on, which is the day it
needs to.

### Q-8 - Two identities for one territory, with opposite conventions

**to** code · **status** open · **raised** 2026-08-28 · **source**
[report 1, finding 12](2026-08-28-crate-boundaries-and-duplication.md#12)

`TerritoryId` counts from one, `RegionId` from zero. Correctly waiting on `Q-4`; merging them before
that is decided would be guessing.

### Q-9 - Small duplication and dead code, six items

**to** code · **status** noted · **raised** 2026-08-28 · **source**
[report 1, finding 14](2026-08-28-crate-boundaries-and-duplication.md#14)

Noted and deliberately not, unless one is already being touched. Listed so a later report does not
present them as new.

### Q-10 - The quotation guard's convention has an unchecked near-miss form

**to** code · **status** open · **raised** 2026-08-29 · **source**
[report 3, finding 6](2026-08-29-coupling-under-the-game.md#6)

The walk was fixed in `b43d9b4`. The form was not: `quotations.rs:138` recognises
`` `spec/x.md`: *italic* ``, and `realistic.rs:117` writes `` `spec/x.md` says … **bold** `` - which
is a claim about the specification's wording, and is false.

### Q-11 - The composition root has grown logic and tests

**to** code · **status** open · **raised** 2026-08-29 · **source**
[report 3, finding 7](2026-08-29-coupling-under-the-game.md#7)

`main.rs` says the crate holds no logic; the crate is 517 lines with eight tests. `inspect.rs` earns
its place by its own argument. The doc comment does not.

### Q-12 - Two hand-rolled option parsers

**to** code · **status** noted · **raised** 2026-08-29 · **source**
[report 3, finding 8](2026-08-29-coupling-under-the-game.md#8)

Noted and deliberately not. Recorded so a third is noticed as a third.

### Q-15 - Who checks the specification is buildable

**to** spec · **status** open · **raised** 2026-08-29 · **decided** 2026-08-30 by Sean · **source**
[the recommendation](2026-08-30-readiness-and-one-surface.md#1-q-15-expand-the-specification-lane-do-not-add-a-lens)

**Decided: expand the specification lane, do not add a lens.** Sean approved the recommendation on
2026-08-30.

What was decided, so it does not have to be re-derived:

- **No readiness lens.** Half the job is already `first_release.rs`, which runs the whole release
  script and reads the release's own tables. That is the 2026-08-26 readiness note's method
  mechanised, and it runs in the gate.
- **The gap is a check of the wrong shape, not a missing perspective.** The promotion protocol asks
  *does this invalidate something* - directional, per item. A contradiction is symmetric and between
  items, so `P-100` and `P-109` passed it correctly.
- **The rule is a trigger, not a duty**, because every hand-held duty here has rotted and every
  mechanical one has held: *when more than one proposal lands in the same file and section, re-read
  that section whole and ask whether all of them can hold at once.*

**Still `open`, deliberately.** A decision is not its implementation. The rule does not exist until
it is in `CLAUDE.md`, and marking this answered would drop it out of the pending list before that -
which is the failure the pending list exists to prevent. It closes when the text lands.

The wording is this lane's to write, not this lens's. The last time this lens wrote a section for
`CLAUDE.md` verbatim, the lane improved it in three places on the way in and the copy here was stale
within a commit.

`Q-34` is the other half: without it the rule is a duty somebody has to remember, which is the one
thing the recommendation rejects.

**Calibration, measured 2026-08-30 after the decision.** The recommendation said *more than one*,
and this lens had not tested how often that fires on days when nothing is wrong. Measured over all
123 proposals: **it fires 13 times across 4 of the 6 active days** - close enough to *always* to be
a duty wearing a tool's clothes, which is the thing the trigger was chosen to avoid.

Raising the threshold to **three or more in one file and section** is much better calibrated:

| Threshold         | Fires | Days  | Catches `P-100` / `P-109`? |
| ----------------- | ----- | ----- | -------------------------- |
| two or more       | 13    | 4     | yes                        |
| **three or more** | **4** | **1** | **yes**                    |
| four or more      | 3     | 1     | yes                        |
| five or more      | 2     | 1     | **no**                     |

The group sizes are why: 70 groups of one, 9 of two, then one each at three, four, five and six. A
pair is routine - a rule and its consequence landing together. **Three in one sitting means the
section was substantially rewritten**, which is exactly when reading it whole is worth the time. All
four fires at that threshold land on 2026-08-28, the day the contradiction was written.

**Threshold refinement withdrawn 2026-08-30**, after the specification lane's reply. They measured
the same 13 fires over 83 section-days independently, agreed both known positives sit in the two
largest groups and that the nine groups of two produced nothing - and argued that fitting a
threshold to two positive examples is overfitting. They are right, and the decisive point is one
this lens did not weigh: **the costs are wildly asymmetric.** A false fire costs re-reading six to
twelve bullets. A missed contradiction cost two days and a full investigation. At that ratio, firing
too often is the correct error, and alarm fatigue is a smaller risk than a silent trigger. The
threshold stays at *more than one*.

**And they found the real defect, which this lens missed entirely: the trigger is date-scoped and
contradictions are not.** `P-100` and `P-109` landed the same day, so it fires - but had `P-109`
landed a week later the collision would be identical and the trigger silent. Nothing about a
contradiction depends on the two rules arriving together. Dropping the date costs three extra fires,
16 rather than 13, which is not a meaningful increase and closes a case the date-scoped version
misses completely.

The rule is therefore: **when a proposal lands in a section another proposal has already landed in,
re-read that section whole and ask whether all of them can hold at once.** They asked whether that
is still this lens's item. It is the same rule with a bug removed, and their version is better than
the one filed.

### Q-31 - Remove the `to sean` address; the queue is the one surface

**to** spec · **status** open · **raised** 2026-08-30 · **source**
[the recommendation](2026-08-30-readiness-and-one-surface.md#2-seans-inbox-should-be-the-queue-he-already-reads)

Sean's inbox should be the document he already opens, not a command he has to remember to run. Both
kinds of thing addressed to him are proposals in waiting: a decision a lens surfaced, and a question
that blocks the code lane - which is almost always *the specification does not say X*. Both go
`to spec`, and step 4 turns them into numbered proposals. `Q-17` becoming `P-123` is the precedent
and it worked.

`tools/outbox` then becomes a thing the **instances** run to check each other, and Sean's surface stays
one document. The cost, stated rather than glossed: the specification lane becomes the filter on what
reaches Sean. It is the lane best placed to be it, and the protocol's existing duty to record the
reason for a rejection is what keeps a filtered finding traceable.

**Rescoped 2026-08-30, after the specification lane objected. They are right and the item as filed
was wrong.** Removing the address outright would re-break the index. `tools/outbox`'s `parse()`
requires the field line to begin `**to**` and `field(fields, "to")` to return `Some`, or the item is
skipped entirely - so proposals carrying no address would make **the whole proposal queue invisible
to the index that guarantees it**, and addressing them `to spec` would be false, since they are not
addressed to that lane. This is not hypothetical: it is the bug they fixed this morning in
`8f1891d`, where the tool read `docs/notes/proposals.md`, extracted zero items and reported fourteen
open while `P-123` sat open in it.

**The correct, smaller change: keep `to sean` as the address the proposal queue uses, and stop a
lens using it.** Sean's objection was never to the address - it was to having to run a command to
see his own inbox, which `Q-33` fixes. What has to go is the lens's direct line, so that a finding
needing him becomes a numbered proposal rather than arriving by a second route. Everything still
emanates from the spec, and `Q-17` becoming `P-123` works unchanged as the precedent.

One honesty note they added about that precedent, worth keeping: `P-123` also sat unaddressed for a
day. It is a precedent for the route working *and* for the route being slow, which is a cost for
Sean to weigh rather than a blocker - and is why step 9 exists, so a blocked code session does not
stall waiting on it.

### Q-32 - `CLAUDE.md` carries two limits of fifteen that count different things

**to** spec · **status** open · **raised** 2026-08-30 · **source**
[the correction](2026-08-30-two-budgets.md)

`CLAUDE.md:317` is the original rule: *keep the open-proposal queue under fifteen*, justified by
Sean's reading time - *reviewing costs as much as writing*. `CLAUDE.md:135` is this lens's
generalisation of it: *keep the open items under fifteen across every outbox together*. Both say
fifteen and they count different things, in the same file.

**The generalisation was wrong and is this lens's error.** It kept a number whose justification is
Sean's attention and applied it to work that never reaches him. Thirteen crate-hygiene findings cost
the code lane, not Sean. The recommendation is to delete `:135` and leave `:317` alone: **Sean's
limit is on his queue; a producer's backlog is a backlog.** If a producer wants a cap it should be
its own, justified by its own capacity.

**Refined by the specification lane, and their version is better:** deleting the sentence outright
throws away a real kernel, since nothing else in `CLAUDE.md` says that a lens producing many true
findings crowds out another lens's fewer, better ones. **Keep the concern, drop the borrowed
number**, so `:317`'s fifteen stays the only figure in the file and keeps meaning what it always
meant.

**The measured cost of the error, recorded because it landed on the person the limit protects.** The
conflated number misled that lane into telling Sean the count was "15 against a limit of 15" and
that a second lens would put him over on day one. His queue was empty at the time. Wrong advice,
given to Sean, traceable to a figure this lens borrowed from a rule about his reading time and
applied to work that never reaches him.

### Q-33 - One generated document that says what is pending

**to** code · **status** open · **raised** 2026-08-30 · **source**
[the correction](2026-08-30-two-budgets.md#one-place-to-look)

Sean: *as a human, I need one place to go to figure out what is currently pending.* `tools/outbox`
already computes exactly that and prints it to a terminal, which is a command to remember rather
than a document to open.

Have it **write** the answer as well as print it - one file, regenerated, never hand-maintained.
Sean's queue first, then each producer's backlog beneath it, so one open document answers both *what
must I decide* and *what is outstanding anywhere*. Wiring it into `hooks/pre-commit` beside
`pad-tables` is what stops it going stale, and it is the same shape: a tool that rewrites a
generated artifact from the source of truth.

The tool's `LIMIT` constant currently encodes the conflated rule and should follow whatever `Q-32`
settles rather than lead it.

---

## Resolved

Kept rather than deleted, so a later report can tell whether a finding was fixed or forgotten.

### Q-13 - Adopt the workflow in `CLAUDE.md`

**to** spec · **status** **acted** 2026-08-30 · `ba4850d`, and improved in three places on the way
in - see [the record](2026-08-30-workflow-to-adopt.md#what-changed-on-the-way-in)

### Q-14 - Build the outbox index

**to** code · **status** **acted** 2026-08-30 · `e233186`. Verified: it reads both outboxes, names
the ones missing, and reports by addressee

### Q-30 - `crates/outbox.md` did not exist, so a blocked question had nowhere to go

**to** code · **status** **acted** 2026-08-30 · `67c8b40`. Verified: the file exists, is empty and
says so, and carries a guarantee in the same shape as this one. `tools/outbox` reads three outboxes

### Q-34 - Emit the same-section flag the trigger depends on

**to** code · **status** **acted** 2026-08-30 · `67c8b40`, corrected in `d6908c9`. Verified: the
date scope is gone, the flag reports 16 sections rather than 13 - the number the specification lane
predicted - and `two_proposals_a_week_apart_in_one_section_still_flag` builds the case the dated
version could not see. Threshold left at more than one. Twelve tests pass

### Q-35 - Two spellings of the same section split one group into two

**to** code · **status** **acted** 2026-08-30 · fixed on both sides independently: `073d5e2`
normalised the arrows in `docs/notes/proposals.md`, and `67c8b40` normalises before grouping with
`one_arrow`, guarded by `the_arrow_style_does_not_split_a_section`. The code lane hit the same thing
while building `Q-34` and had it fixed before the item was filed

### Q-16 - The picture never sees the biome the model has

**to** code · **status** **withdrawn** 2026-08-29 · **source**
[report 3, finding 1](2026-08-29-coupling-under-the-game.md#1)

Declined by the code lane, correctly. The remedy would have made colour uniform per territory,
drawing a boundary along every territory edge and failing
`two_regions_meeting_at_a_point_agree_about_it`. Draining fires zero times at twelve territories,
which is what ships. And the requirement it leaned on - that the drawing must show the biome the
model has - is not in `spec/planet.md`; a comment invented it. What survived became `P-123`.

### Q-17 - The biome rule and the connectivity rule cannot both hold

**to** spec · **status** **acted** 2026-08-29 · filed as `P-123`

### Q-18 - A report reaches one lane by instruction and the other by luck

**to** spec · **status** **acted** 2026-08-29 · `9d3fa25` added the fifth consequence to `CLAUDE.md`

### Q-19 - A contradiction can sit outside the queue the queue promises to hold

**to** spec · **status** **acted** 2026-08-29 · `14d9784`, `9d3fa25`

### Q-20 - Resetting the view was unreachable on a touch device

**to** code · **status** **acted** 2026-08-28 · `464ff45`, and `a1cc5e0` named the control

### Q-21 - The size keys were a binding no document named

**to** spec · **status** **acted** 2026-08-28 · `a1cc5e0`, with no code change needed

### Q-22 - Two comments quoted rules that were not there

**to** code · **status** **acted** 2026-08-28 · `464ff45`, and followed through in `8c395d8`

### Q-23 - `pre-push` and the CI gate disagreed about clippy

**to** code · **status** **acted** 2026-08-28 · `464ff45`

### Q-24 - `planet-terrain` was in neither gate list

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`

### Q-25 - The quotation guard stopped at `crates/`

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`, more broadly than asked. The *form* is
still open as `Q-10`

### Q-26 - The detached globe advertised keys it did not have

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`

### Q-27 - `docs/architecture.md`'s crate table no longer described the tree

**to** spec · **status** **acted** 2026-08-30 · verified: every row now matches its manifest, and
`planet-terrain` and `goldberg-view` have rows

### Q-28 - What *"where there is a pointer they are controls"* binds

**to** spec · **status** **answered** 2026-08-28 · `a1cc5e0` removed the sentence, as `P-95`

### Q-29 - Whether `/new <size>` changes no game state

**to** spec · **status** **answered** 2026-08-28 · `a1cc5e0` reworded it, as `P-95`
