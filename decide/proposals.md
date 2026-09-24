# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-541 - Force, garrison and nature leave the specification; biome comes back to the data

**to** sean · **status** open · **raised** 2026-09-23 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `spec/future/`, `spec/control.md`, `spec/planet.md`

**Your words, 2026-09-23:** *I want to push force, garrison, and nature out of the data. It should
be somewhere we can reach it for future design, but it should not influence the code.* And:
*the biome is a bit trickier, it doesn't affect game mechanics anymore, but it does affect the
realistic rendering of the planet and I am not willing to give this up.*

## What lands

**A new directory, `spec/future/`**, and the rules move into it whole. **Six places, measured
rather than remembered** - this lane first wrote two and one of the two was wrong:

```
from                    what moves
spec/control.md         Force, Producing force, and the force clauses of Gaining and
                        holding ground
spec/structures.md      Garrison
spec/console.md:211     set force <territory> <force>
spec/turn.md:24         nature takes back what is no longer held
spec/data/limit.4x:1    {limit container:territory contained:garrison n:1}
spec/data/biomes.4x     the nature:N field, from five of the six values
```

**`spec/control.md` keeps Winning and Losing**, which is what the release builds.

**Nothing is rewritten, with one exception that cannot be moved whole.** `spec/turn.md`'s clause
is one of five in a single bullet about ending a turn, so it leaves by deletion and the bullet
keeps the other four. **Everything else arrives in `spec/future/force.md` as it is**, under the
heading below.

## The last row is where your two answers meet, and it is the only one that needed thinking about

**`spec/data/biomes.4x` says a biome sets a force of nature** - `{value name:jungle of:biome
nature:2}`. **That is the mechanical effect you are cutting, written into the fact you are
keeping**, so the values stay and the field goes:

```
{value name:jungle of:biome}
```

## Three places keep the words, because they are examples rather than rules

```
spec/console.md:142     `t.nature` as the example of what a path reads
spec/invariants.md:81   a garrison, twice, as the example of an invariant that reads as a contortion
spec/README.md:27       the Control row, which stops being accurate
```

**The first two would have to be rewritten to illustrate the notation with something else**, which
is work for no gain. **The third is a wrong index entry and this lane's to fix**, needing nothing
from you.

## Answered `B2` on 2026-09-23, and the reason is what protects it

**Your words**: *I want to keep the data in the code and enforced specification lean, so no
garrison, force, or nature, but I do want to keep biome because it is needed to render the planet
realistically.*

**So the trait comes back**, and `spec/data/biomes.4x` stops declaring six values of nothing -
which is the state this lane left it in on the 21st by deleting the trait and keeping the values.

**Into `spec/data/traits.4x`:**

```
{trait name:biome admits:value kept:thing}
```

**And into `spec/planet.md`, beside *each territory has a biome*:**

> **A biome earns its place by being shown rather than by being obeyed.** It is what the realistic
> drawing reads to make a territory look like the place it is - which is a reason to keep a fact
> that nothing else in this specification has, and the reason a sweep for data no rule reads must
> not remove it.

## Why it no longer says *no mechanical effect*, which is your words and not this lane's

**Because `spec/planet.md` four lines below it says one.** *No territory can be claimed whose
biome is ocean* - a rule that reads a biome, in the section this sentence lands in.

**It is out of the release and in the specification**, which is why you are right about the game
you are building and the sentence was still wrong about the file. **Nothing needs deciding**: the
clause is gone and the protective half is intact. If you want the ocean rule to leave the
specification too, that is a separate proposal and this lane has not filed one.

## Why that sentence and not just the trait row

**Because this lane already cut it once for having no mechanical effect.** `P-522` removed force,
garrison, nature and biome from the release; regenerating `spec/data/` from the cut release took
the biome trait with them, and nothing in the file said not to.

**A sweep for unused data would do it again.** Every other fact in `spec/data/` is there because a
rule reads it, so *no rule reads this* is normally a reason to remove a row. **Biome is the one
exception and the exception has to be written where the sweeper will meet it.**

## The three that go, and they are a record rather than a deletion

**Your reason, which changes what `spec/future/` is**: *garrison, force, and nature are out of
scope for now but I will want a historic record because I intend to get to them once I play the
game.*

**So `spec/future/force.md` is not a graveyard.** `P-539`'s rule already says such a document is
*kept, linked and findable*; **your sentence says why it is kept**, which is worth carrying in the
file's own opening line so a reader knows it is waiting rather than abandoned.

> **These rules are not built and are not abandoned.** They are here because the game wants them
> once it can be played, and the first release cannot be played while it is building them.

## How to tell it was carried out

**Four assertions, run in the promoting commit.** Every moved line is in `spec/future/force.md`
byte for byte as its source had it; none of them is still in the source; `spec/data/traits.4x`
holds the biome row and `spec/data/biomes.4x` holds six values and no `nature`; and both
quotations above are present where they were offered.

**And the gate goes red until the code lane follows**, because `spec/data/` is what the engine
loads. **That is expected rather than a surprise.** `spec/data/limit.4x` keeps four of its five
rows - this lane first wrote *its only row*, having counted the rows that mention a garrison and
read the answer as the size of the file.

## Two things move with it that are nobody's idea, and are named so they are not surprises

**`releases/first-release.md` cites the sections by their old home.** Its `Out of scope` entries
name `spec/control.md` -> Force and -> Producing force, which is where they are today. **A link
broken by a file move is this lane's to repair** and the entries keep saying exactly what they say.

**And `spec/README.md` gains a row**, because its own rule is *add a file when a topic firms up,
add its row here first* - and `P-539` requires the row to say which kind of document it is. **The
`Control` row also stops being accurate**, which is the third of the three examples above.


### P-514 - Three rows of `spec/data/line.4x` are unwritable, and this lane's own check said there was one

**to** sean · **status** withdrawn · **withdrawn** 2026-09-21 · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**Withdrawn 2026-09-21, and the reason is his.** *I am fine with dumping all rules and replacing them with thin engine* - so the form `spec/data/` should take stops being a question for him and becomes whatever the engine reads. **This item asks how to write a file whose writer is being replaced.**

**And the engine's data has none of these.** Measured across `spec/tests/`: zero rows carry a quantity of more than one word, against three in `spec/data/line.4x`. The shape the engine uses already avoids what this item reports.
**Kept rather than deleted**, because `CLAUDE.md` says a rejection is recorded with its reason or the same item is filed again. **It comes back if the engine's own data turns out to have the same gap** - and for three of the five, measurement says it does not.

**Read against `P-530`, 2026-09-21.** **Same as `P-513`: right about `spec/data/`, and
`spec/data/` is in question.** The defect it reports is real either way - three rows cannot be
read back - so if those files are replaced this is fixed by the replacement rather than by this
proposal. **The third row it could not fix is the part worth keeping**, whatever notation wins.

**`P-497`'s migration wrote three rows that cannot be read back.** Found by the code lane building a
generator against the same data.

```
{line block:work seq:5 role:produce qty:`$where`'s density for that resource kind:resource}
{line block:muster seq:4 role:produce qty:that citizen's strength kind:force}
{line block:stand seq:3 role:produce qty:that unit's strength kind:force}
```

**A value with spaces in it cannot be told from the words after it.** `qty:that citizen's strength
kind:force` reads as a `qty` of `that`, then four words belonging to nothing. **The notation has no
multi-word value and these three rows assume one.**

## And this lane's check reported exactly one such cell

`P-497` asserted that precisely one cell of the release could not be represented - `move`'s *joined
to `$from` by an edge the unit crosses* - and named it so that a second would fail the run.

**It asked only about the Traits column.** Quantities were written straight through without being
classified at all, so three unrepresentable cells passed a check built to catch exactly that, in the
item that introduced the check. **A right answer about the wrong population**, inside the migration
whose whole argument was that counts cannot ask whether each row is right.

## What lands, and the shape is already in the file

`spec/console.md` already has the thing these quantities are: **a path, which reads a trait of
something a name is bound to.** And `P-497`'s `place-line:` already refers to another row of the
same block by its sequence.

> **A quantity that reads a trait names what it reads it from and what it reads.** Where a relation
> writes such a quantity it uses two columns rather than one - which row of the block the thing came
> from, and which trait of it is read. **A value is one word**, and a quantity that needs more than
> one word is more than one fact.

**So the three rows become:**

```
{line block:muster seq:4 role:produce qty-line:2 qty-trait:strength kind:force}
{line block:stand  seq:3 role:produce qty-line:1 qty-trait:strength kind:force}
```

## The third one is not the same and is not fixed here

**`$where`'s density for that resource reads a trait of a place, per resource.** It is not *the
strength of the thing at row two*; it is a density indexed by which resource the block is for.
**Two columns do not hold it**, and this lane is not inventing a third form for one row.

**It is named rather than fixed**, which is the thing `P-497`'s check was supposed to do and did not.

## What this costs you to read

**Nothing in the game changes.** The release's Qty cells are untouched; this is about how the
relational form writes what they already say, and about one row it still cannot.
### P-513 - A relation names its columns, and nothing says so

**to** sean · **status** withdrawn · **withdrawn** 2026-09-21 · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**Withdrawn 2026-09-21, and the reason is his.** *I am fine with dumping all rules and replacing them with thin engine* - so the form `spec/data/` should take stops being a question for him and becomes whatever the engine reads. **This item asks how to write a file whose writer is being replaced.**

**And the engine's notation does not have this gap.** Every field in `prototypes/thin-engine/data/` is named, so a relation's column order is a canonical-form question rather than a parsing one - two writers cannot disagree about what a row means, only about how it looks.
**Kept rather than deleted**, because `CLAUDE.md` says a rejection is recorded with its reason or the same item is filed again. **It comes back if the engine's own data turns out to have the same gap** - and for three of the five, measurement says it does not.

**Read against `P-530`, 2026-09-21.** **This may not survive the change in direction.** It says a
relation of `spec/data/` must declare its column order. The thin-engine states the same facts in
its own notation, and whether `spec/data/` survives at all is open - so this is right about the
file it names and the file may go. **Worth answering `P-530` first.**

**`spec/data/` is the source now and nothing states what order its words go in.** `spec/console.md`
fixes the order for a **description** - *`id` first, then every other trait alphabetically, then
`occupied`, `free` and `capacity` last* - and **that rule describes none of the eight relations**.

```
carries      kind trait                          alphabetical would be: kind trait
member       kind family                                                family kind
limit        container contained n                                      contained container n
above        orbit territory                                            orbit territory
block        id recipe owner                                            id owner recipe
line         block seq role qty kind place-bound                        block kind place-bound qty role seq
constraint   block seq trait compare n                                  block compare n seq trait
for          block seq kind                                             block kind seq
```

**Two of the eight happen to match and six do not** - `carries` and `above`, and nothing else -
which is worse than none matching: the rule appears to hold until it is relied on. **This lane first
wrote three**, from the table directly above it, and the code lane re-derived it to two.

> **A relation names its columns, and a row gives them in that order.** The order is the relation's
> own and is stated where the relation is declared. **A description's order is a different rule** -
> it ranks traits because a description has no declaration to name them in.

## What this is for, and it is the guarantee that is missing

`spec/console.md` already says **the same state is always the same bytes**. **That covers a
description and does not reach a relation row**, so two writers of `spec/data/` could disagree about
column order and both be right. **The code lane hit it building a generator**: `Description::ordered`
cannot write these files, because no single ranking gives both `carries` and `constraint` their
order.

## Where the measurement is weaker than the code lane's and stronger overall

**Their example no longer separates the two.** They cited `carries` ordering kind before trait
against `constraint` ordering trait before kind - and `constraint` has **no** `kind` column today,
since `P-511` deleted the only row that had one. **The finding survives the example dying**: the
eight orders above are measured from the files as they stand, and five of them the description rule
gets wrong.

## What it does not do

**It does not choose the orders.** Each relation's is whatever it is declared to be, and the eight
above are what the code lane's generator writes. **This says they must be declared**, not what they
should say.
