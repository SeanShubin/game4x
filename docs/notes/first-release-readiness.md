# Can the First Release Be Built From What Is Written?

**Derived.** Written by Claude from conversation, 2026-08-26. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

> **Superseded, 2026-08-26.** Every gap this note found has been closed. The design phase gained
> four commands (P-76), a win condition landed (P-77), and the spent flag settled when a command
> takes effect (P-78) - along with `start` and `launch` being added to the command list and
> `end turn` being corrected. **It is kept as the record of what the audit found and how it was
> answered, not as a description of the specification today.**

Sean asked whether the specification is ready to hand to the coding instance, wanting *a test that
ensures the engine runs our entire script of commands and handles the state transitions properly*.

**Method: try to write that script.** Every place it cannot be written is a gap, and nothing else
counts as one.

## The script, as far as it goes

```
                            <- DESIGN PHASE: no command exists to do any of this
start
land ark 1                  <- founds territory 1: garrison, citizen, food extractor
work 1 extractor 1 food     <- the one citizen works the one extractor
end turn                    <- produce 4 food, consume 1, grow to 2
work 1 extractor 1 food
build extractor 1 food      <- ? does an extractor built this turn produce this turn
end turn
...
produce pioneer 1           <- 16 metal, 1 citizen, 12 energy
move pioneer 2              <- founds territory 2
...
build yard 11               <- 30 metal
produce ark 11              <- 24 metal, 24 energy
move ark orbit              <- ? or `launch ark`
                            <- ? and then what: nothing says this is winning
```

## What blocks it

**1. The design phase has no vocabulary at all.** `spec/console.md` -> Phases says a game is
designed and then started. **No command creates a planet, a territory, a node or a starting Ark.**
So the script cannot begin, the twelve territories of P-68 cannot be brought into existence, and
`start` has nothing to start. This is the only gap that blocks everything else.

**2. There is no win condition.** `spec/control.md` -> Losing says when a player has lost. Nothing
anywhere says when one has won. The release loop ends at *launch the colonizer into space* and does
not say that finishes anything. **A test that runs the whole script has no terminal state to assert
against** - it can check that the last command succeeded, which is not the same as checking that
the game was won.

**3. When a command takes effect is unspecified.** `spec/turn.md` says a turn resolves produce,
then consume, then transform. It does not say where a player's commands sit relative to those
phases. Concretely, and each has a different outcome:

- Does an extractor built this turn produce this turn?
- Does `work` schedule labor for the coming resolution, or spend it immediately?
- Does `move` happen when typed, or during resolution?
- When is force compared against the force of nature - every phase, or once per turn?

**Every one of these is a state transition, and P-75 says the state is exactly the fold of the
transitions.** Two implementations that answer these differently produce different states from the
same script, which is precisely what the test is meant to catch.

**4. `start` is in Phases but not in the command list.** A one-line omission, noted when P-74 and
P-69 were promoted together.

**5. Launching has no command.** `spec/orbit.md` says a unit may launch to orbit. `move <unit>
<territory>` takes a territory, and orbit is explicitly *not* a territory. So either `move` accepts
`orbit` as a destination or there is a `launch` verb; P-69 raised this and it was not settled.

## What is ready

Everything else. The rules for founding, growth, starvation, labor, force, taking and holding,
costs, node figures and turn order are all written and consistent. **The gaps are at the two ends -
how a game begins and how it ends - and in the timing of when commands apply.** The middle is
complete.

## The smallest thing that unblocks delegation

Gaps 1, 2 and 3 need Sean. Gaps 4 and 5 are one line each and follow from decisions already made.

Gap 1 is the largest and has the most design in it, because a design-phase vocabulary is a small
language of its own: place a planet, place nodes, place a starting unit. It is also the thing that
makes [P-68's twelve territories](proposals.md) a command file rather than hardcoded data, which is
what Sean said he wanted.

**Against the two acceptance criteria Sean named:**

| Criterion                                   | Blocked by         |
| ------------------------------------------- | ------------------ |
| A test running the whole script of commands | gaps 1, 2, 3       |
| Three modes, desktop and web, on GitHub     | nothing in `spec/` |

The second is entirely the code lane's: `spec/interface.md` says three surfaces in every build, and
what a build is belongs to the crate layout and the CI pipeline rather than to the specification.
