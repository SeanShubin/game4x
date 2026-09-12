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

### P-470 - a kind's line gains its trait names, and a trait of every kind says so

**to** sean · **status** open · **cited** `24260a6` · **raised** 2026-09-12 · **kind** part answered, by you, 2026-09-12 · **shape** an instruction and text · **asks** a decision · **into** `spec/data/kinds.4x`, `spec/data/traits.4x`, and `spec/console.md` -> The language

**You took `C` for `keeps`, so a trait of every kind says so and no kind's line carries it:**

```
{trait admits:number kept:thing name:keeps of:thing}         traits.4x - one line changes
{kind bearing defending laboring name:citizen ...}           kinds.4x - no line carries keeps
```

**`P-451` says a trait *says nothing about which kinds carry it*, and this is that clause yielding.**
Three things make it yield rather than break:

- **`P-469` requires it.** *A fact is stated once* - and *every kind has `keeps`* under `A` is one
  fact stated **eighteen times**, counted. **There is nowhere else to say it once**: a family
  declares only its name, so `thing` cannot carry it
- **`spec/invariants.md` says which wins** - *where an invariant and a specific rule appear to
  conflict, the conflict is a defect in the specific rule*. `P-469` is an invariant and `P-451`'s
  clause is in `spec/console.md`
- **And it is an exception rather than a reversal.** The clause exists so a **family does not list
  its members**, which is `C-98` and `P-448`. **`of:thing` lists nothing; it says all of them.**
  **The exception fires exactly once** - one cell of twenty-four has `Of: thing`

## The sentence, into `spec/console.md` -> The language

**It goes into the paragraph that carries the clause it excepts**, which is offered whole so the
change is visible. Only the final clause is new.

> **A kind declares which traits it has, and a value declares which trait it is one of.** So a trait
> says what it admits and where its value lives, and says nothing about which kinds carry it; and a
> trait whose values are kinds names their family instead, because they are already declared. **The
> thing that belongs to something says so, and the something says only what is true of itself.**
> **A trait of every kind is the one exception, and says so with `of:thing`** - because there is no
> kind for it to belong to and no family that could hold it.

## Why `B` is out, and the reason is not the one this lane first gave

**This lane called `B` ambiguous - *indistinguishable from a kind that genuinely has none*. That was
wrong**, and you were right to push on it: every kind has `keeps`, so there is no kind to confuse it
with.

**`B`'s real defect is that the fact is stated nowhere.** `kinds.4x` would not carry it, `traits.4x`
says nothing about which kinds carry a trait, and **`spec/data/` would hold no record that `keeps` is
universal at all.** It survives only in the release's `Of` cell - the column the code lane's `C-105`
says may not be rendered - and in `age` and `spoil` naming `thing`, which is a recipe rather than a
declaration.

## What is still open, and this lane's lean has changed

**Does a kind name its derived traits?** `control`, `metal-in-it`, `surplus`, `unpaid`.

**Last night this lane leaned *not named* and now leans the other way, because your test for `keeps`
applies to them too.** If a fact must be stated once **and somewhere**, then *which kinds have
`control`* cannot be stated nowhere either:

```
named        {kind biome control family:place id name:territory nature}
not named    {kind biome family:place id name:territory nature}
             ...and no file in spec/data/ says a territory has `control`
```

**Two of the four are recoverable and two are not**, which is what this lane had not worked out:

| Derived trait | `Of`                | Recoverable from `kinds.4x` because                                                          |
| ------------- | ------------------- | -------------------------------------------------------------------------------------------- |
| `metal-in-it` | whatever is built   | **yes** - the same six kinds name `binding`, and it is *binding plus the metal in its parts* |
| `unpaid`      | a thing with upkeep | **yes** - the citizen names `upkeep`, and it is *its upkeep was not met*                     |
| `surplus`     | food                | **no** - nothing on food's line implies it                                                   |
| `control`     | a territory         | **no** - nothing on a territory's line implies it                                            |

**So the honest reading is that *not named* loses two facts and duplicates two**, and *named*
duplicates two and loses none. **This lane will not invent a third option that names two and not the
other two** - splitting a rule by which cases happen to be recoverable today is the kind of exception
that rots.

## What the lines look like either way

**Twenty-two lines in `kinds.4x` whichever you take** - four declaring kinds and eighteen - and the
difference is nine trait mentions, 43 against 34.

```
named        {kind bearing defending laboring name:citizen strength unpaid upkeep}
             {kind binding metal-in-it name:garrison strength}
             {kind family:resource name:food surplus}
             {kind biome control family:place id name:territory nature}

not named    {kind bearing defending laboring name:citizen strength upkeep}
             {kind binding name:garrison strength}
             {kind family:resource name:food}
             {kind biome family:place id name:territory nature}
```

**Five kinds carry no trait at all either way** - `metal`, `energy`, `labor`, `fertility`, `force` -
and that is a fact about them rather than a gap.

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
