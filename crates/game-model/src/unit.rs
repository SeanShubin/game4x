//! Units, and where they are.

use crate::identity::{TerritoryId, UnitId, UnitKind};

/// Where a unit is: on a territory, or in the orbit above one.
///
/// **`S-55`: an orbit is above a particular territory, and this variant carried nothing.**
/// `spec/orbit.md` forbids the state outright - *nothing orbits a planet without being above
/// a particular territory* - and `releases/first-release.md` -> *Where things are* says there
/// are twelve territories and twelve orbits. A bare `Orbit` made *above nowhere* the only
/// thing a unit in orbit could be.
///
/// **The consequence was not cosmetic.** Landing read any unit in orbit and put it on any
/// territory named, because there was no relation between the two to check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Location {
    /// The orbit above this territory. There is exactly one, and it is adjacent to it.
    Orbit(TerritoryId),
    On(TerritoryId),
}

impl Location {
    /// The territory this place is, or is above. Every place in this release has one.
    pub fn territory(self) -> TerritoryId {
        match self {
            Location::Orbit(id) | Location::On(id) => id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    pub id: UnitId,
    pub kind: UnitKind,
    pub location: Location,
    /// What a ground-moving unit has left to move on.
    ///
    /// `spec/units.md`: *a mobile unit that moves over the ground has a bin for fuel. Moving
    /// burns a unit of it, and one with an empty bin cannot move.*
    ///
    /// **A counter, where the specification now says a bin** - `P-365`, and `C-79` carries
    /// why it is still a counter. A bin is filled from the territory it stands in, and no
    /// recipe in the release fills one, so the refill has no moment to happen at. The Ark's
    /// half of the same proposal - *a mobile unit that moves in orbit takes its energy
    /// directly from the sun. It stores no fuel* - waits on the release blanking its Fuel
    /// cell, which is `S-86`.
    ///
    /// **This comment quoted the sentence `P-365` replaced** - *a mobile unit carries energy
    /// cells; moving spends them* - and `quotations.rs` did not catch it, because that check
    /// looks for an attributed quotation in italics and this one was plain prose. Worth
    /// knowing next time that check is touched.
    pub cells: u32,
    /// Whether this unit has already been used this turn. `spec/turn.md` calls this
    /// ready or exhausted; a thing that is merely used up for the turn is exhausted,
    /// where labor and energy cells are genuinely spent because they are consumed.
    pub exhausted: bool,
    /// `spec/control.md`: when nature takes a territory back, any ark on it becomes
    /// unusable. It is still there; it can no longer do anything.
    pub usable: bool,
}

impl Unit {
    /// A new unit, in the orbit above the territory it was put there over.
    pub fn new(id: UnitId, kind: UnitKind, above: TerritoryId) -> Self {
        Self {
            id,
            kind,
            location: Location::Orbit(above),
            cells: kind.cells(),
            exhausted: false,
            usable: true,
        }
    }

    pub fn force(&self) -> u32 {
        if self.usable { self.kind.force() } else { 0 }
    }

    pub fn is_on(&self, territory: TerritoryId) -> bool {
        self.location == Location::On(territory)
    }

    pub fn in_orbit(&self) -> bool {
        matches!(self.location, Location::Orbit(_))
    }

    /// Whether this unit could act at all: it has not been used and is not a wreck.
    pub fn ready(&self) -> bool {
        self.usable && !self.exhausted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_unit_starts_in_orbit_with_a_full_charge() {
        let unit = Unit::new(UnitId(1), UnitKind::Ark, TerritoryId(1));
        assert!(unit.in_orbit());
        assert_eq!(unit.cells, 2);
        assert!(unit.ready());
        assert_eq!(unit.force(), 2);
    }

    /// An unusable ark is still an object in the world; it simply does nothing. Force
    /// included, since it can no longer be used to hold anything.
    #[test]
    fn an_unusable_unit_holds_no_force_and_cannot_act() {
        let mut unit = Unit::new(UnitId(1), UnitKind::Ark, TerritoryId(1));
        unit.usable = false;
        assert_eq!(unit.force(), 0);
        assert!(!unit.ready());
    }

    #[test]
    fn a_spent_unit_is_not_ready_but_still_holds_its_force() {
        let mut unit = Unit::new(UnitId(1), UnitKind::Pioneer, TerritoryId(1));
        unit.exhausted = true;
        assert!(!unit.ready());
        assert_eq!(unit.force(), 2, "it is still standing there");
    }
}
