# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-396 - The release cannot follow `P-390` until a moved thing can be said to hold nothing

**to** sean - **status** open - **raised** 2026-09-11 - **kind** entailed, from `P-390` promoted - **asks** a decision - **into** `releases/first-release.md` -> Recipes

**You asked for the release to be brought into line with the specification, and five of the six
blocks are mechanical.** `create labor`, `work` and `bear` each lose their *produce ... not ready*
row and consume a readiness instead; `renew` stops existing; `refresh` becomes one rule. **`move` is
the one that does not go through**, and the reason is a sentence in `P-390` meeting a habit in the
release.

**`move` is written as consume-and-produce.** It takes the unit at `$from` and makes one at `$to` -
five rows, and the produced one is marked **`not ready`** so it cannot move again this turn.

**`P-390` says a created thing arrives holding its tokens.** *A thing created during a turn begins
holding its tokens and may act at once.* **So the unit `move` produces arrives with a fresh token to
move with**, and a unit can cross the planet in one turn. The trait `not ready` was what stopped
that, and tokens have no trait to be marked with.

**Two ways, and the choice is which one the game is.**

- **A moved thing is not a created thing.** `move` stops being consume-and-produce and becomes a rule
  that changes where one thing is - which is what it always meant, and what `spec/logistics.md`
  already says holds it: *what holds it is what says where it is*. **The notation gains a way to say
  a thing moves**, and `deploy ark` and `found by land` are the other two rules that would want it
- **A produce may say what it does not carry.** `move` stays as it is and its produce row names the
  tokens the new thing arrives without. **Smaller, and it keeps a shape that says a unit is destroyed
  and another created every time one walks** - which is also what makes the no-gain check weigh a
  unit against itself twice

**This lane is not recommending, because the first changes the notation and that is yours.** What it
will say is that the second reads as a workaround for the first being unavailable, and `X-12` already
records the same pressure from another direction - *deriving the territory from the selected ark
deletes row 1 and a `Where`*.

**Nothing is blocked while this waits.** The release is consistent with itself today and inconsistent
with `spec/turn.md`, which is the state a release is allowed to be in: *the spec is the destination
and always wins; a release only says what is true today*. **The other five blocks are held rather
than landed**, because splitting them from `move` would leave the release half in each model, which
is worse than being wholly in the old one.

