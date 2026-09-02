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
