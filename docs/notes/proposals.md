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

To respond, say so in chat: `P-3 yes, P-7 no because X`. **Sean enters accepted lines into
the spec himself**, typing or pasting them - accepting is not permission for Claude to write.
Claude records the acceptance and the rejections below.

Two limits Claude holds itself to:

- **Never more than 15 open proposals.** Past that, reviewing costs as much as writing and
  the mechanism has failed. Surplus proposals are held back, not filed.
- **Invented proposals stay rare.** Repeated guessing at design means Claude should ask one
  question instead of filing ten guesses.

## Open

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
five-neighbour territories.

### P-13 · Measured · `spec/planet.md` → Shape

> The greatest distance between two territories is 3 on a tiny planet, 5 on a small one, 6
> on a medium one, 7 on a large one, and 9 on a huge one.

**Basis:** measured on the built graphs. This is the number weapon ranges are designed
against - it is how many distinct range bands a planet has, so a tiny planet supports three
and a huge one nine.

All five are pinned by `each_planet_size_has_a_known_greatest_distance` in
`sphere-tessellation`, so changing the tessellation cannot move the combat ladder silently.

### P-14 · Recovered · `spec/units.md`

> An **Ark** claims a territory from space. A **Seeder** claims a territory from an adjacent
> territory. An Ark costs more than a Seeder.
>
> An Ark also prints the founding population of a planet, with enough genetic variability to
> be viable.
>
> Everything a player builds is a machine or a building, and citizens operate it. A
> **Foundry** produces machines. A **Yard** produces Arks.
>
> **No structure produces citizens.** Population comes only from the growth rule in
> [population](../../spec/population.md).

**Basis:** settled in conversation on 2026-08-25. The player is a sentient AI that designs
machines and buildings; the population is printed once by an Ark and thereafter reproduces
biologically, because the AI has neither the information nor the compute to anticipate what
will be advantageous on a given planet and delegates that to reproduction.

That removes the need for a unit-producing building on the living side. **Vat is cut** - it
was proposed when citizens were assumed to be manufactured, and they are not. Also rejected
earlier: *Hatchery*, which implies eggs and commits to a reproduction mechanic that is not
chosen; and *Printworks*, as obtuse.

The citizen growth rule already in `spec/population.md` therefore carries more weight than it
looks: **it is the only source of population in the game.**

### P-15 · Recovered · `spec/units.md`

> A planet's dangers are its **Native** life. Life that was printed by a player and has since
> gone wild is **Feral**.

**Basis:** stated in conversation. Native is the general case; Feral is possible but not the
default. This is what a territory's **threat** level measures.

### P-16 · Recovered · `spec/units.md`

> Every unit a player creates carries a name of its own. The name persists when control of
> the unit changes.

**Basis:** stated in conversation, replacing an earlier idea of a *Rival* category of life.
Provenance becomes a property of the individual unit rather than a classification, so
captured units, inherited units and Feral life are all legible by one mechanism. Consistent
with [layers](../layers.md), which already requires canonical identity to be a stable integer
the entity system cannot reuse - a name is that identity made visible.

### P-17 · Recovered · `spec/population.md` → Citizens

> **Depart** does not say how. For a biological population it may mean starving; for a
> machine population, shutting down. The word is left unspecified so that one rule covers
> both.
>
> A departed citizen does not come back. The growth rule is the only way to recover
> population.

**Basis:** decided in conversation on 2026-08-25, when the alternative - citizens who stop
working and feed themselves - was rejected because modelling self-provisioning is not wanted.
The abstraction is load-bearing rather than decorative: it is what lets the same rule govern
a machine population on a world hostile to life, which Sean intends later. Without this line
someone will eventually narrow "depart" to starvation and quietly break that case.

### P-18 · Recovered · `spec/economy.md` → new section, Extraction

> A planet's resources are infinite. What is finite is the **rate** at which they can be
> extracted.

**Basis:** stated in conversation on 2026-08-25, and named earlier as one of the two things
being taken from Distant Worlds. A territory never depletes; it meters. That is what makes
territory a permanent asset rather than a consumable one.

### P-19 · Recovered · `spec/planet.md` → What a territory carries, replacing "Territories have natural resource levels"

> A territory has a separate **rating** for each resource, and they are unrelated to one
> another. One territory might yield 8 food, 4 metal and 6 fuel for each unit of labor
> spent there.

**Basis:** stated in conversation on 2026-08-25. This replaces rather than joins the existing
line - "Territories have natural resource levels" says the same thing less precisely, and two
statements of one rule can drift apart.

### P-20 · Recovered · `spec/economy.md` → Extraction

> Extracting one resource has no effect on extracting any other. There is no shared limit and
> no trade-off between resources on a territory.

**Basis:** stated in conversation on 2026-08-25. Worth stating explicitly because the
opposite is the common assumption - most games make a tile's output a single budget to divide
up. Here each resource has its own independent rate, so a rich territory is rich in every
resource it is rich in, simultaneously.

### P-22 · Recovered · `spec/invariants.md` → Everything is modelled

> Nothing in the game appears or disappears without a cause inside the model. Every quantity
> has an owner, and every change to it has something that did it and a rule that says when.
>
> Conservation is not required. Fiat money is created and destroyed by a government, and that
> is a model - there is an owner and there are rules. What is forbidden is a quantity that
> changes because the game says so, with nothing in the world doing it.

**Basis:** stated in conversation on 2026-08-25, arrived at by pushing on Distant Worlds.
Its resources are fully modelled - located, hauled by freighters with cargo bays, and
construction stalls when the steel has not arrived - while its money is created and destroyed
by no one in particular, has no location, and in one version could go negative until it was
patched. Money is the only part of that game without an underlying explanation, and the rule
above is what would have caught it.

**What it costs:** this forbids a large class of ordinary game conveniences - flat per-turn
bonuses from nowhere, techs that simply make a number larger, a global stockpile with no
place. Each of those would need an owner and a mechanism, or would have to go.

**What it already permits:** everything currently in the spec passes. Citizens are created by
reproduction and consume food to do it; departure has a stated cause even though the
destination is deliberately unspecified; a territory's resources are infinite by an explicit
stated rule rather than by omission.

## Accepted

| Proposal                                                                                | Landed in                                        | Date       |
| --------------------------------------------------------------------------------------- | ------------------------------------------------ | ---------- |
| P-12, "every change to game state is representable and executable as a console command" | `spec/invariants.md` → Everything is expressible | 2026-08-25 |
| P-1, the `10T + 2` territory counts, framed as a consequence of the Goldberg choice     | `spec/planet.md` → Shape, second bullet          | 2026-08-25 |

## Rejected

Nothing yet. Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

## Withdrawn

| Proposal                                                                | Why                                                                                                                                                                                                                                                                                       |
| ----------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P-2, "twenty planet sizes are available below 500"                      | Superseded by Sean's edit of 2026-08-25, which fixes the game at five named sizes. The twenty counts remain *available* from the tessellation, but the game does not use them, so the line would have been misleading.                                                                    |
| P-3, "no two regions are more than `3m` apart"                          | Superseded by P-13, which states the same thing per named planet size instead of as a formula. P-3 was also incomplete: `3m` holds for class I only, and the large planet is class III, where the measured distance is 7 rather than the 6 the formula suggests.                          |
| P-5, "a pentagon's farthest region is its antipodal twin"               | Merged into P-4. It was the same fact split across two bare statements, neither of which said what it was for.                                                                                                                                                                            |
| P-4, "the twelve five-neighbour territories sit in six antipodal pairs" | Derivable from the Goldberg choice, and **no rule in the spec leans on it**. It would become worth stating if P-11 is accepted, since locking the camera axis to an antipodal pentagon pair depends on the pairs existing - at which point it belongs inside P-11 rather than on its own. |
| P-7, "the smallest planet has no six-neighbour territories"             | Derivable from P-6 together with a line Sean has already written - "the minimum planet size is therefore 12, **a dodecahedron**". Naming the solid already says it is all pentagons.                                                                                                      |
