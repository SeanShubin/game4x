# Spec Proposals

**Derived.** Written by Claude. Not binding, and **not the specification** - these are lines
offered for Sean's review. A proposal becomes real only when he accepts it and it lands in
[the specification](../../spec/README.md).

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## How this works

Claude drafts faster than Sean can, but cannot tell which of its inferences are correct.
What it *can* tell is **which inferences it had no business making**, so every proposal is
labelled with its kind. That is the whole point of this file: it lets Sean spend review
effort where the risk actually is.

| Kind          | What Sean is checking                                             | Effort  |
| ------------- | ----------------------------------------------------------------- | ------- |
| **Entailed**  | Claude's logic - it follows from lines already in the spec        | seconds |
| **Measured**  | that the fact is relevant - it came from analysis, not from taste | seconds |
| **Recovered** | that Claude transcribed his intent from conversation correctly    | short   |
| **Invented**  | the design choice itself - Claude is guessing                     | real    |

To respond, say so in chat: `P-3 yes, P-7 no because X`. Claude moves accepted lines into
the spec and records rejections below.

Two limits Claude holds itself to:

- **Never more than 15 open proposals.** Past that, reviewing costs as much as writing and
  the mechanism has failed. Surplus proposals are held back, not filed.
- **Invented proposals stay rare.** Repeated guessing at design means Claude should ask one
  question instead of filing ten guesses.

## Open

### P-1 · Measured · `spec/planet.md` → Shape

> Available region counts are `10T + 2`, where `T = m^2 + mn + n^2` for whole numbers `m`
> and `n`.

**Basis:** the region count of a Goldberg polyhedron `GP(m,n)`. Not a design choice - it is
what the tessellation already chosen permits.

### P-3 · Measured · `spec/planet.md` → Shape

> On a planet of `10m^2 + 2` regions, no two regions are more than `3m` apart.

**Basis:** measured on the built graphs for `m` = 1 through 7. This is the range at which a
weapon reaches anywhere on the planet, so it is the number weapon ranges are designed
against.

### P-4 · Measured · `spec/planet.md` → Shape

> The twelve five-neighbour regions form six antipodal pairs.

**Basis:** they sit at an icosahedron's twelve vertices, which are six antipodal pairs. Holds
for every `GP(m,n)` regardless of class.

### P-5 · Measured · `spec/planet.md` → Shape

> The region farthest from a five-neighbour region is its own antipodal twin, and it is
> unique. For a six-neighbour region the farthest region is not unique.

**Basis:** verified for `GP(1,0)` through `GP(5,0)`. Worth stating because it means only
twelve regions on a planet have a single well-defined "other side".

### P-6 · Entailed · `spec/planet.md` → Shape

> Every region has five or six neighbours. Exactly twelve regions have five.

**Basis:** follows from "divided into regions according to Goldberg polyhedrons" - Euler's
formula forces exactly twelve pentagons in any such tiling, at every size.

### P-7 · Entailed · `spec/planet.md` → Shape

> The smallest planet has no six-neighbour regions; all twelve of its regions have five
> neighbours.

**Basis:** follows from "the minimum planet size is therefore 12, a dodecahedron" together
with P-6.

### P-8 · Recovered · `spec/planet.md` → What a territory carries

> Two territories are adjacent when they share an edge. Territories that meet only at a
> corner are not adjacent.

**Basis:** stated in conversation as a requirement, and it is what makes movement cost
uniform and removes the "does the corner count" question.

### P-9 · Recovered · `spec/planet.md` → Shape

> The distance between every pair of territories is computed once when the world is created
> and stored. The rules read it; they never search for it.

**Basis:** stated in conversation. One byte per pair at the sizes in P-2.

### P-10 · Recovered · `spec/planet.md` → new section, Presentation

> The planet is presented as a three-dimensional sphere and is never projected onto a plane.

**Basis:** decided in conversation on 2026-08-24, and it is what makes the tessellation
choice defensible - see [region schemes](region-schemes.md).

### P-11 · Recovered · `spec/planet.md` → new section, Presentation

> The camera's roll is derived from its direction, never accumulated. The planet cannot come
> to rest upside down.

**Basis:** stated in conversation. A canonical roll is impossible at every point of a sphere
(hairy ball theorem), so the two undefined points are placed on an antipodal pair of
five-neighbour regions.

### P-13 · Measured · `spec/planet.md` → Shape

> The greatest distance between two territories is 3 on a tiny planet, 5 on a small one, 6
> on a medium one, and 9 on a huge one.

**Basis:** measured on the built graphs. This is the number weapon ranges are designed
against - it is how many distinct range bands a planet has, so a tiny planet supports three
and a huge one nine.

**Gap:** the large planet, 72 regions, is not yet measured. It builds correctly - see
[the backlog](spec-backlog.md) - but its diameter has not been computed.

## Accepted

| Proposal                                                                                | Landed in                                | Date       |
| --------------------------------------------------------------------------------------- | ---------------------------------------- | ---------- |
| P-12, "every change to game state is representable and executable as a console command" | `spec/console.md` → Coverage, a new file | 2026-08-25 |

## Rejected

Nothing yet. Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

## Withdrawn

| Proposal                                           | Why                                                                                                                                                                                                                    |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P-2, "twenty planet sizes are available below 500" | Superseded by Sean's edit of 2026-08-25, which fixes the game at five named sizes. The twenty counts remain *available* from the tessellation, but the game does not use them, so the line would have been misleading. |
