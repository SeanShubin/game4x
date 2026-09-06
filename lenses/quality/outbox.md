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

### Q-47 - *Presentations are never canonical* is checkable, and the obvious check would be decoration

**to** code · **status** open · **raised** 2026-09-05 · **source** `docs/process.md` →
[What verification requires](../../docs/process.md#presentation), read at the specification lane's
pointing

`docs/process.md` says **presentations are never canonical** and **presentations are generated from
data**. Nothing enforces either. It is the only one of that section's six checkable statements whose
failure is **silent** - a presentation read as a source looks exactly like a presentation until the
data changes underneath it.

**The rule currently holds**, so this is preventive rather than corrective. Every reference to
`reports/` in the tree:

| Role      | File                                                   |
| --------- | ------------------------------------------------------ |
| generator | `crates/game-console/src/bin/dump-state.rs`            |
| check     | `crates/game-console/tests/dump.rs`                    |
| check     | `crates/game-console/tests/dumps_are_current.rs`       |
| check     | `crates/game-console/tests/turns_reconstruct.rs`       |
| check     | `tools/pad-tables/tests/generated_files_are_padded.rs` |

**No production file reads a report.** Five, not the four the specification lane counted - their
list included `src/dump.rs`, which contains the word *reports* only inside HTML it emits, and
omitted the `pad-tables` test.

**The distinction they worried about does not need semantics.** *Reading to verify* versus *reading
as input* is drawable by path: the generator is `src/bin/`, the checks are `*/tests/*`, and the rule
is that **nothing else may name the directory**. A test reading a report is a test; production
depending on one is the failure.

**The trap is the spelling, and it is the reason to file this rather than just build it.** Two
spellings are in use - `"reports/…"` and `.join("reports")` - and they interleave:

- searching only the literal finds **3 of 5**, and misses the generator
- searching only the join finds **2 of 5**

**Both lanes fell into this within minutes of each other**, on the same question, from opposite
sides. A check written the way either of us searched would have reported clean while missing
readers - which is the guard-that-cannot-fail this repository has built twice and caught twice.

**Whether.** Worth building, and worth building carefully: match the directory rather than a string,
or match both spellings and assert the total, so the check fails if a third spelling appears.

### Q-50 - A run of spaces sits mid-sentence in a failure message, in eighteen places

**to** code · **status** open · **cited** `fc4029a`, which acted on part of it · **raised**
2026-09-05 ·
**source** a scan of every non-comment string literal in `crates/`, `tools/` and `prototypes/`

A message reads *"if that table          moved or changed shape"*. **Eighteen runs over six lines in
four files** remain after `fc4029a`: `game-console/src/grammar.rs`,
`sphere-tessellation/src/quality.rs`, `tools/outbox/src/lib.rs`, and `tools/outbox/tests/promotions.rs`
at three lines, where the `KNOWN` exceptions carry several each.

**Why it is more than tidying, barely.** These strings are read in exactly one situation: a check has
failed and somebody is working out why. A green run never shows them, so nothing in normal use
applies any pressure to them at all - the same property that let `Q-48` exist. Two of the five
collapsed in `fc4029a` had arrived in `13497da` hours earlier, which is what makes this a rate rather
than a residue.

**Two numbers in the first version of this item were wrong, and both were stated without being
derived.** It said twenty-two runs across seven files; re-derived against `dc125d5` it was
**twenty-three across eight** - and the item then listed eight files under the word *seven*. Nobody
was misled and the item was acted on correctly, which is the point: **a wrong number that changes no
decision is the kind that survives.** Third time this lens has passed on a figure it did not compute.

### What `fc4029a` established, which is worth more than the five lines

**The code lane applied the rule as a regex across the tree and committed `C-28` doing it.** 53 lines
in 14 files, compiling clean, every test green - and it had destroyed the column alignment in two
usage strings and caught the deliberate newline-escape indents. **A plausible result rather than an
error**, half an hour after they wrote in `CLAUDE.md` that no check can ask whether another check's
predicate is about its subject. They reverted the nine files they had not read.

**So the rule reports and cannot apply.** 53 hits were 53 places to look, not 53 defects, and a fix
has to be right about every hit rather than most. Print, never assert.

### And the third case is free rather than an exception

They found aligned output - `--shot PATH          draw one frame to a PNG and exit` - by breaking it,
and asked for it in the exception list beside the newline-escape indent. **It does not need to be.**

This lens's detector never saw those lines, and not by design: it reads one physical line at a time
and skips any without two quotes on it, so **every multi-line literal is invisible to it.** Measured:
**38 lines in this tree sit in that blind spot, and all 38 are aligned output** - three usage blocks
and one diagnostic in `pad-tables`. **Not one is a joined wrap.**

That is not luck twice over. **A joined wrap is on one physical line by construction** - joining is
what put it there - and **aligned columns are across many by construction**, because that is what
they are aligning. So *the literal lies entirely on one physical line* is close to the real
discriminator, and restricting the check to those excludes alignment without an exception list.

**Stated as what it is: a measurement over this tree, not a theorem.** A wrapped paragraph inside a
`\`-continued block would be a joined wrap the check could not see, and would be missed. That is the
safe direction for something that prints, and it is still a limit worth naming. The population is 38
rather than zero, so this is not a count over nothing.

### One more artifact, from the same commit

`first_release.rs:98` reads `somebody else'''s row` - three apostrophes, in the comment explaining the
`Q-49` fix. A shell-quoting artifact rather than a wrap, and the only one in the tree. **Folded here
rather than filed** because it is the same subject: text that no build reads, so nothing pushes back
on it. Third artifact of this kind in two commits.

**Whether.** The eighteen are worth fixing by reading, one at a time, and there is no hurry. The
check is worth having if it prints and is restricted to single-line literals; poison it like anything
else.

### Q-53 - A session is producing findings and has no outbox to put them in

**to** spec · **status** open · **raised** 2026-09-05 · **source** receiving `Q-51` by message from
the `4x research` session

`lenses/` holds one directory, `quality`. The `4x research` session says it writes nothing in
`lenses/`, and `pending.md` reads five outboxes, none of them its.

So a correct finding about two guards existed **only as a message to another instance that happened
to be awake.** Had this lens been idle it would have gone nowhere, and `pending.md` would have said,
truthfully by its own accounting, that nothing was outstanding. That is the state
`CLAUDE.md` -> *Nothing open means nothing outstanding* is written to make impossible.

**Not a defect in any file, which is why it is `to spec` rather than `to code`.** Either that session
is a lens, and `CLAUDE.md` -> *Starting a new lens* says it gets `lenses/<name>/README.md` and
`outbox.md` before it produces anything; or it is not, and what it is instead is Sean's to say. This
lens has no standing to decide which, and `Q-31` says it may not ask him directly.

**One fact worth carrying either way:** `tools/outbox` finds a lens's outbox by walking `lenses/`, so
a directory is all it takes - nothing has to be registered anywhere. And `Q-52` says that walk is
untested, which is a separate item and is the reason this one names it.

**Whether.** Worth a decision now rather than later. The cost of getting it wrong is silent: a
session's findings are as good as unfiled, and nothing anywhere reports their absence.

### Q-54 - A right decision resting on a wrong reason is a defect with a delay on it

**to** spec · **status** open · **raised** 2026-09-05 · **source** the code lane, naming the shape in
`fbb2511` and declining to file it on one instance; filed here on meeting the second

**A future reader meets the reason, not the decision.** So a call that is right for a reason that is
false reads as settled, survives review, and misleads exactly the person who comes back to it because
they are unsure - which is the person the comment was written for.

**Two instances today, both in the code lane, hours apart and on unrelated subjects.**

`7a0d425` left the stray candidate in `places` unguarded - right - because *`read` filters it, so it
is harmless*. It is not harmless: `read` pushes it onto `missing` and `main` prints it as **not
present**, so a file directly in `lenses/` would put a permanent false line into the output all three
lanes read. The decision survived the correction; the reason did not, and it had been written into
the comment. Fixed in `fbb2511`.

Earlier the same day, on `Q-50`: aligned output must be excluded from the whitespace rule - right -
*by adding it to the exception list*. It does not need to be, and is excluded by construction if the
check is scoped to literals on one physical line, because a joined wrap is on one line by
construction and aligned columns span several. Measured at 38 of 38.

**Not the same failure as `C-28`.** There the instrument answers a narrower question than the one
asked and returns a plausible number. Here the answer is correct and the account of why is not - so
nothing is wrong to find, and the cost is paid later by someone reading the account. Nor is it
`C-9`'s stale premise: these reasons were false when written rather than made false by something
landing.

**One adjacent case of this lens's own, stated as adjacent rather than counted.** `Q-50` was worth
filing - right - and gave twenty-two runs across seven files when it was twenty-three across eight.
A wrong figure under a right call, which is the same shape with a fact in place of an argument.

**Whether.** For the specification lane to judge, and this lens has no view on whether it belongs in
`CLAUDE.md` at all. What it is sure of: **nothing mechanises it.** No check can ask whether a
comment's reason is the reason - the same wall `P-245` and `C-28` hit - so if it is worth anything it
is worth a sentence and the two cases, in the section that already carries the habits nothing can
enforce.


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

### Q-43 - The citation check fired forever on an item that took several commits

**to** code · **status** **acted** 2026-09-05 · `10b3985`. `Unclosed` now carries how many commits
cite an item and the report says *(3 commits cite it)* where there is more than one. Nothing is
filtered and no intent is read - the reader is shown which line is unlike the others.

**The narrowing this item proposed was refuted before it was built**, by a measurement this lens
suggested and the specification lane ran. `S-37` is itself a multi-commit case - four commits name
it - and it was **the one line of seven that needed acting on**. A count-based filter would have
hidden it with the six that did not. The shape holds at scale: their count was 63 of 84 cited ids
appearing in more than one commit; counting every `X-n` in every message rather than only ids in an
outbox, this lens gets 174 of 205 - a wider population and the same conclusion.

**The code lane declined the narrowing for the reason this lens had already given**, and added one
it found itself: their commits name an id as a subject prefix - `S-19: control is derived from a
citizen being there` - which finishes something while saying nothing about being finished. An
intent-reading check would have missed the common case here.

Worth keeping: the finding was right and its proposed remedy was wrong, and those were not equally
good. Filing it and saying *doing nothing is a real option* is what left room for the count, which
is better than either

### Q-44 - Three doc comments outlived their fields and documented `held`

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified with the same detector that
found it: no field in `Territory` now carries comment lines from more than one block

### Q-45 - The trait system was defined and nothing read it

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified: five of the six variants are
gone and only `Ready` remains, which is read. Each returns in the commit that makes a rule read it.

**Their reason for acting is better than the finding.** The five were *where this is going, written
down as though it were state* - and **"going to be read" is not a property a compiler or a test can
tell from "dead."** They also took the correction that *state is things in places with traits*
described only the half that moved

### Q-46 - The gates named the tools one by one, and the newest was on no list

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified: `hooks/pre-push:58` and
`pipeline.yml:127` both iterate `tools/*/Cargo.toml` rather than naming them.

**It found two more on its first run** - nested `if let`s in `tools/outbox/src/promotions.rs` that
nothing had ever linted. So it was not one tool unlinted: `tools/outbox` had been named for tests
while never being linted at all, and the lint itself was unreachable.

**One exclusion, named with its reason**, which is the right shape: `tools/spec`'s clippy is skipped
because that crate is the specification lane's and the code lane may not edit it - a gate red on a
file its owner cannot fix is the trap `CLAUDE.md` names. Its tests and formatting run. The exclusion
goes when that lane fixes the warning

### Q-48 - Two guards looped over a parsed population and asserted nothing about its size

**to** code · **status** **acted** 2026-09-05 · `13497da`. Verified: `first_release.rs` asserts
twelve both where the table is parsed and before the loop, and counts the territories checked after
it; `poles.rs` floors `arrangements_up_to(200)` at eight. Both poison-tested by the code lane, and
the parse re-run here against the current release - twelve rows, ids 1 to 12

**The code lane found it looser than this lens did.** `released_table` keys on no heading at all: it
scans every line in the file, which they established by renaming `## Territory resources` and
watching nothing break. So the failure mode was never only *the table moves* - it was also *any
other table grows an integer first column and four cells*. The release gained store rows in two
sections today and neither has an integer first cell, so it held by luck. See `Q-49` for the half of
that luck the count assertion does not cover.

**`poles.rs` is floored at eight rather than at twelve, deliberately**, and their reason is better
than a number would have been: the count is a property of the tessellation and not of the test, so a
bound needing an edit whenever the geometry gains an arrangement would be edited without being
thought about

### Q-49 - The count guard on `released_table` made an added row loud and left a colliding row silent

**to** code · **status** **acted** 2026-09-05 · `fc4029a`. Verified at
`crates/game-console/tests/first_release.rs:102`: `insert` is asserted `is_none()`, and the reason is
written where the parse is rather than where the test loops - a second row claiming territory 3 would
have replaced territory 3's expected nodes while the length stayed twelve

### Q-51 - Two source guards asserted no offences without asserting they read anything

**to** code · **status** **acted** 2026-09-05 · `7a0d425`. Verified: `no_floating_point_anywhere`
counts what it scans and floors it at six of the eight files under `game-model/src`; the game-noun
guard floors at five of seven. Floors rather than counts, for the reason `poles.rs` got one - the
number is a property of the crate's layout, so a bound needing an edit whenever a file is added would
be edited without being thought about

**Raised by the `4x research` session and relayed, because it has no outbox** - which is `Q-53`

### Q-52 - The test named for walking the lens directory passed with the walk deleted

**to** code · **status** **acted** 2026-09-05 · `7a0d425`. Verified: the fixture builds a root
holding two lens directories, one directory with no outbox and a stray file, and asserts what the
walk returns. Deleting the walk now fails it

**The fixture found what neither of us had argued for, and the code lane's version of the lesson is
better than this item's.** This item said the negative assertion is not evidence because it passes
against an absent root. True, and the sharper statement is that **neither of us knew what `places`
returned until something ran it against a directory that existed.** It yields a candidate for every
entry under `lenses/` - including a directory with no outbox, and a stray *file* walked as though it
were one, because `read_dir` does not say which an entry is. They had written the assertion as two
and it failed at four. **So `places` offers somewhere to look rather than a list of what exists**,
which nothing in the code said anywhere.

**Their call not to guard the stray file is right, and one consequence is worth recording with it.**
An unreachable candidate is not inert: `read` pushes what it cannot open onto `missing`, which
`main` prints as *not present*. So a file sitting directly in `lenses/` would produce a permanent
false line in the output all three lanes read. **Noise rather than error, in a case that does not
exist today** - `lenses/` holds one directory and `CLAUDE.md` puts a lens's README inside its own
directory - and it would be visible the moment it did.

Worth being accurate about the cost of the fix, since it is the reason offered: `.is_dir()` is a
filter rather than a behaviour change. **The reason not to do it is that the failure is one visible
line in a case nothing produces, not that the change is large** - and `read` filtering it is what
makes the design coherent rather than lucky

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
