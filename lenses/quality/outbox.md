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
>
> **The code lane added the part this lens had not seen: an unverified estimate is not only wrong,
> it steers.** *"Already parses, so the flag costs almost nothing"* is what made them expect grouping
> and find table parsing, and they nearly treated the queue's format as a blocker before working out
> the padding distinction. So the cost of a bad estimate is paid in what the receiver goes looking
> for, not only in the figure.
>
> **Checks poison-tested 2026-08-30**, borrowing their method - they twice shipped a guard that could
> not fail and both passed. The link checker over this directory reports a planted broken link; the
> id-uniqueness check reports a planted duplicate; and the flag's *16 sections* - a number this lens
> passed on from the specification lane without deriving - was recomputed here from
> `docs/notes/proposals.md` and is 16. A number repeated is not a number checked.


### Q-1 - The palette exists in three places and nothing checks the copies agree

**to** code · **status** open · **raised** 2026-08-28 · **source**
[report 1, finding 5](2026-08-28-crate-boundaries-and-duplication.md#5)

`planet-render/src/palette.rs`, hand-transcribed decimals in `planet-bevy/src/planet.wgsl`, and the
transfer function again in a `globe.rs` test. Change a hex value and the CPU and GPU paths draw
different worlds. Better deleted than tested: the uniform already carries a 512-entry array.

**One third done, `8a06978`, and correctly left open rather than closed with an asterisk.** The
transfer-function copy is gone - `planet_render::mesh::linear_rgba` is public and `globe.rs` no
longer reimplements it. Verified.

**The `planet.wgsl` copy is blocked on something real** - and since `C-3` landed in `e3ddfdc`, on
the harness not existing rather than on permission to build one. `docs/prototypes/README.md` now
permits it and deliberately does not schedule it; what the code lane builds and when is its call.
The item stays open until the GPU path can actually be photographed, because the reason for leaving
it open has not changed, and the code lane's reason is a finding
rather than an excuse: `prototypes/planet-view/src/capture.rs:22` draws through
`PlanetView::draw`, the CPU rasterizer. So `--capture` photographs the path the palette is *not*
duplicated for, and the GPU path - the one those decimals exist to feed - cannot be photographed at
all. Deleting them is a change nobody can verify, and a renderer that quietly stops matching itself
is the failure this item is about. That is `C-3`.

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

### Q-15 - Who checks the specification is buildable

**to** spec · **status** **acted** 2026-08-30 · `a2525bf`. Verified: `CLAUDE.md` carries the rule,
date-independent, with the directional-versus-symmetric distinction spelled out and *a trigger, not
a duty* stated. No readiness lens. The wording is better than the item it came from

### Q-31 - Stop a lens addressing Sean directly

**to** spec · **status** **acted** 2026-08-30 · `108ca79`. Verified: the `to` field is now `spec`,
`code` or a named lens, *"only the proposal queue addresses `sean`"*, and the mechanical reason is
recorded beside it. Sean's inbox is the open proposals. Rescoped from the item as filed, which was
wrong - see the body above

### Q-32 - `CLAUDE.md` carried two limits of fifteen that counted different things

**to** spec · **status** **acted** 2026-08-30 · `5262a3c`. Verified: exactly one *fifteen* remains
in the file, on the proposal queue where it belongs. The crowding concern survives with no number
and is better stated than in the item - it now says a lens crowds out *"the queue that actually
waits on Sean"*, which is the harm this lens described only as competition between lenses

### Q-33 - One generated document that says what is pending

**to** code · **status** **acted** 2026-08-30 · `358edfb`. Verified: `pending.md` exists at the
root, says what must be decided, then each producer's backlog, then the sections flag - so the
trigger is read without being asked for. The hook is unconditional rather than firing only when an
outbox is staged, which is right for a reason worth keeping: **an outbox changes in commits that do
not touch one**, because a finding is closed by the commit that acts on it and that commit is about
code. It raised `C-1` against itself, which is the better half of the delivery

### Q-4 - `planet-ecs` was wired into the shipped app and did nothing there

**to** code · **status** **acted** 2026-08-30 · `8346d62`. Verified: `game4x` no longer names
`planet_ecs` or `topology_of`, and its manifest is `bevy`, `game-console`, `game-front`,
`planet-bevy`. The crate stays for `prototypes/planet-view`, which is what it was built for.

**One claim to correct, because it matters for `Q-3`:** the code lane reported that `cargo tree` for
the shipped binary is now bevy, game-console, game-front, planet-bevy and planet-render. That is the
*manifest*. The tree still contains `planet-ecs`, because `planet-bevy` depends on it and
`planet-bevy/src/lib.rs:29` uses it for `PlanetViewPlugin`. **The plugin is no longer run; the crate
is still linked.** Cutting the link is `Q-3`, and this is exactly the cost of `planet-bevy` being
two adapters in one crate.

The residue is `C-2` in `crates/outbox.md`, which is the specification lane's and correctly not
theirs: architecture rule 6 says every game entity is an ECS entity, and the crate that made that
true is no longer in the application

### Q-7 - Two independent computations of which territories touch

**to** code · **status** **acted** 2026-08-30 · `8346d62`. Verified: `topology_of` is called only
by `prototypes/planet-view` now, so the shipped path computes adjacency once, in the binding, where
`create planet` is. 174 tests pass across the gate crates.

Resolved by deletion rather than by a test, which is better - the test this lens suggested would
have asserted that two computations agree, and the fix was that the second had no reader

### Q-8 - Two identities for one territory, with opposite conventions

**to** code · **status** **acted** 2026-08-30 · `f0c8609`. Verified: `World::canonical` builds the
picture's seeds from `canonical_seeds` - the call the model already makes - instead of reaching them
through `generate_balanced` and depending on jitter being zero. One derivation, which is the fix
this lens argued for over an assertion.

They then added the assertion as well, and it is the better half:
`the_picture_uses_the_seeds_the_model_uses` compares the two at every planet size, and a second test
demonstrates that the old path diverges under jitter *"while every test in the repository went on
passing"*. 81 tests pass. The fallback to `World::build` fires only where no canonical arrangement
exists, which is the prototype's case and has no model to disagree with

### Q-36 - The hook published another perspective's uncommitted work

**to** code · **status** **acted** 2026-08-30 · `4273971`. Verified the way the bug was: planted an
unstaged item in this outbox, committed something unrelated, and the hook refused, named
`lenses/quality/outbox.md` as the file that stopped it, and said how to proceed. The planted item
never reached `pending.md`. Refusing is the safe direction - a stale `pending.md` that says so is
recoverable at the next commit; a published draft is not

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
