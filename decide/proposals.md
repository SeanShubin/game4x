# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-558 - which test form is canonical, and where the other one is generated

**to** sean · **status** open · **raised** 2026-09-25 · **asks** a decision · **kind** a direction to reverse · **shape** an instruction · **into** `spec/tests/`, `reviewed/` and the translator

**The same test, in the two forms, both of which exist today:**

```
friendly     {place id:1 of:territory-1 layer:surface name:place-1}
             {pioneer where:place-1 moving:1} -> 1

foundation   {place id:1 of:1 layer:surface}
             {pioneer where:1 moving:1 quantity:1}
```

**Today foundation is the source and friendly is rendered from it.** `crates/thin-engine/tests/
friendly.rs` renders and compares line for line, so the two cannot silently disagree - **but what
you approve is a rendering of what runs.**

## The decision

**Reverse it: friendly becomes the source, foundation becomes generated.** Then the file you
maintain, the file you read, and the file that is canonical are one file, and foundation is a view
you can read when debugging - generated, never edited, regenerable at any commit.

**Your constraint decides where it is generated from.** *The tests will usually be typed by an AI
assistant on my direction, but as they represent my unambiguous will they are not canon until I mark
them as reviewed.* So **foundation is generated from `reviewed/` and never from `spec/tests/`** -
otherwise a test an assistant typed and you have not read reaches the engine.

**That makes the authority chain mechanical after your reading and not before it**: an assistant
types friendly, you mark it reviewed, foundation is derived, the engine runs it. **Today what runs
is checked to match what you read; under this it is derived from what you read**, which is the
stronger of the two.

## One thing this does not decide, and it is why it asks a decision

- ~~**Where generated foundation lives.**~~ **Decided, Sean, 2026-09-25: *foundation lives in
  reports*.** So it is a generated view with no owner, never edited, alongside every other report
- **Whether the translator moves**, which is the only thing left open. It is 843 lines in `crates/thin-engine/tests/common/
  friendly.rs`. Moving it makes it production support, which is the code lane's and gated by the
  full suite; leaving it in `tests/` and having the build call it keeps the ownership where you put
  it - **Sean, 2026-09-15**: *the translator is not part of the engine*

## What the translator is, read rather than sized

**844 lines, of which 561 are code and 243 are prose** - 29% of the file explains itself. **Three
imports and not one of them is test infrastructure**: `BTreeMap`, `thin_engine::notation` and
`thin_engine::schema`. **It is a pure library over the engine's own types**, and it sits in `tests/`
by a decision about what it is for rather than by any dependency.

**Thirteen public items doing three jobs.** Translating - `fold`, `Names::of`, `name`, `row`,
`parse`, `foundation`, `all`. Classifying - `in_a_section`, `states_a_world`, `shown`. Diffing -
`Change` and `compared`, *what moved between two worlds*.

**The two forms differ by exactly two things**, from your own header: every row has a `name`,
generated as `<relation>-<id>` where the relation has none, and every reference is written as the
name rather than the id. **Nothing else.** And keeping the `id` is what makes it exact - an earlier
version dropped it where a row had a name and had to mint one coming back.

**Six tests cover it and one of them is the identity**:
`every_file_survives_the_round_trip`, plus `every_row_keeps_its_id`.

## Two things that bear on the one open choice

**Your reason for it living in `tests/` has changed meaning.** 2026-09-15: *I don't consider the
translation part of the engine. The engine should only know about the foundational format. **The
user friendly format is for the test harness and debugging.*** **The second sentence still holds
and is satisfied either way** - the translator is outside the engine in both options. **The third
has inverted**: friendly is now what you review and foundation is the debugging view.

**And it is already production support in fact.** `examples/report.rs` writes the report you read,
and reaches the translator with `#[path = "../tests/common/friendly.rs"]`. `examples/review-web.rs`
- *let Sean say what he thinks of a test without leaving it* - is the review application, which
`CLAUDE.md` makes the code lane's and puts beyond every lane it judges. **So the translator is a
dependency of the review application, reached by a path attribute out of a test directory.**

## What it costs, measured rather than estimated

**No new machinery.** The translator already goes both ways: `Names::row` is foundation to friendly,
and `Names::parse` and `Names::foundation` are friendly to foundation, called today from
`tests/directories.rs`, `tests/friendly.rs` and `examples/report.rs`. **`foundation()` refuses
rather than guessing** - `{territory id:1 name:home}` is an error, because `territory` declares no
`name` and *silently dropping it would lose an author's work in the format they author in*.

**This lane said the cost was an added build step and that was wrong.** There is nothing to build
that is not built. The remaining cost is ownership of one 843-line file, which is the second choice
above.


