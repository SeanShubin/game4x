//! Every way a game state may change.
//!
//! `spec/invariants.md`: *every change to game state is representable and executable as a
//! console command*. This enum is the other side of that: one variant for each way, and a
//! command is only ever a way of naming one. Nothing in the game changes except by
//! handing one of these to [`crate::Game::after`].
//!
//! Designing the world is not exempt. `create planet` is a transition like `land` is,
//! because which phase a game is in is part of its state.

use crate::identity::{Biome, Resource, StructureKind, TerritoryId, UnitKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Transition {
    /// Make a planet and its territories.
    ///
    /// Adjacency arrives ready-made rather than being worked out here. Which territories
    /// touch is a fact about a sphere, and this crate has no geometry and no floating
    /// point; the layer that does the tessellating hands the graph in.
    ///
    /// Biomes arrive the same way and for the same reason. `spec/planet.md` says a
    /// territory's biome is *what the terrain gives it*, and the terrain is a continuous
    /// field of floating point over a sphere - so it is read above and handed in, one
    /// answer per territory, in id order.
    CreatePlanet {
        territories: usize,
        adjacency: Vec<Vec<TerritoryId>>,
        biomes: Vec<Biome>,
    },
    AddNode {
        territory: TerritoryId,
        resource: Resource,
        density: u32,
    },
    SetForceOfNature {
        territory: TerritoryId,
        force: u32,
    },
    /// Give a territory its biome.
    ///
    /// `spec/planet.md` has required every territory to have one since biomes existed, and
    /// until this there was no way for a designed world to give it one - so the twelve
    /// territories of the first release carried a debt the language could not pay.
    SetBiome {
        territory: TerritoryId,
        biome: Biome,
    },
    /// Place a unit in orbit before play begins.
    AddUnitToOrbit {
        kind: UnitKind,
    },
    /// End the design phase and begin play.
    Start,

    /// Bring a unit down from orbit. It founds the territory.
    Land {
        kind: UnitKind,
        territory: TerritoryId,
    },
    /// Send a unit from the territory it is in up to orbit.
    Launch {
        kind: UnitKind,
    },
    /// Move a unit to an adjacent territory, taking and founding it if it is not already
    /// controlled.
    Move {
        kind: UnitKind,
        territory: TerritoryId,
    },
    Build {
        structure: StructureKind,
        territory: TerritoryId,
        resource: Option<Resource>,
    },
    Produce {
        kind: UnitKind,
        territory: TerritoryId,
    },
    /// Spend that many citizens' labor at a structure this turn.
    Work {
        count: u32,
        structure: StructureKind,
        territory: TerritoryId,
        resource: Option<Resource>,
    },
    /// Consume, transform, discard and unspend.
    EndTurn,
}

impl Transition {
    /// Whether this belongs to the design phase. `spec/console.md` lists five that are
    /// available only before `start`.
    pub fn is_design(&self) -> bool {
        matches!(
            self,
            Transition::CreatePlanet { .. }
                | Transition::AddNode { .. }
                | Transition::SetForceOfNature { .. }
                | Transition::SetBiome { .. }
                | Transition::AddUnitToOrbit { .. }
                | Transition::Start
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_design_transitions_are_the_ones_the_specification_lists() {
        let design = [
            Transition::CreatePlanet {
                territories: 12,
                adjacency: Vec::new(),
                biomes: Vec::new(),
            },
            Transition::AddNode {
                territory: TerritoryId(1),
                resource: Resource::Food,
                density: 4,
            },
            Transition::SetForceOfNature {
                territory: TerritoryId(1),
                force: 1,
            },
            Transition::SetBiome {
                territory: TerritoryId(1),
                biome: Biome::Grassland,
            },
            Transition::AddUnitToOrbit {
                kind: UnitKind::Ark,
            },
            Transition::Start,
        ];
        for transition in design {
            assert!(transition.is_design(), "{transition:?}");
        }

        let play = [
            Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            },
            Transition::EndTurn,
        ];
        for transition in play {
            assert!(!transition.is_design(), "{transition:?}");
        }
    }
}
