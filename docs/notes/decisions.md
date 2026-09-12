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

## Which kinds, and the running game answers it

**The scenario plays through and only places carry an `id`** - twelve territories and twelve orbits,
counted in `scenario/expected/play.4x`, and nothing else in the file has one. **Everything else is
counted instead**: `{citizen bearing:1 defending:1 laboring:1} -> 8`, `{extractor resource:food
working:1} -> 3`, `{store resource:metal} -> 2`.

**And the command language is what makes that work.** The scenario moves a unit like this:

```
{move unit:pioneer territory:2}
```

**A unit is named by its kind and a place by its `id`.** That is `P-356` - a field's value may name a
kind rather than a thing - and it means nothing in this release ever has to tell two units apart.

**So `place` is the answer**, and it is a family the game already declares:

```
{kind name:territory trait:id}
{kind name:orbit trait:id}
```

## The honest caveat, which is the reason this is still yours

**The evidence for units is a population of one.** The scenario has one pioneer and no Ark in its
final state, so *naming a unit by its kind was never ambiguous* is true and proves nothing about two.
**A count over nothing is the same failure with the sign flipped**, and this is the version of it
that matters here.

**What would decide it empirically is your own test**, from `P-445`: *if I never notice I need it, I
don't need it.* Two Arks in one orbit is the case that would notice, and this release cannot build
it - the scenario's second Ark never launches.

**What is not a caveat**: giving a kind an `id` is not free. `spec/logistics.md` - *there is never a
quantity of a thing with an `id`* - so a kind that carries one stops being counted and becomes a row
per thing in every dump. Eight citizens would be eight lines.

## What the new invariant settles, and what it does not

**It settles the shape and not the membership.** *A field that cannot be offered as a choice is the
data asking for a primitive... it is never left as a sentence.* So `a thing that must be named
individually` cannot stay in the *Traits* table's `Of` cell whatever you decide: it becomes kinds
declaring the trait, in words the game already has.

**The membership is still yours**, and this lane recommends **territory and orbit** - the smallest
answer the running game supports, revisable the first time you notice wanting two of something told
apart.

---

### P-457 - what a trait's own line says, which is the other thing `traits.4x` waits on

**to** sean - **status** open - **raised** 2026-09-12 - **kind** entailed, from `P-451` - **asks** a decision - **into** `spec/data/traits.4x`, and one sentence into `spec/console.md` -> The language

**`P-451` says a trait *says what it admits and whether it is stored*, and neither half has a written
form.** Filed by the code lane as `C-100`, which declined to invent the key names because that is
shape. This is `traits.4x`, whole, with the one open cell written `???`:

```
{trait admits:number kept:thing name:id}
{trait admits:number kept:thing name:moving}
{trait admits:number kept:thing name:laboring}
{trait admits:number kept:thing name:working}
{trait admits:number kept:thing name:bearing}
{trait admits:number kept:thing name:defending}
{trait admits:resource kept:thing name:resource}
{trait admits:number kept:kind name:strength}
{trait admits:number kept:kind name:fuel}
{trait admits:number kept:kind name:upkeep}
{trait admits:number kept:thing name:density}
{trait admits:number kept:thing name:total-capacity}
{trait admits:biome kept:thing name:biome}
{trait admits:number kept:thing name:nature}
{trait admits:place kept:thing name:from}
{trait admits:place kept:thing name:to}
{trait admits:number kept:thing name:keeps}
{trait admits:??? kept:nothing name:unpaid}
{trait admits:phase kept:thing name:phase}
{trait admits:??? kept:kind name:movable}
```

**Twenty lines, and the release's table has twenty-three.** `metal in it`, `control` and `surplus`
are the three missing: all derived, and none of the three named by any recipe row, which is the rule
you took last - a derived trait is declared when a recipe names it. **Entries follow the release's
table and the traits inside a line sort**, which is `P-452` and what `kinds.4x` already does.

## `kept` says where the value lives, and it has three values because the release does

`stored` on each thing, `of the kind` on the kind, `derived` nowhere at all - **fifteen, four and
one**. `kept:nothing` is a trait computed when it is read; the sentence that computes it stays in
prose, where rule 7 puts it.

**Two words are in play and neither is settled**: `admits` is the release's own verb - *where a trait
admits a closed set of values* - and `kept` is this lane's, chosen over the code lane's `held`
because in this game `held` is containment and a store holds metal. Say the word if you want
another.

## The question was seven cells and it is two, and `spec/turn.md` is what shrank it

**Five of the seven are not two-valued at all. They are counts, and the release's `0 or 1` is this
release's maxima showing through.** `spec/turn.md` -> *Order of operations*: **what a thing can do is
a count it carries as a trait** - *an action is named, and each kind declares **how many** of each
action a thing of it may take in a turn. A recipe names the action it spends, and firing it lowers
that count by one.*

**`moving`, `laboring`, `working`, `bearing` and `defending` are those counts**, and the release
says so three ways: each is guarded *at least 1*, each is lowered by one, and each is restored by
`refresh` **at its maximum** - six rows, checked. **A trait that has a maximum admits a number**, and
`0 or 1` is what you get when every maximum in this release is one.

**So they are `admits:number`, and that is read from the specification rather than chosen.** It also
means nothing is lost: a unit that one day gets two moves needs no change to `traits.4x`.

**What is genuinely two-valued is `unpaid` and `movable`** - one derived, one of the kind, neither a
count of anything.

## Which leaves two cells, and the new invariant decides the shape of the answer

**`flag` as a word of the notation is out, and the invariant you promoted this morning is what
removes it.** *When a primitive earns its place* gives two branches and `flag` fails both:

- **Does removing it multiply what has to be authored?** No. Two `value` lines per two-valued trait
  is **four lines**, and it grows by two for each one added. The invariant's own calibration is one
  rule with four optional parts becoming sixteen - **that is a multiplication and this is an
  addition**
- **Is the removal exact and a contortion?** No. `{value name:yes of:unpaid}` says exactly what is
  true and reads as what it is

**And the third bullet finishes it**: a list is the right length when every primitive is a thing
rather than a point on an axis that already exists. `admits` already has three points - a number, a
family, a trait whose values declare themselves - and `flag` is the third one with sharing.

**This lane recommended `flag` yesterday and the rule overrules it.** Worth saying plainly, because
it is the rule working rather than this lane changing its mind.

## So the two cells, and the choice is small

**A** - **they are numbers, like the five counts:**

```
{trait admits:number kept:nothing name:unpaid}
{trait admits:number kept:kind name:movable}
```

Then **every trait in the file admits a number, a family, or its own values**, and the words `yes`
and `no` never appear in the game's data at all. An Ark's line reads `movable:1`, matching
`{citizen defending:1}`, which `spec/console.md` already writes. **Nothing is two-valued anywhere**,
so the category stops existing.

**B** - **they are values, declared the way a biome is:**

```
{trait admits:unpaid kept:nothing name:unpaid}
{value name:no of:unpaid}
{value name:yes of:unpaid}
{trait admits:movable kept:kind name:movable}
{value name:no of:movable}
{value name:yes of:movable}
```

Six lines rather than two, and **the data says a thing that `A` leaves unsaid**: that `movable:7` is
not a value. The release's own word is `yes`, and the *Units and structures* table spells it that
way in two columns.

## What this lane would take, and it is `A`

**`A`, and the reason is that `B`'s advantage is smaller than it looks.** Under `A` nothing can say
`movable:7` either, because **what an editor offers is derived from what the game holds** - the
invariant's sixth bullet - and what the game holds for `movable` is what the kinds declare, which is
`0` and `1`. **The constraint is in the kinds' lines rather than in the trait's**, which is where
`P-451` put every other fact about which kinds carry what.

**And it removes a category rather than shrinking one.** After `A` there is no closed set of bare
words in the game except `biome` and `phase`, and both of those are genuinely sets of words with
names - `ocean`, `design` - rather than a two-valued flag wearing them.

## Two small things inside it, said rather than resolved

**What a self-declaring trait's `admits` says.** The block above writes `admits:biome` and
`admits:phase` - the trait naming itself. `admits:value` would say the same thing in one word for
both, and would not state the name twice. **This lane would take `admits:value`**; the block keeps
the longer form so you can see which it is.

**And `unpaid` is `kept:nothing` while being the only one.** `surplus` left with the row count this
morning, so exactly one derived trait is in the file. That is the rule working and not a smell, but
it means a check over `kept:nothing` runs over one case.

---

### P-459 - `refresh` reads a maximum that nothing declares

**to** sean - **status** open - **raised** 2026-09-12 - **kind** contradiction, found working `P-457` - **asks** a decision - **into** `releases/first-release.md` -> Units and structures, and `spec/console.md` -> The language

**`spec/turn.md` says a kind declares a number and the release declares a `yes`:**

```
spec/turn.md    "each kind declares HOW MANY of each action a thing of
                 it may take in a turn"

the release     | **citizen** | ... | Readies: yes |     one cell, three actions
                | **ark**     | ... | Readies: yes |     one cell, two actions
```

**A citizen has three actions** - `laboring`, `bearing`, `defending` - and one `Readies` cell for all
three. **`refresh` puts each of them *at its maximum*, six rows**, so the maximum is read by a
recipe and declared by nothing.

**It agrees today only because every maximum is one**, which is the shape `CLAUDE.md` warns about: a
right answer about a narrower question. Give a unit two moves and there is nowhere to write it.

## The three shapes

**A** - **the `Readies` cell holds the actions and their counts**, and the data file writes one line
per action:

```
| **citizen** | ... | laboring 1, bearing 1, defending 1 |
```

Says exactly what `spec/turn.md` says. **Costs a second name per action** in the data file, because a
kind's line already writes `laboring` to mean *this kind has a stored `laboring`* and cannot also
write `laboring:1` to mean its maximum.

**B** - **an action count is of the kind and stored at once**, and the kind's line gives the maximum:

```
{kind name:citizen bearing:1 defending:1 laboring:1}
```

One name, and the number is both the maximum and where a new thing starts. **Costs a fourth value of
`kept`** - today it is `thing`, `kind` or `nothing`, and this is a thing's value with the kind's
number behind it.

**C** - **the maximum is one, said once, and a number arrives when something needs two:**

```
Every action's maximum is one. Nothing in this release takes an action twice in a turn.
```

Costs nothing and leaves `refresh`'s *at its maximum* reading a constant. **This is your own test** -
`P-445`: *I want to decide this empirically. If I never notice I need it, I don't need it.*

## What this lane would take

**`C` now and `B` when it stops being true.** `B` is the shape that will be right - one name, and the
maximum where every other of-the-kind number already is - but it buys a fourth `kept` value for a
distinction no rule in this release can see. **`C` is not a deferral of the decision, it is the
answer while every number is one**, and the sentence it adds is what makes the constant honest rather
than implied.

**This blocks nothing.** `traits.4x` is unaffected either way: an action count admits a number under
all three, which is what `P-457` now says and reads from `spec/turn.md` rather than from the
release's `0 or 1`.
