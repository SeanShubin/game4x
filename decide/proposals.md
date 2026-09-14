# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-514 - Three rows of `spec/data/line.4x` are unwritable, and this lane's own check said there was one

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

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

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

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
### P-512 - *Where things are* still says a tank holds fuel, and one row of it changes the game

**to** sean · **status** open · **raised** 2026-09-14 · **kind** entailed · **shape** an instruction · **asks** approval · **into** `releases/first-release.md` -> Where things are

**The code lane filed `C-125` and cannot proceed past it.** `P-509` and the release now disagree
about whether a unit's tank holds anything, and the disagreement is load-bearing rather than
verbal.

```
spec/logistics.md      The things in it that can hold that kind contribute capacity and hold nothing

releases/first-release.md -> Where things are
                       | Container     | Holds  | Up to             |
                       | a unit's tank | energy | the unit's fuel   |
```

## What lands

**The section's opening sentence becomes:**

```
Every thing but the game is in another thing, and this release has two sorts of thing that give a
place room.
```

**And the table becomes:**

| Thing         | Gives room for                | Up to           |
| ------------- | ----------------------------- | --------------- |
| a store       | the resource it was built for | 10              |
| a unit's tank | energy                        | the unit's fuel |

**The territory's row goes.** A place does not give itself room - under `P-509` its capacity is the
sum of what is in it, so *a territory holds that kind up to its free capacity for that kind* is the
rule stated as if it were a container.

## Why the heading changes and the row does not

**The row was right and the column was wrong.** *A unit's tank, energy, the unit's fuel* is a true
statement about **capacity** and a false one about holding. **Renaming the column is the whole of the
correction**, and it is why this is an instruction rather than rows: the words offered describe a
table that does not exist yet, so none of them lands as written.

## The thing that is not bookkeeping, and it is yours

**A unit can no longer move out of a place that has no energy.** `P-511` made `move` consume its
energy from `$from`; if a tank holds nothing, the place must have it. **Today a unit moves on fuel it
carries and is refused with `NoCells` when its own tank is empty.**

**It is nearly equivalent and not quite.** A unit hauls energy when it leaves, and that energy joins
the new place - so a unit can still cross an empty territory by bringing fuel, and chain moves on
what it brought. **What changes is that the fuel it brought is the place's**, so anything else
standing there may spend it.

**That is a real change to the game and this lane is not deciding it.** It is the last clause of
`P-509` arriving somewhere visible.

## And it moves something you have already reviewed

**The containment tree stops drawing energy inside a unit.** `P-485` and `S-128` put a pioneer's fuel
there - `{pioneer ...} -> 1` over `{energy} -> 2` - and under pooling that energy is the territory's.
**`scenario/expected/play.4x` changes**, which is the file `R-6` rests on and `P-225`'s protocol says
is reseeded and unreviewed until you read it.

**The code lane has built nothing on this** and says so; it has the reading above and is waiting.

## How to tell it was carried out

**`releases/first-release.md` -> Where things are has two rows and no `Holds` column.** Its heading
row reads `| Thing | Gives room for | Up to |`, the store and the tank rows are present with their
cells unchanged, and **the territory row is gone**.

**And the section says `two sorts of thing that give a place room`**, where it said `three sorts of
capacity`. Three became two by the territory leaving, which is the check that the right row went.

