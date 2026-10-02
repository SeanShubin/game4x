# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-612 - Nothing withholds the review page

**to** sean · **status** open · **raised** 2026-10-02 · **kind** his instruction, with the words offered for reading · **shape** text · **asks** approval · **into** `docs/process.md` -> What I am pushing out

**Sean, 2026-10-02**: *can we structure the build so that the review capability is published as soon
as possible, regardless of whether the build succeeds or fails as a whole?*

**Yes, and it is cheaper than it sounds.** The review page is **440 committed files under
`reports/`** - `reports/review/index.html` and its data are in the repository, and the pipeline
copies them. **So publishing it needs a checkout and nothing else**: no toolchain, no lint, no
tests, no WASM build. Measured by `git ls-files reports`.

The sentence offered, for *What I am pushing out*:

> **Nothing withholds the review page.** It is committed rather than built, so publishing it needs
> a checkout and nothing else - no lint, no test, no build. **A run that fails says so and the page
> publishes anyway**, because the page is where I find out what is waiting on me, and a failing
> build is exactly when I most need to be able to look.

## What it replaces, and why this one asks approval

**`P-612` asked a decision until now** - whether `fmt` and `clippy` might withhold the page, the
choice the code lane surfaced rather than made. **Your instruction answers it and goes further**, so
this is the rewrite that answer turns it into: the question is closed and the words are offered.

## What it implies for the pipeline, which is the code lane's to build

**Today the page is copied into the artifact inside `gate`, after the WASM build**, and `deploy`
needs `gate` - so lint, the build, and until `caf2aa9c` the whole test suite each withheld it.
**Nothing about the page requires any of that.**

**One thing a reader should not take from this.** It says nothing about whether the *game* publishes
from a failed build - that is a separate question with a real answer either way, and **this lane is
not folding it in.**

