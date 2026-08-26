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

### P-29 · Recovered · `spec/planet.md` → What a territory carries

> A territory's threat level comes from what is on it. Ferals are one source of threat; there
> can be others.

**Basis:** stated in conversation on 2026-08-25. The existing line - a territory has a threat
level requiring greater security to claim - is a high-level view and stays true; this says
where the number comes from.

It is [the modelling invariant](../../spec/invariants.md) applied to a specific number.
Without it, a territory's threat is a value someone assigned; with it, threat is the
aggregate of what is actually there, and clearing the Ferals is what lowers it.

### P-32 · Recovered · `spec/control.md`

> **Threat**
>
> Every unit has a threat value. A creature whose damage is accidental has a threat of 1; a
> predator has a threat of 2.
>
> A territory's threat from nature is the **highest** threat present, not the total - it is
> the apex predator of the region. A territory's threat from an organised opponent is the
> **sum** of what they have there.
>
> **Security**
>
> Security is law and order within a territory, and is a single number.
>
> **Gaining and holding ground**
>
> Equal security holds ground. Taking ground requires **greater** security than the
> opposition, whether the opposition is nature or another player.

**Basis:** stated in conversation on 2026-08-25.

The nature-is-maximum, organisation-sums split is the substantive part. It says a hundred
raccoons are no worse than one because raccoons do not coordinate, while a hundred soldiers
are a hundred times the problem because they do. That is a modelled difference rather than a
convenience, which is what [the invariant](../../spec/invariants.md) asks for.

It also means clearing a territory of nature is **tiered**: remove the predators and the
threat falls to 1, the level of whatever accidental damage remains.

## Accepted

| Proposal                                                                     | Landed in                                                                 | Date       |
| ---------------------------------------------------------------------------- | ------------------------------------------------------------------------- | ---------- |
| P-1, the `10T + 2` territory counts, as a consequence of the Goldberg choice | `spec/planet.md` → Shape                                                  | 2026-08-25 |
| P-6, every territory has five or six neighbours; exactly twelve have five    | `spec/planet.md` → Shape                                                  | 2026-08-25 |
| P-8, adjacency is a shared edge, never a shared corner                       | `spec/planet.md` → What a territory carries                               | 2026-08-25 |
| P-10, the planet is presented as a three-dimensional sphere                  | `spec/planet.md` → Presentation                                           | 2026-08-25 |
| P-11, the roll for any point on the planet is fixed                          | `spec/planet.md` → Presentation                                           | 2026-08-25 |
| P-12, every change to game state is a console command                        | `spec/invariants.md` → Everything is expressible                          | 2026-08-25 |
| P-14, the Ark and the Seeder                                                 | `spec/unit-types.md`                                                      | 2026-08-25 |
| P-19, territories have a rating per resource                                 | `spec/planet.md` → What a territory carries                               | 2026-08-25 |
| P-21, resources exist in a place; a cost is paid where it is spent           | `spec/logistics.md`                                                       | 2026-08-25 |
| P-23, territories have an id, unique per planet, starting at 1               | `spec/planet.md` → What a territory carries, Presentation                 | 2026-08-25 |
| P-18, a planet's resources are infinite; the rate is finite                  | `spec/economy.md` → Structures and labor                                  | 2026-08-25 |
| P-31, territories have nodes for each resource, and nodes have density       | `spec/planet.md` → What a territory carries; example in `spec/economy.md` | 2026-08-25 |
| P-30, infrastructure is never a liability; setbacks come from outside        | `spec/invariants.md` → No penalty for building infrastructure             | 2026-08-25 |
| P-24, distance is fixed; roads change traversal, not distance                | `spec/planet.md` → Distance                                               | 2026-08-25 |

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
