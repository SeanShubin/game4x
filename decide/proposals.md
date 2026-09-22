# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-531 - `T3`: the approved copy is the specification, and three columns follow

**to** sean · **status** open · **raised** 2026-09-21 · **kind** recovered · **shape** text · **asks** approval · **into** `CLAUDE.md` -> Perspectives

**You chose `T3` on 2026-09-21**, having read the options and the workflow reading of them. **This
is the rewrite that answer requires**: an answered question is not a promotion, so what follows is
words to read rather than a decision already made. The reasoning is in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

**This file is the one no instance may carry an approval for**, so *promote P-531* has to arrive
from you in your own message. Written into the item rather than left to be remembered, because the
moment it would be easiest to skip is this one.

## What lands, and it is three paragraphs in one section

Into `CLAUDE.md` -> Perspectives, after the paragraph about `temporary-notes/`:

> **A test in `spec/tests/` arrives the way everything in `spec/` arrives: Sean has read it.**
> What differs is the interface. A proposal is read in the queue and a test is read in the review
> application, and the key that records the reading is *promote* said another way. **The guarantee
> is the same and is checked more often** - a promotion is verified once, at the moment of
> copying, and a test is compared against the record on every build.

> **`reviewed/` is the record of what Sean has read, and no instance writes it.** A copy of a test
> as he approved it lands there when he says so, and nothing else puts a file there or takes one
> away. **It is tracked, which is what makes it different from `temporary-notes/`**: that
> directory is addressed to nobody, and this one is what every lane is measured against.
> **The suite runs the copies in it**, so a test nobody has read constrains nothing and a test he
> has read is red until the code obeys it.

> **It is neither producer's, for a different reason each.** The specification lane writes the
> tests, so a lane that could also write the record could approve its own work. The code lane is
> what the tests constrain, so a lane that could write the record could clear a failure by editing
> the approval instead of the code. **The record is the one artifact whose whole value is that
> nobody judged by it can touch it** - which is why the review application is the code lane's, the
> same reason a lens never edits what it reviews.

## What it changes for each lane, stated so you can check it against the table above it

**The specification lane gains a job and loses nothing.** `spec/tests/` is already inside `spec/`,
so the *Writes* column needs no edit; what changes is that this lane turns your prose into tests
rather than only into prose.

**The code lane loses an escape and gains a constraint it cannot argue with.** It cannot edit a
test - not its column, and `hooks/pre-commit` refuses a commit that spans two. **A red test is the
specification saying no**, and the only ways out are changing the code or your re-reading a changed
test.

**And one lane gains nothing, which is the point.** Neither writes `reviewed/`.

## What this does not settle

**Whether `spec/tests/` and `reviewed/` are the right names.** Both are this lane's guess and
neither is load-bearing; say other words and they become other words.

**How the tests get there from the prototype.** Fifty-three exist in
`prototypes/thin-engine/data/foundation/tests`, every one read and approved by you, and moving
them is work rather than a rule. **It is not in this proposal** because the rule should be true
before anything moves under it.

**Bulk approval.** You set it aside and nothing here depends on it. **One test at a time is the
only scale at which *no other kinds of changes* is something you have checked**, and that stays
true until the application can classify a change set.

**And `P-530` is still open.** That one says the test is the primary statement and prose says what
a test cannot; this one says where the tests live and who may touch the record. **They are
independent** - either can land without the other - **and neither is complete alone.**

## The case a record and the tests can disagree about, and nothing may act on it

**Raised by the code lane against this draft, with its own commit as the evidence.** `e15ba69c`
deleted two records in `reviewed/` that named tests which no longer existed - a rename had left
them behind, **and a later test given one of those names would have opened as *reviewed* with
nobody having read a line of it.** That was correct work.

**Under the rule above it is illegal.** If no instance writes `reviewed/`, then no instance may
remove a stale record either, and the false approval it enables has no way out. **The one case
where the record and the tests can disagree is the one case nothing is allowed to fix.**

**The answer is not an exception, it is the application.** The record is written when you press a
key and removed when you press the other one; `u` already does that for a test you can see.
**What is missing is that an orphaned record has no test to navigate to**, so it cannot be reached
in order to be taken back.

> **A record is added and removed only by the review application, acting as Sean.** No instance
> writes `reviewed/` by any other route. **The application shows a record whose test is gone**, so
> that a rename - which leaves an orphaned record and an unread test - is two things he can see
> and act on rather than one thing nobody may touch.

**A rename is the common case rather than an exotic one**, which is what makes this worth a
sentence in the rule: it is a delete and an add, and it orphans a record every time.

## What has to change in `hooks/pre-commit`, and it is not a separate item

**One `case` line**, so that `reviewed/` is its own column rather than the code lane's.
**Measured, and this lane had it wrong**: `hooks/pre-commit:68` maps `prototypes/*` to `code`, so
the record sits in the code lane's column today - not, as an earlier draft of this said, in no
column at all.

**It is not filed to the code lane and this lane was wrong to file it.** Which column a path
belongs to is a statement about who may write what, which this file reserves for you. **The code
lane refused it on exactly that ground** - not doubting the relay, but unable to check it, which
is the reason the rule exists. `S-146` is withdrawn and the line lives here, where the rule it
serves is.

**It also changes what commits are legal today**, before anything is promoted: `e15ba69c` would
have been refused by it. **That is an argument for landing the line with the rule and not before
it**, rather than an argument against the line.

### P-530 - The specification is executable, and prose is what the tests cannot say

**to** sean · **status** open · **raised** 2026-09-21 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/README.md` -> Rules for this directory

**Your words, 2026-09-21:**

```
The specification had gotten to complicated for me to understand, the consequence of which is
that I lost executive control.  Thin-engine was a prototype to see if it was possible to regain
executive control via an executable specification rather than a prose specification.

I was convinced after reviewing tests like the ones in
prototypes/thin-engine/data/friendly/tests via `cargo run --example review-web`, I want to adopt
this workflow for the specification as well.  We can still have prose, and may still need it to
express some things that the tests can't cover, but these tests are now the primary way which I
ensure the game behaves as I intend it to.
```

## What `spec/README.md` says today, and which sentence this changes

```
3. **If it is not here, it is not decided.** Discussion is not decision.

   A rule is decided here. **The game's data is decided in its data file**, reviewed by hand and
   locked by the scenario test. Neither is decided in a discussion, in a note, or in a rendering
   of either one.
```

**That rule already has two homes for a decision - prose and a data file - and this adds the third
and makes it first.** It is the smallest change that says what you said.

## What lands

Rule 3 becomes:

> 3. **If it is not here, it is not decided.** Discussion is not decision.
>
>    **A test is the primary statement.** What the game does is decided by a test that runs, read
>    and approved one at a time, and a rule the tests assert is not written in prose as well.
>    **Prose says what a test cannot** - what a thing is for, why a rule is the shape it is, and
>    anything with no observable behaviour to assert. **The game's data is decided in its data
>    file**, reviewed by hand and locked by the scenario test. None of the three is decided in a
>    discussion, in a note, or in a rendering of any of them.
>
>    **Where prose and a test disagree, the test is right and the prose is a defect.** Prose is
>    the one of the three that can drift without anything noticing.

## The half this proposal was missing, and the prototype already has it

**Raised by the code lane, and it is right.** *Where they disagree the test is right* means
nothing without a record of **which version you approved** - otherwise the rule says a test is
right because it is a test, which is not what you said.

**The prototype has the mechanism.** `reviewed/` holds a byte-for-byte copy of the test as you
read it; the report compares the current test against that copy, normalized on whitespace, and
says *never reviewed*, *reviewed*, or shows where it drifted. **That is `promote` made
continuous** - the protocol in `CLAUDE.md` gets the same guarantee once, at the moment of
copying, and this gets it on every build.

**It is not offered as words above** because it belongs with the answer to where the tests live,
and that is still open. **Named here so the rule is not approved without it**: a test being
primary and a record of your approval are one mechanism, not two.

**And it is one-directional today**, which this lane found by checking it: 54 records against 53
tests, two records pointing at tests that no longer exist and one test with no record. The report
walks tests and nothing walks records. **Filed to the code lane as `S-145`** - small today,
load-bearing if this rule lands.

## Where the tests live, and the code lane's view on it

**This proposal leaves it open and the code lane has argued a position**, recorded here rather
than adopted because it is yours:

> A test he writes and approves is normative - it is the thing the prose obeys - so it cannot
> live where either producer edits it freely. That rules out `prototypes/` and `crates/` as much
> as anything of the specification lane's. The prototype has already demonstrated the split: the
> test is data and the thing that runs it is code. So the tests belong in a column governed like
> `spec/` - his, changed only by promotion - with the runner in `crates/`.

**This lane agrees and is not the one to say so**, which is why it is quoted rather than written
into the rule.

## What this does not decide, and each is a separate question

**Where the tests live.** They are in `prototypes/thin-engine/data/friendly/tests` today, and a
prototype directory is the code lane's column. **Moving them into `spec/` would make them this
lane's to write, which is the opposite of what you want** - so the home is a real question and
this proposal does not answer it.

**What happens to the prose already here.** Seventeen documents state rules the tests will
restate, and *a rule the tests assert is not written in prose as well* makes most of them
candidates for deletion. **That is a large, slow read and not a promotion**; this lane will file
it as a plan rather than as one change.

**What happens to `spec/data/`.** Four of the twelve items waiting on you are about it -
`P-513`, `P-514`, `P-516` and `P-517` - and the thin-engine has its own notation for the same
facts. **They are annotated rather than withdrawn**, because whether `spec/data/` survives is
part of the question above and not this lane's to settle.

## Why this is worth its own proposal rather than being folded in

**It is the sentence that decides what every other item in the queue is worth.** A proposal that
adds prose to `spec/` is a different thing under this rule than it was yesterday - `P-518`, for
one, argues that *a test cannot say what a written form means*, which is an argument this rule
invites you to check rather than accept.

**And it is the one thing here that is about your own control rather than about the game.** Your
reason is stated and this lane has not improved on it: the document grew past what one person
could hold, and a test you have read is a piece of the game you are certain of.

## And the argument arrived as a measurement the day after it was written

**Promoting `P-522` broke thirty-eight checks across fifteen targets**, measured against a
baseline so the attribution is real: 74 targets run before and after, 1 failing test before and
39 after, and the one that was already red is unrelated.

**All thirty-eight read `releases/first-release.md`'s markdown tables as their source.**
`CLAUDE.md` already forbids exactly that - *a table of game data in markdown is a rendering and
never a source* - and the rule was being broken thirty-eight times over, by checks that each
looked reasonable on its own. **The count did not exist until something went red.**

**Nothing but the tests noticed.** Not the padder, not the outbox index, not two lanes reading the
diff. **That is the case for this rule stated as an event rather than as an argument**, and it is
stronger than anything either lane wrote in favour of it.

**It also bears on where the tests live.** The code lane's sentence, 2026-09-21: *a test that
reads a prose table is reading a rendering as a source, and thirty-eight of them did.* **Whatever
column the tests end up in, what they read has to be a stating form** - so the home question and
this one are the same question.

The measurement is in
[thirty-eight checks read a rendering](../docs/notes/2026-09-21-thirty-eight-checks-read-a-rendering.md).

### P-528 - One fact about adjacency is now stated three times, in two files

**to** sean · **status** open · **raised** 2026-09-21 · **kind** entailed · **shape** an instruction · **asks** approval · **into** `spec/orbit.md` -> Crossing between layers, and `spec/planet.md` -> Distance

**Filed the moment `P-526` landed**, and it retires `S-74`, which found the first two in September
and could not close because the third had not arrived.

*Two places on the same layer are adjacent when their territories are, and a place is adjacent to
the place above it* is now written in three places:

```
spec/orbit.md    The orbital layer   Two places on the same layer are adjacent when their
                                     territories are, and a territory's surface and its orbit
                                     are adjacent by being layers of one territory
spec/orbit.md    Crossing between    An orbit is next to the territory below it, and next to
                 layers              the orbits above that territory's neighbours
spec/planet.md   Distance            A territory is adjacent to the space above it, and two
                                     spaces are adjacent when the territories below them are
```

**Two of them are in one file, eight lines apart.** `spec/invariants.md` -> A fact is stated once
forbids exactly this, and says which way to resolve it: *the shorter specification is the one that
says each thing once, so removing the second form is better than checking it.*

## What lands

**The first bullet of `Crossing between layers` goes.** It says what the section above it now
says, in a fourth vocabulary - `P-349` settled that *edge, border and boundary name that shared
thing*, and `next to` is anchored to nothing. **The `orbit boundary` bullet beside it stays
untouched**: it defines a boundary a unit may or may not cross, which is a different fact and the
one that section is for.

**And the whole middle paragraph of `spec/planet.md` -> Distance goes** - both its sentences,
because the second is about the first:

```
A territory is adjacent to the space above it, and two spaces are adjacent when the territories
below them are. Neither is a further rule; both are what sharing a boundary comes to when one
place is above another.
```

**The paragraphs around it stay** - *adjacency is a shared boundary*, and *to cross is to pass
through a shared boundary* - because both are about adjacency in general and neither mentions
layers.

## How to tell it was carried out

**`spec/orbit.md` -> Crossing between layers has one bullet**, the `orbit boundary` one, with its
wording unchanged.

**`spec/planet.md` -> Distance contains no occurrence of *above*.** Measured today: two, both in
the paragraph being removed.

**And *next to* appears once in `spec/`, down from twice.** The survivor is
`spec/logistics.md` -> Containment, *a thing says which of the things in it are next to which* -
which is about what a container states, not about places, and is untouched.

## What it costs, and why this is approval rather than a decision

**A reader of `spec/planet.md` alone loses the orbital case.** That is the cost `S-74` could not
resolve, and `P-526` resolves it: the orbital case is now stated in the file about orbits, under a
heading that says so, rather than in a document a reader of orbits has no reason to open.

**So the choice `S-74` was holding open has closed by itself.** It asked whether `spec/orbit.md`
should keep its own sentence or point at `spec/planet.md`; the answer is neither, because the
sentence now lives in `spec/orbit.md` and it is `spec/planet.md` that was restating.

### P-520 - `R-6` says it is built, and eleven lines down says this lane has not recorded it as such

**to** sean · **status** open · **raised** 2026-09-20 · **kind** measured · **shape** text · **asks** approval · **into** `releases/first-release.md` -> R-6

**One item makes both claims.** Found reading the queue for what is open and addressed to you.

```
first-release.md:404   **to** sean - **status** **built** 2026-09-11
first-release.md:434   **The code lane does not set this `built` and this lane has not
                       recorded it as such.**
```

## The measurement is sound and the sentence wrapped around it is about a wording you deleted

At `21deef50` the *vetted when* read *a scenario reaches a fully exploited planet and launches an
Ark*. **Its first half was the fully exploited planet**, and `C-95` measured that it did not hold.

**Then the clause changed and the item did not.** The *vetted when* now says the scenario takes a
first territory from orbit, takes a second by land, launches an Ark, and **does not win** - *and
that is the win condition working*. `R-6` went to `built` in `2c89457d`, thirty-six commits after
`21deef50` and on the same day. **The bullet was never touched.**

**So the numbers in it are still true and both sentences around them are not**: the half it says
fails is a half that no longer exists, and the failure it once recorded is now the capability
working.

**Re-derived here rather than taken from the item**: `{deploy-ark territory:1}` at
`scenario/commands/play.4x:19`, `{found-by-land territory:2}` at `:154`, `{launch-ark territory:1}`
at `:164`.

## What lands

The bullet at lines 430 to 434 becomes:

> - **Measured 2026-09-11.** `C-95`, in `d7ed1e8`: running `setup.4x`, `{start}` and `play.4x` and
>   asking the model gives **twelve claimable territories, two founded, none at maximum output**,
>   with `is_fully_exploited` and `has_won` both false. It does launch an Ark, at line 164 of a
>   133-command scenario. **Measured against a *vetted when* that has since changed** - it asked
>   for a fully exploited planet, and yours asks that the scenario launch and not win, which is
>   what these numbers show.

## What it costs

**Nothing left in the file will say `R-6` was once measured as unbuilt.** That record is in
`21deef50` and in the history, and not in front of a reader of the release. **If you would rather
it stayed visible, the other version keeps both sentences and marks them superseded** - longer, and
it leaves a *not built* in an item whose status line says built.

### P-519 - One clause is stated twice in `spec/console.md`, and the invariants forbid exactly that

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**The same clause is written by hand in two sections of one file.** Found while checking a
quotation the code lane sent in `C-128`, by grepping for the sentence and getting two hits.

```
spec/console.md:63   **A command is named for the recipe it fires**, and there is one command
                     for each recipe the player may fire.
spec/console.md:183  **There is one command for each recipe the player may fire**, and ending
                     a turn fires the world's.
```

**Verbatim, two sections apart** - `The language` and `Commands` - and bolded in one of them.
Neither is derived from the other, so either could be edited without the other noticing.

## The rule it breaks is the specification's own

`spec/invariants.md` -> A fact is stated once: *a second form kept for a reader is generated, and a
check says the two agree*, and **the shorter specification is the one that says each thing once, so
removing the second form is better than checking it.*

**No check could be written here anyway**, because neither form is generated. Removing one is the
only version of the rule available.

## Which one goes is measured rather than chosen

**The `Commands` copy is load-bearing and the other is a trailing clause.** The paragraph directly
under line 183 opens *The commands are therefore not a list this document keeps* - a `therefore`
that reaches back to the clause above it. **Nothing leans on the copy at line 63**; it rides on a
sentence whose subject is how a command is named.

**And a second reader already depends on line 183.** `C-128` quotes it, to say that `owner:world`
against `owner:player` is exactly the command-or-turn division - 26 blocks against 10, re-derived
here from `spec/data/block.4x`. **Deleting the other copy leaves that citation standing.**

## What lands

Lines 63 and 64 become one sentence:

> **A command is named for the recipe it fires.**

## What it costs

**A reader of `The language` alone loses the count.** That section defines the notation and
`Commands` says what the commands are; the clause is a fact about the second. **If you would rather
the language section keep it, the deletion goes the other way** - and then `C-128`'s citation wants
telling.

**Nothing about the game changes**, and no generated file reads either line.
### P-518 - `{name field:value ...}` does not say whether the fields may be none, and seven forms are

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> Commands

**Read against `P-530`, 2026-09-21.** **Its central argument is the one `P-530` invites you to
check.** It says *a test cannot say what a written form means*, and so a clause about empty fields
belongs in prose. **Under an executable specification that is exactly the claim to test**: seven
forms take no fields and every one is exercised. **If a test showing `{end-turn}` parse is enough,
this proposal is unnecessary** - and if it is not, this is a clean example of what prose is still
for.

**One clause, on a case the committed scenario runs fourteen times.**

`spec/console.md` says: *A command is written `{name field:value ...}`. **Its name is one word**,
dashed where it needs more, and its arguments are named.*

**The `...` does not say whether none is allowed**, and the sentence after it presumes arguments.

> **A command may carry no fields at all**, and seven of them do. Its name is the whole of it, and
> `{end-turn}` is a command exactly as `{move unit:scout from:1 to:2}` is.

## What is measured, by the code lane in `C-127`

**Seven of the console grammar's twenty-six forms take no fields** - `start`, `end-turn`,
`show-planet`, `show-orbit`, `show-units`, `show-turn`, `history` - **and every one of the seven is
exercised by a test.** `{end-turn}` appears **fourteen times** in `scenario/commands/`.

**So this is not a hypothetical and never was.**

## Why a test does not already settle it, which is the usual reason not to file

`CLAUDE.md`: *a fact already asserted by a test does not belong in prose too - the test is the
stronger statement.* **That applies to behaviour and this is not behaviour.** The tests assert that
`{end-turn}` parses; **they cannot say what `{name field:value ...}` means**, and a written form is
the one thing a test cannot disambiguate.

**The tell that it needed deciding is that somebody decided it in code.** `crates/command-language`'s
`parse.rs` carries a dedicated failure message - *no fields at all* - for a field given to a form
that takes none. **A branch written on purpose, for a case the specification left to the reader.**

## What it does not say

**Nothing about a minimum in a data file.** The code lane found `{bad}` parsing in its prototype and
this lane declined to propose from it, because that parser has no grammar and this one is
grammar-driven: **whether a command may carry no fields is per form there and global here.** Same
answer, different question. **This clause is about the console grammar and says nothing about what a
`.4x` row may be.**
### P-515 - Publish the shape of the error, not only the correction

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `CLAUDE.md` -> What done means

**`CLAUDE.md` sets a bar for when a habit becomes a rule and this one has cleared it twice in a
day.** *A habit earns its place by a case it caught, not a case it explains*, and *a habit is
written as where to look until it has caught something it did not come from.*

> **Publish the shape of the error, not only the correction.** A reader can apply a shape to their
> own work; a correction is only something to trust. **An account of how a thing was wrong is a
> tool and a fixed number is not** - so when a defect is reported, the report says what the
> instrument asked, not merely what the right answer turned out to be.

## Promoting this one has an extra rule on it, and it is this file's own

**`CLAUDE.md` is the one file where no instance can carry an approval.** Its own words: *an approval
for this file comes from Sean directly*, because **a reader has no way to tell a relayed approval
from an invented one**, and the file being relayed about is the one that says who may write what.
**Facts relay; authority does not.**

**So *promote P-515* has to arrive from you**, in your own message, and not by way of either lane
saying you said it. **Written into the item rather than left to be remembered**, because the moment
it would be easiest to skip is the moment it comes up - which is the code lane's point and it made
it before the moment rather than during it.

## The three cases, and not one was found by anything failing

**One, across a column boundary.** `P-514` reported three unwritable rows of `spec/data/line.4x`
and said how they got past: `P-497`'s check asserted exactly one unrepresentable cell **and looked
only at the Traits column.** The code lane read that, looked at its own generator, and found the
same hole one column over - `lines` wrote `qty` verbatim, so a fourth sentence-quantity would have
gone in as silently as the three did. **Their words**: *I would not have looked if you had written
the fix instead.*

**Two, across a person, an hour later.** This lane had been telling you *215 unpushed*, a figure
measured once and recited, growing by one per commit while somebody pushed in between. It said so
and named the shape - **a count in prose has no way to notice that it has gone stale.** The code
lane ran the command rather than quoting the corrected figure, and it was **forty-one, not forty**:
this lane's own commit had landed between the measurement and the message. **Their words**: *if you
had sent 40 without the confession I would probably have repeated it to Sean.*

**Three, and this one is the rule being used rather than argued for.** `P-519` reported a clause
written twice in `spec/console.md` and **said where both copies were**. The code lane grepped for
the clause, got **one** hit, and had the two lanes disagreeing - then found its own instrument was
the narrow one: line 63 wraps between *for each recipe the* and *player may fire*, so a
line-oriented search cannot see it. Raw 1, normalized 2. **This lane's pattern escaped only by being
short enough to sit on one line**, which is a discipline `CLAUDE.md` already records failing three
times in an hour - **luck, not a better instrument**, and said so rather than claimed as care.

**What makes it evidence for this item**: the disagreement was checkable because the report named
the lines. *The clause is duplicated and I will fix it* gives a reader nothing to re-derive against.

## Why this is not already covered

**`CLAUDE.md` has the diagnosis and not the duty.** It says *a measurement travels with an
explanation of itself, which is not measured*, and *re-derive what you are told* - both addressed to
the **reader**. **This is the writer's half**, and the second case is what shows they are different:
the reader re-derived because the writer had published a shape. **Without the shape there was
nothing to re-derive against.**

## What it costs, stated because it is the argument against

**Accounts are longer than fixes.** `P-514` is longer than the change it proposes, and a queue of
accounts is a queue somebody reads. **Three cases in one day is the evidence, and one day is the
whole of it** - if it stops paying, it is a rule that should come back out.

The working-out is in
[recording a failure found a second one](docs/notes/2026-09-14-recording-a-failure-found-a-second-one.md).
### P-514 - Three rows of `spec/data/line.4x` are unwritable, and this lane's own check said there was one

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**Read against `P-530`, 2026-09-21.** **Same as `P-513`: right about `spec/data/`, and
`spec/data/` is in question.** The defect it reports is real either way - three rows cannot be
read back - so if those files are replaced this is fixed by the replacement rather than by this
proposal. **The third row it could not fix is the part worth keeping**, whatever notation wins.

**`P-497`'s migration wrote three rows that cannot be read back.** Found by the code lane building a
generator against the same data.

```
{line block:work seq:5 role:produce qty:`$where`'s density for that resource kind:resource}
{line block:muster seq:4 role:produce qty:that citizen's strength kind:force}
{line block:stand seq:3 role:produce qty:that unit's strength kind:force}
```

**A value with spaces in it cannot be told from the words after it.** `qty:that citizen's strength
kind:force` reads as a `qty` of `that`, then four words belonging to nothing. **The notation has no
multi-word value and these three rows assume one.**

## And this lane's check reported exactly one such cell

`P-497` asserted that precisely one cell of the release could not be represented - `move`'s *joined
to `$from` by an edge the unit crosses* - and named it so that a second would fail the run.

**It asked only about the Traits column.** Quantities were written straight through without being
classified at all, so three unrepresentable cells passed a check built to catch exactly that, in the
item that introduced the check. **A right answer about the wrong population**, inside the migration
whose whole argument was that counts cannot ask whether each row is right.

## What lands, and the shape is already in the file

`spec/console.md` already has the thing these quantities are: **a path, which reads a trait of
something a name is bound to.** And `P-497`'s `place-line:` already refers to another row of the
same block by its sequence.

> **A quantity that reads a trait names what it reads it from and what it reads.** Where a relation
> writes such a quantity it uses two columns rather than one - which row of the block the thing came
> from, and which trait of it is read. **A value is one word**, and a quantity that needs more than
> one word is more than one fact.

**So the three rows become:**

```
{line block:muster seq:4 role:produce qty-line:2 qty-trait:strength kind:force}
{line block:stand  seq:3 role:produce qty-line:1 qty-trait:strength kind:force}
```

## The third one is not the same and is not fixed here

**`$where`'s density for that resource reads a trait of a place, per resource.** It is not *the
strength of the thing at row two*; it is a density indexed by which resource the block is for.
**Two columns do not hold it**, and this lane is not inventing a third form for one row.

**It is named rather than fixed**, which is the thing `P-497`'s check was supposed to do and did not.

## What this costs you to read

**Nothing in the game changes.** The release's Qty cells are untouched; this is about how the
relational form writes what they already say, and about one row it still cannot.
### P-513 - A relation names its columns, and nothing says so

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**Read against `P-530`, 2026-09-21.** **This may not survive the change in direction.** It says a
relation of `spec/data/` must declare its column order. The thin-engine states the same facts in
its own notation, and whether `spec/data/` survives at all is open - so this is right about the
file it names and the file may go. **Worth answering `P-530` first.**

**`spec/data/` is the source now and nothing states what order its words go in.** `spec/console.md`
fixes the order for a **description** - *`id` first, then every other trait alphabetically, then
`occupied`, `free` and `capacity` last* - and **that rule describes none of the eight relations**.

```
carries      kind trait                          alphabetical would be: kind trait
member       kind family                                                family kind
limit        container contained n                                      contained container n
above        orbit territory                                            orbit territory
block        id recipe owner                                            id owner recipe
line         block seq role qty kind place-bound                        block kind place-bound qty role seq
constraint   block seq trait compare n                                  block compare n seq trait
for          block seq kind                                             block kind seq
```

**Two of the eight happen to match and six do not** - `carries` and `above`, and nothing else -
which is worse than none matching: the rule appears to hold until it is relied on. **This lane first
wrote three**, from the table directly above it, and the code lane re-derived it to two.

> **A relation names its columns, and a row gives them in that order.** The order is the relation's
> own and is stated where the relation is declared. **A description's order is a different rule** -
> it ranks traits because a description has no declaration to name them in.

## What this is for, and it is the guarantee that is missing

`spec/console.md` already says **the same state is always the same bytes**. **That covers a
description and does not reach a relation row**, so two writers of `spec/data/` could disagree about
column order and both be right. **The code lane hit it building a generator**: `Description::ordered`
cannot write these files, because no single ranking gives both `carries` and `constraint` their
order.

## Where the measurement is weaker than the code lane's and stronger overall

**Their example no longer separates the two.** They cited `carries` ordering kind before trait
against `constraint` ordering trait before kind - and `constraint` has **no** `kind` column today,
since `P-511` deleted the only row that had one. **The finding survives the example dying**: the
eight orders above are measured from the files as they stand, and five of them the description rule
gets wrong.

## What it does not do

**It does not choose the orders.** Each relation's is whatever it is declared to be, and the eight
above are what the code lane's generator writes. **This says they must be declared**, not what they
should say.
### P-512 - *Where things are* still says a tank holds fuel, and one row of it changes the game

**to** sean · **status** open · **raised** 2026-09-14 · **kind** entailed · **shape** an instruction · **asks** approval · **into** `releases/first-release.md` -> Where things are

**Read against `P-530`, 2026-09-21.** **Still applies, and its target moved under it.** `P-522`
cut nine recipe blocks and four table rows from `releases/first-release.md`, and *Where things
are* was not one of the sections touched - this lane checked. **Re-read the section whole before
promoting**, which is the trigger `CLAUDE.md` names for a second proposal landing in one section.

**The code lane filed `C-125` and cannot proceed past it.** `P-509` and the release now disagree
about whether a unit's tank holds anything, and the disagreement is load-bearing rather than
verbal.

```
spec/logistics.md      The things in it that can hold that kind contribute capacity and hold nothing

releases/first-release.md -> Where things are
                       | Container     | Holds  | Up to             |
                       | a unit's tank | energy | the unit's fuel   |
```

## What lands

**The section's opening sentence becomes:**

```
Every thing but the game is in another thing, and this release has two sorts of thing that give a
place room.
```

**And the table becomes:**

| Thing         | Gives room for                | Up to           |
| ------------- | ----------------------------- | --------------- |
| a store       | the resource it was built for | 10              |
| a unit's tank | energy                        | the unit's fuel |

**The territory's row goes.** A place does not give itself room - under `P-509` its capacity is the
sum of what is in it, so *a territory holds that kind up to its free capacity for that kind* is the
rule stated as if it were a container.

## Why the heading changes and the row does not

**The row was right and the column was wrong.** *A unit's tank, energy, the unit's fuel* is a true
statement about **capacity** and a false one about holding. **Renaming the column is the whole of the
correction**, and it is why this is an instruction rather than rows: the words offered describe a
table that does not exist yet, so none of them lands as written.

## The thing that is not bookkeeping, and it is yours

**A unit can no longer move out of a place that has no energy.** `P-511` made `move` consume its
energy from `$from`; if a tank holds nothing, the place must have it. **Today a unit moves on fuel it
carries and is refused with `NoCells` when its own tank is empty.**

**It is nearly equivalent and not quite.** A unit hauls energy when it leaves, and that energy joins
the new place - so a unit can still cross an empty territory by bringing fuel, and chain moves on
what it brought. **What changes is that the fuel it brought is the place's**, so anything else
standing there may spend it.

**That is a real change to the game and this lane is not deciding it.** It is the last clause of
`P-509` arriving somewhere visible.

## And it moves something you have already reviewed

**The containment tree stops drawing energy inside a unit.** `P-485` and `S-128` put a pioneer's fuel
there - `{pioneer ...} -> 1` over `{energy} -> 2` - and under pooling that energy is the territory's.
**`scenario/expected/play.4x` changes**, which is the file `R-6` rests on and `P-225`'s protocol says
is reseeded and unreviewed until you read it.

**The code lane has built nothing on this** and says so; it has the reading above and is waiting.

## How to tell it was carried out

**`releases/first-release.md` -> Where things are has two rows and no `Holds` column.** Its heading
row reads `| Thing | Gives room for | Up to |`, the store and the tank rows are present with their
cells unchanged, and **the territory row is gone**.

**And the section says `two sorts of thing that give a place room`**, where it said `three sorts of
capacity`. Three became two by the territory leaving, which is the check that the right row went.

