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

### P-114 · Recovered · `spec/invariants.md` -> the same new section

**Sean's, stated 2026-08-28.** The meta-layer.

> - A rule is not part of any one game. It can be named, kept, used in a later game, and given to
>   another player
> - A rule does the same thing for whoever holds it

**Basis:** Sean grants the reason games hide this - *"it would be a pain in the ass to respecify
scouting every game, which is why there is going to need to be a meta-layer of the game where players
can store"* - and wants community builds baked in rather than left to a wiki: *"I would like to be
able to take someone else's build from online, plug that into my game."*

**This is a second kind of tedium and P-111 does not touch it.** A rule that dies with its game means
paying the specification cost every new game, which is worse than the automation it replaced. **Rules
have to be objects with a life outside any one game** - and once they are objects, giving one away is
the same act as keeping one.

**The second line is the sharing guarantee, and it constrains.** *Does the same thing for whoever
holds it* forbids a rule whose behaviour depends on anything but the game it is applied to - no hidden
local state, no dependence on who wrote it. Without it, *shareable* means the file copies, not that it
works.

**Sean's builds are a stronger thing than the precedents he cites, and it is worth him knowing why.**
A StarCraft build order is a **sequence** and a Path of Exile passive build is a **fixed allocation**;
both work because the game is the same every time. **A planet is generated**, so a food-generation
build cannot be a recording of what someone did - it has to be a policy that responds to what the
planet gives it. Two consequences: it degrades rather than breaks, since a build applied to a planet
with no jungle simply never fires its jungle rules; and **a published build is also a benchmark** -
two builds on one seed is a controlled experiment, which is the same machinery as the large-planet
testing that was Sean's reason for doing this now.

**Open, and it does not block this line.** Once builds are shared, the names a rule uses become a
compatibility surface: Path of Exile builds break every league because a published artifact
referenced a vocabulary that moved. What a stale build does - fail, warn, or skip the rule it cannot
resolve - wants deciding, and the wrong answer is the silent one.

### P-115 · Entailed · `spec/invariants.md` -> The game is one function

Follows from P-114 and the section it joins. **Claude's, and it settles a question P-114 would
otherwise leave open.**

> - A rule is a source of transitions, not a kind of one. The history records what a rule did,
>   exactly as if the player had done it by hand

**Basis:** a rule is stored outside the game and can arrive from another player, so when one spends a
player's output for them, **the history must record either the commands or the fact that a build did
it.** The existing lines leave no freedom: *there is no other way for state to change*, and *a game
state is exactly the result of applying every transition in order*. If the history recorded the
attachment rather than the actions, replaying it would require a file that is not part of the game.

**Filed rather than left as a note because P-114 makes it live.** Rules that outlive a game are
exactly the rules that can go missing, change under you, or arrive from a stranger. Landing P-114
without this would put a dependency on an external file inside the fold, which is the one thing this
invariant forbids.

**What it buys is larger than tidiness.** A saved game replays without the build that produced it, so
a game can be shared or re-examined a year later with no dependency on a file that may have vanished.
**The rule is readable and so is everything it did** - which is Sean's transparency requirement
satisfied in both directions at once, since reading the rule tells you the intent and reading the
history tells you what actually happened.

### P-116 · Recovered · `spec/interface.md` -> Surfaces

**Sean's, stated 2026-08-28**, answering the question P-111 left open.

> The game presents four surfaces, all reachable from the front end, in every build:
>
> - **The rule editor** - the rules the player has, read and changed

> The rule editor is two-dimensional. It may carry three-dimensional decoration, and nothing the
> player has to read or act on is in that decoration.

**Basis:** *"Regarding the rules editor itself, this definitely needs to be its own screen, and the
interface needs to be 2d, although I am not opposed to decorative 3d elements mixed in."*

**Two mechanical consequences of promoting this**, stated so the move stays pure: the sentence above
the list changes **three** to **four**, and the new bullet joins the existing three rather than
replacing any.

**The second paragraph is Claude's sharpening of *decorative*, and Sean should judge it.** He said he
is not opposed to decorative 3D; the added clause is what makes *decorative* checkable rather than a
matter of taste. **It forbids exactly one thing** - a 3D element the player must read in order to use
the editor - which is what would otherwise creep in and make the screen 3D by degrees. If it says more
than he meant, cut the second paragraph and the first still stands.

**It also settles the HUD question without needing a HUD.** A heads-up layer over the 3D scene and a
2D screen of its own are different answers, and this is the second - so the editor is never drawn over
a planet, and the two need share no camera, scale or input model. That is the same separation
`spec/planet.md` already draws between the practical and realistic drawings: **either one or the
other, never both at once.**

**Availability follows from the section it joins and needs no extra line.** *Nothing is available in
one build and not another* already applies, so a terminal build must offer the rule editor - and *how
a thing is presented may follow the platform it runs on* already licenses whatever that turns out to
mean there.

### P-117 · Recovered · `spec/invariants.md` -> the same new section as P-111

**Sean's, stated 2026-08-28**, turning a suspicion into a requirement. He should judge whether he
wants it held that firmly.

> - A player's rules always finish. Nothing that can be built in the rule editor runs forever

**Basis:** Sean, raising goto, assignment and looping - *"I suspect preventing infinite loops is
something we can guarantee by construction."*

**It is worth stating because everything else leans on it.** The argument for letting the rule editor
be as expressive as it can be is that nothing built there can hang; without this line that argument
has no support, and the natural drift is toward a while loop the moment something is awkward to
express.

**The wording is deliberate: what can be *built*, not what is *accepted*.** The difference between
those two is the difference between a programming language and a user interface - **a programming
language's defining property, from a player's side, is that you can write something wrong and find
out later.** A line saying invalid rules are rejected would permit exactly the experience Sean is
trying to avoid.

**It states the requirement and not the constructions that satisfy it**, which is the pattern four
earlier proposals were corrected for missing. Three constructions are known and they are in
[the note](control-without-tedium.md) - every firing takes a game action, conditions are finite
queries over game state, rule references form an acyclic graph. **They give a bound rather than mere
termination**: at most one firing per thing that can act, a number the planet fixes in advance.
Naming them here would foreclose a fourth.

**Not merged into P-112, though both concern the same editor.** P-112 is about how small a change can
be; this is about what cannot be built at all. A rule set could satisfy either and fail the other.

**It corrects the record on P-112's basis.** That proposal claimed the guarantee falls out of the
exhaustion rule alone. It does not: the exhaustion argument covers the outer loop and holds only while
every firing takes an action. The corrected paragraph is in P-112 above.

## Accepted

| Proposal                                                                                                        | Landed in                                                                                                              | Date       |
| --------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ---------- |
| P-1, the `10T + 2` territory counts, as a consequence of the Goldberg choice                                    | `spec/planet.md` → Shape                                                                                               | 2026-08-25 |
| P-6, every territory has five or six neighbours; exactly twelve have five                                       | `spec/planet.md` → Shape                                                                                               | 2026-08-25 |
| P-8, adjacency is a shared edge, never a shared corner                                                          | `spec/planet.md` → What a territory carries                                                                            | 2026-08-25 |
| P-10, the planet is presented as a three-dimensional sphere                                                     | `spec/planet.md` → Presentation                                                                                        | 2026-08-25 |
| P-11, the roll for any point on the planet is fixed                                                             | `spec/planet.md` → Presentation                                                                                        | 2026-08-25 |
| P-12, every change to game state is a console command                                                           | `spec/invariants.md` → Everything is expressible                                                                       | 2026-08-25 |
| P-14, the Ark and the Seeder                                                                                    | `spec/unit-types.md`                                                                                                   | 2026-08-25 |
| P-19, territories have a rating per resource                                                                    | `spec/planet.md` → What a territory carries                                                                            | 2026-08-25 |
| P-21, resources exist in a place; a cost is paid where it is spent                                              | `spec/logistics.md`                                                                                                    | 2026-08-25 |
| P-23, territories have an id, unique per planet, starting at 1                                                  | `spec/planet.md` → What a territory carries, Presentation                                                              | 2026-08-25 |
| P-18, a planet's resources are infinite; the rate is finite                                                     | `spec/economy.md` → Structures and labor                                                                               | 2026-08-25 |
| P-33, species coexist or prey on each other; nature never exterminates                                          | `spec/control.md` -> Wildlife, **cut again 2026-08-26**                                                                | 2026-08-26 |
| P-37, a citizen is the smallest group that can sustain reproduction                                             | `spec/population.md` → Citizens                                                                                        | 2026-08-26 |
| P-28, an Ark produces the founding citizens; nothing else produces citizens                                     | `spec/population.md` → Citizens                                                                                        | 2026-08-26 |
| P-26, the population acts on its own; the AI designs, the population operates                                   | `spec/narrative.md` → The population                                                                                   | 2026-08-26 |
| P-25, the Ark prints the founding population; the AI designs life generally, selection finishes it              | `spec/narrative.md` → Life                                                                                             | 2026-08-26 |
| P-22, everything is modelled: nothing changes without a cause inside the model                                  | `spec/invariants.md` → Everything is modelled                                                                          | 2026-08-25 |
| P-31, territories have nodes for each resource, and nodes have density                                          | `spec/planet.md` → What a territory carries; example in `spec/economy.md`                                              | 2026-08-25 |
| P-30, infrastructure is never a liability; setbacks come from outside                                           | `spec/invariants.md` → No penalty for building infrastructure                                                          | 2026-08-25 |
| P-24, distance is fixed; roads change traversal, not distance                                                   | `spec/planet.md` → Distance                                                                                            | 2026-08-25 |
| P-42, a count is a density across the territory; an Ark restarts a population from zero                         | `spec/population.md`; the zero-return half **cut 2026-08-26** by P-64, the density line moved into Citizens            | 2026-08-26 |
| P-44, each planet has its own native species                                                                    | `spec/planet.md` -> Native life (filed against What a territory carries; rescoped on promotion)                        | 2026-08-26 |
| P-45, force of nature is inherent to a territory; taking needs greater, holding needs equal                     | `spec/control.md` -> Force, and Gaining and holding ground                                                             | 2026-08-26 |
| P-41, a turn resolves produce, then consume, then transform                                                     | `spec/turn.md` -> Order of operations                                                                                  | 2026-08-26 |
| P-53, the poles are visible on the planet                                                                       | `spec/planet.md` -> Presentation                                                                                       | 2026-08-26 |
| P-61, no action has an intermediate step that is always taken                                                   | `spec/invariants.md` -> No step that is always taken                                                                   | 2026-08-26 |
| P-60, a founding unit takes a territory and becomes a structure, a citizen and a food extractor                 | `spec/unit-types.md` -> Founding units, and `releases/first-release.md`                                                | 2026-08-26 |
| P-63, taking takes force greater than the existing force, whatever holds it                                     | `spec/control.md` -> Gaining and holding ground (replaced the nature-only bullet)                                      | 2026-08-26 |
| P-62, losing your population when no Ark remains is losing the game                                             | `spec/control.md` -> Gaining and holding ground                                                                        | 2026-08-26 |
| P-64, a player has lost with no citizens and nothing that converts into one                                     | `spec/control.md` -> Losing; the Zero section deleted from `spec/population.md`                                        | 2026-08-26 |
| P-32, force is the capacity for violence; organised force sums, unorganised is the highest                      | `spec/control.md` -> Force, and Coordination                                                                           | 2026-08-26 |
| P-54, territories resolve in claim order; unused resources are discarded at end of turn                         | `spec/turn.md` -> Order of operations                                                                                  | 2026-08-26 |
| P-57, command files as subroutines; query commands; a sequence runs interactively or as a test                  | `spec/console.md` -> The language, and Commands                                                                        | 2026-08-26 |
| P-55, a citizen provides labor each turn, spent until the end of the turn                                       | `spec/population.md` -> Labor                                                                                          | 2026-08-26 |
| P-35, one garrison per territory; it makes citizens' force sum and multiplies it                                | `spec/control.md` -> Producing force, `spec/structures.md`, `releases/first-release.md`                                | 2026-08-26 |
| P-58, every territory carries the same nodes: 6 food at 6, 4 metal at 8, 5 energy at 7                          | `releases/first-release.md`, after Scope (filed against Units and structures; moved on promotion)                      | 2026-08-26 |
| P-59, each territory is self-contained; only a mobile unit crosses a boundary                                   | `releases/first-release.md` -> Scope                                                                                   | 2026-08-26 |
| P-47, the loop: land the ark founding a territory, then build force, units and spread                           | `releases/first-release.md` -> The loop (steps 1-4 replaced, later steps renumbered)                                   | 2026-08-26 |
| P-48, the structure a founding unit becomes has one less force, operated by citizens                            | `spec/unit-types.md` -> Founding units                                                                                 | 2026-08-26 |
| P-49, the resources are food, metal and energy                                                                  | `spec/resources.md` -> The list                                                                                        | 2026-08-26 |
| P-38, citizens do not self-coordinate; a structure or a military unit imposes it                                | `spec/control.md` -> Coordination                                                                                      | 2026-08-26 |
| P-39, violence is inherent, coordination is imposed                                                             | `spec/narrative.md` -> Violence and order                                                                              | 2026-08-26 |
| P-52, every territory has a force of nature of 1                                                                | `releases/first-release.md` -> Scope                                                                                   | 2026-08-26 |
| P-34, a citizen works at one structure and cannot be in two places at once                                      | `spec/economy.md` -> Structures and labor (filed against Extraction; retargeted)                                       | 2026-08-26 |
| P-50, units have force, movement and upkeep; a cost may be anything you control, paid in place                  | `spec/units.md`, and `spec/logistics.md` -> Paying a cost                                                              | 2026-08-26 |
| P-51, one generic Extractor; a farm is an extractor on a food node                                              | `spec/structures.md` -> The list, and `releases/first-release.md` (Farm entry deleted)                                 | 2026-08-26 |
| P-65, food is for population, metal for building, energy for moving                                             | `spec/resources.md` -> The list                                                                                        | 2026-08-26 |
| P-66, a mobile unit carries energy cells, filled where it is built                                              | `spec/units.md` -> What a unit is                                                                                      | 2026-08-26 |
| P-27, a Yard produces Arks; the Garrison narrows to land units; the Foundry is cut                              | `spec/structures.md` -> The list                                                                                       | 2026-08-26 |
| P-68, twelve designed territories, each exercising a different consequence                                      | `releases/first-release.md` -> Territory nodes                                                                         | 2026-08-26 |
| P-67, rebalanced costs: Pioneer 16 metal, extractors labor only                                                 | `releases/first-release.md` -> Units and structures; the Yard repriced 64 -> 30 on 2026-08-26, unbuildable as promoted | 2026-08-26 |
| P-74, a game is designed, then started, then played                                                             | `spec/console.md` -> Phases                                                                                            | 2026-08-26 |
| P-70, an Ark costs 24 metal and 24 energy and needs a Yard                                                      | `releases/first-release.md` -> Units and structures                                                                    | 2026-08-26 |
| P-71, orbit is one place; launching and landing each spend a cell                                               | `spec/orbit.md`                                                                                                        | 2026-08-26 |
| P-75, the whole game is one function from state and transitions to state                                        | `spec/invariants.md` -> The game is one function                                                                       | 2026-08-26 |
| P-69, the console command set, its syntax, help, history and error requirements                                 | `spec/console.md`                                                                                                      | 2026-08-26 |
| P-72, a change made any way is indistinguishable from the command that would make it                            | `spec/invariants.md` -> Everything is expressible, **cut again 2026-08-26** as derivable from P-11 and P-75            | 2026-08-26 |
| P-73, three surfaces - the game, the console, the data browser - in every build                                 | `spec/interface.md` -> Surfaces                                                                                        | 2026-08-26 |
| P-76, four design-phase commands: create planet, add node, set force, add unit                                  | `spec/console.md` -> Commands                                                                                          | 2026-08-26 |
| P-77, a planet is fully exploited when nothing more can be taken, built or stored                               | `spec/control.md` -> Winning                                                                                           | 2026-08-26 |
| P-79, the movement allowance is deleted; the spent flag limits how often a unit acts                            | `spec/units.md` and `releases/first-release.md`                                                                        | 2026-08-26 |
| P-78, producing happens in any order; a spent flag limits it, and ending a turn clears it                       | `spec/turn.md` -> Order of operations (both bullets replaced, the discard bullet absorbed)                             | 2026-08-26 |
| P-80, every cost halved so the landing site can expand                                                          | `releases/first-release.md` -> Units and structures                                                                    | 2026-08-27 |
| P-81, the win clause names a storage structure, not a store of resources                                        | `spec/control.md` -> Winning                                                                                           | 2026-08-27 |
| P-82, `run <file>` and `#` comments; `run` is not a transition and is not in history                            | `spec/console.md`                                                                                                      | 2026-08-27 |
| P-83, a citizen has a force of its own; the first release sets it to 1                                          | `spec/control.md` -> Producing force, and `releases/first-release.md`                                                  | 2026-08-27 |
| P-84, a garrison is not built; founding is the only source of one                                               | `spec/control.md` -> Producing force                                                                                   | 2026-08-27 |
| P-85, six release lines reconciled with the spec: transforms, the loop, fuel, the stale note                    | `releases/first-release.md`                                                                                            | 2026-08-27 |
| P-86, a Pioneer must found on leaving friendly territory or perish                                              | `releases/first-release.md` -> Scope                                                                                   | 2026-08-27 |
| P-87, a cost is paid in the territory, not at a building site                                                   | `spec/logistics.md` -> Paying a cost                                                                                   | 2026-08-27 |
| P-88, the poles sit at the centres of two pentagons, never on a boundary                                        | `spec/planet.md` -> Presentation                                                                                       | 2026-08-27 |
| P-89, availability is fixed in every build; presentation and input follow the platform                          | `spec/interface.md` -> Availability and presentation                                                                   | 2026-08-28 |
| P-90, input bindings move to the release; roll is explicitly not user-controlled                                | `spec/planet.md` -> Presentation, and `releases/first-release.md` -> Controls                                          | 2026-08-28 |
| P-91, Controls names a binding for every capability the spec requires                                           | `releases/first-release.md` -> Controls                                                                                | 2026-08-28 |
| P-92, actions that are not manipulations of the planet get on-screen controls                                   | `spec/interface.md` -> Availability and presentation                                                                   | 2026-08-28 |
| P-93, a line beginning with `/` names a surface, not a command; reaching one is typed where there is no pointer | `spec/console.md` -> Commands, and `spec/interface.md`                                                                 | 2026-08-28 |
| P-94, a slash directs the front end; `/new <size>` abandons the fold and starts another                         | `spec/console.md`, and `releases/first-release.md` -> Controls                                                         | 2026-08-28 |
| P-95, the requirement stops prescribing a mechanism; a slash form is not a transition                           | `spec/interface.md`, `spec/console.md`, `releases/first-release.md` -> Controls                                        | 2026-08-28 |
| P-96, two drawings, practical and realistic, sharing only the camera                                            | `spec/planet.md` -> Presentation                                                                                       | 2026-08-28 |
| P-97, the realistic drawing's terrain is continuous and crosses boundaries                                      | `spec/planet.md` -> Presentation                                                                                       | 2026-08-28 |
| P-98, nothing in the terrain reveals how the sphere was divided                                                 | `spec/planet.md` -> Presentation                                                                                       | 2026-08-28 |
| P-99, each territory has a biome                                                                                | `spec/planet.md` -> What a territory carries                                                                           | 2026-08-28 |
| P-100, a territory's biome is what the terrain gives it                                                         | `spec/planet.md` -> What a territory carries                                                                           | 2026-08-28 |
| P-101, four capabilities for the visual work, each with a vetted-when                                           | `releases/first-release.md` -> Capabilities                                                                            | 2026-08-28 |
| P-102, the six biomes; ocean is unclaimable and never adjacent to ocean                                         | `spec/planet.md` -> What a territory carries                                                                           | 2026-08-28 |
| P-103, what each biome gives a territory, and why every force of nature is 1                                    | `releases/first-release.md` -> Biomes                                                                                  | 2026-08-28 |
| P-107, the realistic drawing shows terrain and no borders                                                       | `spec/planet.md` -> Presentation                                                                                       | 2026-08-28 |
| P-104, a drawing never betrays how it was made                                                                  | `spec/planet.md` -> Presentation                                                                                       | 2026-08-28 |
| P-105, a biome has a margin, not a border                                                                       | `spec/planet.md` -> Presentation                                                                                       | 2026-08-28 |
| P-109, oceans never isolate land from land                                                                      | `spec/planet.md` -> What a territory carries                                                                           | 2026-08-28 |
| P-110, `set biome` gives a territory its biome during design                                                    | `spec/console.md` -> Commands                                                                                          | 2026-08-28 |
| P-108, the biome check states plurality, not majority                                                           | `releases/first-release.md` -> Capabilities                                                                            | 2026-08-28 |
| P-106, a fifth capability: terrain resolved as finely as it is shown                                            | `releases/first-release.md` -> Capabilities                                                                            | 2026-08-28 |
| P-111, control without tedium: rules instead of repetition                                                      | `spec/invariants.md` -> Control without tedium                                                                         | 2026-08-28 |
| P-112, the middle layer: rules compose, and edits stay proportional                                             | `spec/invariants.md` -> Control without tedium                                                                         | 2026-08-28 |
| P-113, nothing plays itself, and every rule can be read                                                         | `spec/invariants.md` -> Control without tedium                                                                         | 2026-08-28 |

## Rejected

Nothing yet. Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

## Withdrawn

| Proposal                                                                              | Why                                                                                                                                                                                              |
| ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| P-2, "twenty planet sizes are available below 500"                                    | Superseded by Sean's edit fixing the game at five named sizes.                                                                                                                                   |
| P-3, "no two territories are more than `3m` apart"                                    | Superseded by the per-size statement, and incomplete: `3m` holds for class I only, while the large planet is class III where the measured distance is 7.                                         |
| P-4, "the twelve five-neighbour territories sit in six antipodal pairs"               | Derivable from the Goldberg choice, and no rule leans on it.                                                                                                                                     |
| P-5, "a pentagon's farthest territory is its antipodal twin"                          | Merged into P-4, then withdrawn with it.                                                                                                                                                         |
| P-7, "the smallest planet has no six-neighbour territories"                           | Derivable from P-6 plus a line Sean had already written - the minimum is 12, **a dodecahedron**.                                                                                                 |
| P-9, "the distance between every pair is computed once and stored"                    | An implementation directive, not a rule of the game.                                                                                                                                             |
| P-13, "the greatest distance is 3 / 5 / 6 / 7 / 9 by planet size"                     | Determined by the Goldberg choice and the size, nothing leans on it, and the numbers are **already asserted by a test**.                                                                         |
| P-15, "Native life is a planet's own, Feral is printed life gone wild"                | **Feral is behavioural, not an origin**, and origin is not substantively relevant.                                                                                                               |
| P-16, "every unit carries a name that persists when control changes"                  | **A unit has a type, and the type has a name.** Individual units of the same type are not distinguished.                                                                                         |
| P-17, "depart is left unspecified so one rule covers biological and machine"          | Sean chose **starves**, committing to the biological reading for now; robots come later. Recorded in [the backlog](spec-backlog.md).                                                             |
| P-20, "extracting one resource has no effect on extracting any other"                 | Written against the rating model and contradicted by the node model: **labor is shared**, so working a food extractor does compete with working a metal one.                                     |
| P-29, "a territory's threat level comes from what is on it"                           | Superseded by P-32. Threat is no longer a quantity a territory carries - it is one direction of **force**.                                                                                       |
| P-36, "accidental damage is force 1, a predator is force 2"                           | Superseded on 2026-08-26: **force is inherent to the territory**, not carried by individual creatures, so there is nothing for a per-creature value to attach to.                                |
| P-40, "the least force eats from food nodes; every species grows by the citizen rule" | Superseded on 2026-08-26. Nature has no population and **does not use nodes** - a node is intentional exploitation. The whole food chain goes with it.                                           |
| P-43, "nothing is exterminated; coordination buys suppression"                        | Superseded on 2026-08-26. It described populations held at zero, and nature no longer has a population to hold anywhere.                                                                         |
| P-46, "citizens and food move between adjacent territories"                           | Cut on 2026-08-26. Sean removed logistics for now so that **each territory is self-contained**; the only thing crossing a boundary is a mobile unit. Recorded in [the backlog](spec-backlog.md). |
| P-56, "a territory satisfies its own consumption first"                               | Cut with P-46 on 2026-08-26 - it only had work to do while a remainder could reach a neighbour.                                                                                                  |