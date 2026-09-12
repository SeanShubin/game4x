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

### P-470 - a kind's line gains its trait names, and two of twenty-four do not invert

**to** sean · **status** open · **raised** 2026-09-12 · **kind** entailed, from `P-462`'s form and the code lane's `C-104` · **asks** a decision · **into** `spec/data/kinds.4x`, and `spec/console.md` -> The language if either question needs a rule

**`P-462` promoted the form this evening and nothing uses it yet:**

```
{kind family:place name:territory}                            kinds.4x today
{kind biome control family:place id name:territory nature}    with its traits named
```

**`P-451` says a kind declares which traits it has, and `spec/data/kinds.4x` does not.** Until it
does, the release's *Of* column cannot be derived from anything - it is the one column of *Traits*
that `traits.4x` is **forbidden** to carry, because a trait *says nothing about which kinds carry
it*. **That is why this is the first of the three the code lane's `C-104` names**, before the four
tables can be rendered rather than written.

## Twenty-three of the twenty-four invert mechanically

**Each *Of* cell resolved to kinds, computed rather than read:**

```
citizen     bearing defending laboring strength unpaid upkeep
garrison    binding metal-in-it strength
extractor   binding metal-in-it resource working
yard        binding metal-in-it
store       binding metal-in-it resource
ark         binding defending fuel metal-in-it movable moving strength
pioneer     binding defending fuel metal-in-it movable moving strength
food        surplus
territory   biome control id nature
orbit       id
deposit     density total-capacity
adjacency   from to
game        phase
metal energy labor fertility force      (no trait)
```

**Five kinds carry no trait at all and that is fine** - a line naming none is still a declaration.

## The first question: `keeps` is of every kind

**Its *Of* cell is `thing`, and the recipes mean it.** `age` requires `thing` *keeps at least 1* and
`spoil` consumes `thing` *keeps 0* - **any thing, not food**. Only food is ever **made** with one.

**A** - **every kind's line names it**, eighteen times.

**B** - **no kind's line names it**, the way `thing` is the family every kind is in and **no line says
so kind by kind** - `P-448`'s precedent, and `P-469`'s *a fact is stated once*: naming it eighteen
times states one fact eighteen ways.

**`B` is what both rules point at and it has a cost neither anticipated**: a trait no kind names
would mean *of every kind* **by absence**, and a generator reading `kinds.4x` cannot tell that from a
trait nothing carries. **A family declares only its name**, so there is nowhere to hang it either.

**This lane has no recommendation.** `A` is ugly and readable; `B` is right and unreadable; and the
third way - **a trait says it is of every kind, on its own line in `traits.4x`** - puts back on the
trait a thing `P-451` took off it, which is a rule to change rather than a gap to fill.

## The second question: does a kind name its derived traits

**Four of the inverted names are derived** - `metal-in-it`, `control`, `surplus`, `unpaid` - and
**`Of` cannot be derived without them.**

**`spec/console.md` says a derived trait is never part of a description**, and a kind's declaration
**is** a description - `{kind name:citizen}` is a description whose kind is `kind`. **So the sentence
reads two ways** and neither is silly:

- **it is about a thing's description in a state**, where a derived trait would be recomputed, and
  says nothing about a declaration - so a kind names its derived traits and `Of` derives
- **it is about any description**, so a kind may not name them, and the *Of* cells for those four
  come from somewhere else or not at all

**This lane leans to the first**, because the sentence's reason - *nothing writes one* - is about a
value being stored, and a declaration stores no value. **But it is a reading of his words and not
mine to make**, which is `C-49`'s line.

## What is not in question

**The other twenty-two names**, and the form - `P-462` promoted it and the code lane has a reader
that round-trips it byte-identical. **Nothing here waits on the four tables being rendered**; that is
the third proposal of the three, and this is the first.
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
defect rather than an exception - `P-467`, which Sean has answered: a garrison costs nothing and
binds nothing.

**The garrison row above is the table as it stands today.** Once `P-467` lands it agrees like the
other five, **and the disagreement stays as this proposal's argument rather than as a live defect**
- it happened, which is what the case rests on.

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

**C** - ~~they stay hand-written and a check compares them to the recipes~~. **Withdrawn by
`P-469`, promoted 2026-09-12**: *a second form kept for a reader is generated, and a check says the
two agree.* **Hand-written and checked is no longer one of the ways** - a check would have caught
the garrison, and a generated column makes the garrison impossible.

## What this lane would take

**`P-469` landed and this item is two ways rather than three.** *A second form kept for a reader
is generated, and a check says the two agree* - so `C` is withdrawn, and **whichever of `A` and `B`
you take, the columns must say what they are**: *every other form says that it is one, and says what
it is a form of.*

**What is left for you is whether a reader wants the convenience**, which is a judgement about
readers rather than about facts, and is why this is still a decision. `P-469` leans: *removing the
second form is better than checking it.*

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
