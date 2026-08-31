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

### C-7 - `R-6` cannot be vetted: eight of the twelve territories can never hold a Yard

**to** spec · **status** open · **raised** 2026-08-30 · **source** starting `R-6`

`R-6` is vetted when a person reaches *a fully exploited planet* and launches an Ark.
`is_fully_exploited` asks that **every claimable territory** be founded, hold at least one yard, and
have an extractor on every node. A yard costs 15 metal, spent from that territory's own store, and
`settle` discards every store at the end of every turn - so a territory must produce 15 metal **in a
single turn** or never hold one.

Most cannot. Metal per turn is bounded by the density of the metal nodes a territory's own
population has hands to work, and its population is bounded by its food:

| Territory | Metal nodes | Most metal in one turn | Yard |
| --------- | ----------- | ---------------------- | ---- |
| 1         | `3 x 4`     | 12                     | no   |
| 2         | `2 x 4`     | 8                      | no   |
| 3         | `2 x 4`     | 8                      | no   |
| 4         | `4 x 5`     | 5, on two citizens     | no   |
| 5         | `8 x 8`     | 0, on one citizen      | no   |
| 6         | none        | 0                      | no   |
| 7         | `4 x 5`     | 20                     | yes  |
| 8         | `1 x 2`     | 2                      | no   |
| 9         | `6 x 8`     | 32                     | yes  |
| 10        | `1 x 3`     | 3                      | no   |
| 11        | `5 x 6`     | 30                     | yes  |
| 12        | `8 x 8`     | 16                     | yes  |

**Territory 6 has no metal at all**, so no arrangement of anything reaches a yard there. Territory 1,
the landing site, tops out at 12 against a cost of 15.

Two territories fail a second way as well. **5 is stuck at one citizen forever**: founding gives it
one citizen and one food extractor on a density-1 node, so working that extractor yields one food
for one citizen - `population_after(1, 1)` is `1`, and the labor is spent - while building anything
instead yields no food and `population_after(1, 0)` is `0`. It reaches 1 of its 19 nodes. **4** is
held at two citizens the same way.

Nothing crosses a border to help. Stores are discarded each turn, no transition moves a resource or
a citizen between territories, a pioneer entering a held territory perishes, and `found` refuses an
`AlreadyControlled` one. Not founding a territory does not help either: an unfounded one fails the
same test, and none of the twelve is ocean.

**This is the release disagreeing with itself, not a defect in the code.** The territory table was
built so that each one exercises a different consequence - *no metal*, *food density 1*, *the
minimum a territory can be* - and `commands/biomes.4x` says 5 exists so that *a territory which can
never build anything is demonstrated deliberately rather than produced by accident*. Those are good
things to demonstrate. They cannot coexist with a win that requires **every** claimable territory to
hold a yard.

Four ways out, and choosing is not this lane's:

- **Narrow what fully exploited means** - every territory that *can* be, rather than every one.
  Changes no number Sean chose, and keeps every lesson the table was built to teach.
- **Require the yard once**, on the planet rather than per territory. One shipyard builds the Ark.
- **Let something cross a border** - a resource, or a citizen. A new rule, and a bigger change than
  it looks.
- **Change the numbers** so every territory can reach 15 metal, which costs most of what the table
  exercises.

**Proceeding under the first**, because it is the only one that changes nothing already chosen. The
play-through is being built to exploit every territory that can be and to launch an Ark from one
that can hold a yard, which is the same work under any of the four. It will stop short of `has_won`
until this is settled, and the evidence for `R-6` will say so rather than reporting a pass.

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
