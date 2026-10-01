# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-607 - A verdict on a case is a reminder, and authorizing a regeneration is a separate gesture

**to** sean · **status** open · **raised** 2026-10-01 · **asks** approval · **kind** recovered · **shape** text · **into** `docs/process.md` -> How I know the application is right · **source** him, answering whether a denied case is also a request to regenerate

**Your answers, and they separate two things that are one thing today.** Deleting a case currently
says *I accept what it does now* **and** causes it to be rewritten - one gesture doing a judgement and
an action.

**Offered as a block after *Deleting is the only thing I do to one*:**

> **A verdict on a regression case is a reminder to myself, and nothing is bound by it.** A case
> observes rather than decides, so denying one does not release the code from anything - it says *look
> at this* and keeps saying it until I have. **A case nobody has marked is a case nobody has looked
> at**, which is the same as for a test.
>
> **Authorizing a regeneration is a separate gesture from a verdict.** A denial says the behaviour a
> case now shows is wrong; an authorization says it is right and the stored expectation should be
> rewritten to match. **Both can be true of different cases at once**, which is why they are two
> flags and not three states of one.

## What this makes of the deletion

**Deleting a case is the authorization, expressed by hand.** The suite writes an absent case, so
removing one and running is *accept and rewrite* in a single motion - which is why the two have never
needed telling apart before.

**They need telling apart now** because the first can be done from a page and the second cannot.
Marking a case is a row in a file; rewriting a case means running the generator.

## The trigger you were unsure about, which is most of the way there

```
.github/workflows/review.yml   has workflow_dispatch, checks out, installs stable,
                               and runs cargo - line 43, 62, 64, 66
                               and gates on github.actor == github.repository_owner
the suite                      writes any absent case - regression.rs's Err(_) branch
```

**So a workflow you dispatch can delete the authorized cases, run the suite, and commit what it
wrote.** The owner gate is what makes that you rather than a lane, which is the property the record
rule turns on and the one thing that must not be relaxed to make this convenient.

## What is not offered

**Where a case's verdict lives.** `reviewed/` holds only `rule/`, and a case is not a test - it has no
behaviour you approved, only an observation you have or have not looked at. **A proposal for the
layout follows this one**, and it was waiting on exactly the answer you have just given.
### P-606 - Column order is not behaviour, one function normalizes, and that decides the order for us

**to** sean · **status** open · **raised** 2026-10-01 · **answered** 2026-10-01, twice: column order is not significant, and the comparison delegates to a normalize function · **asks** approval · **kind** recovered · **shape** text · **into** `spec/README.md` -> rule 3

**Your two answers together settle the order, which neither of them does alone.** This replaces the
ambiguous phrase in `P-600`.

**Replacing *entries coalesced to one per description, traits and entries in the order this
specification already gives them, whitespace not significant because the braces say where a row
begins and ends*:**

>    entries coalesced to one per description, **and the columns in a row normalized**, because the
>    order they are written in is not significant; whitespace not significant either, because the
>    braces say where a row begins and ends. **One function does that normalizing and everything
>    that compares delegates to it** - so no comparison can disagree with another about whether two
>    tests say the same thing.
>
>    **The order that function puts columns in must not depend on anything editable.** `id` first,
>    then every other trait alphabetically, then `occupied`, `free` and `capacity` last - **the order
>    this specification already states for an entry**, which depends on names. **An order taken from
>    the schema's `seq:` would mean renumbering those cleared every approval I have given**, and
>    renumbering is a tidy-up nobody thinks twice about.

## Why your two answers together pick the order

**Alone, *order is not significant* permits any deterministic order.** Alone, *one normalize function
that comparison delegates to* permits any order too.

**Together they do not**, because the record is that function's output - `P-605`: *the rows,
canonical*. So the record's order **is** normalize's order, and if normalize derives it from `seq:`
then renumbering `seq:` changes every record and clears every verdict. **A name-derived order cannot
do that**, and `spec/console.md` already states one.

## What it costs, and your own ruling covers it

```
records whose rows would be reordered    57 of 57
rows not in alphabetical trait order     449, with 51 alphabetical by coincidence
```

**Every record changes, and that is not a change of approval by your own words.** *The order of the
columns is not significant, so this should not make tests different* - so rewriting a record into a
different column order does not change what you approved, exactly as moving one did not. **A lane may
convert them**, on the same ruling that let a lane move them.

**And the code lane's cost stands**: the canonical ordering lives in
`crates/game-console/src/containment.rs`, which `crates/game-model` cannot reach because the
dependency runs the other way, so it moves to `friendly-notation` - a refactor of a 712-line module.
**That is the price of the trap not being there.**

