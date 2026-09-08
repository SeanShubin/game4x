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

### P-355 - What is `movable` derived from?

**to** sean - **status** open - **raised** 2026-09-08 - **kind** entailed, by your own three answers
- **asks** a decision - **into** `releases/first-release.md` -> *Traits*

**Your three answers settle everything but one cell.** A *Traits* row needs a **Values** cell and a
**stored or derived** cell. You said **derived**, and **a derived trait names its derivation** -
*metal in it* is *its binding plus the metal in its parts*; *control* is *a citizen of that player is
there*. **`movable` needs one too**, and that is the only thing stopping this becoming words.

**Two candidates, and they are the only two in the data.** Once `P-346` deletes *A move*, the columns
naming exactly `{ark, pioneer}` are:

| From        | It would read                           | So a thing moves because           |
| ----------- | --------------------------------------- | ---------------------------------- |
| **Fuel**    | derived: it has a tank                  | **it has somewhere to put energy** |
| **Crosses** | derived: there is a boundary it crosses | **there is somewhere it can go**   |

**They name the same two kinds today**, so nothing in the game changes either way. **They are
different reasons**, and the one you pick is what a later kind will be measured against - a thing
with a tank and nothing it can cross, or a thing that could cross but carries no energy.

**I am not choosing.** Both are already in the table, so neither is an invention, and nothing in what
you have said prefers one.

**What lands once you answer**: one *Traits* row, and two cells of `move` changed from `unit` to
`thing` plus `movable`. **The grammar allows it already** - *Kind is the kind or the family alone*,
`thing` is a family, and *Traits are the constraints on it*.

