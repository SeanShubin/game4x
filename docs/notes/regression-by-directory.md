# Regression by directory, as `code-structure` does it

**2026-09-04.** Sean pointed at `D:\keep\github\sean\code-structure\console\regression-test` and
asked which parts of that workflow belong here. Read rather than recalled: the harness is
`console/src/test/kotlin/com/seanshubin/code/structure/console/RegressionTest.kt`, and the two
directories are `expected/` and `memory/`, each holding one subdirectory per case - `kotlin-class`,
`kotlin-jar`, `elixir`.

## What it actually does

**`expected/` is committed output, not assertions.** 1.4 MB of it: `browse/*.html`, `*.svg`, `*.txt`,
`count/`, `diff/`, and `observations.json`. The test runs the real application into
`target/regression-test/<case>/` and compares the two trees file by file.

**Three named tests, each twenty lines, differing only in one string.** The duplication is a wart
rather than a pattern and does not transfer.

**`seedExpectationIfNecessary` is the whole workflow in five lines.** If `expected/` does not exist,
copy `actual/` into it. **Absence means acceptance**, so updating an expectation is a deliberate
deletion rather than an edit.

```kotlin
private fun seedExpectationIfNecessary(expectedDir: Path, actualDir: Path) {
    if (Files.exists(expectedDir)) return
    ...
}
```

**`validateDirectoriesEqual` walks both trees and reports three lists** - `missing`, `extra`,
`different` - and `RegressionSummary.regressionCount()` sums them. The assertion is
`assertEquals(0, count)` with every path in the failure message.

**`extra` is the direction most harnesses omit.** Walking only the expected side passes while the
program grows output nobody asked for. Walking both catches it.

**`memory/` is recorded nondeterminism.** `RememberingClock` reads `clock.txt` into a list of
instants and serves them in order; when it runs out it calls the real clock **and appends the result
to the file**. So the first run records a real timeline and every later run replays it. Time stops
producing diff noise without anyone inventing a plausible constant.

**Version control is the diff tool.** Sean's own description of the loop: make possibly drastic
changes, diff the results in version control, decide whether they are what he wanted. If not, change
the code. If so, delete `expected/`, rerun, commit.

## What transfers

**The regeneration protocol.** `P-219` says the scenario test locks the expected data. **It does not
say how the expected data is updated when Sean changes his mind**, and the unstated answer - edit
the file - is exactly the act that cannot be told from an accident. Deletion can. `P-225`.

**The three-way comparison and its count.** `missing`, `extra`, `different`, summed and asserted at
zero. This is [*check the rule over every case and assert how many cases there were*](../../CLAUDE.md)
in a second form, and `extra` is the half this repository's dump tests do not have: `dump.rs` has
seven tests, all about shape, and none of them notices a file the program started producing. `S-32`.

## What does not transfer, and why

**`memory/`, because there is nothing to remember.** Measured 2026-09-04: `grep -rn` for
`SystemTime`, `Instant::`, `rand::`, `thread_rng` and `now()` across `crates/` returns one hit,
`crates/sphere-tessellation/src/icosahedral.rs:767`, and it is inside `#[cfg(test)]` - a benchmark
that prints how long generation takes. **The game path has no clock and no random number generator**,
and no generated file carries a date. The pattern is worth remembering for the day one arrives; it
is not worth building now.

## The failure this could introduce

**Seeding on absence is safe in `code-structure` because there is always a diff to read.** Delete a
directory that had content, regenerate, and version control shows exactly what changed - so
acceptance is always an act of review.

**The first seeding is the exception, and it is the one this repository would be doing.** With no
prior expected data, the diff is the whole file arriving, and there is nothing to compare it
against. Accepting it means accepting that the program is right because the program produced it -
which is [the fifth face](silence-is-not-agreement.md), a comparison of two things that agree being
unable to notice that both are wrong.

**So the first expected data has to come from the pencil, not the program**, which is `S-24`'s
closure test and is already open. After that, seed-on-absence is exactly right.
