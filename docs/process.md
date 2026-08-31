# Process
- Run multiple instances of Claude Code for different purposes
- Purposes
  - 1 instance for specification
  - 1 instance for coding
  - 1 instance for each type of research, but I always include quality as one of my research instance

## Sepcification Instance
- I have Claude generate proposals for changes to the specification
- I work with claude to make sure I approve the exact text of the proposals
- Once I directly confirm the proposal matches my intent I promote it
- Nothing gets into the specification without my direct approval
- Propoals are removed once I approve them
- The main documents I consume as a human are
  - the proposals, where I either approve them or tell claude what to change
  - the specification, especially the invariants
- a proposal is not done until it is committed, and Claude commits it without being asked
- pushing is not part of done - I decide when to push, partly because the branch is shared and a push carries the other instances' local commits too
- I have no preference between one commit per proposal and several proposals in one commit
- The specification instance is the only one that writes a proposal. Other instances raise things by addressing them to it, and it decides what becomes a proposal and what does not. That keeps my queue to one author and one length, which is what makes it reviewable.

## Coding instance
- Brings code in line with the specification
- Decides how to implement the specification when it can
- Files a question when it needs human input on technical details, addressed to the specification instance, and carries on with everything that does not depend on the answer
- The coding instance does not have to be acting on something I said directly, but there must be a traceable path back to me
- Quality is one such path: I approved the lens and what it looks for, so a finding it raises and the coding instance acts on traces back to me through that
- Every commit cites the id of the item it acts on, which is what makes the path checkable rather than assumed

## Research instances
- Makes sure other instances can find it's results
- We have to figure out how to make sure the quality research instance gets acted on without creating an infinite loop

## Quality instance (a type of research instance)
- Makes sure we have a proper module structure emanating from composition roots
- Tries to maximize the separation between generic code and code with dependencies
- Where dependencies are 

## Claude bookkeeping
- Claude manages relevant history regarding how the specification came to be in many documents that a human will never look at

## Dependencies
- A dependency either provides operations or provides a home
- Operations are functions over data I already had - a math library, a PNG writer
- A home decides where my data lives and when my code runs - Bevy, an ECS
- The test is whether it appears in my own types: an operation never does, and a home cannot avoid it
- A dependency with opinions gets exactly one crate to have them in
- Before taking a dependency I ask which kind it is. A home needs a boundary crate before it needs a version number, because the question is not whether it is good but how much of my design it will make
