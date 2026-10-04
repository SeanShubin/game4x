# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-619 - What ran is decided by what changed, and both can run at once

**to** sean · **status** open · **raised** 2026-10-04 · **kind** his requirement · **shape** text · **asks** approval · **into** `docs/process.md` -> What I am pushing out

**Sean, 2026-10-04**: *why do I have to ask for the long one at all? What about a setup where if the
code changed I get the long one, if the tests changed I get the short one, and if both changed I get
both.*

**Measured today, per job, from run `37137539481`:**

```
                      what changed:  tests only   code only   both
publish the reports                        26s         26s   * 12 min
the long analysis                      * 76 min     76 min     76 min
```

**Two stars, and both are the asking you object to.** A tests-only push runs `Checks` and `Sweep`
anyway - 23 and 53 minutes of analysing code nobody touched - and a push that changed both makes you
wait for the gate before the page updates, though the reports were ready in six seconds.

The paragraph offered:

> **What runs is decided by what changed, and both can run at once.** A push that touched only what I
> review publishes the page and runs nothing else. A push that touched code runs the long analysis.
> **A push that touched both does both** - the page first, because it was ready first, and the long
> one on its own schedule.

## What it costs to build, which is small

**`Checks` and `Sweep` get the condition `gate` already has.** They have no `needs:` at all today,
which is why they run on everything.

**And publishing stops waiting for the gate.** The deploy that carries the last bundle needs nothing
from the gate - that is what `P-615` built - so it can run at 26 seconds and **a second deploy after
the gate republishes with the new game.** Pages replaces the whole site each time, so the first
publish is *fresh reports with yesterday's game* and the second is *fresh reports with today's*.

## The one choice this lane will not make quietly

**Whether the fast publish runs on a code-only push.** It would publish reports that did not change,
which is harmless and pointless - 26 seconds of nothing. **Leaving it in makes the rule one sentence
with no exceptions; taking it out saves nothing you would notice.** This lane would leave it in and
says so rather than deciding it inside the words above.

## And one thing it does not change

**Nothing here makes a failure stop publishing.** `P-613` settled that and this builds on it - the
difference is only *when* the page appears, not *whether*.

