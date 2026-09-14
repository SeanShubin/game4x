//! Every rule of the game, in one place.
//!
//! **What a reader of `releases/first-release.md` wants is this file beside it.** The release
//! states twenty-six recipes across thirty-seven blocks of rows; the engine implements them,
//! and until 2026-09-14 it did so interleaved with the state they act on, across a thousand
//! lines of `game.rs` and `territory.rs`. Sean asked to be able to read the data files and a
//! few files of code and see what the engine does. This is the few files of code.
//!
//! # What is a rule and what is not
//!
//! **A rule changes the state and cites a recipe.** `build_store` is `build store`, `work` is
//! `work`, and `end_turn_observed` is the world's fifteen in `P-379`'s order.
//!
//! **A primitive is not a rule.** `Territory::put`, `remove`, `add` and `take` move things
//! about and decide nothing - they are what a rule is written with, and they stayed with the
//! state. The line is whether the item names a recipe, not whether it takes `&mut self`.
//!
//! **A question is not a rule either.** `force_in`, `controlled`, `has_lost` and
//! `is_fully_exploited` answer what is there, and stayed too.
//!
//! # What this file is not yet
//!
//! **It is still Rust rather than a reading of `spec/data/`.** The rules are data now -
//! `block.4x`, `line.4x`, `constraint.4x` and `for.4x`, promoted by `P-497` - and nothing
//! here reads them. **Gathering the rules is what makes the distance measurable**, which is
//! the whole reason for the move: what is in this file and not in those relations is the list
//! of things the data cannot yet say.
//!
//! `build store` is the example to hold in mind. Its rows are four; its body below is
//! eighteen lines, and two of the things it does - requiring the territory to be controlled,
//! and bounding the stores - are stated in no row, because the release bounds a store by *as
//! many as the extractors of its resource*. `C-114` counts eight such relationships in twelve.

use crate::game::cost;
use crate::identity::{Resource, StructureKind, TerritoryId, UnitId, UnitKind};
use crate::rejection::Rejection;
use crate::territory::{Deposit, Garrison, Territory};
use crate::thing::{Kind, Thing, Trait};
use crate::transition::Transition;
use crate::unit::{Location, Unit};
use crate::{Game, Phase};

impl Game {
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
                next.territory_mut(*territory)?.set_force_of_nature(*force);
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
            Transition::Move { kind, from, to } => next.move_unit(*kind, *from, *to)?,
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
    /// **`P-460`: the player says where the unit is standing, and the model no longer
    /// chooses.** `C-101` reported that `move` leaves two places open and the command bound
    /// one, so `$from` was whichever unit `pick` reached first - which is an identity the
    /// specification does not give a unit and, since `P-456`, never will.
    ///
    /// **The complaints separate along the same line.** *There is no pioneer on territory 3*
    /// is now a different answer from *there is no pioneer anywhere*, because the player
    /// named the place; before this, only the second could be said.
    fn move_unit(
        &mut self,
        kind: UnitKind,
        from: TerritoryId,
        to: TerritoryId,
    ) -> Result<(), Rejection> {
        self.territory(from)?;
        self.territory(to)?;
        // Adjacency is asked of the two places the command named rather than of whatever
        // the model found, so "these are not adjacent" is about the move the player
        // described.
        if !self.are_adjacent(from, to) {
            return Err(Rejection::NotAdjacent { from, to });
        }
        // **Standing there is asked before a cell is**, so that "there is no pioneer on 3"
        // and "it has already moved" stay different complaints.
        let there = self.pick(kind, |unit| unit.location == Location::On(from));
        if there.is_none() {
            return Err(match self.pick(kind, |unit| !unit.in_orbit()) {
                Some(_) => Rejection::NoUnitThere {
                    kind,
                    territory: from,
                },
                None => Rejection::NoUnitAvailable {
                    kind,
                    where_from: "on the planet",
                },
            });
        }
        let at = self
            .pick(kind, |unit| {
                unit.location == Location::On(from) && unit.cells >= cost::MOVE_CELLS
            })
            .ok_or(Rejection::NoCells(kind))?;

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
        self.units[at].location = Location::On(to);
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
    /// `spec/control.md`, as `P-425` words it: *a unit is organised force in itself*, and
    /// *several units brought to one place sum their force*. So this is a sum and not a
    /// maximum - and a unit *musters its own force needing nothing to coordinate it*, where
    /// a citizen musters none unless a garrison does.
    ///
    /// **`P-425` also took *military* out of the word.** There were two units in the release
    /// and neither was declared military, so the qualifier named a class with no members -
    /// which is `C-94`, resolved in the release's favour: only a garrison coordinates
    /// citizens, and a unit coordinates nobody but itself.
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
        // unit consumed.** *A unit is organised force in itself*, and *several units brought
        // to one place sum their force. Taking a territory uses the organised force brought
        // to it, and several units may take together.*
        //
        // Two numbers, and they are different: `brought` is what the attack presents, and
        // the unit at `unit_at` is the one the recipe consumes and whose force the garrison
        // inherits. Before this they were the same number, which is why a jungle at nature
        // two could not be taken by anything - `C-24`, and it was never a defect in the
        // model so much as a rule nobody had written.
        self.take(territory, brought)?;
        let force = self.units[unit_at].kind.force();
        // **The pioneer's remaining fuel stays in the territory.** This quoted `P-487`'s
        // *what it held falls loose where it stood* until 2026-09-14, and `P-505` replaced
        // that sentence: `spec/logistics.md` now says **when a thing that holds capacity is
        // consumed, the place it stood in has that much less room** - *nothing falls loose,
        // because nothing was inside it: what a place holds is the place's, and destroying a
        // store leaves the resources where they already were.*
        //
        // **The line below is unchanged and the reason under it is not.** It was *move what
        // was inside out*; it is now *the energy was the territory's all along, and consuming
        // the pioneer only lowers the room*. Both put the same number in the same place, so
        // nothing observable moved - which is why the gate caught this as a stale quotation
        // rather than as a wrong answer.
        //
        // **What has not followed is the model** - `C-124`. Under pooling a unit holds no
        // fuel at all: `unit.cells` is still a number on the unit, and the specification says
        // a place holds one number per kind and a bin only contributes capacity to it. The
        // two agree on the state after a founding and disagree about where the fuel is before
        // one, which is a change to `refuel`, to what a move spends, and to what the
        // containment tree draws inside a pioneer.
        let spare = self.units[unit_at].cells;
        self.units.remove(unit_at);
        if spare > 0 {
            self.territories[territory.index()].add(Resource::Energy, spare);
        }

        let place = &mut self.territories[territory.index()];
        // `spec/unit-types.md`: the structure a founding unit becomes has one less force
        // than the unit. `spec/control.md`: founding is a garrison's only source.
        place.set_garrison(Some(Garrison::from_founding_unit(force)));
        place.put(Kind::Citizen, citizens);
        for resource in leaves {
            if place.has_room_for_extractor(*resource) {
                place.add_extractor(*resource);
            }
        }
        // **`P-427` deleted the two `produce 1 store` rows from both founding recipes**, and
        // this followed them. A founded territory now has an extractor for food and one for
        // metal and **nowhere to put what they produce** until a store is built, at 1 labor
        // and 1 metal each.
        //
        // **The reason is metal, not stores.** A store's binding is 1 metal, so founding
        // created 5 metal of binding while consuming 3 - it was a metal source, and
        // `spec/invariants.md` licenses only the planet, the star and time. Both recipes now
        // consume 3 and create 3.
        //
        // **`P-261`'s rule is not repealed by this and is now about `build store`.** *The
        // three resources are supposed to feel different*, and energy was the one a player
        // had to build for before any of it survived a turn. All three are that now.
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
                // **The fill and the payment are one number** - `P-486`: *it is built with
                // that bin full, and the energy is paid where it is built.* So this is the
                // bin's size read from the kind rather than a `PIONEER_ENERGY` beside it,
                // which is the constant that could drift from the Units table's `Fuel`.
                //
                // **It was six and it is two, and the six was the last trace of a rule the
                // specification had stopped saying.** `P-66` promoted the fill; `0aca92d`
                // lost the sentence on 2026-09-01 and left the number, so a pioneer went on
                // paying for a bin nothing said got filled. Sean found it by asking why a
                // pioneer costs energy at all.
                self.spend(territory, Resource::Energy, kind.cells())?;
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
    /// The five things ending a turn does, in the order `spec/turn.md` gives them.
    ///
    /// > everything with upkeep pays it; then a population grows on surplus food or starves
    /// > for want of it; **what expires expires, and what was not kept in order is lost**;
    /// > then **nature takes back what is no longer held**; and **time restores every count to
    /// > the number that thing's kind declares**
    ///
    /// **One list, read by two callers** - `S-125`. `end_turn` runs them and the turn report
    /// names its sections from them, so a phase added to the rule adds a section and a section
    /// with no clause behind it is a defect a person can see. Writing the names into the report
    /// instead would have been a second copy of the specification, free to drift from this one.
    pub const END_OF_TURN_PHASES: [&'static str; 5] = [
        "everything with upkeep pays it",
        "a population grows on surplus food or starves for want of it",
        "what expires expires, and what was not kept in order is lost",
        "nature takes back what is no longer held",
        "time restores every count",
    ];
    /// Consume, transform, discard, unspend.
    fn end_turn(&mut self) {
        self.end_turn_observed(&mut |_, _| {});
    }
    /// Ending a turn, with the state offered up after each of the five phases.
    ///
    /// **One code path rather than two** - the observer is how the report sees inside without
    /// a second implementation of the order. `end_turn` passes a closure that does nothing, so
    /// ordinary play pays for nothing, and the report's sequence cannot drift from the game's
    /// because there is only one sequence.
    ///
    /// # Phase-major rather than territory-major
    ///
    /// Each phase now runs across every territory before the next begins, where `settle` used
    /// to take one territory through the first three. **The result cannot differ, and the
    /// reason is the one already written here**: in this release nothing crosses a boundary,
    /// so no territory can affect another's outcome and the order they are taken in cannot
    /// change the result - which is what `docs/architecture.md` rule 9 asks for, and what
    /// would make this safe to run in parallel unchanged. The committed dumps are the check.
    ///
    /// **What the phases have to hand each other is one number.** `upkeep` leaves a count of
    /// citizens it could not feed and `perish` consumes that many; the release calls `unpaid`
    /// derived, so nothing is written onto a citizen and the seam needs nothing stored.
    pub fn end_turn_observed(&mut self, seen: &mut dyn FnMut(&Game, &'static str)) {
        let ids: Vec<TerritoryId> = self.territories.iter().map(|t| t.id).collect();

        // **Nothing but a citizen eats** - `P-339`, and it answers `C-62`. A unit that went
        // unpaid used to be marked `usable = false` and left where it was: not consumed, no
        // metal given back, and `usable` a trait the release does not declare - so a pioneer
        // that had starved read exactly like one that had not in the file Sean derives by
        // hand. **An ark and a pioneer take no upkeep now**, so there is no unpaid unit and
        // the twenty lines that shared the food out are gone rather than made unreachable.
        //
        // **`usable` is gone too, as of `P-367`.** It survived `P-339` because nature
        // retaking a territory still wrecked what stood on it - and nature destroys those
        // units now, so nothing sets it.
        let unpaid: Vec<u32> = ids
            .iter()
            .map(|id| self.territories[id.index()].pay_upkeep())
            .collect();
        seen(self, Self::END_OF_TURN_PHASES[0]);

        // `bear`, `breed`, `renew` and `perish`, fired one at a time in `P-379`'s order.
        //
        // **It was one call to `population_after` and is five recipes**, which is `P-373`'s
        // saturating rewrite: `grow` read its quantity from the state and these five do not.
        // The closed form is still there and is the second derivation now -
        // `tests/population_two_ways.rs` compares the two at every pair in a range.
        for (id, unpaid) in ids.iter().zip(unpaid) {
            self.territories[id.index()].grow_or_starve(unpaid);
        }
        seen(self, Self::END_OF_TURN_PHASES[1]);

        // What expires expires and what is over the bound is lost; metal and energy carry.
        // Nothing transforms here: founding happens when a unit arrives, so by the time a
        // turn ends there is never a unit waiting to become something.
        for id in &ids {
            self.territories[id.index()].end_of_turn_losses();
        }
        seen(self, Self::END_OF_TURN_PHASES[2]);

        // Nature reclaims anything no longer held. Done after every territory has
        // settled, because whether force is enough depends on what settling left behind.
        //
        // # Four recipes since `P-494`, and not one of them is a comparison
        //
        // This was `if self.force_in(id) < needed`, which is a zero test read from
        // underneath - an inhibitor arc, and `lenses/research` puts a net with two of those
        // past the point where reachability is decidable. `P-373`'s saturating rewrite is
        // what took it out, the same way it took the `min` out of population growth:
        //
        // - **`hold`** spends one force to mark one nature `met`. It fires `min(force,
        //   nature)` times **because that is when it stops being enabled**, not because
        //   anything computes a lesser.
        // - **`take`** consumes a nature per force on ground nobody has founded, which is
        //   what wears a territory's resistance down to where it can be taken.
        // - **`reclaim`** fires on the **presence** of a nature nobody met. That token is the
        //   shortfall materialised, exactly as `unpaid` is the shortfall of upkeep.
        // - **`renew`** clears the marks, so a committed state holds none.
        //
        // **The condition is the same one, derived a second way.** After `hold`, the natures
        // left unmet are `nature - min(force, nature)`, which is above zero exactly where
        // `force < nature`. So nothing about who loses a territory has changed, and
        // `tests/` says so rather than this comment.
        //
        // **Two assumptions are stated here rather than read from the release, and `C-116`
        // and `C-117` carry them.** The rows say `met at least 0`, which is true of every
        // number and would make `reclaim` fire on every territory every turn; this reads it
        // as `met 0`, the form `spoil` already uses. And the rows put no `Where` on `take`,
        // which as written erodes the nature of ground the player is holding perfectly well;
        // this fires it only where nothing has been founded, because that is the only ground
        // whose defending force is nature's - `Game::defending_force`.
        for id in &ids {
            let force = self.force_in(*id);
            let place = &mut self.territories[id.index()];
            if place.founded() {
                place.hold_with(force);
            } else {
                place.take_natures(force);
            }
        }
        for id in self.controlled() {
            if self.territory(id).map(|t| t.unmet_natures()).unwrap_or(0) == 0 {
                continue;
            }
            // **Destroyed, where they used to be marked unusable** - `P-367`.
            // `spec/control.md` now reads *its entire population perishes, and every unit
            // on it is destroyed*; it said *any ark on it becomes unusable*. The old
            // behaviour left a state the release cannot describe: a unit that is
            // somewhere, owned, and can never act, reading in a hand derivation exactly
            // like one that is merely exhausted.
            //
            // **Every unit, not every ark.** The code already caught pioneers under a
            // sentence that named only arks, which `C-77` raised; the new sentence says
            // every unit, so the breadth is now the specified one rather than an
            // accident that happened to be right.
            //
            // **Neither half is in `reclaim`'s rows**, which say only `consume 1 citizen`.
            // `spec/control.md` says the population perishes *and every unit on it is
            // destroyed*, and the garrison goes with the founding; the rows reach the first
            // and name neither of the others. Reported in `C-117` rather than dropped.
            self.units.retain(|unit| !unit.is_on(id));
            if let Ok(territory) = self.territory_mut(id) {
                territory.lost_to_nature();
            }
        }
        for id in &ids {
            self.territories[id.index()].renew_natures();
        }
        seen(self, Self::END_OF_TURN_PHASES[3]);

        // **Time restores every count, and it is one step and last** - `P-480`. It used to
        // straddle the reclaim: a territory's things were made ready inside `settle`, before
        // it, and units were un-exhausted after - one rule in two places either side of
        // another.
        //
        // **Nothing observable moved**, and that is checkable rather than hoped: the reclaim
        // reads `force_in`, which is a territory's held force plus the force of the units
        // standing on it, and `Unit::force` is a constant of the kind. `refresh` clears
        // `Trait::Ready` and nothing else. So no part of what nature decides can see whether
        // a thing has been readied.
        for id in &ids {
            self.territories[id.index()].make_ready();
        }
        for unit in &mut self.units {
            unit.exhausted = false;
        }
        seen(self, Self::END_OF_TURN_PHASES[4]);

        self.turn += 1;
    }
}

impl Territory {
    /// `hold`, fired as many times as it can be: one force spent marks one nature met.
    ///
    /// **The `min` is the firing rule rather than an expression** - `P-373`. A transition
    /// with two inputs stops when either runs out, so this returns what it spent and the
    /// caller takes that much force. Nothing computes a lesser of anything.
    pub fn hold_with(&mut self, force: u32) -> u32 {
        let mut spent = 0;
        for thing in self.held.iter_mut() {
            if spent == force {
                break;
            }
            if thing.kind != Kind::Nature || thing.trait_of(Trait::Met).unwrap_or(0) > 0 {
                continue;
            }
            thing.set(Trait::Met, 1);
            spent += 1;
        }
        spent
    }
    /// `take`: force wears the ground's resistance down, one nature per force.
    ///
    /// **Fires `min(force, nature)` times, and the strict inequality falls out of what is
    /// left over.** `spec/control.md` asks for force *greater than* the existing force, which
    /// is a comparison between two variable quantities and the worse half of the force rule.
    /// Consuming a nature per force leaves at least one force where it was greater and none
    /// where it was equal - and `found by land` requires one, which is a presence test.
    pub fn take_natures(&mut self, force: u32) -> u32 {
        let taking = force.min(self.force_of_nature());
        let mut left = taking;
        self.held.retain(|thing| {
            if left > 0 && thing.kind == Kind::Nature {
                left -= 1;
                return false;
            }
            true
        });
        taking
    }
    /// `renew`: every nature is unmet again, which is what a committed state holds.
    pub fn renew_natures(&mut self) {
        for thing in self.held.iter_mut() {
            if thing.kind == Kind::Nature {
                thing.clear(Trait::Met);
            }
        }
    }
    /// End-of-turn losses: what expires, and what nothing is holding.
    ///
    /// **`C-11`.** `spec/turn.md`: *what expires expires, and what was not kept in order is
    /// lost*, and *what a territory can keep is bounded. Anything above the bound is lost
    /// when the turn ends.* The model discarded **all three** resources every turn, which is
    /// neither of those rules. Food expires, because the release gives it *a capacity of 20,
    /// and it keeps for one turn*; metal and energy have a capacity and no expiry, so they
    /// carry.
    ///
    /// **`P-258` moved the bound off the territory and onto the things in it.** It was a
    /// flat twenty of each, which was a property of the wrong thing - a territory that had
    /// built nothing could keep twenty of everything. What a territory keeps is now what its
    /// stores hold, and it has none until it builds them.
    ///
    /// **`P-270` says what happens to the rest, and it is not disorder.** *A resource that is
    /// in nothing can be used the turn it is made, and is lost when that turn ends - use it
    /// immediately, store it, or lose it.* That replaced *a resource that is in nothing is in
    /// disorder, and cannot be reached*, which would have been a second behaviour: something
    /// present and unspendable. **There is one behaviour now - spendable within the turn,
    /// gone at its end - so nothing here has to make a resource unreachable**, and the only
    /// thing this function does is decide what survives.
    ///
    /// Labor is not a resource and is not carried either: it is bounded by *the citizens
    /// that make it, one each per turn*, so labor left at the end of a turn was made by a
    /// citizen who is about to be refreshed and would otherwise be counted twice.
    ///
    /// **`P-381` names what those two have in common and `P-380` gave the second one its
    /// row.** A raw material is in one of three states: its source, disorder, or a container.
    /// `labor` and `fertility` are **transient** - neither has a source and nothing holds
    /// either, so both are always in disorder and neither survives the turn's end. **The
    /// `fertility` line is `C-83`**: without it a territory that starved to nobody kept the
    /// fertility its last citizens made and repopulated from stock the moment food arrived,
    /// which `spec/population.md` forbids and a test here is named for.
    pub fn end_of_turn_losses(&mut self) {
        self.held.retain(|thing| {
            // Food keeps for one turn, so what is here at the end was made this turn and
            // expires now. The other two are transient - `P-381` - and `discard` is the
            // recipe that takes all three.
            thing.kind != Kind::Food && thing.kind != Kind::Labor && thing.kind != Kind::Fertility
        });
        for resource in [Resource::Metal, Resource::Energy] {
            let kind = Kind::from_resource(resource);
            let over = self.count_of(kind).saturating_sub(self.capacity(resource));
            if over > 0 {
                self.remove(kind, over);
            }
        }
    }
    /// A population settles on the food it has, one recipe at a time.
    ///
    /// **`P-373`'s saturating rewrite, and what `grow` became.** `grow` consumed *the lesser
    /// of the surplus food and the citizens here*, which is a quantity read from the state -
    /// the thing `P-368` names as a defect. The rule is written instead as five smaller ones,
    /// each with a constant quantity, each firing as many times as it can, in the order
    /// `P-379` states: `upkeep`, then `bear`, `breed` and `renew`, then `perish`.
    ///
    /// **Each block below is one recipe, spelled as the release spells it.** A reader deriving
    /// the dump by hand can follow this against the *Recipes* table row for row, which is what
    /// `R-7` asks of the report and this is the half that makes the report honest.
    ///
    /// # Unpaid is counted, not stored
    ///
    /// The release calls `unpaid` **derived** - *its upkeep was not met* - so nothing here
    /// writes it onto a citizen. It is the number of citizens `upkeep` could not feed, carried
    /// from that block to `perish`.
    ///
    /// **Which citizens perish does not matter, and that is a fact rather than a convenience.**
    /// `upkeep` leaves food over only when every citizen ate, so a territory with an unpaid
    /// citizen has nothing left for `breed` - the two can never both happen in one ending.
    /// By the time `perish` runs, `renew` has made every citizen fertile again, so any two
    /// citizens here are the same thing and removing *some* `n` of them is removing *the* `n`.
    /// **The first of the five, on its own** - `S-125`. `spec/turn.md` ends a turn with
    /// *everything with upkeep pays it; then a population grows on surplus food or starves for
    /// want of it*, and the report says those separately now, so the code has to reach the
    /// seam between them.
    ///
    /// **What it hands on is the count, because the release says `unpaid` is derived.** It is
    /// the number of citizens `upkeep` could not feed, carried to `perish` - see the note
    /// above. So the seam needs nothing stored and nothing the specification does not say; the
    /// state between the two phases is this territory with the eaten food gone and nothing
    /// else moved.
    pub fn pay_upkeep(&mut self) -> u32 {
        // **upkeep** - `require 1 citizen`, `consume 1 food`, `put citizen paid at its
        // maximum`. Fires once per citizen while there is food, and **marks each citizen it
        // fed** rather than counting the ones it could not reach.
        //
        // **The `min` is the firing rule and not an expression** - `P-373`. Two inputs stop
        // the transition when either runs out, so `fed` is what it managed rather than a
        // lesser somebody computed.
        let citizens = self.citizens();
        let fed = citizens.min(self.store(Resource::Food));
        self.take(Resource::Food, fed);
        let mut left = fed;
        for thing in &mut self.held {
            if left == 0 {
                break;
            }
            if thing.kind == Kind::Citizen && !thing.is(Trait::Paid) {
                thing.set(Trait::Paid, 1);
                left -= 1;
            }
        }
        citizens - fed
    }
    /// Both halves of the population's ending, which is what the closed form is compared
    /// against.
    ///
    /// **The two are separate for the report and together for the check** - `tests/
    /// population_two_ways.rs` derives the population a second way and compares, and what it
    /// compares is the whole settling rather than either half. Splitting that test in two
    /// would have made it check the seam instead of the arithmetic.
    pub fn settle_population(&mut self) {
        let unpaid = self.pay_upkeep();
        self.grow_or_starve(unpaid);
    }
    /// The second of the five: a population grows on surplus food, or starves for want of it.
    ///
    /// `unpaid` is what [`Territory::pay_upkeep`] could not feed.
    pub fn grow_or_starve(&mut self, unpaid: u32) {
        // **bear** - `consume 1 citizen fertile`, `produce 1 citizen spent`, `produce 1
        // fertility`. Every citizen that has not borne this turn does, and is spent for it.
        let mut bore = 0;
        for thing in &mut self.held {
            if thing.kind == Kind::Citizen && !thing.is(Trait::Spent) {
                thing.set(Trait::Spent, 1);
                bore += 1;
            }
        }
        self.put(Kind::Fertility, bore);

        // **breed** - `consume 1 fertility`, `consume 1 food`, `produce 1 citizen`. Fires
        // while both last, which is what bounds the increase by the food; the `spent` trait
        // above is what bounds it by the citizens.
        let born = self
            .count_of(Kind::Fertility)
            .min(self.store(Resource::Food));
        self.remove(Kind::Fertility, born);
        self.take(Resource::Food, born);
        // **A citizen `breed` makes arrives paid, and that is an assumption this lane states
        // rather than a row it read** - `C-119`. `breed` is `consume 1 fertility`, `consume 1
        // food`, `produce 1 citizen`, with nothing said about `paid`; `perish` fires below it
        // on `paid 0`. **So the rows as written have every newborn eaten in the ending that
        // made it**, which is a rule nobody wrote and which the assertion below caught the
        // first time this was built the other way.
        //
        // **The reading it proceeds under**: `upkeep` spends one food to mark one citizen
        // paid, and `breed` spends one food to make one - so a newborn has had its food, by
        // the same coin, and is as paid as anybody `upkeep` reached.
        for _ in 0..born {
            self.held
                .push(Thing::of(Kind::Citizen).with(Trait::Paid, 1));
        }

        // **renew** - `consume 1 citizen spent`, `produce 1 citizen fertile`. Once per turn,
        // so nothing can bear twice in one ending and everything can bear in the next.
        for thing in &mut self.held {
            if thing.kind == Kind::Citizen {
                thing.clear(Trait::Spent);
            }
        }

        // **perish** - `consume 1 citizen, paid 0`. After `renew`, per `P-379`, which is why
        // the order is stated rather than read off four moments.
        //
        // **It fires on the absence of a mark and no longer on a number handed to it** -
        // `P-498`. The argument is still taken and is now only checked: see below.
        let starved = self
            .held
            .iter()
            .filter(|thing| thing.kind == Kind::Citizen)
            .filter(|thing| thing.trait_of(Trait::Paid).unwrap_or(0) == 0)
            .count() as u32;
        // **The count and the marks have to agree, and this assertion is here because they
        // did not.** `breed` runs above and makes citizens nothing has fed, so on the rows'
        // literal reading a newborn carries `paid 0` and is eaten in the ending that made it.
        //
        // **The first build of this argued that it could not happen and was wrong.** The
        // argument was `upkeep`'s saturation - if food ran short then `upkeep` emptied the
        // store, so `breed` had nothing to spend - which is true of the *unpaid* and says
        // nothing about the *newborn*: breeding happens exactly when nobody went unfed, and
        // every citizen it makes is unmarked. **The assertion reported two starved against
        // nought unfed within the minute**, which is the whole reason it is an assertion
        // rather than the paragraph it replaced.
        //
        // `breed` marks what it makes now - the assumption is stated where it acts, and
        // `C-119` carries it to the lane that owns the rows.
        assert_eq!(
            starved, unpaid,
            "`perish` fires on the mark and `upkeep` counted {unpaid} unfed - the difference is \
             a citizen nothing marked, which is what `breed` makes"
        );
        self.held.retain(|thing| {
            thing.kind != Kind::Citizen || thing.trait_of(Trait::Paid).unwrap_or(0) > 0
        });

        // **renew** - `require 1 citizen`, `put citizen paid 0`. The second of `P-498`'s two
        // halves, and it is last for the same reason `perish` is after `bear`: clearing the
        // marks before `perish` read them would starve the whole population every turn.
        for thing in &mut self.held {
            if thing.kind == Kind::Citizen {
                thing.clear(Trait::Paid);
            }
        }
    }
    /// Makes everything ready again, which is what ending a turn does.
    pub fn make_ready(&mut self) {
        // Every thing here becomes ready, whatever kind it is - which is `refresh` in the
        // release, a world recipe over `thing, not ready`. Naming the kinds that can stop
        // being ready would be a list to keep in step with the kinds.
        for thing in &mut self.held {
            thing.refresh();
        }
        // **The two loops that used to follow this were mutating copies.** `extractors()`
        // and `garrison()` hand back owned values now, so `extractor.exhausted = false`
        // changed a temporary and vanished. The refresh above already readies every
        // extractor, because it names no kind. **The manning that used to be reset here is
        // gone with `manned` itself** - `S-72`, Sean's decision on `C-46`.
    }
    /// What nature does when it takes a territory back.
    ///
    /// `spec/control.md`: its entire population perishes and any ark on it becomes
    /// unusable. The ark is dealt with by the caller, which is the only place that knows
    /// where units are.
    pub fn lost_to_nature(&mut self) {
        self.set_garrison(None);
        // Everything held goes, which is the one place `clear` is the right verb: nature
        // takes the population, the stores and the yards together. Naming the kinds one by
        // one would be a list to keep in step with the kinds, which is the thing this shape
        // exists to stop.
        //
        // **A line naming one kind stood here until `X-27`** - `retain(kind != Extractor)`,
        // immediately above the `clear` that made it moot. It did exactly what the comment
        // below it argues against, and cost a reader the question *why is an extractor
        // special here*. It is not.
        //
        // **And a line naming one kind stands here again since `P-494`, for the opposite
        // reason.** `nature` is what the ground resists with and is held by the territory,
        // so it is not part of the population nature takes - clearing it would leave a
        // reclaimed territory with nothing to resist the next founding, which is a rule
        // nobody wrote. **The argument above still holds**: the exception is one the
        // specification states, where `Extractor` was one the code invented.
        self.held.retain(|thing| thing.kind == Kind::Nature);
    }
}
