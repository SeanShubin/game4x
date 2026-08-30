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

### Q-30 - `crates/outbox.md` does not exist, so a blocked question has nowhere to go

**to** code · **status** open · **raised** 2026-08-30 · **source** this file

`CLAUDE.md` → Outboxes now says the code lane's outbox is `crates/outbox.md`, and `tools/outbox`
reports `not present: crates/outbox.md`. Until it exists, **step 9 of the cycle has nowhere to
land**: the code lane cannot file a question that blocks it, so it either stops or asks in a reply -
which is the failure `CLAUDE.md` names by name.

It is also a hole in the guarantee directly above it. *Nothing open means nothing outstanding* reads
over the outboxes that exist, and a perspective with no outbox is silently excluded rather than
visibly empty. An empty `crates/outbox.md` with a header is worth more than no file, because empty
is a claim and absent is not.

Found by the specification lane while adopting the workflow, and filed here because `crates/` is not
its column either.

While in that file: the tool still probes the pre-move `quality/outbox.md` and prints it under *not
present*, so a completed move now reads as a missing file. Not worth an item of its own; worth the
one-line deletion whenever `crates/outbox.md` brings someone here.

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

### Q-34 - Emit the same-section flag the trigger depends on

**to** code · **status** open · **raised** 2026-08-30 · **source** `Q-15`

`Q-15` is decided: more than one proposal landing in the same file and section prompts a whole-
section re-read. **That is only a trigger if something emits it.** If the rule lands in `CLAUDE.md`
and nothing flags the condition, it becomes exactly the hand-held duty the decision rejected.

`tools/outbox` already parses `docs/notes/proposals.md` including each proposal's destination and
date, so the flag costs almost nothing: group the accepted proposals by destination and date, and
report any group larger than one.

Tested against history it fires where it should - on 2026-08-28, six into `spec/planet.md` →
Presentation, five into `spec/invariants.md` → Control without tedium, and four into
`spec/planet.md` → What a territory carries, which is the section where `P-100` and `P-109`
collided.

Worth pointing at `Q-33` as well: the flag belongs in the generated pending document, where it is
read without being asked for, rather than only on a terminal.

### Q-31 - Remove the `to sean` address; the queue is the one surface

**to** spec · **status** open · **raised** 2026-08-30 · **source**
[the recommendation](2026-08-30-readiness-and-one-surface.md#2-seans-inbox-should-be-the-queue-he-already-reads)

Sean's inbox should be the document he already opens, not a command he has to remember to run. Both
kinds of thing addressed to him are proposals in waiting: a decision a lens surfaced, and a question
that blocks the code lane - which is almost always *the specification does not say X*. Both go
`to spec`, and step 4 turns them into numbered proposals. `Q-17` becoming `P-123` is the precedent
and it worked.

`tools/outbox` then becomes a thing the instances run to check each other, and `--to sean` stops
existing rather than being remembered. The cost, stated rather than glossed: the specification lane
becomes the filter on what reaches Sean. It is the lane best placed to be it, and the protocol's
existing duty to record the reason for a rejection is what keeps a filtered finding traceable.

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
