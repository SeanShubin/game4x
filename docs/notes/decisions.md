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

### P-390 - A per-turn limit on a recipe, and whether readiness is what it compiles to

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, three times - now around Sean's per-application limit, which subsumes the naming question - **kind** entailed, from `S-93` built - **asks** a decision - **into** `spec/console.md` -> The language, and `releases/first-release.md` -> Recipes, Traits

***Allowance* is taken** and is the word below.

## The name you are looking for is two names

**The objective is minimum description length** - the total is the size of the rules plus the size of
the ground representation, and a change that shrinks one while growing the other has to be paid for
somewhere. **The theory of when it blows up is compilability**, and the research lens already has the
result that matters here: `X-11` cites Nebel's compilation schemes - *conditional effects cannot be
compiled away preserving linear plan size, and can be with polynomial growth.* **Your two things are
the two terms of one sum**, and the question is always which construct pays.

## What readiness costs, measured

**12 of 71 recipe rows** carry `ready`, `not ready`, `fertile` or `spent`, plus **2 rows in
*Traits***. **Four of the twelve are `refresh` and `renew`**, two world recipes that exist only to
refill. A per-turn limit on `move`, `create labor`, `work` and `bear` would replace all fourteen with
**four annotations**. **You are right about the size.**

## What it costs, and it is the thing you just bought

**A Petri net is memoryless.** Whether a transition may fire depends on the marking and on nothing
else. ***This recipe has already fired for this thing this turn* is history, not state** - so a
per-turn limit is not expressible as arcs, and the no-gain check cannot see it.

**The standard way to put a history constraint into a state machine is a token you spend.**
**Readiness is exactly that token.** So the two are not rival designs: **the limit is the source form
and readiness is the compiled form.**

**And the check breaks loudly rather than quietly if the token goes.** Without readiness,
`create labor` makes a labor out of nothing, `work` makes resources out of a labor, and no weighting
exists - `reports/nogain.md` would say so by name. **The invariant would stop being decided and go
back to being believed**, which is the one thing you built it to escape.

## So the third way is to have both, and the release already does this twice

**Declare the limit and ground it into readiness.** `spec/invariants.md` sanctions the pattern in as
many words: a rule *is written once and stands for as many rules as it has cases, **which whatever
reads it may spell out**.* A family grounds into its members; a density unfolds into its cases;
**a per-turn limit would ground into a spent token, and that is a third instance rather than a new
idea.**

- **The source shrinks** - four annotations instead of fourteen rows, and `refresh` and `renew` stop
  being written by hand
- **The ground form keeps the token**, so the net stays ordinary, `time` stays a source, and
  `P-388` is untouched
- **What you read is your choice** - `reports/recipes.md` already shows ground rules beside declared
  ones

## The one place the two forms disagree, which is the real question

**A token is shared across every recipe that wants that thing's action; a per-recipe limit is not.**

- **With readiness**, a citizen does **one thing** per turn. If a second recipe ever wants a
  citizen's action, it competes with `create labor` for the same token
- **With per-recipe limits**, a citizen may do **each thing** once per turn. Two recipes wanting a
  citizen do not compete

**Today they cannot be told apart**, and this lane checked rather than assumed: `move` takes a unit's
readiness, `create labor` a citizen's, `work` an extractor's - **no two recipes draw on the same
thing's readiness.** They agree by accident, which is the shape you have already written about soft
lines: *those differ wherever a capacity is more than one, and agree only by accident where it is
one.*

**So the question is not which notation.** It is **whether a thing acts once a turn, or once per
kind of action.** The notation follows from the answer, and either notation can express either
meaning once you have said which it is.

## What this lane would do, marked as preference

**Declare the limit, ground it into a token, and say which meaning the limit carries.** It gets the
smaller source you want, keeps the invariant decided rather than believed, and forces the sentence
that is missing either way - **a citizen acts once a turn** or **a citizen does each thing once a
turn** - which no file says today and which the release has never had to answer.
