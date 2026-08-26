# Spec Backlog

**Derived.** Written by Claude from conversation, 2026-08-25. Not binding - it is a list of
things Sean has *said* but has not yet *written*, and only the writing counts.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Design intent stated in conversation that has not reached [the specification](../../spec/README.md).
When an item lands in the spec, delete the row. Nothing here is decided.

## Resolved

**`spec/overview.md` is dropped.** Genre, inspirations and theme moved to
[vision](../vision.md); the spec holds mechanical detail only.

## Settled 2026-08-25, but not yet written

**Resources are generic for now.** Sean intends to be far more specific later, possibly
including invented resources. That is scope, not a rule, so it belongs in an **Open
questions** entry rather than as a statement - the spec should not imply that food, metal and
fuel are the final list.


**Incentive structures are a mechanic, and they can fail.** Run out of food and the
population cannot be bothered to work for you; it goes and gets its own food instead.

**Evolutionary adaptation is flavour**, explaining why a sentient AI finds biologicals more
efficient than a robot army in the default case. Not simulated. But Sean intends situations
that *require* robot armies later - a planet rich in energy and resources but hostile to life
- so **the model must not hardwire population as biological.** Labor is the abstraction;
citizens are one source of it, and food is their upkeep rather than the only upkeep there
will ever be.

### Resolved: they depart

Decided 2026-08-25. The line already in `spec/economy.md` stands unchanged - an unfed citizen
departs, and self-provisioning is not modelled at all.

Two things follow, and the second is why the word matters:

- **The incentive structure is the food supply.** There is no separate loyalty or unrest
  measure. Failing to feed the population *is* the incentive failing, and the consequence is
  losing it. `spec/economy.md` therefore already specifies this mechanic in full.
- **"Depart" is deliberately unspecified.** It covers a biological population starving and a
  machine population shutting down, which is exactly what keeps one rule over both and serves
  the not-hardwired-as-biological constraint above. Filed as P-17 so a later session cannot
  helpfully narrow it to starvation.

## What the first release needs and the spec does not have

[The first release](../../releases/first-release.md) is one tiny planet and an eight-step
loop. Walking the loop against the spec, in order:

| Loop step                              | Missing from the spec                                                                                                                                                  |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1. No presence on the planet           | How a game begins. What "presence" is. Where the first colonizer comes from, given there is none on the planet                                                         |
| 2. Colonize a territory                | The colonizer itself - `spec/units.md` is empty. Whether "colonize" is the same act as "claim", which `spec/planet.md` already says needs security greater than threat |
| 3. Infrastructure to grow food         | How a structure is built at all: what it costs, who builds it, how long it takes. The farm's *effect* is specified; its construction is not                            |
| 4. Expand population across the planet | Movement. Whether citizens move, or a new territory is colonized from an adjacent one. Sean mentioned "a vehicle that can expand from one territory to another"        |
| 5. Mine metal and fuel                 | **Metal and fuel do not exist in the spec.** Only food does. And by Sean's own logistics rule, a resource must be moved to where it is used - no transport exists      |
| 6. Build a new colonizer               | A structure that produces units, and its inputs                                                                                                                        |
| 7. Infrastructure to launch            | A launch structure, and what it costs                                                                                                                                  |
| 8. Launch into space                   | Orbit - `spec/orbit.md` is empty - and whatever the strategic layer is                                                                                                 |

Two consequences of choosing **tiny** worth knowing before building:

- Tiny is 12 territories and by [P-7](proposals.md) **every one is a pentagon**. The first
  release therefore never exercises the five-versus-six neighbour distinction that makes the
  tessellation interesting. Fine if the release is about the loop; a gap if it is meant to
  prove the geometry.
- Tiny has a greatest distance of 3, so "expand population across the planet" is at most
  three steps from anywhere.

## Resolved: the large planet is buildable after all

`spec/planet.md` names five sizes: 12, 32, 42, 72, 92 - the five smallest Goldberg counts.
**72 is `GP(2,1)`, the only chiral one**, and
[planet-view](../prototypes/planet-view.md) claimed class III fell back to relaxation.

That documentation was stale. `sphere-tessellation` builds all three classes; the code
comment on `canonical_seeds` records that class III needed every icosahedral face wound the
same way, because a mirrored face lays down a mirrored lattice patch. What was missing was
**test coverage of that path and correct documentation**, not the capability. Both fixed
2026-08-25: `the_chiral_solids_are_built_through_the_same_path_as_the_rest` now asserts 72,
132, 192 and 212 through `canonical_seeds`.

## For `spec/planet.md`

Partly written already: Goldberg tessellation, minimum size 12, threat/security, fertility
and the farm/labour/food rule, and the citizen growth rule.

| Not yet written       | What was said                                                                                                 |
| --------------------- | ------------------------------------------------------------------------------------------------------------- |
| Available sizes       | `10T + 2` where `T = m^2 + mn + n^2`; twenty counts below 500, of which nine are chiral class III and unbuilt |
| Diameter              | `3m` for class I - this is the range at which a weapon reaches anywhere                                       |
| Adjacency             | A shared edge, never a shared corner                                                                          |
| Distance              | A precomputed all-pairs table, one byte per pair                                                              |
| Orientation           | North locked, roll derived not accumulated; an antipodal pentagon pair on the axis                            |
| Pentagon structure    | Twelve pentagons in six antipodal pairs; a pentagon's farthest region is its own twin, at exactly `3m`        |
| Rendering             | 3D sphere only, never projected flat; render geometry decoupled from the tessellation                         |
| What a region carries | Beyond threat, security and resource levels: ownership, terrain, structures                                   |

## For `spec/combat.md`

Nothing written yet.

| Not yet written   | What was said                                                                                        |
| ----------------- | ---------------------------------------------------------------------------------------------------- |
| Scales            | Planet-to-planet is strategic; same-planet is tactical                                               |
| Range             | Counted in regions                                                                                   |
| Point defence     | Fires only on what targets it                                                                        |
| Ballistic weapons | Resolve in one turn, trace no path, are not over the intervening territories                         |
| Cruise weapons    | Have a location, take several turns, behave like aircraft                                            |
| The exchange      | Defence scales with range and rate of fire; offence with number and speed                            |
| Launcher range    | A maximum, and reach is the only reason to move closer                                               |
| Lethality         | Does not decay with distance - a shell from fifteen tiles is a shell from four                       |
| Consequence       | Standoff: launchers sit at the edge of their range                                                   |
| No chokepoints    | The cheapest cut is a region's own neighbours, so there are no defensive lines, only defended places |

## For `spec/orbit.md`

Nothing written yet.

| Not yet written | What was said                                                                                          |
| --------------- | ------------------------------------------------------------------------------------------------------ |
| The layer       | An orbital layer above the surface; relationship to the region graph proposed as 1:1 but never decided |
| Starbases       | Sit in orbit, fire missiles that travel in orbit                                                       |
| Reach           | With enough range, a missile hits the opposite side of the planet                                      |
| Crossing layers | Firefights break out between orbit and land                                                            |

## Not yet in any spec file

### Logistics - the core loan from Factorio and Distant Worlds

Stated as the central idea and entirely unwritten:

- **Every material is at a particular place.** A resource has a location, not just a total.
- **Planets have infinite resources, exploitable only at a finite rate.** From Distant
  Worlds. A territory does not deplete; it meters.
- **A resource cannot be used until it is moved to where it is needed.** From both. This is
  the constraint everything else hangs on.

Nothing about transport exists yet: no carriers, no routes, no throughput, no cost.

### Expansion

- A **colonizer** gets started on a planet.
- A **vehicle** expands from one territory to the next.
- Eventually the whole planet is populated.
- Then **spacecraft** are built, and other planets colonised.

### Breadth

- "Many structure types" - only **farm** exists so far.
- "Many different things we can do" - the verb list is unwritten.

### The console and its language

Sean wants **a simple scripting language that fully describes everything that can be done
in the game**, driven from a console he can alter game state from. Three layers, stated
2026-08-25:

1. the user types single lines
2. a layer translates those lines into commands
3. a layer executes those commands

**The governing invariant, stated 2026-08-25: anything that can happen in the game must
also be representable and executable as a console command.** That is a total-coverage
requirement on the command set, not a convenience - it means no game action may exist that
the language cannot express.

Line-oriented, so no multi-line constructs and no block structure. Unwritten: the verb list,
what a command may alter, and whether a command is the same thing as an intent in
[layers](../layers.md) - which that invariant makes very likely, since intents are already
"a value, not a state, integers, serialisable". See [parser architecture](parser-architecture.md) for the
predecessor's design, where it put the dynamic/static seam, and the one property that made
"describe everything" work.

### Never discussed at all

Turn structure and resolution order. Units and movement cost. Visibility and fog of war.
The strategic-layer graph that planet-to-planet combat happens on. Players, AI, and victory
conditions.
