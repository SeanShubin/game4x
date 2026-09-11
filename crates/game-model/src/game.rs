//! The state, and the one function that changes it.

use crate::identity::{Resource, StructureKind, TerritoryId, UnitId, UnitKind};
use crate::rejection::Rejection;
use crate::territory::{Deposit, Garrison, Territory, population_after};
use crate::thing::Kind;
use crate::transition::Transition;
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
                // Total capacity for `extractors`, each yielding `density`. **The two
                // numbers are stored as the two numbers now** - they were held as that many
                // identical `Node`s, which `P-290` removed the need for by letting capacity
                // bound extractors of a resource directly.
                let place = next.territory_mut(*territory)?;
                place.deposits.insert(
                    *resource,
                    Deposit {
                        capacity: *extractors,
                        density: *density,
                    },
                );
            }
            Transition::SetForceOfNature { territory, force } => {
                next.territory_mut(*territory)?.force_of_nature = *force;
            }
            Transition::SetBiome { territory, biome } => {
                next.territory_mut(*territory)?.biome = *biome;
            }
            Transition::AddUnitToOrbit { kind, above } => {
                // **`S-55`: which orbit.** There are twelve, one above each territory, and
                // a unit put into "orbit" without saying which was above nowhere.
                next.territory(*above)?;
                let id = UnitId(next.units.len() as u32 + 1);
                next.units.push(Unit::new(id, *kind, *above));
            }
            Transition::Start => {
                next.phase = Phase::Play;
                next.turn = 1;
            }

            Transition::Land { kind, territory } => next.land(*kind, *territory)?,
            Transition::Launch { territory } => next.launch(*territory)?,
            Transition::Move { kind, territory } => next.move_unit(*kind, *territory)?,
            Transition::FoundByLand { territory } => next.found_by_land(*territory)?,
            Transition::BuildStore {
                resource,
                territory,
            } => next.build_store(*resource, *territory)?,
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
        let coordinated = territory.garrison().is_some() || units.iter().any(|force| *force > 0);
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
        // **An Ark comes down where it was, and this was unconstrained** - `S-55`. It took
        // any unit in orbit and put it on any territory named, because a bare `Orbit` carried
        // no territory for the two to be compared against. `spec/orbit.md` makes landing a
        // move and a move is between adjacent places, so the only ground an Ark can reach is
        // the territory its orbit is above.
        let at = self
            .pick(kind, |unit| unit.location == Location::Orbit(territory))
            .ok_or_else(|| match self.pick(kind, |unit| unit.in_orbit()) {
                // Told apart before the message is chosen, because a player who is given the
                // wrong reason looks in the wrong place - the same distinction `launch` draws.
                Some(elsewhere) => Rejection::NotAboveThatTerritory {
                    kind,
                    above: self.units[elsewhere].location.territory(),
                    asked: territory,
                },
                None => Rejection::NoUnitAvailable {
                    kind,
                    where_from: "in orbit",
                },
            })?;

        // Landing takes the territory and takes the Ark apart, in one action.
        // `spec/unit-types.md` says *a unit may be taken apart into what a territory needs
        // to sustain itself* and that *an Ark is taken apart on arriving from orbit*, and
        // `spec/invariants.md` forbids an intermediate step that is always taken.
        //
        // P-137 purged the word founding, and this quotation was of the wording it purged.
        // The code still calls it `found`; renaming that is a change to the model rather
        // than to a comment, and the model is being rewritten for P-134 anyway.
        let brought = self.force_brought_to(territory);
        self.found(
            territory,
            at,
            brought,
            &[Resource::Food, Resource::Metal],
            2,
        )
    }

    /// `launch ark`: an Ark's cost is paid at a Yard, and nothing comes back.
    ///
    /// **`P-342`, and it answers the second half of `C-54`.** There were two recipes and one
    /// of them was not a recipe: `produce ark` built an Ark, and `launch` moved it to orbit
    /// while firing nothing the release declared. **Launching is not a move now** - the cost
    /// is consumed, the Yard is required, and nothing is put anywhere.
    ///
    /// `spec/control.md`: *a player wins by launching an Ark from a fully exploited planet.*
    /// Asked before the cost is paid, because it is the planet as it stands that has to have
    /// been finished - and paying first would take two citizens off it.
    fn launch(&mut self, territory: TerritoryId) -> Result<(), Rejection> {
        let place = self.territory(territory)?;
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
        let won = self.is_fully_exploited();
        self.spend(territory, Resource::Metal, cost::ARK_METAL)?;
        self.spend(territory, Resource::Energy, cost::ARK_ENERGY)?;
        self.territories[territory.index()].remove(Kind::Citizen, cost::ARK_CITIZENS);
        self.won = won;
        Ok(())
    }

    /// `spec/control.md`: *a planet is fully exploited when every territory that can be taken
    /// has been taken, every territory is producing the greatest output it can, and every
    /// storage structure on it is full.*
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
    /// `its nodes` where the specification says `how many it has total capacity for`. `C-11`
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
                Location::Orbit(_) => false,
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

        // **`P-214` still holds and this guard no longer serves it.** Moving is moving and
        // founding is a different command - that was the rule, and the guard existed because
        // arriving on unclaimed ground *was* founding, so a move onto it would have fired a
        // recipe the player had not named.
        //
        // **`S-76` separated them.** A pioneer now has to be standing on unclaimed ground
        // before `found by land` can fire, so moving there is the only way to found at all,
        // and refusing it would make the recipe unreachable. Moving still founds nothing:
        // arriving leaves the ground unclaimed and the player still says which recipe.
        self.units[at].cells -= cost::MOVE_CELLS;
        self.units[at].location = Location::On(territory);
        self.units[at].exhausted = true;
        Ok(())
    }

    /// `found by land`: a pioneer standing on unclaimed ground is consumed by founding it.
    ///
    /// **The unit is not an argument because the recipe names it.** `found by land`
    /// consumes one pioneer and nothing else can run it, so a command binding what the
    /// recipe leaves open binds only the place.
    ///
    /// **The pioneer has to be there - `S-76`, Sean's decision on `P-347`.** The blank
    /// *Where* cell of `found by land`'s only row already said so:
    /// `releases/first-release.md:178`, *a blank means the one place the recipe acts*. So the
    /// release and the decision agreed, and only this function disagreed with both.
    ///
    /// **What it used to do, and why, because that reasoning was sound.** It picked a pioneer
    /// on adjacent ground, spent its move, and founded - so a founding unit never stood on
    /// ground it had taken but not founded, and never had to be fed there. That answered a
    /// question Sean has now answered differently. **It is overridden, not mistaken.**
    fn found_by_land(&mut self, territory: TerritoryId) -> Result<(), Rejection> {
        if self.territory(territory)?.founded() {
            return Err(Rejection::AlreadyFounded { territory });
        }
        let kind = UnitKind::Pioneer;
        // **No cell is spent and no adjacency is asked.** Founding is not a move now, so
        // getting there is `move` and costs what a move costs; this consumes what it finds
        // standing on the ground.
        let at = self
            .pick(
                kind,
                |unit| matches!(unit.location, Location::On(here) if here == territory),
            )
            .ok_or(Rejection::NoUnitAvailable {
                kind,
                where_from: "on that ground to found it",
            })?;

        let brought = self.force_brought_to(territory);
        self.found(
            territory,
            at,
            brought,
            &[Resource::Food, Resource::Metal],
            2,
        )
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
    /// The organised force a player can bring to bear on a territory it does not hold.
    ///
    /// `spec/control.md`: *a military unit is organised force in itself, so several brought
    /// to one place sum.* So this is a sum and not a maximum - the coordination rule that
    /// makes unorganised citizens present only their highest does not apply to units, which
    /// carry coordination with them.
    ///
    /// **What counts as brought is *able to arrive*** - adjacent, with a cell to spend. A
    /// unit two territories away is not at the battle, and one with no fuel cannot cross.
    fn force_brought_to(&self, territory: TerritoryId) -> u32 {
        self.units
            .iter()
            .filter(|unit| match unit.location {
                // **A unit standing on the ground is bearing on it - `S-76`.** Founding used
                // to be done from next door, so every unit that counted was adjacent and
                // needed a move left to arrive. A pioneer that has already crossed is on the
                // ground with no move to spend, and it is the one whose force is most
                // obviously brought: `P-275` counts *the organised force brought*, not the
                // force of the unit consumed, and this is what that phrase has to mean once
                // arriving and founding are two acts.
                Location::On(here) if here == territory => true,
                Location::On(from) => {
                    unit.cells >= cost::MOVE_CELLS && self.are_adjacent(from, territory)
                }
                // An Ark invades from orbit, which `spec/unit-types.md` allows and is how
                // the first territory of a game is ever taken.
                Location::Orbit(_) => true,
            })
            .map(|unit| unit.kind.force())
            .sum()
    }

    fn found(
        &mut self,
        territory: TerritoryId,
        unit_at: usize,
        brought: u32,
        leaves: &[Resource],
        citizens: u32,
    ) -> Result<(), Rejection> {
        // `spec/planet.md`: no territory can be claimed whose biome is ocean. Asked before
        // the force is compared, so the answer says what is actually wrong - being at sea
        // is not a matter of not having brought enough.
        if !self.territory(territory)?.biome.is_claimable() {
            return Err(Rejection::CannotClaimOcean(territory));
        }
        // **`P-275`: taking uses the organised force *brought*, not the force of the one
        // unit consumed.** *A military unit is organised force in itself, so several brought
        // to one place sum. Taking a territory uses the organised force brought to it, and
        // several units may take together.*
        //
        // Two numbers, and they are different: `brought` is what the attack presents, and
        // the unit at `unit_at` is the one the recipe consumes and whose force the garrison
        // inherits. Before this they were the same number, which is why a jungle at nature
        // two could not be taken by anything - `C-24`, and it was never a defect in the
        // model so much as a rule nobody had written.
        self.take(territory, brought)?;
        let force = self.units[unit_at].kind.force();
        self.units.remove(unit_at);

        let place = &mut self.territories[territory.index()];
        // `spec/unit-types.md`: the structure a founding unit becomes has one less force
        // than the unit. `spec/control.md`: founding is a garrison's only source.
        place.set_garrison(Some(Garrison::from_founding_unit(force)));
        place.put(Kind::Citizen, citizens);
        for resource in leaves {
            if place.has_room_for_extractor(*resource) {
                place.add_extractor(*resource);
            }
            // **`P-261`: a food store and a metal store, and no energy store.** Deliberate -
            // *the three resources are supposed to feel different*, and energy is the one a
            // player must build somewhere to keep before any of it survives a turn. It is
            // also what makes an Ark expensive in a way a Yard is not: twelve energy, on
            // ground that starts with nowhere to put a single unit of it.
            //
            // The same two resources as the extractors, so a founding leaves each of them
            // somewhere to produce into. An extractor holds nothing - `P-260` - so a
            // founding leaving a mine and no metal store would produce metal it could not
            // keep past the turn it was dug.
            place.add_store(*resource);
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

    /// `build store`: somewhere to keep one resource between turns.
    ///
    /// **`P-258` made this necessary rather than useful.** A territory used to keep twenty
    /// of each by declaration; it now declares capacity only for the things that hold them,
    /// so a territory that has built no store keeps nothing at all overnight.
    ///
    /// Bounded by *as many as the extractors of its resource* - the founding one counted -
    /// so the ceiling is the node count, which is what bounds the extractors themselves.
    fn build_store(&mut self, resource: Resource, territory: TerritoryId) -> Result<(), Rejection> {
        let place = self.territory(territory)?;
        if !place.founded() {
            return Err(Rejection::NotControlled(territory));
        }
        if place.stores(resource) >= place.store_capacity(resource) {
            return Err(Rejection::NoRoomForAnother {
                territory,
                kind: Kind::Store,
            });
        }
        // Labor first, so a territory with the metal and no labor is refused for the reason
        // that is true rather than for the one asked about second - the same order `build`
        // uses below.
        self.spend_labor(territory, cost::STORE_LABOR)?;
        self.spend(territory, Resource::Metal, cost::STORE_METAL)?;
        self.territory_mut(territory)?.add_store(resource);
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
                // Labor first, so a territory with the metal and no labor is refused for
                // the reason that is true rather than for the one asked about second.
                self.spend_labor(territory, cost::YARD_LABOR)?;
                self.spend(territory, Resource::Metal, cost::YARD_METAL)?;
                self.territory_mut(territory)?.put(Kind::Yard, 1);
                Ok(())
            }
            StructureKind::Extractor => {
                let resource = resource.ok_or(Rejection::ResourceNotNamed(structure))?;
                if !self.territory(territory)?.has_room_for_extractor(resource) {
                    return Err(Rejection::NoRoomForExtractor {
                        territory,
                        resource,
                    });
                }
                self.spend_labor(territory, cost::EXTRACTOR_LABOR)?;
                self.spend(territory, Resource::Metal, cost::EXTRACTOR_METAL)?;
                self.territory_mut(territory)?.add_extractor(resource);
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
        // Produced on the ground, so the orbit it is given is the one it would launch into.
        let mut unit = Unit::new(id, kind, territory);
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
                if self.territory(territory)?.garrison().is_none() {
                    return Err(Rejection::NothingToWorkAt {
                        territory,
                        structure,
                    });
                }
                self.spend_labor(territory, count)?;
                // **Working a garrison spends labor and records nothing, which is what it
                // already did.** `manned` was deleted by `S-72`, and the line that used to
                // increment it here was writing to a temporary: `garrison()` returns a copy,
                // so `garrison.manned += count` vanished and every garrison read `manned 0`.
                // The field's own doc claimed production never made that mistake; this was
                // production making it.
                Ok(())
            }
            StructureKind::Extractor => {
                let resource = resource.ok_or(Rejection::ResourceNotNamed(structure))?;
                let ready: Vec<usize> = self
                    .territory(territory)?
                    .extractors_for(resource)
                    .into_iter()
                    .filter(|at| !self.territories[territory.index()].extractors()[*at].exhausted)
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
                // Every extractor of one resource here yields the same, since `P-290` made
                // density a fact about the territory and the resource. The sort is kept
                // because it is what makes the answer independent of iteration order, and
                // costs nothing now that the key is constant across the list.
                let mut by_density: Vec<(u32, usize)> = ready
                    .into_iter()
                    .map(|at| {
                        let place = &self.territories[territory.index()];
                        (place.density_of(place.extractors()[at].resource), at)
                    })
                    .collect();
                by_density.sort_by_key(|(density, at)| (std::cmp::Reverse(*density), *at));

                let mut produced = 0;
                for (density, at) in by_density.into_iter().take(count as usize) {
                    self.territories[territory.index()].exhaust_extractor(at);
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
                // **Destroyed, where they used to be marked unusable** - `P-367`.
                // `spec/control.md` now reads *its entire population perishes, and every unit
                // on it is destroyed*; it said *any ark on it becomes unusable*. The old
                // behaviour left a state the release cannot describe: a unit that is
                // somewhere, owned, and can never act, reading in a hand derivation exactly
                // like one that is merely exhausted. That is the confusion `P-339` removed
                // for `unpaid`, and this is the same one a rule over.
                //
                // **Every unit, not every ark.** The code already caught pioneers under a
                // sentence that named only arks, which `C-77` raised; the new sentence says
                // every unit, so the breadth is now the specified one rather than an
                // accident that happened to be right.
                self.units.retain(|unit| !unit.is_on(id));
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
        // **Nothing but a citizen eats** - `P-339`, and it answers `C-62`. A unit that went
        // unpaid used to be marked `usable = false` and left where it was: not consumed, no
        // metal given back, and `usable` a trait the release does not declare - so a pioneer
        // that had starved read exactly like one that had not in the file Sean derives by
        // hand. **An ark and a pioneer take no upkeep now**, so there is no unpaid unit and
        // the twenty lines that shared the food out are gone rather than made unreachable.
        //
        // **`usable` is gone too, as of `P-367`.** It survived `P-339` because nature
        // retaking a territory still wrecked what stood on it - and nature destroys those
        // units now, so nothing sets it. It was a trait the release does not declare and no
        // artifact could show, which is what made a starved pioneer unreadable in the first
        // place; that whole class of invisible state is now closed.

        // Then a population grows on surplus food, or starves for want of it.
        let food = self.territories[id.index()].store(Resource::Food);
        let citizens = self.territories[id.index()].citizens();
        self.territories[id.index()].set_count(Kind::Citizen, population_after(citizens, food));

        // What expires expires and what is over the bound is lost; metal and energy carry.
        // Nothing transforms here: founding happens when a unit arrives, so by the time a
        // turn ends there is never a unit waiting to become something.
        self.territories[id.index()].end_of_turn_losses();
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
            finish(place);
        }
        assert!(game.is_fully_exploited());
        assert!(!game.has_won(), "nobody has launched anything yet");

        // An Ark has to be on the planet to leave it.
        // **What a launch costs, where an Ark used to be pushed.** `P-342` made launching one
        // recipe: the cost is paid at a Yard and nothing comes back, so what has to be there
        // is the cost rather than a unit.
        {
            let place = &mut game.territories[0];
            place.set_count(Kind::Yard, 1);
            // **Never fewer than it already has** - `P-361`. This was `set_count(.., 2)`, and
            // now that a finished territory is one with the population its food supports,
            // setting two on a territory that had twelve un-finished the planet a line before
            // it was asked whether the planet was finished. Paying a cost is what `Launch`
            // does; this only has to make it affordable.
            let enough = place.citizens().max(cost::ARK_CITIZENS);
            place.set_count(Kind::Citizen, enough);
            place.add(Resource::Metal, cost::ARK_METAL);
            place.add(Resource::Energy, cost::ARK_ENERGY);
        }
        let won = game
            .after(&Transition::Launch {
                territory: TerritoryId(1),
            })
            .unwrap();
        assert!(won.has_won(), "the planet was finished and an Ark left it");
    }

    /// Launching off an unfinished planet is just leaving.
    #[test]
    fn launching_from_an_unfinished_planet_wins_nothing() {
        let mut game = designed().after(&Transition::Start).unwrap();
        // **What a launch costs, where an Ark used to be pushed.** `P-342` made launching one
        // recipe: the cost is paid at a Yard and nothing comes back, so what has to be there
        // is the cost rather than a unit.
        {
            let place = &mut game.territories[0];
            place.set_count(Kind::Yard, 1);
            // **Never fewer than it already has** - `P-361`. This was `set_count(.., 2)`, and
            // now that a finished territory is one with the population its food supports,
            // setting two on a territory that had twelve un-finished the planet a line before
            // it was asked whether the planet was finished. Paying a cost is what `Launch`
            // does; this only has to make it affordable.
            let enough = place.citizens().max(cost::ARK_CITIZENS);
            place.set_count(Kind::Citizen, enough);
            place.add(Resource::Metal, cost::ARK_METAL);
            place.add(Resource::Energy, cost::ARK_ENERGY);
        }
        let after = game
            .after(&Transition::Launch {
                territory: TerritoryId(1),
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
            finish(place);
        }
        // A finished planet nobody has launched from is not a win.
        assert!(game.is_fully_exploited());
        assert!(!game.has_won());

        // **And it stays a win once it is one, however the planet changes afterwards.**
        //
        // This used to check the other half - that a *Pioneer* leaving wins nothing - and
        // `P-342` took that case away rather than answering it: there is one launch recipe and
        // it is `launch ark`, so a pioneer launching is a state the language cannot express.
        // The half that survives is the one the name is about.
        {
            let place = &mut game.territories[0];
            place.set_count(Kind::Yard, 1);
            // **Never fewer than it already has** - `P-361`. This was `set_count(.., 2)`, and
            // now that a finished territory is one with the population its food supports,
            // setting two on a territory that had twelve un-finished the planet a line before
            // it was asked whether the planet was finished. Paying a cost is what `Launch`
            // does; this only has to make it affordable.
            let enough = place.citizens().max(cost::ARK_CITIZENS);
            place.set_count(Kind::Citizen, enough);
            place.add(Resource::Metal, cost::ARK_METAL);
            place.add(Resource::Energy, cost::ARK_ENERGY);
        }
        let after = game
            .after(&Transition::Launch {
                territory: TerritoryId(1),
            })
            .unwrap();
        assert!(after.has_won(), "an Ark left a finished planet");

        // Take the planet apart underneath it: the win has already happened.
        let mut later = after.clone();
        later.territories[0]
            .held
            .retain(|thing| thing.kind != Kind::Extractor);
        assert!(!later.is_fully_exploited(), "the planet is unfinished now");
        assert!(
            later.has_won(),
            "and it was won when the Ark left, which is a moment"
        );
    }

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
        // *nothing orbits a planet without being above a particular territory*, and landing is
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
        // **A founding leaves a metal store** - `P-261` - so five metal has somewhere to be.
        // Before `P-258` a territory kept twenty of everything by declaration and this
        // fixture needed nothing; now what it keeps is what its stores hold.
        assert_eq!(
            game.territories[0].capacity(Resource::Metal),
            10,
            "one store from founding, holding ten"
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
        // One store comes with the founding; this is the second, so the bound is twenty and
        // the arithmetic below is the one this test was written for.
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
        game.territories[1].set_garrison(Some(Garrison::from_founding_unit(2)));
        game.territories[1].put(Kind::Citizen, 1);
        let id = UnitId(game.units.len() as u32 + 1);
        let mut pioneer = Unit::new(id, UnitKind::Pioneer, TerritoryId(1));
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
        let moved = game
            .after(&Transition::Move {
                kind: UnitKind::Pioneer,
                territory: TerritoryId(2),
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
        game.territories[0].end_of_turn_losses();

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
        game.territories[doomed.index()].force_of_nature = 5;
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
