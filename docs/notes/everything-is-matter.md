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
ten more.

**Sean's rule, 2026-08-30: a thing cannot be nested inside itself.** No depth number, which is
right - a chosen depth is a number nobody can justify, and this one is derived.

**It has to mean transitively, and the difference is the whole rule.** Read as *a crate may not sit
directly inside a crate*, it does not terminate: a crate holds barrels, a barrel holds crates, and
alternating the two nests for ever. Read as **no type appears twice anywhere on a containment path**,
it does terminate, and the bound falls out rather than being picked - **the deepest possible nesting
is the number of container types there are.**

**This is the third time today the same argument has settled a termination question.** `P-117`'s
third construction makes rule references acyclic; the trigger for re-reading a section is about a
graph closing on itself; and here the *type* containment graph must be acyclic, which bounds the
*instance* nesting. Worth noticing as a pattern: this project's finiteness arguments keep turning out
to be *some graph is a DAG*, checked when the thing is built rather than while it runs.

### And it collides with an invariant, head on

`spec/invariants.md` -> **No penalty for building infrastructure**: *no structure a player builds ever
has to be removed to make room for something else.*

**If every structure consumes capacity and capacity is finite, that is exactly what happens.** Fill a
territory with Yards and the next container needs one torn down. That invariant is not a preference -
it is one of the oldest lines in the specification and other rules lean on it.

**Sean chose capacity per category, 2026-08-30** - so much for structures, so much for stores, so
much for units. Building never crowds out building, so the invariant stands untouched and nothing has
to be demolished to make room.

What it costs is the single clean rule: *a territory holds only so much matter* becomes *a territory
holds only so much of each kind of thing*, and the categories are now something the design has to
name and defend. The two rejected answers are recorded so they are not re-proposed: exempting
structures entirely, which leaves a Yard occupying no room in a world where a crate does; and
narrowing the invariant to costs-nothing-*to-keep*, which was cheapest to write and would have given
up the thing that makes this game unfussy.

**It also answers `P-126`'s open fork**, or comes very close to it. That proposal asks whether a
territory has a bound of its own or gets one only from a structure. If capacity is per category then
the store category has a bound before any container exists - otherwise nothing could be stored at
all, and a container would have nowhere to stand. So the answer is **a base the territory has, which
containers raise**, and the winnability measurement in `P-126` holds.

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

## Why this is worth the trouble, in Sean's words

*One value of expressing everything as either a bag of traits or a transformation is that I expect I
can specify ALL units with precision, in a way that I can keep in my head as a human, while at the
same time gives you an unambiguous specification. It is a way we could remove a good portion of the
typical human/AI-assistant miscommunication.*

**That is the strongest argument for the whole idea and it is not about the game.** A prose rule can
be read two ways by a careful reader, and the second reading is discovered by building it. A trait
map cannot: `{yard metal=15}` has one meaning.

**Today produced the evidence twice over.** `P-125` exists because *every structure that can be
built* read one way to the specification lane and another to the code lane, and the game was built to
the second - the qualifier that decides whether the release is winnable was doing nothing, and nobody
noticed for two days. Written as a transformation with its conditions named, the ambiguity has
nowhere to hide, because *can be built* would have had to be a clause with a definition.

**And removing ambiguity is the larger half, not a consolation.** Sean's correction, and he is
right: the two costliest defects this project has had were both a sentence read two ways -
`P-125`, and `P-96` leaving borders in a drawing it had just scoped ids and poles out of. Neither
was a hard problem. Both survived review by two careful readers because prose lets a reader supply
the meaning it did not carry, and neither reader knows they did it.

**It does not remove the second kind of failure, which is worth saying only so nothing is oversold.** `C-8` was
not an ambiguity - every number was clear and their consequence was not, and it took an afternoon of
arithmetic to find that only one territory could produce an Ark. A formal specification does not
compute that for you. **What it does is make it computable**, which is why the same result took a
ten-line script once the numbers were in a table. Ambiguity it removes; arithmetic it makes
checkable.

## A transformation is inputs and outputs, and it survives the test

Sean, 2026-08-30: *at the end of the day, a transformation is a set of inputs and a set of outputs...
perhaps there is also a set of conditions, but even that could be listed as a non-consumed input -
for example, something that could only be built in a jungle could take jungle as an input without
consuming the jungle.*

**Non-consumed inputs are the right idea and they get most of the way.** Every requirement in the
game is a thing that must be present and is not used up: a Pioneer needs a garrison, an Ark needs a
Yard, a jungle structure needs jungle. All three become inputs with a *consumed* flag set to no, and
the separate notion of a condition disappears.

**It needs one more column, and only one.** Some conditions are absences: founding requires the
territory **not** be held, and the predecessor wrote that as `equal-to 0 {citizen}`. An absence
cannot be an input at any quantity - until an input carries whether its quantity is a **floor or a
ceiling**. Then *at most 0 garrisons* is an ordinary input row, and the language still has no
conditions in it.

### Every transformation the game has, in four columns

Written out to see whether it holds rather than to argue that it does:

| Transformation  | Role | Thing           | Qty     | Consumed | Bound       |
| --------------- | ---- | --------------- | ------- | -------- | ----------- |
| build yard      | in   | metal           | 15      | yes      | at least    |
|                 | out  | yard            | 1       |          |             |
| build extractor | in   | labor           | 1       | yes      | at least    |
|                 | in   | unworked node   | 1       | no       | at least    |
|                 | out  | extractor       | 1       |          |             |
| produce pioneer | in   | metal           | 8       | yes      | at least    |
|                 | in   | energy          | 6       | yes      | at least    |
|                 | in   | citizen         | 1       | yes      | at least    |
|                 | in   | garrison        | 1       | **no**   | at least    |
|                 | out  | pioneer         | 1       |          |             |
| produce ark     | in   | metal           | 12      | yes      | at least    |
|                 | in   | energy          | 12      | yes      | at least    |
|                 | in   | yard            | 1       | **no**   | at least    |
|                 | out  | ark             | 1       |          |             |
| found           | in   | founding unit   | 1       | yes      | at least    |
|                 | in   | garrison        | 0       | no       | **at most** |
|                 | out  | garrison        | 1       |          |             |
|                 | out  | citizen         | 1       |          |             |
|                 | out  | extractor, food | 1       |          |             |
| move            | in   | unit, here      | 1       | yes      | at least    |
|                 | in   | energy cell     | 1       | yes      | at least    |
|                 | out  | unit, there     | 1       |          |             |
| work            | in   | labor           | 1       | yes      | at least    |
|                 | in   | extractor       | 1       | no       | at least    |
|                 | out  | resource        | density |          |             |

**Seven of the eight commands fit, and the four columns are enough.** *Move* is the one worth
noticing: it is not special at all - the input is a unit here and the output is a unit there, so
location is just a trait that differs between the two sides.

### The one that does not fit, and it is honest to say so

**`end turn` is not a transformation of things in a place.** Everything eats, a population grows or
starves, food expires, and everything becomes ready again - across every territory at once. Inputs
and outputs describe a *local* exchange, and this is a global sweep.

Two ways to take that. Either **the turn is a different kind of thing** and the language does not
have to cover it, which is honest and leaves one hand-written rule at the centre of the game. Or
**the sweep is itself a set of transformations applied everywhere they match**, which would make it
uniform - and that is exactly what the predecessor's `EveryLandUniverseCommand` and
`...WherePossible` commands were doing.

### The tension the table exposes: richer state or richer language

`build extractor` above takes an **unworked node** as a non-consumed input. There is no such thing in
the model - there are nodes, and there are extractors, and *unworked* is the difference between two
counts. The predecessor needed a comparison for exactly this: `less-than {gatherer resource:food}
{node resource:food}`.

**So either the language gains comparisons, or the state gains derived kinds.** They are the same
expressiveness bought in different places:

- **Comparisons in the language** keep the state small and make every transformation potentially
  arithmetic. The predecessor chose this.
- **Derived kinds in the state** keep the language to inputs and outputs, which is what makes it
  tabular - but somebody has to define *unworked node* as a function of nodes and extractors.

**Derived kinds look better here, and the reason is the tabular goal.** A comparison has two operands
and no natural column; a derived kind is just another row in the thing table. And nothing has to
maintain it if derived kinds are **computed rather than stored** - they are views, so no
transformation can leave one inconsistent.

### Turns as an input, when the time comes

Sean: *you could even argue a turn is an input, though we may be able to get away without specifying
it explicitly until we have something that takes multiple turns to make.* Agreed, and when it
arrives there are two shapes, worth choosing rather than falling into: an explicit **turn** input, or
an output that is a **partially built thing** which becomes the next step's input. The second needs
no new column and makes the work in progress visible and interruptible - which is probably what a
player wants to see anyway.

## Pushing it: what is left when everything is a thing

Sean: *it seems every thing can be done with state in a unified manner, let's push on that a bit.*
So: take every field the model stores and ask whether it is a thing somewhere, or something computed
from things.

### `founded` was already gone, and the table is what showed it

Sean had been wondering whether the concept is needed at all - *if all citizens leave, the place has
been abandoned; if a single citizen remains it is occupied.* **The transformation table settled it a
step earlier without either of us noticing.** The `found` row needed *garrison, at most 0* as its
input. It did not need, and could not use, `founded = false`.

**So `founded` is a stored boolean that duplicates something already visible in the territory's
contents**, and the formal shape exposed it the moment it was written out. That is the value Sean
predicted from formalising, arriving on the first real attempt.

**One correction to his version, from `spec/control.md`.** Occupied is not *a citizen remains* -
holding a territory takes force equal to its force of nature, and a garrison has force 1 against a
nature of 1. So a territory whose population has departed but whose garrison stands is **still
held**, and nature takes it back only when the force there falls below nature's. The derivation is
*force present is at least force of nature*, not *citizens above zero*.

### Two more stored fields go the same way

| Stored today  | Actually                                                           |
| ------------- | ------------------------------------------------------------------ |
| `founded`     | force present is at least the force of nature                      |
| `labor_spent` | not needed if a citizen is **ready** or **exhausted**              |
| `won`         | an Ark was launched from a fully exploited planet - already a rule |

**Three of the model's fields are derived, and all three were found by asking the same question.**
None of them can go out of step with what they describe once they are computed, which is a whole
class of bug that stops existing rather than being tested for - tier 1 in
[prevent, detect, bound](prevent-detect-bound.md).

### Ready and exhausted, and why Sean's two-step version is the better one

He offered both shapes:

```
(ready worker, extractor, node food 4)  ->  (exhausted worker, extractor, node food 4, 4 food)

(ready worker, turn)                    ->  (exhausted worker, labor)
(labor, extractor, node food 4)         ->  (extractor, node food 4, 4 food)
```

**The second is better, and the reason is that it makes labor a thing.** Once labor is a thing it is
an ordinary consumed input, and `work` no longer has to know anything about readiness or turns - it
takes labor and an extractor and yields a resource. The turn touches only the first line.

**And it needs the multiset to work.** Citizens are a count, not individuals, so *ready* and
*exhausted* cannot be flags on objects - they are two rows, `(citizen ready) x 5` and
`(citizen exhausted) x 3`. That is exactly the predecessor's `Land`: `List<Pair<Thing, Int>>`, a
thing and how many. **The multiset is not an optimisation, it is what lets a trait vary across a
population that has no individuals.**

### `end turn` fits after all, as a sweep of ordinary transformations

Earlier this note said `end turn` was the one command that would not fit, being global where inputs
and outputs are local. With ready and exhausted it decomposes into four transformations, each local,
each applied everywhere it matches:

- everything that eats, eats - `(citizen, food) -> (citizen)`
- a population grows on surplus or departs for want - `(citizen, surplus food) -> (citizen x 2)`
- food expires - `(food) -> ()`
- everything readies - `(exhausted X) -> (ready X)`

**What is global is not the rules but the mode**: *apply this everywhere it matches* rather than
*apply this here*. The predecessor had exactly that as a first-class idea - `EveryLandUniverseCommand`
and the `...WherePossible` commands. So the unification holds, and what it costs is one concept:
**a transformation is applied either at a place or everywhere.**

### The residue, and it turns out to be smaller than it looks

Four things are not things in a territory. Three of them are functions of the history, which
`spec/invariants.md` already says is the whole of the state:

| Not a thing | What it actually is                                            |
| ----------- | -------------------------------------------------------------- |
| turn number | how many times `end turn` appears in the history               |
| phase       | whether `start` appears in the history                         |
| `won`       | a rule over the state, as above                                |
| adjacency   | **genuinely not a thing** - a fact about the planet's geometry |

**Only adjacency survives**, and it survives because it is not about contents at all. Even it is
tabular if wanted - a row per pair - but nothing transforms it, so it is a fixed table rather than
state.

**So the answer to Sean's push is yes, with one concept added and one exception.** Everything is a
thing in a place or a function of the history; transformations are applied at a place or everywhere;
and the planet's shape sits outside, unchanging, as the board does in any game.
