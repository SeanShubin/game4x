# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

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

