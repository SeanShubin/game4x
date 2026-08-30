# Who reads a report

**Derived.** Written by the quality instance on 2026-08-29. Not binding - an observation about the
process, not a decision about it.

[Quality](README.md) · [CLAUDE.md](../CLAUDE.md) · [The proposal queue](../docs/notes/proposals.md)

Read at `011d284`. One finding and one question, both about routing rather than about code.

Sean asked whether both the specification lane and the code lane need to check a quality report.
They do, they both have, and nothing written down says so - which is why a known contradiction is
currently sitting outside the queue that exists to hold it.

---

<a id="1"></a>

## 1. A report reaches the code lane by instruction and the specification lane by luck

**Where.** [`CLAUDE.md`](../CLAUDE.md) → Three instances; [`quality/README.md`](README.md) line 4.

**What.** `quality/README.md` says who acts:

> Sean decides what is acted on, and **the code instance acts.**

Singular. `CLAUDE.md` names `quality/` twice - once in the lane table, once pointing at this
directory's brief - and never tells the documentation lane to read what is in it.

Both statements are in tension with what the same brief asks quality to look for, forty lines later:

> **A contradiction with the specification is the highest-value finding**, because neither of the
> other two lanes is looking for it.

The code instance cannot act on a contradiction with the specification. `spec/` is not in its
column. So the brief directs this lane to hunt for exactly the findings its named reader is
forbidden to fix.

**Why.** It is not theoretical, and it is not that the specification lane is inattentive - it plainly
reads reports when it runs. `a1cc5e0` opens *"The first quality report put two questions to this
lane"* and answered both, and P-95 came out of it. The practice is right. The instruction is
missing, so the practice depends on somebody remembering.

Three reports in, here is what has actually happened to the findings that were not the code lane's:

| Finding                                           | Column        | Outcome                                     |
| ------------------------------------------------- | ------------- | ------------------------------------------- |
| Q1, what *"where there is a pointer"* binds       | specification | Answered, `a1cc5e0`, and P-95               |
| Q2, *"`/new` changes no game state"*              | specification | Answered, `a1cc5e0`, and P-95               |
| Report 1, finding 2, the digit keys named nowhere | specification | Fixed, `a1cc5e0`                            |
| Report 3, finding 9, the stale crate table        | specification | Not yet picked up                           |
| Report 3, Q3, the three-way biome tension         | specification | Not yet picked up, and **not in the queue** |

The last two are eight hours old against a lane whose last commit predates the report, so this is
not a claim that anything was ignored. It is a claim that nothing would notice if it were.

**Whether.** **Worth fixing**, and it is one line in `CLAUDE.md` - which is the documentation lane's
file, so this report says so and stops. The shape that matches what already happens: *a quality
report is addressed to both lanes; each acts on the findings in its own column and leaves the
others.* Adding the specification lane to the reader list costs nothing, because it is already
reading.

There is a second, better option worth naming since it removes the routing question rather than
answering it: **quality tags every finding with the column that owns it.** Reports have done this
informally - report 3's finding 9 says *"Noted - documentation lane"* and its Q3 is headed *a
question for the documentation lane* - but as prose, not as a field a reader can scan. Making it the
fourth column of the summary table, beside **Whether**, would let either lane read one table and see
its own work.

---

<a id="2"></a>

## 2. A contradiction can sit outside the queue, which the queue promises cannot happen

**Where.** [`CLAUDE.md`](../CLAUDE.md) → An empty queue means no contradictions;
[`docs/notes/proposals.md`](../docs/notes/proposals.md).

**What.** `CLAUDE.md` makes a promise:

> **If the queue is empty, Sean can take it that nothing known is inconsistent.** ... a contradiction
> has exactly one resting place. When Claude finds one - between two spec files, between the spec and
> a release, between a note and either - it becomes a proposal **the moment it is found**, whatever
> else is happening. Never a paragraph in a discussion, never a sentence in a reply, **never a line
> in a note.**

[Q3 of the previous report](2026-08-29-coupling-under-the-game.md#q3) is a contradiction between two
lines of `spec/planet.md`:

- *A territory's biome is what the terrain gives it.* (P-100)
- *Oceans never isolate land from land.* (P-109)

Both are in the specification; both were promoted on 2026-08-28; and no implementation can honour
both for a territory that has to be drained. The code lane found it independently and framed it
better than I did, so two of the three lanes now know.

It is not in `docs/notes/proposals.md`. That file lists P-100 and P-109 as separate landed
proposals and records nothing about their collision.

**Why.** This is precisely the failure the promise was written to prevent - the queue is quiet about
something two lanes know is wrong. The mechanism is not carelessness by anybody; it is structural:

- Quality found it. Quality writes `quality/`, **and nothing else** - it cannot file a proposal.
- The code lane could file one, but a spec contradiction is not its column either.
- The documentation lane can file one, and has not read the report yet.

So a contradiction has three resting places available and reached none of them. `CLAUDE.md` was
written when there were two lanes, and its sentence *"whatever else is happening"* assumed the
finder could always act. With a lane that reads everything and writes one directory, that is no
longer true.

Worth being explicit about the cost, because it is exactly the one the promise names: **the queue is
currently empty of this, and it should not be.** A reader trusting the promise today would conclude
nothing known is inconsistent, and would be wrong.

**Whether.** **Worth deciding**, and it is `CLAUDE.md`'s to settle. The two answers I can see, and
neither is mine to choose:

- **A quality report is a resting place too.** Extend the promise: a contradiction is in the queue
  *or* in an open quality finding, and both must be empty for the guarantee to hold. Cheapest, and
  it makes `quality/` load-bearing in a way the brief currently does not.
- **Quality may file proposals.** Widen this lane's column to `quality/` plus appending rows to
  `docs/notes/proposals.md`. Stronger, because it keeps one queue, and it is a real crack in the
  write-one-directory rule that makes the lanes easy to reason about.

Until one is chosen, the practical mitigation is the one this report is: **whenever a report contains
a specification contradiction, say so on the first screen**, so that the lane which can queue it sees
it without reading to the end. [Report 3](2026-08-29-coupling-under-the-game.md) put Q3 last.

---

## What this is not

Not a complaint about either lane. Both have acted on every report promptly and in their own column,
and the specification lane's answer to Q1 - deleting the ambiguous sentence rather than choosing
between the two readings I offered - was better than either option it was given.

It is an observation that the routing which makes that work is unwritten, and that the one guarantee
`CLAUDE.md` offers Sean about the queue has a gap a third lane opened.
