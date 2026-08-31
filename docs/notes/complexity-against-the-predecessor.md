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

**That is three of the ten extra types** - and **the count above is wrong, which Sean caught within
the hour.** See below.

## Correction: none of the three needs a type

Sean, on adjacency: *I think my original idea was to have a list of territory pairs. We could still do
that, the extra advantage we have is that the pairs now have the formula of the goldberg polyhedron.*

**A pair is a thing.** `{adjacent from=1 to=2}` is two attributes, and a planet's adjacency is a set
of those rows sitting at the planet rather than in a territory. **The predecessor's shape holds it
with no new concept**, so long as things may live at the planet level as well as in a land - which
`Universe -> Planet -> Land` already allows.

**Follow that and the other two go the same way.** *Location* is an attribute on a unit, whose value
is a territory or orbit. *Phase* is an attribute on the game. Neither is a kind, a container or a
variant type; both are values a thing carries.

**So the honest count is that ten of the ten extra types are non-unification**, not seven. The
*capability* is real - this game has geography and travel and a design phase, and the predecessor had
none of them - but **capability did not require a single one of the extra types.** It required extra
*attributes*, and attributes are free in a model that has them.

**And adjacency is better than free: it is derived.** The Goldberg tessellation *generates* the pairs,
so nothing authors them and nothing can author them wrongly. That puts adjacency in the same category
as *node, unworked* - computed from what is stored rather than stored. An implementation may cache it,
and `Game.adjacency` does, but the model does not have to hold it as a fact.

**What the correction changes about the conclusion.** The previous version said adjacency, location
and phase were worth paying for. They are worth *having*; they were never worth *paying for*. The bill
is entirely the five below plus these three, and every line of it is the same missing decision.



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

## What the unified shape would actually be

Sketched to see the size of it rather than to propose it.

| Concept          | Today                                              | Unified                                                  |
| ---------------- | -------------------------------------------------- | -------------------------------------------------------- |
| structures       | `Option<Garrison>`, `Vec<Extractor>`, `yards: u32` | rows in the multiset                                     |
| resources        | `stores: [u32; 3]`                                 | rows in the multiset                                     |
| citizens         | `citizens: u32` + `labor_spent: u32`               | `(citizen ready) x n`, `(citizen exhausted) x m`         |
| units            | `Vec<Unit>` with a location                        | rows with a `location` attribute                         |
| nodes            | `Vec<Node>`                                        | rows                                                     |
| adjacency        | `Vec<Vec<TerritoryId>>`                            | `{adjacent from to}` rows, derived from the tessellation |
| phase, turn      | fields on `Game`                                   | attributes, or read from the history                     |
| `founded`, `won` | fields                                             | derived, as `P-125` and the notes already say            |

**What is left is four ideas**: a **thing** (a bag of attributes), an **attribute** (a name and a
value), a **multiset** (a thing and how many), and a **place** (universe, planet, territory, orbit).
Against fifteen types.

**Three things this does not get for free**, and they are the honest cost:

- **Static typing.** `citizens: u32` cannot be confused with `yards: u32` today only because they are
  different fields; in a multiset both are rows and the checking moves to a validating loader -
  which is what Sean has already chosen, and `prototypes/kinds` is the first instance of it.
- **Cheap lookup.** *How much metal is here* is a field access now and a scan later. Real, and it is
  the reason `Game.adjacency` is cached; it is a decision about representation rather than about the
  model.
- **A place for a value that is not a thing.** Force of nature is a number belonging to the ground.
  As an attribute of a territory it is fine, which means *place* has to be able to carry attributes
  and not only contents.

**Nothing here is proposed.** `P-130` already decided that kinds and transformations are data, and
this is the same decision applied to *state* rather than to *rules*. Whether that is worth doing to a
model that works is Sean's, and the answer probably depends on whether the two halves being different
shapes turns out to hurt.
