# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-532 - Fifty-three tests have to cross a column boundary, and nothing may carry them

**to** sean · **status** answered · **answered** 2026-09-21, `M2` · **raised** 2026-09-21 · **kind** entailed · **into** `spec/tests/`, `reviewed/`, and the prototype

**Filed the moment `P-530` and `P-531` landed**, which is what both said would happen.

**Answered `M2` on 2026-09-21: he moves them himself.** The scripts are asked of the code lane,
because `scripts/` is their column, and he runs them from the project root. **Nothing here is a
proposal** - there is no text to promote, only a move, so this item closes when the directories
exist rather than becoming words for him to approve.

**What the answer resolves, and it is the thing this item said was impossible.** `CLAUDE.md` now
says a record is added and removed only by the review application acting as Sean; **a relocation
is neither, so no instance may carry it.** `M2` resolves it because he is not an instance. **The
scripts are written by a lane and run by him**, which is the whole of the difference.

**Measured for the move, 2026-09-21:**

```
prototypes/thin-engine/data/friendly/tests/   54 .4x, 244K  ->  spec/tests/
prototypes/thin-engine/reviewed/              52 .4x, 232K  ->  reviewed/
```

**`data/foundation/` does not move**: `tests/directories.rs` opens with *`data/friendly/` is the
source and `data/foundation/` is what it converts to*, his words of 2026-09-15 - so it is
generated, and `CLAUDE.md` says a generated file has no owner.

**And five files stay behind that are not tests** - `engine.4x`, `rules.4x`, `schema.4x`,
`script.4x`, `setup.4x`. **They are the game's data and the engine's declaration**, and
`spec/data/` already holds twelve files of the older relational form. **Two representations of one
game in one directory is a decision**, and `P-517` is the open question about which survives.

**The two unreviewed tests are `a-pioneer-settles-the-ground-it-is-standing-on` and
`a-second-settlement-launches-the-ark-the-first-could-not`**, both from the code lane's pioneer
work. **Fifty-four tests against fifty-two records is correct** and is the number to expect on the
other side.

**`CLAUDE.md` now says a test lives in `spec/tests/` and the suite runs the copies in
`reviewed/`. Neither directory exists.** The fifty-three tests you have read are in
`prototypes/thin-engine/data/foundation/tests`, and the records of your reading them are in
`prototypes/thin-engine/reviewed` - **both in the code lane's column, and `spec/` is this lane's.**

```
prototypes/thin-engine/data/foundation/tests/*.4x   53   code lane's column
prototypes/thin-engine/reviewed/*.4x                52   code lane's column
```

## Why nobody can simply do it

**`hooks/pre-commit` refuses a commit that spans two columns**, and a move is a delete in one and
an add in the other. **So the move is two commits by two lanes**, or one by whoever you say.

**And the records are the harder half.** A test file is just bytes; **a record is the evidence
that you read something**, and it is the one artifact `CLAUDE.md` now says no instance may write.
**Copying fifty-two of them is writing them**, which is exactly what the rule forbids - so this
cannot be done under the rule it is implementing.

## The three ways

**`M1` - they stay where they are, and the prototype becomes the home.** `spec/tests/` is a name
for `prototypes/thin-engine/data/foundation/tests`, reached by whatever runs it. **No move, no
records copied, and the rule is satisfied by renaming a column rather than moving a file.** The
cost is that the tests sit inside a directory called `prototypes`, which says the opposite of what
they now are.

**`M2` - you move them.** The rule says no instance writes `reviewed/`; **you are not an
instance.** A copy by hand, or a one-off run of the review application against the new location,
and the records are yours from the first byte. The cost is your afternoon.

**`M3` - the records are rebuilt rather than moved.** Nothing is copied; you re-approve fifty-three
tests through the application in its new home. **The strongest, because every record is then one
you made under the rule**, and the most expensive - it is the reading you have already done, done
again.

## What this lane would say

**`M1` now and `M2` or `M3` when the application moves**, because the thing that makes the tests
awkward where they are is a directory name rather than anything real. **The prototype has stopped
being a prototype** - its question was answered, its differences table is empty, and what is left
in it is the specification and the tool that reads it.

**But `M1` needs one thing to be honest**: `prototypes/README.md` says what a prototype is, and a
directory holding the specification is not one. **Renaming it is a bigger change than this
proposal** and it is the shape the answer probably takes.

## What follows either way, and it is not small

**Rule 3 now says a rule the tests assert is not written in prose as well.** Seventeen documents
in `spec/` state rules that fifty-three tests assert. **Nothing is duplicated today**, because no
test is in `spec/` yet - **the moment one is, that rule starts cutting**, and working out which
prose goes is a read this lane will file as a plan rather than as one change.

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
