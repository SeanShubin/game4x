# The Game As Tables

**Derived.** Written by Claude from conversation, 2026-08-31. Not binding - see
[the specification](../../spec/README.md) for what was actually decided. Every figure here is read
from `releases/first-release.md` and `spec/`, and where the two shapes disagree it is this file that
is wrong.

[Notes index](README.md) · [Everything is matter](everything-is-matter.md) · [Prevent, detect, bound](prevent-detect-bound.md)

The whole of the first release as two tables: **what kinds of thing there are**, and **what turns
some things into others**. Written to be checked at a glance rather than read.

## Kinds

One row per kind, one column per trait, blank where a trait does not apply. **The blanks carry
information** - they show which traits are shared and which belong to one kind alone.

| Kind          | Force | Cells | Upkeep | Resource | Density | Readies | Notes                         |
| ------------- | ----- | ----- | ------ | -------- | ------- | ------- | ----------------------------- |
| **citizen**   | 1     |       | 1 food |          |         | yes     | provides labor                |
| **garrison**  | 1     |       |        |          |         |         | multiplier 1; force sums here |
| **extractor** |       |       |        | one      |         |         | works one node                |
| **yard**      |       |       |        |          |         |         | lets an Ark be produced       |
| **ark**       | 2     | 2     |        |          |         | yes     | may descend from orbit        |
| **pioneer**   | 2     | 2     | 1 food |          |         | yes     | must found what it enters     |
| **node**      |       |       |        | one      | 1-8     |         | fixed by the ground           |
| **nature**    | 1     |       |        |          |         |         | inherent to the ground        |
| **food**      |       |       |        |          |         |         | expires whether eaten or not  |
| **metal**     |       |       |        |          |         |         | conserved; changes form       |
| **energy**    |       |       |        |          |         |         | consumed, but does not expire |
| **labor**     |       |       |        |          |         |         | one per citizen, per turn     |

**Two columns are proposals rather than the game as built.** *Readies* assumes citizens and units
carry **ready** or **exhausted** rather than the territory carrying a `labor_spent` count. And the
three resource rows assume `P-126`'s split, which is open.

**`labor` is a thing here and is not one in the model today.** It is what makes `work` an ordinary
recipe rather than a special case - see the two-step below.

## Recipes

One row per input or output. **Consumed** says whether the input is used up; **bound** says whether
the quantity is a floor or a ceiling; **scope** says whether it applies at one place or everywhere it
matches.

| Recipe              | Scope | Role | Thing              | Qty     | Consumed | Bound       |
| ------------------- | ----- | ---- | ------------------ | ------- | -------- | ----------- |
| **land**            | here  | in   | ark, in orbit      | 1       | yes      | at least    |
|                     |       | in   | garrison           | 0       | no       | **at most** |
|                     |       | out  | garrison           | 1       |          |             |
|                     |       | out  | citizen            | 1       |          |             |
|                     |       | out  | extractor, food    | 1       |          |             |
| **move**            | here  | in   | unit, here         | 1       | yes      | at least    |
|                     |       | in   | cell, on that unit | 1       | yes      | at least    |
|                     |       | out  | unit, there        | 1       |          |             |
| **found by land**   | here  | in   | pioneer, arriving  | 1       | yes      | at least    |
|                     |       | in   | garrison           | 0       | no       | **at most** |
|                     |       | out  | garrison           | 1       |          |             |
|                     |       | out  | citizen            | 1       |          |             |
|                     |       | out  | extractor, food    | 1       |          |             |
| **build extractor** | here  | in   | labor              | 1       | yes      | at least    |
|                     |       | in   | node, unworked     | 1       | no       | at least    |
|                     |       | out  | extractor          | 1       |          |             |
| **build yard**      | here  | in   | metal              | 15      | yes      | at least    |
|                     |       | out  | yard               | 1       |          |             |
| **produce pioneer** | here  | in   | metal              | 8       | yes      | at least    |
|                     |       | in   | energy             | 6       | yes      | at least    |
|                     |       | in   | citizen            | 1       | yes      | at least    |
|                     |       | in   | garrison           | 1       | **no**   | at least    |
|                     |       | out  | pioneer            | 1       |          |             |
| **produce ark**     | here  | in   | metal              | 12      | yes      | at least    |
|                     |       | in   | energy             | 12      | yes      | at least    |
|                     |       | in   | yard               | 1       | **no**   | at least    |
|                     |       | out  | ark                | 1       |          |             |
| **launch**          | here  | in   | ark, here          | 1       | yes      | at least    |
|                     |       | in   | cell, on that unit | 1       | yes      | at least    |
|                     |       | out  | ark, in orbit      | 1       |          |             |
| **spend readiness** | here  | in   | citizen, ready     | 1       | yes      | at least    |
|                     |       | out  | citizen, exhausted | 1       |          |             |
|                     |       | out  | labor              | 1       |          |             |
| **work**            | here  | in   | labor              | 1       | yes      | at least    |
|                     |       | in   | extractor          | 1       | no       | at least    |
|                     |       | out  | resource           | density |          |             |
| **eat**             | every | in   | citizen            | 1       | no       | at least    |
|                     |       | in   | food               | 1       | yes      | at least    |
| **grow**            | every | in   | food, surplus      | 1       | yes      | at least    |
|                     |       | out  | citizen            | 1       |          |             |
| **depart**          | every | in   | citizen, unfed     | 1       | yes      | at least    |
| **spoil**           | every | in   | food               | any     | yes      | at least    |
| **ready**           | every | in   | thing, exhausted   | any     | yes      | at least    |
|                     |       | out  | thing, ready       | any     |          |             |

## What the tables say that the prose did not

**`move` and `launch` are ordinary.** A unit here in, a unit there out - location is a trait like any
other, and orbit is a place. Nothing about either needs its own rule.

**`land` and `found by land` are the same recipe reached two ways**, which is why founding is
one action and not two. The inputs differ only in which unit arrives.

**`end turn` is five rows with scope `every`**, not a special phase. What is global is the scope
column, not the rules.

**Three things in the model are missing from these tables on purpose**, because they are computed
rather than stored: `founded` is *force present at least force of nature*; `labor_spent` is replaced
by the ready and exhausted rows; `won` is a rule over the state. See
[everything is matter](everything-is-matter.md).

**Two things needed by the tables do not exist yet.** *Node, unworked* is the difference between
nodes and extractors rather than a kind - it needs derived kinds, or the language needs comparisons.
And *food, surplus* is likewise a comparison between food and citizens. **Both appear in exactly one
row**, which is a useful measure of how much the derived-kinds decision actually costs.

## What is still missing from this picture

- **Capacity** has no column, because `P-129` is open and unnumbered. When it lands, every kind gains
  *how much capacity it occupies* and *of what kind*, and containers gain *how much it provides*.
- **Biome** appears nowhere, which is accurate rather than an omission: nothing in the rules reads
  one today.
- **Force of nature** is a row in Kinds and no recipe touches it. Taking and holding are
  comparisons of force, and no recipe in the release changes who holds a territory except by
  founding it.

## Traits

The third table, and the one that was missing. **Kinds and recipes cannot be read apart from each
other, because the thing a recipe names is a kind *and a trait*** - `citizen, ready` is not a kind, it
is a citizen with something true of it. Every such qualifier in the recipe table is one of these.

| Trait         | Values                  | Carried by                                    | Stored or derived | What it means                                 |
| ------------- | ----------------------- | --------------------------------------------- | ----------------- | --------------------------------------------- |
| **force**     | a number                | citizen, garrison, ark, pioneer               | stored            | capacity for violence                         |
| **cells**     | a number                | ark, pioneer                                  | stored            | how far it can still move                     |
| **upkeep**    | a resource and a number | pioneer                                       | stored            | what it costs to keep each turn               |
| **readiness** | ready, exhausted        | citizen, ark, pioneer, extractor              | stored            | whether it has acted this turn                |
| **location**  | a territory, or orbit   | ark, pioneer                                  | stored            | where it is                                   |
| **resource**  | food, metal, energy     | node, extractor, and the resources themselves | stored            | which of the three                            |
| **density**   | 1 to 8                  | node                                          | stored            | how much one working yields                   |
| **biome**     | one of six              | territory                                     | stored            | what the terrain is                           |
| **unworked**  | yes or no               | node                                          | **derived**       | no extractor works it                         |
| **surplus**   | yes or no               | food                                          | **derived**       | more food than citizens                       |
| **unfed**     | yes or no               | citizen                                       | **derived**       | no food was left for it                       |
| **unpaid**    | yes or no               | a unit with upkeep                            | **derived**       | its upkeep went unmet                         |
| **held**      | yes or no               | territory                                     | **derived**       | force present is at least the force of nature |

**The derived five are the ones nothing stores.** Each is a comparison between two counts, and
`spec/invariants.md` says a trait may be derived and that nothing can leave one wrong, because nothing
writes one. **They are why the three tables cannot be read separately** - a recipe naming *node,
unworked* is reaching for something no kind carries and no state holds.

## How the three tables join

**One sentence per join, because the joins are the whole difficulty:**

- **A recipe's *Thing* is a kind, optionally narrowed by a trait.** `citizen` is a kind; `citizen,
  ready` is that kind with a trait pinned to a value; `unit, here` is a **family** narrowed by a
  trait.
- **A kind's row in the Kinds table is its traits and their values.** Force 2 and cells 2 for a
  pioneer are two entries in the traits table read across one row.
- **A family is not a kind and not a container.** `unit` covers ark and pioneer, `resource` covers
  food, metal and energy, and `thing` covers whatever can be exhausted. **`unit` and `resource` can be
  read off the Kinds table; `thing` cannot**, which is the one gap `prototypes/kinds` found and left
  open.

**So the reading order is traits, then kinds, then recipes.** A trait is a question you can ask about
something; a kind is a set of answers; a recipe is a rule quoted in terms of both.

## What the three tables together still do not say

- **Which kinds are `thing`.** `ready` names it on both ports and nothing enumerates it. `P-133`
  answered which kinds *ready*, which is the same list - **so the gap is that nobody has said the two
  are the same question.**
- **What a quantity is when it is not a number.** `density` and `any` appear where a count should be,
  which `prototypes/kinds` resolved as `Exactly | Density | Any` and no document states.
- **Where capacity goes.** `P-129` is promoted and no table has a column for it: every kind wants *how
  much it occupies* and *of what kind*, and a container wants *how much it provides*.
