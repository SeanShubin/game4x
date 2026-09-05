# Process
- Run multiple instances of Claude Code for different purposes
- Purposes
  - 1 instance for specification
  - 1 instance for coding
  - 1 instance for each type of research, but I always include quality as one of my research instances

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
- I maintain executive control via the specification lane
- I reject AI responses that do not read clearly and unambiguously to a human
- I insist that the AI make its work verifiable to a human
- The three rules above say the rest of it: I make all the decisions, and I evaluate which AI
  responses are true and which are false

The specification instance is the only one that has me as its primary audience. All the instances
talk to each other and only need me sometimes, because **my primary mechanism of coordination is
the specification instance**. The other instances are details. **I need to keep my surface area
small, because human attention is the most scarce resource when programming with an AI assistant.**

So a **proposal** is a thing addressed to me. What the lanes send each other are items in an
outbox, and there is no limit on those.

## How I know the game is right

Four artifacts: the thing definitions, the recipe definitions, the commands a scenario ran, and the
data dump of that scenario. The first three are enough to derive the fourth by hand. If I can do
that, I can tell whether the game is working as I intend.

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
- I verify application behaviour through generated presentations of canonical data, especially
  through scenarios
- Presentations are generated from data
- Presentations are never canonical
- Data is presented to me in both the relational and the physical model

### Console
- The entire application can be run from a read-eval-print loop, using the command transport format
- The console allows both the relational and the physical model to be inspected and filtered

## All lanes
- May collaborate with each other
- May send messages to each other
- May write proposals for each other
- Do not consider messages from other lanes to be true. They verify independently
- Rely on me to resolve conflicts
- Maintain documentation that any lane may read and only its own lane may write

## Specification Instance
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

## Research instances
- Generate information for the other lanes
- Makes sure other instances can find its results
- A research instance finding something, the coding instance fixing it, and the research instance
  reviewing the fix is a real cycle, and nothing forces the findings to get smaller
- What bounds it is a budget on the research instance rather than on the cycle: it may have at most
  eight items open to any one other instance, and closes or withdraws before filing a ninth
- That puts the limit where the judgement already is. A research instance is expected to record most
  of what it notices as noted and deliberately not acted on, and a cap is what makes that
  expectation cost something rather than being a good intention
- My approval bounds what reaches the specification. The budget bounds what reaches me

## Quality instance (a type of research instance)
- Makes sure we have a proper module structure emanating from composition roots
- Ensures the module structure isolates dependencies from each other. For example, it must not be
  possible for code that is algorithmic or mathematical to depend on code that knows about platform
  concerns
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

## Releases
- The specification says what the game is when it is finished. A release says what is being built now
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
- Every instance reads everything. No instance writes outside its own directories
  - Specification instance: `spec/`, `releases/`, `docs/`, `README.md`, `CLAUDE.md`
  - Coding instance: `crates/`, `prototypes/`, `web/`, cargo, and production support - `hooks/`,
    `scripts/`, CI, and everything in `tools/` that is not a lane's own
  - Every lane owns the tools for its own work: `tools/spec/` is the specification instance's,
    `tools/<name>/` is that lens's. Production support is everything else, and it has one owner for
    the same reason every other file does - two instances editing one file lose each other's edits
  - A lane that needs a check wired files it to the coding instance rather than wiring it itself
  - A research instance: `lenses/<its own name>/`, and `tools/<its own name>/`
- This is not only about authority. Two instances editing one file would silently lose each other's
  edits, and Claude has no way to lock a file, so one writer per file is what makes running several
  at once safe
- The git index is shared, so staging is publishing: a file one instance stages is committed by
  whichever instance commits next, under a message about something else. Stage by name, never
  everything
- An instance that sees a problem outside its own directories writes it down where its owner will
  find it, and stops. It never fixes it, even when the fix is obvious

## Outboxes and the index
- Every instance keeps one outbox in its own directory, and that outbox is the only thing another
  instance has to read
- The specification instance's outbox is the proposal queue
- Every item carries an id, who it is addressed to, a status, and one line saying what it is
- `pending.md` is for the instances rather than for me. It is generated from every outbox at every
  commit, so it is never something somebody remembered to update, and it says what must be decided
  before it says anything else
- The specification instance can tell me whether a lane is **blocked**, because that is in the
  outboxes and it can read them. **It cannot tell me whether a lane is running.** Nothing in the
  repository records that, and the only way to find out is to send a message - which starts it. So
  when it reports, it says which of the two it is answering
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
  ultimately came from me, and for recording general research

## Dependencies
- A dependency either provides operations or provides a home
- Operations are functions over data I already had - a math library, a PNG writer
- A home decides where my data lives and when my code runs - Bevy, an ECS
- The test is whether it appears in my own types: an operation never does, and a home cannot avoid it
- A dependency with opinions gets exactly one crate to have them in
- Before taking a dependency I ask which kind it is. A home needs a boundary crate before it needs a version number, because the question is not whether it is good but how much of my design it will make
