# Interface

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

What the player sees and can reach. The planet's own presentation is in [the planet](planet.md);
the command language is in [console](console.md).

## Surfaces

The game presents four surfaces, all reachable from the front end, in every build:

- **The game itself** - the planet, its territories, and what the player does with them
- **The console** - typing commands, with help listing every command and its syntax, and
  errors that say what was wrong and what was expected
- **The data browser** - the game's own data, read directly, in two views: every entity with its
  components, and the same facts normalized into a table for each relation. **Both name every
  table and every column, whether or not anything is in it.**
- **The rule editor** - the rules the player has, read and changed

The rule editor is two-dimensional. It may carry three-dimensional decoration, and nothing the
player has to read or act on is in that decoration.

## Availability and presentation

Nothing is available in one build and not another.

How a thing is presented, and how the user acts on it, may follow the platform it runs on. A
console is a terminal on the desktop and part of the page on the web; turning the planet is a
drag with a mouse and a finger on a touch screen. What the user can do is the same either way.

Actions that are not a manipulation of the planet - resetting the view, reaching a surface,
choosing a planet size - never require a gesture or a key the platform may lack.

## What an action shows

- An action that would waste part of what it costs says so before it is taken, and says how much

## Open questions
