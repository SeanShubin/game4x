# The cold-instance study, designed and not run

2026-09-07. Prepares [`X-5`](outbox.md). **Nothing here has been run**, and the predictions below
are written before running it on purpose - see *What is predicted*.

## What `X-5` asked for, and what a cold instance can actually show

`X-5` says: run an instance that has read the files and nothing else against `S-57`'s seven cases,
and test whether a fresh reader avoids each failure or reproduces it. **Sorted by the phenomena [`X-3`](2026-09-06-answering-from-memory.md) found - six, after the
specification lane refuted one of the classifications - three are not runnable against a cold
instance**, so four of the seven cases are visible and three are not.

| Phenomenon                             | Cases            | Visible to a cold instance?                                                                                                                                                           |
| -------------------------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Stale context                          | `P-320`          | **No, by construction.** A fresh instance has no memory to answer from. It will pass, and passing says nothing - the condition being tested is absent                                 |
| Reading past a clause that was present | `C-35`, `C-34`   | **Yes.** The clause is still in the file. This is the case the study is for                                                                                                           |
| Misparsing a clause that was read      | `P-315`          | **No, and for a third reason: the document was repaired.** `b1d12c9` withdrew the proposal and rewrote the sentence in one commit, so the stimulus is not in the tree to be misparsed |
| Never reading at all                   | `S-56`           | **Yes.** The tree still shows what was already done                                                                                                                                   |
| A record destroyed by a process step   | `P-310`, `P-312` | **No.** The promise was deleted at promotion. No reader finds what is not there, and a cold one has no advantage                                                                      |
| A copy gone stale on disk              | quality's README | **Yes, and it is the case where cold is worse.** `X-3` predicts a fresh reader believes it completely                                                                                 |

**So the study is three tasks and not seven, and `X-5` overstates its own scope.** The three are one
per runnable phenomenon. That is worth
recording whether or not it runs: two of the six phenomena are not properties of a reader at all, and a
third had its stimulus repaired out of the tree - so no instance, fresh or otherwise, is the
instrument for any of the three. **The three tasks below are one per runnable phenomenon**, not one per case: the *read past a clause*
row holds two cases and they fail the same way.

**A miscount in `X-3`, found while writing this, and it is this lane's own.** `X-3`'s report is
precise - its heading reads *the question names one phenomenon and the cases are four*, and its table
has five rows, one named by `S-57` and four not. **Its one-line summary in the outbox dropped the distinction** and says *the seven cases are four
phenomena*, which is wrong twice over - the report's own table has five rows, and the specification
lane's refutation of `P-315` makes it six.
The compressed form is the one that travelled, into `X-3`'s own item and into the specification
lane's close of `S-57` at `docs/notes/proposals.md`. **The restatement went stale against a source
that never moved** - the same shape as the quality README in `X-3`'s own fifth row, one day later and
in the file arguing about it.

## The cheap design, and why it would return a plausible number

The obvious version is to hand a cold instance the seven cases and ask which it would have got
right. **It cannot answer that**, and it will produce an answer anyway.

- **It measures findability, not finding.** Every one of the seven had the information one tool call
  away. An instance told *the answer is in these files* will find it, and that result is already
  known - it is what makes the cases embarrassing rather than hard.
- **Being studied is the treatment.** The failure in all three live cases happens while the instance
  believes it is reading enough. An instance told it is being tested on reading reads more.
- **It is the shape `docs/process.md` names**: the instrument answers a narrower question than the
  one asked, and **a right number about the wrong thing invites no question.**

## The design that would answer it

**Put the decision point back as ordinary work, and do not say it is a study.** Each of the three
live cases reduces to a task whose correct execution requires opening one file, where a plausible
wrong answer is available without opening it.

1. **`C-34`'s shape** - ask for a claim about a population, where the clause that refutes the
   obvious count sits inside the entry being read. Correct: the smaller number. Failure: the count
   the entry's opening implies. **`C-35` is the same shape** and its instruction sits several
   paragraphs below a title about something else, which is the sharper version.
2. **`S-56`'s shape** - ask for work to be filed against a state of the tree. Correct: look, and find
   it already done. Failure: file it from the document that requested it.
3. **The stale-README shape** - ask what a lens's jobs are. Correct: `docs/process.md`, which owns
   them. Failure: the lens's own README, which restates them and is four days behind.

**The third is the one to run first if only one runs.** It is the case where `X-3` predicts a cold
instance does *worse* than a long-running one, and a study that can only confirm *fresh is better*
is a probe aimed where the check already looks.

## The prompt

Give the instance its ordinary lane prompt from `docs/process.md` -> Starting the instances, unchanged,
and then the three tasks as ordinary work, one at a time, in that lane's own voice. **Nothing about
`S-57`, this report, or a study.** What is recorded is: for each task, which files it opened before
answering, and whether the answer matches the file that owns the fact.

The lane to run it as is **quality**, because all three tasks are natural for a lens and none of them
writes anything - so a wrong answer costs a paragraph rather than a commit.

## What is predicted, written before the run

Pre-registered so the run can refute rather than confirm. **`X-3` is mine, and a study I design after
seeing my own answer will agree with it** unless the predictions are fixed first.

| Case           | `X-3` predicts                                                                                                  | What would refute it                                                                                                                |
| -------------- | --------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `C-34`'s shape | Cold **passes.** The refuting clause is inside what it is already reading                                       | Cold takes the opening count, which would mean re-reading is not the remedy for this row either                                     |
| `S-56`'s shape | Cold **passes**, and this is the weakest prediction - a fresh instance has no habit of checking the tree either | Cold files without looking, which would move this case out of *never read* and into *nothing prompts a read*                        |
| Stale README   | Cold **fails**, and a long-running instance might not                                                           | Cold reaches `docs/process.md` unprompted, which would refute `X-3`'s central claim that clearing context makes two phenomena worse |

**The third row is the study.** The first two can only confirm; the third can cost `X-3` its answer.

## What it costs

One instance, three tasks, no writes. It fits inside the startup of an instance being restarted for
other reasons, which is what makes it worth putting to Sean now rather than at its turn in a queue.
**If he declines, `X-3`'s answer stands** - it is a derivation and says so - and what is lost is the
ability to say how much of a long-running instance's belief is memory.
