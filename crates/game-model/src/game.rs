//! The state, and what can be asked of it.
//!
//! **The one function that changes it moved to [`crate::rules`] on 2026-09-14**, with every
//! other rule. What is left here is a `Game`, the questions a rule asks of it - who controls
//! a territory, how much force is in one, whether the planet is fully exploited - and the
//! release's tuning figures.
//!
//! **The import list below is the measure of the move.** It was eight lines naming a dozen
//! types, because the rules needed every one of them; the state needs four. A file that has
//! stopped reaching for `Transition` is a file that has stopped deciding anything.

use crate::identity::{Resource, TerritoryId, UnitId, UnitKind};
use crate::rejection::Rejection;
use crate::territory::Territory;
use crate::unit::{Location, Unit};

/// Costs, from `releases/first-release.md`.
///
/// Gathered here rather than scattered because they are release tuning rather than rules:
/// `spec/README.md` keeps relationships in the specification and numbers in a release, so
/// these are the numbers and they are meant to move without any rule moving with them.
pub mod cost {
    /// A store costs 1 labor and 1 metal, and holds ten of one resource.
    pub const STORE_LABOR: u32 = 1;
    pub const STORE_METAL: u32 = 1;
    /// A Yard costs 1 labor and 15 metal.
    pub const YARD_LABOR: u32 = 1;
    pub const YARD_METAL: u32 = 15;
    /// An Ark costs 3 metal, 12 energy and 2 citizens, and needs a Yard to produce it.
    pub const ARK_METAL: u32 = 3;
    pub const ARK_ENERGY: u32 = 12;
    pub const ARK_CITIZENS: u32 = 2;
    /// A Pioneer costs 3 metal, 6 energy and 2 citizens.
    ///
    /// **Three is conservation rather than balance.** A landing deploys a garrison and two
    /// extractors at one metal each, so a unit that deploys one has to bind with three. At
    /// four an Ark wasted a metal every landing; at two a Pioneer made one from nothing.
    pub const PIONEER_METAL: u32 = 3;
    /// **A cost again, and it is two rather than the six it was.** `P-486` made the energy a
    /// `put` into the pioneer's bin and this constant went with it; `P-489` made buying a
    /// pioneer a purchase again - *a pioneer's energy is a cost* - so the figure is a
    /// consumption the release states and belongs where the population check can see it.
    ///
    /// **Deleting it was right at the time and restoring it is not a reversal.** While the
    /// energy was a fill, the number was the Units table's `Fuel` and a second copy could only
    /// disagree with it. As a `consume` row it is the release's own figure again.
    pub const PIONEER_ENERGY: u32 = 2;
    pub const PIONEER_CITIZENS: u32 = 2;
    /// An Extractor costs 1 labor and 1 metal.
    ///
    /// The metal is new: `P-152` conserves it, so a thing that can be taken apart for metal
    /// has to have had metal put into it.
    pub const EXTRACTOR_LABOR: u32 = 1;
    pub const EXTRACTOR_METAL: u32 = 1;
    // **A Garrison's two figures are gone, and `C-106` is why.** `P-466` removed the *Costs
    // to produce* column as the Recipes table said twice - true of every other thing, and not
    // of a garrison: no recipe is named for one, so those two figures were stated there and
    // nowhere else and are now stated nowhere. **Nothing here ever charged them**, which is
    // `P-467`'s finding and why deleting them changes no game.
    /// How many Arks one orbit admits.
    ///
    /// **`P-549` moved this bound's container and not its number.** The release says *a
    /// capacity of 2 in the orbit, which is the only place that admits one*, and
    /// `spec/data/limit.4x` says `{limit container:orbit contained:ark n:2}`. It bounded the
    /// territory until 2026-09-24 and nothing read it either way, because nothing produced an
    /// Ark - which is `S-165`.
    pub const ARKS_IN_AN_ORBIT: u32 = 2;
    /// What one firing of `mine energy` produces.
    ///
    /// **`P-552`, and `spec/units.md` says the number in words**: *it mines one unit.* The
    /// release's row says `produce 1 energy`, and `the_costs_in_the_model_are_the_costs_in_the_release`
    /// holds this against it.
    pub const MINED_ENERGY: u32 = 1;
    /// A move costs one energy, **taken from the place the unit leaves**.
    ///
    /// **`$from` is the whole of `S-150`.** `releases/first-release.md` -> Recipes gives
    /// `move` a *consume 1 energy* row whose *Where* cell is `$from`, and `$from` is a place.
    /// It used to come off the unit's own tank, which is why this was `MOVE_CELLS`.
    pub const MOVE_ENERGY: u32 = 1;
}

/// `spec/console.md`: a game has two phases. In the first the world is designed; in the
/// second it is played. Which one it is in is part of the state, which is what lets both
/// go through the same function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Design,
    Play,
}

/// What an orbit holds.
///
/// **An orbit is a place, and a place holds one number per kind** - `spec/logistics.md`. It was
/// nothing at all until `P-552`, because the only thing in an orbit was a unit and a unit held
/// its own fuel; under pooling the fuel is the place's, and `mine energy` produces *above
/// `$where`*, which is this.
///
/// **An orbit declares no capacity of its own.** `spec/logistics.md`: *an orbit has room for the
/// fuel its units carry and for nothing else, because that is what is in it.* So the room comes
/// entirely from [`Game::room_in`] summing the tanks, and an empty orbit has room for nothing -
/// which is why anything mined into one with no Ark left in it is lost at the turn's end.
///
/// **Written over every resource rather than for energy.** Nothing in this release gives an
/// orbit room for anything else, and `UnitKind::holds` answers for all three - so a kind that
/// gained a bin for metal would need no change here. The map is what keeps that true; a single
/// `energy: u32` would be the shape `CLAUDE.md` forbids.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Orbit {
    held: std::collections::BTreeMap<Resource, u32>,
}

impl Orbit {
    /// How much of a resource is in it.
    pub fn store(&self, resource: Resource) -> u32 {
        self.held.get(&resource).copied().unwrap_or(0)
    }

    pub fn add(&mut self, resource: Resource, amount: u32) {
        *self.held.entry(resource).or_insert(0) += amount;
    }

    /// Takes what is there, up to `amount`, and says nothing about the shortfall.
    ///
    /// **The caller asks first.** Every spender in this model checks before it takes, so a
    /// saturating take here cannot hide a shortfall from a player - and `Territory::take` has
    /// the same shape for the same reason.
    pub fn take(&mut self, resource: Resource, amount: u32) {
        let entry = self.held.entry(resource).or_insert(0);
        *entry = entry.saturating_sub(amount);
    }

    /// Every resource it holds any of, in a fixed order.
    pub fn holding(&self) -> Vec<(Resource, u32)> {
        self.held
            .iter()
            .filter(|(_, amount)| **amount > 0)
            .map(|(resource, amount)| (*resource, *amount))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub phase: Phase,
    /// Counts from one once play begins; zero while the world is being designed.
    pub turn: u32,
    pub territories: Vec<Territory>,
    /// Which territories touch, by id. Symmetric.
    pub adjacency: Vec<Vec<TerritoryId>>,
    pub units: Vec<Unit>,
    /// What each orbit holds, keyed by the territory it is above.
    ///
    /// **A map and not a vector beside `territories`, and the reason is a silent no-op.** It was
    /// a `Vec<Orbit>` indexed by territory for an hour, filled only by `create planet` - so a
    /// fixture that pushed territories directly left it empty, and writing energy into an orbit
    /// did nothing at all. **Nothing failed**: the write was dropped, the read said zero, and
    /// the worked example for `mine energy` rendered a recipe that changed nothing.
    ///
    /// **Two collections that have to be the same length are two things that can disagree.**
    /// With a map there is nothing to keep in step: an absent key is an empty orbit, which is
    /// what an orbit nobody has mined into is.
    ///
    /// **`P-552` is what gave an orbit anything to hold.** Before it, an Ark's `Fuel` was blank
    /// and nothing produced into an orbit, so an orbit was a place in the dump and nowhere else.
    pub orbits: std::collections::BTreeMap<TerritoryId, Orbit>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// A game with nothing in it, waiting to be designed.
    pub fn new() -> Self {
        Self {
            phase: Phase::Design,
            turn: 0,
            territories: Vec::new(),
            adjacency: Vec::new(),
            units: Vec::new(),
            orbits: Default::default(),
        }
    }

    // -- constructing -------------------------------------------------------
    //
    // **`Q-99`: a state is built through the model, not by writing its fields from another
    // crate.** Twenty-eight sites outside `game-model` wrote `Game`'s public fields; twelve
    // were fixtures inside `#[cfg(test)]` and **thirteen were the shipped path** - the worked
    // examples in `crates/game-console/src/worked.rs`, a file with no test module at all.
    //
    // **The cost was not an impossible state.** It was a fact stated twice with nothing
    // keeping the two in step: `ground()` wrote `phase = Play` and `turn = 1`, which is
    // exactly and only what [`Transition::Start`] does, and five sites minted `UnitId(1)` by
    // hand where the model mints `units.len() + 1`. **The file that did it is the one whose
    // own header says a hand-derived artifact can show behaviour the code does not have.**
    //
    // **These three keep the states byte-identical**, which is why the repair was available
    // at all: `reports/recipes.md` is what Sean vets for `R-7`, and reaching every
    // before-state through the transitions that produce it would have moved every example
    // under him. What moves instead is where the knowledge lives.

    /// Begin play: turn one.
    ///
    /// **[`Transition::Start`] is this and nothing else**, so the two cannot drift - which is
    /// what `Q-99` found them doing across a crate boundary.
    pub fn start(&mut self) {
        self.phase = Phase::Play;
        self.turn = 1;
    }

    /// Add a territory, with the empty adjacency row that has to come with it.
    ///
    /// **Two collections that must be the same length are two things that can disagree**, which
    /// is what [`Game::orbits`] stopped being a `Vec` over. A caller pushing one and forgetting
    /// the other leaves a territory nothing is adjacent to, and nothing says so.
    pub fn place(&mut self, territory: Territory) {
        self.territories.push(territory);
        self.adjacency.push(Vec::new());
    }

    /// Record that two territories touch, both ways.
    ///
    /// **Adjacency is symmetric** - [`Game::adjacency`] says so - and a caller writing one
    /// direction gets a border a unit can cross once.
    pub fn join(&mut self, one: TerritoryId, other: TerritoryId) {
        for (from, to) in [(one, other), (other, one)] {
            if let Some(row) = self.adjacency.get_mut(from.index())
                && !row.contains(&to)
            {
                row.push(to);
            }
        }
    }

    /// Put a unit at a territory, in the layer its kind belongs to.
    ///
    /// **The id is `units.len() + 1`, which is what [`Transition::AddUnitToOrbit`] computes**,
    /// and `Q-99` found `UnitId(1)` written by hand at five sites outside the crate.
    ///
    /// **Which layer comes from [`Location::of`], not from the caller.** `Unit::new` puts a
    /// unit *in the orbit above* whatever it is given - which is right for the transition it
    /// was written for and wrong for a Pioneer - so all five of those sites followed it with
    /// an assignment putting the unit back where it belonged. **A default that half its
    /// callers correct is a rule stated in the callers**, and `P-549` already says which layer
    /// an Ark is in.
    pub fn station(&mut self, kind: UnitKind, at: TerritoryId) -> UnitId {
        let id = UnitId(self.units.len() as u32 + 1);
        let mut unit = Unit::new(id, kind, at);
        unit.location = Location::of(kind, at);
        self.units.push(unit);
        id
    }

    // -- reading ------------------------------------------------------------

    pub fn territory(&self, id: TerritoryId) -> Result<&Territory, Rejection> {
        self.territories
            .get(id.index())
            .ok_or(Rejection::NoSuchTerritory(id))
    }

    pub(crate) fn territory_mut(&mut self, id: TerritoryId) -> Result<&mut Territory, Rejection> {
        self.territories
            .get_mut(id.index())
            .ok_or(Rejection::NoSuchTerritory(id))
    }

    pub fn units_on(&self, id: TerritoryId) -> Vec<&Unit> {
        self.units.iter().filter(|unit| unit.is_on(id)).collect()
    }

    /// The room a place has for a resource: what it declares, plus what stands in it.
    ///
    /// **One function, because the alternative has already cost a day** - `P-485`. A unit's
    /// fuel was in two views that summed it differently and the state they described drifted
    /// apart by two energy. **The room a tank gives is now read here by everything that asks**
    /// - the turn's end, the containment tree's capacity rows, and the report.
    ///
    /// `spec/logistics.md` -> Containment: *a place's capacity for a kind is the sum of what
    /// is in it that can hold that kind*, and a place *declares none of its own*. A
    /// territory's stores are its own declaration in this release, which
    /// [`Territory::capacity`] keeps; what this adds is everything standing on it.
    pub fn room_for(&self, id: TerritoryId, resource: Resource) -> u32 {
        let declared = match self.territory(id) {
            Ok(place) => place.capacity(resource),
            Err(_) => 0,
        };
        declared + self.brought_room(id, resource)
    }

    /// The room the things standing in a place give it, for one resource.
    ///
    /// **Units are the only such things in this release**, and `UnitKind::holds` is where a
    /// kind says how much. A unit in orbit is not counted: `spec/logistics.md` gives an orbit
    /// its own room for *the fuel its units carry*, and adding it to the ground beneath would
    /// put an orbit's room on a territory.
    pub fn brought_room(&self, id: TerritoryId, resource: Resource) -> u32 {
        self.units_on(id)
            .iter()
            .map(|unit| unit.kind.holds(resource))
            .sum()
    }

    /// What stands in a place, as the turn's end wants it: one pair per resource.
    pub fn brought_room_all(&self, id: TerritoryId) -> Vec<(Resource, u32)> {
        Resource::ALL
            .iter()
            .map(|resource| (*resource, self.brought_room(id, *resource)))
            .collect()
    }

    /// How much of a resource a place holds, whichever layer it is on.
    ///
    /// **`S-168`'s other half, and `P-552` is what made it needed.** `move` spends at the place
    /// the unit leaves, and for an Ark that place is an orbit - so every question a mover asks
    /// has to be askable of either layer. **One function rather than a surface one and an
    /// orbital one**, because two would be able to disagree and this model has already paid for
    /// that once.
    pub fn held_in(&self, place: Location, resource: Resource) -> u32 {
        match place {
            Location::On(id) => self.territory(id).map(|it| it.store(resource)).unwrap_or(0),
            Location::Orbit(id) => self
                .orbits
                .get(&id)
                .map(|it| it.store(resource))
                .unwrap_or(0),
        }
    }

    /// A place's room for a resource: what it declares, plus what stands in it.
    ///
    /// **A territory declares stores and an orbit declares nothing.** `spec/logistics.md`: *an
    /// orbit has room for the fuel its units carry and for nothing else, because that is what
    /// is in it.* So an orbit with no Ark in it has room for nothing, and what was mined into
    /// one goes at the turn's end.
    pub fn room_in(&self, place: Location, resource: Resource) -> u32 {
        let declared = match place {
            Location::On(id) => self
                .territory(id)
                .map(|it| it.capacity(resource))
                .unwrap_or(0),
            Location::Orbit(_) => 0,
        };
        declared + self.tanks_in(place, resource)
    }

    /// The room the units in a place give it, for one resource.
    ///
    /// **Counted per layer rather than per territory**, which is the whole difference from
    /// [`Game::brought_room`]: a pioneer on the ground and an Ark above it are in two places,
    /// and each gives room to its own.
    pub fn tanks_in(&self, place: Location, resource: Resource) -> u32 {
        self.units
            .iter()
            .filter(|unit| unit.location == place)
            .map(|unit| unit.kind.holds(resource))
            .sum()
    }

    /// Put a resource into a place.
    pub(crate) fn add_in(&mut self, place: Location, resource: Resource, amount: u32) {
        match place {
            Location::On(id) => {
                if let Ok(it) = self.territory_mut(id) {
                    it.add(resource, amount);
                }
            }
            // **Inserted rather than found**, which is the whole point of the map: an orbit
            // nobody has mined into has no entry, and mining into it makes one.
            Location::Orbit(id) => self.orbits.entry(id).or_default().add(resource, amount),
        }
    }

    /// Take a resource out of a place, refusing rather than going short.
    pub(crate) fn spend_in(
        &mut self,
        place: Location,
        resource: Resource,
        amount: u32,
    ) -> Result<(), Rejection> {
        let held = self.held_in(place, resource);
        if held < amount {
            return Err(Rejection::NotEnoughResource {
                place,
                resource,
                held,
                needed: amount,
            });
        }
        match place {
            Location::On(id) => self.territory_mut(id)?.take(resource, amount),
            Location::Orbit(id) => {
                if let Some(it) = self.orbits.get_mut(&id) {
                    it.take(resource, amount)
                }
            }
        }
        Ok(())
    }

    pub fn units_in_orbit(&self) -> Vec<&Unit> {
        self.units.iter().filter(|unit| unit.in_orbit()).collect()
    }

    /// How many Arks are in the orbit above a territory.
    ///
    /// **One orbit, not all of them** - `releases/first-release.md` bounds an Ark by *a
    /// capacity of 2 in the orbit*, and `spec/data/limit.4x` says `container:orbit`, so the
    /// container is one orbit rather than the sky. A count over every orbit would refuse the
    /// third launch of a game wherever it happened.
    pub fn arks_above(&self, id: TerritoryId) -> u32 {
        self.units
            .iter()
            .filter(|unit| {
                unit.kind == crate::identity::UnitKind::Ark
                    && unit.location == crate::unit::Location::Orbit(id)
            })
            .count() as u32
    }

    /// All the force present in a territory.
    ///
    /// **Mustered, and `P-416` left nothing else.** `spec/future/force.md` says *force is
    /// mustered each turn and does not outlast it*, and there is no *highest* case anywhere
    /// in the game any more. `P-414` writes the two recipes that produce it: `muster`
    /// requires a garrison and fires once per citizen, `stand` requires none and fires once
    /// per unit. So what a territory presents is a sum and only a sum - what its citizens
    /// mustered, which is nothing without a garrison, plus what each unit standing there
    /// stood with.
    ///
    /// **This is a sum of what the recipes would produce rather than a walk of things
    /// produced.** `discard` sweeps force at the same turn's end that musters it, so no
    /// committed state holds any - the dump names `force` and counts zero for exactly that
    /// reason - and a total computed on demand is the same number a walk would find.
    pub fn force_in(&self, id: TerritoryId) -> u32 {
        let Ok(territory) = self.territory(id) else {
            return 0;
        };
        let stood: u32 = self.units_on(id).into_iter().map(|unit| unit.force()).sum();
        territory.held_force() + stood
    }

    /// Whoever is not you holds a territory with this much force. Nature's, until a
    /// territory is founded.
    pub(crate) fn defending_force(&self, id: TerritoryId) -> u32 {
        let Ok(territory) = self.territory(id) else {
            return 0;
        };
        if territory.founded() {
            self.force_in(id)
        } else {
            territory.force_of_nature()
        }
    }

    pub fn are_adjacent(&self, from: TerritoryId, to: TerritoryId) -> bool {
        self.adjacency
            .get(from.index())
            .map(|near| near.contains(&to))
            .unwrap_or(false)
    }

    /// Every territory the player controls, in id order.
    pub fn controlled(&self) -> Vec<TerritoryId> {
        self.territories
            .iter()
            .filter(|territory| territory.founded())
            .map(|territory| territory.id)
            .collect()
    }

    /// `spec/control.md`: a player has lost when they have no citizens and nothing that
    /// converts into a citizen.
    /// **No citizens anywhere and no units left** - and it used to be *no usable unit*.
    ///
    /// `P-367` made nature destroy what stands on a territory it takes back, which was the
    /// only thing that ever set `usable` to false. With nothing setting it, *no usable unit*
    /// and *no unit* became the same question asked two ways, and the trait went.
    pub fn has_lost(&self) -> bool {
        self.phase == Phase::Play
            && self.territories.iter().all(|t| t.citizens() == 0)
            && self.units.is_empty()
    }

    // -- acting -------------------------------------------------------------

    /// What *fully exploited* means, as `spec/control.md` defined it until `P-527`: *a planet
    /// is fully exploited when every territory that can be taken has been taken, every
    /// territory is producing the greatest output it can, and every storage structure on it is
    /// full.* **That sentence is no longer in the specification** and is quoted here as what
    /// this function was built to, not as what the specification says - `S-151`.
    ///
    /// **The middle clause changed under `P-361` and it is the whole of this function.** It
    /// used to require every structure to have been built everywhere it could be, which counts
    /// what the ground has room for and never asks whether anybody could staff it. Under that
    /// wording the first release **could not be won**: territory 5's nineteen deposits are
    /// every one of them density one, so it never has a spare hand to build a twentieth
    /// extractor with, and territory 6 has no metal at all, so it can never build anything.
    /// Both were permanently short of the bar, so the predicate was false for ever and `R-6`
    /// could not be vetted by playing the game.
    ///
    /// Sean, 2026-09-10: *a fully exploited planet does not mean every territory is fully
    /// exploited, it means that the planet is producing maximum possible resource output.*
    ///
    /// The arithmetic is [`Territory::maximum_output`], which is where the permanent facts
    /// are; the working for all twelve territories is in
    /// `docs/notes/2026-09-10-maximum-possible-output.md`.
    ///
    /// The file is named next to the words rather than back at the top of this comment,
    /// because `crates/game-console/tests/quotations.rs` checks a quotation only where it can
    /// see which document it belongs to. Attributed to *the same section* it reads as prose,
    /// and this comment misquoted the sentence for a whole session under exactly that cover -
    /// `its nodes` where the specification says `how many it has capacity for`. `C-11`
    /// recorded the limitation; this is what it looks like when it bites.
    ///
    /// *Every storage structure is full* holds because there are none. No structure in
    /// `spec/structures.md` stores anything. If one is ever added, this stops being vacuous
    /// and this function will not notice on its own.
    pub fn is_fully_exploited(&self) -> bool {
        self.territories
            .iter()
            .filter(|place| place.biome.is_claimable())
            .all(|place| place.founded() && place.at_maximum_output())
    }

    // -- ending a turn ------------------------------------------------------
}

#[cfg(test)]
mod tests {
    use super::*;
    // **The tests import what the module above no longer does.** They exercise the
    // rules through `after`, so they need the vocabulary a rule needs - and keeping
    // that here rather than at the top is what lets the file's own import list say
    // truthfully that the state decides nothing.
    use crate::Biome;
    use crate::identity::{Resource, StructureKind, UnitId, UnitKind};
    use crate::territory::Garrison;
    use crate::thing::Kind;
    use crate::transition::Transition;
    use crate::unit::Location;

    /// A ring of three territories, enough to test adjacency without a sphere.
    fn ring(count: usize) -> Vec<Vec<TerritoryId>> {
        (0..count)
            .map(|at| {
                let before = (at + count - 1) % count;
                let after = (at + 1) % count;
                let mut near = vec![
                    TerritoryId::from_index(before),
                    TerritoryId::from_index(after),
                ];
                near.sort();
                near.dedup();
                near
            })
            .collect()
    }

    // **Two tests of winning were here and `S-174` removed the concept they tested.**
    // `winning_takes_a_deploy_and_a_launch_in_two_different_territories` and
    // `winning_is_a_moment_rather_than_the_state_afterwards` were built on 2026-09-24 against
    // `spec/control.md`, and Sean removed the win condition hours later: *lets simply remove the
    // concept for now. The user interface is going to just let you keep playing. Launching the
    // ark from a separate territory is the test we are using to make sure the first release
    // works, not a win condition, a requirement.*
    //
    // **`P-556` moved the file whole to `spec/future/control.md`**, so the sentence they
    // asserted is a plan rather than a rule, and `Game::won`, `has_won` and `Game::firings` went
    // with them. **Deleted rather than ignored**: a test of a rule the specification does not
    // have is a claim about the game that nothing backs.
    //
    // **What replaced them is not a test here.** The requirement is that *the scenario* launches
    // from its second territory - `releases/first-release.md` -> `R-6` - which is a fact about a
    // command log and not about any game state, so it is checked where the scenario is checked.

    /// An Ark mines from the sun and crosses on what it mined.
    ///
    /// # What this replaces, and why the old assertion had to go
    ///
    /// **`S-168` left an Ark selectable and unmovable**, and this test asserted that: the Ark was
    /// found in its orbit and refused with `NothingFuelsAnOrbit`, because an orbit held nothing
    /// and nothing put anything there. **That was a gap reported in code**, and `P-552` closed
    /// it - so the assertion is gone rather than adjusted, and the refusal it named is deleted.
    ///
    /// **Sean, `P-552`**: *a mobile unit that moves in orbit gathers its own energy from the
    /// sun. It mines one unit.* The release gives `mine energy` four rows and the Ark a `Fuel`
    /// of 1 and `Readies: moving 1, working 1`.
    ///
    /// # The whole loop, because each half alone passes for the wrong reason
    ///
    /// **Mining with nothing to spend it on** would pass if a crossing were free. **Crossing
    /// without mining first** is the case that must still be refused, and it is asserted here
    /// against an orbit that has an Ark in it - so *no Ark* cannot be the reason. **And the
    /// energy is the orbit's rather than the Ark's**, which is what makes the tank room and not
    /// a charge: it is read back out of the place.
    ///
    /// **One mined unit pays for exactly one crossing**, which is the Ark's `Fuel` of 1 doing
    /// the only thing a tank does - so a second crossing in the same turn is refused for want of
    /// energy rather than for want of a move.
    #[test]
    fn an_ark_mines_from_the_sun_and_crosses_on_what_it_mined() {
        let start = || {
            let mut game = designed().after(&Transition::Start).unwrap();
            game.units.clear();
            game.units
                .push(Unit::new(UnitId(1), UnitKind::Ark, TerritoryId(1)));
            game
        };
        let above = Location::Orbit(TerritoryId(1));
        let mine = Transition::MineEnergy {
            territory: TerritoryId(1),
        };
        let cross = Transition::Move {
            kind: UnitKind::Ark,
            from: TerritoryId(1),
            to: TerritoryId(2),
        };

        // -- an orbit starts empty, and a crossing out of it is refused for the energy.
        let bare = start();
        assert_eq!(
            bare.held_in(above, Resource::Energy),
            0,
            "an orbit holds nothing until something mines"
        );
        let why = bare
            .after(&cross)
            .expect_err("an empty orbit cannot pay for a crossing");
        assert_eq!(
            why,
            Rejection::NotEnoughResource {
                place: above,
                resource: Resource::Energy,
                held: 0,
                needed: cost::MOVE_ENERGY,
            },
            "refused for the orbit's energy, and the Ark is right there"
        );

        // -- mining puts the energy in the orbit, not in the Ark.
        let mined = start().after(&mine).expect("an Ark in orbit can mine");
        assert_eq!(
            mined.held_in(above, Resource::Energy),
            cost::MINED_ENERGY,
            "what it mined is the orbit's"
        );
        assert_eq!(
            mined.room_in(above, Resource::Energy),
            UnitKind::Ark.fuel(),
            "and the room it fits in is the Ark's tank, which is all an orbit declares"
        );

        // -- and one mined unit carries it across.
        let crossed = mined.after(&cross).expect("it can pay now");
        assert!(
            crossed
                .units
                .iter()
                .any(|unit| unit.location == Location::Orbit(TerritoryId(2))),
            "the Ark crossed into the next orbit"
        );
        assert_eq!(
            crossed.held_in(above, Resource::Energy),
            0,
            "the crossing spent what was mined"
        );
        assert_eq!(
            crossed.held_in(Location::Orbit(TerritoryId(2)), Resource::Energy),
            0,
            "and nothing was hauled - `move` has no haul row"
        );
    }

    /// An Ark mines once a turn, and mining is a different count from moving.
    ///
    /// **`P-411`, and the release states it as `Readies: moving 1, working 1`.** Two recipes
    /// naming different actions never compete, so an Ark that has crossed can still mine and one
    /// that has mined can still cross. **Both orders are fired here**, because a single count
    /// would let exactly one of the two through and the other would look like a rule.
    ///
    /// **And a second mining in one turn is refused**, which is what makes `working` a count
    /// rather than a flag nothing reads. **Once a turn and not once a place**: an Ark that mined
    /// in one orbit and crossed into another has still mined this turn, which the second half
    /// below asserts rather than assumes.
    ///
    /// # Why the second order stocks the orbit instead of mining into it
    ///
    /// **An Ark cannot cross before it has mined, because mining is the only thing that fuels an
    /// orbit.** So *cross, then mine* is unreachable by playing - the crossing would need the
    /// energy the mining has not made yet. The orbit is stocked by hand for that half, which is
    /// the only way to separate *crossing spends `moving`* from *crossing spends `working`*.
    #[test]
    fn an_ark_mines_once_a_turn_and_mining_is_not_moving() {
        let start = || {
            let mut game = designed().after(&Transition::Start).unwrap();
            game.units.clear();
            game.units
                .push(Unit::new(UnitId(1), UnitKind::Ark, TerritoryId(1)));
            game
        };
        let mine = Transition::MineEnergy {
            territory: TerritoryId(1),
        };
        let cross = Transition::Move {
            kind: UnitKind::Ark,
            from: TerritoryId(1),
            to: TerritoryId(2),
        };

        let once = start().after(&mine).expect("the first mining of the turn");
        let why = once
            .after(&mine)
            .expect_err("an Ark mines once a turn - `working` is 1");
        assert_eq!(
            why,
            Rejection::AlreadyUsed(UnitKind::Ark),
            "refused as used rather than as missing, and the Ark is still up there"
        );

        // **Mine then cross**, which is the order the loop uses: mining leaves the move alone.
        let crossed = once.after(&cross).expect("mining does not spend the move");
        assert_eq!(
            crossed
                .after(&Transition::MineEnergy {
                    territory: TerritoryId(2),
                })
                .expect_err("it already mined this turn, in the orbit it came from"),
            Rejection::AlreadyUsed(UnitKind::Ark),
            "`working` is once a turn and not once a place"
        );

        // **Cross then mine**, with the orbit stocked rather than mined - see above.
        let mut stocked = start();
        stocked.add_in(
            Location::Orbit(TerritoryId(1)),
            Resource::Energy,
            cost::MOVE_ENERGY,
        );
        let arrived = stocked.after(&cross).expect("it can pay out of the stock");
        let again = arrived
            .after(&Transition::MineEnergy {
                territory: TerritoryId(2),
            })
            .expect("crossing does not spend the working");
        assert_eq!(
            again.held_in(Location::Orbit(TerritoryId(2)), Resource::Energy),
            cost::MINED_ENERGY,
            "it mined in the orbit it arrived in, having crossed first"
        );
    }

    /// Launching leaves an Ark in the orbit above where it was launched, up to two.
    ///
    /// # The check that would have failed before this
    ///
    /// **`launch` fired four of its recipe's five rows and nothing said so.** The release
    /// gives `launch ark` a **produce 1 ark above `$where`** row and `spec/data/line.4x`
    /// writes the same row; the model paid the metal, the energy and the citizens, required
    /// the Yard, and produced nothing. **Every test of launching asked what it cost or whether
    /// it won**, so all of them passed over the missing half.
    ///
    /// **Sean settled it as `P-549`**: *yes launching leaves the ark in orbit, we spend
    /// materials from the surface and end up with an ark in orbit, and an ark is never on the
    /// surface.* `S-165` is the comment in `scenario/commands/play.4x` that said the opposite
    /// and was believed.
    ///
    /// # Both halves, because the bound had no reader either
    ///
    /// **Where it goes, and how many one orbit takes.** An Ark appears above the territory the
    /// launch named and above no other, which is what *above `$where`* means; and the third
    /// launch from the same place is refused, which is the release's *a capacity of 2 in the
    /// orbit*. **The bound was unreadable while nothing produced an Ark** - it bounded the
    /// territory until `P-549` and no code looked at it either way.
    #[test]
    fn launching_leaves_an_ark_in_the_orbit_above_it() {
        let mut game = designed().after(&Transition::Start).unwrap();
        let afford = |game: &mut Game, at: usize, launches: u32| {
            let place = &mut game.territories[at];
            place.set_count(Kind::Yard, 1);
            let enough = place.citizens().max(cost::ARK_CITIZENS * launches);
            place.set_count(Kind::Citizen, enough);
            place.add(Resource::Metal, cost::ARK_METAL * launches);
            place.add(Resource::Energy, cost::ARK_ENERGY * launches);
        };
        // Three launches' worth, so the third is refused by the orbit rather than by the bill.
        afford(&mut game, 0, cost::ARKS_IN_AN_ORBIT + 1);
        // **`designed()` puts an Ark in orbit 1 already**, and this test is about the Ark a
        // launch makes rather than the one a designer placed. Cleared rather than counted
        // from, so the orbit's bound is reached by launching three times and not by two.
        assert!(
            !game.units.is_empty(),
            "the fixture placed a unit, and clearing an empty list would hide a change to it"
        );
        game.units.clear();
        assert_eq!(
            game.arks_above(TerritoryId(1)),
            0,
            "the sky is empty to start with, or the count below says nothing"
        );

        let launch = Transition::Launch {
            territory: TerritoryId(1),
        };
        let once = game.after(&launch).expect("the first launch is affordable");
        assert_eq!(
            once.arks_above(TerritoryId(1)),
            1,
            "launching leaves an Ark in the orbit above where it was launched"
        );
        assert!(
            once.units.iter().all(|unit| unit.in_orbit()),
            "an ark is never on the surface - `P-549`"
        );
        assert_eq!(
            once.arks_above(TerritoryId(2)),
            0,
            "above `$where` and above nowhere else"
        );

        let twice = once.after(&launch).expect("two fit in one orbit");
        assert_eq!(twice.arks_above(TerritoryId(1)), cost::ARKS_IN_AN_ORBIT);

        let refused = twice
            .after(&launch)
            .expect_err("the orbit holds two, and the third has nowhere to go");
        assert_eq!(
            refused,
            Rejection::NoRoomForAnother {
                territory: TerritoryId(1),
                kind: Kind::Ark,
            },
            "refused by the orbit rather than by what it costs"
        );
        assert!(
            twice
                .territory(TerritoryId(1))
                .unwrap()
                .store(Resource::Metal)
                >= cost::ARK_METAL,
            "the third launch was affordable, so the refusal above is about the room"
        );
    }

    // **`launching_from_an_unfinished_planet_wins_nothing` was here and went with the
    // concept** - `S-174`. It asserted that a launch off an unfinished planet was not a win,
    // which was the old condition's second half; there is no win to be nothing now.
    //
    // **What launching does is checked where launching does it**:
    // `launching_leaves_an_ark_in_the_orbit_above_it` below, which is the row `P-549` gave it.

    /// An ocean cannot be taken, so it cannot be what stops a planet being finished.
    #[test]
    fn ocean_does_not_keep_a_planet_from_being_finished() {
        let mut game = designed().after(&Transition::Start).unwrap();
        for place in &mut game.territories {
            // A citizen is what holds it, since `S-19` made control derived. Setting a flag
            // beside an empty territory used to do this, which is the disagreement that
            // rule removes.
            finish(place);
        }
        // Make one of them water and take everything off it.
        game.territories[1].biome = Biome::Ocean;
        game.territories[1].set_count(Kind::Yard, 0);
        game.territories[1]
            .held
            .retain(|thing| thing.kind != Kind::Extractor);
        assert!(
            game.is_fully_exploited(),
            "an unclaimable territory is not an unfinished one"
        );
    }

    /// Put a territory at the greatest output it can reach, without playing it there.
    ///
    /// **A citizen apiece stopped being enough at `P-361`.** These tests used to give each
    /// territory one citizen and every extractor its ground has room for, which satisfied
    /// *every structure has been built everywhere it can be built*. The condition is now
    /// about output, and output needs people: a territory whose food supports twelve is not
    /// at its ceiling with one.
    ///
    /// **The population comes from [`Territory::maximum_output`], and that is worth being
    /// uneasy about**, because the assertion these tests then make is
    /// [`Territory::at_maximum_output`] - the same arithmetic on both sides. It is the right
    /// trade here and only because the arithmetic is checked elsewhere against a source that
    /// is not this code: `the_release_reaches_the_output_the_specification_lane_derived`
    /// compares all twelve territories with the table in
    /// `docs/notes/2026-09-10-maximum-possible-output.md`, which was worked out by hand. What
    /// these three tests are for is *launching from a finished planet wins*, and a fixture
    /// that had to restate the ceiling would be a second copy of it going stale.
    fn finish(place: &mut Territory) {
        let (citizens, _, _) = place.maximum_output();
        place.set_count(Kind::Citizen, citizens.max(1));
        place.set_count(Kind::Yard, 1);
        // **Replaced, not appended.** This was an assignment to `extractors` and became a
        // loop that adds - so a territory the landing had already given two kept them and
        // ended with eleven of nine nodes. An assignment says *these are the extractors now*
        // and a push says *one more*, and only one of those is what finishing a planet by
        // hand means.
        place.held.retain(|thing| thing.kind != Kind::Extractor);
        for resource in Resource::ALL {
            for _ in 0..place.capacity_for(resource) {
                place.add_extractor(resource);
            }
        }
    }

    /// A designed world: three territories with food and metal, one ark in orbit.
    fn designed() -> Game {
        let mut game = Game::new()
            .after(&Transition::CreatePlanet {
                territories: 3,
                adjacency: ring(3),
                biomes: vec![Biome::Grassland; 3],
            })
            .unwrap();
        for at in 1..=3u32 {
            // Room for three of each, said once. `add node` said it three times, because a
            // node was a thing you added rather than a number a territory has.
            for resource in [Resource::Food, Resource::Metal] {
                game = game
                    .after(&Transition::SetResource {
                        territory: TerritoryId(at),
                        resource,
                        extractors: 3,
                        density: 4,
                    })
                    .unwrap();
            }
            game = game
                .after(&Transition::SetForceOfNature {
                    territory: TerritoryId(at),
                    force: 1,
                })
                .unwrap();
        }
        game.after(&Transition::AddUnitToOrbit {
            kind: UnitKind::Ark,
            above: TerritoryId(1),
        })
        .unwrap()
    }

    fn started() -> Game {
        designed().after(&Transition::Start).unwrap()
    }

    /// A landed ark, which is a founded territory: one garrison, one citizen, one food
    /// extractor. Landing is the whole of it - there is no second step.
    /// Labor in hand, which `create labor` is now the only way to get.
    ///
    /// **`P-232` split what `work` used to do in one step.** A test that works an extractor
    /// is testing the working, not the labor, so this says the uninteresting half once
    /// rather than in nine places.
    fn with_labor(game: Game, count: u32, territory: TerritoryId) -> Game {
        game.after(&Transition::CreateLabor { count, territory })
            .unwrap_or_else(|why| panic!("create labor {count} {}: {why}", territory.0))
    }

    fn founded() -> Game {
        started()
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            })
            .unwrap()
    }

    /// Producing a pioneer costs the territory the recipe's energy and fills nothing.
    ///
    /// **This is the check `P-486` left with nothing reading it, and its absence was this
    /// lane's doing.** `the_costs_in_the_model_are_the_costs_in_the_release` compared
    /// `PIONEER_ENERGY` against a `consume 6 energy` row. The promotion deleted the row and
    /// this lane deleted the constant, calling the fill and the payment one number - which is
    /// true, and left the payment itself compared with nothing. **The cost did not go away;
    /// only the comparison did.**
    ///
    /// # Two numbers that were one, and `S-150` separated them
    ///
    /// **The cost is `PIONEER_ENERGY` and the tank is `UnitKind::fuel()`.** Both are 2, and
    /// the whole of what changed is that they are no longer the same fact: the code read the
    /// tank and called it the cost, because *it is built with that bin full, and the energy is
    /// paid where it is built* made them one number. **Under pooling nothing is filled** - the
    /// tank gives the place room and holds nothing - so the payment stands alone and is the
    /// release's `consume 2 energy` row.
    ///
    /// **So this test asserts the cost against the cost**, and the half that asserted the fill
    /// is replaced by the half that asserts the room. Paying without giving the place room
    /// would leave a pioneer that changes nothing about where energy can be, which is what the
    /// payment alone cannot tell apart - the same gap the old second half covered, moved to
    /// where the fuel went.
    #[test]
    fn a_pioneer_is_paid_for_where_it_is_built_and_fills_nothing() {
        let before = founded();
        let paid = cost::PIONEER_ENERGY;
        assert!(paid > 0, "a pioneer that costs nothing makes this vacuous");

        let place = TerritoryId(1);
        let held = |game: &Game| game.territory(place).unwrap().store(Resource::Energy);
        let stocked = {
            let mut game = before.clone();
            game.territories[place.index()].add(Resource::Energy, paid + 3);
            game.territories[place.index()].add(Resource::Metal, cost::PIONEER_METAL);
            game.territories[place.index()].put(Kind::Citizen, cost::PIONEER_CITIZENS);
            game
        };
        let energy_before = held(&stocked);
        let room_before = stocked.room_for(place, Resource::Energy);

        let after = stocked
            .after(&Transition::ProducePioneer { territory: place })
            .expect("a stocked territory can produce a pioneer");

        assert_eq!(
            held(&after),
            energy_before - paid,
            "the territory paid {} energy where the recipe says {paid}",
            energy_before - held(&after)
        );

        // **And the tank it paid for is room the territory now has**, which is what a tank
        // does since `S-150`. The pioneer itself holds none of it.
        let made = after
            .units
            .iter()
            .find(|unit| unit.kind == UnitKind::Pioneer)
            .expect("the pioneer was produced");
        assert_eq!(
            after.room_for(place, Resource::Energy),
            room_before + made.kind.fuel(),
            "a pioneer's tank gives the place it stands in room for its fuel"
        );
        assert!(
            made.kind.fuel() > 0,
            "a pioneer with no tank makes the line above vacuous"
        );
    }

    #[test]
    fn a_game_begins_with_nothing_in_the_design_phase() {
        let game = Game::new();
        assert_eq!(game.phase, Phase::Design);
        assert!(game.territories.is_empty());
        assert_eq!(game.turn, 0);
    }

    /// The invariant this crate is shaped around: applying a transition leaves the old
    /// state alone and produces a new one.
    #[test]
    fn a_transition_yields_a_new_state_and_does_not_touch_the_old_one() {
        let before = started();
        let after = before
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            })
            .unwrap();
        assert!(!before.territory(TerritoryId(1)).unwrap().founded());
        assert!(after.territory(TerritoryId(1)).unwrap().founded());
    }

    /// A game is exactly the result of applying every transition in order to the start.
    #[test]
    fn the_same_transitions_always_produce_the_same_game() {
        let script = vec![
            Transition::CreatePlanet {
                territories: 3,
                adjacency: ring(3),
                biomes: vec![Biome::Grassland; 3],
            },
            Transition::SetResource {
                territory: TerritoryId(1),
                resource: Resource::Food,
                extractors: 1,
                density: 4,
            },
            Transition::SetForceOfNature {
                territory: TerritoryId(1),
                force: 1,
            },
            Transition::AddUnitToOrbit {
                kind: UnitKind::Ark,
                above: TerritoryId(1),
            },
            Transition::Start,
            Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            },
            Transition::EndTurn,
        ];
        let once = Game::new().after_all(&script).unwrap();
        let twice = Game::new().after_all(&script).unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    fn a_design_command_is_refused_once_play_has_begun() {
        let rejected = started()
            .after(&Transition::AddUnitToOrbit {
                kind: UnitKind::Ark,
                above: TerritoryId(1),
            })
            .unwrap_err();
        assert!(matches!(rejected, Rejection::WrongPhase { .. }));
    }

    #[test]
    fn a_play_command_is_refused_before_the_game_starts() {
        let rejected = designed()
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            })
            .unwrap_err();
        assert!(matches!(rejected, Rejection::WrongPhase { .. }));
    }

    #[test]
    fn an_ark_lands_only_on_the_territory_its_orbit_is_above() {
        // **`S-55`, and the check it says does not exist.** `land` took any unit for which
        // `in_orbit()` held and put it on any territory named - there was no relation between
        // the two to compare, because `Location::Orbit` carried nothing. `spec/orbit.md`:
        // *nothing is in orbit without being above a particular territory*, and landing is
        // a move, so the only ground an Ark can reach is the ground beneath it.
        // `designed` puts the one Ark in the orbit above territory 1.
        let game = started();
        let refused = game.clone().after(&Transition::Land {
            kind: UnitKind::Ark,
            territory: TerritoryId(3),
        });
        assert!(
            matches!(
                refused,
                Err(Rejection::NotAboveThatTerritory {
                    above: TerritoryId(1),
                    asked: TerritoryId(3),
                    ..
                })
            ),
            "an Ark above territory 1 cannot come down on 3; got {refused:?}"
        );

        // **And the message is the right one, which is the half that is easy to lose.** With
        // no unit in orbit at all the answer is `NoUnitAvailable`, and a player told the wrong
        // reason looks in the wrong place - the same distinction `launch` already draws.
        let mut empty = started();
        empty.units.clear();
        assert!(matches!(
            empty.after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(3),
            }),
            Err(Rejection::NoUnitAvailable { .. })
        ));

        // The ground beneath it is still reachable, or the rule would forbid every landing.
        let landed = game.after(&Transition::Land {
            kind: UnitKind::Ark,
            territory: TerritoryId(1),
        });
        assert!(landed.is_ok(), "it comes down where it was: {landed:?}");
    }

    #[test]
    fn landing_an_ark_founds_the_territory() {
        let game = started()
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            })
            .unwrap();
        assert!(game.territory(TerritoryId(1)).unwrap().founded());
        assert!(game.units.is_empty(), "the ark is consumed by founding");
    }

    /// Taking a territory takes force *greater* than what holds it. An ark is force 2,
    /// so a force of nature of 5 is out of reach.
    #[test]
    fn a_territory_too_strong_to_take_is_refused_in_the_games_terms() {
        let strong = Game::new()
            .after(&Transition::CreatePlanet {
                territories: 1,
                adjacency: vec![vec![]],
                biomes: vec![Biome::Grassland],
            })
            .unwrap()
            .after(&Transition::SetForceOfNature {
                territory: TerritoryId(1),
                force: 5,
            })
            .unwrap()
            .after(&Transition::AddUnitToOrbit {
                kind: UnitKind::Ark,
                above: TerritoryId(1),
            })
            .unwrap()
            .after(&Transition::Start)
            .unwrap();
        let rejected = strong
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            })
            .unwrap_err();
        assert_eq!(
            rejected,
            Rejection::NotEnoughForce {
                territory: TerritoryId(1),
                force: 2,
                needed: 5
            }
        );
    }

    /// Founding: one action that takes the ground and produces a garrison, two citizens
    /// and an extractor for each resource it leaves. There is no moment in between, which
    /// is what `spec/invariants.md` means by no step that is always taken.
    #[test]
    fn a_landing_becomes_a_garrison_two_citizens_and_an_extractor_for_each_resource() {
        let game = founded();
        let place = game.territory(TerritoryId(1)).unwrap();
        assert!(
            place.garrison().is_some(),
            "a structure that holds the ground"
        );
        assert_eq!(place.citizens(), 2);

        // One per resource a landing leaves, which is food and metal. The rule is *for
        // each resource it leaves*, not *two* - the ground here has room for energy too,
        // and a landing does not open it.
        assert_eq!(place.extractors().len(), 2);
        for resource in [Resource::Food, Resource::Metal] {
            assert_eq!(
                place.extractors_for(resource).len(),
                1,
                "a landing leaves one {resource} extractor"
            );
        }

        // The mine is the one that matters: an extractor costs metal, so ground that
        // arrived with only a farm could never build a second thing.
        assert_eq!(
            place.extractors_for(Resource::Metal).len(),
            1,
            "and something to build with"
        );
        assert_eq!(
            place.extractors_for(Resource::Energy).len(),
            0,
            "and no well, which is the first thing the ground has to earn"
        );
        assert!(game.units.is_empty(), "the ark was consumed");
        assert_eq!(game.turn, 1, "and none of it waited for the turn to end");
    }

    /// Force of nature 1, garrison force 1: equal is enough to hold.
    #[test]
    fn a_garrison_holds_a_territory_against_its_force_of_nature() {
        // Fed, because this is about force and not about starving. Since `S-19` control is
        // derived from a citizen being there, so a test that lets the population die is
        // testing upkeep whatever it says in its name.
        let mut game = founded();
        game.territories[0].add(Resource::Food, 9);
        let game = game.after(&Transition::EndTurn).unwrap();
        assert!(
            game.territory(TerritoryId(1)).unwrap().founded(),
            "still held a turn later"
        );
    }

    /// The reclaim, over every pair of force and resistance in a range, with the count said.
    ///
    /// **`P-494` rewrote the condition and this is the second derivation of it.** It was
    /// `force_in(id) < needed`; it is now the presence of a `nature` token that no `hold`
    /// marked. Those two have to agree at every pair, and *agree on one example* is what
    /// `CLAUDE.md` says stops meaning anything the moment the example is edited away.
    ///
    /// **The closed form is the comparison this no longer performs**, which is exactly the
    /// shape `population_two_ways` takes against `grow`: the rule that was replaced is kept as
    /// the thing the replacement is checked against, rather than deleted and trusted.
    #[test]
    fn a_nature_nobody_met_is_the_condition_the_comparison_used_to_be() {
        let mut cases = 0;
        let mut reclaimed = 0;
        // **From one citizen, because a territory with none is unfounded before the reclaim
        // reaches it** - `S-19` derives control from a citizen being there. Zero would report
        // *taken back* for a territory nature never touched, which is the check answering a
        // narrower question than the one asked.
        for nature in 0..=4u32 {
            for citizens in 1..=4u32 {
                let mut game = founded();
                game.territories[0].set_force_of_nature(nature);
                game.territories[0].set_count(crate::thing::Kind::Citizen, citizens);
                // **Fed exactly, so nothing grows and nothing starves.** A surplus would
                // breed, and the force the reclaim reads would be the population after
                // growing rather than the one this loop set.
                game.territories[0].add(Resource::Food, citizens);
                let force = game.force_in(TerritoryId(1));

                let after = game.after(&Transition::EndTurn).unwrap();
                let held = after.territory(TerritoryId(1)).unwrap().founded();

                cases += 1;
                if !held {
                    reclaimed += 1;
                }
                assert_eq!(
                    !held,
                    force < nature,
                    "force {force} against a resistance of {nature}"
                );

                // **And the resistance survives being reclaimed**, which is the one kind
                // `lost_to_nature` keeps: ground that resisted with two still resists with
                // two when nobody is left on it, and clearing it would leave the next
                // founding nothing to take.
                assert_eq!(
                    after.territory(TerritoryId(1)).unwrap().force_of_nature(),
                    nature,
                    "what the ground resists with is not part of what nature takes"
                );
            }
        }
        assert_eq!(cases, 20, "five resistances against four populations");
        assert!(
            reclaimed > 0 && reclaimed < cases,
            "{reclaimed} of {cases} were reclaimed, so the two outcomes are both represented              and this is not a check that always saw one of them"
        );
    }

    /// A citizen bred in an ending is not eaten by the `perish` in the same ending.
    ///
    /// **`P-498` made this askable and the first build of it got it wrong** - `C-119`. `breed`
    /// produces a citizen and says nothing about `paid`; `perish` fires below it on `paid 0`.
    /// So on the reading where an unmarked citizen is unpaid, **every newborn dies in the
    /// ending that made it** - which the assertion inside `grow_or_starve` caught, reporting
    /// two starved against nought unfed.
    ///
    /// **Both halves, because the mark is only right if it still starves somebody.** A rule
    /// that marked every citizen paid would pass the first assertion here and empty the second
    /// of meaning, and that is the shape a fix reaches for when a check only looks one way.
    #[test]
    fn a_citizen_bred_this_turn_has_had_its_food_and_one_that_was_not_fed_has_not() {
        // Two citizens and four food: two eaten by `upkeep`, two left for `breed` to spend,
        // so the population doubles and nobody goes unfed.
        let mut plenty = crate::Territory::empty(TerritoryId(1), Biome::Grassland);
        plenty.set_garrison(Some(crate::territory::Garrison { force: 0 }));
        plenty.put(crate::thing::Kind::Citizen, 2);
        plenty.add_store(Resource::Food);
        plenty.add(Resource::Food, 4);
        plenty.settle_population();
        assert_eq!(
            plenty.citizens(),
            4,
            "two ate, two were bred from what was left, and `perish` took none of them"
        );

        // The same two citizens and one food: one eats, one does not, and `breed` has nothing
        // to spend - so the one nobody fed is the one that goes.
        let mut lean = crate::Territory::empty(TerritoryId(2), Biome::Grassland);
        lean.set_garrison(Some(crate::territory::Garrison { force: 0 }));
        lean.put(crate::thing::Kind::Citizen, 2);
        lean.add_store(Resource::Food);
        lean.add(Resource::Food, 1);
        lean.settle_population();
        assert_eq!(
            lean.citizens(),
            1,
            "one was fed and kept, and the one `upkeep` could not reach perished"
        );

        // **And a committed state carries no mark**, which is `renew` and not the absence of
        // anything having happened - the two are the same bytes in the state and different in
        // the rule, so this reads the survivor rather than the store.
        assert!(
            lean.held
                .iter()
                .filter(|thing| thing.kind == crate::thing::Kind::Citizen)
                .all(|thing| thing.trait_of(crate::thing::Trait::Paid).unwrap_or(0) == 0),
            "`renew` clears the mark, so nothing carries one overnight"
        );
    }

    /// A committed state holds no mark, and that is `renew` rather than nothing happening.
    ///
    /// **The distinction is the whole reason this test exists.** After an ending, every
    /// `nature` reads `met:0` whether `hold` marked it or not - so the state alone cannot
    /// tell *nothing was spent* from *what was spent has been cleared*. Firing the phase by
    /// hand is what separates them.
    #[test]
    fn holding_marks_a_nature_and_renewing_clears_the_mark() {
        let mut place = crate::Territory::empty(TerritoryId(1), Biome::Jungle);
        place.set_force_of_nature(3);
        assert_eq!(place.unmet_natures(), 3, "nothing has met any of them");

        assert_eq!(place.hold_with(2), 2, "two force marks two");
        assert_eq!(
            place.unmet_natures(),
            1,
            "the third is what `reclaim` fires on"
        );

        assert_eq!(
            place.hold_with(5),
            1,
            "it fires until it stops being enabled"
        );
        assert_eq!(
            place.unmet_natures(),
            0,
            "and no force was spent on nothing"
        );

        place.renew_natures();
        assert_eq!(place.unmet_natures(), 3, "`renew` clears every mark");
        assert_eq!(place.force_of_nature(), 3, "and takes none of them away");
    }

    /// `take` consumes, where `hold` marks - which is what makes ground takeable at all.
    #[test]
    fn force_on_ground_nobody_founded_wears_its_resistance_down() {
        let mut place = crate::Territory::empty(TerritoryId(1), Biome::Jungle);
        place.set_force_of_nature(2);

        assert_eq!(place.take_natures(1), 1, "one force takes one");
        assert_eq!(place.force_of_nature(), 1, "and it does not come back");

        assert_eq!(place.take_natures(4), 1, "it stops when there is none left");
        assert_eq!(
            place.force_of_nature(),
            0,
            "ground that resists with nothing"
        );
    }

    #[test]
    fn working_an_extractor_produces_its_nodes_density() {
        let game = with_labor(founded(), 1, TerritoryId(1))
            .after(&Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Food),
            })
            .unwrap();
        let place = game.territory(TerritoryId(1)).unwrap();
        assert_eq!(place.store(Resource::Food), 4, "the node's density");
        assert_eq!(place.labor_available(), 1, "one citizen's labor is spent");
    }

    /// Once per turn: an extractor already worked cannot be worked again.
    #[test]
    fn an_extractor_works_only_once_a_turn() {
        let game = with_labor(founded(), 1, TerritoryId(1))
            .after(&Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Food),
            })
            .unwrap();
        let rejected = game
            .after(&Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Food),
            })
            .unwrap_err();
        assert!(matches!(rejected, Rejection::NothingToWorkAt { .. }));
    }

    #[test]
    fn labor_runs_out_before_the_citizens_do_anything_twice() {
        // Metal in store, so the refusal below is about labor rather than about metal. An
        // extractor costs both now, and a test that stops at the first missing thing stops
        // testing the second.
        let mut game = founded();
        game.territories[0].add(Resource::Metal, 5);
        assert_eq!(
            game.territory(TerritoryId(1)).unwrap().citizens(),
            2,
            "two labor, so two builds and then a refusal"
        );
        let build = |resource| Transition::Build {
            structure: StructureKind::Extractor,
            territory: TerritoryId(1),
            resource: Some(resource),
        };
        // One of each, because the ground has room for two more of each and a third build
        // of the same resource would be refused for capacity. That refusal would read as
        // this one passing while testing nothing.
        //
        // **The refusal has moved, and that is `P-232` working.** It used to come from
        // `build`, which made its own labor and found there were no citizens left. Now
        // `create labor` refuses, because that is the command that spends a citizen - so
        // the command list says where the population ran out instead of leaving a reader to
        // infer it from a build that did not happen.
        let spent = with_labor(game, 1, TerritoryId(1))
            .after(&build(Resource::Metal))
            .unwrap();
        let spent = with_labor(spent, 1, TerritoryId(1))
            .after(&build(Resource::Food))
            .unwrap();
        let rejected = spent
            .after(&Transition::CreateLabor {
                count: 1,
                territory: TerritoryId(1),
            })
            .unwrap_err();
        assert!(
            matches!(rejected, Rejection::NotEnoughLabor { .. }),
            "{rejected}"
        );
    }

    #[test]
    fn a_population_grows_on_the_food_it_gathered() {
        let game = with_labor(founded(), 1, TerritoryId(1))
            .after(&Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Food),
            })
            .unwrap()
            .after(&Transition::EndTurn)
            .unwrap();
        // Two citizens and four food: two spare feed two new, at most doubling.
        assert_eq!(game.territory(TerritoryId(1)).unwrap().citizens(), 4);
    }

    /// What a turn ends with: food expires, metal and energy carry, and both are bounded.
    ///
    /// **`C-11`.** This used to be called *resources left at the end of a turn are
    /// discarded* and asserted metal went to zero. `spec/turn.md` says *what expires
    /// expires, and what was not kept in order is lost*, and *what a territory can keep is
    /// bounded* - neither of which is *everything goes*. The release gives food *a capacity
    /// of 20, and it keeps for one turn*; metal and energy have a capacity and no expiry.
    ///
    /// **The old name is the tell.** It described the code rather than the rule, so it went
    /// on passing while the code and the rule disagreed, and reading it told you the model
    /// was right.
    #[test]
    fn food_expires_at_the_end_of_a_turn_and_metal_carries() {
        let mut game = founded();
        // **The store is built here because `P-427` stopped founding from leaving one.**
        // Before `P-258` a territory kept twenty of everything by declaration and this
        // fixture needed nothing; then founding left a food store and a metal store and it
        // needed nothing again; now what it keeps is what its stores hold and a founding
        // leaves none, because two `produce 1 store` rows made founding a metal source.
        //
        // **The subject is unchanged** - food expires and metal carries - so the store is
        // set up rather than asserted, and the assertion below is about the expiry.
        assert_eq!(
            game.territories[0].capacity(Resource::Metal),
            0,
            "a founding leaves no store since `P-427`"
        );
        game.territories[0].add_store(Resource::Metal);
        assert_eq!(
            game.territories[0].capacity(Resource::Metal),
            10,
            "one store, holding ten"
        );
        game.territories[0].add(Resource::Metal, 5);
        game.territories[0].add(Resource::Food, 9);
        let after = game.after(&Transition::EndTurn).unwrap();
        let place = after.territory(TerritoryId(1)).unwrap();

        assert_eq!(place.store(Resource::Metal), 5, "metal has no expiry");
        assert_eq!(place.store(Resource::Food), 0, "food keeps for one turn");
    }

    /// Anything above what a territory can keep is lost when the turn ends.
    ///
    /// **`P-258` moved the bound off the territory and onto the things in it**, so this
    /// tests two stores rather than a flat twenty. The difference is not the number: a
    /// territory that has built nothing now keeps *nothing*, where before it kept twenty of
    /// everything without having built anywhere to put it.
    #[test]
    fn what_is_over_the_bound_is_lost_when_the_turn_ends() {
        let mut game = founded();
        // **Both stores are built here since `P-427`**; one used to come with the founding.
        // Twenty is the bound the arithmetic below was written for, so the fixture makes it
        // rather than the release handing it over.
        game.territories[0].add_store(Resource::Metal);
        game.territories[0].add_store(Resource::Metal);
        assert_eq!(game.territories[0].capacity(Resource::Metal), 20);
        game.territories[0].add(Resource::Metal, 25);
        game.territories[0].add(Resource::Food, 9);
        let after = game.after(&Transition::EndTurn).unwrap();
        assert_eq!(
            after
                .territory(TerritoryId(1))
                .unwrap()
                .store(Resource::Metal),
            20,
            "two stores of ten, and five over them"
        );
    }

    /// The rule that makes ending a turn safe to parallelise: no territory can affect
    /// another's outcome, so the order they settle in cannot change the result.
    #[test]
    fn settling_territories_in_any_order_gives_the_same_game() {
        let game = founded();
        let forwards = game.clone().after(&Transition::EndTurn).unwrap();

        // **The phases run over the territories in reverse**, which is the property: nothing
        // crosses a boundary, so the order they are taken in cannot change the result. Written
        // as the five phases since `S-125` made `end_turn` phase-major - it drove `settle`
        // per territory before, which is the same claim about the three phases that function
        // used to contain.
        let mut backwards = game;
        let ids: Vec<TerritoryId> = backwards.territories.iter().map(|t| t.id).rev().collect();
        let unpaid: Vec<u32> = ids
            .iter()
            .map(|id| backwards.territories[id.index()].pay_upkeep())
            .collect();
        for (id, unpaid) in ids.iter().zip(unpaid) {
            backwards.territories[id.index()].grow_or_starve(unpaid);
        }
        for id in &ids {
            let brought = backwards.brought_room_all(*id);
            backwards.territories[id.index()].end_of_turn_losses(&brought);
        }
        for id in &ids {
            backwards.territories[id.index()].make_ready();
        }
        for unit in &mut backwards.units {
            unit.exhausted = false;
        }
        backwards.turn += 1;

        assert_eq!(forwards.territories, backwards.territories);
    }

    #[test]
    fn a_yard_costs_metal_that_has_to_be_there() {
        let rejected = with_labor(founded(), 1, TerritoryId(1))
            .after(&Transition::Build {
                structure: StructureKind::Yard,
                territory: TerritoryId(1),
                resource: None,
            })
            .unwrap_err();
        assert_eq!(
            rejected,
            Rejection::NotEnoughResource {
                place: Location::On(TerritoryId(1)),
                resource: Resource::Metal,
                held: 0,
                needed: cost::YARD_METAL,
            }
        );
    }

    /// `spec/control.md`: a territory has at most one garrison, and a garrison is what a
    /// founding unit becomes rather than something built.
    #[test]
    fn a_garrison_cannot_be_built() {
        let rejected = with_labor(founded(), 1, TerritoryId(1))
            .after(&Transition::Build {
                structure: StructureKind::Garrison,
                territory: TerritoryId(1),
                resource: None,
            })
            .unwrap_err();
        assert_eq!(rejected, Rejection::GarrisonIsNotBuilt);
    }

    #[test]
    fn a_pioneer_needs_a_garrison_to_be_produced_at() {
        let bare = started();
        let rejected = bare
            .after(&Transition::ProducePioneer {
                territory: TerritoryId(1),
            })
            .unwrap_err();
        assert_eq!(rejected, Rejection::NotControlled(TerritoryId(1)));
    }

    #[test]
    fn moving_somewhere_that_is_not_next_door_says_so() {
        let mut game = founded();
        // Produce a pioneer by hand: put one on territory 1 with cells.
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer, TerritoryId(1));
        pioneer.location = Location::On(TerritoryId(1));
        game.units.push(pioneer);

        // In a ring of three every territory is adjacent to both others, so build a
        // world where one is not.
        let mut line = game.clone();
        line.adjacency = vec![
            vec![TerritoryId(2)],
            vec![TerritoryId(1), TerritoryId(3)],
            vec![TerritoryId(2)],
        ];
        let rejected = line
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                from: TerritoryId(1),
                to: TerritoryId(3),
            })
            .unwrap_err();
        assert_eq!(
            rejected,
            Rejection::NotAdjacent {
                from: TerritoryId(1),
                to: TerritoryId(3)
            }
        );
    }

    /// Two pioneers that could both make the move, and the command says which.
    ///
    /// **`spec/console.md`: a command *binds what that recipe leaves open: every place it
    /// leaves open, every ingredient it names by family rather than by kind, and any
    /// ingredient or trait value it names with a `$`*.** `move` names two
    /// places with a `$` - `require 1 place ... $from` and `require 1 place, joined to $from
    /// ... $to` - and since `P-460` the command binds both.
    ///
    /// **This is the check that could not be written before.** `C-101` reported the gap and
    /// this test asserted it: two pioneers stood in different territories, both adjacent to
    /// the same third, both with a move left, and one command was a correct description of
    /// two different moves. The model picked the lowest-numbered - reaching for an identity
    /// the specification does not give a unit and, since `P-456`, never will. The assertion
    /// was *exactly one moved*, which is all that could be said.
    ///
    /// **Now the fixture is the same and the question is answerable.** `from:1` moves the
    /// pioneer on 1 and `from:2` moves the pioneer on 2, from the same starting state - so the
    /// tie is broken by the player rather than by the model, and a tie-break in `pick` cannot
    /// make this pass. **Both directions**, because one of them agrees with the old
    /// lowest-numbered behaviour and would have passed against it.
    #[test]
    fn a_command_names_the_place_the_unit_moves_from() {
        let mut game = founded();
        game.territories[1].set_garrison(Some(Garrison::from_founding_unit(2)));
        game.territories[1].put(Kind::Citizen, 1);
        game.territories[2].set_garrison(Some(Garrison::from_founding_unit(2)));
        game.territories[2].put(Kind::Citizen, 1);
        // **Both places can pay for a crossing** - `S-150`. A move consumes one energy at
        // the place it leaves, so without this both directions are refused for a reason that
        // has nothing to do with what the command named.
        for at in [0, 1] {
            game.territories[at].add(Resource::Energy, cost::MOVE_ENERGY);
        }

        // Two pioneers, in two different places, both next to territory 3.
        for (id, at) in [(9u32, TerritoryId(1)), (10, TerritoryId(2))] {
            let mut one = Unit::new(UnitId(id), UnitKind::Pioneer, at);
            one.location = Location::On(at);
            game.units.push(one);
        }
        assert!(
            game.are_adjacent(TerritoryId(1), TerritoryId(3))
                && game.are_adjacent(TerritoryId(2), TerritoryId(3)),
            "the fixture needs both pioneers next door to the destination"
        );

        let mut checked = 0;
        for (from, expected) in [(TerritoryId(1), 9u32), (TerritoryId(2), 10)] {
            let moved = game
                .after(&Transition::Move {
                    kind: UnitKind::Pioneer,
                    from,
                    to: TerritoryId(3),
                })
                .expect("the command is accepted");
            let there: Vec<u32> = moved
                .units
                .iter()
                .filter(|unit| unit.is_on(TerritoryId(3)))
                .map(|unit| unit.id.0)
                .collect();
            assert_eq!(
                there,
                vec![expected],
                "the command said `from:{}`, so that is the pioneer that went",
                from.0
            );
            checked += 1;
        }
        assert_eq!(checked, 2, "both directions, from one starting state");

        // **And naming a place the unit is not in is refused rather than rounded to the
        // nearest one.** `NoUnitThere` is the complaint this rule made sayable.
        let refused = game
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                from: TerritoryId(3),
                to: TerritoryId(1),
            })
            .expect_err("no pioneer is standing on territory 3");
        assert_eq!(
            refused,
            Rejection::NoUnitThere {
                kind: UnitKind::Pioneer,
                territory: TerritoryId(3),
            }
        );
    }
    /// A unit moves once a turn, whatever fuel it has left.
    ///
    /// **The release's `move` requires two things and they are different limits.** *1 unit,
    /// moving at least 1* and *1 energy, that unit* - and it puts the unit back with *moving
    /// one less*, which only `refresh` restores at a turn's end. So the fuel bounds how far a
    /// unit goes in its life and `moving` bounds how often it goes in a turn.
    ///
    /// **Both hold, and nothing here asserted the second one.** The fuel half had a test -
    /// *a move spends a cell* - and the once-a-turn half had none, so this is the missing
    /// one rather than a repair.
    ///
    /// # Why both territories are stocked with energy
    ///
    /// **Otherwise it cannot tell the two limits apart.** A move out of a place with no energy
    /// is refused by the energy, and a test that watched that refusal would pass whether or
    /// not `moving` was enforced at all. So both places are given more energy than two
    /// crossings cost, and the second move is attempted back the way it came, so adjacency
    /// cannot be the refusal either.
    ///
    /// **It used to be stated of the unit's own bin** - *a pioneer that had spent both cells
    /// would be refused by the bin* - and `S-150` moved the fuel to the place. The shape of
    /// the argument did not move with it.
    ///
    /// **This lane read the rule as unenforced before writing it**, which is worth leaving
    /// here. `move_unit`'s own closure tests `cells >= MOVE_CELLS` and adjacency and says
    /// nothing about readiness, and `exhausted` is set on arriving and appears nowhere in that
    /// function again. The gate is one level up: [`Game::pick`] takes only units where
    /// `unit.ready()`, and `ready` is `!exhausted`. **Reading the predicate at the call site
    /// and not the function it is handed to is how a rule looks missing when it is enforced**
    /// - the same shape as reading a check's predicate without asking what it is about.
    #[test]
    fn a_unit_moves_once_a_turn_however_much_energy_stands_behind_it() {
        let mut game = founded();
        game.territories[1].set_garrison(Some(Garrison::from_founding_unit(2)));
        game.territories[1].put(Kind::Citizen, 1);
        game.territories[2].set_garrison(Some(Garrison::from_founding_unit(2)));
        game.territories[2].put(Kind::Citizen, 1);
        // **Both ends can pay, twice over** - `S-150`. A move consumes one energy at the place
        // it leaves, so without this the first crossing is refused and the test asserts
        // nothing about `moving`.
        for at in [0, 1] {
            game.territories[at].add(Resource::Energy, cost::MOVE_ENERGY * 2);
        }
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer, TerritoryId(1));
        pioneer.location = Location::On(TerritoryId(1));
        game.units.push(pioneer);

        let once = game
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                from: TerritoryId(1),
                to: TerritoryId(2),
            })
            .expect("the first move is the one a turn allows");
        let moved = once
            .units
            .iter()
            .find(|unit| unit.kind == UnitKind::Pioneer)
            .expect("it is still a unit");
        assert!(moved.exhausted, "and its `moving` is now 0");
        assert!(
            once.territory(TerritoryId(2))
                .unwrap()
                .store(Resource::Energy)
                >= cost::MOVE_ENERGY,
            "territory 2 can pay for a crossing, so energy cannot be what refuses the next one"
        );

        // **The energy is not what refuses the second move**, which is the half that makes
        // this about `moving`: the place it would leave can pay and it is refused anyway.
        // **Back the way it came**, so adjacency cannot be what refuses it: the first move
        // proved 1 and 2 are neighbours. A destination that merely happened not to be
        // adjacent would refuse for a reason that has nothing to do with `moving`, and this
        // test would pass while the rule went unenforced.
        let again = once.after(&Transition::Move {
            kind: UnitKind::Pioneer,
            from: TerritoryId(2),
            to: TerritoryId(1),
        });
        let why = again.expect_err(
            "a pioneer moved twice in one turn with fuel to spare, where `move` requires \
             `moving at least 1` and `refresh` is the only thing that puts it back",
        );
        assert!(
            !matches!(why, Rejection::NotAdjacent { .. }),
            "refused for the wrong reason - this says nothing about `moving`: {why:?}"
        );
    }

    #[test]
    /// A move spends the energy of the place it leaves, and the unit arrives whole.
    ///
    /// **The two halves are the whole of `S-150`.** It read *spends a cell and keeps the
    /// unit*, and the cell was the pioneer's; it is the territory's now, so what is asserted
    /// is that territory 1 is one energy poorer and territory 2 is not - a unit hauls nothing,
    /// because `move`'s five rows in `releases/first-release.md` have no haul among them.
    fn a_move_spends_the_energy_of_the_place_it_leaves() {
        let mut game = founded();
        // Found territory 2 as well, so moving there is a move rather than a founding.
        // **A citizen is what founds it now** - `S-19`. The garrison came with a flag
        // beside it before, and the flag was the thing that could be set without anybody
        // being there.
        game.territories[1].set_garrison(Some(Garrison::from_founding_unit(2)));
        game.territories[1].put(Kind::Citizen, 1);
        game.territories[0].add(Resource::Energy, 3);
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer, TerritoryId(1));
        pioneer.location = Location::On(TerritoryId(1));
        game.units.push(pioneer);

        let held =
            |game: &Game, id: TerritoryId| game.territory(id).unwrap().store(Resource::Energy);
        let left_before = held(&game, TerritoryId(1));
        let arrived_before = held(&game, TerritoryId(2));

        let moved = game
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                from: TerritoryId(1),
                to: TerritoryId(2),
            })
            .unwrap();
        let pioneer = moved
            .units
            .iter()
            .find(|u| u.kind == UnitKind::Pioneer)
            .expect("it is still a unit");
        assert!(pioneer.is_on(TerritoryId(2)));
        assert_eq!(
            held(&moved, TerritoryId(1)),
            left_before - cost::MOVE_ENERGY,
            "the place it left paid for the crossing"
        );
        assert_eq!(
            held(&moved, TerritoryId(2)),
            arrived_before,
            "and nothing arrived with it - `move` has no haul row"
        );
    }

    /// A unit cannot leave a place that has no energy, and that is `P-512`.
    ///
    /// # The check that would have failed before this
    ///
    /// **It passed for the wrong reason and would have gone on passing.** Until `S-150` a
    /// pioneer moved on what it carried, so this fixture - a place with no energy at all and a
    /// pioneer built with a full tank standing on it - moved successfully. **The refusal is
    /// the new fact**, and it names the territory rather than the unit, which is the other
    /// half of the change: `Rejection::NoCells` could only say *that pioneer has no energy
    /// cells left*.
    ///
    /// **Territory 2 is founded and territory 1 is emptied**, so neither adjacency nor the
    /// destination can be what refuses it - the same move succeeds in
    /// `a_move_spends_the_energy_of_the_place_it_leaves`, where the only difference is the
    /// three energy.
    #[test]
    fn a_unit_cannot_leave_a_place_with_no_energy() {
        let mut game = founded();
        game.territories[1].set_garrison(Some(Garrison::from_founding_unit(2)));
        game.territories[1].put(Kind::Citizen, 1);
        let held = game.territories[0].store(Resource::Energy);
        game.territories[0].take(Resource::Energy, held);
        assert_eq!(
            game.territories[0].store(Resource::Energy),
            0,
            "the fixture is a place with no energy, or this checks nothing"
        );
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer, TerritoryId(1));
        pioneer.location = Location::On(TerritoryId(1));
        game.units.push(pioneer);

        let why = game
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                from: TerritoryId(1),
                to: TerritoryId(2),
            })
            .expect_err("a place with no energy cannot pay for a crossing");
        assert_eq!(
            why,
            Rejection::NotEnoughResource {
                place: Location::On(TerritoryId(1)),
                resource: Resource::Energy,
                held: 0,
                needed: cost::MOVE_ENERGY,
            },
            "the refusal names the place that is short, not the unit"
        );
    }

    /// Founding needs the pioneer standing on the ground, and getting it there is a move.
    ///
    /// **`S-76`, Sean's decision on `P-347`, and the whole of it is checked here** - both
    /// refusals, the move that founds nothing, and the turn the pioneer needs in between.
    /// It used to be one act: a pioneer on adjacent ground founded and was consumed, so
    /// nothing ever stood on ground it had taken but not founded.
    #[test]
    fn founding_by_land_needs_the_pioneer_on_the_ground() {
        let mut game = founded();
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer, TerritoryId(1));
        pioneer.location = Location::On(TerritoryId(1));
        game.units.push(pioneer);

        // Adjacent is no longer enough, and the refusal says what is missing.
        let refused = game
            .after(&Transition::FoundByLand {
                territory: TerritoryId(2),
            })
            .expect_err("the pioneer is on territory 1, not 2");
        assert!(
            refused.to_string().contains("on that ground to found it"),
            "{refused}"
        );

        // Founding ground already held is still the other refusal.
        let refused = game
            .after(&Transition::FoundByLand {
                territory: TerritoryId(1),
            })
            .expect_err("founding ground already held is `move`");
        assert!(refused.to_string().contains("already founded"), "{refused}");

        // **Moving onto unclaimed ground is allowed and founds nothing.** It was refused
        // until `S-76`, because arriving there used to *be* founding.
        //
        // **Territory 1 is given the energy the crossing costs** - `S-150`. Without it the
        // move is refused for want of energy and the two refusals above would be the whole of
        // what this test observed.
        game.territories[0].add(Resource::Energy, cost::MOVE_ENERGY);
        let moved = game
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                from: TerritoryId(1),
                to: TerritoryId(2),
            })
            .expect("a pioneer may cross onto unclaimed ground");
        assert!(
            !moved.territory(TerritoryId(2)).unwrap().founded(),
            "moving is not founding - the ground is still unclaimed"
        );

        // **And it cannot do both in one turn**, because moving spends the unit.
        moved
            .after(&Transition::FoundByLand {
                territory: TerritoryId(2),
            })
            .expect_err("a pioneer that has moved is spent until the turn ends");

        // The turn ends, it is ready, and founding consumes it.
        let next = moved.after(&Transition::EndTurn).unwrap();
        let founded = next
            .after(&Transition::FoundByLand {
                territory: TerritoryId(2),
            })
            .unwrap();
        let two = founded.territory(TerritoryId(2)).unwrap();
        assert!(two.founded());
        assert!(two.garrison().is_some());
        assert_eq!(two.citizens(), 2);
        assert_eq!(two.extractors().len(), 2, "a farm and a mine");
        assert!(
            !founded.units.iter().any(|u| u.kind == UnitKind::Pioneer),
            "the pioneer became the territory"
        );
    }

    #[test]
    fn asking_for_a_territory_that_is_not_there_says_so() {
        let rejected = started()
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(99),
            })
            .unwrap_err();
        assert_eq!(rejected, Rejection::NoSuchTerritory(TerritoryId(99)));
    }

    /// A tank standing in a place is room the place keeps over the turn's end.
    ///
    /// # The check that would have failed before this
    ///
    /// **It would have failed on the number and passed on the shape.** Until `S-150` a
    /// territory's room for a resource was its stores alone, so a territory with one store and
    /// a pioneer on it kept ten energy of the twelve below and lost the rest. **What a tank
    /// gives is now in the sum** - `spec/logistics.md`: *a place's capacity for a kind is the
    /// sum of what is in it that can hold that kind* - so the same place keeps twelve.
    ///
    /// **The pair is what makes it about the tank.** The same turn is ended twice from the
    /// same state, once with the pioneer standing there and once with it in orbit above, and
    /// the difference between what survives is exactly its fuel. **One run alone would pass
    /// against any capacity large enough**, which is the shape of a check that reads a
    /// plausible number rather than the rule.
    #[test]
    fn a_tank_standing_in_a_place_is_room_that_place_keeps() {
        let stocked = |on_the_ground: bool| {
            let mut game = founded();
            let place = &mut game.territories[0];
            let held = place.store(Resource::Energy);
            place.take(Resource::Energy, held);
            place.add_store(Resource::Energy);
            place.add(
                Resource::Energy,
                crate::territory::HOLDS + UnitKind::Pioneer.fuel(),
            );
            let id = UnitId(game.units.len() as u32 + 1);
            let mut pioneer = Unit::new(id, UnitKind::Pioneer, TerritoryId(1));
            if on_the_ground {
                pioneer.location = Location::On(TerritoryId(1));
            }
            game.units.push(pioneer);
            game
        };

        let held = |game: &Game| {
            game.territory(TerritoryId(1))
                .unwrap()
                .store(Resource::Energy)
        };
        let put_by = crate::territory::HOLDS + UnitKind::Pioneer.fuel();
        assert_eq!(held(&stocked(true)), put_by, "both fixtures start level");
        assert_eq!(held(&stocked(false)), put_by);
        assert!(
            UnitKind::Pioneer.fuel() > 0,
            "a pioneer with no tank makes the difference below zero either way"
        );

        let standing = stocked(true).after(&Transition::EndTurn).unwrap();
        let aloft = stocked(false).after(&Transition::EndTurn).unwrap();

        assert_eq!(
            held(&standing) - held(&aloft),
            UnitKind::Pioneer.fuel(),
            "the pioneer on the ground kept its fuel's worth that the one in orbit did not"
        );
        assert_eq!(
            held(&aloft),
            crate::territory::HOLDS,
            "a store is all the room a place with nothing standing on it has"
        );
    }

    /// A unit eats nothing, so an empty territory does not cost it anything.
    ///
    /// **This asserted the opposite until `P-339`** - *a pioneer that is not fed is lost* -
    /// and what it actually observed was `usable = false`, a mark no artifact could show.
    /// `C-62`. **A citizen is the only thing in the release with upkeep now**, so this is the
    /// same fixture asserting the rule that replaced it.
    #[test]
    fn a_unit_takes_no_upkeep_and_is_not_lost_to_an_empty_territory() {
        let mut game = founded();
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer, TerritoryId(1));
        pioneer.location = Location::On(TerritoryId(1));
        game.units.push(pioneer);
        // Nothing was gathered, so there is no food at all. **The resources only** -
        // `held.clear()` would take the citizens too, and the fixture would be about a
        // territory with nobody in it.
        let brought = game.brought_room_all(TerritoryId(1));
        game.territories[0].end_of_turn_losses(&brought);

        let after = game.after(&Transition::EndTurn).unwrap();
        let pioneer = after
            .units
            .iter()
            .find(|unit| unit.kind == UnitKind::Pioneer)
            .expect("the pioneer is still there");
        assert!(
            pioneer.ready(),
            "a unit eats nothing, so it cannot go unpaid"
        );
    }

    /// Nature taking a territory back destroys what is standing on it - `P-367`.
    ///
    /// **The rule changed and the old one was not a defect**, which is why this is here
    /// rather than a correction. `spec/control.md` said *any ark on it becomes unusable*; it
    /// now says *its entire population perishes, and every unit on it is destroyed*. The old
    /// behaviour left a unit that is somewhere, owned, and can never act - a state the
    /// release cannot describe, reading in a hand derivation exactly like a unit that is
    /// merely exhausted.
    ///
    /// **Two units, one on the territory and one elsewhere**, because a rule that destroyed
    /// every unit in the game would pass a check that only counted the one it was about.
    #[test]
    fn nature_taking_a_territory_back_destroys_the_units_on_it() {
        let mut game = founded();
        let doomed = TerritoryId(1);
        let safe = TerritoryId(2);

        for (at, place) in [(doomed, 9), (safe, 0)] {
            let id = UnitId(game.units.len() as u32 + 1);
            let mut pioneer = Unit::new(id, UnitKind::Pioneer, at);
            pioneer.location = Location::On(at);
            game.units.push(pioneer);
            let _ = place;
        }
        let before = game.units.len();
        assert!(before >= 2, "two units were put down and {before} are here");

        // Nature takes it: no garrison, and a force of nature above what is left to hold it.
        //
        // **Fed, deliberately.** The nature sweep runs over `controlled()` *after* settling,
        // so a territory that starves to nobody stops being controlled and nature never
        // reaches it - which is how the first version of this test passed its setup and
        // failed its assertion.
        game.territories[doomed.index()].set_force_of_nature(5);
        game.territories[doomed.index()].set_garrison(None);
        game.territories[doomed.index()].add(Resource::Food, 10);
        assert!(
            game.force_in(doomed) < 5,
            "the fixture has to leave the territory undefended or nothing happens"
        );

        let after = game.after(&Transition::EndTurn).expect("the turn ends");

        assert!(
            !after.units.iter().any(|unit| unit.is_on(doomed)),
            "a unit is still on the territory nature took back"
        );
        assert!(
            after.units.iter().any(|unit| unit.is_on(safe)),
            "the unit on the territory nature did not take is gone too, so this destroys \
             more than the rule says"
        );
        assert_eq!(
            after.units.len(),
            before - 1,
            "exactly the one unit on the lost territory was destroyed"
        );
    }
}
