# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-569 - `D-5` asks the main scenario for a rule `spec/scenarios.md` says belongs elsewhere

**to** sean · **status** open · **raised** 2026-09-26 · **asks** approval · **kind** entailed · **shape** text · **into** `releases/rules-become-data.md` -> `D-5`

**`D-5` says *every rule the reviewed tests describe fires at least once while it runs*, and
fourteen of fifteen do.** The one that does not is `perish`, because nobody starves.

**`spec/scenarios.md` says that is right rather than short**: *there is one main scenario, and it
touches everything **a typical game uses***, and *a mechanic that only appears in an unusual
situation belongs in a scenario of its own*. **Starving is the unusual situation.**

**The spec wins and the release gives way** - `releases/README.md`: *the spec is the destination
and always wins*, and *a release never invents a rule*. **So this is a correction rather than a
choice**, and what you are approving is the wording.

## Where the clause came from, so it is clear nothing is being quietly dropped

**It is inherited from `R-6`**, which said *every recipe in the release fires at least once while
it runs*. That was written when the release's tables were the ruleset and there was no
`spec/scenarios.md` sentence to disagree with it. **`P-566` carried it forward without re-reading
it against the spec**, which this lane should have done then.

## The words

**Replacing `D-5`'s *vetted when* line:**

> - **Vetted when** - a main scenario exists over the reviewed ruleset and I have watched it run:
>   an Ark deploys, **that first territory is developed**, a second is taken by land and
>   **developed too**, and an Ark launches from the second. **Every rule a typical game uses fires
>   at least once while it runs**, measured by what fired rather than by what the file says, and
>   **a rule that does not fire is named with the unusual situation it needs** - so an omission is
>   something I can read rather than something I have to notice. This is the observation `R-6` was
>   retired without making

## What that changes for the check

**It stays countable.** Today: fifteen rules, fourteen fired, one named - `perish`, which needs a
starvation. **Silence is what the old clause allowed and this does not**: a rule that stops firing
for a reason nobody wrote down is now a failure, where before it was fourteen out of fifteen and
no signal.

## What you would be rejecting

**That the main scenario starves somebody on purpose** so the count reaches fifteen. It is the
other way to satisfy the old clause, and `spec/scenarios.md` argues against it.
