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

### P-467 - a garrison's metal comes from nowhere

**to** sean · **status** open · **raised** 2026-09-12 · **kind** contradiction, found deriving *Units and structures* from the recipes · **asks** a decision · **into** `releases/first-release.md` -> Units and structures, or Recipes

**A garrison is stated to cost 1 labor and 1 metal, and nothing charges it:**

```
Units and structures   | **garrison** | ... | Costs to produce: 1 labor, 1 metal | Binding: 1 |

the recipes            deploy ark     produce 1 garrison     consumes 1 ark
                       found by land  produce 1 garrison     consumes 1 pioneer
                       muster         require 1 garrison
```

**Those are the only three rows in the release that name a garrison.** No recipe consumes labor or
metal to make one, and **no recipe consumes a garrison at all** - it is produced by founding and
never destroyed.

## Why it is a contradiction rather than an unused number

**`spec/invariants.md` -> Everything is modelled: *nothing in the game appears or disappears without
a cause inside the model*.** A garrison arrives with `Binding: 1`, and `metal in it` is declared as
**its binding plus the metal in its parts** - so a garrison holds one metal, and **no line of any
recipe put it there.**

**It does not break *nothing comes back round with more*, and that is worth saying** - nothing
destroys a garrison, so the metal is never recovered and no cycle gains. **What is wrong is the
accounting, not the balance**: one metal exists in the world that was never drawn from the planet.

## The three ways

**A** - **founding charges it.** `found by land` and `deploy ark` each gain a `consume 1 metal` and a
`consume 1 labor`. **The cost becomes true** and founding becomes dearer by a metal and a labor.

**B** - **a garrison costs nothing and binds nothing.** Both cells empty, and a garrison is what a
founding gives you rather than a thing that is built. **Nothing else changes** - no recipe reads
either cell today.

**C** - **a garrison is built, and `build garrison` is a recipe.** The costs are already written,
`P-421` gave `put` the shape for placing one, and the capacity of 1 per territory is already
declared. **This is the largest of the three and the only one that makes the stated cost a rule
rather than deleting it.**

## What this lane would take

**`B`, and weakly.** It is the smallest, it makes the two files agree, and nothing in the release
reads either cell - but **it deletes a number rather than deciding what the number was for**, and
the reason a garrison was given a cost is not in any note this lane can find.

**`A` is the one that keeps the intent** if the intent was that holding ground costs something.
**`C` is the one that keeps it if a garrison was meant to be a choice** rather than a consequence of
founding.

**`P-466` does not depend on which.** That proposal is about three columns being the recipes said
twice; this row is the disagreement that proves it, whichever way it is settled.
### P-466 - three columns of *Units and structures* are the Recipes table said twice

**to** sean · **status** open · **raised** 2026-09-12 · **kind** simplification, found deriving each column from the recipes rather than reading it · **asks** a decision · **into** `releases/first-release.md` -> Units and structures

**Every cell of three columns is already in the Recipes table**, derived here rather than compared by
eye:

| Thing         | Costs to produce               | what a recipe charges          | Binding | metal charged |
| ------------- | ------------------------------ | ------------------------------ | ------- | ------------- |
| **extractor** | 1 labor, 1 metal               | 1 labor, 1 metal               | 1       | 1             |
| **yard**      | 1 labor, 15 metal              | 1 labor, 15 metal              | 15      | 15            |
| **store**     | 1 labor, 1 metal               | 1 labor, 1 metal               | 1       | 1             |
| **ark**       | 3 metal, 12 energy, 2 citizens | 3 metal, 12 energy, 2 citizens | 3       | 3             |
| **pioneer**   | 3 metal, 6 energy, 2 citizens  | 3 metal, 6 energy, 2 citizens  | 3       | 3             |
| **garrison**  | 1 labor, 1 metal               | **nothing**                    | 1       | **none**      |

**`Requires` is the same, and it is one cell**: the Ark's *a Yard* is `launch ark`'s
`require 1 yard`. One of one.

**So `Costs to produce` is the consume rows of the recipe that produces the thing, `Binding` is the
metal among them, and `Requires` is the require rows.** Five of six exactly, and the sixth is a
defect rather than an exception - `P-467`.

## Why this is more than tidying

**`P-458`: a rule the specification can state twice is one two readers can disagree about.** These
are stated twice and **have already disagreed** - the garrison row is the disagreement, and it went
unnoticed because reading the table and reading the recipes are two acts nobody does together.

**And rule 7 says which of the two is the source.** The recipes are the game's data; a column
restating them is a rendering, and **a rendering that is written rather than generated is what the
rule forbids**.

## The three ways

**A** - **the three columns come out of the table**, and a reader who wants a thing's cost reads the
recipe that makes it. *Units and structures* becomes **Thing, Strength, Fuel, Upkeep, Crosses,
Readies, Movable** - seven columns, every one of them of-the-kind data that no recipe carries.

**B** - **they stay and are generated**, from the recipes, with a check that the generated cells and
the table agree. **Costs a generator and keeps a convenience** - a reader sees a thing's cost without
looking it up.

**C** - **they stay hand-written and a check compares them to the recipes.** Cheapest, and **the
check is the whole of the value** - it is what would have caught the garrison.

## What this lane would take

**`A`.** `B` and `C` both keep a second statement of the same fact and pay something to keep the two
agreeing; `A` removes the second statement. **The convenience `B` buys is small and shrinking** -
`spec/data/` is where the game's data lives now, and a reader looking up a cost will be reading a
data file rather than this table within the release.

**What `A` costs is real and worth naming**: a person comparing two things' costs reads two recipes
instead of one row. **That is a rendering's job**, and `reports/` is where renderings go.

## What stays, and one of it needs a trait

**`Crosses` is not a rendering** - `orbit border` for an Ark and `border` for a pioneer, read by
`move`'s *joined to `$from` by an edge the unit crosses*. **It is of-the-kind data and no trait
declares it**, which is the same shape `P-461` fixed for `binding` and is the next proposal rather
than this one.

**`Strength`, `Fuel`, `Upkeep`, `Readies` and `Movable` are declared traits** and stay as they are.
*Nothing is undecided. Every question filed here has been answered.*
