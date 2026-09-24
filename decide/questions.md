# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-546 - Where the architecture check lives, given that it may not be weakened by the lane it constrains

**to** sean · **status** open · **raised** 2026-09-23 · **kind** measured · **shape** an instruction · **asks** a decision · **into** the check's location, and `docs/architecture.md` -> Rules

**`P-545` makes the boundary yours. This asks where the thing that enforces it sits.** It carries
no quotation, because what you are choosing is a location and the words follow from it.

## The four places, and two of them are already in use

**This lane first offered three and called one of them theoretical.** The corrected sweep in
`S-159` found a working example of two, so the choice is between patterns in use rather than
between a safe option and a theory.

```
C1  tools/spec/tests/architecture.rs                            not in use
    yours by column - the code lane may not edit it
    in their gate twice: hooks/pre-push loops over tools/*/Cargo.toml, and CI
    has its own "Test (the documentation tools)" step
    they cannot repair a failure they believe is wrong; they report it

C2  crates/<the crate it constrains>/src/lib.rs, in #[cfg(test)]    IN USE
    no_floating_point_anywhere, in game-model and planet-model
    closest to what it constrains, and run by cargo test --workspace
    the lane the check constrains can weaken or delete it

C3  spec/tests/, with a record in reviewed/                      not in use
    yours, and you read it in the review application like any other test
    that directory is 54 .4x files run by the thin engine; this one is Rust,
    so it needs its own runner and the app must show what it was not built for

C4  tools/outbox/tests/architecture.rs                              IN USE
    an architecture check already lives there - every workspace crate is
    named in docs/architecture.md, iterating the workspace rather than a list
    tools/outbox is production support, so this is the code lane's column:
    C1's location with C2's ownership. This is the status quo.
```

## What this lane recommends

**`C1`, and the reason is narrower than before.** The question is not *can a check live there* -
`C4` proves it can - but *who may weaken it*. `C1` is the only one of the four that is both
outside the code lane's column and inside the gate they must pass.

**`C2` has the best precedent and fails the thing you asked for.**
`crates/game-model/src/lib.rs:57` is the pattern worth copying whichever location wins: it strips
`#[cfg(test)]` before scanning, because *this very test has to name what it forbids in order to
look for it*; it skips comments; and it asserts how many files it read, noting that `read_dir` is
not recursive so a module moved to a subdirectory would go unscanned and stay green.

**`C4` is where you are today**, which is worth saying plainly: the existing architecture check is
already the code lane's to change.

## One thing you could decide instead of the location

**Split it: the rule is yours and the check is theirs.** `P-545` puts the boundary in `spec/`, so
the prose says what is required no matter where the check sits, and a weakened check leaves a rule
it visibly fails to enforce. **The cost is that nothing detects the weakening** - the rule still
reads correctly and only its enforcement has gone - which is the shape `CLAUDE.md` calls *silence
and nobody has looked yet are the same bytes*.

## The asymmetry `C1` creates, stated so you choose it knowingly

**The code lane would be gated by a check it cannot fix.** That is already the rule in the other
direction - `CLAUDE.md` says this lane does not edit code *even to fix an obvious break*; it
reports the break and leaves it. **`C1` makes the arrangement symmetric**, and the cost is a
round trip whenever they believe a boundary is wrong.

**It is a feature for exactly as long as the boundaries are right.** If they turn out to be
wrong often, the round trips are the signal, not the friction.
