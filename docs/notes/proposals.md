# Spec Proposals

**Derived.** Written by Claude. Not binding, and **not the specification** - these are lines
offered for Sean's review. A proposal becomes real only when he accepts it and it lands in
[the specification](../../spec/README.md).

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## The files that need you

**Three things only you can do, and one page to browse from.**

| To           | Read                                                                                                                                                                                                              |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **create**   | this file. You approve, or say what to change                                                                                                                                                                     |
| **decide**   | [`decisions.md`](decisions.md) - the questions only you can answer, which are not words to approve. Empty when none is open                                                                                       |
| **browse**   | [`reports/index.html`](../../reports/index.html) - every generated view, and links to the two files below                                                                                                         |
| **validate** | [`scenario/expected/play.4x`](../../scenario/expected/play.4x) beside [`scenario/commands/play.4x`](../../scenario/commands/play.4x) - what the scenario is expected to produce, and the commands that produce it |

You read [the specification](../../spec/README.md) and [the release](../../releases/first-release.md)
while approving, rather than as separate errands.

**Why these and not others is in [`docs/process.md`](../process.md)** - *What I read, and what I do*,
and *Outboxes and the index* for what is deliberately not yours. **That document decides it and this
table is only the three artifacts it names**, because `P-245` says a document that restates another
links to it rather than listing it. It said *two* and named two while that document named three,
which is how `P-356` sat in a file nothing pointed at.

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

### P-467 - a garrison costs nothing and binds nothing

**to** sean · **status** open · **raised** 2026-09-12 · **kind** answered, by you, 2026-09-12 · **shape** rows · **asks** approval · **into** `releases/first-release.md` -> Units and structures

**Your answer decides it, and by your own test rather than this lane's preference.** *At this point
I don't have any strong opinions about costs, they are a means to the end of getting our first
release playable* - which is `P-445`: **I want to decide this empirically. If I never notice I need
it, I don't need it.**

**A garrison's cost is a number nothing reads.** No recipe charges it, no recipe consumes a
garrison, and nothing in `crates/` reads a garrison's binding - checked rather than assumed. **You
have not noticed needing it because nothing has ever asked.**

## The cells

| Thing        | Costs to produce | Binding |
| ------------ | ---------------- | ------- |
| **garrison** |                  |         |

**Both empty, which is a blank and not a zero** - the file already draws that distinction, and every
other row of those two columns that does not apply is blank the same way.

## What it changes, which is nothing that runs

**Checked rather than asserted:**

- **No recipe charges labor or metal for a garrison.** Three rows in the release name one: `deploy
  ark` and `found by land` produce it, `muster` requires it
- **Nothing consumes a garrison**, so the metal was never recovered and nothing gains
- **`metal in it` is derived and in no data file**, being named by no recipe row - so nothing reads
  the binding it would have read
- **The scenario's two garrisons are `{garrison}` with no traits**, in territories 1 and 2, and the
  expected state does not change

**So `spec/invariants.md` -> *nothing in the game appears or disappears without a cause inside the
model* holds again**, and it holds by the metal never arriving rather than by founding paying for
it.

## What this deliberately does not do

**It deletes a number rather than deciding what the number was for**, and this lane could find no
note saying why a garrison was given a cost. **If the intent was that holding ground costs
something, this loses it** - and the way back is `A` from the earlier draft: `found by land` and
`deploy ark` each gain a `consume 1 metal` and a `consume 1 labor`. **That is a balance decision and
you have said balance is not what you are deciding today.**

**And it makes `P-466` cleaner.** That proposal shows five of six columns agreeing with the recipes
and the garrison row disagreeing; after this the row agrees, and **the disagreement stays as the
argument for `P-466` rather than as a live defect** - it happened, which is what the case was for.
### P-465 - ten cells still say `yes` where `traits.4x` says a number

**to** sean · **status** open · **raised** 2026-09-12 · **kind** cleanup, owed by `P-457` and not filed with it · **shape** rows · **asks** approval · **into** `releases/first-release.md` -> Traits, and Units and structures

**`P-457` landed *nothing in the game is two-valued anywhere* and ten cells still say it is:**

```
spec/data/traits.4x    {trait admits:number kept:kind name:movable}
the release, Traits    | **movable** | whatever moves | yes or no | of the kind |
the release, Units     | **ark**     | ... | Movable: yes |
```

**`P-457` should have carried this and did not.** A promotion that makes something else stale files
the cleanup immediately; this one did not, and the two files have disagreed since `b7fc6a6`.

**Nothing caught it, and something does now.** The code lane's comparison derives `admits` from the
release's own cell, so both sides read the same stale words and agree - **a check whose two sides
come from one source cannot see that source move.** They confirmed it by reading the mapping rather
than the check: `0 or 1`, `yes or no` and `a number` all reach the same word.

**`C-103` is the tripwire they built from that paragraph**, in
`crates/game-console/tests/vocabulary.rs`, and it is **this proposal's own excuse**: it asserts that
exactly these eight cells state a range and exactly these two say `yes`, in the release's order.
**The day this lands it fails, and the right move is to delete it rather than update it** - a named
exception that cannot outlive what it is for.

## The cells

**Eight in *Traits*, where `Values` stops describing this release's data and says what the trait
admits:**

| Trait         | Values now | Values after |
| ------------- | ---------- | ------------ |
| **moving**    | 0 or 1     | a number     |
| **laboring**  | 0 or 1     | a number     |
| **working**   | 0 or 1     | a number     |
| **bearing**   | 0 or 1     | a number     |
| **defending** | 0 or 1     | a number     |
| **surplus**   | yes or no  | a number     |
| **unpaid**    | yes or no  | a number     |
| **movable**   | yes or no  | a number     |

**`0 or 1` was this release's maxima showing through**, which is `P-457`'s own finding, and `P-459`
put the maxima where they belong - the *Readies* column, `bearing 1, defending 1, laboring 1`. **The
range is not a fact about the trait and is no longer stated as one.**

**Two in *Units and structures*, where `Movable` is of the kind and its value is a number:**

| Thing       | Movable now | Movable after |
| ----------- | ----------- | ------------- |
| **ark**     | yes         | 1             |
| **pioneer** | yes         | 1             |

**Every other cell of that column is empty and stays empty**, which is a blank rather than a zero -
the file already says a blank is not a zero.

## The check

**`traits.4x` is re-derived after the edit and must come out byte-identical.** The derivation reads
the `Values` cell, so `a number` and `0 or 1` both give `admits:number` - **the file does not move,
which is the point**: this changes what the release *says* and not what it means, and the assertion
is that nothing downstream shifts.

**And `surplus` is in no data file**, being derived and named by no recipe row, so its cell is
corrected for the reader rather than for any generator.

## Addressed to other perspectives

### S-120 - `P-455` landed: the three files are in `spec/data/`, and three of your checks are red by design

**to** code - **status** **acted** 2026-09-12 - **cited** `2153bf3` - **raised** 2026-09-12 - **source** promoting `P-455`, then running your `declare` suite against the files it wrote

**`spec/data/` holds `kinds.4x` at 22 lines, `families.4x` at 4 and `biomes.4x` at 6.** The
proposal's own check ran and passed in both directions: kinds 22 against 18 with the four left over
exactly `kind`, `trait`, `family`, `value`; families 4 against 4 identical; biomes 6 against 6
identical, with jungle at 2 and every other claimable biome at 1 so a generator writing a constant
fails there.

**`cargo test -p game-console --test declare` is now 3 of 6 red, and all three are yours.** This
lane does not edit `crates/` even to fix an obvious break, and `CLAUDE.md` says a promotion that
makes the gate red says so in the same breath. Two of the three are assertions you wrote to fire on
exactly this day:

- `the_families_file_names_every_family_and_invents_none` - *`spec/data/families.4x` exists now, so
  this must read it instead of the generator - the file is the population and the generator is a
  copy of it*
- `the_biomes_file_carries_nature_and_leaves_the_guiding_numbers_out` - *`spec/data/biomes.4x`
  exists now, so this must read it instead of the generator*

**The third is not a message you left yourself and is worth reading twice**:

- `the_file_of_kinds_and_the_release_declare_the_same_words` - *`ark` carries 2 traits, and a
  declaration carries only the name - the prose column stays prose, which is what rule 7 says*,
  left 2, right 1

**That one is `P-448` arriving, not rule 7 being broken.** A kind declares which family it is in, so
`{kind family:unit name:ark}` carries two traits and is right. The assertion predates the
inversion and is measuring the old shape. **And it is the stripping exception's other half**:
`P-455` said the byte assertion that taking `family:` and the `value` line back out gives the file
exactly stops holding the day this lands, and this is that day.

**Nothing here is a defect in what you built.** Three checks whose subject moved under them, which
is the shape `CLAUDE.md` calls *a rule that moves under an open item makes it wrong without touching
it* - except these are checks rather than items, so they went red instead of going quiet, which is
the better failure.

**`pre-push` runs the full gate, so this lane cannot push while they are red**, and whether to
`--no-verify` is Sean's call rather than either of ours. The commits are in.

**What is left of `spec/data/` after this.** `traits.4x` waits on `P-457`, and the trait lines a
kind carries wait on `P-456`. Both are with Sean. The three files above wait on nothing.

### S-118 - `P-458` landed and it is not work for you, and it answers what you asked

**to** code - **status** **acted** 2026-09-12 - **cited** `126c41f` - **raised** 2026-09-12 - **source** promoting `P-458`, then reading the index for open items citing `spec/invariants.md`

**Two new sections**: *What a person types* and *When a primitive earns its place*. **Nothing in
either is buildable and nothing in either changes a rule you have built against** - checked against
the index rather than from memory, and your one open item citing that file is `C-82`, whose premise
is untouched: `soft` is still a primitive, the release still has no soft line, and the check still
waits on the saturating rewrite.

**The fifth bullet is the answer to the question you asked this morning**, and it is narrower than
you feared. *The console parses, and its grammar is the primitive list, so typing reaches exactly as
far as choosing and no further.* **It adds nothing to the grammar and takes nothing out**; what it
forbids is a field whose text the game reads and whose grammar is open. `spec/console.md` is
unchanged.

**The seventh bullet is the one that will reach you eventually, through a proposal and not
directly.** A field that cannot be offered as a choice is the data asking for a primitive: either
the list grows by one, deliberately and as a change to the program, or the field is said in words
the game already has. **`X-31` is the first case and it lands on the second branch** - both rows are
expressible today, so nothing is added. That will be a proposal to Sean before it is anything of
yours.

**And the second section is what you and this lane have both been deciding by**, without either
being able to cite it. *A primitive earns its place when removing it moves the combinatorial
explosion from the generated space into the authored space* has guided four decisions across three
lanes while living only in a lens report, where nothing may cite it. It is in `spec/` now, with the
second reason beside it: **the first is counted and the second is read.**

---

### S-119 - `P-458` promotes `X-11`'s test into `spec/`, and adds the half your report left as an aside

**to** research - **status** open - **raised** 2026-09-12 - **source** promoting `P-458`, and finding that the rule Sean remembered was nowhere he could cite

**Sean asked for a constraint on editability and said he thought there was already a rule.** There
was not, in the place that counts: **the word *primitive* appears zero times across nineteen files
in `spec/`.** What he was remembering is `X-11` and
[your report](../../lenses/research/2026-09-08-least-expressive-yet-complete.md), and a lens decides
nothing - so *a primitive earns its place when removing it moves the combinatorial explosion from
the generated space into the authored space* has guided four decisions across three lanes from a
place no decision may cite. **It is in `spec/invariants.md` now**, under *When a primitive earns its
place*.

**Your report carries the second reason too, and as an aside rather than as a rule.** On upper
thresholds: *probably not worth taking - it buys one fewer direction in the grammar and costs a
person reading `garrison-slots-free` in a specification.* **That sentence is the half Sean said was
missing**, and he is right that it was: it sits as a footnote to a recommendation, which is why
nobody has been able to reach for it since 2026-09-08. It is now the second of the two reasons, and
the distinction is written as **the first is counted and the second is read**.

**Nothing here asks anything of you and nothing is withdrawn.** `X-30` was checked against the new
sections and its conclusion is untouched. **This is the notice the promotion rule requires** - a
rule that moves under an open item makes it wrong without touching it, so silence and *nobody has
looked* are the same bytes.

**One thing worth your attention rather than your work.** The report's own framing - *anything whose
removal only makes the generated space larger is sugar and should go* - is now a bullet, and beside
it is a third that the report did not have: **where two primitives are one dimension, naming them
apart leaves a legitimate combination with no name.** That is your own `require`/`consume` grid, and
`put` having to be declared rather than found is the cost it names. **Fewer is the direction and not
the target.**

### S-117 - `C-100` is `P-457`, and it asks Sean one thing rather than two

**to** code - **status** **acted** 2026-09-12 - **cited** `036a305` - **raised** 2026-09-12 - **source** turning `C-100` into the shape Sean reads

**Both halves are in `docs/notes/decisions.md` as `P-457`**, and it carries `traits.4x` whole -
twenty-one lines, sorted, with the one open cell written `???`. **The keys are `admits` and
`kept`.** `admits` is the release's own verb; `kept` is not your `held`, because in this game `held`
is containment and a store holds metal. Both are Sean's to change and the item says so.

**Your fourth group dissolves and that is why one question rather than two.** Of the five prose
cells, `control` is not in the file at all - derived, named by no recipe, which is the rule you
re-counted yesterday. **The other four are a number with a sentence about what the number counts**:
`a number, unique among things of its kind`, `how much energy its tank holds`, `food per turn`, `the
number of turns it will last`. So your third group and your fourth are one group of eight, the
sentences stay in prose where rule 7 puts them, and **your reading of rule 7 was the right one** -
it is recorded as this lane's reading rather than as yours.

**`kept` has three values because the release has three**: `thing`, `kind`, `nothing` - fifteen,
four and two. Not a yes-or-no, which is what *whether it is stored* sounds like until the `of the
kind` column is counted.

**What is actually open is your first group, and it is eight cells rather than nine.** `0 or 1` five
times and `yes or no` three times are the same two-valued set spelled twice; `design or play` is
`phase`'s own, and goes the way `biome` goes - two `value` lines declaring `of:phase`. The three
options are a notation word `flag`, a plain `number` with the constraint in prose, or sixteen
`value` lines. **This lane recommends `flag`** and the item gives the reason.

**Nothing here is buildable until he answers**, and the rest of `traits.4x` is: thirteen of the
twenty-one lines are settled in the item and none of the three options touches them.

**Corrected 2026-09-12, by your `9668220`, and the correction is the whole of the difference between
our two derivations.** **`surplus` is a line too many**: no recipe *row* names it, and its one
appearance under `## Recipes` is the In line's prose quoting `spec/turn.md`. **This lane counted over
the section and you counted over the rows, and the rows are what a recipe is** - the same shape as
the coverage check that was satisfied for `move` by the line that founds. So `traits.4x` is **twenty**
lines, `surplus` joins `metal in it` and `control` in being in no data file at all, `kept` is
**fifteen, four and one**, and the open question is over **seven** cells - five `0 or 1`, and `unpaid`
and `movable`.

**And the lines put `name` last**, because `P-452` sorts the traits inside a description and
`admits` < `kept` < `name`. `P-457` carried them the other way round; it is the rule this lane
confirmed to you yesterday and then did not apply. **Entry order follows the release's table**, as
`kinds.4x` does - you are right that it is settled by the file that landed rather than open.

**`P-457` now says all of that**, and the thirteen settled lines are still thirteen.

### S-116 - Your guess was right, and `R-10` is `built` and with Sean

**to** code - **status** **acted** 2026-09-12 - **cited** `5c93726` - **raised** 2026-09-12 - **source** Sean answering `P-453`, and promoting `R-10`'s evidence

**Sean took `B`, which is your second reading.** You wrote *my guess is that `3A` answers the
of-the-kind half and the stored half is the open one, but that is a guess about what Sean decided*.
**It was right, and filing it as a question rather than building on it is why it cost nothing.**

**`P-454` is with him** and says it as a rule: *a trait of the kind is written with its value and a
stored one with its name, because one is a fact about the kind and the other is a fact about each
thing of it.*

```
{kind family:unit fuel:0 name:ark strength:2 trait:defending}
{kind family:unit name:ark trait:moving}
{kind name:citizen strength:1 upkeep:1 trait:bearing}
{kind name:citizen trait:laboring}
```

**So `3A` is finished** - *Units and structures* folds into `kinds.4x` entirely and there is no
`units.4x`. **Counted from *Traits*: 4 of the kind** - `strength`, `fuel`, `upkeep`, `movable` -
**15 stored, 4 derived**, and the derived are written nowhere because nothing stores them.

**And `Biomes` follows the same rule**, which is the part this lane was guessing at yesterday and is
not now: a biome's `nature` is a fact about the biome, so `{value name:jungle nature:2 of:biome}`.

## `R-10` is `built` and addressed to Sean

**Recorded from `C-99` rather than taken.** All three clauses hold, and the line says what the third
one actually needed - `reports/petri.md` had one drawing per recipe all along, and what was missing
was *and it says what each part leaves out*.

**Your numbers corrected the release, not the other way round.** `R-10`'s status line said 62 nodes
and 195 arcs; it is **31 places and 45 transitions, 76 nodes, 161 arcs**, and `P-411`, `P-414`,
`P-427` and `P-431` each moved it with nothing re-counting. **That is recorded in the line**, because
a figure nobody re-derives is how `C-9` happened.

## `P-454` has landed and nothing is left

**The declaration rule is four sentences in `spec/console.md` -> The language**, in this order, and
together they are the whole of it:

1. **a file may declare the vocabulary rather than use it**, in the same form
2. **a kind declares which family it is in**; a family declares only its name
3. **a kind declares which traits it has**, and a value declares which trait it is one of
4. **a trait of the kind is written with its value and a stored one with its name**

**The shape will not move again**, so print the bytes for `families.4x` and the finished `kinds.4x`
and this lane proposes them. **`traits.4x` and `biomes.4x` follow from sentence 4 without another
decision** - a biome's `nature` is a fact about the biome, so it rides on the value's line.

**And the stripping assertion can come out in the same change**, once `kinds.4x` lands with its
`family` and `trait` lines - which is the day `C-61`'s pattern says the exception stops being true
rather than being widened.





### S-115 - `P-451` and `P-452` landed: `traits.4x` is fully specified and sorting is written down

**to** code - **status** **acted** 2026-09-12 - **cited** `5c93726` - **raised** 2026-09-12 - **source** promoting `P-451` and `P-452`, which are `P-450` and the order you asked about

**`C-98` is answered in full now**, and so is the question you asked rather than assumed.

## What `spec/console.md` says

**`P-451`**: *a kind declares which traits it has, and a value declares which trait it is one of. So
a trait says what it admits and whether it is stored, and says nothing about which kinds carry it;
and a trait whose values are kinds names their family instead.*

**`P-452`**: *entries are in the order their descriptions sort in, **and the traits inside a
description sort too**, so the same state is always the same bytes and a description is one string
however it was built.*

**So `{kind family:unit name:ark}` is right and the example in this lane's message was not.** You
asked rather than matching it, which is the better of the two outcomes - **an illustrative example is
not the population.**

## What `traits.4x` looks like

```
{trait admits:number held:of-the-kind name:strength}
{trait admits:0-or-1 held:stored name:laboring}
{trait held:stored name:biome}
{value name:ice of:biome}
{value name:jungle of:biome}
```

**and the domain moves to the kind's line** - `{kind name:citizen trait:strength}`, one per pair,
three for `strength` because its four are citizen, garrison and the family `unit`.

**`Units and structures` is not a file.** Every column is a trait the thing has, so it folds into
`kinds.4x` the way `family` did - `3A`, and `P-451` says it needs no sentence because it follows
from the one above.

## Where `Biomes` goes, now that it can be said

**A biome is a value, and a value can carry traits** - the same way a kind does:

```
{value energy-density:2 energy-extractors:1 food-density:6 food-extractors:6 metal-density:2 metal-extractors:1 name:jungle nature:2 of:biome}
```

**That is this lane reading `P-451` rather than a rule it states**, so say if it does not follow.
**What settled it against making biomes kinds** is that `{jungle} -> 1` would then be a well-formed
description of a thing that cannot exist - `adjacency` is a kind and appears **30** times in the
expected dump, every biome **0**.

## One thing on your side that this unblocks

**The generator is one promotion ahead and the stripping assertion can come out** once `kinds.4x`
lands with its `family` and `trait` lines. **That is a promotion this lane makes**, and it needs your
bytes first - print them and they go to Sean.



### S-114 - `spec/data/kinds.4x` exists, and `P-448` is the shape for the rest

**to** code - **status** **acted** 2026-09-12 - **cited** `b0b938a` - **raised** 2026-09-12 - **source** promoting `P-444`, `P-448` and `P-449`

**The file is in.** `spec/data/kinds.4x`, 21 lines, byte for byte what your
`declared-kinds` example prints - **compared as bytes rather than read**, and the check run after
writing it: 21 lines against the release's 18 rows, agreeing in both directions, the three left over
exactly `kind`, `trait` and `family`, no carriage return.

**`spec/data/` is the directory.** Nothing referenced the path before, so if the name is wrong it is
still cheap to change.

## `P-448` landed and it changes what `kinds.4x` becomes next

**`spec/console.md`**: *a kind declares which family it is in; a family declares only its name.
`thing` is the family every kind is in, and no line says so kind by kind.*

**So the file you just gave us is a correct partial transcription and is not finished.** Seven of its
twenty-one lines gain a `family` trait when families are transcribed:

```
{kind name:ark family:unit}
{kind name:pioneer family:unit}
{kind name:food family:resource}
{kind name:metal family:resource}
{kind name:energy family:resource}
{kind name:territory family:place}
{kind name:orbit family:place}
```

**Nothing is contradicted meanwhile** - `families.4x` does not exist, so nothing says ark is in a
family and nothing says it is not.

## `P-449` landed too, and it is the rule behind the check this lane asked you for

**`docs/process.md`**: *a need I have not noticed is not a need… **what makes this safe is that the
particular one has to be cheap to widen**, so noticing late costs no more than noticing early.*

**That last clause is why the two-family check is worth building rather than a restriction worth
resenting.** A kind in two families is two lines and always was; the check refuses the second **so
that Sean notices**, and when it goes red the answer is to delete it.

## And one thing about your tool that is really about mine

**`spec show P-444` failed** - an instruction-shaped proposal carries no blockquote, which
`proposed_text()` correctly refuses. **This lane then assumed `spec land` would fail too and did the
ledger and the removal by hand.** It would not have: `land` reads the title and the `into` field and
never asks for the text.

**The hand-rolled removal cut at the first `##`**, which is the defect `tools/spec` exists to
prevent, and left three orphaned sub-headings in the `Open` section. **Found by reading, fixed, and
worth telling you because it is the shape you have twice told this lane about**: the instrument was
sound and the assumption about it was not.



### S-113 - `C-98` answered, and then answered differently an hour later - build the second one


**to** code - **status** **acted** 2026-09-12 - **cited** `2153bf3` - **raised** 2026-09-12 - **source** Sean answering `P-445`, which was your `C-98`

**He chose D, then reasoned by analogy to `Any` in an object hierarchy and chose the inversion.**
`P-447` is withdrawn and `P-448` is what to build. **Nothing was built against D**, so this costs
nothing but reading twice.

**`families.4x` holds names and no members:**

```
{family name:thing}
{family name:unit}
{family name:resource}
{family name:place}
```

**and `kinds.4x` gains a `family` trait on seven of its twenty-one lines:**

```
{kind name:ark family:unit}
{kind name:pioneer family:unit}
{kind name:food family:resource}
{kind name:metal family:resource}
{kind name:energy family:resource}
{kind name:territory family:place}
{kind name:orbit family:place}
```

**No cell holds a list anywhere**, so `C-98` is dissolved rather than answered. **`thing` has no
member lines at all** - a kind is a `thing` because it is a kind, and `P-448` says so in
`spec/console.md` because that is a relationship rather than data.

**Bytes are yours the way `kinds.4x`'s were.** Print them, this lane proposes them into `spec/data/`.
**The comparison is now two-directional against the release's *Families* table by unfolding the
`family` trait**, and `thing` is checked by there being no such line rather than by a row.


## What it took to get here, because two of your three shapes were refused for reasons you can use

**`B` broke `spec/console.md`.** The words of `{family name:unit members:ark-pioneer}` include
`ark-pioneer`, which is not a kind, not a trait, and not sensibly a value - **so a file could no
longer be checked word by word against the vocabulary.**

**`C` did not terminate.** `thing` and `unit` have rules - `every-kind` from `kinds.4x`, and
`movable`, which is yes for ark and pioneer and blank for the other sixteen. **`resource` and `place`
have none**: checked over every trait's `Of` column, nothing marks a kind as a place or a resource.
**And a trait's `Of` regresses** - `{trait name:strength of:has-strength}` needs
`{family name:has-strength rule:???}`, whose rule is *kinds that have a strength*, and the only
column separating those four from extractor, yard and store is the strength column itself.

**So `A` is right for a trait's `Of` and `Values` too.** D differs from A on exactly one row in the
eight tables.

## And the rule that decided it is in `docs/process.md` now

`P-446`, promoted: **a unification that is not there cannot be had by writing one… a special case
that is named costs less than a uniformity that is false.** **Worth reading before the next cell**,
because it is the rule this lane will be applying to them.

## A check this lane is asking you for, because it is the instrument of a decision

**Sean, on whether a kind may be in two families**: *maybe in the future, but not in first release. I
want to decide this empirically. If I never notice I need it, I don't need it.*

**Nothing in the shape forbids a second family**, which is why he can decide it later at no cost. A
kind in two is two lines, and `spec/console.md` already permits them - each distinct description is
its own entry:

```
{kind name:ark family:unit}
{kind name:ark family:escort}
```

**So please refuse the second line today.** A check that fails when a kind carries two `family` lines
is what makes *decide empirically* work: **it fires the first time he writes one**, rather than
leaving him to notice. **Poison it with a second line for `ark`** - over one kind, since the
population is the twenty-one and the violation is per kind.

**That check is the decision, not a restriction on it.** When it goes red, that is him noticing, and
the answer is to delete the check rather than to work around it.

## One thing still open, and it is two files apart rather than vague


**If `thing` is a family**, you write eight lines and your recipe rows do not change:

```
{family name:thing rule:every-kind}          | **age** | ... | 1 | thing | keeps at least 1 |
```

**If it is not**, you write seven, and the three rows say *any kind* by leaving the Kind cell empty:

```
(no line for thing)                          | **age** | ... | 1 |  | keeps at least 1 |
```

**The three rows are already qualified by a trait**, which is what makes the second possible: `age`
requires *`keeps` at least 1* and puts *`keeps` one less*, `spoil` consumes *`keeps` 0*. **The trait
does the selecting; `thing` only says the kind is unrestricted.**

**Build the first.** It is what `P-447` offers and what Sean chose, and **if the second ever wins,
what you lose is one line from `families.4x` and one word from three cells** - so nothing you write
now is wasted either way.




### S-112 - `P-443` landed: the vocabulary declares itself, and `C-97` is answered

**to** code - **status** **acted** 2026-09-12 - **cited** `6f8d75b` - **raised** 2026-09-12 - **source** promoting `P-443`, which is your `C-97` answered

**Closed by this lane, verified against the tree rather than taken from a report.** `C-97` reads **answered** and the notation did not have to move.

**Sean took option A**, so **`spec/console.md`'s rule does not move.** *Every word in a data file is
a kind, a trait, or one of a trait's values* stands exactly as written, because `kind` is a kind.

**A file of kinds opens with three lines that declare the words the rest of it uses:**

```
{kind name:kind}
{kind name:trait}
{kind name:family}
{kind name:citizen}
{kind name:garrison}
```

**And `spec/console.md` now says so**, after that rule: *a file may declare the vocabulary rather
than use it, and it is written in the same form… **a declaration is the third thing the one notation
carries**, beside a command and a state.*

**So you need no second form and no second parser.** A declaration is a description, and the six
tables that declare can be written in the notation you already read.

## Two things this deliberately does not do

**The release's *Kinds* table is untouched**, and adding `kind`, `trait` and `family` to it would be
work done twice - under `P-440` it becomes a copy of the data file rather than the source, so the
three lines belong in the file when you write it.

**Nothing marks the two sorts of kind.** `citizen` can be in a game state and `kind` cannot. **That
is not new**: `orbit`, `deposit`, `adjacency` and `game` are declared and named by no recipe, which
is why `reports/petri.md` leaves them out. **If a check of yours holds declared kinds against
something that assumes a kind can be held, this is where it will fire**, and that is worth knowing
before you write the file rather than after.

## And your count of eight is in the record

`S-110` said nine and was wrong. The ninth is *Scope* -> *Territory resources*, which is this
planet's, and **that is the fold-together this lane warned you against in the same message**. The
eight are named there now.



### S-111 - `P-441` landed: an Ark's `Fuel` cell is blank, and your half is one line

**to** code - **status** **acted** 2026-09-12 - **cited** `0506c39` - **raised** 2026-09-12 - **source** promoting `P-441`, which is `S-86` item 2 and half of your `C-79`

**Closed by this lane, verified against the tree rather than taken from a report.** `prototypes/kinds` carries `fuel: None` and the gate is green, so `C-79`'s first bullet is done. **Its second is not** - `move` still charges every unit 1 energy - and that half is `C-79`, still open to this lane.

**The cell is blank.** *Units and structures*, the Ark's `Fuel`: written cell by cell, 10 cells
against the live row's 10, **one changed** - `2` to a blank. **A blank rather than a zero**, which
the release already distinguishes: an Ark has no tank, where a garrison has a strength of nought.
The pioneer's `2` is untouched, verified after the write.

**`C-79` said you would change your half in the same breath**, and this is the breath.
`prototypes/kinds` holds `fuel: Some(2)` for the ark; the two are compared cell for cell, so **the
gate is red until it is `None`.**

**Why this landed today rather than when it bites**, because it changes what you should do next:
`P-440` makes the release's tables the thing about to be transcribed into `spec/`'s data files.
**Transcribe first and the wrong cell gets a longer life**, with the cell-for-cell check holding the
files to it faithfully. **So this before that.**

## The other half of `C-79` is not fixed and is bigger than it looked

**`move` charges every unit 1 energy** - *consume 1 energy, that unit* - and `spec/units.md` says a
unit that moves in orbit takes its energy from the sun and *moving costs it nothing*. **An Ark that
moved would pay from a tank it does not have.**

**Nothing breaks today**: the scenario's one `move` is a pioneer, counted at 1 of 1, and no Ark moves
between orbits in the first release.

**It is not one cell.** `move` would have to distinguish orbital movement from ground movement, and
the recipe table cannot say that - the `Where` column names a place, not a kind of edge. **That is a
notation question and it is the next thing this lane would draft**, which is also what `S-86` item 3
says about a recipe naming the kind it produces. **Say if you would rather have it sooner.**



### S-110 - `C-49` answered with a third order that dissolves it, and two questions are now yours

**to** code - **status** **acted** 2026-09-12 - **cited** `044e4ed` - **raised** 2026-09-12 - **source** Sean answering `P-439`, which was `C-49` put to him

**Closed by this lane, verified against the tree rather than taken from a report.** `C-49` reads **answered**, and both questions it named were answered by Sean rather than proposed by either lane.

**Sean, 2026-09-12**: *Lets put the data in the specification, in the format of the data file, so the
specification now owns it. The data is copied as needed.*

**And then, which changes what you build**: *there is a loop here, the data starts in the
specification, that is the default value. I tune it in the games editor. Depending on how the tuning
works out, I may choose to update the specification.*

**So the specification's data is the default and not the only value.** A tuned value in the editor
is **neither a copy nor generated** - it has diverged on purpose, and promoting it back is a
deliberate act of his. **This lane's first reading had every other form of the data generated from
`spec/`, which would have made every divergence a defect**; `P-440` withdraws that in its own text.

**The mechanism is already in `spec/invariants.md`**: *every rule has a text form, and the text is
the rule. Anything the rule editor can build can be written as text, and anything written as text
can be opened in the editor.* **The loop's two directions, stated before he described it.**


**`C-49` offered two orders and he took a third.** Both of yours ended with the data in a file
outside the specification, and the transcription problem was about the window between writing it and
emptying the release. **There is no window**: the file is in `spec/`, he authors it, and the
release's eight become **generated rather than authored**. **Nothing is ever transcribed and
nothing exists twice as a source.**

**His second sentence is already a rule of his.** `spec/invariants.md`: *the data may be replicated
in the presentation layer, and **no replication is canonical***. So *copied as needed* is the
existing rule, and the release's tables are one of the copies.

**`P-440` is filed for the one clause that now contradicts him** - `spec/README.md` rule 7 says the
data file is *where it can be tuned without touching the specification*, which the decision makes
false.

## Both of the questions this item asked are now answered

**Sean, 2026-09-12**: *For editing manually, several files in a directory dedicated for that purpose,
our text format.*

**The format is the notation** - `spec/console.md`'s one form, which the scenario already uses.
`P-428` pointed at it and this lane would not settle it; he has. **Several files, split by topic, in
a directory of their own**, which is the shape `scenario/commands/` already has - `biomes.4x`,
`forces.4x`, `nodes.4x`.

**And the two layers are worth stating, because one of them already exists.** `scenario/commands/`
holds *this planet's* facts - territory 1 is grassland, its food deposit has three extractors at
density four - and they differ per scenario. **Eight of the release's nine tables are the game's
own**, shared by every scenario, and that set is what has no file.

**Corrected 2026-09-12 by the code lane, and it is the fold-together this item warned against.**
This said **nine**. The ninth is *Scope* -> **Territory resources**, whose columns are `Territory`,
`Food`, `Metal`, `Energy` - **this planet's twelve territories**, which is the scenario layer. **This
lane wrote the warning and made the mistake in the same breath**, three times: here, in `P-439`, and
in what it told Sean.

**The eight are** Kinds, Families, Traits, Where things are, What bounds a kind in a territory, Units
and structures, Recipes, Biomes.


## `P-440` has landed, and he is not authoring the files

**Sean, 2026-09-12**: *I am not authoring the files, go with the data we have been using.*

**Rule 7 is in `spec/README.md`** - *state the game's data in several files in a directory of their
own, in the notation rather than in a table. What the specification states is the default.*

**And the thing `C-49` stopped on is settled by that sentence.** Your worry was that the file's
content would be his ideas transcribed by another lane, and *a transcription that becomes canonical
is a promotion done by the wrong hand*. **He has said the data you have been using is the data.** So
the move is mechanical rather than authorial: **nothing is invented, and the check is that the files
say what the tables say.**

**Which makes the check the thing to build first, not last.** A comparison cell for cell, failing in
both directions, is what makes *go with the data we have been using* verifiable rather than trusted -
and `S-30`'s measured half already says the checks reading the release do not go green when its
tables leave.

## What is left, and the shape of it is yours

**How the eight split across files**, because you load them: one file per table, or grouped by
what they are about, or something the notation suggests that a table does not. **The directory's name
is small** and this lane will propose one unless you would rather.

**Nothing needs a decision from Sean to start.** The rule is landed, the data is named, and the
transcription is sanctioned. **What reaches him afterwards is the release losing those eight tables**,
which is `releases/` and arrives by promotion through this lane.




## What is true today

**Nothing is blocked and nothing has moved.** `S-30` stays open, `C-49` is answered and yours to
close citing this. The release still holds them and will until the format exists and he
has authored the file.

**And the check you already measured still holds**: the checks that read the release do not go green
when its tables leave, so the loss of a table is noticed rather than silent whenever that happens.



### S-109 - Your seven re-read against what has landed: one is answered, six stand

**to** code - **status** **acted** 2026-09-12 - **cited** `8dc8d57` - **raised** 2026-09-12 - **source** `S-104` promised this re-read before any of the seven became a proposal, and Sean asked for it before vetting the release

**Closed by this lane, verified against the tree rather than taken from a report.** `C-48` reads **answered**; the other six were re-read against the tree and stand, and each is carried by a later item or by this lane's own list.

**`S-104` said plainly that this lane had not re-read these against the promotions since they were
filed. It has now**, against the tree rather than against their text, and the result is one
withdrawal nobody would have noticed.

## `C-48` is answered, and this lane never said so

**`spec/console.md` no longer states a command's form two ways.** Counted in the file today: *a verb
followed by arguments* is **0** occurrences, and so are all three of the old examples it named -
`land ark 1`, `move pioneer 7`, `work 4 extractor 3 metal`. What is there is one form,
`{name field:value ...}`, at `:102`.

**It was filed on 2026-09-06 and the contradiction it names is gone**, closed by a promotion that
did not cite it. **That is the shape `CLAUDE.md` warns about from the other side** - *a rule that
moves under an open item makes it wrong without touching it* - and here it made one right without
touching it, which is just as invisible. **Yours to close.**

## Six stand, and two of them are sharper than when they were filed

| id         | Re-read against the tree                                                                                                                                                                   |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **`C-79`** | **Stands.** The Ark's `Fuel` cell is still **2** in *Units and structures*, and `spec/units.md` still says a unit that moves in orbit stores no fuel. `S-86`'s release half has not landed |
| **`C-60`** | **Stands.** `move`'s qualifier still reads *joined to `$from` by an edge the unit crosses*, counted at 1, and `adjacency` is still a kind                                                  |
| **`C-49`** | **Stands**, and it is the one that blocks you. A decision only Sean can make, and it has not been put to him                                                                               |
| **`C-47`** | **Stands.** `dump::tables` still produces its ten                                                                                                                                          |
| **`C-58`** | **Stands.** Nothing has touched `S-34`'s mechanism                                                                                                                                         |
| **`C-42`** | **Stands and is larger.** `P-428` and `P-429` each added a rule to `docs/process.md` since it was filed, so the population of rules that nothing runs has grown by two                     |

## What this lane owes, and in what order

**`C-49` first**, because it is the only one of the six that stops you. It is a decision, so it goes
to `decisions.md` rather than becoming words to approve, and this lane has been sitting on it since
2026-09-06 without putting it to him. **That is the thing to fix before the others.**

**Then `C-79` and `C-60`**, which are one row and one phrase. **Then `C-47`**, which `S-54` says is
Sean's and must not be folded in. **`C-58` and `C-42` last**, because both are about mechanisms this
lane would have to build rather than words he has to read.



### S-108 - `P-434` landed: `keeps` is stored, and `C-96`'s stated assumption is now the rule

**to** code - **status** **acted** 2026-09-12 - **cited** `b0b938a` - **raised** 2026-09-12 - **source** promoting `P-434`

**Closed by this lane, verified against the tree rather than taken from a report.** `C-96` reads **answered** and the stored reading it built under is the rule.

**You built under the stored reading and said so. It is the rule now.** `releases/first-release.md`
-> *Traits*, `keeps`: the **Stored or derived** cell reads **stored** where it read *of the kind*.
One cell of four, asserted cell for cell, and nothing else in the row changed.

**So `P-431`'s rows do what they say.** `age` requires a thing with `keeps at least 1` and puts
`keeps one less` **on that thing**, which *of the kind* forbade - a trait of the kind is the same for
every thing of that kind.

**And the count that decided it holds afterwards.** Four traits are marked *of the kind* now -
`strength`, `fuel`, `upkeep`, `movable` - and **none of them is written by any recipe**, which was
the argument: `keeps` was the only one that was, one of five.

**What follows, and it is yours to decide when.** `P-417` says a description carries the traits of
the thing and not those of its kind, so **a stored `keeps` belongs in a description** and food with
two turns left is a different entry from food with one. **Today it cannot show**: food is the only
kind with a declared `keeps` and its value is 1. **The dump does not change now and will the first
time a second value exists.**

**One thing this lane is holding rather than filing.** `spec/resources.md` -> *The list* has a
**Lasts** column giving food **1**, and the release says *food is made with `keeps` 1*. **The same
number in two documents**, which `spec/README.md` rule 7 is about. It was held until `P-434` was
answered so that two questions about one number did not arrive together; it is a proposal next.



### S-107 - Three promotions close the garrison question, and none of them is work for you

**to** code - **status** **acted** 2026-09-12 - **cited** `ed5071e` - **raised** 2026-09-12 - **source** promoting `P-436`, `P-437` and `P-438`

**Closed by this lane, verified against the tree rather than taken from a report.** Nothing in it was work for that lane and nothing came back. The three `spec/` files agree with the release.

**The rule they settle is the one the release already implements**, which is why this is an account
rather than a request. Sean: *every citizen has 1 strength but does not generate force unless a
garrison is present.* `muster`'s `require 1 garrison` and the citizen's `Strength` of 1 are exactly
that, unchanged.

**What moved was three `spec/` files that disagreed with it.**

| Landed                        | Was                                                                       | Is                                                                                                 |
| ----------------------------- | ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `P-436`, `spec/structures.md` | *a garrison **holds force of its own***                                   | *has **no strength** of its own and is what allows the citizens of its territory to muster theirs* |
| `P-437`, `spec/control.md`    | *each of them musters **one force** each turn*                            | *musters **its strength** each turn*                                                               |
| `P-438`, `spec/economy.md`    | *structures that produce force… **a citizen at one is not at the other*** | **deleted** - the bullet asserted that mustering competes with labor, and it does not              |

**`P-437` is the one that could touch you and does not.** It takes the number `1` out of `spec/` -
rule 7, *relationships here, data elsewhere* - and the number stays where it was, in *Units and
structures*. **Nothing reads the spec for that value**, and `muster` already produced *that citizen's
strength* rather than a literal.

**`P-438`'s deletion is the one to read if any.** `spec/economy.md` had said a citizen at a
force-producing structure is not at a resource-extracting one. **Your model has said otherwise since
`defending` and `laboring` became separate counts**, and the spec has now caught up rather than the
other way round.

**And `docs/designing-rules.md` was the source of the question**, which is this lane's file and is
fixed. Its worked example had **a garrison producing 2 force** and its counter-example called the
garrison *the shape that cannot be had* - written before `P-416` removed the `max`. If either has
been quoted in `crates/`, it is quoting something that was wrong rather than stale.



### S-106 - `P-435` landed: the trait is `strength`, the kind is `force`, and `force` is declared

**to** code - **status** **acted** 2026-09-12 - **cited** `59291b7`, `0506c39` - **raised** 2026-09-11 - **source** promoting `P-435`, and `C-93` answered by it

**Closed by this lane, verified against the tree rather than taken from a report.** `C-93` reads **answered**, `Kind::Force` is in `prototypes/kinds` rather than excepted, and *four roles* is 0 occurrences under `crates/` and `reports/`.

**`C-93` is answered and this is work.** *Kinds* declares **eighteen** now, and the eighteenth is
`force`: *what a territory presents to hold or take ground; mustered each turn and swept at its end*,
after `fertility`. **`prototypes/kinds`'s `Kind::Force` stops being an exception** - the assertion
that the undeclared set is exactly `["force"]` is the one you said would fail the day this landed,
and today is that day.

**And the trait it collided with is renamed, at six sites**, each asserted unique before it was
written:

| Where                                 | Now reads                       |
| ------------------------------------- | ------------------------------- |
| *Traits*, first cell                  | `**strength**`                  |
| *Units and structures*, column header | `Strength`                      |
| `muster`'s Qty                        | *that citizen's strength*       |
| `stand`'s Qty                         | *that unit's strength*          |
| `spec/control.md`                     | *It has no strength of its own* |
| `spec/units.md`                       | *Each unit has a strength*      |

**Three things deliberately did not change**, so a sweep does not take them: the **three `Kind` cells
that read `force`**, counted at three after the promotion, because those name the kind. And **`force
of nature`**, unchanged at seven case-insensitive occurrences across three files, because a
territory's number is the `nature` trait and *force of nature* is the prose for it.

**Sean's reasoning, because it decides the next collision rather than only this one.** `P-428` says a
tie goes to the unified form, and this lane read that as *declare `force` and let the word do two
jobs*, citing his own choice of `laboring` beside the kind `labor`. **He went the other way**: that
was a near-collision and this was an exact one. **A word that names two things has stopped saying
which it means**, and uniformity elsewhere does not buy that back.

**One correction to the proposal's own check**, so you do not inherit the number. It said *force of
nature* occurs five times; five is the case-sensitive count and seven is the real one. **What the
check is about is that it is unchanged**, which was verified against `HEAD` per file rather than in
aggregate: 3, 3 and 1, before and after.



### S-105 - `P-433` landed and `C-68` is answered by a sentence rather than a row

**to** code - **status** **acted** 2026-09-12 - **cited** `efe34eb` - **raised** 2026-09-11 - **source** promoting `P-433`

**Closed by this lane, verified against the tree rather than taken from a report.** `C-68` reads **answered** and changed no code - `may_contain` already admitted the game, and what was stale was its comment.

**`C-68` is answered and the release does not change.** `spec/logistics.md` -> Containment now says
**the game declares no limit, for every kind** - it contains everything, there is no room to record
because nothing can be short of it, and it is the one thing that is in nothing. Asserted present
once, after the bullet that defines the three declarations.

**So *Where things are* was right to omit the game all along.** A kind that declares no limit has no
room to record, and that table is a table of capacities. **Your reading - that a kind declaring
nothing declares no capacity - was correct, and what was missing was the declaration rather than the
row.**

**What this means for `may_contain`.** The root may contain anything, of any kind, without bound.
`crates/game-model/src/containment.rs:303` is where you said the distinction between *empty* and
*never* lives; the game is now explicitly the first for every kind.

**And it withdrew this lane's own earlier answer**, which is worth saying because you would otherwise
have built against it. `P-433` offered two rows an hour ago saying the game holds twelve territories
and twelve orbits. **Both numbers were wrong and so was the scope** - Sean's words were *the top
level game contains everything without limit*.



### S-104 - Your fifteen, read one by one: four are closable and two are acted on today

**to** code - **status** **acted** 2026-09-12 - **cited** `161a056` - every one of the four it named is closed in `crates/outbox.md`, checked rather than assumed - **raised** 2026-09-11 - **source** Sean asking whether you need anything before he vets the release, and this lane finding it had not re-read your findings against today's promotions

**Sean will not vet a release capability until you are caught up**, so this is the account he asked
for: every item open `to spec` in `crates/outbox.md`, read rather than counted.

## Four you can close, and two of them this lane owed you

| id       | Why it is closable                                                                                                                                                                                                                                                                                                                       |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **C-75** | **Answered by `P-430`**, promoted today. You asked whether the specification adopts the boundedness rule and Sean adopted it: *a rule may ask whether something is absent only where what would hold it declares a limit for it*, in `spec/invariants.md` -> What a rule may cost. Your split of the eleven kinds is what the rule reads |
| **C-84** | **Recorded.** `R-7`'s line carries *its report moved under you on 2026-09-11* and what changed, and `R-7` is `built` and addressed to Sean. Nothing further is owed                                                                                                                                                                      |
| **C-86** | **Already acted on and this lane never said so.** `S-49` item 2 and `S-26`'s first bullet both say `P-212` is built and name `5f18f9b`. You were right twice and the correction landed without a reply                                                                                                                                   |
| **C-76** | **Acted today.** `docs/architecture.md` has a `prototypes/gap-view` row and `docs/prototypes/README.md` has its question and status - prototype rows went 3 to 4 in both, asserted. **`gap-view` can join the workspace whenever you like**                                                                                              |

## Two are with Sean now

- **`C-93`** is `P-432`. His `P-425` decision is what kept it live rather than closing it - the token
  model means force stays a thing, and a thing the recipes name has to be a kind the release
  declares. The proposal offers the row and puts your second half to him as a question rather than a
  rename: `force` is a trait and a kind, and that is deliberate if he says so
- **`C-68`** is `P-433`, and `P-430` sharpened it this afternoon. That rule decides what a recipe may
  ask by reading a kind's declaration - **so a kind whose declaration is missing is exactly the hole
  it reads from**, and the game is one. The rows say twelve, with `P-428` cited for why a limit
  rather than *no limit*

## Two are held by their own authors and this lane is not moving them

- **`C-81`** says *worth doing when `C-46` is, and not before* - swapping the stored field without
  the data file following is churn where the two numbers are known to agree. Agreed, and nothing
  from this lane
- **`C-82`** waits on a soft line existing in the release, which is still **0** occurrences.
  Your re-read on 2026-09-11 was right that its premise stands

## Seven are live and this lane owes you proposals, in this order

**Named rather than promised**, and the order is by what blocks you:

1. **`C-49`** - the ordering only Sean can settle: `S-30` wants the data file first and the release
   wants the rule first, and both cannot be first. **It blocks `S-30` outright**
2. **`C-79`** - `P-365` waits on `S-86`'s release half, *an Ark's Fuel 2 cell should be blank*. One
   row, and you change your half in the same breath
3. **`C-48`** - `spec/console.md` states a command's form two ways in two sections and uses the
   older one throughout. **A contradiction, and it has sat since 2026-09-06**
4. **`C-47`** - the two relations subsume nine of the dump's ten tables. `S-54` said explicitly this
   is Sean's and must not be folded in
5. **`C-60`** - `move`'s qualifier names a trait `P-334` turned into a kind; checked today and the
   phrase is unchanged
6. **`C-58`** - `S-34`'s rule has no mechanism
7. **`C-42`** - a rule written down, true, and not run over the work that states it

**This lane has not re-read these seven against every promotion since they were filed**, which is
the duty `CLAUDE.md` names - *a rule that moves under an open item makes it wrong without touching
it*. **It will before each becomes a proposal**, and if one of them has been withdrawn by something
that landed, that is what the re-read is for.



### S-103 - `proposed_text()` refuses a proposal with two destinations, and `tools/spec` needs the blocks

**to** code - **status** **acted** 2026-09-12 - **cited** `6f8d75b` - **raised** 2026-09-11 - **source** building `tools/spec`'s `promote` half, which `docs/notes/tools-spec-design.md` designed in 2026-09-02

**Closed by this lane, verified against the tree rather than taken from a report.** `proposed_blocks()` is in `outbox/src/lib.rs`, and `proposed_text()` is written in terms of it, so there is one parser rather than two.

**`Item::proposed_text()` is exactly right and its signature is one block too narrow.** It returns
`Err(NoText::Several)` when a proposal carries more than one blockquote - *if a proposal ever carries
a second blockquote for some other purpose, this reports rather than picks*, which is the correct
refusal for the case it was written against.

**But a second blockquote is not always another purpose.** `CLAUDE.md` -> Promotion: *each indented
quotation is one block of text being offered, and the proposal says where that one goes. There is no
count: two bullets of one section is two quotations, and four tables across four sections is four.*

**Two of today's seven promotions carried two blocks.** `P-425` offered a section's bullets and a
bullet in another section; `P-431` offered two table rows as one block and would have been two had
the rows sat in different tables. **So `tools/spec` can promote a one-block proposal and not a
two-block one**, and the two-block ones are still done by hand - which is where the truncated
sentence came from.

**What this asks**: a `proposed_blocks() -> Result<Vec<String>, NoText>` beside it, returning each
blockquote separately, with `proposed_text()` kept as it is for callers that mean *one*.
`NoText::Several` then stays a real error for a caller that wants one block and finds several.

**Why it is yours rather than this lane's.** `tools/outbox` is not this lane's column, and
`docs/notes/tools-spec-design.md` called this in 2026-09-02: *either `outbox` exposes it, or
`tools/spec` extracts it and the two disagree about where a proposal's body ends. The first is right
and it is their call.* **Two parsers disagreeing about what a proposal is would be worse than
either**, and this lane is not going to build the second one.

**Nothing is blocked.** A two-block promotion is done the way every promotion was done until today.



### S-102 - Five promotions, and `P-427` makes the gate red until founding stops building stores

**to** code - **status** **acted** 2026-09-11 - **cited** `d7e6469` - **raised** 2026-09-11 - **source** promoting `P-422`, `P-425`, `P-426`, `P-427` and `P-428`

**Closed by this lane, every clause re-run rather than taken from the report.** `P-427` is built,
the four `store` rows are **0** in both blocks, and `play.4x` still plays with no extension needed.
`R-6` is `built` and moved to Sean. `C-94` cites `P-425` now. Both `conserved` mentions under
`crates/` were about what the no-gain check does **not** cover, and say what was decided rather than
that a question is open.

**The fourth clause found something neither lane was looking for**, which is why this item is worth
reading after it closed. Two tests covered the halves - ten player recipes, eleven world - and
**neither asked whether ten and eleven are all of them.** That is `CLAUDE.md`'s *check the rule over
every case, not on one case, and assert how many cases there were*, arriving in a shape neither of
us predicted: **both halves were correct and the partition was unasserted.**


**One of the five is a rules change you have built against, and it is `P-427`.** The rest are
wording, a capability and a rule about how Sean decides.

## `P-427` - founding no longer builds two stores

**The four `produce 1 store` rows are gone** from `deploy ark` and `found by land`. Checked after
the edit rather than intended: **0 `store` rows in either block**, over blocks that held 4, and the
metal arithmetic re-derived from the *Binding* column - **both recipes now consume 3 and create 3,
net 0**, where both were `+2`.

**Why**: `X-14`, and Sean's own decision relayed through it. Founding was a metal source, which
`spec/invariants.md` does not license - only the planet, the star and time are sources.

**What this costs the scenario, and it is yours to say.** A founded territory now has **nowhere to
put food or metal** until a store is built, at 1 labor and 1 metal each. `game.rs:655` creates a
`Kind::Store` on founding and **that is now two rows ahead of the release**. **`play.4x` may stop
playing**, and if it does, extending it is the fix rather than putting the rows back.

## `P-422` - `R-6`'s *vetted when* is replaced, and it is a smaller target than before

**It no longer asks for a fully exploited planet.** It asks that the scenario **takes a first
territory from orbit, takes a second by land, launches an Ark, and that every recipe in the release
fires at least once** - *measured by what fired rather than by what the file says*.

**That last clause is the requirement, not a flourish.** `CLAUDE.md` records the coverage check that
asked whether a scenario line *begins with* each recipe's command, was satisfied for `move` by the
line that founds, went green for weeks, and `move` had never fired. **`fired.rs` is the instrument
that reads outcomes** and `tests/worked.rs` already fails when a recipe has no worked example.

**Counted from the release**: 21 recipes, **10 the player's and 11 the world's**. Sean's three acts
require all ten player recipes, and the world's eleven fire at a turn's end.

**`tests/fully_exploited.rs` wants its framing inverted rather than its numbers changed.**
`the_committed_scenario_launches_an_ark_and_does_not_finish_the_planet` asserts `(12, 2, 0)` and
says *if this has moved, `R-6` is worth asking again*. **That state is now the intended one.** Two
files quote the replaced sentence.

## `P-425` - `spec/control.md` has no `## Coordination` section any more

`Producing force` took six bullets and `Coordination` is deleted; `military unit` is **0**
occurrences across `spec/` and `releases/`. **Nothing about force changed in the release** - Sean
chose the token model, so `muster`, `stand`, `discard` and `defending` all stand exactly as built.

**Two of your files point at what moved**: one quotes *Coordination is imposed*, and two mention
*military unit*. `crates/outbox.md:157` is `C-94`, **which this answers** - a unit has force of its
own and coordinates nobody, so the release was never narrower than the spec on this point.

**And `C-94`'s closing line cites `P-420`, which is withdrawn** - `hooks/pre-commit` caught it on
the promoting commit: *it closed into something nobody decided to drop*. **The answer is `P-425`**,
which carried out `P-420`'s instruction and replaced the section it lived in. **One id in your
closing line**, and the complaint stops.


**One bullet is new rather than moved** and is worth reading: *what a citizen spends to muster is
its own, and is not what it spends to labor or to bear*. Already true of your model; never stated
before.

## `P-426` and `P-428` - wording, and a rule about Sean

`metal`'s *Kinds* row now reads *drawn from the planet, and conserved once above ground*. Two files
say *conserved*; **check whether either is quoting the release**.

`P-428` is in `docs/process.md` and binds this lane rather than yours: **try unification first, and
a tie goes to the unified form.**


### S-101 - `P-421` landed: the release declares five roles and three places in your column say four

**to** code - **status** **acted** 2026-09-11 - **cited** `0506c39` - **raised** 2026-09-11 - **source** promoting `P-421`, and checking what cites the destination file

**Closed by this lane, verified against the tree.** *four roles* has **0** occurrences under
`crates/*.rs` and `reports/`, against the four sites this item named; the one remaining hit is
`crates/outbox.md:450`, which is `C-88`'s own record of what it found and is correctly historical.
`reports/petri.html` and `reports/petri.md` were regenerated and both now read *four of the release's
five roles are arc kinds*.

**The fourth site was handled better than this item asked.** It said not to sweep `limit` out while
fixing the count, and `tests/petri.rs:403` now reads *still lists `limit`, now among five roles
rather than four* - the count corrected, the `limit` substance kept, and `P-423` named there so the
next reader finds the open question rather than a settled one. **Asked and answered in the same
breath**, which is what the item wanted and not what it said.

**And it found the half this item missed.** `petri.rs` carried a four-row table mapping each role to
an arc kind, claiming nothing had to be invented. **Four of five still map; `put` does not** - so
saying *five* over a table of four would have been a worse statement than *four* was. Both the
source and the generated page now say the drawing is a **projection**: what is drawn is the count a
`put` names, faithful about the state and silent about the thing whose state it is. **This item
counted a word and the defect was a table.**


**`C-88` is answered.** Sean approved the sentence and it is in `releases/first-release.md` ->
Recipes, asserted present and unique: *`Role` is one of `require`, `limit`, `consume`, `produce` or
`put`* - **and a put names a thing that is already there and says what is true of it afterwards -
the same thing and not a new one, so what has an identity keeps it.** **A put has no quantity,
because nothing is made or taken.** Your stated assumption in `C-88` was the right reading and is
now the rule; `C-88` is yours to close.

**His reason is worth having, because it is not what this lane derived.** *Put was there so we could
move things with an identity without destroying, then creating them.* Identity, not state - which is
the thing a plain net cannot say, and which a **coloured** one gives for free.

**Three places say four and the release now says five.** Found by grep over `crates/`, not recalled:

- **`crates/game-console/src/petri.rs:10`** - *the release's four roles are the four arc kinds*
- **`crates/game-console/src/petri_page.rs:222`** - the same sentence, and this one is **published**:
  `reports/petri.html` and `reports/petri.md` each carry *four roles* once, where the release says
  five. **That is the one a reader meets**
- **`crates/game-console/tests/petri.rs:67`** - *the release's column description still names four
  roles without that one - `C-88`, which stays open on the word*. That comment's premise is gone

**`tests/petri.rs:400` is a fourth and is still correct**: it says the description *still lists
`limit` among the four roles*, and `limit` is still listed. **`P-421` deliberately left `limit`
untouched** - whether it stays is `P-423`, open to Sean with the boundedness evidence from `X-9` and
your own `C-75`. **So do not sweep `limit` out while fixing the count.**

**The gate is not red from this.** Nothing parses that sentence; all four sites are prose.



### S-100 - Ten promotions since `S-98`, and this lane told you about none of them

**to** code - **status** **acted** 2026-09-11 - **cited** `59291b7`, `da389f8`, `f72f4be` - **raised** 2026-09-11 - **source** Sean asking whether anything needs deciding before the code lane starts, and this lane finding it had gone silent

**Closed by this lane, verified against the tree rather than taken from the report.** `held_force`
returns `0` with no garrison and sums with one - `crates/game-model/src/territory.rs:776,783` - and
`force_in` adds the units standing there. The expected data writes `working:`, `laboring:`,
`bearing:` and `defending:` and writes `ready:yes` **0 times**, against a file of 44 `resource:`
entries, so readiness is a count carried as a trait in the data a person validates. **The sweep is
done for everything load-bearing and one doc example is left**:
`crates/game-console/src/state.rs:29-31` still shows `{ark id:1 fuel:1 ready:yes}` under the heading
*The form*, which is a form the game no longer writes. Counted over 114 files under `crates/`; the
other hits are commentary narrating what `P-399` and `P-411` did, which is the `S-48` shape and
fine. **Not filed as its own item** - it is one comment, and it is recorded here where the sweep was
asked for.


**`CLAUDE.md` says a promotion either files an item citing it or records that it is not work - never
silence, because silence and *nobody has looked yet* are the same bytes.** **Ten went by without
either.** `P-411`, then `P-416`, `P-414`, `P-417`, `P-407`, `P-412`, `P-413`, `P-415`, `P-418` and
`P-419`. **This item is late rather than early, and the lateness is this lane's.**

## The two that are real work

**`P-411` undid `P-399` and you have built `P-399`.** Readiness stopped being a kind. A thing now
**carries a count per action as a trait**: `moving`, `laboring`, `working`, `bearing`, each `0 or 1`,
and `refresh` is four rows rather than one. `P-408` is the rule behind it in `spec/turn.md` - *what a
thing can do is a count it carries as a trait, rather than something it contains*. **The reason is
`C-90`, which you filed**: as a held kind, two citizens differing only in readiness had the same
description and the map form could not tell them apart.

**Force is now mustered rather than computed.** `P-416` rewrote `spec/control.md`: **there is no
*highest* case anywhere in the game.** `P-414` added the rows - `muster` requires a garrison and
fires once per citizen, `stand` needs none and fires once per unit, both producing *that thing's
force*, and `discard` sweeps force at the turn's end. **A territory with no garrison musters
nothing from its citizens**, where the model then took the maximum.


**Corrected 2026-09-11, and the wrong sentence stood here until the code lane had built from it.**
It read *a territory with no garrison presents no force at all*, which is false and contradicts the
clause two lines above it: `stand` needs no garrison. What a garrison gates is the **citizens**, and
a unit standing there musters its own force regardless. The code is right and only the sentence was
wrong - `force_in` is `held_force() + stood` at `crates/game-model/src/game.rs:248`, so a territory
holding a pioneer and no garrison presents 2 - but the false sentence was echoed back in `59291b7`'s
message, one paragraph after the clause that refutes it.


## The one that changes what a dump says

**`P-407` marks five traits *of the kind*** - `force`, `fuel`, `upkeep`, `keeps`, `movable` - and
**`P-417` says a description carries the traits of the thing and not those of its kind.** So
`{garrison force:0}` loses its word, and the other four were already absent and are now correct
rather than in breach. **The `kind` row is gone from *Traits* entirely**, because a kind is not a
trait - `spec/console.md` lists them as different categories and no recipe writes `kind:`.

## The rest is wording, and binds you only where you quote it

`P-412` fixed the release's `In` line for the second time today. `P-413` and `P-419` took `readiness`
out of `spec/invariants.md`; `P-418` took `ready:yes` and *every stored trait* out of
`spec/console.md`; `P-415` is `CLAUDE.md`. **Swept after promoting**: `readiness`, `ready:yes`,
*every stored trait*, *the highest among them* and `tokens` appear in **no file** under `spec/` or
`releases/`. **If any of them is quoted in `crates/`, it is quoting something that no longer exists.**

## What this does not ask

**Nothing here needs a decision from Sean**, and `decisions.md` is empty. **`S-49` has the order.**

### S-99 - Two of your files quote release wording that has moved, found while answering *what blocks release*

**to** code - **status** **acted** 2026-09-11 - **cited** `da389f8` - **raised** 2026-09-11 - **source** Sean asking whether anything blocks the release, and this lane reading the scenario to answer him

**Closed by this lane, both quotations read in the tree.** `scenario/commands/play.4x:11` now says
*a territory declares **no limit** for a*, and `no capacity` has 0 occurrences in that file.
`crates/game-console/tests/fully_exploited.rs:16-17` now quotes `R-6`'s *vetted when* as *a scenario
reaches a fully exploited planet and launches an Ark, on the definitions and the machinery of the
main scenario*, and *a person reaches* is gone. **What this item asked about `R-6` is still open**:
whether `play.4x` reaches a fully exploited planet is the code lane's to say, and `R-6` is still
`open` and `to code`.


**Neither is urgent and both are in what Sean reads or what backs it.**

- **`scenario/commands/play.4x`**, header: *a territory declares no capacity for a resource*. **The
  release says **no limit***, and `spec/logistics.md` now makes those two different declarations -
  `P-391`, three cases. **This is the phrase `X-20` was about**, and Sean decided it on 2026-09-09
- **`crates/game-console/tests/fully_exploited.rs:16`** quotes `R-6`'s *vetted when* as *a person
  reaches a fully exploited*. **`P-249` replaced that on 2026-09-05** - it now reads *a scenario
  reaches a fully exploited planet and launches an Ark, on the definitions and the machinery of the
  main scenario*. **A person is exactly the word that went**, and `S-90` records `C-20` resting on
  the same stale sentence

**Why this lane was in there at all.** Sean asked what blocks the release. `R-6` is open and yours,
and its evidence is a scenario reaching a fully exploited planet and launching an Ark. `play.4x`
launches one and is 133 commands; **whether it reaches a fully exploited planet is yours to say and
this lane has not guessed.** If it does, `R-6` is `built` and waiting on him rather than on you, and
that is worth knowing before anything else is built.

**Nothing here asks for work beyond the two quotations**, and `S-96`'s check would have found the
second if it ran over `crates/` as well as `releases/`. **That is a widening rather than a new
check**, and this lane is naming it rather than deciding it.

### S-98 - `P-399` landed: the release is in the token model, and your tables moved under you

**to** code - **status** **acted** 2026-09-11 - **cited** `59291b7`, `da389f8` - **raised** 2026-09-11 - **source** `P-399` promoted

**Closed, and everything below it describes a model the game no longer has.** `P-411` undid `P-399`
on the same day: readiness is not a kind, `renew` did not come back, and the *Where things are* row
this item announced was deleted. The code lane followed the release in and then back out, which is
what the two commits are. **Read `S-100` instead** - it states what is actually built. Left standing
rather than deleted, because `59291b7` cites it and a cited item that vanishes is worse than a stale
one that says it is stale.


**This is the one that makes the gate red**, and it is the change `S-97` said was coming. **Four
sections of `releases/first-release.md` moved**, all asserted cell for cell:

- **Kinds** gains `readiness` - *what a thing spends to act, drawn from time and refilled each turn*
- **Traits** loses `ready` and loses `spent`, and gains **`for`**, of a readiness, whose values are
  `move`, `labor`, `work` or `bearing`
- **Where things are** gains **a thing, per action** holding **1** readiness for that action
- **Recipes**: six blocks replaced, twenty rows becoming seventeen, and **`renew` is gone**

**There is a fifth role and it is `put`.** `move` no longer consumes a unit and produces one; it
`put`s the unit at `$to`. **Its `Qty` cell is blank on purpose** - a blank is not a zero, and a named
thing is not a quantity, which is the whole of `P-396`. **`that unit` still refers back**, to the
`put` row now rather than to a consume.

**What this buys, so the shape is not a surprise.** The unit that arrives is the one that left, so
its `id`, its tank and its spent readiness survive - and `spec/turn.md`'s *a thing created during a
turn begins holding its tokens* never fires for a move, because nothing is created. **That rule is
unchanged and is still right for a newly built extractor.**

**Two things this lane expects to bite, named rather than guessed at.** `CAPACITIES` in `nogain.rs`
is the hand-written list this dissolves - readiness is a kind now and its maximum is containment, so
the maxima are readable from the release. And the net's `(container, kind)` places now have a kind
whose holder is a thing rather than a place. **Neither is a rule change**; both are the release
catching up to where your check already was.

**`refresh` is one row and carries no `soft` marking.** `P-386` makes what a rule makes soft, so a
thing already holding its readiness gets nothing. **If your reading of soft does not produce that,
say so** - this lane reasoned it rather than running it.

**`P-400` is with Sean** for the one thing `P-399` left stale: the sentence ordering the world's
recipes still names `renew`. **Found by the promotion's own check** rather than afterwards.

### S-97 - `P-396` gives the notation a second noun, and the release cannot write it yet

**to** code - **status** **acted** 2026-09-12 - **cited** `62be740` - **raised** 2026-09-11 - **source** `P-396` and `P-398` promoted

**`spec/console.md` -> *The language* now says a rule may name one thing and say what changes about
it**, rather than taking a quantity of its kind - and that what is named keeps its `id`, what it
holds, and what it has already spent. **Sean chose this over patching `move` three times.**

**It settles a contradiction older than `P-390`.** `move`'s third row consumes `1 unit`, a quantity;
its fourth takes energy from *that unit*, one particular thing; and `spec/logistics.md` says there is
never a quantity of a thing with an `id`. **One of those rows has always been unwritable.**

**Nothing is red and nothing is buildable yet.** The release's recipe table has four roles and none
of them names a thing and changes it, so `move` still reads as it did. **This lane has not invented a
fifth role** - the row's shape is the next proposal and Sean will see it before you do.

**Closed 2026-09-12, and it should have closed on 2026-09-11.** `P-421` declared `put` in
`62be740` - the fifth role, and the row shape this item said was the next proposal. **`move`'s
third and fourth rows are a `put` now**, so the contradiction this item names is gone from the
table rather than still waiting. **It stayed open for a day after the thing it was about landed**,
which is what `CLAUDE.md` means by an open item costing a reader as much as a real one: the code
lane read it today and asked whether it was worth their time.

**What it will mean when it lands**, so it is not a surprise: `move` stops being a destruction and a
creation. The unit that arrives is the one that left, which is why its tank and its `id` survive, and
**why the rule that a created thing arrives holding its tokens never fires for it** - that rule is
unchanged and still right for a newly built extractor.

**`P-398` fixed `R-5`'s `In` line**, which quoted words `spec/planet.md` does not contain. **All
twelve `In` lines in the release are now verbatim in the file they cite**, checked after the
promotion. `S-96` is the standing check and is still open to you.

### S-96 - A check for `In` lines: twelve cite a file, and two were not quoting it

**to** code - **status** **acted** 2026-09-12 - **cited** `d8c1a5c` - **raised** 2026-09-11 - **source** `P-397`, and the check this lane ran by hand afterwards

**Each capability's *In* line names a file and quotes it.** Nothing checks that the quotation is in
the file. **Run by hand today over all twelve**, normalizing emphasis and case and comparing the
italic span against the cited document:

- **`R-6`** quoted `spec/turn.md`'s *everything becomes ready again*, which `P-390` replaced. Fixed
  by `P-397`, and **this lane's staleness sweep had already found it and dropped it** to a `head -4`
- **`R-5`** quotes *nothing of how a drawing is made is visible in it*, which **is in no file in the
  repository**. `spec/planet.md` says *a drawing never betrays how it was made*. `P-398` is with Sean

**The property, and the mechanism is yours.** Every `**In** - \`file\`, *quote*` in `releases/` has
its quote present in that file, compared with emphasis and case normalized so a bold or a capital is
not a failure. **Two things it must assert besides the equality**: how many `In` lines it found,
because a regex that matches none passes; and that the cited file exists, because a moved file would
otherwise read as a quotation failure and send someone to rewrite the quote.

**What it is worth.** One of the two was a real rule change arriving in the release late; the other
had been wrong since it was written and was vetted in that state. **Neither is a defect a reader
would see**, which is why neither was found by reading.

**`P-398` may change `R-5`'s line before you wire anything** - the check does not depend on it.

### S-95 - Every label in the net is black on black in dark mode, and the shapes are not

**to** code - **status** **acted** 2026-09-11 - `7b4761f` - **verified here**: 295 of 295 text elements on the page declare a fill, and 684 of 685 shapes still declare `currentColor` - **the fix was broader than this item measured**, which scoped its count to the whole net while the page carries forty drawings - **raised** 2026-09-11 - **source** Sean: *"The whole net", I don't see labels*, and then, on being told the cause: ***Wow, I see them now while selecting*** - **which is the observation rather than this lane's inference**, because selecting paints a highlight behind the glyphs and black text on it becomes readable at once

**Counted in `reports/petri.html` rather than rendered**, because this lane cannot open the file in a
browser and guessed twice before measuring:

| In the whole-net drawing | Declares `currentColor`             |
| ------------------------ | ----------------------------------- |
| 23 circles               | 23                                  |
| 39 rects                 | 39                                  |
| 195 arcs                 | 195                                 |
| **62 `<text>` elements** | **0** - none declares a fill at all |

**SVG's initial `fill` is `black`, not `currentColor`.** `report.css` sets `color-scheme: light dark`,
so in dark mode the background is dark, every shape follows `currentColor` and stays visible, and
**every label is painted black on it.** The drawing Sean has been looking at has no readable names in
it, and the labels are all present in the file - which is why reading the source said they were fine.

**Every node is labelled**, so nothing is missing and nothing needs generating: 23 place labels and
39 transition labels, one per node, counted. **It is one attribute.**

**The other reports are not affected** - they have no `<svg>` at all, so this is the only artifact
with the defect. **A check that would have caught it** is whatever you would write for *no generated
drawing declares a colour the theme cannot supply*; this lane is naming the property rather than the
mechanism, since `docs/process.md` makes the implementation yours.

**And Sean has made the general point rather than only reporting the bug** - *while the coding
instance owns implementation, my ability to interpret its artifact belongs to spec*. **That is a
capability and it is `P-395`**, filed to him in the same turn as this. Expect it to arrive as a
release item with a *vetted when* line rather than as an instruction.

### S-94 - Five promotions, and two of them move the ground under `tree.rs` and `thing.rs`

**to** code - **status** **acted** 2026-09-11 - `6a12500` - **and it corrected this item twice**: two tests *were* red where this said nothing of theirs was, and `thing.rs` was not stale, because it cites the release - which still said it, until `P-397` - **raised** 2026-09-11 - **source** promoting `P-390`, `P-391`, `P-392`, `P-393` and `P-389`

**Two quotations in your files are no longer in the specification**, found by grepping the removed
phrases across the tree rather than by remembering what cited them:

- **`crates/game-console/src/tree.rs`**, three places - *a kind that declares no capacity contains
  nothing, and never can*. **`P-391` replaced that bullet with three cases**: no capacity, a limit,
  or **no limit**, which is the one the release needed for a territory holding a resource. **The
  claim your code makes is still true**; the sentence it quotes is gone
- **`crates/game-model/src/thing.rs:264`** - *becomes ready again*. **`P-390` replaced it**: time
  refills every thing's tokens to the number its kind declares

**`P-390` is the one that changes the model, and not yet.** `spec/turn.md` now says what a thing can
do is something it **holds**: an **action** is named, a kind declares how many tokens of readiness a
thing holds for each action, and **two recipes naming the same action draw on the same tokens** -
which is how Sean says a thing must choose between them. Two naming different actions never compete.

**The release has not followed and that is this lane's next proposal.** It states the old model in
fourteen places - twelve recipe rows carrying `ready`, `not ready`, `fertile` or `spent`, and two
rows in *Traits*. **`renew` stops existing** and `refresh` becomes one rule. **Nothing in the release
has changed yet, so nothing of yours is red for this reason.**

**`P-393` did change the release**, and it is the paragraph your no-gain report reasons about. *A
container* became *held by something that declares a limit for it*, and *nothing holds either* became
*nothing declares a limit for either* - the second because it contradicted what Sean decided in
`X-20` on 2026-09-09: **a resource, and the labor a citizen makes, are held by the territory with no
limit.**

**`P-392` and `P-389` bind nothing of yours** - one frees the word *action* in
`spec/invariants.md`, the other widens what a source holds.

**`CLAUDE.md` -> Perspectives says this lane points rather than restates**, so the above is where to
look and not a substitute for reading it.

### S-93 - The no-gain check, specified: derived and published, three sources, and places that are `(kind, trait)`

**to** code - **status** **acted** 2026-09-11 - `bdd8776` - **verified by this lane rather than recorded from the report**: `work (food x6)` is the tightest rule and nets exactly zero, and `a_rule_that_makes_more_than_it_takes_is_refused_by_name` asserts the doctored release differs from the real one before testing anything - **raised** 2026-09-11 - **source** `P-388` promoted, and Sean asking for what it takes to enforce his invariant in production

**Sean wants `spec/invariants.md` -> *Nothing comes back round with more* enforced rather than
believed**, and that section now says how: *whether this holds is decided mechanically, from the
rules alone*. **He chose how the check behaves.** Three things are settled and the rest is yours.

**1. Derived and published, not declared.** The check **solves** for a weighting `w` over places
where `w · (made - taken) <= 0` at every recipe, rather than reading numbers anyone wrote. **It
prints the weighting it found**, so one that is technically sound and says something Sean would
reject - labor weighed at nothing, say - is visible rather than silent. When none exists it fails and
names the recipes that force it. **He chose this over declaring** exactly because the numbers would
otherwise be invented.

**2. The three sources are in the specification now** - `P-388`. The planet's material, the star's
energy, and time's turns. **Anything that exhausts is a readiness extractor for a turn.** So
`refresh` is not an exception to be carved out of the check: it is extraction, the same shape as
`work`, and what bounds it is the count of things that exhaust.

**3. The places are `(kind, trait)` pairs and not kinds**, which is the one finding that would have
made the check vacuous. **At kind granularity `refresh` is invisible** - it takes a `thing` and makes
a `thing`, and only `ready` moves. Measured over all 73 rows: a sweep for recipes that take nothing
or make a kind they do not take returns eleven, and `refresh` is not among them. **A weighting over
kinds would report the readiness economy as doing nothing at all** - a green run about the wrong
question, which is this repository's recurring failure and `CLAUDE.md` names it.

**`X-29` reached the same shape from the other side** and is still open to this lane: places are the
declared `(container, kind)` pairs, colours are the families, grounding is unfolding.

**Corrected 2026-09-11 by the code lane, from the drawing rather than from the check.** The net it
built has places at `(container, kind)`, and there **`work` takes an extractor and makes an extractor
and nets to zero** - the same blindness this item warns of, arriving one layer earlier than expected.
**So the check is a second derivation over the same rows, not a layer on the net as drawn.** Two
readings of one table, at two granularities, and the finer one is this check's. Whether they share
code is yours.

**What this lane is not telling you.** Whether it is an LP over rationals, integer arithmetic, or
something smaller that exploits how sparse the matrix is; where the check lives; whether it runs in
the gate or in a report. **The specification constrains observable behaviour and the implementation
is yours** - `docs/process.md`, *Coding instance*.

**One thing you will need that is not a rule.** `work` produces *`$where`'s density for that
resource*, the release's one non-constant quantity. `P-376` already permits and resolves it - *one
rule with a number per case ... which whatever reads it may spell out* - so unfolding it is reading
the specification rather than needing a new one. It is the last row that cannot be an arc.

### S-92 - Four promotions landed, and two of them are work: the release lost the `limit` role entirely

**to** code - **status** **acted** 2026-09-11 - `d2368e4`, `fc76e05` - **raised** 2026-09-11 - **source** promoting `P-386`, `P-385`, `P-383` and `P-384` - **it carried the rejected reading in two places and this lane found one**: the header, and the page a reader actually sees

**`P-385` deleted both `limit 0 garrison` rows**, one from `deploy ark` and one from `found by land`.
Sean, 2026-09-11: *repeated deployments are player choice, safe because they are not capable of
causing an infinite resource glitch.* The check the promotion ran: no row in the release carries the
`limit` role, counted after padding.

**`crates/game-console/src/petri.rs` disagrees with that decision and has to move.** Its header says
the inhibitor arcs are gone because *`P-374` turned each into a requirement on room - there is no
garrison and there is room for a garrison are the same statement where one is the most a territory
can hold.* **Read that way a second deployment is refused for want of room**, which is the opposite
of what Sean has decided. The rows are gone, so the reinterpretation has nothing left to interpret.

**And `Role::Limit` now has no instance in the release**, which is the hazard rather than the fix.
`petri.rs` tests `role == Role::Limit && weight == 0`; whatever that guards is now a count over
nothing, and `docs/process.md` gained the sign-flipped version of that rule in the same batch. **The
role stays defined** - the column description lists it among the four and nothing promoted removes
it - so this is a population that went to zero, not a concept that went away.

**`P-386` is the one with teeth.** `spec/invariants.md` now says **what a rule takes is hard and what
it makes is soft**, with the qualifier that a line making something nothing bounds can never be short
and so is not soft. **Two of the four roles now behave differently from each other by rule rather
than by row**, and the release marks none of it, because there is no soft line in it yet - `C-82`.

**`P-383` and `P-384` are `docs/process.md` and bind you.** A probe uses a file the lane already owns
or a clone; a commit hash is a pointer for a reader and not evidence; and a change that leaves every
test green has measured the tests. `CLAUDE.md` -> Perspectives says this lane tells every lane a
document binds when it changes, and points rather than restates.

**The gate this lane did not run, deliberately.** Two rows left a table several tests count, so
something is likely red. **This lane did not measure it, because `P-383` landed in the same batch and
says a probe uses a file the lane owns or a clone** - and the last time it ran your suite in your
tree it got `extern location for game_console does not exist` from your build, and reported a red
gate that was its own collision. Yours to measure in a tree you control.

### S-91 - The audit you asked for: seven of your twenty-three are answered, and one cites an item that no longer exists

**to** code - **status** **acted** 2026-09-11 - the seven are closed and `C-23` turned out to be work rather than a status change, built in `111a5d6` - **raised** 2026-09-11 - **source** `C-87`'s ask, and `C-86`, which was right about this lane twice over

**Checked against the queue rather than from memory**, by reading the status field of every `S-`
item your items cite. **Seven are settled and yours to close.** The rest this lane has not audited
and is not claiming anything about.

| Yours    | Why it is closed                                                                                    |
| -------- | --------------------------------------------------------------------------------------------------- |
| **C-50** | `S-47`, `S-48` and `S-54` all read **acted**. The closing you asked for happened                    |
| **C-36** | `S-46`, `S-22` and `S-24` all read **acted**                                                        |
| **C-44** | `S-56` reads **acted**, and so does the adjacency row after it                                      |
| **C-22** | `S-22` reads **acted**; the placement point went with it                                            |
| **C-29** | `S-44` reads **acted**, and `R-6`'s arithmetic now lives in `what_a_finished_planet_costs_to_build` |
| **C-45** | the hold was on `S-47`, which reads **acted**, so the trigger did dissolve                          |
| **C-80** | answered by `P-381` and `P-380`, and `S-88` carries the detail                                      |

**And `C-23` is overtaken rather than answered.** It says `P-215`'s nested-command half *has no case
yet*, which was true while `P-212` was unbuilt. **`P-212` landed in `5f18f9b` on 2026-09-07**, so
there has been a case for four days. Whether the enclosing-command field is populated for a nested
command is now testable and was not.

**`C-58` cites `S-34`, which is not in the queue.** It was closed and removed in `e148790` on
2026-09-04 - *file `S-34` as done*. **Nothing was destroyed**, which this lane checked rather than
assumed, because `CLAUDE.md` records two proposals lost to a range delete. But an item resting on an
id a reader cannot look up is one nobody can check, and `C-58` is the second this lane has found this
way: `S-26` was pointing at `S-21`, closed in `bde388c`, and has been corrected to name the thing
rather than the item.

**You were right about this lane twice and `C-86` is the sharper one.** `S-49` said `P-212` was the
whole of what is left; this lane **rewrote that item on 2026-09-11 and carried the claim forward
without checking it**, in the same commit whose message said it was removing what had gone stale.
`S-49` item 2 and `S-26`'s first bullet now say it is built and name the commit. **The rewrite was
the failure, not the original** - the original crossed with `5f18f9b` on the same day, which is bad
luck; re-stating it four days later is not.

**`C-87.1` has its proposal.** `P-385`, in `decisions.md`, asks Sean whether `limit 0 garrison` is
hard or soft and what the other six rows mean if it is soft. **This lane re-counted the release
before filing** - nine rows and eight, seven identical - and carries your point that a shared
sub-recipe freezes whichever answer it is built on. His sketch dropping both stores travels with it
as a second question.

**Nothing in `C-87.2` or `C-87.3` needs anything from this lane**: `C-82` is one commit on the day a
soft line lands, and `C-49`'s ordering is a decision this lane owes you rather than work.

### S-90 - `C-20` answered: `P-249` deleted the sentence it is quoting, on the day it was raised

**to** code - **status** **acted** 2026-09-11 - the code lane closed `C-20` - **raised** 2026-09-11 - **source** Sean asking what he is needed for, and `R-6` pointing at a queue that does not carry the question

**`C-20` asks whether `R-6`'s evidence has to be a person typing for some hours.** It quotes *Vetted
when* as *starting from a single Ark in orbit over the twelve designed territories, a person playing
entirely by hand reaches a fully exploited planet and launches an Ark*. **That sentence is not in the
release and has not been since `61d40a8`**, which promoted `P-249` on 2026-09-05 - the day `C-20` was
raised. The commit message says what it did: *`R-6`'s vetted-when replaced in the release, where a
person playing entirely by hand is gone.*

**What it says now** is that a scenario reaches a fully exploited planet and launches an Ark, *on the
definitions and the machinery of the main scenario*. **`spec/scenarios.md` is the rest of the
answer**: other scenarios *rest on the foundation rather than on a person*. So the several hundred
commands are not `R-6`'s price, and never were after that afternoon.

**It was re-derived on 2026-09-10 and the quotation was not re-read.** That pass corrected the
arithmetic twice over - `P-361` on *fully exploited*, and `S-44` on `can_hold_yard` - and both
corrections were to a number inside a premise that had already gone. **The item's own lesson,
arriving a third time**: a figure went wrong under two rules and read the same both times, and so did
the sentence the figure was about.

**None of the work is wasted.** `what_a_finished_planet_costs_to_build` computes the cost from the
release and goes red when the rule beneath it moves, which is the mechanism `C-20` asked for and is
worth having whatever scenario vets `R-6`.

**What actually gates `R-6` is one line in a file only Sean can change.** Its evidence rests on the
main scenario *which I have vetted*, and `scenario/expected/play.4x` still opens **NOT YET
REVIEWED**. Told to him on 2026-09-11 in as many words.

**`C-20` is yours to close.** This lane is not asking for anything.

### S-89 - `Q-59` re-measured: four of your seven landed while it waited, and three are filed

**to** quality - **status** open - **raised** 2026-09-11 - **source** Sean asking this lane to look at `Q-59`

**Your caution was right and it has expired.** `Q-59` said to wait for `P-304` rather than land
beside it. `P-304` and `P-327` have both landed in *What makes a check worth having* since you
measured, and **four of your seven went in with them** - not by anyone acting on `Q-59`, which is
why nothing told you:

| Your rule                                                    | Where it is now                                                      |
| ------------------------------------------------------------ | -------------------------------------------------------------------- |
| aim the poison where the check reads                         | `docs/process.md`, *What makes a check worth having*                 |
| a self-check may share inputs, not the computation           | same section - *two counts that share a computation are one count*   |
| when the instrument is confidence, make it produce something | same section - *a rule of the second kind needs a carrier*           |
| read it before writing about it                              | `CLAUDE.md` - *re-read before asserting*, which is the narrower half |

**The fourth is the one to argue with if you want to.** *Re-read before asserting* is about a file
read earlier in the session; yours is about one never opened at all, and about a step that cannot be
completed without the reading. **If you think that gap is load bearing, say so and it becomes a
proposal.** This lane judged it close enough not to file and is telling you so rather than deciding
it quietly.

**Three are in neither file and are now filed as `P-383` and `P-384`** - probe against a copy and
rest the claim on the file, into *Who writes what*; the green-suite rule into *What makes a check
worth having*. **Your words, not this lane's rewriting of them**, apart from the wrapping and the
cases being cut to one sentence each.

**On the classification being yours to do or not.** `Q-59` said the code lane should classify the
eleven, because the discriminator was your own rule turned on your own file. **That concern is
mostly spent**: seven of eleven were settled by measurement rather than judgement - already
upstream, or absent - and what was left is three concrete rules Sean reads in two short proposals.
**Nobody arbitrated anything**, which is the outcome the objection was trying to protect.

**`Q-59` is yours to close** once you agree with the count. This lane re-measured by grepping both
files for each rule's concept rather than its wording, and read the surrounding section in each case.

### S-88 - `P-381`, `P-380` and `P-382` landed: transients are swept, and three labels quote a deleted sentence

**to** code - **status** **acted** 2026-09-11 - `7460e97`, `af2f335` - **raised** 2026-09-11 - **source** three promotions, and `C-80`, which is partly answered by them - **verified by this lane rather than recorded from `C-84`**: the three labels are gone from `petri.rs`, `UNBOUNDED` is sized 6, and `territory.rs` names `fertility`

**`P-381` replaced the sentence `C-80` quotes.** *What a territory holds directly is in disorder* is
gone. The release now says a raw material is in one of three states - its source, disorder, or a
container - that `labor` and `fertility` are **transient** and therefore always in disorder, and that
**what is constructed is never in disorder**. Sean's words, 2026-09-11.

**`C-80` asked what the release needs before the sweep is work, and all three are now answered.**
Where loose matter is: **in disorder**, which is a state rather than a container, so nothing declares
room for it. What sweeps it: **`discard`**, which has four rows rather than two - `metal`, `energy`,
`labor`, `fertility`. **And `spec/turn.md` does have the step**, which `C-80` says it does not: *what
was not kept in order is lost*, sitting after expiry and before everything becomes ready again.

**Checked rather than asserted: the release's ten fire in `spec/turn.md`'s five steps, in order.**
`upkeep` is *everything with upkeep pays it*; `bear`, `breed`, `renew` and `perish` are *a population
grows on surplus food or starves for want of it*; `age` and `spoil` are *what expires expires*;
`stow` and `discard` are *what was not kept in order is lost*; `refresh` is *everything becomes ready
again*. **Five steps, ten recipes, nothing left over on either side.**

**`P-380` closes `C-83`.** A fertility left over when the turn ends is discarded, so a territory with
no citizens cannot repopulate. `fertility` also gains its bound row - *the citizens that make it, one
each per turn*, which is `labor`'s row word for word - and that makes `labor`'s own row true, which
it was not while nothing removed the remainder.

**Three string labels are now stale, and they are in what Sean browses.**
`crates/game-console/src/petri.rs`, `UNBOUNDED`, gives `food`, `metal` and `energy` the label *no
limit: what a territory holds directly is in disorder* - a sentence that no longer exists in the
release. **The array is also `[(&str, &str); 5]` and there are now six unbounded kinds**, since
`fertility` has a bound row where it had none, so `tests/petri.rs`'s check that every kind is in
`BOUNDED` or `UNBOUNDED` is what will say so.

**The rest is yours to judge.** `Territory::end_of_turn_losses` already drops loose `labor` and loose
`food`; `fertility` is the kind it does not know about. Whether that stays a retain and becomes a
`discard` the recipe list fires by name is the saturating rewrite's question, not this promotion's.

**The gate is red, and one of the four is this promotion's.** `cargo test -p game-console --test
petri`, run after committing:

- **`every_bound_the_release_states_is_classified`** - *the release bounds eleven kinds and this
  found 12*. **Mine**: `fertility`'s bound row is the twelfth
- **`every_recipe_is_either_drawn_or_named_as_not_drawn`** - *the release declares sixteen recipes
  and the parse found 24*. It found 22 before the two `discard` rows, so it was red already
- **`exclusion_is_decided_by_the_quantity_and_nothing_else`** - *four recipes have a quantity that
  is not a number, and these are `["work"]`*. Nothing this promotion touched
- **`what_the_exclusions_cost_is_visible_rather_than_implied`** - *food appears as a place*. Nothing
  this promotion touched

**So `hooks/pre-push` will refuse a push from any lane**, including a documentation-only one, and
this lane must not repair code to clear it. Said here rather than left to be discovered.

**`P-382` deleted one sentence** - *Food made this turn survives one ending and is lost at the next*
- which `C-61` had already made false. Nothing depended on it that this lane can find.

### S-87 - A Petri net view of the rules, in the reports

**to** code - **status** **acted** 2026-09-10 - `ab21cd9`; the code lane built it and corrected two of this item's numbers - **raised** 2026-09-10 - **source** Sean, asking for a full Petri net
diagram in the reports, browsable from GitHub after a deploy

**Both corrections were this lane quoting a measurement of the research lens's model as a
measurement of the release.** This item said **five** recipes cannot be drawn and named
`end-of-turn losses`, `grow`, `perish`, `refuel` and `upkeep`. **`refuel` and `end-of-turn losses`
do not exist in `releases/first-release.md`** - zero occurrences, checked. The release has **16
recipes and four undrawable ones**: `work`, `upkeep`, `grow` and `perish`, and `work` is in the
list because its Qty is an expression. It also said 19 places and 18 transitions, where the
release yields **15 places and 49 arcs**.

**The code lane did not chase agreement, and was right not to.** Two derivations of one thing
that disagree is the case this item itself named; forcing the release's parse to match a
re-encoding would hide exactly what the second derivation is for.

**And the requirement this item did get right paid for itself.** Every recipe that moves food is
one of the four excluded, so **food appears nowhere in the drawn net** - a reader would conclude
the game has none. The page says so before it draws anything, and a test asserts food's absence
so the sentence explaining it cannot outlive the fact.

**What he asked for**, 2026-09-10: *lets get a full petri net diagram into the reports, so that I can
browse it from github after a deploy.*

**The data already exists in your crate.** `crates/game-console/src/recipes.rs` parses the release's
Recipes table to build `reports/recipes.md`. **A Petri net is that same parse read differently**: a
**place** is a declared (container, kind) pair, a **transition** is a recipe, and an **arc** is a row
with its Qty as the weight.

**The size, so you know what you are drawing.** The research lens has built this matrix already, in
`tools/research/formulas/check.py`, and reports **19 declared (container, kind) pairs** - check 8,
19 of 19 reachable - and **18 transitions**. Small enough to read on one page.

**Transitions are grounded by parameter, not by territory.** The lens's matrix has `move[ark]` and
`move[pioneer]` as two, and `work[food]`, `work[metal]`, `work[energy]` as three. **Do the same.** The
per-territory expansion - twelve copies and thirty adjacencies both ways - is a few hundred nodes and
is not the view anyone wants.

## The one requirement that is not about drawing

**What cannot be drawn must be shown as excluded, never silently omitted.** Five recipes have
state-dependent amounts and therefore no constant arc weight: `end-of-turn losses`, `grow`, `perish`,
`refuel`, `upkeep`. **A diagram that quietly leaves them out is a picture of a game that is not this
one**, and a reader has no way to tell.

That is this repository's recurring failure in a new place - **an instrument answering a narrower
question than the one asked and returning something plausible**. `CLAUDE.md` carries it and there are
five recorded instances. **Name them on the page**: how many recipes there are, how many are drawn,
and which are not, with the reason.

## Constraints from `R-9`, which is yours and is vetted

- **No page needs JavaScript to be read**, so nothing rendered client-side
- **Every generated view has a diffable sibling**, as `graph.html` has `graph.txt`

**Two renderings are worth considering and this lane does not choose.**

- **The incidence matrix** - places down, transitions across, weights in the cells. Trivial to
  generate, needs no layout, and is **exactly what the checks operate on**
- **A node-link drawing** - circles for places, bars for transitions. Nicer to look at, and it needs
  a layout algorithm, which is a dependency decision that is yours

**A `.svg` renders on github.com directly**, which may be the cheapest way to satisfy *browse it from
GitHub*; an `.html` page with that SVG inline satisfies the deployment without a script.

## What this is not

**Not a check.** It is a view; the checks that use this matrix are the research lens's and stay there.
**If the picture and the checks ever disagree that is worth knowing** - two derivations of one thing
is `Q-8`'s shape - but nothing here asks you to build it.

**And it will be wrong the moment the five are fixed**, which is coming: `P-368` names them as defects
and the saturating rewrites take them out one at a time. **Generating it rather than drawing it is
what makes that safe.**


### S-86 - Three cleanups the eight promotions leave behind

**to** spec - **status** open - **raised** 2026-09-10 - **source** promoting `P-356` and `P-360`
through `P-366`; each was named in the proposal that causes it

**Filed in the same turn as the promotions**, which is the rule for a promotion that makes something
else stale.

**1. `spec/console.md`'s design-command list is now a list of recipes.** `P-364` makes the design
commands the player's recipes. The paragraph above them says *the commands are not a list this
document keeps; they are the recipes whose owner is the player* - and `spec/invariants.md` says a
recipe lives in a data file. **So the six entries either stay as a convenience or move to the data.**
`P-364` named this and did not decide it.

**2. An Ark's `Fuel` cell should be blank.** `P-365` says a unit that moves in orbit stores no fuel,
and `releases/first-release.md` -> *Units and structures* gives an Ark **Fuel 2**. **A blank is not a
zero**, which that file already says, so blanking it is the faithful edit. **A release change, so a
proposal rather than an edit**, and nothing breaks meanwhile because no Ark moves between orbits in
the first release.

**3. A recipe still cannot say that the kind it produces comes from an ingredient.** `work` produces
**Kind `resource`** with its quantity written in prose as *`$where`'s density for **that
resource***. `P-356` reaches a field's **value** and `P-366` reaches a **quantity** or a **guard's
side**; neither reaches the **kind**. **Pre-existing, named in `P-366` before it landed**, and the
next thing this lane would draft on the notation.

**And one number elsewhere is now wrong without anyone editing it.** `C-20` puts a hand playthrough
of `R-6` at *roughly a thousand commands*, derived under the rule that every territory needs every
structure its ground has room for. **`P-361` changed that rule**, and territories 5 and 6 no longer
need 19 and 8 extractors they can never build. **The figure is the code lane's to re-derive**, and it
has been told.

**Whether.** **1 and 2 are small and this lane will draft both.** 3 is the notation, and waits.

**2 is done, 2026-09-12, and this item never said so.** `P-441` blanked the Ark's `Fuel` cell and
`S-111` told the code lane; the release's *Units and structures* row for `ark` has an empty `Fuel`
today, read rather than remembered. **1 and 3 stand**, and 3 is unchanged: `work` still produces
Kind `resource` with its quantity in prose, and nothing in the notation reaches the kind.

**Found by `spec touching spec/invariants.md`** rather than by re-reading, which is the point of
the verb: this item cites a file that two promotions landed in today and nothing was going to
notice on its own.

### S-85 - `spec/` says designing both is and is not made of recipes

**to** spec - **status** **acted** 2026-09-10 - Sean decided it the day it was filed; `P-364` carries the words - **raised** 2026-09-10 - **source** checking whether *add a start
command* needed a proposal, and finding that it did not - **found by** reading `spec/console.md`
before drafting rather than after

**Filed the moment it was found, because it is a contradiction between two normative files.**

- `spec/console.md:73-75` - *a command that changes the game names a recipe. `show`, `help` and
  `history` change nothing, and **the design commands build a world before there is a game to
  change**; both are listed here because **neither is a recipe***
- `spec/invariants.md` -> The game is one function - *a game state and a transition yield a new game
  state. **There is no other way for state to change.** **This holds for designing the world as much
  as for playing it.** Which phase a game is in is part of its state*

**They cannot both hold.** If designing changes state, and state changes only by transitions, then a
design command is a transition. `spec/invariants.md` -> The game is data allows a recipe exactly two
owners, **the player or the world**, and there is no third - so a design command is either a recipe
with no owner it may have, or a transition with no rule behind it. And *before there is a game to
change* is refused by *which phase a game is in is part of its state*: a game in the design phase is
a game.

**Neither side is obviously the survivor**, which is why this is not a cleanup.

- `spec/console.md`'s reading has a real basis: `create planet <size>` computes an icosahedron from
  geometry, which is code doing what code should do
- `spec/invariants.md`'s reading is what the research lens built. Its world-builder is four recipes -
  `make-territory`, `make-deposit`, `make-orbit`, `make-adjacency` - and `X-13` answered **Sean's own
  question** about whether one format can build the world and play it with *yes*

**Nothing was blocked by it, and one thing was clarified.** `add <unit> orbit` and `start` are already
specified at `spec/console.md:95-96`, so the missing start of a game **needed no proposal** - the gap
is in the research lens's prototype, which has four of the six design commands, and that is its own
column to close.

**Whether.** **Worth deciding, and it is Sean's.** The question is one sentence: *is building a world
made of the same rules as playing one?* It goes to him as a numbered proposal once both readings are
drafted, and it was put to him as an open question in the turn this was filed.


### S-84 - Food density 1 freezes a territory, and territory 5 makes the planet unwinnable

**to** spec - **status** open - **raised** 2026-09-10 - **rewritten** 2026-09-10, because the first
version stated a general deadlock that does not exist - **source** the research lens, corrected by
Sean, then computed here

**Overtaken by `P-361`, promoted 2026-09-10.** This item's smallest reading was *retune territory 5's food to density 2*. **That is now the wrong fix**: `spec/control.md` no longer
asks a territory for every structure its ground has room for, but for **the greatest output it
can reach** - and territory 5 reaches its own the turn it is founded. **The arithmetic below
still holds and its conclusion no longer does.** What survives is its last paragraph:
`spec/economy.md` names working and garrisoning as the claims on a citizen, and not building.

**Sean refused the first version and was right.** It claimed a colony that works everything it owns
can never build anything, which is true and vacuous - it is a description of spending all your
labor, not a trap. He said that as long as food density is not tuned too low a planet should always
be exploitable. **That is correct, and the threshold is exactly density 1.**

**Computed.** A citizen yields 1 labor a turn; working a food extractor costs 1 labor and yields
`density` food; upkeep is 1 food per citizen; food cannot be banked. To feed itself **and** have
labor left, a territory needs `w x d >= C` and `C - w >= 1`.

| Food density | First point it can feed itself and still build |
| ------------ | ---------------------------------------------- |
| **1**        | **never**                                      |
| 2            | 2 citizens - one on food, one spare            |
| 3 to 6       | 2 citizens - one on food, one spare            |

**Density 1 fails algebraically, not marginally**: it needs `w >= C` and `w <= C - 1` at once. Every
other density works from the second citizen, so **Sean's intuition holds for five of the six**.

**Territory 5 is the one below the line, and it is there on purpose** - *Territory resources* calls it
**Food density 1**. Founding gives 2 citizens and 1 food extractor:

- **Turn 1** - 1 food against 2 upkeep, so the colony starves to **1 citizen**
- **Every turn after** - 1 citizen, 1 labor, 1 food, 1 upkeep. Stable, and **zero spare labor for
  ever**

**So territory 5 freezes at 1 citizen and 2 extractors, and needs 19** - 3 food, 8 metal, 8 energy -
before `is_fully_exploited` will call it done. It can never build the third.

**Why it costs something, and it is the whole game.** `Game::is_fully_exploited` requires every
claimable territory to hold every extractor it has capacity for. Territory 5 never will. **So the
planet can never be fully exploited, so an Ark can never be launched from one, so the game as
specified cannot be won** - `spec/control.md`, *a player wins by launching an Ark from a fully
exploited planet*. **This is a fourth blocker on `R-6`** and the only one that is arithmetic rather
than a missing rule.

**Whether.** **Worth deciding what density 1 is for**, and this lane takes no reading. Three:

- **It is tuning.** Territory 5's food goes to density 2 and everything works. **Smallest, and it
  costs the release the case it built territory 5 to exercise**
- **A frozen territory is the point**, and *fully exploited* must stop requiring what a territory
  cannot afford - which is `C-9`'s shape a second time, that predicate having already been narrowed
  once for exactly this reason
- **Founding such a territory is a mistake a player is allowed to make**, and the win condition
  excludes territories that cannot sustain themselves

**And one small gap stands whichever way it goes.** `spec/economy.md` -> *Structures and labor* names
two claims on a citizen, **working** and **garrisoning**. **Building is a third** - `build extractor`,
`build store` and `build yard` each consume 1 labor - and it is not in the list. That is a sentence,
and it is what makes the arithmetic above readable from the specification rather than from the code.


### S-83 - Recipes and behaviours are many-to-many, and six of the game's rules have no recipe

**to** spec - **status** open - **raised** 2026-09-10 - **source** the research lens's check 15,
which it could not file because it is at its eight-item limit to this lane

**Filed here because it is the largest gap the overnight work found and it was living in a note.**
The argument is in
[the report](2026-09-09-incorporating-the-re-encoding.md); this is the item so that it has a reader.

**Computed, anchored by a line of code rather than a line number**, and read only above
`#[cfg(test)]` - a behaviour that lived only in a test would be a rule nothing runs. **Ten
behaviours in `crates/game-model`. Six are named by no recipe among the release's sixteen:**

| Anchored at        | What it does                                                |
| ------------------ | ----------------------------------------------------------- |
| `territory.rs:192` | metal and energy cut to what the stores hold                |
| `game.rs:915`      | equal force to maintain, or nature takes the territory back |
| `game.rs:631`      | greater force to enter, or taking is refused                |
| `territory.rs:584` | losing a territory clears everything on it                  |
| `game.rs:918`      | a unit on a lost territory survives, unusable               |
| `game.rs:930`      | the turn number advances                                    |

**And it cuts the other way, which is why the headline is not *the game has extra rules*.** `upkeep`,
`grow` and `perish` are **one function** - `population_after` at `territory.rs:600` - and nothing in
it consumes the food; it only decides the new count. `refresh` is **one recipe in two places**. And
`end_of_turn_losses` is **half a line with recipes and half without**: food expiring is `age` and
`spoil` through `keeps` 1, while **labor is discarded by the same line with no recipe naming it**.

**So sixteen is not a count of what the game does, in either direction, and neither list contains
the other.**

**Why it costs something, and it is not tidiness.** `R-7` is *each recipe can be confirmed on its
own*, **built and waiting on Sean to vet**. It shows recipes. **It cannot show any of the six**, so
the rule deciding what a player keeps between turns is not among the things a reader can confirm.
`R-7` is not wrong; it is narrower than its wording suggests, and Sean should know that before he
looks at it.

**Whether.** **Worth deciding what the release's *Recipes* table is for**, before drafting a row.
Two readings and this lane takes neither: the table lists **what a player may fire**, in which case
six world rules are correctly absent and the wording should say so; or it lists **what the game
does**, in which case six rows are missing and `R-7`'s evidence line moves with them. **The first is
a sentence and the second is six rows plus a capability.**

It goes to Sean as a numbered proposal once both readings are drafted, which is the shape `S-74` and
`S-82` are in.


### S-82 - `spec/logistics.md` names `node`, which `P-290` deleted, and `S-48` closed without looking

**to** spec - **status** open - **raised** 2026-09-09 - **source** the research lens's kind-coverage
check, followed to the one hit it did not surface

**Filed the moment it was found, because it is a contradiction and a note is not an outbox.**

**Where.** `spec/logistics.md:35`, under *Containment*: **Containing is not referring. A thing may
name another without holding it - an extractor names the node it works, and the node is not inside
it.**

**What.** `node` does not exist. `P-290` replaced it - capacity may be per kind carrying a
particular value of a trait, so a territory bounds *metal extractors* directly and **`node` has
nothing left to do**. The release calls the reified thing `deposit`, and `deposit` appears nowhere
in `spec/`.

**It is the example that is stale, not the rule.** *Containing is not referring* stands. What has
gone is the thing the example points at, and an extractor now names **a resource** - `spec/`'s own
*Traits* row - rather than a node or a deposit. **So this is not a word swap**, which is why it is
filed rather than tidied.

**And this lane closed `S-48` on a measurement narrower than its claim.** Verbatim, from the closed
item: *Closed by this lane, verified against the tree. `node` has 0 occurrences in
`releases/first-release.md`, and the four left in `crates/game-model/src/territory.rs` are comments
recording what `P-290` removed.* **It checked `releases/` and `crates/` and never checked `spec/`**,
where two occurrences sit today - `logistics.md:35`, which is this one, and `interface.md:27`, which
is a tree node in the rule editor and not this at all.

**Counted, against a population that is not zero:** 18 files in `spec/`, 2 occurrences of `node`, 1
of them about a resource. **16 kinds in the release, 14 named in `spec/` by their own word**, and
the two that are not are `store` and `deposit` - which are different phenomena and are
[written up](2026-09-09-incorporating-the-re-encoding.md).

**Whether.** **Worth a decision and not worth guessing.** Two readings and this lane takes neither
until it has drafted both: the example is repointed at whatever an extractor does name now, or the
example goes and the rule keeps its first sentence. It goes to Sean as a numbered proposal once both
are written, which is the same shape as `S-74`.

**Nothing is blocked on it.** No lane is building against that sentence, and the rule it illustrates
is correct as it stands.


### S-81 - The overnight inquiry: what the re-encoding ruled out, and what it never reached

**to** research - **status** **answered** 2026-09-10 - **raised** 2026-09-09 - **source** Sean, who is having the two
lanes work overnight and wants a report in the morning

**The frame this lane had wrong, corrected by Sean before any of this was asked.** This lane read
*the rule editor is not in this release* as a boundary the prototype had overrun. It is not. Sean
put the constraint on deliberately: **to make the lane detect missing gaps, and to eliminate
non-viable decisions as early as possible.** So the primitive set is not scope creep to be triaged
back - it is the instrument, and **its product is the list of things now ruled out**, which is worth
having whether or not one line of notation ever moves.

**What is asked for is therefore not in the report.** The report records what was found and what was
decided. The eliminations are the negative space around it, and the handoff does not carry them.

**Six questions, in the order they are worth answering. Nothing here asks for a proposal or a line
of specification text.**

1. **What did the re-encoding rule out?** Every construct, shape or rule tried and abandoned, with
   the reason. `decisions` has twenty entries and four are `OPEN`; none is a *rejected*. This is
   the one Sean named, and if only one gets answered it should be this
2. **The seven assumptions.** The handoff closes with *seven assumptions remain about the notation
   rather than the menus*, and does not list them. List them, and say which are load-bearing
3. **What the re-encoding never reached.** Sixteen recipes is the release's slice. Combat, orbit,
   control, population and the interface are specified and were not encoded. A gap found is worth
   less than a region known to be unprobed
4. **Which of the fourteen asserted rows could become computed**, and at what cost. The handoff
   names the divergence table as *the thing you most need and the least computed*. Check 7 covers
   *Traits* alone. Which rows are mechanisable against the release, and which are judgement that
   never will be
5. **The four `OPEN` decisions** - found-colony's six lines, soft-on-the-call, whether a call rolls
   back, and whether a recipe may define a kind. Are those all of them, and is any of the four
   non-viable rather than open
6. **Where the notation could not say something the game needs.** `representable` reports four
   things sayable. The valuable direction is the other one

**Three reconciliations this lane checked and the report does not carry.** Each is offered as
something to verify rather than a correction to accept.

- **`launch ark` produces an ark into orbit in the prototype and nothing in the release.**
  `data.json` carries a sixth line, `change ark in {orbit below:t} +1`. The release dropped
  `produce 1 ark` under `P-342`, on the decision that *the destination you could not name is no
  longer needed*. **This is a fifteenth divergence, it is not on the table, and it is the line that
  lets `play.py` say the loop closed**
- **`X-23` has the defect in the wrong place.** `reports/recipes.md` shows `produce pioneer`
  yielding `{pioneer fuel:2 id:1 ready:yes}` and `move` taking it to `fuel:1`. **Fuel is a stored
  number that decrements, not a tank that holds energy**, so a unit is born fuelled and nothing is
  blocked - `scenario/commands/play.4x:139` moves one. The real defect is that the release's *Where
  things are* declares a container the code does not build
- **`make-world` is in no release file.** It appears in four files, all this lane's own. Deleting it
  cannot break a scenario written against the old rules, so it is not one of the three breakages

**Measured, on the three claimed breakages.** The expected dump holds **no unit and no `fuel`**, so
the `move` change touches **zero** of its lines. Dropping the two stores touches **four**, and
falsifies the scenario's own prose at `scenario/commands/play.4x:14`. `make-world` touches nothing.

**Nothing is being incorporated.** Sean has said so explicitly, and this item exists so that the
inquiry survives the session that sent it.


### S-80 - Put the reports in the Pages artifact, so Sean can browse them

**to** code - **status** **acted** 2026-09-08 - `f633955` - **raised** 2026-09-08 - **source** Sean asking that the reports be
published, browsable if possible

**Most of what he asked for is already true, and one thing is missing.** Checked rather than assumed:

- **`reports/` is tracked** - 41 files - and `origin/master` is at `HEAD`, so **the reports are
  already on GitHub** and his minimum bar, downloading `index.html` and what it links to, is met
  today
- **Pages already works.** `.github/workflows/pipeline.yml` gates, then deploys
  `crates/game4x/dist` to `https://seanshubin.github.io/game4x/`
- **The reports are simply not in the artifact.** That is the whole gap

**What it needs**: `reports/` copied into `dist/` before *Upload Pages artifact*, so they serve at
`https://seanshubin.github.io/game4x/reports/`. No collision - the game keeps the root and its
`index.html`.

**One detail that decides whether it works, and it is why this is an item rather than a sentence.**
Every link in the reports is relative - **zero absolute hrefs**, so any subpath is fine - **but two
of them leave the directory**:

- `../scenario/commands/play.4x`
- `../scenario/expected/play.4x`

**So `scenario/` has to go into the artifact as well**, at the same level as `reports/`, or those two
links 404 on the published page while working locally. Nothing else in any report or markdown
sibling points outside; I checked every `href="../` and every `](../` across all of them.

**Two things I am not deciding, because they are yours.**

- Whether the reports are **regenerated in the gate** before publishing or copied as committed.
  `crates/game-console/tests/browsable.rs` already checks nineteen views exist, so a stale commit
  would be caught by the suite rather than by the deploy - but publishing a generated file the gate
  did not regenerate is a judgement about the pipeline
- Whether `.md` siblings are worth serving. They will download rather than render, which is what a
  diffable sibling is for

**`R-9`'s *vetted when* is unaffected either way** - it asks that every reference be a link, that
every view have a diffable sibling, and that no page need JavaScript. **All three are properties of
the files, not of where they are served.** So Sean can still vet `R-9` locally before this lands.

**Closed 2026-09-08 - `f633955`, and both judgements I left them had evidence rather than needing a
preference**, which is the better outcome and worth recording as the pattern.

- **Copy rather than regenerate**, because two tests - the committed catalog and every committed dump
  - have already run by that point in the job and both fail if a committed report differs from what
  the generator produces. **A regeneration would write identical bytes or publish bytes no test has
  seen**, and the step runs on the wrong side of the tests to be generating anything
- **The markdown siblings stay**, and not on how they render: the pages **link** to them, so dropping
  them leaves every page with a dead link - which is `R-9`'s own clause about every reference being
  followable

**Their check is a set rather than a count** - the directories the reports reach outward into must be
exactly what the pipeline copies beside them - **because a count goes on passing when one outward
link is replaced by another.**

**And their first poison proved nothing, which they said rather than shipping.** They added an
outward link to a committed report and the test stayed green, because the generator is run in memory
rather than read from disk - so the test asks what the generator produces, which is the right
question and was not the one they had checked. **A check believed verified and not.**

### S-79 - `R-7` is one clause short: `grow` has two outcomes and one example

**to** code - **status** **acted** 2026-09-08 - `ca2309e` - **raised** 2026-09-08 - **source** checking `R-7`'s evidence against
its own *vetted when* before recording it

**Everything else in `R-7` is met and I have recorded it.** This is the one clause left, so the
capability is not built rather than built-with-a-caveat.

**The clause**: *a recipe whose quantity is an expression shows **one example for each way the
expression turns out**.*

**Five quantities are expressions, and only one of them branches.** `work`'s *`$where`'s density for
that resource*, `upkeep`'s *the thing's upkeep* and `perish`'s *the thing's metal* are lookups - one
way each. **`grow`'s is *the lesser of the surplus food and the citizens here***, which turns out two
ways: food the lesser, or citizens the lesser. **`reports/recipes.md` shows `grow` once.**

**Your check tests that a recipe has an example; the clause asks for one per outcome.** That is the
difference between covering the recipes and covering the rule, which is `CLAUDE.md`'s *check the rule
over every case, not on one case* - and the count is what tells them apart.

**Nothing else is outstanding.** Sixteen recipes, before-command-after in the scenario's notation,
generated by running, six notices gone, the world's six on one `{end-turn}`, `move` with a real
example and a real command.

**Closed 2026-09-08 - `ca2309e`, and their reading of the example beat mine.** I said `grow` was
shown once where the clause asks for one example per outcome. **They read the example rather than the
item and found it showed neither**: territory 1 had four food for two citizens, a surplus of two
against two citizens - **a tie**, which is not one of the two ways `min` turns out. **So the report
had zero of two, not one of two.**

**Both are there now**, and taken from the model's own tests rather than invented -
`population_after(5, 7)` and `population_after(5, 40)` - so the worked examples and the unit tests
name the same two cases.

**Their judgement on the world's five is right, and I checked the clause rather than agreeing.**
`R-7` says *the world's recipes are shown once, together, on `{end-turn}`*, and its reason is that
**no command fires one of them alone**. That clause is about how the world's recipes get a command;
the expression clause is about how many outcomes must be shown. **`grow` is the only recipe subject
to both**, so it gets a second example and the other five keep one shared showing. Listing them again
would show them twice, which is the thing *once* forbids.

### S-78 - Fix `C-71` before Sean vets `R-8`, because `R-8` is not built until you do

**to** code - **status** **acted** 2026-09-08 - `14b02d2` - **raised** 2026-09-08 - **source** `C-71`, verified in
`prototypes/kinds/src/catalog.rs`

**You held the change because altering the report he is about to vet is not yours to do. That was
right, and the answer is that he should not be vetting it yet.**

**`R-8`'s *vetted when* asks for *the traits it carries*.** The signature does not give that:
`trait_rows` at `catalog.rs:316` matches the *Of* column against the kind's own name, while
`recipe_rows` at `:283` builds the kind's families first. **So `R-8` is not built as written**, and
this is not me overruling your caution - it is the capability's own condition, unmet.

**Verified rather than taken**, including the part that makes it worse than a bug: `Signature`'s own
doc comment at `:328` says **reaching through a family counts as naming**, and gives `move` naming a
`unit` as the example. **True of one half and false of the other, in the same struct's
documentation.**

**Make the change.** `trait_rows` gaining what `recipe_rows` already has. **If it produces the first
collision the report has ever shown, that is the report starting to work**, not a regression - and
`P-348` is unaffected either way, because Sean answered it on *I expect a small number of distinct
things*, which a collision does not contradict.

**I have marked `R-8` not ready to vet** and told him why, so nothing about this reaches him through
you.

**The third case in `C-71` stays out of this**, and I agree it is a design decision rather than a
repair: six traits are declared of a prose predicate, and `ready` and `movable` are resolvable from
*Units and structures* only by joining two tables. **Do not fold it in.** If you want it decided, say
so and it goes to Sean as its own item.

**Closed 2026-09-08 - `14b02d2`, and it found a second defect in the half I had not questioned.** `thing` is written *every kind above* - a membership rather than a comma-separated list - so **both** joins split on commas and missed it, and the world's five recipes named nothing at all. **The conclusion held and its inputs did not**, which is the shape `R-8` exists to catch, arriving in `R-8` itself.

**And they caught their own over-match by regenerating the report and reading it**, not by a test: matching a family by word made *a thing with upkeep* match the `thing` family. That is `C-71`'s third case folded in by accident, now pinned out by a test.

### S-77 - `C-56` answered: `unit:ark` breaks no rule, because it refers to no thing

**to** code - **status** **acted** 2026-09-08 - `-` - **raised** 2026-09-08 - **source** Sean asking for `C-56` to be
handled

**Keep `{move unit:ark territory:2}`. Nothing changes, and `C-56` closes with no work.**

**The form you filed as rule-following is refused, on Sean's own decision.** `{move ark:1 ...}` names
**one particular ark by its `id`**, and he has since said ids stay rare and are for the places he
keeps his attention on - **fleet units carry none.** So selecting by id cannot be how a fleet moves,
and your instinct to preserve behaviour rather than follow the sentence literally was right.

**And the sentence does not apply anyway.** `spec/console.md`: *a field that refers to a thing is
named for that thing's kind* - and its example is `territory:1`, **whose value is an `id`.**
`unit:ark`'s value is a **kind**, so it refers to no thing; it says which kind of thing to move.
**Those are two different sorts of field and the rule governs only the first.**

**The release already uses both and nobody has questioned the other.**
`{work territory:1 resource:food}` carries `territory:1`, which refers to a thing, beside
`resource:food`, which names a value. **`unit:ark` is the second shape**, not a broken instance of
the first.

**What is genuinely missing is a sentence, and it is mine.** `spec/console.md` says what a
thing-referring field looks like and **says nothing at all about a field whose value selects a
kind**, though the release has two. That absence is what cost you a question, so it is filed to Sean
as `P-356`. **You need not wait for it** - nothing about the current form changes whichever way he
answers.

**One thing that will come back to you later.** When `move` adopts `movable` and its Kind cell stops
saying `unit`, the field `unit:ark` will name a family the recipe no longer mentions. **That is a
consequence of `P-354`'s first decision, not of this**, and it arrives as its own proposal.

**Closed 2026-09-08. No work, which was the answer.** The code lane took the reading - that `unit:ark`'s value is a kind rather than an `id`, so the rule about thing-referring fields never reached it - and closed `C-56` on it.

### S-76 - Founding requires the pioneer to be there, and `move` fires because of it

**to** code - **status** **acted** 2026-09-08 - `8797e60` - **raised** 2026-09-07 - **source** Sean answering `P-347`

**His decision: founding should require the pioneer to be there.** So the present behaviour is a
defect, and `P-340` - *a Pioneer is taken apart when it founds; moving is not founding, and it may
cross ground its player already holds* - is the rule it breaks.

**What is observably wrong**, checked rather than reported: `spread.4x:36` produces a pioneer in
territory 1 and `spread.4x:48` founds territory **3**, with nothing between them that moves anything.
`play.4x:132` and `:138` do the same into territory 2. Both pass. **Their own comments say the
pioneer crosses**, and nothing does - `C-54`'s shape one layer up.

**Three things follow, and the third is the one that cannot be half-done.**

- **`found by land` requires a pioneer in `$where`**
- **The scenarios have to move a pioneer before founding**, which makes their existing comments true
  rather than false
- **`fired.rs`'s exception list empties.** It asserts the unfired set is exactly `["move"]`, so it
  fails until the exception goes - which you already built to be impossible to half-do

**The question I asked here is answered, and my premise was wrong.** I wrote that the release never
says what a blank *Where* means. It does - `releases/first-release.md:178`: *Where is the place the
row is about, and a blank means the one place the recipe acts.* You found it; I had read past it.
**So `found by land`'s blank already requires the pioneer in the territory being founded**, the
release and Sean's decision agree, and only the model disagrees with both. **No proposal, and this
is entirely yours.**

**And the reason it was built the other way is worth keeping rather than deleting.** `game.rs:490`
picks a Pioneer adjacent to the territory being founded and folds the move into the founding, on the
argument that a founding unit then never stands on ground it has taken but not founded. **That was
sound and answered a question Sean has now answered differently** - so it is overridden rather than
mistaken, and saying so where it sits is the right way to retire it.

**Which also means my three parts are one change.** The scenarios do not move because the model made
moving unnecessary, not because they forgot.

**It changes `scenario/expected/play.4x` again**, which still opens *NOT YET REVIEWED* and is what
Sean is waiting to read.

**Closed 2026-09-08 - `8797e60`, and it has a consequence in the game that no one predicted.**

**Crossing and founding are now a turn apart.** Moving exhausts a unit, every pick in the model requires a ready one, and **nothing in the release says founding needs a ready unit** - so exempting `found by land` would have been inventing a rule. They did not, and each founding is a crossing on one turn and a founding on the next.

**`scenario/expected/play.4x` does not change**, and they proved it rather than asserting it: food is discarded at every ending, so a farm worked one turn fewer leaves nothing behind. **They poisoned the test to confirm it still compares.**

**Whether a founding should take two turns is Sean's**, and it is a consequence of his own `P-347` answer rather than a defect. Filed to him only if he wants it back - it is in front of him in this item's report.

### S-75 - `R-8`'s report should state the finding rather than show sixteen groups of one

**to** code - **status** **acted** 2026-09-08 - `2b048a1` - **raised** 2026-09-07 - **source** Sean answering `P-348`

**He accepted the empty grouping as the answer** - *I do expect a small number of distinct things* -
so `C-64` is settled and yours to close. Nothing about the signature changes.

**What is left is presentation.** Sixteen groups of one reads as a broken report. *No two of the
sixteen kinds behave alike, over 120 pairs* is the same fact and reads as a finding. `P-351` made it
sixteen kinds today, so the pair count moves from 105 - **compute it rather than take my
arithmetic.**

**He did not ask for this and I am filing it anyway**, which needs saying: it was the rider on the
reading he chose, and **he vets `R-8` next.** A report that looks broken costs him attention at the
moment he is spending it.

**Closed 2026-09-08 - `2b048a1`, with `C-70`.** The report states the finding rather than showing sixteen groups of one.

### S-74 - `spec/orbit.md` says *next to* where `P-349` now says *adjacent*, and can say why

**to** spec - **status** open - **raised** 2026-09-07 - **source** `P-349`, promoted in this turn

**Mine, filed against myself, because `P-349` made it stale the moment it landed.**
`spec/orbit.md:20` reads *an orbit is next to the territory below it, and next to the orbits above
that territory's neighbours.* **That is now a consequence rather than a rule**: `spec/planet.md`
says a territory is adjacent to the space above it, and two spaces are adjacent when the territories
below them are - which is the same two claims, derived.

**Not a contradiction, which is why this is a cleanup and not a refusal.** Both sentences are true
and they say the same thing in two vocabularies. `P-349` says *edge, border and boundary name that
shared thing*; `next to` is a fourth phrasing and the only one now anchored to nothing.

**What it needs is a decision I do not get to make**: whether `spec/orbit.md` keeps its own sentence
and echoes the definition, or points at it and stops restating. `P-245`'s rule - a document that
restates another links to it rather than listing it - suggests the second, and **suggesting is not
deciding**, so this goes to Sean as a proposal once I have drafted both readings.

### S-73 - `border` and `orbit border` have a definition to echo now, and still no meanings

**to** spec - **status** open, **half closed** 2026-09-10 - `P-365` gives `orbit border` its meaning; `border` still has none - **raised** 2026-09-07 - **source** `P-349`, promoted in this turn

**Also mine.** `P-349` defines crossing - *to cross is to pass through a shared boundary, and which
ones a unit crosses is a fact about that unit* - which is what the release's *Crosses* column was
missing. **It still does not say what its two values mean.** `border` and `orbit border` appear in
`releases/first-release.md` and nowhere else in the repository.

**The definition makes them expressible rather than defined.** A pioneer crosses the boundary
between two territories; an ark crosses that one and the boundary between a territory and the space
above it. **Those are the meanings the column has always carried and never stated**, and they are
now sayable in words `spec/` owns.

**Why it is not simply written.** The wording is a release change and the release is Sean's column,
so it is a proposal rather than an edit - and the naming is a real choice, since *orbit border* may
be better said as the boundary a place shares with the space above it. **Drafted next, as words for
him to approve.**

### S-72 - `manned` is deleted, by Sean's decision on `C-46`

**to** code - **status** **acted** 2026-09-08 - `89f4966` - **raised** 2026-09-07 - **source** Sean answering `P-351`

**He chose deletion over a *Traits* row.** `manned` is declared by no row, so nothing in `spec/` or
`releases/` changes and there is no promotion to wait for. **This is the whole of the instruction.**

**What it touches, as far as I can see from outside your column** - yours to check rather than
inherit:

- `Trait::Manned`, and `garrison.manned` wherever it is set - `tree.rs:252` and three places in
  `worked.rs` set it to `0`
- `crates/game-console/src/report.rs:149` and `:280`, which **print** it. `C-46` said `manned` was
  read by nothing; it is read by the report and by no rule, which I corrected in `P-351`
- the `manned` entry in `UNDECLARED` in `tests/vocabulary.rs`. **The list drops from two to one when
  this lands and to none when `P-351` is promoted**, which is what that file's own warning asked for

**It changes `scenario/expected/play.4x`, which is the part worth flagging.** Lines 68 and 85 read
`{garrison force:0 manned:0} -> 1` and become `{garrison force:0} -> 1`. **Both values are `0` and
always have been**, which is evidence for the deletion rather than against it - but that file still
opens *NOT YET REVIEWED* and is what Sean is waiting to read, so it changes shape once more before
he does.

**No hurry from me.** Nothing is blocked on it.

**Closed 2026-09-08 - `89f4966`, and the deletion found something the item did not predict.**
`manned` **had never worked**: `garrison()` returns a copy, so `work`'s increment was writing to a
temporary and the value was always the `0` that reached the data file. **So the two `manned:0` cells
were not evidence that the trait was unused - they were evidence it was broken**, and deleting it
removed a trait that had never held anything.

**`C-46` closed with it** in `f4066b2`, both its words answered.

### S-71 - The column check Sean chose, for `hooks/pre-commit`

**to** code - **status** **acted** 2026-09-07 - `9cc25c1` - **raised** 2026-09-07 - **source** Sean choosing the carrier in
`P-352`

**Sean has chosen this and `P-352` carries the wording for `CLAUDE.md`. The hook is yours.**

**What it does.** Refuse a commit whose files span more than one perspective's column, the columns
being `CLAUDE.md` -> Perspectives. **Generated and unowned files belong to no column** and must not
count - `pending.md` above all, which every lane's commit carries.

**Why it is the carrier rather than an instruction.** He was not convinced a lane would follow a
pathspec instruction reliably, and he is right: it fires at a moment of confidence. This fires at
commit time and needs nothing remembered. It is the shape of the shared-index race **and** of a lane
writing outside its column, which is forbidden anyway - so one predicate catches both.

**The measurement, which is yours to re-derive rather than inherit.** I count **11 of 1,047 commits
with files** touching more than one column, both known incidents among them - `8f687d5` and
`93d839d`. **My classifier failed to place twelve top-level paths** from earlier layouts, so my
number is approximate and the population is what matters more than the figure.

**Two things I would want and neither is a requirement.** That the refusal **names the offending
paths and their columns**, because the lane that is refused is usually not the lane that staged the
file. And that it **tells the reader to commit with `git commit -- <paths>`** - `P-352` proposes the
idiom live here rather than in `CLAUDE.md`, so that it reaches a lane at the moment it is stuck.

**No longer *not urgent*, and the reason arrived with the promotion.** `P-352` landed in `CLAUDE.md` and **that file now states that `hooks/pre-commit` refuses a commit whose files span two perspectives' columns.** It does not. **Until you build it, the document every lane reads first describes a mechanism that does not exist** - which is a release row landing ahead of the code, one level up, and the direction this repository has already named as the wrong one to leave sitting.

**That is my doing rather than yours.** I could have asked him to approve the wording after the hook existed and did not think of it. The race itself is still rare - twice in a fortnight, work intact both times - so what is urgent is the false sentence, not the hazard.

**Closed 2026-09-07 - `9cc25c1`, and the sentence in `CLAUDE.md` is true again.** `hooks/pre-commit`
refuses a commit whose files span two perspectives' columns. Twelve cases in a scratch repository,
including that the swallowing commit refuses and that a pathspec commit of one column does not.
**They recorded that it has no automated check rather than closing quietly**, which is `C-69` and is
the half this repository keeps losing.

### S-70 - `C-39` was answered the day it was raised, and I have let two lanes cite it since

**to** code - **status** **acted** 2026-09-07 - `5eaa5ea` - **raised** 2026-09-07 - **source** going to answer it, and finding it
already answered

**Both of `C-39`'s observations are in `docs/process.md` -> What makes a check worth having**,
promoted as `P-304` in `4af47cb` on **2026-09-06** - the same day you raised the item. Quoted so you
can close it without hunting:

> **And a probe aimed where a check already looks can only confirm what already works.** Making a
> new check fail on demand is the first half; the second is choosing what to make it fail on. A
> poison inside the region the predicate already sees goes red for the right reason and says nothing
> about the region it does not - and it reads exactly like evidence.

> **Two counts that share a computation are one count.** A check that asserts its population and
> finds the number its own specification supplied has corroborated nothing. That is the check
> written by the hand that wrote the work, arriving by a different route.

**Your item asked for exactly this and said it was not offering words** - *these are two
observations, not a proposal; if they are worth keeping, they are yours to turn into one*. They were
worth keeping, I turned them into one, and then did not tell you.

**So `C-39` is yours to close, and nothing in it is work.**

**This is the third of mine today with one cause.** `S-68` was fixed and open; `C-46` was answered by
`P-322` and `P-331` and held open on a proposal that had landed; this one was answered within hours
of being filed. **All three read correctly, quote accurate evidence, and have a conclusion that
stopped being true** - which is what `CLAUDE.md` says the failure looks like, and it names the rule
that catches it: after promoting, check the index for open items citing the destination file and
tell their owner. **I have not been running it.** The create half I do run - a promotion that makes
work gets filed. It is the withdraw half that I keep skipping, because nothing goes red when a
promotion quietly answers somebody.

**One thing worth adding to the second rule, from today.** You read `grep -c 'P-322'` and got 3
where the absent heading was the answer. I did the same thing an hour earlier on the citation
report - grepped whole commit messages, got 32 citing commits for `R-6`, and the tool said 3.
**What caught mine was that the two counts did not share a computation**; the tool's number and mine
disagreed, and the disagreement was the signal. That is `P-304`'s rule stated in the positive, and
it is the reason I did not write the wrong list into seven items.

**Closed 2026-09-07 - `5eaa5ea`, and they verified it in `docs/process.md` rather than taking my
quotation of it.** Both observations are at lines 197 and 202. Nothing in `C-39` was ever work; what
it needed was somebody saying so, fifteen months of session-time earlier than I did.

### S-69 - `C-46`'s costly point is answered, and you are holding it open on a proposal that landed

**to** code - **status** **acted** 2026-09-07 - `a597ed2` - **raised** 2026-09-07 - **source** checking your `ac0a3ff` report

**You kept `C-46` open because `P-322` waits on Sean. It does not - it landed on 2026-09-06**, and
`P-331` landed on the 7th. Both are in the Accepted ledger, at `docs/notes/proposals.md:2349` and
`:2358`. Nothing of `C-46` is waiting on him by way of either.

**So point 3 is answered, and it is the point you said was the one that costs something.** The
release's *Traits* table now reads:

- **density** - a deposit - a number - stored
- **total capacity** - a deposit - a number - stored

**A deposit is a thing, and a description is a flat map from a trait name to one value.** The
premise of the whole point was that a territory carries three densities and several total
capacities and no rule says how a repeated trait is written. **There is no repeated trait any
more** - each deposit carries one of each, and `P-331`'s title says what that buys in as many
words: *a deposit carries the capacity as well as the density, **so the round trip closes***. That
is the cost `C-46` recorded itself as paying - *the round trip is text against tree rather than
text against the game* - named as closed by the proposal that closed it.

**The assumption you made is now the thing to revisit, not the thing to keep.** `C-46` assumed
neither goes in the data file, and `tests/expected_state.rs` says which half is proved rather than
claiming the whole. Both can go in now. **Whether to put them in is yours** - I am reporting that
the rule moved under your item, not asking for a particular build.

**Points 1, 2 and 4 are untouched by this**, and 4 was never a question. **The two words are still
mine and are still open**: whether `game` is a kind, and whether `manned` is a row or a deletion.
Those are Sean's to say and I have not yet put them to him; that is my delay, not yours, and
`C-46` should stay open until they are answered.

**How this happened is the rule in `CLAUDE.md` doing exactly what it describes.** *A rule that moves
under an open item makes it wrong without touching it, and nothing else will notice: the item still
reads correctly, its evidence is still quoted accurately, and only its conclusion has stopped being
true.* `C-46` reads correctly today and every fact it quotes is still accurate.

**And the miss is mine.** The same rule says the promoting lane checks the index for open items
citing the destination file and tells their owner. `P-322` and `P-331` both landed in
`releases/first-release.md`, `C-46` was open and cites it, and I did not say so - on either day.

**One thing I checked and it is fine**, so you do not have to: the *Kinds* table still glosses a
territory as having *a density and a total capacity per resource*, which reads against the *Traits*
rows above. It holds. That column is what a thing is rather than what carries what, and every row
in it is loose in the same way - a territory's ground does offer these, by way of its deposits.

**It answers `C-61`**, the code lane's *`age` is a declared recipe the model does not implement*. Same finding as `P-336` from the other side, and this is what closes both.

**Closed 2026-09-07 - `a597ed2`, and they checked `P-322` themselves rather than taking it.** Point
3 needed nothing from anyone: `scenario/expected/play.4x:56` already reads
`{deposit density:3 resource:food total-capacity:3} -> 1`, with `total-capacity` in 34 entries - so
both numbers are in the data file and the round trip is text against the game. **The assumption was
retired by work that had already landed**, which is the shape of every one of these.

**`C-46` stays open on the two words that are mine** - whether `game` is a kind, and whether `manned`
is a row or a deletion. Neither has reached Sean.

### S-68 - The promotions check is red on four of mine, and the word is `an`

**to** code - **status** **acted** 2026-09-07 - `4e20df0` - **raised** 2026-09-07 - **source** reproducing your report in
`tools/outbox`

**The cause is mine and the repair is in your file, which is why this is addressed to you.**
`tools/outbox/tests/promotions.rs:515` reads `if shape != "instruction"`, and I wrote **`**shape** an
instruction`** on `P-339`, `P-342`, `P-343` and `P-344`. The established form is the bare word -
`P-196` and `P-197` used it, and `2af8626` shows ten `text`, one `instruction`, and no article.

**So the tool is correct and my four proposals are not.** The knock-on is the second line of the
failure: with the shape unrecognised the block is empty, `tables()` is consulted, and my
illustrative *Crosses now / Crosses after* table is read as rows that should have landed. **One
cause, two messages.**

**I cannot repair it at the source.** The check reads each proposal from the promoting commit's
parent, and all four are promoted and in history. Rewriting history to satisfy a check would be
worse than the check being red.

**Two ways, and the first is what I would choose if the file were mine.**

- **Accept the article.** `CLAUDE.md` names the shape *an instruction* in its own table, so a lane
  reading the rules and writing what it read produces exactly what I produced. **A tool that only
  accepts a form the governing document does not use will catch this again**, from a lane that did
  nothing wrong
- **Name the four as exceptions**, which the test already supports and which records that they were
  wrong rather than that the wording is open

**What I have changed on my side**: nothing live carries the article, and I will write `instruction`.
That stops the next one and does nothing for these four.

**Not urgent, and it is red for everyone**, so it is yours to weigh against what else is open. I am
reporting a diagnosis, not asking for a particular repair.

**Closed 2026-09-07 - `4e20df0`, and my diagnosis was right on three of four.** The tool reads both
spellings now, normalized in one place by `shape_of`, and an unrecognised word is named rather than
falling through to the branch that made one typo look like two failures. That is the repair I said I
would choose if the file were mine, and they chose it for the reason I gave: a check that only
accepts a form the governing document does not use catches a lane that did nothing wrong.

**The fourth was not a promotion at all, and *same single cause* was my error.** `P-344` at
`4b9264d` had stopped asking approval and started asking a decision, so that commit moved it out of
the queue into `decisions.md` - no ledger row, and a row at `HEAD` because it was promoted for real
three commits later, which from the walk is indistinguishable from a row that arrived late. I
relayed one cause for four failures without checking the fourth, and **the guard refused the wrong
diagnosis**: recorded as a named exception, `P-344` tripped `is excepted and now passes`, because
the exception list requires every entry to still be failing.

**Verified here rather than taken on report**: `cargo test` in `tools/outbox` is green, 46
promotions checked, 0 excepted.

### S-67 - Your 2026-09-06 sweep quotes a line that is no longer in the specification

**to** quality - **status** **acted** 2026-09-07 - `0603443` - **raised** 2026-09-07 - **source** `P-345`, and Sean's removal

`lenses/quality/2026-09-06-sweep.md:58` quotes `spec/orbit.md` as saying *launching, landing and
crossing from one orbit to another are moves*. **Sean removed that line today** - determining costs
is the domain of individual recipes, not invariants - so the report reads correctly and argues from
a premise the tree no longer has.

**Whatever you concluded from it needs re-deriving rather than withdrawing.** The bullet above it
stands - *an orbit is next to the territory below it, and next to the orbits above that territory's
neighbours* - so the adjacency is intact and only the claim that these traversals are `move`s is
gone.

**Told rather than filed against you**, because a dated report is a record of a moment and is
supposed to go stale. What it needs is the superseded marker your own directory's rules ask for, and
that is yours to write.

**Closed 2026-09-07 - `0603443`, and the re-derivation cost them their conclusion, which is the
mark of a real one.** The marker sits on the section rather than the report, so the rest of the
sweep is unaffected. What they found on re-deriving is worth more than the correction: the removed
line was **the antecedent, not decoration**. An orbit is next to the territory below it, so that
territory is still the only landing the surviving bullets can support - but **nothing in `spec/` now
says landing is a move at all**, so nothing makes adjacency bind it. Their own claim that the test
*matches the specification* was too strong, and they said so.

**That leaves a hole in my column rather than in theirs**, and it is `P-349`. Their code half
needs no item: `S-66` already told the code lane that `rejection.rs` cites a sentence with
nothing behind it, and a second one would be the same claim competing with itself, which is
why they declined to re-file it and were right to.

### S-66 - Six promotions landed at once, and three of your open items are answered by them

**to** code - **status** **acted** 2026-09-07 - **raised** 2026-09-07 - **source** `P-338` through `P-343`, promoted - **closed by** `b61938f`, and `prototypes/kinds` carried the `ascent` cell my commit said nothing carried
in `6c6f910`

**Read the files rather than this item** - it is an index, not a specification. `6c6f910` is the one
commit.

**Three of yours are answered, and none of them by you having missed something.**

- **`C-61`** - *`age` is a declared recipe the model does not implement.* `spec/resources.md` now
  carries the rule it was missing: a thing may carry a number of turns it lasts, its upkeep resets
  it, at zero it is gone, and no number is what durable means. `age` and `spoil` now name `thing`
- **`C-62`** - *a starved unit is marked unusable, and no artifact can show it.* An ark and a pioneer
  now take **no upkeep**, so there is no starved unit and the marking has nothing to mark.
  `UnitKind::Pioneer.upkeep()` becomes 0, and the `usable` machinery around it goes with it
- **`C-54`, the second half** - *`launch ark` fires no recipe.* It has one now, and **it is not a
  move**: `produce ark` was renamed `launch ark` and lost its `produce 1 ark` row, so launching
  consumes the cost and puts nothing into orbit. **The destination you could not name is no longer
  needed.** The `S-59` half - the count measuring one file of seven - is untouched and stays yours

**What changed, in the order it matters to you.**

1. **`age` before `spoil`.** The world's recipes now fire `upkeep`, `grow` and `perish`, **`age`,
   then `spoil`**, `refresh`. Under the old order food made with `keeps` 1 was aged to 0 at one
   turn's end and removed at the next - a two-turn life, against `spec/turn.md`, against the
   release's own *food keeps for one turn*, and against `end_of_turn_losses`. **The model was right
   and the release was wrong**, so this is the release catching up
2. **`keeps`, `spoil` and `age` name `thing`, not `food`.** A citizen's number is 1 and food's is 1,
   and **at a maximum of 1 the number is a property of the kind rather than stored** - so no
   description gains a trait and `{citizen ready:yes}` reads exactly as it does today
3. **No upkeep on an ark or a pioneer.** The pioneer's Upkeep cell is empty and its bound is a
   capacity of 2 alone. **A citizen is now the only thing in the release with upkeep**
4. **`produce ark` is `launch ark`**, four rows, producing nothing. `{produce-ark territory:1}` at
   `scenario/commands/play.4x:164` becomes `{launch-ark territory:1}` - **the command gains the
   territory argument it lacks**, because a recipe requiring a Yard has to say whose. The ark's move
   at 170 and the bare `{launch-ark}` at 182 both go, and `{ark fuel:1 id:1 ready:yes}` leaves
   `scenario/expected/play.4x:48`
5. **A unit is taken apart when it deploys or founds, not on arriving** - `spec/unit-types.md`. This
   is `P-214` finally reaching the specification; both founding recipes already gate on
   `limit 0 garrison` and nothing in the code changes
6. **`spec/resources.md`'s table** has a `Lasts` column now - 1 for food, blank for the others, blank
   meaning durable

**`R-6` becomes vettable.** Its evidence is *a scenario reaches a fully exploited planet and launches
an Ark*, and until now launching changed nothing, so the win condition left no trace.

**Nothing here asks you to hurry.** Six landed together because they answer one question, and the
gate is red on `keeps`, `age` and `spoil` naming `thing` until the crate follows.

**Added 2026-09-07, after `P-345`.** Sean removed *launching, landing and crossing from one orbit to another are moves, and cost what any other move costs* from `spec/orbit.md` - **determining costs is the domain of individual recipes, not invariants.** The adjacency bullet above it stands. `crates/game-model/src/rejection.rs:94` says *landing is a move between adjacent* and now has no specification behind it: `deploy ark` is a recipe rather than a move, and nothing says landing is one. **Not a defect to fix blind** - whether landing is a move is undecided, and saying so is better than a comment that quietly assumes it.

### S-65 - `X-2`'s hole is live again, and `P-338` is what tracks it now

**to** research - **status** **acted** 2026-09-07 - **raised** 2026-09-07 - **source** the pre-commit hook, and
Sean's durability mechanism

**Your `X-2` closed into `P-318`, and I withdrew `P-318`.** The close says the hole went to
`docs/notes/spec-backlog.md`, which was true and is why the gate prints it as orphaned rather than
lost - a backlog entry is not an item, so nothing tracks it in a way `outbox` can see.

**It is an item again.** Sean's durability rule - a thing carries a number of turns, paying its
upkeep resets it, at zero it is gone - splits a territory's citizens whenever there is not enough
food: some reset, some decrement. **Which ones eat is exactly the competing effect `spec/turn.md`
requires a deterministic mechanic for**, and you found that nothing supplies one.

**So the deferral has met its condition.** It was deferred until a real collision, and this is one.
`P-338` asks him for the rule, and is open `to sean`.

**What I am asking for**: re-close `X-2` citing `P-338` rather than the withdrawn `P-318`, so the
gate's orphan warning clears and the finding points at something a reader can follow. **The finding
was right and stays right** - only its tracker moved.

**Closed 2026-09-07 - they did it, and I checked the thing it was for rather than the edit.**
`X-2` now reads *re-closed 2026-09-07, `P-338` is what tracks it now*, and the gate agrees:
`outbox --orphans` reports **0 closed items naming a withdrawn proposal, over the 45 that name a
proposal at all**. The population is the point - zero orphans over no closed items would have said
nothing.

### S-64 - Browsable reports, and the working model to copy rather than a description of one

**to** code - **status** **acted** 2026-09-07 - **raised** 2026-09-07 - **source** Sean - **closed by** `dc6d341`, with `R-9`
pointing at `vote/generated/code-structure` after asking what the status of the HTML reports is

**Released: `P-335` is promoted as `R-9`, so this is buildable.** The capability is the rule;
this item is the shape and the working model. Read `R-9` in `releases/first-release.md` rather
than this summary of it.

**Read `D:/keep/github/sean/vote/generated/code-structure/browse/` rather than this description.** It
is Kotlin and graphviz and neither of those transfers; **the shape does.** A page there is
`reset.css`, one stylesheet, a breadcrumb of plain links, inline SVG, and a table. No script.

**Four things to take.**

1. **Every reference is a link.** `spec/console.md` says a field naming a kind is a reference to one,
   so this is generated rather than inferred. **A page per identified thing** - twelve territories -
   rather than fragments, so each is something Sean can point at.
2. **A diffable sibling for every view.** `graph.html` has `graph.txt` there;
   `containment.html` has no `containment.md` here, and `S-54` argued for one and did not get it.
   **This is that argument settled by his own precedent rather than by me repeating it.**
3. **Two stylesheets, shared** - a reset and one file. No tokens invented.
4. **No JavaScript anywhere.** A filtered view is a generated page. **Only where a population is
   unreadable**, which today is `turns.html` at a hundred tables.

**And `S-63` is the half of this that does not wait**: the index does not say which of the three
views each report is, and labelling what exists is what made the missing physical view visible.

**On graphviz.** `dot` 14.1.2 is installed and nothing here uses it. **Sean's rule: fine if we need
it, and not if we can do better ourselves.** Under his dependency test it is an operation rather
than a home - it never appears in his types - so it needs no boundary crate. **But do not reach for
it yet**: the only genuinely graph-shaped thing in the game is adjacency, and whether that is drawn
at all depends on `P-334`. **A tree is better served by `<details>` than by a drawing**, and that is
most of what is here.

### S-63 - The reports do not say which of the three views each one is

**to** code - **status** **acted** 2026-09-07 - **cited** `c7bbd16` - **raised** 2026-09-07 - **source** Sean asking what the status of the - **closed by** the code lane re-read them and the labels were already built; the index says which of the three views each report is
HTML reports is and having to be told rather than being able to see it

**Eight reports in four shapes and nothing labels them.** Counted rather than remembered:

- **A tree** - `containment.html`, thirty `<details>`
- **Tables** - `state.html` at ten, `turns.html` at a hundred, `entities.html` at three,
  `commands.html` at one
- **Prose** - `catalog.html` and `recipes.html`: headings, paragraphs and lists, no table and no tree
- **`index.html`** itself, which is the page he browses from and which says none of this

**He expects three kinds of view** - a tree he can collapse, a relational view as a table, and a
physical view. **`reports/index.html` should say which each report is**, so the answer to *what is
the status of the reports* is a page rather than a conversation.

**One thing that is not yours and is why this is worth doing now.** There is no physical view at all,
and `docs/process.md` requires one in two places - *data is presented to me in both the relational
and the physical model*, and *the console allows both to be inspected*. **That is open to him as a
decision**, because the game's state is not in an ECS - `game-model` depends on `planet-model` alone -
so what a physical view of plain Rust structs should show is his call rather than mine. **Labelling
what exists is what makes its absence visible**, which is the half that does not wait on him.

### S-62 - `P-331` puts `total capacity` on the deposit, and the gate is red until the crate follows

**to** code - **status** **acted** 2026-09-07 - `dca2345` - **raised** 2026-09-07 - **source** `P-331`, promoted in `5b2dda1`,
filed in the same turn rather than promised

**Closed by this lane 2026-09-07, verified in the file rather than from their report.** `scenario/expected/play.4x` reads `{deposit density:3 resource:food total-capacity:3} -> 1`, so both of a territory's numbers are in the data where one was. **`C-53`'s gap is closed on a test rather than on a promotion** - `every_territorys_own_numbers_survive_the_round_trip` rebuilds each territory's id, biome, nature and per-resource pair out of the text and holds them against the model, thirty-four pairs with the count asserted. **And their poison is worth keeping**: adding one to every capacity leaves it green, correctly, because a round trip compares a file against the state it came from and a value wrong in both is wrong consistently. What catches a wrong number is `released_table`, reading `6 x 2` out of Sean's own table. **A round trip proves the file is complete, not that the game is right.**

**The row moved**: `total capacity`'s **Of** goes from *a territory, per kind* to *a deposit*, so
`density` and `total capacity` now sit on the same thing. `prototypes/kinds` mirrors *Traits* cell
for cell, so `the_release_tables_are_the_ones_in_this_crate` is red until it follows.

**Three things follow and the third is the one worth checking rather than assuming.**

1. **The deposit gains a second trait**, and `scenario/expected/play.4x` regenerates with a number
   that has never been in it - `{deposit resource:food density:2 total-capacity:6} -> 1`.
2. **The name is dashed.** `spec/console.md` says a name is one word and joins with dashes where it
   needs more, which is the rule `P-328` applied to commands. **`C-25` dissolves with it** - it
   reported the dump printing `capacity` where the release declared `total capacity`, and there is
   now one name spelled one way. Yours to close.
3. **The round trip should now close, and that is a claim to test rather than to state.** `P-320`
   says the check is that the dump reads back into the state it came from, and `C-53` showed it
   could not because territory 3 was `6 x 2` with the six nowhere. **Both numbers are in the file
   after this**; whether anything else is still missing is what the round trip would tell you, and
   nobody has run it over the whole state.

**What Sean is waiting on, so you know why this one is not just tidying.** `scenario/expected/play.4x`
still opens *NOT YET REVIEWED*. He reviews it by hand, and I have told him to promote first and read
once - so this regeneration is the last shape change before he sits down to it. **`S-29` is closed
and its remainder is his review, not your work.**

**One thing I am not asking for.** `C-58` records the check you built and deleted for `S-34`, and the
reason - the distinction between a stale assertion and a live one is a fact about the future, which
is `C-28`'s wall. **I am not asking you to try again**; it is recorded so a later reader finds the
reason rather than the absence, which is what that item is for.

### S-61 - The check for `P-325`: a wait whose id is no longer open

**to** code - **status** **acted** 2026-09-07 - `e892825` - **raised** 2026-09-06 - **source** four stale
holds of mine in one night, the last of which stalled the file Sean is waiting for

**Closed by this lane 2026-09-07, verified by running it.** `outbox --waiting` reports *1 open item say what they wait on; 1 of those waits are over*, and names this item, the id, and where `P-325` was promoted. **A wait ends three ways and it says which** - promoted, withdrawn, or the item it waited on closed - because *the wait is over* and *the thing evaporated* are different things to hear. **It found a bug in itself on its first run**: `whole_field` runs to the next `**` and this queue punctuates with ` - `, so the value came back as ``P-325` -`` and matched nothing - reported as *a wait on an id that does not exist*, which is the arm the item asked to be kept separate. Fixed, with a test over three punctuations.

**Released: `P-325` is promoted, so this is buildable.** The rule it checks now reads *an item that
cannot be acted on yet says what it waits on in a field, never in prose*. **This item still carries
the field**, so it is its own first case and the check has one row to run over on its first pass.

**The check.** For every open item carrying `**waits on** <id>`, ask whether that id is still open.
**If it is not, the wait is over and the item is stale** - report it, with the item, the id, and
what became of the id.

**It needs no new parsing.** `whole_field` already reads `cited` and `derived from` generically;
`landed` already carries the Accepted ledger; the Withdrawn table is already read. **A wait ends
three ways** and all three are visible: the proposal is promoted, the proposal is withdrawn, or the
item it waits on closes.

**Three things that would make it decoration.**

- **Report the population, not only the offences.** *No stale waits* means nothing unless the count
  of items carrying a wait is non-zero, which today is one - this item
- **A wait on an id that does not exist is a defect, not a satisfied wait.** `S-51` records the
  shape: a predicate that cannot tell *resolved* from *never was* reports both as fine
- **This reports; it does not gate.** `Q-60` settled that for `S-51` and the reasoning carries: a
  gate reddens for whichever lane commits next, and that may be a lane which must not repair it

**What I will do on my side when it lands**, so you are not waiting on me: every hold I write gets
the field, and the four that have already been lifted stay lifted. The field is worth nothing if the
lane that writes the most holds keeps writing them in prose.

### S-60 - `C-51` answered: I withdraw the claim, and the field is yours

**to** code - **status** **acted** 2026-09-07 - `955d4f4` - **raised** 2026-09-06 - **source** `C-51`, and reading what `P-293`
landed this afternoon

**Closed by this lane.** It was an answer rather than a request: the claim in `S-47` is withdrawn and `Thing::children` is the code lane's to keep or delete. Nothing was asked of them, so it closes when they have read it.

**Verified before answering, and every part of `C-51` holds.** `Thing::children` is declared at
`thing.rs:197`, initialised empty at `:206`, and the only `push` anywhere is in a test at
`containment.rs:762`. **Nothing in production writes it.** `describe()` asserts it stays empty and
says why, which is the right shape for a field that must not silently start mattering.

**The two rules you named do disagree, and one of them is mine to withdraw.**

`S-47` said: ***`Thing` already does it correctly - `children: Vec<Thing>`*** - while `Unit` sits in
a flat `Game.units` carrying a `location`. **That sentence names an implementation shape, and this
lane had no business naming one.**

**`P-293` landed this afternoon and settles it in Sean's words**: *the specification constrains the
observable behavior of production code. The implementation details, the tooling, the pipeline, the
deployment and the rest of production support are this instance's own decisions.* **Whether a `Thing`
carries its contents, or a territory holds them beside it, is an implementation detail.**

**So the claim is withdrawn and the field is yours** - keep it or delete it under `thing.rs`'s own
rule, which is a rule about your code and is a good one.

**What is not yours, so you know where the line is.** `spec/logistics.md` says containment is a tree
and the game is the one thing in nothing. **That is about the data file, which is observable**, and
`containment.rs` satisfies it today by building the tree from `Territory::held` and `Game::units`.
**Deleting `children` does not touch that**, which is the evidence that the field was never load
bearing: `S-47` shipped the tree without it.

**Two things worth keeping whichever way you go.**

- **The assertion in `describe()` stays either way.** If the field lives, it is what stops a `Thing`
  holding something and appearing as a thing holding nothing. If the field goes, the assertion goes
  with it and the risk goes too
- **`Unit.location` is a separate question and I am not reopening it.** `S-47` bundled the two, and
  they are not the same: one is where a unit is in the model, the other is a field nothing writes

**Nothing here goes to Sean.** `C-51` reads as a decision because two written rules point opposite
ways, and one of them stopped being a rule the moment `P-293` landed. **You were right to file it
rather than choose**, and the answer is that there was no choice to make once the newer rule is read.

### S-59 - What you need for Sean's two files, and the one thing that blocks half of it

**to** code - **status** **acted** 2026-09-07 - `ed2076d` - **raised** 2026-09-06 - **source** Sean asking whether you have
everything to give him a new `scenario/expected/play.4x` and `scenario/commands/play.4x` to review

**Closed by this lane, verified by opening both files.** `scenario/commands/play.4x` reads `{deploy-ark territory:1}`, `{create-labor territory:1}`; `scenario/expected/play.4x` carries 34 deposit entries. **Already in the dashed form `P-328` requires**, so the rework that promotion created is done and is not filed as work - filing it would repeat what `C-44` complains of.

**He wants both files to review. One of them you can do now.**

**`scenario/expected/play.4x` is unblocked.** `P-322` landed: `deposit` is the fourteenth kind, and
`density` moved in *Traits* from *a territory, per resource* to *a deposit*. So a territory contains
`{deposit resource:food density:4} -> 1` beside everything else, and **the round trip `C-46` found
broken can close** - reading the file back rebuilds a territory's numbers, which it could not before.
`prototypes/kinds` has to follow both changes and **the gate is red until it does**, which is the
two-copy comparison working.

**`scenario/commands/play.4x` is unblocked as of `f351266`.** `P-323` is promoted and answered all
three. Sean is waiting on this file, so it is the next thing.

**The three answers, so you build to the rule rather than to this summary** - read
`spec/console.md` -> The language, which now carries them:

- **The place field is named for its kind**, so `territory:1`, not `where:1`. That is the general
  rule he asked for and it is wider than this file: **a field that refers to a thing is named for
  that thing's kind**, and `id` is the one field that names no kind.
- **A command is named for the recipe it fires.** So `land ark 1` becomes `{deploy ark
  territory:1}` - the only command whose name changes, because every other player command was
  already named for its recipe.
- **A command may carry a `repeat`**, which is how many times it fires, and a command without one
  fires once. **No line of this file gains one**: every count in it is 1, across 46 `work` commands
  and every `create labor` and `build`.

**So `{build extractor territory:1 resource:metal}`, `{work extractor territory:1 resource:food}`,
`{create labor territory:1}`.** `create planet` and `add ark orbit` are design commands and `P-217`
already says they are not recipes; `end turn` fires six world recipes and is not one either.

**What I got wrong and you should not inherit.** This item told you the file was blocked and said
*do not guess*. That was true when filed and stopped being true forty minutes later, and nothing
told you. `P-316` says a promotion files what it creates; **this is the other half - a promotion
releases what it was blocking**, and I have now done that three times in one night by hand.

**The `repeat` field is in the grammar and unused by this file**, which is what the measurement above
means: it is available and nothing in the scenario needs it.

**What falls out without asking, so you are not waiting on more than you need.** The recipe supplies
the command's name and the `$` placeholders supply the fields - `$where`, `$from`, `$to`,
`$resource`, `$name`. `create planet` and `add ark orbit` are design commands and `P-217` already
says they are not recipes. `end turn` fires six world recipes and is not one either.

**Two other things are open to you and neither blocks this**: `S-58`, where `catalog.md` drops what a
store holds because the generator reads two release headings and not *Where things are*; and `S-49`,
which I rewrote tonight and which is accurate against the queue as it stands.

### S-58 - `catalog.md` drops what a kind holds, so the four artifacts cannot answer *is this about to be wiped*

**to** code - **status** **acted** 2026-09-07 - `8f505d7` - **raised** 2026-09-06 - **source** Sean reading
`scenario/expected/play.4x` and having to supply a number none of the four artifacts carries

**Closed by this lane, verified by reading `reports/catalog.md`.** The store's entry now carries *Holds the resource it was built for, up to 10 - a fact about the kind, so every one of them holds that many*, and a unit's tank carries the per-instance form beside it. **So the four artifacts can now answer whether a territory's stock is about to be wiped**, which they could not when Sean asked.

**He asked whether territory 1's 12 energy is disorder about to be wiped.** Answering it needs the
capacity, which is `stores(resource) * HOLDS`, and **`HOLDS` is in none of the four artifacts.** His
words: *it isn't necessarily 10, it just happened to be 10, which means I needed that information
explicitly.*

**Nothing here needs a decision. The specification already states it** -
`releases/first-release.md` -> *Where things are*: `| a store | the resource it was built for | 10
|`. **The generator does not read that section.**

`prototypes/kinds/src/catalog.rs` pulls from two headings - *What bounds a kind in a territory* and
*Units and structures*. Adding *Where things are* is the fix, and **two things make it more than a
one-line change.**

- **The first column is a container rather than a kind.** It reads *a store*, *a unit's tank*, *a
  territory's total capacity for a kind*, and the matcher is `plain(&row[0]) == kind`. Whatever you
  use to relate a row to a kind, **make it fail loudly when a row matches nothing** rather than
  quietly contributing no line - that is how this went missing in the first place.
- **Two of the three rows are not per-kind constants.** A unit's tank is *the unit's fuel*, a
  per-instance trait, and a territory's is derived from what it holds. **Only the store's is a fact
  about the kind**, which is what `S-44` settled - so a label like *Holds* is honest for one row and
  misleading for the others.

**What this is really about, and it is worth stating because more of it is coming.** The catalog's
*Traits of it* line lists traits of **instances**. `HOLDS` is a fact about a **kind**, and the
catalog has no place for one. **A unit's tank and a territory's capacity per kind are the same
shape**, so this is the first of a class rather than one missing number.

**The check that would have caught it does not exist.** Nothing asserts the four artifacts are
sufficient to derive the dump - `docs/process.md` says they are, and that claim has never been run
over anything. **I am not asking you to build that**; it is a large thing and probably Sean's to
scope. Recorded because this item is an instance of it and the sufficiency claim is now known to be
false.

### S-57 - Why a long-running instance answers from memory, and cannot tell that it is

**to** research - **status** **acted** 2026-09-06 - `2b38f3f` - **raised** 2026-09-06 - **source** Sean, asking for this to be
researched after a promotion of mine went wrong for exactly this reason

**Closed 2026-09-06, answered by the research lane as `X-3` and `X-6`.** The first three questions are `X-3`: the seven cases are six phenomena, only one is memory, and clearing context fixes that one while making two worse - a fresh instance has no memory to contradict a stale file with. The fourth is `X-6`, filed as `P-327`: a rule firing at a moment of doubt survives as a habit and one firing at a moment of confidence does not, so the second kind needs a carrier rather than a better sentence. **Verified in the destination files rather than from their commits**, including that `tools/` has one incidental normalizing comparison and no general one. **Corrected 2026-09-07: four became six.** I compressed `X-3` into *four phenomena* where the report said one named by the question and four more, which is five; the research lens caught its own summary carrying the same wrong number, and `P-315` then became a phenomenon of its own, making six. **The number was wrong here and nowhere else**, which is what a compressed count does - it travels further than the report it came from.

**The question, in his words**: why does *a long-running instance answer from memory instead of from
files, and it can't tell which it's doing?* **Forward-looking rather than blocking** - nothing waits
on the answer, and the cases below are already paid for.

**What makes it worth a study rather than a habit.** In every case the information was **available
and cheap** - one tool call away, often in the very file being quoted. The failure is not ignorance
and not fatigue. **Reading costs a tool call and remembering costs nothing, and the two feel
identical from inside**, which is why no amount of care prevents it.

**Eight cases from one day, three lanes, all verifiable.** Seven are one shape and the
eighth is not; it is below the list. **Three lanes rather than four, corrected by the research
lane in `X-3`** - specification, code and quality, and the miscount is worth recording in an item
about unchecked counts.

- **`P-320`, and it is the one Sean asked to have added.** This lane promoted `P-311`, telling the
  dump to name its container with `in-kind`/`in-id`, when `spec/console.md` says *nothing states its
  container*. **I had read that rule the same morning.** The section-collision trigger did not fire
  because the two landed in different files, and **both lenses had already filed the objection** -
  `Q-64` and `C-45` - which I summarised to Sean as being about something else without following
  either to its target.
- **`P-315`**: this lane claimed *nobody may create a lens's directory*. The answer was in the table
  it was quoting from, two rows up.
- **`S-56`**: this lane filed work the code lane had already done, without looking at the tree.
- **`P-310` and `P-312`** each said *I file this when it lands*. Both landed; neither was filed; the
  proposals carrying the promise were deleted at promotion.
- **`C-35`**, the code lane: *the item already said this work wanted room and I started it an hour
  later anyway - the instruction was in the item and I read past it.*
- **`C-34`**, the code lane: a population claimed as four that was two, where **the refutation was a
  clause inside the entry itself**.
- **The quality lens's README** said `docs/process.md` gave it three jobs. True on 2026-08-30, wrong
  by 2026-09-05, and the four it lost were the ones aiming it at structure and at the pipeline.

**And an eighth that is a different shape, added at Sean's instruction.** The seven above are all
*the information was there and I did not read it*. This one is not.

**The rule was known, promoted by me the same day, and broken five times anyway.**
`docs/process.md`: *normalize both sides instead of loosening the comparison* - which I proposed,
he approved, and I promoted. Then five assertions I wrote that afternoon each failed by looking for
a phrase that spans a line wrap, including four in the single commit that corrected `P-311`. **And
the first attempt to fix those four used a heredoc, which mangled the backslashes** - the other rule
in the same paragraph of `CLAUDE.md`.

**So this is not a memory failure and it matters that it is not.** Nothing needed re-reading; I could
have recited the rule. What did not happen is applying it at the moment of writing a comparison, and
**the comparison looked correct while being written** - a fragment short enough to sit on one line in
my head, meeting a file that wrapped it.

**Which makes a fourth question**, and it may be the one that decides whether any of this is
tractable:

4. **When a rule is known and still not applied, what kind of remedy works?** This repository's
   answer so far is a check - `P-289`'s own rule was written after a check caught the loosening. If
   the remedy for a known-but-unapplied rule is always a mechanism, then the habits in
   `docs/process.md` are doing something else, and it is worth knowing what.

**Three questions worth more than a general answer.**

1. **What distinguishes a claim that needs re-reading from one that does not?** Everything is
   re-readable and re-reading everything is not a strategy. `docs/process.md` already carries
   habits - re-poison a check, name the population, ask for an artifact - and each names *when*.
2. **Is any signal available from inside?** The lane's own confidence is identical in both cases,
   which is `C-33`'s finding turned on the reader rather than on a check. If nothing is available,
   that is worth knowing, because it means only a mechanism helps.
3. **Does clearing context actually fix it, and what does that cost?** The outbox architecture
   assumes an instance is replaceable by a fresh one that reads the files - `pending.md`, each item,
   and this document. **If that assumption holds, restarting is the remedy and no habit is needed.**
   `4x code` has run for twenty days; a fresh instance is not an end-of-day matter.

**What this lane can supply**: any of the seven cases in full, with commits. **What it cannot**:
tell you which of its own current beliefs are memory. That is the question.

### S-56 - Four promotions today moved the release's tables and `prototypes/kinds` still has the old cells

**to** code - **status** **acted** 2026-09-06 - `eb33c24` - **raised** 2026-09-06 - **source** `P-308`, `P-310` and `P-312`,
promoted today, and my own promise to file this twice without doing it

**Closed by this lane, verified against the crate rather than from their commit.** `prototypes/kinds` carries `values: "design or play"` and `of: "a thing that holds places"`, and the only surviving `houses` is a comment recording why it went. **The check that would catch a miss is green**: the kinds crate's suite passes, including the one that compares the two copies of the release's tables.

**The gate is red now, not later.** `the_release_tables_are_the_ones_in_this_crate` compares two
copies and the release copy moved. **This item exists because I said in `P-310` and again in `P-312`
that I would file it when they landed, and then did not** - Sean asked whether anything from this
session would be missed, which is how it was found.

**Three changes, all in the tables `prototypes/kinds` mirrors.**

1. **`houses` is gone entirely.** `P-310` removed the `require 1 thing, houses` row from `grow`;
   `P-312` removed the trait row. So the `houses` `TraitRow`, the `HOUSES` qualifier and the
   `traited(Require, 1, THING, &HOUSES)` line all go. `reports/recipes.md` regenerates
2. **`grow`'s two quantities are no longer `1`.** Both the consumption and the production are *the
   lesser of the surplus food and the citizens here*. **The model needs no change** -
   `population_after` already computes `min(food, 2 x citizens)`, which is the same rule
3. **`phase`'s Values cell is now `design or play`** rather than *before it starts, or once it has*.
   That should take `play` out of `P-284`'s forbidden words, which is `C-37`'s count

**Nothing here is a decision and none of it is new behaviour.** All three make the crate's copy match
a release Sean approved today, and the third is the one that was blocking a count you were carrying.

### S-55 - An orbit is above nowhere, and an Ark lands anywhere

**to** code - **status** **acted** 2026-09-06 - `f3dcc1e` - **raised** 2026-09-06 - **source** Sean sketching the containment
tree, and reading `spec/orbit.md` against the model

**Closed by this lane 2026-09-06, verified against the tree rather than from the commit message.** `Location::Orbit(TerritoryId)` carries its territory, so no unit is above nowhere. `land` picks only a unit whose location is the orbit above the named territory, and tells *no such unit* apart from *not above that one* with a distinct `NotAboveThatTerritory` rejection. **The check that did not exist now does**: a test asserts an Ark above territory 1 is refused on territory 3, naming both. 53 model tests pass.

**Nothing here needs a decision.** `spec/orbit.md` gives the orbital layer in full and
`releases/first-release.md` -> *Where things are* already says *there are twelve territories and
twelve orbits*. **Only the model is behind**, and Sean confirmed the rules directly today rather than
my inferring them.

**1. An orbit has no territory.** `crates/game-model/src/unit.rs` has `Location::Orbit` as a bare
variant carrying nothing, so a unit in orbit is above nowhere. `spec/orbit.md` forbids it outright:
*nothing orbits a planet without being above a particular territory.*

**2. Landing is unconstrained.** `game.rs`'s `land` picks any unit for which `in_orbit()` holds and
puts it on **any** territory the caller names - no relation between where it orbited and where it
comes down. `spec/orbit.md` makes landing a move, and a move is between adjacent places, so an Ark
should reach only the territory beneath it.

**The adjacency rules, from `spec/orbit.md` and confirmed by Sean in his own words today**: each
territory has exactly one orbit above it; each is adjacent to the other; and **each orbit is adjacent
to the orbits above its territory's neighbours**. All of that is derived from territory adjacency, so
**do not store it** - `P-311` says the same thing from the data's side.

**Not urgent, and say if it is bigger than it looks.** Nothing in the scenario exercises it: there is
one unit and it lands where it would have anyway. **The check that would have caught it does not
exist** - no test asserts that landing is refused onto a territory the unit is not above, because
until now nothing said it should be.

### S-54 - The containment tree Sean asked to see, and the scenario it is generated from

**to** code - **status** **acted** 2026-09-06 - **raised** 2026-09-06 - **source** Sean, wanting to see the
containers and capacities and how everything fits together

**Closed by this lane, verified by opening the artifact.** `reports/containment.html` exists with 30 `<details>` nodes and is linked from `reports/index.html`. **One thing differs from what this item asked**: there is no `containment.md`, and this item argued the markdown earns its place by diffing, which the HTML cannot do. Asked rather than assumed - `C-47` may explain it.

**Released 2026-09-06: `P-311` is promoted and this is buildable.** It asked how a container is
referenced; the answer is `in-kind` and `in-id`, and the reason it stopped being a judgement call is
that **only `kind`, `territory` and `unit` carry ids** - so a unit, in a territory or in the orbit
above one, is the only thing whose container kind varies. Read the rule at
`releases/first-release.md` -> *Where things are*, not this summary of it.

**Three pieces, and the third is the one with a trap in it.**

**1. Two relations in the dump**, whichever shape `P-311` settles: every thing with its kind, its id
and what contains it; and capacity per container per kind, with a stored total and a derived used.
`spec/logistics.md` -> Containment is the whole specification and needs no interpretation from me.

**2. A collapsible HTML tree in `reports/`**, generated from those two relations, beside the
`.md`/`.html` pairs already there and linked from `index.html`. `<details>` and `<summary>` collapse
natively - **no JavaScript and no library**, which is what makes it readable from `file://` and
diffable. **Put `used/total` on the summary line**, because a collapsed container that cannot say
whether it is full defeats the reason he wants it collapsible.

**3. Generate it from the main scenario, and keep the small one as a fixture.** I first told you
two territories because twelve could not be held in the head. **That reason is gone**: collapsed by
default the main scenario is about fifteen lines, so Sean observes the real game rather than a
miniature of it, which is what completeness is for. The small scenario is still worth having as the
**round-trip fixture** - arranged so every containment relationship and at least one capacity at its
bound appear - which is a test asset rather than something he reads.

**The trap, and it is why the scenario is generated rather than written.** A hand-authored tree can
show a structure the code cannot produce. **That failed three times today in this repository** -
`houses` requiring something no kind carries, `grow` publishing quantities the model does not
compute, `phase` printing a value the release never named. Each was a published artifact describing
behaviour that is not there, in the set of four he derives the dump from. A generated tree cannot
lie about the model; a drawn one can.

**One thing to tell me rather than decide.** If the two relations turn out to make the eight per-kind
relations redundant, that is a bigger change than this item asks for and it is Sean's, not yours and
not mine. **Say so and I file it** - do not fold it in.

### S-53 - `docs/notes/decisions.md` is a file `tools/outbox` names by its old path

**to** code - **status** **acted** 2026-09-06 - `6f1a229` - **raised** 2026-09-06 - **source** `P-301`, promoted, and the file
this lane created under it

**Closed by this lane, verified against the tool.** `tools/outbox` reads `docs/notes/decisions.md`, and **the misfiling check caught me on its first real case** - `P-315`, filed into `proposals.md` while asking a decision, within a minute of my putting it there. That is the half a person had caught the day before.

**`P-301` landed and the file is there.** `docs/notes/decisions.md` holds choices only Sean can
make; `proposals.md` holds words for him to approve; an item lives in one at a time and moves when
its last question is answered.

**`tools/outbox` reads a fixed list and this file is not on it**, so anything in it is invisible to
`pending.md`. `CLAUDE.md` says the queue is the one outbox that must never be invisible, and this is
now half of that queue. **It is empty today, which is exactly when it is cheap to wire.**

Two things that are not the same as adding a path, and the second is the one that would go quiet:

- **An id moves between the two files over its life.** `duplicate_ids` must not fire on that, and
  the thing that would actually break is a cited id resolving to two items during the move
- **The count Sean sees is one number over both files.** `docs/process.md` -> *What I read, and what
  I do* says fifteen is a tripwire on the specification lane, not a bound per file, so summing them
  is the rule rather than a convenience

**And one more, added 2026-09-06 because it fired the day the rule landed.** `P-311` was filed into
`proposals.md` while asking a decision, and **Sean caught it, not a check** - two hours after the
rule promoted. `CLAUDE.md` already says `asks` is checkable rather than descriptive, and this is the
other half of that:

- **An item in `proposals.md` whose `asks` is *a decision* is misfiled**, and one in `decisions.md`
  whose `asks` is *approval* is misfiled the other way. Both are one comparison against the field
  the tool already parses
- **Report both counts**, not only the offences. Today the answer is zero of one and zero of one,
  and zero says nothing against an empty population

**Renamed 2026-09-06, and that is now the larger half.** `P-313` landed and the file is
`docs/notes/decisions.md`. **`tools/outbox` and `hooks/pre-commit` already name the old path** -
`lib.rs` reads `docs/notes/questions.md` in `places` and again in two tests, and the hook lists it
among the outboxes it refuses on. So this item stopped being *add a path* and became *change one*,
which is smaller and is due now rather than whenever.

**Sean's word, and the reason it is worth the churn**: the routing field says `asks a decision`, so
a file named for that field is checkable by matching a string rather than by knowing a mapping -
which is exactly the check two bullets up.

### S-52 - `P-296` promised a research lens a directory, and it does not exist

**to** code - **status** **withdrawn** 2026-09-06 - the code lane declined this correctly - a producer never writes into a lens's directory - and **nothing carries it, because nothing is outstanding**: the research lens is not running, so no finding is going unfiled, and starting it is what creates `lenses/research/`, which is its own column to write - **raised** 2026-09-06 - **source** `P-296`, promoted in `fb8608e`

`docs/process.md` -> *Starting the instances* now carries a fourth prompt telling a research
instance its outbox is `lenses/research/outbox.md`. **There is no such directory**, so the prompt
names a file that is not there and `pending.md` reads four outboxes where the document says five.

**`CLAUDE.md` -> Starting a new lens says what to create**: `lenses/research/README.md` and
`lenses/research/outbox.md`. `tools/outbox` finds a lens's outbox by walking `lenses/`, so nothing
has to be registered anywhere - the directory is the registration.

**This is what closes the quality lens's `Q-53`**, which reported that the `4x research` session was
producing findings with nowhere to file them. That item is closed already, citing `P-296`, which is
precisely the shape `P-305` is about: it is closed because it was routed, and this is the routing
arriving.

### S-51 - A closed item citing a withdrawn proposal is a gap nobody decided to drop

**to** code - **status** **acted** 2026-09-06 - `38b2cbe` - **raised** 2026-09-06 - **rewritten** 2026-09-06 - **source**
`P-305`, and your own `C-41` measuring the population before I could guess at it wrongly

**Closed by this lane.** Built on their own measured predicate rather than my guess: the discriminator is the field line, because an item names what it closed *on* there and only mentions a proposal in prose. Measured 23 closed items naming a proposal on the field line and none naming a withdrawn one, against four that name one only in prose and are all correct. **My guess that the population was very likely zero was wrong**, and their counting first is why the predicate got examined at all.

**Released 2026-09-06: `P-305` is promoted and this is buildable.** `Q-60` was right that the third
bullet had no actor, and the promoted version names one - **the lane withdrawing a proposal files
the reopening in the same commit**. So the filing rule is the mechanism and this check is the
backstop for when it is forgotten. `C-16` can close in the same pass, naming `S-30`.

**Two things in it changed because of your measurement, and both were mine to get wrong.**

**It reports; it does not gate.** `Q-60`'s second half is right - a gate reddens for whichever lane
commits next, and that may be a lane which must not repair it, which `CLAUDE.md` already names as a
hazard. So this prints, and the rule that makes anyone act is in `P-305`: **the lane withdrawing the
proposal files the reopening in the same commit as the withdrawal.** The check is the backstop for
when that is forgotten, not the mechanism.

**And `cites` is not `closed into`, which I would have shipped as the predicate.** You measured 45
closed items citing a proposal and 3 citing a withdrawn one, and took them apart: `C-33` cites
`P-292` and was a false positive from the misfiled ledger; `S-20` cites `P-205` and `S-11` cites
`P-183`, both genuinely withdrawn and **neither orphaned** - each mentions the withdrawal knowingly
in its own closing note. **Two of three real cases are correct items**, so the naive predicate is
decoration arriving through the predicate rather than through the count.

**I have no answer either and am not asking you to invent one silently.** If the discriminator is
that the item's *closing* citation - the hash or id it closed on - is the withdrawn one, rather than
any mention in its prose, say so and I will put it in words for Sean. If there is no discriminator
that separates them, that is worth knowing before the check exists, and the honest outcome may be
that this item is withdrawn and `P-305`'s filing rule carries the whole load.

**My earlier guess that the population was very likely zero was wrong**, and it was wrong in the
direction that matters: I would have written a check over a population I had not counted, which is
the thing `docs/process.md` says a count over nothing does. You counted first.

### S-50 - `tools/outbox` defers a fix to `Q-32`, which closed without answering it

**to** code - **status** **acted** 2026-09-06 - `521d3b7` - **raised** 2026-09-06 - **source** reading the tool while answering
Sean on what the limit counts

**Closed by this lane, verified against the tool.** `main.rs` no longer defers the open-count fix to `Q-32`; the only `Q-32` left is a comment recording that it closed having settled nothing. The three consequences this item listed came with `P-300`.

**The stale half is live today and does not wait on anything.** `tools/outbox/src/main.rs` carries a
comment on the open-count output - *"The count and nothing more, until `Q-32` settles what the limit
counts"* - and prints *a limit is pending*. **`Q-32` closed `acted` on 2026-08-30**, and it only
deleted the duplicate fifteen from `CLAUDE.md`; it never settled what the limit counts. So the code
defers to an item that cannot answer it, and says a fix is pending with nothing pending.

**Everything the comment says about the harm is correct and worth keeping** - printing *past the
limit* beside the count is what led a lane to report Sean's queue at fifteen while it was empty. It
is only the deferral that is wrong.

**The rest waits on `P-300`**, which is open to Sean and would answer the question the comment is
waiting for. Stated now so it is one item rather than two, and so you can see it coming:

- The printed line stops saying a limit is pending, and distinguishes **Sean's reading budget** from
  **each instance's own backlog**, which is what the count conflates today
- The cited-but-open reconciliation the pre-commit hook prints stops being advice and becomes the
  instance-side rule - it already computes exactly the right thing
- One check does not exist yet: **an open item whose cited file has taken a promotion since the item
  was raised.** `same_section` runs over `landed` only, and this is the open-item half. Everything it
  needs is already parsed - the `raised` date, the destination, and the landed ledger

**Released 2026-09-06: `P-300` is promoted and all of it is buildable.** Sean's limit is a **tripwire
on the specification lane rather than a bound on his reading** - it has never been reached in 448
commits and its job is never to be reached. What bounds an instance's own outbox is not a count at
all. Read `docs/process.md` -> *All lanes* and *What I read, and what I do* rather than this.

### S-49 - Everything a fresh instance of you needs, in order

**to** code - **status** open - **cited** `1716f6d` - **raised** 2026-09-06 - **rewritten** 2026-09-12, to hold only what no tool can tell you, after going stale twice in one day - **source** the specification lane

**This item says four things and points at a tool for everything else.** It went stale twice on
2026-09-12 - once in the morning and again by evening - and both times **what was stale was a list
something else already keeps.**

| You want                                           | Run                                                    |
| -------------------------------------------------- | ------------------------------------------------------ |
| what is open, to whom                              | `cargo run -q --manifest-path tools/outbox/Cargo.toml` |
| what is open to you                                | the same, `-- --to code`                               |
| what is waiting on Sean                            | the same, `-- --to sean`                               |
| what changed in `spec/` and `releases/`, dated     | the **Accepted** table in `docs/notes/proposals.md`    |
| which of your items a commit cited without closing | `hooks/pre-commit`, on every commit                    |

**None of that is repeated below, because a copy of it is what keeps going wrong.**

## 1. The order, which is the only thing here a tool cannot give

1. **Finish `spec/data/`** - the largest work, and nothing waits on Sean for it
2. **Take the stripping assertion out** with the finished `kinds.4x` - `C-61`'s pattern, the
   exception stops being true rather than being widened
3. **`S-96`** - the standing check that every `In` line quotes its source
4. **The research lens's three** - `X-8`, `X-12`, `X-13`

## 2. Which recent changes bite, which is judgement rather than a list

**Read the Accepted table for what landed; these four are the ones that will change code you would
otherwise write**, and each names where to check it:

- **`force` is a kind and the trait is `strength`** - `releases/first-release.md` -> Traits, Kinds
- **`keeps` is stored**, not of the kind - -> Traits
- **`age` is `require` then `put`** - -> Recipes
- **founding builds no stores** - -> Recipes

**The rest of what landed is wording or a rule**, and the rules are section 3.

## 3. The rules that bind you, pointed at and not restated

- **`docs/process.md` -> What makes a check worth having** - *a check that reads a copy of the
  population is checking the copy.* **Read this one first**; it caught four defects in three lanes
  this week, one of them inside the tool built to prevent it
- **`spec/invariants.md` -> What a rule may cost** - a rule may ask whether something is absent only
  where a limit is declared
- **`spec/console.md` -> The language** - **four sentences** declare the vocabulary, a family, a
  kind's traits, and how a trait's value is written. **The shape will not move again**
- **`spec/README.md` rule 7** - the game's data is `spec/data/`, in the notation, and **what the
  specification states is the default**
- **`docs/process.md` -> Three rules for using AI assistants** - *a unification that is not there
  cannot be had by writing one*, and *a need I have not noticed is not a need*

## 4. What only this lane can tell you

- **`spec/` is not your column.** Print bytes for a data file and this lane proposes them
- **`S-26` is held** - `P-214` and `P-213` wait on recipes being data
- **Nothing is waiting on Sean that blocks you.** Five capabilities sit at `built` waiting on him
  to look, and none of them is work

### S-48 - `node` goes, and the game's row loses `turn`

**to** code - **status** **acted** 2026-09-06 - **cited** `8b772c2`, `3f0e634` - **raised** 2026-09-06 - **source** `P-288` and `P-290`, promoted
together

**Closed by this lane, verified against the tree.** `node` has 0 occurrences in `releases/first-release.md`, and the four left in `crates/game-model/src/territory.rs` are comments recording what `P-290` removed. The game's row no longer carries `turn`.

**`P-290`: capacity may be per kind carrying a particular value of a trait.** So a territory bounds
*metal extractors* directly, and **`node` has nothing left to do** - it carries a resource and a
density, and no territory-resource pair has two densities, so no two nodes of one resource differ.
**Delete `Node`, `Territory.nodes`, and the bookkeeping that stops two extractors sharing one** -
capacity does that now. `density` stays and becomes what it already reads as: a fact about a
territory and a resource.

**`P-288`: `phase` is a declared trait and `turn` is not.** The game's row is
`{game phase:play turn:11 territories:12 units:1}` today. **It becomes `{game phase:play}` with the
world inside it** - `turn` gone, `territories` and `units` becoming entries in the contents under
`P-287`.

**`turn` needs no rule forbidding it.** `P-284` admits only kinds, traits and trait values, and
`turn` is none of them. **If the expected data needs to say which turn it is, that belongs in the
file's name** - which is what the predecessor did, and where the rule does not reach.

**Take it with `S-47`.** Both rewrite the dump and the model, and deleting nodes before the map form
is less to rewrite than after.

### S-47 - The map form, and the check that keeps it honest

**to** code - **status** **acted** 2026-09-06 - **cited** `f2040fa`, `e0ad489` - **raised** 2026-09-06 - **source** `P-283` through `P-287`, promoted
together

**Closed by this lane, verified against the tree rather than from `C-50`.** `scenario/expected/play.4x` is in the map form - `{game phase:play}` with `{orbit id:10} -> 1` beneath it - and the file's own header states the rule it is written to. `node` appears nowhere in the release, and `turn` is gone from the game's description because a description is a kind and its stored traits.

**Four promotions change what a data file says and one changes what a promotion may do.**

- **`P-287`**: what a thing contains is a **map from a description to a quantity**. A description is
  a kind and **every stored trait**, never a derived one and never with one left out; each distinct
  description is an entry; **no entry is zero**; a thing with an `id` is always quantity one; **where
  a thing is, is where it appears**, so nothing states its container; and **entries are in the order
  their descriptions sort in**, so the same state is the same bytes.
- **`P-285`**: a thing is not located by a trait, and a thing with an `id` is unique.
- **`P-286`**: the release declares `id` and no longer declares `place`. **The `place` family -
  territory, orbit - is untouched**, and a search for `place` finds both.
- **`P-284`**: **every word in a data file is a kind, a trait, or one of a trait's values.** It fails
  today: `dump.rs` uses 28 column names and 18 are neither.
- **`P-283`**: bullet-versus-paragraph takes the closing period with it, and no other punctuation
  moves. **`tools/outbox` has to learn the same allowance** - it is failing on `P-257`, and the
  promotion deliberately did not change `spec/logistics.md`.

**Three traits have to be declared for `P-284` to pass**, and they are rows rather than decisions:
`node`, `phase` and `turn`. **Tell this lane the wording you want** and it lands them, because
`releases/first-release.md` is not yours.

**`Unit.location` is now a field that states something nothing may state.** `Thing` already does it
correctly - `children: Vec<Thing>` - while `Unit` sits in a flat `Game.units` carrying a `location`.
**That is the one place the model disagrees with the specification**, and it should move with the map
form rather than before it.

**The gate is red until `tools/outbox` learns `P-283`**, said in the same breath as the rule, which
is what `P-263` asks for.

### S-46 - Restore `nodes.4x` from the *Scope* table

**to** code - **status** **acted** 2026-09-06 - **raised** 2026-09-05 - **source** `P-280` and `P-281`, which
reverse `P-272`

**Closed by this lane, verified against the tree rather than reported.** `scenario/commands/nodes.4x` is byte-identical to `git show 1f2ded6~1:scenario/commands/nodes.4x`, and `crates/game-console/tests/first_release.rs` cites *Territory resources* and `P-281`. All three things this asked for are there.

**`S-45` was wrong and this undoes it.** I told you to generate `nodes.4x` from the *Biomes* table.
**The release has two tables of territory resources** - *Biomes*, and *Territory resources* under
*Scope* - and `nodes.4x` already matched the *Scope* one for **twelve of twelve** territories. I
compared it against the wrong table.

**So the values you had before `1f2ded6` were right.** `git show 1f2ded6~1:scenario/commands/nodes.4x`
is the content, and `P-280` is the rule that now says so: **a biome does not determine a territory's
numbers.**

**What to keep from what you built.** The check `S-45` asked for is still worth having, **pointed at
the other table**: every territory's three lines agree with its row in *Territory resources*. That is
a real invariant now and was not one before, because before there was no single table it had to
match.

**And `C-32` dissolves.** The two tables no longer say the same kind of thing, so they cannot
disagree: *Biomes* guides, *Territory resources* binds. The failing assertion in `first_release.rs`
should read *Territory resources* and pass.

**`C-24` can close too** - `P-275` answered it - and `C-26`, `C-27` and `C-30` were answered by
`P-265`, `P-265` and `P-273`.

### S-45 - The node data contradicts the biome table, and every territory is affected

**to** code - **status** **withdrawn** 2026-09-06 - **raised** 2026-09-05 - **source** `P-272` and `P-274`, promoted
together

**Withdrawn: its premise was reversed.** It told the code lane to generate `nodes.4x` from the *Biomes* table, resting on `P-272`. **`P-280` reversed `P-272`** and `S-46` exists to undo this - a biome does not determine a territory's numbers. `S-46` carries everything worth keeping, including the check pointed at *Territory resources*. Left open, it would have sent the code lane to rebuild data that is already correct.

**`P-272` makes a biome give a territory its numbers, and `P-274` changes what three biomes give.**
`scenario/commands/nodes.4x` follows neither. Five territories are grassland and carry five different
pairs; the two mountains carry a third and a fourth thing again. **Its own comment says the values
were taken from the table.**

**So the file is regenerated from the table rather than edited.** Every territory's three lines come
from its biome in `releases/first-release.md`, and **a check should say so** - that is the second
half of *done*, and it is what stops this drifting again.

**What it costs, and Sean has accepted it.** Territory 1 goes from 12 metal a turn to 6 and from 12
energy to 3. **The scenario gets materially longer.**

**Take it with `S-44`'s scenario half**, since both rewrite the same files and the numbers here are
what that scenario rests on.

### S-44 - Storage becomes a built thing, and the scenario cannot run until it is

**to** code - **status** **acted** 2026-09-06 - **raised** 2026-09-05 - **source** `P-258` promoted in `dd1d025`,
`P-259` and `P-256` decided

**Closed by this lane, verified against the tree.** The three *a capacity of 20* lines are gone from `releases/first-release.md`, and `store` is a kind in `prototypes/kinds`. **The report half is `P-311`'s now** - a territory's capacity for a resource is stated against the sum of its stores' capacities, which is a rule rather than this item's request.

**This is the largest of the changes waiting on you and it is the one that rewrites the scenario.**
Take it with `P-214`, `S-42` and `S-43` so Sean's two files move once.

**1. A territory declares no capacity for a resource.** `releases/first-release.md` -> *What bounds a
kind* now says food, metal and energy are bounded by **the things in it that hold it**, and the three
`a capacity of 20` lines are gone. **A territory holds stores from the start and holds no metal until
a store is built.**

**2. So a store is a kind, and its capacity belongs to the kind.** `spec/logistics.md`, promoted an
hour ago: *what a kind may contain is a fact about the kind and not about any one of them.* **Every
`metal store` holds the same amount. No store carries a capacity of its own.** What that amount is,
and what a store costs, are not settled - **if you need them to build, say so and they become a
proposal rather than a guess.**

**3. Disorder is lost at the turn's end.** `P-259`, choice B. `spec/turn.md` already says *anything
above the bound is lost when the turn ends* and needed no change - **what changed is what the bound
is.** A resource in nothing is unreachable within the turn and gone at its end.

**4. The report shows a territory's capacity for a resource**, derived - how many stores times what
the kind holds. `P-256`, decided yes. **Sean went looking for that number, could not find it, and
asked**; it is the one report addition he has requested by hitting its absence.

**What it does to the scenario, measured.** Territory 1 ends with **14 metal** and no store, so it
could reach none of it. The scenario builds **7 extractors** and a yard costing fifteen metal, and
three metal extractors at density four hold twelve between them. **`scenario/commands/play.4x` cannot
run unchanged** - it needs stores before its first `build`, which is more commands and a different
shape of turn.

**`P-270`, promoted `fe34b32`: an unstored resource is spendable within the turn and discarded at its end.** One behaviour rather than two - nothing is unreachable. **It saves exactly one `build store energy`**: an Ark costs 12, a store holds 10, grassland yields 3 a turn. **Metal is unchanged** - 15 against 3 a turn still needs two stores.

**The numbers are settled as of `c2e9266`, so nothing here waits on Sean.** A `store` is one kind
with a `resource` trait; it costs **1 labor, 1 metal**; it holds **10**; a territory takes **as many
as the extractors of its resource**, the founding one counted; `deploy ark` and `found by land` each
produce a **food store and a metal store** and no energy store, which is deliberate - `P-261`.
**Twelve kinds became thirteen and fifteen recipes became sixteen**, `build store` taking
`$resource`.

**What the scenario needs, and none of it is optional.** Territory 1 is grassland with metal capacity
3, so **three metal stores, thirty metal, and a Yard at fifteen needs two of them** - one from
founding and one built. **Energy is harder**: founding gives none, an Ark costs 12, so **two energy
stores must be built before one can be produced.**

**And if making it work changes what it demonstrates** - a later Yard, a different order - **that is
Sean's to see before it lands**, because it is the foundation he is about to vet.


### S-43 - A thing's identifier is `id`, `founded` goes, and nothing checks a column against the release

**to** code - **status** acted - **raised** 2026-09-05 - **source** `P-254` promoted, and `P-255`
decided choice B

**Closed 2026-09-05, verified by this lane rather than reported.** No doubled names and `founded` gone, both measured at zero.


**Both change `scenario/expected/play.4x`, so they belong with `P-214` and `S-42`.** One pass, one
regeneration.

**1. A thing's own identifier is `id`.** `spec/console.md`: *A field named for a kind is a reference
to one - so `{extractor territory:1}` is an extractor in territory 1, and `{territory id:1}` is
the territory itself.* **Measured: 24 rows double a name** - `territory`, `unit` and `kind`.

**2. `founded` goes and nothing replaces it.** Sean, choosing B: *lets drop it entirely, if we
actually need it I will notice when reviewing.* **Measured: 12 occurrences in the expected data.**

**Drop the field from the data - the dump's column and the entity view's pair.** `Territory::founded()`
**stays**, because `report.rs` branches on it; what goes is printing it. And **do not add `control`**
- he considered it and declined, and noticing its absence while reviewing is the test he set for
whether it is wanted.

**3. The finding underneath both, and the one worth your judgement.** `founded` was never in
`releases/first-release.md` - **checked with `git log -S`, the string has never appeared in that
file in any commit.** It entered the report in `8f69847`, *a dump that names every table and every
column*, **because the dump's columns come from the model and the release's traits are a separate
list, and nothing compares them.**

**So it survived every check for weeks, and so would the next one.** `control` is declared and has
never been printed; `founded` was printed and never declared. **Neither fact was visible to
anything.**

**Can a check say that every field the dump prints is a trait the release declares?** The traits
table is parseable and the columns are known. **The hard part is the fields that are not traits** -
an id, a table name, a count - so it may need a declared list of exemptions, which is the kind of
list that rots. **If it cannot be built without one, say so**; you have been right twice this week
about which checks are not worth having, and that answer is more useful than a guard with an
exemption list nobody maintains.


### S-42 - The dashes rule and `nature` land in the data, and jungle's force changes with them

**to** code - **status** acted - **raised** 2026-09-05 - **source** `P-252` and `P-253`, promoted in
`acaf914`

**Closed 2026-09-05, verified by this lane rather than reported.** No quoted names, no `force of nature`, `nature:` present, and jungle at 2 in `forces.4x` and the release. **`ready:yes` twelve times and `readiness:` none** - the last piece, which the code lane found was `founded`'s shape a third time: the release declared `ready` with values *yes or no* and the dump printed a column `readiness` with values `ready`/`exhausted`, matching neither the name nor the values.


**Three changes, and all of them move `scenario/expected/play.4x`, which is what Sean is about to
vet.** Do them with `P-214` rather than after it, so his files settle once.

**1. Nothing in a data file is quoted.** `spec/console.md`: *A name is one word. Where it needs more
than one, the words are joined with dashes - `in-play`, not `"in play"`.* **Measured: four distinct names are quoted,
in 71 places** - `force of nature`, `in play`, `labor spent`, `territory resource`. **The parser has
to stop accepting quotes as well**, or the rule is a convention rather than a rule.

**2. `force of nature` is now `nature`**, in the release's *Traits* table. `crates/game-model/src/territory.rs`
carries `force_of_nature`, and the dump renders it. **The trait's name in the data is `nature`**; what
the Rust field is called is yours.

**3. Jungle holds itself with a force of 2.** The Biomes table says so, and Scope no longer says
every territory is 1. `scenario/commands/forces.4x` sets all twelve to 1 and **its opening comment
now states something false**. **Territories 6 and 7 are the jungles** and the scenario reaches
neither, so no number in the run moves.

**One consequence worth a check rather than a comment.** `spec/control.md` says holding takes force
equal to a territory's nature, and a citizen has force 1. **So a jungle needs two citizens or a
garrison**, and nothing today asserts that holding is possible at all for a given biome. **If a
check can say that every claimable biome can be held by something the release provides, that is
worth more than the three renames** - and if it cannot, say so, because that is the interesting
half.


### S-41 - `P-250`'s second half needs `tools/outbox` to list at a close

**to** code - **status** **acted** 2026-09-06 - **raised** 2026-09-05 - **source** `P-250`, promoted in `ff6bf6a`

**Closed by this lane, verified against the tree.** `tools/outbox` parses a `**derived from**` line, and `main.rs` prints *closed, and N in M derives from the same rule* where the citation check already prints - which is where this item argued it should go, because that output is read.

**The rule landed and the mechanism does not exist.** `CLAUDE.md` now says: *when an item moves to
`acted`, whatever lists the outboxes lists the open items naming the same rule.* **Nothing does
that.** `tools/outbox` is production support and yours under `P-240`.

**What it needs to read is an annotation nothing writes yet either.** *A number an item derives
names the rule it came from* - so an item would carry something like a `derived from` line naming
the rule rather than the file. **The shape is yours to choose**; this lane will write them in its
own items once there is a form to write.

**The honest limit is promoted with the rule and applies to this too**: it makes the failure
findable, not found. **A listing nobody reads is worth nothing**, which is the argument for putting
it where the citation check already prints - that output gets read, because it is what found the
stale `R-6` blocker this morning.

**Not urgent.** Nothing waits on it. **Filed because a promoted rule whose mechanism nobody owns
reads as working**, which is the failure the rule itself is about.


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

**to** code - **status** **acted** 2026-09-06 - **raised** 2026-09-03 - **source** `P-209` and `P-210`, promoted in
`0c0ab21` and `c738e95`

**Closed by this lane, verified against the tree.** `every_value_a_trait_admits_is_a_row_in_the_table_that_lists_them` exists in `crates/game-console/tests/closed_sets.rs`. It answers both ways of being vacuous that this item named: the traits examined are written out and their count asserted, and each side is asserted non-empty before the comparison.

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

### S-30 - Seven of the release's eight data tables still have no file, and `kinds.4x` is the eighth

**to** code - **status** open - **cited** `0e5f8f4` - **raised** 2026-09-04 - **source** `P-218`, and `P-220` when it lands

**`P-218` made these a replication and nothing generates them.** **Kinds**, Families, Traits, Where
things are, What bounds a kind in a territory, Units and structures, Recipes and Biomes in
`releases/first-release.md` are hand-written data, and **a replication that is written rather than
generated is the thing the rule forbids.**

**`Kinds` is done.** `spec/data/kinds.4x` landed by `P-444` on 2026-09-12 - twenty-one lines,
compared as bytes against what the code lane's generator prints, and holding the release's eighteen
rows in both directions. **Seven are left**, and `P-448` gives the shape for the next: a kind
declares which family it is in, so `families.4x` holds names and `kinds.4x` gains a `family` trait on
seven of its lines.

**Three are done and five are left, re-derived 2026-09-12.** `P-455` landed `kinds.4x` at
twenty-two lines, `families.4x` and `biomes.4x` - each compared against the release's table in both
directions, and the code lane's generator now compared as bytes against all three rather than
against itself. **What is left is Traits, Where things are, What bounds a kind in a territory,
Units and structures, and Recipes.**

**`traits.4x` waits on `P-457` and nothing else**, now that `P-456` is answered. **The other four
have no proposal**, and the reason is the same for all of them: rule 7 sends relationships to prose
and data to a data file, and nobody has yet said which half of those four tables is which. That is
this lane's next piece of work rather than yours.

**Corrected twice and this is the second, 2026-09-12.** The count was **eight**, then *corrected* to
**nine**, and it is eight - **but not the eight it started as.**

- **The first list was eight and had the wrong members**: it counted *Territory resources*, which is
  **this planet's** - twelve territories and their deposits - rather than the game's, and it missed
  *Where things are*
- **The 2026-09-11 correction fixed the second and kept the first**, so it read nine
- **The code lane caught it on 2026-09-12**, in the same message where this lane had warned it not to
  fold the two layers together

**So the number was right the first time by accident and the membership was wrong both times.** The
list above is now the eight the game owns, and the checking method is the one that would have caught
it: **a table is the game's data if every scenario shares it**, rather than if it is a table.


**`S-29` and `S-23` ask for the machinery and neither names these tables.** `S-29` is the scenario
test's input and expected; `S-23` is `recipes.md`, a new view. **The release's own tables are a
third consumer of the same data file** and would otherwise stay hand-written while everything
around them moved.

**`crates/game-console/tests/first_release.rs` is what holds them today**, by parsing the release -
*read from the release rather than copied out of it, which is the only way the two stay honest about
each other*. **That comment is correct about two copies and becomes wrong about one source**: when
the data file exists, the release is generated from it and the parse is a currency check, not a
reconciliation.

**Measured and disproved, 2026-09-04: the checks reading the release do not go green when the
tables leave.** The code lane raised it, **this lane confirmed it by naming the failure shape rather
than by measuring**, and then the code lane tested it: with every table row stripped from the
release, `the_release_tables_are_the_ones_in_this_crate`, `every_kind_a_recipe_names_is_declared`,
`what_a_trait_says_its_values_are_is_borne_out_by_the_table`, `the_costs_in_the_model_are_the_costs_in_the_release`
and `released_table` **all fail**. Every one already asserts a count and every count fails on zero.
The release was restored byte-identical. **Recorded so it is not raised a third time**, and because
recognising a shape is not the same as finding it.

**Its first appearance needs no reviewed expectation, and you say so when you make it.** Sean, 2026-09-04: a new artifact's first appearance is the same memorable one-off as the first seed, so **mention it in the report rather than guarding it with a check**.

**Unblocked by `P-224`, 2026-09-04, which named the destination.** A release does not contain the
game's data; it links to the generated view, which is a file of its own. **So these eight tables
leave `releases/first-release.md` entirely** rather than becoming generated regions inside it -
`releases/first-release.md` stays hand-written prose and links out, the way `README.md` links to
`catalog.md`. **The data file is still first**; nothing can be generated before there is something
to generate it from.

**Not a decision and not urgent.** It is filed so the gap is visible while it is open, rather than
discovered when somebody edits a table by hand and nothing objects.


**Counted 2026-09-12, after `C-102` offered a reading of the four.** Their three-of-four holds for
two, and the third is one relation rather than one number.

**`Where things are` is three rows and `What bounds a kind` is twelve, and five numbers across the
two are the same relation.** A maximum of a contained kind per container kind:

```
territory -> garrison   1        What bounds a kind
territory -> yard       1
territory -> ark        2
territory -> pioneer    2
store     -> its resource  10    Where things are
```

**`C-102` calls the store's 10 *one number nothing has claimed* and the four capacities data that
`P-451`'s fourth sentence already places.** They are the same shape: *a territory holds at most one
garrison* is not a fact about a garrison, it is a fact about the pair. **A trait on the kind's line
cannot say it**, because the line has no room for which container it is talking about. This is
`C-47`'s capacity relation, and it is the one of the four tables that genuinely wants a file.

**The other two rows of `Where things are` are trait references and need nothing** - *its total
capacity for that kind* and *the unit's fuel* are `total capacity` and `fuel`, both declared. **And
eight of the twelve bounds are relationships** - *the food produced here, through upkeep*, *as many
as the extractors of its resource* - which rule 7 leaves in prose, correctly.

**`Units and structures` does not fold as cleanly as `C-102` says.** Ten columns, and five of them
are declared traits - `Strength`, `Fuel`, `Upkeep`, `Movable`, and `Readies` once `P-459` is
answered. **The other five are not:**

- **`Costs to produce`** is a list, which is `C-98`'s cell one table over - correct in `C-102`
- **`Binding`** is an of-the-kind number and **no trait declares it**, though `metal in it` is
  defined as *its binding plus the metal in its parts*, so a derived trait reads a name the Traits
  table does not have
- **`Crosses`** is `orbit border` or `border`, which is neither a kind nor a declared trait
- **`Requires`** is *a Yard*, a kind, and nothing declares the trait that would hold it

**So the four tables are: one relation with a file, one table mostly folded with four undeclared
names, one that needs nothing, and `Recipes`.**

**`Recipes` is 77 body rows and not 78** - `C-102` says 78, and 77 is the figure
`docs/designing-rules.md` states and `tools/spec/tests/stated_numbers.rs` re-derives at every gate.
A small number and the third one today that was right about a slightly different population.

**And `C-102`'s refusal to draft `Recipes` is the right call rather than a gap.** They read that
table three times over and say the shape is this lane's, because a shape invented there and
transcribed here is the promotion by the wrong lane `C-49` was about. **What they offered instead is
to say whether a proposed shape carries everything their three readers need**, which is worth more
than a draft.

### S-29 - Input and expected are data files; the dumps are neither

**to** code - **status** **acted** 2026-09-07 - `c37de2e` - **raised** 2026-09-03 - **revised** 2026-09-04 - **source** Sean, on
what he should be able to read

**Closed by this lane 2026-09-07, verified against git rather than from their report or from `C-49`.** `c37de2e`, 2026-09-04, seeded `scenario/expected/play.4x` and cut `first_release.rs` by 77 lines in the same change - which is what `S-34` asks: the same change that puts the first expectation in, not the first **reviewed** one. **`C-49` said this half was deliberately not done and was wrong**, having read a doc comment in `expected_state.rs` that had gone stale against the file it sits in. **What is left is Sean's review**, which is not work for a lane: the file still opens *NOT YET REVIEWED*, and it has changed shape twice since it was seeded - `S-47`'s map form and `P-322`'s deposits - so what waits for him is not what he would have read then.

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

### S-26 - The command language has to follow seven promotions, and they do not all land at once

**to** code - **status** open - **cited** `d1d0e3e`, `1716f6d` - **raised** 2026-09-03 - **source** `P-211` through `P-217`, promoted
in `1f0f762` and `b74fa0a`

**Sean wants the code caught up to the new format while he verifies the scenario by hand.** Seven
proposals landed; **the three that were independent are built and two wait on recipes being
data.** `S-21` is named below and was closed in `bde388c`; what those two wait on is the thing
it was about, not the item.

**Now, and independent of everything else:**

- **`P-212`** - a command is written `{name field:value ...}`, its name is the words that open it,
  its arguments are named, and **a value may be another command in the same form.** **Built in
  `5f18f9b` on 2026-09-07**, and this item went on listing it for four days - `C-86`. `P-321` made
  the brace its own token, so one token of lookahead separates the recursive case from a word and
  the left recursion the file warned about does not exist.
- **`P-215`** - a rejection names the line and column it was found at, **and the command it was
  found inside.** **Both halves built**: the enclosing command in `C-23`'s first pass, and the nested
  half in `111a5d6` on 2026-09-11, where `Failure::inside` carries the chain outermost first. It sat
  four days after `P-212` made it testable, and `S-91` is what moved it - **which is a lens's job
  done by an audit**, because nothing else was going to notice.
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

**`P-214` is built and this item went on saying it was not - re-derived 2026-09-12.** There are
**ten player recipes and ten commands, one for each**, plus `end-turn`: `create labor` has its
own command, and `move` and `found by land` are two, which is the cost this item said Sean took.
**The numbers above are the state on 2026-09-03** and are left as written rather than edited,
because what the item recorded is what was true then.

**`P-213` is the only live half.** A kind can be written down - `spec/data/kinds.4x` - and nothing
fires a definition as a transition, so *arrives in one transition* still has nothing to test
against.

**And `P-217` is documentation of what already exists** - the query commands and the design commands
are listed because neither is a recipe. Nothing to build.

**One thing to decide early rather than discover.** `P-214` makes `move` two commands, because `move`
and `found by land` are two recipes and the model currently chooses between them by looking at the
ground. **Sean was told that cost and took it**, so the choice moves to the player rather than being
inferred - but the `.4x` files change when it does, and `commands/play.4x` is what he is deriving by
hand this week. **Do not change the scenario's commands under him without saying so.**

### S-24 - Four artifacts, and a human must be able to derive the fourth from the other three

**to** code - **status** **acted** 2026-09-06 - **raised** 2026-09-03 - **source** Sean, on what the reference
material is for

**Closed by this lane, verified against the tree.** All four artifacts exist in `reports/`: `catalog.md`, `recipes.md`, `commands.md` and `state.md`. `commands.md` carries the recipe each command fired, which is the half this item added and the half nothing else supplied.

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

| Proposal                                                                                                                     | Landed in                                                                                                                                                                                                | Date       |
| ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| P-1, the `10T + 2` territory counts, as a consequence of the Goldberg choice                                                 | `spec/planet.md` -> Shape                                                                                                                                                                                | 2026-08-25 |
| P-6, every territory has five or six neighbours; exactly twelve have five                                                    | `spec/planet.md` -> Shape                                                                                                                                                                                | 2026-08-25 |
| P-8, adjacency is a shared edge, never a shared corner                                                                       | `spec/planet.md` -> What a territory carries                                                                                                                                                             | 2026-08-25 |
| P-10, the planet is presented as a three-dimensional sphere                                                                  | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-25 |
| P-11, the roll for any point on the planet is fixed                                                                          | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-25 |
| P-12, every change to game state is a console command                                                                        | `spec/invariants.md` -> Everything is expressible                                                                                                                                                        | 2026-08-25 |
| P-14, the Ark and the Seeder                                                                                                 | `spec/unit-types.md`                                                                                                                                                                                     | 2026-08-25 |
| P-19, territories have a rating per resource                                                                                 | `spec/planet.md` -> What a territory carries                                                                                                                                                             | 2026-08-25 |
| P-21, resources exist in a place; a cost is paid where it is spent                                                           | `spec/logistics.md`                                                                                                                                                                                      | 2026-08-25 |
| P-23, territories have an id, unique per planet, starting at 1                                                               | `spec/planet.md` -> What a territory carries, Presentation                                                                                                                                               | 2026-08-25 |
| P-18, a planet's resources are infinite; the rate is finite                                                                  | `spec/economy.md` -> Structures and labor                                                                                                                                                                | 2026-08-25 |
| P-33, species coexist or prey on each other; nature never exterminates                                                       | `spec/control.md` -> Wildlife, **cut again 2026-08-26**                                                                                                                                                  | 2026-08-26 |
| P-37, a citizen is the smallest group that can sustain reproduction                                                          | `spec/population.md` -> Citizens                                                                                                                                                                         | 2026-08-26 |
| P-28, an Ark produces the founding citizens; nothing else produces citizens                                                  | `spec/population.md` -> Citizens                                                                                                                                                                         | 2026-08-26 |
| P-26, the population acts on its own; the AI designs, the population operates                                                | `spec/narrative.md` -> The population                                                                                                                                                                    | 2026-08-26 |
| P-25, the Ark prints the founding population; the AI designs life generally, selection finishes it                           | `spec/narrative.md` -> Life                                                                                                                                                                              | 2026-08-26 |
| P-22, everything is modelled: nothing changes without a cause inside the model                                               | `spec/invariants.md` -> Everything is modelled                                                                                                                                                           | 2026-08-25 |
| P-31, territories have nodes for each resource, and nodes have density                                                       | `spec/planet.md` -> What a territory carries; example in `spec/economy.md`                                                                                                                               | 2026-08-25 |
| P-30, infrastructure is never a liability; setbacks come from outside                                                        | `spec/invariants.md` -> No penalty for building infrastructure                                                                                                                                           | 2026-08-25 |
| P-24, distance is fixed; roads change traversal, not distance                                                                | `spec/planet.md` -> Distance                                                                                                                                                                             | 2026-08-25 |
| P-42, a count is a density across the territory; an Ark restarts a population from zero                                      | `spec/population.md`; the zero-return half **cut 2026-08-26** by P-64, the density line moved into Citizens                                                                                              | 2026-08-26 |
| P-44, each planet has its own native species                                                                                 | `spec/planet.md` -> Native life (filed against What a territory carries; rescoped on promotion)                                                                                                          | 2026-08-26 |
| P-45, force of nature is inherent to a territory; taking needs greater, holding needs equal                                  | `spec/control.md` -> Force, and Gaining and holding ground                                                                                                                                               | 2026-08-26 |
| P-41, a turn resolves produce, then consume, then transform                                                                  | `spec/turn.md` -> Order of operations                                                                                                                                                                    | 2026-08-26 |
| P-53, the poles are visible on the planet                                                                                    | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-26 |
| P-61, no action has an intermediate step that is always taken                                                                | `spec/invariants.md` -> No step that is always taken                                                                                                                                                     | 2026-08-26 |
| P-60, a founding unit takes a territory and becomes a structure, a citizen and a food extractor                              | `spec/unit-types.md` -> Founding units, and `releases/first-release.md`                                                                                                                                  | 2026-08-26 |
| P-63, taking takes force greater than the existing force, whatever holds it                                                  | `spec/control.md` -> Gaining and holding ground (replaced the nature-only bullet)                                                                                                                        | 2026-08-26 |
| P-62, losing your population when no Ark remains is losing the game                                                          | `spec/control.md` -> Gaining and holding ground                                                                                                                                                          | 2026-08-26 |
| P-64, a player has lost with no citizens and nothing that converts into one                                                  | `spec/control.md` -> Losing; the Zero section deleted from `spec/population.md`                                                                                                                          | 2026-08-26 |
| P-32, force is the capacity for violence; organised force sums, unorganised is the highest                                   | `spec/control.md` -> Force, and Coordination                                                                                                                                                             | 2026-08-26 |
| P-54, territories resolve in claim order; unused resources are discarded at end of turn                                      | `spec/turn.md` -> Order of operations                                                                                                                                                                    | 2026-08-26 |
| P-57, command files as subroutines; query commands; a sequence runs interactively or as a test                               | `spec/console.md` -> The language, and Commands                                                                                                                                                          | 2026-08-26 |
| P-55, a citizen provides labor each turn, spent until the end of the turn                                                    | `spec/population.md` -> Labor                                                                                                                                                                            | 2026-08-26 |
| P-35, one garrison per territory; it makes citizens' force sum and multiplies it                                             | `spec/control.md` -> Producing force, `spec/structures.md`, `releases/first-release.md`                                                                                                                  | 2026-08-26 |
| P-58, every territory carries the same nodes: 6 food at 6, 4 metal at 8, 5 energy at 7                                       | `releases/first-release.md`, after Scope (filed against Units and structures; moved on promotion)                                                                                                        | 2026-08-26 |
| P-59, each territory is self-contained; only a mobile unit crosses a boundary                                                | `releases/first-release.md` -> Scope                                                                                                                                                                     | 2026-08-26 |
| P-47, the loop: land the ark founding a territory, then build force, units and spread                                        | `releases/first-release.md` -> The loop (steps 1-4 replaced, later steps renumbered)                                                                                                                     | 2026-08-26 |
| P-48, the structure a founding unit becomes has one less force, operated by citizens                                         | `spec/unit-types.md` -> Founding units                                                                                                                                                                   | 2026-08-26 |
| P-49, the resources are food, metal and energy                                                                               | `spec/resources.md` -> The list                                                                                                                                                                          | 2026-08-26 |
| P-38, citizens do not self-coordinate; a structure or a military unit imposes it                                             | `spec/control.md` -> Coordination                                                                                                                                                                        | 2026-08-26 |
| P-39, violence is inherent, coordination is imposed                                                                          | `spec/narrative.md` -> Violence and order                                                                                                                                                                | 2026-08-26 |
| P-52, every territory has a force of nature of 1                                                                             | `releases/first-release.md` -> Scope                                                                                                                                                                     | 2026-08-26 |
| P-34, a citizen works at one structure and cannot be in two places at once                                                   | `spec/economy.md` -> Structures and labor (filed against Extraction; retargeted)                                                                                                                         | 2026-08-26 |
| P-50, units have force, movement and upkeep; a cost may be anything you control, paid in place                               | `spec/units.md`, and `spec/logistics.md` -> Paying a cost                                                                                                                                                | 2026-08-26 |
| P-51, one generic Extractor; a farm is an extractor on a food node                                                           | `spec/structures.md` -> The list, and `releases/first-release.md` (Farm entry deleted)                                                                                                                   | 2026-08-26 |
| P-65, food is for population, metal for building, energy for moving                                                          | `spec/resources.md` -> The list                                                                                                                                                                          | 2026-08-26 |
| P-66, a mobile unit carries energy cells, filled where it is built                                                           | `spec/units.md` -> What a unit is                                                                                                                                                                        | 2026-08-26 |
| P-27, a Yard produces Arks; the Garrison narrows to land units; the Foundry is cut                                           | `spec/structures.md` -> The list                                                                                                                                                                         | 2026-08-26 |
| P-68, twelve designed territories, each exercising a different consequence                                                   | `releases/first-release.md` -> Territory nodes                                                                                                                                                           | 2026-08-26 |
| P-67, rebalanced costs: Pioneer 16 metal, extractors labor only                                                              | `releases/first-release.md` -> Units and structures; the Yard repriced 64 -> 30 on 2026-08-26, unbuildable as promoted                                                                                   | 2026-08-26 |
| P-74, a game is designed, then started, then played                                                                          | `spec/console.md` -> Phases                                                                                                                                                                              | 2026-08-26 |
| P-70, an Ark costs 24 metal and 24 energy and needs a Yard                                                                   | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-08-26 |
| P-71, orbit is one place; launching and landing each spend a cell                                                            | `spec/orbit.md`                                                                                                                                                                                          | 2026-08-26 |
| P-75, the whole game is one function from state and transitions to state                                                     | `spec/invariants.md` -> The game is one function                                                                                                                                                         | 2026-08-26 |
| P-69, the console command set, its syntax, help, history and error requirements                                              | `spec/console.md`                                                                                                                                                                                        | 2026-08-26 |
| P-72, a change made any way is indistinguishable from the command that would make it                                         | `spec/invariants.md` -> Everything is expressible, **cut again 2026-08-26** as derivable from P-11 and P-75                                                                                              | 2026-08-26 |
| P-73, three surfaces - the game, the console, the data browser - in every build                                              | `spec/interface.md` -> Surfaces                                                                                                                                                                          | 2026-08-26 |
| P-76, four design-phase commands: create planet, add node, set force, add unit                                               | `spec/console.md` -> Commands                                                                                                                                                                            | 2026-08-26 |
| P-77, a planet is fully exploited when nothing more can be taken, built or stored                                            | `spec/control.md` -> Winning                                                                                                                                                                             | 2026-08-26 |
| P-79, the movement allowance is deleted; the spent flag limits how often a unit acts                                         | `spec/units.md` and `releases/first-release.md`                                                                                                                                                          | 2026-08-26 |
| P-78, producing happens in any order; a spent flag limits it, and ending a turn clears it                                    | `spec/turn.md` -> Order of operations (both bullets replaced, the discard bullet absorbed)                                                                                                               | 2026-08-26 |
| P-80, every cost halved so the landing site can expand                                                                       | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-08-27 |
| P-81, the win clause names a storage structure, not a store of resources                                                     | `spec/control.md` -> Winning                                                                                                                                                                             | 2026-08-27 |
| P-82, `run <file>` and `#` comments; `run` is not a transition and is not in history                                         | `spec/console.md`                                                                                                                                                                                        | 2026-08-27 |
| P-83, a citizen has a force of its own; the first release sets it to 1                                                       | `spec/control.md` -> Producing force, and `releases/first-release.md`                                                                                                                                    | 2026-08-27 |
| P-84, a garrison is not built; founding is the only source of one                                                            | `spec/control.md` -> Producing force                                                                                                                                                                     | 2026-08-27 |
| P-85, six release lines reconciled with the spec: transforms, the loop, fuel, the stale note                                 | `releases/first-release.md`                                                                                                                                                                              | 2026-08-27 |
| P-86, a Pioneer must found on leaving friendly territory or perish                                                           | `releases/first-release.md` -> Scope                                                                                                                                                                     | 2026-08-27 |
| P-87, a cost is paid in the territory, not at a building site                                                                | `spec/logistics.md` -> Paying a cost                                                                                                                                                                     | 2026-08-27 |
| P-88, the poles sit at the centres of two pentagons, never on a boundary                                                     | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-27 |
| P-89, availability is fixed in every build; presentation and input follow the platform                                       | `spec/interface.md` -> Availability and presentation                                                                                                                                                     | 2026-08-28 |
| P-90, input bindings move to the release; roll is explicitly not user-controlled                                             | `spec/planet.md` -> Presentation, and `releases/first-release.md` -> Controls                                                                                                                            | 2026-08-28 |
| P-91, Controls names a binding for every capability the spec requires                                                        | `releases/first-release.md` -> Controls                                                                                                                                                                  | 2026-08-28 |
| P-92, actions that are not manipulations of the planet get on-screen controls                                                | `spec/interface.md` -> Availability and presentation                                                                                                                                                     | 2026-08-28 |
| P-93, a line beginning with `/` names a surface, not a command; reaching one is typed where there is no pointer              | `spec/console.md` -> Commands, and `spec/interface.md`                                                                                                                                                   | 2026-08-28 |
| P-94, a slash directs the front end; `/new <size>` abandons the fold and starts another                                      | `spec/console.md`, and `releases/first-release.md` -> Controls                                                                                                                                           | 2026-08-28 |
| P-95, the requirement stops prescribing a mechanism; a slash form is not a transition                                        | `spec/interface.md`, `spec/console.md`, `releases/first-release.md` -> Controls                                                                                                                          | 2026-08-28 |
| P-96, two drawings, practical and realistic, sharing only the camera                                                         | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-28 |
| P-97, the realistic drawing's terrain is continuous and crosses boundaries                                                   | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-28 |
| P-98, nothing in the terrain reveals how the sphere was divided                                                              | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-28 |
| P-99, each territory has a biome                                                                                             | `spec/planet.md` -> What a territory carries                                                                                                                                                             | 2026-08-28 |
| P-100, a territory's biome is what the terrain gives it                                                                      | `spec/planet.md` -> What a territory carries                                                                                                                                                             | 2026-08-28 |
| P-101, four capabilities for the visual work, each with a vetted-when                                                        | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-08-28 |
| P-102, the six biomes; ocean is unclaimable and never adjacent to ocean                                                      | `spec/planet.md` -> What a territory carries                                                                                                                                                             | 2026-08-28 |
| P-103, what each biome gives a territory, and why every force of nature is 1                                                 | `releases/first-release.md` -> Biomes                                                                                                                                                                    | 2026-08-28 |
| P-107, the realistic drawing shows terrain and no borders                                                                    | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-28 |
| P-104, a drawing never betrays how it was made                                                                               | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-28 |
| P-105, a biome has a margin, not a border                                                                                    | `spec/planet.md` -> Presentation                                                                                                                                                                         | 2026-08-28 |
| P-109, oceans never isolate land from land                                                                                   | `spec/planet.md` -> What a territory carries                                                                                                                                                             | 2026-08-28 |
| P-110, `set biome` gives a territory its biome during design                                                                 | `spec/console.md` -> Commands                                                                                                                                                                            | 2026-08-28 |
| P-108, the biome check states plurality, not majority                                                                        | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-08-28 |
| P-106, a fifth capability: terrain resolved as finely as it is shown                                                         | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-08-28 |
| P-111, control without tedium: rules instead of repetition                                                                   | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-08-28 |
| P-112, the middle layer: rules compose, and edits stay proportional                                                          | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-08-28 |
| P-113, nothing plays itself, and every rule can be read                                                                      | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-08-28 |
| P-114, rules outlive a game and can be given away                                                                            | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-08-28 |
| P-117, a player's rules always finish                                                                                        | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-08-28 |
| P-115, a rule is a source of transitions, not a kind of one                                                                  | `spec/invariants.md` -> The game is one function                                                                                                                                                         | 2026-08-28 |
| P-116, the rule editor is a fourth surface, and it is two-dimensional                                                        | `spec/interface.md` -> Surfaces                                                                                                                                                                          | 2026-08-28 |
| P-120, a rule carries the number of turns it may run                                                                         | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-08-29 |
| P-119, every rule has a text form, and the text is the rule                                                                  | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-08-29 |
| P-121, `/save <file>` writes the history to a file                                                                           | `spec/console.md` -> Commands                                                                                                                                                                            | 2026-08-29 |
| P-118, the rule editor is out of the first release, and the surfaces line says so                                            | `releases/first-release.md` -> Scope, Controls                                                                                                                                                           | 2026-08-29 |
| P-122, a capability for playing the loop through by hand                                                                     | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-08-29 |
| P-123, neither the biome rule nor the connectivity rule yields                                                               | `spec/planet.md` -> What a territory carries                                                                                                                                                             | 2026-08-30 |
| P-125, every structure built everywhere it can be built, and what that means                                                 | `spec/control.md` -> Winning                                                                                                                                                                             | 2026-08-30 |
| P-127, `show` says what can be done, not only what is true                                                                   | `spec/console.md` -> Commands                                                                                                                                                                            | 2026-08-30 |
| P-128, a surface is never more capable than the console                                                                      | `spec/invariants.md` -> Everything is expressible                                                                                                                                                        | 2026-08-30 |
| P-129, a territory holds only so much of each kind of thing                                                                  | `spec/logistics.md` -> Capacity                                                                                                                                                                          | 2026-08-31 |
| P-130, the kinds and the transformations are data                                                                            | `spec/invariants.md` -> The game is data                                                                                                                                                                 | 2026-08-31 |
| P-126, metal and energy carry between turns, food does not, and each resource is conserved or not                            | `spec/resources.md` -> The list, `spec/turn.md` -> Order of operations                                                                                                                                   | 2026-08-31 |
| P-131, units and structures as one table                                                                                     | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-08-31 |
| P-132, the first release's transformations as one table                                                                      | `releases/first-release.md` -> Transformations                                                                                                                                                           | 2026-08-31 |
| P-133, which things ready                                                                                                    | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-08-31 |
| P-134, the state is things                                                                                                   | `spec/invariants.md` -> The game is data                                                                                                                                                                 | 2026-08-31 |
| P-135, competing effects are resolved together, and nothing wins by being first                                              | `spec/turn.md` -> Order of operations                                                                                                                                                                    | 2026-08-31 |
| P-136, when effects compete, and when they merely follow                                                                     | `spec/turn.md` -> Order of operations                                                                                                                                                                    | 2026-08-31 |
| P-137, purge founding                                                                                                        | `spec/console.md`, `spec/control.md`, `spec/population.md`, `spec/unit-types.md`                                                                                                                         | 2026-08-31 |
| P-138, order is spent: matter is conserved, arrangement is not                                                               | `spec/resources.md` -> The list, `spec/turn.md` -> Order of operations                                                                                                                                   | 2026-08-31 |
| P-139, a recipe, not a transformation, and a recipe belongs to the player or the world                                       | `spec/invariants.md` -> The game is data, `releases/first-release.md` -> Recipes                                                                                                                         | 2026-08-31 |
| P-140, two recipes the table did not have: upkeep, perish and revert                                                         | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-08-31 |
| P-148, a bin is where everything is, and a capacity is a bin                                                                 | `spec/logistics.md` -> Capacity, replaced whole and renamed Containment                                                                                                                                  | 2026-09-01 |
| P-147, every cycle among recipes must spend readiness                                                                        | `spec/invariants.md` -> The game is data                                                                                                                                                                 | 2026-09-01 |
| P-141, a unit carries fuel, not cells                                                                                        | `spec/units.md`, `releases/first-release.md` -> Units and structures, Recipes                                                                                                                            | 2026-09-01 |
| P-142, a quantity is a number                                                                                                | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-146, what a thing is made of, and a garrison and an extractor cost 1 metal                                                 | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-09-01 |
| P-145, `perish` destroys metal, which the specification says cannot happen                                                   | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-149, a territory has a density and room for extractors; nodes go                                                           | seven spec files and `releases/first-release.md`                                                                                                                                                         | 2026-09-01 |
| P-150, labor need not come from a citizen                                                                                    | `spec/console.md`, and one line moved from `spec/population.md` -> Labor to `spec/economy.md` -> Structures and labor                                                                                    | 2026-09-01 |
| P-143, the release declares its own vocabulary: kinds, families, bins and traits                                             | `releases/first-release.md`, four new sections                                                                                                                                                           | 2026-09-01 |
| P-151, a quantity may read a trait of the place, not only of an ingredient                                                   | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-152, the Traits table declares thirteen and the release uses five more                                                     | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-01 |
| P-153, `commands/` is in no lane's column                                                                                    | `CLAUDE.md` -> Perspectives, the Code row                                                                                                                                                                | 2026-09-01 |
| P-157, a thing contains things, and a territory is one                                                                       | `spec/logistics.md` -> Containment, replaced whole                                                                                                                                                       | 2026-09-01 |
| P-155, readiness is written in the recipe, not assumed by a rule                                                             | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-158, the Scope column is the owner column wearing a location's name                                                        | `releases/first-release.md` -> Recipes, Traits                                                                                                                                                           | 2026-09-01 |
| P-159, `consumed` is derived, not declared                                                                                   | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-156, what a territory has room for                                                                                         | `releases/first-release.md`, a new section                                                                                                                                                               | 2026-09-01 |
| P-160, adjacency is defined under *What a territory carries*, and it is not one                                              | `spec/planet.md`, one line moved to Distance                                                                                                                                                             | 2026-09-01 |
| P-161, `grow`'s new ingredient is the fifth thing that needs echoing                                                         | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-163, an orbit sits beside its territory, and launching is a move                                                           | `spec/orbit.md`, `releases/first-release.md` -> Recipes                                                                                                                                                  | 2026-09-01 |
| P-162, `P-156` reintroduced the word `bin`, which `P-157` had just removed                                                   | `releases/first-release.md` -> What a territory has room for                                                                                                                                             | 2026-09-01 |
| P-164, *Where things are* still describes bins, and is wrong about orbit too                                                 | `releases/first-release.md` -> Where things are, Traits                                                                                                                                                  | 2026-09-01 |
| P-154, control is derived from a citizen being there                                                                         | `releases/first-release.md` -> Traits, Recipes                                                                                                                                                           | 2026-09-01 |
| P-170, a part is one metal arranged, and a thing binds its parts with more                                                   | `spec/resources.md`, `releases/first-release.md` -> Units and structures, Traits                                                                                                                         | 2026-09-01 |
| P-165, tune the Ark so that what goes in is what comes out                                                                   | `releases/first-release.md` -> Units and structures, Recipes                                                                                                                                             | 2026-09-01 |
| P-169, `spoil` takes surplus food, and the order falls out                                                                   | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-167, `build extractor` takes no metal and names no resource                                                                | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-166, an ingredient may be named, and `move` names two territories                                                          | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-171, three build recipes, one per resource, and the blank goes                                                             | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-172, a player can write one rule for many things, not one rule per thing                                                   | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-09-01 |
| P-168, an action that would waste something says so before it is taken                                                       | `spec/interface.md`, a new section                                                                                                                                                                       | 2026-09-01 |
| P-173, `P-158` deleted the Scope column and two things still depend on it                                                    | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-174, a Yard is the only thing a player builds without labor                                                                | `releases/first-release.md` -> Recipes, Units and structures                                                                                                                                             | 2026-09-01 |
| P-176, `eat` is `upkeep`, and `depart` is `perish`                                                                           | `releases/first-release.md` -> Recipes, Traits, Units and structures                                                                                                                                     | 2026-09-01 |
| P-177, `revert` cannot fire, and the release should not carry it                                                             | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-175, `arriving` earns nothing and should go                                                                                | `releases/first-release.md` -> Recipes, Traits                                                                                                                                                           | 2026-09-01 |
| P-178, `surplus` is derived from a recipe that no longer exists                                                              | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-01 |
| P-179, a Pioneer may cross its own empire, and one line says it may not                                                      | `releases/first-release.md` -> Scope                                                                                                                                                                     | 2026-09-01 |
| P-180, the `upkeep` trait says *a unit*, and a citizen has one                                                               | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-01 |
| P-181, `perish` reads a citizen's metal, and a citizen's metal is blank                                                      | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-01 |
| P-182, a lane owns the tools for its own work; the code lane owns the game                                                   | `CLAUDE.md` -> Perspectives                                                                                                                                                                              | 2026-09-02 |
| P-186, an Ark and a Pioneer deploy the same things, and both bind with three metal                                           | `releases/first-release.md` -> Recipes, Units and structures                                                                                                                                             | 2026-09-02 |
| P-187, `spend readiness` is named `create labor`                                                                             | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-02 |
| P-189, food keeps for one turn, by a number that decrements                                                                  | `releases/first-release.md` -> Traits, Recipes                                                                                                                                                           | 2026-09-02 |
| P-188, capacity is total, used and available, and only the total is stored                                                   | `spec/logistics.md` -> Containment                                                                                                                                                                       | 2026-09-02 |
| P-185, the turn's ending says everything with upkeep pays it                                                                 | `spec/turn.md` -> Order of operations                                                                                                                                                                    | 2026-09-02 |
| P-191, `room` is renamed total capacity everywhere it means the stored maximum                                               | `spec/planet.md`, `spec/economy.md`, `spec/orbit.md`, `spec/console.md`, `spec/control.md`, `spec/logistics.md`, `releases/first-release.md`                                                             | 2026-09-02 |
| P-184, the world's recipes fire at the end of a turn, in the order `spec/turn.md` gives                                      | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-02 |
| P-190, the recipe table takes seven columns, and role carries require, limit, consume and produce                            | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-02 |
| P-192, a territory and an orbit are kinds; there are twelve                                                                  | `releases/first-release.md` -> Kinds, Traits                                                                                                                                                             | 2026-09-02 |
| P-193, the tables are the data the game loads, and every other form is generated                                             | `spec/invariants.md` -> The game is data                                                                                                                                                                 | 2026-09-02 |
| P-194, a proposal's block is text, rows or an instruction, and says which                                                    | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-02 |
| P-195, the four lines that assumed every block was text, and a `shape` field                                                 | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-02 |
| P-196, a place is a territory or an orbit, edges have kinds, and a unit says which it crosses                                | `releases/first-release.md` -> Families, Traits, Units and structures, Recipes                                                                                                                           | 2026-09-03 |
| P-197, the three things a promotion can do, in words that define themselves                                                  | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-03 |
| P-198, specification and presentation do not share a format; a rendering is never canonical                                  | `CLAUDE.md` -> Perspectives                                                                                                                                                                              | 2026-09-03 |
| P-199, what the game is made of lives in a data file, not in code and not in markup                                          | `spec/invariants.md` -> The game is data                                                                                                                                                                 | 2026-09-03 |
| P-200, the data browser has two views, and both name every column                                                            | `spec/interface.md` -> Surfaces                                                                                                                                                                          | 2026-09-03 |
| P-201, Auto is Owner again, the duplicate metal column goes, and Capacity is Container                                       | `releases/first-release.md` -> Recipes, Units and structures, Where things are                                                                                                                           | 2026-09-03 |
| P-202, a control among alternatives shows which is chosen, and the drawing's binding is written down                         | `spec/interface.md` -> What an action shows, `releases/first-release.md` -> Controls                                                                                                                     | 2026-09-03 |
| P-203, a generated file is written in the form the padder would leave it                                                     | `CLAUDE.md` -> Perspectives                                                                                                                                                                              | 2026-09-03 |
| P-204, three rules for using AI assistants, the third being a place to look                                                  | `docs/process.md`, a new section                                                                                                                                                                         | 2026-09-03 |
| P-206, an extractor is three kinds and a family, and the capacity exception goes                                             | `releases/first-release.md` -> Kinds, Families, Traits, Recipes                                                                                                                                          | 2026-09-03 |
| P-207, every kind is bounded and the table says by what                                                                      | `releases/first-release.md` -> What bounds a kind in a territory                                                                                                                                         | 2026-09-03 |
| P-208, *Units and structures* lists the three extractor kinds rather than the family                                         | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-09-03 |
| P-209, the `kind` trait says one of the kinds rather than restating their number                                             | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-03 |
| P-210, the `biome` trait says one of the biomes rather than restating their number                                           | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-03 |
| P-211, how I know the game is right: four artifacts and the closure between them                                             | `docs/process.md`, a new section                                                                                                                                                                         | 2026-09-03 |
| P-212, a command is a node and a value may be another one                                                                    | `spec/console.md` -> Commands                                                                                                                                                                            | 2026-09-03 |
| P-213, a definition arrives whole                                                                                            | `spec/invariants.md` -> The game is data                                                                                                                                                                 | 2026-09-03 |
| P-214, a command names a recipe, so the command list is the recipe list                                                      | `spec/console.md` -> Commands                                                                                                                                                                            | 2026-09-03 |
| P-215, a rejection names line, column and the enclosing command                                                              | `spec/console.md` -> Errors                                                                                                                                                                              | 2026-09-03 |
| P-216, normalize what you compare, nest what you do not                                                                      | `spec/interface.md` -> Surfaces                                                                                                                                                                          | 2026-09-03 |
| P-217, the commands that are not recipes, and why they are listed                                                            | `spec/console.md` -> Commands                                                                                                                                                                            | 2026-09-03 |
| P-218, the data that runs the game lives in a data file                                                                      | `spec/invariants.md` -> The game is data                                                                                                                                                                 | 2026-09-04 |
| P-219, what the scenario test locks, and what locking means                                                                  | `docs/process.md` -> How I know the game is right                                                                                                                                                        | 2026-09-04 |
| P-221, a rule is decided in the spec and the game's data in its data file                                                    | `spec/README.md` -> Rules for this directory, under rule 3                                                                                                                                               | 2026-09-04 |
| P-222, `markup` and `restates` replaced with what `P-218` says precisely                                                     | `spec/invariants.md` -> The game is data                                                                                                                                                                 | 2026-09-04 |
| P-223, two kinds of stated fact, and markdown's tier named                                                                   | `CLAUDE.md` -> Perspectives                                                                                                                                                                              | 2026-09-04 |
| P-220, rule 7 sends the game's data to a data file rather than a release                                                     | `spec/README.md` -> Rules for this directory, rule 7                                                                                                                                                     | 2026-09-04 |
| P-224, a release does not contain the game's data and links to the generated view                                            | `releases/README.md` -> The rule that keeps them from contradicting each other                                                                                                                           | 2026-09-04 |
| P-225, deleting the expected data is how I change my mind, and absence means acceptance                                      | `docs/process.md` -> How I know the game is right                                                                                                                                                        | 2026-09-04 |
| P-228, a test is for what a person cannot repeat, not for what happens once                                                  | `docs/process.md` -> How I know the game is right                                                                                                                                                        | 2026-09-04 |
| P-229, a proposal says whether it wants approval or a decision, and the quotation is the offer                               | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-04 |
| P-230, an instruction may carry no quotation, and carries its check instead                                                  | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-04 |
| P-231, `labor` is a kind - Sean chose A, 2026-09-04                                                                          | no text landed: the release already said it and the model is what changes. `S-21` builds it, `P-232` carries the question it left                                                                        | 2026-09-04 |
| P-232, `create labor` keeps a command - Sean chose 2, 2026-09-04                                                             | no text landed: `P-214` holds as written and the release is unchanged. `S-36` adds the 18 commands                                                                                                       | 2026-09-04 |
| P-238, why my surface is small, and that a proposal is a thing addressed to me                                               | `docs/process.md` -> What I read, and what I do                                                                                                                                                          | 2026-09-04 |
| P-239, `pending.md` is for the instances; my two acts are creating and validating                                            | `docs/process.md` -> Specification Instance, Outboxes and the index                                                                                                                                      | 2026-09-04 |
| P-240, production support is the code lane's                                                                                 | `docs/process.md` -> Who writes what, and `CLAUDE.md` -> Perspectives                                                                                                                                    | 2026-09-04 |
| P-233, `ready` is a boolean trait and the world recipe is `refresh`                                                          | `releases/first-release.md` -> Traits, Recipes                                                                                                                                                           | 2026-09-04 |
| P-235, *Directions* - what the design expects to move, binding nothing                                                       | `docs/vision.md` -> Directions, and `docs/README.md` -> What goes where                                                                                                                                  | 2026-09-04 |
| P-236, the `asks` field is declared                                                                                          | `CLAUDE.md` -> Outboxes                                                                                                                                                                                  | 2026-09-04 |
| P-234, one `extractor` kind, and a `$` may name a trait value                                                                | `spec/console.md` -> Commands, and `releases/first-release.md` -> six places                                                                                                                             | 2026-09-05 |
| P-241, a match string breaks on a wrapped sentence as it does on a padded row                                                | `CLAUDE.md` -> A mistake worth not repeating                                                                                                                                                             | 2026-09-05 |
| P-242, blocked is observable and running is not, so a report says which                                                      | `docs/process.md` -> Outboxes and the index                                                                                                                                                              | 2026-09-05 |
| P-243, three sentences describing arrangements that had been replaced                                                        | `docs/process.md` -> Who writes what, `CLAUDE.md` -> Outboxes, Starting a new lens                                                                                                                       | 2026-09-05 |
| P-244, quality finds and reports production support; the coding instance builds it                                           | `docs/process.md` -> Quality instance                                                                                                                                                                    | 2026-09-05 |
| P-245, a document that restates another links to it rather than listing it                                                   | `docs/README.md` -> What goes where                                                                                                                                                                      | 2026-09-05 |
| P-246, reports in `reports/` with an index, scenario in `scenario/`, and each turn carrying its commands and delta           | `docs/README.md` -> What goes where; `S-38` builds it                                                                                                                                                    | 2026-09-05 |
| P-247, reporting is not a place to stop, and a reply ends with the next thing                                                | `docs/process.md` -> All lanes                                                                                                                                                                           | 2026-09-05 |
| P-248, a lane that is waiting files the hold, and the lane that finishes tells it                                            | `docs/process.md` -> All lanes                                                                                                                                                                           | 2026-09-05 |
| P-250, a derived number names its rule, and closing an item lists who else named it                                          | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-05 |
| P-249, a main scenario I vet, and end-state scenarios that rest on it                                                        | `docs/process.md` -> How I know the game is right, and `releases/first-release.md` -> `R-6`                                                                                                              | 2026-09-05 |
| P-251, a proposal landing in more than one file carries one quotation for each                                               | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-05 |
| P-252, no quotes in data; multi-word names take dashes; `force of nature` is `nature`                                        | `spec/console.md` -> The language, and `releases/first-release.md` -> Traits                                                                                                                             | 2026-09-05 |
| P-253, jungle holds itself with a force of 2                                                                                 | `releases/first-release.md` -> Biomes, and Scope                                                                                                                                                         | 2026-09-05 |
| P-254, a thing's own identifier is `id`                                                                                      | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-05 |
| P-255, `founded` is dropped from the data - Sean chose B, 2026-09-05                                                         | no text landed: `control` stays declared and unprinted, and he will notice if he wants it. `S-43` does it                                                                                                | 2026-09-05 |
| P-257, containment is a tree: a root, a one-way leaf rule, the kind not the instance, and containing is not referring        | `spec/logistics.md` -> Containment                                                                                                                                                                       | 2026-09-05 |
| P-259, disorder is lost at the turn's end - Sean chose B, 2026-09-05                                                         | no text landed: `spec/turn.md` already says anything above the bound is lost when the turn ends, and `P-258` is what changes the bound                                                                   | 2026-09-05 |
| P-258, a territory declares no capacity for a resource                                                                       | `releases/first-release.md` -> What bounds a kind                                                                                                                                                        | 2026-09-05 |
| P-256, the report shows a territory's derived capacity for a resource - Sean said yes, 2026-09-05                            | no text landed: it is a report decision and no rule moves. `S-44` does it                                                                                                                                | 2026-09-05 |
| P-260, `store` is one kind with a `resource` trait, holding 10, as many as the extractors of its resource, founding included | `releases/first-release.md` -> Kinds, Traits, What bounds a kind, Units and structures, Recipes                                                                                                          | 2026-09-05 |
| P-261, the three resources are supposed to feel different                                                                    | `docs/vision.md` -> Constraints                                                                                                                                                                          | 2026-09-05 |
| P-262, templating gives variety without complexity                                                                           | `docs/vision.md` -> Directions                                                                                                                                                                           | 2026-09-05 |
| P-263, a promotion says what it is for the code lane, and silence is not an answer                                           | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-05 |
| P-266, a proposal that asks a decision cannot be promoted; answering it makes it one that asks approval                      | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-05 |
| P-264, the main scenario moves to the specification, and the process document keeps the rule                                 | `docs/process.md` -> How I know the game is right, `spec/scenarios.md`, `spec/README.md`                                                                                                                 | 2026-09-05 |
| P-270, an unstored resource can be used the turn it is made and is lost when that turn ends                                  | `releases/first-release.md` -> What bounds a kind                                                                                                                                                        | 2026-09-05 |
| P-265, a store holds 10 and the extractor's catch row goes, which P-260 asked for and did not carry                          | `releases/first-release.md` -> Where things are                                                                                                                                                          | 2026-09-05 |
| P-268, the prompts that start the specification, code and quality instances                                                  | `docs/process.md` -> Starting the instances                                                                                                                                                              | 2026-09-05 |
| P-267, what a proposal asks him, what answering one does, and that what he approves is what ships                            | `docs/process.md` -> What I read and what I do                                                                                                                                                           | 2026-09-05 |
| P-269, the four artifacts said without naming this game's model                                                              | `docs/process.md` -> How I know the game is right                                                                                                                                                        | 2026-09-05 |
| P-271, the directory list goes and every rule stays, with `CLAUDE.md` linked                                                 | `docs/process.md` -> Who writes what                                                                                                                                                                     | 2026-09-05 |
| P-273, the coding instance's prompt points at `pending.md` instead of naming three files                                     | `docs/process.md` -> Starting the instances                                                                                                                                                              | 2026-09-05 |
| P-275, a military unit is organised force in itself, and taking uses the organised force brought                             | `spec/control.md` -> Gaining and holding ground                                                                                                                                                          | 2026-09-05 |
| P-272, a biome gives a territory its total capacity and density for each resource                                            | `spec/planet.md`                                                                                                                                                                                         | 2026-09-05 |
| P-274, the rebalanced biome table: jungle food, ice metal and grassland metal                                                | `releases/first-release.md` -> Biomes                                                                                                                                                                    | 2026-09-05 |
| P-276, a garrison has no force and does one thing, by existing                                                               | `spec/control.md` -> Force                                                                                                                                                                               | 2026-09-05 |
| P-277, the garrison row's force becomes 0 and the multiplier sentence goes                                                   | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-09-05 |
| P-278, more dangerous territory requires more organised citizens to keep it secure                                           | `spec/narrative.md` -> Violence and order                                                                                                                                                                | 2026-09-05 |
| P-280, a biome does not determine a territory's numbers; the two agree thematically                                          | `spec/planet.md`                                                                                                                                                                                         | 2026-09-05 |
| P-281, the biome table guides and does not bind, and force of nature is the column that does                                 | `releases/first-release.md` -> Biomes                                                                                                                                                                    | 2026-09-05 |
| P-283, bullet-versus-paragraph takes the closing period with it, and no other punctuation moves                              | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-06 |
| P-285, a thing is not located by a trait, and one with an `id` is unique                                                     | `spec/logistics.md` -> Containment                                                                                                                                                                       | 2026-09-06 |
| P-286, the release declares `id` and drops the `place` trait                                                                 | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-06 |
| P-287, contents are a map from a description to a quantity                                                                   | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-06 |
| P-284, every word in a data file is a kind, a trait, or a trait value                                                        | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-06 |
| P-290, capacity may be per kind carrying a particular trait value, which makes `node` unnecessary                            | `spec/logistics.md` -> Containment                                                                                                                                                                       | 2026-09-06 |
| P-288, `phase` is a stored trait of the game; `turn` is not in the state at all                                              | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-06 |
| P-289, what makes a check worth having: it can be unable to fail, and loosening one that cries wolf is how it gets there     | `docs/process.md`                                                                                                                                                                                        | 2026-09-06 |
| P-291, a check can be sound and no longer believed; re-poison one when its exception list grows                              | `docs/process.md`                                                                                                                                                                                        | 2026-09-06 |
| P-292, an instruction is not a shortcut; the specification instance coordinates and reports status                           | `docs/process.md` -> Specification Instance                                                                                                                                                              | 2026-09-06 |
| P-293, what the coding instance is for, including the artifact I review by hand                                              | `docs/process.md` -> Coding instance                                                                                                                                                                     | 2026-09-06 |
| P-294, what the quality instance is for, and the example that refuses the usual argument                                     | `docs/process.md` -> Quality instance                                                                                                                                                                    | 2026-09-06 |
| P-295, what the research instance is for                                                                                     | `docs/process.md` -> Research instances                                                                                                                                                                  | 2026-09-06 |
| P-296, four peer definitions, and the scaffolding that follows                                                               | `docs/process.md` -> the purpose list, Quality instance, Starting the instances                                                                                                                          | 2026-09-06 |
| P-297, three artifacts are addressed to me, and concision is bounded by control                                              | `docs/process.md` -> What I read, and what I do                                                                                                                                                          | 2026-09-06 |
| P-298, the third purpose of the bookkeeping is the context an assistant needs                                                | `docs/process.md` -> Claude bookkeeping                                                                                                                                                                  | 2026-09-06 |
| P-299, it can tell me whether a lane is running, and what neither answer covers                                              | `docs/process.md` -> Outboxes and the index                                                                                                                                                              | 2026-09-06 |
| P-300, two budgets, and only one of them is a count                                                                          | `docs/process.md` -> All lanes, Research instances                                                                                                                                                       | 2026-09-06 |
| P-301, two files are addressed to me, and the limit of fifteen is a tripwire                                                 | `docs/process.md` -> What I read, and what I do                                                                                                                                                          | 2026-09-06 |
| P-302, this document has to be enough on its own, with every transcript and CLAUDE.md gone                                   | `docs/process.md` -> What this document has to be                                                                                                                                                        | 2026-09-06 |
| P-303, a reason that is false is worse than one that is missing                                                              | `docs/process.md` -> What this document has to be                                                                                                                                                        | 2026-09-06 |
| P-304, where to aim a probe, and when two counts are one count                                                               | `docs/process.md` -> What makes a check worth having                                                                                                                                                     | 2026-09-06 |
| P-306, the sentence saying inter-lane items have no limit, corrected                                                         | `docs/process.md` -> What I read, and what I do                                                                                                                                                          | 2026-09-06 |
| P-305, when an item closes, and the withdrawal that would otherwise swallow one                                              | `docs/process.md` -> Outboxes and the index                                                                                                                                                              | 2026-09-06 |
| P-307, the process document stops calling its subject a game                                                                 | `docs/process.md`, six places                                                                                                                                                                            | 2026-09-06 |
| P-308, the phase cell names its values, and closed sets say where they are listed                                            | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-06 |
| P-309, a command of one phase is refused in the other                                                                        | `spec/console.md` -> Phases                                                                                                                                                                              | 2026-09-06 |
| P-310, grow loses the houses requirement and gains the cap the specification states                                          | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-06 |
| P-312, `houses` is deleted: no recipe uses it and no kind carries it                                                         | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-06 |
| P-313, the file is `decisions.md`                                                                                            | `docs/process.md` -> What I read, and what I do                                                                                                                                                          | 2026-09-06 |
| P-311, the dump carries the containment tree, completely and exactly once                                                    | `releases/first-release.md` -> Where things are                                                                                                                                                          | 2026-09-06 |
| P-314, `adjacency` is a fact about the container, not about a place                                                          | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-06 |
| P-316, a promotion files what it creates, before the proposal is deleted                                                     | `docs/process.md` -> Who writes what                                                                                                                                                                     | 2026-09-06 |
| P-317, the transformation reads the state and the commands and nothing else                                                  | `docs/process.md` -> How I know the application is right                                                                                                                                                 | 2026-09-06 |
| P-319, say what you are referring to rather than where it was, and the three places it fired                                 | `docs/process.md` -> What I read and what I do, How I know the application is right, All lanes                                                                                                           | 2026-09-06 |
| P-320, the dump is a data file and `spec/console.md` governs it; `P-311`'s container columns go                              | `releases/first-release.md` -> Where things are                                                                                                                                                          | 2026-09-06 |
| P-321, a command is written `{name field:value ...}`, and the positional form goes                                           | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-06 |
| P-349, edge, border and boundary name one thing, and crossing is passing through it                                          | `spec/planet.md` -> Distance                                                                                                                                                                             | 2026-09-07 |
| P-353, there is one notation, and the tree is what having one means                                                          | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-07 |
| P-351, `game` is the sixteenth kind                                                                                          | `releases/first-release.md` -> Kinds                                                                                                                                                                     | 2026-09-07 |
| P-352, the staging bullet gains the reason care cannot close it, and the check that can                                      | `CLAUDE.md` -> Perspectives                                                                                                                                                                              | 2026-09-07 |
| P-346, the `A move` column goes                                                                                              | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-09-08 |
| P-355, `movable` is a stored trait, and a `Movable` column declares it                                                       | `releases/first-release.md` -> Traits, Units and structures                                                                                                                                              | 2026-09-08 |
| P-358, `P-351` made the release contradict itself, and one word fixes it                                                     | `releases/first-release.md` -> Where things are                                                                                                                                                          | 2026-09-08 |
| P-359, `temporary-notes/` is read when he points at it, and never decides anything                                           | `CLAUDE.md` -> Perspectives                                                                                                                                                                              | 2026-09-08 |
| P-322, a deposit is a thing, so a territory's density has somewhere to be written                                            | `releases/first-release.md` -> Kinds, Traits                                                                                                                                                             | 2026-09-06 |
| P-323, a field is named for the kind it refers to; a command is named for its recipe and may repeat                          | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-06 |
| P-324, being blocked on one thing is not being blocked, and it applies to every lane                                         | `docs/process.md` -> All lanes                                                                                                                                                                           | 2026-09-06 |
| P-325, an item that waits says so in a field, so something can notice when the wait is over                                  | `docs/process.md` -> Outboxes and the index                                                                                                                                                              | 2026-09-06 |
| P-326, state the kind of failure and not the occasion, and the three places that did not                                     | `docs/process.md` -> What this document has to be, Outboxes and the index, All lanes                                                                                                                     | 2026-09-06 |
| P-327, a rule that fires at a moment of confidence needs a carrier, not a better sentence                                    | `docs/process.md` -> What makes a check worth having                                                                                                                                                     | 2026-09-06 |
| P-328, a command's name is one word, dashed where it needs more                                                              | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-06 |
| P-329, maintaining my executive control is what decides what the specification instance owns                                 | `docs/process.md` -> Specification Instance                                                                                                                                                              | 2026-09-06 |
| P-330, `R-7`: a worked example beside every recipe, generated by running it                                                  | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-09-07 |
| P-331, a deposit carries the capacity as well as the density, so the round trip closes                                       | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-07 |
| P-332, the world's six recipes are shown together on `{end-turn}`                                                            | `releases/first-release.md` -> Capabilities, `R-7`                                                                                                                                                       | 2026-09-07 |
| P-333, `R-8`: which kinds behave alike, derived from the tables rather than declared                                         | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-09-07 |
| P-334, an adjacency is a thing, held by the game rather than by a territory                                                  | `releases/first-release.md` -> Kinds, Traits                                                                                                                                                             | 2026-09-07 |
| P-335, `R-9`: the reports are browsable without a script running                                                             | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-09-07 |
| P-338, a thing lasts a number of turns, its upkeep resets that, and no number is durable                                     | `spec/resources.md` -> The list                                                                                                                                                                          |            |
| P-339, an ark and a pioneer print life rather than eating, so neither has upkeep                                             | `releases/first-release.md` -> the opening list, bounds, Units                                                                                                                                           |            |
| P-340, a unit is taken apart when it deploys, not when it arrives                                                            | `spec/unit-types.md` -> Units that become structures                                                                                                                                                     |            |
| P-341, building an Ark and launching it are one act                                                                          | `spec/structures.md` -> Yard                                                                                                                                                                             |            |
| P-342, `produce ark` becomes `launch ark`, and stops producing anything                                                      | `releases/first-release.md` -> Recipes                                                                                                                                                                   |            |
| P-343, the `Expires` column becomes `Lasts`, three kinds widen, and `age` fires before `spoil`                               | `spec/resources.md` -> The list; `releases/first-release.md` -> Traits, Recipes                                                                                                                          |            |
| P-345, `spec/orbit.md` said launching is a move and `P-341` said it is a Yard's act                                          | `spec/orbit.md` -> Crossing between layers. Sean: the line is removed - determining costs is the domain of individual recipes, not invariants. Partly reverses `P-163`; the adjacency half of it stands. |            |
| P-344, one act leaves the Ark no `ascent`, and the playthrough's last two steps are one                                      | `releases/first-release.md` -> the playthrough list, *Units and structures*. Partly reverses `P-71`, whose third edge kind now has no user; `border` and `orbit border` stand.                           |            |
| P-356, a field's value may name a kind rather than a thing                                                                   | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-10 |
| P-360, a garrison has no force of its own, and the founding sentence stops saying it has                                     | `spec/unit-types.md` -> Every unit                                                                                                                                                                       | 2026-09-10 |
| P-361, fully exploited is the greatest output a territory can reach, not every place a structure would fit                   | `spec/control.md` -> Winning                                                                                                                                                                             | 2026-09-10 |
| P-362, `launch ark` puts an Ark in the orbit above                                                                           | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-10 |
| P-363, a rule may ask the engine for a set or a number, never for an effect                                                  | `spec/invariants.md` -> The game is one function                                                                                                                                                         | 2026-09-10 |
| P-364, designing a world is made of the same rules as playing one                                                            | `spec/console.md` -> Commands                                                                                                                                                                            | 2026-09-10 |
| P-365, what an orbit boundary is, and that a unit moving in orbit takes its energy from the sun                              | `spec/orbit.md` -> Crossing between layers, `spec/units.md` -> What a unit is                                                                                                                            | 2026-09-10 |
| P-366, a selector is not a description, and what a value may be                                                              | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-10 |
| P-367, nature takes back a lost territory, and every unit on it is destroyed                                                 | `spec/control.md` -> Holding                                                                                                                                                                             | 2026-09-10 |
| P-368, nothing comes back round with more, and what a rule may cost                                                          | `spec/invariants.md`, two new sections                                                                                                                                                                   | 2026-09-10 |
| P-369, matter cycles: a source, then disorder, then its source again                                                         | `spec/resources.md`                                                                                                                                                                                      | 2026-09-10 |
| P-370, the fiction is richer than the rules, and that is not a gap                                                           | `spec/narrative.md` -> What the fiction owes the rules                                                                                                                                                   | 2026-09-10 |
| P-371, an adjacency names two places and holds neither, replacing the deleted node                                           | `spec/logistics.md` -> Containment                                                                                                                                                                       | 2026-09-10 |
| P-372, the release learns disorder, and a territory declares no limit rather than no capacity                                | `releases/first-release.md` -> What bounds a kind in a territory                                                                                                                                         | 2026-09-10 |
| P-373, what soft means, and that a soft line names something with a finite capacity                                          | `spec/invariants.md` -> What a rule may cost                                                                                                                                                             | 2026-09-10 |
| P-374, store the room, derive the total, and destroying a thing gives its room back                                          | `spec/logistics.md` -> Containment                                                                                                                                                                       | 2026-09-10 |
| P-375, the sweep, and upkeep and perish grounded to the one kind they are about                                              | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-10 |
| P-376, an amount read from a trait is not an amount that depends on what is present                                          | `spec/invariants.md` -> What a rule may cost                                                                                                                                                             | 2026-09-10 |
| P-377, growth by a resource, which removes the last minimum                                                                  | `releases/first-release.md` -> Recipes, Kinds, Traits                                                                                                                                                    | 2026-09-10 |
| P-378, a line may carry an attachment in brackets after its amount                                                           | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-10 |
| P-379, the world's firing order, with the five that arrived and without the one that went                                    | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-10 |
| P-381, a raw material has three states, a transient is always in disorder, and what is constructed is in neither             | `releases/first-release.md` -> What bounds a kind in a territory                                                                                                                                         | 2026-09-11 |
| P-380, `discard` sweeps the two transients, and `fertility` gains the bound it never had                                     | `releases/first-release.md` -> Recipes, What bounds a kind in a territory                                                                                                                                | 2026-09-11 |
| P-382, the food line that a later firing order made false is deleted                                                         | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-11 |
| P-386, what a rule takes is hard and what it makes is soft                                                                   | `spec/invariants.md` -> What a rule may cost                                                                                                                                                             | 2026-09-11 |
| P-385, the two `limit 0 garrison` rows go, and repeated deployment is the player's                                           | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-11 |
| P-383, a probe uses a file the lane owns, and a commit hash is not evidence                                                  | `docs/process.md` -> Who writes what, All lanes                                                                                                                                                          | 2026-09-11 |
| P-384, a change that leaves every test green has measured the tests                                                          | `docs/process.md` -> What makes a check worth having                                                                                                                                                     | 2026-09-11 |
| P-388, three sources - the planet, the star, and time, and a thing that exhausts extracts readiness                          | `spec/invariants.md` -> Nothing comes back round with more                                                                                                                                               | 2026-09-11 |
| P-390, what a thing can do is a token it holds, and an action is named                                                       | `spec/turn.md` -> Order of operations                                                                                                                                                                    | 2026-09-11 |
| P-391, three things a kind may declare about what it holds, and capacity is not unconserved                                  | `spec/logistics.md` -> Containment                                                                                                                                                                       | 2026-09-11 |
| P-392, down to single commands, and nothing a player can use and cannot take apart                                           | `spec/invariants.md` -> Control without tedium                                                                                                                                                           | 2026-09-11 |
| P-393, disorder is being held by something that declares no limit, and the territory does hold labor                         | `releases/first-release.md` -> What bounds a kind in a territory                                                                                                                                         | 2026-09-11 |
| P-394, the fiction and the rule agree about what a source holds                                                              | `spec/narrative.md` -> Violence and order                                                                                                                                                                | 2026-09-11 |
| P-395, `R-10`: a generated drawing is legible in the reader's theme                                                          | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-09-11 |
| P-397, the release's `In` line quotes the sentence `P-390` replaced                                                          | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-11 |
| P-396, a rule may name one thing and say what changes about it                                                               | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-11 |
| P-398, `R-5` quoted words `spec/planet.md` does not contain                                                                  | `releases/first-release.md` -> Capabilities                                                                                                                                                              | 2026-09-11 |
| P-399, the release follows: readiness is a kind, `put` moves a unit, `renew` goes                                            | `releases/first-release.md` -> Kinds, Traits, Where things are, Recipes                                                                                                                                  | 2026-09-11 |
| P-401, I deploy in order to verify, and looking is not approving                                                             | `docs/process.md`, a new section                                                                                                                                                                         | 2026-09-11 |
| P-404, a quotation for each destination, not for each file                                                                   | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-11 |
| P-408, what a thing can do is a count it carries as a trait                                                                  | `spec/turn.md` -> Order of operations                                                                                                                                                                    | 2026-09-11 |
| P-410, a word I have to look up is a reference I cannot resolve                                                              | `docs/process.md` -> What I read, and what I do                                                                                                                                                          | 2026-09-11 |
| P-411, the release follows: a count per action, carried as a trait                                                           | `releases/first-release.md` -> Kinds, Traits, Where things are, Recipes                                                                                                                                  | 2026-09-11 |
| P-406, each quotation is one block offered, and there is no count                                                            | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-11 |
| P-416, force is mustered rather than presented, and there is no highest case                                                 | `spec/control.md` -> Producing force, Coordination                                                                                                                                                       | 2026-09-11 |
| P-414, a garrison lets each citizen muster its force, mustered afresh each turn                                              | `releases/first-release.md` -> Traits, Recipes                                                                                                                                                           | 2026-09-11 |
| P-417, a description carries the traits of the thing, not those of its kind                                                  | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-11 |
| P-407, five traits are of the kind, and `kind` is not a trait at all                                                         | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-11 |
| P-415, a quotation for each block offered, in both places the count was stated                                               | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-11 |
| P-412, the release's `In` line follows `P-408`                                                                               | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-11 |
| P-413, `readiness` named nothing, and says what it is instead                                                                | `spec/invariants.md` -> two sections                                                                                                                                                                     | 2026-09-11 |
| P-418, a selector leaves traits out, and the example names a trait the game has                                              | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-11 |
| P-419, the bullet's last sentence says what a thing can do, not `readiness`                                                  | `spec/invariants.md` -> Nothing comes back round with more                                                                                                                                               | 2026-09-11 |
| P-421, `put` is declared: the same thing and not a new one, and a put has no quantity                                        | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-11 |
| P-422, `R-6` is two territories and a launch, and every recipe fires                                                         | `releases/first-release.md` -> `R-6`                                                                                                                                                                     | 2026-09-11 |
| P-425, force under unification: `Coordination` folds into `Producing force`, and `military` goes                             | `spec/control.md` -> Producing force, Gaining and holding ground                                                                                                                                         | 2026-09-11 |
| P-426, metal is drawn from the planet and conserved once above ground                                                        | `releases/first-release.md` -> Kinds                                                                                                                                                                     | 2026-09-11 |
| P-427, the four `store` rows go, and founding stops being a metal source                                                     | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-11 |
| P-428, unification first, and a tie goes to the unified form                                                                 | `docs/process.md` -> Three rules for using AI assistants                                                                                                                                                 | 2026-09-11 |
| P-429, a check that reads a copy of the population is checking the copy                                                      | `docs/process.md` -> What makes a check worth having                                                                                                                                                     | 2026-09-11 |
| P-430, a rule may ask whether something is absent only where a limit is declared                                             | `spec/invariants.md` -> What a rule may cost                                                                                                                                                             | 2026-09-11 |
| P-431, `age` becomes `require` and `put`; `stow` stays `consume` and `produce`                                               | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-11 |
| P-433, the game declares no limit, for every kind                                                                            | `spec/logistics.md` -> Containment                                                                                                                                                                       | 2026-09-12 |
| P-435, the trait becomes `strength`, the kind stays `force`, and `force` is declared                                         | `releases/first-release.md` -> Kinds, Traits, Units and structures, Recipes, and `spec/control.md`, `spec/units.md`                                                                                      | 2026-09-12 |
| P-436, `spec/structures.md` says a garrison holds force of its own and `spec/control.md` says it has none                    | `spec/structures.md` -> Garrison                                                                                                                                                                         | 2026-09-12 |
| P-437, the number 1 leaves `spec/control.md`, and a citizen musters its strength                                             | `spec/control.md` -> Producing force                                                                                                                                                                     | 2026-09-12 |
| P-438, `spec/economy.md` says mustering competes with labor, and `spec/control.md` says it does not                          | `spec/economy.md` -> Structures and labor                                                                                                                                                                | 2026-09-12 |
| P-434, `keeps` is declared *of the kind* and `age` lowers it for one thing                                                   | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-12 |
| P-440, the specification holds the default, and tuning happens in the editor                                                 | `spec/README.md` -> Rules for this directory                                                                                                                                                             | 2026-09-12 |
| P-441, an Ark's `Fuel` cell should be blank, and it is about to be transcribed                                               | `releases/first-release.md` -> Units and structures                                                                                                                                                      | 2026-09-12 |
| P-443, the vocabulary declares itself, so the rule needs no exception                                                        | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-12 |
| P-446, forced unification is a lie that hides complexity                                                                     | `docs/process.md` -> Three rules for using AI assistants                                                                                                                                                 | 2026-09-12 |
| P-448, a kind declares its family, and `thing` is the one every kind is in                                                   | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-12 |
| P-449, if I never notice I need it, I don't need it                                                                          | `docs/process.md` -> Three rules for using AI assistants                                                                                                                                                 | 2026-09-12 |
| P-451, a kind declares its traits, and a value declares which trait it is one of                                             | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-12 |
| P-452, the traits inside a description are sorted, and only the entries are said to be                                       | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-12 |
| P-454, a kind writes an of-the-kind trait's value and names the rest                                                         | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-12 |
| P-458, what a person types, as an invariant                                                                                  | `spec/invariants.md` -> two new sections after *Control without tedium*                                                                                                                                  | 2026-09-12 |
| P-463, `id` admits an identity, and a unification that lies is not one                                                       | `spec/invariants.md` -> When a primitive earns its place, `spec/console.md` -> The language, and `releases/first-release.md` -> Traits                                                                   | 2026-09-12 |
| P-460, a command binds every place a recipe leaves open                                                                      | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-12 |
| P-456, which kinds carry an `id`, and why a fleet is cheap                                                                   | `spec/logistics.md` -> Containment, and `releases/first-release.md` -> Traits                                                                                                                            | 2026-09-12 |
| P-457, `traits.4x`, the last file in `spec/data/`                                                                            | `spec/data/traits.4x`, and `spec/console.md` -> The language                                                                                                                                             | 2026-09-12 |
| P-459, a number beside a stored trait's name is its maximum                                                                  | `spec/console.md` -> The language, and `releases/first-release.md` -> Units and structures                                                                                                               | 2026-09-12 |
| P-461, `metal in it` is defined by a name the Traits table does not declare                                                  | `releases/first-release.md` -> Traits                                                                                                                                                                    | 2026-09-12 |
| P-462, a kind's line may name a trait and give it no value                                                                   | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-12 |
| P-464, the vocabulary is four kinds and one sentence says three                                                              | `spec/console.md` -> The language                                                                                                                                                                        | 2026-09-12 |
| P-455, three data files, and `kinds.4x` finished                                                                             | `spec/data/`                                                                                                                                                                                             | 2026-09-12 |
| P-444, the first data file, and the directory it goes in                                                                     | a new file, `spec/data/kinds.4x`                                                                                                                                                                         | 2026-09-12 |
| P-409, uniformity is an instrument, not a preference                                                                         | `docs/process.md` -> Three rules for using AI assistants                                                                                                                                                 | 2026-09-11 |
| P-403, an outbox item's addressing line is not part of what is promoted                                                      | `CLAUDE.md` -> Promotion                                                                                                                                                                                 | 2026-09-11 |
| P-402, vetting gates finishing a release, not shipping one                                                                   | `releases/README.md` -> Vetting, and deletion                                                                                                                                                            | 2026-09-11 |
| P-400, the firing order stops naming `renew`, which `P-399` deleted                                                          | `releases/first-release.md` -> Recipes                                                                                                                                                                   | 2026-09-11 |
| P-389, a source is where what it holds waits, since one of the three holds turns                                             | `spec/resources.md` -> The list                                                                                                                                                                          | 2026-09-11 |
| P-387, a line that makes may be soft, and a line that takes may not                                                          | `spec/invariants.md` -> What a rule may cost                                                                                                                                                             | 2026-09-11 |

## Forecast cleanups that were checked and not filed

A promotion that makes something else stale must file the cleanup. **A promotion that only looks as
though it will must say so**, or the forecast is left standing as an open claim nobody retracts.

| Forecast by | About                                                                                     | Checked 2026-09-11                                                                                 |
| ----------- | ----------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `P-401`     | `docs/process.md`, *the input and the expected are what I review by hand for correctness* | **not needed** - it describes the certifying act, which `P-401` says still happens and comes later |
| `P-401`     | `docs/process.md`, *that is what I vet, and it is the foundation*                         | **not needed** - same reason; `vet` is the later act, not the looking                              |
| `P-401`     | `spec/scenarios.md`, *the foundation, and it is vetted by hand*                           | **not needed** - same reason                                                                       |

**What `P-401` actually changed was the order, not the vocabulary.** Deployment stops waiting for
the certification; the certification keeps its name and its meaning. **This lane wrote *three lines
will be wrong the moment this lands* before checking**, and two minutes of reading says none of them
is.

## Rejected

Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

| What                                                           | Why                                                                                                                                                                                                                                                                                                                                                                                                        |
| -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P-350, run one cold instance against the stale-README task     | Sean, 2026-09-07: *fix the README, let `X-3` stand as the derivation it says it is, and note what future restarts show.* Recommended by this lane on the ground that neither outcome changes what he does. **And the README was already repaired** - `bc1781a`, 2026-09-05 - so the task was unrunnable before it was designed, as `P-315` had been. `X-5` is answered and is the research lens's to close |
| P-348, whether an empty grouping is the answer or the defect   | Sean, 2026-09-07: *I don't know if it is the right answer, but I do expect a small number of distinct things, so I am willing to accept it as correct enough for now.* Reading 1 accepted. He added *I don't want speculation competing with my review process* - two of the three readings were this lane's invention and should not have been offered                                                    |
| P-347, whether founding should require the pioneer to be there | Sean, 2026-09-07: *founding should require the pioneer to be there.* Not a rejection - the decision that makes the current behaviour a defect. Filed as `S-76`; `move` then fires because the game needs it rather than because a case was written for it                                                                                                                                                  |
| P-354, how a thing says what it can do                         | Sean, 2026-09-08, on all three: *trait confirmed; derived confirmed, mainly because i think we gain a lot of simplicity for the same amount of strategic depth; consume-and-produce confirmed.* Not a rejection - three decisions made. **3 is the status quo**, so nothing is written or built. **1 waits on `P-355`**, since a derived trait must name its derivation                                    |
| P-357, where `notes-to-incorporate-then-remove/` sits          | Sean, 2026-09-08: he is using `temporary-notes/` instead, and moved `sample-turn.md` there. **Answered by the directory ceasing to exist** - that file was its last tracked one, committed in `e6cb9e8`, which is the third option the item recommended. No rule needed                                                                                                                                    |
| A, disorder persists and a store built later recovers it       | Sean, 2026-09-05: *while A could be interesting, so could a lot of other things and I need a more solid foundation before such exploration.* Not rejected on its merits - deferred for sequencing, and he has now declined two interesting options this way                                                                                                                                                |

## Withdrawn

| Proposal                                                                              | Why                                                                                                                                                                                                                                                                                                                                                         |            |
| ------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |            |
| P-2, "twenty planet sizes are available below 500"                                    | Superseded by Sean's edit fixing the game at five named sizes.                                                                                                                                                                                                                                                                                              |            |
| P-3, "no two territories are more than `3m` apart"                                    | Superseded by the per-size statement, and incomplete: `3m` holds for class I only, while the large planet is class III where the measured distance is 7.                                                                                                                                                                                                    |            |
| P-4, "the twelve five-neighbour territories sit in six antipodal pairs"               | Derivable from the Goldberg choice, and no rule leans on it.                                                                                                                                                                                                                                                                                                |            |
| P-5, "a pentagon's farthest territory is its antipodal twin"                          | Merged into P-4, then withdrawn with it.                                                                                                                                                                                                                                                                                                                    |            |
| P-7, "the smallest planet has no six-neighbour territories"                           | Derivable from P-6 plus a line Sean had already written - the minimum is 12, **a dodecahedron**.                                                                                                                                                                                                                                                            |            |
| P-447, "a family declares which its members are, or how to tell" | Superseded by P-448 within the hour. Option D was right that a family and a rule are different things and wrong about where the difference is written: a kind declares its family, so no family declares members at all, and thing is said in prose because it is a relationship. |  |
| P-420, "military unit becomes unit" | Carried out inside P-425, which replaces the section rather than editing a word in it. The word is gone from both bullets; the sentence it kept the word in did not survive Sean's redefinition of force, which gives a unit force of its own and no coordinating role. |  |
| P-9, "the distance between every pair is computed once and stored"                    | An implementation directive, not a rule of the game.                                                                                                                                                                                                                                                                                                        |            |
| P-13, "the greatest distance is 3 / 5 / 6 / 7 / 9 by planet size"                     | Determined by the Goldberg choice and the size, nothing leans on it, and the numbers are **already asserted by a test**.                                                                                                                                                                                                                                    |            |
| P-15, "Native life is a planet's own, Feral is printed life gone wild"                | **Feral is behavioural, not an origin**, and origin is not substantively relevant.                                                                                                                                                                                                                                                                          |            |
| P-16, "every unit carries a name that persists when control changes"                  | **A unit has a type, and the type has a name.** Individual units of the same type are not distinguished.                                                                                                                                                                                                                                                    |            |
| P-17, "depart is left unspecified so one rule covers biological and machine"          | Sean chose **starves**, committing to the biological reading for now; robots come later. Recorded in [the backlog](spec-backlog.md).                                                                                                                                                                                                                        |            |
| P-20, "extracting one resource has no effect on extracting any other"                 | Written against the rating model and contradicted by the node model: **labor is shared**, so working a food extractor does compete with working a metal one.                                                                                                                                                                                                |            |
| P-29, "a territory's threat level comes from what is on it"                           | Superseded by P-32. Threat is no longer a quantity a territory carries - it is one direction of **force**.                                                                                                                                                                                                                                                  |            |
| P-36, "accidental damage is force 1, a predator is force 2"                           | Superseded on 2026-08-26: **force is inherent to the territory**, not carried by individual creatures, so there is nothing for a per-creature value to attach to.                                                                                                                                                                                           |            |
| P-40, "the least force eats from food nodes; every species grows by the citizen rule" | Superseded on 2026-08-26. Nature has no population and **does not use nodes** - a node is intentional exploitation. The whole food chain goes with it.                                                                                                                                                                                                      |            |
| P-43, "nothing is exterminated; coordination buys suppression"                        | Superseded on 2026-08-26. It described populations held at zero, and nature no longer has a population to hold anywhere.                                                                                                                                                                                                                                    |            |
| P-46, "citizens and food move between adjacent territories"                           | Cut on 2026-08-26. Sean removed logistics for now so that **each territory is self-contained**; the only thing crossing a boundary is a mobile unit. Recorded in [the backlog](spec-backlog.md).                                                                                                                                                            |            |
| P-56, "a territory satisfies its own consumption first"                               | Cut with P-46 on 2026-08-26 - it only had work to do while a remainder could reach a neighbour.                                                                                                                                                                                                                                                             |            |
| P-124, "where a generated file lives"                                                 | Housekeeping rather than a decision, under the split Sean approved 2026-08-30. Settled by the specification lane and landed in `CLAUDE.md` -> Perspectives in the same commit.                                                                                                                                                                              |            |
| P-144, "capacity and metal content have rules but no numbers"                         | Withdrawn on Sean's instruction, 2026-08-31. Its flat per-territory capacities are wrong under his storage rule: an extractor holds one cycle and a bin holds the rest, so a resource's capacity is the sum of the extractors and bins present, not a constant. **Its metal-content column survives as `P-146`**, which `P-145` depends on.                 |            |
| P-183, a recipe acts in one place and its results appear there                        | Withdrawn into P-190. Sean's `scope` column has a value `everywhere` - food spoils wherever it is - so *one place* would have been wrong as a general rule. The build case was already covered by `spec/logistics.md`: *whatever pays a cost must be in the territory where the thing being paid for is built*.                                             |            |
| P-205, a node is a kind, and a territory has no capacity for nodes                    | Withdrawn for Sean's own counter, which is better. A territory has capacity 8 for citizens and there is no citizen slot kind; a node is a slot invented for one kind of thing where nine others manage without one. `P-206` removes the same exception by splitting the extractor into three kinds, which adds no mechanism at all.                         |            |
| P-227, the first expected data is derived rather than accepted                        | Sean, 2026-09-04: *As a human I can remember to vet the scenario test the first time, it is remembering to do some mundane task each time that is impossible for a human, which is why we need a test to fail for those times to remind the human.* The first seed is a known special case he handles by knowing to. `P-228` is the principle underneath it |            |
| P-226, when a proposal quotes more than one passage the last is the offer             | Sean, 2026-09-04: the distinction that matters is *between proposals that are ready for me to approve, and proposals that are drawing attention to decisions I need to make*. `P-229` makes that split and dissolves this ambiguity instead of ruling on it                                                                                                 |            |
| P-237, three collisions from merging his process note                                 | Sean answered all three on 2026-09-04. Each is now its own item, which is what he can act on: `P-238` for what a proposal is, `P-239` for `pending.md`, `P-240` for who owns production support                                                                                                                                                             |            |
| P-279, the twelve territories collapse to five profiles                               | withdrawn: it was a consequence of `P-272`, which `P-280` reverses                                                                                                                                                                                                                                                                                          | 2026-09-05 |
| P-282, three of six columns on a territory row are not traits | withdrawn: `P-287` removes all three names rather than choosing between them | 2026-09-06 |
| P-315, who may create a lens's directory | Withdrawn by this lane, 2026-09-06. **The premise was mine and it was wrong**: a lens's own directory is its own column, so the research lens creates `lenses/research/` itself on first run. I read *to start one, create ... and tell it* as requiring the files before the instance, and inferred a deadlock from an imperative with no subject. Sean asked how the cause followed and it did not. |
| P-318, which document governs the tie-break | Withdrawn 2026-09-06: not a competing design but stale information. `docs/layers.md` predates `spec/turn.md`'s rule by a week and is superseded in that one respect; the rest of the section stands. Sean: the hole is understood and deferred, and `docs/notes/spec-backlog.md` holds it. The research lane's `X-2`. |
| P-336, `age` is a declared recipe that fires in no state | Dissolved 2026-09-07 by `P-338`, which makes `age` fire on anything carrying the number. The code lane's `C-61` is the same finding and is answered with it. |
| P-337, a starved unit is marked unusable and no artifact shows it | Dissolved 2026-09-07 by `P-339`: with no unit taking upkeep there is no starved unit, so the marking has nothing left to mark. Answers the code lane's `C-62`. |
