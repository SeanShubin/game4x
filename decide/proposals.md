# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-619 - What runs is decided by what changed, in four cases

**to** sean · **status** open · **raised** 2026-10-04 · **kind** his requirement, in his own four cases · **shape** text · **asks** approval · **into** `docs/process.md` -> What I am pushing out

**Sean, 2026-10-04**, after rejecting this lane's suggestion that he ask for the long one:

```
code changes:    long build and deploy
test changes:    short build and deploy
both change:     short build and deploy, then long build and deploy
neither change:  does nothing but give me a message
```


**Measured over the last 40 commits**, with `reports/` counted as the tests' published form:

```
          commits   today                        under his rule
code            9   long, 76 min                 long
tests           4   short 26s + 76 min of noise  short, 26s
both            3   long only - the page waits    short, then long
neither        24   publish + 76 min of noise    a message
```

**Twenty-four of forty is the commonest case and it is the one that does nothing**, which is the
opposite of what this lane would have guessed. Every one of them runs `Checks` and `Sweep` today.

The paragraph offered:

> **What runs is decided by what changed.** A push that touched code gets the long build and the
> deploy. A push that touched the tests gets the short one. **A push that touched both gets the
> short one and then the long one**, because the page was ready first and the analysis has its own
> schedule. **A push that touched neither does nothing and tells me so** - which is most of what
> lands here, and none of it needs a build.

## One boundary this lane settled, and says so

**`reports/` counts as the tests, not as neither.** It is their published form - the page he reads
is generated from them - so a commit that regenerates reports without touching a test still has to
publish, or the site goes stale while the run says it did nothing. **Three of the forty move bucket
on this**: with `reports/` as neither, the counts are 12 / 3 / 0 / 25.

**It is a judgement rather than a measurement**, which is why it is named here rather than left in
the words.

## What *a message* is, which this lane will not invent

**It has to be something you see without looking**, or it is the same as silence. **A green run whose
only job says *nothing to build* is the cheap version** and it is still something you have to open.
**Whether that is enough is yours** - the alternative is a notification, which is a mechanism this
lane has no view on.

## And one thing that does not change

**`P-613` still holds.** Nothing here makes a failure withhold the page; the difference is only
which jobs start and when.
