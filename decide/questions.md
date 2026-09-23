# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-540 - A notation for the interface, and three candidates with your two tests written in each

**to** sean · **status** open · **raised** 2026-09-22 · **kind** invented · **shape** an instruction · **asks** a decision · **into** a new notation, and `spec/tests/`

**You said to use a precise notation like the game rules use, and the game notation does not
reach this yet.** A test today is `{given}` rows, a `{when}` command and `{then}` rows, where
every row is a thing in a place. **A screen is things in positions, which is the same shape with a
different idea of *where***.

**Your two tests below, written three ways.** They are the same two either way; what differs is
what a row says.

## `U1` - a screen is a list of things, and position is a trait

```
{test name:the-first-screen-offers-three-things}

{given}
{game phase:none}

{then}
{menu at:centre}
{button of:menu seq:1 label:new-game}
{button of:menu seq:2 label:exit-game}
{status-bar at:bottom lines:1}
```

**`seq` is the order down the screen and `at` is the region.** Nothing says pixels. **A menu
button's size is not stated** because your rule is that they are all the same, which is a check
over the set rather than a fact per button.

**What it costs**: *centre*, *bottom* and *bottom-right* become a closed list of regions, which is
a new vocabulary the game notation does not have.

## `U2` - a screen is a tree, and containment does the placing

```
{test name:the-first-screen-offers-three-things}

{given}
{game phase:none}

{then}
{screen}
{region of:screen name:centre}
{region of:screen name:status-bar}
{menu of:centre}
{button of:menu seq:1 label:new-game}
{button of:menu seq:2 label:exit-game}
{line of:status-bar} -> 1
```

**This is the containment the game already uses** - *every thing but the game is in another thing*
- so `rooming`, `stands-in` and the capacity checks apply unchanged. **A status bar with two lines
fails the way a territory with two densities fails.**

**What it costs**: three rows to say what `U1` says in one, and every test carries the screen's
skeleton.

## `U3` - a screen is what the player can reach, and layout is checked once and never stated

```
{test name:the-first-screen-offers-three-things}

{given}
{game phase:none}

{then}
{offered label:new-game}
{offered label:exit-game}
{tooltip for:new-game}
```

**Nothing about position appears in a test at all.** Your three layout rules become three checks
over every screen: a status bar of one line at the bottom, game actions bottom-right, menu items
centred and equal. **A test says what is reachable and the checks say where things sit.**

**What it costs**: a test cannot say *this button is in the wrong place*, because no test mentions
place. **You would review reachability and trust the layout checks**, which is the opposite of
what you have been doing with the rules.

## What your draft already decides, whichever wins

**Load game is absent rather than disabled.** *This menu item absent if none exists* - so the
first screen has two buttons or three, and **the test is two tests**, not one with a condition.

**And the planet sizes are not unknown.** Your draft has `?` for four of them; `spec/planet.md`
states all five: **tiny 12, small 32, medium 42, large 72, huge 92.** The `?`s are answerable from
the specification rather than by you.

## What this lane would say

**`U2`, and not because it is prettiest.** It is the only one of the three where **a layout
mistake fails a test you have read**, rather than failing a check somebody wrote. Your whole
reason for the executable specification was that a test you approved is a piece of the game you
are certain of; `U3` moves the layout out of that set, and `U1` states position without being able
to say what contains what.

**The cost of `U2` is real and is verbosity**, and the answer to it is the answer you already gave
for rules: a family. `{region of:screen name:...}` repeated in every test is the same shape as
`{territory id:1}` repeated in every game test, and nobody has minded that.

## Scoped to capabilities, 2026-09-22, and four named regions

**You narrowed it**: *I am thinking of scoping the tests to capabilities rather than visual
positioning*, with four regions - `status-bar`, `menu`, `actions`, `main-screen` - and *I would
have to specify the visual layout another way.*

**That is `U3` with the regions named**, which is better than `U3` was: a test can say *this is in
the menu* without saying where the menu is. **`U1` and `U2` are answered and this section is the
live question.**

## Your sketch needs three things the notation does not have, and one it forbids

```
{ menu [ new-game exit-game ] }          a list
{status-bar "start new game"}            a quoted sentence, and an unnamed argument
```

**One - nothing in a data file is quoted.** `spec/console.md`, and it is not a convention:
**measured, there is not one quoted string in any data row anywhere in the repository.** The only
quotes are inside `#` comments. A name is `start-new-game`, dash-joined, because *where it needs
more than one word, the words are joined with dashes*.

**Two - every argument is named.** `{status-bar "start new game"}` has a value and no key, and the
rule is *its arguments are named*. What a value is has to be readable from the key beside it.

**Three - there is no list.** `[ new-game exit-game ]` is a form the notation has never carried.
**What it has instead is one row per fact**, which is how `{member kind:ark family:unit}` says a
family's membership rather than listing it.

**A tree it does have**: *a value is a word, a number, or another command in the same form*, so
your first sketch's nesting is legal and your second sketch's list is not.

## Your test, written in the notation as it stands

```
{test name:hovering-new-game-explains-it}

{given}
{menu}
{item of:menu seq:1 name:new-game}
{item of:menu seq:2 name:exit-game}

{when}
{hover item:new-game}

{then}
{status-bar shows:new-game}
```

**`shows:new-game` rather than a sentence.** The status bar shows *the tooltip of the thing named*,
and **where the words live is the question your sketch raises and does not answer.**

## So the real question is where the words live, and there are two answers

**`W1` - the words are not in the specification at all.** A test says `{status-bar
shows:new-game}` and the English lives wherever the interface keeps its text. **A test then
cannot be wrong about the wording**, only about which tooltip appears.

**`W2` - prose becomes a thing the notation carries**, with a form for it, and `spec/console.md`'s
*nothing in a data file is quoted* gains an exception. **Then a test can assert the sentence**, and
you review the words you will actually read on screen.

**`W2` is what your sketch reaches for** - you wrote the English out, twice - and it is the larger
change: it touches the notation every lane reads, not just the interface.

## What this lane would say, and it is less certain than the last recommendation

**`W1` for the first screens and `W2` only if the words turn out to matter to you.** A tooltip is
the one part of an interface whose wrongness is obvious on sight and harmless in the moment - you
will see a bad tooltip the first time you hover, and nothing downstream depends on it.

**Against that**: *this will provide tool tips for where the cursor is hovering or what the player
needs to do* is the status bar's whole purpose, and a capability-scoped test that cannot say what
the bar said has given up on the thing the bar is for. **That is the argument for `W2` and it is
yours to weigh, not this lane's.**

## One notation or two, 2026-09-22, and your precedent is narrower than it reads

**You said it**: *it may need to be a different notation, balancing the need for consistency with
the convenience of using the best tool for the job.*

**`spec/console.md` opens by forbidding that**, and the sentence is yours:

```
There is one notation. A command and a description of game state are written in the same form.
The notation is the game notation. A command, a state and a data file are written in it, and
there is nothing else to write them in.
```

**But the reason given for it is about something else.** Four lines down: *two notations would
cost more to maintain and more to read than one, and a command that takes a tree is expected
rather than hypothetical.* **That argument is flat-versus-tree inside one notation** - whether
simple commands get a simpler form - and it was settled by making everything carry a tree.
**It is not an argument about a second notation for a different domain, and this lane will not
pretend it is.**

**What does bind is the broad clause**: *a command, a state and a data file are written in it, and
there is nothing else to write them in.* **An interface test would live in `spec/tests/` as a
`.4x` file**, which makes it a data file, which makes it bound. **So the rule reaches this whether
or not its stated reason does.**

## What a second notation costs, measured

```
193   .4x files in the tree
  7   modules that parse or render the notation
        command-language/parse.rs      game-console/petri.rs   game-console/state.rs
        thin-engine/engine.rs          thin-engine/schema.rs   thin-engine/script.rs
        thin-engine/store.rs
  1   review application, which reads tests through report.rs
```

**The review application is the cost that matters and it is not the parser.** `scripts/review.sh`
shows you a test, records that you read it, and compares the current test with the copy you
approved. **A second notation means either a second review path or one app that reads both** - and
the app is the thing your whole executive control rests on.

## Three answers, and the middle one is new

**`N1` - one notation, extended.** Give it a form for prose and nothing else; lists stay one row
per fact. **The smallest change that lets an interface test say what the status bar said**, and a
UI test and a game test sit in one directory, read by one app, compared the same way.

**`N2` - two notations.** The interface gets one fitted to it - lists, quoted sentences, unnamed
arguments where position is obvious. **`spec/console.md` gains a sentence saying where each one
applies.** The cost is a second parser and a second path through the review app, and the risk is
that you end up reviewing two kinds of thing in two ways.

**`N3` - one notation unchanged.** Interface tests say `{status-bar shows:new-game}` and the words
live outside the specification. **Nothing new to build and the tooltips are never reviewed.**

## What this lane would say

**`N1`, and the reason is the review app rather than the notation.** The thing that makes your
tests worth anything is that you read them one at a time in one place and a record says you did.
**`N2` puts a second kind of thing in front of you**, and the failure mode is not a bad parser -
it is that one of the two sets quietly stops being reviewed.

**And `N1` is smaller than it sounds**, because prose is the only thing your sketch needs that
one row per fact cannot already say. Lists you do not need; unnamed arguments you do not need.
**One form for a sentence is the whole of it.**

**What would change this lane's answer** is if the interface turns out to need more than prose -
geometry, ordering by pixel, anything continuous. **Then `N2` stops being a convenience and
becomes honest**, and `spec/invariants.md` already says why: *a value that would be compared
across rows is a column; a value that would not may be a node in a cell.*

### P-536 - `spec/data/` sits in `spec/` and holds the release's data, and your ruling today made the two differ

**to** sean · **status** withdrawn · **withdrawn** 2026-09-21 · **raised** 2026-09-21 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `spec/data/`

**Withdrawn 2026-09-21, and the reason is his.** *I am fine with dumping all rules and replacing them with thin engine* - so the form `spec/data/` should take stops being a question for him and becomes whatever the engine reads. **This item asks how to write a file whose writer is being replaced.**

**And it stops being a divergence once the engine's data is the game's data.** This item reported that `spec/data/` holds the release's facts while `spec/` keeps more; under his answer neither file is the source and the question dissolves rather than being decided.
**Kept rather than deleted**, because `CLAUDE.md` says a rejection is recorded with its reason or the same item is filed again. **It comes back if the engine's own data turns out to have the same gap** - and for three of the five, measurement says it does not.

**You ruled today that the model keeps what `spec/` keeps and a release may defer it.**
**`spec/data/` is generated from the release**, so it now holds less than `spec/` does, while
living inside it.

```
                  named in spec/*.md   declared in spec/data/
force                      yes                  no
garrison                   yes                  no
nature                     yes                  no
biome                      yes                  no
```

**`spec/control.md` still has Force and Producing force. `spec/planet.md` still gives every
territory a biome.** None of the four is declared in `spec/data/` any more, because this lane
regenerated those files from the cut release this afternoon - which was right for the release and
is now wrong for the directory's name.

## The thing that makes it more than tidiness

**Your own rule of `spec/README.md`**: *the game's data is decided in its data file*. **If that
file is the release's, then the game's data is decided per delivery** - and a deferred feature has
no data anywhere, not even the part of it `spec/` still states.

**The code lane is living with the consequence today.** Its state report stands up a garrison
because the model keeps one, and links `catalog.html#garrison`, which the catalog does not have
because the catalog is generated from the release. **Twenty-seven dead links over three anchors**,
asserted as a number so a fourth is a finding.

## The three answers

**`D1` - `spec/data/` holds the specification's data, and the release's is generated separately.**
Then `force` and `biome` come back, the catalog and the state report agree, and a release's
narrowing lives only in the release. **The cost is a second generated set and a name for it.**

**`D2` - `spec/data/` holds the release's data and is renamed to say so.** Nothing regenerates;
the directory moves or is called something that does not claim to be the specification. **The
cost is that `spec/README.md`'s *the game's data is decided in its data file* stops having a file
under `spec/` to point at.**

**`D3` - leave it and accept that the specification's data is the current release's.** Cheapest,
and it means a feature you have deferred has no data written down anywhere. **The twenty-seven
dead links stay until the release grows back.**

## What this lane would say, and it is weaker than usual

**`D1` is the one that matches what you ruled**, and it is the most work. **`D3` is what is true
today and nobody has to do anything.**

**What tips it is `P-530`.** Under an executable specification, the tests in `spec/tests/` are the
primary statement and `spec/data/` is a second form of the same facts. **If the tests carry the
game, the question may be which of these files survives rather than which release they follow** -
and `P-517` has been open on that since before today. **So this may be worth leaving until you
answer `P-517`**, and this lane files it now because the divergence is real today and would
otherwise be discovered rather than reported.

### P-517 - `spec/data/` states the cases, and the rules are what you wanted to read

**to** sean · **status** withdrawn · **withdrawn** 2026-09-21 · **raised** 2026-09-14 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `spec/data/`, and `C-114`

**Withdrawn 2026-09-21, and the reason is his.** *I am fine with dumping all rules and replacing them with thin engine* - so the form `spec/data/` should take stops being a question for him and becomes whatever the engine reads. **This item asks how to write a file whose writer is being replaced.**

**And the engine already folds.** Its 15 rules are each named once - `refresh` appears one time, not four - so `F1` is what the thing that runs does, and `spec/data/`'s four `refresh` blocks are the unfolded form of a rule the engine states once.
**Kept rather than deleted**, because `CLAUDE.md` says a rejection is recorded with its reason or the same item is filed again. **It comes back if the engine's own data turns out to have the same gap** - and for three of the five, measurement says it does not.

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

**to** sean · **status** withdrawn · **withdrawn** 2026-09-21 · **raised** 2026-09-14 · **kind** invented · **shape** an instruction · **asks** a decision · **into** `releases/first-release.md` -> Recipes, and `spec/console.md` if the notation moves

**Withdrawn 2026-09-21, and the reason is his.** *I am fine with dumping all rules and replacing them with thin engine* - so the form `spec/data/` should take stops being a question for him and becomes whatever the engine reads. **This item asks how to write a file whose writer is being replaced.**

**And the engine has already answered it by building.** `move` exists in `prototypes/thin-engine/data/friendly/rules.4x` with its clauses written out; whichever of `M1` to `M3` and `A` to `D` it amounts to is a fact to read off the rule rather than a choice to make.
**Kept rather than deleted**, because `CLAUDE.md` says a rejection is recorded with its reason or the same item is filed again. **It comes back if the engine's own data turns out to have the same gap** - and for three of the five, measurement says it does not.

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
