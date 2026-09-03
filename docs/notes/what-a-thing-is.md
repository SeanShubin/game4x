# What a thing is

**Derived**, 2026-09-03, from Sean's proposal that a thing is mostly a list of capacities for what
it contains, with its parts providing capabilities. Not binding.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

> Perhaps a thing is a list of capacities for the other things it contains. I expect a thing is more
> than this, but that seems to be the most important part. The parts that make up a thing may
> provide capabilities, so a metal transport contains the movement capability, metal capacity, fuel
> capacity.

## What the release says a thing is today

`spec/invariants.md` already says **a thing is a set of traits, and one of them names its kind**, and
that **nothing in the state is special to a kind**. Sean's idea is a refinement rather than a
replacement: it says *which* traits matter most, and where they come from.

## Testing it against the twelve kinds

| Kind                       | Capacities it has          | What else it is                   |
| -------------------------- | -------------------------- | --------------------------------- |
| territory                  | ten, one per kind          | biome, force of nature, adjacency |
| orbit                      | units, and nothing else    | above one territory               |
| ark, pioneer               | fuel                       | force, a move, crosses, readies   |
| extractor                  | one cycle of what it makes | resource, readies                 |
| citizen, garrison          | **none**                   | force, upkeep, readies            |
| food, metal, energy, labor | **none**                   | they are what is contained        |

**The idea covers the containers completely and the leaves not at all.** A territory and an orbit
*are* capacity lists. A unit is a capacity plus capabilities. A citizen has no capacity and is
nothing but capability. A resource has neither.

So the honest form of the idea is **a thing is a list of capacities and a list of capabilities**,
and the first list is empty more often than the second.

## What it predicts, and the prediction holds

**The *Units and structures* table mixes five sorts of fact in ten columns.**

| Sort           | Columns                        |
| -------------- | ------------------------------ |
| capacity       | `Fuel`                         |
| capability     | `Force`, `Crosses`, `Readies`  |
| cost of acting | `A move`, `Upkeep`             |
| matter         | `Binding`                      |
| **a recipe**   | `Costs to produce`, `Requires` |

**The last row is the one worth acting on.** A Pioneer's `Costs to produce` reads *3 metal, 6
energy, 2 citizens*, and `produce pioneer` consumes exactly that. `Requires: a Yard` is `produce
ark` requiring a yard. **The same facts are written twice**, and
`the_costs_in_the_model_are_the_costs_in_the_release` exists to hold the copies in step - a test
whose whole purpose is a duplication. `P-199` says *nothing restates what a data file says*.

**And it explains a redundancy already found.** `Metal in it` and `Binding` held the same number in
all six rows, which is why `P-201` removed one. **Parts are the mechanism that would make them
differ**, and nothing in the first release has parts. Sean's transport is the first thing that
would.

## The blanks

**1. Is a capability a part, or a trait?** He says parts *provide* capabilities, which makes a
capability something a thing has because a part is in it. If so, *movement* is a part rather than a
column - and `Crosses` and `A move` become facts of that part.

**2. Does a capacity belong to the thing or to a part?** *Metal capacity* and *fuel capacity* read
as two holds, not two numbers on one hull. If capacity lives on the part, a thing's capacity is the
sum of its parts' - and **`metal in it = binding plus the metal in its parts` finally has work to
do.**

**3. What is upkeep?** It is neither a capacity nor a capability but a standing cost. Either it is a
third list, or it belongs to a part - a citizen eats because of what a citizen is made of.

**4. Is a Pioneer a kind, or a name for a composition?** This is the large one. If a thing is its
parts, then an Ark and a Pioneer differ only in which parts they have, and the twelve kinds become
**a few parts plus named arrangements of them**. `spec/invariants.md` already says adding a kind
adds no field and no case, which is the same instinct one level down.

**5. What holds an arrangement together?** `P-170` already answers it: **binding**, *at least one
metal, so a thing made of nothing still costs something.* The word for the composition exists
before the composition does.

## Does it simplify?

**Yes, and measurably, in three places that are already sore.**

- Two columns of *Units and structures* restate the recipes, and one test exists to hold the copies
  in step
- The capacity table has one exception in ten rows, which [`P-205`](proposals.md) removes by making
  a node a kind - **the same move, one level up**
- `Binding` and `Metal in it` were indistinguishable because nothing has parts

**What it does not simplify is the leaves.** A citizen is not usefully a list of capacities, and
saying it has an empty one buys nothing. **The unification is real for containers and forced for the
rest**, which is worth knowing before it is written as a rule.

## Sean's answers, 2026-09-03, and what they settle

He answered all five. Four of them collapse into one shape.

### The tree node carries a value. A leaf is a node with no children

**His own rule decides it.** `spec/invariants.md`: *nothing in the state is special to a kind. Adding
a kind adds no field and no case, and **whatever reads the state reads it the same way whatever kind
it holds***. A branch-or-leaf split is a case, in the one place the specification forbids one.

**Three more reasons, and each is a fact about this game rather than a preference:**

- **A container has a value of its own.** A territory has a biome and contains citizens. An
  extractor has a resource and contains its catch. Under *branches have no value* neither can be
  said, so the shape collapses back to nodes-with-values anyway - **arriving there by force rather
  than by choice.**
- **A leaf that gains contents would change type.** `P-192` made `territory` and `orbit` kinds
  because only a thing may contain things; if leaves were a different type that argument could not
  have been made without changing what a territory *is*.
- **`P-157` says a thing contains things**, with no exception for the ones that contain none.

**So *leaf* is an observation and not a type**: a node with an empty list of children.

### Parts and contents are the same list, at different depths

This is his (2), and the tree answers it without a second mechanism.

```
pioneer
+-- movement            a part: makes `move` apply
+-- tank                a part: holds energy
|   +-- energy x2       contents
+-- (its own metal)
```

**Capacity belongs to the part that holds, because the part is what holds.** A Pioneer's fuel
capacity is the tank's capacity. Nothing needs to distinguish *parts* from *cargo* - the tree
already does, by depth.

**And `metal in it = binding plus the metal in its parts` becomes a fold**: a node's own metal plus
the metal of its subtree. The rule was written before there was a structure to run it on.

### A part is what makes a recipe apply

This is his (3), and it is more than a name for two cases. *Disappears without upkeep* and *can move*
are both **which recipes reach this thing**. A movement part makes `move` applicable; an upkeep part
makes `upkeep` and `perish` applicable.

**It has a consequence worth weighing before anyone commits to it.** `move` currently requires
`unit, ready`, and the `unit` family is exactly *ark and pioneer* - which is exactly *the things that
can move*. **If a recipe names a part, the family stops being declared and starts being derived**,
and `Families` shrinks by a row that was never saying anything else.

### Binding: keep the constraint, drop the word

His (4), and the tree decides it too. Under a fold there is **one trait - a node's own metal** - and
**one derivation - the sum over its subtree.** `Binding` and `Metal in it` were two names for those,
which is why they held the same number in all six rows while nothing had parts.

**`P-170`'s constraint survives and is worth keeping**: *what binds a thing is at least one metal, so
a thing made of nothing still costs something.* That is a floor on a node's own metal, and it can be
said without the word.

## What is still open

**Only one thing, and it is the one that decides the size of the change.** If a Pioneer is a name for
an arrangement, then the twelve kinds are **a few parts and some named arrangements** - and the
question is whether an arrangement is *data the game loads* or *a kind in its own right*. `P-199`
says what the game is made of lives in a data file. **A named arrangement is exactly that**, which
suggests the kinds table shrinks to the parts and the arrangements move to the world's data.

**That is a bigger change than anything promoted so far, and it is `P-134`'s territory.** It should
not be proposed piecemeal.

