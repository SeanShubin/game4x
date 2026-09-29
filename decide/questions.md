# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-585 - `R-9`, `R-10` and `R-11` wait on your eye and there is nothing to look at

**to** sean · **status** open · **raised** 2026-09-28 · **asks** a decision · **kind** measured · **from** `C-164`

**`pending.md` has been listing three capabilities as waiting on you while their evidence was
deleted.** `D-4` asked for those deletions - *no report is left describing a rule the game does not
play by* - and the reports they rest on were generated from the old ruleset's scenario. **The
deletions were right; nobody said what they cost.**

**Measured here against what `reports/` holds after `09f628d7` rebuilt it over the new engine:**

```
R-9    twelve territory pages, two stylesheets, a sibling per view, no page with a script
       now: 0 territory pages, 1 stylesheet, 6 sibling pairs

R-10   reports/petri.html with 295 of 295 labels declaring a fill, a drawing per recipe
       now: reports/petri.* does not exist

R-11   nineteen links over three directories, each to a .txt twin of a .4x the engine reads
       now: the index has 9 links and none of them reaches a .4x

R-12   the foundation form of every reviewed test, in the reports
       now: reports/foundation/ is 55 files, written from reviewed/ - untouched
```

## The choice, and it may differ per capability

**Retire, the way you retired `R-6`, `R-7` and `R-8`** - *if these mechanics come back I will read
them then*. **Only you retire one**, and a retired capability waits on nobody, which is the state
these three should be in if the answer is *not now*.

**Or reword, over the reports that exist.** `R-11` is the cheapest: its clause is about reaching the
engine's inputs and the engine still has inputs, so what moves is the wording rather than the intent.
`R-9`'s clause is about a browsable set of pages and there is one again, but it names twelve
territories and two stylesheets by hand. **`R-10` has no counterpart at all** - nothing draws a net
over the new engine.

**Or wait for `D-1`.** The console has not moved onto the new engine yet, and a report over a model
that is still being replaced would be vetted twice.

**This lane recommends nothing across all three**, because they are not alike: `R-10` has no
artifact, `R-11` has one under another name, and `R-9` has one that is missing two named things.
**What would help most is one word per capability.**
