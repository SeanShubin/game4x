# Do the sets cross-cut? Measured on the release as it stands

**Derived.** Written by the specification lane, 2026-09-07, answering Sean's question: *based on what
we know about the game so far, do we have enough information to make an informed decision regarding
how we should model the data?*

[Notes index](README.md) · [Decisions](decisions.md) · [The release](../../releases/first-release.md)

## The question, made decidable

Different transformations apply to different sets of entities, and two such sets may overlap without
either containing the other. **That is the property that decides the representation**: if every pair
of sets is disjoint or nested, a single hierarchy holds them all and the candidate paradigms are
indistinguishable on this data. **One crossing pair is a counterexample a hierarchy cannot hold.**

So the question *do we have enough information* becomes: **does the release contain a crossing
pair?** That is countable rather than arguable.

## How this was measured

`releases/first-release.md` -> *Units and structures* is a matrix: one row per kind, one column per
capability, a non-blank cell meaning the kind has it. **Membership is read from which cells are
filled**, over the nine columns after *Thing*. Re-runnable from the table; no judgement in the
extraction.

**Two instruments were wrong first and are named so the number is not trusted twice.** The first
expanded traits whose carrier is self-referential - *a thing with upkeep* - into every kind, so
everything nested and it reported **0 crossings**. The second dropped every continuation row of the
*Recipes* table, so each recipe contributed only its first ingredient. Both returned plausible
numbers about the wrong question, which is `C-28`'s shape twice in one measurement.

## The result

Nine capability columns, **six distinct sets**, 36 pairs, **five of them cross-cutting**.

| Capability           | Kinds that have it                             |
| -------------------- | ---------------------------------------------- |
| **Costs to produce** | ark, extractor, garrison, pioneer, store, yard |
| **Binding**          | ark, extractor, garrison, pioneer, store, yard |
| **Force**            | ark, citizen, garrison, pioneer                |
| **Readies**          | ark, citizen, extractor, pioneer               |
| **Fuel**             | ark, pioneer                                   |
| **A move**           | ark, pioneer                                   |
| **Crosses**          | ark, pioneer                                   |
| **Upkeep**           | citizen                                        |
| **Requires**         | ark                                            |

**The clean counterexample is *Force* against *Readies*.** They share ark, citizen and pioneer;
**garrison has force and does not ready**; **extractor readies and has no force.** Neither set
contains the other.

**And it is not one accident.** Take *is built* - the set both *Binding* and *Costs to produce* name
- against those two, and three kinds each carry a different pair:

- **citizen** has force and readies, and is **not built**
- **garrison** is built and has force, and **does not ready**
- **extractor** is built and readies, and has **no force**
- **ark and pioneer** have all three

**That is a lattice and not a tree.** No single-inheritance hierarchy places citizen, garrison and
extractor consistently, because each is missing a different one of the three.

## What this does and does not decide

**It eliminates one family of answers on evidence already in the tree.** A hierarchy of kinds - the
object-oriented answer - is refuted by the release as it stands, not by a future the game might have.

**It does not choose among what survives.** Structural typing, a trait a thing carries, and a
parameter over a set of kinds all express a lattice. **Nothing measured here distinguishes them**,
so the data is sufficient to rule out and insufficient to select.

**And the surviving answer is already in use, which is the part worth noticing.** *Units and
structures* **is** a capability matrix: a column per capability, a blank meaning *does not have it*.
The release has been representing cross-cutting sets as data since it was written. So the open
question is narrower than the paradigms make it sound - **whether a capability that already exists
as a column should also be a named trait**, rather than which paradigm to adopt.

## Two things the measurement found that nobody was looking for

**Three columns name one set.** *Fuel*, *A move* and *Crosses* all hold exactly `ark, pioneer`.
`P-346` deletes *A move*, taking it to two. **This is the duplication Sean said he wants to refactor
as aggressively as in a programming language**, and it is measurable rather than anticipated.

**Two more name another.** *Binding* and *Costs to produce* hold the same six kinds - *what is
built* - under two names.

**So of nine columns, six sets.** Whatever the representation ends up being, **it has three existing
duplications to justify itself against**, which is what the earlier note asked for when it said an
abstraction should be invented after its instances rather than before them.

## Does one of them fit an ECS? Yes, and the tree is already half-way there

**Asked 2026-09-07.** The correspondence is one-to-one with what was measured above.

| ECS           | Here                                                 |
| ------------- | ---------------------------------------------------- |
| **Entity**    | a thing, which carries an `id` when it must be named |
| **Component** | a capability - a column above, or a trait            |
| **System**    | a transformation - a recipe                          |

**Cross-cutting sets are the problem ECS exists to solve.** *Force* against *Readies* - garrison has
force and never readies, extractor readies with no force - is the textbook case for components over
inheritance. **So the option that fits is the same one the measurement left standing**, and the
option ECS was invented to escape is the one already refuted.

**The model is further along than the release.** `crates/game-model/src/thing.rs:247` is
`Thing { kind, traits: BTreeMap<Trait, u32> }` - **components per entity, not per kind.** The
release's *Units and structures* describes capabilities **per kind**, which is an archetype table.
So a `movable` trait is not a new paradigm; it is **the release's description catching up with what
the model already is.**

**The world's recipes are already systems.** They fire when the turn ends, over everything that
matches - `upkeep`, `grow`, `perish`, `age`, `spoil`, `refresh`. The player's recipes are commands
against a chosen place, which is an ECS command buffer rather than a system.

**And an ECS is already in the tree**, so the vocabulary would be shared rather than imported: Bevy
is a dependency of `game4x`, `game-globe`, `planet-bevy` and `planet-flat`, and there is a
`planet-ecs` crate.

### Three places it does not fit, and the third is the one that matters

**Containment is a tree, and ECS is flat.** `spec/logistics.md` makes every thing in the game and the
game in nothing, and `spec/console.md` says *where a thing is, is where it appears*. **Parent-child
containment is the awkward part of every ECS**, expressed as a parent component and a lot of care.
This is real friction rather than a detail.

**Identity survives a move in an ECS and does not here.** An ECS mutates a location component and
keeps the entity. The release's `move` **consumes** a unit at `$from` and **produces** one at `$to`,
so a thing carrying an `id` is destroyed and another made. **Sean's *same-thing-with-one-less-energy*
is the ECS-natural form**, and `P-354`'s second question is exactly this - which is why it is the
half that changes the game.

**Systems are code and recipes must stay data.** `spec/invariants.md`: *every kind of thing, and
every recipe that turns some things into others, is data rather than code.* **An ECS system is
compiled; a recipe is interpreted**, and it must stay so, because Sean wants a player to write rules
without programming. **So the entity and component halves fit and the system half does not** - and
adopting the word *system* in the specification would import the half that contradicts an invariant.

**Which makes the answer narrower than the question.** Components yes, and the release should say
`movable` where it now says *which family a kind is in*. Systems no, and recipes stay data.

## The example that prompted the question does not decide it

Sean's own pair - *things that can move* against *things that require food* - **does not cross-cut
today.** Moving is `ark, pioneer`; upkeep is `citizen`. **Disjoint**, so that pair alone would have
justified a hierarchy. The evidence for the lattice comes from force, readying and being built
instead.
