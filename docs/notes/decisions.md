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

### P-483 - `P-479`'s order applies to a declaration line and `spec/data/` does not obey it

**to** sean · **status** open · **raised** 2026-09-13 · **kind** contradiction, found by re-reading The language whole after two proposals landed in it · **shape** an instruction · **asks** a decision · **into** `spec/console.md` -> The language, and `spec/data/kinds.4x`

**A declaration is *a description like any other*** - `spec/console.md` says so - so `P-479`'s
order applies to one. **It puts `id` first**, and `spec/data/kinds.4x` does not:

```
today   {kind biome control family:place id name:territory nature}
P-479   id first, then every other trait alphabetically
```

**And `P-481` took the notation's own words out of the trait ordering without saying where they
go.** `name:territory` and `family:place` are not traits, so nothing places them.

## The decision, and it is one question with three answers

**`A` - the notation's own words lead, then the traits rank.**
`{kind name:territory family:place id biome control nature}`, where today it reads
`{kind biome control family:place id name:territory nature}`. **A declaration then reads as what
it declares, then what it is, then what it has** - the shape you asked for when you said the
type comes first.

**`B` - `P-479`'s order applies only where a trait carries a value.** A declaration names traits
without valuing them, so they stay alphabetical and the notation's own words sort among them -
which is `spec/data/` exactly as it is, and no file changes. This is the code lane's current
behaviour, flagged by it as a choice.

**`C` - `P-479` does not reach a declaration at all**, because it is about what a thing contains
and a declaration is not contained. Then a declaration's order is stated separately or left
alphabetical, and `spec/data/` does not change either.

**`A` is the only one that changes a file**, and it is the only one that makes a declaration line
lead with the thing it is about. **`B` and `C` differ only in what the specification says**, not in
any byte on disk.

## What made this findable

**Neither proposal is wrong and neither is stale.** `P-479` ordered a description's traits and
`P-481` said which words are not traits; **the gap is in the join**, which is
`docs/notes/two-conventions-on-one-page.md` a week early and in this lane's own column.

## What this does not ask

**Nothing about a state line.** `{territory id:1 biome:grassland nature:1}` is right under all
three, because there every trait carries a value and no declaration word appears.
*Nothing is open. Everything filed has been decided.*
