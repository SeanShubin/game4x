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

### C-6 - The composition root holds a harness, and the rule says it holds nothing

**to** spec · **status** open · **raised** 2026-08-30 · **source** acting on `Q-11`

`docs/architecture.md:55` - *because it holds no logic, it needs no tests of its own; if it is
large enough to be worth testing, something has leaked into it that belongs elsewhere.*

`crates/game4x` is 517 lines with eight tests. Most of that is `inspect.rs`, the remote control that
puts the camera at stated angles, runs commands, waits for the world to settle and writes a PNG and
a dump. **It cannot move**, and the reason is the whole point of it: it has to drive the *shipped*
binary, because a harness that ran a special path would be evidence about the harness.

`69ab140` fixed the false half - `main.rs` claimed the crate held no logic. But it now says the
rule *is broken here on purpose*, and **a rule owned by `docs/` is not this lane's to carve an
exception into**, even a true one. Either the document says a root may hold the harness that
operates it, and why, or the harness needs a home that does not exist yet.

Worth deciding rather than leaving as a comment, because the rule is load-bearing elsewhere: it is
what stops game logic drifting into the root, and a rule with an undocumented exception is harder
to apply than one with a documented one.

### C-5 - `docs/architecture.md` lists every crate, and there is now one it does not

**to** spec · **status** open · **raised** 2026-08-30 · **source** acting on `Q-5`

`crates/planet-presentation` landed in `49c4c46`. The table at `docs/architecture.md:117` names
every crate with its layer and its dependencies, and the new one is not in it - and the
`planet-bevy` row beside it now understates its dependencies by one.

Not this lane's to fix: `docs/` is the documentation lane's column, and a table of layers is a
claim about the architecture rather than a list of directories.

Two things worth deciding with it rather than after it:

- **A table of every crate goes stale every time a crate is added**, which is the same shape as the
  specification index that lost three files. Whether it should be generated, or checked by a test
  that compares it against the workspace, is a question about that document rather than about this
  crate. This lane can build either once the answer is known.
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
