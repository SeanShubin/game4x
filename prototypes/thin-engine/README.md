# thin-engine

**The question**: can a thin engine run the game from data?

That is `C-114`'s open half. `C-114` is Sean's reason, and it is a reason rather than a
preference: *a thin engine running a data driven game forces inadequacies in the engine and data
structure to come to light sooner*. **A thin engine is an instrument, and the three explosions -
of the code, of the data structure, of the data - are its three readings.**

Built to `S-136`. **Nothing here is a decision** - it is research, and if its answer implies one,
that reaches Sean as a proposal through the specification lane, not from this directory.

**[`layers.md`](layers.md) says what may say what**, and which check holds each boundary - the code,
the words it branches on, the structure, this game's rules, a scenario, and the harness that runs
one.


## Why any of this is a requirement, in Sean's words

**Sean, 2026-09-15**: *The primary reason for these conciseness and simplicity requirements are so
that I can maintain executive control as a human. I felt I was losing control from the spec instance
so now I am redoing everything incrementally from the ground up, making sure I keep the language
understandable by both of us along the way. The friendly format is how I keep track of everything in
my head, the foundation format is the more formal model that is easy for an ai assistant to
understand but hard for a human to grok.*

**So minimal is not a taste here.** A row this lane could add without anybody noticing is a row that
costs the one thing the process depends on - `docs/process.md` is Sean's statement that he keeps
executive control over the specification, and a model he cannot hold is one he cannot exercise it
over. **The two formats have different readers and that is their whole design**: friendly is his,
foundation is this lane's, and `tests/directories.rs` is what says they are the same facts.

**It is also why the invariants below rank the way they do.** Losing the foundation-to-friendly
direction is acceptable and losing conciseness is not, because one costs a tool and the other costs
a reader.

## The invariants


**Sean, 2026-09-15**, saying what is essential here and what may bend. **They are about this
prototype's friendly notation and not about the game's command language**, which is
`spec/console.md`.

**1 - The friendly notation states the minimum.** *The user should only have to specify the minimal
needed information to execute the command in the friendly notation.*

**2 - Arguments are identified by name.** *The user should identify arguments by name rather than
id, except where the id is part of the minimal needed information to execute the command.*

**3 - Friendly to foundation is the direction that must work.** *While we should retain two-way
translation between friendly and foundation notations if we can, this should not come at the
expense of the other invariants, being able to translate from friendly to foundation is the
direction we must support, the translation from foundation to friendly can be abandoned if we
must.*

**4 - A defect is fixed when a test replicates it, and not before.** Sean, 2026-09-16: *We don't
fix problems a test does not replicate. The process is red/green/refactor. [...] we must not fix
this until we have a test to make sure it can't creep back in.*

| Invariant               | May it bend                                       |
| ----------------------- | ------------------------------------------------- |
| **1**, the minimum      | no                                                |
| **2**, names before ids | no, and **1** is written into it as its exception |
| **3**, the round trip   | **yes**, and it yields to **1** and **2**         |
| **4**, red before green | no                                                |

**4 binds a correct diagnosis as hard as a wrong one.** It was said of a defect this lane had just
measured and got right - `move` removes a whole row and adds exactly one, so moving a scout into a
territory that already holds one loses it silently. **Being right about a defect is not permission
to fix it.** A fix with no red test behind it leaves nothing that would notice the defect coming
back, so the repair is unverifiable in exactly the sense `docs/process.md` means: *a quality
improvement's evidence is a test that would have failed before it.*

**So a defect this lane finds gets written down and left alone.** The silent loss above is recorded
in the mutation suite already - `residency.quantity` sits in `NOT_LOAD_BEARING` with the note that
the line goes red when quantities start being read - and that is the whole of what to do with it
until a test replicates it.

**2's exception is 1 applied to an identifier**, so the two cannot collide: an id appears only
where the minimum genuinely is one.

**Neither 1 nor 2 has a check yet**, and cannot until a friendly command is a rule-named row at
all. **3 is a precedence rather than a property**, so what checks it is which test is allowed to
be deleted, named below.

## What the invariants settle, which the prototype had left open

**Minting is permitted.** This README has called the minting question *closed rather than solved*,
and `tests/common/friendly.rs` rests on *nothing is minted*. **That was a consequence of the
friendly format carrying every id**, which 1 removes: a minimal command states no `command` id and
no `argument` ids, so the converter must make them. **3 is what says that cost is acceptable.**

**And 3 names the test that may go.** `tests/directories.rs` asserts both directions;
`the_friendly_source_is_what_the_foundation_renders_to` is the expendable one, and
`the_foundation_is_what_the_friendly_source_converts_to` is not.

## What 1 opens, and it is not what this lane first wrote

**This section said the minimum was `{move what:scout to:territory-2}`, and that was wrong.**
Sean, 2026-09-15: *I don't think that is true, or if it is true it is only true in a coincidental
sense and not a general sense. Say I have 3 territories, all adjacent to each other, and two scouts
in each territory. If I want to move a scout to territory-2 I have to specify if I mean a scout
from territory-1 or a scout from territory-3.*

**The error was generalizing from a one-instance world.** `before.4x` holds one scout, so `what`
happened to identify a thing, so `from` happened to be derivable. **Nothing in the model says
either** - the same failure this directory had demonstrated one commit earlier by adding a second
`residency` row and watching the structure accept it.

**`from` is part of the intent whenever `what` is indefinite**, and *a scout* is indefinite.

## Six scouts, and the second invariant is the one that breaks

**Sean's world does not merely make `from` necessary; it cannot be written down.** Rendered with
six things named `scout`, measured rather than reasoned:

```text
{residency id:1 what:1 where:territory-1}
```

**`what:1` is an id.** `tests/common/friendly.rs` makes nameability all-or-nothing per relation, so
six things sharing a name means `thing` is not nameable and every reference to one falls back to
its id. `where:territory-1` survives because territories are distinct. **That is invariant 2
failing, in the world invariant 1 was being argued about.**

**So referring to one of several interchangeable things is the open question**, and both answers
cost an invariant:

| How a scout is referred to       | What it costs                                                              |
| -------------------------------- | -------------------------------------------------------------------------- |
| a unique name per thing          | **1** - naming `scout-4` is more than the minimum when the intent is *any* |
| a shared name, `scout`           | **2** as it stands - `thing` stops being nameable and `what` renders as id |
| a kind, with `from` to narrow it | neither, and it is what Sean wrote                                         |

**The release is already on the third line, which is evidence rather than proof.**
`releases/first-release.md` -> Recipes names an individual nowhere: `consume 2 citizens`, `consume
1 metal`, `require 1 unit | moving at least 1` at `$from`. **Every ingredient is a kind narrowed by
a trait and a place** - which is `{move what:scout from:territory-1 to:territory-2}` exactly. The
prototype's single named scout is the unrepresentative world, not Sean's six.

## Scouts are fungible, so a row carries a quantity

**Sean, 2026-09-15**: *Scouts are fungable, so the id associated with scout refers to the scout
category. We are going to need to represent quantities, so I could in principle have a million
scouts in the same territory without having a million rows. If I had a million scouts across 3
territories, I would need 3 rows to express the only difference between the scouts.*

**This settles what `what:scout` refers to**, which the section above left open. It is not one of
six things sharing a name - it is **the category**, which has one row and one name. **So invariant 2
was never in danger**: `scout` is unique among categories, and the six-scout world measured above is
one the model does not have.

**And `from` stays necessary, for the reason Sean gave rather than the one this lane argued.** A
million scouts across three territories is three rows, and *a scout* names no one of them. **The
place is what picks the row**, so `{move what:scout from:territory-1 to:territory-2}` states the
minimum exactly.

## What quantities need that the engine has not got

**A key over two columns.** Three rows for one category is what is being asked for, and
`(what, where)` is what tells them apart - so that pair has to be the key. It cannot be:

```text
two `residency` rows have `what` of `1`, so it names neither
```

**A key is one column**: `src/schema.rs` returns `columns[0].name`, and the structure check counts
`(relation, that one value)`. **Keeping the surrogate `id` avoids the refusal and buys the wrong
thing** - three rows load, and so does a fourth putting scouts in territory 1 a second time, which
was tried and accepted. **The same fact then has two spellings, which is what a key exists to
stop.**

**And the three roles do not cover arithmetic.** Moving one scout out of a territory holding two
leaves one; it does not remove the row. `require`, `remove` and `add` are set operations over whole
rows, and `remove` removes *every* match. **The release already has the shape this wants**:
`releases/first-release.md` -> Recipes has a **Qty** column and the roles `consume` and `produce`
beside `require` and `put`. **The prototype's three are a subset that predates quantities.**

## The three answers, 2026-09-15

**A row at zero goes.** Sean: *Definitly goes, the model is a minimal expression of intent.*
**That is invariant 1 said of the data rather than of the notation** - a row recording that no
scouts are somewhere expresses no intent, so it is not written. A quantity reaching zero is a
deletion and not a value.

**A quantity of one is still written.** Sean: *my intuition is that it may not [be omitted], but
that is not a hard requirement, it is my estimation of what will be forced upon us to keep the
model and code simple.* **Marked soft by its author**, and it does not collide with invariant 1:
one scout of five is not derivable from anything, so stating the one *is* the minimum.

## Two sorts of input, which is what `thing` becomes

**Sean, on what the split is for:** *Looking at this from the ability to specify inputs in commands.
Territories are not fungable, they are unique by id and always have a count of 1, but scouts are
fungable, don't have an id and may have a count greater than zero. Both kinds of things may need to
be inputs. I may need to specify territory-2, or 3 scouts.*

| Sort         | Identified by | Count     | Written       |
| ------------ | ------------- | --------- | ------------- |
| non-fungible | an id         | always 1  | `territory-2` |
| fungible     | its category  | 0 or more | `3 scouts`    |

**So an input is one of two things and the `input` relation says which today by accident.** `of`
names a relation, which is enough to say *a territory*; it is not enough to say *three of the scout
category*, because the count has nowhere to go. **That is the next thing the model needs and it is
not built.**

**A reading this lane offers rather than measures**: the split may be *places against contents*.
A territory is never a `what` and always a `where`; a scout is never a `where`. If that holds it is
one rule rather than two sorts - **but it is inference, and the release is where it would be
checked** rather than here.

**What the release does say, counted rather than recalled**: it declares **19 kinds**, and
`reports/catalog.md` gives them **19 distinct signatures over 171 pairs**. Both `territory` and
`orbit` are among them, so the release models a place as a kind like any other - which is evidence
against the split being a rule about kinds and for it being a rule about *roles in a relation*.

## Where a composite key is needed, tried three ways

**Sean, 2026-09-15**: *show me an example of where we need composite keys? And does this need
manifest in the friendly notation, foundation notation, or both?*

**One relation - a residency carrying a count - and each single-column key in turn:**

| Key              | Rows that break it                            | Outcome                                 |
| ---------------- | --------------------------------------------- | --------------------------------------- |
| `what`           | scouts in three territories                   | refused: *two rows have `what` of `1`*  |
| `where`          | one territory holding scouts **and** pioneers | refused: *two rows have `where` of `1`* |
| a surrogate `id` | two rows for scouts in territory 1            | **accepted, and that is the defect**    |

**Neither column can be the key alone**, and each fails on a world the game plainly has. The third
is the one to look at:

```text
{residency id:1 what:1 where:1 count:2}
{residency id:4 what:1 where:1 count:3}
```

**The structure accepted both.** Scouts in territory 1: 2, 3, or 5? Nothing can say. **The only
complaint came from the rule, after the check had passed.**

## Why quantities are what force it, and nothing before them did

**The store is a set, and that did most of a key's work for free.** Two identical rows collapse -
`Store::add` will not hold a fact twice. So before a count existed, a duplicate residency was
*redundant* and the state it described was still unambiguous.

**A count is exactly the column that breaks that.** Two rows differing only in their count are not
identical, so the set does not collapse them, and they do not agree. **The ambiguity arrives with
the quantity**, which is why this was not a defect worth fixing until now.

## Foundation, and the friendly notation only inherits it

**A key does two jobs here, and `src/` splits them:** uniqueness, in the structure check; and
resolving a reference, in `has_key` and in the check that a reference points at a row.

**For `residency` the second job is vacant** - nothing in the schema references a residency,
verified rather than assumed. **So its key is purely a constraint and never an identifier.**

**A constraint is a schema fact, and the schema is foundation data**, so that is where the need is:
*first column wins* has to become something declared, which is rows rather than a rule in `src/`.

**The friendly notation does not change where it would be expected to.**

| Where                    | Does it change                                                         |
| ------------------------ | ---------------------------------------------------------------------- |
| a state row, `before.4x` | **no** - `{residency what:scout where:territory-1 count:2}` either way |
| the schema, `schema.4x`  | yes, but only as rows, which it renders like any other                 |
| the translator           | **it gets smaller**                                                    |

**Smaller because the special case was already redundant.** The renderer leads with the key column
and then writes the rest in declared order - and the key *is* the first column, so that is declared
order written twice. **A composite key deletes the special case rather than complicating it.** And
`Names` only generates a name for a relation something references, so a residency never had one to
lose.

**One caveat, and it is the thing to watch.** All of that holds *because nothing points at a
residency*. **If anything ever needs to, the friendly notation needs a way to name a row whose key
is two columns** - `scout@territory-1`, or whatever it would be - and that is real work on the
friendly side. It does not arise today and would arrive the first time a rule needs to refer to a
residency rather than match one.

## `{refused}` is the other ending, and an error test earns its place

**A command leaves a world or it is refused**, so a test states a `{then}` or a `{refused}` and
never both - `Failed::BothEndings` says so rather than guessing which was meant.

```text
{when}
{move what:scout from:territory-1 to:territory-3}

{refused}
{adjacency from:territory-1 to:territory-3}
```

**`{refused}` names the row the rule needed and the world did not have**, which is exactly what
`Refused::NotSo` already carried. **It reads as the reason**: territory 1 touches 2 and 2 touches 3,
nothing says 1 touches 3, and `move`'s second clause asks for a row that is not there.

**Poisoned both ways rather than one.** Make the move legal and the report says *expected
{adjacency from:1 to:3}, actual nothing was refused*; name the wrong reason and it says *expected
{adjacency from:3 to:1}, actual {adjacency from:1 to:3}*. **A test that asked to be refused and was
not is a failure**, which is the half a refusal test most easily loses.

## What the error test was worth, measured rather than argued

**Two bindings stopped being dead.** The deletable list said six `rules.4x binding` rows and says
four. **A test that only succeeds exercises no constraint**, because a constraint is what stops
something - and nothing in this suite had ever sent a `move` the world refuses.

**And two rows went the other way.** The refusal test states the adjacencies 1-2 and 2-3 and needs
neither; what it turns on is that nothing says 1-3. **They are there so a reader can see there is a
path and it is not a direct one**, and the deletable list carries them rather than the test losing
them.

**A refused test reads less of its own world than one that succeeds** - `thing.name` is dead in it
too, because it never gets as far as moving anything. **Worth knowing before a short dead list is
read as a tidy one.**

## Where the game logic is, and what is beside it rather than in it

**Sean, 2026-09-16**: *I want to make sure the core game logic is in a single, small file focused on
nothing else but game logic. [...] I need to be able to understand the core input->processing->output
loop, so I will need the input and output data structures, but any generic utility logic not
specific to the game should be elsewhere.*

**`src/engine.rs` went from 600 lines that run to 356**, and what left it was not game logic:

| Left for                     | Lines | Why it is not the loop                                               |
| ---------------------------- | ----- | -------------------------------------------------------------------- |
| `store.rs`, `take`/`put`     | 120   | arithmetic over rows; the caller asks the schema which column counts |
| `schema.rs`, `check`         | 63    | whether rows fit the structure, which is the structure's question    |
| `refusal.rs`, `Refused`      | 62    | an output type and thirty lines of the words it is written in        |
| `view.rs`, `shown`/`outline` | 42    | what a reader does with what the engine left                         |

**What stayed is the loop and its two ends.** `play` and `fire` and `apply` and `row_of` and
`offered`, with `Effect` as the output and a command row as the input. **`Game` stayed too**,
because the old state and the new state are the other two ends of the signature.

**`take` and `put` lost their `Game` on the way out**, which is what made them generic: a store is
handed the column that counts and does the arithmetic, rather than asking a schema it should not
know about.

## `build-extractor`, and what it needed from the engine: nothing


**A rule, an input, three clauses and nine rows of binding.** No new role, no new word, no line of
`src/` - the machinery a second rule wanted was already there.

```text
{rule id:2 name:build-extractor}
{input id:4 rule:build-extractor seq:1 name:where of:territory}
{clause id:5 rule:build-extractor seq:1 role:remove relation:residency}
{clause id:6 rule:build-extractor seq:2 role:remove relation:residency}
{clause id:7 rule:build-extractor seq:3 role:add relation:residency}
{literal id:3 clause:clause-5 column:44 value:labor}
{literal id:7 clause:clause-7 column:44 value:extractor}
```

**No `require` clause.** `move` carries one beside each `remove`, and it buys nothing: `remove`
refuses when there is not enough, and `apply` builds a new store it discards on any failure, so the
all-or-nothing does not rest on requiring first. The only difference is which refusal a reader
gets.

**`value:labor` reads because the column says what the value is.** A literal binds `residency.what`,
which references `thing`, so the value is a thing and the friendly form names it. Before that the
rules file stated three category ids and named none of them - `value:2`, `value:3`, `value:4` - which
is the id lookup the friendly format exists to remove.

## What the mutation suite said about it, which is that it is under-tested

**Three bindings and four literals can be deleted and no test notices.** One test builds one
extractor from exactly one labor and one metal, so a `remove` that loses its `what` still takes
something, and one that loses its quantity **falls back to taking the row - the same answer when the
row holds one**.

**That fallback is working as written and masking as a consequence.** It wants a test with two of
something before the quantity on a `remove` can be trusted, and per invariant 4 it is written down
rather than changed.

**And the quantities went live.** Three lines that used to sit in the dead-value list were a test's
`residency.quantity`, dead while `move` removed rows and added ones. `take` and `put` read them now.
Only the refusal test's remains, because that test never moves anything.

## Green: `remove` and `add` count, where the relation counts


**A counted relation is arithmetic and an identified one is a set.** `{residency what:1 where:1
quantity:1}` takes one scout from a territory that may hold five; `{adjacency from:1 to:2}` takes
the row, because there is nothing there to count. **Same two roles, no new words** - the engine
still knows 33.

- **`take`** finds the row by its description, refuses if there are fewer than asked, decrements,
  and **deletes the row at zero** - `spec/console.md`: *an entry is never zero*
- **`put`** finds the row by its description and joins what is there, or adds it where there is
  none

**`move` gained one row of data and no code.** `{literal id:2 clause:3 column:46 value:1}` is
what says the remove takes **one**; before it, the clause named no quantity and meant the row.

**A pattern that names no quantity still means the row**, which is what `remove` meant before any
relation counted. So the change is additive: a clause that says how many gets arithmetic, and one
that says nothing behaves as it always did.

## Three reds, and what each one isolates


**`move` has two defects and one test would have caught both at once.** Sean, 2026-09-16, chose two
instead, so a single red names a single repair:

| Test                                          | World              | What it asks                         |
| --------------------------------------------- | ------------------ | ------------------------------------ |
| `one-scout-of-two-leaves-and-one-stays`       | 2 here, none there | did leaving take **one** or the row  |
| `a-scout-arriving-where-one-stands-makes-two` | 1 here, 1 there    | did arriving **join** what was there |

**The suite passed before these because the two defects cancel.** One scout leaving an empty
destination: removing the whole row takes exactly the one that should go, and adding exactly one
lands where nothing was. **Right answer, wrong reason, twice** - which is why `residency.quantity`
had been sitting in the dead-value list.

**Each new test breaks one masking and not the other.** Two in the source makes removing the row
visibly wrong while the arrival stays correct; one already in the destination makes the arrival
visibly wrong while the departure stays correct. **Neither passes under half a repair**, checked
both ways round.

## Every red at once, because that is the shape of red/green

**The harness collected the first failure and stopped.** With three tests red that showed one of
them, and a test that would not run at all - `build-extractor` - aborted the loop before the others
were reached. **It reports all of them now**, run failures included, because a run that is meant to
be red is one where you want to see everything that is.

```text
3 of 6 tests did not reach their `then`:
  a-scout-arriving-where-one-stands-makes-two   expected quantity:2, actual quantity:1
  an-extractor-is-built-from-labor-and-metal    refused: no command is stated with id `build-extractor`
  one-scout-of-two-leaves-and-one-stays         expected {residency what:1 where:1 quantity:1}
```

**The mutation suite is red too, and correctly.** It mutates the data and asks whether anything
notices; its own control checks the *unmutated* data first, and that control is failing because the
tests are red. **A mutation suite means nothing over a red suite** - that is the control doing its
job rather than a second defect.

## What the suite is testing, which is not the engine


**Sean, 2026-09-16**: *This suite is testing one of many sets of possible rules supported by the
game engine. We are directly testing the interface that the game user interface is going to use.
The support infrastructure, which includes most of the data and all of the game engine, is being
indirectly tested through this surface. All of the tests share the same instance of the game rules,
so the rules for one test are the same as the rules for another. This indirectly tests if our
support infrastructure is flexible enough to implement this particular ruleset.*

**So a `{when}` row is the surface, not a convenience.** `{move what:scout from:territory-1
to:territory-2}` is what a user interface would send, and `offered` is what it would ask to know
what to show. **Nothing here tests the engine directly**, and that is the design rather than a gap:
a rule the engine cannot express shows up as a test that cannot be written.

**And it decides what belongs in a test.** A test's `given` holds **state** - territories,
adjacencies, residencies - and the **ruleset** is shared: the schema, the rules, and which
categories exist. `scout` means the same thing in every test because the ruleset is one instance.

## `thing` moved out of the tests, and the collision is what found it

**Two tests used `thing id:1` for two different categories** - `scout` in one and `labor` in
another - and the translator could not say which, because one name table reads every file. **The
fix is not unique ids across tests**, which is a coupling nothing states and nothing checks; it is
that a category was never a test's to declare.

`data/{d}/things.4x` holds all four, loaded by `setup.4x` beside the rules. **And `thing` stopped
being state**: a test compares `adjacency`, `residency` and `territory`, because what a test asserts
is what its world became and not which categories exist.

## One test per file, and one line of it is not the test



**Sean, 2026-09-15**: *I want the tests to be grouped together in a directory without non-tests.
The supporting infrastructure should eat up no more than one line per test file. [...] I intend to
have one test per file.*

```text
data/friendly/
  schema.4x  engine.4x  rules.4x  script.4x  setup.4x
  tests/
    the-scout-crosses-two-borders.4x
    the-scout-moves-to-an-adjacent-place.4x
```

**A blank line sets each section apart**, which Sean wrote into the friendly files and
`a_test_sets_its_sections_apart` now keeps. **The generator would have taken it straight back
out**: `render` copies the foundation's blank lines, and the foundation had none - so a convention
written on one side only would have lasted until the next render. **It is in both, and checked in
both.**

**A test file is `{test name:...}` and three sections.**
 The four `{load ...}` rows moved to
`setup.4x`, which every test reads before it runs; `{report title:...}` went entirely, because a
test that is one file needs one name. **`report` and `title` left the engine with it**, and the
vocabulary went 34 words to **32**.

## Adding a test is adding a file, and that had to be made true

**Nothing in `src/` or `tests/` names a test.** The harness reads `data/foundation/tests/`, and so
do the round trip, the directory comparison and the mutation suite. **A second test was added to
check that** - `the-scout-crosses-two-borders` - and the only things that broke were three
hand-written row totals.

**Those totals are derived now.** A number every new test has to move is a number that makes adding
a test expensive, which is the opposite of what this reorganization is for. **Each carries a floor**,
because a derived total compared against itself passes over an empty directory - `CLAUDE.md`'s count
over nothing, which the same edit would otherwise have introduced.

## A test's name is its file's name, and the mutation check is why

**Dropping the one literal test name left `{test name:...}` read by nothing.** The mutation suite
said so in the same run: both test rows became deletable. **The fix is a better check than the
literal was** - `report.test` must equal the file's stem - so the name is load-bearing, the
convention is enforced, and no new test adds a line anywhere.

## Given, when, then - one file, and the engine got smaller


**Sean's sketch, 2026-09-15**, and it is what `data/friendly/test.4x` now is:

```text
{given}
{territory id:1 name:territory-1}
{thing id:1 name:scout}
{adjacency id:1 from:territory-1 to:territory-2}
{residency what:scout where:territory-1} -> 1
{when}
{move what:scout from:territory-1 to:territory-2}
{then}
{residency what:scout where:territory-2} -> 1
```

**`{given}`, `{when}` and `{then}` are markers**: every row after one belongs to it, which is line
order carrying grouping as well as sequence. **They carry no values** - the name is the whole of the
row - so they are words the script knows rather than relations `script.4x` declares. A relation with
no columns has no key and holds no data, and `Malformed::NoColumns` is right to refuse one.

**`#` stays a comment.** Written as `# Given` the sections would be invisible: the file would parse
to fifteen rows with the given and then states merged, one scout in two places at once, and `{move
…}` a row of no relation. **The headers had to be structure or nothing.**

## What the merge deleted

**Nothing says to execute or to compare** - the sections say it. `execute` and `compare` went, and
the `actual` and `expected` stores with them. **And a command is a row of its own rule**, so
`{command …}` and `{argument …}` had no rows left at all: the `command` and `argument` relations
went too, with their six columns and three references.

|                        | before | after      |
| ---------------------- | ------ | ---------- |
| files in `data/`       | 8      | **5**      |
| rows                   | 215    | **187**    |
| words the engine knows | 37     | **34**     |
| relations              | 17 + 9 | **15 + 7** |

**The engine's vocabulary shrank.** It gained `given`, `when` and `then` and lost `execute`,
`compare`, `this`, `with`, `actual` and `expected` - **six for three**, which is the first time
adding a feature here has cost negative words.

**And the last decoration found a reader.** `input.name` was read into an error message and nowhere
else; `fire` binds a command's inputs *by name*, so `{move what:… from:… to:…}` reads it. **Both
names this list used to hold now do work** - `rule.name` when a section fires, `input.name` when it
binds - and the dead-value list is down to four entries from ten.

## What the merge cost, which is one real thing

**A test file spans two stores.** Its prologue is script rows and its sections are game rows, and
the two schemas number their relations independently - so one `Names` cannot read the whole file and
which one to use is a fact about where the row sits. `friendly::in_a_section` is that fact, and
every reader of a merged file needs it.

**It was found by the names vanishing.** The `then` section repeats `given`'s rows, and two rows
named `scout` made `thing` non-nameable, which took away every thing's name. **A store is a set**,
so deduplicating was the fix and the store's own doc comment already said so.

## The test is a sequence, so it stopped saying so twice

**`{execute id:1 seq:8 command:1}` is `{execute command:move}`.** Nine columns went - `id` and `seq`
from `load`, `execute`, `compare` and `report`, and `id` from `test` - because **the file already
says what order it is in** and nothing referenced a step. 482 characters became 369.

**`seq` carried a bug and it is now unwritable rather than fixed.** It was sorted as text once, so
`10` came before `2` and every step after the first ran out of order. `Failed::OutOfSequence` and
its refusal are gone with the column: **nothing can say a wrong order when nothing says an order.**

**Sean drew the line this rests on**: line order is a *temporal coupling* - it says when a statement
happens, and never what an argument means. **Every argument stays named.** That also rules out the
bare `territory-1 territory-2` form this README weighed for adjacency, which bought its brevity by
making columns positional.

**And a decoration became load-bearing, which is the better direction.** `rule.name` was on the list
of values nothing reads; `{execute command:move}` resolves a command by the rule it fires, so the
name is now what a step is written in. **The dead-value list went from ten entries to five** - four
by deleting columns, one by finding a reader.

## What may be fired is computed, not listed

`spec/invariants.md` states it and nothing implemented it. Of a recipe: *The player's are offered
wherever their inputs are present, to take or to leave.* And of a choice: *What may be chosen is
whatever the game holds, and the offering is derived rather than listed.*

**Sean, 2026-09-15**, on why it matters more than a tidy reference: *we won't want executing an
invalid command to even be possible in the user interface.*

**`offered` returns rows in the friendly command form**, so what comes back is what a player would
type. **Offerable means would not be refused** - each candidate binding is fired and kept if firing
succeeds, so there is no second copy of what legal means to drift from the first.

**It added no word to the engine.** `rule`, `input`, `clause` and `binding` were all vocabulary it
already had, so the boundary `data/engine.4x` draws did not move: 37 words before and after. **`run`
split into reading a command and firing a rule**, and no logic moved.

## The offering walk found a one-way world

**The scout can go from territory 1 to 2 and cannot come back.** `adjacency` is stated one way -
`{from:1 to:2}`, `{from:2 to:3}` - and `move`'s second clause requires a row in exactly that
direction. **No test could see it**, because the only move in the scenario runs downhill.

**The game does not have this.** `crates/game-model/src/game.rs` holds adjacency as
`Vec<Vec<TerritoryId>>` and its own comment says *Symmetric*; the dump halves it for writing.
**The prototype kept the halved form and lost the symmetry with it.**

**Asserted as it is rather than as it should be**, in
`only_the_moves_the_world_allows_are_offered`. Whether to state both directions or to read one from
either end is a modelling decision and not a test's, and that line goes red when it is made.

## And the isolation check caught a keyword

**`move` is a Rust keyword and the name of the game's one rule.** A closure written
`.map(move |key| …)` put the word in a line that runs, and `tests/isolation.rs` refused it. **It was
right and should not be taught the difference** - the day it can tell a keyword from a noun is the
day it stops catching what it is for. The loop form says the same thing and says no nouns.

## Fuel falsifies the key relation, and `spec/console.md` had already said so


**Sean, 2026-09-15**: *We shoud never have non-determinism form what row happens to be encountered
first. The app spec has the concept of a tree from distinguisishable to quantity. We don't have fuel
yet but if we did, we might have something like `{scout fuel:1} -> 2`, `{scout fuel:2} -> 3`. Does
this change your recommendation regarding the key relation?*

**It does, and the example does not complicate the key relation - it falsifies it.** A key declared
as `(what, where)` makes those two rows collide: same category, same place, different fuel.
**A declared key rejects valid data the moment a trait it does not list starts telling rows apart**,
and it does it silently, by refusing rather than by being wrong.

**The specification already states the rule, and this lane recommended against it without reading
it.** `spec/console.md`:

> **What a thing contains is a map from a description to a quantity.** A description is a kind and
> **every trait of that thing**; a trait **of its kind** is not part of one, because naming the kind
> has already said it. **No trait of the thing may be left out** - `{citizen defending:1} -> 8` and
> `{citizen defending:0} -> 6`, never `{citizen} -> 14`.

**So the key is not a subset anybody chooses. It is every column but the quantity, by rule.**

## Declare the quantity, not the key

**One row per relation instead of one per key column**, and it is the difference between a fact
stated once and a fact stated twice. **A declared key restates the column list**, so adding `fuel`
means remembering to add a key row - and forgetting leaves a check that is green and wrong.
**Declaring which column is the quantity cannot drift**: the key extends itself.

## The quantity declaration, exactly

**Sean asked this lane to be specific, and it had not been.** *One row per relation* named no row.
Ids continue the game store's own numbering - relations end at 16, columns at 46, references at 19,
primitives at 35.

**One new relation, declared the way every other one is:**

```text
{relation id:17 name:quantity}

{column id:47 relation:17 seq:1 name:id}
{column id:48 relation:17 seq:2 name:relation}
{column id:49 relation:17 seq:3 name:column}

{reference id:20 column:48 to:1}
{reference id:21 column:49 to:2}
```

**`residency` loses its surrogate id and gains a count**, reusing the three ids it already holds:

```text
{column id:44 relation:16 seq:1 name:what}
{column id:45 relation:16 seq:2 name:where}
{column id:46 relation:16 seq:3 name:count}

{reference id:18 column:44 to:14}
{reference id:19 column:45 to:13}
```

**And then the declaration itself, which is the whole of it - one row:**

```text
{quantity id:1 relation:16 column:46}
```

*The quantity of a `residency` is its `count` column.* **Its key is columns 44 and 45 and nothing
says so**, which is the point: adding `fuel` as column 50 puts fuel in the key by existing.

**In the friendly notation**, where the state row is what a person writes:

```text
{quantity id:1 relation:residency column:46}
{residency what:scout where:territory-1 count:2}
```

## What changes in `src/`, and it is one word

**`Relation::key` already says this is coming**: *a relation's key is its first column. Said in one
place so that the day a compound key is needed, there is one thing to change and it is findable.*

**The one thing splits into two**, because its two callers want different things:

| Caller                                                   | Wants                                      |
| -------------------------------------------------------- | ------------------------------------------ |
| resolving a reference, and checking one points somewhere | **the identity** - still the first column  |
| the structure check's uniqueness loop                    | **the key columns** - all but the quantity |

So `Relation` carries the quantity column's name, the uniqueness loop counts a tuple rather than a
value, and `TwoWithOneKey` widens to say which tuple. **A relation with no quantity is unchanged**,
which is fifteen of the sixteen.

**The engine learns one word.** `const QUANTITY: &str = "quantity";` in `src/schema.rs`, and
`{primitive id:36 word:quantity}` in `data/engine.4x` - **and `tests/engine.rs` fails if either
exists without the other**, reading the constants out of `src/` rather than from a list. **`count`
never reaches `src/` at all**: the engine learns *that* a relation has a quantity and the data says
*which column*, which is the line this prototype exists to keep.

## Why `column:46` is an id here, and why that is right rather than a wart

**The friendly row reads `column:46` and not `column:count`**, because `column` is the one relation
whose names cannot be unique - sixteen of them are called `id` - so every reference to a column is
an id. Sean settled that already: *binding and column are machinery.*

**And there is a rule underneath it worth keeping.** A relation that something points at needs a
single value to be pointed at by; a relation nothing points at does not.

| Relation    | Pointed at by                      | Key                      |
| ----------- | ---------------------------------- | ------------------------ |
| `column`    | `binding`, `reference`, `quantity` | a surrogate `id`         |
| `residency` | nothing                            | `(what, where)`, by rule |

**So `residency` drops its id for the same reason `column` keeps one.** That also answers the
caveat above from the other side: the day something needs to point at a residency, it needs a single
value to point at, and the composite key is what it would have to give up.

## Two options, and this lane had been asking one question as if it were two

**Sean, 2026-09-15**: *Lets explore two options, and we don't have to go with the same option on
both friendly and foundation. One option is the foreign key to a quantity table, which you just
presented. Another option is a quantity field. Note that it would make no sense to have both an id
and a quantity in the same logical model. Also note that the foundation is the logical model,
friendly is a bridge from the user to the logical model.*

**There are two questions and this README ran them together:**

| Question                                     | Where it is settled                    |
| -------------------------------------------- | -------------------------------------- |
| how a state row says its quantity            | **may differ** between the two formats |
| how the foundation says which column that is | the foundation alone                   |

## Option A - a quantity table, and Option B - a reserved name

**A: the quantity is a column named anything, and a row says which.** Seven rows to stand it up -
one relation, three columns, two references and the declaration - then one per counted relation
after that.

```text
{quantity id:1 relation:16 column:46}
```

**B: the quantity is a column named `quantity`, and that is the whole of it.** **Zero rows added.**
A relation carrying a column named `quantity` is counted and its key is the others; one carrying an
`id` is identified.

```text
{column id:46 relation:16 seq:3 name:quantity}
```

## Sean's *no id and a quantity* is what decides it, and it decides for B

**They are not two properties. They are one slot** - how a relation individuates its rows. The
specification says it of a thing with an `id`, in `spec/console.md`: *A thing carrying an `id` has a
description no other thing shares, so **its quantity is always one**.*

This lane's reading of that sentence, which is not the sentence: a thing is counted or it is
identified.

**And `id` already occupies that slot by name.** `const ID: &str = "id"` is in both `schema.rs` and
`engine.rs`, and every one of the sixteen relations is keyed by a first column called `id`. **So A
would state one half of an exclusive pair as a table and leave the other half a naming convention**
- and the asymmetry is the thing that would read as wrong in a logical model, which is what the
foundation is.

**B also makes the exclusion checkable where it is stated.** *Exactly one of `id` and `quantity`*
is a fact about a relation's column list, so under B it is read off that list. Under A it is a join
between the column list and another table, which is the same fact in two places.

**The cost of B, said rather than skipped**: a third column name becomes load-bearing. `schema.4x`
already records two - *a row names its relation by `relation.name`, and a row's values are keyed by
`column.name`* - and `role.name` is a third. **B adds a fourth, and it is the one that was already
half there.**

## The friendly half is not a choice, because the game already made it

**The dump writes `{description} -> quantity`, nested**, and it is not a sketch: `scenario/expected/play.4x`
uses the arrow **114 times**, and its header says *a line reads `{description} -> quantity`, and the
description is the kind and every trait of that thing.*

| Format         | A residency of two scouts                       |
| -------------- | ----------------------------------------------- |
| **foundation** | `{residency what:1 where:1 quantity:2}`         |
| **friendly**   | `{residency what:scout where:territory-1} -> 2` |

**So the two formats differ here, which is what Sean allowed for** - and the friendly side is the
notation he already derives by hand rather than anything invented for this prototype.

**What it costs is translator work and no engine work.** `src/notation.rs` reads one `{…}` per line
and knows no arrow; it does not learn one. **The arrow is read and written in
`tests/common/friendly.rs`**, which is where Sean put the translator and said it *can be as thick as
it likes and the engine does not grow*.

## Built, 2026-09-15, and one thing it needed that was not in the plan

**`residency` is `(what, where, quantity)`**, keyed by the first two and counted by the third.
`{residency what:scout where:territory-1} -> 1` in friendly, `{residency what:1 where:1 quantity:1}`
in the foundation, and `tests/directories.rs` still converts each into the other row for row.

**Two checks that would have passed before**, which is the only reason they are worth having:

- `two_residencies_of_one_description_are_refused` - the pair that loaded clean when a surrogate id
  told them apart, and which a quantity turns from redundant into contradictory
- `a_relation_cannot_carry_both_an_id_and_a_quantity` - Sean's exclusion, checked where the schema
  is read, with a relation of its own as the control because adding a column to one that has rows
  makes `WrongColumns` the refusal whatever the columns are called

**`Relation::key` split in two, as its own comment said it would.** `identity()` is the first
column and is what a reference resolves against; `key()` is every column but the quantity, or the
first column where there is none. **Fifteen relations noticed nothing.**

## `literal`, which the plan did not have and the build needed

**An input is typed as a relation and carries one of its keys**, so a plain number cannot be one -
and `add` is a whole row, so something had to supply `residency`'s quantity. **That is a gap this
lane predicted in prose and then walked into.**

**`{literal id:1 clause:clause-4 column:46 value:1}`** - a sibling of `binding` that takes its value
from the rule rather than from the caller. **`releases/first-release.md` already does this**, with a
**Qty** written in the recipe, so it is the shape the game has rather than a new idea. It cost one
relation, one engine word, and `literal` joining `binding` in the list of machinery
`tests/isolation.rs` keeps apart from the game's nouns.

**The engine learns two words and the count is asserted**: 35 to 37, `literal` and `quantity`.
`tests/engine.rs` reads the constants out of `src/` and fails either way round, so neither could
have been added quietly.

## What the mutation suite found that nobody predicted the location of

**`before.4x`'s `residency.quantity` can be changed and nothing fails.** One scout or five, the
suite stays green - because `move` never reads it. Its `require` and `remove` clauses match on
`what` and `where` and leave the quantity unbound, and what lands at the destination is the
`literal`.

**That is the arithmetic gap, as dead data rather than as an argument.** Moving one scout out of a
territory holding five should leave four, and `require`, `remove` and `add` are set operations over
whole rows that cannot say so. **It is written into `NOT_LOAD_BEARING` with its reason**, so the day
quantities start being read, that line goes red and says so.

**And the control caught its own instrument first.** With the reference count left at nineteen,
`check` failed for the unmutated data too, so every mutation looked load-bearing and both lists came
back empty - which is exactly what `the_data_as_it_stands_passes_every_check` says it exists to
prevent, in its own words: *without this the two tests below could pass by `check` failing for some
reason of its own.* **The empty lists were read as a result for about a minute.**

## `examples/render.rs`, and why the two directories needed one

**Nothing produced `data/friendly/` from `data/foundation/`**; it was kept by hand while
`tests/directories.rs` compared the two. **That is a test that stays green because somebody
remembered**, which is the arrangement this repository has a note about. `cargo run --example
render` writes the friendly directory, comments and all, and the comparison is still what says they
agree however either was produced.




**And the same sentence settles two of the three earlier answers, which this lane had recorded as
Sean's rather than as the specification's:**

| Answer                                      | `spec/console.md` already says                                                                    |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| a row at zero goes                          | *an entry is never zero*                                                                          |
| a territory is unique by id, always count 1 | *a thing carrying an `id` has a description no other thing shares, so its quantity is always one* |

**That second line is the unification, and it is better than the two sorts this README proposed.**
There are not an individual sort and a fungible sort. **There is one rule** - a description maps to
a quantity - **and an `id` is what makes a description unique**, which is why a territory's count is
always one without anything saying so. `{territory id:2} -> 1` and `{scout fuel:1} -> 2` are the
same shape.

**Where it stops.** This is the rule for a **state** relation. A declaration - `thing`, keyed by an
id with `name` as decoration - is not a description mapped to a quantity, and `spec/console.md`
keeps them apart: *`name` is to a declaration what `id` is to a thing.* **The prototype already has
the hook**, in the `{state relation:...}` rows that say which relations the state is made of.

## What a key still does not do, which is the harder half

**Invariant 4 is not satisfied by any key.** A key makes each description unique. **It says nothing
about which row a selector picks**, and `spec/console.md` is explicit that a selector is the partial
form: *it may **leave traits out***, where a description may not.

**So `consume 2 scouts` against `{scout fuel:1} -> 2` and `{scout fuel:2} -> 3` has three answers
and the specification names none.** Keys cannot reach it - the rows are already distinct and already
legal. **It is filed as the third bullet of `C-114`**, where it was written as *it does not matter
for interchangeable things and does for a thing carrying an `id` or a part-full bin*. **Sean's
invariant promotes it from a detail to a defect**: the engine takes the first match, and nothing may.





## The answer

**Yes, and the test that proves it is data too.** `data/test.4x` is the orchestrator: it sets the
schema up, initializes the state, executes the command, compares what came out with what was
expected, and composes a report. **Nothing in `tests/` says what the test does** - it reads
`test.4x` and runs it.

```text
{test name:the-scout-moves-to-an-adjacent-place}

{load seq:1 file:schema.4x into:game}
{load seq:2 file:rules.4x into:game}
{load seq:3 file:before.4x into:game}
{load seq:4 file:command.4x into:game}
{load seq:5 file:expected.4x into:expected}

{execute seq:6 command:1}
{compare seq:7 this:actual with:expected}
{report seq:8 title:the-first-test}
```

And the report it composes:

```text
the-first-test
  test      the-scout-moves-to-an-adjacent-place
  compared  adjacency, residency, territory, thing
  result    as expected
```

Measured on 2026-09-14:

|                                              |                                                                                            |
| -------------------------------------------- | ------------------------------------------------------------------------------------------ |
| Code that runs, in `src/`                    | **1163 lines** - notation 77, store 33, schema 281, engine 486, script 281                 |
| Rows of data                                 | **202** across **seven** files - five shared and **two tests** - holding **570** values    |
| Of those, rows nothing reads                 | **1**, a binding on a clause nothing in the suite tries to violate                         |
| Values nothing reads                         | **10 of 570** - two `seq`s, one id on a one-row relation, and a quantity per test          |
| Rows that differ between before and expected | **1** - `{residency what:1 where:1 quantity:1}` becomes `… where:2 quantity:1`             |
| Relations declared                           | **15** in the game and **6** in the script; fourteen keyed by an `id`, `residency` counted |
| Game nouns in code that runs                 | **0**, checked against a list read out of `data/`                                          |
| Tests                                        | **41**, all passing                                                                        |

**The engine names nothing the game has.** `territory`, `thing`, `adjacency`, `residency` and
`move` appear in `data/`, in the tests and in comments, and in no line of `src/` that runs.

## What making the structure explicit cost, which is the reading

**873 lines against 232.** An earlier version of this prototype ran the same first test in 232
lines, with rows that had no declared columns and a `$name` substitution in place of a binding.
**Stating the structure explicitly cost 406 lines and making the test data cost another 235**, and
where it went is worth reading off:

| Module        | Was | Is      | Why                                                                     |
| ------------- | --- | ------- | ----------------------------------------------------------------------- |
| `notation.rs` | 77  | **77**  | Unchanged. A line is still `{name key:value}`                           |
| `store.rs`    | 65  | **33**  | **Shrank.** `$name` substitution went away entirely                     |
| `schema.rs`   | 0   | **230** | New. Relations, ordered columns, references, and checking               |
| `engine.rs`   | 87  | **294** | Reads the rule, its clauses, their bindings and the command out of rows |
| `script.rs`   | 0   | **234** | New. Running a test that is written down rather than compiled in        |

**The notation never moved.** Four concepts were built on it and then thrown away, a whole
relational structure was put through it, and it is the same 77 lines it was at the first commit.
**That is the strongest thing this directory has established**, and nothing set out to test it.

**`store.rs` shrinking is the surprise.** Substituting `$what` into a pattern was 47 lines; a
`{binding ...}` row saying *this column takes that input* needs none of it, because the binding is
data rather than a hole in a string. **The structure did not add a mechanism, it moved one out of
the code.**

## What the structure buys, each checked rather than claimed

**A command naming something that is not there is refused by its type.** `{input id:move.to
rule:move seq:3 name:to of:territory}` says the value is a `territory`'s key, so moving to
territory 9 is refused before `move` is looked at. **No rule says the destination must exist** -
an earlier version needed a row per rule for exactly that, and a declared structure does it once
for all of them.

**A reference is checked against the relation it names.** `{residency what:1 where:1}` reads `1`
twice and they mean different things. `tests/structure.rs` checks the two apart with values that
exist in one relation and not the other, because both being `1` is precisely the case where a
crossed reference would go unnoticed.

**A row is exactly its relation's columns** - not at least them. There is no open row, so a typo
in a column name is refused rather than quietly ignored.

**The structure describes itself.** `relation`, `column` and `reference` are declared by
`{relation ...}` rows like everything else, and checked against their own description. **That is
what makes *everything is data* a thing a test can fail.**

## Fully normalized, and what that turned out to mean

**No relation may have a column that depends on which row it is.** The version before this one
wrote an effect as one row:

```text
{effect rule:move does:remove relation:residency what:$what where:$from}
```

**`what` and `where` are `residency`'s columns, not `effect`'s.** Point that row at a different
relation and its columns change - so `effect` had no fixed columns and was not a relation at all.
It splits into two that do:

```text
{clause  id:move.3 rule:move seq:3 role:remove relation:residency}
{binding id:move.3.what  clause:move.3 column:residency.what  input:move.what}
{binding id:move.3.where clause:move.3 column:residency.where input:move.from}
```

**The rule went from 8 rows to 16, and the `$` disappeared from the data entirely.** Both are the
same change seen from two sides: a hole in a string became a row that names a column and an input.

**Every relation's key is its first column**, which is a convention rather than a dependency - so
there is no `key` relation, and where no natural key existed a surrogate was made. The dotted ones
are readable on purpose: `residency.what`, `move.from`, `move.3.where`.

## The eight files

| File          | Rows   | What it is                                                          |
| ------------- | ------ | ------------------------------------------------------------------- |
| `schema.4x`   | **78** | The shared schema: every relation, its columns in order, references |
| `script.4x`   | **16** | The orchestrator's own vocabulary, declared the same way            |
| `engine.4x`   | **36** | Every word the engine implements - where the data delegates to code |
| `rules.4x`    | **16** | `move`, as inputs, clauses and bindings                             |
| `before.4x`   | **7**  | The state before                                                    |
| `command.4x`  | **4**  | The move command                                                    |
| `expected.4x` | **7**  | The state after, written by hand from the note                      |
| `test.4x`     | **11** | The orchestrator                                                    |

## The three places most likely to be unnecessary

**Sean's job is removing what is not needed and mine is filling the structure out**, so these are
marked `WHY` in the data rather than decided here.

**The self-description, which is 56 of `schema.4x`'s 78 rows.** Nothing needs it to run the first
test - the engine could simply know its eleven relations. It is there so that *everything is data*
is checkable. **Cut it and the schema is 22 rows.**

**`{state relation:...}`, the four rows saying which relations are the game's state.** They are
what bounds the comparison: without them a diff would compare the rule and the command too, which
are in the actual because they were loaded rather than because anything happened. The alternative
is scoping to whatever relations `expected.4x` happens to mention - **the same thing decided by
omission**, where a relation left out of `expected.4x` would go unchecked silently.

**`{test name:...}`, which nothing needs while there is one test.**

**`residency`'s key being `what`, so a thing is in one place and cannot be in two.** That is an
inference from the note rather than something it says - the note gives `residency` no key at all.
It is what makes `remove` then `add` an update rather than two unrelated edits.

**`argument.value` has no reference and cannot have one.** What it points at is whatever
`input.of` says, which is data - so its type is checked when the command runs and not when the
data is read. **It is the one place the structure cannot state a constraint that the engine still
enforces**, and if that asymmetry is unacceptable the fix is a decision rather than a patch.

## The three readings, which is what `C-114` asked for

| Does it explode?       | Against the first test, fully normalized, with the test itself as data                                                                                                             |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **The code**           | **Yes, 3.8 times** - 232 to 873, in three modules, for three things: knowing the structure, reading a rule that no longer has holes in it, and running a test that is written down |
| **The data structure** | **No, and it has now taken a third kind of thing.** One structure - a row. The game is rows, the rule is rows, the command is rows, and so is the test that runs them              |
| **The data**           | **Yes, in ceremony rather than in size.** Seven rows of world need 78 of schema, 16 of rule, 4 of command and 9 of test                                                            |

**The readings point in opposite directions and that is the finding.** Everything the code gained,
the data paid for in explicitness - and the thing that did not move in either direction is the
notation. **A thin engine and an explicit structure are not the same goal**, and this is the first
measurement of the gap between them.

## The first test

`temporary-notes/first-test.md`, which `S-136` states in Sean's words: three territories `1, 2,
3`; two adjacencies `1-2` and `2-3`; one vehicle named `scout`. Moving the scout from 1 to 2
succeeds; from 1 to 3 fails. **No mechanic that is not needed to pass it.**

**The test is `data/test.4x` and `tests/first_test.rs` only runs it.** What is left in Rust is the
two things that cannot be data: handing the engine a way to read a file, and asserting the report
says what it should.

- `the_engine_gets_from_before_to_expected` - the report says *as expected*
- `the_report_says_which_relations_it_compared` - **the control**. If `{state relation:...}` were
  dropped, the comparison would scope to no relations, find no differences and report success in
  the same words; this asserts all four are named
- `the_report_reads_as_a_report` - the composed text, exactly
- `a_state_that_is_not_expected_is_reported_as_both_rows` - `expected.4x` served from `before.4x`,
  so the report has to say `NOT as expected` and name the row that left and the row that arrived

`tests/structure.rs` checks what declaring the structure buys - and one of its checks is that the
four files `tests/` assembles are the four `test.4x` loads, **which caught the two lists disagreeing
about their order the first time it ran**. `tests/isolation.rs` checks that the engine reads no
file, depends on no crate, and names no noun the game has.

## Where the data stops describing and starts delegating

**This is the boundary the prototype is for**, and `data/engine.4x` is it written down: every
string the engine holds and compares a data value against. **34 words.**

**The difference between the two kinds of row is the thing to look at.** A `{clause ...}` or
`{binding ...}` row is data the engine walks without knowing what it means, so a second rule is
rows and no code. A word in `engine.4x` is the other thing - the engine branches on it, so adding
one means writing Rust. Three kinds are mixed in that list:

| Kind                                   | Examples                    | Adding one costs |
| -------------------------------------- | --------------------------- | ---------------- |
| Column names the engine reads by name  | `name`, `seq`, `id`, `of`   | code             |
| Relation names it dispatches on        | `clause`, `binding`, `load` | code             |
| **Values in a row that select a path** | `require`, `remove`, `add`  | code             |

**The third is the purest case, and it is already in the game's data.** `{role name:require}` is a
row whose entire meaning is a match arm in `engine.rs`. That is the shape to recognise: a relation
whose rows are a list of things the engine implements, as against one whose rows the engine only
moves around.

**The list is checked against `src/` both ways** by `tests/engine.rs` - a constant with no row
fails, and a row with no constant fails - so **the boundary cannot drift**. That is the only reason
it is worth a file rather than a paragraph. **It caught something on its first run**: four
constants added an hour earlier, when `compare` was made to read the columns it had been ignoring.

**And the game's own nouns are absent from it**, which is what says the engine is thin.
`territory`, `thing`, `adjacency`, `residency` and `move` are declared in `data/` and the engine
has never heard of any of them.

## Declaring the script's vocabulary, and what it did not buy

`data/script.4x` declares `test`, `load`, `execute`, `compare` and `report` the way the game's
relations are declared, so a step is checked like any other row: a misspelt column is refused by
the structure rather than by a special case in Rust.

```text
{report seq:10 titel:the-first-test}: `report` is (seq title) and this row gives (seq titel)
```

**What it did not buy is any less delegation.** `{relation name:load}` says a `load` row has a
`file` and an `into`; it does not say what loading *is*. **Declaring it makes the delegation
nameable rather than removing it** - which is why `engine.4x` exists beside it.

**It also found a lie in the data.** `{compare seq:9 this:actual with:expected}` had been written
with two columns nothing read - data that looks meaningful and is not. They are read now, and a
`compare` naming a store that is not there is refused.

**The bootstrap, said rather than hidden.** `test.4x` loads `script.4x`, so the `load` step that
fetches the declarations runs before its own description exists. **That is the one place the data
cannot describe itself**: the step that fetches the description. Every other step is validated
before it runs.

## And `seq` sorted as text, which a tenth step found

**Every value in this notation is a string**, and ordering steps by `seq` means deciding what a
`seq` is. Sorted as text, `10` comes before `2` - so the first time the script had ten steps, every
step after the first ran in the wrong order and `report` was reached before `compare` had run.

**It failed loudly rather than quietly**, because `report` refuses when there is nothing to report
on. **That was luck rather than design**: nothing about sorting strings would have complained, and
a script whose steps are order-independent would have been silently wrong. `seq` is parsed as a
number now, and a `seq` that is not one is refused.

## Every row is used, and 51 values are not

**Sean's requirement, in his words**: *there should not be a single value I can change or delete
that doesn't end up breaking something.* `tests/mutation.rs` is that, run over the data: every row
deleted in turn, every value replaced in turn, and something has to notice.

**Every one of the 175 rows is load-bearing.** Delete any of them and the suite fails.

**51 of the 429 values are not**, and they are named and counted so the list cannot grow quietly:

| Not read                                | Count | Why                                                        |
| --------------------------------------- | ----- | ---------------------------------------------------------- |
| `column.id` in `schema.4x`              | 18    | A surrogate key nothing points at                          |
| `column.id` in `script.4x`, `engine.4x` | 12    | The same                                                   |
| `binding.id`                            | 8     | Nothing references a binding                               |
| `clause.seq`                            | 4     | Clauses apply in role passes, so same-role order is free   |
| `input.name`                            | 3     | A binding names its input by id; the name reaches an error |
| `input.seq`                             | 3     | Inputs are sorted and the order changes no outcome         |
| `argument.id`                           | 3     | Nothing references an argument                             |

**41 of the 51 are surrogate keys nothing points at.** They exist because the engine takes a
relation's key to be its first column, so **removing them means composite keys** - `binding` would
be keyed by `(clause, column)` and `argument` by `(command, input)`. That is a decision about the
structure rather than a tidy-up.

**`input.name` is redundant with `input.id`**, which is worth seeing next to the note this came
from: `input what thing` made the name the whole identity, and introducing ids made it decoration.

**`clause.seq` and `input.seq` order things whose order does not matter** - yet. A second rule
where two `remove` clauses contend would make `clause.seq` load-bearing, and that line in
`tests/mutation.rs` is where to look when it does.

## What the mutation check taught about checking

**Nineteen `{reference ...}` rows looked dead and were not.** A reference is a constraint, and a
constraint is worth nothing in a run where nothing violates it - so deleting one changed nothing
anybody looked at. **The suite had no case that needed them.** The fix was to generate a violation
for every reference the data declares, which is now part of what the check means by *working*.

**And the count of them had to be exact.** Written as *at least fifteen references*, deleting one
simply meant one fewer was tested - the loop only ever checks the references that are there. **A
floor asks whether there are enough; the question was whether they are all still there.**

**Three real gaps came out of it**, none of which any other test would have found: loads were never
validated against their own declarations, the `{test ...}` row was skipped entirely, and
`engine.4x` was read from the file rather than from what the script had loaded - so the step
loading it was dead.

## The user-facing format, which is the same notation

**Sean, 2026-09-15**, giving the target and the constraint: *I need to add the constraint that name
must be unique, and the user friendly format is:*

```text
{territory id:1 name:territory-1}
{thing id:1 name:scout}
{adjacency id:1 from:territory-1 to:territory-2}
{residency id:1 what:scout where:territory-1}
```

**It is the same notation and the same rows**, with two differences and no others: every referenced
row has a `name`, generated as `<relation>-<id>` where it has none; and every reference is written
as that name rather than as the id. `before.4x` renders as those seven lines, asserted rather than
claimed.

**Keeping `id` is what makes the round trip exact.** An earlier rendering here dropped the id
wherever a row had a name, which is what made minting look like a problem. It is not one: **the
friendly format carries every id**, so foundation to friendly to foundation invents nothing.

**A name is given only where something references the relation.** That rule is read off the
example - the territories and the thing are named, the adjacencies and the residency are not, and
those two are exactly the relations nothing points at. **A name on a row nothing references would
be a value nothing reads**, which is the thing this prototype is meant not to have.

## Editing this directory, and the tool that exists for it

**Every scripted edit here goes through `tools/anchor`**, and the reason is recorded in `C-129`:
the carrier for *normalize both sides before comparing them* and *write a script to a file* lost to
the failures it prevents, because a three-part edit needed six files and three invocations while a
throwaway `str.replace` script needed one file and one command.

**`anchor edit <file> <edits-file>` is one file and one command**, however many edits. It is now
the smaller thing as well as the right one.

## Two directories, and friendly is the source

**Sean, 2026-09-15**: *Lets make friendly the source and not omit anything. This presumes we can
reliably convert between friendly and foundation. Also it is ok that sometimes they happen to be
the same thing.*

`data/friendly/` and `data/foundation/` hold **the same eight files and the same 215 rows**.
`tests/directories.rs` is what says they say the same thing, in both directions: converting the
friendly source gives the foundation row for row, and rendering the foundation gives the friendly
source back.

**One file is byte-identical in both**, and that is the *sometimes* Sean allowed for: `engine.4x`
is 35 `primitive` rows with no references and names of their own, so there is nothing to rename
and nothing to generate.

| Gains from the friendly format                                                                                                              | Renders identically                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `adjacency` `argument` `binding` `clause` `column` `command` `compare` `execute` `input` `load` `reference` `residency` `state` `territory` | `primitive` `relation` `report` `role` `rule` `store` `test` |

**`column` is in the left column, which is why it is not omitted.** Its own rows cannot be named,
so a reference *to* a column is an id - but `column.relation` renders as `residency` rather than
`16`, so the table does support the format and is shown. Sean: *it is ok to omit some machinery
tables if they don't support the friendly format, but if they do support the friendly format I want
to see those too.*

## A name the foundation cannot keep is refused, not dropped

**Making friendly the source turned a harmless asymmetry into a data-loss bug**, and it took
writing the conversion the other way to see it. `territory` declares no `name`, so:

```text
{territory id:1 name:home}   ->   {territory id:1}
```

The name was **silently gone**. In the old direction that never mattered, because a generated name
was all there ever was; with friendly as the source it is an author's work disappearing in the
format they author in. **It is refused now**, and the refusal says what the generated name would
have been.

**Naming a territory therefore needs somewhere in the foundation to keep it** - a `name` column on
every relation, or a table mapping a row to a name. That is a schema decision and is not made here.

## Authoring in the friendly format, and the round trip

**Sean, 2026-09-15**: *I expect to be authoring tests in the friendly format and only
debugging/vetting in the foundation format.* So the translation has to go both ways, and it does:
**foundation to friendly to foundation is the identity for all 215 rows**, asserted per row.

**Nothing is minted.** A friendly row carries its own `id`, so translating back is resolving each
reference from a name to an id and dropping the `name` where the relation does not declare one.
No value is invented anywhere, which is why the minting question closed rather than being solved.

## The one relation that cannot satisfy the constraint

**`column`.** Fifty rows, eighteen distinct names, **sixteen of them called `id`**. It shows
up where it hurts:

```text
{binding id:1 clause:clause-1 column:id input:it}
```

`column:id` names sixteen rows, so it names none of them.

**And it is not a renaming away.** `column.name` is not a name for the row - it is **the token a
row is keyed by**: `{residency id:1 what:1 where:1}` is written with those three words because
`residency`'s columns are called `id`, `what` and `where`. Change the name and every row of that
relation is written differently.

**So two different things are wearing the same column.** A *local* token, unique within its
relation, and a *global* name, unique everywhere. `column.name` is the first and the constraint
wants the second. **`input.name` is the same shape**: `it`, `what`, `from`, `to` are local to their
rule, and a second rule with its own `to` would collide.

**Sean chose to leave it alone**, over renaming the column or adding a second one: *binding and
column are machinery*, and the friendly format is for authoring tests and reading the game rather
than the self-description. **So a reference to a column is written as an id**:

```text
{binding id:1 clause:clause-1 column:44 input:it}
```

**All or nothing, per relation.** Some column names happen to be unique - `what`, `where` - and
taking those while falling back for the rest rendered one kind of thing two ways, `column:what`
beside `column:44`. A relation whose names collide gets none.

**And a relation that declares `name` gets no generated one either.** Its `name` slot is taken by
the token, so a generated name would appear nowhere a reader could find it, and a reference to it
would be a name that resolves against nothing. That was the first version, and `column-44` named a
row that never said it was called that.

## The user-facing style, which is not part of the engine

**Sean, 2026-09-15**: *I don't consider the translation between user friendly format and
foundational format part of the engine. The engine should only know about the foundational format.
The user friendly format is for the test harness and debugging.*

**So it lives in `tests/common/friendly.rs`, and that is what keeps it free.** `src/` may name no
noun the game has, and every constant in it is a word `data/engine.4x` lists as delegated. Neither
applies in `tests/`. **The translator can be as thick as it likes and the engine does not grow a
line** - it is still 1013.

**All eight files render**, which `every_file_in_data_renders` asserts rather than claims:

```text
data/command.4x
  command-1  rule=move
  argument-1  command=command-1  input=move.it    value=residency-1
  argument-2  command=command-1  input=move.what  value=scout
  argument-3  command=command-1  input=move.from  value=territory-1
  argument-4  command=command-1  input=move.to    value=territory-2
```

## What a row is called, in three rules

**A row with no name gets one made** - Sean: *I was thinking of having a generated name for the
user friendly style, in this case `territory-1`.* The rules are tried in order, and each is there
because the one before it was not enough:

| Rule                                         | Example                                               | Why the earlier rule was not enough                  |
| -------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------- |
| The `name`, if it names one row              | `{thing id:1 name:scout}` → `scout`                   | -                                                    |
| The name qualified by what the row points at | `{column id:44 relation:16 name:id}` → `residency.id` | **Twenty-five columns are called `id`**              |
| `<relation>-<id>`                            | `{territory id:1}` → `territory-1`                    | Fifteen of twenty-five relations have no name at all |

**The second rule takes the shortest qualification that names one row.** Joining every reference
gave `move.residency.it` for an input - an input points at its rule *and* at the relation it is
typed as, and only the first of those says which input it is. It is `move.it`.

**And one reference the schema cannot state, the renderer can.** `argument.value` points at
whatever the input's `of` says, which is data rather than schema - so no `{reference ...}` row
describes it, and the translator follows the input itself. **A translator may know that; the engine
may not.**

## What is not done, and the size of it

**This renders and does not parse.** An earlier version of this section said going back could only
reproduce the original *up to renaming*, because ids would have to be minted. **Sean asked why
minting could not simply be deterministic, and it can** - that was never the difficulty.

**The narrower true statement**: the round trip is the identity only if the foundation's ids were
assigned by the same rule the minting uses. That is a constraint on how ids are chosen, not an
obstacle to translating.

**And the exposure is smaller than it reads.** Of 212 rows with an id, **100 already carry it in
the label** - `territory-1`, `binding-6`, `clause-3` - and need no minting at all. The other 112
are almost all `column` (73) and `relation` (25): **the schema and the vocabulary, which are the
stable parts.** The world state carries its own ids.

**The one real cost is what a minting rule does to stability.** Ids derived from content stop being
stable under an edit: insert a thing and, under a sorted rule, another thing's id moves - which
matters as soon as anything outside the data cites one. **Three ways out, and which is right
depends on whether the harness ever authors in the friendly style rather than only reading it**:
mint deterministically and canonicalise the foundation once; have the friendly form carry every id;
or never parse it back at all.

## The foundation style: every relation keyed by one opaque integer

**Sean, 2026-09-15**: *I want the truth of the data model to be fully normalized, which I believe
means always referencing by ids, and a single id at that. The names are decorations, but important
decorations.* And: *there will have to be two styles. The foundational style is equivalent to a
database row. The user friendly style won't be normalised, it will model user intentions.*

**The data is the foundation style now.** Every one of the 25 relations has `id` as its first
column, every reference is to one of those ids, and nothing is readable without the schema:

```text
{clause id:3 rule:1 seq:3 role:2 relation:16}
{binding id:6 clause:3 column:44 input:1}
{residency id:1 what:1 where:1}
```

**Three things fell out of it that were not obvious beforehand.**

**The engine branches on names, and the data references by ids**, so the two have to meet
somewhere. `role:2` means nothing until the row is looked up, so a value that selects a code path
is resolved to its `name` first. **That makes `role.name`, `store.name` and `relation.name`
load-bearing where every other name is decoration** - the opposite of the rule, and a consequence
of it.

**`add` must name every column and `require` must not.** Once a row has an `id`, an `add` clause
has to say what it will be - so `move` gained an input for the residency it moves, and the
residency keeps its identity across the move rather than becoming a different row. **A `require` is
a pattern**, which is what lets the adjacency clause ask whether *any* road runs that way.

**A nonsense mutation asks the wrong question of a key.** Changing an id to `mutated` leaves it
distinct, so every id in the data survived and looked dead. **An id is swapped for another row's id
now**, which collides - and that, with key uniqueness, is what made all but four of them
load-bearing.

## A key names one row, and nothing checked it

**`{thing id:1 name:scout}` and `{thing id:1 name:pioneer}` were both accepted**, and
`{residency what:1}` then pointed at neither. A reference names a row by its key, so **a key that
names two rows is a reference that names nothing** - and the foundation rests on references being
by id.

It is checked now. **Finding it needed looking for it**: no test failed, because no data had ever
had a duplicate.

**It also made two checks honest that had been passing for the wrong reason.** The reference test
added `{residency what:1 where:9}` beside an existing `{residency what:1 where:1}`, so once keys
had to be unique it was refused for the key rather than for the territory - it uses a second thing
now. And the mutation check's generated violations were a row cloned and broken, which kept the
original's key; they replace the original instead, which keeps every reference to it resolving.

## Every relation is keyed by an `id`, and `residency` is the one that should not be

**Sean, 2026-09-15**: *I want the truth of the data model to be fully normalized, which I believe
means always referencing by ids, and a single id at that. The names are decorations, but important
decorations.*

**The data did that, and then one relation stopped - which is the point rather than a regression.**
This section once reported eight relations keyed by an `id` and thirteen keyed by whatever their
first column happened to be, with `adjacency` as the sharpest case; it had gone stale with nothing
editing it, which is `docs/notes/nothing-removes.md` happening in this lane's own file.

**Today, read out of `data/foundation/schema.4x` rather than recalled: seventeen relations,
sixteen keyed by an `id` and one counted.** `residency` carries a `quantity` and is keyed by
`(what, where)`, and **that is the whole of the exception**.

**A surrogate key was not the end of it.** It admits two rows stating the same fact - two
residencies for the same thing in the same place, which the structure accepted when it was tried -
and once a residency carries a quantity that is exactly what must not be admitted. **It is refused
now**, by `two_residencies_of_one_description_are_refused`, which would have passed before.



## `load` exists and the engine still reads no file

**`{load seq:1 file:schema.4x into:game}` is a row, and `src/script.rs` never opens anything.** It
is handed something that can turn a name into text and asks it; the harness in `tests/` is what
reads the directory, which is where `std::fs` is allowed to be.

```rust
pub trait Files {
    fn read(&self, name: &str) -> Option<String>;
}
```

**That is not a dodge, it is the finding.** A command that loads a file cannot be pure data in an
engine that reads nothing, so **the file system is the first thing this game has needed from
outside itself** - and it arrives as one method rather than as a dependency. Everything else the
engine does is a function of rows it was handed.

**It also made the poison into a test.** Serving `expected.4x` from `before.4x` is four lines of a
different `Files`, so the case where the expected state is wrong is in the suite rather than
something run by hand once.

## What was here before, and what it established

**Four concepts were built on the earlier, schemaless structure and then removed** when the first
test was restated in relational form. The code is at `2610ae0` and the findings are worth keeping:

| Concept                 | Cost to `src/` | What it showed                                                                        |
| ----------------------- | -------------- | ------------------------------------------------------------------------------------- |
| A place must exist      | 0 lines        | The machinery for *not adjacent* was already the machinery for *no such place*        |
| A second rule (`found`) | 0 lines        | The engine has no word for *not*; an absence written as a fact stands in for one      |
| A number (fuel)         | 0 lines        | Over a bounded range a sum **is** statable as rows - a prediction here that was wrong |
| A turn (`grow`)         | **+111, 48%**  | A rule with no command has nothing to bind its holes, so the engine needed a search   |

**The engine was thin because the command was doing the work.** Every hole was bound by the player
typing it, so the engine only ever had to check a row, never find one. That is why the fourth
concept cost 48% and the first three cost nothing.

**And the negative rediscovered something `spec/` already says.** `{vacant place:N}` was the
complementary-place construction, reached by trying to state *found* and failing.
`spec/invariants.md` states the rule it obeys and `docs/designing-rules.md` names the construction
and the cliff it falls off. Filed as `C-128`, read and closed - it implied no proposal.

## Why the notation is re-implemented, which looks like a defect and is not

`crates/command-language` already parses this notation and **this crate does not use it**.

Sean, in `S-136`: *I don't want any dependencies or assumptions creeping in from existing code,
even the notation.* **A borrowed parser is a borrowed decision about what a line may say.**
`src/notation.rs` is 77 lines and answers to this directory alone.

The isolation is checked rather than promised, by `tests/isolation.rs`:

- the `[dependencies]` table is empty, so there is no path dependency on any crate
- `std::fs`, `include_str!` and `env!` appear in no line of `src/` that runs, so **the engine
  reads no file at all** - the tests read `data/`, and nothing reaches outside this directory
- its own `[workspace]` in `Cargo.toml`, so a half-built intermediate state cannot redden a gate
  every lane commits against

## Running it

```
cd prototypes/thin-engine
cargo test --no-fail-fast      # every binary, not just up to the first that fails
cargo run --example report     # report.html: every test, whole, failures marked
cargo run --example render     # data/friendly/ from data/foundation/
```

It is not in the workspace, so the root `cargo test` does not reach it.

**`--no-fail-fast` is not a nicety.** `cargo test` stops at the first failing target, so a run with
four red binaries shows one - which is how four stale assertions sat unseen across two commits
here, each reported green.

## Two people building one crate, and only one of them may have the binary

**Sean leaves the review server running in a terminal of his own**, which holds
`target/debug/examples/review-web.exe` open. A build that relinks it then fails with *Access is
denied* - and on 2026-09-17 this lane answered that five or six times with `taskkill /IM
review-web.exe /F`, killing his server each time without saying so.

**So this lane builds somewhere else.**

```
CARGO_TARGET_DIR=target/claude cargo test --no-fail-fast
```

**`target/` is gitignored unanchored**, so `target/claude/` needs no rule of its own and nothing
new is committed.

**Not a `.cargo/config.toml`.** Setting `build.target-dir` there would move Sean's builds as well,
which is the one thing this must not do - his server has to keep the binary it is running.

**It costs 132 MB and a rebuild of one crate**, which is quick because the crate has no
dependencies at all. An estimate of 600 MB was offered first, read off the size of the existing
`target/`; almost all of that is the test binaries and incremental state of repeated runs rather
than anything a fresh build has to produce.

**And the hooks do not collide either**, which was worth checking before relying on this:
`pre-commit` runs rustfmt and `pad-tables` and builds nothing here, and `pre-push`'s gate is
`--workspace`, which this crate is deliberately outside of.

## `report.html`, which is where a red is read

**Sean, 2026-09-16**: *I want an aesthetically pleasing and informative test report. [...] Make sure
I can see the entirety of the test and the failures are highlighted somehow.*

**Every test is shown whole**, in the friendly form and including its prose, so nothing about a test
is off the page. A row the run wanted and did not get is marked in place **inside the section that
asserted it**; what it got instead is appended beneath. A test that could not run at all carries the
refusal above its text.

**It shows the friendly file and runs the foundation one**, which is the split the two formats are
for - and `tests/directories.rs` is what says the two agree, so the page cannot show one thing and
measure another without that test going red first.

**Marking by text alone was wrong and the poison caught it.** A wanted row was marked on a `{given}`
line that happened to read the same as the `{then}` line it was about. **A given says what was
there, and nothing in it can be missing** - the mark is scoped to the section now.

**No script in the page and no stylesheet beside it**, so it opens from disk; and every mark is an
alpha over whatever the page sits on, so it reads in a light reader and a dark one - which is what
`R-10` asks of a generated drawing.


**Every check here was poisoned before being trusted**, and each failed the run it should have:

| The check                            | Poisoned by                           | What failed                                                                                               |
| ------------------------------------ | ------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| The engine gets from before to after | the `add` effect made a no-op         | `the_engine_gets_from_before_to_after` alone                                                              |
| The comparison really compares       | `expected.4x` served from `before.4x` | `a_state_that_is_not_expected_is_reported_as_both_rows`, which is a test rather than a poison run by hand |
| The engine names no game noun        | a game noun added to `src/`           | `no_relation_or_rule_the_data_names...`                                                                   |
| The engine reads no file             | a file read added to `src/`           | `nothing_in_src_reads_a_file_...`                                                                         |

**The second row became a test rather than staying a poison.** A test that the engine turns one
state into another passes trivially if the expected state is the state before, so the suite now
contains that case on purpose: `expected.4x` is served from `before.4x` and the report has to say
`NOT as expected` and name both rows.

**And the report says what it compared**, which is the other half of the same worry. If the
`{state relation:...}` rows were dropped, the comparison would scope to no relations, find no
differences, and report success in exactly the same words - so
`the_report_says_which_relations_it_compared` asserts all four are named.


## Saying you have read a test, and being told when that stops being true

Sean, 2026-09-16: *I need some way to indicate I have reviewed a test, and for that to be undone
should the test change, so I know which tests I need to re-review.* And on what the marker should
be: *I was thinking of something along the lines of a copy of the tests that I put some sort of
marker in... although I don't want to create work for myself by having to manually update the
copies when they change.*

**The copy is the marker, so there is nothing to hand-maintain.**
`cargo run --example review -- <name>` puts the current `data/friendly/tests/<name>.4x` into
`reviewed/`, and that act is the review. `report.html` then says one of three things per test:
**never reviewed**, **reviewed**, or **drifted**, with the lines that differ shown under the test.

**Whole file, whitespace collapsed.** A reworded comment counts and a re-indented row does not.
Rows-only was the first recommendation and Sean's *I don't want to miss anything* overruled it -
in the workflow this is for, the prose is changed by the same lane that changes the rows.

**It never blocks, and this lane never runs it.** An unreviewed test is not a broken one, so no
check fails on it. And a lane that could stamp its own change would be approving its own work,
which is the rule `CLAUDE.md` already has for not marking a capability vetted. **There is no
`--all`** for the same reason: approving everything at once is approving without reading.

**Deliberately, a test you edited two seconds ago reads as drifted.** That is the point rather
than a cost - Sean, weighing it: *that sounds like the whole point, I don't want to miss anything.*
It is cheap because only two workflows are supported: he approves a test, or he asks this lane to
change one and approves it afterwards.

## What the drift lines are, and what they are not

**The status is the fact and the lines are a hint.** Whole-file equality decides `drifted`, so a
reordering with no other change is caught; the set difference underneath it would be empty in that
case, and the badge would still be right.

**Verifying this meant writing stamps and deleting them.** Two copies were written, one mutated in
its prose and in a row and re-indented in a third place, the page read, and `reviewed/` then
removed - so nothing in the repository claims a test was read that was not. The first mutation
attempt changed `quantity:1`, which the friendly form does not contain; it writes `-> 1`. **The
row half of the check was green because nothing had been mutated**, and only the comment was
carrying the result.


## Reviewing from the page, which is where reading already happens

Sean, 2026-09-16: *Lets explore the idea of marking things as reviewed from the web page... The
goal is to be able to look through everything quickly and express my decisions quickly.*

```
cd prototypes/thin-engine
cargo run --example review-web
  http://127.0.0.1:7878  -  6 tests
```

`j`/`k` move, `Enter` opens, `r` marks the test reviewed, `u` takes that back, and `x` opens a box
for one line saying what needs changing. **Each key press is a write that has already happened
before the badge changes** - the page never shows a decision the disk does not hold, and says so
in red if nothing is listening rather than flipping a badge it cannot back up.

**Of the three ways a page can reach a disk, this is the only one with no hop.** The other two
were a page that hands you a command to paste, and Chrome's File System Access API, whose
behaviour from a `file://` page nobody here had checked. The hop is the whole cost of the other
two, and the hop is what was being removed.

## Two halves of a review, in two files

**An approval and an instruction are different things and are kept apart.** A copy in `reviewed/`
says *I have read this*, and it auto-invalidates when the test changes. A bullet in
`reviewed/asked.md` says *change this*, is addressed to this lane, and does not invalidate -
it is cleared by whoever acts on it.

**A note shows on a folded card.** It did not at first: the list sits in the card's body, so a
test you had not opened said nothing about having notes against it - which is the one state the
file exists to make visible. A count in the summary fixed it.

## What the page is, and what the file on disk still is

**`report.html` has no script in it and never gains one.** `build(live)` is told whether anything
is listening: served, the page carries the controls; written to disk, it is exactly what it was.
**A button that writes to the disk would be a lie in a file opened from the disk**, and that is the
reason rather than tidiness.

**The page has one definition.** `review-web` includes `report.rs` as a module and calls `build`,
so the served page and the written one cannot disagree about what a test says. A second renderer
would be a second thing to keep in step, and this repository has already been bitten by a page
that could show one thing and measure another.

## No dependency, and what that cost

**About 200 lines of `std::net` rather than a crate.** Sean: *I don't want any dependencies or
assumptions creeping in from existing code.* What it buys is small and exact: one request at a
time, enough JSON for two string fields, and no TLS - all of which is honest for a thing one
person runs on their own machine while reading.

**It binds `127.0.0.1` and refuses a name that is not a test.** Both are about it being a thing
that writes files on request: the first bounds who can ask, and the second bounds what a bad ask
can do to `reviewed/` - a copy under a name no test has is compared against nothing, forever.

**Every route was driven before it was believed**, by hand with `curl` and then through Chrome:
the page, a refused name, a refused empty note, an unknown route, a note carrying a quote, two
notes landing in one section, and the keys. Then `reviewed/` was deleted, notes included, because
this lane inventing a note would be inventing an instruction from Sean.


## Deposits: density, capacity, and one rule for every resource

Sean, 2026-09-16: *We need some tests around deposits. We need density, capacity, and some form
of parameterization to handle food/metal/energy, so this is going to be some design work too.*

**A deposit is two counted relations rather than one row with numbers in it.**

```
{density where:territory-1 what:food} -> 6
{free    where:territory-1 what:food} -> 3
```

Each is *a description mapping to a number*, which is `residency`'s shape exactly - so neither
needs a new engine word, and the two-rows-one-key check covers them as it stands. **One `deposit`
row carrying named numbers would have needed a new idea**: the key is every column but the
quantity, so two rows disagreeing about density would both be legal until the schema learned
which columns are the key and which are attributes.

**Capacity is not stated, and that is a substitution worth naming.** What is stated is `free`.
Building spends one, and taking what is not there is already refused - so the limit test needs
nothing new. The total is `free` plus the extractors standing there, which the model already
holds, and stating it would be stating a number the model can work out.

## An extractor is not a residency

```
{extractor where:territory-1 for:food}  -> 2
{extractor where:territory-1 for:metal} -> 1
```

**Extractors stop being fungible the moment there is a second resource**, so they leave
`residency` for a relation whose columns are what tells them apart. `for` is then an ordinary
named input, and `{build-extractor where:territory-1 for:food}` parameterizes by argument.

**The parameterization cost no engine at all**, which was not the expectation. A category per kind
- `food-extractor`, `metal-extractor` - would have forced the rule to derive a name from an
argument, which is string work in Rust; a column on `residency` would have put a meaningless `for`
on every scout.

**`{thing id:4 name:extractor}` is gone.** An extractor is a relation now, and the mutation check
found the category deletable the same day it stopped being read.

## `reading`: the first value a rule takes out of the world

Every column of a clause used to get its value from one of two places - a **binding**, which is
what the caller wrote, or a **literal**, which is what the rule says. Neither can express *the
density here*.

```
{binding id:21 clause:clause-12 column:45 input:work.where}
{literal id:11 clause:clause-11 column:46 value:1}
{reading id:1  clause:clause-12 column:46 of:clause-10 takes:53}
```

The third says: this clause's quantity is the quantity of the row `clause-10` matched. That is
`spec/data/line.4x`'s `` qty:`$where`'s density for that resource `` , and it is general - it is
how any rule reads a number out of the world rather than carrying one.

**A clause that matched more than one row is refused rather than read from.** Sean, 2026-09-15:
*We should never have non-determinism from what row happens to be encountered first.* One match is
remembered, several are not, and `Refused::NotOne` is what a rule gets for reading from a clause
that did not narrow to one.

**Two words, and the engine grew by no others.** `reading` and `takes` are in `data/engine.4x`,
which `tests/engine.rs` checks against `src/` both ways - so 31 words became 33 and everything
else about deposits is rows.

## What the four tests are for, and the two poisons

| Test                                                      | What it holds down                                                 |
| --------------------------------------------------------- | ------------------------------------------------------------------ |
| `an-extractor-pulls-its-deposits-density`                 | the quantity comes from the deposit, not from the rule             |
| `an-extractor-cannot-be-built-with-no-free-capacity`      | the limit, which needs no arithmetic the engine lacks              |
| `a-food-extractor-and-a-metal-extractor-do-not-interfere` | `for` is read, and one rule serves every resource                  |
| `two-extractors-on-one-deposit-each-pull-its-density`     | the density is the deposit's and is not divided between extractors |

**Both new claims were poisoned before being believed.** Setting the deposit's density to 9
without touching the rule produced 9, so the number is read rather than written. Sending
`{work where:1 for:metal}` produced two metal instead of six food, so the argument is read rather
than assumed. Each was put back afterwards.

## What adding two rules broke, which is the interesting part

**The translator stopped naming inputs, and the model was never ambiguous.** A binding names its
clause and a clause names its rule, so `input:where` on a binding of `work` can only be `work`'s
`where`. What broke is narrower: `Names::of` decides whether a relation can be referenced by name
by testing that `name` is unique across **every row of that relation**, all-or-nothing, with no
notion of a parent. Two inputs called `where` failed that test, so every input reference fell back
to an id.

**The blast radius is what gives it away, and it is what this lane missed.** `move`'s inputs are
`what`, `from` and `to`, colliding with nothing, and they fell back too:

```
left:  {binding id:1 clause:clause-1 column:44 input:1}
right: {binding id:1 clause:clause-1 column:44 input:what}
```

Nothing about `move` had become ambiguous. **A limitation of the translator was put to Sean as
though it were a property of the model** - the first version of this section said an input's name
*is unique inside its rule and nowhere else*, implying the bare form could not work, and the
reference form was changed to `input:work.where` on that basis.

**Sean, 2026-09-17**: *Wouldn't the invariant still be fine unless one rule took 2 wheres?* It
would. The names are bare again, and the uniqueness test is scoped: `input` names are qualified by
their rule for the purpose of asking whether they collide, and kept bare everywhere else. Reading
one back resolves through the binding's clause to its rule.

**Poisoned, because a round trip can be right for the wrong reason.** Resolving a bare name
without the rule makes `work`'s `where` come back as input `4`, which is `build-extractor`'s, and
`tests/directories.rs` says so line for line. The scoping is load-bearing rather than decorative.

**The mutation check was reading whichever test sorted first.** `every_reference_forbids_something`
loaded one world and needed a row of every relation a reference points at; a deposit test sorted
ahead of the movement tests, that world had no `adjacency`, and a check that had been green for
weeks said a relation had no rows. **The check was right and its world was arbitrary** - it now
violates each reference in whichever world has something to violate it in, and asserts that all
28 were violated somewhere.

**Seven written-down counts moved**, in six files: the engine's word count, the state relations
the report compares, the columns called `id`, the game's nouns and the product over the modules,
the relations in all, the references, and both mutation lists. **Not one was a defect and every
one had to be re-derived** - which is what `docs/notes/checks-outlive-examples.md` is about, seen
from the inside.


## A deposit is a thing you can run out of

Sean, 2026-09-17: *{deposit where:territory-1 density:6 what:food} -> 3* and *We can't place an
extractor if there are no available deposits.*

```
{deposit   where:territory-1 what:food density:6} -> 3
{extractor where:territory-1 what:food}           -> 2
```

Three deposits, two of them worked. **`free` is gone and so is `density` as a relation.** What was
two counted relations and a clause that spent one of them is now one relation and a fact about the
world.

## The trilemma, and which of the three went

Sean, naming it: *I want to not have to specify anything more if there only happens to be 1
choice / I want the option to have more than one choice / I want the notation to be uniform
regardless of number of choices.*

**All three turn on one question - is density part of the key?** Keep it in and two grades are two
legal descriptions, so a command has to say which, either always or only when it is ambiguous; the
first costs minimality and the second costs uniformity. Take it out and the description is
`(where, what)`, both of those are kept, and a second grade is refused.

**The specification asks for one density, twice.** `spec/planet.md`: *For each resource, a
territory has capacity for some number of extractors, and a density that each of them yields.*
`spec/economy.md`: *The territory's density for a resource is what each extractor pulls from it
each turn.* So the option that was dropped is the one the game does not use.

**It cost one row**, and the default rule did not move:

```
{attribute column:53 relation:deposit}
```

**And dropping it only means anything because the second grade is now refused** -
`a_deposit_cannot_have_two_densities` in `tests/structure.rs` is what says so. With density in the
key it would have been a legal state that nothing could name.

## The limit is a reference with a number on it

```
{limit held:extractor by:deposit}
```

**No rule mentions capacity.** `build-extractor` removes a labor, removes a metal and adds an
extractor; the world it would leave is checked as every world is, and an extractor with no deposit
under it does not fit. **So the limit binds rules that have not been written**, which is the
argument for putting it here rather than in the rule that happens to exist today.

**A full deposit and an absent one are one refusal.** A row at quantity zero is never written, so
the two differ only in the number, and the refusal names the row that would have had to be there:

```
{refused}
{deposit where:territory-1 what:food} -> 2
```

Reading *there is no deposit here with room for two*, where a plain *too many* would have named
the symptom.

**Held and holder are compared key for key**, and a limit between relations that disagree about
what a row is keyed by is refused rather than matched on whatever they happen to share.

## Two things the checks caught, and one clippy did

**The limit ran before references and answered the wrong question.** Point a deposit's `where` at
a key nothing has and the extractors over it are suddenly over nothing, so `tests/mutation.rs` got
*too many extractors* where it had asked about a dangling reference. **The narrower fault is the
one to report**, so the limit runs last.

**A density nothing reads.** A deposit declares a `density` column, so every deposit row carries
one - and the test about running out of room never gets as far as working the deposit. It is in
`NOT_LOAD_BEARING` beside that test's `thing.name`, for the same reason: a command that is refused
reads less of its world than one that succeeds.

**And the error type got big enough to pay for.** `Malformed` gained a variant naming a row and a
number, which pushed `Refused` and `Failed` over the size clippy warns about - a cost paid on every
call that succeeds. Both now box the `Malformed` they carry.

## Poisoned, again, and what each one showed

| Poison                                 | What it proved                                            |
| -------------------------------------- | --------------------------------------------------------- |
| deposit `-> 1` becomes `-> 2`          | the limit compares quantities rather than always refusing |
| an extractor with no deposit, at load  | detection does not wait for a rule to run                 |
| two deposits differing only in density | the attribute row is what makes the second grade illegal  |

**The middle one is Sean's sentence run twice.** *The situation should be detectible and therefore
preventable*: detection is `Game::of` refusing the world, and prevention is the same check running
after a rule. There is one check and two places it fires.


## A kind is a relation

Sean, 2026-09-17, on a deposit, an extractor and a labor standing in one territory: *I notice that
in territory-1, we have a deposit, an extractor, and a food. Yet the way we specify this is
different.*

It was. Two of them had their kind in the schema and one had it in a column:

```
{deposit   where:territory-1 what:food density:6} -> 1
{extractor where:territory-1 what:food}           -> 1
{residency what:labor        where:territory-1}   -> 1
```

Now every kind is a relation, `where` says where it is, the remaining columns are that kind's
traits, and the quantity says how many:

```
{deposit   where:territory-1 what:food density:6} -> 1
{extractor where:territory-1 what:food}           -> 1
{labor     where:territory-1}                     -> 1
{food      where:territory-1}                     -> 6
```

**`residency` and `thing` are both gone.** `residency` was the relation for things with nothing to
say about themselves, and everything else grew out of it - which is why the extractor had to leave
it the day it gained a trait. **`spec/data/carries.4x` was already written this way**: a kind and
the traits it carries is a relation and its columns.

## The thing that made it possible, and the thing that made it necessary

**A clause can take its relation from an argument.**

```
{clause id:12 rule:work seq:4 role:add relation:resource name:clause-12}
{relation-of clause:clause-12 input:what}
```

So `work` adds to whichever resource the command named, and one rule still serves them all.
Without this, a kind being a relation would have forced a rule per kind - *the very thing this
prototype exists to avoid*.

**And that is what made families necessary.** A column id belongs to one relation, so a clause that
might be about `scout` or about `food` has no single column id to bind. **The family declares the
shape**: it is an abstract relation, columns and no rows, and its members must declare those
columns - which `Malformed::UnlikeShape` is what says.

```
{relation id:26 name:unit}
{column relation:unit seq:1 name:where}
{column relation:unit seq:2 name:quantity}
{family relation:unit}
{member kind:scout family:unit}
```

## Both of Sean's considerations were one mechanism

Sean, 2026-09-17: *I had considered exists/not-exists traits, such as "movable", to filter out what
is able to move / I had considered sets, such as "resource"=[food, metal, energy], so that I could
have generic recipes that only work on resources.*

**A predicate is a row that is there or is not, and a set is the rows that are there.** One
relation for all of them costs a row per membership where a marker per predicate costs a relation.
**And the specification already writes it that way** - `spec/data/families.4x` and `member.4x` -
with `spec/data/for.4x` using it for the generic recipe: `{for block:build-extractor seq:3
kind:$resource}`.

## What it bought, which was the open question

**An input typed by a family ranges over exactly its members.** `move.what` is `of:unit`, so a
deposit is not offered to `move` because the game says a deposit is not a unit - rather than
because `move`'s add clause happens to leave a `density` unbound, which is how it would have been
excluded under the alternative. **The exclusion is principled rather than accidental**, and
`offered` still gives exactly one command per test world.

## What it cost, measured against what was estimated

|                           | estimated               | actual                                                          |
| ------------------------- | ----------------------- | --------------------------------------------------------------- |
| rows                      | +26                     | **+30** (393 to 423)                                            |
| new engine concepts       | 2                       | 2 - `relation-of` and the family                                |
| `src/`                    | ~25 lines in two places | `keys_of`, `has_key`, `row_of`, and the schema reading families |
| written-down counts moved | 10                      | **15**                                                          |

**The count of counts is the number I was most wrong about**, and by the largest margin. Ten was
read off the last two increments; this one moved every state-relation list, every relation count,
four tests that named `thing` or `residency` in an assertion, and the shared-file count.

## Three things the checks found

**No test states food in its `given`.** The reference check asks that every reference be violated
somewhere, and `food.where` could not be - because food is produced by `work` and stated by
nobody. **The check now names what it cannot reach rather than counting it**, because *36 of 37*
left a reader to find which one, and the one it was turned out to be worth knowing.

**A family's columns can carry no reference.** A reference on a relation with no rows forbids
nothing, ever, so `unit.where` and `resource.where` declare none - the shape is the column names
and each member declares its own.

**Both mutation lists got shorter, which has not happened before.** Three literals that named a
kind are gone rather than dead, because a clause's relation says it now; two bindings went the same
way. `4 things.4x thing.name` left the list by the file leaving. **A unification that removes dead
data is a different kind of evidence from one that reads better.**


## Readiness is a quantity, so it is a relation

Sean, 2026-09-17, reviewing `a-food-extractor-and-a-metal-extractor-do-not-interfere`: *I notice
that an extractor can be used as many times as we have labor for.* And on how to fix it: *we are
going to have multiple kinds of ready, and rediness is going to end up being a number of times we
can do something rather than whether we have done something.*

```
{extractor where:territory-1 what:food} -> 1
{working   where:territory-1 what:food} -> 1
{labor     where:territory-1}           -> 2
```

One extractor, one work in it, two labor - and the second `work` is refused. **`work` spends a
labor and a readiness**, not one or the other.

## Why it is not a column on the extractor

**`{extractor where:t1 what:food working:1} -> 2` is the obvious answer and it costs an engine.**
Going from `working:3` to `working:2` is arithmetic on a named column, which the engine does not
do - it does arithmetic on the quantity and nowhere else. That would have needed a pattern meaning
*at least n* and a supplier meaning *one less*, which are `spec/data/constraint.4x`'s `at-least`
and `one-less`. **Sean asked for a solution that did not presume the specification's**, and this
one does not: running out is the refusal `take` already gives, and the number of times is the
quantity.

**What it gives up is which extractor was worked, and that is the point.** Sean: *not being able
to tell which extractor operated is a feature, not a bug. I expect to use the same idea for
containers and loss due to disorder, we only lose what we don't have the storage for, without
tracking what is stored where.* **Two extractors with one work each and two extractors sharing two
works cannot be told apart by anything the game can ask**, because extractors are fungible - so
the column version pays an engine concept for a distinction that is unobservable.

## Keyed like the thing that has it

**`working` has `extractor`'s key**, `(where, what)`, which is what makes two things true at once.
The `limit` can say an allowance never exceeds the things holding it:

```
{limit held:working by:extractor}
```

And one extractor's readiness is no use to another. Sean: *Will this still work if we have 1 food
extractor and 1 metal extractor and 2 labor? I don't want be able to run the food extractor
twice.* **`one-extractors-readiness-is-not-anothers` is that question as a test**: everything the
second command needs is present except the readiness - labor for it, a deposit under it, an
extractor to do it - so what refuses it is the key and nothing else.

**One relation per kind of readiness**, named for the activity, which is what `spec/data/traits.4x`
does with `working`, `moving`, `laboring` and `bearing` as separate traits.

## Refreshing needs nothing new either

Remove the whole `working` row, require the extractor, add a `working` whose quantity is read off
the extractor count. **`reading` already does the last part**, so a turn boundary is rows when
somebody wants one. It is not written yet, because no test asks for one - and a built extractor
therefore starts unready, which is a consequence worth knowing rather than a decision.

## Four poisons, and one of them measured the hole

| Poison                                      | What it showed                                                 |
| ------------------------------------------- | -------------------------------------------------------------- |
| the readiness clause out of `work` entirely | four tests notice, and the new one is among them               |
| two works instead of one                    | the second `work` succeeds, so the refusal is about the number |
| a labor added to the no-labor test          | it succeeds, so that refusal is about the labor                |
| food given two works beside metal's one     | the second `work` succeeds, so the refusal is about the key    |

**The first is the red half of red-green**, done after the fact: the rule and the test were written
together, so taking the mechanism back out is what shows the test would have failed without it.

## What the mutation check said, and one thing it cannot say

**`{limit held:working by:extractor}` reads as deletable and is not.** What that check measures is
whether a **data test** reads a row, and none can: a world stating more works than extractors does
not load, and a test whose world does not load has no outcome to state.
`an_allowance_cannot_exceed_the_things_that_have_it` in `tests/structure.rs` is what holds it.
**Worth saying out loud, because the list otherwise reads as *nothing needs this*.**

**And three tests state a readiness they do not spend**, each a refusal that stops before reaching
it or refuses on it either way. They are there so a reader can see which of the two costs the test
is named for, and the line in `DELETABLE` is the price of that said rather than trimmed.


## What `{refused}` names, and why it is compared exactly

Sean, 2026-09-17: *What does the refused section actually list? Are they the commands that were
refused to carry out, the state that refused to come into existance, or the row that was missing
that cause the refusal?*

**The third.** Not the command, and not a state - a refused command leaves none, because the fold
stops and the world before it is discarded.

**Three refusals produce a row; everything else produces prose no row can equal.** A test naming
one of those others is red however right it looks.

| Refusal           | The row                                       | What its quantity is                                   |
| ----------------- | --------------------------------------------- | ------------------------------------------------------ |
| `NotSo`           | `{adjacency from:territory-1 to:territory-3}` | none - `adjacency` is not counted                      |
| `NothingToRemove` | `{working where:territory-1 what:food} -> 1`  | what the **rule** tried to take, a literal in the rule |
| `Overfull`        | `{deposit where:territory-1 what:food} -> 2`  | what the **world** would have needed, derived from it  |

**The last two read alike and behave differently**, which is worth knowing when one goes red. A
wrong number in a `NothingToRemove` row means the rule's literal was mis-transcribed; a wrong
number in an `Overfull` row usually means the `given` moved underneath it.

**And `Overfull` is not a missing row at all.** The deposit exists, with room for one. What the
test names is the row that would have had to be there instead - *there is no deposit here with
room for two* - which is a different statement from *this row is absent*, though the section holds
both.

## Exact, deliberately

**The comparison is string equality between the row the test writes and the row the refusal
produces.** Measured three ways on `an-extractor-cannot-be-built-where-the-deposits-are-taken`,
whose refusal says `-> 2`: writing `-> 3` is red, writing `-> 1` is red, and omitting the quantity
is red. **There is no subset matching, no threshold, and no way to say *never mind the number*.**

**The loosening is available and was not taken.** Letting a `{refused}` row omit the quantity and
match on the rest would make the `Overfull` case survive an edit to the test's `given`. Sean,
2026-09-17: *i will keep it exact for now unless i find a reason not to.* **Looser matching is how
a test starts passing for a reason nobody chose**, and the cost of exactness is re-deriving a
number that a check will name for you when it changes.

**One thing it deliberately does not do**, and Sean's reason for leaving it: `{refused}` is the
same whether the first command of a `when` was turned away or the second. He, on whether to add
it: *I don't need to say which command was refused on a multi line command, don't want to
encorage too many lines in the test.* **The cost is real and is written down** -
`an-extractor-cannot-be-worked-twice-on-one-readiness` cannot tell *refused on the second* from
*refused on the first*, and what pins it is the test beside it showing one work succeeding.


## A place has room, and each kind takes up a different amount of it

Sean, 2026-09-17: *I am thinking of inventing a resource that vehicles take up and having a certain
limit per territory that is the same across all territories. A lot of real time strategy games do
this, and I know the board game twilight imperium does this as well.*

```
{pool  id:1 name:berth per:territory n:6}
{draws id:1 kind:scout     pool:berth n:1}
{draws id:2 kind:transport pool:berth n:2}
```

Per place, the sum of *how many × what each takes* must not exceed the pool. **No rule mentions
berths** - a move that would overfill a territory leaves a world that does not fit, and every rule
already refuses that, so this binds rules nobody has written.

**This is the half `spec/data/limit.4x` cannot write.** `{limit container:territory contained:ark
n:2}` counts arks, and a transport worth two scouts has nowhere to be said there. **The
specification's version is a count and this one is a weight**, which is the first place the
prototype is ahead on something Sean asked for rather than on something it noticed.

## Two things about the shape

**The allowance is declared once, not stated per place.** The alternative was `{berth
where:territory-1} -> 6` per territory, which needs no engine and is the deposit pattern exactly.
At two hundred territories that is six hundred rows saying the same thing - **data growing with
data, which is the same failure as code growing with data** and against *the model is a minimal
expression of intent*. A deposit legitimately differs per territory; an allowance Sean has said is
uniform does not.

**The place column is found rather than named.** A pool says `per:territory`, and a drawing kind's
place is whichever of its columns references `territory`. **The engine could have read `where`
instead**, and then a column name of the game's would have been a word the engine branches on -
which is the boundary `data/engine.4x` exists to keep.

## What the checks said

**A `{refused}` row is not a world row, and one check thought it was.**
`a_scenario_states_a_world_and_an_act_and_nothing_else` requires every row of a section to be a
state relation, and a crowded territory is refused for want of a bigger `{pool ...}` - which is
the ruleset's. **The check was right about `given` and `then` and wrong about `refused`**: rows
there are written out and compared as text and never enter a store, so a scenario naming one
declares nothing. Relaxed to *must be a declared relation* for that section alone.

**The mutation check found nothing to delete, which has not happened before.** Every row the pool
added is load-bearing: the two refusal tests turn on the allowance, the rate and the kinds. What
it did find is an `adjacency.id` in each of those tests - a `move` reads `from` and `to`, and the
id is compared only by a `then`, which a refusal test does not have. **The test beside them has
one and does not leak**, which is the difference showing rather than a fault.

## Two poisons

| Poison                             | What it showed                                           |
| ---------------------------------- | -------------------------------------------------------- |
| a transport drawing 1 instead of 2 | both refusals go green - the weight is what refuses them |
| the pool raised from 6 to 7        | the same, so the allowance is read rather than assumed   |

## And a cost that has started to show

**The mutation check now takes about ten minutes.** It deletes each row and re-runs every test, so
its cost is rows times tests - and today both roughly doubled. **It has stopped being something to
run while waiting**, which is worth knowing before it stops being run at all. Nothing is wrong
with it; the shape is quadratic and the data is growing.


## Provides and consumes, where you can read them

Sean, 2026-09-17, on the berth system that preceded this: *I don't like the current berth system
and it is good to have an option to keep it out by not approving any of these tests.* And on why:
*there are certain things I always want to see in tests because I need to compute the tests in my
head. So something like the berth system, I need see what the actual numbers are, not some hidden
default.*

```
{provides kind:territory what:berth} -> 6
{consumes kind:scout     what:berth} -> 1
{consumes kind:transport what:berth} -> 2
```

**They are state, stated in each test's `given` and repeated in its `then`.** So a test reads
whole: a territory provides six, two transports and a scout are five, and a third transport wants
two more.

**`pool` and `draws` are gone** - two relations and eight columns replaced by two relations and
six, with the numbers moved from the ruleset into the world.

## Why the number is a quantity and not a column

Sean offered four spellings: `amount:6`, `quantity:6`, `value:6`, and `-> 6`. **The last two are
the same row** - `-> 6` is how the friendly form writes a quantity - and the argument for them is
the key, not the look.

**A relation with no quantity is keyed by its first column alone.** So
`{provides kind:territory what:berth amount:6}` is keyed by `kind`, a territory could provide only
one thing, and `{provides kind:territory what:slot amount:4}` would be refused as a duplicate key.
With the quantity, the key is `(kind, what)`: one row per kind per thing provided, and a second row
saying four is refused - which is the uniqueness wanted, got for nothing.

**`value:` has a second problem**: it is already `literal.value`, a word the engine branches on.

**And it passes the test the model already has for this** - *a number is a trait when two rows
differing only in it can both be true, and a quantity when it says how many of one description
there are.* Two providings of berth by a territory cannot both be true, so it is not a trait.

## A supply is declared once; its amounts are not

```
{supply id:1 name:berth per:territory}
```

**`per` says what the supply is measured in**, so the check does not have to work out which
relation every provider and consumer has in common. It is the one part of the old `pool` worth
keeping.

**And it makes a typo impossible.** `what` references a supply, so
`{consumes kind:scout what:berht} -> 1` is refused when the world is read rather than silently
meaning *scouts are unlimited*.

## The shape storage needs, reached without another idea

**A provider either is the place or is in one.** A territory provides berths at itself. A store
would provide room at the territory it stands in, and a place's capacity is then the sum of what is
in it that provides - `spec/logistics.md`: *a place's capacity for a kind is the sum of what is in
it that can hold that kind.*

```
{provides kind:store what:metal} -> 10
```

**The pool could not say that at all**, because its allowance was one constant rather than a sum
over what is present. That is the reason to have replaced it rather than patched it, and it is a
better reason than legibility.

## The layer that bent, and the ruling that bent it

**A scenario is now stating a rule**, which `a_scenario_states_a_world_and_an_act_and_nothing_else`
exists to prevent. Sean, 2026-09-17: *the layers weren't meant to hide information relevant to
understanding the test. They may still be a good idea that is simply superceded by test
comprehension being more important.* **That precedence is in `layers.md`** rather than left to be
re-argued, because the layer argument is the one that was written down and would otherwise win by
default.

**A test may now set up a world where territories provide ten**, which for a prototype is a
feature.

## Two poisons, and a metric this lane will stop using

| Poison                                     | What it showed                                         |
| ------------------------------------------ | ------------------------------------------------------ |
| a transport consuming one instead of two   | both refusals go green - the rate is what refuses them |
| a territory providing seven instead of six | the same, so the provision is read rather than assumed |

**Sean, 2026-09-17**: *I don't really care if a change causes every test to have to be reviewed.
This is prototyping and I am going to be making drastic changes to see what happens. So how many
tests need to be rewritten should not be a metric used to judge a change as bad.* **This lane had
used it twice** - six hundred rows at two hundred territories, and every test changing - and both
arguments are withdrawn.


## An allowance is a column of the thing, and `put` is how it is restored

Built from `Q-95`. **Sean designed this in a conversation with the quality lens believing it was
this lane**, so the decisions are his and the record is
`lenses/quality/2026-09-18-what-the-refresh-conversation-settled.md`. He authorised building it
here separately; the lens said not to act on its say-so, and that was right.

```
{scout where:territory-1 moving:1} -> 2
{scout where:territory-1 moving:0} -> 3
```

**Two rows, because the allowance is part of the description.** Sean, 2026-09-18: *I want to make
sure that we don't have one scout allowed to move 3 times because there are 3 scouts with one move
each present, or allow a scout that has moved on a previous turn to pick up a move from a scout on
the place it moved to.* Grouping is what forbids both, and it needed no engine at all -
`Relation::key()` is every column but the quantity, so a trait joins the key by existing.

**`working` stops being a relation.** It was one because there was no `put`; it is a column of
`extractor` now, and `{limit held:working by:extractor}` went with it - that limit would otherwise
have been a second statement of a cap the maximum already gives.

## `put`, and the question the record left open

**A put finds rows and says what is true of them afterwards.** It is needed because refreshing
something already topped off must be a no-op, and `remove` refuses when nothing matches.

**Sean, 2026-09-18**, on why that is right rather than convenient: *This is not a transformation
recipe, it is creation. It is similar in nature to something generated by time. The resources that
come from the ground and sun are infinite but rate limited. So are moves.*

**The open question was which columns match and which assign.** The record's guess was *bindings
match, literals assign*, offered as a guess and not a recommendation. **It cannot work, because a
literal already matches**: `{literal id:13 clause:clause-3 column:98 value:1}` is how `move` picks
the `moving:1` group out of a place holding both, on a clause whose role is `remove`. **Which a
value came from cannot decide what it is for.** So the split is its own statement:

```
{binding clause:clause-15 column:72 input:where}   matches, as for require and remove
{assigns id:1 clause:clause-15 column:98 value:1}  what is true afterwards
```

**A put conserves the count**, which is what separates it from a remove and an add that could drop
or duplicate. Two groups assigned the same description become one, because a store is a set.

**The evidence first given for that refutation was stale by one increment, and the conclusion
survived it.** It cited `{literal ... value:labor}`, which was true until the kinds unification
deleted it - a clause names its relation directly now, and all thirteen literals carry numbers.
**The quality lens caught it by re-deriving the claim and finding nothing there**, which is the one
thing no check does: *a claim that arrives finished is the one to re-derive*. The better evidence
was inside the change being argued for.

## Two refresh rules rather than one, which was this lane's call and did not survive

**Withdrawn on 2026-09-18.** What stood here said one rule cannot do both, *because the column
assigned is named in the rule and a scout has no `working`* - and cited `spec/data/block.4x`'s six
refresh blocks as precedent rather than workaround.

**The premise was about one line of the prototype and was presented as a property of the model.**
The column assigned was named in the rule because `{assigns ... column:98 ...}` named a column;
nothing required that. Sean: *I don't like that I have a separate refresh command for each
resource, I feel like this should be parameterized somehow.* The section below is what replaced it.

**The precedent was not wrong and was not load-bearing either.** Six blocks share the recipe
`refresh` in the specification, which says a block is where a rule is pinned to a kind - it does
not say the prototype's `rule` must be the analogue of a block. **That was the choice being
defended, dressed as the reason for it.**

## Three things in the drafted tests that were wrong

**The second test's `{refused}` asserted a refusal and the world it would have left.** It cannot:
`Failed::BothEndings` refuses a test that states both a `then` and a `refused`, and `{refused}`
names the row the rule needed and did not find rather than a world. It names
`{scout where:territory-2 moving:1} -> 1` now. **The lens flagged this as the thing to check
hardest and was right to.**

**The first test moved a scout back along a one-way border.** `adjacency` is stated one way, so the
return move would have been refused for want of a border rather than for want of a move. It has
`{adjacency id:2 from:territory-2 to:territory-1}` now, which no other test has.

**And `the-scout-crosses-two-borders` could no longer cross two borders.** A scout has one move, so
that test was the first casualty of the rule. It has a `refresh-moving` between its two moves now,
and says what crossing twice costs.

**The sizing the lens gave was exactly right** - 8 of 17 tests, 21 rows, 6 stating a `{working ...}`
row - and I re-counted all three rather than take them.

## What the change forced elsewhere

**The deposit limit had to stop comparing keys.** `{limit held:extractor by:deposit}` required the
two to be keyed alike, and `working` joining the extractor's key broke it. **A deposit holds an
extractor whatever state it is in**, so the container's key is now a *subset* of the held key and
the held rows are summed over it - which is also what stops two groups each fitting while together
they do not.

**Every refresh is offered, everywhere.** A put never refuses, so `offered` lists
`{refresh ...}` for every place and every kind that carries the trait. **In the specification every refresh is
`owner:world`** and would not be offered to a player at all - the prototype has no owner, because
Sean deferred treating the turn as a resource, so refresh is fired by hand from a `when`.

**And a test about the one-way corridor had to refresh first.** After moving, the scout has no move
left and *nothing* is offered - which would have hidden the corridor behind a spent allowance
rather than shown it.

## One refresh, and the trait is an argument

**`{refresh where:territory-1 what:scout trait:moving}`.** Three arguments: the place, the kind or
family to reach, and which allowance to restore. **There is one rule**, and `refresh-moving` and
`refresh-working` are gone.

**The trait is the column's name, so nothing has to say which column.** `{trait id:1 name:moving}`
is the vocabulary and `{assigns id:1 clause:clause-15 input:trait value:1}` is the rule reading it
off an argument - where it read a column id before. A column named `moving` is what a `moving`
restores, in whatever relation the other argument denotes.

**`what` is typed as a relation and not as a family**, which is what lets one rule reach both:

| What is written                                       | What it denotes                               |
| ----------------------------------------------------- | --------------------------------------------- |
| `{refresh where:territory-1 what:scout trait:moving}` | `scout`                                       |
| `{refresh where:territory-1 what:unit trait:moving}`  | `scout` and `transport`, the family's members |

Sean, 2026-09-18, on what the parameterisation had to reach: *I should be able to declare separate
things with separate commands, as well as explicitly declare group commands. So i can refresh a
scout, refresh a transport, refresh all movable, refresh all workable, and their intersections.*
**The intersection needs no third mechanism** - it is the pair of arguments, and `offered` lists
all of them: three places by three denotations for `moving`, plus three for the extractor's
`working`.

**A clause is done once per relation its argument denotes**, which is one sentence for all four
roles rather than a special case for `put`. Only `refresh` can receive a family today, because it
is the only rule whose input is typed as a relation - a fact stated in the data rather than a
branch in the engine.

## `carries` is two statements of one fact, and both are kept

`{carries kind:scout trait:moving}` says a scout has a move to spend; `{column id:99
relation:scout seq:2 name:moving}` is where the number lives. **That is duplication, and it stays.**

Sean, 2026-09-18: *One reason I resist duplication is to guard against the inconsistency. Another
reason is to keep the model simple. Inconsistency can be mitigated by automated checks. Simplicity
is more important from the expression side that I audit than it is for the implementation details.*

**So the check is the mitigation, and it runs both ways**: a kind that carries a trait declares a
column of that name, and a column named for a trait is carried by the relation declaring it.
**Neither direction implies the other** - without the first a `carries` row could name a column
nothing declares; without the second a column could hold an allowance no rule can reach, because
`refresh` finds a kind through `carries` rather than through its columns.

**Two words, and the count is what says so.** `data/engine.4x` went from 48 to 50, which
`tests/engine.rs` asserts precisely so that adding one is a decision rather than a line.

## What the mutation sweep caught here, twice

**Both halves of the `carries` check answered ahead of the reference check**, and both times the
sweep is what said so. It poisons every reference in turn and reads *which* check complained: point
`carries.kind` at a key nothing has and the first version answered *nothing-has-this-key carries
moving and declares no such column*, the second *`unit` declares `moving` and does not carry it* -
neither of which is a dangling reference.

**The fix was to move the check after the references**, where `{limit held:extractor by:deposit}`
already sits for the same reason and says so in a comment this lane had read and not applied.
**The narrower fault is the one to report.**

**And the reason the check gives for itself went stale on the way.** The by-name arm was correct
while the block lived in `Schema::of`, which `tests/directories.rs` hands the friendly rows; moving
it to `check` - reached only from `Game::of`, and only with foundation rows - made the arm dead and
its comment false, and both travelled unchanged. Caught by re-deriving the claim rather than by any
check, which is what `docs/working-with-an-assistant.md` is about.

## A rule is a leaf or a composite, and the composites make a tree

**`{part id:1 of:end-turn is:refresh seq:1}`** says ending a turn refreshes first;
**`{argument id:1 part:part-1 input:what value:unit}`** says which refresh. **It is the clause
layer one level up and deliberately the same shape**: a clause has an id and `{binding ...}` rows
name it, a part has an id and `{argument ...}` rows name it. Nothing here is a new idea about how a
rule is described.

**Sean, 2026-09-18**, on what the structure has to be, and why:

> Whatever it turns out to mean, it must be able to organize the entirety of game rules is some
> type of acyclic graph or tree. Otherwise it will be impossible for a human player to understand
> how to play the game.

**Two graphs, and only one of them can be that.** What a rule produces that another consumes is
already cyclic here - `build-extractor` takes metal and makes an extractor, `work` takes an
extractor and makes metal - and **that cycle is the economy**. Removing it removes the game. What
is checked is *containment*: which rule is a step of which. A weighting is what makes the other one
safe, and acyclicity is not available as an alternative to it.

**Three refusals keep it a tree**, and `the_rules_are_a_tree` states all three with a control:
`TwoParents`, `CycleOfParts`, and `BothLeafAndComposite`. **The cycle check is what lets `run`
recurse with no depth counter** - a composite cannot reach itself in a world that loaded.

**One composite may name a rule twice.** `end-turn` refreshes once for `moving` and once for
`working`; those are two steps of one order rather than two parents, and the check counts distinct
parents rather than part rows.

## The roots are the player's menu, and that is one fact rather than two

**A rule that is somebody's part is fired by that somebody.** So `offered` skips any rule a
`{part ...}` names, and nothing declares an owner - `spec/data/block.4x` writes `owner:world` and
here the tree says it. **The offering went from thirteen commands to two**: `{end-turn}` and the
one legal `{move ...}`.

**Fired by name and offered to a player are two different questions**, and this lane had collapsed
them. Sean:

> Why can't refresh be both a player command and part of the turn. It seems it will make it easier
> to read tests across multiple turns if i see an end turn command, and it will be easier to test
> the end turn command itself if i can test its parts.

**He was right and the restriction was mine, not the model's.** `fire` takes `refresh` by name, so
`refresh-makes-one-entry-of-a-spent-scout-and-a-fresh-one` tests the part on its own and keeps its
own review stamp; `ending-a-turn-restores-a-scout-and-an-extractor` tests the composite; and
`the-scout-crosses-two-borders` reads as `move · end-turn · move`, which is what a turn boundary
looks like in a test.

## What the turn did not need, which is a turn

**Refresh does not gain resources.** A weighting exists over the four rules and `move` against
`refresh` nets exactly zero under it, so a nogain check would have passed this prototype - while a
scout crossed a border **50 times in a row**, measured. **The invariant was green and the game was
broken**, which is this repository's named recurring failure arriving in a new place.

**What refresh made infinite was tempo, not resources.** So the fix is that a player cannot choose
it, and **no turn token is needed**: within-a-turn acting is bounded by the allowances already, which
is `spec/turn.md`'s *when no thing has a count left, there is nothing left to do*. Ending a turn
repeatedly is not a hole - that is time passing.

**A source place earns its keep when something creates from nothing** - a star providing energy -
because then nogain needs it in the arithmetic. `reports/nogain.md` already treats `time` that way:
*a source is a place they take from rather than an exemption from the arithmetic*. Not yet here.

**`refresh` lost its `where` on the way.** Time does not visit one place. **Checked against the
tests rather than assumed**: the four that refresh have one territory or one scout between them, so
none of them could tell a per-place refresh from a global one, and none ever needed to.

## `tree.txt`, which is the artifact the specification never had

```text
end-turn
    1. refresh  what:unit trait:moving
    2. refresh  what:extractor trait:working
```

**`cargo run --example tree` writes it and a test compares against it**, so a rule added to the
tree makes that test red and the only way to green it is to regenerate and read what changed.

**Sean, 2026-09-18**, on why this was worth building before anything else in the turn:

> the spec had no artifact whose whole structure you could read at once

**A root shows what it takes and a part shows what it is given** - `what:unit` under a root is the
type of an argument, under a part it is the argument. **Four rules of prose cannot say what those
five lines say**, which is the whole argument for generating it rather than describing it.

## What was deliberately not built

**No per-kind maximum row.** Sean: *I am not sure I am ready for treating turns as a resource yet,
it feels like trying to unify the 4 fundamental forces of nature before understanding what they
are.* While every maximum is 1 it is a literal in the rule, so nothing has to be unwritten later -
and the word for it (`readies`, `allows`, `affords`) arrives with the concept that needs it.

**No turn.** Refresh is a command like any other.

**And one case is untested**, named here because it is the fifth test Sean capped out: *refresh
where there is nothing to refresh*. The four cover a group already topped off - that is what makes
`refresh-makes-one-entry` come to five rather than refusing - but no test refreshes an empty place.
It is in `backlog.md`.

**None of this is in `spec/`.** Sean: *It is prototype research, so whatever survives reaches the
specification by promotion through the spec lane, the normal way - building it doesn't settle it.*
