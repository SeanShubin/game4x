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

**Six paragraphs, each added to the end of that section, in this order.** `X-48` is the lens's
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

> **One site is enough.** The app at the root and what I read below it, as long as I can navigate
> from the root to either - so nothing here asks for a second publisher.

> **Adding a test is the same loop as approving one.** I write a test, push it, and it shows up
> browsable on my page - without the build failing, and without waiting for the game to compile. A
> test I have not read yet fails nothing; it is a notice that something is waiting on me.

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

## Three of the sixth paragraph's four halves are already true

**Measured here, so you know what it costs:**

```
the page shows an unread test          yes - "never reviewed", 6 of them before today
an unread test fails nothing           yes - reviewed.rs prints "constrain nothing"
generate.yml fires on spec/tests/**    yes - so adding a test already triggers it
pipeline.yml has a paths: filter       no  - so the game's gate runs anyway
```

**So the only missing half is the one the first paragraph already asks for.** Adding a test and
approving one want the same single change, which is why they belong in one proposal.

## Half of the first paragraph is already built, which you may not know

**`generate.yml` already carries the filter this asks for** - `on: push: paths: reviewed/**,
spec/tests/**` - and it runs `cargo run --example foundation`, commits, pushes, and verifies with
`every_reading_reaches_the_suite`. **That is the step this lane has done by hand four times today.**

**It has never run**, because it reached `origin` in the same push that carried your last approval.
**So your next approval fires it.** What is missing is the other half: `pipeline.yml` has no filter,
so it runs too.

## Why the third paragraph is the one that matters

**Everything you review is produced by three crates** - `game-model`, `friendly-notation`,
`planet-model` - and the code lane re-derived the closure rather than taking it: **2, 3 and 1 lines
of `cargo tree`, `bevy=0` in each**, against 544 packages and 65 Bevy in the lockfile.

**`tools/outbox` already checks who may *read* a report** - `only_a_generator_or_a_check_reads_a_report` -
**and nothing checks that the producers of your review surface stay light.** So the property is
true, load-bearing, and held by nothing, which is the state this repository treats as worst.

## What this proposal does not say

**Nothing about the engine-against-presentation split.** You placed that as a code module question
rather than a pipeline one, so it is the quality lens's and the code lane's, and it is not here.
