# Constant amounts

**Specification lane, 2026-09-10.** Why five of the release's recipes cannot be weighed by either
structural check, what Sean's resource X does about it, and why the two holes the notation still has
turn out to be one hole.

[Notes index](README.md) · [Invariants](../../spec/invariants.md) · [The release](../../releases/first-release.md)

## The question

Sean, 2026-09-10, on population growth: *is this coded by a function that takes a minimum now? Is
dependency on a function like that suited to a petri net or does it conflict?*

**It conflicts, and the conflict is already being paid for.**

A Petri net's incidence matrix has **constant** integer entries: a transition consumes exactly `w`
tokens from a place, whatever the marking. Both of the research lens's structural checks are linear
algebra over that matrix - **check 1** looks for a weighting under which net change is zero, a
P-invariant; **check 2** for a non-negative firing vector with `C·x ≥ 0`, a T-invariant. **Neither is
defined when an entry is a function of the marking.**

Measured, from `tools/research/formulas/check.py`:

```
CHECK 1  13 recipes analysed, 5 skipped for state-dependent amounts
         skipped: end-of-turn losses, grow, perish, refuel, upkeep
CHECK 2  18 transitions in the matrix, 3 skipped for state-dependent amounts
```

**A minimum is the cause in four of the five.** `grow` is
`min(count {food surplus:yes in t}, count {citizen in t})`; `refuel` and `end-of-turn losses` are
minima too; `upkeep` reads `thing.upkeep`.

## The state was never the problem

**Sean's phrasing is worth sharpening, because the sharper version says where to look.** He asked to
keep *the game state* representable as a Petri net. **The state already is one.**
`spec/console.md`: *what a thing contains is a map from a description to a quantity* - which is a
marking, exactly.

**What breaks the net is a rule whose amounts depend on the marking**, not the state's shape. So the
constraint belongs on how a rule may be written, and nothing about the dump or the state has to
change.

## Resource X, and why it is better than deriving the minimum

**Sean's proposal.** An arbitrary resource X; each citizen produces one per turn, on an exhaustion
trait of its own; a new citizen costs 1 X and 1 food.

```
make-x:  consume 1 citizen [fertile]  ->  produce 1 citizen [spent], 1 X
breed:   consume 1 X, 1 food          ->  produce 1 citizen
```

**Both have constant arc weights, and no minimum appears anywhere.**

**Checked rather than argued.** Over all 2400 `(citizens, food)` pairs with citizens 0-39 and food
0-59, firing upkeep, `make-x` and `breed` to saturation in turn order gives **exactly**
`population_after(citizens, food)` as `crates/game-model/src/territory.rs:600` computes it today.
**Zero disagreements.**

**The doubling cap stops being a clause and becomes a consequence.** Today it is written
`.min(citizens)` - a stated ceiling. Under resource X there are only ever `C` of X, one per citizen,
so at most `C` births can happen. **The cap is arithmetic on how much X exists rather than a rule.**
That is the move `P-290` made when capacity replaced nodes: a bound stopped being bookkeeping and
became a fact about the thing.

**And two fused facts come apart.** *A citizen can contribute to one birth per turn* and *a birth
costs food* become independent, so tuning one no longer touches the other.

### What it costs

- **+1 kind, +1 trait, +1 recipe.** X is a kind, the second exhaustion is a trait on a citizen, and
  `grow` becomes two rules
- **X must be discarded at a turn's end**, or a territory banks fertility and then explodes. That is
  what `end_of_turn_losses` already does to labor, so X joins that line
- **X is a second labor.** A citizen has one labor and one X per turn, independently, so it may work
  *and* breed. That preserves today's rule, where every citizen counts toward growth whether or not
  it worked. Merging them would change the numbers
- **X has no fiction yet.** An unnamed resource is a modelling artifact showing through, and
  `spec/narrative.md` means this game has a fiction to answer to

### The alternative this lane offered first, and why it is worse

A single transition - *consume 1 citizen ready and 1 surplus food, produce 2 citizens not ready* -
fired to saturation gives the same numbers, because it runs out of whichever side is scarcer.
**It still contains the minimum; it just hides it in the firing count.** Resource X removes it.

## The two holes are one hole

**This is the finding worth keeping.** Two things the notation cannot do have been tracked
separately:

- **State-dependent amounts** - `S-86`, and the five skipped recipes above
- **Grounding, a selector naming a family rather than a kind** - the research lens's heaviest
  assumption at 12 of 80 encoded lines, and the gap `P-366` did not reach

**They are the same problem.** `perish` is skipped because it *consumes a thing and produces that
thing's metal, so its effect depends on which thing* - a family in the selector, and therefore no
constant column. **Ground the family to its kinds and each kind gets its own transition with a
constant weight.** So grounding does not merely make a checker's life easier; **it is what makes a
constant matrix exist at all.**

Four of the five skipped recipes are fixed by saturation, and the fifth by grounding. **Between them
there is no recipe that has to keep a variable amount.**

## What is not settled

**Sean, 2026-09-10:** *the petri net is not the goal either, it is a means to an end, and while I
understand the end well enough to know when it is violated, I don't understand it well enough to
articulate it with precision.*

**So the means is stated and the end is not**, and this note does not pretend otherwise. What the
analysability buys - whether it is that a player's rules always finish, that a rule editor cannot
produce a broken game, that a glitch is detectable before shipping, or something none of those quite
names - is the open question. It is recorded in `spec/invariants.md` under that heading rather than
guessed at here.
