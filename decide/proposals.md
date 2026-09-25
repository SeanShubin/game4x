# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-556 - Winning leaves `spec/` for now, and the launch becomes a requirement rather than a victory

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `spec/control.md` -> Winning, `spec/future/`, and `releases/first-release.md` -> `R-6`

**Your words**: *lets simply remove the concept for now. The user interface is going to just let
you keep playing. Launching the ark from a separate territory is the test we are using to make sure
the first release works, not a win condition, a requirement.*

**This takes out the sentence promoted an hour ago**, which is what *for now* means and is worth
saying plainly rather than quietly.

## What lands

**`spec/control.md` -> Winning goes whole**, into `spec/future/winning.md`, under the line
`P-541` established for a rule that is not built and not abandoned:

> **These rules are not built and are not abandoned.** They are here because the game wants them
> once it can be played, and the first release cannot be played while it is building them.

**And `R-6` stops calling it a win.** Its *vetted when* says *it does not win, and that is the win
condition working*; it becomes the requirement you named:

> - **Vetted when** - the scenario takes a first territory from orbit, takes a second by land, and
>   **launches an Ark from the second**; and every recipe in the release fires at least once while
>   it runs, measured by what fired rather than by what the file says. **This is the requirement
>   the first release is checked against and not a victory** - nothing ends, and the interface lets
>   the player keep going.

## What this asks you to notice, because it is not only a deletion

**`R-6` currently passes by not winning.** Its scenario deploys to territory 1, builds its Yard in
1 and launches from 1 - **so under the requirement you just stated it fails**, because the launch
is not from the second territory. **The scenario has to change**, which is the code lane's and is
`S-174`.

**That is the same gap `P-554` found and the answer inverts it.** `P-554` asked whether the release
should show you a win; you have removed the win, and the thing that replaces it is a requirement the
scenario does not currently meet. **`P-554` is answered and lifted.**

## Two consequences named rather than left

**`S-151` built the `won` latch today**, and there is nothing for it to latch. **Filed as `S-174`**
with the scenario change, because both are the code lane's and both follow from this one sentence.

**And `Losing` is still in `spec/control.md`** - *a player has lost when they have no citizens and
nothing that converts into a citizen*. **You said the win condition and not the loss**, and if the
interface lets you keep playing then losing may be in the same position. **This lane has not touched
it and is not proposing to**; say so if it should follow, and it is one more move into the same
file.

*Nothing is open. Everything filed has been decided.*

