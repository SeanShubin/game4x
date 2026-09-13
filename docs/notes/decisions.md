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

### P-481 - the words that declare the vocabulary are declared nowhere, and one of them is `name`

**to** sean · **status** open · **raised** 2026-09-13 · **kind** contradiction, found by answering your question about `name` · **shape** text and an instruction · **asks** a decision · **into** `spec/console.md` -> The language, and `spec/data/`

## First, your question, because the answer is not what you assumed

```
{territory id:1 biome:grassland nature:1}          a state
{kind biome family:place id name:territory nature} a declaration
```

**The leading word is the kind**, and `spec/console.md` says a description is *a kind and every
trait of that thing*. So on the state line the kind is `territory`, and on the declaration line the
kind is `kind` - that line describes a thing whose kind is `kind`, and `name:territory` is a trait
saying which one.

**So the thing you wanted first is already first and needed no rule.** A kind is not a trait and
always leads. `P-479` only ever ordered what comes after it.

**What you called `name` is the kind on a state line and the trait `name` on a declaration line** -
the same role played by two different things, which is why one sentence could not cover both.

## The problem that question uncovered

**`name` is not a declared trait.** Nor are four others. Measured over `spec/data/`:

```
used as a key          admits 26   kept 26   name 58   of 7   family 7   nature 5
declared in traits.4x  nature only
```

**And the four kinds that declare the vocabulary declare no traits at all:**

```
{kind name:kind}   {kind name:trait}   {kind name:family}   {kind name:value}
```

**`spec/console.md` says every word in a data file is a kind, a trait, or one of a trait's
values**, and that a file using any other word *is wrong about the game rather than describing it*.
By that sentence `spec/data/` is wrong about the game in 124 places.

**The same document says the declaration rule needs no exception** - *`kind`, `trait`, `family` and
`value` are themselves kinds, so a line that declares one is a description like any other*. **A
description like any other carries declared traits**, and these carry five undeclared ones.

## The decision

**`A` - declare them.** `traits.4x` gains `name`, `admits`, `kept`, `of`, `family`, and the four
declaring kinds gain their trait lists - `{kind admits kept name:trait name of}` and so on. The
rule keeps no exception and the file says what every other file says.

**`B` - exempt them, as the expression words already are.** `spec/console.md` already carves out
one set: *the words an expression is built from are the notation's own; they are the one thing in a
data file that is not a kind, a trait, or one of a trait's values.* **`name`, `admits`, `kept`,
`of` and `family` are the same sort of thing** - they are how the notation talks about itself, not
about the game. The sentence stops saying *the one thing* and names two.

**`A` costs five trait lines and four longer kind lines and makes the vocabulary self-describing.
`B` costs one sentence and admits that a data file has a layer that is not game data.** This lane
leans `B`, because `admits` and `kept` describe traits rather than describing anything in a game,
and declaring them invites `{trait admits:value kept:kind name:admits}`, which is true and is not
useful to anybody.

## And then what leads a declaration line

**This is the half that moved out of `P-479`, and it cannot be settled first.** If `name` is a
trait, it can be ranked; if it is the notation's own word, it is not in the ordering at all and
leads because the notation says so - the same way the kind leads a state line.

**Under `B` there is nothing more to decide.** Under `A`, `name` ranks first among traits and the
line reads `{kind name:territory biome family:place id nature}` where today it reads
`{kind biome family:place id name:territory nature}`.

**The code lane's current behaviour is neither**: it ranks `id` first only when valued, so a
declaration's bare `id` stays alphabetical and `name` stays where the alphabet puts it. That was
its choice, flagged as one, and it is what these words replace.
*Nothing is open. Everything filed has been decided.*
