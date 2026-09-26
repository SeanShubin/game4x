# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-562 - retire `R-6`, `R-7` and `R-8` unread, which needs a fourth state the ladder does not have

**to** sean · **status** open · **raised** 2026-09-25 · **answered** 2026-09-26, `W1` · **asks** approval · **kind** entailed · **shape** text · **into** `releases/first-release.md` -> Capabilities, and `releases/README.md` -> Vetting, and deletion

**You said `W1`**: *retire them unread, if these mechanics come back I will read them then.* **This
is the same item asking approval instead**, because an answer to a question is not a promotion.

**What it needs that does not exist.** The ladder is `open` -> `built` -> `vetted` and both
documents say *three states*. **`retired` is a fourth**, and without it the three would sit at
`built` and stay in your queue - `tools/outbox` counts `open` and `built` as outstanding and
nothing else, so the status is the whole mechanism.

**The status lines themselves are not offered here.** A promotion writes an item's addressing line,
as it does for every outbox item; what you are approving is the two places that say what the state
means.

## The words, one

**Replacing the sentence and its three bullets in `releases/first-release.md` -> Capabilities:**

> Each capability is an item in the same shape every outbox uses, so it appears in `pending.md` and in
> `tools/outbox` beside what a lens has found. It moves through four states and changes hands once:
>
> - **`open`, addressed `to code`** - not built yet
> - **`built`, addressed `to sean`** - the code lane says it is done, and nobody has looked
> - **`vetted`** - a person has observed the *vetted when* line and it held
> - **`retired`** - the thing it was evidence about is being removed, so nobody will observe it.
>   **Only Sean retires one**, and the reason is recorded where the status is. A retired capability
>   is not a failed one: the work was done and reported, and what lapsed is the reading

## The words, two

**Added to `releases/README.md` -> Vetting, and deletion, before *When every capability in a
release is vetted*:**

> **A capability can also be retired, and then it is not waiting on anybody.** When what a
> capability was evidence about is being taken out, observing it would confirm that something
> being removed worked. **Only Sean retires one**, and a release finishes when every capability is
> either vetted or retired. **Retiring is not deciding the observation was worthless** - it is
> deciding that this is not when to make it, and if the mechanics return so does the reading.

## What this promotes against, so you can check it landed

```
R-6  The loop can be played through          built 2026-09-11  ->  retired
R-7  Each recipe can be confirmed on its own built 2026-09-08  ->  retired
R-8  I can see which kinds behave alike      built 2026-09-07  ->  retired
```

**Your reason travels with each of the three**, in your words: *if these mechanics come back I will
read them then.*

## What this leaves, and it is the part worth checking before you say yes

**`R-9` through `R-12` stay `built` and stay yours.** None of the four is about the old mechanics,
and `R-12` is about the tests you reviewed. **So the first release does not finish on this
promotion** - four capabilities still wait on your eye, and the file is deleted when they are read.

**And `R-6`'s observation is owed by something.** *The loop can be played through* is the only
capability anywhere that says a person watched the game work. Retiring it means no such observation
exists. **This lane will file it as a capability of `releases/rules-become-data.md`** so the new
model owes what the old one is being excused - filed after this lands rather than folded into it,
because it is a new capability and not part of retiring these.
