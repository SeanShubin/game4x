# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-558 - a test is stated in the friendly form, and the foundation form is a rendering of it

**to** sean · **status** open · **raised** 2026-09-25 · **asks** approval · **kind** a direction to reverse · **shape** text · **into** `spec/README.md` -> Rules for this directory, rule 3

**Both choices this asked are decided**, so it is a rewrite for you to read rather than the same
item with answers attached.

- **Sean, 2026-09-25**: *foundation lives in reports*
- **Sean, 2026-09-25**, on the translator: *go with your recommendation, move it*

## The words

**One block, into rule 3, after *Where prose and a test disagree*:**

> **A test is stated in the friendly form, and the foundation form is a rendering of it.** The
> rendering is generated from `reviewed/` and never from `spec/tests/`, so that what the engine runs
> is derived from what has been read rather than compared with it.

## Why those two sentences and no more

**Rule 3 already carries the rest.** *A test is the primary statement... read and approved one at a
time*, and **None of the three is decided in a discussion, in a note, or in a rendering of any of
them.** So the rendering being undecidable is already stated; **what is new is which of the two
forms is the statement, and what the rendering is generated from.**

**The second sentence is the one your constraint requires.** The tests are typed by an assistant on
your direction and are not canon until you mark them reviewed - so generating from `spec/tests/`
would put a test you have not read in front of the engine. **Generating from `reviewed/` makes
every step after your reading mechanical**, and it is why the direction is worth reversing at all:
today what runs is checked to match what you read, and under this it is derived from it.

**Where the generated file goes is not in these words.** It is a location rather than a rule, and it
belongs in the work order - which is the next step and not this one.

## What follows this, and it is not part of what you are approving

**A release capability for the code lane**, filed after promotion, carrying the three things it
has to do and the observable that says each is done: generate the foundation form into `reports/`
from `reviewed/`; move `crates/thin-engine/tests/common/friendly.rs` into production support, which
`examples/report.rs` already reaches by `#[path]`; and reverse `friendly.rs`'s comparison so it
asserts the generated form against the read one rather than the other way round.

**The translator is 844 lines, 561 of them code, with three imports and no test infrastructure
among them** - so moving it drags nothing with it. `every_file_survives_the_round_trip` already
asserts the identity in both directions.
