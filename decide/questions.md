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

## The three places, and what each costs

```
C1  tools/spec/tests/architecture.rs
    yours by column - the code lane may not edit it
    already in their gate: hooks/pre-push runs every tools/*/Cargo.toml's tests
    they cannot repair a failure they believe is wrong; they report it

C2  crates/game4x/tests/architecture.rs
    beside the tests it is about, run by cargo test --workspace
    the lane the check constrains can weaken it

C3  spec/tests/, with a record in reviewed/
    yours, and you read it in the review application like any other test
    that directory holds .4x files run by the thin engine; this one is Rust
```

## What this lane recommends and the one measurement behind it

**`C1`.** It is the only place that is both outside the code lane's column and inside the gate
they must pass - `hooks/pre-push:79` loops over `tools/*/Cargo.toml` and runs `cargo test` on each,
so a check there stops their push without being theirs to edit.

**`C2` fails the thing you asked for.** A boundary the constrained lane may relax is the state
`P-545` exists to end.

**`C3` is right in spirit and costs a second file kind.** `spec/tests/` is fifty-four `.4x` files
today, and `crates/thin-engine/tests/reviewed.rs` runs them against `reviewed/`. A Rust test there
would need a separate runner, and the review application would have to show you something it was
not built to show.

## The asymmetry `C1` creates, stated so you choose it knowingly

**The code lane would be gated by a check it cannot fix.** That is already the rule in the other
direction - `CLAUDE.md` says this lane does not edit code *even to fix an obvious break*; it
reports the break and leaves it. **`C1` makes the arrangement symmetric**, and the cost is a
round trip whenever they believe a boundary is wrong.

**It is a feature for exactly as long as the boundaries are right.** If they turn out to be
wrong often, the round trips are the signal, not the friction.
