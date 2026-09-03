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

## Perspectives

**[`docs/process.md`](docs/process.md) is Sean's statement of what this process is for.** This file is
the operating detail and must not contradict it. Where the two disagree, raise it rather than picking
a winner - a difference may mean this file has drifted, or may mean it learned something that
document has not caught up with, and which is later is Sean's to say.


Several Claude instances work in this repository at once. **Two of them produce; the rest look.**

**The producers** own what ships. `spec/` is the destination and `crates/` is the artifact, and each
is authored by exactly one instance.

**The lenses** are research perspectives. Each takes the same problem with a different focus, reads
everything, produces claims about the producers' work, and ships nothing. Quality is one. A lens is
not a lane beside the others; it is a way of looking at all of them.

**The code lane owns the game. Every lane owns the tools for its own work.** Production is what
ships to a player and is the code lane's alone. **Production support** - `hooks/`, `scripts/`, CI,
and everything in `tools/` that is not a lane's own - belongs to no lane exclusively: whoever needs
a check wired adds it, and the bar is a narrow reason rather than a permission.

| Perspective                     | Writes                                                               | Reads      |
| ------------------------------- | -------------------------------------------------------------------- | ---------- |
| **Specification**               | `spec/`, `releases/`, `docs/`, `README.md`, `tools/spec/`, this file | everything |
| **Code**                        | `crates/`, `web/`, `commands/`, `prototypes/`, cargo                 | everything |
| **Quality**, and any other lens | its own directory under `lenses/`, and `tools/<its name>/`           | everything |

**`temporary-notes/` is Sean's and no instance writes there**, except to create a file he has
asked for by name. It is not tracked - the files in it are transient, which is what the name says -
and it is not `docs/notes/`, which is Claude's and is kept.

**Everyone reads everything; nobody writes outside their own column.** That asymmetry is what makes
them composable rather than merely separated - a perspective that cannot read the others has to
guess, and one that can write to another has to be trusted. **It binds each producer against the
other producer too**: the specification lane does not edit code, *even to fix an obvious break* - it
reports the break and leaves it - and the code lane does not write specification.

Three rules follow from the asymmetry, and they are not symmetric:

- **A producer never writes into a lens's directory.** A lens whose findings a producer can edit has
  stopped being independent evidence.
- **A lens never writes into a producer's directory.** A lens that can edit `crates/` is a second
  author of the code with none of the responsibility.
- **A lens is refutable, and a producer is the thing that refutes it.** A producer that declines a
  finding says so in the commit that declines it, citing the id. Being refuted is the lens working,
  not failing.

**An approval for this file comes from Sean directly.** A lens or a producer may report that he
approved something, and that report is worth acting on everywhere else - but not here. The risk is
not that a relay is dishonest; it is that **a reader has no way to tell a relayed approval from an
invented one**, and the file being relayed about is the one that says who may write what. Standing
policy, set 2026-08-30, after a lens relayed three approvals for this file and this lane declined
all three until Sean said so himself. Two of the three changed on the way in.

**The line is checkability, not trust.** A relayed fact - *this document exists, these two ids are
open to you* - can be verified by whoever receives it, in seconds. A relayed approval cannot be
verified at all, which is why one travels freely and the other does not. **Facts relay; authority
does not.**

**What needs his approval is what this file says about who may write what** - the columns, the
addresses, and the promotion protocol. Wording, paths, broken links and consistency inside those
rules are the specification lane's to settle and report. Sean, 2026-08-30: he keeps executive
control over the specification, the specification keeps everything else in line, and how the
perspectives operate day to day is something he corrects when he notices it rather than approves in
advance.

**A generated file has no owner and may sit in the repository root.** Nobody edits it: all of its
content comes from files that do have owners, and a hand edit is overwritten at the next commit.
The content comes from those sources as they sit on disk, not as their owners have committed
them, so a generated file can publish work in progress - a defect in whatever writes it rather
than something this rule allows. This covers only a file that is generated in full - one with any hand-written part has an author,
and belongs in that author's directory.

**Quality runs nothing that writes.** Reading the tree and running `cargo clippy`, `cargo test` and
`cargo tree` are all fine. `cargo fmt`, `cargo fix` and `clippy --fix` are not, because they modify
the very files being judged - and a review that alters its subject is no longer a review.

Four things the perspectives make necessary, all of which have teeth:

- **Stage by name, never `git add -A`, and know that staging is publishing.** `git add` writes to an
  index all three perspectives share, and `git commit` commits the index rather than the caller's
  changes - so a file you stage is committed by whoever commits next, under a message about
  something else. This is not a caution about your own carefulness; the hazard is someone else. It
  has happened: one perspective staged a file, lost a race for `.git/index.lock`, and twenty-six
  lines of its work landed in another's commit, touching a file outside that one's column.
- **`hooks/pre-push` runs the full gate** - `cargo fmt`, clippy and the test suite across every
  crate. A documentation-only or report-only push is therefore gated on code that perspective did
  not write and must not repair. If it fails for that reason, **say so and stop**; whether to
  `--no-verify` is Sean's call, not Claude's.
- **Re-read before asserting.** A file read earlier in the session may have been rewritten by another
  instance since. Anything claimed about code needs a fresh look, not a memory.
- **Check your own outbox is not stale before adding to it.** An item marked `open` that another
  perspective has already acted on costs a reader exactly as much as a real one.

Historical exception, so the boundary is not mistaken for a description of the past:
`tools/pad-tables/` and the tests in `crates/sphere-tessellation/` were written from the
specification perspective before the split existed.

## Outboxes

Every perspective keeps **one outbox** in its own directory. It is the only thing another
perspective has to read.

- The specification lane's outbox is [`docs/notes/proposals.md`](docs/notes/proposals.md).
- A lens's outbox is `lenses/<name>/outbox.md`.
- The code lane's outbox is `crates/outbox.md`, and holds the questions that block it.
- A file in `releases/` is an outbox too: each capability is an item addressed to the code
  lane, `open` until its *vetted when* line is observed.

Every item in an outbox carries four things:

| Field      | What it is                                                                                                                                                                                |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **id**     | Stable and unique, so a commit can cite it and a later report can say what became of it                                                                                                   |
| **to**     | `spec`, `code`, or a named lens - or **absent**, meaning *not ready, no reader*. Only the proposal queue addresses `sean`                                                                 |
| **status** | `open`, `acted`, `rejected`, `withdrawn`, `answered` - and for a release capability, `open`, then `built` when the code lane says it is done, then `vetted` when a person has observed it |
| one line   | What it is, so a reader can triage it without opening the source                                                                                                                          |

**Nothing but a proposal is addressed to Sean.** A lens addresses `to spec` and so does the code
lane; the specification lane turns either into a numbered proposal, which is the one shape Sean
reads. A blocked question is almost always *the specification does not say X*, so it was already a
proposal wearing a different hat - `Q-17` became `P-123`.

The cost is a hop: a question routed through the queue waits for this lane to run before it reaches
him. Accepted deliberately, in exchange for one place to look. **What is not negotiable is that the
proposal queue keeps its `to sean` address**, because `tools/outbox` cannot see an item that has no
`to` field, and the queue is the one outbox that must never be invisible.

**Instances can message each other, and a message is never the record.** The sessions are named
`4x spec`, `4x code` and `4x quality`, and `SendMessage` reaches one by that name. A message only
arrives if the other instance is running, so it can make a handoff prompt and can never be the thing
a decision rests on. **Write it in an outbox first, then say so in a message if the other one is
awake.** An approval for `CLAUDE.md` is the one thing a message may not carry at all - see above.

**An item whose completion needs a person is addressed to a person.** This is the rule that was
missing, and its absence produced exactly the state Sean said should be impossible: five of the six
capabilities in `releases/first-release.md` are vetted by somebody *looking* - at a drawing, or at a
whole game played through - and all six sat addressed `to code`, which may build them and may not
mark them done. So five items could never move, while `pending.md` reported that nothing needed
deciding.

**A capability therefore has two addressees in turn.** `open` and `to code` while it is being built;
`built` and `to sean` once the code lane says it is done and a person has not yet looked. The code
lane sets `built`; only Sean sets `vetted`.

**And this lane files a question rather than asking one.** A decision put to Sean in a reply is not
in any file, so it is lost if the session ends and invisible to `pending.md` while it lives. On
2026-08-30 this lane asked him three things in prose - how to bound the research loop, where a
capability's status should live, whether `pending.md` belonged in his own document - and none of the
three was ever an item. **If it is worth his attention it is worth an id.**

**`to` is the field that does the work.** It turns every instance's reading list from a directory
sweep into a query, which is what keeps an instance focused on its own purpose.

**Unaddressed research is addressed to nobody, and that is a feature.** Work that is not ready costs
no one any attention. It sits in a dated note in its own directory and becomes visible the moment
its author gives it a reader - held back by not having one, rather than by discipline.

### What each perspective reads

| Perspective | Inbox                                                            | Never has to read       |
| ----------- | ---------------------------------------------------------------- | ----------------------- |
| **Sean**    | the open proposals, which are the only thing addressed `to sean` | any lens's raw research |
| **Spec**    | `to spec`, plus `docs/notes/spec-backlog.md`                     | code reviews            |
| **Code**    | `to code`, plus `releases/`                                      | the proposal queue      |
| **A lens**  | everything - that is what a lens is for                          | -                       |

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

**A lens competes on the value of a finding, not the count.** One producing many true findings
crowds out another's fewer, better ones, and crowds out the queue that actually waits on Sean - so
filing everything it notices costs the whole system attention even when every item is correct. The
one number in this file is the limit on the open-proposal queue, below, and it counts that queue
and nothing else.

## What done means

**Everything is verified by the thing that would be false if the work had not been done.** What
differs is who can observe that thing, and only one of the three is a person - which is why only one
of the three ever reaches Sean.

| Kind of work          | Verified by                             | Observed by  |
| --------------------- | --------------------------------------- | ------------ |
| A feature             | an observation of the running game      | **a person** |
| A quality improvement | a test that would have failed before it | the gate     |
| Research              | a recorded answer to a stated question  | the index    |

**A feature's evidence is a *vetted when* line** in a release: the observable thing that has to be
true. It moves `open` to `built` to `vetted`, and changes hands at `built`, because the code lane
may build a thing and may not certify it.

**A quality improvement's evidence is a check that did not exist before.** Passing tests prove
nothing here - they passed beforehand too. `Q-8` is the pattern: one test comparing the two
derivations at every planet size, and a second demonstrating that the old path diverged under jitter
while every other test in the repository went on passing. **The second test is the verification.**
So a refactor with no new check is not done, it is *unverified*, and says so rather than closing
quietly. **`Q-1` is the rule doing both halves of its job**: it stayed open while the copy could not be
checked, because deleting blind proves nothing either way, and it closed on 2026-08-30 once a
harness - `--shot`, `--settle` and `--renderer` - made the second half checkable.

**Check the rule over every case, not on one case, and assert how many cases there were.** A test
that shows a rule on one example stops showing anything the moment that example is edited away, and
goes on passing. A test over every case does not - except where there are no cases, and then it
passes for the wrong reason. **The count is what tells those two apart.** `Q-8` is the shape: two
derivations compared at every planet size rather than at one. Three checks stopped meaning anything
on 2026-09-01 and all three stayed green -
[what happened](docs/notes/checks-outlive-examples.md).

**Research's evidence is the answer, written where it will be found.**
[`docs/prototypes/README.md`](docs/prototypes/README.md) already says it for prototypes - *that
answer is the deliverable; the code is a byproduct* - and it generalises: the question is stated, the
answer is recorded, and the document is reachable by links from the root README, which
[`docs/README.md`](docs/README.md) makes the test of existing at all. **Research with no recorded
answer is unfinished**, however much was built. `prototypes/goldberg-view` was exactly that until
2026-08-30 and is not now: it records an answer the question did not expect - appearance was
never the constraint, diminishing strategic depth was - which is the deliverable arriving in a
shape the code could not have predicted.

**Research never reaches Sean as research.** If its answer implies a decision it becomes a proposal;
otherwise it is recorded and findable, and that is the whole of done.

## The cycle

1. **Sean says something.** The specification lane records it in `docs/notes/spec-backlog.md`. Only
   the writing counts.
2. **A lens explores**, in its own directory, addressed to nobody. Research sits here as long as it
   needs to.
3. **A lens is ready**, and addresses a finding: `to spec` if it needs a decision, `to code` if it
   is a defect in the build.
4. **The specification lane turns `to spec` items into numbered proposals**, naming the destination
   file and section. It does not decide anything.
5. **Sean reviews the queue.** He says what should change; the specification lane makes that change
   and **shows the result**; he reads it and says *promote P-n*. He may edit the text in place
   instead, and often does when he already knows the words - see **Promotion** below for the full
   split and why *promote* has to mean *I have read this*. This is the one step that cannot be
   delegated, which is why everything else exists to keep the queue short enough to read.
6. **The specification lane promotes verbatim and asserts it landed.** A promotion that makes
   something else stale files the cleanup immediately.
7. **The specification lane updates `releases/`** to say what is being built now. A release never
   invents a rule. **This is the work order, and a promotion into `spec/` is not one** - the spec is
   the destination and most of it is not buildable when it lands. Of the thirteen proposals promoted
   on 2026-08-29 and 30, four changed what the code should build and nine described a rule system
   nothing can name a condition for yet.
8. **The code lane builds** what is `open` and addressed to it - the lens's findings and the
   release's capabilities, in one list. **It does not mark a capability vetted**: it reports the
   evidence `to spec`, and the specification lane records it, so the account of what has been
   delivered is not kept by whoever built it.
9. **The code lane hits a gap and does not stop.** It does everything that does not depend on the
   answer, files a question `to spec` **stating the assumption it proceeded under**, and carries on.
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

## The rule that matters

**A decision is only real when it appears in `spec/`, reviewed by Sean.**

Anything in `docs/notes/` is a proposal, however confident it sounds and however much
measurement backs it. Analysis does not become policy by accumulation.

- The spec **states**. Terse, present tense, normative.
- The notes **justify**. Long, dated, measured.
- The spec links down to a note for reasoning; the note links up to the spec for the
  decision. Never the reverse. A note must never read as though it settled something.

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

| Step                                                                     | Who    |
| ------------------------------------------------------------------------ | ------ |
| File the proposal, naming the destination file and section               | Claude |
| Say what should change about it                                          | Sean   |
| Make that change in the proposal, and show the result                    | Claude |
| Read the result, then say "promote P-n"                                  | Sean   |
| Copy the text **verbatim** into the destination, then assert it is there | Claude |

Sean may edit a proposal in place instead, and that is still the shortest path when he already
knows the words. **Saying what he wants is the common case**, because it is why the queue exists -
so that he never has to open a file or hunt for a section.

**That shift moves a responsibility, and it is worth naming.** When Sean edited the text himself he
had obviously read it. When Claude edits on his instruction, **the last version may be one he has
not seen** - so *promote* means "I have read this", and Claude shows the changed text rather than
merely reporting that it changed.

**Promotion is a pure move.** The only things Claude may change while promoting are line
wrapping, bullet-versus-paragraph, and heading level - because those depend on the
destination file and cannot be settled in the proposal. Nothing else, ever.

If Claude wants a word changed, it changes the **proposal** and says so, before Sean
approves. Fixing phrasing during promotion would mean the text Sean reviewed and the text in
the spec are not the same text, and he would have no reason to re-read the spec to find out.
The guarantee this buys is that **approved text is byte-identical to shipped text.**

**Never leave a promotion uncommitted.** A promotion changes `spec/` or `releases/` and moves a row
in the queue, and **the other lanes read the working tree** - so an uncommitted promotion is a rule
they may already be building against, with no way to see when it arrived or why. That is the
requirement: not how the commits are grouped, but that none of them is still pending when the turn
ends.

**One commit per promotion is a good default and not a rule.** It makes the history record decisions
rather than sessions, and it makes a single change easy to read back. Several promotions in one
commit is fine when they answer one decision, and the message should then say so.

**A promotion that makes something else stale is not finished.** When landing a proposal
invalidates a line elsewhere - in `spec/`, in a release, or in another proposal - Claude does one of
two things and never a third:

- **Refuse the promotion**, saying what has to be decided first, or
- **File a cleanup proposal immediately after**, so the staleness sits in the queue.

**A promotion that leaves a question open files it before deleting the proposal that asked it.** A
proposal is where a question gets asked and a promotion is where it stops being one - so anything the
proposal said it did **not** settle has nowhere to go, and goes nowhere. Eleven promotions on
2026-09-01 lost four questions this way, all recovered afterwards from a diff, which is not a reader.

Noting it in a discussion paragraph and moving on is the failure. It reads like diligence and
behaves like forgetting. **The queue is what gets read; prose is not.** P-85 found six
contradictions in one release file, and two of them had been flagged at the time and left there -
long enough for the coding instance to implement a rule the specification does not have.

**A section that takes a second proposal gets re-read whole.** The staleness rule above is
directional and per item - *does landing this invalidate something else?* A contradiction is
neither: it is symmetric, and it is between items. P-100 and P-109 both passed the staleness check
correctly and still could not both hold, because nothing asked whether the section they landed in
agreed with itself.

So when a proposal lands in a section another proposal has already landed in, re-read that section
whole and ask whether all of it can hold at once. **A trigger, not a duty** - a duty rots and a
trigger fires. It is not scoped to a session: a contradiction does not depend on the two rules
arriving together, and in a specification written over weeks the second one usually arrives later.

Measured against the 106 proposals landed so far it fires on sixteen sections, and would have caught
both known collisions - P-100 against P-109, and P-96 leaving borders in the realistic drawing while
scoping ids and poles out of it. **Firing too often is the correct error here**: a false fire costs
re-reading ten bullets, and the miss cost two days.

**A promotion can withdraw a finding as easily as it can create work, and both need saying.** After
promoting, check the index for open items that cite the destination file - `outbox` lists them - and
tell their owner. **A rule that moves under an open item makes it wrong without touching it**, and
nothing else will notice: the item still reads correctly, its evidence is still quoted accurately, and
only its conclusion has stopped being true.

This was found by the code lane on 2026-08-31. Fourteen proposals landed, this lane reported that
none of them was work for it - true of *building* - and two of its open findings had been withdrawn by
`P-126` and `P-138` without anyone saying so. **It re-ran them itself and caught it**, which is the
expensive way.

**Every promotion asserts.** Copy, then verify the text is present in the target file, and
fail loudly if it is not. Claude has three times reported that something landed when it had
not - thirteen missing rows in the proposals file, three files missing from the spec index -
and every one was a report of intent rather than of fact. An operation that cannot fail
loudly will fail quietly.

**Accepting a proposal is not permission to write it.** Claude's job ends at proposing. If
Sean explicitly asks Claude to enter an accepted line, that is a separate instruction and
Claude may - but it is never the default, and never inferred from the word "accepted".

**An open proposal is an outbox item, and carries the four fields** so that `tools/outbox` can see
it. Without the `**to**` line the item is invisible to the index, and *nothing open means nothing
outstanding* silently stops covering the queue - which is the one place it most has to.

```
### P-n - a one-line title

**to** sean · **status** open · **raised** <date> · **kind** <kind> · **into** `file` -> section
```

Proposals go in [`docs/notes/proposals.md`](docs/notes/proposals.md), numbered, each labelled
with what kind of inference produced it. Record every rejection with Sean's reason, or the
same proposal will be filed again in a later session.

Never silently rewrite his prose. "I changed lines 40 and 57" costs him a three-line diff; an
unannounced rewrite costs him a re-read of the whole file, and after that he cannot trust the
file.

**A proposal is read once, by one person, who decides from it.** State the question, give the
proposed text, give the facts that make it right, and stop. Reasoning that does not change the
decision belongs in a note. P-124 was filed at sixty lines to say a generated file may sit in the
repository root, under the heading *a file nobody can write needs no owner* - which Sean read as
esoteric and vacuous, and it was both. A queue that is tiring to read stops being a review surface.

**A lens may have at most eight items open to any one producer**, and closes or withdraws before
filing a ninth. That is a different budget from the queue limit below, which counts what waits on
Sean; this one counts what waits on a producer, where the cost is a backlog nobody reads through
before choosing what to do next. Eight is about two days of a producer's throughput as measured on
2026-08-30, when the code lane closed five of thirteen in a day - loose enough that nothing urgent
is blocked, tight enough that a lens has to close something before it can file something.

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
