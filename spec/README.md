# Specification

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Root README](../README.md) · [Documentation map](../docs/README.md) · [Notes](../docs/notes/README.md)

What the game **is**, stated normatively. If a rule is not written here, it is not decided,
no matter how thoroughly it was discussed.

## The documents

| Document                    | What it specifies                                                |
| --------------------------- | ---------------------------------------------------------------- |
| [Invariants](invariants.md) | Statements that are always true; every other document obeys them |
| [Narrative](narrative.md)   | The fiction the rules implement                                  |
| [The planet](planet.md)     | The sphere, its territories, and what a territory carries        |
| [Resources](resources.md)   | The list of resources                                            |
| [Structures](structures.md) | The list of structures and what each one does                    |
| [Units](units.md)           | What is true of every unit                                       |
| [Unit types](unit-types.md) | Each particular unit, one section apiece                         |
| [Economy](economy.md)       | Extraction, structures and labor                                 |
| [Logistics](logistics.md)   | Where materials are, and moving them to where they are needed    |
| [Population](population.md) | Citizens, how they grow, and the labor they provide              |
| [The turn](turn.md)         | The order in which a turn resolves                               |
| [Control](control.md)       | Force, coordination, and how territory is claimed and held       |
| [Interface](interface.md)   | What the player sees and can reach                               |
| [Console](console.md)       | The command language                                             |
| [Combat](combat.md)         | Ranges, weapons, resolution                                      |
| [Orbit](orbit.md)           | The orbital layer and what sits in it                            |

Add a file when a topic firms up. Add its row here first.

## Rules for this directory

1. **Present tense, normative.** "A missile has a range." Not "a missile could have."
2. **One topic per file.** If a file grows its own table of contents, split it.
3. **If it is not here, it is not decided.** Discussion is not decision.

   A rule is decided here. **The game's data is decided in its data file**, reviewed by hand and
   locked by the scenario test. Neither is decided in a discussion, in a note, or in a rendering
   of either one.
4. **Reasoning lives in [notes](../docs/notes/README.md).** State the rule here; link down
   for why. Keep this directory short enough to hold in your head.
5. **Open questions go at the bottom of the file**, under that heading, never scattered.
6. **Record what was rejected** when the rejection is load-bearing.
7. **Relationships here, data elsewhere.** State that a predator has more force than a
   scavenger; state the game's data in a data file, where it can be tuned without touching the
   specification.
