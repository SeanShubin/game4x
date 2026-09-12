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

### P-442 - does the notation's vocabulary declare itself, or is a declaring word a second category?

**to** sean - **status** open - **raised** 2026-09-12 - **rewritten** 2026-09-12, examples first, on your instruction - **kind** entailed, from the code lane's `C-97` - **asks** a decision - **into** `spec/console.md` -> The language

**The two options differ by one line.** Here is a file of kinds under each.

## Option A - the vocabulary declares itself

```
{kind name:kind}
{kind name:trait}
{kind name:family}
{kind name:citizen}
{kind name:garrison}
{kind name:extractor}
```

## Option B - a declaring word is a second category

```
{kind name:citizen}
{kind name:garrison}
{kind name:extractor}
```

**That is the whole difference: whether the first three lines exist.**

## The same choice on the two files that are hardest

**Traits.** Today's row is `| **strength** | citizen, garrison, ark, pioneer | a number | of the
kind |`. Four things it is of, so four entries - a description carries one value per trait:

```
{trait name:strength of:citizen admits:number held:of-the-kind}
{trait name:strength of:garrison admits:number held:of-the-kind}
{trait name:strength of:ark admits:number held:of-the-kind}
{trait name:strength of:pioneer admits:number held:of-the-kind}
```

**Under A that file also contains `{kind name:trait}` somewhere**, so the word `trait` is a kind like
any other. Under B it does not, and `trait` is on a second list.

**Biomes needs neither**, which is why the code lane starts there - it describes rather than
declares. Today's row is `| Jungle | 6 x 6 | 1 x 2 | 1 x 2 | 2 |`:

```
{biome name:jungle food-extractors:6 food-density:6 metal-extractors:1 metal-density:2 energy-extractors:1 energy-density:2 nature:2}
```

**Every word there is already a kind, a trait or a value.** No rule changes for it either way.

## What each costs, now that you can see them

**A pays with one strange line.** `{kind name:kind}` declares the word it is written in. A reader
meets it once and either finds it neat or finds it a trick; nothing else in the notation changes,
and `spec/console.md`'s rule stands **exactly as written** - *every word in a data file is a kind, a
trait, or one of a trait's values.*

**B pays with a second list.** The rule gains a clause - *...or one of the words that declare* - and
a reader checking whether a word is legal now has two places to look. **Three words would be on it**:
`kind`, `trait`, `family`. In exchange, no file contains a line about itself.

## Why this is a decision rather than words to approve

**`P-428` says a tie goes to the unified form, and A is the unified one** - one category of word, one
rule, no second list. **This lane is not applying that**, because twice on 2026-09-11 it reached for
that rule where the answer was outside the options, and **a self-describing vocabulary is the kind of
elegance that reads well and is disliked on sight or not at all.** You are the one who can say which.

## Nothing waits on this

**Biomes and Units and structures describe rather than declare**, need no new form, and are where
the code lane is starting. **The comparison check comes first either way.** This is the question that
arrives when it reaches the other six.

