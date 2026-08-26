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

**Sean edits a proposal here, in place, until he is happy with it.** He never has to open a
spec file or hunt for a section - the destination is in the proposal's heading and Claude
handles the move.

Claude fixes typos, grammar and wrapping **in the proposal**, reporting every change, so that
the text Sean approves is the text that ships. When Sean says *promote P-n*, Claude copies it
verbatim into the destination and asserts it landed. Nothing but line wrapping, bullet-versus-
paragraph and heading level may change during a promotion. The full protocol is in
[CLAUDE.md](../../CLAUDE.md).

To reject instead, say so and why: the reason is recorded below, or the same proposal comes
back in a later session.

Two limits Claude holds itself to:

- **Never more than 15 open proposals.** Past that, reviewing costs as much as writing and
  the mechanism has failed. Surplus proposals are held back, not filed.
- **Invented proposals stay rare.** Repeated guessing at design means Claude should ask one
  question instead of filing ten guesses.

## Open

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

### P-32 · Recovered · `spec/control.md`

**One word below is Claude's, not Sean's:** *population*, for the number of wildlife in a
territory. See the naming note at the end.

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
> creature however much else lives there.
>
> **Gaining and holding ground**
>
> You take a territory when you bring more force than is already there. Equal force holds
> ground; neither side gains.
>
> Force is compared against each neighbour separately and never against their total, because
> neighbours do not coordinate with one another either.
>
> **Clearing wildlife**
>
> Wildlife is cleared all at once or not at all. Because the species in a territory do not
> coordinate, clearing every one of them is no harder than clearing the strongest, so a
> territory's force stays at its apex until nothing is left, then falls to nothing.
>
> The wildlife of a territory has a population. Force greater than the wildlife's kills some
> of that population each turn, and the excess is how much. Where the two forces are equal,
> the killing offsets the growth and the population holds.
>
> Wildlife spreads only from a territory that has some. Clear every territory on a planet and
> it is gone for good; leave one and it can spread back.

**Basis:** stated in conversation on 2026-08-25 and 2026-08-26, over seven exchanges.

**Coordination is one idea underneath three rules.** It is why organised force sums and wild
force does not, why neighbours are compared one at a time rather than in aggregate, and why
nature never exterminates anything. Each is a consequence rather than a separate decision.

**All at once, not species by species.** Species that do not help each other are no harder to
fight together than singly. This also settles what clearing does to force: **nothing, until
it is finished.**

**What "wears it down" meant.** An earlier draft said holding greater force *wears the
wildlife down* without saying what was being worn. The missing quantity is a **population**,
independent of force - a hundred raccoons present the force of one, so population decides how
long clearing takes while force decides who holds the ground.

Because that population grows - by P-40 - "equal force holds ground" is a consequence rather
than a stated exception: a standoff is what happens when the killing exactly offsets the
breeding.

*Population* is Claude's word and collides with `spec/population.md`, which is about citizens.
*Numbers* is the obvious alternative; *density* is taken by nodes.

**The reclaiming lines were replaced.** They read *"a cleared territory is reclaimed from a
neighbouring territory that still has wildlife. Wildlife is gone from a planet only when no
territory on it has any."* The first had "reclaimed from" pointing the wrong way, hid the
actor in a passive, stated no condition, and once a condition was added merely duplicated
"you take a territory when you bring more force than is already there." The second was close
to a tautology. What survives is the part neither said plainly: **a source is required**, so
clearing every territory makes it permanent.

**Threat and security are gone.** They named one quantity from two sides, and no rule needed
to know which side it was looking at. An interface may still label a hostile territory's force
as threat; that is presentation with no rule attached.

**The numbers moved out.** An earlier draft fixed accidental damage at 1 and a predator at 2.
Those are tuning - see P-36 and rule 7 in [the specification's own rules](../../spec/README.md).

### P-36 · Recovered · `releases/first-release.md`

> A creature whose damage is accidental has a force of 1. A predator has a force of 2.

**Basis:** Sean's own values from 2026-08-25, moved out of the specification on his
observation that they are *"an example baseline that can be tuned, not really a
specification."*

They live in a release because a release records what is actually shipped and can change
without touching the spec. Once there is code, the numbers belong in game data and the
release entry becomes a pointer to it rather than the source.

### P-40 · Recovered · `spec/control.md` -> Wildlife

> - The species of least force in a territory eat from its food nodes. Every other species
>   eats what it preys on
> - Every species grows and starves by the same rule as citizens, counting what it eats as
>   its food

**Basis:** stated by Sean on 2026-08-26 - *"the bottom of the food chain eats food from the
food resource nodes, and they cap the higher levels of the food chain that feed on them rather
than the food nodes directly"*, *"we already have a formula for population growth, just reuse
that"*, and *"place in the food chain is determined by force."*

**The food chain is already in the spec.** *Place in the food chain is determined by force*
adds no rule, because `spec/control.md` -> Wildlife already says a species of greater force
preys on the weaker. That line was written to say who eats whom; read again it also says where
each species sits. So the chain is not a new structure to specify - it is the force ordering of
whatever lives there, and only two things above are actually new: **what the bottom eats**, and
**that the same growth rule applies.**

**Force now does three jobs.** It decides who holds a territory, who preys on whom, and where a
species sits in the chain. No trophic level, no diet, no separate attribute.

**Equilibrium falls out with no carrying capacity.** The bottom is capped by the territory's
food nodes and every level above by the level below, so nothing grows without bound. Claude had
been about to propose a capacity number for exactly this; it is not needed.

**And real ecology comes free.** Kill the predators and the herbivores boom. Kill the
herbivores and the predators starve untouched - **starving the apex is cheaper than fighting
it**, which is a strategy nobody had to design. A territory with rich food nodes supports more
herbivores, which support more predators, so a territory is dangerous *because* it is rich,
with no rule placing apex creatures anywhere.

**Open - does eating consume what is eaten?** The citizen rule says what happens when food is
short or plentiful but never says food is spent. For citizens that has not mattered. For a food
chain it decides whether the model is stable: if predators consume their prey, a predator
population can in principle eat its prey to nothing, then starve, leaving both gone - which is
**nature exterminating something**, contradicting the line directly above it in the same
section. Real populations avoid this because the predators crash first, but that is a property
of the arithmetic, not a guarantee. This wants settling before the section is promoted.

**Open - the growth rule is in the wrong place.** It lives in `spec/population.md` under
**Citizens**. If wildlife obeys the same rule, it is a rule about populations, and leaving it
under citizens makes every other population read as a special case. It wants stating once, with
each population saying what its food is.

### P-34 · Recovered · `spec/economy.md` -> Extraction

> A citizen works at one structure and cannot be in two places at once.
>
> Structures that produce force and structures that extract resources are alike in this: a
> citizen at one is not at the other.

**Basis:** stated by Sean on 2026-08-26 - *"security structures and resource gathering
structures are just different structures and the citizen can't be in two places at once."*

**Simpler than the earlier draft, which invented a category.** That draft had a citizen
"spending labor on security" as an alternative to working a structure, making security a
special kind of activity. It is not. A garrison is a structure like a farm, and the
exclusivity everyone was reaching for is just that a citizen occupies one place.

### P-39 · Invented · `spec/narrative.md` -> Violence and order

> Violence is inherent. Coordination is imposed.

**Basis:** Claude's phrasing, from P-38's rationale, which Sean asked to have in the spec on
2026-08-26. The substance is his - *"citizens are capable of violence, but without a structure
they are not capable of coordination"* - and the compression into two sentences is Claude's,
which is why this is filed as **Invented** rather than Recovered even though no idea in it is
new.

It belongs in the theme document rather than in [control](../../spec/control.md) because
`control.md` already carries the mechanical form in P-38, and
[narrative](../../spec/narrative.md) is defined as where the causes behind the rules are told
as a story. This is that cause in one line.

**Deliberately left bare.** Two sentences with nothing after them is the whole point; any
elaboration Claude wrote would be Claude's idea rather than Sean's, and the mechanism is
already stated elsewhere.

### P-38 · Recovered · `spec/control.md` -> Coordination

> Citizens are capable of violence but not of coordination. Without something to coordinate
> them, the force they present is the highest among them rather than the total - the same as
> wildlife.
>
> Coordination is imposed on citizens by a structure, such as a police station, or by a
> military unit, which carries coordination with it rather than needing a place.

**Basis:** stated by Sean on 2026-08-26, and narrowed by him the same day - *"wildlife can
invade, they just can't sum their force."*

**Coordination does one thing: it makes force sum.** Nothing else. Anything may invade,
anything may defend; what differs is only how much force it brings to the comparison. An
earlier draft had the uncoordinated unable to invade or to protect others, and both of those
were consequences dressed up as rules. Being unable to protect others is just what a maximum
does - ten citizens of force 1 present a force of 1, so the tenth adds nothing.

**This is what makes the coordination rule a mechanism rather than an assertion.** P-32 says
organised force sums while unorganised force is only the highest present, and until now that
was declared true of nature and of players by fiat. It follows from one thing: **violence is
inherent, coordination is imposed.** Nature has nothing to impose it. Uncoordinated citizens
are in exactly the same position.

**It also gives military units a reason to exist.** A structure coordinates the citizens of
one territory and cannot move. A military unit is coordination that travels, which is what
lets force reach where the infrastructure does not - a real distinction between the two kinds
of thing rather than a flavour one.

**It resolves the contradiction with P-32.** A cleared territory being reclaimed by wildlife
is now unremarkable: the wild invades like anything else, bringing the force of its apex
creature and no more.

**Open, and it touches P-35.** If coordination is what makes force sum, a police station's
job may be to enable summing rather than to raise a rate. P-35 currently says a citizen
produces 2 force instead of 1 there, which assumes summing already happens. Both cannot be
the whole story: without a structure, a hundred citizens present force 1, and with one they
present a hundred - or two hundred, if the rate rises as well. Which of those the police
station does is undecided.

### P-35 · Recovered · `spec/control.md` -> Producing force, and `spec/structures.md`

**The name `garrison` is Claude's suggestion, not Sean's.** Everything else is his. See the
naming note below.

> **In `spec/control.md` -> Producing force:**
>
> A territory has at most one garrison, because it represents the organisation of the whole
> territory rather than a presence in one part of it.
>
> A garrison does two things. It lets the citizens of that territory sum their force instead
> of presenting only the highest among them. And it has a multiplier, so that a citizen
> working there produces that much force.
>
> A garrison's multiplier is 1 when it is built, and rises as it is equipped.
>
> **In `spec/structures.md`:**
>
> ### Garrison
>
> The structure through which the citizens of a territory apply force.

**Basis:** stated by Sean on 2026-08-26 - the structure *"has both a force multiplier per
citizen and allows the citizens to sum up their force"*, with the multiplier *"representing
equipment and vehicles provided."*

**One structure, not a tier.** An earlier draft had security structures forming a ladder -
police station, then something more militarised, each a separate building. This replaces that
with a single structure whose multiplier rises, which is fewer things and explains the range
better: the same building is a police post at multiplier 1 and a military base at 5.

**It is useful before it is equipped.** At multiplier 1 a garrison already changes a hundred
citizens presenting force 1 into a hundred presenting force 100. That is the transformative
step; the multiplier is the incremental one on top of it.

**Naming.** Sean noted that *police station* is wrong at the top of the range, where the thing
is a military base. `garrison` is Claude's suggestion: it is the ordinary English word for the
body of force that holds a place, it carries no strong civil or military lean, and it was the
word Sean reached for first before the concept was pinned down. Alternatives that also span
the range: **headquarters** or **command post**, which name the coordination rather than the
force; **armory**, which names the equipment the multiplier represents.

**Open:** how a garrison's multiplier rises. Sean has said it represents equipment and
vehicles, not what is spent to get them.

## Accepted## Accepted## Accepted## Accepted## Accepted## Accepted

| Proposal                                                                                           | Landed in                                                                 | Date       |
| -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- | ---------- |
| P-1, the `10T + 2` territory counts, as a consequence of the Goldberg choice                       | `spec/planet.md` → Shape                                                  | 2026-08-25 |
| P-6, every territory has five or six neighbours; exactly twelve have five                          | `spec/planet.md` → Shape                                                  | 2026-08-25 |
| P-8, adjacency is a shared edge, never a shared corner                                             | `spec/planet.md` → What a territory carries                               | 2026-08-25 |
| P-10, the planet is presented as a three-dimensional sphere                                        | `spec/planet.md` → Presentation                                           | 2026-08-25 |
| P-11, the roll for any point on the planet is fixed                                                | `spec/planet.md` → Presentation                                           | 2026-08-25 |
| P-12, every change to game state is a console command                                              | `spec/invariants.md` → Everything is expressible                          | 2026-08-25 |
| P-14, the Ark and the Seeder                                                                       | `spec/unit-types.md`                                                      | 2026-08-25 |
| P-19, territories have a rating per resource                                                       | `spec/planet.md` → What a territory carries                               | 2026-08-25 |
| P-21, resources exist in a place; a cost is paid where it is spent                                 | `spec/logistics.md`                                                       | 2026-08-25 |
| P-23, territories have an id, unique per planet, starting at 1                                     | `spec/planet.md` → What a territory carries, Presentation                 | 2026-08-25 |
| P-18, a planet's resources are infinite; the rate is finite                                        | `spec/economy.md` → Structures and labor                                  | 2026-08-25 |
| P-33, species coexist or prey on each other; nature never exterminates                             | `spec/control.md` -> Wildlife                                             | 2026-08-26 |
| P-37, a citizen is the smallest group that can sustain reproduction                                | `spec/population.md` → Citizens                                           | 2026-08-26 |
| P-28, an Ark produces the founding citizens; nothing else produces citizens                        | `spec/population.md` → Citizens                                           | 2026-08-26 |
| P-26, the population acts on its own; the AI designs, the population operates                      | `spec/narrative.md` → The population                                      | 2026-08-26 |
| P-25, the Ark prints the founding population; the AI designs life generally, selection finishes it | `spec/narrative.md` → Life                                                | 2026-08-26 |
| P-22, everything is modelled: nothing changes without a cause inside the model                     | `spec/invariants.md` → Everything is modelled                             | 2026-08-25 |
| P-31, territories have nodes for each resource, and nodes have density                             | `spec/planet.md` → What a territory carries; example in `spec/economy.md` | 2026-08-25 |
| P-30, infrastructure is never a liability; setbacks come from outside                              | `spec/invariants.md` → No penalty for building infrastructure             | 2026-08-25 |
| P-24, distance is fixed; roads change traversal, not distance                                      | `spec/planet.md` → Distance                                               | 2026-08-25 |

## Rejected

Nothing yet. Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

## Withdrawn

| Proposal                                                                     | Why                                                                                                                                                          |
| ---------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| P-2, "twenty planet sizes are available below 500"                           | Superseded by Sean's edit fixing the game at five named sizes.                                                                                               |
| P-3, "no two territories are more than `3m` apart"                           | Superseded by the per-size statement, and incomplete: `3m` holds for class I only, while the large planet is class III where the measured distance is 7.     |
| P-4, "the twelve five-neighbour territories sit in six antipodal pairs"      | Derivable from the Goldberg choice, and no rule leans on it.                                                                                                 |
| P-5, "a pentagon's farthest territory is its antipodal twin"                 | Merged into P-4, then withdrawn with it.                                                                                                                     |
| P-7, "the smallest planet has no six-neighbour territories"                  | Derivable from P-6 plus a line Sean had already written - the minimum is 12, **a dodecahedron**.                                                             |
| P-9, "the distance between every pair is computed once and stored"           | An implementation directive, not a rule of the game.                                                                                                         |
| P-13, "the greatest distance is 3 / 5 / 6 / 7 / 9 by planet size"            | Determined by the Goldberg choice and the size, nothing leans on it, and the numbers are **already asserted by a test**.                                     |
| P-15, "Native life is a planet's own, Feral is printed life gone wild"       | **Feral is behavioural, not an origin**, and origin is not substantively relevant.                                                                           |
| P-16, "every unit carries a name that persists when control changes"         | **A unit has a type, and the type has a name.** Individual units of the same type are not distinguished.                                                     |
| P-17, "depart is left unspecified so one rule covers biological and machine" | Sean chose **starves**, committing to the biological reading for now; robots come later. Recorded in [the backlog](spec-backlog.md).                         |
| P-20, "extracting one resource has no effect on extracting any other"        | Written against the rating model and contradicted by the node model: **labor is shared**, so working a food extractor does compete with working a metal one. |
| P-29, "a territory's threat level comes from what is on it"                  | Superseded by P-32. Threat is no longer a quantity a territory carries - it is one direction of **force**.                                                   |
