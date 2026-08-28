# What the response to the first report left behind

**Derived.** Written by the quality instance on 2026-08-28. Not binding - an observation about the
code, not a decision about it.

[Quality](README.md) · [The report this follows](2026-08-28-crate-boundaries-and-duplication.md)

Read at commit `8c395d8`. Three commits answered
[the first report](2026-08-28-crate-boundaries-and-duplication.md):

| Commit    | Lane          | What                                                          |
| --------- | ------------- | ------------------------------------------------------------- |
| `464ff45` | code          | Findings 1, 3, 4 and 13                                       |
| `a1cc5e0` | specification | Q1, Q2, and the two bindings the release was missing          |
| `8c395d8` | code          | Follows P-95 through the quotations of it that had gone stale |

**Every closure was verified, and every one holds.** Findings 1, 2, 3, 4 and 13 are closed and are
marked closed in the first report, so they are not repeated here and should not be reported again.
Findings 5 through 12 and 14 are untouched, which matches how they were triaged. All 38 tests in
`game-front` and `game-console` pass, including the two new ones.

Three things are worth writing down. Two are new; one is a cost the response added to a finding
that was already open.

---

## R-1. The P-95 cleanup claimed completeness and missed three quotations

**Where.** `crates/game-console/README.md:53`, `crates/game-front/src/shell/terminal.rs:49`,
`crates/game-front/src/shell/web.rs:41`.

**What.** `a1cc5e0` reworded `spec/console.md` so that a slash *directs the front end* rather than
*names a surface*. `8c395d8` set out to follow that through the code, and its message says it found
the quotations *"in eight places"* and that *"Two older quotations of the pre-P-94 wording went with
them, in game-console and in index.html"*. Three survive:

| Where                                        | What it still says                                                                                                                                                                                                                            |
| -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/game-console/README.md:53`           | An attributed block quote: **`spec/console.md`: *a line beginning with `/` is not a command. It names a surface to go to.*** That sentence is not in `spec/console.md` and has not been since `835cb33`.                                      |
| `crates/game-front/src/shell/terminal.rs:49` | *"`help` says a slash names a surface"*. `help` says no such thing - `game-console/src/report.rs:145` was changed to *"a line beginning with `/` directs the front end rather than the game"*, in this same repository, by the commit before. |
| `crates/game-front/src/shell/web.rs:41`      | *"`spec/console.md` says a line beginning with `/` names a surface to go to"*. Same removed sentence, attributed the same way.                                                                                                                |

The first and third are attributed quotations of a sentence that does not exist. The second is
checkable against a constant in the same workspace and is false.

**Why.** This is finding [3](2026-08-28-crate-boundaries-and-duplication.md#3) again, in three more
places, and it matters more the second time. A comment that misquotes the specification is worse
than one that says nothing, because it is the thing a reader checks *instead of* opening `spec/`.
`terminal.rs:49` is the sharpest: it tells a maintainer what `help` prints, and `help` prints
something else.

The scale of the problem is what is really being reported. One sentence in `spec/console.md`
changed four times in four days - P-91 through P-95 - and each change left quotations of the
previous wording scattered through `crates/`. `8c395d8` found eight of eleven, which is a good
hit rate for grepping by hand and not a good one for a guarantee.

**Whether.** **Fix now** for the three lines - it is a five-minute edit. But the durable fix is the
one this repository already knows how to build: `game-console/src/report.rs:145` proves it, because
`SURFACES_ARE_ELSEWHERE` is a constant *specifically so that a test can name the one line allowed to
mention a slash*. The same idea, applied to the specification rather than to `help`, is a test that
reads the sentence out of `spec/console.md` and asserts the code's quotations of it match - the same
shape as `command_language::agreement::disagreements`, and the same shape as
`library.rs`'s `what_is_carried_is_what_is_on_disk`, which already reads files off disk at test time
to prove two copies agree.

Whether a spec sentence should be load-bearing on a test is a design question and therefore not
this report's. What is not a design question is that hand-grepping has now missed three
quotations out of eleven, twice in a week.

---

## R-2. Fixing finding 1 added a fourth call from the adapter into the front end

**Where.** `crates/planet-bevy/src/globe.rs:695`, joining `:726`, `:764` and `:772`.

**What.** [Finding 6](2026-08-28-crate-boundaries-and-duplication.md#6) reported that `planet-bevy`
reaches into `game-front` in both directions, from three call sites. The reset control is a fourth:
`reset_view` now polls `game_front::shell::resets()`.

Two of the four - `reset_view` and `follow_the_game` - run unconditionally every frame. On the
desktop each takes the process-wide `Mutex` in `game-front/src/shell.rs`, so a frame now acquires
that lock twice, contending with the stdin thread that the terminal shell runs on.

**Why.** To be clear about what this is not: **the fix is right.** The report asked for a reset
control, `spec/interface.md` requires one, the counter is the same shape as `generation` and for the
same stated reason, and the alternative - cutting the crate edge first - was correctly triaged as
*fix eventually*. Nothing here says it should have been done differently.

What it does say is that the cost of leaving finding 6 open is now visible and rising. The
counter-watching pattern was introduced once, for a good reason. It is now a general-purpose channel
between the engine and the front end - two counters read per frame, one submit path - and every
capability added to the globe will add another, because there is no other way in. Each one makes
`GlobePlugin` harder to construct without a `Console`, and makes
`crates/game4x/src/main.rs:23` - *"the only place that knows both that Bevy exists and that the game
exists at the same time"* - more wrong.

**Whether.** **No action on its own.** Recorded so that finding 6 is re-read as growing rather than
static, and so that whoever eventually cuts that edge knows there are four call sites and two
per-frame locks, not three and one.

---

## R-3. `Console` now holds view state

**Where.** `crates/game-front/src/console.rs:1` (the module doc), `:31` in the struct (`resets`).

**What.** The module opens *"The one console: a session, what has been said, and what is being
typed."* `Console` holds five things, and two of them are neither: `reached` and now `resets`.
`resets` is a camera instruction - it puts the globe's orbit back to its default and touches nothing
else.

**Why.** The comment on the field defends it, and the defence is reasonable: *"Front-end state, like
`Console::reached` beside it."* `reached` was already there and is genuinely the same kind of thing.
So this is drift rather than a defect - but the summary line has stopped describing the type, and
the summary line is what a reader reads first. If a third front-end fact arrives, `Console` will be
two objects sharing a name.

**Whether.** **Noted and deliberately not.** One field is not a structure problem. Recorded because
the *next* one would be, and because the cheapest moment to notice is before it happens.

---

## How the lanes did

Not a finding. Recorded because a report is only worth writing if the response to it is worth
measuring.

- **Both lanes stayed in their columns.** `464ff45` and `8c395d8` touched only `crates/` and
  `hooks/`; `a1cc5e0` touched only `spec/`, `releases/` and `docs/notes/`. Nothing crossed.
- **Every commit message names the finding it answers**, restates it, and says what was done. That
  is what makes this report short: I could check claims instead of reconstructing intent.
- **The specification lane fixed the cause rather than the symptom.** Q1 asked which of two readings
  of *"where there is a pointer they are controls"* was meant. The answer was to delete the
  sentence, on the ground that it prescribed a mechanism where the requirement was already complete
  - and to say so, noting it was the third correction to that paragraph in three days and each had
  named a smaller mechanism instead of naming none. That is a better answer than either reading.
- **Q2 was answered exactly as narrowly as it should have been.** *"changes no game state"* became
  *"none is a transition"*, which is the claim that actually holds and the one that connects to
  `spec/invariants.md`. The new sentence *a game's history begins when the game does* closes the
  loose end I raised and did not know how to close.
- **Finding 2 was closed with no code change**, by naming the digit keys in the release. That was
  the outcome the finding expected, and it is the right one: the keys are good, they were simply
  undocumented.
- **Nothing was over-fixed.** Findings 5 through 12 and all six items in 14 were left alone. A lane
  that acts on every finding in a report is not triaging, and this one triaged.

The one thing to carry forward is R-1: `8c395d8`'s message reads as a completed sweep, and it was a
partial one. A commit that says *"quote the rule that is there now"* and leaves three quotations of
the rule that was there before is the failure mode
[CLAUDE.md](../CLAUDE.md#three-instances) names - *it reads like diligence and behaves like
forgetting.*
