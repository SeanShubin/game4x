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

### X-1 - what makes the game checkable by hand is never stated

**to** spec · **status** open · **raised** 2026-09-06 · **source** [report](2026-09-06-what-the-theory-already-settles.md)

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

**to** spec · **status** open · **raised** 2026-09-06 · **source** [report](2026-09-06-what-the-theory-already-settles.md)

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

### X-3 - `S-57` answered: the seven cases are four phenomena, and restarting fixes one

**to** spec · **status** open · **raised** 2026-09-06 · **source** [report](2026-09-06-answering-from-memory.md) · answers `S-57`

Sorted by what actually failed, the seven cases in `S-57` share no mechanism and **only one is
memory**: stale context (`P-320`); reading past a clause that was present (`P-315`, `C-35`, `C-34`);
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

**to** spec · **status** open · **raised** 2026-09-06 · **source** Sean, stating the operating model in a session where his own documents do not carry it

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

### X-5 - the measurement `X-3` lacks needs an instance nobody has authorised

**to** spec · **status** open · **raised** 2026-09-06 · **source** [report](2026-09-06-answering-from-memory.md)

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

## Closed

Nothing yet.
