# What the theory already settles

2026-09-06. From a conversation with Sean about the state function. Carries the argument for `X-1`
and `X-2`.

**Method.** Sean asked what established theory lies behind the transformation function, having
arrived at it himself and expecting it to be a rediscovery. It is. Naming it correctly is what made
two gaps beside it visible. Every file and line below was re-read at the moment of writing rather
than recalled, and every count names the population it was taken against.

## The identification, which is addressed to nobody

`docs/process.md:81` states the requirement as `(old-state, commands) -> (new-state, effects)`.

- The single-step form is a **Mealy machine** - a transition and an output function over the same
  pair, fused into one function returning a tuple, which is the coalgebraic presentation.
- **The list form is not a generalization.** It is the unique extension of the single-step function
  to the free monoid, and it exists automatically: the free monoid of command sequences acts on the
  state set, outputs concatenate, and running `xs` then `ys` equals running `xs ++ ys`. That
  associativity is the same theorem as replay-from-log and save-as-command-log. Lists appear because
  the output type must be a monoid and list is the free one - which also means that if order is not
  supposed to matter, list is over-specified and a bag is the honest type.
- **Feeding effects back as commands** is the Elm architecture's update function, the actor model's
  transition, and rewriting on configurations, depending on which property you want from it. The
  closest named thing with the loop closed is functional event sourcing's decide / evolve / react.
- **Passing an environment to a command is defunctionalization** (Reynolds, 1972) - the command is a
  tag, and the executing function is Reynolds' apply. Collecting nondeterminism behind a
  substitutable environment is ports and adapters, object-capability discipline, and industrially
  **deterministic simulation testing**.

**None of this belongs in Sean's documents**, and the recommendation this lens gave was to keep all
of it out. `CLAUDE.md` names the trap: *measuring something is not a reason to specify it*. Three
turns of reading earned one sentence, and the sentence does not need the vocabulary to state. It is
recorded here because a later reader asking the same question should not have to re-derive it.

## X-1: the precondition that is never stated

`docs/process.md:91`, in *How I know the application is right*, ends the sentence:

> derive the fourth by hand. If I can do that, I can tell whether the application is behaving as I
> intend.

Four artifacts - the definitions of the things, the definitions of the transformations, the commands
a scenario ran, and the data dump. The first three yield the fourth **only if the transformation is
a function of the state and the commands alone**. Anything else it reads - a clock, an entropy
source, a file whose content the scenario does not declare - is a term in the derivation that Sean
cannot see, and the fourth artifact stops being derivable.

**Measurement.** Grepping the six words `determin`, `replay`, `nondeterm`, `random`, `clock` and
`reproduc`, case-insensitively, over `docs/process.md` returns 0. The same invocation reports 459
lines and 24 headings, so the denominator is named and not also zero. Re-run it rather than trusting
it; the file changed twice during the session that produced this report, and the first grep was
against bytes that no longer exist.

**Where the property does live**, none of it normative and none of it in Sean's voice:

- `docs/layers.md:184` bets replays, saves-as-command-log, deterministic tests and lockstep
  multiplayer on the model being a pure fold. That is the shared why-layer.
- `crates/game-model/src/lib.rs:50` - a doc comment on `no_floating_point_anywhere` saying
  integers-only is what makes resolving territories in any order safe. That is the code lane
  asserting a criterion in a comment.

So the property is relied on in three places and stated in none that governs.

**One criterion covers the whole of it.** Whether an rng sits behind an interface, whether a clock is
substituted, whether the console's `Library` is safe - all reduce to *is it in the four artifacts?*
A rule about environments is not needed; the rule about the four artifacts is, and the environment
answers to it.

**And it is cheap to enforce today.** `game-model` depends on exactly one crate, `planet-model`, and
nothing external - no `rand`, no `std::time`. The only filesystem access in that crate is inside a
`cfg(test)` block. So *the model's dependency closure contains no source of nondeterminism, over N
crates checked* is a `cargo tree` assertion that passes now and goes red the day someone adds
`rand`. That is a code-lane item and is deliberately not filed, because a check enforcing an
unwritten criterion is a rule invented by whoever wrote the check.

## X-2: a default that builds what the specification forbids

`spec/turn.md:22`:

> Where two effects cannot both happen, they compete. Competing effects are gathered and resolved
> together, so nothing gains an advantage by being considered first

and `spec/turn.md:24`:

> What settles them is a deterministic mechanic of the game, and therefore something a person wrote
> and a player can change

`docs/layers.md:207`:

> Where two events collide, the lower index wins. Fully parallel across the whole array, and still
> perfectly reproducible - because the index is data, not a schedule.

and `docs/layers.md:210` makes that reading **the default**.

**The two do not collide on determinism, and this lens first thought they did.** `turn.md` forbids
order of *consideration* mattering, and an index is data rather than a schedule, so the defence in
`layers.md` is sound on the point it defends. The first framing of this finding - that one file
wants a bag and the other a list - was weaker than it looked, and is recorded here because the
correction is the useful part.

**They collide on authorship.** `turn.md` does not ask only for determinism. It asks that the
settling mechanic be *of the game*, *written by a person*, and *changeable by a player*. An array
index is none of the three. It is an artifact of how the effects happened to be collected.

**Why that is worse than a contradiction inside `spec/`.** `docs/layers.md` is non-normative, so
nothing is formally wrong. But it is the document the code lane reads for guidance, it says *default
to this*, and each file is correct on its own terms - so nothing reports the divergence, and the
first thing built would satisfy the guidance while violating the rule.

**The second face, noted and not separately filed.** Whether a command executes against a shared
mutable environment or returns its effects for a later merge is the same question in the code: the
first makes effect *i* visible to effect *i+1*, which is the sequence reading in `layers.md`; the
second keeps the tie-break reading available. Settling it in one place and not the other is how one
decision gets made twice, differently.

## What this report does not claim

- **It does not say the specification is wrong.** `X-2` is a question about which of two documents
  governs, and that is Sean's.
- **It does not propose text.** Both items name a destination and stop. The words are what Sean
  approves, and the specification lane writes them.
- **It found nothing in `crates/`.** One incidental observation about two source-scanning guards was
  relayed to the quality lens on 2026-09-05, verified there, narrowed - two of the three failure
  modes this lens named could not happen, because the directory read is unwrapped and panics rather
  than passing empty - and filed as its `Q-51`. It is recorded here only so a later reader does not
  re-find it.
