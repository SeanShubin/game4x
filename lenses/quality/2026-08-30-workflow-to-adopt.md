# The workflow, ready to paste

**Derived.** Written by the quality lens on 2026-08-30, after Sean accepted
[the recommended workflow](2026-08-29-workflow.md). Everything here is destined for a directory this
lens may not write to, so it is written out verbatim rather than described.

[Quality](README.md) · [The outbox](outbox.md) · [Why](2026-08-29-workflow.md)

Two blocks. Each says which lane owns the file it goes in.

| Block                                  | Goes in         | Lane              | Item   |
| -------------------------------------- | --------------- | ----------------- | ------ |
| [The workflow section](#for-claude-md) | `CLAUDE.md`     | **specification** | `Q-13` |
| [The index tool](#for-tools)           | `tools/outbox/` | **code**          | `Q-14` |

### One thing the paste implies

The block below says a lens lives at `lenses/<name>/`, and this one is still at `quality/`. Adopting
it means moving the directory, and **the specification lane owns both halves of that move** - the
six references to `quality/` are in `CLAUDE.md` (three) and `docs/notes/proposals.md` (three), which
is its column. This lens deliberately has not moved itself: doing so would orphan six links it
cannot repair. Once the move lands, the links *inside* `quality/` are this lens's to re-point, and
it will.

The quality lens's own half is already live: [`quality/outbox.md`](outbox.md) exists, carries every
finding this lens has made, and follows the format below.

---

<a id="for-claude-md"></a>

## 1. For the specification lane — into `CLAUDE.md`

Replaces the **Three instances** section. Everything after it - the promotion rules, the spec
editing rules, the mistakes worth not repeating - is untouched.

<!-- paste from here -->

## Perspectives

Several Claude instances work in this repository at once. **Two of them produce; the rest look.**

**The producers** own what ships. `spec/` is the destination and `crates/` is the artifact, and each
is authored by exactly one instance.

**The lenses** are research perspectives. Each takes the same problem with a different focus, reads
everything, produces claims about the producers' work, and ships nothing. Quality is one. A lens is
not a lane beside the others; it is a way of looking at all of them.

| Perspective                     | Writes                                                                      | Reads      |
| ------------------------------- | --------------------------------------------------------------------------- | ---------- |
| **Specification**               | `spec/`, `releases/`, `docs/`, `README.md`, this file                       | everything |
| **Code**                        | `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`, `hooks/`, CI, cargo | everything |
| **Quality**, and any other lens | its own directory under `lenses/`, and nothing else                         | everything |

**Everyone reads everything; nobody writes outside their own column.** That asymmetry is what makes
them composable rather than merely separated - a perspective that cannot read the others has to
guess, and one that can write to another has to be trusted.

Three rules follow from the asymmetry, and they are not symmetric:

- **A producer never writes into a lens's directory.** A lens whose findings a producer can edit has
  stopped being independent evidence.
- **A lens never writes into a producer's directory.** A lens that can edit `crates/` is a second
  author of the code with none of the responsibility.
- **A lens is refutable, and a producer is the thing that refutes it.** A producer that declines a
  finding says so in the commit that declines it, citing the id. Being refuted is the lens working,
  not failing.

**Quality runs nothing that writes.** Reading the tree and running `cargo clippy`, `cargo test` and
`cargo tree` are all fine. `cargo fmt`, `cargo fix` and `clippy --fix` are not, because they modify
the very files being judged - and a review that alters its subject is no longer a review.

## Outboxes

Every perspective keeps **one outbox** in its own directory. It is the only thing another
perspective has to read.

- The specification lane's outbox is [`docs/notes/proposals.md`](docs/notes/proposals.md).
- A lens's outbox is `lenses/<name>/outbox.md`.
- The code lane's outbox is `crates/outbox.md`, and holds the questions that block it.

Every item in an outbox carries four things:

| Field      | What it is                                                                              |
| ---------- | --------------------------------------------------------------------------------------- |
| **id**     | Stable and unique, so a commit can cite it and a later report can say what became of it |
| **to**     | `sean`, `spec`, `code`, or a named lens - or **absent**, meaning *not ready, no reader* |
| **status** | `open`, `acted`, `rejected`, `withdrawn`, `answered`                                    |
| one line   | What it is, so a reader can triage it without opening the source                        |

**`to` is the field that does the work.** It turns every instance's reading list from a directory
sweep into a query, which is what keeps an instance focused on its own purpose.

**Unaddressed research is addressed to nobody, and that is a feature.** Work that is not ready costs
no one any attention. It sits in a dated note in its own directory and becomes visible the moment
its author gives it a reader - held back by not having one, rather than by discipline.

### What each perspective reads

| Perspective | Inbox                                                                   | Never has to read       |
| ----------- | ----------------------------------------------------------------------- | ----------------------- |
| **Sean**    | everything `to sean` - proposals awaiting review, and blocked questions | any lens's raw research |
| **Spec**    | `to spec`, plus `docs/notes/spec-backlog.md`                            | code reviews            |
| **Code**    | `to code`, plus `releases/`                                             | the proposal queue      |
| **A lens**  | everything - that is what a lens is for                                 | -                       |

## Nothing open means nothing outstanding

**If nothing in any outbox is `open` and addressed, nothing known is outstanding.**

That is a promise about the outboxes, not about the product: it does not say the specification is
complete or the code correct, only that **everything any perspective knows to be wrong is sitting
where its reader will find it.**

So a contradiction has exactly one resting place, and it is an outbox. When any perspective finds
one, it files it **the moment it is found**, whatever else is happening. Never a paragraph in a
discussion, never a sentence in a reply, never a line in a note.

This has already failed once, in the way this rule exists to prevent. `P-123`'s contradiction sat in
a quality report for a day while the proposal queue was empty - and for that whole day the queue
said, truthfully by its own old wording and falsely in fact, that nothing known was inconsistent.
The promise now spans every outbox for exactly that reason.

**Keep the open items under fifteen across every outbox together, not fifteen each.** Past that,
reviewing costs as much as doing and the mechanism has failed. A lens producing many true findings
crowds out another lens's fewer, better ones, so a lens competes on the value of a finding rather
than the count.

## The cycle

1. **Sean says something.** The specification lane records it in `docs/notes/spec-backlog.md`. Only
   the writing counts.
2. **A lens explores**, in its own directory, addressed to nobody. Research sits here as long as it
   needs to.
3. **A lens is ready**, and addresses a finding: `to spec` if it needs a decision, `to code` if it
   is a defect in the build.
4. **The specification lane turns `to spec` items into numbered proposals**, naming the destination
   file and section. It does not decide anything.
5. **Sean reviews the queue.** He edits the proposal text in place until satisfied, then says
   *promote P-n*. This is the one step that cannot be delegated, which is why everything else exists
   to keep the queue short enough to read.
6. **The specification lane promotes verbatim and asserts it landed.** A promotion that makes
   something else stale files the cleanup immediately.
7. **The specification lane updates `releases/`** to say what is being built now. A release never
   invents a rule.
8. **The code lane builds** from `releases/` and `spec/`.
9. **The code lane hits a gap and does not stop.** It does everything that does not depend on the
   answer, files a question `to sean` **stating the assumption it proceeded under**, and carries on.
   A blocked question is filed and worked around, never waited on - otherwise a session stalls until
   Sean is free, and the assumption that was made is discovered later rather than recorded now.
10. **A lens reviews the result**, and it returns to step 3.

## Starting a new lens

A lens costs attention, which is the scarce resource. Before starting one, it has to answer: **what
class of finding will this produce that neither producer would?** If the honest answer is *the same
things, sooner*, that is a case for better instructions to the code lane, not a new directory.

To start one, create `lenses/<name>/README.md` and `lenses/<name>/outbox.md`, and tell it:

> You are the `<name>` lens. Read `CLAUDE.md` → Perspectives, then `lenses/<name>/README.md`.
>
> You write `lenses/<name>/` and nothing else. You read everything. You never edit what you review,
> and you never run `cargo fmt`, `cargo fix` or `clippy --fix` - they modify the files you are
> judging.
>
> Your focus is `<one sentence: what this lens looks for that neither producer would>`.
>
> Findings go in `lenses/<name>/outbox.md`, each with an id, a `to`, a `status` and one line, and
> each pointing at a dated report that carries the argument. Research that is not ready is addressed
> to nobody and costs no one anything.
>
> Every finding says four things, and one missing any of them cannot be acted on without coming back
> to you: **where** - file and line; **what** - the defect in one sentence; **why** - what it costs;
> **whether** - worth doing now, eventually, or noted and deliberately not. Most findings should be
> noted and not. A report where everything matters is a report where nothing does.
>
> When a producer declines a finding, check it before defending it. It will often be right, and the
> check is worth more than the finding was.

<!-- paste to here -->

---

<a id="for-tools"></a>

## 2. For the code lane — a new `tools/outbox/`

Sibling of `tools/pad-tables`: outside the workspace, so it never appears in `cargo tree` or
`cargo build --workspace`.

**What it does.** Walks every outbox, parses the items, and answers *what is open and addressed to
whom*.

**Where it looks.** `docs/notes/proposals.md`, `crates/outbox.md`, and `lenses/*/outbox.md`.

**What an item looks like.** A heading, then a field line. Chosen so nothing needs a table row -
`tools/pad-tables` rewrites column widths, so a parser keyed to table bytes breaks on the next pad.

```markdown
### Q-3 - `planet-bevy` depends on `game-front`, so a prototype links the whole game

**to** code · **status** open · **raised** 2026-08-28 · **source** [report 3](...)

One line of what it is.
```

The parser needs only two lines per item: `### <id> - <title>`, and the next line beginning `**to**`
carrying `**to** <who>` and `**status** <state>`. Everything else is prose for a human.

**Usage.**

```
outbox                  every open item, grouped by addressee
outbox --to code        one addressee's inbox
outbox --check          exit non-zero if anything is open and addressed
outbox --count          the aggregate, against the limit of fifteen
```

**Two things worth asserting rather than assuming**, because both are failures this repository has
actually had:

- **Every id is unique across every outbox**, and every id cited in a commit message resolves to
  one. Duplicated or dangling ids are how a status silently stops meaning anything.
- **The file count matches what is on disk.** `tools/pad-tables` exists because thirteen rows went
  missing from a hand-edited table and nothing noticed. A tool that walks directories should say how
  many outboxes it found, so a missing one is visible rather than silently excluded.

**Optionally, and it closes the loop mechanically:** parse `git log` for `finding: <id> <status>`
and reconcile it against what the outbox claims. Commit messages here already cite findings by
number - `b43d9b4` names three and says what it did with each - so the convention is nearly free,
and it is what lets a producer's rejection reach a lens without the producer writing in the lens's
directory.
