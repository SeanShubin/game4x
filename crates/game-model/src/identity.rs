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

/// A border a unit crosses when it moves, which is the layer it moves on.
///
/// **`releases/first-release.md` -> Units and structures, the `Crosses` column**, which was
/// written down and read by nothing until `S-168`: an ark crosses an *orbit border*, a pioneer
/// a *border*, and the four things that do not move cross nothing.
///
/// **`spec/orbit.md` is what makes this a layer rather than a pair of cases**: *an orbit
/// boundary is one an orbit is on either side of*, and *two places on the same layer are
/// adjacent when their territories are*. So the adjacency table already answers both, and a
/// kind's border says which layer to read it on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Border {
    /// Between two territories.
    Surface,
    /// Between two orbits.
    Orbit,
}

impl Border {
    /// What the release's `Crosses` cell says, so the cell can be compared with this.
    pub fn name(self) -> &'static str {
        match self {
            Border::Surface => "border",
            Border::Orbit => "orbit border",
        }
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

    /// Both are strength 2 in this release.
    ///
    /// **The trait is `strength` since `P-435`**, which renamed it so that the kind could
    /// keep the word `force` - `C-93`. This is still `force()` because what it answers is
    /// how much force the unit musters, which `stand` turns into a `force`.
    pub fn force(self) -> u32 {
        match self {
            UnitKind::Ark | UnitKind::Pioneer => 2,
        }
    }

    /// How much room this kind's tank gives the place it stands in, for energy.
    ///
    /// **It was `cells()` and it was what the unit carried** - `S-150`, and the rename is the
    /// change. `spec/logistics.md` -> Containment: *the things in it that can hold that kind
    /// contribute capacity and hold nothing*, so a pioneer's bin holds no energy and the two
    /// it used to carry are the territory's. **The number did not move and its meaning did**:
    /// a pioneer's `Fuel` is 2 either way, and it is now a capacity rather than a charge.
    ///
    /// **`releases/first-release.md` -> Where things are is what this is read from**, which
    /// gives a unit's tank as a thing that *gives room for* energy *up to the unit's fuel* -
    /// one of the release's three sorts of capacity, beside a store's.
    ///
    /// **And `spec/units.md` still says the bin is built full and moving burns a unit of
    /// it**, which is `C-138`. The release is the work order and this follows the release.
    ///
    /// **An Ark carries none, since `S-86` blanked its Fuel cell** - the release half `C-79`
    /// was waiting on, and this is the other half changed in the same breath. `spec/units.md`:
    /// *a mobile unit that moves in orbit gathers its energy from the sun*, a fixed amount each
    /// turn, *and holds it in a bin of its own*.
    ///
    /// **The sentence moved under this comment and the consequence held.** It read *it stores
    /// no fuel*; it now gives such a unit a bin of its own, filled from the sun rather than
    /// from a territory - so what an Ark is *built* carrying is still none, which is the fact
    /// this cell is about.
    ///
    /// **It costs an Ark nothing, which is why this is safe rather than merely correct.** An
    /// Ark reaches the ground by landing and is consumed by `deploy ark`; `Game::land` asks
    /// where it is and not what it has left, and `move` is the only thing that spends a cell.
    /// So a bin of zero takes away a capacity nothing used.
    pub fn fuel(self) -> u32 {
        match self {
            UnitKind::Ark => 0,
            UnitKind::Pioneer => 2,
        }
    }

    /// Which border this kind crosses when it moves, which is the layer it moves on.
    ///
    /// **`releases/first-release.md` -> Units and structures, the `Crosses` column** - an ark
    /// crosses an *orbit border* and a pioneer a *border*. **The cell had no reader until
    /// `S-168`**, which is why `move` looked on the surface for every kind and an ark could
    /// never be found.
    ///
    /// **It is what lets a command name an orbit without a word for one.**
    /// `spec/console.md`: *a place worked out from another is not open - the orbit above a
    /// territory is named by naming the territory.* So `{move unit:ark from:1 to:2}` carries
    /// two territory numbers and means two orbits, and this is what works that out.
    pub fn crosses(self) -> Border {
        match self {
            UnitKind::Ark => Border::Orbit,
            UnitKind::Pioneer => Border::Surface,
        }
    }

    /// How much room this kind gives the place it stands in, for one resource.
    ///
    /// **Written over every resource rather than for energy** - a bin for metal would be read
    /// here without a word changing, which is the shape the rest of this model is in. The
    /// match is exhaustive so that a resource added to the release has to be answered rather
    /// than defaulting to nothing.
    ///
    /// **This is the rule `spec/logistics.md` states and the release instances**: *a place's
    /// capacity for a kind is the sum of what is in it that can hold that kind*, and *Where
    /// things are* gives a unit's tank as one of the three things that give a place room.
    pub fn holds(self, resource: Resource) -> u32 {
        match resource {
            Resource::Energy => self.fuel(),
            Resource::Food | Resource::Metal => 0,
        }
    }

    /// What a unit eats each turn, which is nothing.
    ///
    /// **`P-339`: neither an ark nor a pioneer takes upkeep**, and a citizen is now the only
    /// thing in the release that does. It answers `C-62`: the model marked an unpaid unit
    /// `usable = false` rather than consuming it, and `usable` was a trait the release did not
    /// declare - so a pioneer that had starved read exactly like one that had not, in the file
    /// Sean derives by hand. **There is no starved unit now**, so the marking has nothing to
    /// mark.
    ///
    /// **Kept as a method rather than deleted**, because upkeep is a column of *Units and
    /// structures* and a kind that gains one gains it here. The two cells are empty today.
    pub fn upkeep(self) -> u32 {
        match self {
            UnitKind::Ark | UnitKind::Pioneer => 0,
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
        assert_eq!(
            UnitKind::Ark.fuel(),
            0,
            "`S-86` blanked the Ark's Fuel cell: it takes its energy from the sun and stores              none"
        );
        assert_eq!(UnitKind::Ark.upkeep(), 0);
        assert_eq!(UnitKind::Pioneer.force(), 2);
        assert_eq!(UnitKind::Pioneer.fuel(), 2);
        assert_eq!(
            UnitKind::Pioneer.upkeep(),
            0,
            "`P-339`: a pioneer's Upkeep cell is empty"
        );
        assert!(UnitKind::Ark.lands_from_orbit());
        assert!(
            !UnitKind::Pioneer.lands_from_orbit(),
            "a pioneer travels by land"
        );
    }
}
