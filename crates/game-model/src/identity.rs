//! What things are, and what they are called.

use std::fmt;

/// Which territory.
///
/// `spec/planet.md`: *each territory has an id, unique within its planet, starting at 1*.
/// So this counts from one, and the index into any array is one less - a distinction kept
/// explicit by [`TerritoryId::index`] rather than left for each caller to remember.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerritoryId(pub u32);

impl TerritoryId {
    pub fn index(self) -> usize {
        self.0 as usize - 1
    }

    pub fn from_index(index: usize) -> Self {
        Self(index as u32 + 1)
    }
}

impl fmt::Display for TerritoryId {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{}", self.0)
    }
}

/// Which unit. Counts from one, for the same reason a territory does: it is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitId(pub u32);

impl fmt::Display for UnitId {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{}", self.0)
    }
}

/// `spec/resources.md`: food for population, metal for building, energy for moving.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resource {
    Food,
    Metal,
    Energy,
}

impl Resource {
    pub const ALL: [Self; 3] = [Self::Food, Self::Metal, Self::Energy];

    pub fn name(self) -> &'static str {
        match self {
            Resource::Food => "food",
            Resource::Metal => "metal",
            Resource::Energy => "energy",
        }
    }

    pub fn named(word: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.name() == word)
    }

    pub fn index(self) -> usize {
        match self {
            Resource::Food => 0,
            Resource::Metal => 1,
            Resource::Energy => 2,
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(self.name())
    }
}

/// `spec/planet.md`: *each territory has a biome*, and *a territory's biome is what the
/// terrain gives it*.
///
/// The list is short on purpose. `docs/notes/planet-appearance.md` is emphatic that biomes
/// should not be enumerated and placed but should **fall out of where fields cross** - so
/// this is the set that a temperature, an elevation, a moisture and a drainage actually
/// produce when they are crossed, and not a catalogue somebody wanted.
///
/// No rule reads a biome yet. It is a fact of the model rather than of the picture so that
/// the two cannot drift: without it a territory could be tundra in the model while the
/// realistic drawing paints rainforest across it, and nothing would be violated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Biome {
    Ocean,
    Ice,
    Tundra,
    Desert,
    Grassland,
    Forest,
    Swamp,
    Rainforest,
    Mountain,
}

impl Biome {
    pub const ALL: [Self; 9] = [
        Self::Ocean,
        Self::Ice,
        Self::Tundra,
        Self::Desert,
        Self::Grassland,
        Self::Forest,
        Self::Swamp,
        Self::Rainforest,
        Self::Mountain,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Biome::Ocean => "ocean",
            Biome::Ice => "ice",
            Biome::Tundra => "tundra",
            Biome::Desert => "desert",
            Biome::Grassland => "grassland",
            Biome::Forest => "forest",
            Biome::Swamp => "swamp",
            Biome::Rainforest => "rainforest",
            Biome::Mountain => "mountain",
        }
    }

    pub fn named(word: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|biome| biome.name() == word)
    }
}

impl fmt::Display for Biome {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(self.name())
    }
}

/// `releases/first-release.md` gives the Ark and the Pioneer. Both are founding units:
/// `spec/unit-types.md` says an Ark arrives from orbit and a Pioneer from an adjacent
/// territory, and each transforms into what a territory needs to sustain itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitKind {
    Ark,
    Pioneer,
}

impl UnitKind {
    pub const ALL: [Self; 2] = [Self::Ark, Self::Pioneer];

    pub fn name(self) -> &'static str {
        match self {
            UnitKind::Ark => "ark",
            UnitKind::Pioneer => "pioneer",
        }
    }

    pub fn named(word: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.name() == word)
    }

    /// Both are force 2 in this release.
    pub fn force(self) -> u32 {
        match self {
            UnitKind::Ark | UnitKind::Pioneer => 2,
        }
    }

    /// How many energy cells it carries when built. A move costs one.
    pub fn cells(self) -> u32 {
        match self {
            UnitKind::Ark | UnitKind::Pioneer => 2,
        }
    }

    /// Food eaten each turn, or nothing. An Ark has no maintenance listed; a Pioneer eats
    /// one food per turn and is lost if it is not paid.
    pub fn upkeep(self) -> u32 {
        match self {
            UnitKind::Ark => 0,
            UnitKind::Pioneer => 1,
        }
    }

    /// Only an Ark may come down from orbit; a Pioneer travels by land.
    pub fn lands_from_orbit(self) -> bool {
        matches!(self, UnitKind::Ark)
    }
}

impl fmt::Display for UnitKind {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(self.name())
    }
}

/// `spec/structures.md`: the extractor, the garrison and the yard.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructureKind {
    Extractor,
    Garrison,
    Yard,
}

impl StructureKind {
    pub const ALL: [Self; 3] = [Self::Extractor, Self::Garrison, Self::Yard];

    pub fn name(self) -> &'static str {
        match self {
            StructureKind::Extractor => "extractor",
            StructureKind::Garrison => "garrison",
            StructureKind::Yard => "yard",
        }
    }

    pub fn named(word: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.name() == word)
    }
}

impl fmt::Display for StructureKind {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The distinction the specification draws: an id counts from one, an index from zero.
    #[test]
    fn a_territory_id_counts_from_one_and_its_index_from_zero() {
        assert_eq!(TerritoryId(1).index(), 0);
        assert_eq!(TerritoryId::from_index(0), TerritoryId(1));
        for raw in 1..=12u32 {
            assert_eq!(
                TerritoryId::from_index(TerritoryId(raw).index()),
                TerritoryId(raw)
            );
        }
    }

    #[test]
    fn every_named_thing_survives_a_round_trip_through_its_name() {
        for resource in Resource::ALL {
            assert_eq!(Resource::named(resource.name()), Some(resource));
        }
        for unit in UnitKind::ALL {
            assert_eq!(UnitKind::named(unit.name()), Some(unit));
        }
        for structure in StructureKind::ALL {
            assert_eq!(StructureKind::named(structure.name()), Some(structure));
        }
        assert_eq!(Resource::named("gold"), None);
        assert_eq!(UnitKind::named("colonizer"), None);
    }

    #[test]
    fn the_release_figures_are_what_the_release_says() {
        assert_eq!(UnitKind::Ark.force(), 2);
        assert_eq!(UnitKind::Ark.cells(), 2);
        assert_eq!(UnitKind::Ark.upkeep(), 0);
        assert_eq!(UnitKind::Pioneer.force(), 2);
        assert_eq!(UnitKind::Pioneer.cells(), 2);
        assert_eq!(UnitKind::Pioneer.upkeep(), 1, "one food per turn");
        assert!(UnitKind::Ark.lands_from_orbit());
        assert!(
            !UnitKind::Pioneer.lands_from_orbit(),
            "a pioneer travels by land"
        );
    }
}
