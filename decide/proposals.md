# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-605 - A record holds a verdict and the behaviour it is about, and presence stops meaning two things

**to** sean · **status** open · **raised** 2026-10-01 · **asks** approval · **kind** recovered · **shape** text · **into** `spec/README.md` -> rule 3

**Two blocks.** The first replaces the sentence that says a record is a copy of a test; the second
follows it.

**Replacing *A test is written in `spec/tests/`...* through *...which is what makes generating from
`reviewed/` mean something*:**

>    **A test is written in `spec/tests/`, and `reviewed/` holds what I thought of it.** A record
>    names its verdict and carries the behaviour that verdict is about - the rows, canonical, without
>    the prose. **No record means I have not looked**; a record saying `approved` means the code is
>    bound by it; a record saying `denied` means it is not, and that I owe the specification a
>    statement of what I want instead.
>
>    **The review application writes a record and removes one, acting as me; nothing else puts a file
>    there.** A denied test stays in `spec/tests/` - the verdict is a fact about my response, not
>    about where the test lives.

**Added after it:**

>    **What the engine runs is what the verdicts approve.** The foundation form is generated from the
>    approved records, so a test I have not read constrains nothing and a test I have denied
>    constrains nothing either. **Presence used to mean both *I read this* and *this binds*, and
>    those are now two different facts.**

## The shape, so you can see what a record becomes

```
spec/tests/rule/a-yard-is-built-from-labor-and-metal.4x    the test, with its header prose
reviewed/rule/a-yard-is-built-from-labor-and-metal.4x      {verdict state:approved}
                                                           {test name:a-yard-is-built-...}
                                                           {given} ... {when} ... {then}
```

**The record shrinks by about 45%**, which is how much of the 57 tests is comment lines - 790 to 960
rows. **And what is compared becomes what was approved**, which is `P-600` said about a file rather
than about a rule.

## What this costs the code lane, stated because it is not small

```
tools/outbox/src/lib.rs:1353   compares whole files - `read != said`
tests/generated.rs:59          runs every file in `reviewed/rule/`
examples/foundation.rs         renders from every file in `reviewed/rule/`
examples/review-web.rs         writes a byte copy
```

**All four read the directory and none reads a verdict.** The suite running a denied test is the
failure this most needs to avoid, and it is the one that would look like nothing was wrong.

## What is unaffected

**`reviewed/asked.md`** - the note saying what you have asked to be changed, per test, which
`report.rs` reads. **A denial says *not this* and that note says *this instead***, so they are
different facts and nothing here merges them.

