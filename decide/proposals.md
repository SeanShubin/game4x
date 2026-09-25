# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-551 - Four kinds of typed line, named for what each changes

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/console.md` -> Commands, and `spec/interface.md` -> Logs

**You chose `N1`.** The working - including why naming one in isolation could not be done - is in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## What lands

**After *a command that changes the game names a recipe*:**

> **Four kinds of line may be typed, and what a line changes says which it is.**
>
> - A **game command** changes game state. It names a recipe, it makes a transition, and it is in
>   the history
> - A **local command** changes local state - what is selected, where the view is, which panel is
>   open. **It makes no transition and is not in the history**, and it is logged
> - A **query** changes nothing and reports: `show`, `help`, `history`. It is logged
> - A **front-end line** begins with `/` and changes the application rather than the game.
>   **It is not a command**, and it is not logged
>
> **The game knows nothing of the interface.** Local state is not game state, no rule reads it, and
> **no local command is a transition** - so a replay of the history is a replay of the game and not
> of the clicking.

## Why this adds no word this repository had not spent

**`game` and `local` are the two adjectives you chose for the state**, so the commands are named by
the split rather than beside it. **`query` is already in this file** - *commands to query the game
state are available*, line 185. **And *a line beginning with `/`* is already the phrase**, three
paragraphs down.

## What it settles beyond naming

**`L3`, which asked how a local command differs from a `/` line.** They differ in what they change
- the game's objects against the application - and in that one is a command and the other is not,
which the file already said. **No separate rule was needed.**

## And the second block, because `P-555` landed an hour ago saying two

**`spec/interface.md:75` says each line records *whether it changed the game or the interface*.**
With a query in the set that is two values where there are three - **so the bullet is offered here
rather than left for a cleanup**, and one promotion leaves the two sections agreeing:

> - **`logs/commands.txt`** - every command, each line saying which kind it is: a **game command**,
>   a **local command** or a **query**. **The history is the game commands**, so `history` and this
>   cannot disagree about what happened

**Nothing else in that section changes.** `CLAUDE.md` says a promotion that makes something else
stale is not finished, and the two ways out are to refuse or to file the cleanup - **there is a
third when the staleness is one's own and known before landing**, which is to carry it.

*Nothing is open. Everything filed has been decided.*

