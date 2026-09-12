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


