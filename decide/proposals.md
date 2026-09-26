# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-568 - the unification is already promoted, and one bullet of it cannot be broken

**to** sean · **status** open · **raised** 2026-09-26 · **rewritten** 2026-09-26, twice - the second time because the section already said it · **asks** approval · **kind** recovered · **shape** text · **into** `spec/logistics.md` -> Containment, replacing the *one of three things* bullet

**You asked whether to unify bins, capacity and storage. `spec/logistics.md` -> Containment
already does, and you promoted it.** This lane drafted the unification without re-reading the
section it would land in; the re-read before you approve is what caught it.

```
A bin is a thing. What holds a kind is a store for that kind, and a thing with two bins
holds two stores. A store's capacity is what its kind declares, and it holds nothing
```

**That is your transport with two bins, in the file, already normative.** And the model is more
unified than the draft was: **resources are not in containers at all.**

```
A resource in a place is in that place, not in a container inside it. What a place holds of
a kind is one number. The things in it that can hold that kind contribute capacity and hold
nothing
```

**The earlier draft said *nothing holds anything except by having a container for it*, which
contradicts that.** Stores do not hold; they contribute capacity, and the place holds one number.
**Withdrawn before you read it rather than after.**

## So the bug was the spec working, and one sentence short of catching itself

**A bin whose kind declares no capacity contributes nothing**, so the place's metal capacity
stayed zero and the metal was lost at the turn's end - *at the turn's end what the place holds
beyond that capacity is lost*. **Every step of that is a promoted rule.**

**What no rule says is that this is wrong.** *A store's capacity is what its kind declares* is
silent on a kind that declares nothing, and the *one of three things* bullet makes declaring
nothing a legal third option.

## The words

**Replacing the *one of three things* bullet:**

> - **A store declares a limit, or declares no limit.** With a limit it contributes that much
>   capacity, and a place may happen to be holding less - so a store standing in an empty place
>   is not thereby a store that never fills. With no limit it contributes without bound, and there
>   is no free capacity to record because nothing can be short of it.
> - **A kind that declares neither contributes nothing, and is therefore not a store.** That is
>   the right answer for an extractor, which stands in a deposit and holds nothing. **It is a
>   defect for a kind built to be a store**, which is a store that forgot to say - and the two are
>   not told apart by this sentence, but by the data saying which kinds are stores.

## What it does and does not buy

**It removes the third option**, which is what made the rule unbreakable: a kind declaring nothing
was declaring *no capacity*, so every kind satisfied the sentence.

**It does not make the check writable on its own**, and says so in its own last clause. **The data
must say which kinds are stores** - `{family}` and `{member kind:N family:M}` already exist and are
where that goes. **That is a data change and follows the words rather than preceding them.**

## What is no longer in this

**`P-570` is withdrawn unasked.** Whether a territory may omit a resource was decided by you on
2026-09-15 and is in a test you reviewed: *a row at quantity zero is never written - the model is
a minimal expression of intent.* **Omitting a resource means no deposit of it**, and building an
extractor for it is refused by that test.

**The two only looked alike.** A territory with no deposit is a coherent fact. **A store that
declares no capacity is a thing whose whole purpose is to hold, contributing nothing** - which
*minimal expression of intent* does not cover.

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
