# Three reasons that were not the ones doing the work

**Derived.** 2026-09-08. Requested by the code lane at Sean's asking, everything open to that lane
being built. Not binding.

[Quality](README.md) · [Outbox](outbox.md) · [Reports](README.md#reports)

## The range, and a boundary that would have hidden the largest commit

They named `8797e60..dd93bd1`. **That range excludes `8797e60`**, which is the S-76 commit they
asked me to look at first and the largest in the set. Six commits, not seven. The range actually
reviewed is `8797e60~1..dd93bd1`, plus `9f46bd8` and `bb543c0`.

Recorded rather than filed, because nothing was lost - they described the commit in the message, so
the omission was in the notation and not in what they meant. It is worth a line only because
**`A..B` excluding `A` fails in the quiet direction**: the review runs, the commit is simply not in
it, and nothing says so. This lens has the same rule about baselines recorded from a clock, one
notation over.

## The through-line

Three of the four things they asked about hold, and **in three of them the reason given is not the
reason doing the work.** They found the third themselves and wrote it down - *sixteen groups was the
answer before by accident* - which is what makes it a pattern worth naming rather than three
unrelated notes.

| Where  | The claim                               | Holds? | The stated reason                            |
| ------ | --------------------------------------- | ------ | -------------------------------------------- |
| `S-76` | `expected/play.4x` does not change      | yes    | **false** - food does leave something behind |
| `C-69` | the extraction cannot test a stale copy | yes    | **too weak** - the guard cannot see it       |
| `C-71` | no two of sixteen kinds behave alike    | yes    | **theirs** - right before, by accident       |
| `C-55` | every way the tool refuses is covered   | **no** | the check is a constant                      |

A right answer resting on a wrong reason is `Q-54`'s shape, and the cost is always the same: the
reason is what the next edit is measured against, so a wrong one licenses the change that breaks it.

## 1. `S-76`: the reason is false, and the claim does not need it

**Where.** `8797e60`'s message, on `scenario/expected/play.4x`: *Territory 2 is claimed a turn later
and ends in the same state because food is discarded at every turn ending, so a farm worked one turn
fewer leaves nothing behind.*

**What.** The second half is false. `spec/turn.md:18` puts growth **before** the discard - *everything
with upkeep pays it; then a population grows on surplus food or starves for want of it; what expires
expires* - and `releases/first-release.md:253` makes `grow` consume surplus food and produce
citizens. **Citizens persist.** So a farm worked one turn more can leave a citizen behind, and food
worked is not simply discarded.

**The reasoning also proves too much.** If a farm worked one turn fewer left nothing behind, food
work could never matter at all, and the whole food economy would be a no-op.

**Evidence, in a clone at `dd93bd1`.** Adding one `create-labor` + `work resource:food` to the final
turn fails `the_reviewed_expectation_holds` at `expected_state.rs:454` - the comparison - with a
single different row:

```
different (1):
  {territory biome:grassland id:1 nature:1} {citizen ready:yes} · 8 → 12
```

**The instrument was controlled in both directions**, because a green run and a red one can each be
about the wrong thing. Removing `{found-by-land territory:2}` fails all six at
`expected_state.rs:71` - inside `played()`, a rejected command - which is the *session* being
sensitive and says nothing about the comparison. Only the added food work reaches line 454. And
removing one food work leaves all six green, so the file genuinely does not notice that direction.

**Why it matters.** They wrote *if that reasoning is wrong the claim is wrong*. **It is not.** The
claim rests on the poisoned test they ran, not on the sentence - so the conclusion stands and only
its reason has to change. What the sentence would license is the damage: a reader takes it as
permission to move food work between turns, and the probe above puts four citizens on that.

**The true reason is narrower and is about territory 2 alone.** It has one food extractor, so a
second `work` there in one turn is refused - *territory 2 has no extractor to work at*. Its single
turn of food does not reach a surplus that grows anybody, which is a fact about that territory's one
extractor and its store bound, not about food in general.

**Whether.** Worth correcting the recorded reason. **No code change** - the model, the scenario and
the expectation are all right.

## 2. `C-55`: the coverage check is a constant, and a fourth refusal walks past it

**Where.** `tools/anchor/tests/matching.rs:125`, `every_way_this_refuses_is_covered`.

**What.** It builds three errors in an array literal and asserts `refusals.len() == 3`. **The length
of a three-element array is three by construction**, and nothing ties that number to the number of
`Problem` variants. The name claims a coverage the test does not have.

**Evidence.** In a clone, a fourth variant `PlantedRefusal` was added to `Problem` with its `Display`
arm so the crate still compiles. **All nine tests pass.** The baseline is also nine, so the poison
did not change the population it acted on, and the poison landed in the file the test reads - the
enum it imports.

**Why it matters more here than it would elsewhere.** `tools/anchor` is the *carrier* for the two
rules that fire at a moment of confidence, built because a rule with only attention behind it is not
carried at all. Its own coverage check is the thing it exists to prevent: an assertion that cannot
fail, reading exactly like one that can. This is `Q-48` and `Q-51`'s shape inside the tool built
against that shape.

**One thing that softens it, and should be said.** `impl Display for Problem` matches every variant,
so a new one **cannot** be added without the compiler demanding an arm. The gap is therefore not
silent in practice today - it is the *test's* claim that is false, not the crate's safety. That is
why this is worth fixing and not urgent.

**Whether.** Worth fixing now, because it is cheap: construct the cases through an exhaustive match
over `Problem` so a new variant fails to compile until it has a case, rather than asserting a
literal's length.

**And a second, smaller thing in the same file.** `strip_prefix_per_line` at
`tools/anchor/src/lib.rs:37` is `pub` and **called by nothing** - one occurrence in the tree, its own
definition. It is not merely dead: it is the **superseded approach left reachable**, and `find`'s own
doc says why it was superseded - stripping first *would give offsets into a string that is not the
file, and mapping those back is a second map to get wrong*. A future caller reaching for the public
helper gets exactly that. Worth deleting or making private; noted, not urgent.

## 3. `C-69`: sound, and the guard is not what makes it sound

**Where.** `tools/hooks/src/lib.rs:30`, `column_of_source`.

**Checked and sound.** `hooks/pre-commit` has exactly one `column_of() {`, and the body contains no
`\n}\n` before its own closing brace - the `case` closes with `esac`. So the extraction takes the
whole function today, and their instinct to run the hook's own text rather than a Rust copy is
right: a copy is what rots.

**Where it is weaker than it looks.** The guard asserts the extracted body contains `lenses/` and
`pending.md`. **Both sit in the first four lines of a twenty-line function**, so the guard cannot
detect a truncated extraction - it would pass on a body cut off anywhere after line four. What
actually protects against truncation is that a truncated shell function does not parse, so `sh`
returns non-zero and `column_of` asserts `out.status.success()`.

**Whether.** Noted and deliberately not. The protection is real, it is just somewhere else than the
assertion suggests. If the guard is ever meant to carry that weight, a marker from the *last* case
arm would do it.

## 4. The prose-predicate gap cannot flip `R-8`, and that is worth knowing

They put on the record that they did not touch the six traits declared of a prose predicate, so
`ready` - *whatever readies* - and `movable` - *whatever moves* - are attributed to no kind
(`releases/first-release.md:115` and `:132`).

**That gap cannot change `R-8`'s answer, and the direction is the reason.** A signature's key is its
sorted traits joined to its pairs (`catalog.rs:431`). **Attributing more traits is monotone**: a
trait given to both of two kinds leaves them equal, and one given to only one splits them. So
filling the gap can only **split** groups, never merge them - and *no two of sixteen behave alike*
is exactly the claim that survives splitting.

Said because it is not obvious from the gap itself, and because it means an open hole is not
sitting under a vetted answer. If the sixteen had collided, the gap would have been a live suspect;
they did not, so it is not.

## 5. A fresh clone of this repository fails its own suite, and no existing tree can see it

**Not in the range, found by running it.** This is the one finding here that is not about the
commits reviewed.

**Where.** No `.gitattributes` exists; `core.autocrlf` is `true`; `crates/game-console/tests/dump.rs:324`
reads `reports/turns.md`.

**What.** The generated reports are committed with LF. On checkout with `autocrlf=true` - this
repository's own setting, and the Windows default - they arrive as CRLF, and
`every_turn_of_the_scenario_is_dumped` then fails with *turns.md has 10 sections and no `Turn 1`*,
because the section title it parses ends in a carriage return.

**Measured.** A fresh `git clone` of this repository at `dd93bd1` fails that test - single-threaded
and in isolation, so it is not a race. `reports/turns.md` in the clone is 88817 bytes with 2974 CRLF
and no bare LF; the same blob in the working tree here is 85843 bytes with 2974 LF and no CRLF. Both
are `git status` clean, because the filter normalizes them to the same blob. **Rewriting the clone's
copy to LF and changing nothing else turns the whole binary green: 10 passed.**

**Why it matters, and why nobody has hit it.** Every existing working tree's copy of these files was
**written by the generator**, with LF, rather than checked out - so no instance running here can
observe it. And CI cannot either: the `gate` job that runs the tests is `ubuntu-latest`, where
`autocrlf` is off, and the only `windows-latest` job builds without testing. **So the one platform a
person actually runs the suite on is the one where a fresh checkout is red, and the one platform CI
tests is the one where this cannot happen.** `hooks/pre-push` runs the full gate, so on a fresh
Windows clone the first `git push` fails for a reason that has nothing to do with the change.

**Whether.** Worth fixing now. It is the gate, and it fails in the only direction nobody inside an
existing tree can see. A `.gitattributes` settling the endings is the general fix and a
CRLF-tolerant parse is the local one; **which is the code lane's call**, not this lens's.

**And the instrument nearly hid it, in the shape this lens has already recorded once.** `grep -c
$'\r'` reported **zero** carriage returns in both copies - MSYS `grep` strips them - and on that
number this lens first concluded the files were identical and the failure was real at their tip. It
was a phantom in one direction and a real defect in the other, and only reading the bytes told them
apart. This README already carries the rule from the last time, with `git show` and `grep` named:
**the tool that filters line endings is the tool you cannot ask about line endings.**

## What was checked and found nothing

Recorded so a later report does not present these as unexamined.

- **The `S-76` turn-apart consequence in the scenarios.** Both cross on one turn and found on the
  next, the comments describe the commands rather than the intent, and the expectation is untouched.
  Verified by the probe above rather than by reading.
- **`C-71`'s arithmetic.** Sixteen kinds give 120 unordered pairs, which is what the report claims.
- **The two suites this review poisoned**, named rather than called *the suite*, because a green run
  bounds what it covers and nothing more. `game-console`'s `expected_state` - six tests - and
  `tools/anchor` - nine - were green in a clone at `dd93bd1` before any poison, and green again after
  each was reverted.
- **The full workspace**, which found the fifth thing below and is the reason this report has one.
