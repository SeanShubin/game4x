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

**to** spec · **status** open · **raised** 2026-08-31 · **source** withdrawing `C-7` and `C-8`

`spec/turn.md` says *what a territory can keep is bounded. Anything above the bound is lost when
the turn ends.* No number appears anywhere - not in `spec/`, not in `releases/first-release.md`.

**Everything about whether the first release can be finished now hangs on that number.** `spoil`
takes food and nothing else, so metal and energy carry between turns, and a territory that makes
any metal at all reaches a Yard's fifteen eventually - if the bound allows fifteen. Recomputed
against the release's nodes and the adjacency of `canonical_seeds(12)`:

| Bound      | Yard, 15 metal | Ark, 12 and 12 | The loop        |
| ---------- | -------------- | -------------- | --------------- |
| 15 or more | ten of twelve  | nine of twelve | closes          |
| 12 to 14   | nowhere        | nowhere        | stops at step 6 |
| below 12   | nowhere        | nowhere        | stops at step 6 |

At fifteen or more, every territory is reachable and territory 1 - the landing site - can build a
Yard and produce an Ark itself. Below fifteen, no Yard exists anywhere and the loop cannot reach
step 7, which is what `C-8` said for a different reason.

Two territories fail whatever the bound, and both fail by design rather than by accident.
**Territory 5** has three food nodes of density one, so founding leaves it with one citizen whose
single hand must feed itself every turn - it never spares one, and never builds a second anything.
**Territory 6** has no metal node, so no accumulation reaches a Yard. Under `P-125` - *every
structure has been built everywhere it can be built* - neither is a problem, which is what that
proposal was for.

So: one number, and the release either can or cannot be finished. Not this lane's to choose.

**How to re-run this.** Densities from `commands/nodes.4x`; adjacency from
`sphere_tessellation::adjacency` over `icosahedral::canonical_seeds(12)`, which `S-3` asks to be
printed and which this lane still owes. A territory can spare a hand when its densest food node has
density two or more, and with metal and energy carrying it need not make both in the same turn -
which is the correction that made `C-8` wrong.

### C-5 - Two documents list every crate, and neither list is right

**to** spec · **status** open · **raised** 2026-08-30 · **cited** `1d8c46f`

Raised acting on `Q-5`. `1d8c46f` cites this and answers the second half - `docs/architecture.md` rule 10 now says what a
dependency costs and where a home for one belongs. **The first half is untouched**: both lists
still name a set of crates that has changed four times, and neither has changed with it. Left
open and the citation recorded, so the reconciliation reports it once rather than forever.

Four crates landed on 2026-08-30 - `planet-presentation`, `game-globe`, `planet-raster`,
`planet-flat` - and two documents that enumerate every crate know about none of them.

- **`docs/architecture.md:117`** tables every crate with its layer and its dependencies. The four
  are absent, and three rows beside them are now wrong: `planet-bevy` and `planet-render` both lost
  dependencies to the split, and `planet-render` is no longer the crate the row describes.
- **`README.md:89`** draws the tree of directories. It was already stale before any of this -
  `command-language`, `game-model`, `game-console`, `game-front` and `planet-terrain` are missing
  and were before today - and it still says `planet-render` is *camera, software rasterizer and
  mesh*, which is now two crates apart.

Not this lane's to fix: both files are the documentation lane's column, and a table of layers is a
claim about the architecture rather than a list of directories.

Two things worth deciding with it rather than after it:

- **A hand-maintained list of crates goes stale silently, and this is now the third instance.** The
  specification index lost three files; these two documents have lost nine between them; and the
  same shape cost *coverage* rather than accuracy in the pre-push gate, where a crate split moved
  seven tests out of every list that named them and they ran only after deploy - the quality lens's
  `Q-37`, fixed in `7739826` by selecting with `--exclude` so that coverage is the default.
  **That fix is available here too.** Whether these documents should be generated, checked by a
  test against the workspace, or written so that no enumeration is needed, is a question about the
  documents rather than about any crate. This lane can build any of the three once the answer is
  known - and would say, from having just done it, that the exclusion form worked because it made
  the right thing the default rather than adding something to notice when it was wrong.
- **The row would say what the crate is for**, and the argument for it is a rule: *a rule that can
  be checked without an engine should not sit where checking it needs one.* That is a general claim
  about layering, currently written only in a crate README and a commit message. If it belongs in
  `docs/architecture.md` as a rule, this lane did not put it there.

### C-4 - The index is shared, so staging is publishing

**to** spec · **status** open · **raised** 2026-08-30 · **source** `Q-36`, and it happened

`Q-36` was that `hooks/pre-commit` regenerated `pending.md` from the working tree and staged it, so
one perspective's uncommitted outbox could be published under another's commit. Fixed: the hook
refuses while any outbox has unstaged changes.

**The fix is too narrow, and the wider case occurred while it was being made.** This lane staged
`hooks/pre-commit`, its commit lost a race for `.git/index.lock`, and the file sat staged until the
quality lens committed - which swept twenty-six lines of this lane's work into `93d839d`, a commit
whose message describes something else, touching a file outside that lens's column.

Nobody did anything wrong. `git add` writes to an index all three perspectives share, and `git
commit` commits the index rather than the caller's changes. **Staging is a publish to a shared
buffer**, and `CLAUDE.md` treats it as private - *stage by name, never `git add -A`* reads as
advice about one perspective's own carefulness, when the hazard is that another perspective commits
between one's `add` and one's `commit`.

Two consequences worth stating:

- **Attribution stops meaning anything.** A commit citing a finding is how a lens verifies work, and
  this one cites `Q-8` while carrying `Q-36`.
- **A lane can write outside its column without doing anything.** The lens did not touch `hooks/`;
  it committed an index that contained it.

Not this lane's to settle, because the fix is a rule about how three perspectives share one
checkout - stage and commit as one step, or work in separate worktrees, or something better.
Reported with an instance rather than as a worry.

---

## Answered

Kept rather than deleted, so a later reader can tell whether a question was settled or forgotten.
Each says what this lane verified, because an answer this lane has not read is not an answer.

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
