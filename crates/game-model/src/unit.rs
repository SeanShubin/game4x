//! Units, and where they are.

use crate::identity::{TerritoryId, UnitId, UnitKind};

/// Where a unit is: on a territory, or in the orbit above one.
///
/// **`S-55`: an orbit is above a particular territory, and this variant carried nothing.**
/// `spec/orbit.md` forbids the state outright - *nothing is in orbit without being above a
/// particular territory* - and `releases/first-release.md` -> *Where things are* says there
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
    // **There is no fuel field here, and its absence is `S-150`.**
    //
    // This was `pub cells: u32` - what a ground-moving unit had left to move on, filled at the
    // build and spent a unit at a time. `spec/logistics.md` -> Containment took it away: a
    // resource in a place is in that place, not in a container inside it; what a place holds
    // of a kind is one number; and the things in it that can hold that kind contribute
    // capacity and hold nothing.
    //
    // **So a pioneer's bin is `UnitKind::fuel()` and nothing else**: a trait of the kind, read
    // wherever a place's room for energy is summed, and the two units of energy it used to
    // carry are the territory's. `move` consumes its energy at `$from`, which is why a unit
    // can no longer leave a place that has none.
    //
    // **A field cannot be asserted absent, so the checks are where it showed.**
    // `containment::entry_for_unit` draws a unit as a leaf, the `unit` table's `fuel` column
    // reads the kind, and a territory's room for energy counts the tanks standing in it.
    //
    // **`spec/units.md` still describes the old bin** - it is built with that bin full, and
    // moving burns a unit of it - which is `C-138`, filed rather than settled here.
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

    /// A new unit starts in orbit, ready, and carrying nothing.
    ///
    /// **The charge it used to start with is gone** - `S-150`. A pioneer was built with two
    /// energy in its bin; under `spec/logistics.md` the bin holds nothing and gives the place
    /// two units of room. **What is asserted here is that the tank is a fact about the kind**,
    /// which is the only place the number lives now.
    #[test]
    fn a_new_unit_starts_in_orbit_ready_and_carrying_nothing() {
        let unit = Unit::new(UnitId(1), UnitKind::Ark, TerritoryId(1));
        assert!(unit.in_orbit());
        assert!(unit.ready());
        assert_eq!(unit.force(), 2);
        // **An Ark's tank is none** - `S-86` blanked its Fuel cell, because a unit that moves
        // in orbit takes its energy from the sun. It reaches the ground by landing, which asks
        // where it is and not what it has left.
        assert_eq!(unit.kind.fuel(), 0);

        // A pioneer travels by land and its tank is the room it brings, so the two are
        // asserted apart.
        let pioneer = Unit::new(UnitId(2), UnitKind::Pioneer, TerritoryId(1));
        assert_eq!(
            pioneer.kind.fuel(),
            2,
            "a pioneer's Fuel cell still says two, and it is room rather than a charge"
        );
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
