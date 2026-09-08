# Catalog

**Generated. Do not edit.** `cargo run -p kinds -- catalog`, or `scripts/kinds.sh catalog`.

Every kind the release declares, with everything it says about that kind gathered in one place.
`spec/invariants.md` has the release's tables be the data, and every other form of them derived
and generated rather than written; this is one such form. It is a view and not a copy - each
section is a join across six tables that the document does not perform anywhere.

16 kinds, 4 families, 20 traits, 16 recipes.

## Signatures

**A kind's signature is the traits it carries and every *(recipe, role)* pair that names it.**
Computed from the tables above rather than written by anyone, so two kinds share one exactly
when the release says the same things about them. Quantities are not part of it: two kinds that
are produced in different numbers by the same recipe still behave alike. **Being named through a
family counts**, because a family is how the release addresses several kinds at once.

16 kinds fall into 16 signatures, and 0 of those hold more than one kind.

**Nothing is shown together, and that is the finding.** No two kinds share a signature, so
every group below holds one kind. **The traits alone do collide** - 11 of the kinds carry
exactly the traits another one carries - and every such pair is then separated by the recipes
that name it. So the release has no two kinds it says *the same things* about, and whether that
is what was wanted is a decision rather than a build: `C-64`.

### `s-1` - citizen

**Traits** `force`, `kind`

**Named by** `create labor consume`, `create labor produce`, `deploy ark produce`, `found by land produce`, `grow produce`, `launch ark consume`, `produce pioneer consume`

### `s-2` - garrison

**Traits** `force`, `kind`

**Named by** `deploy ark limit`, `deploy ark produce`, `found by land limit`, `found by land produce`

### `s-3` - extractor

**Traits** `kind`, `resource`

**Named by** `build extractor produce`, `deploy ark produce`, `found by land produce`, `work consume`, `work produce`

### `s-4` - yard

**Traits** `kind`

**Named by** `build yard produce`, `launch ark require`

### `s-5` - store

**Traits** `kind`, `resource`

**Named by** `build store produce`, `deploy ark produce`, `found by land produce`

### `s-6` - ark

**Traits** `force`, `kind`

**Named by** `deploy ark consume`, `move consume`, `move produce`

### `s-7` - pioneer

**Traits** `force`, `kind`

**Named by** `found by land consume`, `move consume`, `move produce`, `produce pioneer produce`

### `s-8` - food

**Traits** `kind`, `surplus`

**Named by** `grow consume`, `upkeep consume`, `work produce`

### `s-9` - metal

**Traits** `kind`

**Named by** `build extractor consume`, `build store consume`, `build yard consume`, `launch ark consume`, `perish produce`, `produce pioneer consume`, `work produce`

### `s-10` - energy

**Traits** `kind`

**Named by** `launch ark consume`, `move consume`, `produce pioneer consume`, `work produce`

### `s-11` - labor

**Traits** `kind`

**Named by** `build extractor consume`, `build store consume`, `build yard consume`, `create labor produce`, `work consume`

### `s-12` - territory

**Traits** `biome`, `control`, `kind`, `nature`

**Named by** `deploy ark require`, `move require`, `work require`

### `s-13` - orbit

**Traits** `kind`

**Named by** `deploy ark consume`, `move require`

### `s-14` - deposit

**Traits** `density`, `kind`, `total capacity`

**Named by** no recipe at all.

### `s-15` - adjacency

**Traits** `from`, `kind`, `to`

**Named by** no recipe at all.

### `s-16` - game

**Traits** `kind`, `phase`

**Named by** no recipe at all.

## citizen

a person: provides labor, eats, and grows on surplus.

**In families** thing

**Traits of it** `kind` (one of the kinds), `force` (a number)

**Signature** `s-1`

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

**Signature** `s-2`

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

**Signature** `s-3`

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

**Signature** `s-4`

**Bounded by** a capacity of 1

**As a thing** Costs to produce: 1 labor, 15 metal · Binding: 15

**In recipes**

- `build yard` produces 1
- `launch ark` requires 1

## store

built to hold one resource, and holds nothing else.

**In families** thing

**Traits of it** `kind` (one of the kinds), `resource` (one of the resources)

**Signature** `s-5`

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

**Signature** `s-6`

**Bounded by** a capacity of 2

**As a thing** Force: 2 · Fuel: 2 · Costs to produce: 3 metal, 12 energy, 2 citizens · Binding: 3 · Crosses: orbit border · Requires: a Yard · Readies: yes · Movable: yes

**Holds** energy, up to the unit's fuel - *a fact about each one rather than about the kind*

**In recipes**

- `deploy ark` consumes 1, in the orbit above `$where`
- `move` consumes 1 (as a unit), ready, in `$from`
- `move` produces 1 (as a unit), not ready, in `$to`

## pioneer

founds a territory.

**In families** thing, unit

**Traits of it** `kind` (one of the kinds), `force` (a number)

**Signature** `s-7`

**Bounded by** a capacity of 2

**As a thing** Force: 2 · Fuel: 2 · Costs to produce: 3 metal, 6 energy, 2 citizens · Binding: 3 · Crosses: border · Readies: yes · Movable: yes

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

**Signature** `s-8`

**Bounded by** the things in it that hold it, and it keeps for one turn

**In recipes**

- `work` produces `$where`'s density for that resource (as a resource)
- `upkeep` consumes the thing's upkeep
- `grow` consumes the lesser of the surplus food and the citizens here, surplus

## metal

what things are built from; conserved.

**In families** thing, resource

**Traits of it** `kind` (one of the kinds)

**Signature** `s-9`

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

**Signature** `s-10`

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

**Signature** `s-11`

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

**Signature** `s-12`

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

**Signature** `s-13`

**In recipes**

- `deploy ark` consumes 1 (as the place holding ark), in the orbit above `$where`
- `move` requires 1 (as a place), in `$from`
- `move` requires 1 (as a place), joined to `$from` by an edge the unit crosses, in `$to`

## deposit

what a territory's ground offers of one resource, and how richly.

**In families** thing

**Traits of it** `kind` (one of the kinds), `density` (a number), `total capacity` (a number)

**Signature** `s-14`

**In recipes** none name it.

## adjacency

two places that share an edge, held by the thing that holds them.

**In families** thing

**Traits of it** `kind` (one of the kinds), `from` (a place), `to` (a place)

**Signature** `s-15`

**In recipes** none name it.

## game

every thing is in it, and it is the one thing that is in nothing.

**In families** thing

**Traits of it** `kind` (one of the kinds), `phase` (design or play)

**Signature** `s-16`

**In recipes** none name it.

