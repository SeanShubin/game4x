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

### P-396 - `move` treats one unit as a quantity and as a thing in the same recipe

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, twice - the second time when Sean asked why the engine touches readiness at all, which is the right question and has an answer that predates `P-390` - **kind** entailed - **asks** a decision - **into** `releases/first-release.md` -> Recipes, and `spec/console.md` -> The language

**Your diagnosis is right and nothing in the engine is doing extra work.** Readiness is not refreshed
by moving. **It is refreshed by a rule**, `spec/turn.md`: *a thing created during a turn begins
holding its tokens and may act at once*. **That rule is correct and wanted** - it is why a newly
built extractor can be worked the turn it is built, which is *No penalty for building infrastructure*
doing its job. **`move` drags it in by claiming to create something.**

## The cause is older than `P-390`, and it is visible inside `move` itself

**Two of `move`'s five rows disagree about what a unit is.**

| Row                              | What it says                                         |
| -------------------------------- | ---------------------------------------------------- |
| `consume 1 unit [ready] @ $from` | **a quantity of a kind** - one unit, any unit        |
| `consume 1 energy @ that unit`   | **one particular thing**, referred to by `that unit` |

**`spec/logistics.md` already forbids the first where the second is possible**: *there is never a
quantity of a thing with an `id` - it is one thing, and anything that holds it holds exactly it.*
**So either a unit carries an `id`, and `consume 1 unit` is not something the release may write; or
it does not, and `that unit` refers to nothing.** The recipe needs both and the notation offers one.

## The answer to your question, of the two you offered

**It is the first - recipes being concrete - and more precisely than that.** **The notation has one
noun: a quantity of a kind.** Everything a recipe says is *how many of what, where*. That is exactly
right for food, metal and labor, which are interchangeable and have no identity. **It has no way to
say *this* thing**, so a rule about one identified thing has to be written as destroy-one-and-make-one,
and `move` is the first rule that is about an identified thing.

**Templating is not the limitation.** `P-390`'s parameterised action shows the templating carries
weight fine - a kind plus a trait, and containment supplies the per-value maximum. **What is missing
is a second kind of noun, not a better way to parameterise the one there is.**

## So the decision is narrower than a patch to `move`

- **The notation gains a way to name one thing and say what changes about it.** `move` says a unit is
  somewhere else. Its `id`, its tank and its spent token all survive because nothing is destroyed,
  and `spec/logistics.md` stops being contradicted. **`deploy ark` and `found by land` want it too** -
  `X-12` records the same pressure, a `require` row that is a parameter declaration wearing a
  threshold's clothes
- **Keep one noun and patch each loss.** Three patches, one per thing that does not survive being
  destroyed: the tokens it arrives without, the `id` it keeps, what its tank still holds. **And
  `spec/logistics.md` stays contradicted**, because `consume 1 unit` is still a quantity of a thing
  with an `id`

**This lane is not recommending, because adding a noun to the notation is a design decision and
those are yours.** What it will say is that the contradiction is there today, with or without
`P-390`, and that this item found it by being asked *why would the engine do that* rather than by
looking for it.
