# Least expressive yet complete, and the test for whether a primitive earns its place

2026-09-08. For Sean, who has sketched four candidate primitives - composition, subtract,
fail-behavior, and constraints on parameters - and expects some to be subsumed by simpler parts.

**His judgement is the measure and this report does not argue otherwise.** What it offers is the one
place where the literature supplies a *decision procedure* rather than a taste, and a decomposition
of his four that he can check against his own specification in an afternoon.

> **Superseded in one respect, 2026-09-11.** The release no longer has the four roles this report
> decomposes: `P-421` declared a fifth, `put`, and `P-385` deleted both `limit` rows. **The
> decomposition is unaffected and the count is wrong** - the grid was always over the roles that
> carry a quantity, and a `put` carries none, so `put` is the other axis of this report's own
> fact-versus-count split rather than a fifth cell in the grid. `X-11` carries the argument and is
> closed.

Citations checked against sources in the session that wrote this.

## The test for a primitive, which is not a matter of taste

*Least expressive yet complete* has a formal counterpart. Nebel's **compilation schemes** compare
formalisms by asking whether one can be translated into another with only polynomial growth, and the
central result for this question is precise: **conditional effects cannot be compiled away if plan
size may grow only linearly, and can be compiled away if polynomial growth is allowed.**

**His fail-behavior is exactly conditional effects.** And the compilation that removes them - Gazen
and Knoblock, 1997 - expands one operator into **one operator per combination of which conditions
hold**, which is exponential in the number of optional parts; that blowup has been shown not to be
improvable.

**So fail-behavior is provably not sugar.** And the reason that matters here is sharper than the
theorem. Removing it does not reduce the complexity of the system - **it moves the explosion out of
the generated space and into the hand-written one.** A recipe with four optional parts becomes
sixteen recipes a person has to author and keep consistent.

**Which yields the test, and it is the whole of what this report is for:**

> **A primitive earns its place when removing it moves the combinatorial explosion from the
> generated space into the authored space.**

That is the rigorous form of *unlimited complexity through an insanely simple structure*: the
explosion is the point, and it belongs on the machine's side of the line. **Anything whose removal
only makes the generated space larger is sugar and should go.**

## His four, decomposed

**Two of the four are one thing, and he half-said so.** *Constraints on parameters* is a **guard**;
*fail-behavior* is **where the guard attaches**. His own example makes them the same sentence: the
garrison limit is a guard, and *failing to build a garrison might not fail the command* is that guard
attached softly.

**And `subtract` is asymmetric** - nothing produces. Making the amount **signed** repairs it and
collapses more than it looks:

| His current role    | Change | Threshold     |
| ------------------- | ------ | ------------- |
| `consume 3 food`    | **-3** | at least 3    |
| `require 3 workers` | **0**  | at least 3    |
| `produce 1 metal`   | **+1** | none          |
| `limit 0 garrison`  | none   | **at most 0** |

**The four roles are not four things.** They are two independent dimensions - a **change** and a
**threshold** - and every role is a point in that grid. `require` and `consume` differ only in
whether the change is zero. `limit` is a threshold pointing the other way.

**This is why `create-if-missing` had nowhere to go.** It is not a missing fifth role; it is a cell
the grid always had and the `Role` column could not name, because that column conflated the two
dimensions into one word.

So a candidate set, offered as a decomposition of his rather than a proposal:

1. **change** - a named thing, a signed amount
2. **threshold** - a named thing, a bound, and a direction
3. **attachment** - hard, so failing it fails the action; or soft, so failing it skips its own line
4. **composition** - the lines of a recipe are applied

### The demonstration, which he can check today

Under that decomposition, the question that took four items across three lanes to circle is **one
word in one column**:

- Today's garrison: `threshold(garrison, at most 0, **hard**)` and `change(garrison, +1)`
- His sketch: `threshold(garrison, at most 0, **soft**)` and `change(garrison, +1)`

**Same two lines. One word different.** If a decomposition makes a distinction that was hard to state
into a single word in a single place, that is the strongest evidence available that it is cutting at
a joint - and it is evidence he can check without building anything.

## What does not collapse, and is worth knowing before it is discovered

- **A threshold with a zero change is not expressible as a change.** *Needs three workers present,
  spends none* has a net effect of nothing, and nothing is not a requirement. So the threshold is a
  genuine second primitive and not a special case of the first. In Petri-net terms it is a read arc,
  and read arcs are famously not simulable by ordinary arcs without changing the net's behaviour.
- **Upper and lower thresholds are one primitive only if complementary quantities exist** - *at most
  one garrison* is *at least zero garrison-slots-free*. That construction is the same one behind
  `C-75`'s boundedness finding. **Probably not worth taking**: it buys one fewer direction in the
  grammar and costs a person reading *garrison-slots-free* in a specification.
- **Composition needs its own decision that his sketch has not yet made.** *Each line is applied* can
  mean the lines see the starting state, or that each sees the effects of the ones before it. Those
  differ whenever two lines touch the same thing, and a recipe that consumes and produces the same
  resource will tell them apart immediately.

## On his expectation that this will find problems in the current specification

**It will, and the first one is already visible from the grid above**: the `Role` column names four
things that are two dimensions, which is why one legitimate combination had no name. That is a defect
in the present structure found by re-encoding rather than by review, which is what he predicted would
happen.

**And on the measurement question from the previous report** - he is right and this lane over-reached.
The definitive measure is his judgement, and a number was never going to substitute for it. **What
the earlier warning is actually good for is narrow**: if he ever reports that the re-representation
*failed to simplify*, the first thing to check is whether the thing being counted was total size,
because that is the one measure that moves the wrong way when the design is working.

## What this lens is not saying

The decomposition above is a reading of his four, not a proposal, and the naming is his. Whether
composition is sequential or simultaneous, whether upper thresholds stay in the grammar, and whether
a change and a threshold sit on one line or two, are all decisions this report deliberately leaves
open.

## Sources

- Compilation schemes, and that conditional effects cannot be compiled away preserving linear plan
  size but can with polynomial growth - [Nebel, *On the Compilability and Expressive Power of
  Propositional Planning Formalisms*, JAIR 12 (2000)](https://arxiv.org/pdf/1106.0247),
  [Compilation Schemes: A Theoretical Tool](https://link.springer.com/chapter/10.1007/3-540-48238-5_15)
- Gazen and Knoblock 1997, expanding one conditional operator into exponentially many STRIPS
  operators, and that the blowup is not improvable -
  [Conditional Effects in Graphplan](https://cdn.aaai.org/AIPS/1998/AIPS98-006.pdf),
  [Nebel, as above](https://arxiv.org/pdf/1106.0247)
