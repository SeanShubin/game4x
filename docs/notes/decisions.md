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

### P-422 - `R-6` asks how much of the planet has to be played, and has pointed at an empty queue since 2026-09-05

**to** sean - **status** open - **raised** 2026-09-11 - **kind** recovered, from `R-6`'s own note and the code lane's `C-95` - **asks** a decision - **into** `releases/first-release.md` -> `R-6`

**`R-6` has carried the question in prose and never as an item.** Its third bullet reads *what is
now in question is not whether it can be played but how much of it has to be - see the proposal
queue*, and **the queue has never held it**: 0 hits for that question across `proposals.md` and
this file, searched for by concept as well as by wording, against a population of **418 rows in
the Accepted ledger** plus every open, rejected and withdrawn item. So the pointer has been
dangling for six days. It now points here.

**What made it urgent is that the answer stopped being hypothetical.** `C-95` measured the
committed scenario on 2026-09-11: **twelve claimable territories, two founded, none at maximum
output**, `is_fully_exploited` and `has_won` both false. It launches an Ark at line 164 of 133
commands, so the *vetted when*'s second half holds and its first does not. `tests/fully_exploited.rs`
derives **57 buildings**, which is 114 commands and **counts nothing but the buildings**. **This is
not a near miss**, and `R-6` cannot be vetted as it stands.


**Three ways out, and the choice is which you want to look at.**

1. **Commit the full scenario.** `play.4x` grows by **at least** the 114 commands the bill names -
   that figure counts labor-and-build pairs only, and founding the other ten territories, moving
   pioneers to them and ending the turns are all on top - and `R-6` is then vetted by reading a
   scenario nobody will read line by line
2. **Play a smaller planet.** A scenario on fewer territories finishes, and *fully exploited* is
   demonstrated on something a person can hold in their head - at the cost of the vetted planet not
   being the one the game ships
3. **Reword the capability.** `R-6` becomes what the current scenario already shows, and *reaching
   a fully exploited planet* moves to a later release

**This lane has no recommendation**, because the three differ in what you would be looking at when
you vet it, and that is the whole of the question. **What it can say** is that 1 and 2 both keep
`docs/process.md`'s *the definitions and the commands are enough to derive the data dump by hand*
literally true of a scenario you could work through, and 3 does not.

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

**Option 2 is the one that reads as your stated criteria**, which is a reason to look at it first
and not a reason to take it. It keeps the building block, bounds it, and makes the bound mechanical
- *limited enough to manage complexity, able to maintain the invariants, flexible enough to design
the rest*. **This lane is not choosing it for you**, because dropping a construct you have no use
for is also a way to manage complexity, and which of those you prefer is the whole question.

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

**Three ways, and no recommendation.**

1. **Rewrite `age` as `require`/`put`**, and the release has one idiom for *the same thing, changed*
2. **Leave it**, and `consume`/`produce` stays the idiom wherever identity does not matter - which
   is a real distinction and could be stated rather than implied
3. **Say which rule decides it**, so the next recipe does not have to be asked one at a time
