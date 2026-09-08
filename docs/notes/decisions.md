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

### P-354 - Two questions in the movable trait, and only one of them changes the game

**to** sean - **status** open - **raised** 2026-09-07 - **kind** recovered, from your sentence -
**asks** a decision - **into** `releases/first-release.md` -> *Traits* and *Recipes*

**The release already tells structures from vehicles**, which is worth knowing before choosing: the
*Families* table says **`unit` is `ark, pioneer`**, and `move` consumes `1 unit`. Movability is
carried by family membership today.

**Your sentence contains two changes, and they are not equally consequential.**

**1. `movable` trait, or `unit` family.** Both name **ark and pioneer and nothing else** today, so
**no behaviour changes either way.** The difference is where it could go: a family is per kind, a
trait is per thing, so a trait admits two things of one kind differing in whether they move. Nothing
in the release needs that.

**2. *Same thing* moves, or a thing is consumed and another produced.** The table does the second:
`consume 1 unit ... $from`, `produce 1 unit ... $to`. **Your formulation says the same thing arrives
with one less energy**, which is not what those two rows say. A thing that must be named
individually carries an `id`, so **consume-and-produce destroys one and makes another**, and the
identity does not survive the move.

**That second one is the change**, and it is the one I would put your attention on. Whether an ark
that crosses is the same ark is a question about the game, not about notation.

**What I am not offering.** Wording for either, because a choice is open, and any account of how a
recipe could move a thing without consuming it would be me inventing a mechanism you have not asked
for.

**Do we know enough to choose how the data is modelled? Measured, not argued** -
[do the sets cross-cut?](2026-09-07-do-the-sets-cross-cut.md).

**Enough to eliminate, not enough to select.** Nine capability columns in *Units and structures*,
six distinct sets, 36 pairs, **five cross-cutting**. The clean one is *Force* against *Readies*:
they share ark, citizen and pioneer, **garrison has force and does not ready**, **extractor readies
and has no force.** Add *is built* and three kinds each carry a different pair - citizen not built,
garrison never readies, extractor has no force. **That is a lattice, so a hierarchy of kinds is
refuted by the release as it stands** rather than by a future it might have. Structural typing, a
trait a thing carries, and a parameter over kinds all express a lattice, and **nothing measured
separates them.**

**The surviving answer is already in use.** *Units and structures* **is** a capability matrix - a
column per capability, a blank meaning *does not have it*. So the question is narrower than the
paradigms make it sound: **whether a capability that already exists as a column should also be a
named trait.** That is question 1 above, and this is what it is really asking.

**And the duplication you want to refactor is measurable.** *Fuel*, *A move* and *Crosses* hold
exactly `ark, pioneer` - **three columns, one set** - and `P-346` takes it to two. *Binding* and
*Costs to produce* hold the same six. **Nine columns, six sets**, which is what any representation
would have to justify itself against.

**And it fits an ECS**, which sharpens question 1 rather than adding a third option. Entity,
component and system map onto thing, capability and recipe; **`crates/game-model/src/thing.rs:247`
already stores `traits` per thing rather than per kind**, so `movable` is the release's description
catching up with the model. **The system half does not fit**: an ECS system is code and
`spec/invariants.md` says a recipe is data. Same note.

**Your own example would not have decided it.** Moving is `ark, pioneer`; upkeep is `citizen`.
**Disjoint** - so that pair alone is consistent with a hierarchy.

**Nothing is blocked.** `P-346` stands as it is - deleting the `A move` column is about cost, and
neither question here touches it.

