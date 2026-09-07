# Catalog

**Generated. Do not edit.** `cargo run -p kinds -- catalog`, or `scripts/kinds.sh catalog`.

Every kind the release declares, with everything it says about that kind gathered in one place.
`spec/invariants.md` has the release's tables be the data, and every other form of them derived
and generated rather than written; this is one such form. It is a view and not a copy - each
section is a join across six tables that the document does not perform anywhere.

15 kinds, 4 families, 19 traits, 16 recipes.

## citizen

a person: provides labor, eats, and grows on surplus.

**In families** thing

**Traits of it** `kind` (one of the kinds), `force` (a number)

**Bounded by** the food produced here, through upkeep

**As a thing** Force: 1 · Upkeep: 1 food per turn · Readies: yes

**In recipes**

- `deploy ark` produces 2
- `found by land` produces 2
- `produce pioneer` consumes 2
- `launch ark` consumes 2
- `create labor` consumes 1, ready
- `create labor` produces 1, not ready
- `grow` produces the lesser of the surplus food and the citizens here

## garrison

what holds a territory; a territory has at most one.

**In families** thing

**Traits of it** `kind` (one of the kinds), `force` (a number)

**Bounded by** a capacity of 1

**As a thing** Force: 0 · Costs to produce: 1 labor, 1 metal · Binding: 1

**In recipes**

- `deploy ark` limits 0
- `deploy ark` produces 1
- `found by land` limits 0
- `found by land` produces 1

## extractor

built for one resource, and worked to produce it.

**In families** thing

**Traits of it** `kind` (one of the kinds), `resource` (one of the resources)

**Bounded by** a capacity, from *Territory resources*

**As a thing** Costs to produce: 1 labor, 1 metal · Binding: 1 · Readies: yes

**In recipes**

- `deploy ark` produces 1, food
- `deploy ark` produces 1, metal
- `found by land` produces 1, food
- `found by land` produces 1, metal
- `build extractor` produces 1, `$resource`
- `work` consumes 1, ready
- `work` produces 1, not ready

## yard

where an Ark is produced.

**In families** thing

**Traits of it** `kind` (one of the kinds)

**Bounded by** a capacity of 1

**As a thing** Costs to produce: 1 labor, 15 metal · Binding: 15

**In recipes**

- `build yard` produces 1
- `launch ark` requires 1

## store

built to hold one resource, and holds nothing else.

**In families** thing

**Traits of it** `kind` (one of the kinds), `resource` (one of the resources)

**Bounded by** as many as the extractors of its resource

**As a thing** Costs to produce: 1 labor, 1 metal · Binding: 1

**Holds** the resource it was built for, up to 10 - *a fact about the kind, so every one of them holds that many*

**In recipes**

- `deploy ark` produces 1, food
- `deploy ark` produces 1, metal
- `found by land` produces 1, food
- `found by land` produces 1, metal
- `build store` produces 1, `$resource`

## ark

carries a landing, and can invade from orbit.

**In families** thing, unit

**Traits of it** `kind` (one of the kinds), `force` (a number)

**Bounded by** a capacity of 2

**As a thing** Force: 2 · Fuel: 2 · A move: 1 fuel · Costs to produce: 3 metal, 12 energy, 2 citizens · Binding: 3 · Crosses: orbit border · Requires: a Yard · Readies: yes

**Holds** energy, up to the unit's fuel - *a fact about each one rather than about the kind*

**In recipes**

- `deploy ark` consumes 1, in the orbit above `$where`
- `move` consumes 1 (as a unit), ready, in `$from`
- `move` produces 1 (as a unit), not ready, in `$to`

## pioneer

founds a territory.

**In families** thing, unit

**Traits of it** `kind` (one of the kinds), `force` (a number)

**Bounded by** a capacity of 2

**As a thing** Force: 2 · Fuel: 2 · A move: 1 fuel · Costs to produce: 3 metal, 6 energy, 2 citizens · Binding: 3 · Crosses: border · Readies: yes

**Holds** energy, up to the unit's fuel - *a fact about each one rather than about the kind*

**In recipes**

- `move` consumes 1 (as a unit), ready, in `$from`
- `move` produces 1 (as a unit), not ready, in `$to`
- `found by land` consumes 1
- `produce pioneer` produces 1

## food

eaten by citizens; expires.

**In families** thing, resource

**Traits of it** `kind` (one of the kinds), `surplus` (yes or no)

**Bounded by** the things in it that hold it, and it keeps for one turn

**In recipes**

- `work` produces `$where`'s density for that resource (as a resource)
- `upkeep` consumes the thing's upkeep
- `grow` consumes the lesser of the surplus food and the citizens here, surplus

## metal

what things are built from; conserved.

**In families** thing, resource

**Traits of it** `kind` (one of the kinds)

**Bounded by** the things in it that hold it

**In recipes**

- `build extractor` consumes 1
- `build store` consumes 1
- `build yard` consumes 15
- `produce pioneer` consumes 3
- `launch ark` consumes 3
- `work` produces `$where`'s density for that resource (as a resource)
- `perish` produces the thing's metal

## energy

what moves things; neither conserved nor expiring.

**In families** thing, resource

**Traits of it** `kind` (one of the kinds)

**Bounded by** the things in it that hold it

**In recipes**

- `move` consumes 1, in that unit
- `produce pioneer` consumes 6
- `launch ark` consumes 12
- `work` produces `$where`'s density for that resource (as a resource)

## labor

what working a machine takes; a citizen provides it each turn.

**In families** thing

**Traits of it** `kind` (one of the kinds)

**Bounded by** the citizens that make it, one each per turn

**In recipes**

- `build extractor` consumes 1
- `build store` consumes 1
- `build yard` consumes 1
- `create labor` produces 1
- `work` consumes 1

## territory

a place things are in, which has a biome, a force of nature, and a density and a total capacity per resource.

**In families** thing, place

**Traits of it** `kind` (one of the kinds), `control` (held by a player, or unclaimed), `biome` (one of the biomes), `nature` (a number)

**Holds** that kind, up to its total capacity for that kind - *a fact about each one rather than about the kind*

**In recipes**

- `deploy ark` requires 1, in `$where`
- `move` requires 1 (as a place), in `$from`
- `move` requires 1 (as a place), joined to `$from` by an edge the unit crosses, in `$to`
- `work` requires 1, in `$where`

## orbit

a place above one territory, which holds units and nothing else.

**In families** thing, place

**Traits of it** `kind` (one of the kinds)

**In recipes**

- `deploy ark` consumes 1 (as the place holding ark), in the orbit above `$where`
- `move` requires 1 (as a place), in `$from`
- `move` requires 1 (as a place), joined to `$from` by an edge the unit crosses, in `$to`

## deposit

what a territory's ground offers of one resource, and how richly.

**In families** thing

**Traits of it** `kind` (one of the kinds), `density` (a number), `total capacity` (a number)

**In recipes** none name it.

## adjacency

two places that share an edge, held by the thing that holds them.

**In families** thing

**Traits of it** `kind` (one of the kinds), `from` (a place), `to` (a place)

**In recipes** none name it.

