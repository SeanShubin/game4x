# Research outbox

**Derived.** The research lens's one outbox. Every finding it has addressed to somebody, and what
became of it. Not binding - a finding is a claim about the tree, not a decision about it.

[Research](README.md) · [Reports](README.md#reports) · [The proposal queue](../../docs/notes/proposals.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to spec` - something for `spec/`, `releases/` or `docs/`. The specification lane turns it into a
  numbered proposal; **it does not decide it.**
- `to code` - a defect or a decision in `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`,
  `hooks/` or CI.
- **Unaddressed** research does not appear here at all. It lives in a dated report and is nobody's
  work until this file gives it a reader.

**Status** is one of `open`, `noted`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` is
outstanding; `noted` is *recorded so it is not re-found*, and is terminal.

> **The guarantee.** If nothing here is `open`, this lens knows of nothing outstanding. That is a
> promise about this file, not about the tree - it does not say the documents are right, only that
> everything this lens knows to be wrong is sitting where its reader will find it.

**A producer may decline a finding, and often should.** It says so in the commit that declines it,
citing the id, and this file records it. Check a rejection before defending it.

## Open

### X-5 - the measurement `X-3` lacks needs an instance nobody has authorised

**to** spec · **status** open · **cited** `d7b0ec8` — lifted into `P-350`, and open until Sean answers it · **raised** 2026-09-06 · **source** [report](2026-09-06-answering-from-memory.md) · **designed** [report](2026-09-07-the-cold-instance-study.md)

**What.** `X-3` answers `S-57` by classifying seven cases rather than by measuring, and says so. The
measurement it wants is a cold instance run against the seven questions - one that has read the files
and nothing else - to test whether a fresh reader avoids each failure or reproduces it. That would
settle question 3 as evidence instead of as derivation.

**Why it is filed rather than done.** It needs an instance started for the purpose, which is Sean's
resource rather than this lane's, and this lane's instructions do not let it spawn one unasked. It
was recorded inside `X-3`'s prose, where `pending.md` cannot see it - which is the same failure as a
promise living only in a proposal that promotion deletes.

**Whether.** Worth doing eventually rather than now. `X-3`'s answer stands without it; what the
measurement would add is the ability to say *how much* of a long-running instance's belief is memory,
which no amount of classifying will produce. **Nothing waits on it.**

**Designed 2026-09-07, and it is smaller than this item said.** The design is in
[its own report](2026-09-07-the-cold-instance-study.md); what it needs from Sean is one instance's
startup, not a study. **Two corrections to the words above.** It is three tasks and not seven - of `X-3`'s
six phenomena, stale context is absent from a fresh instance by construction, a record destroyed at
promotion is not there for any reader, and `P-315`'s misparsed sentence was rewritten by the commit
that withdrew it. **Four of the seven cases stay runnable**, and the three tasks are one per runnable
phenomenon. And the tempting cheap form - hand a cold instance the cases and
ask which it would have got right - **measures findability rather than finding**, which is the shape
`docs/process.md` warns returns a plausible number and invites no question.

**The predictions are pre-registered in that report, before any run.** `X-3` is this lane's own
answer, and a study designed after seeing it will agree with it unless what would refute it is fixed
first. One of the three cases can cost `X-3` its central claim; the other two can only confirm.

**Why it is worth lifting out of the queue rather than waiting its turn.** Sean is restarting the
instances after a crash, so a reader that has read the files and nothing else is a state he is
producing anyway. **The cost is three tasks at one instance's startup, and no writes.** The
specification lane has been asked to put it to him as its own decision.

### X-7 - the carrier for the staging hazard is already built, and nothing tells a lane to use it

**to** spec · **status** open · **raised** 2026-09-07 · **re-addressed** 2026-09-07 from `code`, before that lane read it - the mechanism it asked for already exists · **source** [report](2026-09-07-the-shared-index-race.md), and the code lane reporting it against this lane's `8f687d5`

**Where.** `CLAUDE.md:129`, the *stage by name* bullet. The event: `8f687d5` is this lane's commit
and carries 21 lines of `crates/outbox.md`, outside its column and unmentioned in its message -
verified here with `git show --stat`, not taken from the report.

**What.** The rule's remedy cannot prevent the failure the rule describes. This lane **did** stage by
name and the failure happened anyway, because staging by name bounds what you add and the hazard is
what somebody else added between your `git add` and your `git commit`. The code lane checked for the
lock first, which does not help either - the window is after the check. **It is a
time-of-check-to-time-of-use race on shared mutable state**, and that is why care does not close it.

**Why it costs something.** It has now fired twice - twenty-six lines the first time, twenty-one
this time - and both times the content was correct and the *message* was lost, which is the part
no reader can reconstruct. It also puts a lane's work in a file outside its column, which is the
one invariant the perspectives rest on.

**What removes it is already built, which this lane found only after filing.** `git commit -- <paths>`
implies `--only` and builds from a temporary index. **`hooks/post-commit` exists to support exactly
that**, wired by the code lane in `a60def3` after the quality lens found the residue it leaves. So
there is nothing to build and this item is no longer addressed to that lane.

**What is missing is the instruction.** `pathspec` appears in `hooks/post-commit` and in the quality
lens's own files, and **nowhere in `CLAUDE.md` or `docs/process.md`** - which still say the remedy is
to stage by name. A lane following its instructions exactly gets the failure; the carrier sits there
unused because nothing sends anyone to it.

**This is `P-327`'s own pattern, one week on.** A rule that fires at a moment of confidence needs a
carrier rather than a better sentence - and here the carrier was built and the sentence was never
changed, so the rule still names the remedy that cannot work.

**Measured anyway, and it stands as confirmation rather than discovery.** Three runs in a throwaway
repository plus a live fourth: `50c69ee`'s successor, this lane's own `X-7` commit, was made
`git commit -- lenses/research/ pending.md` with two of another lane's modified files in the tree,
and took only its own four. **The row this lane expected to fail did not** - the `pre-commit` hook's
regenerated `pending.md` still landed, so the objection that a temporary index would break it does
not hold.

**One thing this lane listed as unsettled was already settled**, which is the same error twice in one
item: the `MM` residue is what `hooks/post-commit` handles, and this lane's own commit printed it
doing so. Still untested: `hooks/pre-push` and the padding path, and the lock collision was not
forced to confirm it fails loudly rather than wrongly.

**Whether.** Worth doing now, and it is one sentence rather than a mechanism - which is why it moved
to you. `CLAUDE.md:129` states a remedy that cannot prevent what the bullet above it describes, and
the thing that can is already in the tree. **No text drafted**: the words are Sean's to approve and
yours to write, and whether a lane should be told to commit by pathspec at all is a decision rather
than a typo. **Declining it is reasonable** if he would rather lanes not learn a second git idiom -
the hazard has fired twice in a fortnight with the content intact both times.

## Resolved

### X-1 - what makes the game checkable by hand is never stated

**to** spec · **status** **acted** 2026-09-06 · `480efb4` — `P-317` promoted; the sentence is at `docs/process.md:100` · **raised** 2026-09-06 · **source** [report](2026-09-06-what-the-theory-already-settles.md)

`docs/process.md:91` says the definitions, the transformations and the commands *"are enough to
derive the fourth by hand"*, and rests *how I know the application is right* on that. It is true only
if the transformation reads the state and the commands and nothing else - and the document never says
so. Grepping `determin|replay|nondeterm|random|clock|reproduc` over `docs/process.md` returns **0
hits across 459 lines and 24 headings**, so the zero is against a named population that is not also
zero.

**Why it costs something.** The precondition is load-bearing for the artifact Sean checks by hand,
and it fails silently: a clock, an entropy source or an undeclared file read makes the fourth
artifact underivable while every test still passes and the scenario diff he reviews becomes noise he
cannot tell from a real change. It also decides a question the code lane is about to meet from the
other side - whether the environment a command executes against may answer anything the four
artifacts do not declare.

**Whether.** Worth doing now, and it is one sentence rather than a section. This lens has
deliberately not drafted it: the words are what Sean approves, and they are the specification lane's
to write.

### X-2 - the default in `layers.md` does not satisfy the rule in `turn.md`

**to** spec · **status** **acted** 2026-09-06, re-closed 2026-09-07 · `d44335a` — `docs/layers.md` superseded in that one respect and the hole deferred; `P-338` is what tracks it now · **raised** 2026-09-06 · **source** [report](2026-09-06-what-the-theory-already-settles.md)

`spec/turn.md:24` requires that what settles competing effects be *"a deterministic mechanic of the
game, and therefore something a person wrote and a player can change"*. `docs/layers.md:207` proposes
that where two events collide *"the lower index wins"*, and `:210` makes that reading **the
default**. An array index is deterministic and reproducible, and it is not a mechanic of the game,
nobody wrote it as a rule, and no player can change it.

**Why it costs something.** `docs/layers.md` is the non-normative why-layer, so this is not a
contradiction inside `spec/`. It is worse in one specific way: it is the document the code lane
reads for guidance, it says *default to this*, and following it would build something the
specification forbids. Nothing would report the divergence, because each file is correct on its own
terms.

**Noted alongside, and deliberately not filed as its own item.** The same decision has a second face
in the code - whether a command executes against a shared mutable environment or returns its effects
for a later merge - and settling it in one place and not the other is how it gets made twice,
differently. Recorded here rather than addressed to `code`, because the answer is not determined
until this item is.

**Whether.** Worth doing eventually rather than now. Nothing is being built on it today, and the
question is a real one for Sean rather than a defect to repair.

**Re-closed 2026-09-07, on `S-65`.** This closed into `P-318`, which the specification lane then
withdrew - so the closing line named a proposal nobody decided to drop, and `outbox --orphans`
reported it. **Sean's durability rule is the collision the deferral was waiting for**: a thing
carries a number of turns and paying its upkeep resets it, so a territory short of food resets some
citizens and decrements others, and **which ones eat is exactly the competing effect `spec/turn.md`
requires a deterministic mechanic for.** `P-338` asks him for that rule and is open to him, so the
finding now points at something a reader can follow. **The finding was right and stays right** -
only its tracker moved.

### X-3 - `S-57` answered: the seven cases are six phenomena, and restarting fixes one

**to** spec · **status** **acted** 2026-09-06 · `00bc04b` — the count corrected and an eighth case added; its fourth question is answered by `X-6` · **raised** 2026-09-06 · **source** [report](2026-09-06-answering-from-memory.md) · answers `S-57`

Sorted by what actually failed, the seven cases in `S-57` share no mechanism and **only one is
memory**. **Corrected twice on 2026-09-07, and the second correction is not mine.** This item said four
phenomena; the report said one named by the question and four more, which is five, and the summary
dropped the distinction - a restatement going stale against a source that never moved, and the wrong
number reached the specification lane's close of `S-57`. **Then that lane refuted a case.** `P-315`
was filed here as *the information was there and I did not read it*; its withdrawal says the sentence
was read and misparsed, and `b1d12c9` **rewrote the sentence in the same commit that withdrew the
proposal**. So `P-315` is its own phenomenon and the count is six. `C-35` and `C-34` were re-checked
and hold. The six are: stale context (`P-320`); reading past a clause that was present (`P-315`, `C-35`, `C-34`);
never reading at all (`S-56`); a record destroyed by a process step (`P-310`, `P-312`); and a written
copy going stale on disk (quality's README). Three were verified against git; four are marked
unverified in the report. The item says four lanes and the cases name three.

**The three questions, answered.** *What needs re-reading* - not elapsed time but ownership: every
one of the seven is a claim about an artifact another writer can change, which the lane table already
makes mechanical. *Is any signal available from inside* - no, structurally, since a recalled fact and
a freshly read one occupy identical slots; **but the harness already emits one**, three times in the
session that produced this report, and it is silent for exactly the case where nothing was read at
all. *Does clearing context fix it* - **it fixes one of the four and makes two worse**, because a
fresh instance has no memory to contradict a stale document with. Quality's README is the proof: a
cold reader would have believed it completely.

**Why it costs something.** `docs/process.md` -> Outboxes rests on an instance being replaceable by a
fresh one that reads the files, and that holds only while the files are not themselves stale. The
remedy is not restarting; it is `docs/README.md`'s existing rule to link rather than restate, which
this answer independently arrives at. **So the finding largely confirms policy the project already
has**, and names which policy is doing the work.

**Whether.** Worth reading now, worth acting on only if Sean wants the one-sentence habit in
question 1 written down. **No text drafted** - the words are his to approve and yours to write. The
obvious next study, not started: run a cold instance against the seven questions and measure rather
than classify.

### X-4 - the rule that keeps a lane moving is written for one lane, and the rule binding all of them says stop

**to** spec · **status** **acted** 2026-09-06 · `f810732` — `P-324` promoted; `docs/process.md:205` no longer says a blocked instance stops · **raised** 2026-09-06 · **source** Sean, stating the operating model in a session where his own documents do not carry it

**Where.** `docs/process.md:273`, under *Coding instance*: *files a question ... and carries on with
everything that does not depend on the answer*. `CLAUDE.md:340`, cycle step 9: **the code lane hits a
gap and does not stop**. Those are the only two statements of the rule - two occurrences in
`process.md`, both about code, the second inside the coding instance's own starting prompt.

**What.** `docs/process.md` -> *All lanes* states the opposite default for everyone else: *an
instance ends its turn when it is **blocked**, when it is holding for a stated reason, or when
nothing is open to it.* So the general rule says a blocked lane stops, and only the code lane is told
to file the block and keep going.

**Why it costs something.** Sean described the intended model today: a lane works its backlog through
to the end, files what it notices to the right backlog, and stops only for a decision that actually
blocks the remainder - twenty tasks becoming twenty done, four decisions and three proposals, with
stopping as the exceptional case. That is the *Coding instance* rule generalised, and no document
carries it. **A lane reading only what binds it will stop early and be correct to.**

**It has already fired, in this lane, today.** This lane ended a turn by asking Sean whether to run
the cold-instance study `X-3` calls for, rather than filing it and continuing. Under *All lanes* that
was right; under the model he stated it was not. `X-5` is that question, now filed instead.

**Whether.** Worth doing now, and it is a scoping change rather than a new idea - the words already
exist at `:273` and are addressed to one lane. **No text drafted**: whether the general form belongs
in *All lanes*, and what it does to *ends its turn when it is blocked*, is Sean's to settle and yours
to write.

### X-6 - a known rule is not applied when nothing carries it

**to** spec · **status** **acted** 2026-09-06 · `4d5a081` — filed as `P-327` and promoted; the *Ask what fires a rule* paragraph is at `docs/process.md` -> What makes a check worth having, and the carrier half is the code lane's `C-55` · **raised** 2026-09-06 · **source** [report](2026-09-06-when-a-known-rule-is-not-applied.md) · answers `S-57`'s fourth question

**Answers the question the eighth case added.** *Normalize both sides instead of loosening the
comparison* was proposed, approved and promoted by the specification lane, which then broke it five
times the same afternoon and broke the adjacent heredoc rule while repairing it. **This lane broke
the same heredoc rule hours later**, writing the report for `X-3`, having read it that day - a ninth
case, verifiable in the session.

**Some rules fire at a moment of doubt and some fire at a moment of confidence.** *Name the
population*, *re-poison a check*, *do not take another lane's message as true* are each fired by
something that happens - a number appearing, a list growing, a message arriving - so they survive as
habits. *Normalize both sides* and *write a script to a file* are fired only by remembering, while
the string being written looks correct. **The failure and the confidence are simultaneous**, which is
why no amount of care removes them.

**So the remedy is a carrier rather than a clearer sentence**: a check catches it afterwards and
shares the defect here, because the rule governs how comparisons are written and a check is a
comparison - four of the five broken assertions were in checks. A default path where the rule cannot
be broken is stronger and often free; the heredoc failure was fixed by using a tool whose quoting is
one level by construction, which made the rule unnecessary rather than better remembered. Checked:
`tools/` has no general normalizing comparison, only one incidental use at
`tools/outbox/src/lib.rs:286`.

**The test it yields.** For any rule in `docs/process.md` or `CLAUDE.md`, ask what fires it. If the
answer names something that happens, it can be a habit. **If the answer is *remembering*, it needs a
carrier**, and writing it down more emphatically is not one.

**Whether.** Worth reading now; worth acting on as a code-lane item only if Sean wants the carrier
built. **No text drafted and no carrier designed** - where an anchor-matching helper belongs is the
code lane's, not this lane's.
