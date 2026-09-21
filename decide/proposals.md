# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-526 - An orbit is part of a territory rather than a place beside it

**to** sean · **status** open · **raised** 2026-09-20 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/orbit.md` -> The orbital layer

**Your words, 2026-09-20, recorded in the prototype's backlog:**

```
I no longer think orbits should be separate territories with adjacencies like they are in
the mainline spec. I think orbits should be part of the territory. So we need a way to tell
if something is in orbit, and the orbital area will have different containment rules than
the surface.
```

## What the specification says today, and how much of it already agrees

`spec/orbit.md` already says an orbit is **not** a territory - *it has capacity for no
extractors, and nothing is extracted there* - and `spec/console.md` says *the orbit above a
territory is named by naming the territory*. **So the disagreement is narrower than it sounds**:
what changes is that an orbit stops being a second place in the adjacency graph.

Two of your answers settle the rest. **Can a citizen be in orbit? No.** **Does orbit match the
surface graph? Yes** - so no orbital adjacency is stated, because it is already derivable.

## What lands

The section becomes:

> - **A territory has two layers: its surface and its orbit.** A place is one layer of one
>   territory, and a thing is in orbit by being in the orbital place of the territory it is
>   above. **Nothing is in orbit without being above a particular territory.**
> - **Adjacency is stated between territories and nowhere else.** Two places on the same layer
>   are adjacent when their territories are, and a territory's surface and its orbit are
>   adjacent by being layers of one territory. Neither is a further rule.
> - **The layers do not admit the same things.** What may stand in a layer, and how much room a
>   layer gives, are declared per layer. An orbit admits no extractor and no citizen.

## What it does not settle, and it is the one you left open

**Whether crossing between layers is a move.** Your own answer was *thematically it is a move in
the sense of changing position, but also the moves are very different kinds of things* - so the
text above says which places are adjacent and says nothing about which capability a crossing
demands. `spec/orbit.md` -> Crossing between layers keeps its `orbit boundary` rule untouched.

## What goes stale, and it is one sentence

`spec/planet.md` -> Distance says *a territory is adjacent to the space above it, and two spaces
are adjacent when the territories below them are*. **That survives as a derivation and stops
being a statement about two kinds of place.** If this lands, the cleanup is filed against that
sentence rather than left.

### P-525 - A place declares no capacity of its own, and the prototype has two that do

**to** sean · **status** open · **raised** 2026-09-20 · **kind** measured · **shape** text · **asks** approval · **into** `spec/logistics.md` -> Containment

**One sentence of the specification is false of the model you approved fifty-two tests of.**

```
spec/logistics.md   A place's capacity for a kind is the sum of what is in it that can hold
                    that kind, and a place declares none of its own. This holds of every place
```

**Measured in the prototype's data, two rows contradict it:**

```
{capacity of:place   for:bin       ...}                       a territory's room for bins
{capacity of:deposit for:extractor what:resource per:place}   a deposit's room for extractors
```

## Why the sentence was right when it was written, and is wrong now

**It was written when every container was a thing somebody built.** A bin gives room for metal, a
tank gives room for fuel, and the place is only the sum - that is still exactly true of
resources, and the rule is worth keeping for them.

**What broke it is ground.** A deposit is not a thing anybody put there, and it bounds how many
extractors may stand on it; a territory's room for bins is the same shape. **Neither is the sum
of anything inside**, and both arrived by folding a separate mechanism away rather than by adding
one: the prototype's `limit` relation - two columns, three engine words, a check and two failure
modes - all went when the deposit limit became a capacity row.

## What lands

The bullet becomes:

> - **A place's capacity for a kind is the sum of what is in it that can hold that kind**, and
>   for a kind a place can hold, a place declares none of its own. **What a place has room to
>   stand is a different question**, and a place does declare that: ground states how many of a
>   kind may be built on it, and nothing inside it changes the number. **This holds of every
>   place**: an orbit has room for the fuel its units carry and for nothing else, because that is
>   what is in it.

## What it costs

**Two questions now wear one word.** *How much of this may this place hold* and *how many of
these may stand here* are both capacity, answered by one table, and the text above is what tells
a reader which is which. **The alternative is a second word for the second question** - which is
the mechanism the prototype just removed, and removing it is what let a deployment fall short
where there is no metal deposit instead of being refused outright.

### P-524 - An ark gathers its fuel and spends it, where the spec says orbit is free

**to** sean · **status** open · **raised** 2026-09-20 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/units.md` -> What a unit is

**Your words, 2026-09-20, stating the ark whole:**

```
Ark - can only be in orbit; can spend one energy and one move to move from orbit to orbit;
contains a storage container for 1 energy; can collect 1 energy from the sun each turn
```

**What the specification says:**

```
spec/units.md   A mobile unit that moves in orbit takes its energy directly from the sun.
                It stores no fuel, and moving costs it nothing
```

## What free movement was for, and what replaced it

**Both clauses were written before anything gathered.** *Takes its energy directly from the sun*
had no mechanism, so *moving costs it nothing* was the only way to say that orbital movement is
not paid for out of a territory. **The prototype built the mechanism** - a unit spends a per-turn
allowance and makes a resource, which is the rule that already works an extractor - and once the
sun is a source, free movement stops being what says so.

**A thing that gathers and spends is a game; a thing that moves for free is a rule.** The ark now
has a bin, a rate and a cost, so an ark that has spent its fuel this turn is something the player
can see.

## What lands

The bullet becomes:

> - **A mobile unit that moves in orbit gathers its energy from the sun**, a fixed amount each
>   turn, and holds it in a bin of its own. **Moving in orbit burns a unit of it**, and one with
>   an empty bin cannot move. **The sun is where that energy comes from**, so orbital movement is
>   never paid for out of a territory.

## What it does not change

**The bullet above it is untouched** - a unit that moves over the ground is still built with its
bin full, and the energy is still paid where it is built. **What the two now share is the
shape**: a bin, a cost per move, and a refusal when the bin is empty. They differ in where the
fuel comes from, which is the fact the old sentence was carrying.

### P-523 - What is offered is an end result, and the thing that offers it is not a rule

**to** sean · **status** open · **raised** 2026-09-20 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/interface.md` -> a new section, *What is offered*, after *What an action shows*

**Your words, 2026-09-20**, the last two of the five things you said you wanted promoted:

```
We are going to want to limit offers to end results, so we don't offer spending labor, the
user decided to build a pioneer, or build a storage bin, or move a unit. Spending labor
happens automatically to pay labor costs, and at end of turn if there is space in storage
bins the extractors work automatically to fill them, automatically spending labor and
exhausting citizens to generating labor as needed.

The options on offer is going to be a separate layer from the rules engine, there are no
rules to automatically top off storage bins, this is a policy layer that generates the
proper commands to execute player wishes and automatically do obvious tasks.
```

## What lands

A new section, after *What an action shows*:

> **What the player is offered is an end result and not a step towards one.** Building a Pioneer,
> building a bin and moving a unit are offered; spending the labour they cost is not. **What a
> choice costs is paid by whatever the rules require**, without being chosen a second time.
>
> **And what is obvious is done without being asked.** Ending a turn is the player saying they
> have finished choosing, so at that point every extractor with somewhere to put what it makes is
> worked - spending labour, and spending citizens to make labour, as far as it will go.
>
> **None of this is a rule of the game.** There is no rule that tops off a bin. What is offered,
> and what is done unasked, is a layer above the rules that writes the commands a player would
> have written. **The rules say what is legal; this says what is worth showing.**

## Why the last paragraph is in the specification at all, given that it is about the artifact

**Because the negative is a fact about the game.** *There is no rule that tops off a bin* is a
statement about what the rule set contains, and a reader of `spec/` who found topping-off
happening would otherwise go looking for the rule that did it. **The layer itself is
architecture**, and it is being written into `docs/architecture.md` as the eleventh rule there,
which needs no approval from you.

## What the code lane offers as a fact, and it removes the worry you had

**Nothing competes.** Your own settling of it: *the user will decide on actions, when pressing end
turn they have decided no more actions, so thats when topping off storage occurs, no possibility
of competition.* **And the seam already exists and is already enforced** - the prototype's
isolation test refuses any noun the game's data names from appearing in engine code that runs, so
a policy that must name bins, extractors and labour cannot drift into the engine even by
accident.

### P-522 - The first release is the loop the prototype closed, and the cuts are most of it

**to** sean · **status** open · **raised** 2026-09-20 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `releases/first-release.md`

**Your words, 2026-09-20**, the first, second and fourth of the five:

```
Generate a game board from a 12 space goldberg polyhedron.
Start me out with an ark in orbit
Cut many features from first release, no nature, no biomes, no force
```

**And the reason, said to the prototype**: *I am cutting nature and force from this prototype
because I don't think they are necessary to vet the core game loop: start with an ark -> develop
a planet -> launch an ark.*

## What the cuts come to, measured in the file rather than estimated

| Section              | Today | After | What goes                                                                               |
| -------------------- | ----- | ----- | --------------------------------------------------------------------------------------- |
| Kinds                | 19    | 16    | `garrison`, `nature`, `force`                                                           |
| Traits               | 26    | 23    | `defending`, `met`, `biome`                                                             |
| What bounds a kind   | 12    | 11    | `garrison`                                                                              |
| Units and structures | 7     | 6     | `garrison`                                                                              |
| Recipes, as blocks   | 36    | 27    | `muster`, `stand`, `hold`, `reclaim`, `take`, one `renew`, one `discard`, two `refresh` |
| Biomes               | 1     | 0     | the whole section                                                                       |

**The two `refresh` blocks that go are the ones restoring `defending`**, leaving four; the `renew`
and the `discard` are the ones naming `nature` and `force`. **And `deploy ark` and `found by land`
each lose two rows** - a `require force` and a `produce garrison`. Every count above was read off
the tables as they stand today.

## What replaces *Scope* and *The loop*

**Scope** keeps its first line and the rule editor's exclusion, and gains:

```
The twelve territories and the thirty adjacencies between them are generated from the
twelve-faced Goldberg polyhedron rather than stated by hand.
```

**The loop** becomes three steps in place of today's seven:

```
1. Start with an ark in orbit
2. Land it, and develop the territory it lands on
3. Reach a second territory, build a Yard there, and launch an ark from it
```

**Measured: the board is 12 territories, 30 adjacencies and 24 places. I read *board* as those
rows and not as the resource table** - *Territory resources* keeps its twelve rows and their
densities, because nothing you said touches them and the balance note rests on them. **If you
meant the resources generated too, say so and that table goes as well.**

## An *Out of scope* section, which this release has never had

`releases/README.md` asks for one - *whole areas of the spec this release does not touch, so the
omission reads as deliberate* - and this is the first release with whole areas to name: nature,
biomes and force, each with your reason. **`spec/` keeps all three**; this is scheduling and not a
change to the game.

## What this depends on, and what it leaves alone

**It depends on `P-521`.** A release may not state a win condition the specification does not
have, so the win condition lands in `spec/control.md` first and the release points at it.

**`R-1` through `R-5` stay vetted and are not reopened.** `R-4` recorded a biome per territory in
a drawing that exists; cutting biomes is about the rules, not about work already observed. **If
you read that differently, `R-4` is the one to say so about.**

**`P-512` is also open against this file**, in *Where things are*, and this instruction does not
touch that section. If both land, that section is re-read whole before either closes.

## How to tell it was carried out

**The six counts in the table above**, read off the file after the edit. **No `## Biomes`
heading.** **An `## Out of scope` section naming nature, biomes and force.** **`The loop` has
three steps.** And **`Scope` names the polyhedron.**

**One thing is deliberately left in, and it is worth your eye.** The `Strength` column of *Units
and structures* survives, and with force gone nothing in the release reads it. It stays because
`spec/units.md` says every unit has a strength and `spec/combat.md` is where it is spent. **If you
want it cut too, that is one more column.**

### P-521 - The win condition is two settlements, not a finished planet

**to** sean · **status** open · **raised** 2026-09-20 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/control.md` -> Winning

**Your words, 2026-09-20**, opening *I think we have proven enough to promote this to the main
spec, here is what I want to do*:

```
Win condition is to deploy my ark to one space and launch an ark from a different space
```

**What the specification says today:**

```
spec/control.md   A player wins by launching an Ark from a fully exploited planet.
```

## What lands

That bullet becomes:

> - A player wins by deploying an Ark to one territory and launching an Ark from a different one.

## What this is instead of, and why it is the harder condition to fake

**The old condition is a state of the whole planet** - every territory taken, every territory at
its greatest output, every store full. **The new one is two acts in two places**, which cannot be
reached without a second settlement, and a second settlement cannot be reached without crossing
ground and founding. **It tests the loop rather than the totals.**

**It is also reachable, which the old one measured as not being.** `C-95` ran the committed
scenario and got twelve claimable territories, two founded and none at maximum output - so `R-6`'s
scenario launches an Ark and does not win. **Under this condition it wins**, because it deploys to
territory 1 and founds territory 2.

## What goes stale, and the cleanup is filed rather than folded in

**Four bullets above it define *fully exploited*, and nothing in `spec/` would then read them.**
Measured: the phrase appears twice in `spec/`, in the definition and in the bullet being replaced.
**Outside `spec/` it has readers** - `R-6`'s evidence, and `is_fully_exploited` across
`crates/game-model` and `crates/game-console`, including a test named for it.

**So they are not deleted here.** The definition is still true, and still the thing `C-9` made
decidable from a territory alone; what it loses is the rule that used it. **A cleanup proposal is
filed the moment this lands**, asking whether the definition stays as vocabulary or goes.

## What it does not change

**Losing is untouched** - no citizens and nothing that becomes one. **And nothing about force**:
taking and holding ground read as they do, and this release cutting force is `P-522` rather than
this.

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

