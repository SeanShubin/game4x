# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-579 - One regression case still says the ark kept its move, and only you can accept that it does not

**to** sean · **status** open · **raised** 2026-09-27 · **asks** a decision · **kind** entailed · **from** `C-157`

**The whole workspace is green but for one test**, and what closes it is a deletion no instance may
make. `scenario/regression/t01-gather-1.4x`, line 24:

```
was  {ark where:place-2 moving:1 gathering:0} -> 1
now  {ark where:place-2 moving:0 gathering:0} -> 1
```

**The cause is your own edit** - 2026-09-27, `8c4e0501`: *go ahead and make the edit so that the ark
moves once before the rest happens.* The ark spends its move before `gather` runs, so `moving` is
`0` where the case recorded `1`. **The case is stale rather than wrong about what it saw.**

## The two ways this goes

**Delete `scenario/regression/t01-gather-1.4x`.** The next run writes it back saying `moving:0`, and
the diff is the review. `CLAUDE.md`: *a generated regression case is deleted by Sean, and that
deletion is an approval* - **no instance deletes one while the command it covers is still played**,
and `gather` is still played, at `scenario/main.4x:95`.

**Or the case is right and the order is wrong**, in which case the engine changes and the case
stays. That is the reason this is a decision and not a chore: the case is the only thing in the
repository still asserting the old order, so deleting it is the last place that order could have
been reconsidered.

## What was measured, and by whom

**The code lane bisected it in a detached worktree rather than reading dates**: green at `7282af71`,
this same red at `8c4e0501`, at `e5c15a7b` - the commit before `P-577` - and at `HEAD`, byte for
byte identical at all three. **So it predates both biome promotions by 17 commits** and neither is
implicated.

**Re-derived here rather than taken from their report**: 690 tests pass and one fails,
`every_command_has_an_expectation_and_it_is_current`, on 1 of 35 cases.

**Nothing is pushed.** `hooks/pre-push` runs the full gate, and this one test holds it.

