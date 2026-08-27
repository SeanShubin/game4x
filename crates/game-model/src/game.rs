//! The state, and the one function that changes it.

use crate::identity::{Resource, StructureKind, TerritoryId, UnitId, UnitKind};
use crate::rejection::Rejection;
use crate::territory::{Extractor, Garrison, Node, Territory, population_after};
use crate::transition::Transition;
use crate::unit::{Location, Unit};

/// Costs, from `releases/first-release.md`.
///
/// Gathered here rather than scattered because they are release tuning rather than rules:
/// `spec/README.md` keeps relationships in the specification and numbers in a release, so
/// these are the numbers and they are meant to move without any rule moving with them.
pub mod cost {
    /// A Yard costs 15 metal.
    pub const YARD_METAL: u32 = 15;
    /// An Ark costs 12 metal and 12 energy, and needs a Yard to produce it.
    pub const ARK_METAL: u32 = 12;
    pub const ARK_ENERGY: u32 = 12;
    /// A Pioneer costs 8 metal, 1 citizen and 6 energy.
    pub const PIONEER_METAL: u32 = 8;
    pub const PIONEER_ENERGY: u32 = 6;
    pub const PIONEER_CITIZENS: u32 = 1;
    /// An Extractor costs one labor and nothing else.
    pub const EXTRACTOR_LABOR: u32 = 1;
    /// A move costs one energy cell.
    pub const MOVE_CELLS: u32 = 1;
}

/// `spec/console.md`: a game has two phases. In the first the world is designed; in the
/// second it is played. Which one it is in is part of the state, which is what lets both
/// go through the same function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Design,
    Play,
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
        }
    }

    /// **The function.** A game state and a transition yield a new game state.
    ///
    /// The old state is left alone and a new one is returned, which is what makes a game
    /// exactly the result of applying every transition in order to the start. A rejection
    /// yields no state at all: a command that cannot be run changes nothing.
    pub fn after(&self, transition: &Transition) -> Result<Game, Rejection> {
        if transition.is_design() && self.phase != Phase::Design {
            return Err(Rejection::WrongPhase {
                wanted: "before the game starts",
            });
        }
        if !transition.is_design() && self.phase != Phase::Play {
            return Err(Rejection::WrongPhase {
                wanted: "once the game has started",
            });
        }

        let mut next = self.clone();
        match transition {
            Transition::CreatePlanet {
                territories,
                adjacency,
            } => {
                if !next.territories.is_empty() {
                    return Err(Rejection::PlanetAlreadyCreated);
                }
                next.territories = (0..*territories)
                    .map(|at| Territory::empty(TerritoryId::from_index(at)))
                    .collect();
                next.adjacency = adjacency.clone();
            }
            Transition::AddNode {
                territory,
                resource,
                density,
            } => {
                next.territory_mut(*territory)?.nodes.push(Node {
                    resource: *resource,
                    density: *density,
                });
            }
            Transition::SetForceOfNature { territory, force } => {
                next.territory_mut(*territory)?.force_of_nature = *force;
            }
            Transition::AddUnitToOrbit { kind } => {
                let id = UnitId(next.units.len() as u32 + 1);
                next.units.push(Unit::new(id, *kind));
            }
            Transition::Start => {
                next.phase = Phase::Play;
                next.turn = 1;
            }

            Transition::Land { kind, territory } => next.land(*kind, *territory)?,
            Transition::Launch { kind } => next.launch(*kind)?,
            Transition::Move { kind, territory } => next.move_unit(*kind, *territory)?,
            Transition::Build {
                structure,
                territory,
                resource,
            } => next.build(*structure, *territory, *resource)?,
            Transition::Produce { kind, territory } => next.produce(*kind, *territory)?,
            Transition::Work {
                count,
                structure,
                territory,
                resource,
            } => next.work(*count, *structure, *territory, *resource)?,
            Transition::EndTurn => next.end_turn(),
        }
        Ok(next)
    }

    /// Applies a whole list, stopping at the first that cannot be run.
    pub fn after_all(&self, transitions: &[Transition]) -> Result<Game, (Rejection, usize)> {
        let mut game = self.clone();
        for (at, transition) in transitions.iter().enumerate() {
            game = game.after(transition).map_err(|why| (why, at))?;
        }
        Ok(game)
    }

    // -- reading ------------------------------------------------------------

    pub fn territory(&self, id: TerritoryId) -> Result<&Territory, Rejection> {
        self.territories
            .get(id.index())
            .ok_or(Rejection::NoSuchTerritory(id))
    }

    fn territory_mut(&mut self, id: TerritoryId) -> Result<&mut Territory, Rejection> {
        self.territories
            .get_mut(id.index())
            .ok_or(Rejection::NoSuchTerritory(id))
    }

    pub fn units_on(&self, id: TerritoryId) -> Vec<&Unit> {
        self.units.iter().filter(|unit| unit.is_on(id)).collect()
    }

    pub fn units_in_orbit(&self) -> Vec<&Unit> {
        self.units.iter().filter(|unit| unit.in_orbit()).collect()
    }

    /// All the force present in a territory.
    ///
    /// `spec/control.md`: organised force sums and unorganised force is the highest
    /// present, and coordination comes either from a structure or from a military unit,
    /// which carries it. So a garrison or any usable unit makes the total; with neither,
    /// what is presented is the largest single contribution.
    pub fn force_in(&self, id: TerritoryId) -> u32 {
        let Ok(territory) = self.territory(id) else {
            return 0;
        };
        let units: Vec<u32> = self
            .units_on(id)
            .into_iter()
            .map(|unit| unit.force())
            .collect();
        let coordinated = territory.garrison.is_some() || units.iter().any(|force| *force > 0);
        if coordinated {
            territory.held_force() + units.iter().sum::<u32>()
        } else {
            territory
                .held_force()
                .max(units.into_iter().max().unwrap_or(0))
        }
    }

    /// Whoever is not you holds a territory with this much force. Nature's, until a
    /// territory is founded.
    fn defending_force(&self, id: TerritoryId) -> u32 {
        let Ok(territory) = self.territory(id) else {
            return 0;
        };
        if territory.founded {
            self.force_in(id)
        } else {
            territory.force_of_nature
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
            .filter(|territory| territory.founded)
            .map(|territory| territory.id)
            .collect()
    }

    /// `spec/control.md`: a player has lost when they have no citizens and nothing that
    /// converts into a citizen.
    pub fn has_lost(&self) -> bool {
        self.phase == Phase::Play
            && self.territories.iter().all(|t| t.citizens == 0)
            && !self.units.iter().any(|unit| unit.usable)
    }

    // -- acting -------------------------------------------------------------

    /// The lowest-numbered unit of a kind that satisfies a condition.
    ///
    /// Lowest-numbered so the choice is data-derived rather than an accident of iteration
    /// order - `docs/architecture.md` rule 9. A command names a unit by its kind, so when
    /// several could act, one has to be picked and which one must be predictable.
    fn pick(&self, kind: UnitKind, mut fit: impl FnMut(&Unit) -> bool) -> Option<usize> {
        let mut best: Option<(UnitId, usize)> = None;
        for (at, unit) in self.units.iter().enumerate() {
            if unit.kind == kind && unit.ready() && fit(unit) {
                match best {
                    Some((id, _)) if id <= unit.id => {}
                    _ => best = Some((unit.id, at)),
                }
            }
        }
        best.map(|(_, at)| at)
    }

    fn land(&mut self, kind: UnitKind, territory: TerritoryId) -> Result<(), Rejection> {
        if !kind.lands_from_orbit() {
            return Err(Rejection::CannotLand(kind));
        }
        self.territory(territory)?;
        let at = self
            .pick(kind, |unit| unit.in_orbit())
            .ok_or(Rejection::NoUnitAvailable {
                kind,
                where_from: "in orbit",
            })?;

        self.take(territory, kind.force())?;
        self.units[at].location = Location::On(territory);
        self.units[at].spent = true;
        Ok(())
    }

    fn launch(&mut self, kind: UnitKind) -> Result<(), Rejection> {
        let at = self
            .pick(kind, |unit| !unit.in_orbit())
            .ok_or(Rejection::NotOnThePlanet(kind))?;
        self.units[at].location = Location::Orbit;
        self.units[at].spent = true;
        Ok(())
    }

    fn move_unit(&mut self, kind: UnitKind, territory: TerritoryId) -> Result<(), Rejection> {
        self.territory(territory)?;
        // A unit must be next door with a cell left. Adjacency is checked while choosing
        // so that "no pioneer can reach there" and "there is no pioneer" stay different
        // complaints.
        let anywhere = self.pick(kind, |unit| !unit.in_orbit());
        let at = self
            .pick(kind, |unit| match unit.location {
                Location::On(from) => {
                    unit.cells >= cost::MOVE_CELLS && self.are_adjacent(from, territory)
                }
                Location::Orbit => false,
            })
            .ok_or_else(|| match anywhere {
                Some(other) => match self.units[other].location {
                    Location::On(from) if !self.are_adjacent(from, territory) => {
                        Rejection::NotAdjacent {
                            from,
                            to: territory,
                        }
                    }
                    _ => Rejection::NoCells(kind),
                },
                None => Rejection::NoUnitAvailable {
                    kind,
                    where_from: "on the planet",
                },
            })?;

        if !self.territory(territory)?.founded {
            self.take(territory, kind.force())?;
        }
        self.units[at].location = Location::On(territory);
        self.units[at].cells -= cost::MOVE_CELLS;
        self.units[at].spent = true;
        Ok(())
    }

    /// Takes a territory with a unit of this force, founding it.
    ///
    /// `spec/control.md`: taking a territory takes force greater than the existing force.
    fn take(&mut self, territory: TerritoryId, force: u32) -> Result<(), Rejection> {
        let defending = self.defending_force(territory);
        if self.territory(territory)?.founded {
            return Err(Rejection::AlreadyControlled(territory));
        }
        if force <= defending {
            return Err(Rejection::NotEnoughForce {
                territory,
                force,
                needed: defending,
            });
        }
        self.territory_mut(territory)?.founded = true;
        Ok(())
    }

    fn build(
        &mut self,
        structure: StructureKind,
        territory: TerritoryId,
        resource: Option<Resource>,
    ) -> Result<(), Rejection> {
        if !self.territory(territory)?.founded {
            return Err(Rejection::NotControlled(territory));
        }
        match structure {
            StructureKind::Garrison => Err(Rejection::GarrisonIsNotBuilt),
            StructureKind::Yard => {
                self.spend(territory, Resource::Metal, cost::YARD_METAL)?;
                self.territory_mut(territory)?.yards += 1;
                Ok(())
            }
            StructureKind::Extractor => {
                let resource = resource.ok_or(Rejection::ResourceNotNamed(structure))?;
                let node = self.territory(territory)?.best_free_node(resource).ok_or(
                    Rejection::NoFreeNode {
                        territory,
                        resource,
                    },
                )?;
                self.spend_labor(territory, cost::EXTRACTOR_LABOR)?;
                self.territory_mut(territory)?
                    .extractors
                    .push(Extractor { node, spent: false });
                Ok(())
            }
        }
    }

    fn produce(&mut self, kind: UnitKind, territory: TerritoryId) -> Result<(), Rejection> {
        let place = self.territory(territory)?;
        if !place.founded {
            return Err(Rejection::NotControlled(territory));
        }
        match kind {
            UnitKind::Ark => {
                if place.yards == 0 {
                    return Err(Rejection::NoYard(territory));
                }
                self.spend(territory, Resource::Metal, cost::ARK_METAL)?;
                self.spend(territory, Resource::Energy, cost::ARK_ENERGY)?;
            }
            UnitKind::Pioneer => {
                // `releases/first-release.md`: a garrison allows create pioneer.
                if place.garrison.is_none() {
                    return Err(Rejection::NoGarrison(territory));
                }
                if place.citizens < cost::PIONEER_CITIZENS {
                    return Err(Rejection::NotEnoughCitizens {
                        territory,
                        held: place.citizens,
                        needed: cost::PIONEER_CITIZENS,
                    });
                }
                self.spend(territory, Resource::Metal, cost::PIONEER_METAL)?;
                self.spend(territory, Resource::Energy, cost::PIONEER_ENERGY)?;
                self.territory_mut(territory)?.citizens -= cost::PIONEER_CITIZENS;
            }
        }
        let id = UnitId(self.units.len() as u32 + 1);
        let mut unit = Unit::new(id, kind);
        unit.location = Location::On(territory);
        self.units.push(unit);
        Ok(())
    }

    fn work(
        &mut self,
        count: u32,
        structure: StructureKind,
        territory: TerritoryId,
        resource: Option<Resource>,
    ) -> Result<(), Rejection> {
        if !self.territory(territory)?.founded {
            return Err(Rejection::NotControlled(territory));
        }
        match structure {
            StructureKind::Garrison => {
                if self.territory(territory)?.garrison.is_none() {
                    return Err(Rejection::NothingToWorkAt {
                        territory,
                        structure,
                    });
                }
                self.spend_labor(territory, count)?;
                if let Some(garrison) = &mut self.territory_mut(territory)?.garrison {
                    garrison.manned += count;
                }
                Ok(())
            }
            StructureKind::Extractor => {
                let resource = resource.ok_or(Rejection::ResourceNotNamed(structure))?;
                let ready: Vec<usize> = self
                    .territory(territory)?
                    .extractors_for(resource)
                    .into_iter()
                    .filter(|at| !self.territories[territory.index()].extractors[*at].spent)
                    .collect();
                if ready.is_empty() {
                    return Err(Rejection::NothingToWorkAt {
                        territory,
                        structure,
                    });
                }
                if (ready.len() as u32) < count {
                    return Err(Rejection::NotThatManyToWork {
                        territory,
                        structure,
                        available: ready.len() as u32,
                        asked: count,
                    });
                }
                self.spend_labor(territory, count)?;
                // Densest first, so asking for fewer than every extractor gets the best
                // of them - and so the answer never depends on iteration order.
                let mut by_density: Vec<(u32, usize)> = ready
                    .into_iter()
                    .map(|at| {
                        let place = &self.territories[territory.index()];
                        (place.nodes[place.extractors[at].node].density, at)
                    })
                    .collect();
                by_density.sort_by_key(|(density, at)| (std::cmp::Reverse(*density), *at));

                let mut produced = 0;
                for (density, at) in by_density.into_iter().take(count as usize) {
                    self.territories[territory.index()].extractors[at].spent = true;
                    produced += density;
                }
                self.territory_mut(territory)?.add(resource, produced);
                Ok(())
            }
            // A yard is what allows an Ark to be produced; producing is `produce`, and
            // there is no separate labor step. `spec/invariants.md` forbids a step that
            // is always taken, and working a yard before producing would be one.
            StructureKind::Yard => Err(Rejection::NothingToWorkAt {
                territory,
                structure,
            }),
        }
    }

    fn spend(
        &mut self,
        territory: TerritoryId,
        resource: Resource,
        amount: u32,
    ) -> Result<(), Rejection> {
        let held = self.territory(territory)?.store(resource);
        if held < amount {
            return Err(Rejection::NotEnoughResource {
                territory,
                resource,
                held,
                needed: amount,
            });
        }
        self.territory_mut(territory)?.take(resource, amount);
        Ok(())
    }

    fn spend_labor(&mut self, territory: TerritoryId, amount: u32) -> Result<(), Rejection> {
        let available = self.territory(territory)?.labor_available();
        if available < amount {
            return Err(Rejection::NotEnoughLabor {
                territory,
                available,
                needed: amount,
            });
        }
        self.territory_mut(territory)?.labor_spent += amount;
        Ok(())
    }

    // -- ending a turn ------------------------------------------------------

    /// Consume, transform, discard, unspend.
    ///
    /// Every territory is settled independently. In this release nothing crosses a
    /// boundary, so no territory can affect another's outcome and the order they are
    /// taken in cannot change the result - which is what
    /// `docs/architecture.md` rule 9 asks for, and what would make this safe to run in
    /// parallel unchanged.
    fn end_turn(&mut self) {
        let ids: Vec<TerritoryId> = self.territories.iter().map(|t| t.id).collect();
        for id in ids {
            self.settle(id);
        }

        // Nature reclaims anything no longer held. Done after every territory has
        // settled, because whether force is enough depends on what settling left behind.
        let ids: Vec<TerritoryId> = self.controlled();
        for id in ids {
            let needed = self.territory(id).map(|t| t.force_of_nature).unwrap_or(0);
            if self.force_in(id) < needed {
                for unit in &mut self.units {
                    if unit.is_on(id) {
                        unit.usable = false;
                    }
                }
                if let Ok(territory) = self.territory_mut(id) {
                    territory.lost_to_nature();
                }
            }
        }

        for unit in &mut self.units {
            unit.spent = false;
        }
        self.turn += 1;
    }

    /// One territory's end of turn.
    fn settle(&mut self, id: TerritoryId) {
        // Everything that eats, eats. A unit that is not paid is lost.
        //
        // Except a unit that is about to be consumed. A founding unit arriving on ground
        // it has just taken transforms into what that territory needs - and a territory
        // taken this turn has no extractor yet, so charging it a meal first would starve
        // every pioneer on arrival and make founding by land impossible. `spec/console.md`
        // has ending a turn *consume, transform*; what the transform consumes cannot also
        // be asked to eat.
        let transforming = self.about_to_transform(id);
        let mut starved: Vec<UnitId> = Vec::new();
        let mut owed = 0;
        for unit in self
            .units
            .iter()
            .filter(|unit| unit.is_on(id) && unit.usable && !transforming.contains(&unit.id))
        {
            owed += unit.kind.upkeep();
        }
        if owed > 0 {
            let food = self.territories[id.index()].store(Resource::Food);
            if food >= owed {
                self.territories[id.index()].take(Resource::Food, owed);
            } else {
                // Not enough to go round: the units are lost, lowest id last, and what
                // food there is goes with them.
                self.territories[id.index()].take(Resource::Food, food);
                let mut by_id: Vec<(UnitId, usize)> = self
                    .units
                    .iter()
                    .enumerate()
                    .filter(|(_, unit)| {
                        unit.is_on(id)
                            && unit.usable
                            && unit.kind.upkeep() > 0
                            && !transforming.contains(&unit.id)
                    })
                    .map(|(at, unit)| (unit.id, at))
                    .collect();
                by_id.sort_by_key(|(unit_id, _)| std::cmp::Reverse(*unit_id));
                let mut short = owed - food;
                for (unit_id, at) in by_id {
                    if short == 0 {
                        break;
                    }
                    short = short.saturating_sub(self.units[at].kind.upkeep());
                    self.units[at].usable = false;
                    starved.push(unit_id);
                }
            }
        }

        // Then a population grows on surplus food, or starves for want of it.
        let food = self.territories[id.index()].store(Resource::Food);
        let citizens = self.territories[id.index()].citizens;
        self.territories[id.index()].citizens = population_after(citizens, food);

        // Then whatever can transform, transforms. After the population step rather than
        // before it, so the citizen a founding unit produces is not immediately asked to
        // feed itself from a territory that has not extracted anything yet.
        self.transform_founding_units(id);

        // Unused resources are discarded, and everything becomes unspent again.
        self.territories[id.index()].stores = [0; 3];
        self.territories[id.index()].unspend();
    }

    /// Which unit on this territory a transform is about to consume, if any.
    ///
    /// At most one: a territory takes at most one garrison, and the transform stops as
    /// soon as there is one.
    fn about_to_transform(&self, id: TerritoryId) -> Vec<UnitId> {
        let Ok(place) = self.territory(id) else {
            return Vec::new();
        };
        if !place.founded || place.garrison.is_some() {
            return Vec::new();
        }
        self.units
            .iter()
            .filter(|unit| unit.is_on(id) && unit.usable)
            .min_by_key(|unit| unit.id)
            .map(|unit| vec![unit.id])
            .unwrap_or_default()
    }

    /// `releases/first-release.md`: an Ark or Pioneer transform consumes the unit and
    /// produces a garrison, a citizen and a food extractor.
    ///
    /// Only where the territory has no garrison yet. A founding unit becomes what a
    /// territory needs to sustain itself, and a territory that already has a garrison does
    /// not need another - `spec/control.md` allows it only one.
    fn transform_founding_units(&mut self, id: TerritoryId) {
        if !self.territories[id.index()].founded {
            return;
        }
        loop {
            if self.territories[id.index()].garrison.is_some() {
                return;
            }
            let Some(at) = self
                .units
                .iter()
                .enumerate()
                .filter(|(_, unit)| unit.is_on(id) && unit.usable)
                .min_by_key(|(_, unit)| unit.id)
                .map(|(at, _)| at)
            else {
                return;
            };

            let force = self.units[at].kind.force();
            self.units.remove(at);

            let place = &mut self.territories[id.index()];
            place.garrison = Some(Garrison::from_founding_unit(force));
            place.citizens += 1;
            if let Some(node) = place.best_free_node(Resource::Food) {
                place.extractors.push(Extractor { node, spent: false });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    /// A designed world: three territories with food and metal, one ark in orbit.
    fn designed() -> Game {
        let mut game = Game::new()
            .after(&Transition::CreatePlanet {
                territories: 3,
                adjacency: ring(3),
            })
            .unwrap();
        for at in 1..=3u32 {
            for _ in 0..3 {
                game = game
                    .after(&Transition::AddNode {
                        territory: TerritoryId(at),
                        resource: Resource::Food,
                        density: 4,
                    })
                    .unwrap();
                game = game
                    .after(&Transition::AddNode {
                        territory: TerritoryId(at),
                        resource: Resource::Metal,
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
        })
        .unwrap()
    }

    fn started() -> Game {
        designed().after(&Transition::Start).unwrap()
    }

    /// A landed and transformed ark: one garrison, one citizen, one food extractor.
    fn founded() -> Game {
        started()
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            })
            .unwrap()
            .after(&Transition::EndTurn)
            .unwrap()
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
        assert!(!before.territory(TerritoryId(1)).unwrap().founded);
        assert!(after.territory(TerritoryId(1)).unwrap().founded);
    }

    /// A game is exactly the result of applying every transition in order to the start.
    #[test]
    fn the_same_transitions_always_produce_the_same_game() {
        let script = vec![
            Transition::CreatePlanet {
                territories: 3,
                adjacency: ring(3),
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
            Transition::AddUnitToOrbit {
                kind: UnitKind::Ark,
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
    fn landing_an_ark_founds_the_territory() {
        let game = started()
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            })
            .unwrap();
        assert!(game.territory(TerritoryId(1)).unwrap().founded);
        assert!(game.units[0].is_on(TerritoryId(1)));
    }

    /// Taking a territory takes force *greater* than what holds it. An ark is force 2,
    /// so a force of nature of 5 is out of reach.
    #[test]
    fn a_territory_too_strong_to_take_is_refused_in_the_games_terms() {
        let strong = Game::new()
            .after(&Transition::CreatePlanet {
                territories: 1,
                adjacency: vec![vec![]],
            })
            .unwrap()
            .after(&Transition::SetForceOfNature {
                territory: TerritoryId(1),
                force: 5,
            })
            .unwrap()
            .after(&Transition::AddUnitToOrbit {
                kind: UnitKind::Ark,
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

    /// The transform: consume the unit, produce a garrison, a citizen and a food
    /// extractor.
    #[test]
    fn a_founding_unit_becomes_a_garrison_a_citizen_and_a_food_extractor() {
        let game = founded();
        let place = game.territory(TerritoryId(1)).unwrap();
        assert!(
            place.garrison.is_some(),
            "a structure that holds the ground"
        );
        assert_eq!(place.citizens, 1);
        assert_eq!(place.extractors.len(), 1);
        assert_eq!(
            place.nodes[place.extractors[0].node].resource,
            Resource::Food,
            "working a food node"
        );
        assert!(game.units.is_empty(), "the ark was consumed");
    }

    /// Force of nature 1, garrison force 1: equal is enough to hold.
    #[test]
    fn a_garrison_holds_a_territory_against_its_force_of_nature() {
        let game = founded().after(&Transition::EndTurn).unwrap();
        assert!(
            game.territory(TerritoryId(1)).unwrap().founded,
            "still held a turn later"
        );
    }

    #[test]
    fn working_an_extractor_produces_its_nodes_density() {
        let game = founded()
            .after(&Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Food),
            })
            .unwrap();
        let place = game.territory(TerritoryId(1)).unwrap();
        assert_eq!(place.store(Resource::Food), 4, "the node's density");
        assert_eq!(place.labor_available(), 0, "the citizen's labor is spent");
    }

    /// Once per turn: an extractor already worked cannot be worked again.
    #[test]
    fn an_extractor_works_only_once_a_turn() {
        let game = founded()
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
        let game = founded();
        let rejected = game
            .after(&Transition::Build {
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Metal),
            })
            .unwrap()
            .after(&Transition::Build {
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Metal),
            })
            .unwrap_err();
        assert!(
            matches!(rejected, Rejection::NotEnoughLabor { .. }),
            "{rejected}"
        );
    }

    #[test]
    fn a_population_grows_on_the_food_it_gathered() {
        let game = founded()
            .after(&Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Food),
            })
            .unwrap()
            .after(&Transition::EndTurn)
            .unwrap();
        // One citizen and four food: one spare feeds one new, at most doubling.
        assert_eq!(game.territory(TerritoryId(1)).unwrap().citizens, 2);
    }

    /// `spec/turn.md`: unused resources are discarded.
    #[test]
    fn resources_left_at_the_end_of_a_turn_are_discarded() {
        let game = founded()
            .after(&Transition::Build {
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Metal),
            })
            .unwrap();
        let game = game
            .after(&Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Metal),
            })
            .unwrap_or(game);
        let after = game.after(&Transition::EndTurn).unwrap();
        assert_eq!(
            after
                .territory(TerritoryId(1))
                .unwrap()
                .store(Resource::Metal),
            0
        );
    }

    /// The rule that makes ending a turn safe to parallelise: no territory can affect
    /// another's outcome, so the order they settle in cannot change the result.
    #[test]
    fn settling_territories_in_any_order_gives_the_same_game() {
        let game = founded();
        let forwards = game.clone().after(&Transition::EndTurn).unwrap();

        let mut backwards = game;
        let ids: Vec<TerritoryId> = backwards.territories.iter().map(|t| t.id).rev().collect();
        for id in ids {
            backwards.settle(id);
        }
        for unit in &mut backwards.units {
            unit.spent = false;
        }
        backwards.turn += 1;

        assert_eq!(forwards.territories, backwards.territories);
    }

    #[test]
    fn a_yard_costs_metal_that_has_to_be_there() {
        let rejected = founded()
            .after(&Transition::Build {
                structure: StructureKind::Yard,
                territory: TerritoryId(1),
                resource: None,
            })
            .unwrap_err();
        assert_eq!(
            rejected,
            Rejection::NotEnoughResource {
                territory: TerritoryId(1),
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
        let rejected = founded()
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
            .after(&Transition::Produce {
                kind: UnitKind::Pioneer,
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
        let mut pioneer = Unit::new(id, UnitKind::Pioneer);
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
                territory: TerritoryId(3),
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

    #[test]
    fn a_move_spends_a_cell_and_a_unit_out_of_cells_cannot_move() {
        let mut game = founded();
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer);
        pioneer.location = Location::On(TerritoryId(1));
        pioneer.cells = 1;
        game.units.push(pioneer);

        let moved = game
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                territory: TerritoryId(2),
            })
            .unwrap();
        let pioneer = moved
            .units
            .iter()
            .find(|u| u.kind == UnitKind::Pioneer)
            .unwrap();
        assert_eq!(pioneer.cells, 0);
        assert!(
            moved.territory(TerritoryId(2)).unwrap().founded,
            "and founds it"
        );

        // Next turn it has no cells left.
        let stuck = moved.after(&Transition::EndTurn).unwrap();
        let rejected = stuck
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                territory: TerritoryId(3),
            })
            .unwrap_err();
        assert!(
            matches!(
                rejected,
                Rejection::NoCells(_) | Rejection::NoUnitAvailable { .. }
            ),
            "{rejected}"
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

    #[test]
    fn a_pioneer_that_is_not_fed_is_lost() {
        let mut game = founded();
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer);
        pioneer.location = Location::On(TerritoryId(2));
        game.units.push(pioneer);
        game.territories[1].founded = true;

        let after = game.after(&Transition::EndTurn).unwrap();
        let pioneer = after.units.iter().find(|u| u.kind == UnitKind::Pioneer);
        assert!(
            pioneer.map(|unit| !unit.usable).unwrap_or(true),
            "with no food it is lost"
        );
    }
}
