# Designing rules

What the recipe net buys, what it costs, and how to tell before you write a rule whether it is
expressible.

[Documentation map](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

The recipes are a **Petri net**: each kind-and-state is a **place** holding a count, each recipe is a
**transition**, and each row is an **arc** carrying a constant weight. That is not an analogy - it is
the mapping `crates/game-console/src/petri.rs` builds, and `reports/nogain.md` computes over.

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

**A bad player-written rule becomes refusable at edit time, with a reason.** For an ordinary net,
boundedness and termination are decidable - so an editor can say *this rule never finishes* rather
than letting a game discover it. **A Turing-complete rule language cannot have this feature at all**,
which is why the constructs below are refused rather than merely discouraged.

**It is a second reading of the same table.** The check derives its places at `(kind, state)`; the
drawing derives its own at `(container, kind)`. Two derivations from one source, so a disagreement
between them is visible - which is how `work` was found netting to zero in the drawing.

## What it costs

Every one of those guarantees is bought with the same currency: **no rule may ask how much of
something is present, test that something is absent, or compare one thing against another.** The
constraints are not a tax on the design. They are the guarantee, stated in advance.

## Deciding whether a formula is allowed

Ask three questions in order. **If all three answer no, the formula is expressible.**

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

**3. Does the rule need to compare individual things?** *The largest, the oldest, the strongest of
them.* **Tokens in a place are anonymous** - eight citizens are a count of eight, not eight things
you can rank. `spec/logistics.md` says the same from the other end: *there is never a quantity of a
thing with an `id`*. **A comparison needs identity; a quantity has none.**

## The folds, and what to do with each

| Fold                    | Allowed  | How it is written                                                                    |
| ----------------------- | -------- | ------------------------------------------------------------------------------------ |
| **sum**                 | **free** | it is the marking of a place. You never compute it                                   |
| **min**                 | **yes**  | a pairing - one rule that spends one of each, fired as many times as it can          |
| **at least n**          | **yes**  | an input arc of weight `n` - a `require` or a `consume` of `n`                       |
| **at most n**           | **yes**  | a capacity. `P-374`: what is stored is the room left, so *at most* is *room remains* |
| **a number per kind**   | **yes**  | an amount read from a trait - `P-376`                                                |
| **max**                 | **no**   | it needs to tell the things apart, and a count cannot                                |
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
