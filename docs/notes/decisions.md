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

### P-453 - `P-451` says a kind declares its traits and does not say whether the value rides along

**to** sean - **status** open - **raised** 2026-09-12 - **kind** entailed, from the code lane's question before building `traits.4x` - **asks** a decision - **into** `spec/console.md` -> The language, and `spec/data/`

**`P-451` is right about the relation and silent about the line.** *A kind declares which traits it
has* - and a citizen has laboring, bearing, defending, strength and upkeep, while **a key takes one
value**. So `{kind name:citizen trait:strength}` cannot be the whole of it, and the code lane stopped
rather than guessing.

**The traits split three ways, counted from the table**: **15 stored**, **4 of the kind** -
`strength`, `fuel`, `upkeep`, `movable` - and **4 derived**. **That split is what the two readings
disagree about.**

## A - one line per pair, and values live in `traits.4x`

```
kinds.4x                                traits.4x
{kind name:citizen trait:laboring}      {trait admits:number held:of-the-kind name:strength}
{kind name:citizen trait:strength}      {trait name:strength of:citizen value:1}
{kind name:ark trait:strength}          {trait name:strength of:ark value:2}
```

**Uniform: every trait a kind has is one line, whether it is stored or of the kind.** A citizen
appears on five lines in `kinds.4x`, and **a reader scanning for `citizen` finds five rather than
one**.

## B - of-the-kind values ride on the kind's line, stored traits are named

```
kinds.4x                                          traits.4x
{kind family:unit name:ark strength:2 fuel:0}     {trait admits:number held:of-the-kind name:strength}
{kind name:citizen strength:1 upkeep:1}           {trait admits:0-or-1 held:stored name:laboring}
{kind name:citizen trait:laboring}
{kind name:citizen trait:bearing}
```

**This is `3A` finished.** An of-the-kind trait's value **is** a fact about the kind, so it goes on
the kind's line the way `family` does - and *Units and structures* folds in exactly. **A stored trait
has no per-kind value to write**, so all it can say is that the kind has it.

**Two shapes, and the difference is real rather than cosmetic**: `strength:2` says what an ark's
strength is; `trait:laboring` says a citizen has one and each citizen's is its own.

## What each costs

**`A` is one shape and pays twice.** *Units and structures* does not fold - its eight rows of numbers
become `value:` lines in `traits.4x` - so `3A`, which you approved an hour ago, **stops being
available**. And a citizen is five lines rather than one.

**`B` is two shapes and pays once.** A reader of `kinds.4x` sees each kind's own facts on its own
line and its stored traits listed after. **The cost is that `trait:` and `strength:` are different
kinds of statement** in the same file.

## Where the rules point

**`P-446` favours `B`**: an of-the-kind trait and a stored one differ in *what the value is about*,
and one shape for both says they do not. **`P-428` favours `A`**, on the tie-break, if you judge the
difference small.

**This lane reads it as `B`**, because `3A` is already decided and `A` withdraws it - **and a
proposal that quietly unpicks one you approved an hour ago is the thing to say out loud rather than
let happen.**

