# The First Release As Data

**Derived.** Written by Claude from conversation, 2026-08-31. Not binding - see
[the specification](../../spec/README.md) for what was actually decided. Every value here is read
from `releases/first-release.md`, `spec/` or `commands/`; nothing is invented.

[Notes index](README.md) · [The game as tables](the-game-as-tables.md) · [Prevent, detect, bound](prevent-detect-bound.md)

Sean wants a rules editor over a thin engine - *the rules can be tweaked to give drastically
different gameplay with only editing cells in a table* - and to prove it out he needs the first
release's data complete. **This is what exists, where it is, and the four groups of numbers nobody
has chosen.**

## What exists, and where

| Data                       | Rows                                 | Where it lives                                                              |
| -------------------------- | ------------------------------------ | --------------------------------------------------------------------------- |
| **Kinds and their traits** | 6                                    | `releases/first-release.md` -> Units and structures                         |
| **Recipes**                | 18, ~55 input and output rows        | `releases/first-release.md` -> Recipes                                      |
| **Traits**                 | 13, eight stored and five derived    | [the game as tables](the-game-as-tables.md) - **not in any owned document** |
| **Territory nodes**        | 12 territories, 129 `add node` lines | `commands/nodes.4x`                                                         |
| **Territory biomes**       | 12                                   | `commands/biomes.4x`                                                        |
| **Force of nature**        | 12, all 1                            | `commands/forces.4x`                                                        |
| **Biome yields**           | 6 biomes x 3 resources               | `releases/first-release.md` -> Biomes                                       |
| **Planet size**            | tiny, 12 territories                 | `commands/setup.4x`                                                         |
| **Opening state**          | one Ark in orbit                     | `commands/world.4x`                                                         |
| **Resource behaviour**     | conserved and expires, 3 rows        | `spec/resources.md`                                                         |

**So most of it is already data**, and `commands/*.4x` is already a rules editor's file format in
everything but the editing: `add node 5 metal 8` is a row of a table written as a line.

## The four groups nobody has chosen

Each is required by a rule that is **already promoted**, so these are not future work - they are the
release being incomplete against its own specification.

**Metal content, from `P-126`.** Metal is conserved: *what it was made into can be taken apart to get
it back.* A Yard is 15 metal because that is what it cost. **A garrison, a citizen and an extractor
have no cost and therefore no content** - and the recipe that makes them consumes a Pioneer worth 8.
**Eight metal goes in and nothing accounts for it.** Sean's own figure - *a pioneer disassembled
yields one metal* - implies garrison 7, extractor 0, citizen 0, and 1 loose. **Three numbers.**

**Capacity, from `P-129`.** *Everything in a territory occupies capacity there... a territory has some
of each before anything is built.* No kind says how much it occupies, and no territory says how much
it has. **Six occupancies and one starting allowance per kind.**

**Storage, from `P-138`.** *A thing not kept in order is lost when the turn ends... unless it is caught
where it was made.* A node holds some amount for free; anything else costs labor to store. **Two
numbers: what a node holds, and what a unit of storage costs elsewhere.**

**And one of those two now has a floor, measured by the code lane as `C-10`.** `spec/turn.md` says
what a territory can keep is bounded and nothing says by how much. **At fifteen or more the loop
closes; below fifteen no Yard exists anywhere** and the release stops at step 6. So the bound is not
free tuning - **anything under fifteen makes the release unwinnable**, which is `C-8`'s conclusion
reached from the other side.

**Whether `revert` can ever fire.** Not a number, and it belongs on this list because the data cannot
express it. Force of nature is 1 everywhere, a garrison is force 1, holding takes force *equal* to
nature, and nothing in the release removes a garrison. **So one of the eighteen recipes appears to be
unreachable**, and either the release is meant to demonstrate it or it is not.

## What that means for the editor

**Three of the four groups are twelve numbers between them**, which is the argument for the whole
approach rather than against it: the release is one table-edit away from complete, and the thing
blocking it is that nobody has typed the values, not that anything has to be built.

**And one thing the table shape has already earned.** The traits table exists only in a note, which
means the release's own data is spread across a document this lane owns, a directory nobody owns
(`commands/`), and a note that is explicitly not binding. **A rules editor needs one place to load
from**, and choosing where is a decision that has not been made - see below.

## Where the data should live, which is undecided

`CLAUDE.md` gives `spec/`, `releases/` and `docs/` to this lane and `crates/`, `tools/`,
`prototypes/`, `web/`, `scripts/` and `hooks/` to the code lane. **`commands/` is in neither**, exactly
as the repository root was before `P-124`.

That was fine while `commands/*.4x` was a test fixture. **It is not fine if it becomes the game's
data**, because then the question *who may change the game* has no answer. It is the same question
`P-124` answered for generated files, asked about authored ones.

## What a complete set of tables needs, in order

Sean: *what do we need to get to a complete set of tables with the starting sample data.* **Three
decisions, then four pieces of writing, and the order matters because the later ones cannot be
written until the earlier ones are answered.**

### Three decisions, all Sean's, none of them a number

**Superseded 2026-08-31: all three are answered.** Kept as written because the reasoning below is
what the answers were chosen against; see *What is left* at the end of this note for where each one
landed.

**1. Is a `cell` a thing or a trait?** A unit carries `cells: 2` in the Units table - a trait whose
value goes down. `move` consumes `cell, on that unit`, quantity 1, consumed yes - a thing the unit
holds. **It cannot be both and the tables currently say both.** Everything else waits on this,
because it decides whether the kinds table has twelve rows or thirteen.

**2. Does a port name another port, or is it a convention?** Six ports reach outside themselves -
`cell, on that unit`, `food, its upkeep`, `unit, here` against `unit, there`, `resource` at quantity
`density`. **For these eighteen a convention suffices**: *a port refers to the other ports of the
same recipe*, and since no recipe names two units there is never an ambiguity. **It stops working the
moment one does** - combat, a transfer between two units - and the predecessor shipped the general
answer as `$name` aliases. **Convention now and aliases later is a real option and should be a choice
rather than a default.**

**3. What is a quantity?** Three kinds appear where a count belongs: an integer, `density`, and `any`.
`prototypes/kinds` resolved it as `Exactly | Density | Any` and no document says so. **`Density` is
decision 2 in disguise**, since it is a number belonging to another port.

### Four pieces of writing, once those are answered

**1. Declare the missing kinds.** The Units and structures table has six rows and the recipes
reference thirteen: **node, territory, food, metal, energy, labor** and possibly **cell** are never
declared anywhere. Their traits mostly exist already - a node has a resource and a density, a
territory has a biome and a force of nature, the resources have conserved-and-expires in
`spec/resources.md` - so this is collecting rather than deciding.

**2. Put the traits table somewhere owned.** It exists only in
[the game as tables](the-game-as-tables.md), which is explicitly not binding, while the data it
describes is in `releases/`. **An editor cannot load a note.**

**3. Add the columns two promoted rules already require.** Capacity is a trait every kind carries -
how much it occupies, and of what kind - from `P-129`. Metal content is a trait of the kinds metal
goes into, from `P-126`. **Both are columns, not decisions**; only their values are decisions.

**4. Decide where the data lives.** `commands/` is in no lane's column, which was fine as a test
fixture and is not fine as the game's data, because *who may change the game* then has no answer.

### And then the numbers, which is the part Sean is waiting for

**Every number in the game lives in exactly two columns**: a kind's trait value, and a port's
quantity. So *pull all the costs down* is an edit to one of them, and the four groups this note
listed as missing become cells rather than gaps.

## What is left, 2026-08-31

Sean: *lets work through the remaining questions that need to be answered before I get my tables with
release data.* **All three decisions are answered, and one of the answers was neither option
offered.**

| Question                | Answer                                                | Where it went |
| ----------------------- | ----------------------------------------------------- | ------------- |
| Is a `cell` a thing?    | No: a unit has a fuel capacity and burns energy       | `P-141`       |
| What is a quantity?     | A whole number, written or read from a trait          | `P-142`       |
| Is `labor` a thing?     | Yes: *labor represents a citizen operating a machine* | `P-143`       |
| Is a territory a thing? | No: a place that carries traits                       | `P-143`       |
| Names or convention?    | Convention                                            | `P-143`       |

**The labor answer is the interesting one**, because it was offered as a choice between a thing and a
shorthand and came back as a definition instead. It settles the mechanical question - labor stays a
thing, so the kinds table has eleven rows - and adds a constraint that was not there before. **That
constraint does not hold in one of labor's two uses**: `work` takes labor and an extractor, which is
a citizen operating a machine, and `build extractor` takes labor with nothing yet to operate. `P-143`
puts both ways out to Sean rather than picking one.

**Three of the four pieces of writing are now filed.** Declaring the kinds and siting the traits
table are one act and are `P-143`; the capacity and metal-content columns are `P-144`. Chasing the
metal-content column found a contradiction and that is `P-145`: `perish` consumes a unit and produces
nothing, while `spec/resources.md` says metal is conserved, so an unpaid Pioneer deletes eight metal
from the game.

**What is still open, and none of it blocks the structure.**

- **Where the data lives.** `commands/` is in no lane's column. This is the fourth piece of writing
  and the only one not filed, because it is a governance question rather than a data one and `P-124`
  answered the same question for generated files - so the answer should probably match it
- **The numbers**, all of which `P-144` proposes so that there is something to pull down rather than
  a blank. The one that is not taste is metal capacity, which must reach 15 or no Yard exists
- **Whether `revert` can ever fire.** Nothing in first release reduces a territory's force, so it may
  be a recipe for a situation the release cannot reach. Worth confirming before a rule is spent on
  what it leaves behind
