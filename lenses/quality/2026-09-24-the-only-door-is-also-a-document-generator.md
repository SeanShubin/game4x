# The only door into the model is also a document generator, and the generators use the door

**Derived.** 2026-09-24, the separation-of-concerns review the code lane requested and Sean set the
brief for. Not binding.

[Quality](README.md) · [Architecture](../../docs/architecture.md) · [Console](../../spec/console.md)

**Measured at `a29d17ff`**, which is **not** the `5437d219` the code lane named - the tree moved
between their request and this reading, by `C-140`. Every figure below is from `a29d17ff` and every
command is given so it can be re-run.

Sean's brief: *interacting with the player, storing user interface state, operation of the game
engine, should all be separate and not cluttered with unnecessary details of the other concerns*,
and *no part of the code has access to another part it shouldn't*. He asked the code lane for the
thing it cannot see from inside: **where one concern reads another's details because it happens to
be able to.**

## What is clean, said first so the rest is not read as an indictment

**No engine framework below the adapter layer.** Read from every workspace `Cargo.toml`:
`game-model` depends on `planet-model` alone; `game-console` on `command-language`, `game-model`,
`planet-model`, `planet-terrain`, `sphere-tessellation`. **No `bevy` in either**, which is rule 7
holding by the compiler rather than by discipline, and rule 7 names `cargo tree` as its audit.

**The composition root is 154 lines** - `crates/game4x/src/main.rs` - against a rule that says it
holds no logic.

**`game-front` goes through the console.** Six uses of `game_console::` against two of
`game_model::`, and both of the two are `Phase::Play` inside assertions rather than state access.

**And `spec/console.md:210-222` is a better frame than the brief.** `P-551`, promoted the same day,
divides lines by *what each changes*: a **game command** changes game state and makes a transition; a
**local command** changes local state - what is selected, where the view is, which panel is open -
makes no transition and is not in the history; a **query** changes nothing; a **front-end line**
begins with `/` and changes the application rather than the game. **The rule is normative**: *The
game knows nothing of the interface. Local state is not game state, no rule reads it, and no local
command is a transition - so a replay of the history is a replay of the game and not of the
clicking.* **That last clause is a check rather than a reading**, and it is the observable to reach
for.

## 1. `Game`'s state is public, and production code outside the model writes it

**Where.** `crates/game-model/src/game.rs:143-166` - `phase`, `turn`, `territories`, `adjacency`,
`units`, `orbits`, every one `pub`. And `crates/game-console/src/worked.rs`, thirteen writes.

**What, measured rather than supposed.**
`grep -rnE "\.(territories|units|orbits|adjacency|phase|turn)\s*(=|\.push|\.insert|...)" crates --include=*.rs`
outside `crates/game-model/` gives **28 sites in `game-console`**, and the population splits:

- `containment.rs` - `#[cfg(test)]` begins at line 932 and every write is at 939 or later. **Test
  fixtures.**
- `tree.rs` - `#[cfg(test)]` at 214, every write at 229 or later. **Test fixtures.**
- `worked.rs` - **no `#[cfg(test)]` anywhere in 508 lines**, and thirteen writes between 170 and
  506. **Production code.**
- `fired.rs` - two hits are `one.turn == 0`, reads that the pattern caught. **Not writes.**

**So the claim is thirteen, not twenty-eight**, and the code lane's own suspicion was right about
the mechanism while nobody had separated the fixtures from the shipped path.

**Why, and it is not *this produces an impossible state today*.** `ground()` at
`worked.rs:491-507` writes `game.phase = Play; game.turn = 1;` - which is exactly and only what
`Transition::Start` does at `crates/game-model/src/rules.rs:122-124`. **It matches today.** It is a
second statement of a transition's body, in another crate, that nothing keeps in step:
`Start` gaining a third write leaves `ground()` reading correctly and producing a different state,
and **`spec/invariants.md` -> *A fact is stated once* is the rule it breaks.**

**The same for identity.** `worked.rs` passes `UnitId(1)` by hand at five sites, where the model
mints `UnitId(units.len() as u32 + 1)` - a rule `game-model` itself states **ten times**, at
`game.rs:1622, 1782, 1845, 1907, 1940, 2046, 2092, 2132` and `rules.rs:118, 303`. The console
restating it is the eleventh, across a crate boundary.

**And the cost is specific, because of what `worked.rs` is for.** `R-7`'s *vetted when*: Sean reads
`reports/recipes.md` and **derives the after from the rule and the before by hand.** So the
*after* is trustworthy - it is produced by running the real command, which is `P-330`'s whole point
- and **the *before* is the hand-built half.** The file's own header argues against exactly this:
*a written example can show behaviour the code does not have, and this repository has produced
three of those in one week... All three sat in artifacts whose purpose was hand-derivation.* **The
file exists to stop hand-drawn examples and hand-draws the half he reasons from.**

**Whether. Worth fixing now, and the cheap repair is not privacy.** Making the fields crate-private
would red the fixtures too and they are not the problem. **What `worked.rs` needs is to reach its
before-states through the transitions that produce them** - `Transition::Start` rather than two
assignments, and a minted id rather than `UnitId(1)` - so that the before is a state the game can
be in. **The check that says it worked**: no `game.` field assignment or `push` outside
`crates/game-model/` in any file lacking `#[cfg(test)]`. That is a grep, it is red today at
thirteen, and it would have been red the day `worked.rs` was written.

## 2. The crate called *the only door into the model* is 73% document generation

**Where.** `crates/game-console/`, and `docs/architecture.md:149`.

**What.** The architecture document describes the crate in its own words: **The command language
bound to the game. The only door into the model.** Its *kind* is `binding`, and `:52` says *a kind
says what a crate is for and a layer says what it may know about*.

**Measured over all twenty files, by each module's own `//!` first line rather than by its name:**

| Answering a player                        |     | Generating documents for a reader              |      |
| ----------------------------------------- | --- | ---------------------------------------------- | ---- |
| `lib` *words in, transitions out*         | 719 | `dump` *tables a person can read*              | 1803 |
| `state` *a state as a data file*          | 739 | `nogain` *nothing comes back round with more*  | 1276 |
| `binding` *what each command means*       | 643 | `containment` *the state as a tree*            | 1261 |
| `grammar` *the shape of every command*    | 547 | `petri` *the rules as a Petri net*             | 980  |
| `report` *answering questions*            | 481 | `declare` *the release's declaring tables*     | 803  |
| `fired` *which recipe each command fired* | 269 | `tree` *a page that collapses*                 | 534  |
|                                           |     | `worked` *a worked example beside each recipe* | 508  |
|                                           |     | `relations` *`spec/data/` as relations*        | 417  |
|                                           |     | `petri_page` *the net as a diff and as a page* | 408  |
|                                           |     | `browse` *every reference is a link*           | 315  |
|                                           |     | `style` *two stylesheets*                      | 290  |
|                                           |     | `petri_draw` *the net as a picture*            | 282  |
|                                           |     | `recipes` *every recipe, gathered*             | 260  |
| **3,398**                                 |     | **9,137**                                      |      |

12,535 plus `bin/dump-state.rs` at 52 is the crate's 12,587. **Twenty-seven per cent of the crate
does what the architecture document says the crate is.**

**Why this is Sean's question and not a tidiness one.** *The only door into the model* and *a
document generator* in one crate means **the document generators hold the door's key** - which is
how finding 1 is possible at all. `worked.rs` does not reach past a boundary to write `game.units`;
it is inside the crate that owns the boundary. **Two concerns in one crate is not the defect. The
defect is that one of them is the access control for the other.**

**Two details inside it, neither worth its own item.** `style.rs` is *two stylesheets* and
`petri_draw.rs` is *the net as a picture*, so **rendering lives in a crate the layer table puts under
supporting** - and `docs/architecture.md`'s layer table gives Rendering its own row. And
`declare.rs` is **803 lines with no `#[cfg(test)]` and `assert!` in its body** - verification that
would panic a player's session, inside the player-facing crate.

**Whether. Worth doing, and it is the code lane's own suspicion 1 with a ratio attached rather than
a new claim.** The seam the measurement suggests is the audience: what answers a player, and what
writes a file for a reader. **What this lens is not deciding is where the door goes** - whether the
split leaves the door in the small crate or puts it in a third beneath both is rule 1's question and
theirs.

## 3. Their suspicion 2 is refuted, and something else is in that place

**They offered:** *`crates/game-console/src/declare.rs` reads `releases/first-release.md` and
generates `spec/data/`.*

**It does not generate them.** `declare.rs` contains no write: `fs::write` and `File::create` appear
in `src/bin/dump-state.rs:48` and in `tests/expected_state.rs`, and nowhere in `declare.rs`. What it
holds is `assert!` and `assert_eq!` - it **asserts against** `spec/data/`'s shape rather than
producing it.

**And a sentence in it is easy to read as agreeing with them, which is why this is worth stating.**
`declare.rs:44` says **it cannot write these files** - and *it* is
`game_model::containment::Description::ordered`, three lines earlier, not `declare.rs`. This lens
read that antecedent wrongly on first pass and checked it, which is the only reason the refutation
is here rather than a confirmation.

**What is in that place instead** is the production-assert point above: a library module, 803 lines,
no test module, asserting about the specification's data files at runtime.

## What this review did not do

**It did not measure user-interface state**, because `spec/console.md`'s *local state* - what is
selected, where the view is, which panel is open - landed at 20:56 the same day and **no crate names
it yet**: `grep -rn "local state" crates --include=*.rs` is empty at `a29d17ff`. So Sean's second
concern has a rule and a vocabulary and no code, and there is nothing to find wrong. **A count of
zero over a population of zero**, said rather than reported as clean.

**It did not run the replay check**, which is the one instrument this review would most want:
*a replay of the history is a replay of the game and not of the clicking.* Nothing exercises it
today, and building it is the code lane's. **It is the check that would hold Sean's whole brief in
one assertion**, and it is worth more than either finding above.
