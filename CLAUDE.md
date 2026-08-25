# CLAUDE.md

Conventions for AI assistants working in this repository. Read this before editing anything.

## Authority

Three tiers, and the boundary is the directory it lives in.

| Path          | Author     | What it is                                                                       |
| ------------- | ---------- | -------------------------------------------------------------------------------- |
| `spec/`       | **Sean**   | Normative. He types every line and reviews every line.                           |
| `docs/`       | shared     | The "why" layer. Reviewed. Follow the rules in [docs/README.md](docs/README.md). |
| `docs/notes/` | **Claude** | Derived records of conversation. Not binding. Sean is not expected to read them. |

## The rule that matters

**A decision is only real when it appears in `spec/`, reviewed by Sean.**

Anything in `docs/notes/` is a proposal, however confident it sounds and however much
measurement backs it. Analysis does not become policy by accumulation.

- The spec **states**. Terse, present tense, normative.
- The notes **justify**. Long, dated, measured.
- The spec links down to a note for reasoning; the note links up to the spec for the
  decision. Never the reverse. A note must never read as though it settled something.

## Editing `spec/`

The guarantee is **review**, not authorship. Nothing reaches this directory that Sean has
not read and agreed to. Within that, there are three tiers.

Allowed without asking:

- typos and spelling
- whitespace, table alignment, heading levels
- links that broke because a file moved

Allowed, but **report every line touched**:

- reorganizing: moving a line to a better file or section, adding headings
- rephrasing: saying the same thing more clearly

Never without an explicit accept:

- **adding a line that says something new**

New content reaches the spec by one route only: file it in
[`docs/notes/proposals.md`](docs/notes/proposals.md) with a number and its inference kind,
Sean accepts it in chat, and only then does it land. Record every rejection with his reason,
or the same proposal will be filed again in a later session.

Never silently rewrite his prose. "I changed lines 40 and 57" costs him a three-line diff;
an unannounced rewrite costs him a re-read of the whole file, and after that he cannot trust
the file.

Keep the open-proposal queue under fifteen. Past that, reviewing costs as much as writing
and the mechanism has failed. If a lot of proposals would be guesses at design, ask one
question instead.

## Releases

`spec/` says what the game is; `releases/` says what is being built now. **The spec is the
destination and always wins**; a release spec only says what is true today, so the two never
disagree about the goal, only about the schedule.

A release spec never invents a rule. If a release needs one the spec lacks, propose it into
the spec first, then have the release refer to it. Never resolve a scheduling question by
writing a new rule into `releases/`.

## Writing notes

- Date the note in its header line.
- State how any measurement was produced, so it can be re-run rather than trusted.
- Mark superseded notes explicitly. Never leave two notes disagreeing in silence.
- Add the index line in `docs/notes/README.md` before writing the note.

## Reading

Read `spec/` for what is true now. Notes are records of a moment and go stale. If a note and
the spec disagree, the spec is right and the note needs a superseded marker.
