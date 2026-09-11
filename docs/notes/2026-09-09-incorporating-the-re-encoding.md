# Incorporating the re-encoding

**Specification lane, 2026-09-09, written overnight into the 10th.** What a day of the research lens re-encoding
the release's recipes produced, and what of it the first release wants. Sean asked three questions -
which divergences are corrections the release wants anyway, which are decisions for him, and which
stay in the prototype - then settled the fuel question and handed the night over.

[Notes index](README.md) · [The recipe report](../../lenses/research/formulas.html) ·
[Research outbox](../../lenses/research/outbox.md) · [The inquiry, `S-81`](proposals.md)

**Superseded in one respect, 2026-09-10.** This note was finished at 00:38 and records a state in
which nothing had been promoted. **Twenty-one proposals landed later the same day** - `P-356`
through `P-379` - so every sentence below saying nothing has been incorporated was true when
written and is not now. **The findings and their grades stand**; only the claim about what has
been promoted has moved.

## How to read this

Every claim below is graded, because the two lanes spent the night discovering that the ungraded
ones were where the errors were.

| Grade        | What it means                                         |
| ------------ | ----------------------------------------------------- |
| **Computed** | A check derives it and would go red if it changed     |
| **Executed** | A runner fired the recipes and it happened            |
| **Read**     | Quoted from a file, checked against that file tonight |
| **Asserted** | Someone wrote it down after reading                   |

## 1. Fuel is settled, and it costs the release nothing at all

Sean, 2026-09-09: *the pioneer will have a tank they can load energy to, and that energy comes from
a region, so you can't really start expanding until you start exploiting fuel in a region. I was
thinking one move per fuel cell, and two fuel cells per pioneer, so a pioneer can leave gaps when it
founds.*

**`spec/units.md` already says the whole of it, in two bullets**, and this lane did not find them
until the morning:

> - A mobile unit has a **bin** for fuel. Moving burns a unit of it, and a unit with none cannot move
> - **Fuel moves freely between a controlled territory that has it and anything there that can hold it**

**So there is no `load` recipe, and there must not be one.** This note said last night that the
release needed one new recipe. **That was wrong twice over.** `spec/units.md` makes fuelling
automatic rather than an action, and `spec/invariants.md:59` forbids adding one outright:

> No action has an intermediate step that is always taken. Where one would, the action is defined to
> reach the outcome directly

**A player who intends to move always loads first**, so a `load` command is exactly the step that
invariant names. Found by the research lens while answering *where the notation could not say what
the game needs* - it proposed `load` too, and the invariant refuses both our answers.

### What is already correct, and what is not

| Where                               | Says                                                                            | Verdict                              |
| ----------------------------------- | ------------------------------------------------------------------------------- | ------------------------------------ |
| `spec/units.md:17-18`               | a bin for fuel, burned by moving; fuel moves freely from a controlled territory | **already Sean's decision, in full** |
| *Where things are* `:89`            | `a unit's tank / energy / the unit's fuel`                                      | **correct**                          |
| *Recipes*, `move` `:216`            | `consume 1 energy` in *that unit*                                               | **correct**                          |
| *Units and structures*              | ark Fuel **2**, pioneer Fuel **2**                                              | **correct**                          |
| `crates/`, per `reports/recipes.md` | `fuel` is a counter that decrements, no energy involved                         | **the only thing that diverged**     |

**The release needs no new rule and no edited cell.** What it lacks is a pointer to the two bullets
in `spec/units.md`, and what the code lacks is a bin.

**And the constraint Sean wanted falls out of the word *controlled*.** Fuel moves freely from a
**controlled** territory that has it - so a unit standing on unclaimed ground cannot refuel. It
leaves your territory with two cells, spends one to enter unclaimed ground, and has one left. **That
is the gap-leaving range, and it is a consequence of a sentence already written** rather than
anything that had to be designed.

### Two is the right number, and for a reason worth preserving

**Computed, independently by both lanes** from the 30 adjacencies in `scenario/expected/play.4x`. The
twelve territories are an **icosahedron**: 5-regular, 30 edges. From every territory - all twelve,
uniformly - **5 neighbours at one hop, exactly 5 more at two, and one territory reachable at
neither.**

So a full tank reaches **10 of the other 11**, and the eleventh is always the **antipode**. Two cells
make the planet nearly-but-not-quite reachable in one sortie. **At three cells that property
disappears.** The observation is the research lens's.

One nuance Sean should have: the gate he described was **already mostly in place**, because
`produce pioneer` costs 6 energy. The tank adds 1 per hop on top. What is genuinely new is the
**range limit**, which is the gap-leaving mechanic.

## 2. What blocks `R-6` - and two of the three answers were wrong

`R-6` is *the loop can be played through*, vetted when *a scenario reaches a fully exploited planet
and launches an Ark*.

**Executed.** The research lens encoded Sean's decision and ran it: colony, energy extractor bought,
six turns, two pioneers, four cells loaded, jungle taken at `4 > 2` and held at `2 >= 2`, yard built
- and then `launch ark` pays its cost and produces nothing. **The fuel change blocks nothing.**

**This table had three entries and two of them were wrong.** Both were refuted by reading `crates/`,
which is the failure of section 5 committed twice more by the two lanes that wrote them.

| Claimed blocker                                            | Verdict                                                                                                                                         |
| ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| **Nothing fills a store** - `X-21`                         | **Refuted.** `territory.rs:184` caps metal and energy at store capacity and carries the remainder. Storing is built                             |
| **Nothing can take a jungle** - `crates/outbox.md`, `X-22` | **Refuted, and it was fixed months of items ago.** `P-275`: taking uses the **organised force brought**, not the force of the one unit consumed |
| **`launch ark` produces no ark** - `X-25`                  | **Stands**, and it is a contradiction rather than a defect                                                                                      |

**The force contest is fully built**, which `X-22`'s headline denied. Read tonight: `game.rs:236`
`force_in` is Sean's rule exactly - a garrison or any unit with force coordinates and contributions
**sum**, otherwise the largest single one stands. `game.rs:626` `take` rejects on `force <= defending`,
which is **greater to enter**. `game.rs:914` in `end_turn` loses the territory when
`force_in(id) < needed`, which is **equal or better to maintain**. Sean's two-test reading, built,
each citing `spec/control.md` in its own doc comment.

And the jungle was solved by a rule nobody had written down, in `game.rs:585`:

> **`P-275`: taking uses the organised force *brought*, not the force of the one unit consumed.**
> Before this they were the same number, which is why a jungle at nature two could not be taken by
> anything - `C-24`, and it was never a defect in the model so much as a rule nobody had written.

### So what does block it

**Nothing conceptual.** `R-6`'s own status line has said *nothing in the code blocks it* since
2026-09-05 and that is still true. Two things separate it from `vetted`:

- **Scale.** `is_fully_exploited` requires every claimable territory founded. The scenario founds
  **two of twelve**. `C-20` puts playing it through by hand at **roughly a thousand commands**
- **`X-25`, which is about what the loop *means* rather than whether it runs.** `R-6` is vetted when
  a scenario *reaches a fully exploited planet and launches an Ark* - and launching, as the release
  has it, is a recipe that pays a cost and puts nothing in orbit. **That satisfies `R-6` as written
  and not Sean's sentence**, which ends *launch a new ark into orbit, completing the loop*

Both sides of `X-25` are deliberate. One fact has moved since `P-342` decided it: its reason was
that the destination could not be named, and the prototype names it - `{orbit below:t}`, `below`
being a trait an orbit can carry.

## 3. The store is in the release and in no specification file

**New tonight. Computed, then independently re-computed by the other lane.** This is the largest gap
between the release and the **specification**; section 6 carries the largest gap between the release
and the **game**, which is a different pair.

`store` is a kind in `releases/first-release.md` - capacity 10, buildable, **seven built by the
scenario**. Across all 18 files of `spec/`, searched for the concept rather than the word - `store`,
`storage`, `silo`, `warehouse`, `granary`, `stockpile`, case-insensitive - the storage idea appears
**exactly once as a game object**: `spec/control.md:49`, *every storage structure on it is full*,
inside the win condition. `spec/structures.md` lists three structures: Extractor, Garrison, Yard.

**So the specification's definition of a fully exploited planet depends on a kind the specification
never defines.**

**And the code wrote down its own expiry condition, then expired.**
`crates/game-model/src/game.rs:425`, `is_fully_exploited`, drops the storage clause entirely:

> *Every storage structure is full* holds because there are none. No structure in
> `spec/structures.md` stores anything. **If one is ever added, this stops being vacuous and this
> function will not notice on its own.**

One was added - to `releases/`, not to `spec/` - and it did not notice. `CLAUDE.md` is explicit that
this must not happen: *A release spec never invents a rule. If a release needs one the spec lacks,
propose it into the spec first, then have the release refer to it.*

**The denominator, because a count over nothing proves nothing.** Sixteen kinds, parsed from the
release's own Kinds table rather than typed; word-boundary, case-insensitive, over all 18 `spec/`
files; poisoned on `garrison` and `territory`, either of which failing to appear means the search is
broken. **14 of 16 are named in `spec/` by their own word. Two are not.**

### `stow` is already implemented, unnamed, and that changes what `X-21` is

**Read tonight, in `crates/game-model/src/territory.rs:184`.** `end_of_turn_losses` does this:

- **food and labor are discarded entirely**, every turn, whatever stores exist
- **metal and energy are capped** at the territory's store capacity, the excess removed, and **the
  remainder carries**

Sean's answer to `X-21` was *just automatically cram as much as will fit in storage, the rest goes
into disorder.* **That is exactly what those nine lines do**, for metal and energy. So storing is
not missing from the game. **It is missing from the Recipes table**, and from nowhere else.

That is a smaller defect than either lane had it, and a sharper one:

- **This lane told Sean the scenario was written against a rule that does not exist.** Wrong. The
  scenario's *energy has nowhere to go until it has a store* at `play.4x:50` is **correct against
  the code** - capacity zero means everything made is over the bound and is removed
- **The research lens filed it as *nothing fills a store, so a store is a building that does
  nothing*.** Also wrong: metal and energy stores are what make metal and energy carry at all, and
  the twelve loose energy in the dump are there because two stores hold twenty between them
- **What is true is that a behaviour of the game is not among its recipes.** That was the first
  statement of it; **section 6 has the measured version**, which is six such behaviours and a
  relation that runs both ways

### The refutation moved a number, and the new one is better

**Computed, and poisoned by taking the stores away.** While *nothing stores* stood, a Yard's 15 metal
had to be mined in a single turn, and **six of twelve territories could ever build one**. Metal
carries up to the stores' capacity, and a territory may have one store per extractor of that
resource - so the question is **what it can be standing on, not what it mines in one turn**.

**Nine of twelve can build a Yard**, six of them in one turn. **The old six was not overturned**; it
was one column of a table, and territories 1, 2 and 3 join at two turns. Taking the stores away drops
it back to six, which is what makes the check honest.

## 4. `deposit` is a different phenomenon, and the distinction is the useful half

The second of the two is `deposit`, and it is **not** a gap. It is absent by word and present by
concept, properly specified:

- `spec/economy.md:14` - *the territory's density for a resource is what each extractor pulls from it
  each turn*
- `spec/logistics.md:35` - *an extractor names **the node** it works, and the node is not inside it*

Those are properties of a territory. **The release turned them into a thing.** So the class has two
members needing opposite treatment: **a gap gets promoted; a reification gets asked whether the
release was entitled to make the thing.** The distinction is the research lens's and it is right.

### And `spec/` still calls it `node`

**Read tonight, and it is this lane's own error.** The rename already happened. `S-48`, *`node`
goes*, closed 2026-09-06 on `P-290`, which deleted `Node`, `Territory.nodes` and the bookkeeping
under it. Its verification, verbatim:

> **Closed by this lane, verified against the tree.** `node` has 0 occurrences in
> `releases/first-release.md`, and the four left in `crates/game-model/src/territory.rs` are comments
> recording what `P-290` removed.

**It checked `releases/` and `crates/`. It never checked `spec/`.** The claim was *node goes*; the
measurement was *node is gone from two of the three columns*, and it returned a true number about a
narrower population than the claim covered. `spec/logistics.md:35` has said `node` for three days.

## 5. The failure shape, five times in two days, in two lanes

This is the finding worth keeping longest, because it is about how every lane checks things.

| Who      | The claim                                          | What was measured                  | What was missed                               |
| -------- | -------------------------------------------------- | ---------------------------------- | --------------------------------------------- |
| Research | no territory has a biome value                     | searched `spec/` and `releases/`   | `reports/`, where all twelve are published    |
| Research | nothing fuels a unit, so nothing moves             | read `releases/`                   | `crates/`, where fuel already decrements      |
| Research | nothing fills a store - `X-21`                     | read the release's *Recipes*       | `crates/`, where `end_of_turn_losses` does it |
| Research | force is stated and nothing implements it - `X-22` | read the release's *Recipes*       | `crates/`, where all three halves are built   |
| **Spec** | `node` goes - `S-48`                               | verified `releases/` and `crates/` | **`spec/`, where it still lives**             |

**One shape: the verification named a proper subset of the columns the claim covered, and returned a
plausible number rather than an error.** `CLAUDE.md` already has the words - *the instrument answers
a narrower question than the one asked*.

**A first draft of this note said *nobody checks the column they do not write*, and the data does not
support it.** `spec/` is this lane's own column and is exactly what `S-48` failed to check. The
honest version is duller and more useful:

> **The claim spans the whole pipeline; the check covers the part its author was thinking in.**

The research lens reasons in documents, so four times it read `releases/` and never opened `crates/`.
This lane was asking whether a rename had **propagated**, so it looked downstream - `releases/`,
`crates/` - and never back at the source. **Each search was correct. Each scope was unexamined**, and
in every case the unexamined part was the end of the pipeline its author was not facing.

Four of the five are the research lens's and one is this lane's. **Two were refuted by the other lane
inside one night**, which is the only reason the count is five rather than one.

Three more of the same family, all caught before being believed. The research lens's kind-coverage
check said *0 of 16* for twenty minutes while `garrison` sat in `spec/` five times; its
row-comparison check read an empty first cell as a table separator and cried wolf about eighteen
rows; and **this lane's own concept search returned two false positives** - `spec/interface.md:27`,
which is a tree node in the rule editor, and `spec/planet.md:89`, which is a terrain seam. All three
were caught by **reading the hits rather than counting them**, which is the whole defence.

That last one has a tail worth recording. The research lens searched for `deposit` through its
*properties* - `density`, `total capacity` - and never through `node`, the word `spec/` actually
uses for the thing. **It got the right answer on weaker evidence than was available**, which is the
failure that does not announce itself. Its check now prints every match with file and line, so
`interface.md:27` sits visibly beside `logistics.md:35` and a reader decides which is which.

### Why the shape was visible at all

**Two of the three instances are the research lens's and one is this lane's, and that is the only
reason the pattern could be seen.** A lens checking a producer finds the producer's error; a
producer checking a lens finds the lens's. It took both directions on one night, and **neither lane
could have named the shape alone** - each had only its own half and no reason to think the half was
half of anything.

That is an argument about how the perspectives are arranged rather than about any of the three
findings, and it is the one thing here that no single lane's work could have produced.

### The check both lanes wanted, and what came of it

For a claim of the form *X is gone* or *nothing does Y*: **assert over `spec/`, `releases/`,
`crates/` and `reports/`, and fail if the assertion names fewer than four.** Not *search all four*
but **declare which four you searched** - because in every instance the search was correct and the
**scope was never examined**. The sharpening is the research lens's and it is the better half of the
idea.

**It has not been built as stated, and something better was built instead.** Check 15 - section 6 -
takes the same problem from the other end: rather than making a claim declare its scope, it
**enumerates the behaviours from the code and asks which the documents name.** That answers the
population question directly, and it is what found the six. **The declare-your-scope check is still
worth having** - it would catch a claim about `spec/` that never opened `spec/`, which check 15
cannot - and neither lane has built it.

**One check that could not have failed, found while building these.** The old check 11 poison set a
garrison's force to 5, called the check, and **discarded the answer** - in the file whose entire
premise is that a green means nothing without a poison that can turn it red. Reported by the
research lens against itself.

## 6. The fifteen divergences, triaged

Fourteen are on the report's own table, with whose decision each was; the fifteenth this lane found
and is now `X-25`. **The attribution is read from `tools/research/formulas/data.json` rather than
from the rendered page.**

**The frame matters and this lane had it wrong at first.** Sean, 2026-09-09: the rule editor being
out of the first release is true, but *I had to put some constraint on the research instance to
make sure it detected any missing gaps. I wanted to eliminate non-viable decisions as early as
possible.* **So the language work is the instrument, not cargo to be triaged back.** Its product is
the list of things now ruled out, and that is worth having whether or not a line of notation moves.

### Corrections the release wants - content, no new notation

| #   | Divergence                       | Whose                | Cost                                                 |
| --- | -------------------------------- | -------------------- | ---------------------------------------------------- |
| 11  | the two stores in `found-colony` | **Sean**             | four cells, and the scenario's prose in three places |
| 13  | storing - `stow`                 | **Sean**             | one recipe, writable in the release's four roles     |
| 12  | a territory's capacity for arks  | **Sean**             | one cell; an ark is only ever in orbit               |
| 4   | `fuel` and the tank              | **reversed tonight** | see below                                            |

Row 4 was *delete the tank, spend energy where the unit stands*, taken under Sean's leave to remove.
**His decision tonight reverses it**: the tank is real, the release's words were right, and what
changes is the code plus one `load` recipe. The research lens had already reversed its own half
before this lane's message reached it.

Two more belong here that are not divergence rows: **`X-20`**, *no capacity* to *no limit*, and
**`X-19`**, one sentence saying biomes come from `biomes_of`.

### Decisions for Sean - one, not four

| Divergence                            | Whose        | Why it is his                                    |
| ------------------------------------- | ------------ | ------------------------------------------------ |
| **15** - `launch ark` produces an ark | unattributed | reverses `P-342`; both sides deliberate - `X-25` |

**Three rows were here in the first draft and all three were wrong in the same way.** `X-22`, the
`unsustained` rename and the merged `perish` were filed as *waiting on the force rule*, and the
force rule is built. Each is a place where **the code has a behaviour and the documents do not
describe it** - which is one finding rather than three.

### Recipes and behaviours are many-to-many, in both directions

**Computed.** The research lens's check 15 anchors behaviours in `crates/game-model` by a **line of
code rather than a line number** - a number cries wolf on every edit above it, and an absent anchor
passes silently - and reads only above `#[cfg(test)]`, because **a behaviour that lived only in a
test would be a rule nothing runs**. Its first run refused three rows correctly: `game.rs:1618`
reimplements the end of a turn backwards to prove the settling order cannot matter, so three anchors
matched twice.

**Ten behaviours found. Six are named by no recipe among the release's sixteen:**

| Anchored at        | What it does                                                | Note                                                  |
| ------------------ | ----------------------------------------------------------- | ----------------------------------------------------- |
| `territory.rs:192` | metal and energy cut to what the stores hold                | **the rule that decides what a player keeps**         |
| `game.rs:915`      | equal force to maintain, or nature takes the territory back | the `X-22` half                                       |
| `game.rs:631`      | greater force to enter, or taking is refused                | the other `X-22` half                                 |
| `territory.rs:584` | losing a territory clears everything on it                  | garrison, population, stores, yards, extractors       |
| `game.rs:918`      | a unit on a lost territory survives, unusable               | `X-27`, and against Sean's *we just delete the units* |
| `game.rs:930`      | the turn number advances                                    | bookkeeping, listed so the count is honest            |

**And it cuts the other way too, which is what this lane had wrong.** A first draft of this note said
the game runs *more* rules than the release lists, as though the sixteen were a subset. They are not:

- **Three recipes are one function.** `upkeep`, `grow` and `perish` are `population_after` at
  `territory.rs:600` - `if food < citizens { food } else { citizens + (food - citizens).min(citizens) }`.
  **Nothing there consumes the food**; it only decides the new count
- **One recipe is two places.** `refresh` readies a territory's contents in one place and units in
  another
- **Half a line has recipes and half has none.** Splitting `end_of_turn_losses` shows food expiring
  **is** `age` and `spoil` through `keeps` 1 - and **labor is discarded by the same line with no
  recipe mentioning it**

**So sixteen is not a count of what the game does, in either direction, and neither list contains the
other.** That is a sharper statement than *the game has rules the release does not list*, and it is
the one the measurement supports.

**`R-7` still cannot show the six**, because it shows recipes and these are not recipes. It is built
and waiting on Sean, correct over all sixteen, and **narrower than its wording suggests** - the rule
deciding what a player keeps between turns is not among the things it can display.

### The instrument - what the constraint was for

| #   | Divergence                                      | Whose     |
| --- | ----------------------------------------------- | --------- |
| 1   | the four roles become six primitives + `attach` | **Sean**  |
| 2   | `limit` deleted                                 | **Sean**  |
| 3   | `require` compares expressions                  | **Sean**  |
| 7   | `border` / `crosses` as two traits              | this lane |
| 8   | `below` as a trait of an orbit                  | this lane |
| 9   | `location` becomes `holder of`                  | this lane |
| 10  | `control`'s values enumerated                   | this lane |

**Row 7 is the exception worth naming**: the *problem* is already open against the release in its own
terms, as `C-60` and `S-73`. The two-traits answer is one solution, not the only one.

### Not a divergence

**14, `make-world`.** It appears in four files, all the research lens's own, and in no `spec/` or
`releases/` file. **Deleting it cannot break a scenario written against the old rules.** It was
listed as one of three breakages; the true count is one - the stores - and its blast radius is four
lines of the expected dump plus the scenario's own reasoning.

## 7. What the constraint actually produced

Sean set the rule-editor constraint **to detect missing gaps and eliminate non-viable decisions**.
This is what came back, and the first part was a surprise to both lanes.

### The notation was already specified, and the lens re-derived it without looking

**Verified tonight against `spec/console.md` -> The language.** The research lens designed its
notation over two days without opening that file, and the file already says:

- **`{extractor territory:1}`** - *a field named for a kind is a reference to one*
- **a description is a kind and every stored trait** - word for word
- **a command is named for the recipe it fires**
- **`repeat` is a count of firings and not an argument of the recipe** - which is Sean's own answer
  about moving pioneers one at a time, written down before he was asked it
- **every word in a data file is a kind, a trait, or one of a trait's values**

**That last line is what the lens's check 10 measures**, at 498 of 498 tokens and 23 of 23 recipes.
**So it is not a property the lens invented - it is the first measurement of a specified invariant**,
and it passes. Same for `X-8`'s grounding: `spec/console.md:82` already defines `show <subject>` as
*for each action the rules permit on that subject, whether it is possible now, and when it is not,
what is missing.* The lens built an implementation of a specified command believing it was open.

**Two independent derivations reaching the same design is evidence the design is right.** It is also
two days spent re-deriving something readable in ten minutes, and both are true.

### Three regions the encoding never reached

- **Combat is not a gap in the encoding.** `spec/combat.md` is **27 lines and zero normative ones**
  - Scales, Range, Weapons and Resolution all still carry *scaffolding prompt - delete this line*.
  There is nothing to encode, and that is this lane's side of the wall
- **Force has every word and no recipe.** The encoding carries force and nature as traits with the
  release's numbers, and `max`/`sum` as expression forms - and **over 23 recipes they appear in one
  line**, `make-territory`, which writes nature at design time. The jungle sequence is computed in
  Python, outside the notation. **A defect in the prototype, not in the notation**
- **The world-builder cannot start a game.** Four of `spec/console.md`'s five design commands are
  there; **nothing places the ark and nothing ends the design phase**. `play.py:72` puts the
  starting ark in orbit with a line of Python, so **the loop reported as playable begins from a
  state no recipe produces**

### Seven things the notation cannot say

| What                                | Where it is needed                    | Why it does not fit                                                                         |
| ----------------------------------- | ------------------------------------- | ------------------------------------------------------------------------------------------- |
| competition and its resolver        | `spec/turn.md`                        | no way to name contenders, none to state what settles them                                  |
| reachability                        | `spec/planet.md`                      | oceans never isolate land - needs recursion                                                 |
| a quantity shared rather than moved | `spec/units.md`                       | *fuel moves freely* collides with the invariant against a step always taken                 |
| a rule that is itself a thing       | -                                     | a recipe has no id and no traits, so a turn budget has nowhere to live                      |
| a derived relation                  | `spec/logistics.md`, `spec/planet.md` | orbital adjacency can only be materialized, so a consequence becomes data that can disagree |
| a predicate over the whole game     | `spec/control.md`                     | winning and losing                                                                          |
| taking a conserved thing apart      | -                                     | -                                                                                           |

**The lens recommends two of these stay unsayable**, and this lane has no basis to disagree yet:
**reachability**, because recursion costs the boundedness that makes *a player's rules always
finish* checkable; and possibly **the win condition**, because a question asked *about* the game is
not a transition. The second is uncomfortable - **leaving it in Rust is the option that already
failed once**, which is section 3 of this note.

## 8. The seven assumptions, which complete the inquiry

**Question 2 answered, and answering it corrected its own tally.** The `expression` assumption was
recorded only where `let` and `require` raised it, never where an *amount* did - so three encoded
lines used a form the notation has no way to state without anything noting it. **It read 3 and it is
11.** Over 80 encoded lines:

| Assumption          | Lines | What the notation cannot say                                                | Weight                                                                                                                       |
| ------------------- | ----- | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| **family-not-kind** | 12    | a description names a family, not a kind - `{thing}`, `{resource kind:...}` | grounding, `X-17`. **The one that produced four wrong answers in a week**                                                    |
| **expression**      | 11    | `count {…}`, `sum force of {…}`, `min(a, b)`, a path, a number              | **the biggest real hole.** `spec/console.md` says a value is a word, a number or another command                             |
| **path**            | 4     | a path as a value - `ark.location.below`                                    | same root as `expression`; every `let` needs one                                                                             |
| **root-container**  | 3     | `game:game` - the game is inside nothing, so the field repeats the kind     | cosmetic                                                                                                                     |
| **recipe-as-thing** | 2     | `recipe:` names what a `call` fires, treating a recipe as a nameable thing  | the deferred question in miniature                                                                                           |
| **derived-trait**   | 1     | a derived trait is **tested** - `surplus`, `unsustained`, `control`         | **one line by count, load-bearing by consequence**: `perish` fires on `{thing unsustained:yes}`, where both force rules live |
| **unnamed-trait**   | 1     | `location` and `below` read as traits, which `spec/logistics.md` forbids    | the other side of `C-56`                                                                                                     |

**How to weigh them.** `expression` and `path` are **one gap wearing two hats**, and it is the
notation's rather than the prototype's. `family-not-kind` and `derived-trait` are **Sean's own
decisions arriving faster than the notation could carry them** - a lag, not a flaw. `recipe-as-thing`
is a question he has already deferred. The other two are cosmetic and known.

**The build asserts every declared assumption is used at least once**, which is why an eighth never
appeared quietly: when `refuel` wanted arithmetic inside an amount, the encoder refused it by name.

## 9. Where this stands, and what is still open

**Everything asked of the research lens has been answered except two**, and both are answerable by
this lane if it runs out of night: the **seven remaining assumptions** behind its editor test, and
whether the four `OPEN` decisions in its data are all of them.

**Filed, and therefore not lost if either session ends:**

| Item    | Where                   | What it is                                                        |
| ------- | ----------------------- | ----------------------------------------------------------------- |
| `P-360` | **promoted** 2026-09-10 | a garrison has no force; landed in `spec/unit-types.md`           |
| `S-81`  | **answered** 2026-09-10 | the overnight inquiry itself                                      |
| `S-82`  | to spec                 | `spec/logistics.md` still names `node`, which `P-290` deleted     |
| `S-83`  | to spec                 | recipes and behaviours are many-to-many; six rules have no recipe |

**Nothing had been incorporated into `spec/` or `releases/` when this was written.** `P-360` was then a proposal in the queue,
which is where a decision is offered rather than taken.

### The elimination half, which is what the constraint was for

**Question 1 is answered and is on the report page** as *What the re-encoding ruled out*: twelve
constructs tried and removed, and **four ruled out without being built**. The four are one shape -
an effect behind an opaque call, a zero test on an unbounded quantity, recursion in `call`, and
recipes creating recipes unbounded. **Each would take the model out of the class where its checks
work.**

The lens rates that set above the twelve, and this lane agrees: **a construct removed can be put
back; a decidability boundary cannot be uncrossed cheaply.**