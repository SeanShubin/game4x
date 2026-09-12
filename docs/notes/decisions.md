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

### P-450 - three questions `traits.4x` waits on, each written out

**to** sean - **status** open - **raised** 2026-09-12 - **rewritten** 2026-09-12, examples first - **kind** entailed, from the code lane's `C-98` and `C-97` - **asks** a decision - **into** `spec/console.md` -> The language, and `spec/data/`

**`P-448` answered one of `C-98`'s three cells** - a kind declares its family, so `Members` is gone
and `families.4x` is fully specified. **Three questions are left**, and the first two are what
`traits.4x` stops on.

# 1. Where a trait's kinds are written

**Three traits are *of* more than one thing**, counted from the *Traits* table rather than recalled:
`defending` is *a citizen or a unit*, `resource` is *an extractor or a store*, and **`strength` is
citizen, garrison, ark, pioneer**.

**`strength` is the sharpest, and the family vocabulary already shortens it**: its four are citizen,
garrison and the family **`unit`**, so three lines rather than four whichever way this goes.

**The other traits name one thing or a prose phrase** - *a thing with upkeep*, *whatever is built* -
and those phrases stay prose under rule 7 rather than becoming lines.


## 1A - the trait declares its kinds, in `traits.4x`

```
{trait name:strength of:citizen admits:number held:of-the-kind}
{trait name:strength of:garrison}
{trait name:strength of:unit}
{trait name:laboring of:citizen admits:0-or-1 held:stored}
```

## 1B - the kind declares its traits, in `kinds.4x`

```
{kind name:citizen family:thing trait:strength}
{kind name:citizen trait:laboring}
{kind name:garrison trait:strength}
{kind name:unit trait:strength}

{trait name:strength admits:number held:of-the-kind}
{trait name:laboring admits:0-or-1 held:stored}
```

**Same number of lines either way.** What differs is **whether a trait's declaration is in one
place**. `1A` keeps `admits` and `held` beside the first `of` and repeats the name; `1B` gives the
trait one clean line and scatters its domain across `kinds.4x`.

**`1B` is `P-448`'s principle - the member declares what it belongs to.** `1A` is the shape the
release's table already has.

# 2. Where a trait's values are written, when they are not kinds

**Three sorts of `admits`, and only the third has nowhere to go.**

```
{trait name:laboring admits:0-or-1}      a closed pair - fine today
{trait name:resource admits:resource}    a family - P-448 already declares families
{trait name:biome admits:???}            ice, desert, grassland, jungle, mountain, ocean
```

**Those six are not kinds and belong to no family.** They exist only as this trait's values.

## 2A - a value declares its trait

```
{value name:ice of:biome}
{value name:desert of:biome}
{value name:grassland of:biome}
{value name:jungle of:biome}
{value name:mountain of:biome}
{value name:ocean of:biome}

{trait name:biome held:stored}
```

**`value` becomes a fourth declaring kind** beside `kind`, `trait` and `family`, and `admits` is
dropped where the values declare themselves. **This is `P-448`'s shape a third time.**

## 2B - biomes become kinds, and `admits` always names a family

```
{kind name:jungle family:biome}
{kind name:ice family:biome}
{family name:biome}

{trait name:biome admits:biome held:stored}
```

**Then `admits` is uniform**: a closed pair, or a family, and nothing else. **The cost is calling a
jungle a kind** - a sort of thing that can be in a game state - **when it is a property of a
territory.** `P-446` is the rule that makes that worth saying out loud rather than taking for the
uniformity.

# 3. Where the two tables of facts go

**`C-97` found that *Biomes* and *Units and structures* declare facts about kinds rather than
vocabulary.** Under `P-448`'s shape, one of them is not a new file at all.

## 3A - *Units and structures* folds into `kinds.4x`

```
{kind name:ark family:unit strength:2 binding:3 crosses:orbit-border requires:yard movable:yes}
{kind name:citizen family:thing strength:1 upkeep:1 readies:yes}
```

**Every column of that table is a trait the thing has**, so it folds the way `family` did and there
is no `units.4x`.

## 3B - it becomes a file of its own

```
units.4x
{thing name:ark strength:2 binding:3 crosses:orbit-border requires:yard movable:yes}
```

**Which needs a `thing` declaring kind**, and says the same facts about the same kinds in a second
place from `kinds.4x`.

**And *Biomes* waits on question 2.** Under `2A` it is `{value name:jungle of:biome nature:2
food-extractors:6 food-density:6 ...}` - **a value carrying traits**, which nothing has said is
allowed. Under `2B` a biome is a kind and it folds into `kinds.4x` like the ark.

## Evaluated against *least expressive yet complete*, which answers two of the three

**The test, from `lenses/research/2026-09-08-least-expressive-yet-complete.md`:** *a primitive earns
its place when removing it moves the combinatorial explosion from the generated space into the
authored space. Anything whose removal only makes the generated space larger is sugar and should
go.*

### Question 1 - the test is silent, and saying so is the answer

**`1A` and `1B` are the same relation written from opposite ends.** Neither adds a primitive, and
the line count is identical - `strength` is three lines either way. **Removing either does not move
an explosion anywhere, because each one *is* the other.**

**So this is a placement question wearing an expressiveness question's clothes**, and applying the
test to it would be the shape `CLAUDE.md` warns about: an instrument answering a narrower question
than the one asked. **`P-446` is silent too** - neither writes down anything false.

**Which leaves `P-428`'s tie-break, and it is a real tie.** The unified form wins by default, and
`1B` is the same shape as `family:` - **the member declares what it belongs to**. So **`1B`**, on the
weakest of the three grounds rather than the strongest, and this item says so rather than dressing it
up.

### Question 2 - the test chooses `2A`, once *expressive* is read correctly

**Counting words, `2B` looks cheaper**: it adds no declaring kind where `2A` adds `value`. **Counting
what the notation admits, it is the opposite.**

**A description names a kind.** Make `jungle` a kind and **`{jungle} -> 1` becomes a well-formed
description of a thing that cannot exist.** Checked in the data: `adjacency` is a kind and appears as
an entry **30** times in `scenario/expected/play.4x`; every biome appears **0** times, because a
biome is something a territory *has* rather than something that is anywhere.

**So `2B` is not one primitive fewer - it is one primitive doing two jobs, and the price is a
formalism that admits states with no meaning.** That is expressiveness bought and not used, which is
exactly what the test sends away.

**And the authored cost of `2A` is one line fewer, not more**: six `value` lines and a trait line
against six kind lines, a family line and a trait line. **Nothing explodes either way**, which is why
the decision rests on what is admitted rather than on what is typed.

### Question 3 - the test chooses `3A` directly

**`3B` adds a declaring word and a second file that names the same kinds.** Removing it moves no
explosion into the authored space; **it removes a copy.** Sugar, by the test, and `3A` needs nothing
new at all - every column of *Units and structures* is a trait, and traits already attach to kinds.

## So: `1B`, `2A`, `3A` - and only two of them for the reason you asked about

**Two are settled by the test and the third is not a question it can answer.** `1B` rests on
`P-428`, which is the weakest ground of the three, and **if a trait's domain belongs with the trait
then `1A` is right** - the principle would have an exception, and `P-446` says a named exception
costs less than a false uniformity.
