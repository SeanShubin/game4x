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

### P-463 - `id` admits a number, and its magnitude means nothing

**to** sean · **status** open · **raised** 2026-09-12 · **kind** entailed, from your reading of `P-462` · **asks** a decision · **into** `spec/data/traits.4x` and `spec/console.md` -> The language

**Your words, and they are about a different file from the one `P-462` asks about:** *a territory id
is a unique identifier, its magnitude doesn't have relevance typical to a petri net, different ids
are different things.*

**`P-457` writes `id` as a number, and the expression language then lets a rule do arithmetic on
it:**

```
{trait admits:number kept:thing name:id}      what P-457 offers

max id of {territory}          well-formed, and means nothing
id of x > 5                    a legal guard, and means nothing
sum id of {territory}          well-formed, and means nothing
```

**Nothing forbids these**, because `spec/console.md` says an expression is `sum <trait> of {…}`,
`max <trait> of {…}` and a comparison with `<` - **for any trait**, and `id` is a trait admitting a
number.

**The notation already makes this distinction once.** `from` and `to` admit **`place`**, not a
number - and a place is written as its id, so the bytes are identical and the meaning is not. **An
id is the same case one step further in.**

## What it costs, which is small today and is the editor's problem tomorrow

**No rule in the release does arithmetic on an id**, so nothing is broken now. **What it costs is at
the editor**, which `P-458` says derives what to offer from what the game holds: `admits:number`
means a stepper and a comparison, and **neither is right for an identity** - an id is assigned, not
picked, and two ids are equal or they are not.

## The two ways

**A** - **`admits:identity`, a fourth word beside `number`, a family, and `value`:**

```
{trait admits:identity kept:thing name:id}
```

**And the notation says what it means**: two identities are equal or they are not, and no expression
orders or sums them.

**B** - **leave it `number`, and let the prose carry it.** `spec/logistics.md` already says *a thing
may carry an `id`, and one that does is unique; there is never a quantity of a thing with an `id`*.
**Rule 7 sends relationships to prose**, and this is one.

## Why this is a decision and not a recommendation

**`P-458` does not settle it, and it comes close enough to look as though it should.**

- **Its second bullet removes `identity`**: a primitive earns its place when removing it would move
  work to a person, and removing this moves none. **Nothing is authored twice** - the data just says
  something looser
- **Its third bullet keeps `identity`**: a list is the right length when every primitive is a thing
  rather than a point on an axis that already exists - and an identity is **not** a number, a family
  or a declared value, so it is a thing rather than a fourth point on a list of three

**The two bullets disagree, and this lane is not going to pick which one governs** - that is the
`C-49` line. **What the disagreement means is that `P-458`'s test is about authoring cost and this
is about correctness**, and you may want to know that the test has a case it does not reach.

## What it does to what is already open

**`A` changes one line of `P-457` and nothing else.** `B` changes nothing at all. **`P-457` can land
first either way** - if `A` wins afterwards, the line is a one-word cleanup rather than a rewrite,
and this proposal is the record that it was seen rather than missed.
### P-462 - a territory's line has to say it carries an `id`, and there is no form for it

**to** sean · **status** open · **raised** 2026-09-12 · **kind** entailed, found checking the five open proposals against each other · **asks** a decision · **into** `spec/console.md` -> The language

**Two files, and only the second one is in question.**

**`spec/data/traits.4x` says what each trait is, and `P-457` settles it:**

```
{trait admits:number kept:thing name:id}       an id is a number, each thing has its own
{trait admits:value kept:thing name:biome}     a biome is one of a declared list
{trait admits:number kept:thing name:nature}   nature is a number, each thing has its own
```

**`spec/data/kinds.4x` says which traits a kind has, and that line has nowhere to put them:**

```
{kind family:place name:territory}             today - it says nothing about traits at all
```

**`P-451` says a kind declares which traits it has**, so that line has to grow. **`P-459` gave the
form for a citizen**, and it works only because each of those traits has a maximum:

```
{kind name:citizen bearing:1 defending:1 laboring:1}      the 1 is the MAXIMUM, not the value
```

**A territory's `id`, `biome` and `nature` have no maximum**, so there is no number to put after the
colon - and a `{…}` holds `key:value` pairs and nothing else:

```
{kind family:place name:territory ???}
```

**The question is what the key takes when there is nothing to bound.** It is the shape of a
declaration line and not what an id or a biome means.

## What the invariants rule out, which is both of the obvious answers

**Several lines for one kind is out.** `{kind name:territory trait:id}` beside
`{kind name:territory trait:biome}` would work, and `spec/invariants.md` forbids it: **a definition
arrives in one transition, and there is no state in which a kind is half defined.** Between the two
lines there is one.

**A key appearing twice is out.** `{kind name:territory trait:id trait:biome}` is a key taking
several values, which is `C-98`, and `P-448` answered it by inverting - **a kind declares its family
rather than a family listing members** - precisely so that no key would have to.

**And a bare word is what `P-458` argues against.** `{kind name:territory id biome nature}` adds a
second form to `{…}`, which today is `key:value` and nothing else. **A list is the right length when
every primitive is a thing rather than a point on an axis that already exists** - and *what a `{…}`
may hold* is an axis that exists.

**So the invariants narrow it to one line with one key per trait, and do not choose the value.**

## The three candidates

**A** - **a bare word after all**, taking the `P-458` cost knowingly:

```
{kind biome id name:territory nature}
```

**B** - **a word of the notation meaning *no maximum*:**

```
{kind biome:any id:any name:territory nature:any}
```

One new word, `key:value` throughout. **It says nothing `traits.4x` does not already say**, which is
the objection - `traits.4x` already gives `biome` its values and `nature` its type.

**C** - **a kind's line lists only what it bounds**, so a territory's line stays
`{kind family:place name:territory}` and **which traits a kind has is read from somewhere else.**

**That contradicts `P-451`** - *a kind declares which traits it has* - so `C` is a proposal to change
that rule rather than to fill a gap. **Named because it is the one that costs nothing at the line and
something at the rule**, and you may prefer that trade.

## What this lane would take, and how weakly

**`A`**, and less confidently than anything else filed today. `P-458`'s third bullet is about
**primitives**, and a bare word in a declaration may be a form rather than a primitive - **this lane
cannot tell**, which is why this is a decision and not words to approve.

**Nothing is blocked by it.** `kinds.4x` as it stands carries `name` and `family` and no trait lines,
so the five proposals open to you land whole without this being answered. **It is the next thing, not
a condition on those.**
