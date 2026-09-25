# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-551 - Four kinds of typed line, and the names fall out of the two you already chose

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** a decision · **asks** a decision · **into** `spec/console.md` -> Commands

**You said you cannot name one in isolation and you are right** - `spec/console.md` already
distinguishes four kinds of typed line and **two of the four have no name**, which is why naming
the fifth had nothing to sit beside.

## What the file already distinguishes, and what it calls each

```
{move unit:ark from:1 to:2}   named: "a command that changes the game names a recipe"
{show territory:1}            UNNAMED: "and three that change nothing"
/console                      UNNAMED: "a line beginning with `/`... none of these is a command"
{select unit:ark}             does not exist yet
```

**So the set is four and the vocabulary is one word plus two descriptions.**

## `N1` - named for what each changes, which costs no new words

```
{move unit:ark from:1 to:2}   a game command     changes game state    makes a transition, in the history
{select unit:ark}             a local command    changes local state    logged, not in the history
{show territory:1}            a query            changes nothing        logged, not in the history
/console                      a front-end line   changes the application   not logged
```

**Nothing here is invented.** *Game state* and *local state* are the two you chose this afternoon,
and the command names are those two adjectives; `query` is already in `spec/console.md` -
*commands to query the game state are available*; and *a line beginning with `/` directs the front
end* is already the phrase. **Four names, four kinds, and no word this repository has not already
spent.**

**And it answers `L3` without a separate rule**: a `/` line is not a command at all, which the file
already says. **That is the difference** - a local command acts on the game's own objects and a
front-end line acts on the application.

## `N2` - one noun, qualified where it matters

**No new names**: *a command names the game, the interface, or nothing*, and `/` lines stay
not-commands. **Cheapest to read and there is nothing to check** - a tool cannot ask which kind a
command is, so the log's mark and `help`'s contents rest on whoever writes them.

## `N3` - name the act after the effect

**A game command becomes *a transition***, since `spec/invariants.md` already says a game state
changes only by a transition. **Costs**: it conflates the act with its effect - a command *makes* a
transition - and `transition` is used nine times in `spec/` for the change rather than the typing.

## What this lane would say

**`N1`.** It is the only one of the three that adds no vocabulary, and the reason is that you
already did the work: the state split you chose names the commands for free. **`N2` leaves nothing
to enforce and `N3` spends a word that is already doing another job.**

## One thing `N1` forces, and it is small

**The command log's mark becomes three-valued rather than two.** `P-555` says each line says
*whether it changed the game or the interface*; with a query in the set it is **game, local or
query**, and a `/` line is not in the log at all. **One word in a sentence not yet promoted**, so
nothing landed has to change.
