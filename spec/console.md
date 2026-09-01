# Console

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

## Phases

- A game has two phases. In the first the world is designed. In the second it is played.
- `start` ends the first and begins the second.
- The rules of the game govern the second phase. In the first, the designer is the cause of what
  appears.

## The language

- Commands may be organized in a hierarchy of files, one file invoking another as a subroutine
- A `#` begins a comment. The rest of the line is ignored

A command is a verb followed by arguments, one command to a line. A territory is named by its
id, a structure by its kind and the territory it is in.

```
land ark 1
move pioneer 7
build extractor 3 metal
produce pioneer 11
work 4 extractor 3 metal
end turn
show territory 5
help move
```

## Commands

- Commands to query the game state are available
- A sequence of commands may be run one at a time interactively, or run in full as a test

One command for each way the game state can change:

- `land <unit> <territory>` - bring a unit down from orbit. It founds the territory
- `launch <unit>` - send a unit from the territory it is in up to orbit
- `move <unit> <territory>` - move a unit to an adjacent territory. If the territory is not already
  controlled, it is taken
- `build <structure> <territory> [<resource>]` - build a structure, paying its cost there
- `produce <unit> <territory>` - produce a unit at a structure that allows it
- `work <count> <structure> <territory> [<resource>]` - spend that many citizens' labor at a
  structure this turn
- `end turn` - consume, transform, and ready everything
- `run <file>` - run the commands in a file, as though they had been typed in its place

And three that change nothing:

- `show <subject>` reports what is true of it and what can be done with it. For each action the
  rules permit on that subject, it says whether it is possible now, and when it is not, what is
  missing.
- `help [<command>]` - list every command, or give one command's syntax
- `history` - list every command executed so far, in order

Available only before `start`:

- `create planet <size>` - make a planet and its territories
- `add node <territory> <resource> <density>` - give a territory one node
- `set force <territory> <force>` - set a territory's force of nature
- `set biome <territory> <biome>` - give a territory its biome
- `add <unit> orbit` - place a unit in orbit before play begins
- `start` - end the design phase and begin play

A line beginning with `/` directs the front end rather than the game. `/game`, `/console` and
`/browser` choose a surface; `/new <size>` abandons the current game and starts one on a planet
of that size. `/save <file>` writes the history of the current game to a file, which `run` can
then execute. None of these is a command and none is a transition: history does not record them,
and help does not list them. A game's history begins when the game does.

## Errors

A command that cannot be run says why, and says it in terms of the game rather than the parser.
A rejection names what was wrong, where, and what was expected instead.

## Open questions
