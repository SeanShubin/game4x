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
    ///
    /// **`P-411` names it `moving`**, a count of `0 or 1` that `move` spends and `refresh`
    /// puts back. The storage is unchanged and the data file says the number.
    pub exhausted: bool,
    /// Whether this unit has already stood this turn - `P-414`'s `defending`, `0 or 1`.
    ///
    /// **A separate count from `moving`, because `P-411` makes them separate**: two recipes
    /// naming the same action draw on the same count, and two naming different actions never
    /// compete. A unit that has moved can still stand.
    pub stood: bool,
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
            stood: false,
        }
    }

    pub fn force(&self) -> u32 {
        self.kind.force()
    }

    pub fn is_on(&self, territory: TerritoryId) -> bool {
        self.location == Location::On(territory)
    }

    pub fn in_orbit(&self) -> bool {
        matches!(self.location, Location::Orbit(_))
    }

    /// Whether this unit could act at all, which is now only whether it has been used.
    ///
    /// **It used to ask whether the unit was a wreck as well** - `P-367` made nature destroy
    /// what stands on a territory it takes back, so there are no wrecks to ask about.
    pub fn ready(&self) -> bool {
        !self.exhausted
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

    // **The test for an unusable unit is gone, and so is the state** - `P-367`. It asserted
    // that a wrecked ark holds no force and cannot act; nature destroys what it takes now,
    // so there is no wrecked ark to ask about. The rule that replaced it is checked in
    // `game.rs`, at `nature_taking_a_territory_back_destroys_the_units_on_it`, because it is
    // about what a turn does and not about what a unit is.

    #[test]
    fn a_spent_unit_is_not_ready_but_still_holds_its_force() {
        let mut unit = Unit::new(UnitId(1), UnitKind::Pioneer, TerritoryId(1));
        unit.exhausted = true;
        assert!(!unit.ready());
        assert_eq!(unit.force(), 2, "it is still standing there");
    }
}
