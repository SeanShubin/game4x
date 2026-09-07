//! Every way a game state may change.
//!
//! `spec/invariants.md`: *every change to game state is representable and executable as a
//! console command*. This enum is the other side of that: one variant for each way, and a
//! command is only ever a way of naming one. Nothing in the game changes except by
//! handing one of these to [`crate::Game::after`].
//!
//! Designing the world is not exempt. `create planet` is a transition like `land` is,
//! because which phase a game is in is part of its state.

use crate::Biome;
use crate::identity::{Resource, StructureKind, TerritoryId, UnitKind};

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
    /// What a territory has of one resource: how many extractors it has total capacity for,
    /// and the density each of them yields.
    ///
    /// `spec/planet.md`: *for each resource, a territory has total capacity for some number
    /// of extractors, and a density that each of them yields.* It replaces rather than adds,
    /// because a territory has one answer per resource - `add node` added them one at a
    /// time, which was the shape when a node was a thing rather than a number.
    SetResource {
        territory: TerritoryId,
        resource: Resource,
        extractors: u32,
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
        /// Which orbit. `S-55`: there are twelve, one above each territory, and a unit put
        /// into orbit without saying which was above nowhere.
        above: TerritoryId,
    },
    /// End the design phase and begin play.
    Start,

    /// Bring a unit down from orbit. It founds the territory.
    Land {
        kind: UnitKind,
        territory: TerritoryId,
    },
    /// Send a unit from the territory it is in up to orbit.
    /// `launch ark`: pay an Ark's cost at a Yard and send it up. Nothing comes back.
    ///
    /// **`P-342` made this one recipe where there were two.** `produce ark` built an Ark and
    /// `launch` moved it to orbit, so winning took two commands and the second was a move
    /// with no recipe behind it - `C-54`. **Launching is not a move**: it consumes the cost
    /// and puts nothing into orbit, which is why the destination this lane could not name is
    /// no longer needed.
    ///
    /// It carries a territory because a recipe requiring a Yard has to say whose.
    Launch {
        territory: TerritoryId,
    },
    /// Move a unit to an adjacent territory, taking and founding it if it is not already
    /// controlled.
    /// Move a unit onto adjacent ground that is already held.
    ///
    /// **`P-214`: this used to be two recipes.** Arriving on unclaimed ground founded it,
    /// so one command fired `move` or `found by land` depending on what was there, and the
    /// player never said which. The model decided by looking. Now the player says, and
    /// `FoundByLand` is the other one.
    Move {
        kind: UnitKind,
        territory: TerritoryId,
    },
    /// Send a pioneer onto adjacent unclaimed ground and found it there.
    ///
    /// The unit is not named, because the recipe names it: `found by land` consumes one
    /// pioneer and nothing else can run it. A command binds what a recipe leaves open, and
    /// this recipe leaves only the place open.
    FoundByLand {
        territory: TerritoryId,
    },
    /// Build a store for one resource.
    ///
    /// **`P-260`: one recipe with a `$resource`, not three.** `build store` takes the
    /// resource the way `build extractor` always has, which is what makes `store` one kind
    /// with a trait rather than three kinds differing in one word.
    BuildStore {
        resource: Resource,
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
    /// Spend that much labor at a structure this turn.
    /// `create labor <count> <territory>`.
    ///
    /// **`P-232`, choice 2.** The release's `create labor` takes a *citizen, ready* and
    /// gives back a *citizen, exhausted* and a *labor*, and `P-214` says every player recipe
    /// has a command. `work` used to do both halves in one step - Sean's seam through
    /// `S-21` - and this is where they separate.
    ///
    /// **The closure test is why he chose it**: with the definitions and the commands, a
    /// reader can account for every citizen spent. Under a world recipe nothing in the
    /// command list would say one was.
    CreateLabor {
        count: u32,
        territory: TerritoryId,
    },
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
                | Transition::SetResource { .. }
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
            Transition::SetResource {
                territory: TerritoryId(1),
                resource: Resource::Food,
                extractors: 3,
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
                above: TerritoryId(1),
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
