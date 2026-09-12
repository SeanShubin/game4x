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

**to** sean - **status** open - **raised** 2026-09-12 - **kind** contradiction, found working `P-457` - **asks** a decision - **into** `releases/first-release.md` -> Units and structures, and `spec/console.md` -> The language

**`spec/turn.md` says a kind declares a number and the release declares a `yes`:**

```
spec/turn.md    "each kind declares HOW MANY of each action a thing of
                 it may take in a turn"

the release     | **citizen** | ... | Readies: yes |     one cell, three actions
                | **ark**     | ... | Readies: yes |     one cell, two actions
```

**A citizen has three actions** - `laboring`, `bearing`, `defending` - and one `Readies` cell for all
three. **`refresh` puts each of them *at its maximum*, six rows**, so the maximum is read by a
recipe and declared by nothing.

**It agrees today only because every maximum is one**, which is the shape `CLAUDE.md` warns about: a
right answer about a narrower question. Give a unit two moves and there is nowhere to write it.

## The three shapes

**A** - **the `Readies` cell holds the actions and their counts**, and the data file writes one line
per action:

```
| **citizen** | ... | laboring 1, bearing 1, defending 1 |
```

Says exactly what `spec/turn.md` says. **Costs a second name per action** in the data file, because a
kind's line already writes `laboring` to mean *this kind has a stored `laboring`* and cannot also
write `laboring:1` to mean its maximum.

**B** - **an action count is of the kind and stored at once**, and the kind's line gives the maximum:

```
{kind name:citizen bearing:1 defending:1 laboring:1}
```

One name, and the number is both the maximum and where a new thing starts. **Costs a fourth value of
`kept`** - today it is `thing`, `kind` or `nothing`, and this is a thing's value with the kind's
number behind it.

**C** - **the maximum is one, said once, and a number arrives when something needs two:**

```
Every action's maximum is one. Nothing in this release takes an action twice in a turn.
```

Costs nothing and leaves `refresh`'s *at its maximum* reading a constant. **This is your own test** -
`P-445`: *I want to decide this empirically. If I never notice I need it, I don't need it.*

## What this lane would take

**`C` now and `B` when it stops being true.** `B` is the shape that will be right - one name, and the
maximum where every other of-the-kind number already is - but it buys a fourth `kept` value for a
distinction no rule in this release can see. **`C` is not a deferral of the decision, it is the
answer while every number is one**, and the sentence it adds is what makes the constant honest rather
than implied.

**This blocks nothing.** `traits.4x` is unaffected either way: an action count admits a number under
all three, which is what `P-457` now says and reads from `spec/turn.md` rather than from the
release's `0 or 1`.
