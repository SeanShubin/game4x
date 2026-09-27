# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-576 - `H3` moved the rendering and left the source, so the rules you author are still not yours

**to** sean · **status** open · **raised** 2026-09-27 · **asks** approval · **kind** recovered · **shape** an instruction · **into** `spec/data/`, and `crates/game-model/data/`

**`P-563` moved the wrong file and this lane wrote the instruction.** It said *move `rules.4x` and
`schema.4x` from `crates/game-model/data/foundation/` into `spec/data/`* - **and `foundation/` is
the generated one.**

```
crates/game-model/data/friendly/rules.4x   703 lines   THE SOURCE, still the code lane's
spec/data/rules.4x                         703 lines   the conversion of it, now yours
```

**`tests/directories.rs` says so in its first line**: *`data/friendly/` is the source and
`data/foundation/` is what it converts to*, quoting you on 2026-09-15 - *lets make friendly the
source and not omit anything.*

## So `H3` bought the opposite of its own argument

**`P-563`'s reason**: *carrying the design across as it stands would put the game's rules in the one
column Sean does not author.* **What landed put a rendering in your column and left the authored
rules in theirs.** You can read `spec/data/rules.4x`; you cannot usefully edit it, because editing
the conversion is editing the output.

**Nothing is broken and nothing noticed**, because the two are held equal row for row by
`directories.rs`. **A check that they agree is not the same as the right one being yours.**

## And it blocks `N1`, which is why it surfaced now

**You chose `N1`: write the four missing design rules and review them.** Writing a rule means
editing the friendly source - **which is the code lane's column**, so this lane cannot write them
and you would not be authoring them.

## The instruction

**Swap which form lives where.** `spec/data/` takes the friendly files, which are the source;
`crates/game-model/data/` keeps the converted foundation form as generated output. **Nothing else
changes** - `directories.rs` still asserts the two agree, and `foundation.rs` still ships the
foundation form by `include_str!`.

## How to tell it was carried out

```
spec/data/rules.4x, spec/data/schema.4x   the FRIENDLY form - {input rule:move of:unit},
                                          names rather than numbers
crates/game-model/data/                   holds the converted foundation form and nothing
                                          authored
directories.rs                            still green, converting one into the other
the game                                  still builds and plays from them
```

## What this does not settle

**Whether the foundation form should be committed at all**, now that it is generated output in a
lane's own column. `CLAUDE.md` says a generated file has no owner. **It is left alone because the
binary `include_str!`s it** and changing that is the code lane's business.
