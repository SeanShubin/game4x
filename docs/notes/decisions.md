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

### P-492 - A fuel tank, and containment may already be one

**to** sean · **status** open · **raised** 2026-09-13 · **rewritten** 2026-09-13, on your answer · **kind** recovered · **shape** rows · **asks** a decision · **into** `releases/first-release.md` -> Recipes, Kinds, and What bounds a kind in a territory

**Your answer, 2026-09-13**: *what about having the pioneer contain a fuel tank that can contain
fuel?*

**A, B and C are withdrawn.** All three argued about which trait a qualifier could name. You changed
the subject to what a pioneer *contains*, and containment already has the vocabulary the qualifier
was missing.

## What was checked before writing this, because it changes the size of the answer

`spec/logistics.md` -> Containment: **what a thing may contain is a maximum per kind, and the three
names are its capacity for that kind, how much is occupied, and how much is free.** The trio is
**per kind contained** rather than a trait of the container - so anything that declares a capacity
for energy has free energy capacity already, with nothing added.

And the release says the same from the other end. *What bounds a kind in a territory* gives
**energy** as bounded by *the things in it that hold it*, and: **a raw material is in one of three
states: its source, disorder, or held by something that declares a limit for it.** A store holds
what it was built to hold. **A fuel bin is that sentence applied to a unit.**

## So there are two ways to do what you said, and the difference is whether the tank is a thing

**`D1` - the pioneer declares a capacity for energy.** No new kind. `refuel` reads `require 1 unit |
free at least 1 | $where`, naming `free`, which containment defines. The `fuel` trait retires: *how
big the bin is* becomes the declared capacity, stated once in *What bounds a kind in a territory*
beside `garrison`'s 1 and `ark`'s 2.

**`D2` - the pioneer contains a `tank`, and the tank declares the capacity.** A new kind, and the
first thing in this game to have a part. `refuel` requires a tank with `free at least 1` in the
unit. **The release already has the word**: `metal in it` is *its binding plus the metal in its
parts*, and nothing has parts yet.

## What `D2` buys, and it is one thing

**Two units of one kind could hold different amounts of fuel**, because the tank would be what
carries the capacity and a thing may hold different things. Under `D1` they cannot: *what a kind may
contain is a fact about the kind and not about any one of them*, so every pioneer has the same bin
for ever.

**Nothing in the release wants that today** - there is one pioneer kind and one ark kind. It is a
door rather than a feature, and `D2` is the price of leaving it open.

## What this lane would pick, and the reason is not economy

**`D1`.** Not because it is smaller, but because **`D2` states the same fact twice.** A tank whose
only property is a capacity for energy *is* the pioneer's capacity for energy, wearing a kind's
name - and `spec/invariants.md` says a fact is stated once. `D2` earns its place the moment a tank
can differ, be damaged, be built separately or be counted in metal; until then it is a level of
containment that holds exactly one thing and decides nothing.

**The question is therefore whether you want the door**, and that is not a thing this lane can read
off the files.
