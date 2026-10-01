# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-595 - A prototype replicates what it needs and links none of it, and the current one links seven crates

**to** sean · **status** open · **raised** 2026-09-30 · **asks** approval · **kind** recovered · **shape** text · **into** `docs/architecture.md` -> Rules

**Your three sentences, and the measurement that says the arrangement today breaks all of them.**

**Offered as a new rule at the end of that list:**

> **A prototype must not influence the main code, even indirectly, and the way that is kept is that
> it links none of it.** A prototype that depends on main code puts pressure not to refactor main
> code in ways that break the prototype; main code that depends on prototype code has stopped being
> prototype code by definition. **Both directions are forbidden, and the second is forbidden by
> saying so rather than by anything mechanical.**
>
> **So a dependency a prototype needs is replicated rather than linked**, and what it replicates is
> a snapshot of that code as it was when the prototype was made. **It is almost always a smaller and
> modified version** - a prototype asks one question, and the code that answers it is rarely the code
> the game needs.
>
> **A prototype left linking main code is a defect in the arrangement rather than in the prototype.**
> Nothing a prototype does is wrong because the main code moved; what is wrong is that the main code
> could not move freely.

## What the dependency graph says today, measured with `cargo tree`

```
the game links        graph-coloring  planet-bevy  planet-model  planet-presentation
                     planet-render   planet-terrain  sphere-tessellation
the prototype links   all seven of those, and three more
prototype only        planet-ecs  planet-flat  planet-raster
```

**So `prototypes/planet-view` links seven crates the game also links**, which is the pressure your
first sentence names - and `C-187` felt it from the other side, declining to delete 399 lines because
*deleting a prototype's dependency decides something about research*.

**And three crates in `crates/` are linked by nothing the player runs.** `planet-ecs` is one of them,
which the quality lens reached independently as `Q-103`: it *says it is the one home of game state,
and the shipped binary does not link it*.

## What follows, and it is the code lane's

**The prototype takes a copy of what it needs and the three prototype-only crates go with it.** Then
no `crates/` file is held in place by research, and the 399 lines `C-187` found become prototype code
your rule exempts rather than main code it does not.

**Not costed here.** Whether the copy is small and modified, as your third sentence expects, is a fact
about the code and theirs to find. **`S-226` said a move was the resolution and this is the same move
seen whole** - seven links rather than one crate.
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

