# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-572 - who owns the regression cases, and the answer is nobody except for one gesture that is yours

**to** sean · **status** open · **raised** 2026-09-26 · **asks** approval · **kind** entailed · **shape** text · **into** `CLAUDE.md` -> Perspectives, after the `reviewed/` paragraphs

**You asked who should own the regression scenario. Three things are being asked about and they
answer differently.**

```
scenario/main.4x           the code lane's - it is the source, and scenario/ is its column
the generator and check    the code lane's
scenario/regression/*.4x   no owner. CLAUDE.md: a generated file has no owner, nobody edits
                           it, and a hand edit is overwritten at the next commit
```

**So two of the three need nothing said. The third does, and not about editing.**

## The gap, which is deletion rather than editing

**Deleting one of those files is your approval**, and the generated-file rule does not cover it - it
says a hand **edit** is overwritten, and a deletion is not overwritten. **It is honoured**: the
generator writes a fresh case, which is the whole point.

**So a lane deleting a case is indistinguishable from you accepting a change you never saw.**
Nothing forbids it today.

**`reviewed/` has exactly this rule and the regression cases have none.** `CLAUDE.md`: *`reviewed/`
is the record of what Sean has read, and no instance creates or deletes one.* **Same gesture,
opposite meaning, and only one of the two is guarded.**

## One deletion that is not yours, so the rule has to say which

**`crates/game-model/tests/regression.rs` already tells the code lane to delete files** - *file(s)
in `scenario/regression/` are of commands the scenario no longer plays... delete them*. **That is
housekeeping and not acceptance**, and the two are told apart by whether the command is still in
`scenario/main.4x`, which that test already computes.

## The words

**Two paragraphs, into `CLAUDE.md` -> Perspectives, after the `reviewed/` paragraphs:**

> **A generated regression case is deleted by Sean, and that deletion is an approval.** The cases
> under `scenario/regression/` have no owner and nobody edits them - they are generated in full,
> and a hand edit is overwritten. **Deleting one is different**: it is not overwritten, it is
> honoured, and it says *I accept what it does now*. **So no instance deletes one while the
> command it covers is still played**, and a lane that thinks a case is wrong says so in an
> outbox rather than removing it.
>
> **A case the scenario no longer plays is housekeeping and any lane may remove it.** The two are
> told apart by `scenario/main.4x`: a case whose command is still there is waiting on Sean, and a
> case whose command is gone is waiting on nobody. **That is the same line `reviewed/` draws** -
> a record is the person's and an orphan is a mess - and it is drawn here because the gesture
> means the opposite in the two places.

## Why this one cannot be relayed

**`CLAUDE.md` says an approval for itself comes from you directly**, and this is what it says
about who may write what. **You asked the question in your own words, which is why it is being
put to you now** rather than filed and left.

### P-571 - the main scenario has generated detail under it, and nothing says it is not the specification

**to** sean · **status** open · **raised** 2026-09-26 · **answered** 2026-09-26, the layout · **asks** approval · **kind** entailed · **shape** text · **into** `spec/scenarios.md`

**You chose the code lane's layout**: *I actually do like the multiple file solution the code
lane did.* **34 files in `scenario/regression/`, one directory, one case per command** - which
also meets your rule that what you delete sits in a single directory.

**Nothing else about the layout needs saying and none of it is this lane's.** `scenario/` is the
code lane's column and the files are generated. **What is left is one sentence, and it is about
what the specification is.**

## Why a sentence, when the suite works without one

**`spec/scenarios.md` says the main scenario is *vetted by hand*, and now has 34 generated cases
under it that are not.** A reader of that file learns nothing about them, and the risk is the one
`CLAUDE.md` already names: *a check that pins the present state cannot report a gap against what
should be... it says nothing about what it ought to do.*

**The code lane put the distinction better than this lane had**, and it is the reason the sentence
is worth having rather than a tidy-up:

```
a specification test states a world so it can be read alone
a regression case states a delta so a diff is legible
```

**They are not interconvertible**, so nobody should later try to generate one from the other, and
nobody should read a regression case as intent. **That was found by measurement, not argument**:
the first shape stated the whole world per case, one density moved from six to seven, and all 34
files changed - where the took-and-made shape moves 13, and those 13 are the cases whose behaviour
actually differs.

## The words

**One bullet, into `spec/scenarios.md`, after the main-scenario bullet:**

> - **The main scenario has generated detail beneath it, and none of it is vetted by hand.** One
>   case per command, recording what that command took and what it made. **It says what the game
>   does; the tests I have read say what it must** - so a generated case is never a statement of
>   intent, however exactly it describes the behaviour.

## What this does not say, deliberately

**Not where the cases live, not how many, and not the deletion convention.** The first two are the
code lane's and would go stale; the third is already yours in `docs/process.md` - *Absent expected
data means I accept what it does now* - and `CLAUDE.md` says a fact already stated is not restated
in prose.
