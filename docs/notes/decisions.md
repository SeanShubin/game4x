# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-439 - the release's data has no file to come from, and the order is yours

**to** sean - **status** open - **raised** 2026-09-12 - **kind** recovered, from the code lane's `C-49` - **asks** a decision - **into** `releases/first-release.md`, and `S-30`

**This has been the code lane's to raise since 2026-09-06 and it has been this lane's to put to you
since then. It never was.** It is the one thing between the scenario working - which it does - and
the release being fully specified.

## What it is

**`releases/first-release.md` holds the game's data, in nine tables** - Scope, Kinds, Families,
Traits, What bounds a kind in a territory, Where things are, Units and structures, Recipes, Biomes -
counted from the file. **Your own rule says it should not.**

- **`spec/README.md` rule 7**: *relationships here, data elsewhere… state the game's data in a data
  file, where it can be tuned without touching the specification*
- **`releases/README.md`**, from `P-224`: *a release states scope, capabilities and what is
  deliberately left out, in prose. **It does not contain the game's data.** Where a release needs to
  show data, it links to the generated view of it*

**`S-30` asks for the data file those tables are generated from, and says the data file comes
first.** The release says the rule comes first. **Both are right and they cannot both be first.**

## Why the code lane stopped rather than starting

**It may write the file and it may not empty the release.** `scenario/` and `crates/` are its
column; `releases/first-release.md` is not. **So between the two moments the game's data exists
twice**, once in your document and once in its file - which is the state `S-30` exists to end.

**And the deeper reason is the one that makes this yours.** The content of that file would be **your
ideas transcribed by another lane**, and a transcription that becomes canonical is a promotion done
by the wrong hand. A cell-for-cell check would stop them diverging **and would not make the
transcription yours**.

## The two orders

1. **You author the data file, and the tables leave in the same change.** The data is never
   transcribed and never exists twice. The code lane builds the loader and the generated view
   against what you wrote. **The cost is that nine tables' worth of data is yours to move.**

2. **The code lane transcribes first**, holds the file to the release with a check that fails in
   both directions, and **the tables leave afterwards, explicitly** - a separate moment at which you
   say *that file is the data now*. **The cost is a window in which your data lives in a file you
   did not write**, with a check as the only thing keeping the two honest.

**This lane has no recommendation and will not apply `P-428` here.** Twice today the unified form
was the wrong answer and both times this lane reached for the rule rather than asking - and this is
a question about **who authors what**, which is the one subject `CLAUDE.md` puts beyond a rule of
thumb.

## What is true either way

**Nothing is blocked today.** The scenario plays, all 21 recipes fire, the gate is green. `S-30`'s
measured half is done and recorded: the checks that read the release **do not go green when the
tables leave**, so whatever order you pick, the loss of a table is noticed rather than silent.

**And two smaller instances of the same thing are already waiting.** `spec/resources.md` gives food
`Lasts` **1** and the release says *food is made with `keeps` 1*; `spec/control.md` held the number
`1` for a citizen's strength until `P-437` took it out this morning. **The same defect, one cell at a
time** - which is what makes the nine-table version worth answering rather than living with.

