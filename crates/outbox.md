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
