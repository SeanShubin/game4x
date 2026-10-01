# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-596 - All five prototypes link main code, and rule 18 cannot be kept by replicating

**to** sean · **status** open · **raised** 2026-09-30 · **asks** a decision · **kind** recovered · **shape** text · **into** `docs/architecture.md` -> Rules · **source** `C-191`, against the rule you promoted the same day

**Rule 18 says a prototype replicates rather than links, and *it is almost always a smaller and
modified version*.** The code lane measured what replicating would actually cost, and the second half
of your sentence is what the measurement contradicts.

```
                 own lines    main-code lines linked
gap-view               717                 4,129
goldberg-move          888                 9,023
goldberg-view          181                 9,023
hex-torus-view       1,004                   367
planet-view            450                12,431
```

**Replicating what `planet-view` links copies 12,431 lines into a prototype of 450**, and about 35,000
lines across the five. **That is not a smaller and modified version; it is the mainline with a copy
date.** So rule 18 as written cannot be satisfied by replicating here - which is the rule working, not
failing: it says a prototype must not hold main code in place, and the honest reading of these numbers
is that four of these five are not prototypes any more.

## The choice, and only you can make it

**Delete, or keep and accept the breach.** `docs/prototypes/README.md` says *that answer is the
deliverable; the code is a byproduct*, and **three of the five have their answer recorded** - so
deleting takes the breach, `planet-ecs`, `planet-flat`, `planet-raster` and `C-187`'s 399-line rules
engine with it, and loses nothing written down.

**Against that is your own keep-rule**, stated the same day: *prototypes about experiments I will want
to keep until I have already implemented their results into code.* **`planet-view`'s note records an
open question** - *GPU is an open question, not a decision that has been made* - and `planet-raster` is
the apparatus for answering it. **So the keep-rule protects at least one of the five and the link-rule
condemns it**, and that is the contradiction rather than a cost to weigh.

## What is not being asked

**Not whether to delete `planet-view`.** The two rules disagree about it, so a lane choosing either
would be deciding which of your rules wins. **Three of the five may be a different answer from the
other two**, and which three is a fact the code lane has - the recorded answers - rather than a
judgement.

**And nothing has been deleted.** `C-191` says so plainly: *I have not deleted a prototype on my own
reading of one rule.* **`S-226` is held rather than refused** for the same reason - its resolution
moves 399 lines into a prototype that is itself in breach.
*Nothing is open. Everything filed has been decided.*
