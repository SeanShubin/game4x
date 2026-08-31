# Complexity Against the Predecessor

**Derived.** Written by Claude from conversation, 2026-08-31. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [The game as tables](the-game-as-tables.md) · [Everything is matter](everything-is-matter.md)

Sean: *this structure is much more complicated than the predecessor. Let's explore if that is because
it is more capable, or if it has failed to unify concepts where the predecessor succeeded.*

**Both, and they separate cleanly.** Counted rather than judged: `game-model` declares **fifteen**
types - eight structs and seven enums. The predecessor's game module declares **five** - `Thing`,
`Attribute`, `Land`, `Planet`, `Universe`.

## Three places the extra weight buys something

**Adjacency.** The predecessor's `Planet` is a list of `Land` and nothing says which touches which.
There is no geography, so there is nothing to cross, and *move*, *found*, *reach* and *supply* cannot
exist. This game's whole subject is a sphere divided into territories that touch, so a graph is not
avoidable.

**Location, and therefore movement.** `Unit` carries `Location::On(t) | Orbit`. The predecessor's
things are *in* a `Land` and cannot be anywhere else, so nothing can travel. Orbit alone needs it.

**Phase.** Designing a world and playing it are different, and `spec/invariants.md` says which phase a
game is in is part of its state. The predecessor had no design phase.

**That is three of the ten extra types**, and none of them is a failure to unify.

## Five places it is complexity for nothing

**Structures come in three shapes.** `garrison: Option<Garrison>`, `extractors: Vec<Extractor>`,
`yards: u32`. One is optional, one is a list with identity, one is a bare count - **three
representations of "a structure standing in a territory"**, and the difference between them is
history rather than meaning. The predecessor holds all three as rows in one multiset.

**Resources are a fixed array.** `stores: [u32; 3]`, indexed by an enum. Adding a resource is a type
change. In the predecessor a resource is a `Thing` like anything else and adding one is data.

**Citizens are a bare count, and that is what forced `labor_spent` into existence.** `citizens: u32`
cannot carry a trait, so *this citizen has worked and that one has not* has nowhere to live - hence a
second field counting labor spent, sitting beside the population it describes and able to disagree
with it. **The predecessor gets this free**: `(citizen ready) x 5` and `(citizen exhausted) x 3` are
two rows in a multiset, and no counter is needed because the trait is on the thing.

**This is the one worth dwelling on.** A whole day went into ready-and-exhausted - what a *thing* is,
which kinds ready, whether `P-133` should exist. **In the predecessor's shape that question does not
arise**, because readiness is a trait and a trait is just another attribute. The unification that was
lost cost a field, a proposal, and an afternoon.

**Units live in a separate list.** `Game.units` with a location, rather than in the territory they
stand on. So *what is here* is two lookups and two shapes, and the transformation table papers over it
by writing *unit, here* as though it were an ordinary input.

**Three closed enums where one open set would do.** `Resource`, `UnitKind`, `StructureKind` - a new
kind of anything is a code change in three places. This is precisely what `P-130` has now decided
against, so the direction is already set; the note records how much of the weight it accounts for.

## What the comparison actually shows

**The added capability costs three types. The lost unification costs seven.** So the honest answer to
Sean's question is that the structure is more complicated for both reasons, and **the second reason is
the larger one.**

**And the predecessor was not simpler by being smaller.** It was simpler by making one decision -
*everything is a thing with attributes, held in a multiset* - and then not making it again per kind.
Every place this model is heavier is a place that decision was not taken.

**Nothing here argues for going back.** The predecessor could not represent a planet whose territories
touch, which is the whole game. What it argues is that adjacency, location and phase are the parts
worth paying for, and that the other seven types are a bill that `P-130` has already agreed to stop
paying.
