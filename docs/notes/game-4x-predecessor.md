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
[`spec/population.md`](../../spec/population.md) in his own words, so the spec is now canonical
for it and this section is only provenance.

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
[the backlog](spec-backlog.md) and P-41 in [proposals](proposals.md).

## How the predecessor composed a turn, and why it cannot be copied

Read 2026-08-26, to settle the question P-54 raises: does a turn run **phase at a time** across
the whole planet, or **territory at a time**?

**The predecessor ran territory at a time.** The only place a full turn is assembled is
`UniverseCommandRunnerTest`:

```kotlin
val command = EveryLandUniverseCommand(GenericLandStrategy)
```

`EveryLandUniverseCommand` takes **one** land command and applies it to every land in turn;
`GenericLandStrategy` is the whole eight-step sequence. So each land gathered, ate, grew,
discarded and reset **before the next land began**. Ordering was `planet.lands` index order, not
claim order.

**But it never faced the question, because nothing crossed a border.** `Land` has no adjacency -
no neighbour field, and no command that reads one. Every land was a closed system, so the order
of the two loops could not possibly matter. Its answer is not evidence.

**And its structure shows why that answer would now be wrong.** `DiscardSupply` is step 7 of
`GenericLandStrategy`, *inside* the per-land sequence. Territory at a time therefore means the
first territory destroys its surplus before the second is reached. Under P-46 and P-56 - food
moving to adjacent territories, neighbours taking the remainder - **cross-border supply would be
impossible**, not merely awkward. The first territory's leftovers are gone before anyone can ask
for them.

**The inspiration worth taking is that the nesting is data, not control flow.** The same pieces
compose either way, and it is one line:

```kotlin
EveryLandUniverseCommand(GenericLandStrategy)                    // territory at a time
CompositeUniverseCommand(EveryLand(produce), EveryLand(consume)) // phase at a time
```

A turn is a list of commands and a rule for iterating lands, so choosing between the two readings
in P-54 does not require a different engine - only a different composition. That is the design
property to keep.

Three smaller things worth carrying over:

- **`CompositeUniverseCommand` threads state sequentially** - each command sees the previous
  one's universe - and **short-circuits on the first failure**, abandoning the rest of the turn.
- **`ZeroOrMoreCommand` repeats a step until it fails**, which is how *build as many farms as you
  can* worked. Failure is the loop's terminator rather than an error, so a command's failure is
  ordinary control flow.
- **Every command is `toObject()`-serializable**, which is what let a turn be written out as
  data. That is the same property Sean now wants for
  [the console](../../spec/console.md) - a turn that can be replayed as a test.

## Naming, then and now

| game-4x           | the spec                 |
| ----------------- | ------------------------ |
| node              | territory                |
| density           | density (of a node)      |
| gatherer          | farm                     |
| activated citizen | citizen expending labour |
| supply            | (unnamed so far)         |

## The language

`language/` and `script/` hold a parser combinator library and a command notation. Reviewed
separately in [parser architecture](parser-architecture.md).

## Not from the game

`foo.txt` at the repository root is personal writing unrelated to the project. Worth knowing
it is there, given the repository carries an UNLICENSE and a Maven Central deploy config.
