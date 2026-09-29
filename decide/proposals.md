# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-591 - A command that can be expressed as a row is not needed, and two of them can be today

**to** sean · **status** open · **raised** 2026-09-29 · **answered** 2026-09-29, the sequencing · **asks** approval · **kind** recovered · **shape** text · **into** `spec/console.md` -> Commands

**You took the sequencing rather than the cull**, so the rule lands and each command leaves when the
notation can say what it said. **Two can leave now** and need nothing built; one waits for the tree.

**Offered as a bullet at the end of the design-phase list, after `start`:**

> **A command that can be expressed as a row is not needed.** What a design command does is put rows
> in the world before play begins, so one that writes rows a person could write is a shorthand rather
> than a capability - **and it goes when the notation can say what it said, not before.** What earns
> a command is that its rows are computed rather than written: `create planet` runs a tessellation,
> and the adjacency of a ninety-two-face Goldberg polyhedron is not tedious to write but impossible
> to write correctly.

## Which go now, which waits, and which stays

```
set biome          one cell on a row that already exists      goes with this promotion
add <unit> orbit   one row                                    goes with this promotion
set resource       a deposit and a capacity, two rows         waits for a tree that normalizes
create planet      96 rows at tiny-12, 816 at huge-92         stays, and is what the rule saves
generate-planet    a planet and everything a designed one     stays, same reason
start              changes the phase, writes no rows          stays, and is not a shorthand
```

**`set force` is not on the list** - `P-522` cut force from the release - so there is nothing to
remove for it.

**The two that go are removed in the promoting commit**, with the count asserted, because a rule
whose consequences sit in a queue is a rule nobody has applied. **`set resource` stays until a tree
normalizes into rows**, which is your own reason for the sequencing: removing it first would remove
the capability rather than replace it.

## What this does not decide

**How a tree is written or what normalizes it.** That is the larger thing you named alongside this -
*the entirety of game state as a tree*, wanted for debugging and possibly for specification and
regression tests - and it is in `docs/notes/spec-backlog.md` unproposed, because nothing has been
said about how one is written or what refuses an ambiguous one.

