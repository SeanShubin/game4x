# The game-4x Predecessor

**Derived.** Written by Claude from conversation, 2026-08-25. Not binding - a record of what
the previous attempt contained, so its mechanics are not lost when it is retired.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

`../game-4x` is a Kotlin/Maven project that **this repository supersedes**. It is not a
dependency and nothing here should import from it. It is worth reading only for mechanics
that were worked out there and have not yet been restated in
[the specification](../../spec/README.md).

76 commits, ~294 tracked files, modules `domain`, `command`, `game`, `language`, `script`,
`format`, `console`, `contract`, and `prototype` through `prototype4`.

## The population rule, recovered

The terminology is **citizen**, not population. From `NewCitizensEnterCommand`:

```kotlin
val totalCitizenCount = land.countPartiallyMatches(Thing("name" to "citizen"))
val foodSupplyCount   = land.quantityByThing[foodSupply] ?: 0
val newCitizenCount   = min(totalCitizenCount, foodSupplyCount)
```

**New citizens = min(current citizens, food remaining).** The population doubles, capped by
the food left after everyone has eaten.

`EatOrLeave` supplies the other half: each citizen eats exactly one food, and any citizen
the food does not cover **leaves** - it does not die. It runs twice in priority order, so
citizens who worked this turn eat before idle ones, and the idle are what starve out.

Sean has since written this rule into
[`spec/planet.md`](../../spec/planet.md) in his own words, so the spec is now canonical for
it and this section is only provenance.

## The turn, from `GenericLandStrategy`

```
1. ColonizeLand           zero or more
2. RunGatherer("food")    zero or more - activates a citizen and a node,
                                          yields food equal to the node's density
3. BuildGatherer("food")  zero or more
4. ActivatedCitizensEatOrLeave      workers eat first
5. NonActivatedCitizensEatOrLeave   idle eat the remainder; unfed ones leave
6. NewCitizensEnter                 population doubles, capped by leftover food
7. DiscardSupply                    food does not carry over
8. ResetActivated                   everyone available again
```

Two details make the loop work and neither is obvious from the names. `RunGatherer` takes
the **highest-density node first** (`sortedByDescending { density }`). And `DiscardSupply`
**throws away all remaining food at end of turn**, so surplus cannot be stockpiled - it can
only be spent on growth in step 6. That is what makes the `min()` a real constraint rather
than bookkeeping.

Whether food carries over is unspecified in the new spec. See
[the backlog](spec-backlog.md).

## Naming, then and now

| game-4x           | spec/planet.md           |
| ----------------- | ------------------------ |
| node              | territory                |
| density           | fertility                |
| gatherer          | farm                     |
| activated citizen | citizen expending labour |
| supply            | (unnamed so far)         |

## The language

`language/` and `script/` hold a parser combinator library and a command notation. Reviewed
separately in [parser architecture](parser-architecture.md).

## Not from the game

`foo.txt` at the repository root is personal writing unrelated to the project. Worth knowing
it is there, given the repository carries an UNLICENSE and a Maven Central deploy config.
