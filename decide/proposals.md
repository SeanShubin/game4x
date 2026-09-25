# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-556 - `spec/control.md` becomes a future plan whole, because nothing is left in it

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `spec/control.md` -> `spec/future/control.md`, `spec/README.md`, and `releases/first-release.md` -> `R-6`

**Two answers, hours apart, and together they empty a file.** *Lets simply remove the concept for
now... launching the ark from a separate territory is the test we are using to make sure the first
release works, not a win condition, a requirement.* And then: *lets not specify a loss condition
either.*

**Measured: `spec/control.md` holds exactly those two bullets.** Winning, Losing, and an Open
questions heading with nothing under it. **So this is not two sections moving out of a file - it is
the file.**

## What lands

**`spec/control.md` becomes `spec/future/control.md`**, whole, under the line `P-541` established:

> **These rules are not built and are not abandoned.** They are here because the game wants them
> once it can be played, and the first release cannot be played while it is building them.

**And `spec/README.md`'s row follows it**, from *How a game is won and lost* to a future plan
beside `Force`.

**`R-6` stops calling it a win** and states the requirement you named:

> - **Vetted when** - the scenario takes a first territory from orbit, takes a second by land, and
>   **launches an Ark from the second**; and every recipe in the release fires at least once while
>   it runs, measured by what fired rather than by what the file says. **This is the requirement
>   the first release is checked against and not a victory** - nothing ends, and the interface lets
>   the player keep going.

## The part that is not a deletion

**`R-6` currently passes by not winning, and under your requirement it fails.** The scenario
deploys to territory 1, builds its Yard in 1 and launches from 1 - so the launch is not from the
second territory, and **the scenario has to change.** That is `S-174`, with the `won` latch `S-151`
built this afternoon.

## Your four interface items, measured against what exists

```
start a new game   `/new <size>`, in spec/console.md
save a game        `/save <file>`, which writes the history
load a game        `run <file>` on what `/save` wrote - a composition, not a line of its own
exit the game      NOTHING. No exit and no quit anywhere in spec/
```

**Three of the four are already front-end lines**, which is the category `P-551`'s `N1` names -
they change the application and not the game. **The fourth is not specified at all**, and your own
sketch has *exit game* as a menu item. **Filed as `S-175`** rather than added here, because this
proposal is about a file emptying and that is about a file gaining a line.
