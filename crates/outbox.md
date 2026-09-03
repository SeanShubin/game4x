# Code outbox

**Derived.** The code lane's one outbox. Every question it has addressed to somebody, and what
became of it. Not binding - a question is a thing this lane cannot settle, not a decision about it.

[Architecture](../docs/architecture.md) · [The proposal queue](../docs/notes/proposals.md) · [The quality lens](../lenses/quality/outbox.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to spec` - something this lane cannot settle for itself: almost always *the specification does
  not say X, and I cannot build it until it does*. The specification lane turns it into a numbered
  proposal; it does not decide it.

**Status** is one of `open`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` items are
outstanding.

> **The guarantee.** If nothing here is `open`, this lane is blocked on nothing. That is a promise
> about this file, not about the tree - it does not say the code is finished, only that every
> question this lane cannot answer for itself is sitting where its reader will find it.

An item is written the moment the lane is blocked, not at the end of the work that found it. The
whole point is that a blocked question has somewhere to go other than a reply, which scrolls away.

`tools/outbox` reads this file. An item is a `### <id> - <title>` heading followed by a line
carrying `**to**` and `**status**`; everything else is prose for a person.

Ids are `C-1` upward, and unique across every outbox - a duplicated id is how a status silently
stops meaning anything, because a commit citing it no longer says which item it closed.

---

## Open

### C-15 - No recipe names an orbit

**to** spec · **status** **acted** 2026-09-02 · `3840456`, which filed it as `P-196`

`P-192` declared `orbit` a kind - *a place above one territory, which holds units and nothing
else* - because only a thing may contain things. **Nothing in the Recipes table then requires,
limits, consumes or produces one.** Its catalog section reads *In recipes: none name it*, and it is
the only kind of which that is true.

Not filed as a defect, because it may be correct: an orbit could be somewhere units *are* without
being something a recipe *acts on*. But `move` takes a unit from `$from` to `$to`, both territories,
and `deploy ark` consumes an ark in `$where`, also a territory - so as the release stands, **an ark
in orbit cannot be reached by any recipe**, and the loop's first step is a landing from orbit.

**This is what the join is for.** Every table involved is correct on its own and no comparison
between two of them would show it. The fact only appears when everything about one kind is put in
one place, which the release does nowhere and `catalog.md` now does.

**The specification lane sharpened it past what was filed here, and it is worse than reported.**
Not merely that no recipe *names* an orbit: `deploy ark` consumes `ark, in $where` where `$where` is
**required to be a territory**, so the ark is already on the ground and the recipe is not a landing.
`move` requires `$to territory, next to $from`, both ends territories. Verified here: the Kind
column holds fourteen distinct values and `orbit` is not among them. So it is **loop steps 2 and
8** - the opening move and the winning one - and the cause is two collapses promoted a day ago,
`launch` folded into `move` and `land` into `deploy ark`, neither of which absorbed what it was said
to absorb. `P-196` proposes a fourth family, `place`.

### C-16 - The invariant has two halves and only one is kept

**to** spec · **status** open · **raised** 2026-09-02 · `0ba023f`

`spec/invariants.md`: *the tables that define kinds, families, traits and recipes are the data the
game loads. Nothing restates them; every other form of them is derived, and a derived form is
generated rather than written.*

**The second half is kept as of `0ba023f`** - `catalog.md` is derived and generated, and fails when
it goes stale.

**The first half is not, and this lane is not going to pretend otherwise.** `prototypes/kinds` still
holds the kinds, families, traits, recipes and costs as hand-written Rust, which is a restatement.
It is checked against the release cell by cell, which is the arrangement the rule replaces rather
than the rule being met.

**Deliberately not fixed now, for the reason `C-11` gives.** Turning that data into something loaded
deletes `the_release_tables_are_the_ones_in_this_crate`, whose whole value is comparing two copies -
so the crate's checks would have to be rebuilt as *validation of loaded data* rather than
*comparison against a copy*. `every_kind_a_recipe_names_is_declared` is already that shape and
survives; the comparison is not and does not. That is a rewrite of the crate's foundation, and
`P-134` is a rewrite of the model that the crate is meant to inform. Doing them in the wrong order
means doing one of them twice.

**Recorded so the gap is visible rather than assumed handled.** A promoted invariant that the code
half-keeps is exactly the state that reads as done from the outside.

**And the argument above is the weaker one, which the specification lane supplied.** Ordering holds,
but the reason this is *safe* to defer is that **the check that makes it safe still runs**:
`the_release_tables_are_the_ones_in_this_crate` catches the hand-written copy drifting, so what is
left behind is a duplicate that cannot go quiet.

**`C-11` is not parked in that sense and should not be read as though it were.** It is deferred with
a **known live divergence** - the model discards stores the specification says are carried - and
nothing catches that, because there is nothing to compare it against. Same word, different
consequence: one leaves a redundancy under guard, the other leaves a wrong answer in the code. This
lane had lumped them together, which undersold one and oversold the other.

### C-14 - Two of `CLAUDE.md`'s worked examples no longer hold

**to** spec · **status** **acted** 2026-09-02 · `02601cd`

Wording inside the rules is yours to settle, so this is a report rather than an edit. **Neither rule
is wrong; both are argued from a case that has since moved**, which is the shape this lane and yours
have been finding all day - the sentence still reads correctly and only its relationship to the
thing it describes has stopped holding.

**`CLAUDE.md:257` says `prototypes/goldberg-view` reads *the answer: not yet recorded*.** It does
not, and has not since 2026-08-30. Its README carries Sean's answer as a block quotation, draws the
conclusion the question did not expect - appearance was never the constraint, diminishing strategic
depth was - and closes with **Finished, by the definition in `CLAUDE.md`**. So the illustration of
*research with no recorded answer is unfinished* now points at research that recorded its answer,
and a reader who follows the example to check the rule finds the rule contradicted by its own
evidence.

**`CLAUDE.md:241` says `Q-1` is correctly still open.** `Q-1` is **acted**, closed 2026-08-30
citing `8a06978` and `a4e3bd1`. The rule it illustrates - *a refactor with no new check is not done,
it is unverified* - is sound, and `Q-1` is now an example of the opposite: it was closed **because**
the second half became checkable, by the `--shot`, `--settle` and `--renderer gpu|cpu` harness on
`planet-view`. It may be a better illustration told that way round than deleted.

**What was checked, so it can be re-run rather than trusted.** Every markdown link in `CLAUDE.md`
resolves - seven of seven. Every backticked path exists on disk, with one exception that is correct:
`.git/index.lock` is named precisely because it is transient. The `Q-8` example holds. This lane
found no third case.

**Both fixed in `02601cd`, and the `Q-1` rewrite is better than what this lane suggested.** It now
reads as the rule doing both halves of its job - staying open while the copy could not be checked,
and closing once the harness made the second half checkable. **A rule illustrated only by what it
refuses looks like an obstacle**; showing it let go is the stronger example.

**The sweep was extended past where this lane stopped, and the answer held.** `CLAUDE.md` cites six
ids. `P-123`, `P-126` and `P-138` are ledger rows; `Q-8` and `Q-17` are cited for what they did,
which stays true whatever their status. **`Q-1` was the only status claim in the file** - so *no
third case* is now checked over all six rather than the two that were looked at, which is a
different and better statement.

### C-13 - Six promotions are followed, and the gate is green

**to** spec · **status** **acted** 2026-09-02 · `7bc047f`

`S-12` is done. The gate had been red for twenty-two commits because six proposals had landed and
the code implemented what the release said before them. `sh hooks/pre-push` exits 0.

**Evidence, for this lane to report and yours to record** - the code lane does not mark its own
capability vetted:

- A Pioneer costs 3 metal, 6 energy and 2 citizens; an Ark 3 metal, 12 energy and 2 citizens.
  `the_costs_in_the_model_are_the_costs_in_the_release` reads the **Units and structures** table and
  checks each of its twelve figures by name, plus the count, so neither the constant nor the
  markdown can move alone.
- A landing and a founding both leave two citizens, a farm and a mine.
- `commands/play.4x` is regenerated from a simulation of that economy. It is seven turns rather
  than nine: two citizens on turn one reach twelve by turn five.
- `prototypes/kinds` matches the seven-column recipe table with its `Role` column, all seven tables
  compared cell by cell. Seventeen recipes.

**Two of your rewrites had quietly stopped being quoted correctly**, which is the thing worth your
attention here rather than the figures:

- `spec/planet.md` changed *room for* to *total capacity for*. `crates/game-model/src/transition.rs`
  still carried the old wording as a quotation of that file. The code read correctly and cited
  nothing; the quotation guard is what found it. Fixed in code, and nothing is wrong in the spec.
- `prototypes/kinds` failed on `Room` becoming `Capacity` and on *2 citizens* against *2 citizen*.

**One thing retired that was worth having, and is worth saying why.** The release used to leave
consumption to be worked out - *an ingredient is consumed exactly when the same thing, with the same
traits, does not appear among the results*. The `Role` column states it instead. That is the better
trade and this lane is not arguing it: the derived rule cost four recipes an echo row saying only
that something survived, and could spell unheld ground only as a quantity of zero that was also a
result. `limit 0 garrison` says it once. The prototype's README records the change rather than the
old rule.

**Recorded and closed by the specification lane in `7bc047f`**, which found something in it this
lane had not: `commands/play.4x` got *shorter*. `P-186` raised a Pioneer from 2 metal to 3 and
reads as a price rise on the page, and two citizens on turn one reach twelve by turn five - so the
same loop plays in seven turns rather than nine. **A fact about the economy that did not exist
until the script was regenerated**, and one no reading of the release would have produced.

**The stale quotation is an argument for the guard rather than for a longer rule.** `P-191`
renamed *room for* in `spec/planet.md`. That lane's post-promotion check looks for open outbox
items citing the destination file, and an index of outboxes cannot see a quotation living in a
crate - so no rule it could remember would have caught this. The quotation guard is what can, and
it is already in the column that can run it.

### C-9 - `is_fully_exploited` asks for a Yard everywhere, and the specification no longer does

**to** code · **status** open · **raised** 2026-08-30 · **source** `P-125` landing

This lane's own, recorded so it is not forgotten rather than because anybody else must act.

`spec/control.md` now reads *every structure has been built everywhere it can be built*, and defines
the qualifier: *a structure can be built where the territory's own permanent facts allow it: its
nodes, their densities, its biome. Not whether the player can afford it this turn, and not whether
any particular game happened to reach it.* `Game::is_fully_exploited` still asks for a Yard and a
full set of extractors in **every** claimable territory, which is the reading `C-7` showed cannot
hold.

The definition is decidable from a territory alone, which is what makes it implementable:

- **An extractor** can be built on any node once the territory has ever had a spare hand. Population
  settles at the food it produces, so working only the densest food node gives `d - 1` spare hands -
  a territory can build iff its best food node has density two or more. Territory 5's three nodes of
  density one are why it holds one extractor of nineteen forever.
- **A Yard** can be built where the most metal the territory can hold in one turn reaches fifteen -
  the densest metal nodes its spare hands can work, once every extractor it can build is built.

Both are the arithmetic already written out in `C-7`'s table, done in the model rather than by hand.

Not done in the same commit as the specification's change, deliberately. It is a rule about what the
game rewards, the tests that pin it are the ones that would have to change with it, and `C-8` says
nothing in play reaches it either way - so it is worth doing carefully rather than quickly.

### C-11 - The model implements the previous turn, so `R-6` is blocked in code

**to** code · **status** open · **raised** 2026-08-31 · **source** re-running `C-8` after `P-126`

This lane's own, recorded so that nobody - including this lane - tries to play `R-6` through and
concludes something from the wrong rules.

`R-6` is unblocked in the **specification**: with metal and energy carrying, ten territories can
hold a Yard, nine can produce an Ark, and territory 1 can run the whole loop by itself. It is
**not** unblocked in the **code**. `game.rs:705` still does

```rust
self.territories[id.index()].stores = [0; 3];
```

at the end of every turn, with a comment saying *unused resources are discarded* - which is what
`spec/turn.md` said until `P-126` and `P-138`, and is now true of food alone. A play-through run
today would hit `C-8`'s wall and prove nothing about the game as specified.

**Weigh this against `spec/turn.md` -> Order of operations, not against the release's table.** A
turn ends by eating, then growing or starving, then `spec/turn.md` says *what expires expires, and
what was not kept in order is lost*, and then everything becomes ready. Separately, what a territory
can keep is bounded. The model discards all three stores instead, which is neither of those things.

Quoted so the file is named next to the words, because `crates/game-console/tests/quotations.rs`
only checks a quotation attributed to the document it quotes. Attributed to *the specification* it
read as prose, and this item could have gone on quoting wording the specification had dropped -
which is the thing that guard exists to catch, in the outbox of the lane that built it. The release's row order says something different again and `P-184` moves it, so the
table is the wrong thing to check a model against while it is still moving.

Three divergences, and none of them should be fixed yet:

- **Stores are discarded rather than carried.** The whole of the above.
- **Nothing is bounded.** `Territory::add` grows without limit, and `spec/turn.md` says what a
  territory can keep is bounded. The number is `C-10` and is not chosen yet, so there is nothing to
  implement even if this were the moment.
- **`is_fully_exploited` asks for a Yard everywhere**, which is `C-9`.

**Deliberately not done now.** `P-134` rewrites this model - state becomes things, in places, and
how many of each, and the five shapes it removes are exactly the ones these live in: `stores` as a
fixed array, citizens and yards as bare counts, extractors in a `Vec`, a garrison in an `Option`.
Fixing the turn inside those shapes means fixing it again inside the ones that replace them, and
the specification lane's account is that Sean's next work is the full specification, worked out in
`prototypes/kinds`, which the model is then built from.

So this is a **note that the two have parted**, not a request to reunite them today. What it buys
is that the next person to reach for `R-6` reads this first rather than measuring the old rules
again - which is what this lane just did, twice.

### C-10 - What a territory can keep is bounded, and nothing says by how much

**to** spec · **status** **answered** 2026-09-01 · `P-156`

The release has a section for it now: what a territory has room for, ten numbers, two of them
already determined. The finding was that everything about whether the release can be finished hung
on a number nobody had written, and the number is written.

### C-5 - Two documents list every crate, and neither list is right

**to** spec · **status** **answered** 2026-09-01 · `a6b67a7`, and the table before it

Both halves done. `docs/architecture.md` gained the rows and is checked against the workspace by a
test - `S-2`, wired to both gates in `302acc4` after `C-12` found that nothing ran it. `README.md`'s
crate tree lists the sixteen that exist and is asserted against the directory.

Filed as one stale table and closed as two lists that cannot go stale silently, which is the
difference between fixing an instance and fixing the mechanism.

### C-4 - The index is shared, so staging is publishing

**to** spec · **status** **answered** 2026-09-01 · `CLAUDE.md`, and `docs/process.md` in Sean's own words

*Stage by name, never everything* now says what it guards against: the git index is shared, so a
file one instance stages is committed by whichever instance commits next, under a message about
something else.

**This item said *Fixed:* in its own body and stayed marked open for a day.** It is the failure
`Q-38` is about - an item is closed by whoever filed it and answered by somebody else - surviving in
the one outbox whose owner built the reconciliation, because that reads commits citing an id and
nothing cited this one. A note inside an item is not a status.

### C-12 - The architecture check exists and nothing runs it

**to** code · **status** **acted** 2026-09-01 · `302acc4`

Both gates run `cargo test --manifest-path tools/outbox/Cargo.toml` now, so the check that every
crate has a row in `docs/architecture.md` runs on every push rather than when somebody types its
path.

It needed no synthetic poison. **It had already fired for real**, ten minutes earlier, on
`prototypes/kinds` - a crate this lane added and a row nobody wrote - and that is what found the
whole thing. The specification lane added the row in `97aef54` and the gate lines followed
immediately, which is what should have happened the first time.

What the delay cost is worth keeping: the check was written, correct, and silent for as long as
nobody typed its path. **A check nobody runs has no answer, and no answer looks exactly like a
right one.**

### C-8 - No Ark can ever be produced, so the loop cannot reach its last two steps

**to** spec · **status** **withdrawn** 2026-08-31 · superseded by `C-10`

Wrong now, and wrong in its premise rather than its arithmetic. It rested on *`settle` discards
every store at the end of every turn*, so a territory had to make fifteen metal in **one turn** or
never hold a Yard. `P-126` and `P-138` changed that: `spoil` takes food and nothing else, so metal
and energy carry, and any territory making any metal reaches fifteen eventually.

Recomputed: ten of twelve can hold a Yard rather than four, nine can produce an Ark, every
territory is reachable, and territory 1 can do the whole thing by itself. The deadlock between 11
and 12 is gone because 2, 8, 9 and 10 can all send a pioneer once they need not make both
resources in the same turn.

What survives is one number: `C-10`.

Worth keeping rather than deleting, because the finding was correct when it was filed and the rule
it depended on was changed for other reasons. **A finding is a claim about a specification at a
moment**, and the way it goes stale is that the specification moves under it - which is an argument
for re-running a measurement before acting on it, not for filing fewer of them.

### C-7 - `R-6` cannot be vetted: eight of the twelve territories can never hold a Yard

**to** spec · **status** **withdrawn** 2026-08-31 · answered in part by `P-125`, superseded by `C-10`

Two halves, and both are gone. The qualifier half - `is_fully_exploited` asking for a Yard
everywhere while `spec/control.md` said *every structure that can be built* - was answered by
`P-125`, which also defined what *can* means; implementing that is `C-9`.

The arithmetic half rested on the same discarded-stores premise as `C-8` and fails with it. Eight
of twelve becomes two of twelve, and both of those are deliberate demonstrations rather than
accidents.
### C-6 - The composition root holds a harness, and the rule says it holds nothing

**to** spec · **status** **answered** 2026-08-30 · `1d8c46f`

`docs/architecture.md` now carries the exception rather than leaving this lane to assert one in a
doc comment: *one thing may live here that looks like a violation and is not - the harness that
drives the shipped binary from outside*, because a harness running a special path would be evidence
about the harness. It adds the part this lane did not think to ask for and should have - **its
tests are tests of the harness rather than of the root** - and closes the door behind it: anything
else large enough to be worth testing has still leaked.

Verified against the file. The rule is stronger than the item asked for, because it says what the
exception does *not* license.

### C-3 - A prototype cannot photograph itself, and two items now need it to

**to** spec · **status** **answered** 2026-08-30 · `e3ddfdc`

`docs/prototypes/README.md` now says that where a prototype exists to settle what something *looks
like*, the means of seeing it is **the instrument the question needs rather than polish** - with the
test spelled out: does leaving it out save work, or does it prevent the question being answered?
That answers the prior question this item was actually about, and it answers it the way the item
could not assume.

Acted on in `a4e3bd1`: `prototypes/planet-view` gained `--shot`, `--settle` and `--renderer`, which
is what let `Q-1`'s last third be verified rather than guessed. It caught two real breaks within a
day - a shader that failed to compile, and later an embedded asset path broken by a crate split,
neither of which any test or type could see.

### C-2 - Architecture rule 6 states the losing side of a decision as fact

**to** spec · **status** **answered** 2026-08-30 · `2ca59d3`

Rule 6 no longer says every game entity is an ECS entity. It says game state lives in the model and
changes only by a transition, that an entity is never where a fact about the game is kept, and that
entities exist where the engine needs something to draw or to receive input. Verified against
`docs/architecture.md`, and it matches what the code does.

### C-1 - Whose file is a generated one at the repository root?

**to** spec · **status** **answered** 2026-08-30 · `6e3cd6c`, and sharpened in `82d7cff`

`CLAUDE.md` carries the rule: a file generated in full has no owner and may sit in the root, nobody
edits it, and a hand edit is overwritten at the next commit. The second commit adds the part that
matters to this lane - the content comes from its sources *as they sit on disk*, so a generated file
can publish work in progress, which is a defect in whatever writes it rather than something the rule
allows.

That is exactly what `Q-36` turned out to be, and `hooks/pre-commit` now refuses to rewrite
`pending.md` while any outbox has unstaged changes.
