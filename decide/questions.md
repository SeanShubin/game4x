# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-616 - Do you still want the issue-checkbox route, now that you use the page?

**to** sean · **status** open · **raised** 2026-10-02 · **kind** a question only you can answer · **asks** a decision · **into** nothing yet - the answer decides whether anything is written

**You asked for the issue route in `S-228`** - *a task list in an issue, ticked from the GitHub app.*
**It was built, and you have never used it.** You approved all 63 tests from `reports/review/`
instead.

```
review.yml       zero runs, ever
gh issue list    no issue exists, in any state
your approvals   63 of 63, all through the page
```

## Exercising it costs you two gestures and nobody else can make either

**Creating the issue is outward-facing and no lane does it**, and ticking a box is the gesture
itself. **A dispatch cannot stand in**: `relist` is the only job a `workflow_dispatch` reaches, it
rewrites an issue body and **never writes a record**, and it takes an issue number it has nothing to
be given. The job `X-47` fixed is `apply`, and only an `issues: edited` event reaches it.

## What one tick would prove, and what it is the only way to prove

**The owner gate, the record write, the push with `GITHUB_TOKEN`, and `Generate` firing on
`workflow_run` rather than `push`** - end to end. **Only one of those is shared with the page
route**, so 63 approvals have exercised none of the rest.

## So the question is whether it stays

**If you want it**, create an issue once and tick one box, and the route is tested rather than
correct-by-reading. **If you do not**, `review.yml` and `X-47`'s fix guard a path nobody will take,
and retiring them is cheaper than keeping them right.

**This lane has no view and will not pick one for you.** What it will not do is leave an untested
route in the tree with nothing recording that you were asked.
