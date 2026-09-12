# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-423 - `limit` is safe exactly where what it tests is bounded, and nothing says which kinds are

**to** sean - **status** open - **raised** 2026-09-11 - **kind** entailed, from the research lens's `X-9` and the code lane's `C-75` - **asks** a decision - **into** `releases/first-release.md` -> Recipes, and possibly `spec/invariants.md` -> Nothing comes back round with more

**This is the half of `P-421` your answer did not settle**, re-asked against what you said the net
is for. **You said the end is that a user can edit recipes without worrying about infinite resources
on one turn.** `X-9` is the finding that speaks to exactly that, and it has been open since
2026-09-08.

**`limit 0` is a zero test, and a zero test is an inhibitor arc.** Petri nets with inhibitor arcs
are Turing-complete; two of them model a two-counter machine. **What that costs is the thing you
want**: with a plain net, *an unintended infinite-resource loop is computable rather than something
you playtest for*. Crossing that line gives it away silently.

**But the line is not `limit` and `X-9` corrected itself on this.** Its first draft said moving the
test from a precondition to a guard saves it, and that was wrong. **The real line is boundedness**:
an unbounded place cannot be zero-tested safely, and a **bounded** one can, by the standard
complementary-place construction, with no inhibitor arc and no loss. So `limit 0 garrison` was
always safe - a garrison's capacity is 1 - and `limit 0 food` would be the cliff.

**`C-75` measured what adopting that rule would cost, and the answer was nothing.** *What bounds a
kind in a territory* already splits the eleven kinds:

- **Bounded by a stated capacity**, where a zero test is free: garrison, extractor, yard, ark,
  pioneer
- **Bounded by something else**, where it is the cliff: citizen, store, labor, food, metal, energy

**The six are exactly the ones a resource game invites a zero test on** - *if there is no food*, *if
the store is empty*. So the rule is free now and is not free later.

**Two numbers in `C-75` have gone stale since it was filed and the conclusion is stronger, not
weaker.** It counted *two zero tests, both `limit 0 garrison`*; `P-385` deleted both in `795f053`,
so there are now **zero**. Adopting the rule today is vacuous, which means it costs nothing and
proves nothing - it is a guard against a row nobody has written yet.

**So the question is which of three, and this lane has no recommendation.**

1. **Drop `limit` from the declaring sentence.** The language loses the one construct that can cost
   you decidability, and a future rule that needs a maximum is a question you answer then
2. **Keep `limit`, and declare the constraint**: a `limit` row may name only a kind bounded by a
   stated capacity. **That is a property a check can enforce over the whole recipe set**, which is
   what makes it worth having rather than believing
3. **Keep `limit` unconstrained**, and accept that the first `limit 0 food` moves the rule editor
   into a class where your invariant is no longer decidable, with nothing saying so

**Option 2 is the one that reads as your stated criteria.** It keeps the building block, bounds it,
and makes the bound mechanical - *limited enough to manage complexity, able to maintain the
invariants, flexible enough to design the rest*.

## `P-428` bears on this, and it points the opposite way from the obvious reading

**Added 2026-09-11, after you stated the rule.** *Try for unification first and look for a reason
not to; where there is not much difference, the unified form wins.* **The obvious application says
drop `limit` - fewer constructs is simpler.** That reading is wrong, and `X-11` is why.

**`limit` is not a fifth thing beside the others. It is a cell of a grid the four roles already
form** - a **change** and a **threshold**:

| Row                 | Change | Threshold     |
| ------------------- | ------ | ------------- |
| `consume 3 food`    | −3     | at least 3    |
| `require 3 workers` | 0      | at least 3    |
| `produce 1 metal`   | +1     | none          |
| `limit 0 garrison`  | 0      | **at most 0** |

**So dropping `limit` is the exception and keeping it is the uniform form**: the language would
otherwise offer every threshold direction except *at most*, for no reason a reader could state.
**Under `P-428` that is the default, and option 3 - keep it unconstrained - is refused for a good
reason rather than a preference**, because it is the one that costs the invariant.

**Which leaves option 2 as what your two rules jointly say**, and this lane is now saying so rather
than presenting a close call as open - which is `P-428`'s other half. **What it still cannot decide
is whether you want the constraint written as a rule in `spec/invariants.md` or as a line in the
release**, and that is a real choice this item does not resolve.


### P-424 - `age` destroys and recreates a thing, which is what `put` was introduced to stop

**to** sean - **status** open - **raised** 2026-09-11 - **kind** entailed, from your answer on `P-421` - **asks** a decision - **into** `releases/first-release.md` -> Recipes

**You said `put` was to simplify the destroy-then-recreate mechanics of `move`.** `age` still does
it, in the release, for the same operation:

```
| **age** | world | consume | 1 | thing | keeps at least 1 |
|         |       | produce | 1 | thing | keeps one less   |
```

**That is the readiness pattern written the other way.** `work` is `require extractor working at
least 1` then `put extractor working one less`; `age` is `consume thing keeps at least 1` then
`produce thing keeps one less`. **Same shape, same intent, two idioms** - and the release now says
one operation two ways, which is the unification you said you wanted less of, not more.

**It works today because nothing that ages has an identity.** `age` names the family `thing` and
what actually ages is food, which is counted rather than identified. **So this is not a bug and
nothing is broken** - it is the question of whether `age` should be rewritten as `require` and
`put` now that the construct exists.

**One thing that would change if it were.** Under `consume`/`produce`, a thing that ages is
destroyed and a new one made, so any rule keyed to creation sees an event each turn for every
perishable thing. Under `require`/`put` it does not. **Nothing in the release is keyed to creation
today**, counted over the 21 recipes - so the rewrite is invisible now and would stop being
invisible the moment such a rule is written.

**Three ways.**

1. **Rewrite `age` as `require`/`put`**, and the release has one idiom for *the same thing, changed*
2. **Leave it**, and `consume`/`produce` stays the idiom wherever identity does not matter - which
   is a real distinction and could be stated rather than implied
3. **Say which rule decides it**, so the next recipe does not have to be asked one at a time

## `P-428` answers this one outright

**Added 2026-09-11, after you stated the rule.** This is the case the rule was made for: **one
operation written two ways, with no stated reason for the difference.** *Try for unification first
and look for a reason not to* selects **way 1**, and the reason not to would have to be that
identity matters somewhere - which is exactly what way 2 would have to state and never has.

**And there is no cost to look for.** Nothing in the release is keyed to creation, counted over the
21 recipes, so the rewrite is invisible today. **`spoil` is untouched and stays a `consume`**,
because it genuinely destroys - so the two idioms end up meaning two different things rather than
one thing twice, which is the unification rather than a casualty of it.

**So this lane's answer is way 1**, offered as a reading of your rule rather than as a preference.
**What would change it** is a reason to keep `consume`/`produce` for fungible things - and if you
have one, it is way 2 and wants stating, because nothing states it now.


