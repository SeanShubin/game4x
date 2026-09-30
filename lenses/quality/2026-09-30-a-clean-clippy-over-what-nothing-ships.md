# A clean clippy over what nothing ships

**Derived.** 2026-09-30, at `e85c2980`. The review Sean asked for: dead code, simplicity, and
whether implementations are isolated behind composition roots. The `code-quality-ecs` plugin's
nine rules were read first and are answered in their own section.

[Quality](README.md) · [Architecture](../../docs/architecture.md) · [The rules](../../docs/architecture.md#rules)

**The headline is the pair, not either half.** `cargo clippy --workspace --all-targets` reports
**zero warnings over 23 packages**, and **3,408 lines of `crates/` reach no shipped binary** while
three declared dependency edges are named by no line of code. **Every instrument that would see
this reads the dependency graph, and clippy does not** - it judges a crate against its own
declared surface, where a `pub` item is reachable by construction and a linked crate is a fact
rather than a claim.

**So *the lints are clean* and *nothing is dead* are different sentences**, and the first has been
standing in for the second.

## How each number here was produced

Four scripts, each asserting the size of every population it ranged over, because a count over
nothing is the same failure with the sign flipped. They are scratch and are not committed; each is
one screen and is described where it is used.

**One of them was wrong and was caught by re-deriving from the manifests.** A first run said
`docs/architecture.md` omitted nine dependency edges. Five of the nine were dev-dependencies,
which that column does not claim to list - **the instrument answered *declared in any dependency
section* where the question was *declared as a runtime dependency***. The real number is four.
Found before it was reported, by reading four manifests the script had flagged and seeing that
`graph-coloring` really does declare nothing. **Measured: four. The reason the first number was
nine is a section filter that accepted any heading ending in `dependencies`, which is not an
inference - it is the line that was changed.**

---

<a id="1"></a>

## 1. Three crates in `crates/` reach no binary, and one of them states an invariant the game breaks

**Where.** `crates/planet-ecs/` (387 src lines), `crates/planet-flat/` (764),
`crates/planet-raster/` (2,257); and `crates/planet-ecs/README.md:5-9`.

**What.** Walking runtime `[dependencies]` from every binary in the tree, the only binary under
`crates/` is `game4x`, and its closure is 13 local crates. Three mainline crates are outside it and
reachable only through `prototypes/planet-view`:

```
game4x        (13) command-language, game-console, game-front, game-globe, game-inspect,
                   game-model, graph-coloring, planet-bevy, planet-model,
                   planet-presentation, planet-render, planet-terrain, sphere-tessellation
planet-view   (10) ... planet-ecs, planet-flat, planet-raster ...
```

That is **3,408 of the 27,141 source lines in `crates/`, 12.6%**, whose only path to a binary is a
prototype. `crates/friendly-notation` (859) is also outside the closure and is **not** one of
these: it is a deliberate dev-dependency of `game-model`, used by nine test and example files,
and its own manifest comment explains the cycle Cargo permits for it.

**And `planet-ecs/README.md` claims the opposite of what ships:**

> Every thing in the world with identity and state lives here as a Bevy entity with components.
> There is no second way of holding game state — no parallel `Vec` of regions, no side-table of
> ownership.

`crates/game4x/src/main.rs:111` says, in the comment where the plugin is not added: *the
game's state lives in `game-model`, reached through the one console, and the globe follows it by
watching a counter. The regions it spawned were a second, unread copy.* **Both cannot be true, and
the binary settles it.**

**Why.** The README states a global invariant about the whole program from inside a crate the
program does not link. A reader deciding where game state belongs is told there is one home and
shown the wrong one - and this is the highest-value shape in this lens's brief, a crate
contradicting what the tree actually does. The reachability half costs less but costs steadily:
`docs/architecture.md`'s layer table lists all three beside the shipped adapters with nothing
marking them, so every later reader re-derives which ones ship.

**Whether.** The README sentence is **worth fixing now** - it is one paragraph and it is false.
The reachability is **worth stating rather than fixing**: rule 17 is Sean's own *all I really care
about on the mainline is the rendering work*, and `planet-raster` and `planet-flat` are rendering,
so their being in `crates/` is arguably intended. `planet-ecs` is not rendering, and whether it
belongs is a decision rather than an observation.

---

<a id="2"></a>

## 2. Three declared dependencies are named by no line of code, and one move left four residues

**Where.** `crates/game4x/Cargo.toml`, `crates/game-front/Cargo.toml`,
`crates/planet-flat/Cargo.toml`; and `crates/game4x/src/main.rs:28-36` and `crates/game4x/Cargo.toml:15`.

**What.** Of **63 local path dependencies** across 29 manifests, three runtime edges and one
dev edge appear in no non-comment line of the crate that declares them:

| Edge                                   | Where it is instead                                       |
| -------------------------------------- | --------------------------------------------------------- |
| `game4x` → `game-console`              | nowhere at all                                            |
| `game-front` → `game-model`            | nowhere at all; every import comes through `game_console` |
| `planet-flat` → `planet-bevy`          | one doc comment, `src/lib.rs:18`                          |
| `goldberg-move` → `planet-model` (dev) | nowhere at all                                            |

**The `game4x` edge has a traceable cause, and three siblings.** `8628c437` - *S-160: the remote
control becomes an adapter* - moved `src/inspect.rs` and `src/options.rs` out to
`crates/game-inspect`. `game_console::Outcome` was used at the old `inspect.rs:149` and nowhere
else in the crate, so the dependency was live before that commit and dead after it. Left behind
with it:

- **The module doc still apologises for the exception the move removed.** `main.rs:28-36` says
  *the crate as a whole does not* meet *a composition root holds no logic*, and names
  [`options`] and [`inspect`] as the two reasons. Neither module exists here; `game4x` declares
  no `mod` at all. **The rule it excuses itself from is now held.**
- **`Cargo.toml:15` explains a feature by a file that is gone**: *`png` is here for the
  screenshot path in `inspect.rs`*.
- **`docs/architecture.md:152` lists `game-console` and omits `game-inspect`**, which
  `main.rs` names four times.

**And `planet-flat`'s sentence is false in four of five places.** Its README and module doc both
say it names `planet-bevy` for `window_plugin`, *which is how every composition root here asks for
a window with vsync*. `window_plugin` has one caller in the tree - `prototypes/planet-view` -
while `game4x`, `goldberg-view` and `goldberg-move` each write their own `WindowPlugin`, and
`planet-flat` itself calls it from no code.

**Why.** For the composition root the dependency list *is* the architecture statement, and
`docs/architecture.md`'s column transcribes it, so a wrong manifest propagates into the document
that readers check the manifest against. A lens cannot tell from the outside whether the row or
the manifest is the thing that is right, which is why this is reported rather than resolved.

**Whether.** **Worth fixing now.** Four line deletions and two comment repairs, and the
composition root then documents itself correctly - which is the part of Sean's question this
answers directly.

---

<a id="3"></a>

## 3. Six uncalled `pub` functions, verified against every tracked file

**Where.**

```
crates/game-console/src/state.rs:456      pub fn as_a_turn
crates/planet-model/src/biome.rs:63       pub fn is_claimable
crates/command-language/src/syntax.rs:127 pub fn optional_command
crates/planet-model/src/world.rs:53       pub fn owned_by
crates/planet-render/src/mesh.rs:99       pub fn recolor
crates/planet-flat/src/gpu.rs:180         pub fn render_asset_usages
```

**What.** Of **488 `pub` names declared in `crates/*/src`**, 73 appear in no other tracked `.rs`
file. Eight of those are `#[wasm_bindgen]` exports in `game-front/src/shell/web.rs`, whose caller
is the page. Fifty-nine are used inside their own file and are over-wide `pub` rather than dead.
**Six have exactly one mention in the tree: their own declaration.** Each was then searched across
every tracked file of any type, with `owner` as a control at 50 files, so a zero would have been a
zero. None is a trait method; all six are free functions or inherent methods.

**Two of them say they are used.** `recolor`'s doc reads *Used for selection and, later,
ownership*, and `render_asset_usages` exists to name two flags that `planet-flat/src/lib.rs:226`
writes inline - the duplication this lens filed on 2026-08-28, when the two halves were in
different crates. **They are in the same crate now**, which makes it a smaller change than when it
was raised.

**Why.** Sean asked for dead code removed, and this is the part of it a compiler will never
mention: `pub` in a library is reachable by construction, so `dead_code` cannot fire and clippy is
green over all six. Two of the six carry a comment asserting a caller that does not exist, which
costs the next reader more than the function does.

**Whether.** **Worth fixing now**, and it reopens `Q-9` rather than adding to it: that item
recorded five of these in August as *noted and deliberately not, unless one is already being
touched*, and Sean asking for dead code to be removed is that condition.

---

<a id="4"></a>

## 4. `fn check` is six invariants in one function, and its own sentence names three

**Where.** `crates/game-model/src/schema.rs:947`, 206 code lines.

**What.** Its doc comment is *Every row fits its relation, its key is its own, and every reference
points at a row* - three clauses. The body runs **six independent top-level passes** and can
return **five distinct `Malformed` variants**: `TwoWithOneKey`, `NoSuchRow`, `CarriesNothing`,
`DoesNotCarry`, `TwoParents`. The three the sentence does not mention are the `carries` pass at
`+103`, the `part` parentage pass at `+173`, and a reachability walk over `parent.keys()` at
`+206`.

It is the second-longest function in the tree. Measured over **681 non-test functions in 86 source
files**, 21 are over 60 code lines and 10 over 100; five of the top eleven are in this one file.

**Why.** Rule 14 is *a check asserts the size of the population it checked*, and six invariants
sharing one entry point cannot each say what they ranged over - a caller learns that something is
malformed, not which of six things looked. The doc comment naming half of them is the ordinary
cost: a reader who trusts it will not know the `part` graph is validated here at all.

**Whether.** **Worth fixing eventually**, and the shape is given by the code: six named functions
and a `check` that calls them, each keeping its own variant. Not now - nothing is wrong with what
it computes, and this is the only one of the four findings here where that is true.

---

## What the `code-quality-ecs` plugin's nine rules say

99 tracked source files outside `tools/`, so the plugin's **medium** strategy: deep reading where
it matters, pattern scanning everywhere. **The ECS surface is eight files in `crates/` and four in
`prototypes/`** - `use bevy` appears nowhere else - so the ECS rules were checked over all eight
rather than sampled, and the general rules over the crate graph.

**Seven of the nine hold, and two are structurally enforced rather than merely obeyed.**

| Rule                    | Verdict                                                                                                                                                                            |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 Coupling and cohesion | **Holds.** `game-globe` exists precisely to keep it: `planet-bevy` draws a planet and names no game, so a polyhedron prototype no longer links the command language                |
| 2 System dependencies   | **Broken, deliberately** - see below                                                                                                                                               |
| 3 Event architecture    | **Not used, and the reason is stated.** Four systems watch monotonic counters because the one `Session` is off the engine's call stack on the web                                  |
| 4 Architectural layers  | **Enforced by the graph.** No non-adapter crate declares `bevy`; the four `bevy` mentions outside the adapter layer are doc comments naming the crate `planet-bevy`                |
| 5 Abstraction levels    | **Holds in the adapters**; the one real case is `fn check` above, which is not ECS code                                                                                            |
| 6 Module hierarchy      | **Holds.** No cycle in runtime dependencies; the single cycle in the tree is `game-model` ↔ `friendly-notation` through dev-dependencies, documented at `game-model/Cargo.toml:27` |
| 7 Naming and clarity    | **Holds, unusually well.** `FollowsTheGamePlugin`, `DecidesWhatToDraw`, `a_control_asks_for_a_reset` name design concepts, not data structures                                     |
| 8 System organization   | **Holds.** Every multi-system registration is `.chain()`ed or `.in_set()`; no implicit ordering found in five registrations                                                        |
| 9 Component design      | **Holds.** The resources are single-field newtypes - `Followed(u64)`, `ResetsSeen`, `DrawingAsksSeen`. No god resource                                                             |

**Rule 2 is the plugin's own top-severity class, and this tree breaks it nine times.**
*Undeclared dependencies: a system accesses global state not in parameters.* The game's `Session`
lives in a `static OnceLock<Mutex<Console>>` on the desktop and a `thread_local!` on the web
(`crates/game-front/src/shell.rs:27-58`), and **nine Bevy systems reach it through free
functions** rather than declaring it:

```
crates/game-globe/src/lib.rs:73, 80, 112, 140, 157      five systems
crates/game-inspect/src/lib.rs:107, 112, 162, 174       four
crates/game4x/src/main.rs:86, 106                       the root, which may
```

**This lens does not think the restructure is worth doing, and the reason is in the code.**
`game-globe/src/lib.rs:17-25` states it: the page calls into the console, so on the web the
`Session` is not on the engine's call stack at all and cannot be handed over. A `Res` wrapping a
global is a declaration in form and not in fact.

**What is worth knowing is the cost, because it was measured rather than predicted.**
`game-globe` holds five of the nine reads and has **1 test function**, the fewest of the five
adapter crates; `planet-ecs`, which ships in nothing, has 10.

```
game-globe     1 test fn    5 global reads
game-inspect   8            4
planet-bevy    7            0
planet-flat    7            0
planet-ecs    10            0
```

**Measured: the counts above. I think the reason is that a system reading a process-global cannot
be driven from a minimal `World`, which is the cost the rule predicts - but that is an inference
about why, and nothing here tested it.** A reader should take the five-versus-one as the finding
and the sentence after it as a guess.

---

## Noted and deliberately not

- **59 `pub` items used only inside their own file.** Narrowing each is mechanical and changes no
  behaviour; the ones worth touching will be touched for other reasons. Listed nowhere, because a
  list of 59 is a list nobody reads.
- **`crates/game-model/examples/` is a 6,352-line test-support library**, larger than the 5,234
  lines of `src/` it exemplifies, reached by **15 `#[path]` inclusions** from `tests/` and from
  other examples - `render.rs` by five consumers, `scenario.rs` by two, and one more in
  `game-console` makes 16 in the tree. Each needs `#[allow(dead_code)]`, which is why **16 of the
  tree's 17 dead-code suppressions sit in this crate's `tests/` and `examples/`, and not one is in
  any `src/`**. **So the one region where dead code is structurally invisible is the shared test
  scaffolding**, and `tests/common/mod.rs` already exists doing the same job a second way. Real,
  and a bad trade to open while `D-5` is the thing waiting.
- **`window_plugin` has one caller and four hand-written rivals**, up from three in August. Part
  of `Q-9`; not worth separating from it.

## Five of the six were deleted while this was being written, and the measurement stands

**Every number here was taken at `e85c2980`.** While the report was being written the shared tree
acquired 56 deletions across five files - `optional_command`, `as_a_turn`, `is_claimable`,
`owned_by` and `recolor`, all removed, nothing added. `render_asset_usages` is still at
`planet-flat/src/gpu.rs:180`, which is the one outstanding since 2026-08-28.

**This is `scripts/gate.sh`'s own warning happening**: several instances work in this checkout, so a
measurement includes whatever else is uncommitted. **The finding is unaffected because it was made
against the commit**, and the deletions are evidence for it rather than against it - but a reader
re-running the sweep today will find five of the six already gone, and would otherwise conclude the
sweep was wrong.

**Nothing open is stranded by the deletions, checked rather than assumed.** Two items in
`crates/outbox.md` name `is_claimable` - `C-123`, withdrawn, and `C-24`, acted 2026-09-05. No open
item in any outbox names any of the five.

**This lens did not make those edits and may not.** It does not know whether they answer `Q-105` or
were found independently, and does not claim either.

## What this review did not do


**It did not run the gate**, because `hooks/pre-push` runs `cargo fmt` and this lens may not
modify the files it is judging. Clippy and the whole workspace build were run with
`CARGO_TARGET_DIR=target-quality`, a lane-private directory, so no other instance's artifacts were
evicted.

**It did not look at UI state or at player interaction as separate concerns.** That is the brief
Sean set on 2026-09-24 and it is still only half answered: no crate names UI state, and this
review found nothing that changes that.
