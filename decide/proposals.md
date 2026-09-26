# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-568 - holding is done by containers, and a container that declares nothing is a defect

**to** sean · **status** open · **raised** 2026-09-26 · **rewritten** 2026-09-26, around the unification you approved · **asks** approval · **kind** recovered · **shape** text · **into** `spec/logistics.md`, replacing the *one of three things* bullet

**Your words, 2026-09-26**: *a territory has a bin that can hold a certain number of extractors; a
vehicle has a bin that can hold a certain amount of energy; a transport vehicle will have 2 bins,
one for energy it uses for fuel, and one for the thing it transports; things that don't have bins
are in disorder and clear at end of turn.* And: *I think it makes sense to unify
bins/capacity/storage.*

**The bug this began with**: a bin was built for metal with no row saying how much a bin holds, so
`discard-disorder` took all five metal with the bin standing there.

## Why the rule in the file could not catch it

**`spec/logistics.md` says a kind declares one of three things - no capacity, a limit, or no
limit - and your 2026-09-18 default says omitting a capacity means carrying none of that thing.**
Together, a kind that declares nothing is declaring the first of the three. **Every kind that
could exist satisfies the sentence**, so the bin was not breaking a rule; it was using a default.

## What the unification changes, and it is the second sentence that does the work

**Holding stops being something any kind may declare and becomes something a container does.**
Then a kind with no declaration is not *a thing that holds nothing* - it is **not a container**,
and a container with no declaration is a **defect**, which is the sentence that was missing.

## The words

**Replacing the *one of three things* bullet:**

> - **A thing that holds is a container, and a container is a thing.** A place has its deposits and
>   its bins; a vehicle has containers of its own, and a transport has two - one for the fuel it
>   burns and one for what it carries. **Nothing holds anything except by having a container for
>   it**, so a kind that is not a container holds nothing and never can.
> - **A container declares a limit, or declares no limit.** With a limit it holds up to that many
>   and may happen to be empty - so a thing holding nothing today is not thereby a thing that never
>   could. With no limit it holds any number, and there is no free capacity to record because
>   nothing can be short of it.
> - **A container that declares neither is a defect.** What holds nothing is a kind that is not a
>   container, and that is a different thing from a container that forgot to say.

## What this rests on that is already true

```
containers are already things        deposit holds 1 extractor, bin holds 10 resources
capacity is already per kind         spec/logistics.md, and a territory varies by how many
                                     deposits it has rather than by its own capacity row
disorder is already this rule        spec/resources.md: order is being in a container
the tree already isolates a vehicle  a transport's fuel container is in the transport, so it
                                     takes capacity there and never in the place
```

**That last line is a correction this lane owes you.** It told you a vehicle's container would
compete with the metal bins for slots in the place. **It would not** - containment is a tree and
what holds a thing is what says where it is. That was the only argument against unifying and it
was wrong.

## What it costs, stated so you can refuse it

**Vehicles gain container kinds they do not have today.** An ark holds its energy directly now;
afterwards it holds a container that holds the energy. **A fuel container cannot be a `bin`**,
because every bin holds ten and a fuel container holds one, and capacity is a fact about the kind.

**And the data must say which kinds are containers**, or *every container declares a capacity* is
unfalsifiable again. `{family}` and `{member kind:N family:M}` already exist and are where that
goes. **This proposal does not decide that** - it is a data change, not a rule, and it follows the
words rather than preceding them.

## What is not in this, and it is still open

**Whether *some number of extractors* may be zero** - `spec/planet.md`, the scenario's territory 1
having no energy density and nothing objecting. **It is a separate question and is filed as
`P-570`**, because this one asks approval and that one asks a decision.

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
