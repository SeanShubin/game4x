# Two budgets, counted as one — a correction

**Derived.** Written by the quality lens on 2026-08-30. **A correction to this lens's own
recommendation**, not an observation about the tree.

[Quality](README.md) · [The outbox](outbox.md) · [Where the rule came from](2026-08-29-workflow.md)

Sean: *"My current confusion is why there are no proposals in `docs/notes/proposals.md` yet we are
at exactly the limit of 15 things to consider."*

**Nothing is wrong with the queue. The counter is wrong, and it is this lens's error.**

---

## The state, plainly

`docs/notes/proposals.md` → Open is genuinely empty. `P-123` has landed. **Sean's review queue is
zero, and that is accurate.**

The fifteen were never his. Thirteen were addressed to the code lane - palette duplication, dead
code, crate boundaries - and two to the specification lane. **None of them was anything Sean had to
decide.** He was told "15 open" because this lens reported the aggregate in every summary, which is
precisely the noise `Q-31` exists to remove. That habit stops here.

## What I got wrong

`CLAUDE.md:317`, the original rule:

> Keep the open-proposal queue under fifteen. Past that, **reviewing costs as much as writing** and
> the mechanism has failed.

The justification is *Sean's reading time*. The subject is *proposals* - things he reviews.

`CLAUDE.md:135`, this lens's generalisation, now pasted into the same file:

> Keep the open items under fifteen across every outbox together, not fifteen each.

**I kept the number and changed what it counts.** Thirteen crate-hygiene findings cost the code
lane, not Sean; measuring them against a limit justified by his attention is a category error. And
the two sentences now sit in one file, both saying fifteen, counting different things - a
contradiction of exactly the kind this lens is meant to catch, introduced by this lens.

There are two budgets and they need separating:

| Budget                     | Who pays      | Natural limit                            |
| -------------------------- | ------------- | ---------------------------------------- |
| Proposals awaiting review  | Sean          | fifteen, for the stated reason. Today: 0 |
| A producer's open findings | that producer | its own capacity. Not fifteen borrowed   |

That is `Q-32`, to the specification lane: delete `:135`, leave `:317` alone.

## A second error, smaller

The status vocabulary had no terminal value for **noted and deliberately not**. The five values were
`open`, `acted`, `rejected`, `withdrawn`, `answered` - and a finding recorded *so it is not
re-found*, which nobody should act on, is terminal rather than open. With nowhere else to sit, `Q-9`
and `Q-12` sat as `open` and were reported as outstanding work nobody intends to do.

They are now `noted`. `tools/outbox` needed no change: it treats anything but `open` as not
outstanding, and its own doc comment already says *"`open`, `acted`, `rejected`, `withdrawn`,
`answered`, or likewise."*

That alone takes the count from 15 to 13, none of which was ever Sean's.

<a id="one-place-to-look"></a>

## One place to look

Sean: *"As a human, I need one place to go to figure out what is currently pending."*

Agreed, and the earlier answer - *run `outbox --to sean`* - was the wrong shape for the same reason
the aggregate count was: **it asks a person to remember a command instead of giving them a document
to open.** Every hand-held habit in this repository has rotted; every generated artifact has held.

`tools/outbox` already computes exactly what is pending. It should **write** it as well as print it:
one file, regenerated, never hand-maintained, with Sean's queue at the top and each producer's
backlog beneath it. Then one open document answers both *what must I decide* and *what is
outstanding anywhere*, and neither question needs a command.

`hooks/pre-commit` already runs `pad-tables` over staged markdown and re-stages the result. This is
the same shape - a tool rewriting a generated artifact from the source of truth - so wiring it in
beside that is what keeps it from going stale. That is `Q-33`, to the code lane.

What it must not become is a second place that has to agree with the first. It is generated **from**
the outboxes, so it cannot disagree with them; the moment anyone edits it by hand it is a copy, and
this lens has already retired one of those today.

## What this means for the queue Sean reads

Nothing changes about it, which is the point. It stays the one thing he acts on, it is empty right
now, and under `Q-31` everything that needs him arrives there as a numbered proposal. The generated
document tells him what is pending; the queue tells him what is *his*.
