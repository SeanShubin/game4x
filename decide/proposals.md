# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-602 - A squad groups units, and two global limits replace the berth

**to** sean · **status** open · **raised** 2026-10-01 · **asks** approval · **kind** recovered · **shape** text · **into** `spec/units.md` -> a new section after *What a unit is*

**`berth` is in no prose anywhere in `spec/`** - only in `spec/data/rules.4x`, `spec/data/schema.4x`
and three tests. **So this adds a rule rather than replacing one**, and what it replaces is data.

**Offered as a block, with the heading at `##`:**

> ## Squads
>
> **A squad is a grouping of units.** It is a shorthand, so that a move order reaches several units
> rather than one at a time. **And it is a deliberate limit**: the game caps how many squads I may
> control and how many units may be in one, so that play does not become micromanagement.
>
> **Those two numbers are settings of the game rather than facts about any place in it.** A berth was
> a capacity of a place and a squad limit is not - it holds wherever the units are, and does not
> change because a settlement grew.
>
> **The berth goes when the squad limits arrive.** A place no longer limits how many units may stand
> in it; the two global numbers do.

## What this does not settle, and will not until you say

**Where a game-wide setting lives in the notation.** `planet` was made a thing so the sun had
somewhere to be; a setting might go there, or want something new. **This proposal states the rule and
not the form**, which is why it offers no row.

**Whether a squad is a thing in the state.** A grouping that a move order names has to be nameable,
which points at yes - but it is your call and nothing here assumes it.

**What becomes of `provides` and `consumes`.** They have exactly two users between them and both are
the berth - `{provides kind:place what:berth} -> 6` and `{consumes kind:pioneer what:berth} -> 1`.
**So removing the berth leaves two relations with no users**, which is either a cleanup or a sign they
were always about something more general.

## What lands red

**Three tests name the berth**, one of them by name:
`a-scout-cannot-move-where-every-berth-is-taken`. **Promoting this does not break them** - it states a
rule the data does not yet follow - but the test that asserts the berth and the rule that says it is
gone cannot both be right for long, and clearing that is a reading of yours either way.
### P-601 - Two sets of unit tests, and only one of them decides what the game does

**to** sean · **status** open · **raised** 2026-10-01 · **asks** approval · **kind** recovered · **shape** text · **into** `spec/README.md` -> rule 3, after *a copy of it in `reviewed/` is the record that I have read it*

**The directories exist already** - `spec/tests/rule/` and `spec/tests/interface/`, with the 57 moved
and `reviewed/rule/` beside them - **and rule 3 does not mention either.** It says *a test is written
in `spec/tests/`*, which is still true and no longer says which tests.

**Offered as a block, indented three spaces:**

> **There are two sets of unit tests and they state different kinds of thing.**
> [`spec/tests/rule/`](tests/rule/) states what the game does - rows in, one command, rows out.
> [`spec/tests/interface/`](tests/interface/) states what the interface shows: which items are
> displayed, whether each is active, and which one has my attention.
>
> **Only the first decides what the game does.** The second decides what a player sees, which is a
> separate concern and is why it is a separate directory rather than more files in the same one.
>
> **What has my attention is a property of the interface and names at most one item.** It is not a
> flag on an item, because two items could then hold it.

## Why the third paragraph is in a rule rather than left to the tests

**Because a test cannot state it.** A test over a set of menu items can assert that this one has
attention and that one does not; **nothing in a set of such assertions says that at most one may**.
That is a fact about the interface rather than about any item in it, so it is the kind of thing
`spec/README.md` rule 3 calls *what a test cannot* say.

**And it is the one thing already known to be easy to model wrongly.** Both this lane and the code
lane reached for a per-item boolean first.

## What is not offered

**What an interface test is written in.** The friendly form states game rows and *displayed*,
*active* and *has attention* are not game rows, so the form is open and
`docs/notes/spec-backlog.md` holds it. **This proposal says the two sets exist and what each is
for**, which is what the directories already assert and the specification does not.
### P-600 - What an approval is about, when it survives, and when it is cleared

**to** sean · **status** open · **raised** 2026-10-01 · **asks** approval · **kind** recovered · **shape** text · **into** `spec/README.md` -> rule 3, after *generating from `reviewed/` mean something*

**Your answers today, in one block.** Rule 3 says a record is the thing that says you read a test
and says nothing about what *reading it* was about, which is why a reworded comment unapproves a test
now.

**Offered as a block, indented three spaces to sit inside the list item:**

> **My approval is about what a test says, not where it is or how it is written.** The behaviour is
> every `{...}` row, including a `{load}`, and nothing else - a comment explains and does not decide,
> and `{test name:}` identifies rather than states. **So two tests are the same test when their rows
> say the same thing**, however the text differs: entries coalesced to one per description, traits and
> entries in the order this specification already gives them, whitespace not significant because the
> braces say where a row begins and ends.
>
> **An approval survives a change that does not change the behaviour** - a reworded comment, a
> reordered state, a repadded row. **It is cleared by any change that does**, including a `{load}`
> naming a different file. A cleared approval costs me one reading; a stale one costs me nothing I
> would notice, which is why they are not treated alike.
>
> **An approval is never inferred for a test that has not been given one.** Matching a test to an
> approval it was not given - because it was only renamed, or because it looks like one that was
> approved - is the one failure that is silent, and no convenience is worth it.
>
> **A change to what a test loads, or to the rules it runs against, clears nothing.** If it changed
> what a test means, the test is red and the suite names which ones; if it did not, nothing happened.
> **Clearing is for changes nothing else can see**, and the suite can see all of these.

## What it changes about what exists

**`tools/outbox` compares `read != said`, whole files.** Measured across the 57: **790 comment lines
to 960 rows**, so 45% of what is compared is not what is approved, and
`breeding-does-not-reach-the-citizens-it-just-made` is 27 comment lines to 14 rows. **So this
loosens a comparison rather than tightening one**, and the gate goes red for the code lane until the
comparison follows.

**Nothing in it is new about where authority lies.** Rule 3 already says *where prose and a test
disagree, the test is right and the prose is a defect* - the authority was always in the rows, and
the comparison never knew it.

