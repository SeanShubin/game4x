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

### P-456 - which kinds carry an `id`, which is the one thing `traits.4x` cannot read

**to** sean - **status** open - **raised** 2026-09-12 - **kind** entailed, from the code lane declining to interpret a predicate - **asks** a decision - **into** `releases/first-release.md` -> Traits, or `spec/logistics.md` -> Containment

**`P-451` turned the *Traits* table's `Of` column onto the kinds**, and four of its cells are prose
predicates rather than kind lists. **Three of the four resolve from the release's own columns and one
does not.**

```
upkeep   "a thing with upkeep"    -> citizen        the Upkeep column has one value
unpaid   "a thing with upkeep"    -> citizen        the same predicate
movable  "whatever moves"         -> unit           the Movable column has ark and pioneer
id       "a thing that must be
          named individually"     -> ???            no column says which
```

**So `id` is the question, and it is the only one.** `spec/logistics.md` says *a thing may carry an
`id`, and one that does is unique; there is never a quantity of a thing with an `id`* - **which says
what an id does and never which kinds have one.**

**The scenario is evidence and not an answer.** `scenario/expected/play.4x` shows `id` on `territory`
and `orbit` and on nothing else - **but that is one scenario at one moment**, and an Ark had one
until it launched. **A kind that happens to carry no id today is not a kind that may not.**

## The two ways

**Name them in the *Traits* table**, replacing the predicate with the kinds the way every other row
has them:

```
| **id** | territory, orbit, ark, pioneer | a number, unique among things of its kind | stored |
```

**Or give the kinds the trait**, which is `P-451`'s direction and needs no table change at all -
`{kind name:territory trait:id}` - **and then the *Traits* table's `Of` cell for `id` is deleted
rather than rewritten**, because the kinds carry the answer.

**The second is what `P-451` already decided for every other trait**, so this is really the question
of **which kinds**, not of where to write it.

## Which kinds, and this lane will not guess

**`spec/logistics.md` gives the test** - *there is never a quantity of a thing with an `id`* - so a
kind carries one exactly when two of them must be told apart. **A territory and an orbit must.** An
ark and a pioneer must, because `move` names one and leaves the other where it was. **A citizen, a
store and a unit of food need not**, which is why the dump counts them.

**That reading is a reading**, and the code lane declined to make it for `C-49`'s reason. **This lane
declines for the same one**, and offers it as the shape of the answer rather than the answer.

