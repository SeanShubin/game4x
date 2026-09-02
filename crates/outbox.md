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
