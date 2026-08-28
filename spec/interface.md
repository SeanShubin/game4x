# Interface

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

What the player sees and can reach. The planet's own presentation is in [the planet](planet.md);
the command language is in [console](console.md).

## Surfaces

The game presents three surfaces, all reachable from the front end, in every build:

- **The game itself** - the planet, its territories, and what the player does with them
- **The console** - typing commands, with help listing every command and its syntax, and
  errors that say what was wrong and what was expected
- **The data browser** - every entity in the game and its components, read directly

## Availability and presentation

Nothing is available in one build and not another.

How a thing is presented, and how the user acts on it, may follow the platform it runs on. A
console is a terminal on the desktop and part of the page on the web; turning the planet is a
drag with a mouse and a finger on a touch screen. What the user can do is the same either way.

## Open questions
