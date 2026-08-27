//! Units, and where they are.

use crate::identity::{TerritoryId, UnitId, UnitKind};

/// Where a unit is. There are only two places in this release: above the planet, or on a
/// territory of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Location {
    Orbit,
    On(TerritoryId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    pub id: UnitId,
    pub kind: UnitKind,
    pub location: Location,
    /// `spec/units.md`: a mobile unit carries energy cells; moving spends them, and a
    /// unit with none cannot move.
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
    pub fn new(id: UnitId, kind: UnitKind) -> Self {
        Self {
            id,
            kind,
            location: Location::Orbit,
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
        self.location == Location::Orbit
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
        let unit = Unit::new(UnitId(1), UnitKind::Ark);
        assert!(unit.in_orbit());
        assert_eq!(unit.cells, 2);
        assert!(unit.ready());
        assert_eq!(unit.force(), 2);
    }

    /// An unusable ark is still an object in the world; it simply does nothing. Force
    /// included, since it can no longer be used to hold anything.
    #[test]
    fn an_unusable_unit_holds_no_force_and_cannot_act() {
        let mut unit = Unit::new(UnitId(1), UnitKind::Ark);
        unit.usable = false;
        assert_eq!(unit.force(), 0);
        assert!(!unit.ready());
    }

    #[test]
    fn a_spent_unit_is_not_ready_but_still_holds_its_force() {
        let mut unit = Unit::new(UnitId(1), UnitKind::Pioneer);
        unit.exhausted = true;
        assert!(!unit.ready());
        assert_eq!(unit.force(), 2, "it is still standing there");
    }
}
