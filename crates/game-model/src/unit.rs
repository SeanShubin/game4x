//! Units, and where they are.

use crate::identity::{Border, TerritoryId, UnitId, UnitKind};

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

impl std::fmt::Display for Location {
    /// How a place is named to a player.
    ///
    /// **A refusal has to say which layer it is about** - `P-552` gave an orbit a number of
    /// energy, so *territory 3 has 0 energy* and *the orbit above territory 3 has 0 energy* are
    /// two different complaints and a player who is given the wrong one looks in the wrong
    /// place. `Rejection::NotEnoughResource` carries a place for that reason and writes it
    /// through here.
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::On(id) => write!(out, "territory {id}"),
            Location::Orbit(id) => write!(out, "the orbit above territory {id}"),
        }
    }
}

impl Location {
    /// The territory this place is, or is above. Every place in this release has one.
    pub fn territory(self) -> TerritoryId {
        match self {
            Location::Orbit(id) | Location::On(id) => id,
        }
    }

    /// The place a unit of this kind is in, over a territory.
    ///
    /// **`S-168`, and it is a derivation rather than a choice.** A kind's `Crosses` cell says
    /// which layer it moves on, and `spec/console.md` says *a place worked out from another is
    /// not open - the orbit above a territory is named by naming the territory*. **So a
    /// command carrying two territory numbers already names an ark's two orbits.**
    ///
    /// **`P-549` is what makes it total**: *an ark is never on the surface*, so no kind could
    /// be in either place with something having to pick.
    pub fn of(kind: UnitKind, territory: TerritoryId) -> Self {
        match kind.crosses() {
            Border::Orbit => Location::Orbit(territory),
            Border::Surface => Location::On(territory),
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
    /// Whether this unit has already worked this turn - `P-552`'s `working`, `0 or 1`.
    ///
    /// **A third count and not a reuse of `moving`**, because `P-411` makes two recipes naming
    /// different actions never compete: an Ark that has mined can still cross, and one that has
    /// crossed can still mine. `releases/first-release.md` gives the Ark **Readies: moving 1,
    /// working 1** for that reason, and the citizen beside it has had two counts all along.
    ///
    /// **`mine energy` is the only thing that spends it**, and `refresh` puts it back with the
    /// others at the turn's end.
    pub worked: bool,
}

impl Unit {
    /// A new unit, in the orbit above the territory it was put there over.
    pub fn new(id: UnitId, kind: UnitKind, above: TerritoryId) -> Self {
        Self {
            id,
            kind,
            location: Location::Orbit(above),
            exhausted: false,
            worked: false,
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
        // **An Ark's tank is 1 since `P-552`, and was none from `S-86` until then.** `S-86`
        // blanked the cell because nothing an Ark did spent a cell; `P-552` gave it `mine
        // energy` and a crossing to spend it on, so the tank is the room one mined unit sits
        // in - which is the whole of an orbit's capacity for energy.
        assert_eq!(unit.kind.fuel(), 1);

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
