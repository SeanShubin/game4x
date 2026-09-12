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

### P-456 - which kinds carry an `id`, which is the one thing `traits.4x` cannot read

**to** sean - **status** open - **cited** `036a305` - **raised** 2026-09-12 - **kind** entailed, from the code lane declining to interpret a predicate - **asks** a decision - **into** `releases/first-release.md` -> Traits, or `spec/logistics.md` -> Containment

**`P-451` turned the *Traits* table's `Of` column onto the kinds**, and four of its cells are prose
predicates rather than kind lists. **Three of the four resolve from the release's own columns and one
does not.**

```
upkeep   "a thing with upkeep"    -> citizen        the Upkeep column has one value
unpaid   "a thing with upkeep"    -> citizen        the same predicate
movable  "whatever moves"         -> unit           the Movable column has ark and pioneer
id       "a thing that must be
          named individually"     -> ???            no column says which
```

**So `id` is the question, and it is the only one.** `spec/logistics.md` says *a thing may carry an
`id`, and one that does is unique; there is never a quantity of a thing with an `id`* - **which says
what an id does and never which kinds have one.**

**The scenario is evidence and not an answer.** `scenario/expected/play.4x` shows `id` on `territory`
and `orbit` and on nothing else - **but that is one scenario at one moment**, and an Ark had one
until it launched. **A kind that happens to carry no id today is not a kind that may not.**

## The two ways

**Name them in the *Traits* table**, replacing the predicate with the kinds the way every other row
has them:

```
| **id** | territory, orbit, ark, pioneer | a number, unique among things of its kind | stored |
```

**Or give the kinds the trait**, which is `P-451`'s direction and needs no table change at all -
`{kind name:territory trait:id}` - **and then the *Traits* table's `Of` cell for `id` is deleted
rather than rewritten**, because the kinds carry the answer.

**The second is what `P-451` already decided for every other trait**, so this is really the question
of **which kinds**, not of where to write it.

## Which kinds, and this lane will not guess

**`spec/logistics.md` gives the test** - *there is never a quantity of a thing with an `id`* - so a
kind carries one exactly when two of them must be told apart. **A territory and an orbit must.** An
ark and a pioneer must, because `move` names one and leaves the other where it was. **A citizen, a
store and a unit of food need not**, which is why the dump counts them.

**That reading is a reading**, and the code lane declined to make it for `C-49`'s reason. **This lane
declines for the same one**, and offers it as the shape of the answer rather than the answer.


---

### P-457 - what a trait's own line says, which is the other thing `traits.4x` waits on

**to** sean - **status** open - **raised** 2026-09-12 - **kind** entailed, from `P-451` - **asks** a decision - **into** `spec/data/traits.4x`, and one sentence into `spec/console.md` -> The language

**`P-451` says a trait *says what it admits and whether it is stored*, and neither half has a written
form.** Filed by the code lane as `C-100`, which declined to invent the key names because that is
shape. This is `traits.4x`, whole, with the one open cell written `???`:

```
{trait name:bearing admits:??? kept:thing}
{trait name:biome admits:biome kept:thing}
{trait name:defending admits:??? kept:thing}
{trait name:density admits:number kept:thing}
{trait name:from admits:place kept:thing}
{trait name:fuel admits:number kept:kind}
{trait name:id admits:number kept:thing}
{trait name:keeps admits:number kept:thing}
{trait name:laboring admits:??? kept:thing}
{trait name:movable admits:??? kept:kind}
{trait name:moving admits:??? kept:thing}
{trait name:nature admits:number kept:thing}
{trait name:phase admits:phase kept:thing}
{trait name:resource admits:resource kept:thing}
{trait name:strength admits:number kept:kind}
{trait name:surplus admits:??? kept:nothing}
{trait name:to admits:place kept:thing}
{trait name:total-capacity admits:number kept:thing}
{trait name:unpaid admits:??? kept:nothing}
{trait name:upkeep admits:number kept:kind}
{trait name:working admits:??? kept:thing}
```

**Twenty-one lines, and the release's table has twenty-three.** `metal in it` and `control` are the
two missing: both derived, and neither named by any recipe, which is the rule you took last - a
derived trait is declared when a recipe names it. **`id` is here whatever `P-456` decides**, because
which kinds carry it is on their lines and not on this one.

## `kept` says where the value lives, and it has three values because the release does

`stored` on each thing, `of the kind` on the kind, `derived` nowhere at all - **fifteen, four and
two**. `kept:nothing` is a trait computed when it is read; the sentence that computes it stays in
prose, where rule 7 puts it.

**Two words are in play and neither is settled**: `admits` is the release's own verb - *where a trait
admits a closed set of values* - and `kept` is this lane's, chosen over the code lane's `held`
because in this game `held` is containment and a store holds metal. Say the word if you want
another.

## The question: eight traits admit two values, and the release writes the set two ways

`0 or 1` five times - `moving`, `laboring`, `working`, `bearing`, `defending` - and `yes or no`
three - `surplus`, `unpaid`, `movable`. Nothing distinguishes them; they are the same set spelled
twice. **The other thirteen lines are settled and none of the three options touches them.**

**A** - **`flag` is a word of the notation, beside `number`:**

```
{trait name:defending admits:flag kept:thing}
{trait name:movable admits:flag kept:kind}
```

One new word. `yes or no` becomes `0 or 1`, so an Ark's line reads `movable:1`, matching
`{citizen defending:1}`, which `spec/console.md` already writes. **The two-valued set is in the data**,
so the editor that offers you a value for `defending` can refuse `7`.

**B** - **they are numbers, and being two-valued is a rule in prose:**

```
{trait name:defending admits:number kept:thing}
{trait name:movable admits:number kept:kind}
```

Nothing is added to the notation, and sixteen of the twenty-one lines then say `number`. **The file
stops telling a flag from a strength**, so nothing an editor reads says `defending:7` is wrong.

**C** - **the values declare themselves, as a biome's do:**

```
{trait name:defending admits:defending kept:thing}
{value name:0 of:defending}
{value name:1 of:defending}
```

Uniform with `biome` and `phase`, and adds no word. **Sixteen extra lines, two per flag**, saying the
same two words eight times - and two more for every flag the game ever grows.

## What this lane would take, and the reason

**A.** *Least expressive yet complete* is the test you set, and its form is that **a primitive earns
its place when removing it moves the combinatorial explosion from the generated space into the
authored space.** Removing `flag` gives C, which authors two lines per flag forever, or B, which
authors nothing and loses the constraint. **A authors one word, once.**

**The shared set is what makes it worth a word at all.** Were it one trait, C would be two lines and
the better answer; it is eight, and the release already spells the same set two ways because nothing
named it.
