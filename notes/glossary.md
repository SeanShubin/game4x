# Glossary

**Claude's understanding, written 2026-08-30 at Sean's request.** Not the specification and not
binding. Where this disagrees with [`spec/`](../spec/README.md), the spec is right and this is
wrong - tell Claude and it gets corrected.

Each entry says what the term means, then **where it is fixed** so a disagreement can be settled.
Anything Claude is inferring rather than reading is marked **(inferred)**.

---

## The ground

**Territory** - one region of the planet's surface, and the unit almost everything is counted in.
It carries an id, a biome, its nodes, and a force of nature; once founded it also carries citizens,
stores, and structures. **Nothing crosses a territory boundary in this release** except a mobile
unit, so a territory is very nearly a separate little economy. `spec/planet.md`, `spec/logistics.md`

**Biome** - what the terrain of that ground is: ocean, ice, desert, grassland, jungle or mountain.
It is *what the terrain gives it*, not a label chosen independently. Ocean cannot be claimed. Today
**nothing in the rules reads a biome** - it decides the picture and the generator's node table, and
no mechanic asks about it. `spec/planet.md`

**Node** - one place in a territory where a resource can be extracted. A territory has zero or more
per resource, and each node has a **density**. The count of nodes sets how many extractors can
exist; the density sets how much one extractor yields. `spec/economy.md`

**Nature**, or **force of nature** - force inherent to the ground, which nature holds it with. It is
what you must exceed to take a territory and match to keep it; if the force present ever falls
below it, **nature takes the territory back and the population perishes**. Nature has no quantity,
consumes nothing, and exploits nothing. It is 1 on every territory in this release, deliberately, as
a baseline to tune from. `spec/control.md`

## What is counted

**Food** - for population. **Metal** - for building things. **Energy** - for moving things.
Those three, and no others yet. `spec/resources.md`

A territory's **stores** of each are held in that territory and **discarded at the end of every
turn**, so anything you buy must be paid for by one turn's production. This single fact drives most
of what makes the first release hard to win. `spec/turn.md`

**Citizen** - population, and **not one person**: it is the smallest group that can sustain
reproduction, so a count of one is a founding population rather than an individual. A citizen
provides **one labor each turn**, works at one structure, and cannot be in two places. Citizens grow
on surplus food and depart when unfed. Only a founding unit creates one; nothing else does.
`spec/population.md`

**Labor** - what a citizen provides. Spent when used, restored at the end of the turn. It is the
real constraint on a territory: **a territory can only work as many nodes as its own food can feed
hands for**, so spare labor is roughly `food nodes x (density - 1)` - which is why a territory whose
food density is 1 can never do anything at all. **(inferred - the formula is arithmetic from
`spec/economy.md` rather than a line in it)**

## What sits on the ground

**Extractor** - works one node and produces its resource. Costs 1 labor to build and nothing else,
and must be *worked* by citizens to yield anything. One per node, so the node count is the ceiling.
`spec/structures.md`, `releases/first-release.md`

**Garrison** - the structure through which a territory's citizens apply force. It does two things:
it lets their force **sum** instead of counting only the largest, and it holds force of its own.
**A garrison is never built** - a territory gains one by being founded and in no other way, and has
at most one, because it represents the organisation of the whole territory. It is also what allows a
Pioneer to be produced. `spec/control.md`

**Yard** - produces Arks, and is the only thing that can. Costs 15 metal, paid from that territory's
own store in one turn. `spec/structures.md`, `releases/first-release.md`

## What moves

**Ark** - the thing you arrive in and the thing you leave in. Force 2, two energy cells, one cell
per move, and **the only unit that can come down from orbit onto unclaimed ground**. Landing *is*
founding: the Ark is consumed and becomes a garrison, a citizen and a food extractor. Producing one
costs 12 metal and 12 energy and needs a Yard. **Launching an Ark from a fully exploited planet is
how you win.** `spec/control.md`, `releases/first-release.md`

**Pioneer** - the Ark's overland equivalent, and how a planet gets settled. Force 2, two cells,
upkeep of 1 food per turn. Costs 8 metal, 6 energy and a citizen, and needs a garrison to be
produced at. It **must found the territory it enters or it perishes**, so it is spent rather than
kept. `releases/first-release.md`

---

## Three things worth knowing that no single term says

**Founding is one action, not two.** An Ark landing or a Pioneer entering unclaimed ground takes the
territory *and* founds it in the same act - there is no intermediate step where you own bare ground.
The founding unit is consumed and becomes garrison, citizen and food extractor, each at one less
force. That is why `spec/invariants.md` forbids an intermediate step that is always taken.

**Force is organised or it is not.** Organised force sums; unorganised force is only the highest
present. Citizens can fight but cannot coordinate, so without a garrison a territory of twelve
citizens has the force of one. `spec/control.md`

**A territory is an island.** No resource and no citizen crosses a boundary in this release, and
stores vanish each turn, so every cost is paid from what that one territory produced that turn.
Logistics is deliberately deferred, and it is the single largest thing the game does not yet have.
