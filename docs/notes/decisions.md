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

### P-346 - Three statements fix what a move costs, and only one of them is a mechanism

**to** sean - **status** open - **raised** 2026-09-07 - **kind** the quality lens's `Q-68` - **asks**
a decision - **into** `releases/first-release.md` -> *Units and structures* and *Recipes*, and
`spec/units.md`

**The same fact is stated three times.** The quality lens found it looking for what `move` becoming
an ordinary recipe left behind, and I checked all three.

| Where                                       | What it says                                                  | Does it work?                   |
| ------------------------------------------- | ------------------------------------------------------------- | ------------------------------- |
| `move`, its energy row                      | `consume` `1` `energy` from *that unit*                       | **yes** - this is the mechanism |
| *Units and structures*, the `A move` column | `1 fuel`, for an ark and for a pioneer                        | **no**                          |
| `spec/units.md`                             | *Moving burns a unit of it, and a unit with none cannot move* | it states the rule              |

**The column cannot work, and that is the finding.** The recipe consumes a literal `1`, so **a row
saying `2 fuel` would change nothing** - the unit would still spend one. A per-thing column beside a
rule that is no longer per-thing can only repeat what the recipe already fixes.

**Two ways, and I recommend the second.**

**Delete the column.** Two statements remain: the specification states the rule and the release
implements it, which is the ordinary relationship between them. Nothing in the game changes.

**Make the recipe read the column**, so the cell becomes `the unit's move` rather than `1`. **The
language already does this and `upkeep` is the precedent** - it consumes *the thing's upkeep* in
food, which is exactly a per-kind number read by a world recipe. One cell changes, the column starts
doing work, and **a unit that costs more to move becomes expressible** without another decision
later.

**What the second costs**: `spec/units.md` would have to widen too, because *burns **a unit** of it*
fixes the number in prose. Something like *moving burns fuel* leaves the amount to the recipe, which
is where you put cost when you removed the line from `spec/orbit.md`.

**Two things the lens checked so they are not swept up with this**, and I confirmed both:

- **`Crosses` is read by the recipe** - *joined to `$from` by an edge the unit crosses*. It is an
  ingredient's trait, not a second copy of a rule
- **`Fuel` is the tank's size** and is a different fact from the cost of one move

**Nothing here is urgent.** Every mobile unit costs one today, so both ways describe the same game;
the difference is only whether a later unit can cost more without a proposal.
