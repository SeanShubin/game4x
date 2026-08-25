# planet-ecs

[Architecture](../../docs/architecture.md) · [Layers](../../docs/layers.md) · [Root README](../../README.md)

Game entities, as ECS entities. **Contains no rules.**

Every thing in the world with identity and state lives here as a Bevy entity with
components. There is no second way of holding game state — no parallel `Vec` of regions,
no side-table of ownership.

## What the systems do

Exactly two things:

- **gather** — read the ECS into plain data, keyed by `RegionId`
- **apply** — write plain data back into the ECS

Between them they call [`planet-model`](../planet-model/README.md), which is a pure
function and has never heard of Bevy. Entities are nouns, algorithms are verbs, systems
are the glue with no opinions of their own.

```rust
// GATHER: ECS to plain data, indexed by RegionId so query order cannot show.
let before = World::with_owners(topology, &owners);
// RESOLVE: a pure function, with no idea any of this exists.
let after = before.advance(&intents);
// APPLY: plain data back to the ECS, one write per region.
```

## Why gathering is safe

Query iteration order is not a contract — it follows archetype layout, which follows
insertion history. So gathering never *accumulates* in iteration order; it writes into a
vector indexed by `RegionId`. The result is identical however the query happened to walk,
which is what lets the rest of the schedule run in parallel without changing the answer.

`Entity` is never identity. Bevy reuses ids and does not keep them stable across runs or
saves, so an `Entity` is never serialised, never ordered, and never a tie-break.
`RegionId` means the same thing in every run.

## Components and resources

| Type               | Kind      | Meaning                                            |
| ------------------ | --------- | -------------------------------------------------- |
| `Region(RegionId)` | component | a region, carrying its canonical identity          |
| `Owner(PlayerId)`  | component | who holds it; **absent means unowned**             |
| `WorldTopology`    | resource  | the adjacency graph, fixed when the world is made  |
| `PendingIntents`   | resource  | the ordered array waiting to be folded in          |
| `TurnAdvanced`     | message   | raised after a turn, so other layers need not poll |

## Tests

Run against a real Bevy `App`:

- `spawn_order_does_not_change_the_outcome` — entities spawned in four different orders,
  which changes archetype layout and therefore query iteration order, must produce
  identical ownership. This is the confluence property at the ECS boundary.
- `identity_is_the_region_id_not_the_entity` — two runs agree on the answer keyed by
  `RegionId`, whatever entities they happened to allocate.
- `abandoning_removes_the_component_rather_than_blanking_it` — unowned means absent.
- `replaying_the_same_turns_reproduces_the_world` — the replay guarantee, through the ECS.
