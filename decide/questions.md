# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-551 - Two invariants leave a hole exactly where you say a command is needed, and one word is taken twice

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** an instruction · **asks** a decision · **into** `spec/invariants.md`, `spec/console.md` -> Commands, and a term used throughout

**Your instinct is already required by what you promoted**, and the hole is measurable.

## The hole

```
spec/invariants.md   Every change to game state is representable and executable as a
                     console command
spec/invariants.md   Anything the player can do through a surface can be done by typing
spec/console.md:241  A line beginning with `/` directs the front end rather than the game
                     ... None of these is a command and none is a transition
```

**Selecting an ark is something the player does through a surface**, so the second clause makes it
typeable. **It is not a change to game state**, so the first clause does not reach it. **And it is
not `/console` or `/save`** - those direct the front end, and this is about the game's own objects.

**So there is a typeable thing that is neither a command nor a `/` line, and `spec/` has no name
for it.**

## Which words are free, measured

```
                    uses in spec/   in docs/architecture.md
game state                8                4     the existing term, keep it
surface                  17                3     TAKEN TWICE - one of five surfaces,
                                                 and a territory's surface layer
presentation             10                3     taken; `planet-presentation` is a crate
view model                0                3     a layer name in the architecture
session                   0                2     near it: two uses in the architecture
view state                0                0     free, but one word from `view model`
interface state           0                0     free
interaction               0                0     free
```

**`surface` is the one to avoid.** It already means two different things in `spec/`, and a third
would make the word useless.

## The three vocabularies, shown rather than described

**`N1` - game state and interface state, and a command says which it addresses.**

```
{move unit:ark from:1 to:2}   a game command      a transition, in history
{select unit:ark}             an interface command  not a transition, not in history
/console                      a front-end line    not a command at all
```

**`N2` - the world and the view.** The same split, shorter words: a command changes the world or
changes the view. **Costs**: `view` is one word from `view model`, a layer name, and `world` is
already used for the world's own recipes - *ending a turn fires the world's*.

**`N3` - one word, qualified in place.** No new noun: a command *names the game or the interface*,
and the state each holds is described where it matters rather than named. **Costs**: nothing to
learn, and nothing to check either - a tool cannot ask which kind a command is.

## What this lane would say

**`N1`.** Both halves are free words, `interface state` cannot be mistaken for a layer, and it
makes the distinction checkable: **a command is one kind or the other, and a tool can require that
every command declares which.** `N2` reads better and collides with two live terms; `N3` is the
only one that leaves nothing to enforce.

## One sub-decision that comes with any of them

**Is an interface command recorded in history?** `/save` writes *the history of the current game*,
and history is transitions. **If selections are in it, replaying a game replays the clicking**; if
they are not, an interface test has to state them some other way, which the review application
would show.

**This lane would keep them out of history and let a test state them** - a test is a file of
commands already, and nothing says that file must be a game's history.
