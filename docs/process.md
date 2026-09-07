# Process
- Run multiple instances of Claude Code for different purposes
- Purposes
  - 1 instance for specification
  - 1 instance for coding
  - 1 instance for quality
  - 1 instance for research

## What this document has to be

This document has to be enough on its own. If I lose every transcript, every note, and every
`CLAUDE.md` in the repository, what is here has to be enough to rebuild the process and start the
instances again.

Two things follow, and the second is the one that gets forgotten. **An insight that lives only in
a conversation, a note, or an operating file is lost**, so a rule worth keeping is written here.
And **the reason a rule exists is part of the rule** - one recorded without its reason survives as
a ritual, and the first person to find it inconvenient deletes it correctly, for the wrong reason.

**And a reason that is false is worse than one that is missing.** A rule with no reason survives
as a ritual; a rule with a wrong reason is kept for something that will not hold, and nothing
looks wrong from outside. The decision is right and the code is right - only the account of why is
false, so no check reaches it, and the person it misleads is the one who came back because they
were unsure.

**State the kind of failure and why it happens; do not narrate the occasion.** *This misfires, and
here is what makes it misfire* keeps working. *This misfired on Tuesday, to a lane, at fifteen*
is a fact about a moment - a reader cannot check it, does not need it, and is reading a document
that is supposed to be true whenever it is opened. **The reason is part of the rule and the
anecdote is not.**

## Three rules for using AI assistants

- Never let AI make a decision
- Never blindly trust anything an AI tells you
- Have a place you can look to tell what the AI did

The third one is the data dump in this case.

## What I read, and what I do

- From the specification lane, I read and approve proposals, and I make sure the specification is
  comprehensible to me
- From the coding lane, I read and verify the input and the expected data from the scenario test
- Generally I do not read anything but the proposals from the specification lane, the specification
  itself, and the scenario tests from the coding lane

**Three artifacts are addressed to me**: the specification proposals, the specification questions,
and the scenario test's input and expected data. Human attention is the most scarce resource when
programming with an AI assistant, so each of the three is as concise as it can be made.
**Concision is bounded by control, not the other way round** - each must stay precise and detailed
enough that I keep executive control of what it decides. One short enough to read and too vague to
govern has failed at the thing it was shortened for.

**Say what you are referring to, not where it was.** *The fourth*, *the first three*, *the
former*, *the latter*, *the above* - an assistant resolves these without effort and I cannot
resolve them at all on one read, which is why they get written and never noticed. **The test is
whether resolving the reference needs counting or remembering an order.** A pronoun for the thing
just named needs neither and is fine; anything that does gets the thing's name instead.

- I maintain executive control via the specification lane
- I reject AI responses that do not read clearly and unambiguously to a human
- I insist that the AI make its work verifiable to a human
- The three rules above say the rest of it: I make all the decisions, and I evaluate which AI
  responses are true and which are false

The specification instance is the only one that has me as its primary audience. All the instances
talk to each other and only need me sometimes, because **my primary mechanism of coordination is
the specification instance**. The other instances are details. **I need to keep my surface area
small, because human attention is the most scarce resource when programming with an AI assistant.**

**Two files are addressed to me and they hold different things.** `docs/notes/decisions.md` holds
choices only I can make; `docs/notes/proposals.md` holds words for me to approve. An item lives in
one at a time - it sits in the decisions file while any question in it is unanswered, and moves to
the proposals file when the last one is answered. Questions about the specification's own content
stay at the bottom of the `spec/` file they concern, where the context is.

**The limit of fifteen is a tripwire on the specification instance, not a bound on my reading.**
Its job is never to be reached. If it is ever reached, proposals are being filed faster than I am
deciding, which means guessing at design, and the remedy is to ask one question instead of filing
ten guesses. **So an unreached limit is the limit working, and is not evidence that it is
unnecessary.**

So a **proposal** is a thing addressed to me. What the lanes send each other are items in an
outbox, and **there is no limit of my kind on those** - my fifteen is about my reading, and what
bounds theirs is under *All lanes*, where it is not a count.

**A proposal asks me one of two things, and says which. Approval** means the words are final and
reading them is the whole of the work. **A decision** means a choice is open, and no wording can be
final until I make it.

**Answering the questions in a proposal is not promoting it**, and a proposal with a question in it
cannot be promoted. My answer turns it into one that asks approval, which is a rewrite I read
before I promote it.

**What I approve is what ships** - text byte for byte, a table's rows cell for cell, and an
instruction run with the check it carries. **That is what makes reading it worth my time.**

## How I know the application is right

Four artifacts: the definitions of the things, the definitions of the transformations over them,
the commands a scenario ran, and the data dump of that scenario. **The definitions and the
commands are enough to derive the data dump by hand.** If I can do that, I can tell whether the
application is behaving as I intend.

**The transformation reads the state and the commands and nothing else** - no clock, no
randomness, nothing about the machine it runs on. That is what makes the data dump derivable from
the definitions of the things, the definitions of the transformations, and the commands a scenario
ran. **It fails silently if it stops holding**: every test still passes, and the diff I review by
hand becomes noise I cannot tell from a real change.

The scenario test reads the data files for its input, reads the data files for what is expected,
computes what actually happens, and compares. **The input and the expected are what I review by
hand for correctness.** Once I have confirmed them, the test locks them in place - **not so that
they cannot change, but so that my changing my mind can be told apart from something slipping in
by accident.**

When I change my mind, I delete the expected data and run the scenario again. **Absent expected
data means I accept what it does now**, so the test writes it, and what I review is the diff in
version control. Nothing else may write it: an expectation that can be edited in place is one that
can be edited by accident, which is the thing it exists to prevent.

A test is there for what I cannot reliably repeat. **I can remember to do a thing the first time;
what I cannot do is remember a mundane check every time after that**, and that is what a failing
test is for. So a check earns its place by guarding the repetition, not the one-off.

There is a **main scenario** that a person can follow end to end. **That is what I vet, and it is
the foundation.** I do not need to walk every step of the end-to-end path by hand.

As long as that foundation is maintained, other scenarios check that particular end states are
reachable. **Those rest on the foundation rather than on me**, and they will make heavy use of the
automation. **What the main scenario has to cover is a fact about the application and is stated with it.**

## What verification requires

### State
- Must be some form of this function, depending on the underlying architecture:
  `(old-state, commands) -> (new-state, effects)`
- Effects may be omitted
- May have a single command instead of a list of commands
- May be slightly different to account for parallelism and concurrency

### Commands
- Must have a transport format that can be read by both human and machine
- Must support structured data, including nesting
- May be a custom format

### Data model
- Must be presented as relational
- The logical model is relational; the physical model is up to the coding instance, which should
  choose an appropriate implementation, and that may or may not be relational

### Scenario
- Starts from an empty model
- Creates the initial state with commands
- Runs the scenario with commands
- Verifies the resulting model

### Presentation
- Canonical data exists in a data format, not a presentation format and not code
- I verify application behavior through generated presentations of canonical data, especially
  through scenarios
- Presentations are generated from data
- Presentations are never canonical
- Data is presented to me in both the relational and the physical model

### Console
- The entire application can be run from a read-eval-print loop, using the command transport format
- The console allows both the relational and the physical model to be inspected and filtered

## What makes a check worth having

A check has two ways to be worthless, and the second is how you get the first.

**It can be unable to fail.** Zero occurrences counted against a population that is also zero. A
coverage check that reads the commands in a file rather than what ran. A guard that permits what
its rule forbids, because the same hand wrote the rule, the work and the guard.

**It can fail when nothing is wrong** - a comparison broken by a line wrap, a table's padding, a
capital letter. **That is the more dangerous one, because the fix that comes to hand is to loosen
it**, and a loosened check is the first kind.

**So normalize both sides instead of loosening the comparison.** Trim, sort, reformat, parse a
table to its cells - whatever makes two forms of one thing identical - and then compare strictly.
**A check that cannot raise a false alarm is one nobody has a reason to weaken.**

**And a check written by the hand that wrote the work will agree with the work.** That is what a
review lane is for, and it is why an independent finding is worth more than a count of findings.

**And a check can be sound and no longer believed.** A long green run is what does it, and **a
growing list of recorded exceptions is where belief decays fastest** - every entry is a reason it
stayed green, so *it stayed green* stops carrying information, and those lists grow on exactly the
checks guarding what people keep getting wrong. **A new check is made to fail on demand before it
is trusted; an old one never is**, so what is trusted after the first run is a memory of it.
**Re-poison a check when its exception list grows.**

**And a probe aimed where a check already looks can only confirm what already works.** Making a
new check fail on demand is the first half; the second is choosing what to make it fail on. A
poison inside the region the predicate already sees goes red for the right reason and says nothing
about the region it does not - and it reads exactly like evidence.

**Two counts that share a computation are one count.** A check that asserts its population and
finds the number its own specification supplied has corroborated nothing. That is the check
written by the hand that wrote the work, arriving by a different route.

**Ask what fires a rule.** Some fire at a **moment of doubt** - a number appears, a list grows, a
message arrives from another lane - and something in the work announces that the rule applies.
**Those survive as habits.** Some fire at a **moment of confidence**, while what is being written
looks correct, and nothing announces anything. **Those do not survive as habits at all**, however
well they are written or however recently they were read, because the failure and the confidence
are the same instant. **A rule of the second kind needs a carrier** - a check that catches it
afterwards, or better a default path on which it cannot be broken.

## All lanes
- May collaborate with each other
- May send messages to each other
- May write proposals for each other
- Do not consider messages from other lanes to be true. They verify independently
- Rely on me to resolve conflicts
- Maintain documentation that any lane may read and only its own lane may write

An instance ends its turn when **everything left waits on somebody else**, when it is holding for
a stated reason, or when nothing is open to it. **Being blocked on one thing is not being
blocked**: it files the question where its reader will find it, says what it assumed in order to
continue, and goes on to everything that does not depend on the answer. **Never merely because it
has just reported** - reporting is something done on the way past, not a place to stop.

And a reply to a report **ends with the next thing to do, or says plainly that there is nothing**. A
reply that only acknowledges wakes an instance, gives it nothing, and it stops again.

A lane that is waiting **files it**, addressed to the lane it is waiting on, saying what it is
waiting for. The lane that finishes that thing tells it. A hold that lives only in a message is gone
when the session ends, and nobody can see who is waiting on what.

- **Two different limits, and only one of them is a count.** Mine is a count - fifteen open
  proposals - and what it is actually for is under *What I read, and what I do*
- **What bounds an instance's own outbox is not a count**, because neither thing that makes a
  backlog expensive grows with the number of items. An item goes **out of date** as what it cites
  changes, however few of them there are, and two items **conflict** as a pair, however many
  others sit beside them. A count is a proxy for both and a good measure of neither
- **Counting the two together misfires**, in the direction that costs most: one number cannot tell
  a reading budget from a producer's backlog, so a queue of mine that is empty can be reported as
  full
- So an instance may not file a new item while one of its own is open and cited by a commit saying
  it is done. **It closes the item the commit cites**, or records the hash to say it looked and it
  is still open. That is the same forcing function a cap gives - close something before filing
  something - attached to the cost that is actually there
- An item whose cited file has taken a promotion since it was raised is re-read before it is
  relied on. The ground moving under an item is what makes it wrong without anybody touching it
- Eight items open to any one instance stays, as a backstop rather than as the rule. An outbox
  nobody reads through is a real cost, only a second one. An instance is still expected to record
  most of what it notices as noted and deliberately not acted on

## Specification Instance
- **Maintaining my executive control is what this instance exists for, and that is what decides
  what it owns.** Anything in the surface I vet belongs to it: the proposals, the decisions, and
  the scenario's commands and expected data. **The coding instance's job is to follow the
  specification**, and what it owns is how, never what I see.
- I have Claude generate proposals for changes to the specification
- I work with claude to make sure I approve the exact text of the proposals
- A proposal clearly indicates which text in it is destined for the specification
- Once I directly confirm the proposal matches my intent I promote it
- Nothing gets into the specification without my direct approval
- An approved proposal leaves the queue, and a one-line row stays in a ledger saying what
  was approved and where it landed. That ledger is what stops the same idea being proposed
  again a month later
- Two things necessarily require my attention, and everything else has to earn its place against
  them: **the proposals are where I create, and the scenario test is where I validate**
- The main documents I consume as a human are
  - the proposals, where I either approve them or tell claude what to change
  - the scenario test's input and expected data, which I check by hand
  - the specification, especially the invariants - which I read while approving a proposal rather
    than as a separate errand
- a proposal is not done until it is committed, and Claude commits it without being asked
- pushing is not part of done - I decide when to push, partly because the branch is shared and a push carries the other instances' local commits too
- I have no preference between one commit per proposal and several proposals in one commit
- The specification instance is the only one that writes a proposal. Other instances raise things by addressing them to it, and it decides what becomes a proposal and what does not. That keeps my queue to one author and one length, which is what makes it reviewable.
- It also writes proposals addressed to the other lanes
- Telling the specification instance what to change is not a shortcut into the specification. It
  changes the proposal and shows me the result, and the words land only when I then approve them
- It is the primary coordinator between the instances. They may talk to each other directly, and
  it is this one's job to see that they are all on the same page
- It can tell me the status of every other instance, what needs to be done, and what is waiting
  on me

## Coding instance
- Implements what the specification requires, and what a research instance proposes
- The only lane allowed to touch production code
- Pipeline, automation and tooling are not production code. They are production support
- The specification is a constraint rather than a work list. It is what keeps the coding instance
  from going off the rails, and a research instance is free to work within it
- Decides how to implement any of that when it can
- Files a question when it needs human input on technical details, addressed to the specification instance, and carries on with everything that does not depend on the answer
- The coding instance does not have to be acting on something I said directly, but there must be a traceable path back to me
- Quality is one such path: I approved the lens and what it looks for, so a finding it raises and the coding instance acts on traces back to me through that
- Every commit cites the id of the item it acts on, which is what makes the path checkable rather than assumed
- The specification constrains the observable behavior of production code. The implementation
  details, the tooling, the pipeline, the deployment and the rest of production support are this
  instance's own decisions
- It produces the second artifact I review by hand - the input and the expected data for the
  scenario test, described under *How I know the application is right*
- It notifies the specification instance of its status
- After any significant change to production code, it asks the quality instance for a review
- When a quality review is wrong, it says so, and gives the quality instance enough information
  to understand why

## Research instances
- Takes on one-off research projects that might be useful in the future
- This is forward thinking rather than a matter of immediate concern
- Makes its research discoverable by the other instances, against the time they come to need it
- A research instance finding something, the coding instance fixing it, and the research instance
  reviewing the fix is a real cycle, and nothing forces the findings to get smaller

## Quality instance
- Is in charge of making sure the code stays easy to maintain
- Makes sure we have a proper module structure emanating from composition roots
- Ensures the module structure isolates dependencies from each other. A utility that is a pure
  mathematical transformation knows nothing about a graphics card, even if the computation is
  only usable by the graphics card
- Ensures a separate, thin module drives dependencies via composition roots, in a way that ensures
  no implementation of one thing knows about the implementation details of another thing
- Tries to maximize the separation between generic code and code with dependencies
- Where a dependency provides a home rather than operations, makes sure it is confined to one
  crate - see Dependencies below
- Makes sure we have automation in our process instead of repetition, which means a proper automated
  pipeline and proper support tooling
- Makes sure that where possible rules are enforced via reliable code rather than relying on human
  or AI habits

Every one of these is something the quality instance **finds and reports**. It builds none of them:
production support is the coding instance's, so quality says what is missing and the coding instance
wires it.

## Starting the instances

**These prompts are not the process; they are what starts it.** They are here because a process
that needs a chat transcript to restart is not repeatable. **Each points at a document rather than
restating one**, so none of them goes stale when the document it points at changes.

The specification instance:

```
You are the specification lane. Read CLAUDE.md, then spec/README.md,
docs/notes/proposals.md and docs/notes/spec-backlog.md.

CLAUDE.md -> Perspectives says what you write and what you read. Your outbox is
docs/notes/proposals.md, and it is the only outbox addressed to me.

Start by telling me what is open and addressed, read from the files rather than
remembered.
```

The coding instance:

```
You are the code lane. Read CLAUDE.md, then releases/first-release.md and
crates/outbox.md.

CLAUDE.md -> Perspectives says what you write and what you read. Your work is what is
open and addressed to you, and `pending.md` lists it - it is generated from every
outbox at every commit, so it cannot go stale the way a list in a prompt can.

When the specification does not say something, do everything that does not depend on
the answer, file a question to spec stating the assumption you proceeded under, and
carry on.

Start by telling me what is open and addressed to you, read from the files rather than
remembered.
```

The quality instance:

```
You are the quality lens. Read CLAUDE.md, then lenses/quality/README.md and
lenses/quality/outbox.md.

CLAUDE.md -> Perspectives says what you write and what you read, and -> Starting a new
lens says what every finding has to say and how few of them should matter. You never
edit what you review, and you never run cargo fmt, cargo fix or clippy --fix, because
they modify the files you are judging.

Your outbox is lenses/quality/outbox.md. Each finding carries an id, a to, a status and
one line, and points at a dated report that carries the argument.

Start by telling me what is open and addressed, read from the files rather than
remembered.
```

The research instance:

```
You are the research lens. Read CLAUDE.md, then docs/process.md ->
Research instances.

CLAUDE.md -> Perspectives says what you write and what you read. You write
lenses/research/ and tools/research/, and nothing else. You never edit what
you review.

Your outbox is lenses/research/outbox.md. Each finding carries an id, a to,
a status and one line, and points at a dated report that carries the
argument. Research that is not ready is addressed to nobody.

Start by telling me what is open and addressed, read from the files rather
than remembered.
```

**A new lens is started from `CLAUDE.md` -> Starting a new lens**, which is where the question a
lens has to answer before it is worth starting lives. The four above are the ones that exist.

## Releases
- The specification says what the application is when it is finished. A release says what is being built now
- A release never invents a rule. If it needs one the specification lacks, that becomes a proposal
  first and the release then refers to it
- Approving a proposal fills in the destination. It is the release that orders work, because most of
  the specification is not buildable at the moment it lands
- Each capability in a release is one observable line - what has to be visibly true for it to count -
  addressed to the coding instance
- The coding instance does not mark its own capability done. It reports the evidence, and the
  specification instance records it, so the account of what has been delivered is not kept by whoever
  built it
- A release file is deleted once everything in it has been vetted

## Who writes what
- Every instance reads everything. No instance writes outside its own directories, and
  [`CLAUDE.md`](../CLAUDE.md) says which those are
- Every lane owns the tools for its own work. Production support is everything else, and it has one
  owner for the same reason every other file does
- A lane that needs a check wired files it to the coding instance rather than wiring it itself
- This is not only about authority. Two instances editing one file would silently lose each other's
  edits, and Claude has no way to lock a file, so one writer per file is what makes running several
  at once safe
- The git index is shared, so staging is publishing: a file one instance stages is committed by
  whichever instance commits next, under a message about something else. Stage by name, never
  everything
- An instance that sees a problem outside its own directories writes it down where its owner will
  find it, and stops. It never fixes it, even when the fix is obvious
- A promotion either files what it creates as an item addressed to the lane that must do it, or
  records that it is not work for that lane. **Never silence**, because silence and *nobody has
  looked yet* are the same bytes
- It files it **before the proposal is deleted**. A promotion removes the document that named the
  work, so a promise to file later goes with it and nothing is left to notice

## Outboxes and the index
- Every instance keeps one outbox in its own directory, and that outbox is the only thing another
  instance has to read
- The specification instance's outbox is the proposal queue
- Every item carries an id, who it is addressed to, a status, and one line saying what it is
- **An item that cannot be acted on yet says what it waits on in a field, never in prose** -
  `**waits on** P-n`. **When that id is no longer open the wait is over**, and whatever lists the
  outboxes says so. A hold written into a paragraph is invisible to every tool and to every reader
  who does not re-read the whole item, so an item goes on telling a lane not to start work that is
  no longer blocked.
- **An item closes when the instance it is addressed to has done what it can**, not when the thing
  it reports is finally fixed. Waiting for the fix keeps items open longer, and age is what makes
  one go out of date or contradict another - so the rule that looks like it protects against
  losing things is the one that causes the trouble
- **A closing item names what now tracks the thing it reported**, so the chain can be followed.
  Without that, closing on being routed just means closing
- **And a withdrawal that would orphan a closed item does not silently drop it.** The lane
  withdrawing the proposal files the reopening as an item addressed to whoever owns the closed
  one, in the same commit as the withdrawal - the same rule as a promotion that makes something
  else stale. A decision of mine not to do something is recorded and is not a thing lost; a
  proposal Claude withdraws is neither, so an item that closed into one would otherwise go quiet
  with nobody having decided anything
- `pending.md` is for the instances rather than for me. It is generated from every outbox at every
  commit, so it is never something somebody remembered to update, and it says what must be decided
  before it says anything else
- The specification instance can tell me whether a lane is **blocked**, because that is in the
  outboxes and it can read them. It can also tell me whether a lane is **running**, because it can
  list the other sessions without starting them. What neither answers is what a running lane is
  doing between commits, so when it reports it says which of the three it is answering
- `scripts/outbox.ps1` answers the same question from a terminal, and filters by who an item is
  addressed to. That is mostly for the instances rather than for me
- If nothing anywhere is open and addressed, then nothing any instance knows to be wrong is
  unattended. That is a promise about the outboxes, not about the code
- Every decision that needs me is an item in a file. Claude does not put one to me in a reply, where
  it is invisible to `pending.md` and gone when the session ends
- It should not be possible to have work outstanding that needs nobody's decision and is not being
  worked on. I cannot enforce that - an instance only runs when I start it, so I am the scheduler -
  but I must be able to see it, which means an item that is waiting on a person says so
- A contradiction goes into an outbox the moment it is found - never a paragraph in a reply, and
  never a line in a note nobody reads

## Claude bookkeeping
- Claude manages relevant history regarding how the specification came to be in many documents that a human will never look at
- That documentation is not meant for me. It is for remembering the history of how decisions
  ultimately came from me, for recording general research, and for the context an assistant needs
  in order to interpret what I say correctly. There is a lot of that, I never need to look at it,
  and it has to be organized so that an assistant finds the right part when it becomes relevant

## Dependencies
- A dependency either provides operations or provides a home
- Operations are functions over data I already had - a math library, a PNG writer
- A home decides where my data lives and when my code runs - Bevy, an ECS
- The test is whether it appears in my own types: an operation never does, and a home cannot avoid it
- A dependency with opinions gets exactly one crate to have them in
- Before taking a dependency I ask which kind it is. A home needs a boundary crate before it needs a version number, because the question is not whether it is good but how much of my design it will make
