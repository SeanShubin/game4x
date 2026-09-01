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
