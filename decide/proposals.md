# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-598 - A regression case takes the test format: the whole mutable world in, one command, the whole mutable world out

**to** sean · **status** open · **raised** 2026-09-30 · **asks** approval · **kind** recovered · **shape** text · **into** `docs/process.md` -> How I know the application is right · **source** him reading `regression/scenario/01/06-end-turn.4x`

**Your words and your answers, offered as a block after *I never modify a regression case*.**

> **A regression case is written the way a test is written**: the rows that go in, the single command,
> and the rows that come out. **It omits no row that can change** - so the flow from input to output is
> on the page and nothing is left for me to remember, which is what a projection onto the columns a
> rule happened to name cost me. **A case will therefore show rows the command does not read, and that
> is acceptable.**
>
> **What never changes is stated once and referred to.** A kind belongs to the scenario's setup rather
> than to each case exactly when no rule ever adds or removes one, so the test is mechanical rather
> than a judgement somebody maintains: if a rule ever starts creating a kind, that kind moves into the
> cases by itself. **The reference is what the runner follows, not only what I click** - and that is
> what lets a case be executed directly as a test, because it is how `place-1` exists.
>
> **A state has one entry per description**, so two things alike are one quantified row and never a row
> per firing. **What fired, and how many times, is the scenario's account rather than the case's.**

## What it replaces, and the measurement that sizes it

**Today a case is a projection onto the columns each firing read**, which is why `{end-turn}` showed
four citizens as ten entries and twelve, two of them with no `laboring` column because `breed` never
names it. **Nothing on the page said so.**

```
the whole final world            36 rows - the largest the scenario reaches
never added or removed           19 - capacity 9, place 4, territory 2, adjacency 2, provides 1, consumes 1
changes during play              17 - extractor 5, deposit 5, citizen 2, yard, metal, bin, ark, planet
```

**So the constant rows are more than half of a complete state**, and repeating them across 36 cases
would be 684 lines saying the same thing. **Referring to them instead leaves a case at seventeen rows
at its very largest**, and fewer earlier in the scenario.

## What this costs the code lane, stated because it is not small

**The generator changes shape rather than gaining an option.** It currently writes what each firing
took and made; it would write the world before and after one command. **And the four suites are not
all the same**: 36 of the 165 cases use `{given}`/`{when}`/`{then}` and 129 do not, so what this says
about `regression/scenario/` does not reach `types`, `primitives` or `rules` and the proposal does not
claim it does.

**It also makes your hypothesis true rather than nearly true.** Measured: 56 of 57 tests in `reviewed/`
carry `{territory}` and `{place}` rows and 36 of 36 cases carry none, which is the only reason a case
cannot be run as a test today.

