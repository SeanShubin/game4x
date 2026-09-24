# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-548 - Two sentences close the gaps in who writes what, and a check keeps them closed

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** text · **asks** approval · **into** `CLAUDE.md` -> Perspectives

**You answered both cells, and gave a better reason than this lane's for one of them.** The 43-row
table is not what lands - it restates what the columns already say and goes stale the next time a
directory is added. **What lands is your two sentences and a check.** The table is kept in
[`docs/notes/decisions.md`](../docs/notes/decisions.md) as the measurement it was.

## What lands

**After the column table:**

> **`decide/` is the specification lane's.** The specification puts things there for Sean to read,
> and they are removed from there according to his interactions with that lane - so it is written
> by the lane whose queue it is, and nothing is filed there for another instance.

**And after the production-support paragraph:**

> **The pipeline and the local build belong to the code lane too.** `.gitignore`,
> `.gitattributes`, `.git/`, `.idea/` and `target/` are mechanical details of how things get
> implemented rather than production support proper, **and that is near enough** - the lane that
> implements owns how implementing works.

## What the check does, and what it would have caught

**`tools/spec` asserts that every top-level path is owned by exactly one column**, reading the
directory rather than a list. **A list is the thing that went stale**: the Code row named
`commands/` for nineteen days after `ddbaed66` deleted it.

```
would have caught   .idea/, lenses/, tools/, .git/    four paths this lane missed
                    by hand, and the assertion caught
will catch          the next directory anybody adds
```

**This is the half that makes the sentences worth landing rather than just being true.** Written
after `P-546` lands, since that settles where such a check lives.

## Why not the table

**`CLAUDE.md` says a consequence belongs in the spec only when another rule leans on it**, and no
rule leans on *`docs/theory/` is the specification lane's* - it follows from `docs/`. **Forty-three
rows would need editing on every new directory**, which is the failure the check removes.

### P-546 - The architecture check lives in `tools/spec/`, where the lane it constrains cannot edit it

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/README.md` -> Rules for this directory

**You chose `C1`.** This is the rewrite; the working, including the false premise the first
version nearly carried, is in [`docs/notes/decisions.md`](../docs/notes/decisions.md).

## What lands

**Appended to rule 8**, which `P-545` adds:

> **The check that holds a boundary lives outside the column it constrains.** A constraint the
> constrained lane may weaken is a constraint nobody is holding, so an architecture check is the
> specification's and not the code's - and it runs in the same gate, because a check the
> constrained lane never runs is no better.

## Why this is one sentence and not a path

**A path in `spec/` would go stale the first time a tool moved.** The rule is the property -
outside the column, inside the gate - and `tools/spec/tests/architecture.rs` is where that
property is satisfied today. **`docs/architecture.md` records the location**, which is the layer
that may change without your approval.

## What it commits you to

**A failure the code lane cannot repair.** They report it and leave it, which is already the rule
in the other direction: this lane does not edit code *even to fix an obvious break*. **`C1` makes
the arrangement symmetric**, and the round trip is the cost.

**And the first such failure is already waiting.** Rule 4 is broken - measured, four crates, filed
as `S-160` - and `P-547` asks you what rule 4 should say before a check is written to enforce it.
**A check written against a rule that turns out to be wrong is worse than no check**, because it
makes the rule look held.

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
