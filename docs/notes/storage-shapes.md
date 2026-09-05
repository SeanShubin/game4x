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
