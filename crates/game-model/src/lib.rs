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

// **The model being replaced.** Eight modules, a struct per noun and a method per rule, and the
// list `tests/isolation.rs` excepts by name. **It is a countdown rather than a catalogue**: a
// module deleted here is deleted there, and when both are empty the exception goes with them.
pub mod game;
pub mod identity;
pub mod rejection;
pub mod rules;
pub mod territory;
pub mod thing;
pub mod transition;
pub mod unit;

pub use game::{Game, Phase};
pub use identity::{Resource, StructureKind, TerritoryId, UnitId, UnitKind};

/// What kind of ground a territory is, from [`planet_model`].
///
/// Re-exported rather than moved out of sight: a biome is part of this model's vocabulary
/// and every rule that reads one is here. What is *not* here is the definition, because
/// `planet-terrain` and `planet-render` need it too and neither of them is the game.
pub use planet_model::Biome;
pub use rejection::Rejection;
pub use territory::{Deposit, Extractor, Territory};
pub use transition::Transition;
pub use unit::{Location, Unit};

#[cfg(test)]
mod tests {
    /// The integers-only rule, enforced rather than asserted in prose.
    ///
    /// Beyond reproducing identically on every machine, this is what makes resolving
    /// territories in any order safe: integer addition is associative, so a sum does not
    /// depend on how the work was split. Floating point addition is not, so it would.
    #[test]
    fn no_floating_point_anywhere() {
        let mut offences = Vec::new();
        let mut scanned = 0;
        for entry in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            scanned += 1;
            let text = std::fs::read_to_string(&path).unwrap();
            // The rule binds the code that ships. This very test has to name what it
            // forbids in order to look for it, and so does any test that builds a fixture.
            let code = match text.find("#[cfg(test)]") {
                Some(at) => &text[..at],
                None => &text[..],
            };
            for (number, line) in code.lines().enumerate() {
                if line.trim_start().starts_with("//") {
                    continue;
                }
                if line.contains("f32") || line.contains("f64") {
                    offences.push(format!(
                        "{}:{}",
                        path.file_name().unwrap().to_string_lossy(),
                        number + 1
                    ));
                }
            }
        }
        assert!(
            offences.is_empty(),
            "floating point in the model:\n{}",
            offences.join("\n")
        );
        // **`Q-51`: how many files it read, because an empty scan finds nothing.**
        //
        // `read_dir` is not recursive, and a directory entry has no `rs` extension - so it
        // is skipped by the same `continue` that skips a `Cargo.toml`. **A module moved into
        // a subdirectory of `src/` would be unscanned and this would stay green**, which is
        // the shape where a rule quietly stops binding the code it names.
        //
        // A floor rather than an exact count: the number is a property of how this crate is
        // laid out, and a bound needing an edit whenever a file is added would be edited
        // without being thought about. What it has to catch is the scan collapsing.
        assert!(
            scanned >= 6,
            "only {scanned} files scanned for floating point, which is not this crate"
        );
    }
}
