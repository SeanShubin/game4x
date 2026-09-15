# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-516 - Moving resources: put or consume-and-produce, and how the fuel says who burnt it

**to** sean · **status** open · **raised** 2026-09-14 · **kind** invented · **shape** an instruction · **asks** a decision · **into** `releases/first-release.md` -> Recipes, and `spec/console.md` if the notation moves

**You are declaring resources with the unit and letting the interface default.** The command is the
same under every option below:

```
{move unit:transport from:1 to:2 metal:7 energy:2}
```

## First, the thing you were worried about is not a problem

**Energy not balancing is correct and checked.** `spec/invariants.md`: *no sequence of rules ends
holding **more** than it began with.* **Ending with less is not what that forbids.** A move burns
fuel, the star is where fuel comes from, and the arithmetic that would fail is a rule that **made**
energy from nothing.

**So no option here has to make energy balance**, and one that did would be hiding the cost rather
than paying it.

## Second, and this lane had it wrong until Sean pushed on it

**The first version of this item said: `put` for what has an `id`, `consume` and `produce` for what
is counted.** Sean: *I am not so sure put applies to units anymore... if a 100 identical transports
are moving 1000 resources it is not clear that there is any substantive difference between the units
and the resources.*

**He is right, and the measurement is worse than he put it.**

```
kinds carrying an `id`        territory, orbit
put rows on either of them    0 of 17
```

`releases/first-release.md` justifies `put` as *the same thing and not a new one, **so what has an
identity keeps it***. **That sentence is true of no row in the game.** Every put is on a `unit`, a
`citizen`, an `extractor`, a `thing` or a `nature`, and not one of those carries an `id`.

## What `put` is actually for, which is in the code and not in the release

`crates/game-console/src/petri.rs`: **a count is a place of its own, and the kind's own place is
untouched.** *A citizen that spends its `laboring` is the same citizen afterwards, so the arc is on
`citizen laboring` and nothing goes in or out of `citizen`.* **Drawing it on the kind instead would
show `create labor` eating a citizen.**

**So `put` is about a state change not reading as a destruction**, and has nothing to do with
identity. The release names the wrong reason, and names it in the one place a reader would look.

## And that sharpens where the line falls

```
put rows that change a state, in place     16
put rows that change a place               1     move, `moving one less` at `$to`
```

> **A `put` is a change of state where the thing already is. A change of place is a `consume` where
> it was and a `produce` where it is.**

**Sixteen of seventeen already obey that.** The exception is `move`, which crosses to `$to` with a
put - and it is the row Sean was looking at when he said a unit and a resource are not different.
**They are not.** Both are counted things changing place, so both are consumed at one end and
produced at the other:

| Recipe   | Owner  | Role    | Qty | Kind | Traits            | Where   |
| -------- | ------ | ------- | --- | ---- | ----------------- | ------- |
| **move** | player | consume | 1   | unit | moving at least 1 | `$from` |
|          |        | produce | 1   | unit | moving one less   | `$to`   |

**`put` then never names a place**, which is a check a tool can make.

## `M1` - consume at one end and produce at the other

| Recipe   | Owner  | Role    | Qty | Kind   | Traits            | Where   |
| -------- | ------ | ------- | --- | ------ | ----------------- | ------- |
| **move** | player | require | 1   | unit   | moving at least 1 | `$from` |
|          |        | put     |     | unit   | moving one less   | `$to`   |
|          |        | consume | 7   | metal  |                   | `$from` |
|          |        | produce | 7   | metal  |                   | `$to`   |
|          |        | consume | 1   | energy |                   | `$from` |

**The metal balances inside the rule and the energy does not, and the reader can see which is
which.** The cost of `M1` is that seven is a constant in a rule, and `spec/invariants.md` says *a
rule's amounts are constants* - so a haul of seven and a haul of three are two rules, or one rule
fired seven times and three times.

## `M2` - give `put` a quantity and a place

```
|          |        | put     | 7   | metal  |                   | `$to`   |
```

**One row instead of two**, and it reads as what it is: the same metal, elsewhere. **It costs the
sentence that says a put has no quantity**, and that sentence is what currently tells a reader that
`put` never creates anything. **Relaxing it is a change to the notation, not to this recipe.**

## `M3` - say nothing, because `P-509` already moved it

**Rows mention no metal at all.** `spec/logistics.md`, promoted: *a unit moving out of a place is
given an amount of each kind, no more than its own capacity for that kind, and that amount joins the
number the new place holds.* **The haul is a containment rule and the command's argument feeds it.**

**The cost is that the net cannot see it.** `reports/petri.md` draws what the rows say, so a haul
would be invisible to the drawing and to the weighting - **which is safe, because moving conserves,
and blind, because nothing would catch a haul that did not.**

## And three ways to say whose fuel it was

**`E1` - as now.** `consume 1 energy $from`. **Nothing says the vehicle paid**; a reader infers it
from the rule being `move`.

**`E2` - the consume names the payer.** The row points at the unit's row, the way `P-514` proposes a
quantity point at a row: `qty-line:` for a quantity, and something like `by-line:` for a cost.
**Explicit, and a new column for one use.**

**`E3` - the move is free and being ready costs.** `move` spends `moving one less` and consumes
nothing; `refresh` pays for putting it back:

| Recipe      | Owner | Role    | Qty | Kind   | Traits                | Where |
| ----------- | ----- | ------- | --- | ------ | --------------------- | ----- |
| **refresh** | world | require | 1   | unit   |                       |       |
|             |       | consume | 1   | energy |                       |       |
|             |       | put     |     | unit   | moving at its maximum |       |

**Then the energy is visibly what a vehicle burns to be able to move**, rather than a toll on the
move itself, and `move` becomes purely a change of place. **The cost is that it is paid at the
turn's end rather than when you move**, so a unit moves on credit and is charged later - and a unit
that moved into a place with no energy stops being ready rather than being refused.

## What this lane would pick

**`M1` and `E3`, and `M1` for a better reason than this lane first gave.** Not *`put` keeps identity
and a resource has none* - nothing in the game has an identity to keep. **`M1` because a change of
place is not a change of state**, and the net has two places to draw it between. `E3` because it
answers your question directly - **the loss stops needing an explanation once the thing being bought
is readiness rather than distance.**

**`M2` is the one to take if the seven matters more than the sentence**, and that is a judgement
about how often a haul will be a constant rather than a repeat.
*Nothing is open.*
