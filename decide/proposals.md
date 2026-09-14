# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-509 - Resources sit in the territory, and are allocated only when something leaves

**to** sean · **status** open · **G3** chosen 2026-09-14, and the text follows it · **raised** 2026-09-14 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/logistics.md` -> Containment

**This dissolves the problem the last four proposals were solving.** Your three scenarios were hard
to express because the game was being asked **where each metal sits while nothing is moving**, and
nothing in the game can tell those arrangements apart.

> **A resource in a place is in that place, not in a container inside it.** What a place holds of a
> kind is one number. **The things in it that can hold that kind contribute capacity and hold
> nothing**, and at the turn's end what the place holds beyond that capacity is lost.
>
> **A place's capacity for a kind is the sum of what is in it that can hold that kind**, and a
> place declares none of its own. **This holds of every place**: an orbit has room for the fuel its
> units carry and for nothing else, because that is what is in it.
>
> **A bin is a thing.** What holds a kind is a store for that kind, and a thing with two bins holds
> two stores. **A store's capacity is what its kind declares**, and it holds nothing.
>
> **A thing that leaves takes what it hauls.** A unit moving out of a place is given an amount of
> each kind, no more than its own capacity for that kind, and that amount joins the number the new
> place holds. **Allocation happens at the moment of leaving and at no other.**

## Your three scenarios stop being three

```
{territory id:1}
  {energy} -> 5
  {metal} -> 25
  {store resource:metal} -> 3
  {transport resource:metal} -> 2
```

**Fifty metal of capacity, twenty-five metal held, and no question about which store or which
transport.** *15 in storage and 3 and 7 in the transports* is not a state the game can be in while
all three are standing in territory 1 - **it is a fact about two transports that have not moved
yet.** It becomes true the moment they do:

```
{move unit:transport to:2 haul-metal:3}
{move unit:transport to:5 haul-metal:7}
```

**And the two transports stay one entry of two until one of them leaves**, which is the stacking you
said this would conflict with. It does not: **nothing distinguishes them until something does.**

## The three cases you named, and they are the whole interface

|               |                                                                       |
| ------------- | --------------------------------------------------------------------- |
| **haul most** | the default - as much as capacity allows, because you were picking up |
| **haul none** | you were dropping off                                                 |
| **a number**  | `haul-metal:3`                                                        |

**The command's legality is its own capacity**, which the rule above states and nothing else has to
check.

## What this withdraws, and it is most of this week

**`P-508`** - positions. They existed to say *which transport*, and there is no which. **`P-507`** is
already answered and that answer goes with it. **`P-504`** shrinks: waste is `what the place holds`
against `the capacity in it`, one comparison rather than a `sum` over an expression, so the notation
needs no new word.

**`P-502` survives in half.** *A bin is a store* stands - a store is a thing and declares a capacity.
*What it holds is what it contains* does not: **it holds nothing.**

**`P-505` stands and gets easier.** Nothing is stored without being asked, because nothing is stored
at all - a resource is simply in the place, and the turn's end takes what will not fit.

## The one thing this lane checked and it is a real tension

**`spec/logistics.md`: *a rule may ask whether something is absent only where what would hold it
declares a limit for it.*** A territory declares **no limit** for a resource - the release says so
outright - so **a rule may not ask after its free capacity.**

**Detecting waste is a query and not a rule**, which is the distinction that saves it: a user
interface asking *would ending the turn lose metal* is not a recipe firing. **But `free <kind> of x`
is listed among the expressions a guard is built from**, and a guard is part of a rule. So the word
exists on the rule side of a line this leans on being the other side of.

**Answered on 2026-09-14, in `P-510`, and the answer is that it was not a tension.** Under this
proposal the limit stops being the container's and becomes the **place's**: what a place holds beyond
the capacity in it is lost, and that capacity is a number the place has. **So a territory does
declare a limit for a resource** - derived from what is in it - and *a rule may ask whether something
is absent only where what would hold it declares a limit* is satisfied rather than strained.

**And `P-510` is answered: `G3`, 2026-09-14.** An orbit holds units and nothing else, and `move`
burns energy the unit carries, so an ark in orbit had nothing to pool with. **The answer costs no
rule**: a place's capacity is already the sum of what is in it, so an orbit holding an ark whose fuel
store is 2 has room for 2 **without anything declaring it**, and an empty orbit has room for nothing.
The sentence that changes is the release's *an orbit holds units and nothing else*, and
`spec/orbit.md` never said it - it forbids extraction and is silent about holding.

**So pooling has no exception**, which is the whole of what `G3` bought over `G2`: under `G2` an ark
in orbit would hold its own contents, and everything `P-507` and `P-508` were built to solve would
come back inside orbits.

## What it absorbs, so these can land together

**`P-502`'s surviving half is in the text above** - *a bin is a thing* - and the rest of that item is
withdrawn: a store holds nothing, so there is nothing for capacity, occupied and free to be about on
one, and no two entries ever share a description because **only a place holds anything and every
place carries an `id`**.

**`P-508` is withdrawn with it.** A position said *which of two alike things*, and there is never a
which.
### P-506 - The notation and the two models get their names

**to** sean · **status** open · **raised** 2026-09-14 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**`spec/console.md` says *there is one notation* and never names it.** Three things get named here,
and the third is this lane's reading of your words rather than a transcription - say so if it is
wrong and it changes before it lands.

> **The notation is the game notation.** A command, a state and a data file are written in it, and
> there is nothing else to write them in. **The game notation is the source**: what is stated is
> stated in it, and every other form of the same facts is a presentation, generated and never
> canonical.
>
> **A presentation has a name too.** The **relational model** shows the game as relations - one
> table per relation, each fact stated once, nothing derived shown beside what it came from. The
> **physical model** shows it as the game holds it: every thing inside the thing that holds it, a
> tree from the game down.
>
> **The two show the same facts and neither is the game.** A reader chooses by what they are
> checking: whether a fact is stated once, or where a thing actually is.

## What each name already points at

| Name                 | What it is today                                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------------------------------- |
| **game notation**    | `spec/data/*.4x`, `scenario/commands/*.4x`, `scenario/expected/*.4x`                                                |
| **relational model** | `reports/state.md` - *the scenario's result as one table per relation, fully normalized*                            |
| **physical model**   | `reports/entities.md` - *every thing with its components* - and the containment tree the dump writes by indentation |

## The one this lane is least sure of

**`physical model`.** You named three and this lane matched two to things that exist and inferred the
third. **The reading taken is *how the game actually holds it* - the containment tree** - because
that is the presentation the dump writes and the only one of the three with no name. If you meant
something else by physical, this is the line to change.

## Why it is worth naming at all, and it is your own argument

**You said it in the same message**: *if you communicate specifics in game notation, I suspect these
proposals would never have been created in the first place, only to be rejected.* **A thing with no
name gets described instead of used**, and a described notation is prose, which is what let `P-501`
and `P-503` be written vaguely enough to need withdrawing.
### P-505 - Nothing spills, because nothing was inside anything

**to** sean · **status** open · **raised** 2026-09-14 · **rewritten** 2026-09-14, because pooling reversed it · **kind** entailed · **shape** an instruction · **asks** approval · **into** `spec/logistics.md` -> Containment

**This item said the opposite yesterday and was wrong in the light of `P-509`.** It wanted to delete
*at the turn's end what there is room for is kept and the rest is lost*, on the grounds that nothing
should be stored without being asked. **Under pooling that sentence is simply true** - what a place
holds beyond its capacity is lost - so it is not the half that has to go.

**What has to go is the half above it.** `spec/logistics.md` lines 26 to 28 read:

```
- **When a thing that contains things is consumed, what it held falls loose where it stood.** It
  is not destroyed with its container: it goes into disorder, and at the turn's end what there is
  room for is kept and the rest is lost.
```

**It becomes:**

```
- **When a thing that holds capacity is consumed, the place it stood in has that much less room.**
  Nothing falls loose, because nothing was inside it: what a place holds is the place's, and
  destroying a store leaves the resources where they already were.
```

## Why the whole bullet changes rather than one clause

**Under `P-509` a store holds nothing**, so *what it held falls loose* has no referent. Destroying a
store does not move anything - it lowers the place's capacity, and **the excess is lost at the
turn's end by the rule that was already there**, which is the clause this item previously wanted to
delete.

## What that makes of disorder

**Disorder stops being a place things are.** It was the state of a resource that was in no
container; under pooling a resource is in the **place**, always, and either the place has room for it
or it is lost at the turn's end. **The word can go from `releases/first-release.md` too**, which is a
cleanup this does not do and `P-509` landing will make findable.

## How to tell it was carried out

**`spec/logistics.md` no longer contains `falls loose`**, and contains `has that much less room`
exactly once. **And the clause this item used to attack is still there**, in the bullet below, which
is the check that this rewrite reversed the right half.

### P-504 - Waste is free capacity gone negative, and one sentence allows it

**to** sean · **status** open · **raised** 2026-09-14 · **rewritten** 2026-09-14, and pooling shrank it to one sentence · **kind** entailed · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**You asked for a way to detect waste so a player can be warned.** Yesterday this needed a new word
in the expression language. **Under `P-509` it needs none** - only permission for a number that was
assumed non-negative to go below zero.

> **`free <kind> of x` may be less than zero.** A place holds what it holds whatever room there is,
> so where what it holds exceeds the capacity in it, its free capacity for that kind is the
> shortfall written as a negative number. **That is the amount the turn's end will take.**

## What it lets a player be warned with, and it is one guard

```
free metal of {territory id:1} < 0
```

**Nothing else is added.** `free <kind> of x` is already promoted, and `<` is already how a guard
compares.

## Why it goes negative rather than something else being invented

**Because it is the same number.** Free capacity is `capacity - held`, and the case being detected is
`held > capacity`. **A second expression for the same subtraction with the sign flipped would be one
fact stated twice**, which `spec/invariants.md` forbids.

**And it is reachable in ordinary play, not just by mistake.** A territory with 500 fixed storage and
a transport standing in it has room for 510; the transport leaves and the room goes with it. **The
place did not change what it holds, and its free capacity went negative.**

## What it does not do

**It does not decide what happens about waste.** You said refusing the turn is a user interface
concern; this stops exactly where you stopped, by making the guard writable so that refusing is
implementable.

**And it does not need `sum`.** The earlier version of this item added *`sum` aggregates an
expression rather than only a trait*, so that free capacity could be added up across containers.
**Under pooling a place's capacity is one number already**, so there is nothing to add up and that
sentence is withdrawn with the rest of the old item.
