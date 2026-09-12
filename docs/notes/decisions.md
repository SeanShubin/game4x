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

### P-459 - `refresh` reads a maximum that nothing declares

**to** sean · **status** open · **raised** 2026-09-12 · **kind** contradiction, found working `P-457` · **asks** a decision · **into** `releases/first-release.md` -> Units and structures, and `spec/console.md` -> The language

**Your reading is relevant, and it is already the rule - in your own words.** `spec/invariants.md`
-> *Nothing comes back round with more*:

> **Anything that exhausts draws on time for a turn**: it spends a count it carries, and only the
> turn's end restores that count, the way an extractor draws material out of the planet and is spent
> doing it. So what a thing can do is gathered rather than made, and **what bounds the gathering is
> the number of things that exhaust**.

**Time is the endless well and the count is the bounded pump.** `refresh` is a named source draw, so
it takes nothing away from *nothing comes back round with more* - exactly as you say, and the
specification already says it.

## And it rules out `C`, which is what this lane recommended

**The same section ends with the sentence that decides this:**

> **Whether this holds is decided mechanically, from the rules alone**, and stays so however many
> rules there are.

**`C` was *the maximum is one, said once*.** Under `C`, the one invariant the game is built around
is checked by a program reading a number **the rules do not contain** - it is in the checker as a
constant, which is where the code lane found it today and named it `UNDECLARED_MAXIMUM`. **A
boundedness argument that runs on a literal is not decided from the rules alone.**

**So your lean is right and this lane's recommendation was wrong.** The maximum has to be declared,
because the check that reads it is the check the invariant promises.

## The obvious way to avoid `B`'s cost is already closed, and worth knowing

**If a count were a quantity a thing holds rather than a trait, its maximum would be a capacity** -
the same relation that says a territory holds at most one garrison and a store holds ten. No new
`kept` value, and one relation instead of two mechanisms.

**That is `P-399`, and `P-411` undid it.** The reason was `C-90`, filed by the code lane: **as a held
kind, two citizens differing only in readiness had the same description**, and the map form could
not tell them apart. `spec/turn.md` now says the conclusion - *what a thing can do is a count it
carries as a trait, rather than something it contains*.

**So the trait is right and `B` is the shape.** What is left is one question inside it.

## The question inside `B`: how a kind's line says a maximum

**`P-454` already gives a kind's line one meaning for a number.** *A trait of the kind is written
with its value and a stored one with its name* - so `{kind name:citizen bearing:1}` reads today as
*`bearing` is of the kind, and its value is 1*. **But `bearing` is stored**: each citizen has its
own. The same line has to say two things.

**B1** - **a fourth value of `kept`:**

```
{trait admits:number kept:refilled name:bearing}
{kind name:citizen bearing:1 defending:1 laboring:1}
```

`kept:refilled` means *each thing carries the value and the kind carries its maximum*. **Costs a
fourth word** where `P-457` has just settled three, and every reader of `kept` gains a case.

**B2** - **`P-454`'s rule gains a case, and `kept` stays three:**

```
{trait admits:number kept:thing name:bearing}
{kind name:citizen bearing:1 defending:1 laboring:1}
```

**A number on a kind's line for a `kept:thing` trait is its maximum** - where a new thing starts and
the most it can hold. `kept` is still `thing`, `kind`, `nothing`. **Costs one sentence in
`spec/console.md`** and nothing in `traits.4x`.

## What this lane would take

**`B2`.** `kept` says where the value lives and that answer has not changed - it lives on the thing.
What the kind's line adds is a **bound**, not a second home, and `P-458` is the test: a list is the
right length when every primitive is a thing rather than a point on an axis that already exists.
**`refilled` is `thing` with a bound**, so it is not a fourth place.

**And `B2` costs the release one column change rather than a rewrite.** *Readies* says `yes` for
four kinds; it becomes the count per action, which is where a citizen's three actions stop sharing
one cell. The numbers are all `1` today, so nothing in the game moves.
