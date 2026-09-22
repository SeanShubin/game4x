# Prototype: moving a disk across a Goldberg planet

[Prototypes](../../docs/prototypes/README.md) · [Root README](../../README.md)

Run it with `scripts/goldberg-move.sh` or `scripts/goldberg-move.ps1`.

## The question

Sean, 2026-09-21, asking for a prototype rather than a feature: **can a player lay out a move of
any length across a sphere by clicking, without the interface ever having to guess which way
round he meant to go?**

The answer offered here is that the interface never guesses, because a destination is only
clickable when **exactly one shortest route reaches it**. Where several do, the click is refused
with the number of routes, and the player clicks a territory on the way first - which turns one
question with six answers into two questions with one answer each.

**There are no game mechanics.** No cost, no turn, no limit on distance, and no rule about what
a disk is. What is being tried out is the gesture.

## What you do

Forty-two territories - `GP(2,0)`, the third-smallest Goldberg solid, after the dodecahedron's
twelve and the truncated icosahedron's thirty-two - and four coloured disks.

| Click                             | What happens                                                |
| --------------------------------- | ----------------------------------------------------------- |
| a disk                            | it is picked up                                             |
| the same disk again               | it is put down                                              |
| a territory, while holding a disk | the move is laid out to it, and the planet turns to face it |
| that same territory again         | the move is made                                            |
| a different territory             | the move goes further, and the planet turns again           |
| the disk's own territory          | the move is abandoned                                       |

Drag, a finger or the arrows turn the planet; the wheel zooms; `R` resets the view.

## Where the answer is

**In the tests**, which is the point of the split. `src/board.rs` answers *which routes reach
there* and `src/plan.rs` answers *what does this click do*, and neither knows Bevy exists - so
`tests/routes.rs` drives the whole interaction with no window at all.

Two of those tests are the ones worth reading:

- `some_destinations_are_unambiguous_and_some_are_not` counts both answers over all 1,764 pairs
  of territories. A rule that refused everything would pass every test about refusals and one
  that refused nothing would pass every test about routes; the two counts together say the board
  actually poses the question.
- `clicking_a_territory_on_the_way_makes_the_rest_unambiguous` is the escape, and it is what
  makes the constraint a constraint rather than a wall.

## How the interface reaches the globe

`planet-bevy` gives the sphere, the camera, the drag and the zoom. It does not offer picking or
recentring, and both are done here from its **public** parts rather than by reaching inside it:

- **Picking** is a ray against the unit sphere and then the nearest centre. That is exact rather
  than approximate, because the territories are the Voronoi cells of those centres - so the
  nearest centre to a point on the sphere *is* the territory it is in. Nothing reads the mesh,
  so picking cannot disagree with the adjacency the routes are computed over.
- **Recentring** is two `atan2`s. The globe's rotation is `pitch * yaw * upright`, and `upright`
  comes from `Direction::NORTH_POLE`, which is public - so this reproduces it exactly rather
  than guessing at it.

It links no game: no console, no command language, no biome, no terrain. That is the same test
`goldberg-view` applies to the same boundary.
