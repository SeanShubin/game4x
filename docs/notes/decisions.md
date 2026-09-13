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

### P-477 - one sentence in the invariants still promises something about the layout

**to** sean · **status** open · **raised** 2026-09-12 · **kind** entailed, from `P-476` removing its three siblings · **shape** text · **asks** a decision · **into** `spec/invariants.md`

**`P-476` removed three statements about what is held and left a fourth**, which I flagged in that
proposal and you did not rule on. It is now the only place in `spec/` outside the debug rule that
talks about storage:

```
spec/invariants.md:157
- A trait may be derived rather than stored, computed from what is there. Nothing can leave a
  derived trait wrong, because nothing writes one
```

**It leaks nothing** - it names the class without naming a member, so no view becomes able to tell
which trait is which. **What it does is promise a property of the layout from the invariants**,
which is the one file that should not need to know there is one.

## What it is really saying, and where that already lives

**The guarantee is real and is already had another way.** `spec/invariants.md` -> *The data is a
normalized relational model* says *one thing knows how it is held, and everything else asks it* -
so nothing outside that one thing can write a derived value, because nothing outside it writes at
all. **The sentence restates a consequence of the rule four bullets above it**, which is the test
`CLAUDE.md` gives for whether a consequence belongs in the spec: *another rule leans on it*, and
none does.

## The words

> - A fact may be stated once and every other form of it computed. **Nothing can leave a computed
>   form wrong, because nothing writes one** - what would have to be written is the one form, and
>   one thing holds it

**Or delete the bullet outright**, which is the shorter specification and loses nothing this lane
can find a reader for. **Say which** - the offered words are the conservative half, and deleting is
the half `A fact is stated once` would prefer.

## What this does not touch

`spec/invariants.md` -> *A fact is stated once* and *The data is a normalized relational model* are
both untouched. This is one bullet, in the section above them.
*Nothing is open. Everything filed has been decided.*
