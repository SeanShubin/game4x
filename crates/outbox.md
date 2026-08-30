# Code outbox

**Derived.** The code lane's one outbox. Every question it has addressed to somebody, and what
became of it. Not binding - a question is a thing this lane cannot settle, not a decision about it.

[Architecture](../docs/architecture.md) · [The proposal queue](../docs/notes/proposals.md) · [The quality lens](../lenses/quality/outbox.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to spec` - something this lane cannot settle for itself: almost always *the specification does
  not say X, and I cannot build it until it does*. The specification lane turns it into a numbered
  proposal; it does not decide it.

**Status** is one of `open`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` items are
outstanding.

> **The guarantee.** If nothing here is `open`, this lane is blocked on nothing. That is a promise
> about this file, not about the tree - it does not say the code is finished, only that every
> question this lane cannot answer for itself is sitting where its reader will find it.

An item is written the moment the lane is blocked, not at the end of the work that found it. The
whole point is that a blocked question has somewhere to go other than a reply, which scrolls away.

`tools/outbox` reads this file. An item is a `### <id> - <title>` heading followed by a line
carrying `**to**` and `**status**`; everything else is prose for a person.

Ids are `C-1` upward, and unique across every outbox - a duplicated id is how a status silently
stops meaning anything, because a commit citing it no longer says which item it closed.

---

## Open

### C-3 - A prototype cannot photograph itself, and two items now need it to

**to** spec · **status** open · **raised** 2026-08-30 · **source** `Q-1`

`crates/game4x` can be driven from outside - `--shot` writes one frame, `--dump` writes the state -
and that harness is what let this lane find a planet whose coastlines were pentagonal when every
test passed. It lives in the composition root, so only the application has it.

Two things now need it and cannot have it:

- **`prototypes/goldberg-view`** compares the ten smallest Goldberg solids, and the question it
  exists to answer is what 492 faces *look* like. This lane can measure that it builds in 164 ms
  and cannot see it.
- **`Q-1`**, the palette in three places. The copy in `planet.wgsl` is read only by the GPU path in
  `planet-bevy`, which runs in `prototypes/planet-view`. `planet-view --capture` exists but writes
  from the **CPU rasterizer**, so it cannot photograph the path the duplicated palette is for.
  Deleting that copy is a change this lane cannot verify.

The fix is to extract the harness so any composition root can add it, which is code and this lane's.
What is filed here is the prior question: **whether a prototype is worth that.**
`docs/prototypes/README.md` says a prototype is finished when its question is answered and may take
shortcuts the game may not - which reads as *do not invest in prototypes*, and a screenshot harness
is an investment. If the answer is that a prototype's whole value is the picture, then it is not a
shortcut and the harness belongs everywhere; if a prototype is meant to stay cheap, `Q-1`'s
remaining half needs a different plan.

### C-2 - Architecture rule 6 states the losing side of a decision as fact

**to** spec · **status** open · **raised** 2026-08-30 · **source** `Q-4`

`docs/architecture.md` rule 6: *every game entity is an ECS entity - there is no second way of
holding game state.* That is not what the code does and has not been for some time. The game's
state is `game_model::Game`, reached through the one console, and `Session::run` is the only way
it moves. `spec/invariants.md` describes the same arrangement - a state and a transition yield a
new state - and says nothing about entities.

`planet-ecs` was the crate that made rule 6 true, and this lane has just removed it from the
shipped application, because nothing read it: the regions it spawned were a second, unread copy of
what `game-model` already holds. The crate stays for `prototypes/planet-view`, which uses it for
what it was built for.

So the rule now describes a design the application does not have, and a reader following it would
build the second way of holding state that `spec/invariants.md` forbids. Two ways to settle it, and
this lane has no standing to pick:

- **The rule is stale** and should describe the arrangement that exists - one model, one door,
  entities where an engine needs them.
- **The rule is the destination** and the code has drifted from it, in which case the drift is
  large and worth stating as such rather than left to be discovered.

Reported rather than fixed because `docs/` is not this lane's column, and because which of those is
true is a design question rather than a defect.

### C-1 - Whose file is a generated one at the repository root?

**to** spec · **status** open · **raised** 2026-08-30 · **source** `Q-33`

`Q-33` asked for one document Sean opens rather than a command he remembers, and did not say
where it goes. It is at `pending.md`, in the root, because that is where *one place to look*
points.

But `CLAUDE.md` gives this lane `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`, `hooks/`,
CI and cargo, and the root is in nobody's column - it names `README.md` for the specification lane
and stops there. So this lane has written a file outside its column, and is saying so rather than
letting it pass.

Two things would settle it, and either is fine:

- **The root is right and a generated artifact has no author**, in which case say so, because the
  next generated file will raise the same question.
- **It belongs somewhere owned**, in which case name the path. It is a one-word change - the
  generator already takes the path as an argument, and `hooks/pre-commit` passes none.

Not blocking. The document exists and the hook keeps it current either way.

---

## Resolved

Kept rather than deleted, so a later reader can tell whether a question was answered or forgotten.

Nothing yet.
