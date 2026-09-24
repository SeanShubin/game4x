# Specification

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Root README](../README.md) · [Documentation map](../docs/README.md) · [Notes](../docs/notes/README.md)

What the game **is**, stated normatively, **and the shape of the thing that runs it**. If a rule
is not written here, it is not decided, no matter how thoroughly it was discussed.

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
| [Control](control.md)       | How a game is won and lost                                       |
| [Force](future/force.md)    | **A future plan.** Force, garrisons, and holding ground          |
| [Interface](interface.md)   | What the player sees and can reach                               |
| [Console](console.md)       | The command language                                             |
| [Combat](combat.md)         | Ranges, weapons, resolution                                      |
| [Orbit](orbit.md)           | The orbital layer and what sits in it                            |
| [Scenarios](scenarios.md)   | The scenarios that demonstrate the game, and what each is for    |

Add a file when a topic firms up. Add its row here first.

## Rules for this directory

1. **Present tense, normative.** "A missile has a range." Not "a missile could have."
2. **One topic per file.** If a file grows its own table of contents, split it.
3. **If it is not here, it is not decided.** Discussion is not decision.

   **A test is the primary statement.** What the game does is decided by a test that runs, read
   and approved one at a time, and a rule the tests assert is not written in prose as well.
   **Prose says what a test cannot** - what a thing is for, why a rule is the shape it is, and
   anything with no observable behaviour to assert. **The game's data is decided in its data
   file**, reviewed by hand and locked by the scenario test. None of the three is decided in a
   discussion, in a note, or in a rendering of any of them.

   **Where prose and a test disagree, the test is right and the prose is a defect.** Prose is
   the one of the three that can drift without anything noticing.
4. **A document says what the game is, or it says what the game will be, and it says which.**
   What is built and asserted by a test is the specification. **What is wanted and unbuilt is a
   future plan** - kept, linked and findable, and not mistaken for a rule anything obeys today.
   **The two are told apart by where a document sits**, not by a reader remembering which is
   which.
5. **Reasoning lives in [notes](../docs/notes/README.md).** State the rule here; link down
   for why. Keep this directory short enough to hold in your head.
6. **Open questions go at the bottom of the file**, under that heading, never scattered.
7. **Record what was rejected** when the rejection is load-bearing.
8. **Relationships in prose, data in data files, and both are in this directory.** State that a
   predator has more force than a scavenger; **state the game's data in several files in a
   directory of their own**, in the notation rather than in a table. **What the specification
   states is the default.** Tuning happens in the editor and does not touch the specification, and
   a tuned value becomes the default only when I say it does.
9. **A document says what the game is, or how the thing that runs it is shaped.** Rule 4's two
   kinds are about the game; an architecture document says what is true of the artifact. **A
   boundary stated here is one the build keeps**, and the check that fails when it stops being
   kept is part of stating it.

   **A check on the artifact's shape lives outside the column it constrains.** A constraint the
   constrained lane may weaken is a constraint nobody is holding, so an architecture check is the
   specification's and not the code's - and it runs in the same gate, because a check the
   constrained lane never runs is no better.
