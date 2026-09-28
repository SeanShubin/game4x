# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-581 - A regression case is read and never authored, and behaviour changes through a unit test

**to** sean · **status** open · **raised** 2026-09-27 · **asks** approval · **kind** recovered · **shape** text · **into** `docs/process.md` -> How I know the application is right

**Your own words, cut to what is not already in a file.** Three things: which of the two kinds of
test decides anything, what a confusing case is for, and that deleting is your only gesture on one.
**`spec/README.md` rule 3 and `docs/process.md` each say half of the first and neither says which
wins**, and the other two are nowhere.

**Offered as a block after the three paragraphs `P-580` added**, which end *the day a local command
has a case is the day the game reads the interface*:

> **A unit test decides and a regression case observes.** I change what the game does by telling a
> lane to write a unit test, which I read one at a time, and I add one when we find behaviour the
> specification does not state and a unit test could. **A regression case is never where a behaviour
> is decided** - not one of the four suites, and not the one that holds a case per rule.
>
> **So a case that confuses me is a question, not a defect I fix.** I say so to the specification and
> the code lanes, and one of two things comes back: it is confusing because it is wrong, or it is
> right and I understand it once it is explained. **Either way the correction lands in `spec/` and in
> a unit test**, and the case moves afterwards because what it observes moved.
>
> **I never modify a regression case.** I ask a lane to change it - which means changing whatever
> generates it, since a hand edit is overwritten - then I read the result, and I delete it if I
> approve. **Deleting is the only thing I do to one.** And I may ask a lane to add cases so that a
> new feature is covered, which is asking for coverage rather than for a behaviour.

## What this settles that was open

**`regression/rules/` is not the rules.** Fifteen cases, one per rule, landed today - and a reader
could take that suite for where a rule is stated. **This says it is not**, and the file it lands in
is the one that already says a unit test read one at a time is the primary statement.

**And it answers a question this lane was about to put to you.** `C-160` asks whether `regression/`
becomes your column the way `reviewed/` is. **Your third paragraph decides it**: lanes write cases
and you delete them, so a column that refused a lane's commit would refuse the half you asked for.
That proposal is now `P-582` and asks only where the remaining distinction lives.

