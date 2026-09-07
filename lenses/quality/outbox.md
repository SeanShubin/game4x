# Quality outbox

**Derived.** The quality lens's one outbox. Every finding it has addressed to somebody, and what
became of it. Not binding - a finding is a claim about the tree, not a decision about it.

[Quality](README.md) · [Reports](README.md#reports) · [The proposal queue](../../docs/notes/proposals.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to code` - a defect or a decision in `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`,
  `hooks/` or CI. The code lane acts; Sean never has to see it.
- `to spec` - something for `spec/`, `releases/` or `docs/`. The specification lane turns it into a
  numbered proposal; **it does not decide it.**
- `to sean` - a question or a decision only Sean can make.
- **Unaddressed** research does not appear here at all. It lives in a dated report and is nobody's
  work until this file gives it a reader.

**Status** is one of `open`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` items are
outstanding.

> **The guarantee.** If nothing here is `open`, this lens knows of nothing outstanding. That is a
> promise about this file, not about the tree - it does not say the code is correct, only that
> everything this lens knows to be wrong is sitting where its reader will find it.

An item may be rejected. A producer that declines one says so in the commit that declines it, citing
the id, and this file records it. **A rejected finding is not a failure of the process** - `Q-16`
was wrong, and being refuted is the lens working.

---

## Open

> **A correction to how this lens estimates cost, 2026-08-30.** `Q-34` told the code lane that
> `tools/outbox` *"already parses `docs/notes/proposals.md` including each proposal's destination and
> date, so the flag costs almost nothing."* **It did not.** The tool read that file and parsed nothing
> from it, because the queue is a table and the tool was deliberately built not to parse tables. The
> work was table parsing plus grouping, not grouping alone.
>
> This is the third time this lens has asserted something without checking it - a specification
> requirement taken from a code comment (`Q-16`), a missing channel that existed the whole time, and
> now a capability that did not. The pattern is specific and worth naming: **claims about state get
> verified here; claims about capability and cost do not.** An estimate handed to a producer is a
> claim about code and earns the same check as any other. Verified before filing from now on, or
> filed without a number.
>
> **The code lane added the part this lens had not seen: an unverified estimate is not only wrong,
> it steers.** *"Already parses, so the flag costs almost nothing"* is what made them expect grouping
> and find table parsing, and they nearly treated the queue's format as a blocker before working out
> the padding distinction. So the cost of a bad estimate is paid in what the receiver goes looking
> for, not only in the figure.
>
> **Checks poison-tested 2026-08-30**, borrowing their method - they twice shipped a guard that could
> not fail and both passed. The link checker over this directory reports a planted broken link; the
> id-uniqueness check reports a planted duplicate; and the flag's *16 sections* - a number this lens
> passed on from the specification lane without deriving - was recomputed here from
> `docs/notes/proposals.md` and is 16. A number repeated is not a number checked.


### Q-9 - Small duplication and dead code, six items

**to** code · **status** noted · **raised** 2026-08-28 · **source**
[report 1, finding 14](2026-08-28-crate-boundaries-and-duplication.md#14)

Noted and deliberately not, unless one is already being touched. Listed so a later report does not
present them as new.

**Re-checked 2026-08-30**, after two days of splitting and moving crates. Five of the six stand;
`render_asset_usages` is still uncalled and now lives in `planet-flat`. One has **grown**: the item
recorded `game4x` writing its own `WindowPlugin` while `planet_bevy::window_plugin` existed, and
`goldberg-view` now writes a third, so the shared helper is used by `planet-view` alone.

Recorded because this lens nearly logged it as resolved on a grep for `fn window()` that could not
match `fn window(asked: &options::Options)` - a pattern written against a signature that had since
gained an argument.

### Q-12 - Two hand-rolled option parsers

**to** code · **status** noted · **raised** 2026-08-29 · **source**
[report 3, finding 8](2026-08-29-coupling-under-the-game.md#8)

Noted and deliberately not. Recorded so a third is noticed as a third.

### Q-57 - `phase` declares no values, so `play` is an eighteenth forbidden word

**to** spec · **status** open · **raised** 2026-09-06 · **source**
[review of `ba9bd41..217dcba`, finding 2](2026-09-06-review-of-the-six.md)

`P-288` landed the `phase` row this morning so that `P-284` would pass on it. Its Values cell reads
*before it starts, or once it has*, which **names neither value** - and `play` and `design` appear
nowhere in `releases/first-release.md` or `spec/turn.md`, while `scenario/expected/play.4x:9` writes
`{game phase:play ...}`.

Classifying all **49** distinct words in that file independently gives `C-37`'s seventeen exactly,
word for word, **plus `play`. The count is eighteen.** Three instruments have now miscounted from
one cause: a trait whose values are described rather than named. `phase` is the only closed-set
trait in the table that neither names its values nor points at a table that does.

**`C-37`'s proposed rule already rejects it** - *a value of a trait that names a closed set* - so
the rule is right and only the count moves. What is needed is a row, and the row is Sean's.

**Also, and not a defect:** `unit` and `place` are **families**, which `P-284` as written does not
admit. The file uses both correctly, so a check built on its literal words would flag them.

**Whether.** Worth a decision now. Both producers believe `phase` is settled -
`docs/notes/proposals.md:122` already writes `{game phase:play}` as the target form.

**Corrected by the code lane 2026-09-06, `52eb1b7`, and the correction is right.** `phase` is **not
the only one**. Of the nine closed-set traits, three point at a table - `kind`, `resource`, `biome` -
three name their values - `ready`, `surplus`, `unpaid` - and **three do not**: `houses` describes the
question and names neither answer, `phase` describes both, and `control` names one value and
describes the other.

**The count of eighteen is unaffected, and I checked why rather than assuming it.** Neither `houses`
nor `control` appears in `scenario/expected/play.4x` or in `dump.rs` - zero occurrences in each, and
neither is among the 49 words this item classified. **They do not bite because nothing prints them
yet**, which is `founded`'s history read from the other end.

**It changes the fix rather than the finding.** Repairing `phase` alone leaves the trap armed for
whichever of the other two is printed next. **And no existing guard catches it**: `one_word` in
`expected.rs` guards the *form* - one word, unquoted - and not the *vocabulary*. An undeclared
**single** word passes it, which is exactly how `play` got there.


### Q-59 - `P-302` binds this lens's own README, and this lens cannot act on it

**to** spec · **status** open · **raised** 2026-09-06 · **source** reading `docs/process.md` →
[What this document has to be](../../docs/process.md#what-this-document-has-to-be) at the
specification lane's pointing

**`P-302`:** *an insight that lives only in a conversation, a note, or an operating file is lost, so
a rule worth keeping is written here* - and what is in `docs/process.md` has to be enough to rebuild
the process with every `CLAUDE.md` and every note gone.

**[`README.md`](README.md) is an operating file and it carries rules of exactly that kind.** Poison
the thing the check reads; a count of zero says nothing unless the population is non-empty; probe
against a clone rather than the shared tree; commit by pathspec because staging is publishing; a
green suite under a poison bounds the tests and not the code. **None of them is a finding about the
game.** They are how the process is run, they were each learned by getting something wrong, and
`P-302` says the reason is part of the rule - which is the half this file is actually good at
keeping.

**What makes this a finding rather than a chore is who can act on it.** `docs/` is the specification
lane's column, so this lens can neither move them nor propose the wording. It can only say that the
rule landed and that the file it binds is one this lens owns and cannot fix.

**The scope needs deciding rather than assuming, which is why nothing is drafted here.** Some of
these are craft local to a lens and some are process. `docs/process.md` already defines what a
quality instance is, so what is at stake is the accumulated habit rather than the definition.

**Whether.** Worth doing eventually, not now. Sean has eleven proposals open, nothing is blocked on
it, and the cost of leaving it is that a rediscovery is expensive rather than that anything is
wrong. Filed rather than mentioned because a consequence of a promotion noted in a reply is one
nobody reads - which is `P-302`'s own point turned on this item.

### The scope split, supplied 2026-09-06 because the specification lane asked for it

**The discriminator, which is the part worth keeping if the list is wrong.** A rule is **process**
if losing it produces a **wrong result that survives review**; it is **craft** if losing it produces
a **worse result somebody notices**. Silent failure is what has to be written down, because nothing
else will report it. A loud one reports itself.

**Counted over the file rather than over the six that were listed.** `README.md` has **eleven**
rule sections. Ten are process by that test and one is craft.

| Rule                                                         | Losing it costs                                                      | Which       |
| ------------------------------------------------------------ | -------------------------------------------------------------------- | ----------- |
| A pattern is a claim about the bytes as they are now         | a `str.replace` no-op, silent, already cost thirteen rows            | **process** |
| Cite the commit, rest the claim on the file                  | a closure resting on a citation that is not evidence                 | **process** |
| Commit by pathspec, not by staging                           | another lane's work published under your message - it has happened   | **process** |
| Probe against a copy, not the shared tree                    | a foreign file in a tree three sessions commit to - it has happened  | **process** |
| Poison the thing the check reads                             | a green run that reads exactly like a check working                  | **process** |
| A green suite under a poison bounds the tests                | a live defect filed as small - `Q-58`, this week                     | **process** |
| When the instrument is confidence, make it produce something | the parent of the four above                                         | **process** |
| Read it before writing about it                              | confident prose about an artifact nobody opened - two lanes, one day | **process** |
| A self-check may share inputs, not the computation           | circular verification read as verification - `Q-56`, this week       | **process** |
| Re-read the source, not the summary                          | a stale copy that reads correctly                                    | **process** |
| Say it and stop                                              | a longer report, and a reader who says so                            | **craft**   |

**Where this disagrees with the first pass.** **Probe against a clone is process, not craft**, and
for the same reason as committing by pathspec: both exist because three instances share one working
tree, which is a fact about the process and not a preference of this lens. Its failure is
cross-lane and silent in the same way. **Say it and stop is craft**, agreeing with the first pass
and now with a reason - Sean read a sixty-line proposal and said so, which is a failure that
reports itself.

***Already in `CLAUDE.md`* is not a reason to leave one out.** `P-302` names `CLAUDE.md` among the
files that may be lost, so a rule sitting only there is exactly what it is about. Four of the ten
are partly there today, and that would quietly halve the list.

**The split that matters is not which rules move.** It is that **the rule moves and the case stays**.
`P-302` says the reason is part of the rule - and a reason is not a war story. `docs/process.md`
needs *a poison that lands where the check never looks produces a green run that reads exactly like
a check working*. It does not need which lane did it on which day. **The instances are what makes a
lens's file worth keeping, and they are correctly local.**

**One caution, and it is why this should wait for `P-304` rather than land beside it.** Ten of
eleven is a lot, and this is one lane's file. If three lanes each move ten rules with their cases
attached, `docs/process.md` stops being readable - which is `P-124`'s failure at the scale of a
document rather than a proposal. **Settle the general shape first**, and the lists become mechanical.

### The specification lane pushed back on *ten*, and it does not survive - measured 2026-09-06

**Their objection: several of those sections are mostly case, and what is left after the war story
is removed may be one sentence that already exists somewhere.** Testable now rather than at
drafting, so it was tested. **Seven, not ten.**

- **Three are already stated upstream**, and this file's versions are third copies with cases
  attached: *staging is publishing* is `docs/process.md:315`; the narrower-question tell and *a
  count over nothing* are `CLAUDE.md:286` and `:293`; *re-read before asserting* is in `CLAUDE.md`.
  Those need `CLAUDE.md`'s words moved, which is not a contribution from this lens.
- **One is craft** - say it and stop.
- **Seven are in neither file**: aim the poison where the check reads; probe against a clone; rest
  the claim on the file rather than the citation; a self-check may share inputs but not the
  computation; a green suite bounds the tests; make confidence produce an artifact; read it before
  writing about it.

**`docs/process.md:149` is the near-miss worth naming.** *Re-poison a check when its exception list
grows* is `P-291`, and it reads as coverage. It says **when** to re-poison and never **where to aim
one** - which is the whole of the rule this lens learned by getting a green run out of an inert
poison. A word in common, a different rule.

**Six of the seven are one idea with six faces**: how to tell a verification that verifies from one
that agrees with itself. **That is a section, not a list.** The seventh - probe against a clone - is
a shared-tree rule and belongs beside *staging is publishing*, which is already there.

**On the second objection, that the discriminator is this lens's applied to this lens's own file:
they are right, and it is conceded without argument.** It is *a self-check may share inputs; it may
not share the computation* - a rule in the very list being classified, turned on the classifier.
**The measurement above is the evidence rather than the rebuttal**: this lens's own *ten* did not
survive this lens's own method. **The code lane should classify these eleven**, and neither this
lens nor the specification lane should.





---

## Resolved

Kept rather than deleted, so a later report can tell whether a finding was fixed or forgotten.

### Q-62 - `S-53` closed one instance of its hole and left two, in the file it was named for

**to** code · **status** **acted** 2026-09-06 · `ba5943e` · **raised** 2026-09-06 · **source** checking `S-53`'s own sentence -
*a file the generator reads and the refusal omits is exactly that hole* - against both lists

**The two lists disagree, and nothing compares them.**

- `tools/outbox::places` reads six: `proposals.md`, `questions.md`, `crates/outbox.md`, **every
  `.md` under `releases/`**, and every `lenses/*/outbox.md`.
- `hooks/pre-commit:50` refuses on four: `docs/notes/proposals.md docs/notes/questions.md
  crates/outbox.md lenses`.

**`releases/` is read and not guarded.** So a half-written capability in `releases/first-release.md`
is rendered into `pending.md` and staged into whoever commits next - which is the hazard the hook's
own comment states, in the words it states it: *publishing another perspective's draft, under a
commit that touches nothing of theirs, to the one document Sean opens.*

**Demonstrated in a clone, both directions, because a claim about a hook is a claim about
behaviour.**

| Draft left unstaged in      | Hook says                  | Draft reaches `pending.md` |
| --------------------------- | -------------------------- | -------------------------- |
| `releases/first-release.md` | *rewriting pending.md*     | **yes** - `R-99` landed    |
| `lenses/quality/outbox.md`  | *NOT rewriting pending.md* | no                         |

**The mechanism is correct and only the list is wrong**, which the control establishes - without it
this would be a claim about the refusal rather than about what it names.

**Whether.** Worth fixing now, and worth fixing structurally rather than by adding `releases` to the
string. **The hook's list is hand-written while `places` discovers `releases/` and `lenses/` by
walking**, so the two cannot be kept in step by hand: a `releases/second-release.md` would be read
and unguarded the moment it existed, with nobody having decided that. **Derive the refusal from the
tool - one declaration, asked twice** - which is what makes it stay fixed after `S-53`'s instance
did not.

**Not this lens's own exposure**, which is worth saying plainly: `lenses` is on the list and the
control shows it holds. The file left open is the specification lane's, and neither producer would
find it - one does not read `hooks/`, and the other fixed the instance it was standing in.

**Closed 2026-09-06 · `ba5943e`, and fixed structurally rather than by adding a name to the
string.** `outbox --places` prints what the tool reads and the hook asks for it - one declaration,
asked twice.

**Verified in a clone, including the case their own three did not cover.** They tested that
`releases/first-release.md` now refuses, that an untracked outbox refuses, and that clean outboxes
still rewrite. **None of those tests the property this item argued for**, which is that it stays
fixed for a file nobody has written yet.

| Case                                              | Hook                       | Draft reaches `pending.md` |
| ------------------------------------------------- | -------------------------- | -------------------------- |
| a **new, untracked** `releases/second-release.md` | *NOT rewriting pending.md* | no                         |
| clean outboxes, a commit touching none of them    | *rewriting pending.md*     | n/a - and it is current    |

**The first row is the one that matters**: that file did not exist when the hook was written, and it
is guarded because `--places` walks rather than because anybody listed it. **The second is the
control** - a hook that had simply started refusing everything would pass the first test and be
worse than the defect.

**They found a second hole while in there**, which this item did not see: the generator reads the
working tree, so an **untracked** outbox is read like any other and `git diff` cannot see it. That
one would have bitten whoever starts `lenses/research/`, which `S-52` makes imminent.

### Q-61 - `S-51`'s input was wrong for eleven rows today, and one of them is its own poison target

**to** code · **status** **acted** 2026-09-06 · `38b2cbe` · **raised** 2026-09-06 · **source** `C-40` read against `S-51`, which
`P-305` landing in `0e9c9ac` has just unblocked

**`S-51` asks whether a closed item's cited `P-n` appears in the Withdrawn table.** That table is
hand-maintained and **it was wrong for eleven rows today** - `C-40`, fixed in `8d03a73`, which moved
`P-292` through `P-302` out of Withdrawn and into Accepted.

**`P-296` is one of the eleven, and `Q-53` is closed citing `P-296`.** Verified rather than inferred
from the range: `8d03a73` removes the `P-296` row from Withdrawn and adds it to Accepted, and it
sits in Accepted now. **`S-51` names `Q-53` as its poison target.** So had the check existed during
that window it would have reported `Q-53` as orphaned - **a false positive from a filing error
rather than from a withdrawal** - and under the rule just promoted the remedy is to file a reopening
into another lane's outbox. A ledger typo would have arrived in this file as a reopened finding.

**Per-commit detection does not rescue it**, which is the part worth checking before building.
`C-40`'s repair for the promotion checker was to judge per commit; here the wrong row was written
**in the promotion's own commit**, so *left the queue and gained a Withdrawn row* was true at the
moment it happened. The tell is elsewhere: **a promotion puts the text in a destination file and a
withdrawal puts nothing anywhere.** `a_promotion_lands_what_was_approved` already asks that
question.

**Whether.** Worth getting right before the first run rather than after. **Corroborate, or report
the disagreement rather than acting on it** - a row saying *withdrawn* while the destination file
gained the approved text is a ledger defect, and the check that cannot tell those apart will file
work into somebody's outbox on the strength of it. The population is named and non-empty: eleven
rows, one of them the proposal this check's own example cites.

**Closed 2026-09-06 · `38b2cbe`, built in rather than noted.** They asked one question back - whether
corroborating against `a_promotion_lands_what_was_approved` is stronger than their own discriminator,
since a destination cell is hand-maintained too. **Measured over both ledger tables, and the answer
is no: theirs is stronger, and stronger than the two alternatives tested here.**

| Discriminator                                   | Withdrawn, 26 rows | Accepted, 281 rows | Verdict    |
| ----------------------------------------------- | ------------------ | ------------------ | ---------- |
| third cell is empty *(this lens's, refuted)*    | 24                 | 0                  | **breaks** |
| cell 2 opens with a withdrawal word *(refuted)* | 13                 | 0                  | **breaks** |
| cell 2 opens with a `` `x.md` `` destination    | **0**              | 275                | **holds**  |

**The one this lens was about to recommend is the one that breaks.** *Third cell is a date* would
misread `P-279` and `P-282` - both genuine withdrawals, both dated - as misfiled Accepted rows, and
**skip the orphan check on them**. That is a false negative, which is the direction `P-305` exists to
guard. Found by running the rule over every row rather than over the case that suggested it.

**Their discriminator produces no false positives across the whole current population**, and the two
conditions together are load-bearing: `P-279` and `P-282` satisfy *dated* and fail *destination*.

**And corroborating against the destination file adds cost without adding separation.** The case it
would resolve - text landed, row misfiled - is `C-40` itself, where the destination check and their
discriminator agree. **A second instrument that agrees everywhere the first one is used is not
corroboration**, it is the same reading twice.

**One guard worth having, on their own principle.** The safety rests on **0 of 26**, a count over a
small population that will grow. A withdrawal reason opening with a backticked filename - *`spec/
planet.md` already says this* - would be read as a misfiled Accepted row. **Assert both counts**: the
misfiled figure, and the number still classified as genuine withdrawals, so the check cannot quietly
start looking at nothing.

### Q-60 - `P-305`'s third bullet has no actor, and no lane that could be one

**to** spec · **status** **acted** 2026-09-06 · `P-305`, promoted in `0e9c9ac` · **raised** 2026-09-06 · **source** reading `docs/process.md` at
`65f2627` at the specification lane's pointing, and following it to `P-305`, which is still open

**The bullet:** *a withdrawal that would orphan a closed item reopens that item.* **Reopens is
passive and names nobody**, and the two candidates cannot do it.

- The **specification lane** performs the withdrawal, and `Q-53` lives in
  `lenses/quality/outbox.md`. `CLAUDE.md`: *a producer never writes into a lens's directory.*
- The **code lane**'s `S-51` check reports. A check can name an orphaned item and cannot reopen one.
- Which leaves the **owning lens**, which has no signal that a proposal was withdrawn unless it
  happens to be running and happens to look.

**The proposal already contains the observation, one paragraph below the bullet**: *two items change
status the moment this lands, and neither is mine to change.* That is the same boundary, noticed for
the landing and not for the withdrawal it is proposing.

**So the rule as worded is the failure it was written to prevent.** An orphaned item goes quiet with
nobody having decided anything - which is the third bullet's own reason - because the step that
un-quiets it has no owner.

**And the check inherits the problem rather than solving it.** If `S-51` only prints, nothing makes
anyone act. If it asserts, the gate goes red for whichever lane commits next, and that lane may be
one that **may not** fix it - `CLAUDE.md` already names this shape, where a lane is gated on
something it did not write and must not repair.

**A shape that avoids both, offered as a shape and not as words.** Withdrawal is already covered by
a pattern this repository has: *a promotion that makes something else stale files the cleanup
immediately*. The same lane, in the same commit, **files the reopening as an item addressed to the
owner**. Nothing new is invented, the actor is named, and `S-51` stays a report rather than becoming
a gate.

**Whether.** Worth a clause now rather than a cleanup proposal later. **`P-305` is open**, so the
words can still change and this costs one line; after promotion it costs a proposal, and the rule
would be unbuildable in between. This lens is also the live case - `Q-53` is `S-51`'s poison
target - so it is the item that would go quiet.

**Closed 2026-09-06 · `P-305`, `docs/process.md` -> Outboxes and the index.** Read at the source: the passive is gone and the actor is named - *the lane withdrawing the proposal files the reopening as an item addressed to whoever owns the closed one, in the same commit as the withdrawal*. **The specification lane held a promotion Sean had already instructed** because this finding arrived after he last read the words, which is *promote means I have read this* working. What now tracks the building of it is `S-51`, and its input is `Q-61`.

### Q-53 - A session is producing findings and has no outbox to put them in

**to** spec · **status** **acted** 2026-09-06 · `P-296` · **raised** 2026-09-05 · **source** receiving `Q-51` by message from
the `4x research` session

`lenses/` holds one directory, `quality`. The `4x research` session says it writes nothing in
`lenses/`, and `pending.md` reads five outboxes, none of them its.

So a correct finding about two guards existed **only as a message to another instance that happened
to be awake.** Had this lens been idle it would have gone nowhere, and `pending.md` would have said,
truthfully by its own accounting, that nothing was outstanding. That is the state
`CLAUDE.md` -> *Nothing open means nothing outstanding* is written to make impossible.

**Not a defect in any file, which is why it is `to spec` rather than `to code`.** Either that session
is a lens, and `CLAUDE.md` -> *Starting a new lens* says it gets `lenses/<name>/README.md` and
`outbox.md` before it produces anything; or it is not, and what it is instead is Sean's to say. This
lens has no standing to decide which, and `Q-31` says it may not ask him directly.

**One fact worth carrying either way:** `tools/outbox` finds a lens's outbox by walking `lenses/`, so
a directory is all it takes - nothing has to be registered anywhere. And `Q-52` says that walk is
untested, which is a separate item and is the reason this one names it.

**Whether.** Worth a decision now rather than later. The cost of getting it wrong is silent: a
session's findings are as good as unfiled, and nothing anywhere reports their absence.

**Closed 2026-09-06 · `P-296`**, which creates `lenses/research/` with a README and an outbox and is open to Sean. **Closed on the routing rather than on the landing, and this lens said the opposite two messages earlier.** The correction is a consistency one: an item addressed to a lane closes when that lane has done what it can, and the specification lane has - it cannot promote its own proposal. Holding this open while closing `Q-54` on identical facts would have been two rules. **The gap itself is tracked by `P-296` now**, which is the one surface Sean reads, and it is a better tracker than an item in a lens's file.

### Q-54 - A right decision resting on a wrong reason is a defect with a delay on it

**to** spec · **status** **acted** 2026-09-06 · `P-303` · **raised** 2026-09-05 · **source** the code lane, naming the shape in
`fbb2511` and declining to file it on one instance; filed here on meeting the second

**A future reader meets the reason, not the decision.** So a call that is right for a reason that is
false reads as settled, survives review, and misleads exactly the person who comes back to it because
they are unsure - which is the person the comment was written for.

**Two instances today, both in the code lane, hours apart and on unrelated subjects.**

`7a0d425` left the stray candidate in `places` unguarded - right - because *`read` filters it, so it
is harmless*. It is not harmless: `read` pushes it onto `missing` and `main` prints it as **not
present**, so a file directly in `lenses/` would put a permanent false line into the output all three
lanes read. The decision survived the correction; the reason did not, and it had been written into
the comment. Fixed in `fbb2511`.

Earlier the same day, on `Q-50`: aligned output must be excluded from the whitespace rule - right -
*by adding it to the exception list*. It does not need to be, and is excluded by construction if the
check is scoped to literals on one physical line, because a joined wrap is on one line by
construction and aligned columns span several. Measured at 38 of 38.

**Not the same failure as `C-28`.** There the instrument answers a narrower question than the one
asked and returns a plausible number. Here the answer is correct and the account of why is not - so
nothing is wrong to find, and the cost is paid later by someone reading the account. Nor is it
`C-9`'s stale premise: these reasons were false when written rather than made false by something
landing.

**One adjacent case of this lens's own, stated as adjacent rather than counted.** `Q-50` was worth
filing - right - and gave twenty-two runs across seven files when it was twenty-three across eight.
A wrong figure under a right call, which is the same shape with a fact in place of an argument.

**Whether.** For the specification lane to judge, and this lens has no view on whether it belongs in
`CLAUDE.md` at all. What it is sure of: **nothing mechanises it.** No check can ask whether a
comment's reason is the reason - the same wall `P-245` and `C-28` hit - so if it is worth anything it
is worth a sentence and the two cases, in the section that already carries the habits nothing can
enforce.

**The specification lane read this as close enough to `C-28` and `P-291` to be a copy rather than an
addition, 2026-09-06, and invited a challenge.** Checked before defending, and the check changed the
answer rather than confirming it.

**Against those two it is an addition, and the discriminator is clean.** `C-28` and `P-291` are both
about an artifact that is **wrong or stale** - an instrument answering a narrower question, a check
whose green has stopped carrying information. There is something to find in each. Here **there is
nothing to find**: the decision is correct, the code is correct, and only the account of why is
false. That is why no check reaches it and why it survives review.

**And `P-302`, which landed after that judgement was formed, is the argument this item did not
have.** *The reason a rule exists is part of the rule - one recorded without its reason survives as
a ritual, and the first person to find it inconvenient deletes it correctly, for the wrong reason.*
**`P-302` covers a missing reason; this covers a false one**, and the two failures are the same
sentence read from either end. A rule with no reason is deleted correctly for the wrong reason; a
rule with a wrong reason is **kept** correctly for the wrong reason, and the person it misleads is
the one who came back because they were unsure.

So the place it belongs may be beside `P-302` rather than beside `C-28`, which is a different
section from the one this item first named. **Still the specification lane's call, and if it reads
as a copy after this, that stands** - being refuted is the lens working.

**Closed 2026-09-06 · `P-303`**, *a reason that is false is worse than one that is missing*, into `docs/process.md` -> What this document has to be. **Filed after being refuted and then re-argued**, and the proposal records that this lens changed its own answer while checking, which is the part a later reader should get.

### Q-56 - The `Q-47` check cannot see the spelling `Q-47` was filed about

**to** code · **status** **acted** 2026-09-06 · `a60def3` · **raised** 2026-09-06 · **source**
[review of `ba9bd41..217dcba`, finding 1](2026-09-06-review-of-the-six.md)

`tools/outbox/tests/architecture.rs:186` admits a literal that **is** `reports` or **starts with**
`reports/`. Two files reach the directory as `../../reports/...` and match neither, and one of them
- `prototypes/kinds/src/main.rs` - is a generator in `src/main.rs`, so it is **a trespass the check
does not report**.

**The population assertion confirmed the blind spot instead of catching it.** It asserts `>= 5` and
found five, agreeing with the figure in `Q-47` - which was already stale. Two counts that share a
computation are one count. And the poison landed inside the sighted region: a literal starting
`reports/`, which could not have found this.

**Verified in a clone, against the real check rather than a replica.** Adding
`|| literal.contains("/reports/")` turns it red, names `prototypes/kinds/src/main.rs`, and reports
**seven** readers.

**Whether.** Worth fixing now. Whether that file is the violation or the rule is too narrow is
yours - a generator that is a crate's only binary has no reason to sit in `src/bin/`. Raise the
population figure with the predicate: `>= 5` tolerates losing two readers in silence.

**Closed 2026-09-06.** The predicate matches the directory wherever it sits in the path, the population is asserted at seven, and the rule was widened rather than the file moved - `src/main.rs` is Cargo's default binary target and a crate whose only binary that is has no reason to use `src/bin/`. **Verified here rather than taken:** a probe naming `../../reports/state.md` from `crates/game-model/src/` - the spelling the old predicate could not see - is now reported by path, and the check names eight readers.

### Q-58 - A `saturating_sub` names density zero as its reason and no case has one

**to** code · **status** **acted** 2026-09-06 · `a60def3` · **raised** 2026-09-06 · **source**
[review of `ba9bd41..217dcba`, finding 5](2026-09-06-review-of-the-six.md)

`crates/game-model/src/territory.rs`, `most_in_one_turn`: the comment gives the reason as *a
density-zero or density-one food extractor buys no hand at all*. Density one is covered by a case;
**density zero is not**, in either table. Changing it to `density - 1` leaves every test green.

One row, worth adding while the file is open, and recorded so it is not re-found if it is not.

**Closed 2026-09-06, and the *whether* was wrong.** The code lane declined *small*, made the change, and found that `Territory::empty` has no deposits at all - `create planet` makes twelve before `set resource` fills any in, and `is_fully_exploited` asks `can_hold_yard` about them. Plain subtraction panics there. **Confirmed here by poisoning a clone: 51 passed, 1 failed, and their new test is the only one that catches it.** Their `C-38`.

**The lesson is mine and is in [the README](README.md#a-green-suite-under-a-poison-bounds-the-tests-not-the-code).** I wrote *changing it to `density - 1` leaves every test green* as though it measured the risk. It measured the coverage.

### Q-47 - *Presentations are never canonical* is checkable, and the obvious check would be decoration

**to** code · **status** **acted** 2026-09-06 · `19a8752` · **raised** 2026-09-05 · **source** `docs/process.md` →
[What verification requires](../../docs/process.md#presentation), read at the specification lane's
pointing

`docs/process.md` says **presentations are never canonical** and **presentations are generated from
data**. Nothing enforces either. It is the only one of that section's six checkable statements whose
failure is **silent** - a presentation read as a source looks exactly like a presentation until the
data changes underneath it.

**The rule currently holds**, so this is preventive rather than corrective. Every reference to
`reports/` in the tree:

| Role      | File                                                   |
| --------- | ------------------------------------------------------ |
| generator | `crates/game-console/src/bin/dump-state.rs`            |
| check     | `crates/game-console/tests/dump.rs`                    |
| check     | `crates/game-console/tests/dumps_are_current.rs`       |
| check     | `crates/game-console/tests/turns_reconstruct.rs`       |
| check     | `tools/pad-tables/tests/generated_files_are_padded.rs` |

**No production file reads a report.** Five, not the four the specification lane counted - their
list included `src/dump.rs`, which contains the word *reports* only inside HTML it emits, and
omitted the `pad-tables` test.

**The distinction they worried about does not need semantics.** *Reading to verify* versus *reading
as input* is drawable by path: the generator is `src/bin/`, the checks are `*/tests/*`, and the rule
is that **nothing else may name the directory**. A test reading a report is a test; production
depending on one is the failure.

**The trap is the spelling, and it is the reason to file this rather than just build it.** Two
spellings are in use - `"reports/…"` and `.join("reports")` - and they interleave:

- searching only the literal finds **3 of 5**, and misses the generator
- searching only the join finds **2 of 5**

**Both lanes fell into this within minutes of each other**, on the same question, from opposite
sides. A check written the way either of us searched would have reported clean while missing
readers - which is the guard-that-cannot-fail this repository has built twice and caught twice.

**Whether.** Worth building, and worth building carefully: match the directory rather than a string,
or match both spellings and assert the total, so the check fails if a third spelling appears.

**Corrected 2026-09-06, and the number had already travelled.** The population is **seven**, not
five: `prototypes/kinds/src/main.rs` and `prototypes/kinds/tests/catalog_is_current.rs` reach the
directory as `../../reports/...`. The check built from this item asserts `>= 5` and says *the rule
was written against five* - it agreed with this figure because it shares the computation, not
because either is right. `Q-56`.

**Closed 2026-09-06.** The check exists and matches the path rather than a spelling, which is what this asked for. **It has a blind spot in the one dimension this item was about**, and `Q-56` carries that rather than reopening this.

### Q-50 - A run of spaces sits mid-sentence in a failure message, in eighteen places

**to** code · **status** **acted** 2026-09-06 · `fc4029a` and `72391f7` · **raised**
2026-09-05 ·
**source** a scan of every non-comment string literal in `crates/`, `tools/` and `prototypes/`

A message reads *"if that table          moved or changed shape"*. **Eighteen runs over six lines in
four files** remain after `fc4029a`: `game-console/src/grammar.rs`,
`sphere-tessellation/src/quality.rs`, `tools/outbox/src/lib.rs`, and `tools/outbox/tests/promotions.rs`
at three lines, where the `KNOWN` exceptions carry several each.

**Why it is more than tidying, barely.** These strings are read in exactly one situation: a check has
failed and somebody is working out why. A green run never shows them, so nothing in normal use
applies any pressure to them at all - the same property that let `Q-48` exist. Two of the five
collapsed in `fc4029a` had arrived in `13497da` hours earlier, which is what makes this a rate rather
than a residue.

**Two numbers in the first version of this item were wrong, and both were stated without being
derived.** It said twenty-two runs across seven files; re-derived against `dc125d5` it was
**twenty-three across eight** - and the item then listed eight files under the word *seven*. Nobody
was misled and the item was acted on correctly, which is the point: **a wrong number that changes no
decision is the kind that survives.** Third time this lens has passed on a figure it did not compute.

### What `fc4029a` established, which is worth more than the five lines

**The code lane applied the rule as a regex across the tree and committed `C-28` doing it.** 53 lines
in 14 files, compiling clean, every test green - and it had destroyed the column alignment in two
usage strings and caught the deliberate newline-escape indents. **A plausible result rather than an
error**, half an hour after they wrote in `CLAUDE.md` that no check can ask whether another check's
predicate is about its subject. They reverted the nine files they had not read.

**So the rule reports and cannot apply.** 53 hits were 53 places to look, not 53 defects, and a fix
has to be right about every hit rather than most. Print, never assert.

### And the third case is free rather than an exception

They found aligned output - `--shot PATH          draw one frame to a PNG and exit` - by breaking it,
and asked for it in the exception list beside the newline-escape indent. **It does not need to be.**

This lens's detector never saw those lines, and not by design: it reads one physical line at a time
and skips any without two quotes on it, so **every multi-line literal is invisible to it.** Measured:
**38 lines in this tree sit in that blind spot, and all 38 are aligned output** - three usage blocks
and one diagnostic in `pad-tables`. **Not one is a joined wrap.**

That is not luck twice over. **A joined wrap is on one physical line by construction** - joining is
what put it there - and **aligned columns are across many by construction**, because that is what
they are aligning. So *the literal lies entirely on one physical line* is close to the real
discriminator, and restricting the check to those excludes alignment without an exception list.

**Stated as what it is: a measurement over this tree, not a theorem.** A wrapped paragraph inside a
`\`-continued block would be a joined wrap the check could not see, and would be missed. That is the
safe direction for something that prints, and it is still a limit worth naming. The population is 38
rather than zero, so this is not a count over nothing.

### One more artifact, from the same commit

`first_release.rs:98` reads `somebody else'''s row` - three apostrophes, in the comment explaining the
`Q-49` fix. A shell-quoting artifact rather than a wrap, and the only one in the tree. **Folded here
rather than filed** because it is the same subject: text that no build reads, so nothing pushes back
on it. Third artifact of this kind in two commits.

**Whether.** The eighteen are worth fixing by reading, one at a time, and there is no hurry. The
check is worth having if it prints and is restricted to single-line literals; poison it like anything
else.

**The single-physical-line discriminator has counterexamples as of 2026-09-06, and this item's own
range created them.** **Corrected again the same day, because the first correction understated it by an
order of magnitude.** I gave three counterexamples; the code lane's own run of the restricted
detector gives **51 hits of which 33 are not defects** - whitespace a test is deliberately
parsing, indentation after an escaped newline, and aligned output columns. The 38-of-38
measurement showed that *multi-line* literals are all alignment. **It does not establish the
converse**, and I wrote as though it did: single-physical-line literals are not all joined wraps,
and two thirds of them here are not. The refinement: **a run of
spaces immediately following an escaped newline is alignment.** The conclusion is unchanged - the
check prints, and never asserts.

**Closed 2026-09-06**, both halves. The eighteen are gone, re-derived here rather than taken from the commit. **The check half is answered rather than abandoned: it is not worth building as this item specified it.** The restriction that was supposed to exclude alignment leaves 33 false positives in 51 hits, so the fixer naming its lines and asserting each line's count - which is what landed - is the better instrument, and a standing check would print two thirds noise.

### Q-13 - Adopt the workflow in `CLAUDE.md`

**to** spec · **status** **acted** 2026-08-30 · `ba4850d`, and improved in three places on the way
in - see [the record](2026-08-30-workflow-to-adopt.md#what-changed-on-the-way-in)

### Q-14 - Build the outbox index

**to** code · **status** **acted** 2026-08-30 · `e233186`. Verified: it reads both outboxes, names
the ones missing, and reports by addressee

### Q-30 - `crates/outbox.md` did not exist, so a blocked question had nowhere to go

**to** code · **status** **acted** 2026-08-30 · `67c8b40`. Verified: the file exists, is empty and
says so, and carries a guarantee in the same shape as this one. `tools/outbox` reads three outboxes

### Q-34 - Emit the same-section flag the trigger depends on

**to** code · **status** **acted** 2026-08-30 · `67c8b40`, corrected in `d6908c9`. Verified: the
date scope is gone, the flag reports 16 sections rather than 13 - the number the specification lane
predicted - and `two_proposals_a_week_apart_in_one_section_still_flag` builds the case the dated
version could not see. Threshold left at more than one. Twelve tests pass

### Q-35 - Two spellings of the same section split one group into two

**to** code · **status** **acted** 2026-08-30 · fixed on both sides independently: `073d5e2`
normalised the arrows in `docs/notes/proposals.md`, and `67c8b40` normalises before grouping with
`one_arrow`, guarded by `the_arrow_style_does_not_split_a_section`. The code lane hit the same thing
while building `Q-34` and had it fixed before the item was filed

### Q-15 - Who checks the specification is buildable

**to** spec · **status** **acted** 2026-08-30 · `a2525bf`. Verified: `CLAUDE.md` carries the rule,
date-independent, with the directional-versus-symmetric distinction spelled out and *a trigger, not
a duty* stated. No readiness lens. The wording is better than the item it came from

### Q-31 - Stop a lens addressing Sean directly

**to** spec · **status** **acted** 2026-08-30 · `108ca79`. Verified: the `to` field is now `spec`,
`code` or a named lens, *"only the proposal queue addresses `sean`"*, and the mechanical reason is
recorded beside it. Sean's inbox is the open proposals. Rescoped from the item as filed, which was
wrong - see the body above

### Q-32 - `CLAUDE.md` carried two limits of fifteen that counted different things

**to** spec · **status** **acted** 2026-08-30 · `5262a3c`. Verified: exactly one *fifteen* remains
in the file, on the proposal queue where it belongs. The crowding concern survives with no number
and is better stated than in the item - it now says a lens crowds out *"the queue that actually
waits on Sean"*, which is the harm this lens described only as competition between lenses

### Q-33 - One generated document that says what is pending

**to** code · **status** **acted** 2026-08-30 · `358edfb`. Verified: `pending.md` exists at the
root, says what must be decided, then each producer's backlog, then the sections flag - so the
trigger is read without being asked for. The hook is unconditional rather than firing only when an
outbox is staged, which is right for a reason worth keeping: **an outbox changes in commits that do
not touch one**, because a finding is closed by the commit that acts on it and that commit is about
code. It raised `C-1` against itself, which is the better half of the delivery

### Q-4 - `planet-ecs` was wired into the shipped app and did nothing there

**to** code · **status** **acted** 2026-08-30 · `8346d62`. Verified: `game4x` no longer names
`planet_ecs` or `topology_of`, and its manifest is `bevy`, `game-console`, `game-front`,
`planet-bevy`. The crate stays for `prototypes/planet-view`, which is what it was built for.

**One claim to correct, because it matters for `Q-3`:** the code lane reported that `cargo tree` for
the shipped binary is now bevy, game-console, game-front, planet-bevy and planet-render. That is the
*manifest*. The tree still contains `planet-ecs`, because `planet-bevy` depends on it and
`planet-bevy/src/lib.rs:29` uses it for `PlanetViewPlugin`. **The plugin is no longer run; the crate
is still linked.** Cutting the link is `Q-3`, and this is exactly the cost of `planet-bevy` being
two adapters in one crate.

The residue is `C-2` in `crates/outbox.md`, which is the specification lane's and correctly not
theirs: architecture rule 6 says every game entity is an ECS entity, and the crate that made that
true is no longer in the application

### Q-7 - Two independent computations of which territories touch

**to** code · **status** **acted** 2026-08-30 · `8346d62`. Verified: `topology_of` is called only
by `prototypes/planet-view` now, so the shipped path computes adjacency once, in the binding, where
`create planet` is. 174 tests pass across the gate crates.

Resolved by deletion rather than by a test, which is better - the test this lens suggested would
have asserted that two computations agree, and the fix was that the second had no reader

### Q-8 - Two identities for one territory, with opposite conventions

**to** code · **status** **acted** 2026-08-30 · `f0c8609`. Verified: `World::canonical` builds the
picture's seeds from `canonical_seeds` - the call the model already makes - instead of reaching them
through `generate_balanced` and depending on jitter being zero. One derivation, which is the fix
this lens argued for over an assertion.

They then added the assertion as well, and it is the better half:
`the_picture_uses_the_seeds_the_model_uses` compares the two at every planet size, and a second test
demonstrates that the old path diverges under jitter *"while every test in the repository went on
passing"*. 81 tests pass. The fallback to `World::build` fires only where no canonical arrangement
exists, which is the prototype's case and has no model to disagree with

### Q-36 - The hook published another perspective's uncommitted work

**to** code · **status** **acted** 2026-08-30 · `4273971`. Verified the way the bug was: planted an
unstaged item in this outbox, committed something unrelated, and the hook refused, named
`lenses/quality/outbox.md` as the file that stopped it, and said how to proceed. The planted item
never reached `pending.md`. Refusing is the safe direction - a stale `pending.md` that says so is
recoverable at the next commit; a published draft is not

### Q-5 - Engine-free policy lived in `planet-bevy`, where the gate could not test it

**to** code · **status** **acted** 2026-08-30 · `49c4c46`. Moved rather than gated, which is the
better answer. `planet-presentation` holds `Orbit`, `Fingers`, `readable_on` and `summary`; twelve
tests, all passing, and `cargo tree` shows no Bevy anywhere beneath it - the only two mentions in
its source are comments saying where it came from. Both gate lists carry it.

They also did the half this item did not ask for: `cargo test -p planet-bevy` now runs in the gate
and in `pre-push`, in debug, reusing the build clippy already paid for. The rotations correctly did
*not* move - composing quaternions is engine arithmetic - but
`turning_the_world_never_moves_the_poles_sideways` is a regression test for a bug that shipped, so
leaving it after deploy would have half-answered the finding. *"None runs before deploy"* is now
false in both halves rather than one

### Q-10 - The quotation guard's convention had an unchecked near-miss form

**to** code · **status** **acted** 2026-08-30 · `e40629c`. Verified: the guard passes and its floor
is 40 checked quotations, up from 8. **The measurement inverted this lens's assumption** - the
colon form it was built for was the rare one, and the *unchecked* form was most of them.

Four quotations were wrong. The one that matters is `game4x/src/inspect.rs`, which said *the terrain
is continuous* where `spec/planet.md:73` says *the terrain **of the realistic drawing** is
continuous* - and the practical drawing's terrain is not continuous at all, so the dropped qualifier
was the whole claim.

Worth keeping: they measured before building, because the obvious rule - any emphasis near a
mention - reports 35 failures of which 7 are the author's own emphasis and 13 are asterisks in Rust
read as markdown. And their third poison caught an off-by-one they had just written, where the
scanner consumed up to the closing marker rather than past it, so a closer was read as the next
opener and a whole README came back attributed to `spec/planet.md`

### Q-2 - `Biome` lived in the game, so terrain and rendering depended on the game

**to** code · **status** **acted** 2026-08-30 · `7283650`. Decided as `planet-model`, beside
`PlanetSize`. Verified: `Biome` is `planet-model/src/biome.rs`, and neither `planet-terrain` nor
`planet-render` names `game-model` any more. `game-model` re-exports it, so the game still reads as
owning its vocabulary without owning the definition.

Their second argument is the one that settles it and this lens did not have it: **every rule about a
biome is written in `spec/planet.md`.** It is the planet's vocabulary, and it sat in the game only
because that is where the first rule reading one happened to be

### Q-6 - `planet_ecs::gather` was dead, and its body existed twice more

**to** code · **status** **acted** 2026-08-30 · `9cade4c`. Neither option this item offered was
available as written - `gather` collects owners and `advance_turn` needs entities too, from a
different query - so the shared thing became `by_region(count, rows)`, a function over *what is
being placed* rather than over what an owner is. Verified: three call sites, two in `planet-ecs` and
one in `planet-bevy`. Tested directly as well as through the turn, and poisoned by making it push in
arrival order, which failed three tests

### Q-11 - The composition root had grown logic and tests

**to** code · **status** **acted** 2026-08-30 · `69ab140`. One sentence, as the item said. The
header now calls the crate a composition root *and the remote control that operates it*, names both
exceptions and why each is there, and picked up `planet-presentation`, which the diagram was
missing.

It raised `C-6` against itself, which is the better half: fixing the false claim left a new one -
`main.rs` now says a rule owned by `docs/architecture.md` is broken there deliberately, and carving
an exception into another perspective's rule is not the code lane's to do

### Q-1 - The palette existed in three places and nothing checked the copies agreed

**to** code · **status** **acted** 2026-08-30 · `8a06978` and `a4e3bd1`. Verified: no palette
literals remain in `planet.wgsl`, the uniform carries both palettes plus background, border,
duplicate strength and owner tint, and `linear_rgba` is public so the transfer curve is defined
once. The harness that made the second half checkable is `--shot`, `--settle` and
`--renderer gpu|cpu` on `planet-view`.

**Their evidence argues this item better than the item did, and it checks out.** The transcription
had already drifted: `0x1B3A5C` is `0.10588…` and the shader said `0.106`. Recomputed here through
sRGB to linear and back to eight bits, on four channels - `1B`, `8B`, `4F`, `E8` - the exact value
and the transcribed one produce the **same byte** every time.

So the two copies disagreed in source and agreed in output. **A test comparing them would have
passed while they diverged**, and the disagreement would have surfaced only when someone changed a
hex value, with nothing to attribute it to. This lens argued *better deleted than tested* because a
test keeps both lists; the stronger reason is that the test would not have worked.

Also worth keeping: the harness caught a surviving `BACKGROUND` reference within ten minutes, in a
branch they had not read. Without it, that ships as a shader that fails to compile on the one path
nothing photographs

### Q-3 - `planet-bevy` was two adapters in one crate, and `planet-render` two crates

**to** code · **status** **acted** 2026-08-30 · `465437a` and `253418d`. Both halves done. Verified
by `cargo tree`: `game4x` carries `game-globe`, `planet-bevy`, `planet-render` and no rasterizer;
`goldberg-view` carries `planet-bevy` and `planet-render`; `planet-view` is the only binary with
`planet-flat` and `planet-raster`. Neither producer of a globe carries a rasterizer, and the
prototype that needs one is the only thing that has one.

Their report that the only edge between `planet-render`'s two halves was a doc comment is the
measure of how real the seam was.

Two things the split found that no compiler could. The embedded shader path contains the crate name,
so moving `planet.wgsl` left `embedded://planet_bevy/planet.wgsl` pointing at nothing and the flat
projection rendered an empty window **with no error at all** - noticed because the PNG was a quarter
of the expected size, on the path that had no instrument until `Q-1`'s harness that morning. And a
crate split dropped tests out of the gate, which is `Q-37`

### Q-37 - The gate listed crates by name, so a split silently dropped tests

**to** code · **status** **acted** 2026-08-30 · `7739826`. Verified: clippy is `--workspace` with no
list at all, the release step is `--workspace` minus seven, and a debug step names those seven. The
exclusion states one checkable fact - *does this crate link an engine* - rather than a set to
remember.

408 tests in the release step where eleven named crates were, and both prototypes are gated for the
first time. On the question this lens declined to guess at: none of the seven needs a GPU, because
no test in the workspace constructs `DefaultPlugins`. `game4x`'s 8 tests are now gated too, since
the exclusion form has no way to leave a crate out without saying so - which is the property that
makes it the right shape

### Q-38 - An outbox went stale because its filer could not see it being answered

**to** code · **status** **acted** 2026-08-30 · `954c224`. Verified: `tools/outbox` reads the log
for commits citing an open item, skipping commits that touch the item's own outbox - filing and
closing being exactly that shape. 18 tests.

**It found a live one before it was finished**: `C-5` and `C-6`, cited by `1d8c46f`, settled while
the tool to notice was being built.

The design question this lens did not anticipate is the good part. `C-5` was cited, read, and
correctly stayed open, because the citation answered half of it. Without somewhere to record that,
the report would name `C-5` on every run forever - **and a signal that always fires is one nobody
reads, which is the failure it exists to prevent.** So an item may carry `**cited** <hash>`, an
author saying *I looked, and it stays open*. Poisoned by deleting `C-5`'s.

And it prints rather than refuses, scoped the way `Q-36`'s refusal was: every perspective commits in
this tree, and failing one lane's commit because another has not closed an item would be the wrong
perspective paying

### Q-39 - Nothing checked shipped text against approved text

**to** spec · **status** **acted** 2026-09-02 · filed as `S-10` to the code lane, verified at
`docs/notes/proposals.md:98`, carrying this lens's design **and its argument against building it
yet**. They did not present it as their own.

They verified every factual claim before acting rather than after, including the two that were
corrections to their own write-up.

**Caveat withdrawn 2026-09-02.** This item said there was a real argument against building `S-10`
yet, because `P-182` may make `edit.py` reviewable and a reviewable tool may not need a check
downstream of it. **That is wrong, and the specification lane's reason is decisive and evidenced
rather than argued: a tool cannot enforce that it is used.** Three of the eleven defects were
commits chained after `python - <<PY`, an ad-hoc script rather than the guarded tool - so `spec/` was
edited outside the guards precisely on the occasions something went wrong. Reviewability lowers the
defect rate inside the tool and says nothing about the edits that never enter it.

Recorded here rather than only in a reply, because this file is the record and it carried an
argument this lens no longer holds.

The part worth keeping is their own reading of the correction: they framed the day as a question
about tempo, reached the right lever anyway, and could only find out which by being measured. *Right
about what to fix and wrong about why, and only the second is checkable*

### Q-40 - The visible half of the editing tool was the half that was not making the mistakes

**to** spec · **status** **acted** 2026-09-02 · `172ea26`. Verified: `asserts_about_the_tree`,
`check_claims` and `proposals_without_text` are in the crate, seventeen tests, and
`a_claim_of_zero_over_an_empty_population_is_refused` reproduces this lens's own error. **Poisoned
from outside the lane, which is what the item asked for and could not do** - see `Q-41` for the one
hole it has.

**Half of the item's subject remains and they said so themselves**: the crate has no binary and
nothing calls it, so `edit.py` is still what runs. `promote` is the operation that would change
that, and it waits on `outbox` exposing a proposal's text, which sits with the code lane. Not
reopened - the finding was that the guards were unreadable, and they are not any more

### Q-41 - The denominator guard checked that a denominator was non-empty, not that it was the right one

**to** spec · **status** **acted** 2026-09-02 · `40b74c0`. The narrowing taken whole: needle and
denominator are both counted inside a named section, and a claim of zero must name one. Re-poisoned
from outside the lane rather than taken:

| Case                                                     | Now                                     |
| -------------------------------------------------------- | --------------------------------------- |
| the hole as reported - `P-` within `## Open`             | **refuses** - zero out of zero          |
| the residual they documented - `P-` within `## Accepted` | passes, knowingly                       |
| a section that does not exist, `## Opne`                 | refuses - *no section*                  |
| the old form, no section named                           | refuses - a claim of zero must name one |

The third was the case worth checking and it was mine to worry about: **the fix could have
reintroduced the error one level up**, a typo'd region silently counting zero in an empty slice. It
does not - a missing section is an error rather than an empty one.

The residual is correctly out of scope and is documented on the type rather than in a note, which is
where the next author will be standing. Naming the region does not make the choice right; it makes
it written down, where picking a convenient denominator over a whole file was invisible

### Q-42 - Two sentences about a lens's column contradicted each other, and a third was stale

**to** spec · **status** **acted** 2026-09-05 · filed as `P-243`, `0c07376`, which `pending.md` now
carries under *What must be decided*. All three verified by that lane rather than taken.

They named the cause without being asked: promoting `P-240` put *every lane owns the tools for its
own work* four bullets above *and nothing else*, so **the older bullet was true until the moment the
newer one landed**. Adding to a list without re-reading the list - the same trigger they apply to
specification sections and had not applied to that one.

**One count of this lens's was wrong.** It reported six items addressing Sean; five were real and
the sixth was prose *inside* `S-17` describing capabilities, matched by a pattern that did not
require a field line. It is six now only because `P-243` has since been filed. Same family as the
zero-over-nothing error - a pattern that matched writing *about* the thing rather than the thing

### Q-43 - The citation check fired forever on an item that took several commits

**to** code · **status** **acted** 2026-09-05 · `10b3985`. `Unclosed` now carries how many commits
cite an item and the report says *(3 commits cite it)* where there is more than one. Nothing is
filtered and no intent is read - the reader is shown which line is unlike the others.

**The narrowing this item proposed was refuted before it was built**, by a measurement this lens
suggested and the specification lane ran. `S-37` is itself a multi-commit case - four commits name
it - and it was **the one line of seven that needed acting on**. A count-based filter would have
hidden it with the six that did not. The shape holds at scale: their count was 63 of 84 cited ids
appearing in more than one commit; counting every `X-n` in every message rather than only ids in an
outbox, this lens gets 174 of 205 - a wider population and the same conclusion.

**The code lane declined the narrowing for the reason this lens had already given**, and added one
it found itself: their commits name an id as a subject prefix - `S-19: control is derived from a
citizen being there` - which finishes something while saying nothing about being finished. An
intent-reading check would have missed the common case here.

Worth keeping: the finding was right and its proposed remedy was wrong, and those were not equally
good. Filing it and saying *doing nothing is a real option* is what left room for the count, which
is better than either

### Q-44 - Three doc comments outlived their fields and documented `held`

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified with the same detector that
found it: no field in `Territory` now carries comment lines from more than one block

### Q-45 - The trait system was defined and nothing read it

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified: five of the six variants are
gone and only `Ready` remains, which is read. Each returns in the commit that makes a rule read it.

**Their reason for acting is better than the finding.** The five were *where this is going, written
down as though it were state* - and **"going to be read" is not a property a compiler or a test can
tell from "dead."** They also took the correction that *state is things in places with traits*
described only the half that moved

### Q-46 - The gates named the tools one by one, and the newest was on no list

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified: `hooks/pre-push:58` and
`pipeline.yml:127` both iterate `tools/*/Cargo.toml` rather than naming them.

**It found two more on its first run** - nested `if let`s in `tools/outbox/src/promotions.rs` that
nothing had ever linted. So it was not one tool unlinted: `tools/outbox` had been named for tests
while never being linted at all, and the lint itself was unreachable.

**One exclusion, named with its reason**, which is the right shape: `tools/spec`'s clippy is skipped
because that crate is the specification lane's and the code lane may not edit it - a gate red on a
file its owner cannot fix is the trap `CLAUDE.md` names. Its tests and formatting run. The exclusion
goes when that lane fixes the warning

### Q-48 - Two guards looped over a parsed population and asserted nothing about its size

**to** code · **status** **acted** 2026-09-05 · `13497da`. Verified: `first_release.rs` asserts
twelve both where the table is parsed and before the loop, and counts the territories checked after
it; `poles.rs` floors `arrangements_up_to(200)` at eight. Both poison-tested by the code lane, and
the parse re-run here against the current release - twelve rows, ids 1 to 12

**The code lane found it looser than this lens did.** `released_table` keys on no heading at all: it
scans every line in the file, which they established by renaming `## Territory resources` and
watching nothing break. So the failure mode was never only *the table moves* - it was also *any
other table grows an integer first column and four cells*. The release gained store rows in two
sections today and neither has an integer first cell, so it held by luck. See `Q-49` for the half of
that luck the count assertion does not cover.

**`poles.rs` is floored at eight rather than at twelve, deliberately**, and their reason is better
than a number would have been: the count is a property of the tessellation and not of the test, so a
bound needing an edit whenever the geometry gains an arrangement would be edited without being
thought about

### Q-49 - The count guard on `released_table` made an added row loud and left a colliding row silent

**to** code · **status** **acted** 2026-09-05 · `fc4029a`. Verified at
`crates/game-console/tests/first_release.rs:102`: `insert` is asserted `is_none()`, and the reason is
written where the parse is rather than where the test loops - a second row claiming territory 3 would
have replaced territory 3's expected nodes while the length stayed twelve

### Q-51 - Two source guards asserted no offences without asserting they read anything

**to** code · **status** **acted** 2026-09-05 · `7a0d425`. Verified: `no_floating_point_anywhere`
counts what it scans and floors it at six of the eight files under `game-model/src`; the game-noun
guard floors at five of seven. Floors rather than counts, for the reason `poles.rs` got one - the
number is a property of the crate's layout, so a bound needing an edit whenever a file is added would
be edited without being thought about

**Raised by the `4x research` session and relayed, because it has no outbox** - which is `Q-53`

### Q-52 - The test named for walking the lens directory passed with the walk deleted

**to** code · **status** **acted** 2026-09-05 · `7a0d425`. Verified: the fixture builds a root
holding two lens directories, one directory with no outbox and a stray file, and asserts what the
walk returns. Deleting the walk now fails it

**The fixture found what neither of us had argued for, and the code lane's version of the lesson is
better than this item's.** This item said the negative assertion is not evidence because it passes
against an absent root. True, and the sharper statement is that **neither of us knew what `places`
returned until something ran it against a directory that existed.** It yields a candidate for every
entry under `lenses/` - including a directory with no outbox, and a stray *file* walked as though it
were one, because `read_dir` does not say which an entry is. They had written the assertion as two
and it failed at four. **So `places` offers somewhere to look rather than a list of what exists**,
which nothing in the code said anywhere.

**Their call not to guard the stray file is right, and one consequence is worth recording with it.**
An unreachable candidate is not inert: `read` pushes what it cannot open onto `missing`, which
`main` prints as *not present*. So a file sitting directly in `lenses/` would produce a permanent
false line in the output all three lanes read. **Noise rather than error, in a case that does not
exist today** - `lenses/` holds one directory and `CLAUDE.md` puts a lens's README inside its own
directory - and it would be visible the moment it did.

Worth being accurate about the cost of the fix, since it is the reason offered: `.is_dir()` is a
filter rather than a behaviour change. **The reason not to do it is that the failure is one visible
line in a case nothing produces, not that the change is large** - and `read` filtering it is what
makes the design coherent rather than lucky

### Q-55 - `C-34`'s population said four and two of its entries could not be exhibited

**to** code · **status** **acted** 2026-09-06 · `af6b8ed`. Verified: `C-34` now says two, one
unconditional and one conditional on `S-47` not giving a thing an id, and the three that fell are
kept written down with the reason each fell

**The code lane tried to write the exhibits and could not**, which is the outcome the test exists to
produce. They also named the tell this lens had only pointed at: **the refutation was inside the
entry.** *Nothing stops two territories both listing a unit, because neither lists it* - the clause
after the comma refutes the clause before it, and the next sentence credited containment with
removing what was never there.

**The finding they credit to this lens is the direction of their own guard.** `C-34` said a claim
naming more than four has grown past what was true, and pointed it upward only. **A population that
is too large makes the eventual claim look better tested than it is** - the exact failure the record
was written to prevent, committed by the record, within a day of it being written

### Q-16 - The picture never sees the biome the model has

**to** code · **status** **withdrawn** 2026-08-29 · **source**
[report 3, finding 1](2026-08-29-coupling-under-the-game.md#1)

Declined by the code lane, correctly. The remedy would have made colour uniform per territory,
drawing a boundary along every territory edge and failing
`two_regions_meeting_at_a_point_agree_about_it`. Draining fires zero times at twelve territories,
which is what ships. And the requirement it leaned on - that the drawing must show the biome the
model has - is not in `spec/planet.md`; a comment invented it. What survived became `P-123`.

### Q-17 - The biome rule and the connectivity rule cannot both hold

**to** spec · **status** **acted** 2026-08-29 · filed as `P-123`

### Q-18 - A report reaches one lane by instruction and the other by luck

**to** spec · **status** **acted** 2026-08-29 · `9d3fa25` added the fifth consequence to `CLAUDE.md`

### Q-19 - A contradiction can sit outside the queue the queue promises to hold

**to** spec · **status** **acted** 2026-08-29 · `14d9784`, `9d3fa25`

### Q-20 - Resetting the view was unreachable on a touch device

**to** code · **status** **acted** 2026-08-28 · `464ff45`, and `a1cc5e0` named the control

### Q-21 - The size keys were a binding no document named

**to** spec · **status** **acted** 2026-08-28 · `a1cc5e0`, with no code change needed

### Q-22 - Two comments quoted rules that were not there

**to** code · **status** **acted** 2026-08-28 · `464ff45`, and followed through in `8c395d8`

### Q-23 - `pre-push` and the CI gate disagreed about clippy

**to** code · **status** **acted** 2026-08-28 · `464ff45`

### Q-24 - `planet-terrain` was in neither gate list

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`

### Q-25 - The quotation guard stopped at `crates/`

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`, more broadly than asked. The *form* is
still open as `Q-10`

### Q-26 - The detached globe advertised keys it did not have

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`

### Q-27 - `docs/architecture.md`'s crate table no longer described the tree

**to** spec · **status** **acted** 2026-08-30 · verified: every row now matches its manifest, and
`planet-terrain` and `goldberg-view` have rows

### Q-28 - What *"where there is a pointer they are controls"* binds

**to** spec · **status** **answered** 2026-08-28 · `a1cc5e0` removed the sentence, as `P-95`

### Q-29 - Whether `/new <size>` changes no game state

**to** spec · **status** **answered** 2026-08-28 · `a1cc5e0` reworded it, as `P-95`
