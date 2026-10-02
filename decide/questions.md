# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-612 - What may withhold the deploy

**to** sean · **status** open · **raised** 2026-10-02 · **kind** a choice the code lane surfaced rather than made · **shape** text · **asks** a decision · **into** `docs/process.md` -> What I am pushing out

**`caf2aa9c` moved the test suites out of `gate` on your rule.** `gate` is now three things, and
`deploy` still waits for all three.

| What `gate` runs                          | If it fails                        | Withholds the page? |
| ----------------------------------------- | ---------------------------------- | ------------------- |
| `cargo fmt --all -- --check`              | the code is correct and ill-spaced | **yes, today**      |
| `cargo clippy --workspace -- -D warnings` | the code is correct and inelegant  | **yes, today**      |
| the WASM build                            | **there is no artifact**           | yes                 |

**The code lane left this rather than deciding it**, in its words: *a lint failure withholding the
page costs exactly what a test failure did.*

## Three answers, and the middle one is this lane's reading of what you already wrote

```
one     nothing withholds the deploy; a red run reports and publishes anyway
two     only a failure that means there is no artifact withholds it - the WASM build
three   as today; fmt and clippy keep blocking
```

**Two, said as a sentence for the section:**

> **Nothing withholds the deploy except not having built.** A check that fails says so and the
> deploy proceeds, because deploying is how I verify and a deploy that waits for correctness has
> the order backwards. **The one exception is a build that produced no artifact**, where there is
> nothing to publish rather than a decision not to.

## Why this is a decision and not an approval

**Your section already implies two**: *I deploy it so that I can verify correctness on something
concrete*, and *what is out there is closer to a staging area, or a prototype, than to a product.*

**But it does not settle whether a lint failure is different from a test failure**, and there is a
real argument that it is: **a lint failure means nobody has looked**, where a test failure means
something was looked at and disagreed. **This lane will not resolve that quietly.**

## What the page costs when it is withheld

**It is the one surface that tells you what is waiting on you.** Six tests are unread as of
`77aac86e`, and until today a red gate meant you could not read any of them - **the thing that tells
you to look was withheld by the thing you were meant to look at.** That is already fixed for tests
and is still true for `fmt`.
