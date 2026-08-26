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

### P-25 · Recovered · `spec/narrative.md` → Life

> An Ark prints a planet's founding population, with enough genetic variability for it to be
> viable.
>
> The AI does not try to design life suited to a particular planet. It has neither the
> information nor the compute to know what would be advantageous there, so it leaves that to
> biological reproduction.

**Basis:** stated in conversation on 2026-08-25. This is flavour rather than a simulated
mechanic - no adaptation is modelled - and its job is to explain why an AI with advanced
technology works through biologicals at all instead of building machines that need no food.
Sean intends worlds where that calculation reverses, rich in energy but hostile to life; that
is recorded in [the backlog](spec-backlog.md) and is not part of this.

### P-26 · Recovered · `spec/narrative.md` → The population

> The population acts on its own. It operates the machines and works for the AI because of
> incentive structures, not because it is commanded.
>
> The AI designs machines and buildings. The population operates them.

**Basis:** stated in conversation on 2026-08-25. The second line is the division of labour
the whole game rests on, and it is why nothing manufactures citizens - see P-14.

The incentive structure is concrete rather than decorative: it is **the food supply**. There
is no separate loyalty or unrest measure. Failing to feed the population is the incentive
failing, and the departure rule in [population](../../spec/population.md) is what that costs.

### P-27 · Recovered · `spec/structures.md`

> ### Foundry
>
> A Foundry produces machines.
>
> ### Yard
>
> A Yard produces Arks.

**Basis:** vocabulary settled in conversation on 2026-08-25, split out of P-14 because a
Foundry and a Yard are structures rather than units. *Vat* was cut in the same conversation:
it was proposed when citizens were assumed to be manufactured, and they are not - see P-28.

### P-28 · Recovered · `spec/population.md` → Citizens

> No structure produces citizens. Population comes only from the growth rule below.

**Basis:** settled in conversation on 2026-08-25 and split out of P-14. The population is
printed once by an Ark and thereafter reproduces biologically, so there is no
unit-producing building on the living side and no build order for population.

This makes the growth rule the **only** source of population in the game, which is worth
stating where the growth rule lives rather than leaving a reader to infer it from an absence.

### P-32 · Recovered · `spec/control.md`

> **Force**
>
> Force is the capacity for violence. Every actor in a territory has some, whether it is a
> player or the life already living there.
>
> Every creature has a force. A predator has greater force than a creature whose damage is
> only accidental.
>
> **Coordination**
>
> Organised force **sums**. Unorganised force is the **highest** present, not the total.
> Nature does not coordinate, so the force of the wild in a territory is that of its apex
> predator however much else lives there.
>
> **Gaining and holding ground**
>
> You take a territory when you bring more force than is already there. Equal force holds
> ground; neither side gains.
>
> Force is compared against each neighbour separately and never against their total, because
> neighbours do not coordinate with one another either.
>
> **Culling**
>
> Holding greater force than the life in a territory drives its force down. Falling below it
> lets that force grow again.

**Basis:** stated in conversation on 2026-08-25, over five exchanges.

**Threat and security are gone.** They named one quantity from two sides, like uphill and
downhill, and no rule needed to know which side it was looking at - so three words were doing
the work of one, and the pair invited a future rule that only made sense if they were
genuinely separate. An interface may still label a hostile territory's force as threat; that
is presentation with no rule attached.

**The numbers moved out.** An earlier draft fixed accidental damage at 1 and a predator at 2.
Those are tuning, not specification, so the spec now states only the ordering and the values
go to a release - see P-36 and rule 7 in [the specification's own rules](../../spec/README.md).

### P-36 · Recovered · `releases/first-release.md`

> A creature whose damage is accidental has a force of 1. A predator has a force of 2.

**Basis:** Sean's own values from 2026-08-25, moved out of the specification on his
observation that they are *"an example baseline that can be tuned, not really a
specification."*

They live in a release because a release records what is actually shipped and can change
without touching the spec. Once there is code, the numbers belong in game data and the
release entry becomes a pointer to it rather than the source.

### P-33 · Recovered · `spec/control.md` → a section on wildlife

> Several species may live in one territory, each with its own force.
>
> Species of equal force coexist. Where one has greater force than another, it preys on the
> weaker.
>
> **Nature never exterminates.** Extermination requires coordination, and nature does not
> coordinate. Only a player can drive a species to nothing.

**Basis:** stated in conversation on 2026-08-25. It follows from the coordination principle
in P-32 rather than standing alone.

Two consequences worth seeing. A wild territory always has life in it, so the threat ladder
reaches zero only by a player's hand. And a territory's force can **rise on its own** when a
player's grip slips - the apex is whatever survives, and something always survives.

### P-34 · Recovered · `spec/economy.md` → Extraction, and `spec/control.md` → Security

> **In `spec/economy.md`:**
>
> A structure is operated by citizens spending labor. Labor spent in one structure is not
> available in another.
>
> **In `spec/control.md`:**
>
> A citizen may spend labor on security instead of on a structure, producing 1 force. A
> citizen doing so is not working a farm or a mine.

**Basis:** stated in conversation on 2026-08-25. Every citizen can fend for itself - *"say we
armed them all with pistols"* - so no building is required to produce force. What a building
buys is efficiency, not permission.

**Correction:** an earlier draft of this proposal said citizens could produce no force at all
without a structure, and built an argument on it that the coordination rule in P-32 was
therefore modelled rather than asserted. That was Claude's misreading and is withdrawn.
Coordination distinguishes a player from nature because a civilisation coordinates and
wildlife does not; it is not conferred by a building.

### P-35 · Recovered · `spec/control.md` → Security, and `spec/structures.md`

> **In `spec/control.md`:**
>
> Security structures form a tier. Each is built once per territory, because each represents
> coverage of the whole territory rather than a presence in one part of it, and a territory
> has at most one. The higher the tier, the more force a citizen produces for a unit of labor.
>
> **In `spec/structures.md`:**
>
> ### Police station
>
> While a police station stands, a citizen spending a unit of labor on security produces 2
> force instead of 1.

**Basis:** stated in conversation on 2026-08-25. *Militarised* was Sean's first word for the
rungs above a police station and he has since rejected it: what varies is **the level of
security infrastructure available**, a tiered list of structures.

**What a tier actually buys is labor, not safety.** Holding a given force costs half as many
citizens at tier two, so the rest are free to farm and mine. A police station does not make a
territory safer; it makes it richer at the same safety - *"allowing the farmers and miners to
focus without worry."*

**Open:** the rungs. Only the police station is named, and only its multiplier is fixed. The
tier below it - if any - and everything above need names and numbers.

## Accepted## Accepted## Accepted## Accepted## Accepted## Accepted

| Proposal                                                                       | Landed in                                                                 | Date       |
| ------------------------------------------------------------------------------ | ------------------------------------------------------------------------- | ---------- |
| P-1, the `10T + 2` territory counts, as a consequence of the Goldberg choice   | `spec/planet.md` → Shape                                                  | 2026-08-25 |
| P-6, every territory has five or six neighbours; exactly twelve have five      | `spec/planet.md` → Shape                                                  | 2026-08-25 |
| P-8, adjacency is a shared edge, never a shared corner                         | `spec/planet.md` → What a territory carries                               | 2026-08-25 |
| P-10, the planet is presented as a three-dimensional sphere                    | `spec/planet.md` → Presentation                                           | 2026-08-25 |
| P-11, the roll for any point on the planet is fixed                            | `spec/planet.md` → Presentation                                           | 2026-08-25 |
| P-12, every change to game state is a console command                          | `spec/invariants.md` → Everything is expressible                          | 2026-08-25 |
| P-14, the Ark and the Seeder                                                   | `spec/unit-types.md`                                                      | 2026-08-25 |
| P-19, territories have a rating per resource                                   | `spec/planet.md` → What a territory carries                               | 2026-08-25 |
| P-21, resources exist in a place; a cost is paid where it is spent             | `spec/logistics.md`                                                       | 2026-08-25 |
| P-23, territories have an id, unique per planet, starting at 1                 | `spec/planet.md` → What a territory carries, Presentation                 | 2026-08-25 |
| P-18, a planet's resources are infinite; the rate is finite                    | `spec/economy.md` → Structures and labor                                  | 2026-08-25 |
| P-22, everything is modelled: nothing changes without a cause inside the model | `spec/invariants.md` → Everything is modelled                             | 2026-08-25 |
| P-31, territories have nodes for each resource, and nodes have density         | `spec/planet.md` → What a territory carries; example in `spec/economy.md` | 2026-08-25 |
| P-30, infrastructure is never a liability; setbacks come from outside          | `spec/invariants.md` → No penalty for building infrastructure             | 2026-08-25 |
| P-24, distance is fixed; roads change traversal, not distance                  | `spec/planet.md` → Distance                                               | 2026-08-25 |

## Rejected

Nothing yet. Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

## Withdrawn

| Proposal                                                                | Why                                                                                                                                                                                 |
| ----------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P-2, "twenty planet sizes are available below 500"                      | Superseded by Sean's edit fixing the game at five named sizes.                                                                                                                      |
| P-3, "no two territories are more than `3m` apart"                      | Superseded by the per-size statement, and incomplete: `3m` holds for class I only, while the large planet is class III where the measured distance is 7.                            |
| P-4, "the twelve five-neighbour territories sit in six antipodal pairs" | Derivable from the Goldberg choice and no rule leans on it. Belongs inside a camera-orientation rule if one is ever written.                                                        |
| P-5, "a pentagon's farthest territory is its antipodal twin"            | Merged into P-4, then withdrawn with it.                                                                                                                                            |
| P-7, "the smallest planet has no six-neighbour territories"             | Derivable from P-6 plus a line Sean had already written - the minimum is 12, **a dodecahedron**. Naming the solid already says it is all pentagons.                                 |
| P-9, "the distance between every pair is computed once and stored"      | An implementation directive, not a rule of the game. How distance is computed belongs in a crate README.                                                                            |
| P-13, "the greatest distance is 3 / 5 / 6 / 7 / 9 by planet size"       | Determined by the Goldberg choice and the size, nothing leans on it, and the numbers are **already asserted by a test** - a prose copy could drift from it.                         |
| P-15, "Native life is a planet's own, Feral is printed life gone wild"  | **Feral is a behavioural description, not an origin one**, and origin is not substantively relevant, so the distinction had no mechanical consequence.                              |
| P-16, "every unit carries a name that persists when control changes"    | **A unit has a type, and the type has a name.** Individual units of the same type are not distinguished, so provenance is not tracked at all - consistent with the P-15 withdrawal. |
