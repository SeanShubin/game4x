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

## The working behind `P-548`, answered on 2026-09-24

**Not an item** - `P-548` is a proposal and lives in
[`decide/proposals.md`](../../decide/proposals.md).

**The table is kept because it is the measurement**, and because the rewrite deliberately
does not land it: a 43-row list of directories restates what the columns already say and
goes stale the next time one is added. **What lands instead is the two missing sentences
and a check.**


**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** rows · **asks** a decision · **into** `CLAUDE.md` -> Perspectives

**You asked for the full list.** Here it is, built against the filesystem and asserted: every
top-level directory and every tracked root file appears exactly once.

**It asks a decision rather than approval because three cells are this lane's choice**, not
something derivable from what the file already says. They are the bold ones.

| Path                | Who writes it                                  | Where that comes from      |
| ------------------- | ---------------------------------------------- | -------------------------- |
| `spec/`             | Specification, by promotion                    | stated                     |
| `spec/data/`        | Specification, by promotion                    | stated, as `spec/`         |
| `spec/future/`      | Specification, by promotion                    | stated, as `spec/`         |
| `spec/tests/`       | Specification drafts, Sean approves            | stated                     |
| `releases/`         | Specification, by promotion                    | stated                     |
| `docs/`             | Specification                                  | stated                     |
| `docs/notes/`       | Specification                                  | stated, as Claude's        |
| `docs/postmortems/` | Specification                                  | stated, as `docs/`         |
| `docs/prototypes/`  | Specification                                  | stated, as `docs/`         |
| `docs/recipes/`     | Specification                                  | stated, as `docs/`         |
| `docs/theory/`      | Specification                                  | stated, as `docs/`         |
| `decide/`           | **Specification**                              | **nowhere**                |
| `tools/spec/`       | Specification                                  | stated                     |
| `README.md`         | Specification                                  | stated                     |
| `CLAUDE.md`         | Specification, and its columns need Sean       | stated                     |
| `crates/`           | Code                                           | stated                     |
| `web/`              | Code                                           | stated                     |
| `prototypes/`       | Code                                           | stated                     |
| `scenario/`         | Code                                           | stated                     |
| `reports/`          | Code                                           | stated                     |
| `hooks/`            | Code, production support                       | stated                     |
| `scripts/`          | Code, production support                       | stated                     |
| `.github/`          | Code, production support                       | stated, as CI              |
| `tools/anchor/`     | Code, production support                       | stated                     |
| `tools/hooks/`      | Code, production support                       | stated                     |
| `tools/outbox/`     | Code, production support                       | stated                     |
| `tools/pad-tables/` | Code, production support                       | stated                     |
| `Cargo.toml`        | Code                                           | stated, as cargo           |
| `Cargo.lock`        | Code                                           | stated, as cargo           |
| `.gitignore`        | **Code**                                       | **nowhere**                |
| `.gitattributes`    | **Code**                                       | **nowhere**                |
| `lenses/quality/`   | The quality lens                               | stated                     |
| `tools/quality/`    | The quality lens                               | stated                     |
| `lenses/research/`  | The research lens                              | stated                     |
| `tools/research/`   | The research lens                              | stated                     |
| `reviewed/`         | Nobody. The review application, acting as Sean | stated                     |
| `temporary-notes/`  | Sean, and no instance reads it uninvited       | stated                     |
| `pending.md`        | Nobody. Generated from every outbox            | stated                     |
| `target/`           | Nobody. Untracked build output                 | not mentioned, not tracked |
| `lenses/`           | Nobody at its root. Each lens writes its own   | stated, per lens           |
| `tools/`            | Nobody at its root. Each entry is owned        | stated, per entry          |
| `.git/`             | Nobody. Git's own                              | not mentioned, not tracked |
| `.idea/`            | Nobody. Ignored by `.gitignore:17`             | not mentioned, not tracked |

## The three that come from nowhere

**`decide/` has no writer.** `CLAUDE.md` names it four times and says what it is for - *it holds
what waits on a person* - and never says who may write it. **This lane has been writing it all
along**, which is the obvious reading and still a choice nobody approved.

**`.gitignore` and `.gitattributes` are not mentioned at all.** Production support covers
*`hooks/`, `scripts/`, CI, and everything in `tools/` that is not a lane's own*, and a dotfile in
the root is none of those.

**Nothing else needed a guess.** The four `docs/` subdirectories and `spec/data/`, `spec/future/`
follow from their parent, and every `tools/` entry follows from the production-support sentence or
from a lane's name.

## One thing this lane changed rather than asked about

**The Code row named `commands/`, which was deleted on 2026-09-05** - `ddbaed66` moved those files
into `scenario/commands/`, already covered by `scenario/`. **A path that names nothing grants
nothing**, so removing it changes no permission, and `CLAUDE.md` makes paths this lane's to settle
and report. Reported here.

## What it does not do

**It does not say what a lane may write into another's directory, because the answer is nothing.**
The three asymmetric rules under Perspectives already cover that and this table does not restate
them.

## The working behind `P-546`, answered `C1` on 2026-09-24

**Not an item** - `P-546` is a proposal and lives in
[`decide/proposals.md`](../../decide/proposals.md).

**What is worth keeping is that the first version of this question was wrong about the
world.** It offered three locations and called the recommended one theoretical. Two of
them were already in use - `tools/outbox/tests/architecture.rs` and
`no_floating_point_anywhere` in the two model crates - and the reason this lane did not
know is in `S-159`: the sweep read `crates/*/tests/` and the checks live in
`#[cfg(test)]` blocks inside `src/`.

**So the decision was nearly made on a false premise**, and the premise that would have
carried it - *nothing outside `crates/` does this today* - was the opposite of true.


**to** sean · **status** open · **raised** 2026-09-23 · **kind** measured · **shape** an instruction · **asks** a decision · **into** the check's location, and `docs/architecture.md` -> Rules

**`P-545` makes the boundary yours. This asks where the thing that enforces it sits.** It carries
no quotation, because what you are choosing is a location and the words follow from it.

## The four places, and two of them are already in use

**This lane first offered three and called one of them theoretical.** The corrected sweep in
`S-159` found a working example of two, so the choice is between patterns in use rather than
between a safe option and a theory.

```
C1  tools/spec/tests/architecture.rs                            not in use
    yours by column - the code lane may not edit it
    in their gate twice: hooks/pre-push loops over tools/*/Cargo.toml, and CI
    has its own "Test (the documentation tools)" step
    they cannot repair a failure they believe is wrong; they report it

C2  crates/<the crate it constrains>/src/lib.rs, in #[cfg(test)]    IN USE
    no_floating_point_anywhere, in game-model and planet-model
    closest to what it constrains, and run by cargo test --workspace
    the lane the check constrains can weaken or delete it

C3  spec/tests/, with a record in reviewed/                      not in use
    yours, and you read it in the review application like any other test
    that directory is 54 .4x files run by the thin engine; this one is Rust,
    so it needs its own runner and the app must show what it was not built for

C4  tools/outbox/tests/architecture.rs                              IN USE
    an architecture check already lives there - every workspace crate is
    named in docs/architecture.md, iterating the workspace rather than a list
    tools/outbox is production support, so this is the code lane's column:
    C1's location with C2's ownership. This is the status quo.
```

## What this lane recommends

**`C1`, and the reason is narrower than before.** The question is not *can a check live there* -
`C4` proves it can - but *who may weaken it*. `C1` is the only one of the four that is both
outside the code lane's column and inside the gate they must pass.

**`C2` has the best precedent and fails the thing you asked for.**
`crates/game-model/src/lib.rs:57` is the pattern worth copying whichever location wins: it strips
`#[cfg(test)]` before scanning, because *this very test has to name what it forbids in order to
look for it*; it skips comments; and it asserts how many files it read, noting that `read_dir` is
not recursive so a module moved to a subdirectory would go unscanned and stay green.

**`C4` is where you are today**, which is worth saying plainly: the existing architecture check is
already the code lane's to change.

## One thing you could decide instead of the location

**Split it: the rule is yours and the check is theirs.** `P-545` puts the boundary in `spec/`, so
the prose says what is required no matter where the check sits, and a weakened check leaves a rule
it visibly fails to enforce. **The cost is that nothing detects the weakening** - the rule still
reads correctly and only its enforcement has gone - which is the shape `CLAUDE.md` calls *silence
and nobody has looked yet are the same bytes*.

## The asymmetry `C1` creates, stated so you choose it knowingly

**The code lane would be gated by a check it cannot fix.** That is already the rule in the other
direction - `CLAUDE.md` says this lane does not edit code *even to fix an obvious break*; it
reports the break and leaves it. **`C1` makes the arrangement symmetric**, and the cost is a
round trip whenever they believe a boundary is wrong.

**It is a feature for exactly as long as the boundaries are right.** If they turn out to be
wrong often, the round trips are the signal, not the friction.

## The working behind `P-541`, answered `B2` on 2026-09-23

**Not an item** - `P-541` is one proposal and lives in
[`decide/proposals.md`](../../decide/proposals.md). **These two sections argued for a
decision that is now made**, and the proposal is shorter without them.

## The first three are half done and the half that is left is prose

**They are already out of the data.** `spec/data/` declares no `force`, `garrison` or `nature` -
this lane regenerated those files from the cut release on the 21st. **What is left is `spec/`
prose**, which still states all three as rules of the game:

```
spec/control.md   ## Force, ## Producing force, and three bullets of Gaining and holding ground
spec/planet.md    a territory's force of nature
```

**`P-539` gave you the rule for this**: *a document says what the game is, or it says what the
game will be, and it says which.* **So these sections move rather than being deleted**, which is
what *somewhere we can reach it for future design* asks for.

## Biome: the thing you are unwilling to give up is not in the data and never was

**Measured, and this is the part that changes your answer.** **Four** different things are called
biome, and two of them are files of that name:

```
spec/data/biomes.4x                 6 value rows        read by nothing
scenario/commands/biomes.4x         43 lines            compiled into game-front by include_str!
crates/planet-model/src/biome.rs    enum Biome          the list actually in use
the terrain field                                       what the drawing samples
```

**The second one is read and this lane nearly missed it.** A first pass said *every mention of
`biomes.4x` under `crates/` is a comment or a link*, which was true of `spec/data/biomes.4x` and
false of the other. `crates/game-front/src/library.rs:22` compiles
`scenario/commands/biomes.4x` into the binary.

**`planet-render/src/realistic.rs:117` says it in its own comment**: *the biome comes from the
field, sampled here rather than taken from the model.* **So the realistic rendering does not read
game data for biome at all** - it reads the terrain, and the terrain is generated.

**Every mention of `biomes.4x` in `crates/` is a comment or a link**, not a read. Checked across
the tree.


## The working behind `P-542`, answered `L3` and `S3` on 2026-09-23

**Not an item** - `P-542` is one proposal and lives in
[`decide/proposals.md`](../../decide/proposals.md).

**What is worth keeping is that two of the three points were already answered in the file.**
`{create-planet size:tiny}` was already the scenario's first line and `{run file:world}` was
already one command standing for the commands in a file - with the rule Sean wanted for the
log written into it: *as though they had been typed in its place*. **The new part was one
choice and two fields**, and it looked like a whole command's worth of design until it was
measured.


**to** sean · **status** open · **raised** 2026-09-23 · **kind** invented · **shape** an instruction · **asks** a decision · **into** `spec/console.md` -> Commands

**Two of your three points are already settled and the third is the real question.**

## What exists, so the new part is small

```
{create-planet size:tiny}       the scenario's own first line - takes a name, not a number
{run file:world}                one command standing for the commands in a file
```

**`spec/console.md` already gives `run` the rule you want for the log**: *run the commands in a
file, **as though they had been typed in its place***. And **the refusal you describe is already
required**: *a rejection names what was wrong, where, and what was expected instead.* Today it
says *there is no planet size called tiny* and does not list them, so **listing is a repair to an
existing rule rather than a new one.**

## So `generate-planet` adds three things, and one is a choice

**`policy` and `seed` are new and uncontroversial.** `docs/architecture.md` already promises
*same seed, same world* as a guarantee rather than a hope, and a policy name is a word like any
other.

**`size` is the choice.** Your sketch says `size:123`; the command today says `size:tiny`.

- **`S1` - a name.** `size:tiny`, as built. **The valid set is the vocabulary** and `size:123`
  never parses.
- **`S2` - a number.** `size:12`, as your first sketch. The refusal listing 12, 32, 42, 72, 92
  becomes load-bearing, and a sixth size is geometry rather than a new word.
- **`S3` - the name carries the number.** `size:tiny-12`, which you are leaning towards. **One
  word, so the notation needs nothing** - *a name is one word, dashed where it needs more* - and
  `small-12` is a name that does not exist, so the refusal lists the five that do.

## `S3` removes a duplication, which is the argument for it

**The pairing of a name to a count is currently stated three times:**

```
spec/planet.md:20              - tiny: 12
planet-model/src/size.rs:40    Self::Tiny => 12        in territory_count()
planet-model/src/size.rs:50    Self::Tiny => "tiny"    in name()
```

**Under `S3` the name is the pairing** and all three collapse to one. `spec/planet.md`'s list
becomes the five names, and the counts are in them.

**The one check it needs**: every size name's suffix equals the count its polyhedron gives.
`spec/planet.md` has the formula - `10T + 2` where `T = m² + mn + n²` - so the check derives the
number rather than holding a second list, and **a name reading `tiny-13` fails rather than lying.**

**The one thing to decide with it** is whether `tiny-12` is also what a player reads. Under
`P-540`'s prose form the menu can show a sentence while the value stays `tiny-12`, so **the name
being terse costs the interface nothing** - but that is a choice and not a consequence.

## The log, which is the part you asked to discuss

**Your priority, in your words**: *I must have the resulting commands, and it would be nice to know
how they were generated, but that is not as important.*

**`L1` - like `run`: only the generated commands appear.** The rule exists and nothing new is
needed. **Replay works and provenance is lost.**

**`L2` - both appear.** Then replaying the history runs the generator **and** its output, and the
planet is made twice. **This one is wrong and is here to be ruled out rather than weighed.**

**`L3` - the generated commands appear, and the generator appears as a comment.**

```
# {generate-planet size:42 policy:earth-like seed:12345}
{create-planet size:42}
{set-biome territory:1 biome:grassland}
...
```

**`#` begins a comment and history is a file `run` can execute**, so a comment costs nothing and
replay ignores it. **You get the commands as facts and the provenance as a note**, which is the
priority you stated, in that order.

## Answered `L3` on 2026-09-23

**`L3`, and it is nearly free.** The one thing to notice is that **the seed makes provenance
recoverable even under `L1`** - `same seed, same world` means the comment is a convenience rather
than the only record. **So `L3` is worth having and is not load-bearing**, which is the honest
version of *nice to know*.

**And `L2` should be ruled out in writing**, because it is the reading someone reaches for first:
a log of what happened ought to contain what the player did. **The reason it fails is that this
log is also an input.**

## The working behind `P-543`, answered `C1` and then a third kind on 2026-09-23

**Not an item** - `P-543` is one proposal and it lives in
[`decide/proposals.md`](../../decide/proposals.md).

**The part worth keeping is a wrong number and why it was wrong.** This item first said the
prose rule would cost nine values - the planet sizes, menu items and regions Sean had
sketched. **Then his own classification made the rule editor a gameplay surface**, and the
cost measured 80: twenty kinds, twenty-three traits, four families, seven members, six
biomes and twenty recipe names, before roles and comparisons. **Then he made the rule editor
a third kind and the eighty went away again.**

**So the number moved twice and neither move was a measurement error.** Nine was right for
the scope this lane assumed, eighty was right for the scope his first answer implied, and
nine is right again. **What changed each time was which surfaces were in the population**,
which is the thing this repository keeps finding: a count is only as good as the population
it names, and this one never named it until the third try.


**to** sean · **status** open · **raised** 2026-09-23 · **kind** recovered · **shape** text · **asks** a decision · **into** `spec/console.md` -> The language

**Your words, 2026-09-23:** *making it a requirement that the user ONLY ever sees prose. So a
quoted string is only displayed to the user and never has a mechanical effect, and anything with a
mechanical effect is never displayed to the user.*

**This is the converse of `P-540`'s sentence and it is the larger half.** `P-540` says prose is
never compared; this says a name is never shown. **Together they are a total separation**, and a
test can check both: a displayed value that is not prose fails, and a compared value that is
prose fails.

## It collides with exactly one surface, and it is the console

**The console is where the player types the notation**, so the player necessarily reads it:

```
help [<command>]     list every command, or give one command's syntax
show <subject>       reports what is true of it and what can be done with it
a rejection          names what was wrong, where, and what was expected instead
```

**`help` displays command names by definition.** `spec/interface.md` says so - *the console,
typing commands, with help listing every command and its syntax.* **And a rejection shows the
player what they typed**, which under `size:small-12` means showing a name that does not exist.

**`spec/invariants.md` makes this unavoidable rather than accidental**: *anything the player can do
through a surface can be done by typing*, and *every change to game state is representable and
executable as a console command.* **If the player can type a name, the player can read one.**

## So the question is how the console is excepted

**`C1` - the console is exempt, and the rule binds every other surface.** *In the console the
player addresses the machine in the machine's language; everywhere else they read prose.* **One
sentence, and the boundary is a surface rather than a judgement.**

**`C2` - the rule binds everywhere and the console shows prose too.** `help` lists sentences, a
rejection describes rather than quotes. **The cost is that the player cannot see the words they
must type**, which contradicts *anything the player can do through a surface can be done by
typing*.

**`C3` - the rule binds everywhere except where the player supplied the word.** A rejection may
echo what was typed, because the player wrote it; `help` may not, because the game chose it.
**Finer, and it is a judgement per message rather than a surface.**

## Answered `C1` on 2026-09-23, and your reason is better than the one offered

**Your words**: *the console is exempt because it is a different type of surface. It is an
admin/debug tool rather than a gameplay surface.*

**That classifies rather than excepts**, which is stronger: the rule binds gameplay surfaces and
the exemption is a kind of surface rather than one named surface. **`C2` and `C3` are ruled out.**

## But it reaches further than the console, and `spec/interface.md` has five surfaces

**Three are admin or debug by your reading and two are gameplay:**

```
the game itself     gameplay        the planet, its territories, what the player does with them
the rule editor     gameplay        the rules the player has, read and changed
the console         admin/debug     your words
the data browser    admin/debug     the game's own data, read directly
the debug view      admin/debug     already excepted in its own words - an output and not a path
```

**The rule editor is the one that costs.** `spec/invariants.md` puts it inside play - *rules are
specified through the interface; playing the game never requires writing a program* - so it is a
gameplay surface, and **a player cannot edit a rule without seeing the kinds and traits it names.**

## So the cost is much larger than this item first said, and here it is measured

**This item said nine prose values.** That was the cost if only your two sketched screens bind.
**Under your own classification the rule editor binds too, and then every mechanical word the
player can see needs prose:**

```
kinds        20        traits      23        families      4
members       7        biomes       6        recipe names 20
                                             -----------------
                                             80 words, before roles and comparisons
```

**Eighty rather than nine**, and this lane got it wrong by assuming only the screens you had
sketched were in scope.

## What that leaves you, and it is one question rather than a cost to accept

**`E1` - the rule editor is a gameplay surface and the eighty words get prose.** The rule holds
everywhere it should, and the player never reads a machine word.

**`E2` - the rule editor joins the admin surfaces.** A player editing rules is doing something
closer to administration than to playing, and `spec/invariants.md`'s *playing the game never
requires writing a program* is satisfied by never *requiring* it. **Then the cost really is nine
words** and the rule binds the game surface alone.

**This lane has no recommendation worth the name.** `E2` is what makes the rule cheap and `E1` is
what makes it mean what you said. **Which the rule editor is, is a question about who a player is**
- and that is further from anything measurable than this lane should guess at.

## The working behind `P-540`, answered `U2` and `N1` on 2026-09-22

**Not an item, deliberately** - `P-540` is one proposal and it lives in
[`decide/proposals.md`](../../decide/proposals.md), so a cited id resolves to one thing.

**And this is what a proposal should not look like.** It grew to 250 lines and fourteen
sections over three exchanges, because this lane appended each refinement to the live item
as the conversation moved. **Sean read the decision and stopped**, which was right: most of
what followed his answer could not matter once it was given. `CLAUDE.md` already says it -
*a proposal is read once, by one person, who decides from it: state the question, give the
proposed text, give the facts that make it right, and stop.* **Three options with worked
examples was the useful part; the two later rounds of options should have replaced it rather
than been added to it.**

**What survived into the proposal**: the prose form, the three notation constraints his
sketch tripped, the two facts his draft left blank, and what `U2` costs. **What did not**:
`U1` and `U3`, three recommendations, and the whole one-notation-or-two measurement.


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

## The working behind `P-527`, answered `W2` on 2026-09-21

**Not an item, deliberately.** `P-527` is one proposal and it lives in
[`decide/proposals.md`](../../decide/proposals.md); a second heading carrying the same id
would make a cited id resolve to two things. **The question it asked was:** *Fully exploited* is defined in four bullets and now read by nothing in `spec/`

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

## The working behind `P-529`, answered `B1` on 2026-09-21

**Not an item, deliberately.** `P-529` is one proposal and it lives in
[`decide/proposals.md`](../../decide/proposals.md); a second heading carrying the same id
would make a cited id resolve to two things. **The question it asked was:** Three lines of the release still name what `P-522` cut, and one of them is a vetted capability

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

### P-532 - Fifty-three tests have to cross a column boundary, and nothing may carry them

**to** sean · **status** answered · **answered** 2026-09-21, `M2` · **raised** 2026-09-21 · **kind** entailed · **into** `spec/tests/`, `reviewed/`, and the prototype

**Closed 2026-09-21: the move is done and committed.** `spec/tests/` holds 54 tests and
`reviewed/` holds 54 records, both tracked at HEAD; nothing remains at either old path. Sean ran
the code lane's scripts, this lane committed what they left, and every record now matches its
test byte for byte. **No text was ever promoted from this item** - it asked a decision, he
answered `M2`, and what followed was a move rather than words.

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
