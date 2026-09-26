# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-563 - the game's rules now ship from the one column you do not author, and rule 8 says where they go

**to** sean · **status** open · **raised** 2026-09-26 · **rewritten** 2026-09-26, with the size arguments taken out · **asks** a decision · **kind** entailed · **into** where `rules.4x`, `schema.4x` and `engine.4x` live, and `spec/data/`

**Three answers, separated by which files name a game noun.**

```
H1  all three move to spec/data/                 engine.4x goes too, and it is the one
                                                 file with nothing of yours in it
H2  all three stay in crates/game-model/data/    the game's rules stay in the column the
                                                 code lane writes and you do not author
H3  rules.4x and schema.4x move, engine.4x stays the split follows the measurement below
```

## The measurement, and it is about ownership rather than size

```
rules.4x     {rule id:1 name:move}       the game's rules           yours
schema.4x    82 lines name a game noun   the game's kinds, traits   yours
engine.4x     0 lines name a game noun   the engine's primitives    the code lane's
```

**`engine.4x` naming no game noun is checked rather than intended** - `docs/architecture.md` rule
12, and `tests/isolation.rs` fails if it stops being true. **That is what makes `H3` a boundary
that already exists rather than a line drawn for this.**

## Why it is live now

**As of `c438986a` the game ships these files.** `crates/game-model/src/foundation.rs` carries them
with `include_str!` and builds a `Game` from them, so they stopped being a prototype's fixture and
became what a player's build runs.

## What `spec/data/` holds today, which is the part that makes room

**Its eleven files are a rendering and not a source**, generated from `releases/first-release.md`.
`C-144` measured that nothing reads them at run time: deleting a row changes no game, reddens a
test, and the next generator run puts it back. **When the release's tables stop being the ruleset
that rendering has no subject**, so the directory is free exactly when the new ruleset needs it.

**Nothing currently orders it to go.** `releases/rules-become-data.md` never mentions `spec/data/`;
`D-3` forbids a transcription *in Rust* and says nothing about a rendering in your own directory.
**Whichever answer you pick, that deletion needs ordering**, and this lane will file it rather than
leave it to be noticed.

## What changes for you afterwards, which is the one thing worth weighing

**Under `H1` or `H3`, a rule changes by promotion** - today the code lane edits `rules.4x` and
afterwards a rule change is something you read. **Rule 8 already draws that line and it may be the
line you want**: *tuning happens in the editor and does not touch the specification, and a tuned
value becomes the default only when I say it does.* So trying a number costs nothing and changing
a rule costs a reading.

**Under `H2` the reverse**: the code lane can change what the game does without you, and `D-1`'s
observable - *I change a recipe by editing a data file* - is satisfied by a file you do not own.

## What this lane would say

**`H3`**, because rule 8 already says it and the file it leaves behind is the one the architecture
already forbids from naming anything of yours. **Not because it moves less than `H1`.**

**Neither answer gates the switch.** The engine reads these files through `include_str!` from
wherever they sit, so where they live does not hold up getting your ruleset running, and the code
lane should not wait on this.

### P-565 - `spec/tests/` is a byte-identical copy of `reviewed/` with no stated status, and rule 3's lock is being deleted

**to** sean · **status** open · **raised** 2026-09-26 · **asks** a decision · **kind** entailed · **into** `spec/README.md` rule 3, and whether `spec/tests/` exists

**You said the approved tests are canonical, and rule 3 already says so** - *a test is the primary
statement*, *where prose and a test disagree, the test is right*. **Two things it does not say, and
your sentence is what makes them matter.**

## One - what is `spec/tests/`?

```
spec/tests/   54 files
reviewed/     54 files, the same 54 names, 0 differing in content
```

**Rule 3 names `spec/tests/` once and only to say the rendering does not come from it.** So the
normative directory holds an exact copy of the canonical one, nothing says which is which, and
nothing would notice them drifting.

```
T1  delete spec/tests/         reviewed/ is canonical and a copy of it is the clutter
                               you just said to keep out. CLAUDE.md names spec/tests/,
                               so this needs your approval there too
T2  keep it, and say what      spec/tests/ is where a test is written and reviewed/ is
    it is                      where one that has been read lands. Rule 3 gains a sentence
T3  delete reviewed/ instead   the review application writes it and no lane may touch it;
                               this is listed so the option is visibly considered, not
                               because this lane recommends it
```

**This lane would say `T2`.** The two directories are not a duplicate but a before and an after -
a test you have not read yet has to live somewhere, and `reviewed/` is exactly the set you have
read. **They are identical today because you have read all 54.**

## Two - the data's lock is being deleted

**Rule 3**: *The game's data is decided in its data file, reviewed by hand and locked by the
scenario test.* **`P-564`'s `D-4` deletes the scenario**, so that clause becomes false when it
lands.

```
L1  the reviewed tests are the lock      they exercise the data the engine reads, and
                                         they are the thing you have read
L2  name a new scenario                  a scenario over the new ruleset, written later
L3  drop the clause                      the data is reviewed by hand and locked by nothing
```

**This lane would say `L1`**, and it is nearly a restatement rather than a new rule: rule 3's own
first sentence already makes a test the primary statement, and the reviewed tests run against the
data the engine reads.

## What this is not

**It does not ask whether the approved tests are canonical.** You have said they are and rule 3
agrees. **It asks what the other directory is for, and what locks the data once the scenario is
gone** - neither of which rule 3 answers today.
