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

### P-355 - Does movability need a name of its own, or does `fuel` already say it?

**to** sean - **status** open - **raised** 2026-09-08 - **rewritten** 2026-09-08, because the first
phrasing was wrong - **asks** a decision - **into** `releases/first-release.md` -> *Traits* and
*Recipes*

**You chose *select by trait*. The question left is which trait.**

|                   |                                                                                 |
| ----------------- | ------------------------------------------------------------------------------- |
| **A new one**     | `movable`, derived from having a tank. `move` consumes `1 thing movable`        |
| **The one there** | **none added.** `move` consumes `1 thing fuel` - *a thing with a tank can move* |

**`fuel` already means what `movable` would mean.** Its Values cell is *how much energy its tank
holds*, and a thing with a tank is a thing that carries what a move spends. **A `movable` derived
from having a tank is a second name for having a tank.**

**I recommend the second, on your own test**: same strategic depth, one fewer name.

**And I withdraw the way I put this yesterday.** I offered *derived from `Fuel`* against *derived
from `Crosses`* as though the two were rival sources for one fact. **They are not duplicates and I
had that wrong.**

- **`fuel` says whether** a thing can move at all - it holds what a move spends
- **`Crosses` says where** it may go, which is what `P-349` promoted: *which ones it crosses is a
  fact about that unit*

**They name the same two kinds today by coincidence of the current data**, not because they say the
same thing. A later kind with a tank and nothing it can cross, or something that could cross but
carries no energy, separates them - and **deriving movability from `Crosses` would answer *whether*
with a fact about *where*.**

**What lands once you answer.** If `fuel`: two cells of `move` change from `unit` to `thing` plus
`fuel`, and **no row is added**. If `movable`: a *Traits* row as well, naming what it derives from.

**One thing neither answer settles, so it is not hiding in here.** The *Traits* table's *Carried by*
column says `fuel` is carried by *a unit* - a family. **Selecting by trait while the trait's carrier
is named by a family leaves the family doing the work**, so that column has to become descriptive of
the matrix rather than the source of it. It is a consequence of the choice you already made in
`P-354`, and I will file it as its own item rather than fold it in here.

