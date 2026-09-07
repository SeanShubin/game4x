# Answering from memory, and why the instance cannot tell

2026-09-06. Answers `S-57`, filed by the specification lane from Sean's question. Carries the
argument for `X-3`.

**Method.** The seven cases in `S-57` were taken as claims rather than as facts, per
`docs/process.md` -> All lanes. Three were verified against git and the tree: `P-315` from its
withdrawal text in `b1d12c9`, `C-34` from `crates/outbox.md:768` and its correction, and the quality
README from `3765aac` and `1f1d4f4`. The other four were read as filed and are marked below as
unverified. **Nothing here was measured on a fresh instance**, because this lane did not run one -
the answer to the third question is derived from the classification rather than observed, and that
limit is load-bearing on how much it is worth.

**Corrected 2026-09-07, by the specification lane, and it is the lens working.** That lane checked
all four of its own cases from the record and refused one: **`P-315` is not a case of the
information being there unread.** Its withdrawal says the sentence *was* read and misparsed - an
imperative with no subject, from which a deadlock was inferred - and reading it again is not what
fixes a wrong inference. Verified here rather than taken: `b1d12c9`, the commit that withdrew
`P-315`, **also rewrote the sentence**, from *To start one, create `lenses/<name>/README.md` and
`lenses/<name>/outbox.md`, and tell it:* to today's *`lenses/<name>/` is its own column, so **it
creates** ... itself if they are not there*. So `P-315` leaves the second row for one of its own,
and **the table below is five phenomena over seven cases and should read six.** `C-35` and `C-34`
were checked the same way and both hold - `C-35`'s *the instruction was in the item and I read past
it* is in the item, several paragraphs below a title that is about something else, and this lane
briefly concluded otherwise from the title alone. **And three of the four cases this report marks unverified below are
now verified**, by that lane from the record rather than by this one: `P-320` against
`spec/console.md`, `S-56` against the tree and the code lane's `C-44`, and `P-310` with `P-312`
against `P-316`, the rule they produced. **Not re-checked here**, and taken as a claim from a lane
reporting on its own record - which is the weakest kind and is why it is written as one.

**An eighth case, 2026-09-07, volunteered by the code lane against itself, and it is a seventh
phenomenon.** It filed `C-65` saying `P-215`'s enclosing-command half was unbuilt; its own `C-23`
had said it was built two days earlier; it had checked `command-language`'s `Failure` rather than
`game-console`'s `Where`. **The check was made and its answer was true - about the wrong type.**
Verified here rather than taken: `crates/game-console/src/lib.rs:118` carries *the `run` commands
enclosing it*, and `crates/command-language/src/failure.rs:66` does not. **This is the first case
in the collection where the instance verified**, which is why it is its own row: the others are
cured by looking and this one is caused by it. `docs/process.md` already names the shape - *a
right number about the wrong thing invites no question*. Corrected in `5eaa5ea`.

## The finding: the question names one phenomenon and the cases are four

`S-57` asks why a long-running instance answers from memory. Sorted by what actually failed, the
seven cases do not share a mechanism, and **only one of them is memory**.

| What failed                                   | Cases                   | Would a fresh instance avoid it? |
| --------------------------------------------- | ----------------------- | -------------------------------- |
| Stale context - read it, then contradicted it | `P-320`                 | Yes                              |
| Read past a clause that was present           | `P-315`, `C-35`, `C-34` | No                               |
| Never read at all - asserted about the tree   | `S-56`                  | No                               |
| The record was destroyed by a process step    | `P-310`, `P-312`        | No, and worse                    |
| A written copy went stale on disk             | quality's README        | **No, and worse**                |

Seven bullets, eight artifacts, and **three lanes rather than the four the item claims** -
specification for `P-320`, `P-315`, `S-56`, `P-310` and `P-312`; code for `C-35` and `C-34`; quality
for the README. Recorded because this is an item about unchecked counts, so its own count is fair
game; it changes nothing about the finding.

### Why the distinction is not pedantic

The remedies do not overlap. **Re-reading fixes exactly one row.** The three in the second row had
the text in front of them; reading it again is what they already did. `P-315`'s withdrawal is
explicit about what went wrong instead:

> I read *to start one, create ... and tell it* as requiring the files before the instance, and
> inferred a deadlock from an imperative with no subject.

The source was genuinely ambiguous - an imperative with no subject - and the instance resolved the
ambiguity silently and confidently rather than noticing it was resolving one. **The fix was to the
document, not to the reader**: `CLAUDE.md:352` now reads *`lenses/<name>/` is its own column, so
**it creates**...*, and the subject is supplied. `C-34` is the same family with the refutation even
closer - the entry's own sentence contradicts the entry, one clause away.

## Question 2: is any signal available from inside?

**No, and the reason is structural rather than a lapse.** A fact recalled from earlier in the
context and a fact read a moment ago occupy the same kind of slot in the same window. Neither
carries a provenance marker, a timestamp, or a staleness bit. There is nothing to consult, so the
lane's confidence is identical in both cases - which is `C-33` turned on the reader, exactly as the
item says.

**But the signal exists outside, and this repository already receives it.** Three times during the
session that produced this report, the harness announced a file had changed since this lane last
read it - `CLAUDE.md`, `lenses/quality/README.md`, and one of this lane's own files. That is the
missing mechanism, already built and already firing.

**It has one gap worth naming, and it is the shape of the whole problem.** The notice can only fire
for a file the instance has read. It is therefore silent for exactly the case where nothing was read
at all - `S-56`, work filed without looking at the tree. **The instrument rewards reading and is
mute about not reading**, which is the same asymmetry as a count of zero against a population of
zero.

## Question 1: what distinguishes a claim that needs re-reading?

Not elapsed time, and not importance. **Every one of the seven is a claim about an artifact some
other writer can change** - another lane's column, the working tree, or a queue that promotion
deletes from. None is a claim about arithmetic, about an immutable fact, or about this lane's own
files.

That yields a test that is mechanical rather than a matter of judgement, because the lane table
already says who writes what:

> **A claim about a file outside your own column, or about the state of the tree, is read at the
> moment of the claim. A claim about your own column or about something nobody can edit is not.**

It is narrow enough to follow. `docs/process.md` already carries habits of this shape - name the
population, re-poison a check, ask for an artifact - and each one names *when*, which is what keeps
it from decaying into *be careful*.

**This lane tested the rule on itself while writing the report**, which is the only measurement here
that is not retrospective. `docs/process.md` changed under it mid-session: an outline taken early
put *Specification Instance* at line 159, and the section was at 219 an hour later. Every line number
in `X-1` and `X-2` was re-derived at the moment of filing rather than carried forward, and two of
them had moved. The rule fired correctly on a file in another lane's column.

## Question 3: does clearing context fix it, and what does it cost?

**It fixes one of the four, and it makes two of them worse.** This is the part worth Sean's
attention, and it inverts the assumption in the item.

The outbox architecture assumes an instance is replaceable by a fresh one that reads the files. That
holds **only to the extent the files are not themselves stale** - and a fresh instance is strictly
more vulnerable there, because it has no memory to contradict a document with. The quality README is
the proof, and it is already in the tree: the copy said `docs/process.md` gave that lens three jobs,
it was true on 2026-08-30 and wrong by 2026-09-05, and **a fresh instance would have believed it
completely.** The only reader who could have caught the drift is one that had seen both versions.

The same inverts for `P-310` and `P-312`. A promise deleted at promotion is invisible to a fresh
instance by construction; the long-running one at least had the promise in its context, which is how
Sean's question surfaced it at all.

**So restarting is not the remedy, and the project has already built the real one for a different
reason.** `docs/README.md`'s rule - a document that restates another links to it instead - is what
makes a fresh instance safe, because a link cannot go stale the way a copy does. The quality lens
reached it the expensive way and said so:

> having just been burned by a stale copy, this file's first correction was to make a better copy

`3765aac` made the better copy; `1f1d4f4` replaced it with a link one commit later.

**What it costs, stated as a limit rather than a number.** This lane cannot say what fraction of a
long-running instance's beliefs are memory, and neither can the instance - that is the question's own
point. What can be said is that the two failure modes trade against each other, so *restart more
often* buys less than it looks like it buys, and *copy less* buys more.

## What this report does not claim

- **It does not say the specification lane was careless.** Four of the seven are defects in
  documents or in the workflow rather than in a reader, and one of them was fixed by adding a subject
  to a sentence.
- **It did not measure a fresh instance.** Question 3 is answered by classifying the cases, not by
  running the experiment. An instance started cold and asked the seven questions would settle it, and
  is the obvious next study if Sean wants one.
- **Four of the seven cases are unverified**, taken as filed: `P-320`, `S-56`, `P-310` and `P-312`.
  The three that were checked all held.
