# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-531 - Where the tests live, and the question it turns out to be instead

**to** sean · **status** open · **raised** 2026-09-21 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `P-530`, and wherever the tests end up

**`P-530` left this open and you asked to puzzle it through.** Having read `../vote`'s scenario
test and `../code-structure`'s regression test, **the *where* turns out to follow from something
else**, and that something else is a fact measured today.

## First, the thing you already have

**You described two kinds of test and this repository already has both shapes.**

```
behavioural, thin-engine   data/foundation/tests/*.4x    {given} {when} {then} {refused}
regression, code-structure scenario/commands/*.4x        commands, and a frozen dump beside them
                           scenario/expected/*.4x
```

**`../code-structure`'s `RegressionTest` and this repository's scenario are the same mechanism.**
Both run everything over one input, freeze the output, and compare directory against directory;
both seed the expectation from the actual run the first time. `P-225` already gives you the
protocol for changing your mind about it - **delete the expected data, and absence means
acceptance.** So the regression half is designed and running, and this item is about the other
half.

## The three artifacts answer three questions, and none of them is a copy

**This is your last question answered first**, because the rest depends on it.

| artifact                  | canonical for                 | authored, or derived          |
| ------------------------- | ----------------------------- | ----------------------------- |
| a behavioural test        | **is this right?**            | authored - you say the `then` |
| a frozen expected output  | **did anything change?**      | derived - seeded from a run   |
| the record in `reviewed/` | **which version did I read?** | derived - from your keypress  |

**A frozen expected output cannot be evidence of correctness and it is important that it is not
asked to be.** `../code-structure` proves it in one function: `seedExpectationIfNecessary` copies
`actual/` into `expected/` when `expected/` is absent. **Nobody wrote those 291 files.** They
answer *did this change* perfectly and *is this right* not at all.

**An authored `then` answers both**, which is why it is the primary statement and the frozen dump
is not. **So there is no copy of one fact in two places** - there are three artifacts, and asking
any of them the other's question is the mistake.

## Where each belongs, and only one of the three is hard

**The runner is code.** It names no game noun; `tests/isolation.rs` already enforces that.

**The frozen output is generated**, and `CLAUDE.md` says a generated file has no owner. It sits
beside the scenario and is reseeded deliberately.

**The behavioural test is the hard one**, and it is hard for the reason you named: executable
leans code, looked-at leans specification. **But `spec/data/*.4x` already settles that shape** -
data in `spec/`, engine in `crates/` - so *executable* does not argue for `crates/` at all. A
runner reaching a file is not the same as the file living beside the runner.

## So the real question is not where, it is which mechanism holds your approval

**Two mechanisms exist and they give the same guarantee differently.**

**Promotion prevents.** The specification lane may not introduce an idea into `spec/`; you say
*promote* and the words are copied verbatim. **Checked once, at the moment of copying.**

**`review-web` detects.** A test is drafted, you read it, you press `r`, and a byte-for-byte copy
lands in `reviewed/`. **Checked on every build, forever** - which is strictly stronger, *if
anything fails when the two disagree.*

**Measured today: nothing does.** `review_of` lives in `examples/report.rs`, so drift is shown on
a page and gates nothing. `tests/reviewed.rs` refuses a record naming no test - the code lane
built it this morning - and no test in the suite compares a test's content to the copy you
approved.

**That is the whole decision.** If drift turns the build red, an approved test cannot be changed
under you without the gate saying so, and the tests are safe in a column a producer drafts into.
If it does not, only promotion protects them, and they must live in `spec/`.

## The three answers

**`T1` - the tests live in `spec/`, promotion-gated, runner in `crates/`.** The strongest
prevention and the shape `spec/data/` already has. **The cost is `review-web`**: every new test
becomes a proposal you read in a queue rather than a page you press a key on, and there are
fifty-three of them already.

**`T2` - the tests live in a column of their own, `reviewed/` is the approval record, and drift
fails the gate.** Keeps the workflow that convinced you. **The cost is that prevention becomes
detection** - a producer can edit an approved test, and what stops it shipping is a red build
rather than a rule.

**`T3` - the approved copy is the specification, and the working copy is a proposal.** The suite
runs `reviewed/`; `data/.../tests/` is where a draft sits until you have read it. **Drift stops
being a defect and becomes an unpromoted proposal**, which is the protocol you already have with
`review-web` as its interface. **The cost is that a test takes effect only when you have read
it**, so a fix the code lane makes to a test it wrote is inert until you look - which is either
exactly right or intolerable, and that is yours.

## What this lane would say

**`T3`, and it is the one the evidence points at rather than the one that was obvious.** It is
`CLAUDE.md`'s promotion protocol with a keypress instead of a sentence and a check on every build
instead of one at the moment of copying. **It also makes the *where* stop mattering**: the draft
can live in the code lane's column because a draft is not normative, and the normative copy is
the one your reading created.

**What it needs before it could be chosen is one test**, in either column: that the suite runs the
approved copies, or fails when a working copy has drifted from one. **`T2` needs the same test.**
Only `T1` needs no new mechanism, which is the honest argument for it.

## Sean asked whether `T3` means the code lane implements what he has reviewed and ignores the rest

**Half right, and the wrong half is a hazard this item had not named.**

**What is right**: only a reviewed test compels. A test you have not read cannot turn the build
red, so it cannot force the code lane to build anything - which is the whole point, and is the
same guarantee `promote` gives.

**What is wrong is *ignores*.** The code lane **writes** the drafts, so it cannot ignore them; it
authored them. **But as `T3` was stated above, an unreviewed test compels nothing and reports
nothing** - and that is a way to hide a failure. **A draft that goes red could simply never be put
in front of you**, and the gate would be green, and the page would say *never reviewed* in the
same tone it uses for a draft written five minutes ago.

**So `T3` needs a second half: an unreviewed test still runs and still reports; it just does not
gate.** Reviewed tests gate. Then nothing is hidden, and only what you approved compels.

```
reviewed     runs, and a failure turns the build red
unreviewed   runs, and a failure is reported and counted
```

**The count already exists.** `report.rs` carries `reviewed` and `unreviewed` - today fifty-three
tests, fifty-two reviewed, one not. **What is missing is that the number reaches you** rather than
sitting on a page: a growing unreviewed count is the shape of work being shelved, and it is
invisible in a queue that only shows proposals.

## And the cost of `T3` this makes visible, which is the one to weigh

**A reviewed test the code lane cannot make pass leaves the gate red until you look.** The test is
the specification saying no; the fix is either the code changing or a changed test you re-read.
**Either way your reading is on the critical path**, which is `R-6` today generalised to every
test.

**`T1` has the same property and says so more plainly** - a proposal you have not read is a rule
that does not exist. **The difference is only how many times a week it happens**, and with
fifty-three tests and counting, that is the number worth guessing before choosing.

## Sean's requirement: once reviewed, an error state until the code behaves that way

**All three give you that, and the requirement selects on something none of them names.** Each
runs the reviewed test and each turns the build red when it fails. **What separates them is the
escape hatch** - whether the red can be cleared without the code changing.

**And the mechanism already allows the state you are describing.** Measured: `review-web`'s
`/reviewed` copies the test with no check that it passes, so you can approve a test that is not
built. **Approval is a statement of intent, and red is the correct answer until it is met.**

|          | how the red could be cleared without the code changing                                                                                      |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| **`T1`** | edit or delete the test in `spec/` - **which the code lane may not write at all**, and `hooks/pre-commit` refuses a commit spanning columns |
| **`T2`** | edit the test **and** the record - editing the test alone makes it drift, which is also red                                                 |
| **`T3`** | edit the record - editing the test is inert, because the suite runs the approved copy                                                       |

**So the question your requirement actually asks is: who may write the approval record?**

**`T1` answers it for free**, because `spec/` already has that property and a hook already
enforces it.

**`T2` and `T3` answer it only if `reviewed/` moves.** Today it is
`prototypes/thin-engine/reviewed/`, which `hooks/pre-commit` maps to the **code lane's** column -
**so today, the lane whose work the test constrains may edit the record of your approval.** That
is the hole, and it is not in where the tests live.

**`T3` is one moving part better than `T2`.** Under `T2` the escape is two edits and under `T3` it
is one, because editing a test that nothing runs achieves nothing. **Neither is safe while the
record is theirs; both are as strong as `T1` the moment it is not.**

## What would have to be true, and it is small

**`reviewed/` sits in a column no instance writes**, like `temporary-notes/` but tracked -
written by `review-web` running on your machine when you press a key, and by nothing else.
**`hooks/pre-commit` has to learn that column**, which is the code lane's file and one `case`
line; an unrecognised path is currently *unassigned*, which is listed in a refusal and never
causes one.

**Then `T3` gives you your requirement with the same strength as `T1`** and keeps the page you
review from. **Choosing `T1` gets it today and costs `review-web`; choosing `T3` gets it after one
line in a hook and one decision about where the record lives.**

## The workflow you described is `T3`, and it moves one thing in the analysis above

**Two of your sentences settle what was open.** *I expect the spec lane to convert my prose into
tests for me to review* says who drafts, and *I want some kind of application to present what I
need to review* says the approval is made in a page rather than in a queue. **That is `T3` with
the specification lane holding the pen.**

**And it changes the escape hatch, in your favour.** The table above assumed the code lane drafts
the tests. **If this lane drafts them and the code lane is the one they constrain, the code lane
cannot clear a red by editing a test at all** - it is not their column, and the hook refuses it.

**What that leaves is a narrower risk and a worse one.** This lane would then both write the tests
and own the directory they sit in, so **the only thing separating *Claude wrote this* from *Sean
approved this* is the record in `reviewed/`.** It follows that the record cannot be this lane's
either - **not the code lane's because they are constrained by it, and not this lane's because it
writes what the record is about.**

**Which also says who builds the application**, and it is not this lane. An instrument that shows
you this lane's work, and that writes the record of your approving it, is the one thing this lane
must not control - the same reason a lens may not edit what it reviews. **The code lane builds it,
which is where `review-web` already is.**

## The part nothing here can do yet, and it is your last sentence

**Bulk approval after a spot check is safe exactly when *no other kinds of changes* is measured
rather than assumed**, and today it is neither - it is not even askable.

**What exists**: per test, a status and a list of lines that differ, each marked *what you read* or
*not what you read*. **What does not exist is a sentence about the whole set** - *these forty
differ only in this one mechanical way, and no test differs in any other way, and here is the
count of each*.

**Without that, a spot check of three is evidence about three.** `CLAUDE.md` already has the rule
this is an instance of: *check the rule over every case, not on one case, and assert how many
cases there were.* **The count is what tells a real sweep from a lucky sample.**

**It is also exactly what this lane did to your release and got right.** `P-522` predicted the
shape of its own change before making it - nine recipe blocks, eight table rows, one section - and
the counts were re-derived after. **The same instrument applied to a hundred drifted tests is what
makes *mark all reviewed* an act rather than a hope.**

**So the application has a third job**, beside presenting and filtering: **classify a change set
and assert its population.** Group the drifted tests by what kind of difference they carry, show
the count of each group, and show that the groups cover every drifted test. **Then a spot check of
three in a group of forty is a claim about forty**, and *no other kinds of changes* is something
you read rather than something you hope.

## What this lane would now say, given the workflow

**`T3`, and the three homes fall out of your own sentences rather than from a preference.**

```
the draft test        the specification lane's column - it writes them from your prose
the approval record   a column no instance writes - it is the only thing that says you read it
the application       the code lane's - it must not be built by the lane whose work it shows
```

**And one requirement that is not about homes at all**: until the application can classify a change
set and assert its population, **bulk approval should be one test at a time**, because that is the
only scale at which *no other kinds of changes* is something you have checked.

### P-529 - Three lines of the release still name what `P-522` cut, and one of them is a vetted capability

**to** sean · **status** open · **raised** 2026-09-21 · **kind** entailed · **shape** an instruction · **asks** a decision · **into** `releases/first-release.md`

**Filed the moment `P-522` landed.** Its instruction named six tables and a section and carried
them out exactly; **it did not name these three, and they survive saying things the release no
longer has.** Measured by grepping the file after the edit.

```
:14   Scope    A mobile unit may move across a boundary, usually to conquer and start
               another self-contained territory
:63   Kinds    territory | a place things are in, which has a biome, a force of nature, and
               a density and a capacity per resource
:329  R-4      A biome per territory - vetted 2026-09-03
```

## Two of them are wording and one is not

**Line 14 and line 63 are the easy half.** *Conquer* is force, and a territory's *biome* and
*force of nature* are two of the three cuts, sitting in a cell describing what a territory is.
**Neither states a rule the release still has**, so both are stale text rather than open
questions - and this lane could fix them as rephrasing if they were only rephrasing, which they
are not: removing *conquer* changes what the bullet says a unit moves across a boundary **for**.

**`R-4` is the one that is yours.** It is `vetted`, on 2026-09-03, against a drawing that exists.
Nothing about the drawing has changed. **But the release now says biomes are out of scope and
carries a vetted capability that delivered them**, which is a file disagreeing with itself.

## The three answers for `R-4`

**`B1` - leave it vetted and say why.** Work observed is work observed; add one line to `R-4`
saying biomes were delivered before the cut and the cut is about rules rather than about the
drawing. **The release stays honest about its own history.**

**`B2` - move it to the log.** `releases/README.md` -> Shipped is where a delivered capability
goes, and the log is empty. **The cost is that the log is meant for whole releases**, not for one
capability leaving early.

**`B3` - cut it with the rest.** **The cost is that it deletes the record of something a person
looked at and approved**, which is the one kind of evidence this process treats as final.

## What this lane would say

**`B1`.** It is the only one of the three that does not lose information, and the disagreement is
between a scope statement and a history, which a sentence can resolve. **`B2` and `B3` both
answer a bookkeeping question by discarding an observation**, and observations are the scarce
thing here.

## And the two wording lines, once you have said

```
- A mobile unit may move across a boundary to start another self-contained territory
```

```
| **territory** | a place things are in, which has a density and a capacity per resource |
```

**Offered as words rather than carried out**, because the first changes what the bullet claims a
move is for, and `CLAUDE.md` says where a change would alter what a line claims, raise it rather
than make it.

### P-527 - *Fully exploited* is defined in four bullets and now read by nothing in `spec/`

**to** sean · **status** open · **raised** 2026-09-21 · **kind** entailed · **shape** an instruction · **asks** a decision · **into** `spec/control.md` -> Winning

**Filed the moment `P-521` landed**, which is what that proposal said would happen.

`spec/control.md` -> Winning now opens with four bullets defining *fully exploited* and closes
with a win condition that does not use the phrase. **Measured after the promotion: the phrase
appears once in `spec/`, in its own definition.**

```
A planet is fully exploited when every territory that can be taken has been taken, every
territory is producing the greatest output it can, and every storage structure on it is full.
Exploiting a territory is putting labor to work at its extractors. ...
What that greatest output is follows from the territory's own permanent facts ...
A territory that cannot feed a citizen has no output to reach ...
```

## Why this is a decision and not a tidy-up

**The definition is still true and something still reads it.** `is_fully_exploited` is built
across `crates/game-model` and `crates/game-console`, with a test named for it, and `R-6`'s
evidence quotes the old win condition to explain why the committed scenario does not win.
**Deleting the words does not delete any of that.**

## The three answers

**`W1` - keep all four, as vocabulary.** *Fully exploited* stays a defined term the game can use
later - for scoring, for an end-of-game report, for a second win condition. **The cost is four
bullets at the top of a section whose subject is now something else**, and a reader who looks for
what reads them and finds nothing.

**`W2` - cut all four.** The section becomes one bullet and says exactly what winning is.
**The cost is the built code**, which then implements a term the specification does not define,
and `R-6`'s evidence stops resolving.

**`W3` - move them, and keep them.** They are a statement about a territory's output rather than
about winning, so they would sit in `spec/economy.md` or `spec/planet.md`. **The cost is a move
that has to pick a home**, and this lane will not pick one without you.

## What this lane would say

**`W3` if the term survives your new direction, `W2` if it does not** - and that is the part
this lane cannot see. Under an executable specification the question is whether a test will ever
assert *the planet is fully exploited*; if none will, the words are prose with no reader and `W2`
is honest. **You are the one who knows whether that test is coming.**

### P-517 - `spec/data/` states the cases, and the rules are what you wanted to read

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `spec/data/`, and `C-114`

**Read against `P-530`, 2026-09-21.** **This is the one the change in direction most likely
retires.** It asks whether `spec/data/` should state four rules with their cases or thirty-six
blocks. If the tests become the primary statement, the question is about a file that may not exist
- **and the underlying observation still holds**: fifteen blocks that are four rules is a fact
about the game, and the prototype folded exactly this kind of repetition away.

**Neither of your first two, and it is measurable rather than a matter of blame.** Normalizing did
not make this complexity. **It removed the layout that was hiding it.**

## What the data actually holds

```
{block id:refresh-citizen-bearing    recipe:refresh owner:world}
{block id:refresh-citizen-defending  recipe:refresh owner:world}
{block id:refresh-citizen-laboring   recipe:refresh owner:world}
{block id:refresh-extractor-working  recipe:refresh owner:world}
{block id:refresh-unit-defending     recipe:refresh owner:world}
{block id:refresh-unit-moving        recipe:refresh owner:world}
```

**Six blocks, identical in every respect but a `(kind, trait)` pair.** They are one rule - *put that
count back at its maximum* - written out six times.

| Recipe      | Blocks | Differing only in |
| ----------- | ------ | ----------------- |
| **refresh** | 6      | `(kind, trait)`   |
| **discard** | 5      | `kind`            |
| **stow**    | 2      | `kind`            |
| **renew**   | 2      | `(kind, trait)`   |

**Fifteen blocks of thirty-six are four rules and their cases.** The other twenty-one are each the
only one of their name.

## And the repository already has the word for it

`docs/designing-rules.md`, about `reports/nogain.md`: **a family becomes its members, a density
becomes its cases.** That describes the unfolding `nogain` does **in order to check**, which means
the folded form is the one it thinks of as the rules. **The data is the unfolded form.**

## Why this is not `P-497`'s doing and not yours

**The release was already unfolded.** `releases/first-release.md` -> Recipes has six `refresh`
blocks and five `discard` blocks, and has had since long before `spec/data/` existed. **`P-497`
transcribed faithfully, which is what a migration should do.**

**What changed is that the layout stopped hiding it.** In the markdown table the six `refresh` blocks
are six rows among ninety-two, separated by blank continuation cells, and read as one paragraph of a
long table. **As rows they are six things with six names, and six is a number you can see.**

**So the complexity was always there and was always the release's.** You asked for relational; what
arrived is relational and correct; **and the first thing it showed you is a thing worth knowing.**

## The decision

**`F1` - state the rules and let the reader unfold.** One `refresh` block with six cases, one
`discard` with five. **Thirty-six blocks become twenty-five**, and `block` stops needing a written id
for the twenty-one that are the only one of their name.

**`F2` - state the cases, as now.** Every block stands alone and nothing has to be unfolded to be
read. **The cost is that `refresh` is six things and a reader must notice they are one.**

## What this lane would say, and it is less a recommendation than a connection

**`F1` is `C-114`'s engine argued from the data side.** Your own words there: *if the data itself
explodes in complexity, that tells us something needs to be unified.* **Fifteen blocks that are four
rules is that reading, taken off the instrument you asked for.**

**But folding requires the engine to unfold**, and nothing reads these files at run time yet. **So
`F1` is not a change to `spec/data/` that stands on its own** - it is the first half of the
restructuring you have not decided, and this lane would rather name that than smuggle it in as
tidying.
### P-516 - Moving resources: put or consume-and-produce, and how the fuel says who burnt it

**to** sean · **status** open · **raised** 2026-09-14 · **kind** invented · **shape** an instruction · **asks** a decision · **into** `releases/first-release.md` -> Recipes, and `spec/console.md` if the notation moves

**Read against `P-530`, 2026-09-21.** **The notation question may have been answered elsewhere.**
This asks whether moving a resource is a `put` or a consume-and-produce, in the release's Recipes
table and `spec/console.md`. The thin-engine has built `move` and has its own answer. **Before
deciding `M1` to `M3` and `A` to `D`, it is worth asking what the prototype does and whether this
question is now about a notation being retired.**

**You are declaring resources with the unit and letting the interface default.** The command is the
same under every option below:

```
{move unit:transport from:1 to:2 metal:7 energy:2}
```

## First, the thing you were worried about is not a problem

**Energy not balancing is correct and checked.** `spec/invariants.md`: *no sequence of rules ends
holding **more** than it began with.* **Ending with less is not what that forbids.** A move burns
fuel, the star is where fuel comes from, and the arithmetic that would fail is a rule that **made**
energy from nothing.

**So no option here has to make energy balance**, and one that did would be hiding the cost rather
than paying it.

## Second, and this lane had it wrong until Sean pushed on it

**The first version of this item said: `put` for what has an `id`, `consume` and `produce` for what
is counted.** Sean: *I am not so sure put applies to units anymore... if a 100 identical transports
are moving 1000 resources it is not clear that there is any substantive difference between the units
and the resources.*

**He is right, and the measurement is worse than he put it.**

```
kinds carrying an `id`        territory, orbit
put rows on either of them    0 of 17
```

`releases/first-release.md` justifies `put` as *the same thing and not a new one, **so what has an
identity keeps it***. **That sentence is true of no row in the game.** Every put is on a `unit`, a
`citizen`, an `extractor`, a `thing` or a `nature`, and not one of those carries an `id`.

## What `put` is actually for, which is in the code and not in the release

`crates/game-console/src/petri.rs`: **a count is a place of its own, and the kind's own place is
untouched.** *A citizen that spends its `laboring` is the same citizen afterwards, so the arc is on
`citizen laboring` and nothing goes in or out of `citizen`.* **Drawing it on the kind instead would
show `create labor` eating a citizen.**

**So `put` is about a state change not reading as a destruction**, and has nothing to do with
identity. The release names the wrong reason, and names it in the one place a reader would look.

## And that sharpens where the line falls

```
put rows that change a state, in place     16
put rows that change a place               1     move, `moving one less` at `$to`
```

> **A `put` is a change of state where the thing already is. A change of place is a `consume` where
> it was and a `produce` where it is.**

**Sixteen of seventeen already obey that.** The exception is `move`, which crosses to `$to` with a
put - and it is the row Sean was looking at when he said a unit and a resource are not different.
**They are not.** Both are counted things changing place, so both are consumed at one end and
produced at the other:

| Recipe   | Owner  | Role    | Qty | Kind | Traits            | Where   |
| -------- | ------ | ------- | --- | ---- | ----------------- | ------- |
| **move** | player | consume | 1   | unit | moving at least 1 | `$from` |
|          |        | produce | 1   | unit | moving one less   | `$to`   |

**`put` then never names a place**, which is a check a tool can make.

## `M1` - consume at one end and produce at the other

| Recipe   | Owner  | Role    | Qty | Kind   | Traits            | Where   |
| -------- | ------ | ------- | --- | ------ | ----------------- | ------- |
| **move** | player | require | 1   | unit   | moving at least 1 | `$from` |
|          |        | put     |     | unit   | moving one less   | `$to`   |
|          |        | consume | 7   | metal  |                   | `$from` |
|          |        | produce | 7   | metal  |                   | `$to`   |
|          |        | consume | 1   | energy |                   | `$from` |

**The metal balances inside the rule and the energy does not, and the reader can see which is
which.** The cost of `M1` is that seven is a constant in a rule, and `spec/invariants.md` says *a
rule's amounts are constants* - so a haul of seven and a haul of three are two rules, or one rule
fired seven times and three times.

## `M2` - give `put` a quantity and a place

```
|          |        | put     | 7   | metal  |                   | `$to`   |
```

**One row instead of two**, and it reads as what it is: the same metal, elsewhere. **It costs the
sentence that says a put has no quantity**, and that sentence is what currently tells a reader that
`put` never creates anything. **Relaxing it is a change to the notation, not to this recipe.**

## `M3` - say nothing, because `P-509` already moved it

**Rows mention no metal at all.** `spec/logistics.md`, promoted: *a unit moving out of a place is
given an amount of each kind, no more than its own capacity for that kind, and that amount joins the
number the new place holds.* **The haul is a containment rule and the command's argument feeds it.**

**The cost is that the net cannot see it.** `reports/petri.md` draws what the rows say, so a haul
would be invisible to the drawing and to the weighting - **which is safe, because moving conserves,
and blind, because nothing would catch a haul that did not.**

## And three ways to say whose fuel it was

**`E1` - as now.** `consume 1 energy $from`. **Nothing says the vehicle paid**; a reader infers it
from the rule being `move`.

**`E2` - the consume names the payer.** The row points at the unit's row, the way `P-514` proposes a
quantity point at a row: `qty-line:` for a quantity, and something like `by-line:` for a cost.
**Explicit, and a new column for one use.**

**`E3` - the move is free and being ready costs.** `move` spends `moving one less` and consumes
nothing; `refresh` pays for putting it back:

| Recipe      | Owner | Role    | Qty | Kind   | Traits                | Where |
| ----------- | ----- | ------- | --- | ------ | --------------------- | ----- |
| **refresh** | world | require | 1   | unit   |                       |       |
|             |       | consume | 1   | energy |                       |       |
|             |       | put     |     | unit   | moving at its maximum |       |

**Then the energy is visibly what a vehicle burns to be able to move**, rather than a toll on the
move itself, and `move` becomes purely a change of place. **The cost is that it is paid at the
turn's end rather than when you move**, so a unit moves on credit and is charged later - and a unit
that moved into a place with no energy stops being ready rather than being refused.

## The four, in game notation

**The command is the same under all four.**

```
{move unit:transport from:1 to:2 metal:7 energy:2}
```

**And writing them this way found something the relational tables hid** - it is the section below
this one.

**`A` - a place change is a consume and a produce, whatever is moving. `move` pays.**

```
{block id:move recipe:move owner:player}
{line block:move seq:1 role:require qty:1 kind:place place-bound:from}
{line block:move seq:2 role:require qty:1 kind:place place-bound:to}
{line block:move seq:3 role:consume qty:1 kind:unit place-bound:from}
{constraint block:move seq:3 trait:moving compare:at-least n:1}
{line block:move seq:4 role:produce qty:1 kind:unit place-bound:to}
{constraint block:move seq:4 trait:moving compare:exactly n:0}
{line block:move seq:5 role:consume qty:7 kind:metal place-bound:from}
{line block:move seq:6 role:produce qty:7 kind:metal place-bound:to}
{line block:move seq:7 role:consume qty:1 kind:energy place-bound:from}
```

**`B` - `A` without `seq:7`, and `refresh` pays instead. RECOMMENDED.**

```
{block id:refresh-unit-moving recipe:refresh owner:world}
{line block:refresh-unit-moving seq:1 role:require qty:1 kind:unit}
{line block:refresh-unit-moving seq:2 role:consume qty:1 kind:energy}
{line block:refresh-unit-moving seq:3 role:put kind:unit}
{constraint block:refresh-unit-moving seq:3 trait:moving compare:at-maximum}
```

**`C` - the unit is put and the cargo is not. Closest to today.**

```
{line block:move seq:3 role:require qty:1 kind:unit place-bound:from}
{constraint block:move seq:3 trait:moving compare:at-least n:1}
{line block:move seq:4 role:put kind:unit place-bound:to}
{constraint block:move seq:4 trait:moving compare:one-less}
{line block:move seq:5 role:consume qty:7 kind:metal place-bound:from}
{line block:move seq:6 role:produce qty:7 kind:metal place-bound:to}
{line block:move seq:7 role:consume qty:1 kind:energy place-bound:from}
```

**`D` - a put may carry a quantity, and everything moving is put.**

```
{line block:move seq:4 role:put qty:1 kind:unit place-bound:to}
{line block:move seq:5 role:put qty:7 kind:metal place-bound:to}
{line block:move seq:6 role:consume qty:1 kind:energy place-bound:from}
```

## What the notation showed and the tables did not

**`compare:one-less` cannot sit on a `produce` row.** *One less* is relative, and a produced thing is
a new token - **there is nothing for it to be one less than.** Under `A` and `B` it has to become
`compare:exactly n:0`, which is what the blocks above say.

**That works here only because `moving` is 0 or 1.** `docs/designing-rules.md` measured it: *moving
is 0 or 1 and `move` requires moving at least 1*. **So the exact value is writable for this game and
not in general** - a counter with a range would need the produced row to refer to the consumed one,
and nothing in the relation can say that.

**`C` and `D` do not have the problem**, because a put changes a thing that is already there and
*one less* has its referent. **That is a real point for `C` and `D` that the tables did not show**,
and it is the first argument against the recommendation that came from the data rather than from
taste.

## What separates them, in one line each

|         |                                                                                                                                                                           |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`A`** | a unit and a resource are the same kind of thing, which is what you said. Energy is still a toll on the move and nothing says who paid                                    |
| **`B`** | the same, and the fuel is visibly what a vehicle burns to be ready. **`put` then never names a place**, which a tool can check                                            |
| **`C`** | keeps `put` for the unit on a justification the release states and no row satisfies - zero of seventeen puts are on a kind carrying an `id`. **Keeps `one-less` working** |
| **`D`** | one row per thing moved, and it costs the sentence *a put has no quantity*, which is what currently tells a reader that a put makes nothing                               |

## What `B` costs, said plainly

**The charge lands at the turn's end rather than when you move.** A unit moves on credit; a unit
that moved into a place with no energy **stops being ready** next turn rather than being refused
this one. **That is a different game, not a different notation**, and it is the part of `B` that is
yours rather than this lane's.

**And it does not help with scale.** `qty:1` on the unit row moves one, so a hundred transports is
still `repeat:100` under every option here. **Your hundred-transports observation is answered as a
question about sameness and not as one about firing**, and the second is still open.

## What this lane would pick

**`B`, and less confidently than an hour ago.** A change of place is not a change of state, and the
net has two places to draw it between; the fuel stops needing an explanation once what is bought is
readiness rather than distance.

**What weakened it is the `one-less` finding**, and it came from writing the blocks in game notation
rather than as tables. `B` can only say *one less* as *exactly zero*, and that is writable because
`moving` is 0 or 1 today. **A trait with a range would break it and nothing in the relation could
say what the produced thing is one less than.**

**So `C` is the answer if you expect a counter with a range**, and `B` if you do not. **This lane
does not know which**, and that is a question about the game rather than about the notation.

**`M2` is the one to take if the seven matters more than the sentence**, and that is a judgement
about how often a haul will be a constant rather than a repeat.
*Nothing is open.*
