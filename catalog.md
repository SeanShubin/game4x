# Catalog

**Generated. Do not edit.** `cargo run -p kinds -- catalog`, or `scripts/kinds.sh catalog`.

Every kind the release declares, with everything it says about that kind gathered in one place.
`spec/invariants.md` has the release's tables be the data, and every other form of them derived
and generated rather than written; this is one such form. It is a view and not a copy - each
section is a join across six tables that the document does not perform anywhere.

14 kinds, 5 families, 17 traits, 17 recipes.

## citizen

a person: provides labor, eats, and grows on surplus.

**In families** thing

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in), `force` (a number)

**Bounded by** the food produced here, through upkeep

**As a thing** Force: 1 · Upkeep: 1 food per turn · Readies: yes

**In recipes**

- `deploy ark` produces 2
- `found by land` produces 2
- `produce pioneer` consumes 2
- `produce ark` consumes 2
- `create labor` consumes 1, ready
- `create labor` produces 1, not ready
- `grow` produces 1

## garrison

what holds a territory; a territory has at most one.

**In families** thing

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in), `force` (a number)

**Bounded by** a capacity of 1

**As a thing** Force: 1 · Costs to produce: 1 labor, 1 metal · Binding: 1

**In recipes**

- `deploy ark` limits 0
- `deploy ark` produces 1
- `found by land` limits 0
- `found by land` produces 1

## food extractor

built for food, and worked to produce it.

**In families** thing, extractor

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in)

**Bounded by** a capacity, from *Territory resources*

**As a thing** Costs to produce: 1 labor, 1 metal · Binding: 1 · Readies: yes

**In recipes**

- `deploy ark` produces 1
- `found by land` produces 1
- `build food extractor` produces 1
- `work` consumes 1 (as a extractor), ready
- `work` produces 1 (as a extractor), not ready

## metal extractor

built for metal, and worked to produce it.

**In families** thing, extractor

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in)

**Bounded by** a capacity, from *Territory resources*

**As a thing** Costs to produce: 1 labor, 1 metal · Binding: 1 · Readies: yes

**In recipes**

- `deploy ark` produces 1
- `found by land` produces 1
- `build metal extractor` produces 1
- `work` consumes 1 (as a extractor), ready
- `work` produces 1 (as a extractor), not ready

## energy extractor

built for energy, and worked to produce it.

**In families** thing, extractor

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in)

**Bounded by** a capacity, from *Territory resources*

**As a thing** Costs to produce: 1 labor, 1 metal · Binding: 1 · Readies: yes

**In recipes**

- `build energy extractor` produces 1
- `work` consumes 1 (as a extractor), ready
- `work` produces 1 (as a extractor), not ready

## yard

where an Ark is produced.

**In families** thing

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in)

**Bounded by** a capacity of 1

**As a thing** Costs to produce: 1 labor, 15 metal · Binding: 15

**In recipes**

- `build yard` produces 1
- `produce ark` requires 1

## ark

carries a landing, and can invade from orbit.

**In families** thing, unit

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in), `force` (a number)

**Bounded by** a capacity of 2

**As a thing** Force: 2 · Fuel: 2 · A move: 1 fuel · Costs to produce: 3 metal, 12 energy, 2 citizens · Binding: 3 · Crosses: orbit border, ascent · Requires: a Yard · Readies: yes

**In recipes**

- `deploy ark` consumes 1, in the orbit above `$where`
- `move` consumes 1 (as a unit), ready, in `$from`
- `move` produces 1 (as a unit), not ready, in `$to`
- `produce ark` produces 1

## pioneer

founds a territory.

**In families** thing, unit

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in), `force` (a number)

**Bounded by** a capacity of 2, and the food produced here

**As a thing** Force: 2 · Fuel: 2 · A move: 1 fuel · Upkeep: 1 food per turn · Costs to produce: 3 metal, 6 energy, 2 citizens · Binding: 3 · Crosses: border · Readies: yes

**In recipes**

- `move` consumes 1 (as a unit), ready, in `$from`
- `move` produces 1 (as a unit), not ready, in `$to`
- `found by land` consumes 1
- `produce pioneer` produces 1

## food

eaten by citizens; expires.

**In families** thing, resource

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in), `keeps` (the number of turns it will last), `surplus` (yes or no)

**Bounded by** a capacity of 20, and it keeps for one turn

**In recipes**

- `work` produces `$where`'s density for that resource (as a resource)
- `upkeep` consumes the thing's upkeep
- `grow` consumes 1, surplus
- `spoil` consumes 1, keeps 0
- `age` consumes 1, keeps at least 1
- `age` produces 1, keeps one less

## metal

what things are built from; conserved.

**In families** thing, resource

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in)

**Bounded by** a capacity of 20

**In recipes**

- `build food extractor` consumes 1
- `build metal extractor` consumes 1
- `build energy extractor` consumes 1
- `build yard` consumes 15
- `produce pioneer` consumes 3
- `produce ark` consumes 3
- `work` produces `$where`'s density for that resource (as a resource)
- `perish` produces the thing's metal

## energy

what moves things; neither conserved nor expiring.

**In families** thing, resource

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in)

**Bounded by** a capacity of 20

**In recipes**

- `move` consumes 1, in that unit
- `produce pioneer` consumes 6
- `produce ark` consumes 12
- `work` produces `$where`'s density for that resource (as a resource)

## labor

what working a machine takes; a citizen provides it each turn.

**In families** thing

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in)

**Bounded by** the citizens that make it, one each per turn

**In recipes**

- `build food extractor` consumes 1
- `build metal extractor` consumes 1
- `build energy extractor` consumes 1
- `build yard` consumes 1
- `create labor` produces 1
- `work` consumes 1

## territory

a place things are in, which has a biome, a force of nature, and a density and a total capacity per resource.

**In families** thing, place

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in), `density` (a number), `total capacity` (a number), `control` (held by a player, or unclaimed), `biome` (one of the biomes), `force of nature` (a number)

**In recipes**

- `deploy ark` requires 1, in `$where`
- `move` requires 1 (as a place), in `$from`
- `move` requires 1 (as a place), joined to `$from` by an edge the unit crosses, in `$to`
- `work` requires 1, in `$where`

## orbit

a place above one territory, which holds units and nothing else.

**In families** thing, place

**Traits of it** `kind` (one of the kinds), `place` (the thing it is in)

**In recipes**

- `deploy ark` consumes 1 (as the place holding ark), in the orbit above `$where`
- `move` requires 1 (as a place), in `$from`
- `move` requires 1 (as a place), joined to `$from` by an edge the unit crosses, in `$to`

