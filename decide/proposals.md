# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-575 - the four design rules, three of which the schema can already say

**to** sean · **status** open · **raised** 2026-09-27 · **answered** 2026-09-27, `N1` · **asks** approval · **kind** entailed · **shape** an instruction · **into** `spec/tests/`, then `spec/data/rules.4x` and `spec/data/schema.4x`

**You chose `N1`**: write the four missing design rules and review them. **The reviewed fifteen can
play a game and cannot make one** - `CreatePlanet`, `SetResource`, `SetBiome` and `AddUnitToOrbit`
have no counterpart, and `spec/console.md` says *designing is therefore made of the same rules as
playing*.

## What you are approving is four behaviours; the tests are what you will read

**A rule is not read in this queue.** `spec/README.md` rule 3 makes the test the primary statement,
and you read a test in the review application. **So this offers the four one-line behaviours**, and
each becomes a test you approve there and a rule written to satisfy it.

```
create-planet   makes a territory, its two places - surface and orbit - and the crossings
                between territories, for a planet of a named size
set-resource    gives a territory a deposit of one resource, with a density
set-biome       gives a territory its biome
add-to-orbit    puts a unit in the orbit of a territory before play begins
```

## One of the four cannot be written yet, and it is a schema change rather than a rule

**There is no `biome` relation.** Measured over `spec/data/schema.4x`'s forty-seven relations:
`territory`, `place`, `adjacency` and `deposit` are all there; **`biome` is not, and neither is
`planet` nor `orbit`** - a planet is its territories and an orbit is a `place` with `layer:orbit`.

**So `set-biome` needs the schema to gain a relation before its rule can exist.** `P-541` already
brought biome back to the data and nothing has carried it into this schema. **The other three need
no schema change at all.**

## Your determinism point, answered without measuring

**Under `N1` there is no randomness to seed.** Every one of the four states a value: a size, a
resource and a density, a biome, a unit and a territory. **The same commands make the same world
because nothing chooses anything.**

**The seed belongs to `generate-planet`**, which chooses what is not specified according to a
policy - `spec/console.md` already says *the same seed and the same policy give the same planet*,
and it is specified and unbuilt. **`N1` does not build it**, so your recollection about biomes being
the only randomness bears on `N2` and not on this.

## How to tell it was carried out

```
spec/tests/                 four new tests, one per behaviour above, each stating a world,
                            firing one design command, and asserting what it made
the review application      shows all four as unread, so they are yours to read
spec/data/schema.4x         gains a `biome` relation and nothing else
spec/data/rules.4x          gains four rules, and the suite runs the four tests against them
reviewed/                   unchanged until you read them - no test is approved by this
```

**The last line is the point.** This lane writes the tests and never the record.
