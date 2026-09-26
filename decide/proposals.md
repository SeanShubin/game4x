# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-560 - the release that makes the rules data, and the specification needed nothing

**to** sean · **status** open · **raised** 2026-09-25 · **asks** approval · **kind** a work order with no rule behind it · **shape** text · **into** `releases/rules-become-data.md` - a new file

**Last turn this lane told you the migration needed a promotion first. It does not.** Checked
element by element: the specification already states the whole thin-engine design, and every
piece of it is promoted and binding.

```
rules and recipes are data, not code       spec/invariants.md -> The game is data
a definition arrives in one transition     the same section
state is things in places, with traits     the same section
nothing in the state is special to a kind  the same section
a thin engine is design pressure           the same section - P-493, your words
the engine reads the id form               spec/invariants.md -> normalized relational model, P-559
a test is the primary statement            spec/README.md rule 3 - P-558
a command names a recipe                   spec/console.md -> Commands
places have layers, nothing crosses        reviewed/nothing-moves-between-the-layers.4x
```

**So there is no rule to write, and that is exactly why the code lane has nothing to build.**
Every capability in `releases/first-release.md` is vetted or built, so the repository orders no
work at all today.

## The gap the release has to close, measured now

```
crates/game-model/src/rules.rs        1,424 lines of Rust that ARE the rules
  the same file on 2026-09-21           1,198 - grown 226 lines since you said "dumping all"
crates/thin-engine/src                4,513 lines that name no game noun
spec/data/                              234 rows, transcribed into game-console/src/declare.rs
                                              as Rust consts; nothing opens the file at run time
reviewed/                                54 tests, read by crates/thin-engine and nothing else
```

## The words, and the addressing line under each capability is written at promotion rather than offered here

> # Release: Rules Become Data
>
> **Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
> present, reporting every change; a new idea is entered by Sean himself, whether he types it
> or pastes it from a [proposal](../docs/notes/proposals.md).
>
> [Releases](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)
>
> ## Goal
>
> I can change what the game does by editing data, and the game plays by the tests I have read.
>
> ## Scope
>
> The thin-engine design becomes the mainline's. **The crate does not become a dependency** -
> Sean, 2026-09-25: *I don't mean to actually delegate to thin-engine*. Whether its code moves
> across or is written again is the code lane's to choose, and no capability here rests on which.
>
> ## Capabilities
>
> ### D-1 - A rule changes when I edit data, and not before
>
> - **In** - `spec/invariants.md` -> The game is data, *every kind of thing, and every recipe that
>   turns some things into others, is data rather than code*
> - **Vetted when** - I change a recipe by editing a data file, with no Rust edited, and the game
>   fires the changed rule; and `crates/game-model` holds no rule at all. **The measure is that it
>   stops holding rules, not that it holds fewer**
>
> ### D-2 - The game plays by the tests I have read
>
> - **In** - `spec/README.md` rule 3, *what the game does is decided by a test that runs, read and
>   approved one at a time*
> - **Vetted when** - the tests in `reviewed/` run against the model the game itself plays on, and
>   one of them goes red when that model disobeys it. Today they run against `crates/thin-engine`
>   and against nothing else
>
> ### D-3 - The game's data is stated once
>
> - **In** - `spec/invariants.md` -> The game is data, *nothing states by hand what a data file
>   says; every other form of it is derived*
> - **Vetted when** - the game reads its data from the data files at run time, and deleting a row
>   changes the game. No transcription of those rows survives in Rust
>
> ## Out of scope
>
> - **The drawing.** `crates/` keeps the planet, the reports and the console. Sean, 2026-09-21:
>   *all I really care about on the mainline is the rendering work*
> - **The console's own shape.** It operates on the new model and is not rewritten. Sean,
>   2026-09-25: *I also expect the current game console is going to operate on the thin-engine
>   design that gets into mainline*
> - **`releases/first-release.md`.** Its seven built capabilities still wait on you, and nothing
>   here changes what they assert
>
> ## Open questions

## Why it is filed now

**Six of the code lane's open items are about the encoding this release drops** - `C-47`, `C-102`,
`C-119`, `C-120`, `C-123`, `C-131` - and it has said it will not spend a day on any of them until
a release says the design is what is being built.

### P-561 - `spec/combat.md` is four scaffolding prompts, and it is the last of your three still undone

**to** sean · **status** open · **raised** 2026-09-25 · **asks** approval · **kind** entailed · **shape** an instruction · **into** `spec/combat.md` -> `spec/future/combat.md`, and `spec/README.md`

**The second of the three you accepted on 2026-09-21 is incomplete.** *`spec/` splits into what
the game is now and what it will be.* `P-556` moved control; force was already there. **Combat was
named at the time as the clearest case and never moved.**

## What is in the file, which is less than "a game nothing is building"

**Twenty-seven lines, four sections, and no rule in any of them.** Every section is the scaffolding
prompt it was created with:

```
## Scales       *Scaffolding prompt - delete this line when the section is written.*
## Range        the same
## Weapons      the same
## Resolution   the same
```

**And `spec/README.md` rule 4 decides this without anyone judging it.** *What is built and asserted
by a test is the specification; what is wanted and unbuilt is a future plan.* **No test in
`reviewed/` or `spec/tests/` mentions a weapon, a missile, an attack, a starbase or combat** -
**counted over 54 files and 54 files, zero in both, and the same search finds `ark` in 26 of the
108** - so it is a zero against a population, not a zero because nothing was read. Orbit stays live
by the same rule, because `an-ark-is-launched-from-the-ground-into-the-orbit-above.4x` asserts it.

## The instruction

**Move the file to `spec/future/combat.md`, unchanged apart from the two relative links its new
depth requires** - `../docs/notes/proposals.md` becomes `../../docs/notes/proposals.md`, and the
header line becomes `[Specification](../README.md) · [Root README](../../README.md)`, which is
exactly what `spec/future/control.md` carries. **Then repoint its row in `spec/README.md` and
prefix it the way the other two are marked**, so the row reads its path as `future/combat.md` and
its description begins `**A future plan.**`.

## How to tell it was carried out

```
spec/combat.md                    does not exist
spec/future/combat.md             exists; its body is byte-identical apart from the two links
spec/README.md                    the Combat row points at future/combat.md and begins "A future plan."
a repository-wide search          no link anywhere resolves to spec/combat.md
```

**One link outside `spec/` needs following**: `docs/README.md:26` lists it in the documentation
map. That file is this lane's and the fix needs no approval; it is named here so the count above is
the whole of it.
