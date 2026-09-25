# Interface

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

What the player sees and can reach. The planet's own presentation is in [the planet](planet.md);
the command language is in [console](console.md).

## Surfaces

The game presents five surfaces, all reachable from the front end, in every build:

- **The game itself** - the planet, its territories, and what the player does with them
- **The console** - typing commands, with help listing every command and its syntax, and
  errors that say what was wrong and what was expected
- **The data browser** - the game's own data, read directly, in two views: every entity with its
  components, and the same facts normalized into a table for each relation. **Both name every
  table and every column, whether or not anything is in it.**
- **The rule editor** - the rules the player has, read and changed. **It is for automating the
  game to the player's preference**: how a scout behaves, a script for taking a planet, a supply
  line. **It is the only gameplay surface where the player types text to name things**, and the
  game can be played through without ever opening it.
- **The debug view** - how the data is actually held, shown as it is held. **It is an output and
  not a path**: nothing in the game reads through it, and it is the one place the layout every
  other surface is shielded from can be seen

The rule editor is two-dimensional. It may carry three-dimensional decoration, and nothing the
player has to read or act on is in that decoration.

**A value that would be compared across rows is a column; a value that would not may be a node in
a cell.** So the normalized view has no nested cells - that is what normalizing is - and the entity
view may have them, because it groups a thing's parts into one row.

## Availability and presentation

Nothing is available in one build and not another.

How a thing is presented, and how the user acts on it, may follow the platform it runs on. A
console is a terminal on the desktop and part of the page on the web; turning the planet is a
drag with a mouse and a finger on a touch screen. What the user can do is the same either way.

Actions that are not a manipulation of the planet - resetting the view, reaching a surface,
choosing a planet size - never require a gesture or a key the platform may lack.

## What an action shows

- An action that would waste part of what it costs says so before it is taken, and says how much
- A control that chooses among alternatives shows which one is chosen, and names the others.

## What is offered

**What the player is offered is an end result and not a step towards one.** Building a Pioneer,
building a bin and moving a unit are offered; spending the labour they cost is not. **What a
choice costs is paid by whatever the rules require**, without being chosen a second time.

**And what is obvious is done without being asked.** Ending a turn is the player saying they
have finished choosing, so at that point every extractor with somewhere to put what it makes is
worked - spending labour, and spending citizens to make labour, as far as it will go.

**None of this is a rule of the game.** There is no rule that tops off a bin. What is offered,
and what is done unasked, is a layer above the rules that writes the commands a player would
have written. **The rules say what is legal; this says what is worth showing.**

## Logs

**Everything that happens is logged, in files under `logs/`.** A log is not a record of the game
- `history` is that - and it is not a debugging convenience either. **It is a surface to review**:
what the application did is read from it rather than described by whoever was watching.

- **`logs/input.txt`** - what the player's device did: where the pointer was, where the window
  was, what was clicked and pressed
- **`logs/commands.txt`** - every command, each line saying which kind it is: a **game command**,
  a **local command** or a **query**. **The history is the game commands**, so `history` and this
  cannot disagree about what happened
- **`logs/screen.txt`** - what is on the screen: the viewport, which screen and which view, each
  region, and each thing in it with its position. **Every heading states how many it holds**, so
  a section that holds nothing says so rather than being blank

**The logs are emptied when the application starts**, so that what is in them is what the
version now running did.

**A log carries positions and an interface test carries structure.** The test says what regions
exist and what is in them; the log says where they were drawn. **So a test survives a change of
layout and a log is enough to see what was on the screen**, and neither is asked to do the
other's work.

## Open questions
