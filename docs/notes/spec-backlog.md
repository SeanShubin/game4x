# Spec Backlog

**Derived.** Written by Claude from conversation, 2026-08-25. Not binding - it is a list of
things Sean has *said* but has not yet *written*, and only the writing counts.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Design intent stated in conversation that has not reached [the specification](../../spec/README.md).
When an item lands in the spec, delete the row. Nothing here is decided.

## Resolved

**`spec/overview.md` is dropped.** Genre, inspirations and theme moved to
[vision](../vision.md); the spec holds mechanical detail only.

## Flagged: the large planet is not buildable

`spec/planet.md` names five sizes: 12, 32, 42, 72, 92. Those are exactly the five smallest
Goldberg counts, so the choice is sound - but **72 is `GP(2,1)`, the only chiral one**, and
[planet-view](../prototypes/planet-view.md) records that `sphere-tessellation` does not build
class III and falls back to relaxation. Today "large" would not be a true Goldberg solid.

Two ways out, and it is Sean's choice: build class III in `sphere-tessellation`, or move
"large" to 122 (`GP(2,2)`, class II, the sixth smallest) and accept a wider gap between
medium and large.

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
