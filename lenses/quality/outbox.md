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

**to** code · **status** open · **raised** 2026-08-28 · **source**
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

**to** code · **status** open · **raised** 2026-08-29 · **source**
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

### Q-15 - Consider a spec-readiness lens before any other

**to** sean · **status** open · **raised** 2026-08-29 · **source**
[the workflow](2026-08-29-workflow.md)

*The specification must give code enough to execute* is the one constraint with no owner: the spec
lane does not read code, and the code lane reads the spec as instructions, so a gap looks like
something to invent. Done once by hand in `first-release-readiness.md`.

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
