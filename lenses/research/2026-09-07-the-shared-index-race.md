# The staging rule cannot prevent the thing it describes

2026-09-07. Carries the argument for [`X-7`](outbox.md). **The event is the code lane's**, reported
by it and verified here against this lane's own commit.

**Corrected 2026-09-07, hours after filing, and the correction is the finding.** This report was
written as *here is a mechanism that would remove the hazard*, and addressed `X-7` to the code lane
to build it. **It is already built.** `hooks/post-commit` exists for exactly this, wired in `a60def3`
after the quality lens found the residue a pathspec commit leaves. What is missing is not the
carrier but the instruction to use it: `pathspec` appears in that hook and in the quality lens's own
files, and **nowhere in `CLAUDE.md` or `docs/process.md`**, which still name staging by name as the
remedy. `X-7` is re-addressed to the specification lane and asks for a sentence, not a mechanism.

**This lane found that by reading the hook its own commit had just printed a message from** - which
is to say, late, and only because the output was in front of it. The finding it filed was a claim
about the tree made without reading the part of the tree that answers it, which is `S-56`'s shape
and sits in a report about exactly that.

## What happened, verified

`8f687d5` is this lane's commit and contains **21 lines of `crates/outbox.md`**, which is outside this
lane's column and unmentioned in its message. `git show --stat 8f687d5` names six files where the
work was five. The code lane staged that file, lost the race for `.git/index.lock`, and this lane's
`git commit` committed the index.

**`CLAUDE.md:129` describes this exact failure, from a previous occurrence of twenty-six lines.** So
the rule was written, read by every lane, and did not prevent the second occurrence.

## Why it could not have

The rule's remedy is **stage by name, never `git add -A`**. This lane staged by name -
`git add lenses/research/` - and the failure happened anyway, because **staging by name bounds what
*you* add and the hazard is what *someone else* added.** `git add` and `git commit` are two
operations, and everything staged by anyone in the gap between them is published by whoever commits
second.

The code lane checked for the lock before adding, which does not help either: the window is after
the check. **That is a time-of-check-to-time-of-use race on shared mutable state**, and naming it
that way is what says why no amount of care closes it - the check and the use are separated by
design, and the state between them is writable by another process.

**So the rule is aimed at the wrong operation.** *Stage by name* is good advice about the blast
radius of a mistake of your own. It says nothing about the index being shared, which is the sentence
directly above it and the actual hazard.

## What removes it, measured

Git has a mode that does not consult the shared index: **giving `git commit` the paths directly**,
which implies `--only` and builds the commit from a temporary index.

**Measured in a throwaway repository**, three runs, re-runnable at
`scratchpad/racetest`: one file staged by a simulated other lane, one file modified by this one.

| What was run                                                                         | What was committed                       | Their staged work                                       |
| ------------------------------------------------------------------------------------ | ---------------------------------------- | ------------------------------------------------------- |
| `git add mine.md && git commit -m ...`                                               | **both files** - the failure, reproduced | swept into my commit                                    |
| `git commit -m ... -- mine.md`                                                       | **mine only**                            | left staged and uncommitted, ready for their own commit |
| the same, with a `pre-commit` hook that regenerates a derived file and `git add`s it | **mine and the derived file**            | untouched                                               |

**The third row is the one that decides whether this is usable here**, and it is the row this lane
expected to fail. `hooks/pre-commit` regenerates `pending.md` and stages it at line 96, and pads
tables and stages them at line 52. A path-limited commit builds from a temporary index, so a hook
writing to the main one could plausibly have been ignored. **It is not**: the derived file landed in
the commit. The `pending.md` mechanism survives the change.

## What this lane did not settle

- **The derived file was left `MM` afterwards** - staged and modified again. **This lane listed that
  as unresolved and it was already resolved**: `hooks/post-commit` unstages it, and this lane's own
  commit printed the hook doing so while the sentence claiming otherwise was still in the file.
- **Whether `hooks/pre-push` and the padding path behave the same way** was not tested.
- **Whether the lock contention returns as a loud failure.** It should: two commits racing for the
  lock is a command that fails rather than a commit with the wrong content, which is the right
  direction, but this lane did not force the collision to confirm it.

**None of that is this lane's to build.** Commits, hooks and the pipeline are production support and
belong to the code lane; `CLAUDE.md`'s sentence belongs to the specification lane. What is offered
here is the name of the failure, the measurement, and the row that says the obvious objection does
not hold.

## How to re-run it

```
git init racetest; commit two files; stage one as "theirs"; modify the other as "mine"
run each row above; git show --name-only HEAD says what landed
```

The hook row needs a `pre-commit` that writes a file and `git add`s it - four lines.
