# Catalog

**Generated. Do not edit.** `cargo run -p kinds -- catalog`, or `scripts/kinds.sh catalog`.

Every kind the release declares, with everything it says about that kind gathered in one place.
**This is a derived form of the release's tables** - `spec/invariants.md`: *a fact is stated
once and every other form of it is derived*, and a derived form is generated rather than written.
It is a view and not a copy - each section is a join across six tables that the document does not
perform anywhere.

**The release's tables are not themselves canonical for all of this.** Four of them - Kinds,
Families, Biomes and Traits - have files in `spec/data/`, which is where those facts are stated; the
release's four are a second hand-written form, and `P-465` is about what that costs. This reads
the release because that is where all six tables are today, and says so rather than implying the
release is the source.

18 kinds, 4 families, 24 traits, 31 recipes.

## Signatures

**A kind's signature is the traits it carries and every *(recipe, role)* pair that names it.**
Computed from the tables above rather than written by anyone, so two kinds share one exactly
when the release says the same things about them. Quantities are not part of it: two kinds that
are produced in different numbers by the same recipe still behave alike. **Being named through a
family counts**, because a family is how the release addresses several kinds at once.

**No two of the 18 kinds behave alike**, over all 153 pairs of them.

**Every group below holds one kind**, which is what that sentence means when
you reach them. **The traits alone do collide** - 8 of the kinds carry exactly
the traits another one carries - and every such pair is then separated by the recipes
that name it. So the release has no two kinds it says *the same things* about, and Sean
has accepted that as the answer: he expects a small number of distinct things.

### `s-1` - citizen

**Traits** `bearing`, `defending`, `keeps`, `laboring`, `strength`

**Named by** `age put`, `age require`, `bear put`, `bear require`, `breed produce`, `create labor put`, `create labor require`, `deploy ark produce`, `found by land produce`, `launch ark consume`, `muster put`, `muster require`, `perish consume`, `produce pioneer consume`, `refresh put`, `spoil consume`, `upkeep require`

### `s-2` - garrison

**Traits** `keeps`, `strength`

**Named by** `age put`, `age require`, `deploy ark produce`, `found by land produce`, `muster require`, `spoil consume`

### `s-3` - extractor

**Traits** `keeps`, `resource`, `working`

**Named by** `age put`, `age require`, `build extractor produce`, `deploy ark produce`, `found by land produce`, `refresh put`, `spoil consume`, `work put`, `work require`

### `s-4` - yard

**Traits** `keeps`

**Named by** `age put`, `age require`, `build yard produce`, `launch ark require`, `spoil consume`

### `s-5` - store

**Traits** `keeps`, `resource`

**Named by** `age put`, `age require`, `build store produce`, `spoil consume`, `stow produce`

### `s-6` - ark

**Traits** `fuel`, `keeps`, `moving`, `strength`

**Named by** `age put`, `age require`, `deploy ark consume`, `launch ark produce`, `move put`, `move require`, `refresh put`, `spoil consume`, `stand put`, `stand require`

### `s-7` - pioneer

**Traits** `fuel`, `keeps`, `moving`, `strength`

**Named by** `age put`, `age require`, `found by land consume`, `move put`, `move require`, `produce pioneer produce`, `refresh put`, `spoil consume`, `stand put`, `stand require`

### `s-8` - food

**Traits** `keeps`, `surplus`

**Named by** `age put`, `age require`, `breed consume`, `spoil consume`, `upkeep consume`, `work produce`

### `s-9` - metal

**Traits** `keeps`

**Named by** `age put`, `age require`, `build extractor consume`, `build store consume`, `build yard consume`, `discard consume`, `launch ark consume`, `produce pioneer consume`, `spoil consume`, `stow consume`, `stow produce`, `work produce`

### `s-10` - energy

**Traits** `keeps`

**Named by** `age put`, `age require`, `discard consume`, `launch ark consume`, `move consume`, `produce pioneer consume`, `spoil consume`, `stow consume`, `stow produce`, `work produce`

### `s-11` - labor

**Traits** `keeps`

**Named by** `age put`, `age require`, `build extractor consume`, `build store consume`, `build yard consume`, `create labor produce`, `discard consume`, `spoil consume`, `work consume`

### `s-12` - territory

**Traits** `biome`, `control`, `id`, `keeps`, `nature`

**Named by** `age put`, `age require`, `deploy ark require`, `launch ark require`, `move require`, `spoil consume`, `work require`

### `s-13` - orbit

**Traits** `id`, `keeps`

**Named by** `age put`, `age require`, `deploy ark consume`, `launch ark produce`, `move require`, `spoil consume`

### `s-14` - deposit

**Traits** `density`, `keeps`, `total capacity`

**Named by** `age put`, `age require`, `spoil consume`

### `s-15` - adjacency

**Traits** `from`, `keeps`, `to`

**Named by** `age put`, `age require`, `spoil consume`

### `s-16` - game

**Traits** `keeps`, `phase`

**Named by** `age put`, `age require`, `spoil consume`

### `s-17` - fertility

**Traits** `keeps`

**Named by** `age put`, `age require`, `bear produce`, `breed consume`, `discard consume`, `spoil consume`

### `s-18` - force

**Traits** `keeps`

**Named by** `age put`, `age require`, `discard consume`, `muster produce`, `spoil consume`, `stand produce`

## citizen

a person: provides labor, eats, and grows on surplus.

**In families** thing

**Traits of it** `laboring` (a number), `bearing` (a number), `defending` (a number), `strength` (a number), `keeps` (the number of turns it will last)

**Signature** `s-1`

**Bounded by** the food produced here, through upkeep

**As a thing** Strength: 1 · Upkeep: 1 food per turn · Readies: bearing 1, defending 1, laboring 1

**In recipes**

- `deploy ark` produces 2
- `found by land` produces 2
- `produce pioneer` consumes 2
- `launch ark` consumes 2
- `create labor` requires 1, laboring at least 1
- `create labor` puts , laboring one less
- `upkeep` requires 1
- `bear` requires 1, bearing at least 1
- `bear` puts , bearing one less
- `breed` produces 1
- `perish` consumes 1, whose upkeep is unpaid
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `refresh` puts , laboring at its maximum
- `refresh` puts , bearing at its maximum
- `muster` requires 1, defending at least 1
- `muster` puts , defending one less
- `refresh` puts , defending at its maximum

## garrison

what holds a territory; a territory has at most one.

**In families** thing

**Traits of it** `strength` (a number), `keeps` (the number of turns it will last)

**Signature** `s-2`

**Bounded by** a capacity of 1

**As a thing** Strength: 0

**In recipes**

- `deploy ark` produces 1
- `found by land` produces 1
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `muster` requires 1

## extractor

built for one resource, and worked to produce it.

**In families** thing

**Traits of it** `working` (a number), `resource` (one of the resources), `keeps` (the number of turns it will last)

**Signature** `s-3`

**Bounded by** a capacity, from *Territory resources*

**As a thing** Readies: working 1

**In recipes**

- `deploy ark` produces 1, food
- `deploy ark` produces 1, metal
- `found by land` produces 1, food
- `found by land` produces 1, metal
- `build extractor` produces 1, `$resource`
- `work` requires 1, working at least 1
- `work` puts , working one less
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `refresh` puts , working at its maximum

## yard

where an Ark is produced.

**In families** thing

**Traits of it** `keeps` (the number of turns it will last)

**Signature** `s-4`

**Bounded by** a capacity of 1

**In recipes**

- `build yard` produces 1
- `launch ark` requires 1
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0

## store

built to hold one resource, and holds nothing else.

**In families** thing

**Traits of it** `resource` (one of the resources), `keeps` (the number of turns it will last)

**Signature** `s-5`

**Bounded by** as many as the extractors of its resource

**Holds** the resource it was built for, up to 10 - *a fact about the kind, so every one of them holds that many*

**In recipes**

- `build store` produces 1, `$resource`
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `stow` produces 1 (as the place holding metal), in a store for metal
- `stow` produces 1 (as the place holding energy), in a store for energy

## ark

carries a landing, and can invade from orbit.

**In families** thing, unit

**Traits of it** `moving` (a number), `strength` (a number), `fuel` (how much energy its tank holds), `keeps` (the number of turns it will last)

**Signature** `s-6`

**Bounded by** a capacity of 2

**As a thing** Strength: 2 · Crosses: orbit border · Readies: defending 1, moving 1 · Movable: 1

**Holds** energy, up to the unit's fuel - *a fact about each one rather than about the kind*

**In recipes**

- `deploy ark` consumes 1, in the orbit above `$where`
- `move` requires 1 (as a unit), moving at least 1, in `$from`
- `move` puts  (as a unit), moving one less, in `$to`
- `launch ark` produces 1, in the orbit above `$where`
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `refresh` puts  (as a unit), moving at its maximum
- `stand` requires 1 (as a unit), defending at least 1
- `stand` puts  (as a unit), defending one less
- `refresh` puts  (as a unit), defending at its maximum

## pioneer

founds a territory.

**In families** thing, unit

**Traits of it** `moving` (a number), `strength` (a number), `fuel` (how much energy its tank holds), `keeps` (the number of turns it will last)

**Signature** `s-7`

**Bounded by** a capacity of 2

**As a thing** Strength: 2 · Fuel: 2 · Crosses: border · Readies: defending 1, moving 1 · Movable: 1

**Holds** energy, up to the unit's fuel - *a fact about each one rather than about the kind*

**In recipes**

- `move` requires 1 (as a unit), moving at least 1, in `$from`
- `move` puts  (as a unit), moving one less, in `$to`
- `found by land` consumes 1
- `produce pioneer` produces 1
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `refresh` puts  (as a unit), moving at its maximum
- `stand` requires 1 (as a unit), defending at least 1
- `stand` puts  (as a unit), defending one less
- `refresh` puts  (as a unit), defending at its maximum

## food

eaten by citizens; expires.

**In families** thing, resource

**Traits of it** `keeps` (the number of turns it will last), `surplus` (a number)

**Signature** `s-8`

**Bounded by** the things in it that hold it, and it keeps for one turn

**In recipes**

- `work` produces `$where`'s density for that resource (as a resource)
- `upkeep` consumes 1
- `breed` consumes 1
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0

## metal

what things are built from; drawn from the planet, and conserved once above ground.

**In families** thing, resource

**Traits of it** `keeps` (the number of turns it will last)

**Signature** `s-9`

**Bounded by** the things in it that hold it

**In recipes**

- `build extractor` consumes 1
- `build store` consumes 1
- `build yard` consumes 15
- `produce pioneer` consumes 3
- `launch ark` consumes 3
- `work` produces `$where`'s density for that resource (as a resource)
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `stow` consumes 1
- `stow` produces 1, in a store for metal
- `discard` consumes 1

## energy

what moves things; neither conserved nor expiring.

**In families** thing, resource

**Traits of it** `keeps` (the number of turns it will last)

**Signature** `s-10`

**Bounded by** the things in it that hold it

**In recipes**

- `move` consumes 1, in that unit
- `produce pioneer` consumes 6
- `launch ark` consumes 12
- `work` produces `$where`'s density for that resource (as a resource)
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `stow` consumes 1
- `stow` produces 1, in a store for energy
- `discard` consumes 1

## labor

what working a machine takes; a citizen provides it each turn.

**In families** thing

**Traits of it** `keeps` (the number of turns it will last)

**Signature** `s-11`

**Bounded by** the citizens that make it, one each per turn

**In recipes**

- `build extractor` consumes 1
- `build store` consumes 1
- `build yard` consumes 1
- `create labor` produces 1
- `work` consumes 1
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `discard` consumes 1

## territory

a place things are in, which has a biome, a force of nature, and a density and a total capacity per resource.

**In families** thing, place

**Traits of it** `id` (an identity), `control` (held by a player, or unclaimed), `biome` (one of the biomes), `nature` (a number), `keeps` (the number of turns it will last)

**Signature** `s-12`

**Holds** that kind, up to its total capacity for that kind - *a fact about each one rather than about the kind*

**In recipes**

- `deploy ark` requires 1, in `$where`
- `move` requires 1 (as a place), in `$from`
- `move` requires 1 (as a place), joined to `$from` by an edge the unit crosses, in `$to`
- `launch ark` requires 1, in `$where`
- `work` requires 1, in `$where`
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0

## orbit

a place above one territory, which holds units and nothing else.

**In families** thing, place

**Traits of it** `id` (an identity), `keeps` (the number of turns it will last)

**Signature** `s-13`

**In recipes**

- `deploy ark` consumes 1 (as the place holding ark), in the orbit above `$where`
- `move` requires 1 (as a place), in `$from`
- `move` requires 1 (as a place), joined to `$from` by an edge the unit crosses, in `$to`
- `launch ark` produces 1 (as the place holding ark), in the orbit above `$where`
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0

## deposit

what a territory's ground offers of one resource, and how richly.

**In families** thing

**Traits of it** `density` (a number), `total capacity` (a number), `keeps` (the number of turns it will last)

**Signature** `s-14`

**In recipes**

- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0

## adjacency

two places that share an edge, held by the thing that holds them.

**In families** thing

**Traits of it** `from` (a place), `to` (a place), `keeps` (the number of turns it will last)

**Signature** `s-15`

**In recipes**

- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0

## game

every thing is in it, and it is the one thing that is in nothing.

**In families** thing

**Traits of it** `keeps` (the number of turns it will last), `phase` (design or play)

**Signature** `s-16`

**In recipes**

- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0

## fertility

a citizen's capacity to raise one more, spent by raising one and renewed each turn.

**In families** thing

**Traits of it** `keeps` (the number of turns it will last)

**Signature** `s-17`

**Bounded by** the citizens that make it, one each per turn

**In recipes**

- `bear` produces 1
- `breed` consumes 1
- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `discard` consumes 1

## force

what a territory presents to hold or take ground; mustered each turn and swept at its end.

**In families** thing

**Traits of it** `keeps` (the number of turns it will last)

**Signature** `s-18`

**In recipes**

- `age` requires 1 (as a thing), keeps at least 1
- `age` puts  (as a thing), keeps one less
- `spoil` consumes 1 (as a thing), keeps 0
- `muster` produces that citizen's strength
- `stand` produces that unit's strength
- `discard` consumes 1

