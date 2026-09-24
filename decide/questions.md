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

## The common terms, since you asked whether your vocabulary should follow them

**One of them is your split exactly, and it is the settled practice of a whole genre.** In
deterministic-lockstep real-time strategy - Age of Empires, StarCraft, Warcraft III and the games
after them - the division is:

```
commands / orders   affect the simulation, are sent to the other players, are recorded in
                    the replay, must be deterministic
local state         selection, camera, control groups, open panels. Never sent, never
                    recorded, never affects the simulation
```

**Selection is that tradition's textbook example of local state**, which is to say your instinct
is not unusual - it is the standard answer. **And you already have the replay**: `/save` writes
the history of the current game and `run` executes it, which is exactly a replay, and the
convention is that a replay carries commands and not clicks.

**The other established pairs, and what each costs here:**

```
simulation / presentation   idiomatic game-dev - "the sim", "sim tick". `simulation` is
                            unused in this repository: 0 in spec/, 0 in crates/src
model / view                MVC and its descendants. Universal, not game-specific, and
                            `model` is taken - 10 in spec/, 141 in the code
authoritative / client      networked games. Precise, and carries multiplayer baggage
intent                      simulation games: input becomes an intent, validated, applied.
                            ALREADY IN YOUR CODE 64 times and in spec/ zero times
```

## One caution the measurement turned up, and it argues against adding a third word

**You already have two terms for the mechanics side.** `spec/invariants.md` says **game state**;
`docs/architecture.md` says **the model**. **Adding *simulation* would make three**, all naming the
same thing, which is the opposite of what you asked for.

**So the term worth taking from common practice is the one for the other side**, where you have no
word at all. `local state` is the RTS term and is nearly free here - 0 in `spec/`, 9 in the code.

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

## What this lane would say, revised after measuring the common terms

**Keep `game state` and take `local state`** - which is `N1` with the industry's word rather than
an invented one. It gives the mechanics side the name it already has in your own invariants, gives
the other side the name the genre uses for exactly this, adds no third synonym, and stays
checkable: a command is one kind or the other, and a tool can require every command to say which.

**`N1`'s own `interface state` is the second choice** and its only advantage is being plainer to a
reader who has never built an RTS. **`N2` collides with two live terms and `N3` leaves nothing to
enforce.**

**And there is no settled term for a typed command that drives the interface**, because most games
never reify one - they handle input and move on. **The places that do are consoles and test
harnesses**, which is what you are building, so the pairing `command` / `local command` is as close
to standard as this gets and the second word is yours to pick.

## One sub-decision that comes with any of them

**Is an interface command recorded in history?** `/save` writes *the history of the current game*,
and history is transitions. **If selections are in it, replaying a game replays the clicking**; if
they are not, an interface test has to state them some other way, which the review application
would show.

**This lane would keep them out of history and let a test state them** - a test is a file of
commands already, and nothing says that file must be a game's history.
