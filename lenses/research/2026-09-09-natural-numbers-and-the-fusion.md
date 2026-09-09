# Natural numbers, and what the fusion removed

**Research, 2026-09-09.** Derived. Not binding - a finding is a claim about the tree, not a decision
about it.

[Research](README.md) · [The formula report](formulas.html) · [Outbox](outbox.md)

Sean, 2026-09-09, in two steps. First: **the guard and the spend are one operation** - a threshold
followed by a destroy of the same thing is a subtraction that fails rather than two lines. Then:
**treat anything that would result in a negative number as an error condition**, with negatives
still expressible conceptually and never literally - *the amount of population lost due to
starvation*.

This records what that costs, what it removes, and what the literature already knows about it.

## What the formulas were already doing

Measured over every formula in `tools/research/formulas/data.json`, not sampled.

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

|                                        | Before            | After                |
| -------------------------------------- | ----------------- | -------------------- |
| lines across all formulas              | 80                | **65**               |
| the release's rows, re-encoded         | 58 → 63, **up 5** | 58 → 48, **down 10** |
| primitives                             | 6                 | **5**                |
| line-counts the report has to explain  | 3                 | **1**                |
| assumptions the console encoding needs | 10                | **9**                |

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

One cost of Sean's constraint, named rather than hidden: a VASS uses a single **signed** vector.
Because a negative may not be written, the model needs two operations - `create` and `consume` -
where the formalism uses one. That is the price, and it is cheap.

**`clamp` is monus.** Truncated subtraction, `a ∸ b = 0` when `b` exceeds `a`, is the operator that
makes the natural numbers a commutative monoid - the standard way to total a subtraction that is
otherwise partial. It was a candidate value with no call site and an invented meaning; it is now a
named operation with a definition older than the game.

So the `attach` column stops being a list of failure behaviours and becomes **four answers to one
question** - what happens when this subtraction would go below zero:

|            |                                                                        |
| ---------- | ---------------------------------------------------------------------- |
| *(blank)*  | it provably cannot                                                     |
| **hard**   | subtraction as a partial function: undefined, and the caller fails     |
| **soft**   | skip the line, and the action still succeeds                           |
| **clamp**  | monus - go to zero and no further                                      |
| **record** | take what is there, write the shortfall where another formula reads it |

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
be the other one.

This is the general form of what he described as *a model only capable of representing things that
actually exist*: the representable states are exactly the possible ones, so a defect that would have
been a wrong value becomes an unrepresentable one. It buys less than it promises only when the
representation admits a value the domain does not - which is precisely the unsigned-wrap case.

## What does not collapse

Three things, and saying so is the point of the exercise.

- **Grounding.** Territory 6 has no metal deposit, so `build extractor[metal]` is not *refused*
  there - it does not *exist* there. That is a question about which instances a formula has, not
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
  same thing back. The loop is the classical encoding and costs one line in one formula; a read arc
  is the better model of *checking without taking*, and reading and returning an adjacency
  represents something that never happens, which is the kind of stored abstraction the rule exists
  to remove. Left as a primitive, used once.
- Whether `clamp` is adopted at all. It now has a definition and still has no call site.
- The nine remaining assumptions in the console encoding, three of which are holes in the
  specification rather than in the notation - see the report.
