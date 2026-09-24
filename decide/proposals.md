# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-547 - The engine adapter is a layer, and rule 4 has to say so too

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `docs/architecture.md` -> The layers, and Rules

**You chose `A1`.** The working is in [`docs/notes/decisions.md`](../docs/notes/decisions.md).

**Two blocks, and the second is why the promotion sweep was worth running.** Widening the layer
row alone leaves rule 4 reading *engine types live only in **the** adapter* - the exact sentence
that made this lane report four violations where there were two.

## What lands

**The `Engine adapter` row of the layer table**, in its *Knows about* cell:

> Bevy, windows, input, vsync, and what the game is when a surface has to follow it

**And rule 4's first sentence becomes:**

> 4. **Engine types live only in the adapter layer.** No `bevy::` anywhere else, including in
>    the composition root's own logic - the root may assemble plugins, but it may not compute
>    with engine types.

## How to tell it was carried out

**Three assertions in the promoting commit.** The layer row's *Knows about* cell reads the new
text; rule 4 says *the adapter layer*; and **the `bevy::` population is unchanged at five
crates** - because this is a change to what is written and not to what is built.

## What it settles and what it leaves red

**Settled**: `planet-bevy`, `planet-flat` and `planet-ecs` are the adapter layer, and `game-globe`
joins them - engine code that knows the game, which the widened row now admits.

**Still red**: `crates/game4x/src/inspect.rs`, which writes a Bevy plugin in the composition root
where rule 4 allows assembling one. **That is `S-160` and needs nothing further from you** - the
fix is a crate of its own that `game4x` adds, keeping its header's promise that *the same binary
plays and poses*.

**And one sentence in the code lane's column is still false**: `crates/planet-bevy/README.md` says
it is *the only crate in the project that knows a graphics engine exists*. Four others do. Also
`S-160`.

## What this does not settle, stated so it is not lost

**Nothing maps a crate's *Kind* onto a layer.** The crate table uses seven kinds - algorithm,
model, entities, view model, view, binding, binary - and the layer table has four layers, and no
sentence connects them. **After this lands, `planet-ecs` is *entities* and `game-globe` is
*binding* and both are the adapter layer**, which a reader can only work out the way this lane
did: by reading rule 6 and inferring.

**Filed as `S-161` rather than fixed here.** It is a table this lane can propose once, and it is
not what you were asked about.

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

> **The pipeline and the local build belong to the code lane too.** `.gitignore` and
> `.gitattributes` are mechanical details of how things get implemented rather than production
> support proper, **and that is near enough** - the lane that implements owns how implementing
> works. **What a tool writes for itself is owned by nobody**, on the rule above: `.git/`,
> `.idea/` and `target/` have no owner because no instance edits them.

## Where this differs from your sentence, and why

**You named five paths and this lands two.** `.git/`, `.idea/` and `target/` are written by git,
an IDE and cargo, and nobody edits them - so giving them an owner contradicts the rule five
paragraphs above, **a generated file has no owner... nobody edits it**. `.gitignore` and
`.gitattributes` are hand-written and take your reason exactly.

**The distinction is yours to overrule.** The effect either way is nil - nothing is at stake in
who owns a directory no instance writes - but the two sentences would disagree, and this file is
one a lane reads to find out what it may do.

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

**Appended to rule 9**, which `P-545` adds:

> **A check on the artifact's shape lives outside the column it constrains.** A constraint the
> constrained lane may weaken is a constraint nobody is holding, so an architecture check is the
> specification's and not the code's - and it runs in the same gate, because a check the
> constrained lane never runs is no better.

## One word changed after the promotion sweep, and `P-548` is why

**This said *the check that holds a boundary*, which is general enough to forbid `P-548`'s own
check.** That one asserts every path in `CLAUDE.md` is owned, and `CLAUDE.md` is this lane's -
so a general reading puts the check inside the column it constrains and makes the two proposals
disagree.

**Now it says *a check on the artifact's shape*.** The narrow claim is the one you chose: a
boundary on the code, checked by somebody the code lane cannot overrule. **A lane checking its own
completeness is a different thing** and this sentence no longer speaks to it.

## It follows `P-545`, in order and in numbering

**`P-545` has to land first**, because this sentence is appended to the rule that one creates.
**And the number moved with it**: this said rule 8 while `spec/README.md` already had a rule 8.

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

**And a new rule 9**, appended after rule 8:

> 9. **A document says what the game is, or how the thing that runs it is shaped.** Rule 4's two
>    kinds are about the game; an architecture document says what is true of the artifact. **A
>    boundary stated here is one the build keeps**, and the check that fails when it stops being
>    kept is part of stating it.

## It is rule 9 because rule 8 is taken, which this lane first got wrong

**This proposal said *a new rule 8, after record what was rejected*.** Rule 8 exists - *relationships
in prose, data in data files* - and *record what was rejected* is rule 7. **So the placement would
have renumbered an approved rule without saying so**, which is the kind of quiet change promotion
exists to prevent. Appending as rule 9 renumbers nothing.

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
