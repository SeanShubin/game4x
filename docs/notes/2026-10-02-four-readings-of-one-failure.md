# Four readings of one failure, each narrower than the question - 2026-10-02

**`Checks (every test suite)` failed three runs in a row on the same step, and four instruments
in a row answered a narrower question than the one asked.** Every one returned a plausible answer
rather than an error, and the last was inside the correction of the third.

**How to re-run any of it**: the run is `37079431559`, the log reachable with
`gh run view --job <id> --log`, and the per-job package lists with the script at the end.

## The four

**One, the specification lane's hypothesis.** *`Checks` installs a plain toolchain where `Gate`
installs Bevy's Linux dependencies.* **Stated as a hypothesis beside its measurement**, which is
the only thing that went right - the measurement was *that exact command exits 0 here*, and the
explanation was separate and wrong.

**Two, the code lane's refutation.** `bevy deps: True` for `checks`, read from the file. **A true
reading of *does the step exist* where the question was *which packages does it install*** - so it
refuted the reason and took the mechanism with it, and sent both lanes to cargo's feature
unification.

**Three, the specification lane reading the log.** The log named `wayland-client`, and the item said
**`Checks` needs `libwayland-dev` and nothing else changes.** **The log names the first package to
fail, not the set that is absent.** `libxkbcommon-dev` was missing too and would have failed next.

**Four, and this is the sharpest, the instrument that found three.** The per-job comparison that
produced the right answer used a regex of four hand-written name patterns -
`libwayland|libx11|libasound|libudev`. **It asked *which of these four appear* where the question
was *what is the whole list*.** It could not have seen `libxkbcommon-dev` and reported a complete-
looking list anyway. **Written inside the correction of the same class, by the lane naming it.**

## What the sequence shows that no single instance does

**Each step forward was a true measurement**, and three of the four conclusions were wrong. **The
only thing that survived was the habit of saying which half was measured** - *measured: the two
commands differ; inferred: feature resolution* - because a receiver could then discard the
inference without discarding the fact.

**And the cause was mundane**: `S-251` split the tests out of `gate` into a new `checks` job, and
the step's **name** was copied while its **package list** was shortened. Four copies of the list
existed and nothing compared them.

## The carrier, which is the code lane's

**`every_job_that_links_bevy_installs_the_same_packages`** in `tools/outbox/tests/architecture.rs`
parses every `apt-get install` line with the job it sits in and asserts the lists are equal.
**Both floors are there**: `lists.len() >= 3`, or an equality over one list passes vacuously, and
`packages.len() >= 4` per list, **or jobs installing nothing would agree with each other.**
Verified by putting the bug back in `checks` alone - red, naming the job - then restoring.

**Measured after the fix**: four jobs, four packages each, all four lists identical.

```python
import re
t = open(".github/workflows/pipeline.yml", encoding="utf-8").read()
for m in re.finditer(r"^  ([a-z-]+):\n", t, re.M):
    job = m.group(1)
    nxt = re.search(r"^  [a-z-]+:\n", t[m.end():], re.M)
    body = t[m.end(): m.end() + (nxt.start() if nxt else len(t))]
    for line in body.splitlines():
        if "apt-get install" in line:
            pkgs = [w for w in line.split() if w.startswith("lib") or w.endswith("-dev")]
            print(job, len(pkgs), sorted(pkgs))
```

**That reads the whole list rather than four names**, which is the difference between instrument
four and the one above it.
