//! The model: what is true about the world, and how it changes.
//!
//! This crate is one function and the data it needs:
//!
//! ```text
//! (old world, intent array) -> new world
//! ```
//!
//! It is the only part of the project that has to be *certain*. Everything in here is
//! therefore:
//!
//! - **Integers only.** No `f32`, no `f64`, anywhere. Enforced by a test, not by
//!   discipline. See [`no_floating_point_anywhere`](#).
//! - **Pure.** No clocks, no randomness, no I/O, no globals.
//! - **Engine-free.** No Bevy. An algorithm never names `Entity`, `Query` or `Commands`.
//! - **Order-independent.** The result depends on the *contents and order of the intent
//!   array*, and on nothing about how the work was scheduled.
//!
//! The reasoning is in `docs/layers.md`. The short version: the intent array's order is
//! an input and is free to matter; the schedule's order is an accident and must not.
//!
//! # Why there is so little game here
//!
//! There is one rule — claiming a region — and it exists to make the architecture real
//! and testable rather than to be good game design. It is meant to be replaced. What
//! should survive is the *shape*: gather into plain data, resolve with a pure function,
//! apply the result.

pub mod intent;
pub mod size;
pub mod topology;
pub mod world;

pub use intent::Intent;
pub use size::PlanetSize;
pub use topology::Topology;
pub use world::World;

/// Which region. This is the canonical identity of a region, everywhere, forever.
///
/// Deliberately not a Bevy `Entity`: entity ids are reused and are not stable across
/// runs or saves, so they can never be serialised, ordered, or used as a tie-break.
/// A `RegionId` is an index into the topology and means the same thing in every run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionId(pub u32);

impl RegionId {
    pub fn index(self) -> usize {
        self.0 as usize
    }

    /// The territory's id as a player sees it: unique within its planet, **starting at
    /// one**.
    ///
    /// A `RegionId` counts from zero because that is what indexes an array, and every
    /// lookup in this crate is an array lookup. The id a player reads counts from one.
    /// Those are two different numbers for the same territory, and the only way to stop
    /// one being shown where the other was meant is to make each of them say which it is.
    pub fn number(self) -> u32 {
        self.0 + 1
    }
}

/// Which player.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerId(pub u16);

#[cfg(test)]
mod id_tests {
    use super::*;

    /// The first territory is number one, not number zero.
    #[test]
    fn ids_shown_to_a_player_start_at_one() {
        assert_eq!(RegionId(0).number(), 1);
        assert_eq!(RegionId(0).index(), 0);
        assert_eq!(RegionId(91).number(), 92, "the last of ninety-two");
        // Distinct territories keep distinct numbers, which is the whole point of an id.
        let numbers: std::collections::BTreeSet<u32> =
            (0..92u32).map(|raw| RegionId(raw).number()).collect();
        assert_eq!(numbers.len(), 92);
        assert_eq!(numbers.iter().next(), Some(&1));
        assert_eq!(numbers.iter().next_back(), Some(&92));
    }
}

#[cfg(test)]
mod tests {
    /// The integers-only rule, enforced rather than asserted in prose.
    ///
    /// It is not only about cross-machine reproducibility. Integer addition is
    /// associative, so a parallel reduction gives the same answer however the work is
    /// split; floating point addition is not, so it does not. A single `f64` in here
    /// would quietly break both guarantees.
    #[test]
    fn no_floating_point_anywhere() {
        // Built at runtime so that this test's own source does not match itself.
        let banned = [["f", "32"].concat(), ["f", "64"].concat()];
        let sources = [
            ("lib.rs", include_str!("lib.rs")),
            ("topology.rs", include_str!("topology.rs")),
            ("world.rs", include_str!("world.rs")),
            ("intent.rs", include_str!("intent.rs")),
        ];
        for (name, source) in sources {
            // Comments are prose and may name the thing they forbid; only code counts.
            let code: String = source
                .lines()
                .map(|line| line.split("//").next().unwrap_or(""))
                .collect::<Vec<_>>()
                .concat();
            for needle in &banned {
                assert!(
                    !code.contains(needle.as_str()),
                    "{name} uses {needle} in code; the model is integers only"
                );
            }
        }
    }
}
