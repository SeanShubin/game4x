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


### Q-9 - Small duplication and dead code, six items

**to** code · **status** noted · **raised** 2026-08-28 · **source**
[report 1, finding 14](2026-08-28-crate-boundaries-and-duplication.md#14)

Noted and deliberately not, unless one is already being touched. Listed so a later report does not
present them as new.

**Re-checked 2026-08-30**, after two days of splitting and moving crates. Five of the six stand;
`render_asset_usages` is still uncalled and now lives in `planet-flat`. One has **grown**: the item
recorded `game4x` writing its own `WindowPlugin` while `planet_bevy::window_plugin` existed, and
`goldberg-view` now writes a third, so the shared helper is used by `planet-view` alone.

Recorded because this lens nearly logged it as resolved on a grep for `fn window()` that could not
match `fn window(asked: &options::Options)` - a pattern written against a signature that had since
gained an argument.

### Q-12 - Two hand-rolled option parsers

**to** code · **status** noted · **raised** 2026-08-29 · **source**
[report 3, finding 8](2026-08-29-coupling-under-the-game.md#8)

Noted and deliberately not. Recorded so a third is noticed as a third.

### Q-43 - The citation check fires forever on an item that takes several commits

**to** code · **status** open · **raised** 2026-09-05 · **source** the specification lane, about a
mechanism this lens caused to be built

`tools/outbox` warns when a commit cites an open item without touching its outbox - the shape of
answering something and not closing it. **For work that spans several commits it fires on every
one**, truthfully, because the item really is open and really is cited. `S-21` is mid-rewrite and
has tripped it repeatedly.

So the noise is proportional to the length of the work, and it is loudest on the items getting the
most attention. **This is the case this lens told that lane its `R-6` example did not demonstrate** -
a check firing truthfully and uselessly. `R-6` turned out to be a parser bug and a false positive;
this one is neither, and it is the real instance of *a signal that always fires is one nobody reads*.

`cited` is the existing escape and it is the wrong shape here: it means *I looked, and it stays
open*, which fits a single stray citation. For ongoing work it means adding a hash per commit, which
is the hand-kept repetition `docs/process.md` asks this lens to notice.

**A narrowing, offered as a lead.** The convention already distinguishes them: a commit that finishes
something says `finding: <id> acted`, and a commit merely working on it says the id and nothing
else. Firing only on the first would leave long work quiet and still catch the case the check exists
for.

**Whether.** Small, and worth weighing against doing nothing - noise on the most-worked item is a
real cost, but so is a check that has to parse intent out of a message. This lens has been wrong
about a cost before and is not confident here.

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

### Q-5 - Engine-free policy lived in `planet-bevy`, where the gate could not test it

**to** code · **status** **acted** 2026-08-30 · `49c4c46`. Moved rather than gated, which is the
better answer. `planet-presentation` holds `Orbit`, `Fingers`, `readable_on` and `summary`; twelve
tests, all passing, and `cargo tree` shows no Bevy anywhere beneath it - the only two mentions in
its source are comments saying where it came from. Both gate lists carry it.

They also did the half this item did not ask for: `cargo test -p planet-bevy` now runs in the gate
and in `pre-push`, in debug, reusing the build clippy already paid for. The rotations correctly did
*not* move - composing quaternions is engine arithmetic - but
`turning_the_world_never_moves_the_poles_sideways` is a regression test for a bug that shipped, so
leaving it after deploy would have half-answered the finding. *"None runs before deploy"* is now
false in both halves rather than one

### Q-10 - The quotation guard's convention had an unchecked near-miss form

**to** code · **status** **acted** 2026-08-30 · `e40629c`. Verified: the guard passes and its floor
is 40 checked quotations, up from 8. **The measurement inverted this lens's assumption** - the
colon form it was built for was the rare one, and the *unchecked* form was most of them.

Four quotations were wrong. The one that matters is `game4x/src/inspect.rs`, which said *the terrain
is continuous* where `spec/planet.md:73` says *the terrain **of the realistic drawing** is
continuous* - and the practical drawing's terrain is not continuous at all, so the dropped qualifier
was the whole claim.

Worth keeping: they measured before building, because the obvious rule - any emphasis near a
mention - reports 35 failures of which 7 are the author's own emphasis and 13 are asterisks in Rust
read as markdown. And their third poison caught an off-by-one they had just written, where the
scanner consumed up to the closing marker rather than past it, so a closer was read as the next
opener and a whole README came back attributed to `spec/planet.md`

### Q-2 - `Biome` lived in the game, so terrain and rendering depended on the game

**to** code · **status** **acted** 2026-08-30 · `7283650`. Decided as `planet-model`, beside
`PlanetSize`. Verified: `Biome` is `planet-model/src/biome.rs`, and neither `planet-terrain` nor
`planet-render` names `game-model` any more. `game-model` re-exports it, so the game still reads as
owning its vocabulary without owning the definition.

Their second argument is the one that settles it and this lens did not have it: **every rule about a
biome is written in `spec/planet.md`.** It is the planet's vocabulary, and it sat in the game only
because that is where the first rule reading one happened to be

### Q-6 - `planet_ecs::gather` was dead, and its body existed twice more

**to** code · **status** **acted** 2026-08-30 · `9cade4c`. Neither option this item offered was
available as written - `gather` collects owners and `advance_turn` needs entities too, from a
different query - so the shared thing became `by_region(count, rows)`, a function over *what is
being placed* rather than over what an owner is. Verified: three call sites, two in `planet-ecs` and
one in `planet-bevy`. Tested directly as well as through the turn, and poisoned by making it push in
arrival order, which failed three tests

### Q-11 - The composition root had grown logic and tests

**to** code · **status** **acted** 2026-08-30 · `69ab140`. One sentence, as the item said. The
header now calls the crate a composition root *and the remote control that operates it*, names both
exceptions and why each is there, and picked up `planet-presentation`, which the diagram was
missing.

It raised `C-6` against itself, which is the better half: fixing the false claim left a new one -
`main.rs` now says a rule owned by `docs/architecture.md` is broken there deliberately, and carving
an exception into another perspective's rule is not the code lane's to do

### Q-1 - The palette existed in three places and nothing checked the copies agreed

**to** code · **status** **acted** 2026-08-30 · `8a06978` and `a4e3bd1`. Verified: no palette
literals remain in `planet.wgsl`, the uniform carries both palettes plus background, border,
duplicate strength and owner tint, and `linear_rgba` is public so the transfer curve is defined
once. The harness that made the second half checkable is `--shot`, `--settle` and
`--renderer gpu|cpu` on `planet-view`.

**Their evidence argues this item better than the item did, and it checks out.** The transcription
had already drifted: `0x1B3A5C` is `0.10588…` and the shader said `0.106`. Recomputed here through
sRGB to linear and back to eight bits, on four channels - `1B`, `8B`, `4F`, `E8` - the exact value
and the transcribed one produce the **same byte** every time.

So the two copies disagreed in source and agreed in output. **A test comparing them would have
passed while they diverged**, and the disagreement would have surfaced only when someone changed a
hex value, with nothing to attribute it to. This lens argued *better deleted than tested* because a
test keeps both lists; the stronger reason is that the test would not have worked.

Also worth keeping: the harness caught a surviving `BACKGROUND` reference within ten minutes, in a
branch they had not read. Without it, that ships as a shader that fails to compile on the one path
nothing photographs

### Q-3 - `planet-bevy` was two adapters in one crate, and `planet-render` two crates

**to** code · **status** **acted** 2026-08-30 · `465437a` and `253418d`. Both halves done. Verified
by `cargo tree`: `game4x` carries `game-globe`, `planet-bevy`, `planet-render` and no rasterizer;
`goldberg-view` carries `planet-bevy` and `planet-render`; `planet-view` is the only binary with
`planet-flat` and `planet-raster`. Neither producer of a globe carries a rasterizer, and the
prototype that needs one is the only thing that has one.

Their report that the only edge between `planet-render`'s two halves was a doc comment is the
measure of how real the seam was.

Two things the split found that no compiler could. The embedded shader path contains the crate name,
so moving `planet.wgsl` left `embedded://planet_bevy/planet.wgsl` pointing at nothing and the flat
projection rendered an empty window **with no error at all** - noticed because the PNG was a quarter
of the expected size, on the path that had no instrument until `Q-1`'s harness that morning. And a
crate split dropped tests out of the gate, which is `Q-37`

### Q-37 - The gate listed crates by name, so a split silently dropped tests

**to** code · **status** **acted** 2026-08-30 · `7739826`. Verified: clippy is `--workspace` with no
list at all, the release step is `--workspace` minus seven, and a debug step names those seven. The
exclusion states one checkable fact - *does this crate link an engine* - rather than a set to
remember.

408 tests in the release step where eleven named crates were, and both prototypes are gated for the
first time. On the question this lens declined to guess at: none of the seven needs a GPU, because
no test in the workspace constructs `DefaultPlugins`. `game4x`'s 8 tests are now gated too, since
the exclusion form has no way to leave a crate out without saying so - which is the property that
makes it the right shape

### Q-38 - An outbox went stale because its filer could not see it being answered

**to** code · **status** **acted** 2026-08-30 · `954c224`. Verified: `tools/outbox` reads the log
for commits citing an open item, skipping commits that touch the item's own outbox - filing and
closing being exactly that shape. 18 tests.

**It found a live one before it was finished**: `C-5` and `C-6`, cited by `1d8c46f`, settled while
the tool to notice was being built.

The design question this lens did not anticipate is the good part. `C-5` was cited, read, and
correctly stayed open, because the citation answered half of it. Without somewhere to record that,
the report would name `C-5` on every run forever - **and a signal that always fires is one nobody
reads, which is the failure it exists to prevent.** So an item may carry `**cited** <hash>`, an
author saying *I looked, and it stays open*. Poisoned by deleting `C-5`'s.

And it prints rather than refuses, scoped the way `Q-36`'s refusal was: every perspective commits in
this tree, and failing one lane's commit because another has not closed an item would be the wrong
perspective paying

### Q-39 - Nothing checked shipped text against approved text

**to** spec · **status** **acted** 2026-09-02 · filed as `S-10` to the code lane, verified at
`docs/notes/proposals.md:98`, carrying this lens's design **and its argument against building it
yet**. They did not present it as their own.

They verified every factual claim before acting rather than after, including the two that were
corrections to their own write-up.

**Caveat withdrawn 2026-09-02.** This item said there was a real argument against building `S-10`
yet, because `P-182` may make `edit.py` reviewable and a reviewable tool may not need a check
downstream of it. **That is wrong, and the specification lane's reason is decisive and evidenced
rather than argued: a tool cannot enforce that it is used.** Three of the eleven defects were
commits chained after `python - <<PY`, an ad-hoc script rather than the guarded tool - so `spec/` was
edited outside the guards precisely on the occasions something went wrong. Reviewability lowers the
defect rate inside the tool and says nothing about the edits that never enter it.

Recorded here rather than only in a reply, because this file is the record and it carried an
argument this lens no longer holds.

The part worth keeping is their own reading of the correction: they framed the day as a question
about tempo, reached the right lever anyway, and could only find out which by being measured. *Right
about what to fix and wrong about why, and only the second is checkable*

### Q-40 - The visible half of the editing tool was the half that was not making the mistakes

**to** spec · **status** **acted** 2026-09-02 · `172ea26`. Verified: `asserts_about_the_tree`,
`check_claims` and `proposals_without_text` are in the crate, seventeen tests, and
`a_claim_of_zero_over_an_empty_population_is_refused` reproduces this lens's own error. **Poisoned
from outside the lane, which is what the item asked for and could not do** - see `Q-41` for the one
hole it has.

**Half of the item's subject remains and they said so themselves**: the crate has no binary and
nothing calls it, so `edit.py` is still what runs. `promote` is the operation that would change
that, and it waits on `outbox` exposing a proposal's text, which sits with the code lane. Not
reopened - the finding was that the guards were unreadable, and they are not any more

### Q-41 - The denominator guard checked that a denominator was non-empty, not that it was the right one

**to** spec · **status** **acted** 2026-09-02 · `40b74c0`. The narrowing taken whole: needle and
denominator are both counted inside a named section, and a claim of zero must name one. Re-poisoned
from outside the lane rather than taken:

| Case                                                     | Now                                     |
| -------------------------------------------------------- | --------------------------------------- |
| the hole as reported - `P-` within `## Open`             | **refuses** - zero out of zero          |
| the residual they documented - `P-` within `## Accepted` | passes, knowingly                       |
| a section that does not exist, `## Opne`                 | refuses - *no section*                  |
| the old form, no section named                           | refuses - a claim of zero must name one |

The third was the case worth checking and it was mine to worry about: **the fix could have
reintroduced the error one level up**, a typo'd region silently counting zero in an empty slice. It
does not - a missing section is an error rather than an empty one.

The residual is correctly out of scope and is documented on the type rather than in a note, which is
where the next author will be standing. Naming the region does not make the choice right; it makes
it written down, where picking a convenient denominator over a whole file was invisible

### Q-42 - Two sentences about a lens's column contradicted each other, and a third was stale

**to** spec · **status** **acted** 2026-09-05 · filed as `P-243`, `0c07376`, which `pending.md` now
carries under *What must be decided*. All three verified by that lane rather than taken.

They named the cause without being asked: promoting `P-240` put *every lane owns the tools for its
own work* four bullets above *and nothing else*, so **the older bullet was true until the moment the
newer one landed**. Adding to a list without re-reading the list - the same trigger they apply to
specification sections and had not applied to that one.

**One count of this lens's was wrong.** It reported six items addressing Sean; five were real and
the sixth was prose *inside* `S-17` describing capabilities, matched by a pattern that did not
require a field line. It is six now only because `P-243` has since been filed. Same family as the
zero-over-nothing error - a pattern that matched writing *about* the thing rather than the thing

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
