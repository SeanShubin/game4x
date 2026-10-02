# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-613 - A failed run publishes, and the report root says what failed

**to** sean · **status** open · **raised** 2026-10-02 · **kind** his decision, with the words offered for reading · **shape** text · **asks** approval · **into** `docs/process.md` -> What I am pushing out

**Sean, 2026-10-02**: *lets go ahead and publish even if broken, I just need to be able to tell whats
broken by navigating from the report root.*

**This is what makes `P-612` buildable.** GitHub Pages serves one deployment per site -
`concurrency: group: pages`, `cancel-in-progress: true` - so publishing the review page early and
the game later is not available: the second replaces the first. **Your answer resolves the half
`P-612` deliberately left out.**

Two paragraphs, both into *What I am pushing out*, after *Nothing withholds the review page*:

> **The whole site publishes whether or not the run succeeded.** A failing build leaves a broken
> game published rather than nothing published, because deploying is how I verify and a staging
> area that vanishes when it breaks is no use to me.

> **And what broke is reachable from the report root.** I find out by navigating from the reports
> index rather than by reading a run's log, so a run that fails leaves a page saying what failed,
> linked from there.

## What this lane did not settle, so you are not approving it

**How much a page says.** *What failed* could be the step's name, or its output, or both; the second
paragraph asks for the first and does not forbid the second. **The code lane chooses**, and if its
choice turns out to be too little you will find out by using it.

**And nothing here says a failed run is not a failure.** The run still reds, `hooks/pre-push` still
gates your push, and `S-251` already moved the tests so a red test does not withhold the page. **This
is only about what gets published, not about what passes.**

