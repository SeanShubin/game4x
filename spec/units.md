# Units

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

What is true of every unit. Particular units are in [unit types](unit-types.md).

## What a unit is

- Each unit is of a certain type, which remains even if control of the unit changes
- Each type of unit has a unique name
- Each unit has a strength
- A unit may require upkeep each turn, and is lost if it is not paid
- A mobile unit that moves over the ground has a bin for fuel. **It is built with that bin full,
  and the energy is paid where it is built.** Moving burns a unit of it, and one with an empty
  bin cannot move
- **A mobile unit that moves in orbit gathers its energy from the sun**, a fixed amount each
  turn, and holds it in a bin of its own. **Moving in orbit burns a unit of it**, and one with
  an empty bin cannot move. **The sun is where that energy comes from**, so orbital movement is
  never paid for out of a territory.
- Fuel moves freely between a controlled territory that has it and anything there that can hold it

## Open questions
