# Natural numbers, and what the fusion removed

**Research, 2026-09-09.** Derived. Not binding - a finding is a claim about the tree, not a decision
about it.

[Research](README.md) · [The recipe report](formulas.html) · [Outbox](outbox.md)

Sean, 2026-09-09, in two steps. First: **the guard and the spend are one operation** - a threshold
followed by a destroy of the same thing is a subtraction that fails rather than two lines. Then:
**treat anything that would result in a negative number as an error condition**, with negatives
still expressible conceptually and never literally - *the amount of population lost due to
starvation*.

This records what that costs, what it removes, and what the literature already knows about it.

## What the recipes were already doing

Measured over every recipe in `tools/research/formulas/data.json`, not sampled.

|                                                              |              |
| ------------------------------------------------------------ | ------------ |
| thresholds paired with a destroy of the **identical** amount | **15 of 16** |
| paired, but where the guard and the spend differ             | **0**        |
| guard with no destroy - a read                               | **1**        |
| destroys with no guard                                       | **5**        |

The single read is `move`'s adjacency: *joined to `$from` by an edge the unit crosses*, checked and
not consumed. The five unguarded destroys are not exceptions either - four destroy the **selected**
thing, where selection is the guard, and `grow` spends `n = min(surplus food, citizens)`, where the
`let` has already made the subtraction safe.

**So nothing in the game wanted a guard and a spend to differ.** The split was carrying a
distinction the game never used.

## What it did to the numbers

Every figure below is computed by the renderer from the data, not typed.

|                                        | Before             | After the fusion      | After both decisions |
| -------------------------------------- | ------------------ | --------------------- | -------------------- |
| lines across all recipes               | 80                 | 65                    | **56**               |
| the release's rows, re-encoded         | 58 -> 63, **up 5** | 58 -> 48, **down 10** | unchanged            |
| primitives                             | 6                  | 6                     | **5**                |
| line-counts the report has to explain  | 3                  | **1**                 | 1                    |
| assumptions the console encoding needs | 10                 | **9**                 | 9                    |
|                                        |                    |                       |                      |
| **the world-building `set`s**          | 9                  | 9                     | **0**                |

**The third row was wrong when this report was first written, and the error is worth more than the
row.** It said six primitives became five. They did not: `destroy` and `threshold` became `consume`
and `require` split out of the same move, so **six became six**. The count was a word typed in prose
beside a table that disagreed with it - the same failure as the typed verdict recorded below, and
found the same way, by reading the data next to the claim. **The primitive saving arrived only with
the signed `change`**, a second decision later the same day. Every count on the generated page is
now read from the data, this one included.

The second row is the one worth pausing on. Re-encoding the release into primitives used to **cost**
five lines against the release's own row count, and the report had to explain that the rise was
almost entirely the split. It now **saves** ten.

The fourth row is the same fact from the other side. `consume n k` was *sugar* for a guard plus a
destroy, so what the machine executed and what a person wrote were different counts, and the report
had to carry both and say which was honest. They are now the same number, and the section that
explained the gap is deleted rather than reworded.

The fifth row was found by an assertion rather than by reading. `age` was
`set thing.keeps = thing.keeps - 1` - **the only subtraction in the game that was not a destroy, and
the only arithmetic expression anywhere in the eighty lines.** As a `consume` of 1 from a numeric
trait it stops being arithmetic, and the encoder's own check - *every declared assumption is used at
least once* - went red because nothing needed the arithmetic entry any more.

And `age`'s selector went with it. It selected *each thing with keeps at least 1*, which **is** the
non-negativity condition, so the selector dissolved into the operation. It now selects *each thing
with keeps* - which is a question about whether the trait exists, not about its value.

## What the literature calls it

A finite control state plus a vector of counters that may never go below zero, with rules that add
integer vectors and are legal exactly when nothing goes negative, is a **vector addition system with
states**. `phase` is the control state; every quantity is a counter. This is not an analogy: it is
the definition, and it is why boundedness and coverability stay decidable.

**Corrected 2026-09-09, same day.** This section first said the model needs two operations where
the formalism uses one, because a negative could not be written. That was wrong about Sean's
intent - **amounts are signed, and it is the *result* that may not go negative.** So the fit is
exact rather than approximate: a signed delta vector, counters that must stay non-negative, and no
adaptation in between.

**The consequence was a question, and Sean took it the same day.** If amounts carry a sign then
`create` and `consume` are one operation with the sign fixed, and they now are: **`change`**, with
17 positive lines and 21 negative ones. `X-11` reached the same place from the other direction -
*`subtract` has no counterpart, which a signed amount repairs.*

What that costs is one thing worth naming: **the operator no longer says which way a line goes**, so
the sign has to be read. The generated tables colour it for that reason, which is a presentation
answer to a real loss of legibility rather than a denial that there is one.

**`clamp` is monus.** Truncated subtraction, `a ∸ b = 0` when `b` exceeds `a`, is the operator that
makes the natural numbers a commutative monoid - the standard way to total a subtraction that is
otherwise partial. It was a candidate value with no call site and an invented meaning; it is now a
named operation with a definition older than the game.

So the `attach` column stops being a list of failure behaviours and becomes **four answers to one
question** - what happens when this subtraction would go below zero:

|            |                                                                       |
| ---------- | --------------------------------------------------------------------- |
| *(blank)*  | it provably cannot                                                    |
| **hard**   | subtraction as a partial function: undefined, and the caller fails    |
| **soft**   | skip the line, and the action still succeeds                          |
| **clamp**  | monus - go to zero and no further                                     |
| **record** | take what is there, write the shortfall where another recipe reads it |

**`record` is the value Sean's rule names.** It was already in use and had no name: `unpaid` is a
derived trait and `perish` fires on it, so a failed upkeep is neither a hard failure nor a skip.
Starvation is exactly the case he raised - **the population lost is an event, never a negative
number** - and it is now the same mechanism as everything else rather than the one exception.

A capacity joins them by the same route. An upper bound is a count of **free space**, so filling a
container is subtracting from that counter, and *the container will not take another* is the same
condition as any other underflow. That is the standard complement-place construction for bounded
Petri nets. Check 4 already computes this for creates, mechanically: five attachments matter, each
because its container can refuse, and none is unobservable.

## The warning, which is the half worth having

**The usual way to build "only numbers that can exist" is unsigned arithmetic, and it is a notorious
defect class.** C requires unsigned subtraction to wrap, so `0 - 1` is not an error but a very large
positive number, and the result is `CWE-191`, *Integer Underflow (Wrap or Wraparound)* - still
producing memory-corruption and code-execution vulnerabilities in shipped software.

The distinction matters because it is exactly the thing that makes Sean's version safe: **the value
of the design is in the error, not in the unsignedness.** A model whose counts cannot be negative
and whose underflow *wraps* is worse than one that allows negatives, because the impossible state
becomes a plausible number instead of a stop. Named here so that the implementation cannot quietly
be the other one - and his shape avoids it by construction, because **a signed delta against a
checked result has somewhere to put the failure**, where an unsigned counter has only a number.

This is the general form of what he described as *a model only capable of representing things that
actually exist*: the representable states are exactly the possible ones, so a defect that would have
been a wrong value becomes an unrepresentable one. It buys less than it promises only when the
representation admits a value the domain does not - which is precisely the unsigned-wrap case.

## What does not collapse

Three things, and saying so is the point of the exercise.

- **Grounding.** Territory 6 has no metal deposit, so `build extractor[metal]` is not *refused*
  there - it does not *exist* there. That is a question about which instances a recipe has, not
  about a count, and it is `X-8` and `X-17`.
- **Phase.** A design command in the play phase is a test on a trait's value. In the VASS reading it
  is the control state, which is a different thing from a counter by construction.
- **`spoil` selects *each thing with keeps 0*, which is a zero test** - the inhibitor arc `X-9`
  warned ends every analysis. It is safe **only because `keeps` is bounded**, and a zero test on a
  bounded counter folds into the finite control. So the rule needs `X-9`'s qualifier travelling with
  it: *no zero test on an **unbounded** quantity*. Without that clause, "negatives are an error"
  reads as though it had banned zero tests, and it has not.

And `age` and `spoil` do not merge for free. Destroying a thing when its decrement would underflow,
rather than when `keeps` is already 0, moves every death one turn later.

## Composition, and the objective measure Sean asked for

Added the same day, after he stated the objective: **minimum expressiveness and maximum
conciseness, constrained by completeness** - and that if being less expressive forces longer
expressions to say the same thing, that is a failure.

**That test has a name and a definition.** Felleisen, *On the expressive power of programming
languages* (1991): a construct is **eliminable** if it can be removed by a **local, syntax-directed
transformation** - a macro - that leaves the surrounding program's structure untouched. If removing
it forces a global restructuring, it is genuinely more expressive rather than sugar. That is exactly
his criterion, stated as a property rather than a preference, and it is the same instrument `X-11`
already used through Nebel's compilation schemes.

**His suspicion about `call` is correct, and it is the standard reason abstraction exists.**
Procedural abstraction is what decouples succinctness from expressive power: it lets shared
structure be named once instead of copied, so a small primitive set does not force long recipes.
This is already measured here rather than argued - `X-12` found `deploy ark` and `found by land`
sharing seven rows verbatim, and factoring them into `found-colony` took **17 release rows to 10
lines**. The caveat is `X-9`'s and has not moved: **acyclic** calls buy this for free, and recursive
ones make the analysis undecidable.

### Whether the lines are sequential

Sean's inclination is that they are not, and that sequential composition is too expressive and will
produce confusing recipes. Measured over all 22 recipes and 65 lines:

|                                                                    |       |
| ------------------------------------------------------------------ | ----- |
| a line reading a **count** another line in the same recipe changes | **0** |
| a line reading a **trait** another line writes                     | **1** |
| the same trait assigned twice                                      | **0** |
| a `set` on a thing a `create` in the same recipe brings into being | **4** |

**Nothing in the game uses sequential composition.** The one trait case is `move`: `let from =
unit.location` on line 1, and `set unit.location = $to` on line 4. Under non-sequential semantics
every line reads the starting state, so `from` is the origin **by construction**. Under sequential
semantics it is the origin **because the `let` was written first** - and reordering two lines would
silently make a unit move from its own destination. That is one recipe, and it is the whole
argument from defects rather than from taste.

The four `create`-then-`set` cases are all world-building, and they are the real objection: under
non-sequential semantics a `set` on a thing that does not exist at the start has no referent. **The
fix makes the recipes shorter, which is the outcome his test asks for.** A `create` already takes a
description, and a description is a kind with its traits - so `create territory in game` followed by
three `set`s is one line that creates a territory with its id, biome and nature. Across the four:
**13 lines become 4**, and the total goes from 65 to **56**.

**And the checks already assume it.** `effects()` in `check.py` accumulates a recipe's net effect
as `net[k] += sign * n` over its lines, in any order - because check 1 is a P-invariant and check 2
is a linear program, and both need a recipe to *be* a single vector. **If composition were
sequential, both checks would be unsound**, since the guard would apply at each step rather than to
the net. That is not an argument from taste either: it is the analysis the report already runs.

So sequential composition is strictly more permissive - `create 3, consume 2` starting from nothing
succeeds under it and fails under the other - and by his own test it does not earn that: nothing
uses it, removing it makes the recipes shorter rather than longer, and keeping it would invalidate
two of the checks.

**Decided the same day, and the prediction held exactly.** Composition is not sequential, and a
`change` carries its traits. The four world-building recipes went from 13 lines to 4, the total
from 65 to 56, and **the nine `set`s in world-building became none** - so building the world is now
a positive `change` and a `call`, and nothing else. The finding at the top of the generated report
gets stronger rather than weaker: it used to say world-building needs `create` and `set`, and it
needs neither a guard nor a `set` now.

`set` survives for the five places that change a thing which already exists - `move`'s location and
ready, `create labor`'s and `work`'s ready flips, and `refresh`. **That is what `set` is now for**,
and the primitive's own description says so, which it could not while world-building depended on it.

## Two defects this found, both in this lane's own tooling

Recorded because they are the same failure the repository keeps writing down.

**A verdict that was typed rather than computed.** The checker printed `UNBOUNDED WITHOUT MINING`
and the page beside it printed `clean`. Both were true - the only loop gains no metal and lives in a
declared-free kind - but the badge was a hand-written word, so **nothing could have turned it red.**
It was found only because swapping the data made me read the checker's output next to the page.
Fixed by having check 2b report the breakdown its verdict needs and deriving the badge from it.

**An assumption list that would have kept a dead entry.** The encoder asserts that every declared
assumption is used at least once. When `age` stopped being arithmetic, that assertion failed - which
is the assertion working. Without it the list would have gone on claiming the model contained an
arithmetic expression it no longer had.

Neither was found by a check aimed at it. Both were found by an assertion firing on the way past,
which is the argument for writing the assertion rather than the note.

## What is not settled

- Whether `require` stays a primitive or becomes sugar for a self-loop - `consume` then `create` the
  same thing back. The loop is the classical encoding and costs one line in one recipe; a read arc
  is the better model of *checking without taking*, and reading and returning an adjacency
  represents something that never happens, which is the kind of stored abstraction the rule exists
  to remove. Left as a primitive, used once.
- Whether `clamp` is adopted at all. It now has a definition and still has no call site.
- **Whether `create` and `consume` are one operation.** They are, if amounts are signed - which is
  Sean's intent - and that would make the set four primitives. Not decided, and not implemented.
- **Whether composition is non-sequential**, and with it whether `create` carries its traits. The
  measurement above says nothing uses sequential composition, the checks already assume it is not,
  and removing it shortens the recipes by nine lines. Not implemented either.
- The nine remaining assumptions in the console encoding, three of which are holes in the
  specification rather than in the notation - see the report.
