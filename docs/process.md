# Process
- Run multiple instances of Claude Code for different purposes
- Purposes
  - 1 instance for specification
  - 1 instance for coding
  - 1 instance for each type of research, but I always include quality as one of my research instances

## Specification Instance
- I have Claude generate proposals for changes to the specification
- I work with claude to make sure I approve the exact text of the proposals
- Once I directly confirm the proposal matches my intent I promote it
- Nothing gets into the specification without my direct approval
- An approved proposal leaves the queue, and a one-line row stays in a ledger saying what
  was approved and where it landed. That ledger is what stops the same idea being proposed
  again a month later
- The main documents I consume as a human are
  - `pending.md`, which is where I start: what must be decided, then what is outstanding
  - the proposals, where I either approve them or tell claude what to change
  - the specification, especially the invariants
- a proposal is not done until it is committed, and Claude commits it without being asked
- pushing is not part of done - I decide when to push, partly because the branch is shared and a push carries the other instances' local commits too
- I have no preference between one commit per proposal and several proposals in one commit
- The specification instance is the only one that writes a proposal. Other instances raise things by addressing them to it, and it decides what becomes a proposal and what does not. That keeps my queue to one author and one length, which is what makes it reviewable.

## Coding instance
- Implements what the specification requires, and what a research instance proposes
- The specification is a constraint rather than a work list. It is what keeps the coding instance
  from going off the rails, and a research instance is free to work within it
- Decides how to implement any of that when it can
- Files a question when it needs human input on technical details, addressed to the specification instance, and carries on with everything that does not depend on the answer
- The coding instance does not have to be acting on something I said directly, but there must be a traceable path back to me
- Quality is one such path: I approved the lens and what it looks for, so a finding it raises and the coding instance acts on traces back to me through that
- Every commit cites the id of the item it acts on, which is what makes the path checkable rather than assumed

## Research instances
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
- Tries to maximize the separation between generic code and code with dependencies
- Where a dependency provides a home rather than operations, makes sure it is confined to one
  crate - see Dependencies below

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
  - Coding instance: `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`, `hooks/`, CI, cargo
  - A research instance: `lenses/<its own name>/`, and nothing else
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
- `pending.md` is the one place I go to see what is outstanding. I open the file rather than run a
  command: running the command still leaves me reading the result, so it is two things where
  opening the file is one
- It is generated from every outbox at every commit, so it is never something somebody remembered
  to update, and it says what must be decided before it says anything else
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

## Dependencies
- A dependency either provides operations or provides a home
- Operations are functions over data I already had - a math library, a PNG writer
- A home decides where my data lives and when my code runs - Bevy, an ECS
- The test is whether it appears in my own types: an operation never does, and a home cannot avoid it
- A dependency with opinions gets exactly one crate to have them in
- Before taking a dependency I ask which kind it is. A home needs a boundary crate before it needs a version number, because the question is not whether it is good but how much of my design it will make
