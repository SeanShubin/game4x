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

## Out of scope

- **The drawing.** `crates/` keeps the planet, the reports and the console. Sean, 2026-09-21:
  *all I really care about on the mainline is the rendering work*
- **The console's own shape.** It operates on the new model and is not rewritten. Sean,
  2026-09-25: *I also expect the current game console is going to operate on the thin-engine
  design that gets into mainline*
- **`releases/first-release.md`.** Its seven built capabilities still wait on you, and nothing
  here changes what they assert

## Open questions
