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

### P-462 - a territory's line has to say it carries an `id`, and there is no form for it

**to** sean · **status** open · **raised** 2026-09-12 · **kind** entailed, found checking the five open proposals against each other · **asks** a decision · **into** `spec/console.md` -> The language

**`P-459` gives a bounded stored trait a form and an unbounded one still has none:**

```
{kind name:citizen bearing:1 defending:1 laboring:1}      P-459 - the number is the maximum
{kind name:territory ???}                                 id, biome and nature have no maximum
```

**A citizen's three actions each get their own key because each has a number to carry.** A
territory's `id`, `biome` and `nature` are stored too, and `P-459`'s own closing sentence is *a
stored trait with no number on the kind's line has no maximum* - **which presumes the trait is on
the line and does not say how.**

**`P-454` says it is *named there*.** That was written when no stored trait carried anything, and
`{…}` holds `key:value` pairs.

## What the invariants rule out, which is both of the obvious answers

**Several lines for one kind is out.** `{kind name:territory trait:id}` beside
`{kind name:territory trait:biome}` would work and `spec/invariants.md` forbids it: **a definition
arrives in one transition, and there is no state in which a kind is half defined.** Between the two
lines there is one.

**A key appearing twice is out.** `{kind name:territory trait:id trait:biome}` is a key taking
several values, which is `C-98`, and `P-448` answered it by inverting - **a kind declares its family
rather than a family listing members** - precisely so that no key would have to.

**And a bare word is what `P-458` argues against.** `{kind name:territory id biome nature}` adds a
second form to `{…}`, which today is `key:value` and nothing else. **A list is the right length when
every primitive is a thing rather than a point on an axis that already exists** - and *what a `{…}`
may hold* is an axis that exists.

**So the invariants narrow it to one shape and do not choose the value.** One line, one key per
trait - and the question is what the key takes when there is no maximum.

## The three candidates

**A** - **a bare word after all**, taking the `P-458` cost knowingly:

```
{kind biome id name:territory nature}
```

**B** - **a word of the notation meaning *no maximum*:**

```
{kind biome:any id:any name:territory nature:any}
```

One new word, `key:value` throughout. **It says nothing `traits.4x` does not already say**, which
is the objection.

**C** - **every stored trait carries a number, and the unbounded ones carry none because a kind's
line lists only what it bounds** - so a territory's line is `{kind name:territory}` and **which
traits a kind has is read from somewhere else.**

**That contradicts `P-451`** - *a kind declares which traits it has* - so `C` is a proposal to
change that rule rather than to fill a gap. **Named because it is the one that costs nothing at the
line and something at the rule**, and you may prefer that trade.

## What this lane would take, and how weakly

**`A`**, and less confidently than anything else filed today. `P-458`'s third bullet is about
**primitives**, and a bare word in a declaration may be a form rather than a primitive - **this lane
cannot tell**, which is why this is a decision and not words to approve.

**Nothing is blocked by it.** `kinds.4x` as it stands carries `name` and `family` and no trait
lines, so the five proposals open to you land whole without this being answered. **It is the next
thing, not a condition on those.**
