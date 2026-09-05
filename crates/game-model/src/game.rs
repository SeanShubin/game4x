//! The state, and the one function that changes it.

use crate::identity::{Resource, StructureKind, TerritoryId, UnitId, UnitKind};
use crate::rejection::Rejection;
use crate::territory::{Extractor, Garrison, Node, Territory, population_after};
use crate::thing::Kind;
use crate::transition::Transition;
use crate::unit::{Location, Unit};

/// Costs, from `releases/first-release.md`.
///
/// Gathered here rather than scattered because they are release tuning rather than rules:
/// `spec/README.md` keeps relationships in the specification and numbers in a release, so
/// these are the numbers and they are meant to move without any rule moving with them.
pub mod cost {
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
    pub const PIONEER_ENERGY: u32 = 6;
    pub const PIONEER_CITIZENS: u32 = 2;
    /// An Extractor costs 1 labor and 1 metal.
    ///
    /// The metal is new: `P-152` conserves it, so a thing that can be taken apart for metal
    /// has to have had metal put into it.
    pub const EXTRACTOR_LABOR: u32 = 1;
    pub const EXTRACTOR_METAL: u32 = 1;
    /// A Garrison costs 1 labor and 1 metal, for the same reason.
    pub const GARRISON_LABOR: u32 = 1;
    pub const GARRISON_METAL: u32 = 1;
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
    /// Whether this game has been won.
    ///
    /// State rather than a question asked later, because winning happens at a *moment*:
    /// `spec/control.md` says a player wins by launching an Ark from a fully exploited
    /// planet, and once the Ark is in orbit the launch is over. Recomputing it afterwards
    /// would ask whether the planet is fully exploited *now*, which is a different
    /// question and would keep answering yes long after nobody launched anything.
    pub won: bool,
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
            won: false,
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
                biomes,
            } => {
                if !next.territories.is_empty() {
                    return Err(Rejection::PlanetAlreadyCreated);
                }
                if biomes.len() != *territories {
                    return Err(Rejection::BiomesDoNotCoverThePlanet {
                        territories: *territories,
                        biomes: biomes.len(),
                    });
                }
                next.territories = biomes
                    .iter()
                    .enumerate()
                    .map(|(at, biome)| Territory::empty(TerritoryId::from_index(at), *biome))
                    .collect();
                next.adjacency = adjacency.clone();
            }
            Transition::SetResource {
                territory,
                resource,
                extractors,
                density,
            } => {
                // Room for `extractors`, each yielding `density`, held for now as that many
                // identical nodes. The release gives one density per territory per resource,
                // so the two are the same fact written differently - and `P-134` replaces
                // this representation entirely, which is why it is not rebuilt here.
                let place = next.territory_mut(*territory)?;
                place.nodes.retain(|node| node.resource != *resource);
                for _ in 0..*extractors {
                    place.nodes.push(Node {
                        resource: *resource,
                        density: *density,
                    });
                }
            }
            Transition::SetForceOfNature { territory, force } => {
                next.territory_mut(*territory)?.force_of_nature = *force;
            }
            Transition::SetBiome { territory, biome } => {
                next.territory_mut(*territory)?.biome = *biome;
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
            Transition::CreateLabor { count, territory } => {
                next.create_labor(*count, *territory)?
            }
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
        if territory.founded() {
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
            .filter(|territory| territory.founded())
            .map(|territory| territory.id)
            .collect()
    }

    /// `spec/control.md`: a player has lost when they have no citizens and nothing that
    /// converts into a citizen.
    /// Whether this game has been won. `spec/control.md` gives exactly one way.
    pub fn has_won(&self) -> bool {
        self.won
    }

    pub fn has_lost(&self) -> bool {
        self.phase == Phase::Play
            && self.territories.iter().all(|t| t.citizens() == 0)
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

        // Landing takes the territory and takes the Ark apart, in one action.
        // `spec/unit-types.md` says *a unit may be taken apart into what a territory needs
        // to sustain itself* and that *an Ark is taken apart on arriving from orbit*, and
        // `spec/invariants.md` forbids an intermediate step that is always taken.
        //
        // P-137 purged the word founding, and this quotation was of the wording it purged.
        // The code still calls it `found`; renaming that is a change to the model rather
        // than to a comment, and the model is being rewritten for P-134 anyway.
        self.found(territory, at, &[Resource::Food, Resource::Metal], 2)
    }

    fn launch(&mut self, kind: UnitKind) -> Result<(), Rejection> {
        let at = self
            .pick(kind, |unit| !unit.in_orbit())
            .ok_or(Rejection::NotOnThePlanet(kind))?;
        // `spec/control.md`: *a player wins by launching an Ark from a fully exploited
        // planet.* Asked before the Ark leaves, because it is the planet it left that has
        // to have been finished.
        if kind == UnitKind::Ark && self.is_fully_exploited() {
            self.won = true;
        }
        self.units[at].location = Location::Orbit;
        self.units[at].exhausted = true;
        Ok(())
    }

    /// `spec/control.md`: *a planet is fully exploited when every territory that can be
    /// taken has been taken, every structure has been built everywhere it can be built, and
    /// every storage structure on it is full.*
    ///
    /// **This does not implement that sentence yet, and the difference matters.** It asks
    /// for a Yard in every claimable territory; the specification asks for one everywhere a
    /// Yard *can* be built, which `spec/control.md` defines as *where the territory's own
    /// permanent facts allow it: its nodes, their densities, its biome. Not whether the
    /// player can afford it this turn, and not whether any particular game happened to
    /// reach it.*
    ///
    /// Eight of the release's twelve territories can never hold a Yard - one of them has no
    /// metal at all - so as written this can never be true. That is `C-7`, and the
    /// specification moved to settle it; this function has not caught up. `C-9`.
    ///
    /// Nothing in play depends on the difference, for a worse reason: `C-8`. An Ark can
    /// only be produced in territory 11, which can never be claimed, so no Ark ever reaches
    /// the planet to be launched from it.
    ///
    /// *Every storage structure is full* holds because there are none. No structure in
    /// `spec/structures.md` stores anything. If one is ever added, this stops being vacuous
    /// and this function will not notice on its own.
    pub fn is_fully_exploited(&self) -> bool {
        self.territories
            .iter()
            .filter(|place| place.biome.is_claimable())
            .all(|place| {
                place.founded() && place.yards() > 0 && place.extractors.len() == place.nodes.len()
            })
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

        self.units[at].cells -= cost::MOVE_CELLS;
        if self.territory(territory)?.founded() {
            self.units[at].location = Location::On(territory);
            self.units[at].exhausted = true;
            return Ok(());
        }
        // Unclaimed ground is taken and founded by arriving on it, which consumes the
        // unit. So a founding unit never stands on ground it has taken but not founded,
        // and never has to be fed there.
        self.found(territory, at, &[Resource::Food, Resource::Metal], 2)
    }

    /// Takes a territory and founds it with a unit, which the founding consumes.
    ///
    /// `releases/first-release.md`, Founding: produces garrison, citizen, food extractor.
    /// All of it happens here, at the moment of arriving, because the specification
    /// describes taking and transforming as one act.
    /// Taking a territory, and taking the unit apart into what it leaves behind.
    ///
    /// **What a claiming unit leaves is the release's, and it has moved twice.** Both
    /// recipes now leave two citizens and two extractors, for food and for metal - a
    /// landing used to leave one citizen and three extractors, and a Pioneer one and one.
    ///
    /// The metal extractor is what makes either survivable: an extractor costs metal, so
    /// ground that arrived with only a farm could never build a second thing. Taking the
    /// count and the resources as arguments keeps that a fact about the recipe rather than
    /// about this function.
    fn found(
        &mut self,
        territory: TerritoryId,
        unit_at: usize,
        leaves: &[Resource],
        citizens: u32,
    ) -> Result<(), Rejection> {
        // `spec/planet.md`: no territory can be claimed whose biome is ocean. Asked before
        // the force is compared, so the answer says what is actually wrong - being at sea
        // is not a matter of not having brought enough.
        if !self.territory(territory)?.biome.is_claimable() {
            return Err(Rejection::CannotClaimOcean(territory));
        }
        let force = self.units[unit_at].kind.force();
        self.take(territory, force)?;
        self.units.remove(unit_at);

        let place = &mut self.territories[territory.index()];
        // `spec/unit-types.md`: the structure a founding unit becomes has one less force
        // than the unit. `spec/control.md`: founding is a garrison's only source.
        place.garrison = Some(Garrison::from_founding_unit(force));
        place.put(Kind::Citizen, citizens);
        for resource in leaves {
            if let Some(node) = place.best_free_node(*resource) {
                place.extractors.push(Extractor {
                    node,
                    exhausted: false,
                });
            }
        }
        Ok(())
    }

    /// Takes a territory with a unit of this force, founding it.
    ///
    /// `spec/control.md`: taking a territory takes force greater than the existing force.
    fn take(&mut self, territory: TerritoryId, force: u32) -> Result<(), Rejection> {
        let defending = self.defending_force(territory);
        if self.territory(territory)?.founded() {
            return Err(Rejection::AlreadyControlled(territory));
        }
        if force <= defending {
            return Err(Rejection::NotEnoughForce {
                territory,
                force,
                needed: defending,
            });
        }
        Ok(())
    }

    fn build(
        &mut self,
        structure: StructureKind,
        territory: TerritoryId,
        resource: Option<Resource>,
    ) -> Result<(), Rejection> {
        if !self.territory(territory)?.founded() {
            return Err(Rejection::NotControlled(territory));
        }
        match structure {
            StructureKind::Garrison => Err(Rejection::GarrisonIsNotBuilt),
            StructureKind::Yard => {
                // Labor first, so a territory with the metal and no hands is refused for
                // the reason that is true rather than for the one asked about second.
                self.spend_labor(territory, cost::YARD_LABOR)?;
                self.spend(territory, Resource::Metal, cost::YARD_METAL)?;
                self.territory_mut(territory)?.put(Kind::Yard, 1);
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
                self.spend(territory, Resource::Metal, cost::EXTRACTOR_METAL)?;
                self.territory_mut(territory)?.extractors.push(Extractor {
                    node,
                    exhausted: false,
                });
                Ok(())
            }
        }
    }

    fn produce(&mut self, kind: UnitKind, territory: TerritoryId) -> Result<(), Rejection> {
        let place = self.territory(territory)?;
        if !place.founded() {
            return Err(Rejection::NotControlled(territory));
        }
        match kind {
            UnitKind::Ark => {
                if place.yards() == 0 {
                    return Err(Rejection::NoYard(territory));
                }
                if place.citizens() < cost::ARK_CITIZENS {
                    return Err(Rejection::NotEnoughCitizens {
                        territory,
                        held: place.citizens(),
                        needed: cost::ARK_CITIZENS,
                    });
                }
                self.spend(territory, Resource::Metal, cost::ARK_METAL)?;
                self.spend(territory, Resource::Energy, cost::ARK_ENERGY)?;
                self.territory_mut(territory)?
                    .remove(Kind::Citizen, cost::ARK_CITIZENS);
            }
            UnitKind::Pioneer => {
                // A garrison is no longer required. It was, and the release's Requires
                // column is empty for a Pioneer now - what a Pioneer needs is metal,
                // energy and the people who go with it.
                if place.citizens() < cost::PIONEER_CITIZENS {
                    return Err(Rejection::NotEnoughCitizens {
                        territory,
                        held: place.citizens(),
                        needed: cost::PIONEER_CITIZENS,
                    });
                }
                self.spend(territory, Resource::Metal, cost::PIONEER_METAL)?;
                self.spend(territory, Resource::Energy, cost::PIONEER_ENERGY)?;
                self.territory_mut(territory)?
                    .remove(Kind::Citizen, cost::PIONEER_CITIZENS);
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
        if !self.territory(territory)?.founded() {
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
                    .filter(|at| !self.territories[territory.index()].extractors[*at].exhausted)
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
                    self.territories[territory.index()].extractors[at].exhausted = true;
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

    /// `create labor`: turn ready citizens into labor.
    ///
    /// **`P-232`, choice 2.** The release's recipe consumes a *citizen, ready* and produces
    /// a *citizen, exhausted* and a *labor*. That is exactly this, and until now `work` and
    /// `build` did it implicitly - which is why nothing in a command list said a citizen had
    /// been spent, and why the definitions and the commands were not enough to derive the
    /// dump by hand.
    fn create_labor(&mut self, count: u32, territory: TerritoryId) -> Result<(), Rejection> {
        let place = self.territory_mut(territory)?;
        let ready = place.labor_available();
        if ready < count {
            return Err(Rejection::NotEnoughLabor {
                territory,
                needed: count,
                available: ready,
            });
        }
        place.spend_labor(count);
        place.put(Kind::Labor, count);
        Ok(())
    }

    fn spend_labor(&mut self, territory: TerritoryId, amount: u32) -> Result<(), Rejection> {
        // **The labor was made by `create labor` and is here as things.** This used to ask
        // whether there were ready citizens, which was the same question while `work` made
        // its own labor. It is not the same question now: `create labor` exhausts the
        // citizen and leaves the labor, so asking about citizens says *none* at exactly the
        // moment the labor is sitting there.
        //
        // I left the old check above the new one and it fired first. Two checks for one
        // thing, and the wrong one answered - which is `S-34`'s two expectations in
        // miniature, inside a single function.
        let place = self.territory_mut(territory)?;
        let taken = place.remove(Kind::Labor, amount);
        if taken < amount {
            return Err(Rejection::NotEnoughLabor {
                territory,
                needed: amount,
                available: taken,
            });
        }
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
            unit.exhausted = false;
        }
        self.turn += 1;
    }

    /// One territory's end of turn.
    fn settle(&mut self, id: TerritoryId) {
        // Everything that eats, eats. A unit that is not paid is lost.
        //
        let mut starved: Vec<UnitId> = Vec::new();
        let mut owed = 0;
        for unit in self
            .units
            .iter()
            .filter(|unit| unit.is_on(id) && unit.usable)
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
                    .filter(|(_, unit)| unit.is_on(id) && unit.usable && unit.kind.upkeep() > 0)
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
        let citizens = self.territories[id.index()].citizens();
        self.territories[id.index()].set_count(Kind::Citizen, population_after(citizens, food));

        // Unused resources are discarded, and everything becomes ready again. Nothing
        // transforms here: founding happens when a unit arrives, so by the time a turn
        // ends there is never a unit waiting to become something.
        self.territories[id.index()].discard_resources();
        self.territories[id.index()].make_ready();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Biome;

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

    /// `spec/control.md`: *a player wins by launching an Ark from a fully exploited
    /// planet.*
    ///
    /// The whole condition, built by hand: every claimable territory taken, every node
    /// worked, a yard everywhere. Then launching wins, and it is the launch that does it.
    #[test]
    fn launching_an_ark_from_a_finished_planet_wins() {
        let mut game = designed().after(&Transition::Start).unwrap();
        game = game
            .after(&Transition::Land {
                kind: UnitKind::Ark,
                territory: TerritoryId(1),
            })
            .unwrap();
        assert!(
            !game.is_fully_exploited(),
            "one territory of three is not a planet"
        );

        // Finish the planet by hand rather than by playing it, so the test is about the
        // condition rather than about the economy.
        for place in &mut game.territories {
            // A citizen is what holds it, since `S-19` made control derived. Setting a flag
            // beside an empty territory used to do this, which is the disagreement that
            // rule removes.
            place.put(Kind::Citizen, 1);
            place.set_count(Kind::Yard, 1);
            place.extractors = (0..place.nodes.len())
                .map(|node| Extractor {
                    node,
                    exhausted: false,
                })
                .collect();
        }
        assert!(game.is_fully_exploited());
        assert!(!game.has_won(), "nobody has launched anything yet");

        // An Ark has to be on the planet to leave it.
        game.units.push(Unit {
            id: UnitId(99),
            kind: UnitKind::Ark,
            location: Location::On(TerritoryId(1)),
            cells: 2,
            exhausted: false,
            usable: true,
        });
        let won = game
            .after(&Transition::Launch {
                kind: UnitKind::Ark,
            })
            .unwrap();
        assert!(won.has_won(), "the planet was finished and an Ark left it");
    }

    /// Launching off an unfinished planet is just leaving.
    #[test]
    fn launching_from_an_unfinished_planet_wins_nothing() {
        let mut game = designed().after(&Transition::Start).unwrap();
        game.units.push(Unit {
            id: UnitId(99),
            kind: UnitKind::Ark,
            location: Location::On(TerritoryId(1)),
            cells: 2,
            exhausted: false,
            usable: true,
        });
        let after = game
            .after(&Transition::Launch {
                kind: UnitKind::Ark,
            })
            .unwrap();
        assert!(!after.has_won());
    }

    /// Winning is a moment, not a standing condition. Once it has happened it stays
    /// happened, and it does not start being true later because the planet still looks
    /// finished.
    #[test]
    fn winning_is_the_launch_rather_than_the_state_afterwards() {
        let mut game = designed().after(&Transition::Start).unwrap();
        for place in &mut game.territories {
            // A citizen is what holds it, since `S-19` made control derived. Setting a flag
            // beside an empty territory used to do this, which is the disagreement that
            // rule removes.
            place.put(Kind::Citizen, 1);
            place.set_count(Kind::Yard, 1);
            place.extractors = (0..place.nodes.len())
                .map(|node| Extractor {
                    node,
                    exhausted: false,
                })
                .collect();
        }
        // A finished planet nobody has launched from is not a win.
        assert!(game.is_fully_exploited());
        assert!(!game.has_won());

        // And a Pioneer leaving it is not one either.
        game.units.push(Unit {
            id: UnitId(98),
            kind: UnitKind::Pioneer,
            location: Location::On(TerritoryId(1)),
            cells: 2,
            exhausted: false,
            usable: true,
        });
        let after = game
            .after(&Transition::Launch {
                kind: UnitKind::Pioneer,
            })
            .unwrap();
        assert!(!after.has_won(), "only an Ark wins");
    }

    /// An ocean cannot be taken, so it cannot be what stops a planet being finished.
    #[test]
    fn ocean_does_not_keep_a_planet_from_being_finished() {
        let mut game = designed().after(&Transition::Start).unwrap();
        for place in &mut game.territories {
            // A citizen is what holds it, since `S-19` made control derived. Setting a flag
            // beside an empty territory used to do this, which is the disagreement that
            // rule removes.
            place.put(Kind::Citizen, 1);
            place.set_count(Kind::Yard, 1);
            place.extractors = (0..place.nodes.len())
                .map(|node| Extractor {
                    node,
                    exhausted: false,
                })
                .collect();
        }
        // Make one of them water and take everything off it.
        game.territories[1].biome = Biome::Ocean;
        game.territories[1].set_count(Kind::Yard, 0);
        game.territories[1].extractors.clear();
        assert!(
            game.is_fully_exploited(),
            "an unclaimable territory is not an unfinished one"
        );
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
            place.garrison.is_some(),
            "a structure that holds the ground"
        );
        assert_eq!(place.citizens(), 2);

        // One per resource a landing leaves, which is food and metal. The rule is *for
        // each resource it leaves*, not *two* - the ground here has room for energy too,
        // and a landing does not open it.
        assert_eq!(place.extractors.len(), 2);
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
        // Metal in hand, so the refusal below is about hands rather than about metal. An
        // extractor costs both now, and a test that stops at the first missing thing stops
        // testing the second.
        let mut game = founded();
        game.territories[0].add(Resource::Metal, 5);
        assert_eq!(
            game.territory(TerritoryId(1)).unwrap().citizens(),
            2,
            "two hands, so two builds and then a refusal"
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

    /// `spec/turn.md`: unused resources are discarded.
    #[test]
    fn resources_left_at_the_end_of_a_turn_are_discarded() {
        let mut game = founded();
        game.territories[0].add(Resource::Metal, 5);
        let game = with_labor(game, 1, TerritoryId(1))
            .after(&Transition::Build {
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Metal),
            })
            .unwrap();
        // The labor is made either way; whether the work succeeds is what this tolerates.
        let ready = with_labor(game, 1, TerritoryId(1));
        let game = ready
            .clone()
            .after(&Transition::Work {
                count: 1,
                structure: StructureKind::Extractor,
                territory: TerritoryId(1),
                resource: Some(Resource::Metal),
            })
            .unwrap_or(ready);
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
    fn a_move_within_your_own_ground_spends_a_cell_and_keeps_the_unit() {
        let mut game = founded();
        // Found territory 2 as well, so moving there is a move rather than a founding.
        // **A citizen is what founds it now** - `S-19`. The garrison came with a flag
        // beside it before, and the flag was the thing that could be set without anybody
        // being there.
        game.territories[1].garrison = Some(Garrison::from_founding_unit(2));
        game.territories[1].put(Kind::Citizen, 1);
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer);
        pioneer.location = Location::On(TerritoryId(1));
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
            .expect("it is still a unit");
        assert_eq!(pioneer.cells, 1, "one cell spent");
        assert!(pioneer.is_on(TerritoryId(2)));
    }

    /// Moving onto ground you do not hold takes it and founds it, in one action - so the
    /// unit is consumed and there is never a pioneer standing on ground it has taken but
    /// not founded.
    #[test]
    fn a_move_onto_unclaimed_ground_founds_it_and_consumes_the_unit() {
        let mut game = founded();
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer);
        pioneer.location = Location::On(TerritoryId(1));
        game.units.push(pioneer);

        let moved = game
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                territory: TerritoryId(2),
            })
            .unwrap();
        let two = moved.territory(TerritoryId(2)).unwrap();
        assert!(two.founded());
        assert!(two.garrison.is_some());
        assert_eq!(two.citizens(), 2);
        assert_eq!(two.extractors.len(), 2, "a farm and a mine");
        assert!(
            !moved.units.iter().any(|u| u.kind == UnitKind::Pioneer),
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

    #[test]
    fn a_pioneer_that_is_not_fed_is_lost() {
        let mut game = founded();
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer);
        pioneer.location = Location::On(TerritoryId(1));
        game.units.push(pioneer);
        // Nothing was gathered, so there is no food to pay it with. **The resources
        // only** - `held.clear()` would take the citizens too now that they are things in
        // the same list, and the fixture would be testing starvation with nobody to starve.
        game.territories[0].discard_resources();

        let after = game.after(&Transition::EndTurn).unwrap();
        let pioneer = after.units.iter().find(|u| u.kind == UnitKind::Pioneer);
        assert!(
            pioneer.map(|unit| !unit.usable).unwrap_or(true),
            "with no food it is lost"
        );
    }
}
