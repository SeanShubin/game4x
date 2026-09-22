# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

**They are in [`decide/questions.md`](../../decide/questions.md).** This file keeps the answered
ones, for their reasoning.

## Answered, kept for the reasoning

## The working behind `P-531`, answered `T3` on 2026-09-21

**Not an item, deliberately.** `P-531` is one proposal and it lives in
[`decide/proposals.md`](../../decide/proposals.md); a second heading carrying the same id would
make a cited id resolve to two things, which `tools/outbox` refuses and which is the whole reason
an item lives in one file at a time. **This is the working that produced the answer** - the two
sibling repositories read, the three artifacts and their three questions, the escape-hatch table,
and the workflow reading that made `T3` the one Sean's own sentences described.

**Raised** 2026-09-21 · **answered** 2026-09-21, `T3` · **kind** measured

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

### P-510 - Pooling cannot be universal, and the choice is now two

**to** sean · **status** **answered** 2026-09-14 · `G3`: an orbit holds what its units can hold, and pooling has no exception · **raised** 2026-09-14 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `P-509`, before it is promoted

**`P-509` says a resource is in the place and the things in it hold nothing.** Two lines of the
release say that cannot be true everywhere, and which way it gives is yours.

```
releases/first-release.md:94   An orbit holds units and nothing else.
releases/first-release.md      | move | consume | 1 | energy | | that unit |
```

**A move burns energy the unit carries.** An ark sitting in an orbit is in a place that can hold no
energy, so **there is nothing to pool with** - and if it holds nothing itself, it can never move
again.

## The three ways, and the third is the one that keeps `P-509` whole

**`G1` - a unit holds fuel; everything else pools.**

```
{orbit id:4}
  {ark}  {energy} -> 2

{territory id:1}
  {metal} -> 25              pooled: no transport holds any of it
  {transport resource:metal} -> 2
```

Cargo behaves as `P-509` says and fuel does not. **Two rules for storage**, which is the thing you
said you would rather over-specify than have.

**`G2` - a thing pools what its place can hold, and holds what its place cannot.**

```
{territory id:1}                {orbit id:4}
  {energy} -> 5                   {ark}  {energy} -> 2
  {ark}                           

```

One rule with a condition. **An ark holds fuel in orbit and holds none in a territory**, so the same
thing reads differently in two places and *when does a unit hold something* has no short answer.

**`G3` - an orbit can hold resources, and pooling is universal.**

```
{orbit id:4}
  {energy} -> 2
  {ark} -> 1
```

`releases/first-release.md:94` becomes *an orbit holds units and the resources they carry*, and
**`P-509` needs no exception at all**. The cost is a change to what an orbit is, which is a rule
rather than a shape - and `spec/orbit.md` says only that an orbit *has capacity for no extractors,
and nothing is extracted there*, which does not forbid it.

## The difference, in the notation, and it is two states and one command

**They agree everywhere a place can hold the kind.** In a territory the two are the same bytes, so
every example so far has failed to separate them. **An orbit is where they part.**

## One ark in orbit, carrying two energy

```
G2                          G3
{orbit id:4}                {orbit id:4}
  {ark}                       {ark} -> 1
    {energy} -> 2             {energy} -> 2
```

**Under `G2` the energy is inside the ark; under `G3` it is in the orbit** and the ark's tank is what
gave the orbit the room.

## Two arks, one with two energy and one with one

```
G2                          G3
{orbit id:4}                {orbit id:4}
  {ark}                       {ark} -> 2
    {energy} -> 1             {energy} -> 3
  {ark}
    {energy} -> 2
```

**`G2` has two entries and `G3` has one.** Under `G2` they share a description and are told apart
only by their contents, which is the whole of what `P-507` could not write and `P-508` needed
positions for. **Under `G3` there is nothing to tell apart** - two arks, three energy, and a
capacity of four.

## And the command that cannot be written

```
G2   {move unit:???  to:5 haul-energy:2}    which ark? both are `{ark}`
G3   {move unit:ark  to:5 haul-energy:2}    either; they are the same
```

**That is the cost of `G2` in one line.** Not that orbits are odd, but that **the problem this week
was spent on comes back inside them.**

## The same ark, before and after landing

```
G2                                    G3
{orbit id:4}                          {orbit id:4}
  {ark}                                 {ark} -> 1
    {energy} -> 2                       {energy} -> 2

{move unit:ark to:1 haul-energy:2}

{territory id:1}                      {territory id:1}
  {energy} -> 7                         {energy} -> 7
  {ark} -> 1                            {ark} -> 1
```

**Both end in the same place.** Under `G2` the ark's contents emptied into the pool on arrival and
under `G3` nothing happened, because they were pooled already. **`G2` is a thing that changes shape
depending on where it stands; `G3` is a thing that never holds anything.**

## `G1` is eliminated, 2026-09-14, by Sean's own constraint

**Sean**: *I don't want the gas tank to be special in mechanics, only in defaults.*

**`G1` is exactly that specialness.** Fuel held and cargo pooled is two mechanics, chosen by which
resource it is. **Withdrawn**, and not on taste - it is the one thing he ruled out by name.

**`G2` and `G3` both survive it**, because neither is about the resource: `G2` branches on the
place, `G3` branches on nothing. **A gas tank under either is a store like any other, and the only
thing left that is fuel-shaped is a default** - *haul most* topping one off because you were picking
up.

## `G3` needs no capacity declared, which this lane did not see the first time

**Under pooling a place's capacity is the sum of what is in it that can hold the kind.** An orbit
holding an ark whose fuel store is 2 **already has an energy capacity of 2**, derived, with nothing
declared:

```
{orbit id:4}
  {energy} -> 2
  {ark} -> 1        its fuel store is what gives the orbit the room
```

**An empty orbit holds nothing because it can hold nothing** - capacity 0, no rule needed. So the
release's *an orbit holds units and nothing else* becomes *an orbit holds units, and what they can
hold*, and **nothing anywhere declares an orbit a store.**

## Sean's 500 / 500 / 10 case, which `P-509` already gets right

```
{territory id:1}                       {territory id:2}
  {metal} -> 500                         {metal} -> 0
  {store resource:metal} -> 50           {store resource:metal} -> 50
  {transport resource:metal} -> 1
```

```
{move unit:transport to:2 haul-metal:10}
```

**Ten, and only ten, however much room is at either end.** `P-509` bounds a haul by *its own capacity
for that kind* and by nothing else, so the destination having 500 free changes nothing. **No option
here affects it** - it is `P-509`'s rule rather than this choice.

## One consequence of pooling worth seeing before you choose

**Capacity can walk away.** A territory with 500 fixed storage and a transport standing in it has a
capacity of 510. If it holds 505 and the transport leaves carrying nothing:

```
{territory id:1}
  {metal} -> 505      capacity is now 500, so five is lost at the turn's end
```

**That is correct rather than a bug** - the five had nowhere to be - and it is exactly what the
waste check is for. **It is also a thing a player can do to themselves by accident**, which is the
argument for warning on it.

## This lane's reading, and it is not confident

**`G3`, and more strongly than before.** It is the only one under which *a resource is in the place*
has no exceptions; the sentence it changes is a release's rather than the specification's; and its
capacity turns out to be derived rather than declared, so **it adds no rule at all** - it removes
one.

**`G2` is the live alternative** and its cost is now clearer: an ark in orbit holds its own contents,
so everything `P-507` and `P-508` were built to solve - describing a container by what it holds,
telling two alike things apart - **comes back, in orbits only.** One place where the rules differ is
the whole of what `G2` buys over `G3`.

## And one smaller thing `P-509` left open, which the same choice settles

**`spec/logistics.md`: *a rule may ask whether something is absent only where what would hold it
declares a limit for it*** - and a territory declares no limit for a resource.

**Under `P-509` the limit stops being about the container and becomes about the place**: what a place
holds beyond the capacity in it is lost, and that capacity is a number the place has. **So the
territory does declare a limit** - a derived one - and the sentence is satisfied rather than
strained. `free metal of {territory id:1}` then means what it says.

**That holds under all three**, so it is settled by `P-509` landing rather than by this choice. It is
written here because `P-509` names it as open and this is the answer to it.
*Nothing is open.*

### P-507 - A description cannot say what a thing holds, so only an empty container can be named

**to** sean · **status** **answered** 2026-09-14 · `E4`, and `P-508` carries it concretely · **raised** 2026-09-14 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `spec/console.md` -> The language, and `P-501` before it is promoted

**You asked how `repeat:7` finds the empty transport, and then asked the question that breaks it.**
The first has an answer. The second does not, and `P-501` is held until it does.

## Why the first one works

`P-501` says a description used to name is **exact** - it leaves out no trait **and no content**. So
after

```
{stow into:{transport metal} kind:metal repeat:3}
```

that transport holds `{metal} -> 3` and **is no longer named by `{transport metal}`**, which states
no contents and therefore names things holding nothing. The second command has one candidate left.

## Why the second one does not

**One transport holding 1 metal, one holding 2, and you want 2 into the first and 5 into the
second.** Both are non-empty, so both must be named by their contents - and **the game notation has
no way to write contents inline.**

Containment is written by indentation, in a state file:

```
{transport resource:metal}
  {metal} -> 1
```

**A command is one line.** `P-212` lets a value be another command - `{a b:{c d:1}}` - but that is a
**named argument**, and contents are a map from a description to a quantity with no name to hang on.
**So there is no expression for *the transport holding one metal*.**

**`P-501` is therefore incomplete rather than wrong.** It works for exactly one case - the empty
container - and your example is the first that is not it.

## Three ways, and the choice is which

**`E1` - a command carries a tree by indentation, as a state does.**

```
{stow kind:metal repeat:2}
  into {transport resource:metal}
    {metal} -> 1
```

**One notation, and containment written the one way it is already written.** The cost: a command
stops being a line, and `scenario/commands/*.4x` is line-oriented today.

**`E2` - contents as a named argument.**

```
{stow into:{transport resource:metal holding:{metal}:1} kind:metal repeat:2}
```

Stays on one line. The cost: `holding` is a word the notation does not have, and a thing holding two
kinds needs it twice, which is a map wearing a field's clothes.

**`E3` - the entry form inline, in brackets.**

```
{stow into:{transport resource:metal [{metal} -> 1]} kind:metal repeat:2}
```

Stays on one line and reuses `->`. The cost: `[` and `]` are punctuation the notation does not have,
which is a second way of writing containment.

## `E4` - name the entry by its position, and write no contents at all

**Sean, 2026-09-14**: *for manual allocation we will likely be able to select individual transports,
which implies all we really need is a positional notation, to know what is in each transport, and to
be able to add/remove from a transport by position.*

```
{stow into:2 kind:metal repeat:2}
{stow into:3 kind:metal repeat:5}
```

**A position is an entry's index in the order the state already puts it in.** `spec/console.md`
already fixes that order - *entries are in the order their descriptions sort in* - so **nothing is
invented and no contents are written.** `P-502`, as corrected today, makes the order total where two
entries share a description.

**It is not a fourth identity, it is the second one indexed.** `id` names a thing for ever; a
description names a set; **a position names an entry in a container's own listing**, and it changes
when the state changes because the listing does.

## Why `E4` beats the other three, and it is a different kind of argument

**The other three invent a way to write contents. `E4` writes none.** `E1` makes a command
multi-line, `E2` adds a word, `E3` adds punctuation - and all three restate what a container already
displays. **A position points at it instead.**

**It is also the only one that does not grow with what a thing holds.** A transport holding four
kinds needs four contents clauses under `E2` or `E3`, and one number under `E4`.

## What Sean raised against it, and it is the real limit

**Stacking.** *How are we to have massive fleets if I have to make each one selectable by the user.*

**A position names an entry, not a thing**, so a million identical transports are **one entry at one
position**. Positions do not grow with the fleet. **They grow with the number of distinct
(description, contents) combinations**, which is the honest bound and is the one he named: *an
additional problem when we have more possible combinations of contents than can fit on a user
interface.*

**This lane has not bounded that number and does not know it.** What can be said is that it is the
number of distinct states, which `spec/logistics.md` already relies on being small - *a kind has few
states however many things of it there are.* **That sentence is load-bearing for `E4` and was
written before anything held cargo**, so it is a premise to check rather than a reassurance.

## And the simple algorithm he named needs no addressing at all

*Fill up each transport and move the ones that are full.* **That is `repeat` against a description
that names the not-yet-full ones**, and the set shrinks as they fill - `P-501`'s mechanism, no
position required. **So position is for manual allocation only**, which is the case it was proposed
for.

## What this lane would pick and why it is not confident

**`E4`, and this lane changed its mind on being given the position idea.** It argued for `E1` on the
grounds that *there is one notation* and a state writes containment by indentation. **`E4` is better
by that same rule**: it writes no containment at all, so there is nothing to write a second way.

`E1` remains the answer if a command should be able to name a thing that is **not** in a listing
anybody has - a hypothetical container, or one being created. **Nothing in the three scenarios needs
that.**

**The reason this is a decision and not an approval**: the cost of `E1` falls on files you derive by
hand this week, and whether a command may stop being one line is a judgement about your own reading
rather than about the notation.
*Nothing is open.*

### P-499 - `P-497` refused: two columns it treats as atomic are not, and the counts passed anyway

**to** sean · **status** **answered** 2026-09-14 · `J1` for the constraint, and `P-500` for the `Where` column · **raised** 2026-09-13 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `P-497`, before it is promoted

**You said promote `P-497` and this lane refused it rather than landing a mangled migration.**
`CLAUDE.md` gives two ways out of a promotion that breaks something, and this is the first: say what
has to be decided first.

**It was found by carrying the migration out.** All seven files were written, **every count matched
`P-497` exactly - 45, 7, 5, 37, 96, 29, 6 - and the natural key was unique over the 37 blocks.**
The check the instruction carries passed in full, and two of the relations were wrong.

**That is the failure this repository keeps recording, committed by the check written to prevent
it.** A row count asks *did every row arrive*; it cannot ask *is each row right*. The instrument
answered a narrower question than the one asked and returned a plausible number.

## One - a constraint that names a kind has nowhere to put it

`refuel`'s qualifier is `free energy at least 1`, which **you chose this morning** so that a bin
could say *free of what*. Decomposed by `P-497`'s schema, which has `trait` and `compare` and
nothing else:

```
{constraint block:refuel seq:2 trait:free compare:energy-at-least-1}
```

**The kind has been swallowed into the comparison.** `energy-at-least-1` is not a comparison; it is
a kind and a comparison run together, and a reader cannot get `energy` back out without parsing a
string. **The E1 form has three parts and the relation has two.**

**What it wants is a fourth column** - `{constraint block:refuel seq:2 trait:free kind:energy
compare:at-least n:1}` - which also splits `at-least-1` into a comparison and a number, and those
are two facts as well. **Whether `n` is its own column is the same question one level down.**

## Two - `Where` holds three different things, and this lane slugified them

**Seven distinct values, and they are not one kind of thing:**

| The cell                                   | What it is                                      |
| ------------------------------------------ | ----------------------------------------------- |
| `` `$where` ``, `` `$from` ``, `` `$to` `` | a reference to something the command bound      |
| `that unit`                                | a reference to an ingredient of this same block |
| `the orbit above `$where``                 | a place **derived** from another place          |
| `a store for energy`, `a store for metal`  | a place **described** by what it holds          |

The migration wrote the third as `place:the-orbit-above-where`, **which is prose flattened into an
identifier and means nothing**. The same for `a store for metal`.

**This is the Traits column's own finding, one column over**, and this item did not look for it -
it found the Traits column by decomposing and took `Where` on trust.

## What this lane is not doing

**Not guessing the shape.** Both are the same question - *what are the parts of this cell* - and
`P-497` answered it for Traits by measurement and for `Where` by assumption. **The measurement for
`Where` is above; the shape is yours**, as the notation is.

## What it does not block

**The five relations that came out clean** - `carries` 45, `member` 7, `limit` 5, `block` 37,
`for` 6 - are unaffected by either question. **`line` is affected only in its `place` column**, and
`constraint` only in how a qualifier splits. So this is two columns, not a rethink.
*Nothing is open.*
