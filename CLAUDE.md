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

**A decision is only real when it appears in `spec/`, in Sean's words.**

Anything in `docs/notes/` is a proposal, however confident it sounds and however much
measurement backs it. Analysis does not become policy by accumulation.

- The spec **states**. Terse, present tense, normative.
- The notes **justify**. Long, dated, measured.
- The spec links down to a note for reasoning; the note links up to the spec for the
  decision. Never the reverse. A note must never read as though it settled something.

## Editing `spec/`

Allowed without asking:

- typos and spelling
- whitespace, table alignment, heading levels
- links that broke because a file moved

Not allowed without asking:

- adding, removing, or reordering sentences
- rewriting an argument, however much clearer it would be
- tidying prose

Phrasing improvements are welcome, but either propose them in chat first or apply them and
**report every line touched**. Never silently rewrite his prose. "I changed lines 40 and 57"
costs him a three-line diff; an unannounced rewrite costs him a re-read of the whole file,
and after that he cannot trust the file.

## Writing notes

- Date the note in its header line.
- State how any measurement was produced, so it can be re-run rather than trusted.
- Mark superseded notes explicitly. Never leave two notes disagreeing in silence.
- Add the index line in `docs/notes/README.md` before writing the note.

## Reading

Read `spec/` for what is true now. Notes are records of a moment and go stale. If a note and
the spec disagree, the spec is right and the note needs a superseded marker.
