# The workflow, ready to paste — **spent**

> **Superseded 2026-08-30. Both blocks have landed and this is a record, not a source.** The live
> text is [`CLAUDE.md`](../../CLAUDE.md) → Perspectives (`ba4850d`) and
> [`tools/outbox`](../../tools/outbox) (`e233186`). The paste blocks that were here have been
> removed rather than kept, because a second copy of a live document is the drift this lens spends
> its time reporting - and this one had already drifted, one commit after landing. See
> [what changed on the way in](#what-changed-on-the-way-in).

**Derived.** Written by the quality lens on 2026-08-30, after Sean accepted
[the recommended workflow](2026-08-29-workflow.md), and retired the same day.

[Quality](README.md) · [The outbox](outbox.md) · [Why](2026-08-29-workflow.md)

## What was proposed, and where it went

| Block                        | Went to                    | Lane          | Item   | Landed    |
| ---------------------------- | -------------------------- | ------------- | ------ | --------- |
| The workflow section         | `CLAUDE.md` → Perspectives | specification | `Q-13` | `ba4850d` |
| The index tool specification | `tools/outbox/`            | code          | `Q-14` | `e233186` |

The lens's own half - [`outbox.md`](outbox.md), and the convention in [the brief](README.md) - was
live already and is unaffected.

<a id="what-changed-on-the-way-in"></a>

## What changed on the way in

Worth recording, because it is the argument for deleting the copy rather than keeping it. The
specification lane did not paste the block; it improved it in three places, and every change is an
improvement this lens would not have made:

- **The write rule binds the producers against each other, not only against the lenses.** The
  original said nobody writes outside their column and left the producer-to-producer case implied.
  The landed text says it: the specification lane does not edit code *even to fix an obvious break*,
  and the code lane does not write specification.
- **The four consequences with teeth were carried across.** Staging by name, the pre-push gate,
  re-reading before asserting - all of which the block dropped by replacing a section that had them.
  Removing a rule by rewriting the section it lived in is a real hazard of a wholesale paste, and
  the lane that owns the file caught it. A fourth was added: **check your own outbox is not stale
  before adding to it**, which is the failure this lens then committed within the hour.
- **Step 5 of the cycle is more careful about what *promote* means.** The block had Sean editing
  proposal text in place; the landed version splits *he says what should change, the lane makes it
  and shows the result, he reads it and says promote* from the case where he writes the words
  himself, and points at the promotion rules for why the distinction matters.

Three edits, one commit after this file was written. Had both copies stayed, the one in this
directory would have been wrong about the process by the end of the day - and it is the copy a lens
reads when it starts up.

## The one link that was broken, and why it is gone

A check of every relative link in this directory after the move found exactly one failure: this file
linked `docs/notes/proposals.md` unprefixed, because inside a paste block the path had to be correct
*relative to `CLAUDE.md` at the root*, not relative to here. It was right for its purpose and wrong
in its location - which is what a document containing a copy of another document does. Removing the
block removes it.

The other 93 links in this directory resolve.
