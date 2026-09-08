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

**Systems are code and recipes are data - and Sean corrected me here, rightly.** I wrote that the
system half does not fit, because `spec/invariants.md` says *every kind of thing, and every recipe
that turns some things into others, is data rather than code*. **He answered: thin systems that
crunch the recipes.** That is not a contradiction and my objection was too strong.

**The invariant forbids a rule being code, not the engine.** A handful of generic systems that read
recipe rows and apply them is engine; the recipes stay data and a player can still write one. **The
number of systems stays fixed as the number of recipes grows**, which is the test that tells the two
apart. The earlier backlog entry saw this coming - *a thin engine mapping data to visuals becomes an
interpreter somewhere along that line* - and he has said he wants the interpreter.

## What *the correct traits-vs-alternatives decisions* actually are

Sean: *as long as I make the correct traits-vs-alternatives decisions.* Three, and **only the third
needs machinery the release does not have.**

**1. Selecting by trait rather than by family - already expressible, no change to the grammar.**
The *Recipes* preamble says **Kind** is *the kind or the family alone* and **Traits** are *the
constraints on it*. `thing` is a family covering every kind. So

> `consume 1 thing movable ready` at `$from`

is legal today: `thing` in the Kind cell, `movable` in the Traits cell. **His formulation needs a
`movable` row in *Traits* and two cells changed in `move`** - and nothing else.

**2. A marker against a valued component - a convention, not machinery.** `Thing.traits` is
`BTreeMap<Trait, u32>`, so every trait carries a number and a component that means only *present* has
to encode it. **`ready` already does this**, taking *yes or no* into a `u32`. So a marker is
expressible and the decision is only whether to say so once rather than per trait.

**3. Identity across a move - this one has no mechanism.** The four roles are `require`, `limit`,
`consume` and `produce`. **None of them relocates a thing.** *Same-thing-with-one-less-energy* cannot
be said with what the table has, so it needs either a fifth role or a stated convention that
consuming and producing one kind in one recipe preserves the thing. **That is the decision with a
cost**, and it is `P-354`'s second question.

**So the fit is cheap and the identity is not.** Components cost a row and two cells. Keeping a
thing's identity across a move costs a new piece of the language, which is exactly the kind of thing
that should be decided deliberately rather than arrived at.

## Going over the reasoning, 2026-09-07

Sean gave his leaning on each of the three and said **the reasoning matters more than the answers.**
Two hold and are stronger than he put them. **The third is sound about the risk and points the
opposite way.**

### 1. Trait, *because recipes are easier to maintain when I don't know what recipes I will add*

**The argument is right, and the reason is sharper than *easier*.** The two mechanisms differ in
**where membership lives**. A family centralises it: `unit` is a row listing `ark, pioneer`. A trait
distributes it: each thing says what it has.

**Under unknown future recipes you also do not know what families will exist** - and every new family
is a new row that must be filled by revisiting every kind. **A trait costs one edit where the kind is
declared**, whatever sets later turn out to matter. That asymmetry is the whole argument and it holds.

**The measurement supports it more strongly than the argument does.** The release **already has both
mechanisms**, and they already duplicate: `unit` names `{ark, pioneer}` and so do the *Fuel*, *A
move* and *Crosses* columns. **Three names for one set, and a fourth in the Families table.** The
duplication he wants to refactor away is **caused by keeping both**, so choosing one collapses it.

**The one real cost, and it is already paid.** A family answers *what is a unit?* at a glance; traits
make you scan every thing. **`R-8` already computes exactly that** - a signature per kind derived
from the tables - so the answer is generated rather than read off a row.

### 2. A number, *because it is not durable-or-not, it is how-durable*

**His example is already in the release.** `keeps` is *the number of turns it will last* - how
durable, carrying a number, today.

**And markers are the minority, counted rather than assumed.** Of the nineteen traits: **5 are plain
numbers, 3 are yes-or-no, and 11 are neither.** So a trait system built around presence would be
built around the smallest of the three groups.

**But the question I put to him was too narrow, and that is my error rather than his.** I offered
*number or marker*. The release has **three** value shapes: a number, yes-or-no, and **one of a
closed set** - `kind`, `resource`, `biome`, `control`, `phase`, and the references `from` and `to`.
**The model flattens all of them into `u32`.** So the live question is not whether a trait carries a
number; it is **what a trait's value may be**, and *a number* is already not the whole answer.

**One consequence of answering 1 and 2 together.** If movability is a trait and traits carry values,
then **`movable` wants a value**, and the natural one is his own reasoning applied to movement: not
*can it move* but *how far*, or *how many moves a turn*. That is a question his two leanings raise
rather than one either answers.

### 3. Torn - *simplicity of destroy-then-create with same identity*, against *chaos at a million objects*

**The worry is well founded and the conclusion inverts.** Three things, and the first is the one that
decides it.

**The model already aggregates, so identity is what creates the million.** `spec/console.md`: *each
distinct description is its own entry*, and **a thing carrying an `id` has a description no other
thing shares, so its quantity is always one.** So `{citizen ready:yes} -> 8` is **one entry, not
eight things**. Individuation is caused by carrying an id. **Preserving identity across a move is
therefore what forces a million movers to exist as a million entries** - and consume-and-produce over
aggregates lets eight citizens move as one entry with a count. **The unification he wants is the
thing that creates the scale problem he fears.**

**And in an ECS, mutation is the cheap path.** Changing a location component is a write; destroying
and creating is an allocation and an archetype move. **So at a million movers he wants mutation** -
which means a thin system has to *know* it may mutate. **A fifth role is what tells it.** Without
one, the system either takes the expensive path always, or **infers relocation from the shape of a
consume-and-produce pair** - and inferring intent from shape is the failure this repository records
most often. So the fifth role is not the machinery that risks chaos; **it is what avoids it.**

**Which makes decision 3 narrower than it looks.** Identity only means something for a thing that
carries an `id`, and **no unit carries one today** - no ark or pioneer appears in the played state at
all, and the only ids there belong to orbits. **So the real question is: does a unit ever need to be
named individually?** If no, consume-and-produce is right and nothing is needed. If yes, that is
where the fifth role earns itself, and only for the things that carry ids.

## The example that prompted the question does not decide it

Sean's own pair - *things that can move* against *things that require food* - **does not cross-cut
today.** Moving is `ark, pioneer`; upkeep is `citizen`. **Disjoint**, so that pair alone would have
justified a hierarchy. The evidence for the lattice comes from force, readying and being built
instead.
