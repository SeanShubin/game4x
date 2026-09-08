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

### P-347 - Founding does not require the pioneer to be there, which is why `move` never fires

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07, after you asked
whether `move` is unfired because a pioneer need not move to claim -
**asks** a decision - **into** `releases/first-release.md` -> Recipes, if founding should require it

**You asked, and the answer is yes.** I checked the scenarios rather than reasoning about them.

- `spread.4x:36` produces a pioneer in territory 1. `spread.4x:48` founds territory **3**. **Nothing
  between them moves anything**, and it passes
- `play.4x:132` produces a pioneer in territory 1. `play.4x:138` founds territory **2**. Same

**The scenario says otherwise in its own prose**, which is how this stayed invisible: *the second
pioneer crosses to territory 3*, and *the pioneer crosses into territory 2 and becomes it.* **Nothing
crosses.** A comment described the intended rule while the commands beneath it did something else -
`C-54`'s shape, one layer up: what a file says rather than what ran.

**So `move` is unfired because nothing needs it.** Founding consumes a pioneer without that pioneer
being in, or next to, the territory it founds.

**Territory 3 is adjacent to 1** - `play.4x:14` - so both foundings happen to be legal. **By luck
rather than by rule**: nothing checked.

**And it collides with a rule you promoted today.** `P-340` says a Pioneer is taken apart **when it
founds**, that *moving is not founding*, and that it **may cross ground its player already holds** -
all of which presume the pioneer travels to where it founds. **Nothing implements that.**

**One gap in the release, and it is the same class as `crosses`.** `deploy ark` states its places -
`$where`, and *the orbit above `$where`* - and **every other recipe leaves the *Where* column
blank.** The release never says what blank means. If it means `$where`, then `found by land` already
requires a pioneer in the territory being founded, and the code does not enforce it.

**Two readings, both entailed by the above rather than invented.**

- **Founding should require the pioneer to be there.** Then this is a defect, `P-340` is the rule it
  breaks, and **`move` begins firing on its own** - a scenario has to move a pioneer before it can
  found, which answers the original question without a case written to exercise anything
- **Founding should not.** Then `P-340` needs revisiting, and `move` has no use a player reaches

**I recommend the first**, because it is the only one that leaves `P-340` standing as you promoted
it.

### P-346 - Three statements fix what a move costs, and only one of them is a mechanism

**to** sean - **status** open - **cited** `0361e25` - **raised** 2026-09-07 - **kind** the quality lens's `Q-68` - **asks**
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

**Delete the column.** Two statements remain: the specification states the rule and the release implements it, which is the ordinary relationship between them. Nothing in the game changes. **What it costs is the ability to say a unit moves for more** - traded for tidiness, and not recoverable without a later proposal.

**Make the recipe read the column**, so the cell becomes `the unit's move` rather than `1`. **This is not a new form: the release already declares it.** *A quantity is a whole number. It is written in the recipe, read from a trait of one of the ingredients, or read from a trait of a named ingredient.* The second of those three is exactly this, and `upkeep` is it in use - consuming *the thing's upkeep* in food. One cell changes, the column starts
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
