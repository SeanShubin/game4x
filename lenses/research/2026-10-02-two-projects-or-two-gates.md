# Two projects, or two gates

**2026-10-02.** From Sean asking whether the tests and the game should be separate projects, and
then whether independent pipelines are possible in one repository at all. Carries `X-42` to the
specification lane and `X-43`, now acted, to the code lane - `X-44` came out of verifying that fix.

[Research](README.md) · [Outbox](outbox.md) · [`CLAUDE.md`](../../CLAUDE.md)

**The answer is two gates rather than two projects, and most of it was built the same day this
was asked.** What is left is one decision of Sean's, one paragraph of `CLAUDE.md` that has
outlived its condition, and one defect that publishes an empty site.

## Amended the same day: the case for the split is weaker than this report argues

**`514573e4` fixed `X-43`, and in doing so it removed the strongest argument in here.** This
report's recommendation rests on a coupling that was about **availability** - a lint failure taking
the review page off the internet. With `Copy the reports into the artifact` carrying
`if: always()`, a failing gate now publishes the reports in full. **So the coupling is about
latency, not availability**, and the three remaining costs are smaller than the ones argued below:

| What is left                                                       | Cost                                                                       |
| ------------------------------------------------------------------ | -------------------------------------------------------------------------- |
| The gate still lints the workspace and builds WASM before `deploy` | The page is published minutes late rather than not at all                  |
| `hooks/pre-push` still runs `fmt`, clippy and the full suite       | Pushing a record in `reviewed/` is gated on the Bevy workspace being green |
| One Pages site per repository                                      | Unchanged, and still the only thing two repositories uniquely buy          |

**The conclusion does not move and its margin does.** *Do not split the repository* was argued on
four single-tree properties, and those are untouched. **What has gone is the sharp edge** - the
thing that would have made a split urgent rather than merely available.

**Recorded here rather than rewritten below**, because a report is a record of a moment:
[the README](README.md) says a superseded one says so at the top. **The amendment is the second
time in one day that this report's premise moved under it**, both times because the lane that owns
the pipeline was fixing the thing being measured, and both times found by re-deriving rather than
by anything failing.

## What was measured, and how to re-run it

**The review application's whole dependency closure is three local crates and no third-party
package.** Built cold into an empty target directory:

```
CARGO_TARGET_DIR=<empty> cargo build \
  --manifest-path crates/game-model/Cargo.toml --example review-web
  -> 12 seconds, 3 packages compiled
```

Against the workspace it sits in:

```
grep -c '^\[\[package\]\]' Cargo.lock   544
grep -c '^name = "bevy'   Cargo.lock    65
```

`game-model` depends on `planet-model` and, to test and to run its examples, on
`friendly-notation`. **Every tool in `tools/` has an empty `[dependencies]`** - `outbox`, `spec`,
`anchor`, `pad-tables`. So the non-game half of this repository is dependency-free Rust, and the
Bevy half is sixteen crates and 65 packages.

**The seam is engine-against-presentation and it is already in the crate graph.** Measured by
reading each manifest's `[dependencies]`:

```
bevy-free   command-language  graph-coloring  sphere-tessellation  planet-model
            game-model  friendly-notation  planet-terrain  planet-render
            planet-presentation  planet-raster  game-console  game-front
bevy        game4x  game-globe  game-inspect  planet-bevy  planet-ecs  planet-flat
```

## The review application runs the engine, which is why it is not a separate product

`crates/game-model/examples/report.rs` calls `game_model::engine::Game::of` and
`game_model::engine::play`. **The page is a test runner with an approval UI**, not a document
browser: a card shows a test and what the engine did with it. So anything that reviews tests
needs the engine compiled in.

**That is the fact that decides the repository question.** A separate review project would hold
the engine as a cross-repository dependency - a git pin or a submodule - and a pin is a version
that can be wrong. Inside one tree the same relationship is a path dependency that cannot be
stale.

## Four things that are single-tree properties, and a split removes the tree

| What                      | Where it lives                                                                                                                                                                   | What a split costs it                                                                      |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| The index                 | `tools/outbox` walks one tree and reads ten outboxes; `pending.md` and `decide/attention.md` are generated from that walk                                                        | *Nothing open means nothing outstanding* loses its scope - there is no single walk         |
| The column map            | `hooks/pre-commit`'s `column_of`, one table of paths                                                                                                                             | Two tables, and the `spec`/`code`/`sean` boundary falls between repositories               |
| The drift invariant       | `spec/tests/` against `reviewed/` against `crates/game-model/data/foundation/tests/`, asserted equal by `every_reading_reaches_the_suite_and_everything_the_suite_runs_was_read` | Three populations in two repositories, and the check that holds them together has to clone |
| One statement of the data | `D-3`, open: *the game's data is stated once*                                                                                                                                    | A generated second copy becomes a fetched one across a boundary                            |

**The fourth is already not held, in the smaller form.** `spec/data/rules.4x` and
`crates/game-model/data/foundation/rules.4x` differ on 518 lines - the name form against the id
form, which is the generator working rather than drift. A repository boundary turns that into a
fetch.

## Independent pipelines in one repository: yes, and one already runs

Sean's intuition was that push-based triggering rules this out. **The event is push-based; which
workflows wake is a per-workflow decision.**

| Mechanism                            | What it gives                                                         |
| ------------------------------------ | --------------------------------------------------------------------- |
| `on: push: paths:` / `paths-ignore:` | A workflow that does not start unless the push touched matching files |
| Job-level `if:` with a paths filter  | The same selectivity, and a skipped job reports **success**           |
| Separate `concurrency:` groups       | Two pipelines that never cancel each other                            |
| Non-push triggers                    | `issues`, `workflow_dispatch`, `schedule`, `repository_dispatch`      |

**`.github/workflows/review.yml` is the existing proof.** It fires on `issues: edited`, installs a
bare toolchain, runs `tools/outbox --review-apply`, and touches no Bevy. Approving a test from a
phone already works with the game's build entirely out of the way.

**Two caveats.** A workflow skipped by a `paths:` filter reports nothing, so a required status
check stays pending and blocks a merge - which is why the filter belongs on the job rather than on
the workflow. And **GitHub Pages is one site per repository**; `actions/deploy-pages` replaces the
whole site, which `pipeline.yml`'s own comment already records as *a repo-wide singleton*.

**The mechanism that gets two independent publishers out of one repository** is setting the Pages
source to *deploy from a branch* rather than to Actions. Two workflows then commit into different
subdirectories of `gh-pages` and the served site is the union. The cost is a soft limit of ten
builds an hour and a source folder of `/` or `/docs` only. **Named because it is the feature the
question was reaching for**, not because it is recommended - see below.

## What the code lane built on 2026-10-02, which this report's first draft got wrong

**An earlier draft of this argument said the review page publishes only when the game's gate
passes. That was true when this session began and is false now.** `30422f9e` and the three commits
after it took the test suites out of the deploy path, on Sean's words recorded in the file:

> *I want to make sure a test I have not reviewed does not fail the build, and my ability to
> review it comes online as soon as possible. Once I have reviewed it, it can fail the next
> build.*

So `checks` and `sweep` have no `needs:` and nothing points at them, `Upload Pages artifact` is
`if: always()`, and `deploy` is `needs: gate` with `if: always()` - it publishes when the gate
fails. **The run still goes red; only the dependency went.** `P-613` is the promotion: *a failing
build leaves a broken game published rather than nothing published, because deploying is how I
verify.*

**The correction is the finding.** The shape this report would have recommended is three-quarters
built, by the lane that owns it, from Sean's own instruction - and what is left is smaller and
sharper than a repository split.

## What can still withhold the review page, and it empties the site rather than staling it

**`X-43`.** In the `gate` job, these steps carry no condition and run in this order:

```
Check formatting                     (no if)
Clippy (everything, tests included)  (no if)
Build (WASM, release)                (no if)
Write build provenance               (no if)
Copy the reports into the artifact   (no if)
Say what this run did                if: always()   <- mkdir -p dist/reports
Upload Pages artifact                if: always()
```

**A step that fails skips every later step that is not `if: always()`.** So a `cargo fmt --check`
failure in any of nineteen crates skips *Copy the reports into the artifact*, while
*Say what this run did* still runs its `mkdir -p crates/game4x/dist/reports` and
*Upload Pages artifact* still uploads. `deploy` then publishes a `dist/` containing
`reports/run.html` and `reports/run.md` and nothing else.

**The published review page is not stale, it is gone.** `reports/review/index.html` 404s, and
with it the 57 test cards and 165 case entries that `E-4` is vetted by. Pages is a singleton whose
artifact replaces the site, so there is no previous copy left underneath.

**The comment in the file states this one notch too weakly** - it says `fmt` and `clippy`
*still block the deploy*. They do not block the deploy; the deploy succeeds and publishes a site
with the reports missing. The distinction matters because a blocked deploy leaves the last good
site up and this does not.

**Nothing checks it.** No test in `crates/` or `tools/` reads `pipeline.yml`;
`crates/game-model/tests/browsable.rs` checks that linked paths are named in the workflow text,
which is a static property of the file and says nothing about which steps run on a failure.

**Reachable rather than theoretical.** `hooks/pre-push` runs `cargo fmt --all -- --check` and
`clippy --workspace --all-targets -- -D warnings`, so a local push is refused first - but
`CLAUDE.md` contemplates `--no-verify` as Sean's call, and records that `pre-push` ran zero tests
from 2026-09-22 until it was found.

**Measured: the step order and conditions above, read from `pipeline.yml` at `c683e211`. I think
the reason the ordering was never noticed is that `if: always()` was added to the two steps that
had to survive a failure, and the step that assembles the artifact was not one of them** - but
that is an inference about intent and no instrument here reads it.

## The suspension paragraph has outlived its condition

`CLAUDE.md` lines 130-147 suspend the rule that a generated regression case is deleted only by
Sean. Its own end condition:

> **It ends on a condition rather than a date, and the condition is the later of two things.** The
> format is nearly done; the app is not started. **So the suspension outlives what motivated it**,
> and whoever sees both land says so and this paragraph goes. **Nothing restores it
> automatically**, which is the risk the condition is written down to carry.

**Both halves are observably built.** `releases/marking-state.md`:

```
E-3  A case takes a verdict and a regeneration is authorized separately
     status built 2026-10-01 · cited db4717c2, c123b840      <- the format
E-4  I can do all of it from a page, from anywhere
     status built 2026-10-01 · cited e2ef89ac                <- the app
```

**Two things a proposal has to state rather than assume.**

**The phrase is *hosted* and the thing built says it is not.** `review.yml`'s own header reads
*Nothing is hosted. The issue is the list and the control at once, so there is no app to keep
alive and no OAuth to hold.* `E-4` is a page on Pages that writes through the API with Sean's
token. Neither is a hosted application in the sense the paragraph's wording suggests, and
*hosted review app* appears nowhere else in the tree - only twice in that one paragraph, once in
Sean's own quoted words.

**All four capabilities are `built` and none is `vetted`.** `E-1` through `E-4` are
`to sean · status built`. So *done* under the paragraph's condition is ambiguous between built and
vetted, and un-suspending on `built` would let a lane resume deleting cases before Sean has
confirmed the replacement works.

**What this lens does not do here.** Sean said in conversation on 2026-10-02 that the condition is
met and the paragraph should go. **That is not relayed as an approval and must not be acted on as
one.** `CLAUDE.md`: *a reader has no way to tell a relayed approval from an invented one, and the
file being relayed about is the one that says who may write what.* The suspension governs what a
lane may delete, which is squarely who-may-write. **So what travels here is the checkable half** -
the two capabilities, their statuses, the paragraph's own condition and the `hosted` mismatch -
and the authority half reaches the specification lane from Sean, in the queue, where promoting is
him reading it. Facts relay; authority does not.

**And the fourth paragraph is not purely about the suspension.** Lines 145-147 say *`reviewed/` is
untouched by this*, ending *a lane still creates, deletes or changes no record* - which restates a
rule already stated at lines 109-111. Whether that paragraph goes with the other three, or loses
only its first clause, is a wording question inside the rules and therefore the specification
lane's to settle and report.

## Recommendation

**Do not split the repository. Finish splitting the gate.**

**The only thing two repositories uniquely buy is an independently publishable Pages site**, and
that is obtainable inside one repository, either by branch-based Pages publishing or - far more
cheaply - by making the artifact assemble whether or not the lint passed. Everything else a split
was wanted for is already there: `review.yml` is a game-independent review channel, `checks` and
`sweep` are out of the deploy path, and a failed run publishes.

**The costs of a split land on the invariants rather than on convenience.** One index, one column
map, one drift comparison, one statement of the data. All four are written as properties of a
single tree, and a split does not weaken them a little - it removes the tree they are quantified
over.

**And the boundary being reached for is not review-against-game.** It is engine-against-
presentation, it is already in the crate graph, and no pipeline respects it yet. Making the
pipeline respect it yields the twelve-second gate and the always-available review page with the
index, the columns and the drift check all still ranging over one tree.

**So the shape is two gates and a publish, in one repository** - three jobs where there is one: an
engine gate over `spec/`, `reviewed/`, `regression/`, `game-model` and `tools/`, which is seconds;
a presentation gate over everything Bevy; and a publish that assembles the site from the committed
reports regardless of either. The review page then goes stale in its game half when the renderer
breaks, and never in its review half.

**This sentence said *three gates* until 2026-10-02 and that was wrong on this report's own
terms.** A gate is a thing that can refuse, and **the publish cannot** - `P-613`, recorded in the
amendment at the top of this file: *a failing build leaves a broken game published rather than
nothing published.* **So there are three jobs and two of them are gates**, which is what the title
says and what this sentence contradicted.

**Found by the specification lane**, reading the title against the recommendation after Sean asked
whether it had the material. **The count came from counting jobs and calling them gates** - a word
used where a property was meant, which is the class this report's closing section is about, in the
document that records it. **It had travelled**: this lens said *three-gates shape* to the code lane
and *two-gate shape* to the specification lane, from one file that said both.

**One decision is Sean's and the code lane has already named it** in `pipeline.yml`: *a lint
failure withholding the page has the same cost as a test failure doing it, so this is the obvious
next thing to move - and it is his to say, not this lane's to assume.* `X-43` is the sharper
version of that question, because the present behaviour is not withholding but replacing.

## How the cycle closed, and the one thing in it that is against this lens

**Seven findings, all seven closed, none found by a check firing.** `X-42` to spec became `P-614`;
`X-43` through `X-47` to code were each fixed, four of them within the hour of being filed, and each
fix was verified here by driving it rather than by reading it.

**Three of the seven were compositions and four were sentences.** A sentence had gone stale, or
spoke about something adjacent to its subject. **A composition was two correct artifacts with
nothing between them** - `review.yml` as a writer against `reviewed.rs` as a reader; `E-2`'s third
verdict against an assertion written when there were two; `review.yml`'s push against
`generate.yml`'s trigger. **No check can catch one, because a check is one of the two things being
composed** - `P-245`'s wall from a new side, and the only answer is somebody putting two correct
things together on purpose.

**And an absence can be the thing that works.** `actions/checkout` with no `token:` was `X-47`;
`actions/checkout` with no `ref:` is what makes its fix correct, because `github.ref` differs by
trigger and is right both times. **The defence is not to choose the default** - `push` and
`workflow_run` want different refs and the default serves both - **it is to make the absence say
why it is there**, which `f6eeb394` does.

**The instance against this lens is the praise.** This report's lane called the code lane's
partition guard - `approved.len() + denied.len() == records.len()` - *the better half*, and said it
catches the failure mode that would have replaced the one being fixed. **The property is true and
the reason was invented here**: its author wrote it because splitting one set into two invites that
slip, not because they had identified that failure mode. **The reading was correct and was not
theirs.**

**That is `CLAUDE.md`'s *a measurement travels with an explanation of itself, which is not
measured*, pointed at somebody else's code** - and it is the direction nobody audits, because an
over-generous account of another lane's reasoning is the one no reader checks. **Found only because
its author said so.** Eighth of the class, first in the praise direction, and the only one that
needed the subject to volunteer the correction.

**The code lane's statement of the rule is sharper than this lens's and is the one to keep**, given
on 2026-10-02 after volunteering the correction: **attributing reasoning to another lane is an
inference about a mind, which is the least checkable thing there is.** And the carrier it suggests
is a form of words rather than a reminder: **say *this holds, and I do not know whether it was
meant*, not *they wrote it because*.**

**Checked in the other direction, because a self-review in the flattering direction is exactly what
this finding says nobody audits.** Over six messages the code lane's claims about this lens were
about artifacts it had run or read - which probe tests the floor, what the baseline ordering buys,
that the checkout default is load-bearing - and its judgements were labelled as judgements. **One
candidate, and it is weaker than the instance above**: *I verified it the way you would want*, which
is a predicted preference rather than an invented reason. **It was right, and it was refutable by
this lens in one sentence** - which is the distinction the rule draws, since what made the praise
instance expensive was that only its subject could check it. **Not filed, and recorded so the check
is not mistaken for a courtesy.**
