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

### P-491 - `refuel` has no command, and the rule that would give it one does not cover `move` either

**to** sean · **status** open · **raised** 2026-09-13 · **kind** contradiction · **shape** text · **asks** a decision · **into** `spec/console.md` -> Commands

## What a player types to refuel, which is the whole of it

**Nothing can fire `refuel` today**, and the three candidate commands are:

```
A   {refuel unit:pioneer where:1}     you name which unit
B   {refuel unit:pioneer where:1}     the same, but `move` and `refuel` both gain a `$unit` row
C   {refuel where:1}                  you name the territory and the game picks the unit
```

**`A` and `B` type the same and differ in whether two rows change. `C` takes the choice away.**

## The decision

**What lets a command name an ingredient?** Three readings, and they give `refuel` different
commands:

- **`A` - a recipe naming a family leaves the kind open.** `move` and `refuel` both require *1
  unit*, which is a family, so the command picks which kind: `{refuel unit:pioneer where:1}`. **This
  explains `move` as it already is and needs no row to change**
- **`B` - only a `$` opens an ingredient**, and `move`'s row should say `$unit`. Then `refuel`'s
  does too, and both rows change
- **`C` - the command names the territory and the game chooses the unit.** `{refuel where:1}`, and
  where two units have room the rule picks. **You called refuelling a distributive decision** - two
  pioneers and two energy is one each or both in one - which `C` takes away

## Why this is a decision and not a typo

**The rule that should say which is `A` or `B` does not explain `move`, which already works.**
`spec/console.md`:

```
A command names a recipe and binds what that recipe leaves open: every place it leaves open,
and any ingredient or trait value it names with a `$`.
```

**`{move unit:pioneer from:1 to:2}` has been binding a unit for weeks** and `move`'s row is
`require 1 unit | moving at least 1 | $from` - **no `$unit` anywhere.** So either a family already
opens the kind, or every recipe with an open ingredient is missing a `$`.

**`P-485` landed `refuel` and `P-489` neither caused nor fixes this.** The gate is red on
`the_scenario_fires_every_player_recipe_the_release_declares` - eleven declared, ten fired - which
is the check working.

## What this lane would pick and why it is yours

**`A`.** It is the only reading under which the notation already works, and it keeps the allocation
a player's choice - `{refuel unit:pioneer where:1 repeat:2}` puts two energy in one pioneer.

**But `A` and `B` differ about a sentence you promoted**, not about `refuel`, and `B` means every
recipe that leaves an ingredient open is missing a `$` today. That is a bigger claim than this item
can settle on its own.
*Nothing is open. Everything filed has been decided.*
