# Layers: Intent to Pixels

[Documentation map](README.md) · [Root README](../README.md)

How the game is arranged so that it is **fully parallel and perfectly predictable at the
same time**, and where the boundary between those two demands falls.

The companion document is [architecture](architecture.md), which covers the crate graph
and the dependency rules. This one covers why those layers exist.

## 1. The thesis

There is exactly one function the game must be certain about:

```
(old world, event array) -> new world
```

Everything else — where a region is drawn, what colour it is, how many milliseconds a
frame took — is free to vary, and mostly should.

The requirement on it is exactly this and nothing more: **it is a pure function of its
two arguments.** Same old world, same event array, same new world. On any machine, on
any core count, forever.

Two properties are wanted, and they are usually treated as opposites:

- **Predictability.** The same inputs always reach the same output.
- **Parallelism.** The work spreads across every core available.

They are only opposites if predictability is bought by **fixing the order in which work
is scheduled**. It does not have to be. Arrange things so that scheduling order *cannot*
affect the outcome, and both properties hold at once with neither paying for the other.

That property has a name — **confluence** — and it is the constraint this document
exists to serve.

### Two different orders

The word "order" does two jobs here, and conflating them is the mistake to avoid:

| Which order                                      | Part of the input?             | Allowed to affect the result? |
| ------------------------------------------------ | ------------------------------ | ----------------------------- |
| Position within the **event array**              | **Yes** — it is data           | **Yes**                       |
| Which system, core, or entity **resolves first** | No — an accident of scheduling | **No**                        |

The event array is an ordered argument, so its order is free to matter as much as the
rules like. What must never matter is the schedule. "Same inputs, same output" is the
whole requirement; "the same things happen in the same order" is not required and is not
worth paying for.

## 2. Three ideas that are easy to confuse

Precision matters here, because the three sound alike and demand different things.

| Property                       | Statement                                                    | Do we need it?                                                          |
| ------------------------------ | ------------------------------------------------------------ | ----------------------------------------------------------------------- |
| **Reproducible**               | Same inputs give the same output, on any machine, any run    | **Yes** — this is the requirement                                       |
| **Confluent**                  | The outcome does not depend on the order *work is scheduled* | **Yes** — it is how the first is achieved without giving up parallelism |
| **Sequentially deterministic** | A *fixed* schedule gives the same outcome                    | No — too weak *and* too restrictive                                     |

The third is the one to avoid leaning on. It is weaker, because it says nothing about
whether that fixed schedule is achievable in parallel. And it is more restrictive,
because achieving it usually means serialising. A confluent system is sequentially
deterministic for free; the reverse does not hold.

So the rule is not "make the schedule predictable". It is **"make the schedule
irrelevant"** — which leaves the event array free to be as order-sensitive as the rules
require.

## 3. Transcendental functions: the precise rule

The instinct that transcendental functions are dangerous is right, but the reason is
narrower than "floating point is unpredictable".

`sin(x)` is a pure function. It returns the same bits every time on the same binary and
the same machine. What it is not is *the same on a different machine*, because IEEE 754
draws an explicit line:

| Correctly rounded — bit-identical everywhere             | Unspecified — the implementation chooses                                       |
| -------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `+` `-` `*` `/`, `sqrt`, `fma`, comparisons, conversions | `sin` `cos` `tan` `asin` `acos` `atan` `atan2` `exp` `ln` `pow` `hypot` `cbrt` |

`sqrt` is safe. `acos` is not. So the question is never "are floats allowed here". It is:

> **Does this value ever need to be identical on a different machine?**

Which is only true for replays, saves that regenerate rather than store, lockstep
multiplayer, and cross-platform test fixtures.

### Does modelling a sphere force them into the model?

No. Three different jobs hide under "the sphere", and only one of them is the model's:

| Job                                        | How often   | Transcendentals | Must match across machines |
| ------------------------------------------ | ----------- | --------------- | -------------------------- |
| Deciding **who touches whom**              | once        | **no**          | yes, if worlds are shared  |
| Deciding **where things are**              | once        | yes             | no                         |
| Deciding **where things appear on screen** | every frame | yes             | never                      |

The model — the part that evolves turn after turn — needs only the first, and the first
is **combinatorial, not metric**. "Region 5 borders region 12" is a fact about a graph.
Once decided it is `Vec<Vec<u32>>`, and there is not a float in sight.

So the sphere does not push transcendentals into the model. It pushes them into a
**boundary function that runs once and whose output is integers**. World generation is
not a simulation step; it is what produces the model's first state.

### The one place this is not theoretical

Adjacency is currently decided by `atan2` and `hypot`, with the test
`border length > 1e-9`. For a typical border of about 0.3 radians, a one-unit-in-last-place
difference between two platforms' maths libraries cannot flip that comparison.

But [we measured](theory/region-splitting.md#why-a-regions-sides-are-different-lengths)
that some borders are very nearly zero — the cases where four regions almost meet at a
point. Those are exactly the comparisons a 1-ULP difference *could* flip, and flipping
one changes the adjacency graph, which changes the game.

Three ways out, in order of cost:

1. **Ship worlds as data.** Generate once, serialise the graph. Reproducibility stops
   being a question. *This is the current plan.*
2. **Make the predicate exact.** Adjacency reduces to sign-of-determinant tests, which
   can be evaluated exactly in integer or rational arithmetic — standard robust-predicate
   work in computational geometry. Then generation itself is portable.
3. **Accept it**, and require that any given world is generated on one machine.

Choose 2 only when two machines must generate the same world independently.

## 4. The four layers

```
   Intent  ──▶  Model  ──▶  View model  ──▶  View
  (values)   (integers)     (floats)      (Bevy, GPU)

        ▲                                     │
        └───────────  new intents  ◀──────────┘
```

### Intent

A **value**, not a state. `Generate { seed, regions }`, `MoveArmy { from, to }`.
Integers, serialisable, and carrying a position in an array.

That last part matters: the array index is a legitimate, data-derived tie-break key.
When two entries genuinely conflict, the conflict is settled by *where they sit in the
input*, never by which system happened to run first.

**Intent or event?** Worth deciding deliberately, because it moves where the rules live:

- An **intent** is a request, and the model may reject it. `MoveArmy` might be illegal.
  The log records what a player *asked for*, and the rules are applied when it is folded
  in — so a later change to the rules changes the replay.
- An **event** is a fact that has already been validated. The log records what *happened*,
  so replays survive rules changes, but validation has to live somewhere earlier and the
  log is larger and derived.

The default here is **intent**, because it keeps the rules in one place and matches what
a save file naturally is: the record of what everyone chose to do.

### Model

The state produced by folding intents in order. **Integers only.** No transcendentals,
no floats. Reproducible and confluent. This is the only layer that must be certain.

World generation is not special here — it is simply the first intent.

### View model

A projection of the model into renderable terms: positions on a sphere, colours,
screen geometry, camera orientation. Floats and transcendentals are entirely welcome,
because nothing here needs to match anything.

It is derived from the model and **never feeds back into it**.

### View

Bevy entities, meshes, shaders, pixels.

### The invariant

**Data flows one way, and the only thing travelling back up is a new intent.** Input
never mutates the model; it produces an intent that gets folded in.

That single rule is what buys replays, saves as a command log, deterministic tests, and
lockstep multiplayer — because the model becomes a pure fold over a list.

## 5. Confluence: how order is made irrelevant

Bevy's scheduler guarantees there are no **data races** — two systems that conflict on
component access will not run at the same time. That is a safety guarantee, not a
confluence guarantee: mutual exclusion in an *arbitrary order* still changes the outcome
if the operations do not commute.

Confluence is the application's job.

### 5.0 Two ways to spend the array's order

Since the event array's order is allowed to matter, the design question is *how* it
matters — and the two answers have very different parallelism.

**As a sequence.** Event *i*'s effects are visible to event *i+1*. Simple and obviously
correct, and inherently serial: *i+1* cannot start until *i* has landed. Parallelism is
then confined to within a single event's own effect.

**As a tie-break.** Every event in the array sees the same starting world. Where two
events collide, the lower index wins. Fully parallel across the whole array, and still
perfectly reproducible — because the index is data, not a schedule.

Both are deterministic. Only the second is parallel. **Default to the tie-break reading**
and reach for sequencing only where a rule genuinely requires one event to observe
another's result — which, for simultaneous turn resolution, is rare. Where a batch does
need internal stages, it is usually cleaner to split the tick into two batches than to
serialise one.

The remaining four techniques are how the tie-break reading is made to work.

### 5.1 Separate reading from writing

Within a tick, no system observes another system's writes. Everything reads the state as
it was at the start of the tick and writes somewhere else; the results are swapped in at
a sync point. If no system can see a partial result, no ordering can change one.

### 5.2 Combine with operations that commute

Where several intents affect the same value, combine them with an operation that is
**associative and commutative** — sum, min, max, union, bitwise or. A parallel tree
reduction over such an operation gives the identical answer regardless of how the tree
is shaped.

> **This is why "integers only" is load-bearing for parallelism, not just for
> portability.** Integer addition is associative. Floating point addition is *not*:
> `(a + b) + c` and `a + (b + c)` can differ. A parallel float reduction therefore gives
> different answers depending on how the work was split — order-dependence introduced by
> the arithmetic itself, no matter how careful the scheduling. The integer rule from
> [vision](vision.md#whole-numbers-only-in-the-game-logic) was adopted for cross-machine
> reproducibility; it turns out to be exactly what makes parallel reduction safe.

Integer overflow must also be a decision rather than an accident: pick `checked_`,
`saturating_` or `wrapping_` deliberately, since debug and release builds disagree about
the default.

### 5.3 Gather, resolve, apply

The general shape of a tick, and the answer to everything that looks stubbornly
sequential:

| Phase       | Parallelism    | Why it is confluent                                                        |
| ----------- | -------------- | -------------------------------------------------------------------------- |
| **Gather**  | Embarrassing   | Read-only. Nothing is written, so nothing can be observed out of order.    |
| **Resolve** | Tree reduction | A pure function of the *multiset* of proposals, with tie-breaks from data. |
| **Apply**   | Full, disjoint | Each piece of state is written by exactly one source.                      |

Things that appear to demand sequencing usually just need restructuring into this shape:

- **Two armies claim one region.** Do not let the first mover win. Gather all claims,
  then resolve over the set: highest strength, tie-broken by owner id, then by intent
  index.
- **Several cities draw from one stockpile.** Not first-come-first-served. Gather all
  draws, then allocate by a stated policy — proportional, or by priority — as a function
  of the whole set.
- **Combat.** Gather all attacks on a target and resolve them as one batch.

In each case the fix is the same: **stop asking "what happens next" and start asking
"what is true of the whole set".**

### 5.4 Canonicalise results, do not order the work

If a phase must produce a sequence — a list of events, an ordering of turns — sort it by
a **stable, data-derived key** at the end. Region id, player id, intent index; never
`Entity`, never insertion order, never hash order.

This is worth separating from the advice it superficially resembles. Sorting by id is
*not* a way to make execution ordered. It is a way to let execution be completely
unordered and still end up with one canonical answer.

## 6. Entities and algorithms are different kinds of thing

The commitment is **ECS everywhere ECS applies** — which is a real rule, not a hedge,
because what it applies to has a clear test.

**Every game entity is an ECS entity.** Regions, units, players, stockpiles — if it is a
thing in the world that has state and identity, it is an entity with components, and
there is no second way of holding game state. No parallel `Vec` of regions, no
side-table of ownership, no "just this once" struct.

**Algorithms are categorically not that.** An algorithm is a pure function over plain
data. It is not a system, not a plugin, and it never sees the ECS at all. This is not an
exemption granted to a few awkward crates; it is a different kind of thing, and the
distinction holds everywhere, including the game rules themselves.

The test for which side something falls on is simply: **does it have identity and state
that persists across a tick?** If yes, it is an entity. If it is a transformation from
values to values, it is an algorithm. Nothing is both, and the cases that feel like both
are usually an entity plus an algorithm that has not been separated yet.

### The division

|                 | Entities                             | Algorithms                                        |
| --------------- | ------------------------------------ | ------------------------------------------------- |
| What they are   | nouns: the state of the world        | verbs: transformations of values                  |
| Where they live | Bevy components and resources        | plain functions in crates with no Bevy dependency |
| May name        | `Entity`, `Query`, `Commands`, `Res` | none of those, ever                               |
| Tested by       | running a schedule                   | calling a function                                |

Between them sits a third thing that is neither: a **system**, whose entire job is glue.
It gathers plain data out of the ECS, calls an algorithm, and applies the result back. A
system that contains a rule is a mistake; the rule belongs in the function it should have
called.

### It is the same rule as the tick shape

The three phases of [gather, resolve, apply](#53-gather-resolve-apply) turn out to be
this distinction viewed from the other side:

| Phase       | Who does it  | What it touches        |
| ----------- | ------------ | ---------------------- |
| **Gather**  | a system     | ECS in, plain data out |
| **Resolve** | an algorithm | plain data only        |
| **Apply**   | a system     | plain data in, ECS out |

Systems only ever gather and apply. Algorithms only ever resolve. Nothing else is
allowed to happen, and the two rules reinforce each other: the confluence argument works
because the resolve step is a pure function, and the resolve step is a pure function
because algorithms are categorically not ECS.

### Why this is not a compromise

Keeping algorithms out of the ECS buys three things that would otherwise need arguing
for one at a time:

- **They are trivially confluent.** A pure function has no shared state, so there is no
  order for it to depend on. Most of section 5 exists to recover, for systems, a property
  algorithms have for free.
- **They are testable without an engine.** No `App`, no schedule, no window. Call it,
  assert on the answer.
- **The boundary is compiler-enforced.** An algorithm crate does not depend on Bevy, so
  it *cannot* name `Query` or `Entity` even by accident. `cargo tree` is the audit.

`sphere-tessellation` and `graph-coloring` already work this way — `fn(seeds) -> graph`,
zero dependencies. They are not special cases to be tolerated; they are the shape every
algorithm takes. What changes as the game grows is that their *output* becomes entities:
the tessellation function emits a graph, and a system spawns one entity per region from
it.

### What ECS demands in return

The entity side is where the care is needed, because these are invisible failure modes:

- **`Entity` is not identity.** Ids are reused and are not stable across runs or saves.
  The canonical identity of a region is its `RegionId`. `Entity` is a runtime handle and
  must never be serialised, compared for ordering, or used as a tie-break.
- **Query iteration order is not a contract.** It follows archetype layout, which follows
  insertion history. Never let it be observable in a result.
- **`Commands` are deferred**, and applied at a sync point in system order — so spawn
  order, and therefore `Entity` allocation, varies between runs. Harmless, given the rule
  above.
- **Hash iteration is unordered.** Prefer `BTreeMap` in the model, or sort before use.

### The consequence for saving and replay

If the ECS *is* the model state, then "old world" and "new world" are ECS worlds, and the
replay guarantee depends on being able to serialise one deterministically. Component
iteration order is not a contract, so a snapshot has to be written in a canonical order —
sorted by `RegionId`, never by `Entity` or archetype layout.

This is the strongest argument for saves being an **intent log** rather than a state
snapshot: a log is a list of plain values with an order already defined, and it sidesteps
the question entirely. See [the open questions](#9-open-questions).

## 7. How this gets tested

Confluence is invisible in normal operation. A tick that accidentally depends on order
will pass every test and then disagree with itself on a different machine, or under a
different core count, or after an unrelated system is added. It has to be tested
directly:

- **Vary the schedule, hold the input.** Given the same old world and the *same* event
  array, run the tick repeatedly with system ordering and thread count varied, and assert
  the resulting worlds are byte-identical. Note what is held fixed: the event array. Its
  order is an input, so shuffling it is expected to change the answer — shuffling the
  *schedule* is not. This is the test that catches accidental order-dependence, and it is
  worth running over generated inputs rather than one fixed case.
- **Shuffle the array deliberately.** Where the rules intend the tie-break reading,
  reordering two events that do *not* conflict must leave the result unchanged. That is a
  real property of the rules, and a good way to find accidental coupling between events.
- **Fold twice.** Applying an intent log to a fresh state must reproduce the state it
  originally produced. This is the replay guarantee, asserted rather than hoped for.
- **No floats in the model.** Cheap to check structurally, and worth checking, because a
  single `f64` field is enough to break both reproducibility and parallel reduction.
- **Core count independence.** Run the tick with one thread and with many, and compare.
  This is the cheapest way to catch a system that reads another's partial writes.

## 8. Where the current code sits

| Crate                                   | Layer            | State                                                              |
| --------------------------------------- | ---------------- | ------------------------------------------------------------------ |
| `sphere-tessellation`, `graph-coloring` | model algorithms | conforms — pure, no dependencies, integer output                   |
| `planet-model`                          | intent + model   | conforms — the fold, integers enforced by test                     |
| `planet-ecs`                            | model entities   | conforms — entities are ECS, systems gather and apply only         |
| `planet-render`                         | view model       | conforms in structure; still contains the CPU rasterizer           |
| `planet-bevy`                           | view             | window, input, vsync; presentation is still a blitted pixel buffer |
| `planet-view`                           | composition root | wiring only                                                        |

The one-way flow is real and runs end to end: a keypress becomes an `Intent`, the intent
is folded by a pure function, the result is applied to ECS components, and the view reads
ownership back out to tint regions. Nothing travels the other way.

**What is not done:** the sphere is still drawn by a CPU loop rather than as GPU
geometry, so the view layer has not yet been re-cut. Section 9 and
[architecture](architecture.md#open-questions) carry the rest.

## 9. Open questions

- Where does the intent log live — a resource in the ECS, or outside it entirely?
- Are intents applied one array per turn, or continuously? Turn-based suggests the
  former, which makes the tick boundary obvious and the fold trivial.
- Does the view model get rebuilt each frame from the model, or incrementally updated?
  Rebuilding is simpler and obviously correct; incremental is faster and needs its own
  confluence argument.
- Are saved games an intent log, a state snapshot, or both? A log is smaller and proves
  the replay property; a snapshot loads instantly and does not depend on the rules
  staying identical between versions.
- How is AI expressed? If AI emits intents like any other player, it inherits replay and
  determinism for free.
