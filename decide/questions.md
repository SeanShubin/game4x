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

### P-552 - An Ark cannot move, because two rules of `spec/` disagree and the orbital half is the one it needs

**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `spec/units.md`, and `spec/logistics.md` if you choose otherwise

**You asked whether the code has everything it needs from the spec to move an Ark. It does not, and
this is the whole of what is missing.** One decision, and it is yours because both sentences are
yours.

## Why an Ark cannot move today, measured

```
spec/data/line.4x:10   {line block:move seq:5 role:consume qty:1 kind:energy place-bound:from}
spec/data/carries.4x   an orbit carries `id` and nothing else
spec/data/*.4x         the word `sun` appears nowhere
```

**A move burns one energy at the place it leaves. An Ark leaves an orbit. Nothing puts energy in
an orbit.** So the command is refused for want of fuel, always.

## The two rules, and they cannot both be carried out

**`spec/units.md:20`**, the older:

```
A mobile unit that moves in orbit gathers its energy from the sun, a fixed amount each turn,
and holds it in a bin of its own. Moving in orbit burns a unit of it.
```

**`spec/logistics.md:29`**, which the code lane built:

```
A resource in a place is in that place, not in a container inside it. The things in it that
can hold that kind contribute capacity and hold nothing.
```

**One says the unit holds the fuel; the other says the place does.** `releases/first-release.md`
agrees with logistics three times - *fuel* is *how much energy its tank gives room for*, the tank
is a capacity row in *Where things are*, and `move`'s energy is at `$from`. **`spec/units.md` is
the only file that does not**, and its ground half says the same thing about a pioneer.

## The three answers

**`F1` - pooling wins, and the sun fills the orbit.** The unit's bin becomes room the orbit has,
and **an orbit gains a fixed amount of energy each turn from the sun**, up to the capacity the
things in it contribute. `spec/units.md`'s two bullets are rewritten to say where the energy is
rather than who holds it.

**`F2` - `spec/units.md` wins for units.** A unit holds its own fuel and pooling governs places
only. **This reverses what is built** and what the release says three times, and the tank stops
being a capacity.

**`F3` - pooling on the ground, the sun in orbit.** Two mechanisms with a boundary between them.
**Cheapest to write and the most to remember**: a reader has to know which layer they are on before
they know where fuel lives.

## What this lane would say

**`F1`.** It keeps your fiction - the sun is where orbital energy comes from - and changes only
*who holds it*, which is the half `spec/logistics.md` already settled for everything else. **It is
also the only one of the three that needs no new mechanism**: an orbit gaining energy each turn is
a place gaining a resource, which the turn already does.

**And it is the smallest change to what exists.** `F2` undoes pooling; `F3` adds a second rule for
fuel. `F1` rewrites two bullets and gives an orbit an income.

## What it does not settle, and what the code lane reported

**A unit that moves into an empty place cannot move again.** `spec/logistics.md` has *a thing that
leaves takes what it hauls*, defaulting to fill, **and `move`'s five rows in the release have no
haul** - so a unit arrives with nothing. The code lane built the table and says plainly it did not
build the general rule. **Whether hauling fires in this release is a separate question** and this
proposal does not ask it; under `F1` an Ark is unaffected, because the orbit it arrives in has its
own income.
