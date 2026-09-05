# Storage: capacity and bins are one thing, and the freedom is the factorization

**2026-09-05.** Sean asked to brainstorm a storage system, then corrected the analysis. **This
document is the second version and the first was wrong**; what it got wrong is recorded at the
bottom, because the mistake is instructive. This is thinking, not a decision. Nothing is settled
until it is in [the specification](../../spec/README.md).

## His principle is already written down

> The per-type is really a re-statement of no building working against you, you only have to build
> more, you don't have to decide which.

[`spec/logistics.md`](../../spec/logistics.md) says it in almost those words: *a total capacity of
four extractors is a maximum of four, **so nothing a player builds ever crowds out something of
another kind**.* **Per-type is not a new decision, it is a rule he already promoted.**

## The correction: capacity and bins are the same thing

> A capacity can be thought of as a number of bins... Zelda could be thought of as having one bin
> with capacity for 255 rupees, or 255 bins with capacity for 1 rupee each. However exposing this
> level of freedom to the user is madness.

**A capacity for N of a kind, where each holds M, is a capacity for N x M.** The two numbers are a
**factorization of one product**, and every factorization describes the same game. **That is the
whole of the freedom, and it is why it is madness to expose** - the player would be choosing between
descriptions rather than between outcomes.

## So the organizing principle has to fix the factorization

**One rule does it: a bin's capacity is a property of its kind, never of the instance.**

Every `metal store` holds ten metal. Not *this one holds ten and that one holds fifteen*. **Then the
only number the player moves is how many**, and the product follows.

Everything wanted falls out of that one sentence:

- **Per-type, no competition.** A territory's capacity for metal stores is separate from its capacity
  for energy stores. Build seven and eight; neither crowds the other
- **Build more, never decide which** - his own restatement of the promoted rule
- **No arranging.** There is nothing to arrange, because two stores of the same kind are
  interchangeable
- **The factorization is not a choice.** M is fixed by the kind and N is bounded by the place, so
  there is exactly one way to describe any amount of storage

## And it is one rule, not two, which is the part worth noticing

**Capacity for N things of a kind is already the only rule there is.** `spec/logistics.md`: *what a
thing may contain is a maximum per kind, or per family of kinds.*

- a territory has capacity **3** for `extractor`
- a territory has capacity **10** for `metal store`
- a metal store has capacity **10** for `metal`

**Three applications of one sentence at three levels**, and nothing new is needed to say any of them.
**The recursion terminates because the containment graph is acyclic**, which he already promoted:
*nothing contains itself, directly or through anything else.*

## What makes a bin real rather than presentational

If nothing can happen to a bin that cannot happen to a unit of capacity, **the bin is a display
choice** and should not exist as a thing. A bin earns being a thing when something distinguishes N
from N x M:

- **It costs something to build**, so N is a decision with a price
- **It can be destroyed**, so a loss takes ten metal rather than one
- **It has upkeep**
- **It is captured with the territory**

**At least one of those has to be true, or the kind is a noun with no consequences.** This is the
question to answer before writing any of it down.

## If a bin is only for communicating intent, take each intent separately

Sean, 2026-09-05: *if we need bins at all, it is for communicating intent to the user, not mechanics,
but I don't want to add them if I can make the same intentions just as clear without bins.*

**Four intents need communicating. Three of them are clearer without a bin.**

| Intent                        | With a bin                | Without                                                                           |
| ----------------------------- | ------------------------- | --------------------------------------------------------------------------------- |
| this limit is **per type**    | one bin kind per resource | `metal 14/70` and `food 3/70` on their own lines - a shared pool would be one bar |
| you are **near the limit**    | count the full bins       | `14/70`                                                                           |
| what happens **at the limit** | -                         | a rule about the resource                                                         |
| you can **build more**        | **build a metal store**   | *raise metal capacity* - vaguer, and it names no object                           |

**Only the last one needs the noun, and it needs it for a hard reason**: a verb needs an object. **You
cannot say *build more capacity* without naming what is built.** So the bin exists if and only if
capacity is something the player builds.

**The release today gives capacity rather than building it** - twenty each, from *What bounds a kind*
- so **today no bin is needed at all.**

## And behavioural differences do not belong on the container

The other half of his question is whether different bins need different names *so the expectation is
set properly* - a silo behaving unlike a stockpile.

**Food spoils because it is food, not because of what it is in.** `keeps` is already a trait of food.
**Putting the expectation on the container is putting the fact in the wrong place**, and it breaks
the moment the same resource can sit in two kinds of container - a silo in a territory and a hopper
on a transport, both holding food, both spoiling, for a reason neither name explains.

**So: differences in behaviour belong to what is stored. Differences in reach belong to what stores
it** - a vehicle's contents move because the vehicle moves, which the vehicle already says.

## A third option, if capacity should grow but bins should not exist

**Let capacity be a property of buildings that exist for other reasons.** A Yard raises metal
capacity as a consequence of being a Yard. Then:

- the player builds more capacity, so growth is a decision with a price
- **there is no storage vocabulary at all** - no store, silo, hopper or bin
- the count of nouns does not grow, which is his stated worry

**The cost is that capacity stops being independent** - you cannot raise metal room without wanting
a Yard for its own sake, and that is a real coupling rather than a free lunch. **It is the option
that adds the least and it is not the option that adds nothing.**

## The two-type structure, and why neither type needs a name

Sean, 2026-09-05, with two future use cases - **the player can build more metal storage but only up
to a point**, and **the player can upgrade how many things fit in a single container** - and a
structure:

> Type A bins can store each kind of type B bin according to a per-kind limit. Type B bins can not
> store other bins at all but only non-bin materials, and only a single kind. So a territory would
> also be a type A bin, a metal transport would be a type A bin with two type B bins inside, one
> fuel and one metal.

**A and B are not two kinds of thing. They are two constraints on what a thing may contain**, and
under `spec/logistics.md`'s existing rule - *a maximum per kind, or per family of kinds* - they say:

- **A thing that holds material holds exactly one kind of it, and holds no containers.**
- **A thing that holds containers holds no material.**

Together: **nothing mixes two materials, and nothing mixes material with containers.** That is the
organizing principle, and it is two sentences of constraint rather than any new noun.

**Which is why neither type needs a name.** *Type A* and *type B* are **derived from what a thing can
contain** - ask what its capacities are over and the answer tells you which it is. **Naming a derived
property is the mistake `founded` was**: a name in the data for something the rules already
determine, which then has to be kept in step with the thing it was derived from.

**The player never needs the category either.** They need *this territory holds up to ten metal
stores* and *this metal store holds ten metal*. **Neither sentence is helped by knowing that one is
an A and the other a B.**

## One thing his sketch leaves out, and a transport is the case

**A holds A.** A metal transport is a type A and it sits in a territory, which is also a type A - so
the structure is not two levels. `spec/logistics.md` already handles it: *a thing that contains
things takes up capacity in whatever contains it*, so a transport occupies territory capacity like
anything else.

**So the real distinction is binary and not layered**: a thing either **holds material** - one kind,
no containers - or it **holds things**. Leaf or branch. **The depth is however deep the game happens
to go**, and it terminates because nothing contains itself.

## The upgrade use case is the one that can bring the madness back

> The player can upgrade how many things can be stored in a single container.

**If that upgrade is per kind - every metal store now holds fifteen - the factorization stays fixed
and nothing is exposed.** It is Far Cry's pouch: one number, raised once, for all of them.

**If it is per instance - this store holds ten and that one fifteen - the freedom comes straight
back**, and the player is arranging again. **This is the same distinction that made *a bin's capacity
is a property of its kind* work**, and the use case is worded ambiguously between the two.

**Worth settling deliberately**, because it is the one place where a natural-sounding feature undoes
the organizing principle.

## Is everything-is-a-tree internally consistent? Yes, with one correction

Sean, 2026-09-05: *the whole game could be a storage container, so would a territory, so would a
vehicle, and only the raw materials would be leaves. Is that internally consistent?*

**The three properties a tree needs are already promoted**, which is the strongest thing that can be
said for it:

| A tree needs   | `spec/logistics.md` already says                             |
| -------------- | ------------------------------------------------------------ |
| **one parent** | *a thing... is itself in **at most one** other thing*        |
| **no cycles**  | *nothing contains itself, directly or through anything else* |
| **a root**     | his own *the whole game could be a storage container*        |

Without the root it is a **forest** - *at most one* permits orphans. **Making the game the root is
what turns it into a tree**, and that is the one piece he is adding.

**And adjacency is already tree-shaped**: *a thing says which of the things in it are next to which.
That is a fact about the container rather than about its contents.* **Siblings are adjacent, and the
parent records it** - which is exactly how a tree carries a graph without stopping being a tree.

## The correction: the implication runs one way

**Not *only raw materials are leaves*. Rather: raw materials are always leaves.**

Check it against the twelve kinds. Materials - `food`, `metal`, `energy`, `labor` - hold nothing and
never can. Branches - `territory`, `orbit`, `ark`, `pioneer`, and `extractor`, which the release says
*holds one cycle of what it makes*. **And then `citizen`.**

**A citizen is not a raw material and holds nothing.** Under *only raw materials are leaves* it has to
be a branch, and it is a branch with nothing in it - which is fine in a tree and makes the sentence
as worded false. **The true statement is the one-way one**: a material *cannot* contain, so it is
always a leaf; anything else *may* contain, and may happen to be empty.

## Two conditions, and both are about not letting the shape drift

**1. Leaf or branch is a property of the kind, not of the instance.** If it is defined by what a
thing *does* hold, **a citizen picking something up changes its type**, and the tree's shape becomes
a fact about the current state rather than about the rules. **Defined by what a kind *may* hold, it
is stable** - and the constraint two sections above is already written over kinds.

**2. Containment is the tree; a reference is not containment.** A citizen working an extractor is
**in the territory**, and the extractor **refers** to it. If working meant containment the citizen
would have two parents and the tree would break. **Nothing currently says which links are
containment**, and this is the sentence that would need to.

## So: it works, and it is less new than it looks

**He is not adding an organizing principle. He is naming one the specification already has** - one
parent, no cycles, capacity per kind, adjacency recorded by the container. **What is genuinely new is
the root**, and the one-way phrasing of the leaf rule.

## The planet breaks *one kind against many kinds*, and shows what the axis really is

*Our leaf containers only hold one kind of thing, our branch containers hold many types of
containers, but our planet seems to be both. Territories are the same kind of thing but they have
different ids, they are not fungible the way metal is.*

**He is right and the fault is in my phrasing.** A planet holds twelve territories - **one kind, and
not fungible.** So *how many kinds* was a proxy, and it breaks here.

**The axis is whether the contents are counted or listed**, and `spec/invariants.md` already draws
it: *a game's state is things, in places, and **how many of each**.*

- **Counted.** Fourteen metal is a number. Two metal with the same traits **collapse into one row
  with a count**, and nothing is lost
- **Listed.** Twelve territories do not collapse. **Territory 1 and territory 2 differ**, and a count
  of twelve throws the state away

**And what prevents collapsing is an id.** `P-254` just settled that a thing's own identifier is
`id`, and the data shows it exactly: `{{territory id:1 ...}}` carries one and `{{metal ...}}` does
not. **So the marker already exists and is already promoted.**

**Corrected**: a **leaf** holds a quantity of one fungible kind. A **branch** holds identified
things, of any kinds. **A branch holding one kind is still a branch** - the planet - and that is no
longer a contradiction.

## Is a thing that contains nothing automatically a leaf?

*Can we represent the same thing with a leaf and an empty branch? If so how do we decide, or do we
allow both?*

**They are not the same thing, and the difference is what is permitted rather than what is present.**
An empty branch **may come to hold something**; a leaf **may never**. Identical today, different in
what the rules allow tomorrow.

**So the decision is never made per instance and never made by looking.** It follows from the kind:

- **a kind that declares no capacity is a leaf**
- **a kind that declares capacity is a branch**, whether or not anything is in it

**You never choose between representing something as a leaf or as an empty branch.** You choose
whether the kind may ever hold anything, and the representation follows. **A citizen is a leaf until
the day a rule lets one carry something**, and on that day it is a branch because a rule changed -
visibly, in the specification - rather than because a citizen picked something up.

**And both are allowed, because they are different declarations.** A locked box and a solid block
look alike and are not alike. What is forbidden is the same kind being **either**, decided case by
case.

## One thing worth saying because *leaf* invites it

**A leaf is not featureless.** A citizen has `force`; food has `keeps`. **Being a leaf means holding
nothing, not being simple** - `spec/invariants.md` says a thing is a set of traits whatever else it
is, and that applies at both ends of the tree.

## His adjacency example names the cause, and the id was only the marker

*I couldn't just add a territory to a planet, because it plugs into adjacency.*

**Adding a leaf's contents changes nothing else.** Fourteen metal becomes fifteen; no other row moves.

**Adding a territory rewrites its neighbours.** `adjacency` is a **stored** trait of a place - *which
places it touches, and by which kind of edge* - so a thirteenth territory means writing adjacency
for it **and editing every neighbour's**. **Adding a child mutates its siblings.**

**That is the real difference, and identity is downstream of it.** The chain runs:

1. something must refer to a thing **individually** - adjacency refers to territory 7
2. so it needs an **id**, because *next to metal* refers to nothing
3. so it **cannot collapse into a count**
4. so its container **lists** rather than counts, and is a branch

**So *fungible* is the consequence, *id* is the marker, and *something refers to it individually* is
the cause.** The test to apply to a new kind is the third one, and it is the only one that can be
answered before the kind exists.

## Which makes the planet a particular sort of branch

**Its children are settled when it is made, because its adjacency is geometry rather than
bookkeeping.** `spec/planet.md`: *the distance between any two territories is fixed, and can be
computed by adjacency.* **You cannot add a territory at runtime because you cannot add a face to a
solid.**

**Not a new category, and worth not making one.** It is a branch whose parent's structure makes
adding expensive - and the general statement covers it: **adding a child costs whatever the parent's
structure requires**, which for most containers is nothing beyond capacity, and for a planet is a new
sphere.

**A territory's extractors are the contrast.** Identified, listed, in a branch - and **nothing says
which extractor is next to which**, so adding one costs only capacity. **Same shape, different price,
and the price is a fact about the parent rather than about the child.**

## What this leaves open, which is his to settle

- **Where N's bound comes from** - given by the place, upgraded, or derived from what the place
  produces
- **Whether a vehicle is the same rule.** A metal transport with a bin for fuel and a bin for metal
  is capacity for `fuel store` and `metal store` on a thing that moves. **It looks like the same
  rule with no exception**, which is a good sign
- **Whether every kind of stored thing needs a store**, or only some. Twenty metal with no store is
  the release today

## One thing to reconsider while deciding

**20, 20, 20 is arbitrary uniformity.** Food already differs - it has `keeps` 1, so its limit is
about spoilage while metal's is about room. **Under this model food's store is a kind whose contents
expire**, which is a real difference between store kinds rather than an exception to bolt on.

## What the first version of this note got wrong

It argued that **a bin in a territory is either redundant or reintroduces the trade-off he
dislikes**, and concluded bins only earn their existence in transit.

**The first half was right and the conclusion was wrong.** A bin *is* redundant with capacity - that
is the isomorphism, and he pointed it out rather than disputing it. **Redundancy is not an argument
against the bin, it is an argument that the two are one concept**, and the question is which face
of it to show. Arguing from the redundancy to *therefore no bins in a territory* skipped the step
where the redundancy is the finding.
