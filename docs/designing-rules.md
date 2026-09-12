# Designing rules

What the recipe net buys, what it costs, and how to tell before you write a rule whether it is
expressible.

[Documentation map](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

The recipes are a **coloured Petri net**, and the difference from a plain one is load-bearing.

**A place holds a count, a recipe is a transition, and a row is an arc carrying a constant weight.**
That is not an analogy - it is the mapping `crates/game-console/src/petri.rs` builds and
`reports/nogain.md` computes over.

## Two nets, and the guarantees belong to the second

**What we write is coloured.** A rule whose subject is a **family** is one rule standing for several
- `discard` over the resources, `refresh` over the things that act. An amount **read from a trait**
is the same move: `work` makes a territory's density, which is one rule with a number per case.
**The family and the trait are the colours.**

**What the guarantees are about is the plain net you get by unfolding it.** `reports/nogain.md` says
so in its own words: *41 rules, ground from 23 blocks of recipe rows - a family becomes its members,
a density becomes its cases.* **Boundedness and termination are decidable for the unfolded net**,
and that is the net the weighting is solved over.

**So every colour set must be finite, and that is a real constraint rather than a formality.** A
family has a listed membership; a trait read as an amount has a listed set of values. **A rule
parameterised over something unbounded has no finite unfolding, and every guarantee in this document
is about a net that would not exist.**

**There is more than one legitimate unfolding.** The check grounds to places at `(kind, state)`; the
drawing grounds to `(container, kind)`. Both are correct readings of the same coloured net at
different granularities, which is why one can see a thing the other cannot - `work` nets to zero in
the drawing and not in the check.

**What breaks it is an arc the unfolding cannot flatten**: one that tests a place is **empty** - an
inhibitor arc - or one whose weight **reads the marking** rather than being a constant, which is a
reset or transfer arc. `X-29` counted ten of the second kind in this game and found **all ten in
world recipes and none in a player's**.

**This document exists so the constraints are known in advance.** The design space is wide, and
almost all of it is reachable; what is not reachable is small, specific, and worth recognising on
sight rather than discovering after a rule is written.

## What the net buys

**The no-gain invariant is decided rather than believed.** `spec/invariants.md` says *there is a
weighting of the kinds, and under it no sequence of rules ends holding more than it began with*, and
that **whether this holds is decided mechanically, from the rules alone.** That is a **place
invariant**: a weighting `w` where `w · (made - taken) <= 0` at every rule. `reports/nogain.md`
solves for one from the *Recipes* table and publishes the weighting it found, so a number that looks
wrong is a fact about the rules rather than about anybody's judgement of them.

**It says which rule shapes are legal before one is written.** The table below is the whole of it.
A shape that is not in it is not a restriction discovered late; it is a shape the formalism never
had.

**The rules stay data, because a net is rules-as-data by construction.** Anything expressible as a
net is expressible as rows of a table, which is what makes a rule something a player can edit rather
than something the program knows.

**A bad player-written rule becomes refusable at edit time, with a reason.** For the unfolded net,
boundedness and termination are decidable - so an editor can say *this rule never finishes* rather
than letting a game discover it. **A Turing-complete rule language cannot have this feature at all**,
which is why the constructs below are refused rather than merely discouraged.

**It is a second reading of the same table.** The check derives its places at `(kind, state)`; the
drawing derives its own at `(container, kind)`. Two derivations from one source, so a disagreement
between them is visible - which is how `work` was found netting to zero in the drawing.

## What it costs

Every one of those guarantees is bought with the same currency: **no rule may ask how much of
something is present, and no rule may test that something is absent.** Everything else refused
below is one of those two wearing different clothes. **The constraints are not a tax on the design.
They are the guarantee, stated in advance.**

## Deciding whether a formula is allowed

Ask two questions in order. **If both answer no, the formula is expressible.**

**1. Does the amount depend on how much is present?** `spec/invariants.md`: *a rule's amounts are
constants*, and *what one firing takes and makes does not depend on how much of anything is
present*.

**This is usually a yes that becomes a no**, because the same document gives the rewrite: *where a
rule would need a quantity that varies, it is written as a smaller rule that fires as many times as
it can*, and *the quantity is then how often it fired*.

**An amount read from a trait is not an amount that depends on what is present.** `work` produces a
territory's density; that is one rule with a number per case, and it is allowed.

**2. Does the rule need to know something is absent?** A zero test is an **inhibitor arc**, and with
one the net is Turing-complete: reachability stops being decidable and so does everything this
document promises. **This is the one that costs the invariant outright.**

**A comparison is an instance of the second, which is why there are two questions and not three.**
*The largest of them* is *this one, and nothing is greater* - and **nothing is greater** is a test
that a set of places is empty. **Max is not refused for being a comparison; it is refused for being
a zero test wearing a comparison's clothes.**

**Comparing two things you have named is a different act and is allowed.** `spec/logistics.md`:
*there is never a quantity of a thing with an `id`* - a named thing is one thing, and `P-396` lets a
rule name one and say what changes about it. **What cannot be done is ranking the contents of a
place**, because the contents are a count and finding the greatest means proving the rest are not.

## The folds, and what to do with each

| Fold                    | Allowed  | How it is written                                                                    |
| ----------------------- | -------- | ------------------------------------------------------------------------------------ |
| **sum**                 | **free** | it is the marking of a place. You never compute it                                   |
| **min**                 | **yes**  | a pairing - one rule that spends one of each, fired as many times as it can          |
| **at least n**          | **yes**  | an input arc of weight `n` - a `require` or a `consume` of `n`                       |
| **at most n**           | **yes**  | a capacity. `P-374`: what is stored is the room left, so *at most* is *room remains* |
| **a number per kind**   | **yes**  | an amount read from a trait - `P-376`                                                |
| **max**                 | **no**   | *nothing is greater* is a zero test. Over things you have named it is allowed        |
| **a branch on absence** | **no**   | an inhibitor arc. Legal in a file, fatal to the guarantee                            |

### `min` is a pairing, and the game already contains one

| Recipe    | Auto  | Role    | Qty | Kind      |
| --------- | ----- | ------- | --- | --------- |
| **breed** | world | consume | 1   | fertility |
|           |       | consume | 1   | food      |
|           |       | produce | 1   | citizen   |

It fires as many times as it can, which is **min(fertility, food)**. `P-379` replaced the old `grow`
- which read *the lesser of the surplus food and the citizens here* and was forbidden - with this.
**The min survived; the reading of it did not.**

### A worked example: force is `2 + min(guns, citizens)`

| Recipe    | Auto  | Role    | Qty | Kind     | Traits              |
| --------- | ----- | ------- | --- | -------- | ------------------- |
| **stand** | world | require | 1   | garrison | standing at least 1 |
|           |       | put     |     | garrison | standing one less   |
|           |       | produce | 2   | force    |                     |
| **arm**   | world | require | 1   | citizen  | arming at least 1   |
|           |       | put     |     | citizen  | arming one less     |
|           |       | require | 1   | gun      | arming at least 1   |
|           |       | put     |     | gun      | arming one less     |
|           |       | produce | 1   | force    |                     |

`stand` fires once per garrison, making **2**. `arm` fires once per pair it can make, making
**min(guns, citizens)**. Every arc carries a constant weight. **The formula is expressible exactly as
written, and nothing about it strains the net.**

### A worked counter-example: a garrison that makes citizens sum instead of max

This is the shape that cannot be had, and it is worth knowing why, because it looks harmless.

**Sum is free** - it is what a marking is. **And max here is degenerate**: every citizen has force
1, so *the highest among them* is *1 if any citizen is present*, which is an ordinary threshold.
**Both halves are individually legal.**

**What is not legal is choosing between them.** Two behaviours selected by whether a garrison exists
means two rules: one requiring a garrison, and one requiring that there is **none**. The second is a
zero test. **The fold was never the problem; the branch was.**

## How to rewrite a formula that fails the test

**A varying amount becomes a repeated firing.** Write the rule for one, and let it fire as many
times as its inputs allow.

**A branch on absence becomes a presence.** Ask what is true when the thing is missing, and require
*that* instead. A rule that should fire only without a garrison is usually a rule that should fire
when something else is there.

**Absence is sometimes a capacity, and sometimes only looks like one.** `P-374` makes *room for a
garrison* a count, so *there is no garrison* and *there is room for one* coincide - **but only where
the capacity is exactly one.** `spec/invariants.md` states the trap: *those differ wherever a
capacity is more than one, and agree only by accident where it is one.* Use it knowing which case
you are in.

**A comparison becomes an identity or a redesign.** If the things being compared carry an `id` they
are distinguishable and a rule may name one - `P-396`. If they do not, the comparison is asking a
question the state cannot answer, and the rule wants a different shape.

## Open questions

**Nothing states that the player's sublanguage is the ordinary one.** `X-29` measured that every
marking-reading arc in the game is in a **world** recipe and none in a player's, so the property
holds **by accident**. Until a rule says so and a check enforces it, the first player-authored recipe
that empties a place moves the editor into a class where none of this is decidable.
