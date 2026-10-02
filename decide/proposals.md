# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-615 - Approving a test does not deploy the app

**to** sean · **status** open · **raised** 2026-10-02 · **kind** his requirement, with the research lens's measurements behind it · **shape** text · **asks** approval · **into** `docs/process.md` -> What I am pushing out

**Four paragraphs, each added to the end of that section, in this order.** `X-48` is the lens's
item; the words below are this lane's draft of your requirement.

> **Approving a test does not deploy the app.** A push that touches only what I review runs what
> produces what I read and nothing else - not the app's gate, not its build, not its deployment.
> My loop is approve and look, and nothing in it waits on the game compiling.

> **A test about the code stays with the code.** This is about which pushes run which jobs rather
> than about where a test lives. There may be other tests in the code, and those are fine to be
> with the code.

> **What produces something I read stays in the light dependency closure.** That is what makes the
> rule above cheap rather than merely desirable, and it is the half nothing checks: a convenient
> dependency acquired by whatever writes my reports would put the game's build back in front of my
> approval without anyone deciding to.

> **Deploying the app is a different deployment, and so is any slow analysis.** Deploying the game,
> and any analysis of the game or of the tests that takes a long time, belong somewhere that is not
> between me and a page I am reading.

## It is violated now, and your own approvals are the evidence

**`pipeline.yml` has no `paths:` filter.** So a commit touching nothing but `reviewed/` runs clippy
over nineteen crates, builds a release WASM bundle and deploys the game.

```
your surface, cold      54s    zero Bevy crates
the gate in front of it  9m 35s · 14m 07s · 70m 15s cold
the deploy itself        9s · 11s · 28s
```

**544 packages in the lockfile, 65 of them Bevy, and none reachable from anything you read.**

**Seven of the last eight runs are titled `approved: <test name>`.** Six cancelled each other -
`cancel-in-progress` keyed on the ref - and **the one that completed failed because a Wayland system
library would not build.** You approved a test about a territory's biome and the run went red over a
graphics dependency.

## Why the third paragraph is the one that matters

**Everything you review is produced by three crates** - `game-model`, `friendly-notation`,
`planet-model` - measured by walking every crate for writers of `reports/`, `reviewed/`,
`spec/tests/` and `regression/`. **The partition is clean today and nothing holds it.**

**`tools/outbox` already checks who may *read* a report** - `only_a_generator_or_a_check_reads_a_report` -
**and nothing checks that the producers of your review surface stay light.** So the property is
true, load-bearing, and held by nothing, which is the state this repository treats as worst.

## One cost you should know before approving the fourth

**GitHub Pages is one site per repository and `actions/deploy-pages` replaces the whole site.**
Two genuinely independent publishers out of one repository means branch-based publishing, which
carries a ten-builds-an-hour soft limit. **If *a different deployment* has to mean a second Pages
publisher, that limit is the price**; if it can mean a different job on the same publisher, there is
no price. **This lane does not know which you mean and has not assumed.**

## What this proposal does not say

**Nothing about the engine-against-presentation split.** You placed that as a code module question
rather than a pipeline one, so it is the quality lens's and the code lane's, and it is not here.

