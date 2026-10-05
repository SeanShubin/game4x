# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-620 - `P-613`'s sentence says a broken game is published, which is not what you meant

**to** sean · **status** open · **raised** 2026-10-05 · **kind** his correction of his own promoted text · **shape** an instruction · **asks** approval · **into** `docs/process.md` -> What I am pushing out

**Sean, 2026-10-05**: *you said a failing run publishes anyway - true, but I meant it publishes what
it can. I don't expect it to publish an artifact that can't be built.*

**`docs/process.md:250` currently says the other thing.** The sentence you approved on 2026-10-02:

```
**The whole site publishes whether or not the run succeeded.** A failing build leaves a broken
game published rather than nothing published, because deploying is how I verify and a staging
area that vanishes when it breaks is no use to me.
```

**A failing build produces no artifact at all**, so *leaves a broken game published* cannot be
carried out as written - there is nothing to publish. **What is published is the last game that
built**, which is what the code lane built and what you have just said you meant.

## The instruction

**`a broken game published` becomes `the last game that built published`.** Nothing else in the
sentence moves.

**The check**: `docs/process.md` then contains *A failing build leaves the last game that built
published rather than nothing published* and does not contain *a broken game published*.

## Why this is a correction rather than a quibble

**The code lane already built the right thing** - the job is named *Publish (the reports, with the
last game that built)* - so nothing is broken today. **What is wrong is that the document a future
reader checks against says something the code deliberately does not do**, and the gap is the
direction that would look like a defect: somebody reading the rule would file the carried bundle as
not obeying it.

**And `S-257` is the case where it mattered.** The publish jobs failed, nothing was deployed, and the
rule as written offered no reading of what should have happened - *publish anyway* and *there is
nothing to publish* were both true at once.

