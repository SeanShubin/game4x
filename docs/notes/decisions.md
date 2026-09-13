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

### P-479 - the traits in a description no longer sort, and what they do instead is yours

**to** sean · **status** open · **raised** 2026-09-13 · **kind** contradiction, from your order of relevance landing at `ed8c1bb` · **shape** text and an instruction · **asks** a decision · **into** `spec/console.md` -> The language, and possibly `spec/data/traits.4x`

**`spec/console.md` says something that is now false**, and it is one clause of one sentence:

```
Entries are in the order their descriptions sort in, and the traits inside a
description sort too, so the same state is always the same bytes
```

**They rank now, they do not sort.** The guarantee that clause exists for is untouched - the order
is still total and still a function of the description alone, so the same state is still the same
bytes. Only *which* order is wrong.

## What the reports say today

```
before   {territory biome:grassland id:1 nature:1}
after    {territory id:1 biome:grassland nature:1}

before   {deposit capacity:3 density:4 free:3 occupied:0 resource:food}
after    {deposit density:4 resource:food occupied:0 free:3 capacity:3}
```

**You said `id` first and `occupied`, `free`, `capacity` last, and said nothing about the other
twenty-two.** So the code lane built a rank with an open middle: `id`, then everything else
alphabetically, then those three. **The middle is still the alphabet nobody chose** - which is what
you noticed in the first place.

## The decision: where a global order lives

You asked whether defining one belongs to this lane. It does. **What it must not be is a second
list of the same twenty-six names**, and that rules out the obvious answer.

**`A` - the order is the order of `spec/data/traits.4x`.** That file already names all twenty-six
exactly once, in a line order nothing currently reads. Make that order the rank and **a trait
cannot be added without being placed**, because adding it *is* placing it. The file would be
reordered once, `id` first and `occupied`, `free`, `capacity` last, with the middle in whatever
order you want relevance to run.

**`B` - keep the rank with the open middle**, which is on disk now. Nothing ever goes stale and a
new trait lands somewhere alphabetical and readable - but the middle is unchosen, permanently.

**`C` - a stated list of all twenty-six in `spec/console.md`.** This is the one to refuse: two
lists of one set, and *a fact is stated once* says the second is generated or removed. The code
lane asked for a check that every declared trait appears in the ordering exactly once, **which is
exactly the check `A` makes unnecessary rather than satisfies** - a carrier rather than a reminder.

**`A` is this lane's recommendation.** Under `B` you are choosing not to choose the middle; under
`C` you are choosing to maintain a second list forever.

## The second question, which the code lane made quietly and flagged

**On a declaration a trait is named and not valued**, and `id` is one of them:

```
{kind biome family:place id name:territory nature}
```

Ranking `id` first there would pull it ahead of `name:territory`, which is what says which kind the
line is about. **So the rank reads the value as well as the name**: a valueless trait keeps its
alphabetical place and only a valued one moves. That is why `spec/data/` did not change by a byte.

**It is defensible and it is a choice nobody stated.** Your order is about a thing, and `id:1`
identifies one. **What a declaration line leads with is the same question as the global order**, so
it is here rather than in a separate item.

## What is not in question

**Nothing about the guarantee.** Total, a function of the description alone, same state same bytes.
Whatever you choose, the sentence keeps that half and loses the word *sort*.
*Nothing is open. Everything filed has been decided.*
