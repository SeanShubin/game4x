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

### P-334 - `adjacency` is stored and appears in nothing you read

**to** sean - **status** open - **raised** 2026-09-07 - **kind** contradiction - **asks** a decision
- **into** `releases/first-release.md` -> Where things are

**Found while looking at whether a drawing would help, which is not what I was looking for.**

**`adjacency` is declared stored** - `releases/first-release.md`, *Traits*: *a thing that holds
places · which of the places it holds are next to which, and by which kind of edge · **stored***. It
is `Game.adjacency` in the model, a `Vec<Vec<TerritoryId>>`.

**It appears zero times in `scenario/expected/play.4x`, zero times in `reports/containment.html`, and
zero times in `reports/state.md`.** Counted, not remembered.

**So the round trip does not close, and `S-62` closed on a test that does not cover this.**
`every_territorys_own_numbers_survive_the_round_trip` rebuilds each territory's id, biome, nature and
per-resource pair. **Adjacency is none of those**, and `P-320` says the check is that the dump reads
back into the state it came from. **A file with no adjacency cannot rebuild the planet.**

**This is the same shape as `C-53` twice over.** That one found `total capacity` stored and unwritten;
I fixed it and reported the round trip closed. **It closed for a territory's own numbers and I said it
closed**, which is the overstatement `C-53` was filed about, made again by me in the closure of the
item that fixed it.

**Two ways, and the second is not obviously worse.**

- **Adjacency goes in the file.** It is a fact about the container, so it sits under the game rather
  than inside each territory - which is the shape you drew when you hoisted it. **The round trip
  closes for real**, and the cost is a large block of text in a file you read
- **`P-320`'s check is narrowed on purpose**, saying the file states what things contain and what
  they are, and that the planet's shape is stated once in `create planet` and not repeated. **The
  commands already carry it**: `{create-planet tiny}` determines every adjacency, so the file is
  reconstructible from commands plus expected even if expected alone is not

**I lean to the second and I am not confident.** The commands file is one of the four artifacts and
is reviewed beside the expected data, so *the planet's shape is in the commands* may be a true
statement of where that fact already lives rather than a narrowing. **What decides it is whether you
read the expected file alone or always beside the commands** - and that is about how you work, which
I should not guess.

### P-332 - `R-7` covers ten of sixteen recipes, and the other six cannot be fired alone

**to** sean - **status** open - **raised** 2026-09-07 - **kind** gap - **asks** a decision -
**into** `releases/first-release.md` -> Capabilities, `R-7`

**The code lane's `C-59`, and it is the honest half of a capability you approved yesterday.** Ten
player recipes now show a state, the command, and the state after, in the expected data's notation.
**The world's six do not**, and the reason is not effort.

**`R-7` asks for *the command that fires it*. The world's six all fire on `{end-turn}`**, so there is
no command that fires one of them.

**And four cannot be shown alone even in principle.** `grow` never fires without `upkeep` having run
first - a citizen has upkeep, so there is no state with citizens and surplus food where `upkeep` does
nothing. `perish` is the same coupling from the other side. `age` and `spoil` are two halves of one
rule about food. **They are not six independent things that happen to share a command.**

**Three shapes, and none is the code lane's to choose.**

- **One example per turn ending.** A state, `{end-turn}`, and the state after - **the six shown
  working together, which is how they work.** You would read one example rather than six and see the
  couplings rather than lose them. What it gives up is *which recipe did which part*
- **A command per world recipe**, so each can be fired alone. **A change to the game to serve a
  report**, and `grow` alone is a state the game cannot reach
- **`R-7` scoped to the player's ten**, with `turns.md` covering what a turn ending does. **Cheapest,
  and it says plainly that six recipes are confirmed only in aggregate** - which is what you asked to
  stop being true

**My recommendation is the first.** The coupling is a fact about the game rather than a limitation of
the report, and an example that shows six recipes resolving together is closer to what you would
derive by hand than six examples that pretend they are separable.

**What is already true whichever you choose.** The six say under their own rule that they have no
example and why - *No worked example: this is the world's, and the world's six all fire on
`{end-turn}`* - and **the excuses fail if one is repaired**, so none can outlive its reason.
