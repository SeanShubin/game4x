# planet-model

[Architecture](../../docs/architecture.md) · [Layers](../../docs/layers.md) · [Root README](../../README.md)

The model: one function and the data it needs.

```
(old world, intent array) -> new world
```

This is the only part of the project that has to be **certain**. Everything else may
vary.

## The four rules, and how they are kept

| Rule                  | How it is enforced                                                                           |
| --------------------- | -------------------------------------------------------------------------------------------- |
| **Integers only**     | a test scans this crate's own source and fails on `f32`/`f64` in code                        |
| **Pure**              | no clocks, no randomness, no I/O, no globals                                                 |
| **Engine-free**       | zero dependencies; the crate cannot name `Entity` or `Query`                                 |
| **Order-independent** | the result depends on the intent array's contents and order, and on nothing about scheduling |

The float check is not decoration. Integer addition is associative, so a parallel
reduction gives the same answer however the work is split; floating point addition is
not, so it does not. One `f64` here would quietly break both reproducibility and
parallelism.

## The shape of a turn

`World::advance` runs in three phases, and the phases *are* the argument for why this is
safe to parallelise:

1. **Gather** — every intent is judged against the world as it was at the *start* of the
   turn. No intent can see another's effect, so no ordering of this phase changes what it
   produces.
2. **Resolve** — proposals that collide on a region are settled by the intent's position
   in the array. That is data, not schedule, so the answer is fixed. It could be computed
   as a parallel `min` reduction without changing.
3. **Apply** — each region is written by exactly one decision.

## Why there is so little game here

There is one rule — claiming a region, which must be adjacent to something you already
hold unless it is your opening move — and it exists to make the architecture real and
testable, not to be good game design. **It is meant to be replaced.** What should survive
is the shape.

## Tests

The interesting ones are properties rather than examples:

- `the_same_inputs_always_give_the_same_output` — the requirement, stated directly.
- `replaying_a_log_reproduces_the_world` — the replay guarantee.
- `the_intent_order_settles_a_genuine_collision` — the array's order is an input, and it
  is *allowed* to matter.
- `reordering_intents_that_do_not_collide_changes_nothing` — and where there is no
  collision, it must not. This is what lets a whole turn resolve in parallel.
- `no_intent_sees_another_intents_effect_within_a_turn` — the read/write separation, which
  is the thing most likely to be broken by accident later.
- `the_resolve_phase_does_not_care_what_order_it_reduces_in` — confluence of the reduction.
- `no_floating_point_anywhere` — the integers-only rule, enforced.
