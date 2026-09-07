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

### P-318 - `docs/layers.md` tells the code lane to default to a tie-break `spec/turn.md` forbids

**to** sean - **status** open - **raised** 2026-09-06 - **kind** contradiction - **asks** a decision
- **into** `spec/turn.md` -> Order of operations, or `docs/layers.md` -> the tie-break

**The research lens's `X-2`.** Both lines read verbatim, and they do not disagree about determinism.

`spec/turn.md`: *What settles them is a deterministic mechanic of the game, and therefore something a
person wrote* **and a player can change**.

`docs/layers.md`: *Where two events collide, the lower index wins... because the index is data, not a
schedule.* And then: **Default to the tie-break reading.**

**They agree that an index is deterministic and disagree about authorship.** An array index is
reproducible, and **nobody wrote it as a rule and no player can change it** - so it satisfies the
first half of `spec/turn.md`'s requirement and fails the second.

**Nothing is formally wrong, and that is what makes it worth your time.** `docs/layers.md` is
non-normative, so no rule is broken. But **it is the document the code lane reads for guidance and
it says *default to this***, each file is correct on its own terms, and nothing reports the
divergence.

**Two ways, and the choice is which document governs.**

- **`spec/turn.md` governs.** A tie-break is authored data a player can see and change, and
  `docs/layers.md`'s default is guidance that has to change
- **The scope is narrower than it reads.** `spec/turn.md` is about ties the player can observe, and
  an internal ordering inside one turn's resolution is not one - in which case `spec/turn.md` gains
  the scope it is missing

**Not urgent, and the lens said so** - it filed this as worth doing eventually. It is here rather
than held because it is a contradiction, and `CLAUDE.md` says those are filed the moment they are
found.
