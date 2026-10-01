# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-594 - Upgrade early, and no pin, because a test that names no technology cannot be broken by one

**to** sean · **status** open · **raised** 2026-09-30 · **answered** 2026-09-30, option three and more aggressively · **asks** approval · **kind** recovered · **shape** text · **into** `docs/process.md` -> How I know the application is right

**Your answer to `C-189` and the reason you gave for it**, which is the half worth writing down
because it is why the policy is cheap rather than brave.

**Offered as a block at the end of that section:**

> **I take a new toolchain early rather than late, and there is no pin.** Pinning would make the
> pipeline predictable by making it old, and it would stay old, because bumping a pin needs a
> toolchain to test the bump against. **Keeping this machine current makes the gate predictive
> without a pin**, because *stable* and a current machine are the same thing.
>
> **What makes that cheap is that the tests do not know what the game is built on.** A reviewed test
> names a territory, a citizen and a deposit; it names no language, no crate and no version. **So an
> upgrade cannot change what the game is specified to do** - it can only break the code, which the
> gate catches and has caught.
>
> **The red that arrives after a green gate means my machine is behind**, not that the pipeline is
> broken. It is the only notice I get that a new toolchain exists, and taking it promptly is what
> keeps it rare.

## The reason was measured rather than accepted

**Over all 57 files in `reviewed/`, with word boundaries: zero rows and zero comment lines** name
`rust`, `cargo`, `wasm`, `bevy`, `crate`, `binary` or `impl`. **A first pass said eight** and all
eight were the word *structure* matching `struct` as a substring - which is worth recording because
it happened inside the check of your own sentence.

## What follows, and it is not in the offered text

**Nothing in the repository changes today.** `pipeline.yml` already installs `@stable` in two places
and that is what the policy wants; the pin that `C-189` offered is declined rather than built.

**Two things are left and neither is this lane's.** `rustup` on this machine, which is yours. And a
notice that stable has moved, which is the pipeline's and so the code lane's - today the notice was a
lint firing, which arrives late and only by luck. **`C-189` closes as decided rather than as a
defect**, which is why the sentence above exists: an unexplained red becomes a signal nobody reads.

