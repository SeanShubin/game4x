# Crate boundaries, duplication, and where Bevy has spread

**Derived.** Written by the quality instance on 2026-08-28. Not binding - an observation about the
code, not a decision about it.

[Quality](README.md) · [Architecture](../../docs/architecture.md) · [Specification](../../spec/README.md)

Read at commit `835cb33`, *A slash directs the front end, and `/new` abandons the fold*. Reading
began while that work was still uncommitted and every line number was re-checked against the commit
before this was filed.

The question asked was: is Bevy confined to where it is needed, and is engine-independent code
staying out of it. The short answer is that **the crate graph holds and the crate contents do
not**. `cargo tree` shows no Bevy below the adapter, exactly as
[architecture rule 7](../../docs/architecture.md#rules) requires. But `planet-bevy` has accumulated
policy that has nothing to do with an engine, and has grown an edge to `game-front` that makes it
a second front end.

Along the way, four contradictions with `spec/` and `releases/` turned up. Those are first,
because no other lane is looking for them.

---

## Findings

| #         | What                                                        | Whether                   |
| --------- | ----------------------------------------------------------- | ------------------------- |
| [1](#1)   | Resetting the view is unreachable on a touch device         | **Fix now**               |
| [2](#2)   | The size keys are a binding no document names               | **Fix now**               |
| [3](#3)   | `index.html` says the release names no surface key; it does | **Fix now** (one comment) |
| [4](#4)   | `follow_the_game`'s doc describes the route it replaced     | **Fix now** (one comment) |
| [5](#5)   | The palette exists three times, unchecked                   | **Fix now**               |
| [6](#6)   | `planet-bevy` reaches into `game-front`, both ways          | Fix eventually            |
| [7](#7)   | `planet-ecs` is wired into the shipped app and does nothing | **Decide now**, fix after |
| [8](#8)   | Engine-free policy lives in `planet-bevy`, below the gate   | Fix eventually            |
| [9](#9)   | Gathering owners out of the ECS, three copies, one dead     | Fix eventually            |
| [10](#10) | Two independent computations of territory adjacency         | Fix eventually            |
| [11](#11) | `planet-bevy` is two unrelated adapters in one crate        | Fix eventually            |
| [12](#12) | Two identities for one territory, opposite conventions      | Noted and not             |
| [13](#13) | `pre-push` and the CI gate disagree about clippy            | Fix eventually (cheap)    |
| [14](#14) | Small duplication and dead code                             | Noted and not             |

Two places where the **specification itself** is the thing to settle are in
[Questions for the documentation lane](#questions-for-the-documentation-lane). Per
[the brief](README.md#what-is-not), this report says so and stops.

---

## Status, as of 2026-08-28 evening

Both lanes have responded. Recorded here so that nothing already acted on is reported again.

| Finding                                           | Outcome                               |
| ------------------------------------------------- | ------------------------------------- |
| [1](#1) reset unreachable on touch                | **Closed** - `464ff45`, `a1cc5e0`     |
| [2](#2) size keys named nowhere                   | **Closed** - `a1cc5e0`                |
| [3](#3) `index.html` misquotes the release        | **Closed** - `464ff45`                |
| [4](#4) `follow_the_game`'s stale doc             | **Closed** - `464ff45`                |
| [5](#5) palette in three places                   | Open                                  |
| [6](#6) `planet-bevy` reaches into `game-front`   | Open, and one call site deeper        |
| [7](#7) `planet-ecs` dead in the shipped app      | Open - the decision has not been made |
| [8](#8) engine-free policy below the gate         | Open                                  |
| [9](#9) three gathers, one dead                   | Open                                  |
| [10](#10) two adjacency computations              | Open                                  |
| [11](#11) `planet-bevy` is two adapters           | Open                                  |
| [12](#12) two territory identities                | Open - correctly, it waits on [7](#7) |
| [13](#13) `pre-push` versus the CI gate           | **Closed** - `464ff45`                |
| [14](#14) small duplication and dead code         | Open - all six, correctly deferred    |
| [Q1](#q1) what *"where there is a pointer"* binds | **Answered** - `a1cc5e0` removed it   |
| [Q2](#q2) *"`/new` changes no game state"*        | **Answered** - `a1cc5e0` reworded it  |

What the responses left behind is in
[the follow-up report](2026-08-28-response-to-the-first-report.md), not here.

---

<a id="1"></a>

## 1. Resetting the view is unreachable on a touch device

> **Closed** by `464ff45`: the page carries a reset control, which asks through a counter the engine
> watches. `a1cc5e0` added *and to a control* to the release. Verified - the two new tests in
> `game-front` pass.

**Where.** `crates/planet-bevy/src/globe.rs:683` (`reset_view`), against
`spec/interface.md` → Availability and presentation, `spec/planet.md` → Presentation, and
`releases/first-release.md:104`.

**What.** Reset is bound to the `R` key and to nothing else. There is no button on the page
(`crates/game4x/index.html` has `#surfaces` and `#sizes` and no third group), no gesture, and no
typed form - `/` directs the front end to a surface or to `/new <size>`, and neither resets the
view. A tablet has no `R`.

**Why.** Three documents, and the code contradicts all three:

- `spec/planet.md` → Presentation: *the user can reset the view to a default*.
- `spec/interface.md`: *Actions that are not a manipulation of the planet - resetting the view,
  reaching a surface, choosing a planet size - **never require a gesture or a key the platform may
  lack.*** A touch device lacks every key.
- `spec/interface.md`: *Nothing is available in one build and not another.* On a phone, reset is
  not available at all.

The crate already knows this shape of problem and solved it twice. `touch_to_turn`'s own doc at
`globe.rs:636` says a tablet *"reaches none of `drag_to_turn`, `keys_to_turn` or `wheel_to_zoom`,
and without this the planet cannot be turned or zoomed at all there. `spec/interface.md` does not
allow that."* Turning and zooming were given a touch route. Reset was left behind, and it is named
in the same sentence of the same rule.

`releases/first-release.md:104` says *Reset is bound to `R`* and stops there, so the release
schedules a binding the spec does not permit on its own. Per
[CLAUDE.md](../../CLAUDE.md#releases), the spec wins.

**Whether.** **Fix now.** It is a user-visible hole on the platform the touch code was written
for, and the cheapest honest fix is a control on the page beside `#sizes`, which is what
`spec/interface.md` asks for where there is a pointer. Whether the release line also needs
rewording is a documentation-lane question - see [Q1](#q1).

---

<a id="2"></a>

## 2. The size keys are a binding no document names

> **Closed** by `a1cc5e0`: `releases/first-release.md` now reads *bound to `1` through `5`, to a
> control for each size, and to `/new <size>`*. No code change was needed, which is the outcome this
> finding expected.

**Where.** `crates/planet-bevy/src/globe.rs:712` (`SIZE_KEYS`), `:703` (`keys_to_choose_size`),
`:549` (the HUD line advertising them).

**What.** `Digit1` through `Digit5` start a new game on each planet size. `releases/first-release.md`
→ Controls lists rotation, zoom, reset, the three surfaces, and choosing a planet size - and for
that last one says it *"is bound to a control for each size, and to `/new <size>`"*. Digit keys
appear nowhere in `spec/` or `releases/`; I grepped both.

**Why.** This crate states the rule it is breaking, in its own module header at `globe.rs:24`:

> Which device does which is not in `spec/planet.md` any more - it is a binding, and
> `releases/first-release.md` -> Controls holds the ones this release names.

If the release holds the bindings, a binding not in the release is not a binding. The HUD at
`globe.rs:549` then advertises it to the player as though it were, which is how an undocumented
control becomes an expectation.

Worth being clear about what is *not* wrong here: routing the key through
`console.submit("/new tiny")` rather than writing `Planet::size` is right, and the reasoning at
`globe.rs:686-700` is good. The defect is only that the release does not name the key.

**Whether.** **Fix now**, and it is probably a documentation fix rather than a code one - the keys
are useful and the release line is a list that a line could be added to. Either way the two have
to agree, and today they do not. Filing the addition is the documentation lane's - see the closing
paragraph of [Q2](#q2).

---

<a id="3"></a>

## 3. `index.html` asserts the release is silent about surface keys; it is not

> **Closed** by `464ff45`: the comment now quotes the release line it was contradicting. The native
> F-key gap is untouched, correctly - [Q1](#q1) was answered by removing the sentence that made it a
> question.

**Where.** `crates/game4x/index.html:568-570`.

**What.** The comment above the `F1`/`F2`/`F3` handler reads:

> Neither `spec/` nor `releases/` names a binding for reaching one, so these match the buttons and
> the terminal's `/game`, `/console`, `/browser` rather than inventing a fourth way.

`releases/first-release.md:105` says: *The three surfaces are reached by `F1`, `F2` and `F3`, by
buttons on the page, and by `/game`, `/console` and `/browser` typed at the console.*

**Why.** The comment is a claim about the specification, and it is false. It reads as a
justification for a choice that no longer needs one - the release now mandates exactly what the
code does. The next reader who wants to change the F-keys will check the comment, believe nothing
constrains them, and be wrong.

A second point falls out of the same release line: **the native build does not implement `F1`,
`F2` or `F3` at all.** They exist only in `index.html`. `crates/planet-bevy/src/globe.rs` handles
no function keys. The release states the binding without qualification.

**Whether.** **Fix now** for the comment - it is one line and it is currently misinformation. The
native F-key gap is a real gap but its resolution depends on [Q1](#q1), because on the desktop the
console is a terminal and there are no panels to switch between.

---

<a id="4"></a>

## 4. `follow_the_game` documents the route it was just changed away from

> **Closed** by `464ff45`: the paragraph now describes both ways the counter moves, a transition and
> `/new <size>`.

**Where.** `crates/planet-bevy/src/globe.rs:735-739`.

**What.** The doc comment on `follow_the_game` says:

> The size of a planet is game state. It arrives by `create planet <size>`, through the one
> function, like everything else.

Forty lines above it, `chooses` at `globe.rs:722` now emits `/new <size>`, and its own doc explains
at length why it must *not* be `create planet <size>`. `spec/console.md` says `/new` *"is not a
command"*, and `game-front/src/console.rs:220` documents `begin` as *"Not a transition"* - so
neither half of the older sentence still holds.

**Why.** Two comments in one file give opposite accounts of how the size reaches the globe, and
the wrong one sits on the function that actually observes it.

**Whether.** **Fix now.** One paragraph.

---

<a id="5"></a>

## 5. The palette exists three times, and nothing checks the copies agree

**Where.**

| Copy                         | File                                                          |
| ---------------------------- | ------------------------------------------------------------- |
| Source, hex sRGB             | `crates/planet-render/src/palette.rs:13` and `:41`            |
| Hand-transcribed decimals    | `crates/planet-bevy/src/planet.wgsl:40-73`                    |
| The transfer function, again | `crates/planet-bevy/src/globe.rs:928` (`linear_of`, in tests) |

**What.** `planet.wgsl` re-declares all six region colours, all six player colours, `BACKGROUND`,
`BORDER`, `DUPLICATE_STRENGTH` and `OWNER_TINT` as three-decimal WGSL constants, under a comment
saying *"matching planet-render/src/palette.rs"*. Nothing enforces the match. Separately,
`globe.rs:928` copies the sRGB-to-linear conversion from `planet-render/src/mesh.rs:180` - its own
doc says *"Mirrors the conversion in `planet_render::mesh`"* - because `mesh::linear_rgba` is
private.

**Why.** Change `0x8B4A9C` in `palette.rs` and the CPU path changes, the GPU path does not, and
pressing `G` toggles between two worlds that look different. That reads as a rasterizer bug and is
not one. `gpu.rs` already pins the one thing it had to agree with the shader about - the
`colour + 8 * (owner + 1)` packing, tested at `gpu.rs:ownership_packs_above_the_colour` - so the
habit exists; the palette is the table that never got it.

This project is unusually good at exactly this: `command_language::agreement::disagreements`
mechanically checks the grammar and binding tables agree, and `binding::handled()` is compared
against the grammar in a test. The palette deserves the same treatment - or, better, no treatment
at all.

**Whether.** **Fix now**, and prefer deletion over a test. `PlanetUniform` already carries a
512-entry `seeds` array; twelve more `Vec4`s would carry both palettes and the four scalars, and
the WGSL copy would cease to exist. That is strictly better than a test asserting two hand-written
lists match, because it removes the second list. Making `mesh::linear_rgba` public removes the
third.

---

<a id="6"></a>

## 6. `planet-bevy` reaches into `game-front`, in both directions

**Where.** `crates/planet-bevy/Cargo.toml` (`game-front.workspace = true`),
`crates/planet-bevy/src/globe.rs:741` (reads `shell::generation()`, `shell::territory_count()`),
`:703` (**writes** through `shell::with(|console| console.submit(...))`).

**What.** The engine adapter both polls the one `Console` and submits lines to it. It is a front
end, by the definition `game-front`'s own module doc gives.

**Why.** Three things follow.

**It falsifies the composition root's central claim.** `crates/game4x/src/main.rs:23`:
*"It is the only place that knows both that Bevy exists and that the game exists at the same
time."* It is not. `planet-bevy` knows both, and now acts on both.

**`docs/architecture.md` is out of date because of it.** The crate table lists `planet-bevy`'s
dependencies as *"`planet-render`, `planet-ecs`, Bevy"*. The manifest also has `game-front` and
`sphere-tessellation`.

**The geometry prototype links the entire game.** `cargo tree -p planet-view`:

```
planet-view
└── planet-bevy
    ├── game-front
    │   ├── game-console
    │   │   ├── command-language
    │   │   ├── game-model
    │   │   ├── planet-model
    │   │   └── sphere-tessellation
    │   └── game-model
```

A prototype described as *"a sphere fanned out flat"* compiles and links the parser, the rules, the
console and the whole `Session`, none of which it names.

The polling mechanism itself is right, and the reasoning for it - that on the web the change
happens on the page's call stack and there is nothing safe to call back into - is sound. The
problem is not *how* the globe learns; it is *that the plugin fetches its own source* instead of
being handed one.

**Whether.** **Fix eventually**, together with [11](#11). `GlobePlugin::new` could take the two
closures it needs, constructed in `game4x/src/main.rs`:

```rust
GlobePlugin::new(spec)
    .following(game_front::shell::generation, game_front::shell::territory_count)
    .submitting(|line| game_front::shell::with(|console| { console.submit(line); }))
```

That restores `main.rs`'s claim, drops `game-front` out of `planet-bevy`'s manifest, and makes
`GlobePlugin` testable without a `Console`.

---

<a id="7"></a>

## 7. `planet-ecs` is wired into the shipped binary and does nothing there

**Where.** `crates/game4x/src/main.rs:74`, `crates/planet-ecs/src/lib.rs`.

**What.** `game4x` adds `PlanetEcsPlugin`, which spawns one `Region` entity per territory and
installs `advance_turn`. In the shipped app:

- **Nothing ever pushes a `PendingIntent`.** The only writer is `read_input` in
  `planet-bevy/src/lib.rs:249`, which belongs to `PlanetViewPlugin`, which `game4x` does not add.
  So `advance_turn` returns at its first line (`planet-ecs/src/lib.rs:119`) on every frame,
  forever.
- **Nothing ever reads `Region` or `Owner`.** The only reader is `read_ownership`
  (`planet-bevy/src/lib.rs:385`), same plugin, same story.
- **`WorldTopology` is inserted once and never updated.** It is built from the territory count at
  startup (`main.rs:57-69`). After `/new huge` the globe rebuilds through `follow_the_game`, and
  the ECS still holds the twelve-region topology.

**Why.** This is not tidiness. It is an unresolved decision showing through, and
`docs/architecture.md` rule 6 currently states the losing side as fact:

> Every game entity is an ECS entity - there is no second way of holding game state.

There are two ways. The real game state lives in `game_console::Session`, behind a `Mutex` on the
desktop and a `thread_local` on the web (`game-front/src/shell.rs`). The ECS copy holds ownership
of regions that no rule in `game-model` has ever heard of - `planet-model`'s own doc says its one
rule *"is meant to be replaced"*. Both halves are half-built, and the architecture document
describes the half that is not running.

**Whether.** **Decide now, fix after.** Two coherent answers:

- **The ECS is the store.** `Game` moves into components, `PendingIntents` is what the console
  writes, and `planet-model` is absorbed or retired. Rule 6 becomes true.
- **The `Session` is the store.** `PlanetEcsPlugin` comes out of `game4x` and stays as the
  prototype's demonstration. Rule 6 is rewritten to say what is actually true.

Which one is a design decision and therefore Sean's, not this report's. What is not optional is
that `docs/architecture.md` rule 6 and the code currently disagree, and the disagreement is
invisible because the dead half compiles.

---

<a id="8"></a>

## 8. Engine-free policy lives in `planet-bevy`, where the gate cannot test it

**Where.** `crates/planet-bevy/src/globe.rs:170` (`Orbit`), `:229` (`PINCH_FLOOR`), `:233`
(`Gesture`), `:253` (`Fingers`), `:491` (`readable_on`), `:538` (`summary`).

**What.** None of these needs an engine. `Orbit` is three floats with a pitch clamp and a
proportional zoom clamp. `Fingers` is gesture recognition over `Vec2`. `readable_on` is a
luminance threshold picking an ink. `summary` formats model facts into a string, which is the same
job `planet_render::app` does for the prototype HUD at `app.rs:261`. Every one is a *policy* about
how the world may be turned or drawn - decisions the project puts below the engine line
everywhere else.

**Why.** Two costs, one of them measurable.

**The tests are in the wrong tier.** All 18 tests in `globe.rs` are pure - not one constructs an
`App`. Same for all 7 in `gpu.rs`. But `.github/workflows/pipeline.yml`'s pre-deploy gate runs:

```
cargo test --release -p command-language -p game-model -p game-console -p game-front \
  -p sphere-tessellation -p graph-coloring -p planet-model -p planet-render
```

`planet-bevy` is absent, because compiling it needs Bevy's Linux system libraries. So **25 tests
of engine-free logic run only in the notify-only `full-tests` job, after deploy** - including
`turning_the_world_never_moves_the_poles_sideways`, whose doc says it is *"the regression test for
a bug that shipped once"*. A regression test that runs after deploy cannot stop a regression from
being deployed.

**There are now two camera policies at two layers.** `planet_render::camera::GlobeView`
(`camera.rs:403`, `:414`) drags and clamps zoom in f64 about view axes; `Orbit` (`globe.rs:186`)
drags and clamps zoom in f32 about yaw and pitch. They are legitimately *different* - `GlobeView`
accumulates roll and `Orbit` must not, because `spec/planet.md` fixes the roll - but both are
policies, and one of them is on the wrong side of the line.

**Whether.** **Fix eventually.** Move `Orbit`, `Fingers`, `Gesture`, `readable_on` and `summary`
down - into `planet-render` beside `camera`, or into a small `planet-input` crate if a direct
`glam` dependency is unwelcome (`planet-render` already hand-rolls `Mat3`/`Vec3` in f64, so
hand-rolling two f32s is also open). `globe.rs` drops from 1,141 lines to roughly 400 - meshes,
materials, labels, and the systems that translate `CursorMoved`/`TouchInput`/`MouseWheel` into
method calls. The gate gains 25 tests for the cost of one `-p` flag.

---

<a id="9"></a>

## 9. Gathering owners out of the ECS, three copies, one of them dead

**Where.**

| Copy                                          | Where                               |
| --------------------------------------------- | ----------------------------------- |
| `pub fn gather`                               | `crates/planet-ecs/src/lib.rs:101`  |
| the `// GATHER` block, which does not call it | `crates/planet-ecs/src/lib.rs:123`  |
| `read_ownership`                              | `crates/planet-bevy/src/lib.rs:385` |

**What.** All three allocate `vec![None; region_count]` and fill it by `region.0.index()`.
**`gather` is called by nothing in the workspace** - the only other hits for the name are
`planet_model::World::gather`, an unrelated private method. It is public API that was inlined into
`advance_turn` and never removed.

**Why.** Three copies of the load-bearing rule that gathering must be keyed by identity rather
than accumulated in iteration order. `planet-ecs` documents that rule at length in its module
header, and `spawn_order_does_not_change_the_outcome` proves it - for the copy inside
`advance_turn`. The other two are unproven, and one of them is in a different crate.

**Whether.** **Fix eventually.** Delete `gather`, or make `advance_turn` call it and give
`read_ownership` a way to reach it. Deleting is smaller; calling it is better, because then the
confluence test covers the code that runs.

---

<a id="10"></a>

## 10. Two independent computations of which territories touch

**Where.** `crates/game-console/src/binding.rs:213` (`adjacency_for`) and
`crates/planet-render/src/lib.rs:35` (`topology_of`).

**What.** `adjacency_for` goes `icosahedral::canonical_seeds` → `sphere_tessellation::adjacency`,
and feeds `game-model`. `topology_of` goes `World::build` → `Tessellation::generate_balanced` →
adjacency, and feeds `planet-ecs` from `game4x/src/main.rs:69`. Two paths, two graphs, one planet.

**Why.** **They agree only because `Params::default()` sets `jitter: 0.0`**, which makes
`generate_balanced` short-circuit to `canonical_seeds` at `sphere-tessellation/src/lib.rs:185`.
The doc comment on `Params::default` describes turning jitter on as a future step and even records
the tuned values (*"jitter 0.20 with 16 relaxation passes"*). The day that happens, the graph the
rules use and the graph the planet is drawn from diverge - a move refused as non-adjacent between
two territories that visibly share an edge. Nothing asserts the two are the same graph.

The same latency covers numbering: `globe.rs:456` labels a panel with
`planet_model::RegionId(region).number()`, from the mesh's region order, while `show territory N`
names a `game_model::TerritoryId` from `canonical_seeds` order. Same reason, same day.

**Whether.** **Fix eventually.** One source, or - cheaper and nearly as good - a test in
`game-console` asserting that for every `PlanetSize`, `adjacency_for(size)` and the topology
`game4x` builds are the same graph. That test would fail loudly on the day jitter is switched on,
which is the day it needs to.

---

<a id="11"></a>

## 11. `planet-bevy` is two unrelated adapters sharing a crate

**Where.** `crates/planet-bevy/src/lib.rs` and `crates/planet-bevy/src/gpu.rs` versus
`crates/planet-bevy/src/globe.rs`.

**What.** `PlanetViewPlugin` and the whole CPU/GPU flat-projection path are used **only** by
`prototypes/planet-view`. `GlobePlugin` is used **only** by `game4x`. Neither binary uses the
other's half.

**Why.** Each binary pays for the other's dependencies. The prototype gets the game (see
[6](#6)); `game4x` gets `planet-render`'s CPU rasterizer, camera, font and app - `raster.rs` 622 +
`camera.rs` 746 + `app.rs` 572 + `font.rs` 261, about 2,200 lines reachable only through code it
never calls. `planet-render` is really two crates already: *world building and mesh*, which the
game uses, and *software rasterizer, camera and app*, which only the prototype does.

**Whether.** **Fix eventually**, and do it in the same pass as [6](#6) - the two share a cause.
Splitting into `planet-bevy-view` (prototype) and `planet-bevy-globe` (game) makes each binary's
`cargo tree` describe what it actually contains, which is what makes the tree an audit rather than
a diagram.

---

<a id="12"></a>

## 12. Two identities for one territory, with opposite conventions

**Where.** `crates/game-model/src/identity.rs:11` (`TerritoryId`) and
`crates/planet-model/src/lib.rs:44` (`RegionId`).

**What.** `TerritoryId` counts from **1**; `index()` subtracts one. `RegionId` counts from **0**;
`number()` adds one. They name the same territory of the same planet.

**Why.** Each is internally right and well documented - `RegionId`'s doc explains exactly why the
two numbers must each say which they are. The cost is at the seam, where a `usize` from one world
is used as an index into the other: `globe.rs:456` builds a `RegionId` from a mesh position and
prints its `number()`, which the player then types as a `TerritoryId`. They line up by coincidence
of seed ordering, which is [10](#10) again.

**Whether.** **Noted and deliberately not.** Merging them is only sensible once [7](#7) is
decided, and doing it before then would be guessing at the answer. Recorded so that whoever
decides [7](#7) knows it is part of the same knot.

---

<a id="13"></a>

## 13. `pre-push` and the CI gate disagree about which crates clippy sees

> **Closed** by `464ff45`: the two clippy lists are now character-for-character the same eleven
> crates. The *test* lists still differ, which is [8](#8) and remains open.

**Where.** `hooks/pre-push:19-21` versus `.github/workflows/pipeline.yml` → *Clippy (the shipped
path)*.

**What.** The hook's header says it *"mirrors the gate job in `.github/workflows/pipeline.yml`
minus the WASM build"*. Its clippy invocation omits `command-language`, `game-model`,
`game-console` and `game-front`, all four of which the CI gate lints.

**Why.** A clippy warning in any of the four game crates passes the local hook and fails CI - which
is precisely the round trip the hook exists to prevent, and the four crates in question are where
most of the game is being written right now.

**Whether.** **Fix eventually**, though it is a two-line change and the argument for doing it now
is that it costs nothing. Either add the four to the hook, or amend the header to say which crates
it deliberately skips and why.

---

<a id="14"></a>

## 14. Small duplication and dead code

All **noted and deliberately not**, unless one of them is already being touched for another
reason. They are recorded so that a later report does not present them as new.

| Where                                                                           | What                                                                                                                                                                                                                                                                                                                                     |
| ------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/planet-bevy/src/gpu.rs:146`                                             | `render_asset_usages()` is called by nothing. `blank_image` at `lib.rs:230` writes the same two flags inline.                                                                                                                                                                                                                            |
| `crates/planet-bevy/src/lib.rs:38` and `crates/game4x/src/main.rs:88`           | `window_plugin` says it exists so the composition root can assemble the window in one place; `game4x` ignores it and writes the same title / resolution / `AutoVsync` shape itself. Only the prototype uses the helper.                                                                                                                  |
| `crates/game-front/src/library.rs:12`, `:31`, `:59`                             | The file list `["setup", "world", "nodes", "forces", "play"]` appears three times, and grew from four to five entries this week in all three places. A `const NAMES` would make it one.                                                                                                                                                  |
| `crates/planet-bevy/src/lib.rs:74` and `crates/planet-render/src/camera.rs:176` | `Renderer::other()/name()` and `Projection::other()/name()` are the same two-variant toggle written twice. Genuinely too small to abstract; noted only so it is not re-found.                                                                                                                                                            |
| `crates/planet-render/src/world.rs:47`                                          | `World::build` eagerly computes `verification` and `quality` on every build. In `game4x` neither is ever read - only `planet_render::app`'s prototype HUD uses them (`app.rs:261`, `:312`). Every `/new <size>` pays for a `Quality::measure` and a `verify_truncated_icosahedron` it discards. Cheap at 92 regions; wrong in principle. |
| `crates/game-console/src/binding.rs:215`                                        | `.expect("every planet size is a Goldberg count; planet-render asserts it")` - `game-console` does not depend on `planet-render`, so the test backing this could be deleted without breaking anything `game-console` compiles against. It depends on `planet-model` and `sphere-tessellation`, so it can assert this itself.             |

---

## Questions for the documentation lane

Two findings turned on wording in `spec/` that this report cannot settle. Per
[the brief](README.md#what-is-not), they are stated and left.

<a id="q1"></a>

### Q1. `spec/interface.md`: what does *"where there is a pointer they are controls"* bind?

> Actions that are not a manipulation of the planet - resetting the view, reaching a surface,
> choosing a planet size - never require a gesture or a key the platform may lack. Where there is
> a pointer they are controls; where there is not they are typed.

The native desktop build has a pointer, and none of the three actions is a control there: reset is
`R`, sizes are digits or `/new`, surfaces are typed at the terminal. Two readings:

- **Per platform.** The desktop has a pointer, so it owes controls, and the native build is
  incomplete on all three counts.
- **Per surface.** The desktop's console *is* a terminal, which has no controls at all, so typing
  is the correct form there and only the web owes buttons.

Finding [1](#1) holds either way - a touch device has no `R` under any reading. Findings [3](#3)
and the native F-key gap depend on which reading is meant.

<a id="q2"></a>

### Q2. `spec/console.md`: *"`/new <size>` ... changes no game state"*

> A line beginning with `/` directs the front end rather than the game. `/game`, `/console` and
> `/browser` choose a surface; `/new <size>` abandons the current game and starts one on a planet
> of that size. It is not a command: it changes no game state, history does not record it, and
> help does not list it.

The sentence appears to contradict itself: a line that *abandons the current game and starts one
on a planet of that size* changes every observable of game state. `game-front/src/console.rs:236`
replaces `self.session` outright.

The implementation's defence, at `console.rs:220`, is careful and I think correct as far as it
goes: `/new` *"produces no new state from an old one - it begins a second fold"*, so
`spec/invariants.md`'s *the game is one function* is untouched. But that is the claim **"it is not
a transition"**, which is a different and much narrower claim than **"it changes no game state"**.
Under the plain reading of the spec's own words, the implementation violates the sentence that
introduces the feature.

Related, and worth deciding at the same time: `spec/invariants.md` says *a game state is exactly
the result of applying every transition in order to the starting state*. After `/new`, the
starting state is a different one, and the console's transcript spans both folds while `history`
spans only the second. That is defensible, and it is not written down anywhere.

Also for the same lane: `releases/first-release.md:104`, *Reset is bound to `R`*, schedules a
binding `spec/interface.md` does not permit on its own - see [1](#1). And
`releases/first-release.md:107-108` omits the digit keys the code ships - see [2](#2).

---

## What is working, and should not be disturbed

Worth saying explicitly, because several of these are unusual enough that a later pass might
"simplify" them.

- **`cargo tree` is a real audit.** No Bevy below the adapter. Rule 7 is enforced by the compiler,
  not by discipline, and it is the reason the leaks above are leaks *inside* crates rather than
  across them.
- **`command-language` knows no game nouns**, and `agreement::disagreements` mechanically checks
  the grammar and the binding table agree. That is the right answer to [5](#5), already built once
  in this repository.
- **`no_floating_point_anywhere`** in both model crates, in two different implementations, both of
  which correctly exclude comments and test fixtures.
- **`spawn_order_does_not_change_the_outcome`** varies archetype layout to test confluence rather
  than asserting it in prose. Much stronger than a comment.
- **Almost no production `unwrap`/`expect`.** Of 85 occurrences across `game-model` and
  `game-console`, exactly one is outside a test module.
- **`direction_to_screen` taking `Direction` rather than `Vec3`** (`camera.rs:299`), which makes a
  bug that shipped once unrepresentable, with the reasoning recorded next to it.
- **Routing the size keys through `console.submit`** (`globe.rs:703`). Finding [2](#2) is about
  the key not being documented, not about how it works - how it works is right, and the
  alternative it rejects is the one that caused the earlier bug.
