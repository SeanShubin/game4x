# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-575 - five small design rules, and `create planet` writes commands rather than being one

**to** sean · **status** open · **raised** 2026-09-27 · **asks** approval · **kind** recovered · **shape** an instruction · **into** `spec/tests/`, then `spec/data/schema.4x` and `spec/data/rules.4x`

**This lane refused this an hour ago on a wrong argument and you corrected it.** It said
`create-planet` must add sixty-six rows in one rule and therefore needed `repeats`, which needs
removal. **`repeats` was never the mechanism**, and your first framing is already specified and
already built.

**`spec/console.md`**: *A command that writes commands appears as a comment. What it wrote is the
history - those are the commands that ran, and running the history again does what happened again.*
**And `crates/game-console/src/lib.rs` implements it** -
`history_records_what_a_subroutine_did_rather_than_the_call_to_it`, with a recursion limit and
failures reported against the line inside.

## So the shape is your second framing too, and the two are the same answer

**The geometry is computed in code and what it emits is commands.** Which twelve territories and
which thirty crossings comes from the Goldberg polyhedron, which is arithmetic no rule should do;
**what reaches the game is a list of ordinary commands, each naming a rule.**

**That makes determinism free and not a seed problem.** The emitted commands are the history, so
replaying them rebuilds the same world however they were computed. **A seed only matters where the
generator chooses** - biomes - and once the choice is written as a command it is fixed for good.

## The rules that are needed, all small

```
add-territory   makes a territory and its two places, a surface and an orbit    one add each
add-crossing    makes a crossing from one territory to another                  one add
set-resource    gives a territory a deposit of one resource, with a density     one add
set-biome       gives a territory its biome                                     one add, needs
                                                                                a `biome` relation
add-to-orbit    puts a unit in the orbit of a territory before play begins      one add
```

**None needs `repeats` and none adds more than a handful of rows.** `create planet <size>` is not
among them: **it is a command that writes these**, and the history records what it wrote.

## The one schema change

**`spec/data/schema.4x` has no `biome` relation** - measured over its forty-seven. `territory`,
`place`, `adjacency` and `deposit` are all there. `P-541` brought biome back to the data and
nothing carried it into this schema.

## How to tell it was carried out

```
spec/tests/            five new tests, one per rule, each stating a world, firing one
                       command, and asserting what it made
the review application shows all five as unread
spec/data/schema.4x    gains a `biome` relation and nothing else
spec/data/rules.4x     gains five rules, and the suite runs the five tests against them
reviewed/              unchanged - this lane writes tests and never the record
```

**Not in this**: `create planet <size>` itself, which is the console writing commands and is the
code lane's. **It is unblocked by this rather than part of it** - the commands it would write do
not exist until these five rules do.
