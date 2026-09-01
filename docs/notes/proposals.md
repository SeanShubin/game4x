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

### P-141 - A unit carries fuel, not cells

**to** sean · **status** open · **raised** 2026-08-31 · **kind** recovered · **into** `spec/units.md`,
**and** `releases/first-release.md` -> Units and structures, Recipes

**Sean's, decided 2026-08-31**: *perhaps we can replace cell with fuel capacity. So units have a gas
tank, they burn a unit of fuel every time they move.*

> `spec/units.md`, the two lines about cells become:
>
> - A mobile unit holds fuel, up to a capacity of its own. Moving burns a unit of it, and a unit with
>   none cannot move
> - Fuel moves freely between a controlled territory that has it and anything there that can
>   hold it

> `releases/first-release.md` -> Units and structures: the **Cells** column becomes **Fuel**, and the
> **A move** column reads `1 fuel` for the Ark and the Pioneer.

> `releases/first-release.md` -> Recipes: the two `cell, on that unit` ingredients become
> `energy, in that unit`.

**Basis:** it removes a kind rather than adding one, and **the specification already said so**.
`spec/units.md`: *a unit's cells are filled when it is built, and **the energy** is paid in the
territory that built it.* **A cell was always energy in a wrapper** - the wrapper had a name, a
column and two recipe ports, and no rule of its own.

**So the kinds table has twelve rows and not thirteen**, which was the question blocking everything
else in [the data inventory](first-release-data.md). `cell` was the doubtful kind, and the answer is
that it is not one.

**And it makes a unit a container, which `P-129` already describes.** *A container occupies capacity
of its own kind and provides capacity of another.* **A fuel capacity is exactly that** - a unit takes
up room in a territory and provides room for fuel inside itself. So *gas tank* is not a new mechanic;
it is the capacity rule applied to something that was already carrying energy around.

**Amended 2026-08-31, one word, before Sean approves.** The ingredient was written `fuel, in that
unit` and is now `energy, in that unit`. **`fuel` as an ingredient name would put the wrapper back**,
which is the whole thing this proposal removes: there would again be a name in the recipes that is
not a kind. The **Fuel** column stays, because there it names a *capacity* - how much energy the unit
can hold - and a capacity is a trait rather than a kind.

**Amended again 2026-08-31, and this one is a rule rather than a word.** Sean: *fuel should be freely
transferable between a controlled territory that has it and a thing that can hold it.* The line that
said a unit's fuel is put in when it is built was **too narrow, not wrong** - fuelling at build time
is one case of the general rule, so the general rule replaces it rather than joining it.

**Free means no labor and no limit but capacity**, and one move a turn is what bounds it: a unit that
moves is exhausted, so nothing can shuttle. **It also raises the stakes on upkeep.** Loading a
transport costs nothing, so a vehicle parked in a controlled territory is storage with no friction at
all - and upkeep is then the only thing making that a choice rather than a free win.

**And it makes a territory's containers one pool for reach and many for the bound.** Within a
controlled territory a player never has to say which bin holds what, because it can always be moved
for nothing; the container boundaries decide **how much** can be kept and nothing else.

**One thing left as it is, deliberately.** *Fuel* rather than *energy* in the unit's column, because
Sean said fuel and the word does work: energy in a store and energy in a tank behave differently -
one can be spent on anything and one only on moving. **They are the same resource and the distinction
is where it sits**, which is what a container is for.

### P-142 - A quantity is a number

**to** sean · **status** open · **raised** 2026-08-31 · **kind** recovered · **into**
`releases/first-release.md` -> Recipes

**Sean's, decided 2026-08-31**: *I would like a quantity to always be an integer, unless there is a
compelling reason otherwise.*

> The three ports reading `any` become `1`:
>
> | Recipe | Role | Thing | Qty |
> | ------ | ---- | ----- | --- |
> | **spoil** | in | food | 1 |
> | **ready** | in | thing, exhausted | 1 |
> |           | out | thing, ready | 1 |
>
> And two quantities become lookups rather than literals:
>
> | Recipe | Role | Thing | Qty |
> | ------ | ---- | ----- | --- |
> | **work** | out | resource | the worked node's density |
> | **upkeep** | in | food | the unit's upkeep |

**Basis:** `any` was never a quantity. It meant *all of it*, and **the scope column already says that**
- both recipes are the world's, applied everywhere they match, so a recipe removing one food applied
everywhere removes all the food. **The `any` was the scope written into the wrong column**, and
deleting it changes no behaviour.

**And nothing else is an exception, because there never was one.** This lane wrote that `work`'s
`density` output *is not an integer*. **Sean: a food node of density 8 spent one labor gives 8 food -
where is the quantity that is not an integer?** He is right. Eight is an integer. **Every quantity in
the game is an integer and always was.**

**What varies is not the type but where the number comes from.** `15 metal` is written in the recipe.
`density` is read off one of the ingredients. **Both are integers; one is a literal and one is a
lookup**, and calling the second *not an integer* confused a question about representation with a
question about type.

**So the rule is simply his**, with no exception attached:

> A quantity is a whole number. It is written in the recipe, or read from a trait of one of the
> ingredients.

**And the correction found a second one this lane had mis-encoded.** `upkeep` reads *food, its upkeep*
at quantity **1** - but *its upkeep* is the amount, and 1 is only the Pioneer's. **The quantity was
put in the thing column as a note and a literal left in its place**, so a unit with upkeep 2 would eat
one food. Written as a lookup it is right for every unit.

**Which makes the lookup general rather than a special case for density.** `prototypes/kinds` types it
as `Exactly | Density | Any`; **`Any` goes away with this proposal, `Density` becomes a trait lookup,
and the type is two cases rather than three** - a literal, and a trait of a named ingredient.

### P-143 - The release does not declare its own vocabulary

**to** sean - **status** open - **raised** 2026-08-31 - **kind** gap - **into**
`releases/first-release.md`, four new sections before *Units and structures*

**Sean's, decided 2026-08-31**, answering three questions this was waiting on: **labor represents a
citizen operating a machine**; a **territory is a place that carries traits** rather than a thing;
and an ingredient refers to the other ingredients of the same recipe **by convention**, with no
names.

**The recipes reference eleven kinds and the release declares six.** *Units and structures* is named
correctly and contains exactly what it says; the gap is that nothing else is written down anywhere,
so an editor loading this release finds no row for `metal`.

> `## Kinds`
>
> | Kind | What it is |
> | --- | --- |
> | **citizen** | a person: provides labor, eats, and grows on surplus |
> | **garrison** | what holds a territory; a territory has at most one |
> | **extractor** | built on a node, so that a citizen can work it |
> | **yard** | where an Ark is produced |
> | **ark** | carries a landing, and can invade from orbit |
> | **pioneer** | founds a territory |
> | **node** | where a resource comes from; its density is what one turn's work yields |
> | **food** | eaten by citizens; expires |
> | **metal** | what things are built from; conserved |
> | **energy** | what moves things; neither conserved nor expiring |
> | **labor** | a citizen operating a machine |

> `## Families`
>
> | Family | Members |
> | --- | --- |
> | **thing** | every kind above |
> | **unit** | ark, pioneer |
> | **resource** | food, metal, energy |

> `## Places`
>
> A thing is always in a place, and a place may carry traits of its own.
>
> | Place | How many | Traits it carries |
> | --- | --- | --- |
> | **a territory** | 12 | biome, force of nature, its nodes, which territories adjoin it |
> | **orbit** | 1 | none |
> | **a container** | any unit | what kind it holds, and how much - here, energy as fuel |

> `## Traits`
>
> | Trait | Of | Values | Stored or derived |
> | --- | --- | --- | --- |
> | **kind** | every thing | one of the eleven | stored |
> | **place** | every thing | a territory, orbit, or a unit | stored |
> | **readiness** | whatever readies | ready, exhausted | stored |
> | **force** | citizen, garrison, ark, pioneer | a number | stored |
> | **fuel** | a unit | a number: how much energy it holds | stored |
> | **upkeep** | a unit | food per turn | stored |
> | **resource** | node, extractor | food, metal or energy | stored |
> | **density** | a node | a number | stored |
> | **arriving** | a pioneer | yes or no | stored, cleared at end turn |
> | **worked** | a node | yes or no | derived: an extractor is on it |
> | **surplus** | food | yes or no | derived: left after everything ate |
> | **unfed** | a citizen | yes or no | derived: it did not eat |

**Basis:** none of this is a new rule. Every kind, family and trait above is already referenced by a
recipe, a column or a promoted line, and the only act is writing them where the data can be loaded.
Two things in it are nonetheless **Claude's naming rather than Sean's**, and should be read as such:
the *What it is* column, and the trait names `readiness`, `arriving`, `worked`, `surplus` and
`unfed`, which the tables use as bare adjectives and never name.

**Sean's definition of labor holds in one of its two uses, and the conflict should be settled before
this lands.** `work` takes labor and an extractor, which is a citizen operating a machine exactly.
**`build extractor` takes labor and there is no machine** - the machine is what it produces. Two ways
out, and this lane has no view on which:

- **The definition says what labor is for, and is not a precondition.** Building is a citizen's
  effort too, so the sentence describes the usual case rather than a rule the engine checks
- **Building an extractor should not take labor.** It would take a citizen directly, and labor would
  be reserved for what a machine multiplies - which is also the only place `density` appears

**Why `fuel` is a container and not a kind.** Sean chose places-that-carry-traits, and `P-129` says a
container occupies capacity of its own kind and provides capacity of another. A unit holding energy
is both, so *inside a unit* is a place and **`fuel` is the size of it** - which is why the third row
of *Places* is there and there is no `fuel` row in *Kinds*.

**The convention, written once so that it is not inferred six times.** *An ingredient refers to the
other ingredients of the same recipe.* It resolves all six references because **no recipe in this
release names two units or two nodes**, so there is never a second candidate. It breaks the day one
does - combat, or a transfer between two units - and the fix then is the predecessor's `$name`
aliases from `language/Expressions.kt`. **Not adopting them now is a bet that first release ships
first**, and it is worth writing down as a bet rather than meeting it later as a surprise.

### P-146 - What a thing is made of, which `P-145` needs and `P-144` was carrying

**to** sean - **status** open - **raised** 2026-08-31 - **kind** gap - **into**
`releases/first-release.md` -> Units and structures

**Filed because withdrawing `P-144` would otherwise take something still needed with it.** That
proposal carried two halves. The flat per-territory capacities are wrong and are gone. **The
metal-content column is not**, and `P-145` reads it: without it there is no trait for a destroyed
unit's metal to be looked up from.

> *Units and structures* gains one column, **Metal in it**:
>
> | Thing | Metal in it |
> | --- | --- |
> | **citizen** | |
> | **garrison** | 1 |
> | **extractor** | 1 |
> | **yard** | 15 |
> | **ark** | 12 |
> | **pioneer** | 8 |

**Basis: it is not a new number.** It is what the thing cost, which *Costs to produce* already gives,
and conservation is what makes that the right answer rather than a convenient one - `spec/resources.md`
says a conserved resource *changes form, and what it was made into can be taken apart to get it back*,
so **the metal did not go anywhere**. A citizen is blank rather than zero because a citizen is not
built.

**Two of these six numbers are about to move**, and that is not a reason to wait. The Ark's 12 is
almost certainly wrong now: Sean's deployment lever means an Ark deploys into what it cost, so the
figure is whatever its deployment is worth - three, on the opening worked through today. The column
should exist either way, because **what changes is the cell and not the schema**, which is the whole
point of the tables.

### P-145 - `perish` destroys metal, which the specification says cannot happen

**to** sean - **status** open - **raised** 2026-08-31 - **kind** contradiction - **into**
`releases/first-release.md` -> Recipes

`spec/resources.md`: *a **conserved** resource is not destroyed by being used. It changes form, and
what it was made into can be taken apart to get it back*, and *matter is conserved and its
arrangement is not.* **`perish` consumes a unit and produces nothing.** A Pioneer costs 8 metal, so
an unpaid Pioneer deletes 8 metal from the game.

> `perish` gains an output:
>
> | Recipe | Scope | Role | Thing | Qty | Consumed | Bound |
> | --- | --- | --- | --- | --- | --- | --- |
> | **perish** | every | out | metal | the unit's metal | | |

**Basis:** this is Sean's own resolution, from 2026-08-31, applied to the one recipe in the release
that destroys a unit. *A destroyed vehicle renders no usable metal... perhaps the metal from a
destroyed vehicle should stay around for the turn, and metal does not expire until end turn happens
and there is no one to expend the labor to put it in a bin or no bin.*

**So the wreck is not a special case and needs no new kind.** It is metal, in the territory, under
the same rule as any other metal: `spec/resources.md` says a thing not kept in order is lost when the
turn ends, and that keeping it in order costs labor. **The metal appears and is then almost always
lost**, which is the intended feel - and it is what makes salvage something a player could later be
given, rather than something the engine has to know about.

**It is a lookup and not a literal**, in the shape `P-142` proposes: the quantity is the unit's
*Metal in it* trait, which `P-144` adds. **The three only work together** - without `P-144` there is
no trait to read, and without `P-142` there is no way to read one.

**One thing this does not fix.** `revert` hands a territory back to nature with everything on it, and
what becomes of that is stated nowhere. It is out of scope here because **`revert` may never fire in
first release** - nothing in the release reduces a territory's force - and that is worth confirming
before spending a rule on it.

## Addressed to other perspectives

Items this lane has sent outward. **Nothing here waits on Sean** - the open proposals above are the
only thing that does.

### S-5 - The gate is red, this lane moved the sentence, and this lane must not fix it

**to** code - **status** open - **raised** 2026-08-31 - **source** a blocked push - **cited**
`ba9f945`

**`cargo test -p game-console --test quotations` fails on `master` and blocks every push, including
report-only ones.**

```
crates/outbox.md
  attributes to spec/control.md: "every structure that can be built"
  which spec/control.md does not say
```

**The quotation was accurate when `C-7` was filed and this lane moved the sentence out from under
it.** `ba9f945`, promoting `P-125`, rewrote the line to *every structure has been built everywhere it
can be built*. `C-7` is withdrawn and its text is kept deliberately, which is right - and the test
does not distinguish a withdrawn finding from a live one.

**The fix is one word in `crates/outbox.md` and this lane may not make it**, because `crates/` is
yours. Naming it rather than doing it is the whole point of the boundary.

**Two ways, and the second is the interesting one.** Re-quote the current wording, or **let the test
allow a withdrawn item to quote what the specification said when it was filed**. `C-7` itself argues
for the second: *a finding is a claim about a specification at a moment, and the way it goes stale is
that the specification moves under it.* If that is right, then a withdrawn finding quoting old
wording is **not a defect the gate should catch** - it is the record working. This lane has no view
on which, and it is your file and your test.

**What this blocks meanwhile.** Five commits of specification work are committed and unpushed, and
`hooks/pre-push` runs the full gate, so a documentation-only push is held on a code test. Per
`CLAUDE.md` this lane says so and stops; `--no-verify` is Sean's call.

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

**to** code · **status** **withdrawn** 2026-08-31 · **raised** 2026-08-30 · **source** `C-8`, for `P-126`

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

**Withdrawn 2026-08-31, because the problem it asks about may no longer exist.** `S-3` asked the code
lane for measured candidate node changes to make every territory reachable. **`P-126` made metal and
energy carry between turns, and that appears to have done it** - the code lane recomputes ten of
twelve territories able to hold a Yard, nine able to produce an Ark, and every territory reachable,
with the 11/12 deadlock gone and territory 1 able to run the whole loop alone. **This lane's own
measurement in `P-126` agrees**, having reached all-but-5-and-6 and all-but-5-6-and-7 independently.

**So the request stands withdrawn rather than answered**, and measuring candidates for a problem that
may have dissolved is work nobody should do. What survives is one number, and it is `C-10`: what a
territory can keep is bounded and nothing says by how much. **At fifteen or more the loop closes; below
fifteen no Yard exists anywhere** and the release stops at step 6 - which is `C-8`'s conclusion
arrived at by a different route.

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
| P-139, a recipe, not a transformation, and a recipe belongs to the player or the world                          | `spec/invariants.md` -> The game is data, `releases/first-release.md` -> Recipes                                       | 2026-08-31 |
| P-140, two recipes the table did not have: upkeep, perish and revert                                            | `releases/first-release.md` -> Recipes                                                                                 | 2026-08-31 |

## Rejected

Nothing yet. Rejections are recorded with Sean's reason, so the same proposal is not filed
again in a later session.

## Withdrawn

| Proposal                                                                              | Why                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P-2, "twenty planet sizes are available below 500"                                    | Superseded by Sean's edit fixing the game at five named sizes.                                                                                                                                                                                                                                                                              |
| P-3, "no two territories are more than `3m` apart"                                    | Superseded by the per-size statement, and incomplete: `3m` holds for class I only, while the large planet is class III where the measured distance is 7.                                                                                                                                                                                    |
| P-4, "the twelve five-neighbour territories sit in six antipodal pairs"               | Derivable from the Goldberg choice, and no rule leans on it.                                                                                                                                                                                                                                                                                |
| P-5, "a pentagon's farthest territory is its antipodal twin"                          | Merged into P-4, then withdrawn with it.                                                                                                                                                                                                                                                                                                    |
| P-7, "the smallest planet has no six-neighbour territories"                           | Derivable from P-6 plus a line Sean had already written - the minimum is 12, **a dodecahedron**.                                                                                                                                                                                                                                            |
| P-9, "the distance between every pair is computed once and stored"                    | An implementation directive, not a rule of the game.                                                                                                                                                                                                                                                                                        |
| P-13, "the greatest distance is 3 / 5 / 6 / 7 / 9 by planet size"                     | Determined by the Goldberg choice and the size, nothing leans on it, and the numbers are **already asserted by a test**.                                                                                                                                                                                                                    |
| P-15, "Native life is a planet's own, Feral is printed life gone wild"                | **Feral is behavioural, not an origin**, and origin is not substantively relevant.                                                                                                                                                                                                                                                          |
| P-16, "every unit carries a name that persists when control changes"                  | **A unit has a type, and the type has a name.** Individual units of the same type are not distinguished.                                                                                                                                                                                                                                    |
| P-17, "depart is left unspecified so one rule covers biological and machine"          | Sean chose **starves**, committing to the biological reading for now; robots come later. Recorded in [the backlog](spec-backlog.md).                                                                                                                                                                                                        |
| P-20, "extracting one resource has no effect on extracting any other"                 | Written against the rating model and contradicted by the node model: **labor is shared**, so working a food extractor does compete with working a metal one.                                                                                                                                                                                |
| P-29, "a territory's threat level comes from what is on it"                           | Superseded by P-32. Threat is no longer a quantity a territory carries - it is one direction of **force**.                                                                                                                                                                                                                                  |
| P-36, "accidental damage is force 1, a predator is force 2"                           | Superseded on 2026-08-26: **force is inherent to the territory**, not carried by individual creatures, so there is nothing for a per-creature value to attach to.                                                                                                                                                                           |
| P-40, "the least force eats from food nodes; every species grows by the citizen rule" | Superseded on 2026-08-26. Nature has no population and **does not use nodes** - a node is intentional exploitation. The whole food chain goes with it.                                                                                                                                                                                      |
| P-43, "nothing is exterminated; coordination buys suppression"                        | Superseded on 2026-08-26. It described populations held at zero, and nature no longer has a population to hold anywhere.                                                                                                                                                                                                                    |
| P-46, "citizens and food move between adjacent territories"                           | Cut on 2026-08-26. Sean removed logistics for now so that **each territory is self-contained**; the only thing crossing a boundary is a mobile unit. Recorded in [the backlog](spec-backlog.md).                                                                                                                                            |
| P-56, "a territory satisfies its own consumption first"                               | Cut with P-46 on 2026-08-26 - it only had work to do while a remainder could reach a neighbour.                                                                                                                                                                                                                                             |
| P-124, "where a generated file lives"                                                 | Housekeeping rather than a decision, under the split Sean approved 2026-08-30. Settled by the specification lane and landed in `CLAUDE.md` -> Perspectives in the same commit.                                                                                                                                                              |
| P-144, "capacity and metal content have rules but no numbers"                         | Withdrawn on Sean's instruction, 2026-08-31. Its flat per-territory capacities are wrong under his storage rule: an extractor holds one cycle and a bin holds the rest, so a resource's capacity is the sum of the extractors and bins present, not a constant. **Its metal-content column survives as `P-146`**, which `P-145` depends on. |