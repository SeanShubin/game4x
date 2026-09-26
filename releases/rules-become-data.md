# Release: Rules Become Data

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Releases](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

## Goal

I can change what the game does by editing data, and the game plays by the tests I have read.

## Scope

The thin-engine design becomes the mainline's. **The crate does not become a dependency** -
Sean, 2026-09-25: *I don't mean to actually delegate to thin-engine*. Whether its code moves
across or is written again is the code lane's to choose, and no capability here rests on which.

## Capabilities

### D-1 - A rule changes when I edit data, and not before

**to** code · **status** open · **raised** 2026-09-25 · **from** `P-560`

- **In** - `spec/invariants.md` -> The game is data, *every kind of thing, and every recipe that
  turns some things into others, is data rather than code*
- **Vetted when** - I change a recipe by editing a data file, with no Rust edited, and the game
  fires the changed rule; and `crates/game-model` holds no rule at all. **The measure is that it
  stops holding rules, not that it holds fewer**

### D-2 - The game plays by the tests I have read

**to** code · **status** open · **raised** 2026-09-25 · **from** `P-560`

- **In** - `spec/README.md` rule 3, *what the game does is decided by a test that runs, read and
  approved one at a time*
- **Vetted when** - the tests in `reviewed/` run against the model the game itself plays on, and
  one of them goes red when that model disobeys it. Today they run against `crates/thin-engine`
  and against nothing else

### D-3 - The game's data is stated once

**to** code · **status** open · **raised** 2026-09-25 · **from** `P-560`

- **In** - `spec/invariants.md` -> The game is data, *nothing states by hand what a data file
  says; every other form of it is derived*
- **Vetted when** - the game reads its data from the data files at run time, and deleting a row
  changes the game. No transcription of those rows survives in Rust

### D-4 - The old ruleset is gone, not archived

**to** code · **status** open · **raised** 2026-09-26 · **from** `P-564`

- **In** - `spec/README.md` rule 4, *what is built and asserted by a test is the specification*,
  and Sean, 2026-09-26: *I am confident i can re-create any old rules*
- **Vetted when** - nothing in the repository states a rule of the game except the files the
  engine reads. The hand-written ruleset, the rendering of it in `spec/data/`, the scenario that
  exercised it and the tables it was generated from are **deleted rather than moved**, and no
  test, report or document is left describing a rule the game does not play by. **A search for
  any recipe name the reviewed tests do not use finds nothing outside the history**

### D-5 - I have watched the new game play through

**to** code · **status** open · **raised** 2026-09-26 · **from** `P-564`

- **In** - `spec/scenarios.md`, *there is one main scenario, and it touches everything a typical
  game uses. It is the foundation, and it is vetted by hand*
- **Vetted when** - a main scenario exists over the reviewed ruleset and I have watched it run:
  an Ark deploys, **that first territory is developed**, a second is taken by land and
  **developed too**, and an Ark launches from the second. **Every rule a typical game uses fires
  at least once while it runs**, measured by what fired rather than by what the file says, and
  **a rule that does not fire is named with the unusual situation it needs** - so an omission is
  something I can read rather than something I have to notice. This is the observation `R-6` was
  retired without making

## Out of scope

- **The drawing.** `crates/` keeps the planet, the reports and the console. Sean, 2026-09-21:
  *all I really care about on the mainline is the rendering work*
- **The console's own shape.** It operates on the new model and is not rewritten. Sean,
  2026-09-25: *I also expect the current game console is going to operate on the thin-engine
  design that gets into mainline*
- **`releases/first-release.md`.** Its seven built capabilities still wait on you, and nothing
  here changes what they assert

## Open questions
