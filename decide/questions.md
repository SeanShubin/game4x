# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-575 - promotion refused: three of the four are rules and `create-planet` cannot be one

**to** sean · **status** open · **raised** 2026-09-27 · **refused** 2026-09-27, before writing anything · **asks** a decision · **kind** measured · **into** `spec/tests/`, `spec/data/`, and whether world-making is a rule at all

**You said promote and this lane is refusing**, because carrying the instruction out would have
produced a rule the engine rejects. **Found before writing, by asking whether a rule can make a
varying number of rows.**

## It cannot, and the reason is written in your own schema

**`{repeats rule:R}` is the only way a rule does a varying amount** - `spec/invariants.md`: *where a
rule would need a quantity that varies, it is written as a smaller rule that fires as many times as
it can.* **And it requires the rule to remove something:**

> **That is the termination proof**: the pool it draws from is finite and only ever shrinks, so the
> repetition stops without anything counting the turns. **A rule that removes nothing has no such
> pool and is refused** - `Malformed::NeverStops`.

**`create-planet` removes nothing.** It makes territories out of nothing, so it can never repeat,
and **a rule without `repeats` adds exactly one row per `add` clause.**

## What that means for a tiny planet

```
tiny-12   12 territories, 24 places - a surface and an orbit each - and 30 crossings
          = 66 add clauses, in one rule, for one size
and five sizes   tiny-12 small-32 medium-42 large-72 huge-92
huge-92          92 territories, 184 places, and the crossings of a 92-faced Goldberg
```

**So `create planet <size>` is not one rule; it is five rules, the largest of which states a whole
planet row by row.** That is expressible and it is not a rule anybody would write.

## The other three are fine and this lane is not holding them hostage

```
set-resource   one add clause - a deposit with a density          expressible today
add-to-orbit   one add clause - a unit in a place with layer:orbit expressible today
set-biome      one add clause, once the schema has a `biome`      a schema change first
```

**`P-541` brought biome back to the data and nothing carried it into this schema**, which is the
only reason the third is not immediate.

## The decision

```
N1a  the three become rules now, and world-making is answered separately.
     The app can set up a world it is given and still cannot make one
N2   `generate-planet` is built - one thing outside the ruleset that makes a world.
     spec/console.md already specifies it, seed and policy included, and it is the
     only one of the two that a player would ever use
N4   create-planet becomes five rules, one per size, each stating its planet
```

**This lane would say `N1a` now and `N2` next**, and would not write `N4`. **`N2` is where your
determinism point lands** - the seed exists precisely because a generator chooses, and nothing in
`N1` chooses anything.

**What this changes about `spec/console.md`**: it offers both `create planet <size>` and
`generate-planet`. **Under `N1a` plus `N2` the first one goes**, because five hand-written planet
rules is what it would have to mean.
