# The Structure

**Derived.** Written by Claude from conversation, 2026-08-31. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [The game as tables](the-game-as-tables.md) · [The first release as data](first-release-data.md)

Sean: *what I really need is the structure. Once I have the structure I can tune the numbers, and I
expect I am going to pull all the numbers WAY down.* **So this is the shape without any values in
it**, tested against every port the release actually has.

## Seven tables

| Table                | Columns                                      | Rows in the first release                |
| -------------------- | -------------------------------------------- | ---------------------------------------- |
| **kind**             | name                                         | ~13, and the release lists 6 - see below |
| **trait**            | name, what its values are, stored or derived | 13                                       |
| **kind trait**       | kind, trait, value                           | the Units table, one cell per row        |
| **family**           | family, member kind                          | 3 families - `unit`, `resource`, `thing` |
| **recipe**           | name, owner: player or world                 | 18                                       |
| **ingredient**       | recipe, thing, quantity, consumed, bound     | 37                                       |
| **result**           | recipe, thing, quantity                      | 18                                       |
| **ingredient trait** | ingredient, trait, value                     | ~14                                      |

**The wide tables in the release are these narrow ones joined.** A row of the Units table is one kind
and its trait values; a block of the Recipes table is one recipe and its ports. **Nothing here is new
information** - it is the same data with the joins made explicit, which is what an editor needs and a
reader does not.

## What the ports proved the structure needs

Every port in the release was checked against the schema. **Ten of them do not fit a plain
kind-and-quantity, and they fail in exactly two ways.**

### One port must be able to name another

| Port                                         | What it refers to                                 |
| -------------------------------------------- | ------------------------------------------------- |
| `cell, on that unit`                         | the unit named by another port of the same recipe |
| `food, its upkeep`                           | the unit's upkeep trait                           |
| `unit, here` and `unit, there`               | two places, bound together by the move            |
| `ark, here`                                  | the place the ark is                              |
| `resource` at quantity `density`             | the density of the node named by another port     |
| `territory, force below its force of nature` | the territory's own trait, compared with itself   |

**So a port is not self-contained**, and the schema needs a way to say *this one and that one are the
same thing*. **The predecessor had it**: `language/Expressions.kt` defines `alias` as `$name`, and it
shipped. Without it, `move` cannot say the unit that arrives is the unit that left, and `work` cannot
say which node's density it yields.

**This is the single largest thing the tables hide.** Read as prose, *cell, on that unit* is obvious.
Read as data, *that* has no referent.

### A quantity is not always a number

Three values appear where a count belongs: `density`, `any`, and plain integers.
`prototypes/kinds` resolved it as `Exactly(n) | Density | Any` and no document says so. **`Density` is
the previous problem in disguise** - it is a number belonging to another port.

## The kinds table is not the kinds

**The release's Units and structures table has six rows and the recipes reference at least thirteen
kinds**: citizen, garrison, extractor, yard, ark, pioneer - and also **node, territory, food, metal,
energy, labor and cell**, none of which appear in it.

That is not a defect in the table, which is named *Units and structures* and contains exactly those.
**It is a gap in the data**: seven kinds are referenced and never declared, and an editor loading the
release would find no row for `metal`.

**`cell` is the doubtful one and worth deciding rather than assuming.** A unit carries `cells: 2` as a
trait, and `move` consumes `cell, on that unit` as though it were a thing. **It cannot be both** - it
is either a trait whose value goes down, or a thing the unit holds and spends. The tables currently
say both.

## What this means for pulling the numbers down

**Every number lives in exactly two places**, which is the point of the structure: the **kind trait**
table, and the **port** table's quantity column. Nothing else in the seven tables holds a value.

So *pull all the costs down* is an edit to one column of one table. **Force, cells, upkeep and density
are the other**, and biome yields and territory nodes are the same shape one level out.

**And two of the four missing groups stop being missing once the structure exists**, because they are
columns rather than decisions: capacity is a trait every kind carries, and metal content is a trait of
the kinds metal goes into. **What remains a decision is their values**, which is where Sean said he
wanted to be.

## A correction: *port* was a word this lane invented

Sean asked what a port is. **It appears in no specification, no release and nothing he wrote** - it
was borrowed from dataflow, where a component has input and output ports, and used here as a
collective noun for one row of a recipe's inputs and outputs.

**Asking exposed that the concept is two concepts.** Counted against the release: **all eighteen
output rows leave *consumed* and *bound* empty**, and none could sensibly fill them - an output is not
consumed, and its quantity is exactly what it is rather than a least or a most. **A single table with
two always-empty columns for half its rows** is the wide-table smell this whole exercise exists to
remove.

**Split, and the collective noun stops being needed.** A recipe has **ingredients** - a thing, how
many, whether it is consumed, and whether that is a least or a most - and **results** - a thing and
how many. Thirty-seven and eighteen. **Nothing is empty, and the recipe metaphor carries both words
without explanation**, which is the test *port* failed.

**The release's table keeps its Role column, and that is right.** One visual table with a role reads
better than two side by side; two tables underneath store better. That is the split `docs/layers.md`
already draws - normalise where you write, denormalise where you read - **arriving in the data rather
than in the code.**
