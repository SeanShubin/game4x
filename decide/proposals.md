# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-541 - Force, garrison and nature leave the data; biome was never in it

**to** sean · **status** open · **raised** 2026-09-23 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `spec/future/`, `spec/control.md`, `spec/planet.md`

**Your words, 2026-09-23:** *I want to push force, garrison, and nature out of the data. It should
be somewhere we can reach it for future design, but it should not influence the code.* And:
*the biome is a bit trickier, it doesn't affect game mechanics anymore, but it does affect the
realistic rendering of the planet and I am not willing to give this up.*

## The first three are half done and the half that is left is prose

**They are already out of the data.** `spec/data/` declares no `force`, `garrison` or `nature` -
this lane regenerated those files from the cut release on the 21st. **What is left is `spec/`
prose**, which still states all three as rules of the game:

```
spec/control.md   ## Force, ## Producing force, and three bullets of Gaining and holding ground
spec/planet.md    a territory's force of nature
```

**`P-539` gave you the rule for this**: *a document says what the game is, or it says what the
game will be, and it says which.* **So these sections move rather than being deleted**, which is
what *somewhere we can reach it for future design* asks for.

## What lands

**A new directory, `spec/future/`**, and the three sections move into it whole:

```
spec/future/force.md      Force, Producing force, and the force clauses of Gaining and holding
                          ground - taken from spec/control.md
```

**`spec/control.md` keeps Winning and Losing**, which is what the release builds. **`spec/planet.md`
loses *a territory's force of nature*** and keeps everything else.

**Nothing is rewritten.** The sections arrive in `spec/future/force.md` as they are, under a
heading saying they are not built and what would have to happen for them to be.

## Biome: the thing you are unwilling to give up is not in the data and never was

**Measured, and this is the part that changes your answer.** Three different things are called
biome:

```
spec/data/biomes.4x                 6 value rows        read by nothing
crates/planet-model/src/biome.rs    enum Biome          the list actually in use
the terrain field                                       what the drawing samples
```

**`planet-render/src/realistic.rs:117` says it in its own comment**: *the biome comes from the
field, sampled here rather than taken from the model.* **So the realistic rendering does not read
game data for biome at all** - it reads the terrain, and the terrain is generated.

**Every mention of `biomes.4x` in `crates/` is a comment or a link**, not a read. Checked across
the tree.

## So biome needs nothing from you, and one thing needs saying

**The drawing you are unwilling to give up is safe** and was never at risk from the mechanical
cut. **What is actually incoherent is `spec/data/biomes.4x`**: it declares six values of a trait
that `traits.4x` no longer declares, because this lane deleted the trait on the 21st and left the
values. **Six rows declaring values of nothing.**

**Two ways to make it honest, and this is the only biome decision left:**

**`B1` - the values go too.** The list that matters is the Rust enum and the terrain field;
`biomes.4x` is a seventh copy of a list nothing reads.

**`B2` - the trait comes back.** `{trait name:biome admits:value kept:thing}` returns, and a
territory carries a biome in the data again - **for the drawing's sake rather than the rules'**,
which is a reason the data has never had before.

**`B1` unless you want a territory's biome to be a fact the game holds** rather than a fact the
terrain implies. **That is a real question about the game and not about tidiness**: if the biome
is only ever sampled from terrain, two territories can disagree with their own drawing and
nothing notices.

### P-540 - One sentence so a test can say what the status bar said

**to** sean · **status** open · **raised** 2026-09-22 · **kind** invented · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**You chose `U2` and `N1`.** A screen is a tree, containment places things, and there is one
notation extended rather than a second. **This is what is left to approve**, and the working
behind it is in [`docs/notes/decisions.md`](../docs/notes/decisions.md).

## What lands

After *nothing in a data file is quoted*:

> **A trait may admit prose, and a value of one is quoted.** That is the one exception, and it is
> narrow on purpose: **a quoted value whose trait does not admit prose is a defect**, and so is an
> unquoted sentence. **Prose is shown to a person and never compared** - nothing sorts it, matches
> on it, or reads a word out of it.

## Why the exception is where it is

**`admits` already says what a value may be** - a number, a family, or `value` where the values
declare themselves. **Prose is a fourth**, and putting it there makes the edge checkable: the
trait says which fields may be quoted, so a tool can name a quotation in the wrong place.

**It is invented.** You asked for the words to be sayable and did not ask for this shape; if you
want prose carried another way, this is the paragraph to change.

## The three constraints your sketch tripped, which still hold under `U2`

**Two of the three are unchanged by this proposal and worth having in front of you**, because your
sketch used all three:

```
no list form       [ new-game exit-game ]  is not a thing the notation carries
                   one row per fact is how it says a set
arguments named    {status-bar "start new game"} has a value with no key
prose quoted       this proposal, and only where a trait admits it
```

## Two facts your draft left blank, answerable from `spec/`

**The planet sizes are stated**: tiny 12, small 32, medium 42, large 72, huge 92.

**And *load game absent if none exists* makes the first screen two tests**, not one with a
condition - because absent is not disabled, and a test asserts one screen.

## What `U2` costs, stated once so you have accepted it knowingly

**Every interface test carries the screen's skeleton** - a `{screen}`, its regions, then its
contents. Three rows before anything interesting. **It is the same repetition as `{territory
id:1}` in every game test**, which has never bothered you, and it is the price of a layout mistake
failing a test you read rather than a check somebody wrote.

## What happens next needs nothing from this queue

**The first interface tests are drafted by this lane and read by you in the review application.**
`P-531` settled that: a test is not a proposal. **So your queue stays empty while the screens get
written.**

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