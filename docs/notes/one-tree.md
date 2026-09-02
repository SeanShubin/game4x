# One tree

**Derived, 2026-09-01.** Written by Claude at Sean's request: *lets explore making territory not
privileged... push towards unification, things-are-trees, and recipes.* Not binding, and not yet
proposed.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is privileged today, and where it is written

| #   | The privilege                                           | Where                                                                |
| --- | ------------------------------------------------------- | -------------------------------------------------------------------- |
| 1   | A territory is a **place**, and a place is not a thing  | `spec/logistics.md`, `releases/first-release.md` -> Where things are |
| 2   | A recipe's scope is `here`, meaning **the territory**   | the Recipes table's Scope column                                     |
| 3   | Anything moves freely **within a controlled territory** | `spec/units.md`, of fuel                                             |
| 4   | `revert` acts on **a territory**                        | the Recipes table                                                    |
| 5   | Adjacency relates **territories**                       | `spec/planet.md`                                                     |

## What dissolves

**A thing contains things, up to a maximum per kind.** That is the whole model, and it is what
`spec/logistics.md` already says with one word changed - *a bin belongs to something* becomes *a
thing has room for things*. **A bin stops being a separate concept** and becomes the name for one
`(kind, maximum)` pair on a container, which is what *a capacity is a bin* already said.

**The planet is a thing. A territory is a thing in it. A citizen is a thing in that.** Nothing about
containment mentions a level, and nothing needs to.

## The four rules restated without a territory in them

**Scope becomes a condition rather than a level.** A recipe does not apply *at a territory*; it
applies **in any container whose traits satisfy it**. `build extractor` needs a container with room
for extractors and a density - **and only territories have those**, so a territory is selected by
what it is rather than privileged by where it sits. Sean's jungle case is then *in a container whose
biome is jungle*, and the same machinery covers both.

**`revert` goes the same way.** It acts on a thing whose force is below its force of nature. Only
territories have a force of nature. **The privilege was never doing any work** - the trait was.

**Free transfer becomes parent-and-child.** *Anything moves freely between a container and the thing
that holds it, where a player controls both.* Unloading a vehicle is one step; moving between two
vehicles in the same territory is two, and both are free. **Depth-free, and it stops naming a
territory** while still forbidding the thing it was written to forbid.

**Adjacency becomes a trait of the container.** A planet holds territories and says which of them
touch. **That is a fact about the planet**, not about territories, and stating it that way makes it
uniform: a vessel with two bays could say the same about its bays, and nothing else ever would.

## The one thing that is not containment

**Containment is a tree and adjacency is a graph**, and no amount of unification makes one the
other. What the restatement buys is that the graph is a **trait of whatever holds the things it
relates**, so it needs no level of its own - but it stays a second relation, and `move` is the recipe
that uses it.

That is worth saying plainly, because *everything is a tree* is nearly true and the exception is the
one thing the game's geography rests on.

## What it costs

**Recipes get longer**, because a condition that used to be assumed becomes a row. Four ingredients
also need echoing before `consumed` can be derived, which is the same direction. Sean's answer to
this is presentation: *the uniformity will allow us to choose presentations that remove the clutter*,
and that is the right shape - **the table is the data, and what a person reads is a rendering of
it.**

## What is not settled

- **Control at depth.** Is a citizen inside a vehicle inside a territory controlled by whoever holds
  the territory, or does each thing carry its own control? The first is a rule that walks up, the
  second is a trait that can disagree with its container
- **Whether a thing may contain a thing of its own kind.** Acyclicity permits a vehicle in a vehicle
  and forbids a vehicle in itself. Nothing needs it and nothing forbids it
- **What `here` means when a recipe could match at two depths.** A recipe satisfied by both a
  territory and a vehicle inside it would fire twice, which is either a feature or the first special
  case creeping back

## The test: simpler or more complex

Sean: *the test is if we end up with something simpler or something more complex... my promoted
rules may be wrong because I couldn't see the unification at the time, but they would be right if
attempting to unify them creates more complexity than it reduces.*

**It removes seven concepts and adds about ten rows to a table of forty-nine.**

| Goes away                                                              | Arrives                                        |
| ---------------------------------------------------------------------- | ---------------------------------------------- |
| the **place**-versus-**thing** distinction                             | a condition row where a level was assumed      |
| **bin** as a concept of its own                                        | four echo rows, so `consumed` derives          |
| the **Scope** column's `here` and `every` as special values            | two readiness rows on `work` and `move`        |
| the **consumed** column, derived instead                               | adjacency restated as a trait of the container |
| `revert`'s territory-as-ingredient - the code lane's `Noun::Territory` | parts, on any unit with more than one          |
| `P-155`'s second trait `moved`, if parts land instead                  |                                                |
| the *Where things are* section, which becomes one line                 |                                                |

**What is removed are rules and what is added is data**, which is the trade `spec/invariants.md`
already asks for. A row costs a reader a glance and a rule costs them a thing to remember at every
other row.

## Where it genuinely gets worse, and how much

**One rule that today does not exist would have to.** `here` prevents a recipe matching at two depths
by fiat. Under conditions, something must say what happens when a recipe is satisfied by a territory
**and** by a vehicle standing in it. **That is a new rule, not a removed one**, and it is the
strongest argument against.

**It is smaller than it looks, and the eight world recipes are how to see that.** `eat`, `grow`,
`depart`, `spoil`, `ready`, `upkeep`, `perish` and `revert` are scoped `every`, and under a tree they
match at every depth rather than at every territory.

- **Five are simply right that way.** Food spoils in a cargo hold; an exhausted thing readies wherever
  it is; an unpaid unit perishes wherever it stands
- **`revert` is right by accident and for the right reason** - only a territory has a force of
  nature, so only a territory matches
- **`grow` is the one that breaks**, and it breaks usefully: a citizen must not be born in a fuel
  tank. The fix is a condition saying where population lives, which is **a trait rather than a rule**,
  and it is a real fact about the world that the current model simply cannot state

**Two other things get harder and neither is fatal.** Control at depth needs deciding - a rule that
walks up, or a trait each thing carries that can disagree with its container. And *everything is a
tree* stays false while adjacency exists, so a reader still holds one exception.

## The verdict this note reaches

**Simpler, and the saving is in rules rather than rows.** Seven concepts for one new question, where
the question turns out to be a player-facing choice at most depths and a missing trait at one.

**The one thing worth keeping from the privileged version** is that `grow` found it. A rule that is
right only because nothing else can match it is not obviously right - and the exercise of removing
the privilege is what showed that population lives somewhere in particular, which nothing in the
specification says today.
