# Starting from nothing: separate files, one history, one test

**Derived**, 2026-09-03, answering Sean's question of whether defining the rules, setting up a game
and running a scenario are separate things or one thing end to end. Not binding.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

> I am of starting with zero recipes and zero things, and building everything up from commands. So
> all we start with is an empty game engine. Then we use commands to define the game rules, then
> commands to setup a game, then commands to run a game scenario. Finally I want this to be in a
> test.

## The question is narrower than it looks, because one half is already decided

**`spec/invariants.md`, promoted:** *the definitions are part of the game state. **Defining one is a
transition like any other**, so a game's history is a complete account of it, including what its
rules were.*

So the rules are defined by transitions, in the same history as everything else. **It is one history
whether or not it is one file** - and `spec/console.md`'s `run <file>`, *run the commands in a file
as though they had been typed in its place*, makes several files into one history for free.

**So the real question is only: one file or three?** And composition costs nothing either way.

## Three promoted rules already point where he is going

- **Definitions are transitions** - `spec/invariants.md`
- **What the game is made of lives in a data file** - `P-199`
- **`run <file>` executes a file of commands as though typed**

**Together: the data file is a command file, and there is no separate loader.** The thing that
defines a kind and the thing that plays a turn are the same mechanism. That is not a new direction;
it is three rules meeting.

## Recommendation: three files, one history, one test

| File     | Holds                                   | Changes when              |
| -------- | --------------------------------------- | ------------------------- |
| rules    | the kinds and the recipes               | the game changes          |
| world    | this planet, its territories and biomes | a different map is wanted |
| scenario | the commands a player ran               | a different run is wanted |

**Separate files, because the three answer different questions and change for different reasons.**
Sean has already drawn this line himself in the reports: thing and recipe definitions are
context-free, the dump is context-specific. **A single file would put both sides of that line in one
place**, and no report could render the rules without running a game.

**One history, because the specification already requires it** and because the closure test needs a
single derivation from an empty engine to the final dump.

**One test, because that is `R-6`.** *The loop can be played through* is the last unvetted capability
in the release, and a test that runs rules, world and scenario in order is its evidence.

## What it costs, said plainly

**The command language has to grow commands that define a kind and a recipe.** `spec/console.md`
currently has `create planet`, `set resource`, `set force`, `set biome`, `add <unit> orbit` and
`start` - **all of which configure a world, none of which define a rule.**

**And that language is the rule editor.** `spec/interface.md` names four surfaces and the fourth is
*the rules the player has, read and changed*. **A console grammar that can define a kind is that
surface's text form** - `spec/invariants.md` already says *every rule has a text form, and the text
is the rule. Anything the rule editor can build can be written out.*

So the cost is real and it is a cost the specification has already committed to paying.

## The one thing to be careful of

**A rules file that is executed is not the same as a rules file that is loaded**, and the difference
shows up in error handling. A loader can report *this table is malformed* against a whole document; a
command file fails at a line. **`spec/console.md` already chose**: *a script stops at the first line
that fails and says which*.

That is the right behaviour for a scenario and it is worth checking it stays right for a rules file
of several hundred lines, where the first failure may be a long way from the cause.
