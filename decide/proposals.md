# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-545 - `spec/` covers the shape of the artifact, not only the rules of the game

**to** sean · **status** open · **raised** 2026-09-23 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/README.md` -> the opening line, and Rules for this directory

**You decided this on 2026-09-23**, choosing it over leaving the architecture advisory. These are
the words.

## What lands

**Replacing the opening line**, which today says only the game:

> What the game **is**, stated normatively, **and the shape of the thing that runs it**. If a rule
> is not written here, it is not decided, no matter how thoroughly it was discussed.

**And a new rule 8**, after *record what was rejected*:

> 8. **A document says what the game is, or how the thing that runs it is shaped.** Rule 4's two
>    kinds are about the game; an architecture document says what is true of the artifact. **A
>    boundary stated here is one the build keeps**, and the check that fails when it stops being
>    kept is part of stating it.

## Why the second block is a new rule and not a wider rule 4

**Rule 4 sorts documents into *is* and *will be*.** An architecture document is neither of those
about the game, so widening rule 4 would have to say *the game or the code* twice inside a
sentence whose whole job is a two-way split. **The new rule names the third kind and leaves rule 4
alone.**

**Nothing is justified in either block**, because rule 5 puts reasoning in the notes. The reason
is the one you gave for this whole direction: a constraint on the code that sits where Claude may
reword it without approval is a constraint nobody is holding.

## What this promises that does not exist yet

**There is no `spec/architecture.md`.** The rules are in `docs/architecture.md`, seventeen of
them, and this lane classified them by one question - *can a program decide it by reading the
repository?*

```
already checked      7   11, 12, 13, 15, 16, and 3 and 5, which this lane first miscounted
checkable, unchecked 6   1, 2, 4, 6, 7, 17
judgement            4   8, 9, 10, 14
```

**This lane first wrote that no test asserts any crate boundary, and that was wrong.** Three do:

```
rule 3  crates/game-model/src/lib.rs:57     no_floating_point_anywhere
rule 3  crates/planet-model/src/lib.rs:98   the same check in the second model crate
rule 5  tools/outbox/tests/architecture.rs  every workspace crate is named in the document
```

**The instrument searched `crates/*/tests/` and these are `#[cfg(test)]` tests inside `src/`.**
So it answered *do the test directories mention it* and was read as *is it checked* - and it
returned a plausible zero rather than an error, which is the failure this repository keeps
naming.

**What is not found is weaker than what is checked.** The six above are *no named check was
found*, by searching for test functions that collect offences - not a proof that none exists.
`bevy::` is the one worth a second look: **five crates name `bevy` in their `Cargo.toml`** -
`game-globe`, `game4x`, `planet-bevy`, `planet-ecs`, `planet-flat` - where rule 4 says engine
types live in *the* adapter, singular.

**The migration is its own work and not this proposal.** Filed as `S-159` so the gap sits in an
outbox rather than in this paragraph. **Promoting this makes `spec/` promise a document that is
not there yet**, which is worth knowing before you say the word.
