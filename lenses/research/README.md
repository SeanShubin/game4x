# Research

**Derived.** Written by the research instance. Not binding - a finding is a claim about the tree,
not a decision about it. Sean decides what is acted on.

[Root README](../../README.md) · [Process](../../docs/process.md) · [Specification](../../spec/README.md)

This directory is the research lane's only writable place, along with `tools/research/`. No other
lane writes here - see [the lane table](../../CLAUDE.md#perspectives).

## What bounds this lens

[`docs/process.md`](../../docs/process.md) is Sean's own statement and is authoritative. What this
lens is for is under
[Research instances](../../docs/process.md#research-instances); what every finding has to carry, and
how few of them should matter, is under [`CLAUDE.md` -> Starting a new
lens](../../CLAUDE.md#starting-a-new-lens). **None of it is restated here**, because restating is
what went stale in the sibling lens's README - a copy close to the source's words was wrong four days
later, and closeness is not what keeps a copy current.

## Its focus

**It takes this project's home-grown abstractions to the established literature, and reports what
that literature already settles or already warns about.**

That is the class neither producer reaches. The code lane reads the specification as instructions;
the specification lane records what Sean says. Neither goes and asks *who else has solved this, and
what did they learn the expensive way*.

The output is usually terminology and a named failure mode, not a defect. **Its value is diagnostic**:
naming an abstraction correctly is what makes the gap beside it visible. `X-1` was found that way -
the theory of state functions was the route to it and appears nowhere in the finding.

## What it does not do

**It does not import theory into the specification.** A vocabulary that helped this lens reach a
finding has not thereby earned a place in Sean's documents, and `CLAUDE.md` names the trap directly:
*measuring something is not a reason to specify it*. Three turns of reading produce at most a
sentence, and usually none.

It is also **forward-looking rather than immediate**, per `docs/process.md`. Work that is not ready
is addressed to nobody, lives in a dated report, and costs no one any attention until this lane
gives it a reader.

## The outbox

**[`outbox.md`](outbox.md) is the only file another perspective has to read.** A report carries the
argument; the outbox carries the claim, its reader and its state.

Discovery is structural: `tools/outbox` walks `lenses/`, so an outbox here appears in `pending.md`
at the next commit. Nothing is registered anywhere.

## Naming

One file per report, dated: `2026-09-06-what-the-theory-already-settles.md`. Reports are records of
a moment and go stale - a superseded one says so at the top rather than being deleted, so a later
reader can tell whether a finding was acted on or merely forgotten.

## Reports

Newest first.

- [Answering from memory, and why the instance cannot tell](2026-09-06-answering-from-memory.md)
  - 2026-09-06. Answers `S-57`. The seven cases are four phenomena; only one is memory, and
    clearing context fixes that one while making two of the others worse.
- [What the theory already settles](2026-09-06-what-the-theory-already-settles.md)
  - 2026-09-06. From a conversation with Sean about the state function. The transformation is a
    Mealy machine and the list form is derived rather than new; `exec(environment)` is
    defunctionalization. Both findings fell out of the vocabulary and neither one needs it.
