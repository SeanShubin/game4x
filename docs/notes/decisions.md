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

### P-492 - `with room for energy` names no trait a unit has

**to** sean · **status** open · **raised** 2026-09-13 · **kind** contradiction · **shape** rows · **asks** a decision · **into** `releases/first-release.md` -> Recipes, and possibly Units and structures

## The problem in one line

**`refuel` requires a unit *with room for energy*, and a pioneer has no trait meaning room.** Its
kind line is `{kind name:pioneer family:unit binding defending fuel metal-in-it movable moving
strength}` - **`fuel`, which is how big the bin is, and nothing saying how full it is.**

`P-476` gave `capacity`, `occupied` and `free` to `deposit` and to nothing else.

## Three ways out

- **`A` - drop the qualifier.** `require 1 unit | | $where`. **Containment already refuses to
  exceed a maximum**, so producing energy into a full bin cannot happen and the recipe simply does
  not fire. The cost: a player is offered a refuel that then does nothing, where a qualifier would
  have let `show` say why
- **`B` - give a unit the trio**, as a deposit has it. `free` becomes a trait of a unit too, the
  qualifier reads `free at least 1`, and the dump carries `capacity`, `occupied` and `free` on every
  unit entry
- **`C` - say it with `fuel` and containment.** The qualifier becomes *holding less energy than its
  fuel*, which names only traits that exist - but no qualifier in the table compares a thing's
  contents to a trait today, so it is a new shape

## What this lane would pick

**`A`**, on the grounds that the rule it leans on is already promoted and the other two add
something. **But `A` is the one that makes `show` worse**, and `spec/console.md` says `show` reports
*whether it is possible now, and when it is not, what is missing* - which `A` can still answer from
containment rather than from the recipe.

**The code lane encoded it as `free` to make the table parse at all**, and flagged that as a reading
rather than a reading-off. Whatever you choose, that line is the one that was provisional.
