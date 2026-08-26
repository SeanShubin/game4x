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

### P-14 · Recovered · `spec/unit-types.md`

> ### Ark
>
> An Ark claims a territory from space. It costs more than a Seeder.
>
> An Ark also prints the founding population of a planet.
>
> ### Seeder
>
> A Seeder claims a territory from a territory adjacent to it.

**Basis:** vocabulary settled in conversation on 2026-08-25. Both perform the same act -
`claim` - and differ only in where they can operate from and what they cost, so one rule
covers both and the difference lives in the unit. Rejected in the same conversation:
*Hatchery*, which implies eggs and so commits to a reproduction mechanic that is not chosen;
and *Printworks*, as obtuse.

**Split note:** this proposal originally carried the structures and the no-manufactured-
citizens rule as well. Those are now P-27 and P-28, so each lands in one place.

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
