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

## The decision, and it is one visible question with a second behind it

**The visible one: does a declaration line lead with what it declares?**

```
today   {trait admits:number kept:thing name:movable}
        {kind biome control family:place id name:territory nature}

yes     {trait name:movable admits:number kept:thing}
        {kind name:territory family:place biome control id nature}
```

**`A` - yes.** The notation's own words lead, `name` first among them, then the traits rank.
**Forty of the fifty-eight declaration lines in `spec/data/` change**; the other eighteen already
lead with `name` by alphabetical accident. **It needs a third rule this lane would have to draft**:
an order among `name`, `admits`, `kept`, `of` and `family`, since alphabetical among them puts
`admits` first and defeats the point.

**If you answer no, the second question is why, and the two answers differ in what happens next
rather than in any byte today.**

**`B` - because `P-479` only moves a trait that carries a value**, and a declaration names traits
without valuing them. One rule with a qualifier. **Nothing changes now**, and this is the code
lane's current behaviour. **But `{value name:mountain nature:1 of:biome}` already carries a valued
trait**, so the rule is live on declaration lines - the day one carries a valued `id`, `occupied`,
`free` or `capacity`, the line moves and nobody decided that it should.

**`C` - because `P-479` is about a state and does not reach a declaration.** Two scopes, no
qualifier. **Nothing changes now and nothing changes later without a decision**; a declaration's
order is alphabetical until something states otherwise.

**So the three are: change forty lines, or leave a rule that can move one later, or close it.**

## What made this findable

**Neither proposal is wrong and neither is stale.** `P-479` ordered a description's traits and
`P-481` said which words are not traits; **the gap is in the join**, which is
`docs/notes/two-conventions-on-one-page.md` a week early and in this lane's own column.

## What this does not ask

**Nothing about a state line.** `{territory id:1 biome:grassland nature:1}` is right under all
three, because there every trait carries a value and no declaration word appears.
*Nothing is open. Everything filed has been decided.*
