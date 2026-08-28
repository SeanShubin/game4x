# CLAUDE.md

Conventions for AI assistants working in this repository. Read this before editing anything.

## Start here

Sean designs; Claude drafts, organises and checks. New content reaches the specification only
by **promotion** - the protocol below - and never by Claude deciding something is settled.

To pick up where the last session left off, read in this order:

1. [`spec/README.md`](spec/README.md) - what the game is, and the rules for that directory
2. [`docs/notes/proposals.md`](docs/notes/proposals.md) - the live queue: what is proposed,
   what has landed, what was withdrawn and why
3. [`docs/notes/spec-backlog.md`](docs/notes/spec-backlog.md) - things Sean has said but not
   yet written

## Authority

Four tiers, and the boundary is the directory it lives in.

| Path          | Author     | What it is                                                                          |
| ------------- | ---------- | ----------------------------------------------------------------------------------- |
| `spec/`       | **Sean**   | Normative, about the end state. Every idea is his; Claude writes only by promotion. |
| `releases/`   | **Sean**   | Normative for one delivery. Same rules as `spec/`. Temporary; deleted once vetted.  |
| `docs/`       | shared     | The "why" layer. Reviewed. Follow the rules in [docs/README.md](docs/README.md).    |
| `docs/notes/` | **Claude** | Derived records of conversation. Not binding. Sean is not expected to read them.    |

## Two instances

Two Claude instances work in this repository at once. **The boundary is the kind of work, not
the directory.**

| Lane                                | Owns                                                                        |
| ----------------------------------- | --------------------------------------------------------------------------- |
| **Documentation and specification** | `spec/`, `releases/`, `docs/`, `README.md`, this file                       |
| **Code and deployment**             | `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`, `hooks/`, CI, cargo |

Neither crosses. The documentation instance does not edit code, **even to fix an obvious
break** - it reports the break and leaves it. The code instance does not write specification.

Three consequences, all of which have teeth:

- **Stage by name, never `git add -A`.** The other instance's work is often uncommitted in the
  same tree. `-A` sweeps it into your commit, and the two lanes then share a history entry that
  describes only half of it.
- **`hooks/pre-push` runs the full gate** - `cargo fmt`, clippy and the test suite across every
  crate. A documentation-only push is therefore gated on code this lane did not write and must
  not repair. If it fails for that reason, **say so and stop**; whether to `--no-verify` is
  Sean's call, not Claude's.
- **Re-read before asserting.** A file read earlier in the session may have been rewritten by
  the other instance since. Anything claimed about code needs a fresh look, not a memory.

Historical exception, so the boundary is not mistaken for a description of the past:
`tools/pad-tables/` and the tests in `crates/sphere-tessellation/` were written from the
documentation lane before the split existed.

## The rule that matters

**A decision is only real when it appears in `spec/`, reviewed by Sean.**

Anything in `docs/notes/` is a proposal, however confident it sounds and however much
measurement backs it. Analysis does not become policy by accumulation.

- The spec **states**. Terse, present tense, normative.
- The notes **justify**. Long, dated, measured.
- The spec links down to a note for reasoning; the note links up to the spec for the
  decision. Never the reverse. A note must never read as though it settled something.

## An empty queue means no contradictions

**If [the queue](docs/notes/proposals.md) is empty, Sean can take it that nothing known is
inconsistent.** That is a promise about the queue, not about the specification: it does not say the
spec is complete or correct, only that **everything Claude knows to be wrong is sitting where he
will see it.**

So a contradiction has exactly one resting place. When Claude finds one - between two spec files,
between the spec and a release, between a note and either - it becomes a proposal **the moment it is
found**, whatever else is happening. Never a paragraph in a discussion, never a sentence in a reply,
never a line in a note.

This is what makes an empty queue mean something. If contradictions can sit in prose, an empty queue
means only that nobody has written anything down lately.

## Editing `spec/` and `releases/`

**The boundary is the idea, not the words.** Sean owns every idea in these directories.
Claude owns how clearly they are expressed and where they sit.

| What                                                            | Who                                           |
| --------------------------------------------------------------- | --------------------------------------------- |
| Typos, whitespace, table alignment, links broken by a file move | Claude, no report needed                      |
| Restating an idea already present, in clearer words             | Claude, **reporting every line touched**      |
| Moving an idea to a better file, section or order               | Claude, **reporting every line touched**      |
| **Introducing an idea that is not already there**               | **Sean's alone**, and it arrives by promotion |

None of these need asking first. The reporting split is deliberate: typos and whitespace
cannot change meaning, so a diff is enough. Rephrasing and reorganizing can, so they are
reported.

**Reorganizing is not automatically substance-neutral.** It is the operation most likely to
change meaning while looking like tidying:

- Moving a bullet under a different heading changes what it is scoped to
- Merging two bullets asserts a relationship neither one stated
- Reordering a list can imply a sequence or priority that was not there
- Splitting a sentence can drop a qualifier that was doing work

Where a move would change what a line claims, do not make it - raise it instead. The report
is what lets Sean catch the cases where Claude thought it was tidying and was in fact
deciding.

A new idea reaches the spec one way, whatever its origin - Claude's proposal, Sean's own
thought, or something that fell out of a conversation. That way is promotion.

## Promotion

New content reaches `spec/` by promotion, and by nothing else.

| Step                                                                        | Who    |
| --------------------------------------------------------------------------- | ------ |
| File the proposal, naming the destination file and section                  | Claude |
| Edit the proposal text in place until satisfied                             | Sean   |
| Fix typos, grammar and wrapping **in the proposal**, reporting every change | Claude |
| Say "promote P-n"                                                           | Sean   |
| Copy the text **verbatim** into the destination, then assert it is there    | Claude |

**Promotion is a pure move.** The only things Claude may change while promoting are line
wrapping, bullet-versus-paragraph, and heading level - because those depend on the
destination file and cannot be settled in the proposal. Nothing else, ever.

If Claude wants a word changed, it changes the **proposal** and says so, before Sean
approves. Fixing phrasing during promotion would mean the text Sean reviewed and the text in
the spec are not the same text, and he would have no reason to re-read the spec to find out.
The guarantee this buys is that **approved text is byte-identical to shipped text.**

**A promotion that makes something else stale is not finished.** When landing a proposal
invalidates a line elsewhere - in `spec/`, in a release, or in another proposal - Claude does one of
two things and never a third:

- **Refuse the promotion**, saying what has to be decided first, or
- **File a cleanup proposal immediately after**, so the staleness sits in the queue.

Noting it in a discussion paragraph and moving on is the failure. It reads like diligence and
behaves like forgetting. **The queue is what gets read; prose is not.** P-85 found six
contradictions in one release file, and two of them had been flagged at the time and left there -
long enough for the coding instance to implement a rule the specification does not have.

**Every promotion asserts.** Copy, then verify the text is present in the target file, and
fail loudly if it is not. Claude has three times reported that something landed when it had
not - thirteen missing rows in the proposals file, three files missing from the spec index -
and every one was a report of intent rather than of fact. An operation that cannot fail
loudly will fail quietly.

**Accepting a proposal is not permission to write it.** Claude's job ends at proposing. If
Sean explicitly asks Claude to enter an accepted line, that is a separate instruction and
Claude may - but it is never the default, and never inferred from the word "accepted".

Proposals go in [`docs/notes/proposals.md`](docs/notes/proposals.md), numbered, each labelled
with what kind of inference produced it. Record every rejection with Sean's reason, or the
same proposal will be filed again in a later session.

Never silently rewrite his prose. "I changed lines 40 and 57" costs him a three-line diff; an
unannounced rewrite costs him a re-read of the whole file, and after that he cannot trust the
file.

Keep the open-proposal queue under fifteen. Past that, reviewing costs as much as writing and
the mechanism has failed. If a lot of proposals would be guesses at design, ask one question
instead.

A consequence of something already written belongs in the spec only when **another rule leans
on it**. Everything is derivable from something; derivability is not the test.

Two failure modes Claude has actually shown, both worth checking before filing:

- **Measuring something is not a reason to specify it.** A number that took work to obtain
  feels valuable, and that feeling is not evidence the spec needs it. Three proposals were
  withdrawn for this.
- **A fact already asserted by a test does not belong in prose too.** The test is the
  stronger statement, and a prose copy can drift from it. Point at the test instead.

## Releases

`spec/` says what the game is; `releases/` says what is being built now. **The spec is the
destination and always wins**; a release spec only says what is true today, so the two never
disagree about the goal, only about the schedule.

A release spec never invents a rule. If a release needs one the spec lacks, propose it into
the spec first, then have the release refer to it. Never resolve a scheduling question by
writing a new rule into `releases/`.

## A mistake worth not repeating

**Never edit a markdown table by string-replacing one of its rows.** The padder rewrites
column widths, so a pattern that matched yesterday silently stops matching, and
`str.replace` with no match is a no-op rather than an error. Thirteen rows went missing from
`docs/notes/proposals.md` this way before anyone noticed, and the same bug dropped three
files from the specification index.

**And never delete a range between two markers without checking what is inside it.** Deleting
from `### P-28` up to `### P-32` also deletes anything filed between them, silently. Two
proposals were destroyed that way. Count what the range contains and assert it is what you
expect before removing it.

**The reason it keeps happening is worth stating plainly.** The padder runs *after* an edit, so
the file on disk is always in padded form, while a match string drafted while writing the edit is
not. Identical content, different bytes. Two habits remove the whole class:

- **Never put a table row in a match string.** Locate the row by its prefix - `| P-42,` - and
  rebuild it. A match string containing `|` is a bug waiting for the next pad.
- **Pad before reading, not only after writing**, so every match is made against the bytes that
  are actually there.

Rebuild a table from a declared list instead, and **assert** the result: that every item is
accounted for exactly once, and that a list of files matches what is actually on disk. A
scripted edit that cannot fail loudly will fail quietly.

## Writing notes

- Date the note in its header line.
- State how any measurement was produced, so it can be re-run rather than trusted.
- Mark superseded notes explicitly. Never leave two notes disagreeing in silence.
- Add the index line in `docs/notes/README.md` before writing the note.

## Reading

Read `spec/` for what is true now. Notes are records of a moment and go stale. If a note and
the spec disagree, the spec is right and the note needs a superseded marker.
