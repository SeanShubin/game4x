//! The game.
//!
//! # One function
//!
//! `spec/invariants.md`: *a game state and a transition yield a new game state, and there
//! is no other way for state to change.* That is not a description of this crate, it is
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

pub mod game;
pub mod identity;
pub mod rejection;
pub mod territory;
pub mod transition;
pub mod unit;

pub use game::{Game, Phase};
pub use identity::{Biome, Resource, StructureKind, TerritoryId, UnitId, UnitKind};
pub use rejection::Rejection;
pub use territory::{Extractor, Node, Territory};
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
        for entry in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
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
    }
}
