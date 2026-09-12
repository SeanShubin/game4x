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

## What this lane would say, asked

**`1B`, `2A`, `3A`** - the member declares what it belongs to, every time, and no table becomes a
file that repeats another.

**And the one thing that would change its mind** is question 1's cost: **`1B` is the only place the
principle splits a declaration across two files**, and `P-446` says a uniformity that is false
hides its complexity. **If a trait's domain belongs with the trait, `1A` is right and the principle
has an exception that is worth naming rather than denying.**

