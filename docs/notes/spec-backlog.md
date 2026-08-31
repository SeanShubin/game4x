# Spec Backlog

**Derived.** Written by Claude from conversation, 2026-08-25. Not binding - it is a list of
things Sean has *said* but has not yet *written*, and only the writing counts.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Design intent stated in conversation that has not reached [the specification](../../spec/README.md).
When an item lands in the spec, delete the row. Nothing here is decided.

## Resolved

**`spec/overview.md` is dropped.** Genre, inspirations and theme moved to
[vision](../vision.md); the spec holds mechanical detail only.

## Deferred: "starves" will not cover a machine population

`spec/population.md` says an unfed citizen **starves**. That was chosen deliberately over the
more abstract "departs" on 2026-08-25 - *"for now lets just make it starves, we will get to
robots later."*

It is a debt rather than a defect. Sean intends worlds rich in energy but hostile to life,
worked by machines rather than biologicals, and a machine has no food to be denied. When that
arrives, either the word widens again or the machine case needs a rule of its own.

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

### Resolved: they starve - **this entry was wrong and is superseded**

**Superseded 2026-08-31.** It said an unfed citizen *departs* and that the word was deliberately
unspecified. **Sean chose *starves***, `P-17` was withdrawn for exactly that reason, and
`spec/population.md` says *each unfed citizen starves* while `spec/turn.md` says *starves for want of
it*. There is no such line in `spec/economy.md` at all.

**Found by the re-read trigger** on 2026-08-31, when `P-126` landed in a section that had already
taken two proposals. The note had disagreed with the specification since the day it was written, and
nothing looked.

What survives is the mechanic below, which is right whichever word is used. What does not is the
claim that the word is left open: **it is not, and the debt that creates is the deferred entry above**
- *starves* will not cover a machine population, and when robots arrive either the word widens again
or the machine case gets a rule of its own.

The original reasoning, kept because it is why the debt exists:

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

### Supply lines cut a unit off

Stated by Sean on 2026-08-26: *"mobile units are vulnerable to having their supply lines cut off,
and are lost if so."* P-50 carries the rule that a unit is lost when its upkeep is unpaid; what is
not written anywhere is **what a supply line is** - whether it is a path of controlled territories,
a distance from a structure, or something else. It cannot be written until logistics returns, since
nothing crosses a territory boundary today.

Sean drew the boundary the same day: the *no penalty for building infrastructure* invariant covers
**infrastructure, not the military**. Structures cost nothing to keep; units do.

### Supply shocks, and routes that get cheaper when you clear them

Stated by Sean on 2026-08-26, as design intent rather than a rule:

> Supply lines matter, and you are actually vulnerable to supply shocks. The idea that clearing
> an area of pirates opens up a cheaper route for resource flow was fantastic, and supply lines
> are going to be a big part of this game.

He added that he does not know whether any of it fits in the first release. It does not, and
nothing is lost by that - **the substrate is already in the specification**, so this arrives later
as content rather than as a redesign. What is already there:

| Piece                                                | Where                                                                 |
| ---------------------------------------------------- | --------------------------------------------------------------------- |
| A unit dies when its upkeep goes unpaid              | `spec/units.md` - this is what makes a supply shock hurt              |
| Moving things costs energy                           | P-65, once promoted - this is what makes a route have a price         |
| A cost is paid where it is spent                     | `spec/logistics.md` - this is what stops a player pooling one economy |
| Taking ground needs more force than is already there | `spec/control.md` - this is what makes clearing a route an actual act |
| Force of nature is a property of each territory      | `spec/control.md` - this is what makes some ground expensive to cross |

**The Distant Worlds effect may need no new rule at all.** If moving costs energy per territory
crossed, and a route is a path of territories, then a shorter or safer path is *literally* cheaper -
not by a rule that says so, but by arithmetic. Clearing a territory that sits between two of yours
shortens every route through it at once. That is the feeling Sean is describing, and it falls out
of pieces that already exist rather than needing a supply-route mechanic bolted on.

**What is genuinely undecided** is how a hostile or uncontrolled territory affects a route:
whether it blocks passage outright, whether it costs more energy, or whether goods crossing it are
lost at some rate. Blocking is the simplest and gives the sharpest supply shock - an enemy taking
one territory severs everything beyond it. Rates are more Distant Worlds and more forgiving.

**Nothing in the first release forecloses any of it.** Movement already has a cost, resources
already have locations, territories already have adjacency and force. The risk to watch for is the
opposite one: **do not let the first release give the player a pooled inventory or free movement**,
because taking those away later is a redesign, whereas adding routes to a world that already
counts locations and distances is content.

### Perlin noise hotspots, when nodes go random

Sean, 2026-08-26: *"when we do go random, I am thinking I want to use perlin noise to create
hotspots that can be discovered by following increasing values."* **Explicitly not the first
release and not a proposal.** The first release uses a designed twelve-territory fixture instead -
see P-68 - because at that size chosen numbers exercise the mechanics more reliably than a roll.

**What the idea buys.** Independent rolls per territory make node counts noise in the statistical
sense: knowing one territory tells you nothing about its neighbours, so there is nothing to
prospect for. A smooth field makes richness *correlated across adjacency*, which turns exploration
into a gradient you can climb. That is a different activity from revealing a map, and it is the
reason to prefer noise over rolling.

**One constraint worth recording before anyone implements it.** The planet is a sphere - a Goldberg
polyhedron - so the field has to be sampled in **three dimensions at each territory's centre**, not
on a two-dimensional projection. [Region schemes](region-schemes.md) and
[the planet-view prototype](../prototypes/planet-view.md) both record that any flat map of a sphere
folds, and the folds land on the poles. A 2D noise field inherited from a projection would put a
visible seam of discontinuity there, and the gradient a player is following would break exactly at
the two places [the spec already marks as visible](../../spec/planet.md). Sampling 3D noise on the
unit sphere has no seam anywhere.

**And the gradient must be legible through adjacency**, since that is the only thing a player can
walk. The useful property is that neighbouring territories differ little and distant ones differ
more - which is what a noise frequency low relative to territory size gives. Too high a frequency
and neighbours are uncorrelated again, which is the rolled planet with extra steps.

### A single document to hand the coding instance

Sean, 2026-08-26: *"perhaps we will eventually need to create a document I can point the coding
instance to for the full implementation."*

Not yet written. What it would have to gather, none of which lives in one place today:

- `spec/` for the rules, `releases/first-release.md` for the figures - already the two canonical
  sources and already linked from [the spec index](../../spec/README.md)
- [Parser and assembler architecture](parser-architecture.md) for what a proper command interpreter
  must do, now framed as requirements rather than as a port
- [`docs/architecture.md`](../architecture.md) for the crate layout and the ECS rules, including
  that model ids rather than Bevy entity ids are canonical identity
- The **setup command** vocabulary, which does not exist yet and which nothing can be built without
  - see P-73

**The obstacle is not writing it, it is that it goes stale.** A hand-assembled digest disagrees with
its sources the first time a proposal lands. If it is written it should be a **map** - one page
saying which file answers which question - rather than a copy of what those files say.

### Refuelling a unit after it is built

Sean, 2026-08-26: *"in general I think we are going to need the ability to refuel after something
is built, but perhaps not in this case since we have no need of standing armies yet."*

**Nothing in the specification currently lets a built unit take on energy.** P-66 fills a unit's
cells when it is built and says nothing further. `spec/logistics.md`'s *whatever pays a cost must
be in the territory where it is spent* does **not** cover this - that rule is about the cost of
producing something, and refuelling produces nothing.

**The trigger is standing units.** Every unit in the first release is a founding unit, and P-61
makes founding consume it, so no unit lives long enough to run dry. The first unit expected to
**persist** - a defender, a transport, anything that is not spent on arrival - is the point at
which this must be written.

**Worth deciding when it is:** whether refuelling is free where energy is present, costs a turn,
or needs a structure. The last would give a reason to build depots at the frontier and would pair
with the storage entry below.

### A planet can be generated to a requested biome distribution

Sean, 2026-08-28: *"I want some distribution of biomes in territories but I don't actually care where
they are."*

**The outcome, which is what has to be written down somewhere:** a generator is asked for a mix -
mostly forest and ocean, some desert and grassland - and produces a planet whose territories match
it, without being told where anything goes.

**It is checkable, which is what makes it a requirement rather than a wish.** Generate a planet, count
the territories of each biome, compare to what was asked. That is a **vetted when** line waiting for
the release that delivers generation, and it names no technique:

> **Vetted when** - a planet generated from a requested mix has, for each biome, a territory count
> within a stated tolerance of the request

**How it would be done is recorded and is not the requirement.**
[Generating versus designing](generating-versus-designing.md) argues for moving the cut points in
parameter space rather than placing biomes, because that keeps coherence: nothing is moved, only the
lines are redrawn, and lines are not visible in the world. **That belongs to whoever implements it.**

**Why it is here rather than in the queue.** The first release generates nothing - its twelve
territories are hand-designed - so a capability entry would promise something that release does not
do. This entry exists so the requirement is not lost between now and the release that can carry it.

**And the reason it needed writing at all**, on 2026-08-28: Claude said the cut-point idea should not
be a proposal because it is a technique, and Sean asked how we then make sure it happens. **The
answer is that the technique is not the thing to ensure - the outcome is** - and Claude had discarded
the outcome along with the mechanism. Anything worth making happen can be stated as something
someone can check; if it cannot, what is wanted is not yet known.

### Biomes want revisiting every time a rule learns to read a territory

Sean, 2026-08-28: *"we will need to re-address biomes as we add new mechanics that enable more
substantive diversity of biomes."*

**Biomes are as distinct as the rules can see, and no more.** The rules read exactly two things
about a territory - its nodes and its force of nature - so six biomes chosen by role collapse into
**four**, because two of the six were distinguished by things nothing reads. The failure is not in
the biome list; it is that the list has more resolution than the rules do.

**So the trigger is not a date, it is a capability.** Every time the rules gain something new to
read about a territory, the biome table gets one more axis to differ along, and should be re-read.
The candidates are set out in [the biome note](biomes.md):

| When the rules can read                 | Biomes could then differ in                    |
| --------------------------------------- | ---------------------------------------------- |
| A per-territory movement cost           | how expensive ground is to cross               |
| The territory, from the population rule | where people grow well                         |
| A structure's requirements              | what can be built where                        |
| Which native species lives where        | danger with a stated cause rather than by fiat |

**Jungle is the one waiting, and it is deferred rather than broken.** Sean, 2026-08-28: *"yes we
are going to need that, but it is correct that for the first playable game I want force of nature to
always be one. I need a baseline to tune from."*

**Two independent reasons put force at 1, and they agree**, which is why nothing needs fixing:

- **Chosen.** A single value everywhere is a baseline. Every later number is tuned against a game
  that is known to be playable, and a variable that never varies cannot be the cause of anything.
- **Forced.** P-63 says taking a territory needs force **greater** than what is there, and both
  founding units have force 2 - so a territory at force 2 or more could never be taken by anything
  that exists. Even a decision to vary it now would have had nowhere to go.

**What Jungle costs while it waits.** It was picked to be the one biome that is good and dangerous at
once - the only one producing a real decision rather than a preference. With danger capped it is
grassland with less food, so the first release has **six biomes and four distinct roles**.

**Recovering it changes no rule.** Rescale the force numbers together - founding units at 4,
dangerous ground at 3 - and P-48's *the structure a founding unit becomes has one less force* still
lands exactly. It is release tuning, not a spec change, which is what makes it safe to defer.

**And the note to re-read is [biomes](biomes.md)**, which sets out the six roles and the test they
were chosen against: does knowing the biome change what a player *does* with a territory, or only
how pleased they are to have it.

### Terrains beyond land, and the units that cross them

Sean, 2026-08-28: *"we are going to eventually add land, sea, air, submersible, subterranean, space,
and units that can traverse multiple terrains, but I want a playable game before we get into all
that detail."*

Six terrains, then. Today the game has one - a unit moves between adjacent territories on the ground
- plus orbit, which `spec/orbit.md` treats as a single place rather than as a terrain.

**The near-term consequence is a restriction that expires.** `spec/planet.md` requires that oceans
never isolate land from land, so that every land territory is reachable on foot. **That lifts the
moment a unit can operate in an ocean territory.**

It was tighter until 2026-08-28: *no two ocean territories are adjacent*, which made water a
scattering of isolated cells with no coastline and no sea. Sean relaxed it to the requirement the
adjacency rule had merely been sufficient for. **What remains forbidden is an island** - a land
territory unreachable from the rest - and that is precisely what a ship would fix.

**Worth deciding when the time comes**, and not before: whether a terrain is a property of a
territory (so a territory is one of six) or a property of the boundary between two (so crossing is
what is permitted or not). A biome already makes the first reading natural, and orbit already makes
the second one exist - launching crosses between layers rather than moving within one.

### Why the size list stops at five

Sean, 2026-08-30, answering `prototypes/goldberg-view`: *the first 5 goldberg polyhedrons are fine
as planets... the others after the first 5 all look like planets but the issue isn't really
appearance, it is the diminishing returns on strategic depth regarding large planet sizes. Two units
with ranges of 5 or 6 vs 50 or 51 have different gameplay feels.*

**`spec/planet.md` already fixes five sizes and does not say why.** The reason is recorded here so a
later session does not helpfully propose the sixth: twenty Goldberg counts exist below 500, all of
them build, and the constraint on stopping is not the geometry or the drawing.

**The argument generalises past range.** Every quantity the game counts in territories - range,
distance, movement, the reach of a weapon, how far a supply line stretches - is a whole number, and
one step of it is a fifth of the span on a twelve-territory planet and a fiftieth on a large one.
**Resolution per step falls as the planet grows**, so the same rule set makes fewer distinguishable
decisions on a bigger world.

**What would change the answer** is a mechanic whose interest does not scale with the count: several
players, or terrain that makes some ground expensive to cross, or logistics, where distance buys
something rather than merely costing. Until then a larger planet is more of the same board rather
than a deeper one.

### Automation budgets: unbounded now, measured now, priced later

Sean, 2026-08-29: *"I need to experiment with more freedom than makes sense for a regular player to
have so I can tune it later. For now lets make the budget finite but effectively infinite... For now,
lets make them concepts I can collect data on."*

**This is a schedule, not a rule, which is why none of it is in the queue.** Three things were
decided and only the third needs anyone to do anything.

**The budget belongs to the rule, and the number is overkill.** Sean, 2026-08-29: *"analogous to
pre-allocated stack space for a function. We will just set an overkill number that will tell us
something has gone horribly wrong if we ever actually hit it."* So a rule carries its own allowance -
that half is in P-120 - and **the provisional half is what exhausting it means**. For now, hitting the
number is a defect report: the rule was expected to stop because it reached its goal or ran out of
things to do, and the budget only ever fires when neither happened.

**That is why it is here rather than in the spec.** Under fragments the same exhaustion would be an
ordinary stop rather than a fault, so a spec line committing to either reading would go stale the
moment the other arrived. P-120 says the rule carries the number and stays silent on what running out
means, which is true under both.

**What the backstop actually catches is not what it looks like.** Runaway recursion - the usual cause
of a stack overflow - is already impossible, since rule references are acyclic and the editor never
offers the cycle. What is left is a rule that keeps finding something legitimate to do for ever
without reaching its goal, which no static check can see. See
[control without tedium](control-without-tedium.md) for why that is a different failure from the one
cycle detection was aimed at.

**Fragments are deferred.** Whether the budget becomes an in-game artifact - *AI fragments*, acquired
and spent - waits until a few builds exist. See
[control without tedium](control-without-tedium.md) for what the idea buys and the one choice that
decides whether it encourages modular design at all: **a rule used by three parents must cost one
element, not three**, or the system rewards copy-paste.

**The budget does not bind during design, and that is not a conflict with P-120.** That line says a
rule runs for *at most a stated number of turns*. A very large number is stated and finite, so the
invariant holds untouched while the number is out of the way. **Nothing has to be relaxed or
excepted** - which is worth noticing, because a rule that needed suspending during design would be a
badly drawn rule.

**The dimensions are measured while they are unbounded, and this is the part that gets lost if nobody
writes it down.** Limits have to be set from data. If the first builds are written without recording
how many turns they ran, how deep they nested and how many elements sat in one rule, **the numbers to
set the limits from do not exist**, and they cannot be recovered afterwards without rewriting the
builds. Measuring is nearly free while the builds are being made and expensive to reconstruct later.

So the checkable form, which is what makes it happen rather than remain an intention:

> **Vetted when** - running a build reports the turns it used, the greatest depth it reached, and the
> largest number of elements in a single rule

**The designer's licence here is a shape the specification already has.** `spec/console.md` carries
design-phase-only commands - `add node`, `set force`, `set biome` - that no play-phase rule would
permit, on the grounds that the designer is the cause of what appears there. **A budget that does not
bind while Sean is designing builds is the same exception rather than a new kind**, so nothing needs
inventing to allow it.

**Where the experiments can run without any new surface.** P-118 keeps the rule editor out of the
first release, and this strengthens that rather than straining it: Sean wants *more freedom than makes
sense for a regular player*, and **a user interface is precisely the thing that would confine him to
what it offers**. So the text form is the experimental surface and the editor is the player surface -
P-119 plus `run <file>` is close to enough to begin.

**What is still undecided is scheduling and not design.** No release file covers automation at all;
`releases/first-release.md` is one tiny planet and the eight-step loop, and P-118 states the editor is
out. **Whether rules-as-text join that release or get one of their own is Sean's call**, and nothing
here can be built until one of those happens.

### Conservation, and units defined as data

Sean, 2026-08-30: *perhaps a pioneer is actually just a personnel transport made of metal that stores
fuel so it can move. However it could be stripped down for parts in order to be consumed in
transformation formulas.* And: *I am leaning towards a more data driven game where the units and
transformations are simply data inputs to rust, and rust is providing a statically typed engine to
run and validate the data.*

**Leaning, not decided**, so it sits here rather than in the queue. The working-out is in
[everything is matter](everything-is-matter.md); three things from it are worth having where the
schedule is read.

**Conservation collapses three limits into one.** Sean has asked for a bound on stored resources, a
bound on units *for the same reason*, and storage structures with capacity. If a unit **is** the
metal it was made from and so is a structure, all three become *a territory holds only so much
matter, and everything in it is matter* - which closes the finiteness question outright rather than
per category.

**It bears on `P-126`'s open fork.** That proposal asks what sets a territory's bound, and offers a
default the territory has against capacity that only a structure grants. Under conservation the
first is the natural answer, because a territory must already hold the matter of anything standing
on it before any storage is built.

**Only metal can be conserved.** Energy is spent moving and food is eaten; metal is the one resource
that is still a thing after it is used. That happens to match `P-126`'s carry/spoil split, arrived at
from a different direction, and the two should not be allowed to drift apart.

### Two shapes of logistics, and they are not the same game

Sean, 2026-08-30: *the logistics I had planned involved being able to use resources from adjacent
territories, but I can also implement logistics more directly by creating units that can transport
resources.*

Both are recorded because the choice is not an implementation detail - it decides what a player
spends attention on.

**Reach.** A territory may spend from its neighbours. Nothing is built and nothing moves; the rule
simply widens what *paid from that territory's store* means. Cheap to specify, cheap to play, and it
makes adjacency the whole of geography: a territory's worth becomes the sum of what it can touch.
**The danger is that it removes a decision rather than adding one** - if reach is automatic, the
player never chooses to move anything.

**Carriage.** A unit picks resources up, crosses, and puts them down. Every transfer is a thing
somebody built, fuelled and routed, so distance costs, routes can be cut, and *clearing an area
opens a cheaper route* becomes true by arithmetic rather than by a rule saying so - which is the
Distant Worlds effect [the backlog already records](#supply-shocks-and-routes-that-get-cheaper-when-you-clear-them)
as falling out of pieces that exist. **The danger is tedium**: a fleet of haulers is exactly the
thing [control without tedium](control-without-tedium.md) exists to answer, and it is not answerable
until rules can be written.

**They compose, and the order matters.** Reach first makes carriage feel like a downgrade, because
the player already had the resources. Carriage first makes reach feel like an upgrade a technology
might grant. **Nothing needs deciding until logistics returns**, and this note exists so the second
option is not forgotten - the original plan was reach, and carriage arrived later and is arguably the
better one.

### Does every structure cost metal, or can a territory bootstrap?

Sean, 2026-08-30, on his own glossary saying *metal is required to build all structures and units*
while an Extractor costs one labour and nothing else: *this reveals a design tension I am working
out, I have to choose between a certain kind of consistency or make sure I can bring extra
resources.*

**Recorded rather than resolved.** Both sides are real and the choice is his.

**What consistency buys.** One rule - everything built costs metal - is easier to state, easier to
price, and makes metal genuinely the building resource rather than the resource for the two
expensive things. A player learns one economy instead of two.

**What it costs, and this is the sharp end.** An Extractor is how a territory turns ground into
anything at all. If it costs metal, then **a territory with no metal node can never build one** -
territory 6 would be permanently stuck with the single food extractor founding gives it, unable to
work its own energy at density 5. The free Extractor is what lets a territory bootstrap from
nothing, and it is the reason founding hands over an extractor rather than bare ground.

**So the tension is between one economy and self-starting ground**, and *bringing extra resources*
is the third way out: if metal can arrive from elsewhere, a metal-poor territory can still build,
and the consistency costs nothing. **That is logistics**, which is cut from this release, so the
choice is only forced while nothing crosses a boundary.

**Worth deciding with `P-126` rather than after it.** Storage changes what a territory can afford
over time but not what it can ever obtain: with stores carrying, territory 6 accumulates energy for
ever and still has no metal. So storage answers the *lumpy demand* half and leaves this one exactly
where it was.

### Storage, spoilage and decay

Stated by Sean on 2026-08-26, all as later work:
> **The schedule moved, 2026-08-30.** Sean: *I am anticipating having to add storage a bit earlier
> than originally planned to make the scenario winnable.* Measuring it confirms the reason - with
> metal and energy carrying, the first release becomes winnable with its node table untouched, which
> is `P-126`. What that proposal takes is only the carrying; storage **structures** and a capacity
> ceiling stay here.


- **Storage facilities for energy and metal**, while food spoils initially
- **Later technology lets food last longer**
- **Decay as a general mechanic** - food spoils, but radioactive materials also lose potency over
  time

**This is the structural fix for a mismatch that already exists.**
[`spec/turn.md`](../../spec/turn.md) discards unused resources every turn, and
[the balance trace](first-release-balance.md) shows why that bites: food demand is **continuous**
and metal and energy demand is **lumpy**, so the two lumpy resources arrive every turn with nothing
to buy and are thrown away. Storage is what lets lumpy demand meet steady supply.

Which makes *food spoils, metal and energy store* the right asymmetry rather than an arbitrary one:
**the resource that spoils is the one whose demand never pauses, so buffering it would buy nothing
anyway.** Flavour and mechanism agree, which is worth preserving if the rule is ever written
differently.

### A transport unit for energy

Sean, 2026-08-26: *"I might just need a fuel transport unit to bring energy across the map."*
Raised alongside energy cells (P-66) as the alternative to giving every unit its own range. Cells
are the leaner of the two and are what he is considering first; a transport is what becomes
necessary if energy has to reach a territory that cannot extract it.

### A structure with a logistics value - considered and cut

Sean, 2026-08-26: a structure carrying a number where *0 means any material can be instantly
transported within the same territory, 1 meaning it can be transported 1 territory away in 1
turn*. **Cut for simplicity**, in favour of energy cells on the unit. Recorded so it is not
re-proposed: it needs a structure, a range rule and a projection radius to reach where one stat on
a unit already reaches.

### Logistics - the core loan from Factorio and Distant Worlds

> **Deferred for the first release, 2026-08-26.** Sean cut logistics *for now* so that each
> territory is self-contained and only a mobile unit crosses a boundary - see P-59. The intent
> below is unchanged and still the destination; it is the schedule that moved. Two proposals were
> withdrawn with it, P-46 and P-56.

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
