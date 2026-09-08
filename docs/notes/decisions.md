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

### P-355 - `movable` is stored, or it is derived from the coincidence you just rejected

**to** sean - **status** open - **raised** 2026-09-08 - **rewritten** 2026-09-08, after you named it
- **asks** a decision - **into** `releases/first-release.md` -> *Traits*

**The name is settled: `movable`.** Your reason - *fuel in a tank is a coincidence of the recipe we
happen to have on first release* - is right, and it rules out what I recommended. **It also collides
with something you confirmed yesterday**, which is why this is back in front of you.

**A derived trait names its derivation**, and **the only thing in the release to derive movability
from is `fuel`.** There is no second mechanism yet. So on today's data:

|             |                                                                                 |
| ----------- | ------------------------------------------------------------------------------- |
| **derived** | can only read *it has a tank* - **the coincidence you just rejected**           |
| **stored**  | a thing simply carries `movable`, and each mechanism is a recipe's own business |

**`derived` and *not from fuel* cannot both hold today.** One of the two has to give.

**I recommend stored, and the reason derived was attractive does not apply here.** Derived was better
because **a stored trait fragments a fleet when its value varies** - and `movable` does not vary
among things that have it, so a million movable things stay one entry either way. **Storing it costs
nothing at scale.**

**It is also what your own reason asks for.** A stored `movable` says *this thing can move* without
saying how, so **a sail or a rail arrives as another recipe and nothing that says `movable` is
rewritten.** A derivation from `fuel` would have to be edited every time a mechanism is added.

**One cell still needs you either way: what `movable` holds.** `ready` is *yes or no*; `fuel` is a
number. **If movability is the capability and the mechanisms carry the numbers, yes-or-no is the
answer** - but you said traits are better graded than absolute, so I am asking rather than assuming.

