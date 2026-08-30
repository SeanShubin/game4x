# What the new prototype exposed, and what it did not

**Derived.** Written by the quality instance on 2026-08-29. Not binding - an observation about the
code, not a decision about it.

[Quality](README.md) · [Architecture](../docs/architecture.md) · [Specification](../spec/README.md)

Read at `9357258`, with the code lane's work on `goldberg-view` uncommitted in the tree
(`Cargo.toml`, `Cargo.lock`, `crates/planet-bevy/src/globe.rs`, `scripts/`, and the untracked
`prototypes/goldberg-view/`). Line numbers are against the working tree.

Forty-nine commits since [the first report](2026-08-28-crate-boundaries-and-duplication.md). This
pass looks for one thing: **a layer that has learned a fact about the layer above it**, and the
sideways reaches that go with it. That is the shape the globe had when it was locked to the game's
five sizes, and the shape a prototype puts pressure on precisely because it has no game behind it.

The pressure worked. It also stopped one layer short.

---

## Findings

| #       | What                                                                   | Whether                                       |
| ------- | ---------------------------------------------------------------------- | --------------------------------------------- |
| [1](#1) | The picture never sees the biome the model has                         | **Withdrawn** - see the correction            |
| [2](#2) | `Biome` lives in the game, so terrain and rendering depend on the game | **Decide now**                                |
| [3](#3) | `detached` is a runtime flag over a compile-time coupling              | Fix eventually                                |
| [4](#4) | The detached globe's readout advertises keys it does not have          | **Closed** - `b43d9b4`                        |
| [5](#5) | `planet-terrain` is in neither gate list                               | **Closed** - `b43d9b4`                        |
| [6](#6) | The quotation guard stops at `crates/`                                 | Coverage closed - `b43d9b4`; the form is open |
| [7](#7) | The composition root has grown logic and tests                         | Fix eventually                                |
| [8](#8) | Two hand-rolled option parsers                                         | Noted and not                                 |
| [9](#9) | `docs/architecture.md`'s crate table no longer describes the tree      | Noted - documentation lane                    |

One question for the documentation lane is in [Q3](#q3). Findings from the first report that are
still open are summarised in [Carried forward](#carried-forward), with what moved and what did not.

---

<a id="1"></a>

## 1. The picture never sees the biome the model has

> **Corrected 2026-08-29, after the code lane's reply. The remedy below is withdrawn and the
> severity is wrong.** The code lane rejected this finding's fix, and checking it, they are right on
> every count. What survives is smaller than what was filed, and is set out in
> [Correction](#correction-1) at the end of this section. The original text is kept rather than
> rewritten, so the mistake is visible.

**Where.** `crates/planet-render/src/realistic.rs:245` (`build`), `:117` (the claim), `:159`
(`biome_of` on the raw field); `crates/planet-terrain/src/lib.rs:354` (`join_the_land`);
`crates/planet-bevy/src/globe.rs:453` (the call).

**What.** `realistic.rs:117` says, in a doc comment:

> Biome first, because that is the fact the model holds and `spec/planet.md` says the drawing must
> show **the biome the model has**.

The signature makes that impossible. `pub fn build(solid: &Solid, seed: u64) -> PlanetMesh` takes a
solid and a seed. No biomes are passed in, and `globe.rs:453` calls it as
`realistic::build(&solid, planet_terrain::WORLD_SEED)`. The colour of every point comes from
`tone_of`, which calls `planet_terrain::biome_of(sample)` - the **raw field**, re-derived per point.

The model's biomes are not the raw field. `binding.rs` builds them with
`planet_terrain::biomes_of`, which runs `join_the_land` (`planet-terrain/src/lib.rs:354`). That
function exists to satisfy `spec/planet.md`'s *oceans never isolate land from land*, and it does so
by **overwriting** ocean territories with `covering_land` until the land is one piece. The code
knows this and tests for it - `a_territory_drained_to_join_the_land_becomes_its_own_ground`
(`:795`) asserts exactly the case where `covering(&ground) == Biome::Ocean` and the stored biome is
not ocean. `binding.rs` says it plainly too: *"a territory whose ground is under water may still
have to be land."*

**Why.** A drained territory is grassland in the model and painted as open water. `spec/planet.md`
says *no territory can be claimed whose biome is ocean*, so a player looks at a blue territory,
claims it, and it works. The reverse never happens, because `join_the_land` only converts ocean to
land - which makes the failure quiet and one-directional rather than obviously broken.

Two spec lines are contradicted at once:

- *A drawing never betrays how it was made. A viewer sees the planet, never the process.* The
  discrepancy **is** the process showing: the only territories that look wrong are the ones the
  connectivity pass touched.
- *A territory's biome is what the terrain gives it.* For a drained territory it is not.

This is worth stating carefully, because the design is right and only the wiring is wrong.
`planet-terrain`'s module doc is emphatic that one field feeding both readers is the whole point -
*"They cannot disagree, because one is derived from the other rather than authored beside it"* - and
that is true of `sample`. It stopped being true when a resolution step was added on the model's side
only. The field is still one field; there are now two answers derived from it, and the picture
reads the earlier one.

**Whether.** ~~**Fix now.**~~ Withdrawn - see below.

<a id="correction-1"></a>

### Correction

The code lane declined this finding and gave three reasons. All three hold. I checked each rather
than taking them, and the checks are reproducible.

**The remedy was wrong.** I proposed that `build` take the resolved biomes and that `tone_of` use
the territory's biome for the choice of colour. That makes the choice uniform per territory, so the
transition between two biomes falls exactly on a territory edge - which is a drawn boundary, and is
what `spec/planet.md`'s *nothing in the terrain reveals how the sphere was divided* forbids. It
would also fail an existing test, `two_regions_meeting_at_a_point_agree_about_it`
(`crates/planet-render/src/realistic.rs:369`), which asserts that two vertices at the same position
in different regions carry *exactly* the same colour and normal. Under my remedy they would carry
their own territory's. That test is a better statement of the rule than my finding was, and I did
not read it before proposing something that breaks it.

**The measurement is correct.** They report draining firing zero times at 12 territories and once
each at 32 and 92. Verified independently, by replicating `planet-terrain`'s private `ground_of`
and `covering` outside the repository and comparing the natural biome with the resolved one:

| Territories | Natural oceans | Drained | Which                      |
| ----------- | -------------- | ------- | -------------------------- |
| 12          | 8              | **0**   | -                          |
| 32          | 22             | 1       | territory 4, to desert     |
| 42          | 19             | 0       | -                          |
| 72          | 36             | 0       | -                          |
| 92          | 48             | 1       | territory 10, to grassland |

**So there is no defect in what ships.** The first release is twelve territories, and at twelve the
natural ocean set already leaves land connected. The divergence is reachable - `/new small` and
`/new huge` each carry one drained territory - but it is one territory on two of five sizes, not a
live fault. "Fix now" was wrong on frequency as well as on remedy.

**And the requirement I leaned on does not exist.** `realistic.rs:117` says *"`spec/planet.md` says
the drawing must show **the biome the model has**"*. It does not. Lines 62-74 of `spec/planet.md`
are the drawing rules and none of them says that; the nearest is *the realistic drawing shows the
world: terrain*. The comment invents a requirement and attributes it to the specification, and my
finding took it at face value instead of opening the file - which is the exact failure the
misattribution causes in a reader, committed by the report complaining about misattribution.

That is the finding worth keeping here, and it is
[finding 3 of the first report](2026-08-28-crate-boundaries-and-duplication.md#3) for a third time.
Note how it escaped the new guard: `quotations.rs` checks the form `` `spec/x.md`: *italic* ``, and
this is `` `spec/x.md` says ... **bold** `` - a near miss, in a file written after the guard landed.
That widens [finding 6](#6): the guard's convention has an unchecked neighbour in active use, so
covering `prototypes/` is necessary and not sufficient.

### What remains, and the data for it

The real problem is the three-way tension the code lane names, and it is better framed than mine
was. It lives in the model and exists whether or not anything is drawn:

- *Oceans never isolate land from land* forces draining.
- *A territory's biome is what the terrain gives it* is then false for a drained territory.
- *Nothing in the terrain reveals how the sphere was divided* forbids painting the drained answer.

No implementation satisfies all three for a drained territory. That is [Q3](#q3), and it is the
documentation lane's to settle.

Their alternative - **retry rather than repair**: search for a world whose natural ocean set already
leaves land in one piece - keeps all three, because nothing is overwritten and the two readers read
the same field. Their reason for not starting it is that "which world" becomes a value both readers
share rather than one constant, at the end of a session. That is a fair call, and worth noting that
it is the same plumbing as [findings 2 and 3](#2) - except that it moves one `u64` rather than a
`&[Biome]`, and it cannot break the per-vertex continuity test, because the picture still derives
everything from the field.

The one number that decision needs is what the search costs. Measured the same way, over 120
consecutive seeds from `WORLD_SEED`:

| Territories | Worlds needing no draining | First clean seed | Worst natural split |
| ----------- | -------------------------- | ---------------- | ------------------- |
| 12          | 54/120 (45%)               | `WORLD_SEED`     | 2 pieces            |
| 32          | 77/120 (64%)               | `WORLD_SEED + 1` | 5 pieces            |
| 42          | 91/120 (76%)               | `WORLD_SEED`     | 4 pieces            |
| 72          | 78/120 (65%)               | `WORLD_SEED`     | 4 pieces            |
| 92          | 85/120 (71%)               | `WORLD_SEED + 1` | 5 pieces            |

So retry costs between one and three attempts on average, and at every size a clean world is within
one step of the constant already in use. It also stays a save file: the search is deterministic from
a base seed, so `create planet <size>` still rebuilds the same world on another machine, which is
what `binding.rs` says the biomes not being written into the history depends on.

One caveat for whoever takes it: rejection-sampling the seed on a property of the tessellation does
condition the terrain on the arrangement. The conditioning is global - land is in one piece - rather
than local, so nothing about a *particular* cell is readable from it, and it is far weaker than a
drawn boundary. It belongs in whatever note records the decision, not as an objection to it.

**How the measurements were produced**, so they can be re-run rather than trusted: a throwaway crate
outside the repository, depending on `planet-terrain`, `sphere-tessellation` and `game-model` by
path, replicating the private `ground_of`, `covering` and `pieces_of_land` (`GROUND_SAMPLES = 7`,
ties broken by `Biome::ALL` order) and calling the public `biomes_of` beside them. Nothing in the
repository was written to.

---

<a id="2"></a>

## 2. `Biome` lives in the game, so terrain and rendering depend on the game

**Where.** `crates/game-model/src/identity.rs:91` (the definition);
`crates/planet-terrain/Cargo.toml` and `crates/planet-render/Cargo.toml` (`game-model.workspace =
true`); `crates/planet-terrain/src/lib.rs:51` and `crates/planet-render/src/realistic.rs:157` (the
two `use` lines that are the entire reason for those dependencies).

**What.** `Biome` is a six-variant enum naming terrain: ocean, ice, desert, grassland, jungle,
mountain. It is defined in `game-model`, the crate `docs/architecture.md` describes as *the rules of
the game*. Two crates below the game depend on the whole of it to name that enum, and each uses
nothing else from it - I checked; `grep -rn game_model crates/planet-render/src/` returns exactly
one line.

**Why.** This is the same shape as the globe being locked to the game's five sizes, one layer down
and with the dependency made real by Cargo rather than only by a field. `docs/architecture.md` rule
1 says what to do about it:

> **Dependencies point one way.** If two modules need each other, the shared part belongs in a third
> module beneath both.

The repository has already answered this exact question once, and answered it well.
`planet-model/src/size.rs` explains why the five territory counts live in `planet-model` rather than
beside the geometry that produces them: *"putting them here keeps them on the integer side of the
boundary."* `PlanetSize` is a fact the game and the geometry share, and it was put beneath both.
`Biome` is the same kind of fact and went into the game instead.

The cost is not theoretical. It is the reason
`cargo tree -p goldberg-view` - a prototype that draws polyhedra - contains `game-model`,
`game-console`, `command-language` and `game-front`. See [3](#3).

**Whether.** **Decide now**, because [3](#3) cannot be finished without it and both are cheap
afterwards. Two coherent homes:

- **`planet-model`.** Where `PlanetSize` already is, for the stated reason. Costs `game-model` a
  dependency on `planet-model`, which it does not have today - `game-model` depends on nothing, and
  that is a property somebody may want to keep.
- **`planet-terrain`.** Where it is produced. `game-model` would depend on `planet-terrain`, which
  drags `sphere-tessellation` in with it - almost certainly the wrong trade.

Which one is a design decision and therefore not this report's. What is not a design decision is
that two crates beneath the game currently depend on it for one enum.

---

<a id="3"></a>

## 3. `detached` is a runtime flag over a compile-time coupling

**Where.** `crates/planet-bevy/src/globe.rs:130` (`GlobePlugin::detached`), `:167` (the three
systems it omits); `crates/planet-bevy/Cargo.toml` (`game-front.workspace = true`);
`prototypes/goldberg-view/src/main.rs:20` and `prototypes/goldberg-view/README.md`.

**What.** `GlobePlugin::detached` is a real improvement and I want to be clear about that first:
`Planet` now holds a region count instead of a `PlanetSize`, and the three systems that touch
`game-front` are not installed when the globe is detached. The reasoning written on it is exactly
right, including the sentence about why a prototype is the test of whether a boundary is real.

What it does not do is change the crate graph. `planet-bevy` still depends on `game-front`, so:

```
cargo tree -p goldberg-view
└── planet-bevy
    ├── game-front
    │   ├── game-console
    │   │   ├── command-language
    │   │   ├── game-model
    │   │   ├── planet-model
    │   │   ├── planet-terrain
    │   │   └── sphere-tessellation
    │   └── game-model
    ├── planet-ecs, planet-model, planet-render, planet-terrain, sphere-tessellation
```

The prototype's own README and module doc say otherwise:

> It borrows nothing that plays: there is no game here, no console, no biome and no terrain, which
> is why it asks for [`GlobePlugin::detached`].

There is a game here, a console, a biome and terrain - all four, linked into the binary. The claim
is true of the *systems that run* and false of the *code that ships*, and the document does not
distinguish them.

**Why.** `docs/architecture.md` names the audit: *"Algorithm crates do not depend on Bevy. That is
what makes rule 6 enforced by the compiler rather than by discipline, and `cargo tree` is the
audit."* The same audit, run the other way, currently fails. A runtime flag is discipline; it is the
thing the crate graph exists to replace.

**Whether.** **Fix eventually**, and it is the same work as
[finding 11 in the first report](2026-08-28-crate-boundaries-and-duplication.md#11) - splitting
`planet-bevy` so the half that knows a game exists is a separate crate from the half that draws a
sphere. `detached` has already done the hard part by identifying exactly which three systems are on
the game side; what remains is moving them and the `game-front` dependency with them. With [2](#2)
settled as well, `goldberg-view` would link `sphere-tessellation`, `graph-coloring`,
`planet-model`, `planet-terrain`, `planet-render` and the drawing half of `planet-bevy`, and its
README would be true as written.

Until then, the sentence in the README should say what is actually true: nothing that plays *runs*
here, and the crate graph has not caught up.

---

<a id="4"></a>

## 4. The detached globe's readout advertises keys it does not have

> **Closed** by `b43d9b4`: the readout builds its list of bindings from the same flag that installs them, and the planet's name is gone from `summary`. `PlanetSize` still enters `globe.rs` at `:839`, for `keys_to_choose_size` - which is one of the three game-side systems, so it leaves with them under [3](#3) rather than here.

**Where.** `crates/planet-bevy/src/globe.rs:635` (`summary`), reached from `build_globe:565`
regardless of `follows_the_game`.

**What.** The heads-up display ends with:

```
1-5 start a new game on a planet of that size - T for the {} drawing
```

In a detached globe neither key is installed - `keys_to_choose_size` and `keys_to_change_drawing`
are inside the `if self.follows_the_game` block at `:167`. So `goldberg-view` tells the user to
press five keys that start no game and a `T` that changes no drawing.

The same function still reaches for the game's vocabulary:

```rust
PlanetSize::with_territory_count(regions).map_or("planet", PlanetSize::name),
```

which is why `globe.rs:39` still has `use planet_model::PlanetSize;`. The comment above it is
honest about what it is doing - *"a solid the game has no word for, which a prototype is entitled to
draw"* - but the fallback string is `"planet"`, so `goldberg-view` showing `GP(2,1)` at 132 faces
labels it *planet*, which is the one word this prototype exists to question.

**Why.** Small, and exactly the class this pass is looking for: the last place in the drawing layer
that still knows what the game calls things. It is also immediately visible - it is the first line
of the first screen of the newest prototype.

**Whether.** **Fix now**, and it is small. The readout's last line is a list of bindings, and the
plugin already knows which bindings it installed. Building that line from the same flag that decides
the systems would make the two impossible to disagree, and would take the `PlanetSize` import out of
`planet-bevy` on the way - finishing what `Planet::regions` started.

---

<a id="5"></a>

## 5. `planet-terrain` is in neither of the gate's lists

> **Closed** by `b43d9b4`: `planet-terrain` is now in the clippy list and the test list of both `pipeline.yml` and `hooks/pre-push` - four additions, verified.

**Where.** `.github/workflows/pipeline.yml:74` (clippy) and `:82` (tests);
`hooks/pre-push:20` and `:25`.

**What.** `planet-terrain` is a new crate with 13 tests. It appears in the gate's clippy list: no.
In the gate's test list: no. In `pre-push`: neither. It runs only in `full-tests`, which is
`--workspace` and runs *after* deploy.

**Why.** This is the crate that decides what the world looks like and which territories can be
claimed. It is pure, engine-free, and needs no system libraries - the exact profile the gate's list
was drawn up around, and the reason `planet-bevy` is legitimately excluded does not apply to it. Its
tests include `land_is_never_cut_in_two_by_water`, which is the assertion behind a `spec/planet.md`
rule; that assertion currently cannot fail the build until the build is already published.

The two lists in `pipeline.yml` were correct for the tree that existed when they were written. They
are enumerations, so a new crate joins them only if somebody remembers.

**Whether.** **Fix now.** Two words in two files. Worth noting the durable version for later: both
lists could be *"the workspace, less `planet-bevy` and `game4x` and the prototypes"* rather than an
enumeration, at which case a new crate is covered by default and an *exclusion* is what needs
justifying - which is the right way round.

---

<a id="6"></a>

## 6. The quotation guard stops at `crates/`

**Where.** `crates/game-console/tests/quotations.rs:37` (`sources()` collects
`root().join("crates")` only); `prototypes/goldberg-view/README.md:27`.

**What.** The test written in answer to
[R-1](2026-08-28-response-to-the-first-report.md#r-1) is better than what that report asked for, and
it works - it passes, and its module doc explains the convention it checks. Its scope is
`crates/`. The very first document written outside `crates/` since it landed contains an attributed
quotation in the checked form:

> `spec/planet.md` says that drawing exists to *make adjacency legible*

That quotation is accurate today. Nothing would tell anyone when it stopped being.

**Addendum, after `b43d9b4`.** The walk is fixed, and more thoroughly than this asked for: the
guard now covers `crates/`, `prototypes/`, `scripts/`, `tools/` and `hooks/`, shell scripts
included, and stops at `spec/`, `releases/` and `docs/` for the stated reason that a stale quotation
there is another lane's to repair and would red this lane's gate on files it must not touch. That
reasoning is right.

**What remains is the convention, not the coverage.** `quotations.rs:138` recognises a quotation by
`tail.strip_prefix(": *")` - the form `` `spec/x.md`: *italic* `` exactly - and the comment above it
at `:137` explains the skip: *"Anything else is prose that merely mentions the file, and prose is
not a claim about its wording."*

There is a counterexample in the tree. `crates/planet-render/src/realistic.rs:117` reads
`` `spec/planet.md` says the drawing must show **the biome the model has** ``. That is a claim about
the specification's wording, it is false - the specification says no such thing - and the guard
skips it, because `says` is not `: ` and `**bold**` is not `*italic*`. See
[the correction to finding 1](#correction-1), which took that sentence at face value.

**Whether.** **Fix eventually**, and the coverage half is already done. What is left is to decide
whether the near-miss form is a quotation. Two ways: widen the pattern to accept `says` and bold, or
keep one permitted form and fail any line that names a spec file within a sentence of emphasis
without matching it. The second is stricter and can say so in its failure message, which is what
makes a convention learnable rather than guessable - and the first would have caught this one and
not the next variant somebody writes.

---

<a id="7"></a>

## 7. The composition root has grown logic and tests

**Where.** `crates/game4x/src/main.rs:3`, `crates/game4x/src/options.rs` (228 lines, 8 tests),
`crates/game4x/src/inspect.rs` (165 lines).

**What.** `main.rs` still opens *"This module contains no logic. Its whole job is to decide what to
build and to wire the pieces together."* The crate is now 517 lines across three files and has 8
tests. `inspect.rs` is a Bevy plugin - it schedules systems, drives a camera, waits frames, writes
a PNG and a dump.

`docs/architecture.md` states the test this fails:

> Because it holds no logic, it needs no tests of its own - **if it is large enough to be worth
> testing, something has leaked into it that belongs elsewhere.**

**Why.** Two mitigations are real and should be said. Option parsing in a root has explicit
precedent - the same document blesses `planet-view`'s three small files, one of which is option
parsing. And `inspect.rs` earns its place in the root by its own argument: it must drive the
*shipped* binary, and *"a harness that ran a special path would be evidence about the harness."*
That is a good reason for it to be in `game4x` rather than in a test crate.

What has no argument is the doc comment. `main.rs` claims the crate holds no logic while the crate
beside it holds a scheduling plugin and eight tests. And `inspect.rs` itself is the third
screenshot-and-quit path in the tree, after `prototypes/planet-view/src/capture.rs` and the
`scripts/shot.*` wrappers.

**Whether.** **Fix eventually**, and start with one sentence rather than a refactor. `main.rs`'s
opening line should describe the crate as it is: a composition root plus the harness that operates
it, with the reason the harness cannot live anywhere else. Whether `inspect.rs` eventually becomes
its own crate is a design question; the doc comment being false is not.

---

<a id="8"></a>

## 8. Two hand-rolled option parsers

**Where.** `prototypes/planet-view/src/options.rs` (204 lines) and
`crates/game4x/src/options.rs` (228 lines).

**What.** Two independent argument parsers, 432 lines between them. The flags differ almost
entirely - `--capture --generated --globe --jitter --regions --relax --seed --soccer --turn-right
--turn-up --zoom` against `--distance --drawing --dump --pitch --run --settle --shot --size --yaw` -
and three are common: `--width`, `--height`, `--help`. What is duplicated is the *mechanism*: the
walk over `args()`, the "flag expects a value" error, the number parsing, the usage text.

**Whether.** **Noted and deliberately not.** Two parsers with three flags in common is not a
shared abstraction waiting to happen, and a third root would be the point to reconsider - a
prototype whose whole argument is `[` and `]` did not need one, and correctly does not have one.
Recorded so it is not re-found, and so that a third is noticed as a third.

---

<a id="9"></a>

## 9. `docs/architecture.md`'s crate table no longer describes the tree

**Where.** `docs/architecture.md:110-125`, the *What exists today* table.

**What.** Checked row by row against the manifests:

| Row              | What the table says                                                     | What the manifest says                                                     |
| ---------------- | ----------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `planet-terrain` | absent entirely                                                         | a crate, with two dependants                                               |
| `planet-render`  | the algorithms, `planet-model`                                          | also `game-model`, `planet-terrain`                                        |
| `planet-bevy`    | `planet-render`, `planet-ecs`, Bevy                                     | also `game-front`, `planet-model`, `planet-terrain`, `sphere-tessellation` |
| `game-console`   | `command-language`, `game-model`, `planet-model`, `sphere-tessellation` | also `planet-terrain`                                                      |
| prototypes       | `planet-view` only                                                      | `goldberg-view` too                                                        |

Rule 5 in the same document requires every crate to have a `README.md` *linked from this document*.
`planet-terrain` has a README and no link.

**Why.** The table is where a reader goes to learn the shape before opening `cargo tree`, and it
now understates the coupling this whole report is about - it shows nothing beneath the game
depending on the game, which is the finding in [2](#2).

**Whether.** **Noted** - and it is the documentation lane's file, so this report says so and stops.
The `planet-bevy` row has been stale since the first report flagged it in
[finding 6](2026-08-28-crate-boundaries-and-duplication.md#6), so this is the second time.

---

<a id="q3"></a>

## Q3. A question for the documentation lane

Continuing the numbering from the first report, whose Q1 and Q2 are both answered.

`spec/planet.md` now says both of these, six lines apart:

> - A territory's biome is what the terrain gives it. It is not chosen independently of the surface
>   the territory covers.
> - Oceans never isolate land from land. Every territory that is not ocean can be reached from every
>   other without crossing one.

An implementation cannot honour both. Connectivity is a property of the arrangement; the terrain
does not know the arrangement exists, and `planet-terrain`'s design says so as its central claim.
So something has to give, and `join_the_land` gives the first: it takes a territory whose surface is
water and calls it grassland.

That may well be the right trade - the code's choice of `covering_land`, the most common *land*
biome under that same territory, keeps as much of the first rule as the second allows. But the
specification does not say which yields, so the resolution currently lives only in a function.
[Finding 1](#1) stands whichever way this is settled: the picture and the model must agree about a
territory's biome, and today they do not.

---

## What landed well

- **The quotation guard.** [R-1](2026-08-28-response-to-the-first-report.md#r-1) asked for a test
  that reads the sentence off disk. `crates/game-console/tests/quotations.rs` does that and defines
  a checkable convention for what counts as a quotation, which is more than was asked for and is
  the part that makes it maintainable. [Finding 6](#6) is about its reach, not its design.
- **`Planet` holding a region count.** The right fix, for the reason given on it, and it removed a
  fact about the game from the thing that draws it.
- **`WORLD_SEED` shared rather than copied.** `binding.rs` takes it from `planet_terrain` with a
  comment explaining that separate seeds would let the model and the picture disagree - the exact
  failure mode of [finding 1](#1), correctly anticipated one level down.
- **`goldberg-view` deriving its counts from `sphere-tessellation`** rather than writing a table,
  with the reason stated: *"a written-down table could disagree with the geometry, and this
  cannot."*

---

## Carried forward

From [the first report](2026-08-28-crate-boundaries-and-duplication.md). Re-verified against the
tree today; nothing here is new.

| Finding                                                                                       | Now                                                                                                                                                             |
| --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [5](2026-08-28-crate-boundaries-and-duplication.md#5) palette in three places                 | Unchanged - the shader still hard-codes ten colours                                                                                                             |
| [6](2026-08-28-crate-boundaries-and-duplication.md#6) `planet-bevy` reaches into `game-front` | Improved at runtime by `detached`, unchanged at the crate level - see [3](#3)                                                                                   |
| [7](2026-08-28-crate-boundaries-and-duplication.md#7) `planet-ecs` dead in the shipped app    | Unchanged - still added at `main.rs:95`, still fed nothing                                                                                                      |
| [8](2026-08-28-crate-boundaries-and-duplication.md#8) engine-free policy below the gate       | Worse - `planet-terrain` joins it, see [5](#5)                                                                                                                  |
| [9](2026-08-28-crate-boundaries-and-duplication.md#9) `planet_ecs::gather` dead               | Unchanged                                                                                                                                                       |
| [10](2026-08-28-crate-boundaries-and-duplication.md#10) two adjacency computations            | Unchanged, and now with a third consumer: the biomes are computed from `canonical_seeds` in `binding.rs` while the picture is drawn from `World::build`'s solid |
| [11](2026-08-28-crate-boundaries-and-duplication.md#11) `planet-bevy` is two adapters         | Unchanged, and now three composition roots want the split rather than two                                                                                       |
| [12](2026-08-28-crate-boundaries-and-duplication.md#12) two territory identities              | Unchanged - still correctly waiting on 7                                                                                                                        |
| [14](2026-08-28-crate-boundaries-and-duplication.md#14) small duplication and dead code       | Unchanged                                                                                                                                                       |
