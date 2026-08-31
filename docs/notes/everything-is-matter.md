# Everything Is Matter, and Rust Is the Engine

**Derived.** Written by Claude from conversation, 2026-08-30. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Spec backlog](spec-backlog.md) · [Control without tedium](control-without-tedium.md)

Sean, thinking about how units should be defined: *perhaps a pioneer is actually just a personnel
transport made of metal that stores fuel so it can move. However it could be stripped down for parts
in order to be consumed in transformation formulas.* And separately: *I am leaning towards a more
data driven game where the units and transformations are simply data inputs to rust, and rust is
providing a statically typed engine to run and validate the data.*

Two ideas, and the first is larger than it sounds.

## Conservation makes a unit a container, and that collapses three rules into one

Today a Pioneer **costs** 8 metal: the metal is spent and gone. Under conservation a Pioneer **is** 8
metal, held in a particular arrangement, and stripping it gives the metal back.

**Follow that and a structure is the same kind of thing.** A Yard is not a thing that cost 15 metal,
it is 15 metal arranged as a Yard. So a territory's metal is not its loose pile - it is the loose
pile **plus every structure plus every unit standing there**.

**Which means Sean's three separate limits are one limit.** He has asked for a bound on stored
resources *to make sure the game is finite*, and noticed a bound on units is needed *for the same
reason*, and the backlog wants storage structures with capacity. If everything is matter, all three
are:

> **A territory holds only so much matter, and everything in it is matter.**

One rule, and it closes the finiteness question completely rather than per-category: the total in a
territory is bounded, the number of territories is bounded, so the reachable state space is bounded.
No separate unit cap, no separate store cap, no argument about whether a unit parked on a full
territory counts.

## The asymmetry that falls out, and it is the right one

Conservation cannot apply to everything, and which resources it applies to is not arbitrary:

| Resource   | Persists between turns | Recoverable from what it built |
| ---------- | ---------------------- | ------------------------------ |
| **Metal**  | yes                    | **yes** - it is stuff          |
| **Energy** | yes                    | no - it is spent moving        |
| **Food**   | no                     | no - it is eaten               |

**Metal is the only conserved one**, because it is the only one that remains a thing after being
used. That matches `P-126`'s split for a different reason - that one is about *demand being lumpy or
continuous*, this one is about *whether the resource survives its use* - and the two agree, which is
worth noticing before anybody writes a rule that makes them disagree.

**And the specification already permits this without requiring it.** `spec/invariants.md`:
*conservation is not required; for example, fiat money is created and destroyed by a government, and
that is a model.* What is forbidden is a quantity changing with nothing in the world doing it.
Conservation of metal is a stronger promise than the invariant demands, and nothing has to change to
allow it.

## The question that decides whether it adds depth or drains it

**Is stripping lossless?** If a Pioneer returns all 8 metal, then building one is a decision with no
material cost, and a decision you can fully undo is not much of a decision. If it returns 4, then
conservation is not conservation and the appeal of the idea - one clean rule - is gone.

**The way out is that matter is conserved and time is not.** Building took turns and labour, and
neither comes back. So a lossless strip is materially neutral and remains expensive, because what it
cost was **the turns you were not doing something else**. That preserves the weight of the decision
without a fudge factor, and it is the honest version of what a 50% refund is usually trying to
approximate.

**It also gives stranded units somewhere to go.** A Pioneer that cannot reach anywhere useful is
currently 8 metal thrown away; under conservation it is 8 metal standing in the wrong place, which is
a problem with a solution.

## Capacity is organization, not room

Sean, 2026-08-30: *if you have a bunch of metal lying around it gets lost, but if you have storage
containers to keep it organized you have more usable metal in the same area... capacity is not
conserved, but it has limits imposed by precise rules.*

**This separates two things the word capacity was hiding.** How much fits is about space; how much is
*usable* is about arrangement. Loose metal is not lost because the territory ran out of room - it is
lost because nobody organised it. A container does not make the territory bigger, it makes more of
the territory's contents count.

**Which makes infrastructure productive rather than merely permissive**, and that is the part worth
keeping. A storage building that only *allowed* more metal would be a permission slip; one that turns
unusable metal into usable metal is doing work.

### It needs a depth rule or it is unbounded

A container takes some top-level capacity and provides more inside. **If a container may hold a
container, capacity is infinite** - one crate provides room for ten crates, each providing room for
ten more. The precise rules Sean wants are exactly the ones that stop that, and there are three:

- **Containers hold resources, never containers.** Depth one, and it terminates by construction.
- **Bounded nesting depth.** More expressive, one number to tune.
- **Diminishing capacity with depth.** Most flexible, hardest to reason about.

**The first is the same shape as the rule language's termination argument** - `P-117`'s third
construction makes rule references acyclic so nothing can recurse. Containment is another graph that
must not close on itself, and the cheapest answer is again structural rather than a check at runtime.

### And it collides with an invariant, head on

`spec/invariants.md` -> **No penalty for building infrastructure**: *no structure a player builds ever
has to be removed to make room for something else.*

**If every structure consumes capacity and capacity is finite, that is exactly what happens.** Fill a
territory with Yards and the next container needs one torn down. That invariant is not a preference -
it is one of the oldest lines in the specification and other rules lean on it.

Three ways out, and the choice is Sean's:

- **Structures do not consume capacity; only loose resources and units do.** Keeps the invariant
  untouched and makes a container purely a giver. The oddity is a Yard occupying no room in a world
  where a crate does.
- **Capacity is per category** - so much for structures, so much for stores, so much for units - so
  building never crowds out building. Keeps the invariant, costs the single clean rule.
- **The invariant narrows** to say infrastructure costs nothing *to keep*, dropping the promise about
  room. Cheapest to write and the likeliest to be regretted, since never having to demolish is a
  large part of what makes the game unfussy.

**Worth settling before the mechanic is designed further**, because the first two change what a
container is and the third changes what the game is.

## Rust as the engine, data as the game

Sean's leaning answers the question [the entity-model discussion](game-4x-predecessor.md) left open -
which side is the source of truth. **The data is, and Rust validates it.**

That dissolves the objection that killed the attribute-bag model on its own. The predecessor's
`Thing` was stringly-typed and `requireInt()` threw at runtime; a validating loader checks the same
things **once, at load, against a declared schema**, and everything downstream is typed. You get
*adding a submarine is data* without giving up the compile-time guarantees the model needs.

**There is precedent one level down.** `commands/*.4x` already specifies a *scenario* in data - `add
node`, `set force`, `set biome` - and the console validates each line and reports where it went
wrong. What Sean is describing is the same machinery one level up: data that defines **kinds**
rather than instances.

### The predecessor already had the syntax, which dissolves the objection below

Sean: *the predecessor had a language that specified a trait map, with the first trait automatically
being the name, so you would have something like `{node type:food quantity:6 density:8}`, that you
could type into the console.*

**Checked, and the grammar shipped.** `language/Expressions.kt` defines `named` as an open brace, a
name, then an attribute list - so the first token is the name - and `call` is a name followed by
parameters, where a parameter may be a trait map or a primitive. It also has `alias`, a `$name`
reference, which is what a transformation needs in order to speak about the thing it just matched.

**So data-defined kinds and *every change is a console command* were never in tension.** A trait map
is a console argument. The invariant does not have to widen; the command language has to grow a
literal, and it has grown one before.

**What did not ship is the layer above it**, and it matters that it was designed rather than built.
The sketch below sits in a comment in `UniverseCommandRunnerTest.kt`, and the test beside it uses
Kotlin objects:

```
colonize
required has-food-node
required no-citizens
required remove-colonizer
optional add-citizen
optional add-farm

has-food-node
greater-than 0 {node, resource:food}
```

**Three things in eight lines that this project currently lacks.** A transformation is a **named list
of named clauses**, so it is openable and a small change is a small edit - `P-112` satisfied by
construction rather than by intention. `greater-than 0 {node, resource:food}` is a **named, reusable
condition**, which is the vocabulary this lane has repeatedly said does not exist. And `required`
against `optional` is the same split as [cost against requirement](intermediate-steps.md): one must
hold or the transformation fails, the other applies when it can.

The same comment carries **test cases in the same language**, with the result written as deltas -
`1 -> 0 {colonizer}`. A transformation and its test as one artifact.

### The one thing that still needs deciding

`spec/invariants.md` says **every change to game state is representable and executable as a console
command**, and `P-115` says a rule's actions are recorded exactly as if typed. Both are about
*state*.

**A unit type is not state, it is rules** - and that stays true even though the syntax is typeable.
Being able to *say* a trait map at a console does not settle whether saying it changes the game's
state or the game's rules. **The answer that keeps one function over one state is that types are
state**, and defining a Pioneer is a transition like any other.

That second answer is stranger and better. It keeps one function over one state, it makes the
history a complete account of the game *including what the game was*, and it is what would make the
thing moddable. What it costs is that the definitions become part of every saved game, and two games
could disagree about what a Pioneer is - which is either a bug or the whole point, depending on
whether the game is one game or a family of them.

**Nothing needs deciding until transformations are written.** It is recorded here because it will be
easier to answer before there is code than after.
