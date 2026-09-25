# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-557 - `spec/data/above.4x` states what the naming rule already fixes

**to** sean · **status** open · **raised** 2026-09-25 · **asks** approval · **kind** a contradiction inside `spec/` · **shape** an instruction · **into** `spec/data/above.4x` - the file

**Delete `spec/data/above.4x`.** Its twelve rows state which orbit is above which territory:
`{above orbit:1 territory:1}` through `{above orbit:12 territory:12}`.

**`spec/console.md` already says they cannot be stated:**

```
**A place worked out from another is not open** - the orbit above a territory is named
by naming the territory.
```

**So the twelve rows are a second copy of what the naming rule fixes** - which is the reason the
release already gives for refusing to state orbital adjacency: *stating it would be a second copy
that can disagree*.

**You said why it is there, 2026-09-25**: it was created intentionally when the design had orbits
in an adjacency map rather than layers of a territory. **`P-500` promoted it under that design**;
`spec/orbit.md`'s layer rules replaced the design and nothing filed the cleanup.

## What survives, because it is not what this deletes

**The `above` operator stays.** `place-above:where` appears in five rows of `spec/data/line.4x` and
`above $where` in five cells of the release's *Where* column. **`P-500`'s other half - the four
prose cells it took out of *Where* - is untouched.** What goes is the table of pairs, not the way a
recipe names the place above a territory.

## How to tell it was carried out

`spec/data/above.4x` does not exist; `spec/data/` holds eleven files; the five `place-above:where`
rows and five `above $where` cells are unchanged. **The promoting commit runs that check.**

## What this makes stale, filed in the same turn

**`R-11`'s evidence** counts *twenty links over three directories - twelve in `spec/data/`*, which
become nineteen and eleven. **The code lane's `declare.rs` names four relations** - `carries`,
`member`, `limit`, `above` - which become three, and its `dump.rs` says *one table for each of the
twelve in `spec/data/`*.

**And `C-132` inverts rather than closing.** It asks the code lane to write `above` into the dump;
this says there is nothing to write. **That unblocks them without inventing a kind** - they could
not build it, because `Description::of` takes a closed enum and `above` is not a kind, so the dump
physically cannot name it.


