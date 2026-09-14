# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-503 - The sweep is even-handed, which is why nothing has to merge

**to** sean · **status** open · **raised** 2026-09-14 · **kind** entailed · **shape** text · **asks** approval · **into** `spec/logistics.md` -> Containment

**This is the proposal your steer changed.** It was going to say that containers which cannot be
separated share one bin - three metal stores in a territory being one number of 15, because they
never move. **That is a second way of talking about storage**, and you said you would rather
over-specify than have two.

**So it says the opposite, and gets the same result.**

> **Nothing merges.** Every store is its own bin, and a sweep that has a choice divides what it
> sweeps as evenly as the capacities allow. **Stores alike in what they hold stay alike**, so
> containers nobody has distinguished appear as one entry of several rather than as several
> entries.

## Why the appearance you wanted falls out rather than being arranged

**Your fifteen metal into three stores of ten is five, five and five** - one entry, `{store
resource:metal} -> 3`, each holding five, and no story about which store. Not because the stores
were merged, but because **nothing distinguished them and an even sweep keeps it that way.**

**Twenty-five into the same three is ten, ten and five** - two entries. That is honest rather than
awkward: the capacities could not take it evenly, so the state says so.

## It is entailed rather than invented, and `spec/turn.md` is where

> Where two effects cannot both happen, they compete. **Competing effects are gathered and resolved
> together, so nothing gains an advantage by being considered first.**

**A first-fit sweep is precisely one thing gaining an advantage by being considered first.** Filling
store one to ten before store two sees anything is the order deciding the outcome, which that
sentence forbids. **Even division is not a preference here; it is the only rule that obeys what is
already promoted.**

## What it means for your two reasons

**You said you both do and do not care which store holds the metal.** This says you never have to
care and may always specify: the sweep leaves them alike unless you have made them unalike, and
`P-501` lets you name one when you have.

**And the transports are not an exception to it.** Two transports get an even sweep too - the reason
your example fills them three and seven is that you *said so*, with two commands, not that they obey
a different rule. **One rule, and the difference is only whether you spoke.**
### P-502 - A bin is a store, and what a thing holds is what it holds

**to** sean · **status** open · **raised** 2026-09-14 · **kind** invented · **shape** text · **asks** approval · **into** `spec/logistics.md` -> Containment

**Your transport definition is this proposal.** `1 * storage[fuel] capacity 2` and `1 *
storage[$resource] capacity 10` - two bins in one thing, each a container in its own right.

> A bin is a thing. **What holds a kind is a store for that kind**, and a thing with two bins holds
> two stores. A store's capacity is what its kind declares; what it holds is what it contains; and
> how much is free is the difference. **None of the three is a trait of the store**, because any two
> give the third and the third is then a second statement of the same fact.

> **Each distinct description and contents is its own entry.** Two things alike in every trait but
> holding different things are two entries, not one entry of two.

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

## One thing this lane checked and one it did not

**Checked**: `store` already carries a `resource` trait, so `storage[metal]` is `{store
resource:metal}` today and needs nothing invented. Your parameterised notation is expressible now.

**Not checked, and it is the risk**: `free` becomes derived under this, and `P-496` deleted a
constraint that named a derived trait because the Petri net materialises free capacity as tokens.
**The data model would derive what the net holds.** This lane has not measured whether that bites,
and says so rather than discovering it during a promotion.
### P-501 - A description names a thing wherever it appears, and leaves nothing out

**to** sean · **status** open · **raised** 2026-09-14 · **kind** invented · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**This is the one rule your scenario needs that nothing supplies.** Two transports, one holding 3
metal and one holding none, and a command that has to say which. Today a description states a thing;
nothing says what one **names**.

> A description names a thing as well as stating one. **A description used to name is exact**: it
> leaves out no trait and no content, and it names the things whose description and contents are
> exactly it. There is no second form that names some of them.

## Why exact rather than partial, and it is your rule rather than this lane's

**A partial description would be the second way of expressing a thing.** `{transport}` meaning *any
transport* reads identically to `{transport}` meaning *the one holding nothing*, and which it meant
would depend on where it stood. **One form that over-specifies beats two forms that each
under-specify**, which is what you asked for in as many words.

The cost is real and worth stating: **naming the empty transport means writing that it holds
nothing**, and naming a full one means writing its contents. You say more than you care about. **In
exchange there is one thing to learn and one thing to check.**

## What it leans on, already promoted

`spec/console.md` already says a description leaves nothing out - *no trait of the thing may be left
out* - and already allows a description as an argument, since **a value may be another command in
the same form**. This says the first of those holds when the description is doing the naming, which
is the case neither sentence covers.

## What it does not decide

**Nothing about matching several things at once.** If a command should ever act on every transport
holding metal, that is a second idea and this rule does not smuggle it in - it would arrive as a
description that is explicitly a pattern, and it would say so.
