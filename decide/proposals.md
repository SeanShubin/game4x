# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-509 - Resources sit in the territory, and are allocated only when something leaves

**to** sean · **status** open · **raised** 2026-09-14 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/logistics.md` -> Containment

**This dissolves the problem the last four proposals were solving.** Your three scenarios were hard
to express because the game was being asked **where each metal sits while nothing is moving**, and
nothing in the game can tell those arrangements apart.

> **A resource in a place is in that place, not in a container inside it.** What a place holds of a
> kind is one number. **The things in it that can hold that kind contribute capacity and hold
> nothing**, and at the turn's end what the place holds beyond that capacity is lost.
>
> **A thing that leaves takes what it hauls.** A unit moving out of a place is given an amount of
> each kind, no more than its own capacity for that kind, and it holds that amount until it arrives
> - where the amount rejoins the number the new place holds. **Allocation happens at the moment of
> leaving and at no other.**

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

**This lane did not resolve it and is not guessing.** It may want *the capacity in a place* to be a
different expression from *free capacity of a container*, or it may want the invariant read as
being about recipes rather than about queries. **That is the question this proposal leaves open**,
and it is smaller than the four it closes.
### P-508 - Storage, concretely: a position names an entry and a command acts on one of it

**to** sean · **status** open, **held** 2026-09-14 pending `P-509`, which removes the need for a position · **raised** 2026-09-14 · **kind** invented · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**This picks `E4` and stops offering options**, on your *something concrete that can't possibly work
can be adapted*. It replaces `P-501` and answers `P-507`. **Two things it cannot do are written out
at the bottom rather than left to be discovered.**

> **A container's contents are a listing, and an entry's place in it is its position.** The first
> entry is at 1. **A command names a thing by position**, and the position is read against the state
> the command is applied to.
>
> **A position names an entry, and an entry may be several things.** A command acts on **one** of
> them, and which one is not a choice: the things in an entry are alike in every respect the game
> records, so each gives the same state afterwards.
>
> **A position is not an identity.** An `id` names one thing for ever; a position names whatever is
> at that place in a listing now, and the same thing is at different positions as the listing
> changes.

## Your first three scenarios, against a real listing

```
{territory id:1}
  1  {energy} -> 5
  2  {metal} -> 25
  3  {store resource:metal} -> 3
  4  {transport resource:metal} -> 2
```

```
25 in storage       {stow into:3 kind:metal repeat:10}
                    {stow into:3 kind:metal repeat:10}
                    {stow into:3 kind:metal repeat:5}

15, and 3 and 7     {stow into:4 kind:metal repeat:3}
                    {stow into:5 kind:metal repeat:7}
                    {stow into:3 kind:metal repeat:5}   three times

5, and 10 each      {stow into:4 kind:metal repeat:10}
                    {stow into:4 kind:metal repeat:10}
                    {stow into:3 kind:metal repeat:5}
```

**`into:3` three times is three different stores**, because the first command takes one of the three
out of that entry and into one of its own.

## Your fourth, which is the one the other three hid

**One transport holding 1 metal, one holding 2; two into the first and five into the second.**

```
{territory id:1}
  4  {transport resource:metal}  {metal} -> 1
  5  {transport resource:metal}  {metal} -> 2
```

```
{stow into:4 kind:metal repeat:2}
{stow into:4 kind:metal repeat:5}
```

**Both are `into:4`, and that is the flaw rather than a typo.** After the first command that
transport holds 3, so the listing re-sorts: the one holding 2 is now at 4 and the one holding 3 at
5. **A position is read against the state it is applied to, so the second command's 4 is a different
transport from the first command's 4.**

## What that costs, said plainly

**A written sequence of positional commands is fragile.** Insert a command, or change a `repeat`, and
every position after it may mean something else. `scenario/commands/play.4x` is a file you derive by
hand, and this makes a hand edit in the middle of it dangerous in a way it is not today.

**It is the cost of the thing being concrete.** The alternative was three ways of writing contents
inline, each of which grows with what a thing holds. **This one is wrong in a way that shows up
immediately** - a mis-positioned command puts metal somewhere visible - rather than in a way that
shows up as a notation nobody can read.

## Two holes this lane found writing it out, and neither is closed here

**The tie-break needed the quantity, and `P-502` now says so.** Two transports holding `{metal} -> 1`
and `{metal} -> 2` have contents whose **descriptions are equal** and whose quantities differ, so
sorting by description alone did not separate them - and a position into an order that is not total
names nothing. **Corrected in `P-502` rather than left here**, because that is the item the rule is
in. It was found by writing this one's fourth scenario out.

**A stack cannot be split by position.** `{transport resource:metal} -> 2` is one entry, so `into:4`
twice fills the same transport unless the first command made them differ. Your *fill up each
transport and move the ones that are full* works because filling one makes it differ; **a command
that had to act on a particular one of two still-identical things could not say so.** Nothing in
your four scenarios needs that, and this lane does not know whether something later will.

## What it withdraws

**`P-501`.** Its *a description names a set* is replaced by a position, and its *a command acts on
one of them* survives here, applied to an entry rather than to a set. **One way to name a thing
rather than two**, which is the uniformity you asked for.
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
### P-505 - Nothing is stored without being asked, and one sentence says so

**to** sean · **status** open · **raised** 2026-09-14 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `spec/logistics.md` -> Containment

**One sentence changes, and it is the half of a rule that stores without being asked.**
`spec/logistics.md` line 26 to 28 reads:

```
- **When a thing that contains things is consumed, what it held falls loose where it stood.** It
  is not destroyed with its container: it goes into disorder, and at the turn's end what there is
  room for is kept and the rest is lost.
```

**It becomes:**

```
- **When a thing that contains things is consumed, what it held falls loose where it stood.** It
  is not destroyed with its container: it goes into disorder, and what is in disorder at the
  turn's end is lost.
```

## What that is, in one line

**`and at the turn's end what there is room for is kept and the rest is lost` becomes `and what is
in disorder at the turn's end is lost`.** Nothing else in the bullet moves.

## Why

**Sean, 2026-09-14**: *we don't need to automatically store excess that we have capacity for, we
need to be able to detect if we do have excess we would lose.*

**And the storing half takes a choice away rather than saving work.** Filling every container with
room means a transport is loaded whether or not the player wanted cargo in it, and *twenty-five in
storage and none in the transports* becomes unsayable. `P-503` proposed making that sweep even-handed
and was withdrawn for the same reason.

## How to tell it was carried out

**`spec/logistics.md` no longer contains the phrase `what there is room for is kept`**, and contains
`what is in disorder at the turn's end is lost` exactly once. The rest of the bullet is byte for
byte what it was.

## What it leaves consistent rather than changing

**`releases/first-release.md` already says the surviving half** - *what is in disorder may be spent
the turn it is made and does not survive that turn's end*. The two said different things about the
same moment and now say one.
### P-504 - Waste is what could have been kept and was not, and one word is missing to say it

**to** sean · **status** open, **held** 2026-09-14 pending `P-509`, which makes the guard one comparison and needs no new word · **raised** 2026-09-14 · **kind** entailed · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**You asked for a way to detect waste so a player can be warned.** The expression language already
has three of the four pieces, and this adds the fourth.

## What is already there

```
count {metal}                how many match a description
free metal of {store metal}  one container's capacity for a kind, less what it holds
min(a, b)                    the lesser of two
```

**`free <kind> of x` is already the right idea and already promoted**, and it takes one container.

## What is missing, and it is one word

```
sum free metal of {store metal}     every store's free metal, added up
```

**`sum` takes a trait today** - *`sum <trait> of {…}`, that trait aggregated over all of them* - and
`free <kind> of` is an expression rather than a trait, so it cannot be the thing summed.

> **`sum` aggregates an expression over the things a description names**, not only a trait. Where
> that expression is a trait it reads as before.

## Then waste is a guard and needs nothing else

```
min(count {metal}, sum free metal of {store metal}) > 0
```

**What is loose, against what there is room for, whichever is smaller.** Greater than zero means
ending the turn loses metal that a container could have held - which is the warning you want, and a
user interface may refuse the turn on it or merely say so.

**It is one guard per resource**, and the guard is the same shape each time, because `metal` is the
only word in it that changes.

## Why this is entailed rather than invented

**`P-491` already made a recipe over a family one rule per member**, and this is the same move in an
expression: one form, one word varying. **Nothing new is being said about the game** - only that an
aggregate may aggregate the thing the language already computes.

## What it does not do

**It does not decide what the interface does about waste.** You said that is a user interface
concern and this stops where you stopped: the guard is expressible, so refusing the turn is
implementable, and the specification says nothing about whether it should be.

**And it does not sweep anything.** Nothing is stored without being asked - that is `P-505`.
### P-502 - A bin is a store, and what a thing holds is what it holds

**to** sean · **status** open, **held** 2026-09-14 pending `P-509`, which keeps *a bin is a store* and drops *what it holds is what it contains* · **raised** 2026-09-14 · **kind** invented · **shape** text · **asks** approval · **into** `spec/logistics.md` -> Containment

**Your transport definition is this proposal.** `1 * storage[fuel] capacity 2` and `1 *
storage[$resource] capacity 10` - two bins in one thing, each a container in its own right.

> A bin is a thing. **What holds a kind is a store for that kind**, and a thing with two bins holds
> two stores. A store's capacity is what its kind declares; what it holds is what it contains; and
> how much is free is the difference. **None of the three is a trait of the store**, because any two
> give the third and the third is then a second statement of the same fact.

> **Each distinct description and contents is its own entry.** Two things alike in every trait but
> holding different things are two entries, not one entry of two. **Where two entries share a
> description they sort by their contents, by this same rule, and by quantity where the contents'
> descriptions are equal too** - which is total, because a thing holds finitely many entries and
> each is shorter than what holds it.

## What this removes, which is the test of it

**Nothing gains a field when a resource is added.** `spec/invariants.md` requires that - *adding a
kind adds no field and no case* - and it is what rules out the obvious alternative of naming the
trio per kind, `free-metal` beside `free-energy`. **That alternative is not rejected on taste; it is
already forbidden.**

**And a unit's tank stops being its own idea.** `releases/first-release.md` has a *unit's tank*
listed as one of three sorts of capacity, with its own row and its own rules. Under this it is a
store inside a unit, and there are not three sorts of capacity - there is one.

## The uniformity you asked for, and what it costs

**Every bin is the same kind of thing, everywhere.** A territory's three metal stores, a transport's
cargo hold and its fuel tank are four stores, differing only in what they are for and how much they
take. There is no second vocabulary for *a capacity a thing has* beside *a container a thing holds*.

**What it over-specifies** is that a thing's own capacity now has a container to live in even where
only one bin will ever exist. `store` gains no complexity from this - it is the kind that was
already there - but a reader meets a store where they might have expected a number.

## A defect this item had until 2026-09-14, found by Sean asking about position

**`Entries are in the order their descriptions sort in`** - `spec/console.md`. This item says two
things alike in every trait but holding different things are **two entries**, and it did not say how
they sort. **They have the same description, so the order between them is undefined**, and *the same
state is always the same bytes* stops being true the moment a container holds two.

**The rule needs one more clause, and it is offered above rather than left implied:**

> **Where two entries share a description they sort by their contents, by this same rule.** The
> order is total because a thing holds finitely many entries and each is shorter than what holds it.

**It matters beyond tidiness**: a position is an index into that order, and an index into an order
that is not total names nothing. **Nothing in the item warned of this** - it was found by a question
about a user interface.

## One thing this lane checked and one it did not

**Checked**: `store` already carries a `resource` trait, so `storage[metal]` is `{store
resource:metal}` today and needs nothing invented. Your parameterised notation is expressible now.

**Not checked, and it is the risk**: `free` becomes derived under this, and `P-496` deleted a
constraint that named a derived trait because the Petri net materialises free capacity as tokens.
**The data model would derive what the net holds.** This lane has not measured whether that bites,
and says so rather than discovering it during a promotion.
