# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-476 - nothing outside the debug view says which of a set is held

**to** sean · **status** open · **cited** `9baf0a2`, `1bbf7f7` · **raised** 2026-09-12 · **kind** contradiction, from your rule that only the debug report knows what is derived · **shape** an instruction · **asks** a decision · **into** `spec/console.md` -> The language, `spec/logistics.md` -> Containment, `spec/data/traits.4x`, and `releases/first-release.md` -> Traits

**Four places say which traits are stored.** All four are outside the debug view.

```
spec/console.md:58    "and a derived trait is never part of one"
spec/console.md:89    "`nothing` where it is derived and nothing carries it"
spec/logistics.md:18  "What is stored is the room left ... Nothing records the total"
first-release.md:113  column heading: "Stored or derived"
```

## The distinction that clears it, and it keeps the model whole

**A definition is logical and stays.** *`free` is capacity less what it holds* is a fact about the
game, and deleting it would delete the model. **What may not be said is which of an interdependent
set is the one actually held** - that is the layout, and `spec/invariants.md` already gives it one
reader.

So `binding` keeps its definition - *the metal the recipe that makes it consumes* - and loses
`kept:nothing`, which claimed nothing carries it.

## The decision

**Three names, one of which is on disk today.** A deposit in territory 1 holds three food extractors
on ground with capacity for three.

```
A   {deposit density:4 free:0 resource:food}
B   {deposit capacity:3 density:4 free:0 occupied:3 resource:food}
```

**`A` is today plus the rename.** One of the three is a trait and the other two are not, so every
dump says which one the model keeps. **That is the leak, in the one file a player reads most.**

**`B` makes all three traits.** No view can tell which is held, because all three are always there,
and the debug view is the only place the answer exists. It is your *at maximum we could show all of
them*, and it costs two more fields on every deposit entry - **thirty-four entries in this
release**.

There is no middle: naming any proper subset is a statement about the layout.

## What follows either way

- **`spec/console.md` loses *and a derived trait is never part of one***. The clause before it - *a
  trait of its kind is not part of one* - then decides a description's contents by itself, and it
  is logical. **The two rules select the same traits** once `kept:nothing` is gone, so nothing else
  moves.
- **`kept` loses `nothing`** and says only where a value belongs: **`thing`** where each thing of a
  kind carries it, **`kind`** where the kind carries it once.
- **The release's *Stored or derived* column becomes *Belongs to*** - `each thing` or `the kind` -
  and each derived trait's definition moves into its **Values** cell, where it is a definition
  rather than a storage claim.

## The five reclassifications, which are this lane's reading and want your eye

| Trait           | Today          | Becomes      | Because                                                   |
| --------------- | -------------- | ------------ | --------------------------------------------------------- |
| **binding**     | `kept:nothing` | `kept:kind`  | the recipe is the kind's, so every one is the same        |
| **metal-in-it** | `kept:nothing` | `kept:kind`  | its binding plus its parts', and the parts are the kind's |
| **surplus**     | `kept:nothing` | `kept:kind`  | declared on `food`, which is a kind                       |
| **control**     | `kept:nothing` | `kept:thing` | each territory has its own                                |
| **unpaid**      | `kept:nothing` | `kept:thing` | each citizen has its own                                  |

**`binding` and `metal-in-it` moving to `kind` is what keeps every dump the length it is today** -
they become traits of the kind, which the first clause already excludes.

## What this withdraws, and it is an hour old

**`P-474`'s sentence *Nothing records the total* goes**, and `P-475` renames it on the way. What
replaces it keeps the guarantee and drops the naming:

```
Any two give the third, so only two are ever held and nothing can disagree with anything.
```

**That is weaker on purpose.** *Only two are held* is what makes disagreement impossible; *which
two* was never needed for it, and was the leak.

## What this does not touch

**`spec/invariants.md`: *A trait may be derived rather than stored, computed from what is there.
Nothing can leave a derived trait wrong, because nothing writes one.*** It names the class without
naming a member, so it leaks nothing - but it is a promise about the layout sitting in the
invariants, and you may want it moved beside the debug view. **Say so and it becomes a second
proposal rather than a quiet edit here.**
*Nothing is undecided. Every question filed here has been answered.*
