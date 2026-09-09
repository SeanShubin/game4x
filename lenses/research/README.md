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

- [`deploy ark`, worked, and the seven rows that are already duplicated](2026-09-08-deploy-worked.md)
  - 2026-09-08. Carries `X-12`. `deploy ark` and `found by land` share seven rows verbatim, which
    is the first real call site for nesting; deriving the territory deletes a row and a `Where`;
    and a soft garrison threshold silently decides five other rows.

- [Least expressive yet complete, and the test for whether a primitive earns its place](2026-09-08-least-expressive-yet-complete.md)
  - 2026-09-08. Carries `X-11`. A primitive earns its place when removing it moves the explosion
    from the generated space into the authored one; the four roles are two dimensions, a change
    and a threshold, and `create-if-missing` is a cell the grid always had.

- [The menu is an information channel](2026-09-08-the-menu-leaks.md)
  - 2026-09-08. Carries `X-10`, **whose main claim Sean refuted the same day** - the information-set
    condition is definitional and restricts no game. What stands: a seed in the state beats
    nature-as-a-player because it keeps the dump derivable, one generator leaks across subsystems,
    and the metric for re-representation must be chosen before the measurement.

- [Simple, finite, and still decidable: where the cliff edges are](2026-09-08-simple-finite-and-decidable.md)
  - 2026-09-08. Carries `X-9`, for the recipe structure Sean is replacing. A recipe network is a
    Petri net and `limit 0` is an inhibitor arc; nesting is HTN and recursion is undecidable;
    boundedness is the line that saves the first and acyclicity the second. Citations checked in
    session rather than recalled.

- [One fact, a console that types and an interface that selects](2026-09-08-one-fact-two-interfaces.md)
  - 2026-09-08. Carries `X-8`, answering `C-74`. Grounding is the one question behind all three
    of its puzzles; `create-if-missing` is a conditional effect, noun-verb is the Xerox Star, and
    a rule table and a selection table are IDB and EDB - different predicates, not one table
    twice. Citations checked in session rather than recalled.

- [The staging rule cannot prevent the thing it describes](2026-09-07-the-shared-index-race.md)
  - 2026-09-07. Carries `X-7`. Staging by name bounds what you add and the hazard is what someone
    else added; a path-limited commit removes it, and the hook that regenerates `pending.md`
    survives the change - measured, not argued.

- [The cold-instance study, designed and not run](2026-09-07-the-cold-instance-study.md)
  - 2026-09-07. Prepares `X-5`. Five of the eight cases are runnable against a cold instance and
    three are not; the cheap design measures findability rather than finding; the predictions are
    written before the run, and two of the four tasks can refute `X-3` rather than confirm it.
- [When a rule is known and still not applied](2026-09-06-when-a-known-rule-is-not-applied.md)
  - 2026-09-06. Answers `S-57`'s fourth question. Some rules fire at a moment of doubt and survive
    as habits; the ones that fire at a moment of confidence need a carrier, not a clearer sentence.
- [Answering from memory, and why the instance cannot tell](2026-09-06-answering-from-memory.md)
  - 2026-09-06. Answers `S-57`. The cases are seven phenomena - corrected three times, twice by the
    producers it reports on; only one is memory, and clearing context fixes that one while making
    two of the others worse.
- [What the theory already settles](2026-09-06-what-the-theory-already-settles.md)
  - 2026-09-06. From a conversation with Sean about the state function. The transformation is a
    Mealy machine and the list form is derived rather than new; `exec(environment)` is
    defunctionalization. Both findings fell out of the vocabulary and neither one needs it.
