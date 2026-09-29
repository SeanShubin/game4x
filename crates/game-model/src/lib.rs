//! The game, during the one migration this crate has ever had.
//!
//! **Two models live here and one of them is leaving.** `releases/rules-become-data.md` says a
//! rule changes when Sean edits data and not before, and its measure is that this crate **stops**
//! holding rules rather than holding fewer. So the thin engine moved in beside the model it
//! replaces, and the model it replaces is removed a piece at a time.
//!
//! - **The engine** - [`notation`], [`store`], [`schema`], [`engine`], [`script`], [`refusal`],
//!   [`view`]. It knows no noun the game has; everything it does is read out of `data/`.
//! - **The model being replaced** - [`game`], [`rules`], [`territory`], [`thing`], [`unit`],
//!   [`transition`], [`identity`], [`rejection`]. A struct per noun and a method per rule.
//!
//! **`tests/isolation.rs` names the second list and asserts its length**, so the migration has a
//! number: eight modules to go, and the exception disappears when the list is empty rather than
//! when somebody remembers to delete it.
//!
//! **It arrived as `crates/thin-engine` and that crate is gone.** Sean, 2026-09-25: *I don't mean
//! to actually delegate to thin-engine* - so it was moved rather than depended on, and there is
//! nothing left to delegate to. `ENGINE.md` is the README it was written under and holds the
//! question it was built to answer.
//!
//! # One function
//!
//! `spec/invariants.md`: *a game state and a transition yield a new game state. There is no
//! other way for state to change.* That is not a description of this crate, it is
//! its entire shape. [`Game::after`] is the function; everything else is the state it
//! reads or the transition it is given.
//!
//! Two consequences worth stating, because both are easy to erode:
//!
//! - **Designing the world goes through it too.** Which phase a game is in is part of its
//!   state, so `create planet` and `land ark` are the same kind of thing and take the same
//!   path. There is no separate constructor that builds a world some other way.
//! - **A game is exactly its transitions.** Applying the same list to the same start
//!   yields the same game, always. Nothing is seeded from a clock, nothing reads the
//!   environment, and there is no floating point - which is what makes that a guarantee
//!   rather than a hope. See `docs/architecture.md` rule 3.
//!
//! # What this crate does not know
//!
//! It has never heard of a parser, a renderer or an engine. It does not know where a
//! territory sits on a sphere: adjacency arrives as a graph of integer ids, computed
//! above and handed in with the transition that creates the planet.

// **The engine.** Seven modules that name no noun the game has - `tests/isolation.rs` is what
// says so, and `data/engine.4x` lists every word any of them branches on.
pub mod engine;
pub mod notation;
pub mod refusal;
pub mod schema;
pub mod script;
pub mod store;
pub mod view;

// **Beside the engine rather than part of it.** The foundation the engine runs on, carried in
// the binary so that a browser build reads the same bytes a desktop build does. It does what the
// engine may not - `include_str!` - which is why `common::BESIDE` names it and the engine's own
// walk skips it.
pub mod foundation;

// **The model being replaced is gone.** Eight modules, a struct per noun and a method per rule,
// and `tests/common`'s `BEING_REPLACED` counted them down from eight to none.
//
// **`D-1` measures this by the crate stopping holding rules rather than holding fewer** - *the
// measure is that it stops holding rules, not that it holds fewer* - so the list that excepted
// them from the engine's own checks goes with them, and the checks are about the whole crate
// again.

/// What kind of ground a territory is, from [`planet_model`].
///
/// Re-exported rather than moved out of sight: a biome is part of this model's vocabulary
/// and every rule that reads one is here. What is *not* here is the definition, because
/// `planet-terrain` and `planet-render` need it too and neither of them is the game.
pub use planet_model::Biome;
