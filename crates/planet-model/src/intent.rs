//! What someone asked to happen.
//!
//! An intent is a *request*, not a fact: the model may refuse it. That keeps the rules
//! in one place, and makes a save file the record of what everyone chose to do. The
//! trade-off is recorded in `docs/layers.md` — a later change to the rules changes what
//! a replay produces.

use crate::{PlayerId, RegionId};

/// A single request to change the world.
///
/// Intents are plain values: integers, comparable, and cheap to serialise. Their
/// position in the array is meaningful and is the tie-break when two of them collide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    /// Take an unowned region. Succeeds only if the region is free at the start of the
    /// turn, and only if the player is already next to it — or has nothing at all yet,
    /// in which case this is their opening move.
    Claim { region: RegionId, player: PlayerId },
    /// Give up a region.
    Abandon { region: RegionId },
}

impl Intent {
    /// Which region this intent is about. Used to detect collisions between intents
    /// without knowing what any of them mean.
    pub fn region(self) -> RegionId {
        match self {
            Intent::Claim { region, .. } => region,
            Intent::Abandon { region } => region,
        }
    }
}
