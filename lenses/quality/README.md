# Quality

**Derived.** Written by the quality instance. Not binding - a report is an observation about the
code, not a decision about it. Sean decides what is acted on, and the code instance acts.

[Root README](../../README.md) · [Architecture](../../docs/architecture.md) · [Specification](../../spec/README.md)

Code quality reports. This directory is the quality lane's only writable place, and no other lane
writes here - see [the lane table](../../CLAUDE.md#perspectives).

## The rule that makes a report worth reading

**Quality never edits what it reviews.** It reads the tree, runs read-only tools, and writes here.
It does not fix the thing it found, does not reformat, and does not run `cargo fmt`, `cargo fix` or
`clippy --fix` - a review that alters its subject is no longer a review, and the next report would
be measuring its own last one.

The consequence worth stating plainly: **a report is only useful if someone acts on it.** A finding
that is true, well-argued and never acted on is indistinguishable from one that was never written.
So a report names what to do, not merely what is wrong.

## What this lens is for, and what bounds it

[`docs/process.md`](../../docs/process.md) is Sean's own statement and is authoritative; this file is
detail beneath it and must not contradict it.

**What this lens is for, what bounds it, and how it tells a dependency that provides a home from one
that provides operations are all in `docs/process.md` - under
[Quality instance](../../docs/process.md#quality-instance-a-type-of-research-instance),
[Research instances](../../docs/process.md#research-instances) and
[Dependencies](../../docs/process.md#dependencies). None of it is restated here.**

Not restated because restating it is what failed. This file said the Quality instance section gave
three jobs; it was right when written and wrong four days later, and the four it lost included both
of the ones that aim this lens at structure and at the pipeline. **Listing them again more
faithfully would have been the same mistake with a longer half-life** - the first copy was close to
the source's words too, and closeness is not what kept it current. `docs/README.md` now carries the
rule: a document that restates another links to it instead of listing it.

### What is not in `docs/process.md`, and is this lens's own

**Two findings, as worked examples of the dependency test rather than a copy of it.** Both were
reached the long way, by counting what a prototype linked, and both fall straight out of asking
whether the dependency appears in this project's own types:

- `Q-2` - `Biome` belonged to `game-model`, and appeared in `planet_terrain::Sample`'s surface. A
  fact about the game had become a fact about the terrain crate's types.
- `Q-3` - Bevy is a home, and it had two crates to have opinions in rather than one.

## The outbox

**[`outbox.md`](outbox.md) is the only file another perspective has to read.** A report carries the
argument; the outbox carries the claim, its reader and its state. Each item has four things:

| Field      | What it is                                                                                                                                                |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **id**     | `Q-n`, stable, so a commit can cite it and a later report can say what became of it                                                                       |
| **to**     | `sean`, `spec`, `code` - or absent, meaning *not ready, no reader*                                                                                        |
| **status** | `open`, `noted`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` is outstanding; `noted` is *recorded so it is not re-found*, which is terminal |
| one line   | what it is, so a reader can triage without opening the report                                                                                             |

**Research that is not ready is addressed to nobody.** It lives in a dated report, costs no one any
attention, and becomes visible the moment this file gives it a reader. Nothing is held back by
discipline; it is held back by not yet having one.

**A producer's backlog is not Sean's queue.** Items here are addressed to a producer and cost that
producer, not Sean. `CLAUDE.md`'s limit of fifteen is on the *proposal* queue, justified by his
reading time; counting a lens's findings against it was this lens's error and is
[corrected](2026-08-30-two-budgets.md). **This file's length is not a number to report to Sean.**

**If nothing in the outbox is `open`, this lens knows of nothing outstanding.** That is a promise
about the file, not about the tree.

**A producer may decline a finding, and often should.** It says so in the commit that declines it,
citing the id, and the outbox records it. `Q-16` was wrong and was withdrawn after the code lane
refused it - and the refutation produced better output than the finding had. **Check a rejection
before defending it.**

## What a report says

Every finding carries four things, because a finding missing any of them cannot be acted on without
going back to whoever wrote it:

|             |                                                                          |
| ----------- | ------------------------------------------------------------------------ |
| **Where**   | file and line, so it can be found without searching                      |
| **What**    | the defect, in one sentence                                              |
| **Why**     | what it costs - a bug, a trap for the next reader, a rule it breaks      |
| **Whether** | worth fixing now, worth fixing eventually, or noted and deliberately not |

That last column is the one that keeps a report from becoming a wish list. **Most findings should be
"noted and not".** A report where everything matters is a report where nothing does.

## A pattern is a claim about the bytes as they are now

Two failures that look alike and are not, both of which cost this lens a wrong conclusion this
session.

**All of these are one shape, named by the code lane and confirmed across three lanes: a proxy
standing in for the thing, passing silently.** The two families below differ in their remedy, which
is why they stay apart, but the parent is worth having because it is what makes them recognisable
before they are diagnosed.

Instances, one per lane, within a week: a **count** read as evidence about behaviour - one `move`
line taken to mean the recipe fired; a **directory** read as the population - `crates/*/tests/`
searched when the assertion was in `src/`, twice, five times in variants; and a **filtered tool
output** read as the bytes - `git show` and `grep` both applying a line-ending filter, so a phantom
CRLF problem survived four tool calls until `git cat-file blob` read the bytes themselves.

**The tell is the same every time: the instrument answers a narrower question than the one asked,
and returns a plausible number rather than an error.** Nothing distinguishes it from the true answer
except going and getting the thing.

**A match string is a claim about the current bytes.** A table row written before the padder ran, a
grep for `fn window()` against a signature that had since gained an argument. The remedy is to
locate by prefix and rebuild rather than match, and to read the file at the moment of matching
rather than earlier.

**A command answers the question it was given, not the one in mind.** `cargo tree --depth 1` read as
the tree; `git diff` read as HEAD against the working tree when it compares the index. The remedy is
different: check what the command scopes or compares *before* drawing anything from it.

Its sharpest form, because it reads as evidence and is not: **a count of zero says nothing unless
the population is non-empty.** `grep -c "into" docs/notes/proposals.md` returned 0 and this lens
reported the field did not exist. The field exists on every open proposal; the *queue* was empty, and
promotion deletes the field with the body. The number was right, the denominator was zero, and the
conclusion happened to survive for a reason other than the one given.

The code lane grouped all four as trusting a tool's output. That is right about the cause and blurs
the remedy - the first family is fixed by when you read, the second by knowing what you ran.

## Cite the commit, rest the claim on the file

**A closure here names a commit. The claim rests on file state, and must keep resting there.**
`C-4`: the index is shared between three sessions, so `git commit` publishes whatever is staged
rather than what the caller changed. A commit can carry work it does not mention.

It has happened once, to this lens. `93d839d` is titled *finding: Q-8 acted* and carries twenty-six
lines of `hooks/pre-commit` - the code lane's `Q-36` fix, staged when their commit lost a race for
the index lock that this one won. Neither party did anything wrong.

The verification method survives, because it reads files and runs tests rather than diffs. What is
weaker than it looks is the **citation**: a pointer for a reader, not evidence. Where the two could
differ, say which one the claim rests on.

## Commit by pathspec, not by staging

**`git commit -- <paths>` rather than `git add` then `git commit`.** The index is shared between
three sessions, so staging is a publish to a shared buffer and a commit takes whatever is in it.
Committing by pathspec takes only the named files and leaves the rest of the index alone.

It closes the window in the one direction this lens controls - it does not stop another session
committing while this one's files are staged. `C-4` is the general problem and it is the
specification lane's.

**It has a residue: unstage it.** `hooks/pre-commit` stages `pending.md`, and a pathspec commit does
not take the staged copy, so a stale one is left behind in the shared index for the next
perspective's commit to publish. `git reset pending.md` afterwards, having checked it matches what
was committed. Using the workaround without this feeds the problem it works around.

Live instance, 2026-08-30: an empty `docs/process.md` sat staged in the index while this lens was
closing four findings. It belongs to the specification lane, was not committed here, and was left
alone rather than unstaged - another perspective may be mid-operation on it.

## Probe against a copy, not the shared tree

Verifying the `Q-36` fix needed a commit, and this lens made one by creating `scratch-probe.txt` at
the repository root - a new file, outside its column, in a working tree three sessions were
committing to. Reverted in `2804f83` rather than rewritten, because rewriting shared history is
worse than an honest revert.

**A probe uses a file this lens already owns, or a clone.** This lens had been reporting for two
days that a boundary erodes by crossings too small to stop for, and then made one in order to test
a fix for another one.

## Poison the thing the check reads

A poison that lands somewhere the check never looks is **inert**, and an inert poison produces a
green run that reads exactly like a check working. So before believing a poison test, say what the
check reads and put the poison there.

Two cases in one day, both the same shape and neither an error at the time. The code lane poisoned
the working tree to test a promotion checker that reads `git show <commit>:<file>` - history, not the
tree - and came close to reading the green as evidence. This lens ran `cargo test --quiet <filter>`
against a poisoned copy and got **`ok. 0 passed`** three times over, because the filter matched no
test in those targets; *ok* over an empty set is the same bytes as *ok*.

**So a poison test carries the same burden as any other check: name the population it acted on.**
*The poison changed something the check reads, and the check went red* is evidence. *I changed
something and it stayed green* is not, until the first half is established.

This is `C-28` turned on the instrument used to verify instruments, which is why nothing catches it:
the poison is the last thing in the chain, and there is nothing behind it to check it.

## A green suite under a poison bounds the tests, not the code

**`Q-58`, 2026-09-06.** This lens found a `saturating_sub` whose comment gave density zero as its
reason, observed that no case in the tree has one, and wrote: *changing it to `density - 1` leaves
every test green.* True, and the code lane confirmed it. The item then called the finding **small**
on the strength of it.

**That sentence measured the coverage and was read as measuring the risk.** The code lane made the
change and ran the program rather than the suite: `Territory::empty` carries no deposits, `create
planet` makes twelve of them before `set resource` fills any in, and the reachable path panics with
*attempt to subtract with overflow*. The branch was load-bearing and the silence was the tests'
silence, not the code's.

**So it is the poison rule with the sign flipped.** That rule says a poison landing where the check
never looks produces a green run that reads like a check working. This is the same green read as
*low stakes* instead of *low coverage* - and the remedy is the same one: name the population the
poison acted on. *Every test stays green* names the test suite. It says nothing about the program.

**The finding survived; the `whether` did not.** Their `C-38`, and the check exists because the item
was filed, which is the part worth keeping.

## Report the claim, not the line

**`Q-66`, 2026-09-06.** This lens found a comment asserting that a stored trait was derived, and
filed it as `expected_state.rs:77`. **The same claim was in three places, and a fourth doc seeded
them.** The code lane grepped for the sentence rather than fixing the line reported, and their
commit says plainly that grepping is the only reason the other two are not still there.

**Their correction, which is the sharper form: one imprecise sentence seeded three false copies, and
the imprecise one was the least wrong of the four.** `Capacity`'s struct doc said *`used` is
derived* - true, and silent about `total` - and the three that were flatly false read `capacity is
derived` downstream of it. **So the copies were not independent mistakes.** That is the better
reason grepping beats fixing a location: the same true-as-far-as-it-goes sentence is what a reader
generalises from, and it generalises wrong every time.

**A finding reported as a location gets fixed as a location.** That is not the producer being
careless - it is the report answering a narrower question than the one it was asked, one step
earlier than usual: not the instrument narrow, but the *finding* narrow, and the fix inherits the
width of the finding.

**And it fails silently, which is what makes it process rather than craft.** Three surviving copies
of a refuted claim read exactly like none, because the item says `acted` and the line it named is
correct.

**So a finding says what is wrong before it says where**, and where there is a sentence to grep for,
this lens greps before filing rather than leaving that to whoever acts on it.

## When the instrument is confidence, make it produce something

The general form of the rule above, and the code lane's sentence rather than this lens's: **the
instrument was a person's confidence, and the fix was the same each time - make it produce
something.**

Three cases in one week, none of them a defect in an artifact and all of them defects in a reader:

- **A check believed less than it deserved.** `a_promotion_lands_what_was_approved` had only ever
  passed, over a list of known exceptions, and the lane that owned it had begun reading it as a check
  about its own exception list. It caught `P-257`. Their `C-33`.
- **A population believed more than it deserved.** `C-34` listed four writable errors and two of them
  could not be written. Nobody could tell until somebody tried. `Q-55`.
- **A poison believed without asking what it acted on.** Above.

**What the three have in common is that the thing to ask for is an artifact, not an argument.** A
poison that goes red, an exhibit that compiles, a count against a named population. Confidence
produces none of those and reads exactly like all of them.

## Read it before writing about it

**Quote the artifact's own words for what it is, before arguing about what it is for.** Not a
resolution to read first - a step that cannot be completed without the reading.

This lens argued that a prototype needs the instrument its question requires, and sent it as a
distinction. `prototypes/goldberg-view/README.md` had said it already, and so had the first
paragraph of its `main.rs`, which this lens had read and quoted from earlier the same day. The code
lane had the reasoning and was asking permission.

The specification lane did the same thing on the same file and was caught only by an unrelated rule
against inventing text: it needed the prototype's question in its own words for an index row. **The
check that worked was mechanical, and the one that would have relied on judgment did not exist.**

The general shape, of which this is one instance: *asserting without checking* and *arguing without
reading* are the same error at different stages - one skips verification after the claim, the other
before it. Both are cheaper than the alternative and both produce confident prose. What beats them
is not care. It is a step that cannot be completed without the reading, which is why the quotation
guard in `game-console` works.

## A self-check may share inputs; it may not share the computation

When a report is checked against itself, the question is whether the check is real or decoration.
This lens first put the line in the wrong place - *the derivation must come from somewhere the
printer does not* - which forbids too much. The specification lane sharpened it and was right:

**Sharing the inputs is fine. Sharing the computation is what makes it circular.**

Two derivations reading the same recorded states and disagreeing is evidence. Two derivations
produced by one code path agreeing proves only that the path agrees with itself.

The distinction earned itself the same day. The code lane declined a delta check on this lens's
over-broad caution, was pushed back on, built it against the printed artifact, and **it found a real
bug on its first run**: `expected::compare` paired rows by table and first column, so two extractors
built in one territory read as one row changing, and turn 2's delta claimed `node: 3 → 2` - true of
nothing that happened. That was the artifact Sean was about to check the state function against.

**A caution drawn too wide costs a check that would have worked.** That is a different failure from
a caution drawn too narrow, and this lens had not weighed it.

## Re-read the source, not the summary

This file said `docs/process.md` *gives this lens three jobs* and listed three. **It was right on
2026-08-30 and wrong by 2026-09-05**, because the section grew to seven and nothing connected the
two. Four jobs were missing, including the two that aim this lens at structure and at the pipeline
rather than at prose - so the omission was not neutral about what got looked at.

**Staleness, not a miscount**, and the difference matters: no amount of care at writing time would
have prevented it. It is a countable claim about another file, made once and never re-checked -
which is the same shape as *the word bin appears nowhere*, and the shape `tools/spec`'s
`check_claims` was built for.

Nothing checks this lens's claims about `docs/process.md`, and the answer turned out not to be a
checker. **The answer is to stop copying**: the section is linked above rather than listed, so there
is no second copy to go stale and nothing to check.

That is `P-245`, filed by the specification lane from a decision this lens left in a reply - which
`docs/process.md` forbids, because a decision in a reply is invisible to `pending.md` and gone when
the session ends. **The rule this lens quoted at another lane, broken by this lens, three days
later.**

Worth keeping as the sharpest instance available: **having just been burned by a stale copy, this
file's first correction was to make a better copy** - seven bullets instead of three, closer to the
source's words. One commit later, linking replaced them.

## Say it and stop

**State the question, the facts, and what to do. Then stop.** Sean read a sixty-line proposal that
said a generated file may live in the repository root and called it esoteric and vacuous. He was
right, and the specification lane has written a rule against itself about it.

This lens shares the failure mode and should say so. A report is long when the argument is long, not
when the finding is small dressed up. Two tests before filing:

- **Would the finding survive being cut to its Where, What, Why and Whether?** If the rest is
  reasoning nobody has to follow to act, it belongs in a dated report or nowhere.
- **Is a sentence carrying its weight, or is it a phrase that sounds settled?** An aphorism that
  compresses a real finding earns its place. One that decorates a small one costs a reader's trust
  in every other line.

It is the same idea as competing on the value of a finding rather than the count, one step out: a
lens spends a reader's attention by the word as well as by the item.

## What is in scope

The tree, its structure, and whether the code says what the specification says. Concretely: whether
[architecture's rules](../../docs/architecture.md#rules) hold, whether crate boundaries are real,
whether tests assert what they claim, whether names mean one thing, and whether anything in
`crates/` contradicts anything in `spec/`.

**A contradiction with the specification is the highest-value finding**, because neither of the other
two lanes is looking for it: the code instance reads the spec as instructions and the documentation
instance does not read the code.

## What is not

Style the formatter already settles, preferences with no argument behind them, and anything that
would be a design decision rather than an observation. **When a report finds that the specification
itself is wrong or unclear, it says so and stops** - that becomes a proposal in
[the documentation lane](../../docs/notes/proposals.md), not a change here.

## Naming

One file per report, dated: `2026-08-28-crate-boundaries.md`. Reports are records of a moment and go
stale like any note - a superseded one says so at the top rather than being deleted, so a later
reader can tell whether a finding was fixed or merely forgotten.

## Reports

Newest first.

- [Review of `ba9bd41..217dcba`](2026-09-06-review-of-the-six.md)
  - 2026-09-06. Requested by the code lane. The `Q-47` check cannot see the spelling `Q-47`
    was filed about, `phase` declares no values so the forbidden count is eighteen, and two
    corrections to this lens's own open items.
- [One notation, two readers](2026-09-06-one-notation-two-readers.md)
  - 2026-09-06. Sean asking about duplication and parsing isolation. The isolation is
    clean; the tokenizer is written twice, and the two readers already disagree about
    where a comment may start.
- [Review of the map form](2026-09-06-review-of-the-map-form.md)
  - 2026-09-06. `S-47` landed and `Q-64` closed - the forbidden vocabulary went from
    nineteen words to two. One false reason sitting next to the assertion it explains, and
    a probe finding no silent drop in `containment::tree`.
- [Sweep of `ba9bd41..f3dcc1e`](2026-09-06-sweep.md)
  - 2026-09-06. Sean's whole-day sweep. The data file moved away from `P-284` rather than
    toward it, `S-55` holds under poison on all three points the code lane raised, and one
    check is named for a claim it does not make.
- [What `S-47` will need looking at](2026-09-06-what-s-47-will-need-looking-at.md)
  - 2026-09-06. Held, addressed to nobody. Why there is no sweep yet, and the four things the
    code lane asked for a second pair of eyes on before it had built them.
- [Sweep, 2026-09-05](2026-09-05-sweep.md)
  - Three findings, all acted the same evening, and the measurements that found nothing -
    a baseline to re-measure against rather than a memory of having looked.
- [What changed was not the rate](2026-09-01-what-changed-was-not-the-rate.md)
  - 2026-09-01. Invited by the specification lane. The defect rate tracks operations rather
    than promotions, and nothing checks shipped text against approved text.
- [Two budgets, counted as one — a correction](2026-08-30-two-budgets.md)
  - 2026-08-30. Why the queue is empty and the counter said fifteen. This lens's error,
    and the one generated document that should replace a remembered command.
- [Who checks the specification is buildable, and where Sean looks](2026-08-30-readiness-and-one-surface.md)
  - 2026-08-30. Two recommendations, both against adding machinery: no readiness lens, and
    no `to sean` address.
- [The workflow, ready to paste](2026-08-30-workflow-to-adopt.md) — **spent**
  - 2026-08-30. The `CLAUDE.md` section and the index tool, written out verbatim for the
    two lanes that own the files they go in.
- [A recommended workflow](2026-08-29-workflow.md)
  - 2026-08-29. **Input to a decision, not a finding.** Seven things move between the
    perspectives, six have a channel, and a blocked question from code has none.
- [Notes on lenses, from the one that exists](2026-08-29-lenses.md)
  - 2026-08-29. **Input to a decision, not a finding.** What three reports suggest about
    several research lenses, and what has to change before there is a second one.
- [Who reads a report](2026-08-29-who-reads-a-report.md)
  - 2026-08-29. Both lanes must read a report, nothing says so, and a known spec
    contradiction is sitting outside the queue that promises to hold it.
- [What the new prototype exposed, and what it did not](2026-08-29-coupling-under-the-game.md)
  - 2026-08-29. `Biome` in the game pulls terrain and rendering up into it, the picture and the
    model disagree about a territory's biome, and a detached globe still links the whole game.
    Finding 1 is withdrawn after the code lane's reply; the correction is in the report.
- [What the response to the first report left behind](2026-08-28-response-to-the-first-report.md)
  - 2026-08-28. Five findings closed and verified; three quotations the P-95 sweep missed.
- [Crate boundaries, duplication, and where Bevy has spread](2026-08-28-crate-boundaries-and-duplication.md)
  - 2026-08-28. Whether Bevy is confined to the adapter, what is duplicated, and four places the
    code and the specification disagree. Findings 1, 2, 3, 4 and 13 are closed; the rest stand.
