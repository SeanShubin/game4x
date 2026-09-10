# Incorporating the re-encoding

**Specification lane, 2026-09-09, written overnight.** What a day of the research lens re-encoding
the release's recipes produced, and what of it the first release wants. Sean asked three questions -
which divergences are corrections the release wants anyway, which are decisions for him, and which
stay in the prototype - then settled the fuel question and handed the night over.

[Notes index](README.md) · [The recipe report](../../lenses/research/formulas.html) ·
[Research outbox](../../lenses/research/outbox.md) · [The inquiry, `S-81`](proposals.md)

**Nothing here has been incorporated.** Sean said so explicitly and it holds until he says otherwise.

## How to read this

Every claim below is graded, because the two lanes spent the night discovering that the ungraded
ones were where the errors were.

| Grade        | What it means                                         |
| ------------ | ----------------------------------------------------- |
| **Computed** | A check derives it and would go red if it changed     |
| **Executed** | A runner fired the recipes and it happened            |
| **Read**     | Quoted from a file, checked against that file tonight |
| **Asserted** | Someone wrote it down after reading                   |

## 1. Fuel is settled, and it costs the release one recipe

Sean, 2026-09-09: *the pioneer will have a tank they can load energy to, and that energy comes from
a region, so you can't really start expanding until you start exploiting fuel in a region. I was
thinking one move per fuel cell, and two fuel cells per pioneer, so a pioneer can leave gaps when it
founds.*

**The release already says this.** Read, tonight, against the file:

| Already written          | Reads correctly under Sean's decision                                                        |
| ------------------------ | -------------------------------------------------------------------------------------------- |
| *Where things are* `:89` | `a unit's tank / energy / the unit's fuel` - a container holding energy, capacity being Fuel |
| *Recipes*, `move` `:216` | `consume 1 energy` in *that unit* - spent from the tank                                      |
| *Units and structures*   | ark Fuel **2**, pioneer Fuel **2**                                                           |

So **the release needs one new recipe and no edit to anything already in it**: a `load` that requires
the unit here, consumes 1 energy in the territory and produces 1 energy in that unit. It needs no
limit row - *Where things are* declares the tank's capacity and `spec/logistics.md` containment
refuses the overfill.

**What diverged is the code.** `reports/recipes.md` shows `produce pioneer` yielding
`{pioneer fuel:2 id:1 ready:yes}` and `move` taking it to `fuel:1`. Fuel is built as a **counter that
decrements**; under Sean's decision it is the tank's **capacity** and the contents are energy.

**Cost to the scenario, computed:** the pioneer makes one crossing, so it needs one cell. **One extra
command, and territory 1's `{energy} -> 12` becomes `11`.** One line.

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

## 2. Three things block `R-6`, and fuel is not one of them

`R-6` is *the loop can be played through*, vetted when *a scenario reaches a fully exploited planet
and launches an Ark*.

**Executed.** The research lens encoded Sean's decision and ran it: colony, energy extractor bought,
six turns, two pioneers, four cells loaded, jungle taken at `4 > 2` and held at `2 >= 2`, yard built
- and then `launch ark` pays its cost and produces nothing. **The fuel change blocks nothing.**

| #   | What blocks it                                                                                                                                                                            | Filed                       |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- |
| 1   | **`launch ark` produces no ark.** `P-342` dropped its `produce 1 ark` row deliberately, because the destination could not be named. Sean's stated loop ends *launch a new ark into orbit* | `X-25`, to spec             |
| 2   | **Nothing fills a store**, and `spec/control.md` makes *every storage structure full* part of *fully exploited*                                                                           | `X-21`, and section 3 below |
| 3   | **Nothing can take a jungle.** `Biome::is_claimable` says it can be; no recipe implements force against nature                                                                            | `crates/outbox.md`, `X-22`  |

Both sides of 1 are deliberate, which makes it a contradiction rather than a defect. One fact has
moved since: `P-342`'s reason was that the destination could not be named, and the prototype names it
- `{orbit below:t}`, `below` being a trait an orbit can carry.

## 3. The store is in the release and in no specification file

**New tonight. Computed, then independently re-computed by the other lane.** This is the largest
thing found.

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

## 5. The failure shape, three times in one day, in three lanes

This is the finding worth keeping longest, because it is about how all three lanes check things.

| Who      | The claim                              | What was measured                  | What was missed                            |
| -------- | -------------------------------------- | ---------------------------------- | ------------------------------------------ |
| Research | no territory has a biome value         | searched `spec/` and `releases/`   | `reports/`, where all twelve are published |
| Research | nothing fuels a unit, so nothing moves | read `releases/`                   | `crates/`, where fuel already decrements   |
| **Spec** | `node` goes                            | verified `releases/` and `crates/` | **`spec/`, where it still lives**          |

**One shape: the verification named a proper subset of the columns the claim covered, and returned a
plausible number rather than an error.** `CLAUDE.md` already has the words - *the instrument answers
a narrower question than the one asked* - and what it does not yet have is **which** subset keeps
getting dropped. **Nobody checks the column they do not write.**

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

### The check both lanes want built

For a claim of the form *X is gone* or *nothing does Y*: **assert over `spec/`, `releases/`,
`crates/` and `reports/`, and fail if the assertion names fewer than four.** Not *search all four*
but **declare which four you searched** - because in all three instances the search was correct and
the **scope was never examined**. The sharpening is the research lens's and it is the better half of
the idea.

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

### Decisions for Sean

| Divergence                            | Whose            | Why it is his                                          |
| ------------------------------------- | ---------------- | ------------------------------------------------------ |
| **15** - `launch ark` produces an ark | unattributed     | reverses `P-342`; both sides deliberate - `X-25`       |
| `X-22` - force against nature         | **Sean**, stated | the whole rule is his and is written nowhere normative |
| 5 - `unpaid` becomes `unsustained`    | **Sean**         | only means something once force is implemented         |
| 6 - losing a territory is `perish`    | **Sean spotted** | same dependency                                        |

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

## 7. What is still owed, and by whom

- **Research, tonight** - what the encoding never reached (combat, orbit, control, population, the
  interface); where the notation could **not** say what the game needs; the seven remaining
  assumptions; whether the four `OPEN` decisions are all of them
- **This lane** - the triage table of all fifteen divergences, once the above lands

**Question 1 of the inquiry is answered and is on the report page** as *What the re-encoding ruled
out* - twelve constructs tried and removed, four ruled out without being built. The four untried are
one shape: each would take the model out of the class where its checks work. **That set is the
elimination half of what Sean asked the constraint to produce.**
