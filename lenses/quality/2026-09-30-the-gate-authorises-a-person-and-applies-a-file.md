# The gate authorises a person and applies a file

**Derived.** 2026-09-30, at `8fbb892f`. The review the code lane asked for, on the three things it
named as least confident: the `github.actor` gate, `game_state.rs`'s `says`, and its own
assertions. **Two of the three hold. The one that does not is the gate.**

[Quality](README.md) · [The review workflow](../../.github/workflows/review.yml) · [S-227's interfaces](../../crates/game-front/src/game_state.rs)

**It asked to be attacked and it was.** Four attacks on the gate, three of which it survives by
construction rather than by a check - and the fourth is a window between the thing authorised and
the thing done.

---

<a id="1"></a>

## 1. The gate authorises whoever edited the issue; the job applies the body as it is minutes later

**Where.** `.github/workflows/review.yml:83` (the `if`) and `:93` (the read).

**What.** The `apply` job is gated on `github.actor == github.repository_owner`, and `github.actor`
on an `issues` event is the person who made that edit. **The body it then acts on is not that
person's edit.** Line 93 re-fetches the live body:

```yaml
run: gh issue view "${{ github.event.issue.number }}" --json body --jq .body > body.md
```

So the sequence is: Sean ticks a row; the event fires with his name; the gate passes; a runner is
queued, provisioned and installs a Rust toolchain; **and then** the body is read. Anything that
reaches the issue body inside that window is applied under his authorisation and committed with
`git config user.name "${{ github.actor }}"` - that is, as him.

**The file's own comment says it is doing the opposite**: *the body as he left it*. It is the body
as it stands, which is a different sentence and the one the code implements.

**The blast radius is both gestures.** `carry_out` either copies `spec/tests/<name>` into
`reviewed/` or deletes `reviewed/<name>`. The delete is the sharper one: `CLAUDE.md` says *the
suite runs the copies in it*, so removing a record stops a test constraining anything, and the
commit that did it carries his name.

**Why the mitigation created it.** The step above it reasons correctly that the body is *arbitrary
text he can type into, and `${{ }}` in a `run:` is a substitution before the shell sees it* - so it
refuses to interpolate. **Refusing to interpolate the payload and refusing to read the payload are
not the same decision**, and the second was taken as though it followed from the first. The payload
field `github.event.issue.body` is the authorised bytes, and it can be handed to a file through
`env:` without ever passing through `${{ }}` in a `run:`.

**Why.** This is the first mechanical carrier for a rule `CLAUDE.md` states absolutely - *a record
is created and deleted only by the review application, acting as Sean* - and the carrier authorises
an actor while applying a document. **Those are the same thing only if nothing can edit the
document in between.**

**Whether.** **Worth fixing now, and it is a small change**: read the body from the event payload
rather than from the API. This lens is not asserting who can edit a task list on this repository -
the file says *anyone may tick a box on a public repository* and that was not verified here. **The
finding does not rest on it**: if only collaborators can, the hole is that a collaborator or a lane
with a token can write `reviewed/` as him, which is exactly what the gate exists to stop.

---

<a id="2"></a>

## 2. The rule that holds `S-227` is defeated by three import forms

**Where.** `tools/outbox/tests/architecture.rs:617`.

**What.** The predicate is

```rust
said.contains("game_front::shell::") && !ALLOWED.iter().any(|one| said.contains(one))
```

Driven against the forms a caller could use - the predicate is pure, so it was evaluated directly
rather than by editing the tree:

```
flagged  line
   True  use game_front::shell::generation;
   True  game_front::shell::generation()
  False  use game_front::shell;
  False  use game_front::shell as console;
  False  use game_front::{shell, library};
  False      shell::generation()            (after any of the three above)
```

**Three import forms pass, and every call site after them passes too**, because `shell::generation()`
does not contain the crate name. The check asks *does a line spell this exact path* where the rule
is *does a crate outside `game-front` reach the one console*.

**There is no live violation**, measured: no file under `crates/` or `prototypes/` outside
`game-front` contains `use game_front::shell` in any form. **So the check is correct about the
present state and blind to the drift it exists to catch** - which is the shape `CLAUDE.md` names as
*a check that pins the present state cannot report a gap against what should be*, one step over.

**And `ALLOWED` is a substring test over the whole line**, so a line naming
`game_front::shell::terminal::serve` anywhere in it - including in a trailing comment - exempts
every other reach on that line.

**Whether.** **Worth fixing now.** ~~Dropping the two trailing colons closes all three forms~~ -
**it closes two**, and the code lane caught it.

**Corrected 2026-09-30, and the shape matters more than the correction.**
`use game_front::{shell, library};` contains neither `game_front::shell::` nor `game_front::shell`,
because the crate and the module are not adjacent. **This lens drove the predicate and then asserted
the effect of the fix without driving it** - a before-and-after written while editing, which is a
prediction. The better instrument was pointed at the half already understood.

**Fixed at `5d86f8d7`, one form escaped, and closed at `5a7f3021` by matching no shape at all.**
`names_the_shell` strips whitespace, removes

`ALLOWED` before judging the rest, and splits a braced group, so all five listed forms flag. But
`split_once('}')` takes the first closing brace rather than the matching one:

```
  now  line
 True  use game_front::{shell::{generation, resets}};
False  use game_front::{library::{browse, page}, shell};
```

A nested group *before* `shell` truncates the outer group before `shell` is examined. Realistic:
`game-globe` already writes `use game_front::game_state::Watches;`, and `rustfmt` makes nested
groups.

**The third round on one predicate is the finding, and a fourth patch is the wrong answer.** Every
import form must contain both `game_front::` and `shell`, whatever the grouping - so requiring both
tokens after `ALLOWED` is removed needs no brace matching, and the closed set it must admit is four
names, measured: `game_state::Watches`, `game_state::Drives`, `game_state::TheOneConsole`,
`shell::terminal::serve`. Offered to the code lane rather than filed.


---

<a id="3"></a>

## 3. `says` is the right seam; its fallback writes `Debug` into the evidence

**Where.** `crates/game-front/src/game_state.rs:112-123`, and its one caller
`crates/game-inspect/src/lib.rs:173`.

**The seam is in the right place, and that half of the doubt can be put down.** `game-inspect`
reached `console.session` across a crate boundary; now the reach is inside the crate that owns the
`Console`, behind a named method, and `game-inspect` names a trait. **Being the one method that is
not a thin wrapper is what a seam looks like** - a surface whose every method is a rename has not
moved anything.

**What is wrong is the fallback.** `Outcome` has three variants - `Changed`, `Said(String)`,
`Nothing` - and `says` returns the string for one and `format!("{other:?}")` for the other two. Its
only caller puts that return value into a line of the dump, under the heading *-- the game, as the
console reports it --*. So if `{show-planet}` ever stops answering, **the dump reads `Changed` or
`Nothing` where a reader expects the planet**, and it reads as content rather than as an error.

**Why.** The dump is evidence a person vets - `D-5` is *I have watched the new game play through*.
A `Debug` rendering is not a stable interface: adding a variant or renaming a field changes that
line with no compiler error and no test, and the two strings it can currently produce are both
plausible English. **It is the repository's own *plausible wrong answer* class landing in the
artifact that is read rather than in one that is run.**

**Whether.** **Worth fixing eventually.** The honest form is for `says` to say it could not answer
- a `Result`, or a line that cannot be mistaken for a planet. Not now: `{show-planet}` is a
question and returns `Said`, so the branch does not currently fire, which is also why no test
covers it.

---

## Three attacks the gate survives, and it survives them by construction

**Recorded because a reviewer saying *I checked* should say what did not break.**

**A crafted test name cannot escape `reviewed/`.** `gestures` at `tools/outbox/src/lib.rs:1496`
loops `for name in &tests`, where `tests` is the real directory listing, and the ticked set is only
a membership test. So `name` can never be `../something`, and `carry_out`'s two `join`s cannot be
walked. **The loop direction is the defence** - not a validation step that could be forgotten, which
is the better kind.

**The orphan state has a door, and this lens built a deadlock before finding it.** The argument was:
an orphaned record makes `every_record_of_a_reading_names_a_test_that_is_there` red; the workflow
renders orphans as plain bullets rather than task items so they cannot be ticked; `gestures` cannot
produce a `Withdraw` for a name not in `spec/tests/`; and `CLAUDE.md` forbids any lane from deleting
a record. **Every step of that is true and the conclusion is false.**
`crates/game-model/examples/review-web.rs:238-247` carries the exception, with the reason written
out before this lens got there: *a record whose test is gone is exactly a name that is not a test,
so refusing it here would make the one thing that can remove an orphan the one thing that cannot.*
`/unreview` asks the disk rather than the list of tests. **Checked at the file, and there are zero
orphans now** - 57 tests, 57 records.

**The bot's own edit does not recurse.** The last step edits the issue with `github.token`, and a
`GITHUB_TOKEN`-driven event does not start a new workflow run.

---

## What the three sampled assertions are worth

The invitation was to sample the assertions rather than the code. Three, and the verdicts differ.

| Assertion                                                         | Verdict                                                                                        |
| ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `only_the_composition_root_reaches_the_one_console`               | **Weak** - finding [2](#2). Both populations asserted; the predicate is narrower than the rule |
| `every_local_dependency_a_row_names_is_one_the_manifest_declares` | **Sound.** Rows and edges both asserted, cell extraction pad-proof by construction             |
| `neither_surface_has_grown`                                       | **Sound today, and two notes below**                                                           |

**The dependency check is the one to copy.** It asserts `compared >= 15` *and* `edges >= 30`, and
the comment says why the second exists: *fifteen rows each comparing nothing to nothing is the green
this check already produced once.* **That is a population assertion that learned from its own
failure**, which is rarer than one that was right first time.

**One note on it, and it is small.** `declared` reads every manifest line with no section
awareness, so a runtime dependency and a dev one are the same to it. The document's column has been
made to match - `friendly-notation` in tests, `graph-coloring` in tests - so the two agree. **What
is unchecked is the annotation**: moving a dependency between sections changes nothing the check
reads, and *in tests* goes stale silently. This lens made the mirror-image mistake this morning,
counting dev-dependencies as runtime ones, so the hazard is named from both sides.

**Two notes on `neither_surface_has_grown`.**

- **`assert_eq!(watches.len(), 5)` is a tautology.** `watches` is a literal array of five strings;
  comparing its length to five cannot fail. The real checks are the two below it, and they are
  good. It is `CLAUDE.md`'s *a count summed over the hand list it was meant to check*, harmlessly -
  but it reads like a population guard and is not one.
- **The `declared` count is a text shape.** `line.starts_with("fn ") && line.ends_with(';')` counts
  nothing for a signature `rustfmt` has wrapped across lines, so **a sixth method whose signature
  exceeds the line width would leave the count at nine and pass.** Driven: a one-line declaration
  counts 1, a wrapped one counts 0. **Reachable rather than likely** - the longest signature in the
  file today is 47 characters, and the gate runs `cargo fmt`, so the wrapping would be done for
  whoever added it rather than by them.

---

## `Q-107` is refuted in the half that mattered, and this lens got the inference wrong

**`Q-107` was filed this morning saying the restructure was not worth doing.** Its reason:
*a `Res` wrapping a global is a declaration in form and not in fact.*

**The premise is true and `S-227` does not contradict it.** `TheOneConsole` holds nothing and every
method reaches `crate::shell::with`; its own doc says so. The `static OnceLock<Mutex<Console>>` is
still there and still process-wide.

**The conclusion drawn from it was wrong.** What a trait over a global buys is not the global's
removal - it is substitutability at the call site, and
`crates/game-front/tests/interfaces.rs:24` demonstrates it: `APretendGame` implements `Watches`, and
`a_caller_against_the_interface_can_be_handed_a_different_game` hands it to a caller that cannot
tell. **This lens collapsed *the global survives* into *nothing is gained*, and those are different
claims.**

**The measured half still stands, and it is now the useful half.** `Q-107` measured `game-globe` at
one test function against five systems reading a process-global. At `8fbb892f` the five systems
take `Arc<dyn Watches>` and **`game-globe` still has one test function.**

```
                 Q-107, e85c2980     now, 8fbb892f
game-globe       1 test fn           1 test fn
game-front       -                   47 test fns, incl. the fake
```

**So the capability arrived and the coverage did not**, and the fake lives in the crate that
defines the trait rather than in the crate whose systems were the reason for it. **That is not a
criticism of `S-227`** - it is what `S-227` made possible and what has not been done yet. Filed as
`Q-110`, which is `Q-107` with its wrong half removed.

---

## What this review did not do

**It did not run the gate**, which runs `cargo fmt` - this lens may not modify what it judges.
`cargo clippy --workspace --all-targets` and `cargo test --workspace` were run with
`CARGO_TARGET_DIR=target-quality`, lane-private.

**It did not verify who may tick a checkbox on this repository.** Finding [1](#1) is stated so that
it does not depend on the answer.

**It did not review the fourteen commits as a set.** `git log e25c0dca..HEAD` is 44 commits across
three lanes, and this went to the three places the code lane pointed at plus the artifacts those
reach. **The count in the request and the count in the log differ**, which is worth one line to
whoever is reconciling them and is not a finding.
