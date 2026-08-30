# Who checks the specification is buildable, and where Sean looks

**Derived.** Written by the quality lens on 2026-08-30, answering `Q-15` and a concern Sean raised
about his own inbox. **Input to two decisions, not observations about the tree.**

[Quality](README.md) · [The outbox](outbox.md) · [Why the question exists](2026-08-29-workflow.md)

Two recommendations, both against adding machinery.

---

## 1. `Q-15`: expand the specification lane, do not add a lens

I proposed the readiness lens. Having looked at what it would actually do, **I recommend against
it.** Half its job is already a test, and the other half is a missing check rather than a missing
perspective.

### Half of it is already mechanised

The one readiness pass this repository has done, `docs/notes/first-release-readiness.md`, states its
method exactly:

> **Method: try to write that script.** Every place it cannot be written is a gap, and nothing else
> counts as one.

It found four real gaps - no design vocabulary, no win condition, an unsettled spent flag, missing
commands - and all four were closed.

**That method is now a test.** `crates/game-console/tests/first_release.rs` runs the entire release
script through the real console, and reads `releases/first-release.md` rather than restating it, so
the two cannot drift. A gap that stops the script from being written stops the test from passing.
The standing executability check exists, runs in the gate, and needs no perspective to operate it.

That is the part that genuinely needed a builder's eye, and it is the part already done.

### The other half is a check the protocol structurally cannot make

`P-100` (*a territory's biome is what the terrain gives it*) and `P-109` (*oceans never isolate land
from land*) both landed on **2026-08-28**, in **`spec/planet.md` → What a territory carries** - the
same file, the same section, the same day. They cannot both hold. It took until 2026-08-30 to
notice, and it took reading the code to do it.

The natural conclusion is that the specification lane has a blind spot about its own work. I think
that is the wrong diagnosis, and the right one changes the answer.

`CLAUDE.md` already requires a staleness check at promotion:

> **A promotion that makes something else stale is not finished.** When landing a proposal
> invalidates a line elsewhere … refuse the promotion, or file a cleanup proposal immediately after.

That question is **directional and per item**: *does this invalidate something?* A contradiction is
**symmetric and between items**. Neither `P-100` nor `P-109` invalidates the other - each is fine,
and only the pair is unsatisfiable. So the protocol was followed correctly and could not have caught
it. **The lane was not careless; the check was the wrong shape.**

A check of the wrong shape is not fixed by adding a perspective. It is fixed by adding the missing
question.

### Make the trigger mechanical and the check human

The duty *re-read the whole section after promoting into it* is exactly the kind of thing that rots
here - thirteen proposal rows, three specification index files, eight of eleven quotations twice,
and this lens's own outbox stale within the hour of writing the rule against it. Every hand-held
duty in this repository has rotted; every mechanical one has held.

So do not add a duty. **Add a trigger**, and let it prompt the human check:

> **When more than one proposal lands in the same file and section, re-read that whole section
> asking one question: can all of these hold at once?**

`tools/outbox` already parses `docs/notes/proposals.md`, including the destination and the date, so
it can emit the flag for free. Tested against history, it would have fired loudly on the day it
mattered:

| Landed     | Destination                                   | Count |
| ---------- | --------------------------------------------- | ----- |
| 2026-08-28 | `spec/planet.md` → Presentation               | 6     |
| 2026-08-28 | `spec/invariants.md` → Control without tedium | 5     |
| 2026-08-28 | `spec/planet.md` → What a territory carries   | **4** |
| 2026-08-28 | `releases/first-release.md` → Capabilities    | 3     |

The third row is where the contradiction was, and it is the third-loudest signal of that day. A
person told *four rules landed in this section today, read it whole* would have found it.

### Why not a lens, plainly

- The executability half is a test that already runs. A lens would re-do it by hand.
- What remains is fixed in `spec/`, so a lens adds a hop and every finding still becomes a proposal
  Sean reviews. The hop buys nothing.
- The open count is 14 against a limit of 15. A lens's first act is to compete for that budget.
- A lens is justified by producing findings neither producer would. Here the producer *can*
  produce them once asked the right question - which is a cheaper fix than a new perspective.

**Reconsider a lens if the trigger fires and the pass still misses something.** That is a real
possibility and it would be the evidence this recommendation lacks.

---

## 2. Sean's inbox should be the queue he already reads

> *"The idea of me having to run `.\scripts\outbox.ps1 --to sean` seems a bit vulnerable to human
> error, but I seem to be able to manage reading the proposals just fine."*

That is right, and it is the same argument as everything above: **a habit that has to be remembered
is a habit that rots.** Reading the proposal queue works because it is one document already open at
the start of a review, not because it is well disciplined.

So the recommendation is to **remove the `to sean` address entirely.**

### Where the traffic goes instead

There are only two kinds of thing addressed to Sean, and the queue already handles both:

- **A decision a lens surfaced.** That is a proposal in waiting. It goes `to spec`, and the
  specification lane's existing job - step 4 of the cycle - is to turn it into a numbered proposal.
  This has already been done once and worked exactly right: `Q-17` became `P-123` and reached the
  queue without Sean querying anything.
- **A question that blocks the code lane.** Almost always *the specification does not say X*, which
  is a proposal. It goes `to spec` too, with the code lane's stated assumption attached, so the
  queue row carries both the question and what was done in the meantime.

Then `tools/outbox` is a thing the **instances** run to check each other, and Sean's surface stays
one document. The `--to sean` invocation stops existing rather than being remembered.

`Q-15` itself is the demonstration: it is currently `to sean`, and it should have been `to spec` -
a process question whose answer belongs in `CLAUDE.md`, which is the specification lane's column.
Addressed that way, it would have reached Sean through the queue he already reads instead of waiting
for him to ask about it.

### What this costs

One thing, and it is worth stating rather than glossing. Routing everything through the queue means
the specification lane decides what reaches Sean. It is the lane best placed to do it - and it is
still a filter that did not exist before. The mitigation is the one already in the protocol:
**a proposal records the reason for a rejection**, so a lens finding that never becomes a proposal
leaves a trace rather than vanishing.
