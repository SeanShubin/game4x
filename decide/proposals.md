# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-567 - why the relational model matters, which `spec/` states as a property and not as a reason

**to** sean · **status** open · **raised** 2026-09-26 · **asks** approval · **kind** recovered · **shape** text · **into** `spec/invariants.md` -> The data is a normalized relational model, after the first bullet

**Your words, 2026-09-26**: *The relational model is important because it is what guarantees our
model is coherent, even though it could still be inaccurate. An incoherent model can not be
accurate.*

**The file gives the property and not the argument.** Its first bullet says the data is fully
normalized and that this makes *one consistent model with no duplication*. **Your sentence says
why that is worth paying for**, and it carries two halves the file does not: coherence is
**necessary** for accuracy, and it is **not sufficient**.

**Measured: `coherent`, `accurate` and `accuracy` appear zero times in `spec/`.**

## The words

**One bullet, after *the game's data is a set of fully normalized relations*:**

> - **The relational model is what guarantees the model is coherent.** It could still be
>   inaccurate - coherence is not correctness - but **an incoherent model cannot be accurate**, so
>   this is the part that has to hold before accuracy is worth asking about.

## Why this is worth a promotion rather than a note

**It bounds a claim as much as it makes one.** Without the second half, a reader can take a
normalized model as evidence the game is right. **The sentence says plainly that structure cannot
buy that** - which is what leaves the reviewed tests as the thing that can, and rule 3 already
says they are the primary statement.

**And you said reasons are load-bearing.** 2026-09-25: *reasons being discoverable by an AI
Assistant has made the AI Assistant much more capable of giving me good recommendations.* This is
the reason under a rule that `a fact is stated once`, the notation bullet and `P-559`'s engine
bullet all lean on.

## Where it did work today, before it was written down

**It decided the one-relation-per-file question an hour ago.** `relations.rs` asserts one relation
per file to stop a rendering with blank cells; you answered *no nulls*, and that is this sentence
applied. **The reader gives way and the invariant does not** - which is the shape the bullet makes
available to whoever hits the next case.
