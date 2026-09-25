# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-559 - the engine reads the form whose references are ids, and why

**to** sean · **status** open · **raised** 2026-09-25 · **asks** approval · **kind** a rule held by nothing · **shape** text · **into** `spec/invariants.md` -> The data is a normalized relational model

**The rule the two-format design rests on is not in `spec/`.** *The engine should only know about
the foundational format* is stated in exactly two places, `crates/thin-engine/README.md:1362` and
`crates/thin-engine/tests/common/friendly.rs:28` - **both doc comments, both the code lane's, and
neither reachable by a reader of `spec/`.**

## The words

**One bullet, into *The data is a normalized relational model*, after *the notation is a text form
of that model*:**

> - **The engine reads the form whose references are ids.** An id is unique by construction and a
>   name is unique by constraint, so a form written with names carries a naming rule that something
>   has to enforce. **Identity is the engine's concern and uniqueness of names is not**, so a name
>   is resolved to an id before the engine sees a row.

## Why this reason and not the one on record

**The reason currently given has inverted.** *The user friendly format is for the test harness and
debugging* - true on 2026-09-15, and after `P-558` friendly is the authored canonical artifact and
foundation is the debugging view. **The sentence now argues the opposite of what it argued.**

**The surviving reason is measurable rather than stylistic.** `Names::of` is **333 of the
translator's 561 code lines** - building the name-to-id map and testing uniqueness, *all or nothing,
per relation*, and *an input's name is unique inside its rule and nowhere else*. **If the engine
read names, that is what would move into it.**

**And your engine already states the distinction.** `crates/thin-engine/src/schema.rs`: *this is
identity, not uniqueness, and the two parted company when a quantity arrived.* The engine works on
a relation's first column; names are a second thing that must be kept unique by somebody.

## What would make this rule wrong, stated so it can be checked

**If unique names turned out to be something the game's data needs for its own reasons**, rather
than something the authoring format needs to resolve references, the engine would need them anyway
and this reverses. **Measured: uniqueness appears only in the translator**, and only in service of
resolving a reference.

## What follows, and it is not part of what you are approving

**A cleanup to the code lane**: the two doc comments cite `spec/invariants.md` rather than being
where the rule lives. Also `spec/README.md` rule 3 states which form is the statement and this
states which form the engine reads - **the two halves of one design, and neither implied the
other.**


