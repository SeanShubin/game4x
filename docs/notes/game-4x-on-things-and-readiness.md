# What `../game-4x` did with things, quantities and readiness

**2026-09-04.** Sean asked for the earlier project's approach before answering `P-231`, which asks
whether `labor` is a kind. Read at `D:\keep\github\sean\game-4x`, in
`game/src/main/kotlin/com/seanshubin/game_4x/game/`. Kotlin, 245 source files, of which the four
that matter are `Thing.kt`, `Things.kt`, `Land.kt` and the two tests under `game/src/test`.

## The shape

```kotlin
data class Land(val things: List<Pair<Thing, Int>>)
data class Thing(val attributes: List<Pair<String, Attribute>>)
```

**A place is things with quantities. A thing is a bag of named attributes.** `Land` has one field
and it is the list; there is no field per kind, because there are no fields for kinds to have. An
`Attribute` is a string, an integer or a boolean, and that is the whole type system.

**This is `P-134` and `spec/invariants.md`'s *a game's state is things, in places, and how many of
each*, already built and already tested.** It is worth knowing that the rule the current model is
being rewritten towards is one the earlier project reached and used.

`Land.findUnique` and `Land.countPartiallyMatches` take a **partial thing as a query** -
`Thing.isPartOf` returns true when every attribute of the query matches - so *how many food
extractors are here* is a query against attributes rather than a field lookup. That is
`spec/invariants.md`'s *whatever reads the state reads it the same way whatever kind it holds*,
mechanised.

## Readiness

**There is no `labor`.** The kinds in `Things.kt` are colonizer, node, gatherer, citizen and supply.
What labor would have carried is carried by an attribute:

```kotlin
fun createCitizen(activated: Boolean = false): Thing
fun createGatherer(resource: String, activated: Boolean = false): Thing
fun createNode(resource: String, density: Int, activated: Boolean = false): Thing
```

**And a thing's readiness makes it a different thing.** `ApiSetupTest.populateLand` puts four
unactivated food nodes of density six and two activated ones into the same land, and reads them back
as **two rows with their own counts**. Nothing tracks *how many nodes are used*; the used ones are
simply a different row.

## What this does and does not settle for `P-231`

**It settles the shape.** A quantity belongs in the list with a count, not in a field beside it. The
earlier project could not have had `labor_spent: u32` because it had nowhere to put it.

**It does not settle whether `labor` should exist**, because the earlier project never had recipes
that cost labor. Its gatherers activate; nothing pays for them. **Measured in the current release,
`labor` is consumed by five recipes** - the three extractors, the yard, and `work` - and produced by
one, `create labor`.

So the earlier project's answer, applied here, is a third option: **no `labor`, and each of those
five recipes consumes a ready citizen and produces an exhausted one directly.** That costs `create
labor`'s three rows and adds one row to each of five recipes - **59 recipe rows becomes 61** - and
states the citizen's exhaustion five times rather than once.

**Which is the argument for keeping `labor` that neither lane had made**: it is not a bookkeeping
counter, it is what lets *spending a citizen* be written once and referred to from five places.

## One thing not to carry over

`Thing.setBooleanValue` mutates by copy and `Land.updateQuantity` deletes a row when its count
reaches zero, so **a thing that exists in quantity zero and a thing that never existed are the same
state.** That is fine for a prototype and would break `spec/interface.md`'s requirement that the
normalized view **name every table and every column, whether or not anything is in it** - a rule
this project has and that one did not.
