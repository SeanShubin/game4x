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

### P-139 - A recipe, not a transformation

**to** sean · **status** open · **raised** 2026-08-31 · **kind** recovered · **into**
`spec/invariants.md` -> The game is data, **and** `releases/first-release.md` -> Transformations

**Sean's, decided 2026-08-31**, converging the vocabulary. **Thing** and **recipe** are the words;
everything else was a synonym in flight.

> `spec/invariants.md` -> The game is data, three lines, *transformation* becoming *recipe*:
>
> - Every kind of thing, and every recipe that turns some things into others, is data rather than
>   code
> - A recipe expresses inputs and outputs. Where its inputs are available, it can be used to create
>   its outputs. Some inputs are consumed and some are not, and an input says whether its quantity
>   is a least or a most
> - Most recipes are options the player may take. The rest are not offered at all: they apply
>   themselves wherever they match, and nobody chooses

> `releases/first-release.md`, the section heading and the table's first column head:
>
> - `## Recipes`
> - `| Recipe | Scope | Role | Thing | Qty | Consumed | Bound |`

**Reworded a third time, from Sean's own sentences.** His: *recipes represent options the player may
take in the game. Recipes express inputs and outputs. Where inputs are available, the recipe can be
used to create an output. Some inputs are consumed, others are not.*

**That is better prose than anything this lane produced** and it is kept nearly whole. *Expresses
inputs and outputs* and *where its inputs are available, it can be used* are his, and they say in
plain words what three attempts said awkwardly.

**Two things were added back, and the first is load-bearing.**

**The bound.** *A least or a most* is missing from his version, and without it two rows of the
release cannot be written. Counted: **twenty-seven inputs say *at least* and two say *at most***, and
both of the two are *garrison, 0* - the way *this territory is not already held* is expressed. There
is no other way to say an absence, because an absence is not a quantity of anything.

**The unoffered case.** His first sentence defines a recipe **as** a player option, which would make
the five that end a turn - eat, grow, depart, spoil, ready - not recipes at all, and ending a turn
would go back to being a hand-written rule at the centre of the game. **So *most* rather than
*represent***, which keeps his reading true of the ten it is true of and leaves the five somewhere.

**What is deliberately not resolved here.** Whether the unoffered five deserve a name of their own is
a real question and this proposal does not answer it. It says only that they are the same shape, which
the release's table shows, and that nobody chooses them.

**Amended again, and this lane was wrong twice about the same sentence.** Sean: *recipes are options
available to me where their inputs are present. What are the other cases you are talking about?*

**He is right about the first half, and it shows the reworded version was wrong on a second axis.**
The previous attempt contrasted *at one place* with *at every place its inputs are present* - but an
offered recipe is available at every place its inputs are present **too**. A Yard can be built in any
territory holding 15 metal. **What differs is not how many places, it is whether anybody chooses.**

**The other cases, which is what he asked.** Six things happen without being chosen:

| Not chosen                                                                                    | When                         |
| --------------------------------------------------------------------------------------------- | ---------------------------- |
| eat, grow, depart, spoil, ready                                                               | ending a turn                |
| a unit lost when its upkeep is unpaid - `spec/units.md`                                       | ending a turn                |
| nature taking a territory back when force falls below its force of nature - `spec/control.md` | **whenever it becomes true** |

**The last one is why the distinction is about cause and not timing.** Move a garrison away and nature
reclaims the ground at that moment, not at the end of the turn. **So the axis cannot be *during the
turn* against *ending it*** - it has to be *somebody chose* against *nobody did*.

**And that is the whole of it**: everything in the release's Scope column marked `every` is something
nobody chooses, and everything marked `here` is an option. The column is named for the symptom rather
than the cause.

**One consequence for the release, not folded in.** `releases/first-release.md`'s Scope column reads
`here` and `every`. Under this wording those become something like *offered* and *automatic*, which is
a release change and gets its own cleanup rather than riding along in a specification proposal.

**Amended 2026-08-31: the third line is reworded and not only renamed.** Sean asked what *applies
either where it is invoked or everywhere it matches* means, **and asking is the evidence.** Three
faults in nine words: *invoked* does not say by whom; the two halves are not parallel, since one is
about **who chose the place** and the other about **how many places**; and nothing says the choice
belongs to the recipe rather than being made each time it is used.

**What it means, which the release already shows.** Ten of the fifteen recipes are `here` - `build
yard 11` is pointed at territory 11, and whoever uses it says which. Five are `every` - `eat` is
pointed nowhere and runs wherever its inputs sit. **The five are exactly what ending a turn consists
of**: eat, grow, depart, spoil, ready.

**And that is why the line exists.** Without *every*, ending a turn is a hand-written rule at the
centre of the game that no recipe describes. With it, ending a turn is five ordinary recipes with a
different scope, and nothing is special.

**`P-130` promoted the old wording**, so this is a correction to a landed line rather than a rename of
it. It is folded in here because these are the same three lines and splitting it would put two
proposals against one sentence.

**Basis:** a rename usually earns nothing. **This one removes an ambiguity that was already in the
specification and that nobody had noticed.**

`spec/turn.md` says a turn has three parts: *producing, which is the player acting, then consuming
and **transforming**, which are what ending it does.* `spec/invariants.md` says *a **transformation**
is a set of inputs and a set of outputs.* **Two different things, near-identical names, four
documents apart** - one is a phase of the turn, the other is any rule that turns things into other
things. The turn's *transforming* phase does not contain the recipes; every phase runs them.

**So the rename is not tidying, it is separating two concepts that had one name.** After it,
*transforming* is unambiguous because nothing else is called that.

**Why *recipe* rather than the alternatives.** *Transition* is taken and load-bearing -
`spec/invariants.md` says *a game state and a transition yield a new game state*, and a transition is
**one applied step** where a recipe is **the definition**. Keeping them distinct is what lets a
history record transitions while the rules hold recipes. *Formula* and *rule* are both already used
for other things.

**Why *thing* stays *thing***, since it is the one word that sounds like a placeholder. It is already
in `spec/turn.md` - *a thing created during a turn begins ready* - it is the predecessor's word, and
its vagueness is exactly right: anything narrower excludes something the model holds. **`entity` is
the one to avoid**, because `docs/architecture.md` rule 6 spends a paragraph saying an entity is not
where a fact about the game is kept.

**Four words, each doing one job**: a **thing** has a **kind**; a **recipe** is a definition; applying
one produces a **transition**. Three of the four are already in the specification.

**What this does not cover.** `docs/` and the notes are this lane's and are being corrected without a
proposal. The accepted-proposal ledger keeps its old wording, because those rows record what Sean
approved and he approved something called a transformation. And `prototypes/kinds` follows the
release, which is the code lane's to change once this lands.

### P-140 - Two recipes the table does not have

**to** sean · **status** open · **raised** 2026-08-31 · **kind** entailed · **into**
`releases/first-release.md` -> Transformations

**Found by asking what *the rest* are in `P-139`.** The release's table has fifteen rows and the game
has seventeen recipes. Two of the unoffered ones are in `spec/` and nowhere in the release:

> | Recipe    | Scope | Role | Thing                      | Qty | Consumed | Bound       |
> | --------- | ----- | ---- | -------------------------- | --- | -------- | ----------- |
> | **upkeep** | every | in   | unit with upkeep           | 1   | no       | at least    |
> |            |       | in   | food, its upkeep           | 1   | yes      | at least    |
> | **perish** | every | in   | unit whose upkeep is unpaid | 1   | yes      | at least    |
> | **revert** | every | in   | territory, force below its force of nature | 1 | no | at least |
> |            |       | out  | territory, unclaimed       | 1   |          |             |

**Basis:** `spec/units.md` says *a unit may require upkeep each turn, and is lost if it is not paid*,
and `releases/first-release.md` gives a Pioneer *1 food per turn* in the units table. **Neither is a
row in the recipe table**, and `eat` does not cover it - that row names a **citizen**, not a unit.

`spec/control.md` says *should the force in a territory fall below its force of nature, nature takes
it back. Its entire population perishes and any ark on it becomes unusable.* **That is not in the
release at all**, in either table.

**Why it matters more than a missing row.** `P-139` says most recipes are options and *the rest apply
themselves wherever they match*. **A reader counting the rest finds five and there are seven** - so
the table understates the class the sentence is about, and the two it omits are the two that can take
things away from a player without being chosen. **Those are exactly the ones a player most needs to
see written down.**

**The names are this lane's and are the weakest part.** *Upkeep*, *perish* and *revert* describe what
happens; the specification names none of them, because until now none of them needed naming. Rename
freely.

**One thing left undone deliberately.** *Its entire population perishes and any ark on it becomes
unusable* has a second half about arks that no row here expresses, because *unusable* is not a state
anything else in the release has. **It wants a decision rather than a guess**, so `revert` above stops
at the territory.

## Addressed to other perspectives

Items this lane has sent outward. **Nothing here waits on Sean** - the open proposals above are the
only thing that does.

### S-4 - A compilable specification of the kinds and the transformations

**to** code · **status** **acted** 2026-08-31 · **raised** 2026-08-31 · **source** Sean, and `P-130` · **cited** `3ca8675`, `7ced668`

Sean wants the kinds and transformations **in a form he can compile and read** - his words: *something
like a sql specification with enums and foreign keys, or a set of rust data types with hardcoded
values and enums... I don't need anything playable, I just want to see what the inputs to the
gameplay logic would be.*

**The content already exists and is not the work.** `releases/first-release.md` -> Units and
structures and -> Transformations carry every kind and all fifteen transformations, and `P-130` fixes
the shape: a transformation is inputs and outputs, and each input says how many, whether it is
consumed, and whether its quantity is a least or a most.

**What is asked for.** A prototype crate - not the shipped model - holding those two tables as Rust
data: enums for the kinds, a struct per transformation, the figures hardcoded. No gameplay logic, no
turn, no board. **Its whole job is to be read and to compile.**

**And one test, which is what makes it worth building rather than writing twice.** Render the data
back into the release's two tables and compare against the file on disk, the way
`crates/game-console/tests/quotations.rs` reads a sentence off disk. Then the compilable
specification and the written one cannot drift, and the check is wired to the gate rather than to
somebody remembering.

**Four things it will force into the open, which is the real value.** None of them can be answered
from prose and all four block a real implementation:

- **`work` outputs `density` of a resource** - a quantity that is not a constant but a property of
  the node being worked. What type is a quantity?
- **`move` takes *unit, here* and yields *unit, there*** - so location is a trait of a thing rather
  than a container holding it. How is a trait that varies per instance typed?
- **`node, unworked` and `food, surplus` are not kinds**, they are differences between two counts.
  Either the data gains derived kinds computed from stored ones, or the shape gains comparisons.
  **Each appears in exactly one row**, which is the measure of what that choice costs.
- **Scope** - *here* against *everywhere it matches*. Is that a field on a transformation, or two
  different types?

**Where it goes is the code lane's**, and `docs/prototypes/README.md` applies: state the question up
front and record the answer when it has one. The question here is *what do the inputs to the gameplay
logic actually look like*, and the answer is whatever the four above turn out to be.

**Not blocking anything.** The release is specified without it. This exists so Sean can review the
shape before it is built into the model, which is cheaper than reviewing it after.

### S-3 - Which cells make the twelve reachable, measured rather than guessed

**to** code · **status** open · **raised** 2026-08-30 · **source** `C-8`, for `P-126`

`C-8` establishes that the loop cannot reach steps 7 and 8. Deciding what to change needs two things
this lane cannot produce.

**The adjacency of `canonical_seeds(12)`, printed.** `game4x --dump` reports every territory's nodes
and not its neighbours, so the reachability argument in `C-8` cannot be re-run outside your lane.
It is the load-bearing half of the finding and it should be checkable by anyone.

**The smallest change that makes every territory reachable and an Ark producible somewhere
reachable.** You already have the capacity arithmetic. What Sean needs is two or three measured
candidates rather than one guess - ideally ones that keep each territory's stated demonstration
intact, since that is what the table is for.

One worked example, to show the shape rather than to propose it: territory 1's role is *the landing
site, everything works*, and it cannot build a Yard - twelve metal against fifteen. Raising its metal
density from 4 to 5 gives exactly fifteen and makes its stated role true. Whether anything then
reaches 10, 11 and 12 is the part that needs the graph.

### S-2 - The crate enumerations in `docs/architecture.md` need a gate, not a rewrite

**to** code Â· **status** **acted** 2026-08-30 Â· **cited** `2ac3ab9` Â· **source** `C-5`, paired with `Q-37`

That document enumerates every crate twice - the table of layers and dependencies, and rule 5's
requirement that each crate's `README.md` be linked from it. Both have gone stale twice: once when
`planet-terrain` landed, and again now, with `planet-raster`, `planet-flat` and `game-globe`.

**Asked for: a test that fails when a crate in the workspace has no row, and when a row names a
crate that is not there.** The same check covers rule 5, since a row carries the README link.

**Not a generated table**, and this is where the pairing with `Q-37` stops rather than continues.
The gate's exclusion list was right because the thing being fixed *was* the gate, so a detector
would have needed something trustworthy to report to and there was nothing. A table check has no
such problem: the gate is now the trustworthy thing, so a test in it is wired to a failure by
construction. Coverage-by-default is the right instinct and it is already satisfied here by
`--workspace` being what the test iterates.

**This lane will not hand-rebuild the table a third time**, and has not, though it is stale as this
is written. Anything written during a refactor that is moving crates is wrong within the hour. It
gets rebuilt once, when the split lands and the test can hold it.

### S-1 - `tools/outbox` should read `releases/*.md`

**to** code Â· **status** **acted** 2026-08-30 Â· **cited** `2ac3ab9`

Each capability in `releases/first-release.md` now carries an id, `R-1` to `R-6`, and the `**to**
code` field line every outbox item carries. The tool does not look in `releases/`, so all six are
invisible to `outbox --to code` - which is the one place they need to appear, since they are the
work the release exists to order.

## Accepted

| Proposal                                                                                                        | Landed in                                                                                                              | Date       |
| --------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ---------- |
| P-1, the `10T + 2` territory counts, as a consequence of the Goldberg choice                                    | `spec/planet.md` -> Shape                                                                                              | 2026-08-25 |
| P-6, every territory has five or six neighbours; exactly twelve have five                                       | `spec/planet.md` -> Shape                                                                                              | 2026-08-25 |
| P-8, adjacency is a shared edge, never a shared corner                                                          | `spec/planet.md` -> What a territory carries                                                                           | 2026-08-25 |
| P-10, the planet is presented as a three-dimensional sphere                                                     | `spec/planet.md` -> Presentation                                                                                       | 2026-08-25 |
| P-11, the roll for any point on the planet is fixed                                                             | `spec/planet.md` -> Presentation                                                                                       | 2026-08-25 |
| P-12, every change to game state is a console command                                                           | `spec/invariants.md` -> Everything is expressible                                                                      | 2026-08-25 |
| P-14, the Ark and the Seeder                                                                                    | `spec/unit-types.md`                                                                                                   | 2026-08-25 |
| P-19, territories have a rating per resource                                                                    | `spec/planet.md` -> What a territory carries                                                                           | 2026-08-25 |
| P-21, resources exist in a place; a cost is paid where it is spent                                              | `spec/logistics.md`                                                                                                    | 2026-08-25 |
| P-23, territories have an id, unique per planet, starting at 1                                                  | `spec/planet.md` -> What a territory carries, Presentation                                                             | 2026-08-25 |
| P-18, a planet's resources are infinite; the rate is finite                                                     | `spec/economy.md` -> Structures and labor                                                                              | 2026-08-25 |
| P-33, species coexist or prey on each other; nature never exterminates                                          | `spec/control.md` -> Wildlife, **cut again 2026-08-26**                                                                | 2026-08-26 |
| P-37, a citizen is the smallest group that can sustain reproduction                                             | `spec/population.md` -> Citizens                                                                                       | 2026-08-26 |
| P-28, an Ark produces the founding citizens; nothing else produces citizens                                     | `spec/population.md` -> Citizens                                                                                       | 2026-08-26 |
| P-26, the population acts on its own; the AI designs, the population operates                                   | `spec/narrative.md` -> The population                                                                                  | 2026-08-26 |
| P-25, the Ark prints the founding population; the AI designs life generally, selection finishes it              | `spec/narrative.md` -> Life                                                                                            | 2026-08-26 |
| P-22, everything is modelled: nothing changes without a cause inside the model                                  | `spec/invariants.md` -> Everything is modelled                                                                         | 2026-08-25 |
| P-31, territories have nodes for each resource, and nodes have density                                          | `spec/planet.md` -> What a territory carries; example in `spec/economy.md`                                             | 2026-08-25 |
| P-30, infrastructure is never a liability; setbacks come from outside                                           | `spec/invariants.md` -> No penalty for building infrastructure                                                         | 2026-08-25 |
| P-24, distance is fixed; roads change traversal, not distance                                                   | `spec/planet.md` -> Distance                                                                                           | 2026-08-25 |
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
| P-114, rules outlive a game and can be given away                                                               | `spec/invariants.md` -> Control without tedium                                                                         | 2026-08-28 |
| P-117, a player's rules always finish                                                                           | `spec/invariants.md` -> Control without tedium                                                                         | 2026-08-28 |
| P-115, a rule is a source of transitions, not a kind of one                                                     | `spec/invariants.md` -> The game is one function                                                                       | 2026-08-28 |
| P-116, the rule editor is a fourth surface, and it is two-dimensional                                           | `spec/interface.md` -> Surfaces                                                                                        | 2026-08-28 |
| P-120, a rule carries the number of turns it may run                                                            | `spec/invariants.md` -> Control without tedium                                                                         | 2026-08-29 |
| P-119, every rule has a text form, and the text is the rule                                                     | `spec/invariants.md` -> Control without tedium                                                                         | 2026-08-29 |
| P-121, `/save <file>` writes the history to a file                                                              | `spec/console.md` -> Commands                                                                                          | 2026-08-29 |
| P-118, the rule editor is out of the first release, and the surfaces line says so                               | `releases/first-release.md` -> Scope, Controls                                                                         | 2026-08-29 |
| P-122, a capability for playing the loop through by hand                                                        | `releases/first-release.md` -> Capabilities                                                                            | 2026-08-29 |
| P-123, neither the biome rule nor the connectivity rule yields                                                  | `spec/planet.md` -> What a territory carries                                                                           | 2026-08-30 |
| P-125, every structure built everywhere it can be built, and what that means                                    | `spec/control.md` -> Winning                                                                                           | 2026-08-30 |
| P-127, `show` says what can be done, not only what is true                                                      | `spec/console.md` -> Commands                                                                                          | 2026-08-30 |
| P-128, a surface is never more capable than the console                                                         | `spec/invariants.md` -> Everything is expressible                                                                      | 2026-08-30 |
| P-129, a territory holds only so much of each kind of thing                                                     | `spec/logistics.md` -> Capacity                                                                                        | 2026-08-31 |
| P-130, the kinds and the transformations are data                                                               | `spec/invariants.md` -> The game is data                                                                               | 2026-08-31 |
| P-126, metal and energy carry between turns, food does not, and each resource is conserved or not               | `spec/resources.md` -> The list, `spec/turn.md` -> Order of operations                                                 | 2026-08-31 |
| P-131, units and structures as one table                                                                        | `releases/first-release.md` -> Units and structures                                                                    | 2026-08-31 |
| P-132, the first release's transformations as one table                                                         | `releases/first-release.md` -> Transformations                                                                         | 2026-08-31 |
| P-133, which things ready                                                                                       | `releases/first-release.md` -> Units and structures                                                                    | 2026-08-31 |
| P-134, the state is things                                                                                      | `spec/invariants.md` -> The game is data                                                                               | 2026-08-31 |
| P-135, competing effects are resolved together, and nothing wins by being first                                 | `spec/turn.md` -> Order of operations                                                                                  | 2026-08-31 |
| P-136, when effects compete, and when they merely follow                                                        | `spec/turn.md` -> Order of operations                                                                                  | 2026-08-31 |
| P-137, purge founding                                                                                           | `spec/console.md`, `spec/control.md`, `spec/population.md`, `spec/unit-types.md`                                       | 2026-08-31 |
| P-138, order is spent: matter is conserved, arrangement is not                                                  | `spec/resources.md` -> The list, `spec/turn.md` -> Order of operations                                                 | 2026-08-31 |

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
| P-124, "where a generated file lives"                                                 | Housekeeping rather than a decision, under the split Sean approved 2026-08-30. Settled by the specification lane and landed in `CLAUDE.md` -> Perspectives in the same commit.                   |