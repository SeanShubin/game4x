# *Did it land* is not *is it still there*

**2026-09-13.** Prompted by the specification lane mentioning `P-486` in passing: it restored a
clause `P-66` promoted and a reword lost on 2026-09-01, twelve days gone, found only because a
number in a recipe could not be explained. **The first promoted idea this repository has found
missing from `spec/`** - and *found* is not *only*. Nobody had asked whether there were others, so
this asks.

## The gap is real and it is by design

`CLAUDE.md` states the guarantee in the present tense: **approved text is byte-identical to shipped
text**. The check that enforces it, `a_promotion_lands_what_was_approved`, answers a different
question, and says so itself:

> This compares against the destination as it stood in the promoting commit, which is the precise
> question at the time - and no later commit can change what a past one contained.

That reasoning is right and the design is right: a deviation caught and repaired would otherwise
stay red forever. **The consequence is that nothing asks whether approved text is still there**, and
`P-66` is twelve days of what that costs.

## The obvious check would be wrong, and here is the number

**78 of 192** checkable promotions no longer match their destination at `HEAD`. That is not 78
defects. It is mostly the specification working.

The first one examined, `P-468`, promoted *how many extractors it has **total** capacity for*. The
file says *capacity*. The commit that changed it is `506ff08`, **Promote P-478: `a stored trait` and
`total capacity` are words nothing means any more.** A later approved proposal renamed the term.

**So *the approved text is gone* is the normal state of any sentence a later promotion edited**, and
a check that reported it would be noise at a 40% rate. This is why the check does not exist, and
measuring it was the only way to know that rather than assume it.

## The narrower instrument, and what it found

The question that separates `P-478` from `P-66` is **what changed it**. So: for each approved text
that no longer matches, find the newest version of the destination that still held it, and ask
whether the very next commit to touch that file promoted anything.

**78 became 4.** All four are explained and none is a defect:

| Item             | Changed by                               | Why it is not a loss                                            |
| ---------------- | ---------------------------------------- | --------------------------------------------------------------- |
| `P-330`          | `cd43864`, recording citation hashes     | the needle is an **addressing line**                            |
| `P-333`, `P-335` | `870b26d`, marking `R-8` and `R-9` built | the same - `**to** code - **status** open` became `**to** sean` |
| `P-241`          | `e642331`, *Normalize before comparing*  | superseded in Sean's own words, quoted in the replacement       |

**Three of the four are one mistake of this lens's**, and `CLAUDE.md` already ruled on it: *an
outbox item's addressing line is not part of what is promoted ... a proposal does not offer it and a
promotion writes it.* The instrument treated it as approved text. **A predicate wider than its
subject**, which is the thing this lens has spent two days finding in other people's checks.

## What the zero does not cover, which is the part that matters

**A claim of zero names what it counted against**, and this one counted against less than half the
ledger.

| Population                                                | Count   |
| --------------------------------------------------------- | ------- |
| Accepted proposals in the ledger                          | **441** |
| Whose text is recoverable from the queue's own history    | 335     |
| Of those, offering verbatim text into a `.md` destination | **192** |
| Not matching at `HEAD`                                    | 78      |
| Survived the *what changed it* filter                     | 4       |
| Defects among them                                        | **0**   |

**The 106 with no recoverable body are not a random sample.** They are every proposal accepted on or
before **2026-08-29**, without exception; from 2026-08-30 every one is recoverable. The queue began
writing proposals as `### P-n` items around that date, and before it their text is not in this
file's history at all.

**`P-66` was accepted 2026-08-26.** So the one known instance of this defect sits inside the blind
spot, and this sweep could not have found it - not before `P-486` repaired it, and not after.
**Zero found over 192 is a statement about the fortnight in which the machinery existed**, and says
nothing about the fortnight in which the loss happened.

**And the instrument cannot see a repaired loss at all**, because it asks whether the text is
missing now. `P-66` matches today.

## What to do

**Build it eventually, not now, and take the noise measurement rather than re-deriving it.** The
shape is known: *approved text whose last-holding version was followed by a non-promoting commit*,
with addressing lines excluded, which is one rule and cuts 78 to 1.

**Whether: eventually.** The era the instrument can see is clean over 192 texts, and the known loss
predates the promotion machinery entirely - which is evidence the process got better rather than
evidence it is failing. **What argues for building it at all is the cost when it does happen**:
twelve days, and found by a number nobody could explain rather than by anything looking.

**What this lens is not claiming.** Not that `spec/` is intact - 249 of 441 accepted proposals were
never checked, and the whole first fortnight is unreadable to this method. **Only that in the part
that can be read, nothing is missing that a later promotion did not deliberately change.**
