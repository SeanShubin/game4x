//! A territory: what it holds, what it can produce, and what it is worth in force.
//!
//! In this release a territory is self-contained. No resource and no citizen crosses a
//! boundary, which is why ending a turn can resolve every territory independently and in
//! any order - see [`crate::game::Game::after`].

/// What one store holds, whatever it was built for.
///
/// `releases/first-release.md` -> *Where things are*: **a store holds the resource it was
/// built for, up to 10.** A fact about the kind and not about any one store -
/// `spec/logistics.md` says what a kind may contain is a fact about the kind.
///
/// **This number was decided and then lost for a day.** `P-260` asked for it, Sean answered,
/// and the promotion asserted the two countable things around it - thirteen kinds, sixteen
/// recipes - while dropping the number itself, so nothing noticed it was gone. It was
/// relayed between lanes as settled while appearing in no document, and `P-265` is what
/// finally wrote it down. `C-27`.
pub const HOLDS: u32 = 10;

use crate::Biome;
use crate::identity::{Resource, TerritoryId};
use crate::thing::{Kind, Thing, Trait};
use std::collections::BTreeMap;

/// What one citizen is worth in violence. `releases/first-release.md`: Citizen, force 1.
pub const CITIZEN_FORCE: u32 = 1;

/// What a territory offers for one resource.
///
/// `spec/planet.md`: *for each resource, a territory has total capacity for some number of
/// extractors, and a density that each of them yields.* Two numbers, which is what the
/// release's *Territory resources* table writes as `3 x 4`.
///
/// **`P-290` is why this is one value rather than a list.** Capacity may be declared per
/// kind carrying a particular value of a trait, so a territory bounds *metal extractors*
/// directly and nothing has to model the individual deposits an extractor sits on. The
/// predecessor was a `Node { resource, density }` per extractor slot, and since no
/// territory-resource pair has two densities, no two nodes of one resource ever differed -
/// the list was `capacity` copies of one fact. `S-48`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Deposit {
    /// How many extractors of this resource the territory has total capacity for.
    pub capacity: u32,
    /// What each of them yields when worked.
    pub density: u32,
}

/// A structure worked for one resource.
///
/// `spec/structures.md`: once per turn it may take a unit of labor from a citizen and
/// produce its territory's density in that resource. `exhausted` is that "once per turn" -
/// the extractor is not consumed by working, only used up until the turn ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extractor {
    /// What it was built for. It was an index into a list of nodes until `P-290` removed
    /// the need for the list.
    pub resource: Resource,
    pub exhausted: bool,
}

/// `spec/control.md`: the structure through which the citizens of a territory apply force.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Garrison {
    /// Force of its own, which is **zero** since `P-277`.
    ///
    /// It was *one less force than the founding unit*. `spec/control.md` now says a
    /// garrison has none, and the release's *Units and structures* row says 0 - the field
    /// stays because a garrison is a thing with traits like any other and the release still
    /// gives it a Force cell, which happens to hold zero.
    pub force: u32,
}

impl Garrison {
    /// The garrison a founding unit of this force becomes.
    /// **`P-276` and `P-277`: a garrison has no force of its own.** It was *one less force
    /// than the founding unit*, which is 1 for both units that found; the release's *Units
    /// and structures* row now says **0**, and `spec/control.md` says it outright.
    ///
    /// The unit's force is still the argument because the caller has it and because a
    /// founding still turns a unit into this - what changed is what the result presents, not
    /// where it came from.
    pub fn from_founding_unit(_unit_force: u32) -> Self {
        Self { force: 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Territory {
    pub id: TerritoryId,
    /// What the terrain gives this ground. `spec/planet.md` puts it under what a territory
    /// carries, beside the id and what it offers, rather than under presentation - the
    /// realistic drawing illustrates this fact rather than inventing one.
    pub biome: Biome,
    /// What this ground offers, per resource. A resource with no entry offers nothing.
    ///
    /// **A map rather than `[Deposit; 3]`**, for the reason `held` records below: an array
    /// indexed by resource cannot carry a fourth resource, and a map adds nothing when one
    /// arrives.
    pub deposits: BTreeMap<Resource, Deposit>,
    /// `spec/control.md`: force inherent to the territory, which nature holds it with.
    pub force_of_nature: u32,
    /// What is here now, as things rather than as fields.
    ///
    /// `spec/logistics.md`: there is no general inventory, so this is per territory and
    /// nothing crosses a boundary.
    ///
    /// **`stores: [u32; 3]` could not carry a fourth resource** and, more to the point, made
    /// a unit of food something other than a thing in a place - which is what
    /// `spec/invariants.md` says the state is. Adding a resource added an array element;
    /// now it adds nothing, because a resource is a kind and a kind is already general.
    ///
    /// Held as one `Thing` per unit. Twelve food is twelve things, because *how many of
    /// each* is a fact the state is read for rather than a compression of it.
    pub held: Vec<Thing>,
}

impl Territory {
    /// Ground with a biome and nothing on it yet.
    ///
    /// The biome is asked for rather than defaulted, because there is no such thing as a
    /// territory without one and a default would be a fact nothing in the world put there.
    pub fn empty(id: TerritoryId, biome: Biome) -> Self {
        Self {
            id,
            biome,
            deposits: BTreeMap::new(),
            force_of_nature: 0,

            held: Vec::new(),
        }
    }

    pub fn store(&self, resource: Resource) -> u32 {
        self.held
            .iter()
            .filter(|thing| thing.kind == Kind::from_resource(resource))
            .count() as u32
    }

    pub fn add(&mut self, resource: Resource, amount: u32) {
        for _ in 0..amount {
            self.held.push(Thing::of(Kind::from_resource(resource)));
        }
    }

    pub fn take(&mut self, resource: Resource, amount: u32) {
        let kind = Kind::from_resource(resource);
        let mut left = amount;
        self.held.retain(|thing| {
            if left > 0 && thing.kind == kind {
                left -= 1;
                false
            } else {
                true
            }
        });
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
    pub fn end_of_turn_losses(&mut self) {
        self.held.retain(|thing| {
            // Food keeps for one turn, so what is here at the end was made this turn and
            // expires now.
            thing.kind != Kind::Food && thing.kind != Kind::Labor
        });
        for resource in [Resource::Metal, Resource::Energy] {
            let kind = Kind::from_resource(resource);
            let over = self.count_of(kind).saturating_sub(self.capacity(resource));
            if over > 0 {
                self.remove(kind, over);
            }
        }
    }

    /// The garrison holding this ground, if there is one.
    ///
    /// **A thing, since `S-21` finished.** It was `Option<Garrison>` with three named
    /// fields, so a garrison was the one structure the state knew by name - which is what
    /// `spec/invariants.md` forbids: *nothing in the state is special to a kind*.
    pub fn garrison(&self) -> Option<Garrison> {
        self.held
            .iter()
            .find(|thing| thing.kind == Kind::Garrison)
            .map(|thing| Garrison {
                force: thing.trait_of(Trait::Force).unwrap_or(0),
            })
    }

    pub fn set_garrison(&mut self, garrison: Option<Garrison>) {
        self.held.retain(|thing| thing.kind != Kind::Garrison);
        if let Some(garrison) = garrison {
            self.held
                .push(Thing::of(Kind::Garrison).with(Trait::Force, garrison.force));
        }
    }

    /// Every extractor here, in the order they were built.
    pub fn extractors(&self) -> Vec<Extractor> {
        self.held
            .iter()
            .filter(|thing| thing.kind == Kind::Extractor)
            .map(|thing| Extractor {
                resource: thing
                    .trait_of(Trait::Resource)
                    .and_then(|at| Resource::ALL.get(at as usize).copied())
                    .unwrap_or(Resource::Food),
                exhausted: !thing.is_ready(),
            })
            .collect()
    }

    /// Build one for a resource.
    pub fn add_extractor(&mut self, resource: Resource) {
        self.held
            .push(Thing::of(Kind::Extractor).with(Trait::Resource, resource.index() as u32));
    }

    /// How many stores this territory has for a resource.
    ///
    /// `P-260`: a store is one kind with a `Resource` trait, the same shape an extractor
    /// has had since `P-234` - templating rather than three kinds differing in one word.
    pub fn stores(&self, resource: Resource) -> usize {
        self.held
            .iter()
            .filter(|thing| thing.kind == Kind::Store)
            .filter(|thing| thing.trait_of(Trait::Resource) == Some(resource.index() as u32))
            .count()
    }

    /// Build a store for a resource.
    pub fn add_store(&mut self, resource: Resource) {
        self.held
            .push(Thing::of(Kind::Store).with(Trait::Resource, resource.index() as u32));
    }

    /// How many stores this territory could ever have for a resource.
    ///
    /// `releases/first-release.md` -> *What bounds a kind*: a store is bounded by **as many
    /// as the extractors of its resource**, the founding one counted. So the bound is the
    /// territory's capacity for extractors of that resource.
    pub fn store_capacity(&self, resource: Resource) -> usize {
        self.capacity_for(resource)
    }

    /// What this territory can keep of a resource between turns.
    ///
    /// **`P-258`: a territory declares no capacity for a resource.** It declares capacity
    /// for the things that hold them, and this is derived from those - how many stores it
    /// has, times what a store holds. It replaced a flat bound of twenty on the territory
    /// itself, which was a property of the wrong thing: a territory that had built nothing
    /// could keep twenty of everything.
    ///
    /// `P-256`, decided yes: the report shows this. Sean went looking for the number, could
    /// not find it, and asked - the one report addition requested by hitting its absence.
    pub fn capacity(&self, resource: Resource) -> u32 {
        self.stores(resource) as u32 * HOLDS
    }

    /// Spend the readiness of the extractor at that position.
    pub fn exhaust_extractor(&mut self, at: usize) {
        if let Some(thing) = self
            .held
            .iter_mut()
            .filter(|thing| thing.kind == Kind::Extractor)
            .nth(at)
        {
            thing.set(Trait::Ready, 0);
        }
    }

    /// Whether a player holds this ground.
    ///
    /// **`S-19`: derived, not stored.** `releases/first-release.md` gives `control` as
    /// *derived: a citizen of that player is there*, and `spec/invariants.md` says a derived
    /// trait cannot be left wrong **because nothing writes one**. `founded: bool` was
    /// written in four places and cleared in one, and every one of them was a chance for it
    /// to disagree with the citizens it was meant to describe.
    ///
    /// A population that starves to nothing therefore loses the ground, without anybody
    /// remembering to say so.
    pub fn founded(&self) -> bool {
        self.citizens() > 0
    }

    /// How many of a kind are here.
    ///
    /// **The one way to ask.** `citizens` and `yards` were fields, so counting a citizen and
    /// counting a yard were different operations and counting a kind nobody had thought of
    /// was impossible. `spec/invariants.md`: *whatever reads the state reads it the same way
    /// whatever kind it holds.*
    pub fn count_of(&self, kind: Kind) -> u32 {
        self.held.iter().filter(|thing| thing.kind == kind).count() as u32
    }

    pub fn citizens(&self) -> u32 {
        self.count_of(Kind::Citizen)
    }

    pub fn yards(&self) -> u32 {
        self.count_of(Kind::Yard)
    }

    /// Put this many of a kind here.
    pub fn put(&mut self, kind: Kind, count: u32) {
        for _ in 0..count {
            self.held.push(Thing::of(kind));
        }
    }

    /// Remove up to this many of a kind, and say how many went.
    pub fn remove(&mut self, kind: Kind, count: u32) -> u32 {
        let mut left = count;
        self.held.retain(|thing| {
            if left > 0 && thing.kind == kind {
                left -= 1;
                false
            } else {
                true
            }
        });
        count - left
    }

    /// Leave exactly this many of a kind here.
    pub fn set_count(&mut self, kind: Kind, count: u32) {
        let now = self.count_of(kind);
        if now > count {
            self.remove(kind, now - count);
        } else {
            self.put(kind, count - now);
        }
    }

    /// Labor not yet spent this turn. A citizen provides one each turn.
    pub fn labor_available(&self) -> u32 {
        self.held
            .iter()
            .filter(|thing| thing.kind == Kind::Citizen && thing.is_ready())
            .count() as u32
    }

    /// Labor already made and spent this turn, which is a citizen no longer ready.
    pub fn labor_spent(&self) -> u32 {
        self.citizens() - self.labor_available()
    }

    /// Spend a citizen's readiness, which is what `create labor` consumes.
    ///
    /// **`P-231`: labor is a kind, and `labor_spent: u32` was the model disagreeing with the
    /// release.** The release's `create labor` takes a *citizen, ready* and gives back a
    /// *citizen, exhausted* and a *labor* - so what was spent is a trait of the citizen, not
    /// a counter beside it. Adding a kind added a field, which is what the rewrite removes.
    ///
    /// **The seam Sean approved:** `work` still creates and consumes the labor in one step,
    /// so `scenario/commands/play.4x` does not change and `scenario/expected/play.4x` stays a check on this
    /// rewrite rather than something regenerated with it. `P-232` is where the labor becomes
    /// visible between the two halves, by a command or by a rule for when the world fires
    /// one; either is a small change from here, because the kind already exists.
    pub fn spend_labor(&mut self, amount: u32) {
        let mut left = amount;
        for thing in &mut self.held {
            if left == 0 {
                break;
            }
            if thing.kind == Kind::Citizen && thing.is_ready() {
                thing.spend_readiness();
                left -= 1;
            }
        }
    }

    /// What this ground offers for one resource, which may be nothing.
    pub fn deposit(&self, resource: Resource) -> Deposit {
        self.deposits.get(&resource).copied().unwrap_or_default()
    }

    /// What one extractor of this resource yields when worked.
    pub fn density_of(&self, resource: Resource) -> u32 {
        self.deposit(resource).density
    }

    /// How many extractors this territory has total capacity for, for a resource.
    ///
    /// **`P-290`: capacity is per kind carrying a particular value of a trait**, so this
    /// bounds *extractors of this resource* directly. It used to be the number of nodes,
    /// which is the same number counted the long way round.
    pub fn capacity_for(&self, resource: Resource) -> usize {
        self.deposit(resource).capacity as usize
    }

    pub fn extractors_for(&self, resource: Resource) -> Vec<usize> {
        self.extractors()
            .iter()
            .enumerate()
            .filter(|(_, extractor)| extractor.resource == resource)
            .map(|(at, _)| at)
            .collect()
    }

    /// The greatest output this territory can ever reach, as the state that produces it.
    ///
    /// `spec/control.md`: *a territory produces the greatest output it can when as many
    /// citizens as it can feed are working, as many of them at its food extractors as those
    /// will take - which is what sets the population - and every remaining citizen at a metal
    /// or an energy extractor, the player choosing how they divide.*
    ///
    /// Returned as `(citizens, food extractors, other extractors)`, because the condition is
    /// about a state rather than a number: the output itself is not stored anywhere, and what
    /// can be looked at is whether the territory has the people and the structures that
    /// produce it.
    ///
    /// # Why it is not simply capacity times density
    ///
    /// `spec/control.md` says the answer follows from *how many extractors it has total
    /// capacity for, their densities, and its biome* - and two of the release's twelve
    /// territories reach a ceiling below that, for reasons that are themselves permanent
    /// facts rather than history.
    ///
    /// **Building an extractor costs a metal and a labor.** A territory has spare labor only
    /// when a food extractor feeds more than the citizen working it, which is density two or
    /// more; and it has metal only if it has metal capacity, since no resource crosses a
    /// territory boundary. **A territory with neither can never build anything**, so its
    /// ceiling is what founding left it: one food extractor.
    ///
    /// Territory 5 is the first case - three food deposits, every one of density one, so
    /// each farmer eats exactly what they gather and there is never a spare hand. Territory 6
    /// is the second - food four by four, and no metal deposit at all, so the fourth citizen
    /// has nothing to build with. Both reach their ceiling on the turn they are founded, and
    /// under the wording `P-361` replaced neither could ever finish, which is why the release
    /// could not be won.
    ///
    /// **Nothing here reads history**, which is what `P-125` rejected an earlier definition
    /// for. Every input is a `(capacity, density)` pair off *Territory resources*.
    ///
    /// # One cell of the note this is derived from disagrees, and it does not change anything
    ///
    /// `docs/notes/2026-09-10-maximum-possible-output.md` gives territory 5 a `Cmax` of 3,
    /// which is its food capacity times its density. Its own prose says the opposite two
    /// paragraphs later - *it starves to one citizen and holds there for ever ... its maximum
    /// possible output is one food* - and the table's own row 6 applies the same reasoning
    /// this does, giving a `Cmax` of 4 where capacity times density is 16. **The `Spare` and
    /// `Staffed` columns are unaffected either way**, so the conclusion the note draws stands
    /// on either reading. Reported as `C-78`.
    pub fn maximum_output(&self) -> (u32, usize, usize) {
        let food_capacity = self.capacity_for(Resource::Food);
        let food_density = self.density_of(Resource::Food);
        if food_capacity == 0 || food_density == 0 {
            // *A territory that cannot feed a citizen has no output to reach, and never holds
            // the condition open* - `spec/control.md`.
            return (0, 0, 0);
        }

        // Spare labor needs a food extractor to feed more than its own worker; metal to build
        // with needs metal capacity, because nothing crosses a boundary.
        let can_ever_build = food_density >= 2 && self.capacity_for(Resource::Metal) >= 1;
        let food_extractors = if can_ever_build { food_capacity } else { 1 };

        let citizens = food_extractors as u32 * food_density;
        let spare = citizens.saturating_sub(food_extractors as u32) as usize;

        // **A territory that can never build has only what founding left it**, which is one
        // food extractor and - where there is a deposit to attach it to - one metal
        // extractor. Counting its whole capacity here was wrong by three on territory 6: it
        // has energy capacity 4 and no metal, and building an energy extractor costs a metal
        // it can never obtain. Spare hands with nothing to build are not output.
        let elsewhere = if can_ever_build {
            self.capacity_for(Resource::Metal) + self.capacity_for(Resource::Energy)
        } else {
            self.capacity_for(Resource::Metal).min(1)
        };

        (citizens, food_extractors, spare.min(elsewhere))
    }

    /// Whether this territory is producing the greatest output it can - `spec/control.md`.
    ///
    /// **At least, rather than exactly.** An extractor nobody staffs adds no output and takes
    /// none away, so a territory that has built more than it can work has not failed the
    /// condition; what the condition asks is that nothing more is being produced than is.
    pub fn at_maximum_output(&self) -> bool {
        let (citizens, food_extractors, elsewhere) = self.maximum_output();
        let others = Resource::ALL
            .iter()
            .filter(|resource| **resource != Resource::Food)
            .map(|resource| self.extractors_for(*resource).len())
            .sum::<usize>();

        self.citizens() >= citizens
            && self.extractors_for(Resource::Food).len() >= food_extractors
            && others >= elsewhere
    }

    /// Every extractor this territory has total capacity for, across every resource.
    ///
    /// What *fully exploited* is measured against: it used to be `nodes.len()`, and the
    /// nodes were one per unit of capacity, so this is the same number stated directly.
    pub fn total_extractor_capacity(&self) -> usize {
        Resource::ALL
            .iter()
            .map(|resource| self.capacity_for(*resource))
            .sum()
    }

    /// Whether another extractor of this resource would fit.
    ///
    /// **The bookkeeping that stopped two extractors sharing a node is gone with the
    /// nodes.** Capacity is the whole of the rule now: an extractor fits while the ones
    /// already here are fewer than the territory has total capacity for.
    pub fn has_room_for_extractor(&self, resource: Resource) -> bool {
        self.extractors_for(resource).len() < self.capacity_for(resource)
    }

    /// Whether this territory can ever build an extractor, from its nodes alone.
    ///
    /// `spec/control.md`: *what that greatest output is follows from the territory's own
    /// permanent facts: how many extractors it has total capacity for, their densities, and
    /// its biome. Not whether the player can afford it this turn, and not whether any
    /// particular game happened to reach it.*
    ///
    /// **`P-361` rewrote that sentence and left this rule alone.** It used to say a
    /// *structure* can be built where those facts allow it; it now says the greatest
    /// *output* follows from them. The qualifier - permanent facts, never affordability and
    /// never history - is word for word what it was, and this function only ever used the
    /// qualifier.
    ///
    /// So the question is answered from what the ground offers and nothing else - not from
    /// what is standing there, and not from how the game went.
    ///
    /// **Population settles at the food the territory produces.** A citizen yields one
    /// labor and eats one food, so a food extractor sustains `density` citizens while
    /// costing the one holding it. A density-one extractor adds a citizen and eats what
    /// that citizen gathers; a density-two one leaves a hand over. **So spare labor exists
    /// exactly when a food extractor yields two or more**, and there is capacity for one.
    ///
    /// Territory 5 has capacity for three food extractors at density one, which is why it
    /// holds the one it was founded with and can never build a second.
    pub fn can_build_extractors(&self) -> bool {
        let food = self.deposit(Resource::Food);
        food.capacity >= 1 && food.density >= 2
    }

    /// The most of one resource this territory could produce in a single turn.
    ///
    /// Every extractor it has capacity for is built, and its citizens are split between
    /// food, which is what sets how many of them there are, and the resource asked for.
    ///
    /// **The search this used to do is gone, and `P-290` is what removed it.** One density
    /// per territory per resource means every food extractor buys the same `density - 1`
    /// spare hands, so working one more is never worse and the best split is always at the
    /// end. It was a maximisation over how many food extractors are worked because the
    /// densities could in principle differ; they never could.
    ///
    /// A territory with no food capacity has no population and so produces nothing, whatever
    /// else it offers. That falls out rather than being a case.
    pub fn most_in_one_turn(&self, resource: Resource) -> u32 {
        let food = self.deposit(Resource::Food);
        // Food asked for is the one case where the hands are already producing the answer.
        if resource == Resource::Food {
            return food.capacity * food.density;
        }
        let wanted = self.deposit(resource);
        // **Saturating, and the two reasons are not the same reason** - `Q-58` asked whether
        // one of them was real.
        //
        // A density-one food extractor feeds exactly the citizen holding it, so it buys no
        // hand: that is territory 5, and the arithmetic gives zero without any help.
        //
        // **Density zero is ground with no food capacity at all**, which no territory in the
        // release is and which the model produces anyway: `Territory::empty` is what
        // `create planet` makes, and `set resource` fills it in afterwards. `is_fully_exploited`
        // reaches `can_hold_yard` on whatever is there. Plain subtraction underflows on it -
        // verified by making the change and probing it, not by reading the type.
        let spare = food.capacity * food.density.saturating_sub(1);
        spare.min(wanted.capacity) * wanted.density
    }

    /// Whether this territory can ever hold a Yard.
    ///
    /// **`C-29`, and this rule has now been derived three times from three different
    /// premises.** A Yard costs fifteen metal, so the question is whether fifteen can ever be
    /// brought together here at once.
    ///
    /// - Under the original rule every store was discarded at the end of a turn, so it was
    ///   *fifteen in one turn*. `C-9` recorded that and it qualified four territories.
    /// - `C-11` had metal carry to a flat twenty, so it became *any metal at all reaches
    ///   fifteen by waiting*. Ten territories.
    /// - `P-258` moved the bound onto the things in a territory, and `P-270` made what is in
    ///   nothing spendable on the turn it is made. So it is what its stores can hold, plus
    ///   what one turn can make.
    ///
    /// A store holds ten and a territory may build as many as it has extractors of that
    /// resource, so **a territory with one metal node can never hold more than ten** and
    /// needs five a turn on top to reach a Yard. Territories 8 and 10 have one metal node
    /// each and make two and three a turn: twelve and thirteen, and no Yard, though they
    /// produce metal every turn. **Eight territories, not ten** - and the two it loses are
    /// not the two that produce nothing.
    ///
    /// **The compile-time assert that used to sit here did its job by failing.** It said
    /// `KEEPS >= YARD_METAL`, and `P-258` deleted `KEEPS`, so this stopped compiling the
    /// moment its premise moved rather than going quietly stale the way `C-9` did.
    pub fn can_hold_yard(&self) -> bool {
        let can_store = self.store_capacity(Resource::Metal) as u32 * HOLDS;
        let in_one_turn = self.most_in_one_turn(Resource::Metal);
        // Nothing at all in a turn means no labor to spare, so the stores would never fill.
        in_one_turn >= 1 && can_store + in_one_turn >= crate::game::cost::YARD_METAL
    }

    /// The force the territory itself presents, before any unit standing on it.
    ///
    /// `spec/control.md`: organised force sums, unorganised force is the highest present.
    /// A garrison is what organises citizens, so with one the garrison's own force and
    /// every manned citizen's contribution add up; without one, the citizens present the
    /// highest among them rather than the total.
    pub fn held_force(&self) -> u32 {
        match self.garrison() {
            // **`P-276`: a garrison has no force of its own and nothing has to work it.**
            // *It does one thing: it lets the citizens of that territory sum their force
            // instead of presenting only the highest among them. It does this by existing.*
            //
            // Two rules ago this read `garrison.force + manned * multiplier`, which dropped
            // idle citizens and left a taken jungle to nature - `C-31`. One rule ago it
            // added the idle ones back at their own force. Sean then changed the rule rather
            // than the number: the multiplier is gone from both documents and the garrison's
            // own force is zero, so what a founding leaves is exactly its two citizens.
            //
            // **Two against a jungle's nature of two, and holding takes force equal to
            // nature - so it holds exactly, and falls the moment it drops to one citizen.**
            // `spec/narrative.md` is why that is the point rather than a rounding artefact:
            // *more dangerous territory requires more organised citizens to keep it secure.*
            Some(garrison) => garrison.force + self.citizens() * CITIZEN_FORCE,
            // Citizens are capable of violence but not of coordination, so what they
            // present is the highest among them rather than the total - and a citizen is
            // force 1, so however many there are the answer is one.
            None if self.citizens() > 0 => CITIZEN_FORCE,
            None => 0,
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
        self.held.clear();
    }
}

/// How a population changes on the food it has.
///
/// `spec/population.md`: fewer food than citizens and each unfed citizen starves; equal
/// and nothing changes; more and one citizen is generated for each citizen with extra
/// food, so it at most doubles.
pub fn population_after(citizens: u32, food: u32) -> u32 {
    if food < citizens {
        food
    } else {
        citizens + (food - citizens).min(citizens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ground offering, per resource, capacity for that many extractors at that density.
    ///
    /// **The predecessor took one density per node**, so a case could give one resource two
    /// densities. `P-290` makes that unwriteable: a territory has one answer per resource.
    /// Three cases below were stated in the old shape and are re-stated here rather than
    /// translated, because there is nothing to translate them to.
    fn offering(deposits: &[(Resource, u32, u32)]) -> Territory {
        let mut territory = Territory::empty(TerritoryId(1), Biome::Grassland);
        for (resource, capacity, density) in deposits {
            territory.deposits.insert(
                *resource,
                Deposit {
                    capacity: *capacity,
                    density: *density,
                },
            );
        }
        territory
    }

    #[test]
    fn a_population_starves_down_to_the_food_available() {
        assert_eq!(population_after(5, 2), 2);
        assert_eq!(population_after(5, 0), 0);
    }

    #[test]
    fn a_population_with_exactly_enough_food_does_not_change() {
        assert_eq!(population_after(5, 5), 5);
        assert_eq!(population_after(0, 0), 0);
    }

    /// One new citizen for each citizen with extra food, so at most a doubling.
    #[test]
    fn a_population_grows_on_surplus_and_at_most_doubles() {
        assert_eq!(population_after(5, 7), 7, "two spare feed two new");
        assert_eq!(population_after(5, 10), 10, "five spare, five new");
        assert_eq!(population_after(5, 40), 10, "plenty still only doubles");
    }

    /// Nothing comes from nothing: `spec/invariants.md` forbids a quantity appearing with
    /// nothing in the world causing it, and a citizen comes only from a founding unit or
    /// from this rule applied to citizens already there.
    #[test]
    fn a_population_of_none_never_grows_however_much_food_there_is() {
        assert_eq!(population_after(0, 50), 0);
    }

    #[test]
    fn capacity_is_declared_per_resource_and_a_resource_absent_offers_nothing() {
        let territory = offering(&[(Resource::Food, 2, 4), (Resource::Metal, 1, 3)]);
        assert_eq!(territory.capacity_for(Resource::Food), 2);
        assert_eq!(territory.density_of(Resource::Food), 4);
        assert_eq!(territory.capacity_for(Resource::Metal), 1);
        assert_eq!(territory.capacity_for(Resource::Energy), 0);
        assert_eq!(
            territory.density_of(Resource::Energy),
            0,
            "no entry is no capacity and no density, rather than a missing answer"
        );
        assert_eq!(territory.total_extractor_capacity(), 3);
    }

    /// **Capacity is the whole of the rule, which is what `P-290` bought.**
    ///
    /// This replaces a test that built extractors densest-node-first and checked the order
    /// they were taken in. There is no order left to get wrong: every extractor of one
    /// resource here yields the same, so the only question an extractor can ask is whether
    /// there is room. The bookkeeping that stopped two extractors sharing a node went with
    /// the nodes.
    #[test]
    fn extractors_fit_until_the_capacity_for_that_resource_is_used() {
        let mut territory = offering(&[(Resource::Food, 2, 6), (Resource::Metal, 1, 3)]);
        assert!(territory.has_room_for_extractor(Resource::Food));
        territory.add_extractor(Resource::Food);
        assert!(
            territory.has_room_for_extractor(Resource::Food),
            "one of two"
        );
        territory.add_extractor(Resource::Food);
        assert!(
            !territory.has_room_for_extractor(Resource::Food),
            "two of two, and the third does not fit"
        );
        assert!(
            territory.has_room_for_extractor(Resource::Metal),
            "a resource is bounded by its own capacity and not by another's"
        );
        assert!(
            !territory.has_room_for_extractor(Resource::Energy),
            "and ground that offers no energy has room for no energy extractor"
        );
        assert_eq!(territory.extractors_for(Resource::Food).len(), 2);
        assert_eq!(territory.extractors_for(Resource::Metal).len(), 0);
    }

    /// Organised force sums; unorganised force is the highest present.
    #[test]
    fn a_garrison_lets_citizens_add_their_force_together() {
        let mut territory = offering(&[]);
        territory.set_count(Kind::Citizen, 4);
        assert_eq!(
            territory.held_force(),
            1,
            "uncoordinated, the highest present"
        );

        // **`P-276` and `P-277`.** A garrison has no force of its own and does one thing:
        // it lets the citizens sum instead of presenting only the highest. **It does that by
        // existing**, which is the whole assertion now that `manned` is gone - `S-72`. The
        // manning half set a field to 3 and checked the force had not moved; it could not
        // have moved, because nothing ever read that field.
        territory.set_garrison(Some(Garrison { force: 0 }));
        assert_eq!(territory.held_force(), 4, "four citizens, summed");
    }

    /// `spec/control.md` and the release: **a garrison has no force of its own.**
    ///
    /// It was *one less force than the founding unit*, which is 1 for both units that found.
    /// `P-277` made the *Units and structures* row 0 and `P-276` says it in prose, so what
    /// holds newly taken ground is its citizens rather than the structure that organises
    /// them.
    #[test]
    fn a_founding_unit_becomes_a_garrison_with_no_force_of_its_own() {
        assert_eq!(Garrison::from_founding_unit(2).force, 0);
    }

    #[test]
    fn ending_a_turn_makes_everything_ready_again() {
        let mut territory = offering(&[(Resource::Food, 1, 4)]);
        territory.set_count(Kind::Citizen, 2);
        territory.spend_labor(2);
        // Built, then worked - `extractors()` hands back a copy, so pushing to it changed a
        // temporary and left the territory with none. That is what made this fail with an
        // index out of bounds rather than with a wrong answer.
        territory.add_extractor(Resource::Food);
        territory.exhaust_extractor(0);
        territory.set_garrison(Some(Garrison { force: 1 }));

        territory.make_ready();
        assert_eq!(territory.labor_available(), 2);
        assert!(!territory.extractors()[0].exhausted);
    }

    #[test]
    fn nature_taking_a_territory_back_leaves_nothing_of_it() {
        let mut territory = offering(&[(Resource::Food, 1, 4)]);
        // Six citizens is what makes it founded now - `S-19`, control derived rather than
        // stored. Setting a flag beside them was the thing that could disagree with them.
        territory.set_count(Kind::Citizen, 6);
        assert!(territory.founded(), "citizens are what holding it means");
        territory.set_garrison(Some(Garrison::from_founding_unit(2)));
        territory.add_extractor(Resource::Food);
        territory.add(Resource::Metal, 10);
        // **A yard, which none of the named assertions below looks at** - `X-27`. Without a
        // kind that only the whole-population check covers, that check catches nothing the
        // four named ones do not, and a mutation leaving a kind behind passes. Verified by
        // running one rather than by reasoning about it.
        territory.set_count(Kind::Yard, 1);

        // **What it held, over every kind rather than four named ones** - `X-27`. The four
        // assertions below used to be the whole of this, and they are a list to keep in step
        // with the kinds, which is what `lost_to_nature` is written the way it is to avoid. A
        // kind added tomorrow and left behind by nature would pass all four.
        let before = territory.held.len();
        let kinds_before: std::collections::BTreeSet<Kind> =
            territory.held.iter().map(|thing| thing.kind).collect();
        assert!(
            kinds_before.len() >= 3,
            "only {} kinds were on it, too few for `nothing remains` to be a claim",
            kinds_before.len()
        );

        territory.lost_to_nature();

        assert_eq!(
            territory.held.len(),
            0,
            "{before} things of {} kinds were on it and nature left {} - it takes all of \
             them, whatever kinds exist",
            kinds_before.len(),
            territory.held.len()
        );

        assert!(!territory.founded());
        assert_eq!(territory.citizens(), 0, "its entire population perishes");
        assert!(territory.garrison().is_none());
        assert!(territory.extractors().is_empty());
        assert_eq!(territory.store(Resource::Metal), 0);
        assert_eq!(
            territory.capacity_for(Resource::Food),
            1,
            "the land itself remains"
        );
    }

    /// The extractor rule, either side of its two boundaries and not on an example of one.
    ///
    /// `C-9`. A citizen yields one labor and eats one food, so a food extractor sustains
    /// `density` citizens while occupying one of them. A density-one extractor adds a hand
    /// and eats what it gathers; a density-two one adds a citizen and half feeds another.
    /// **So spare labor exists exactly when a food extractor yields two or more, and there
    /// is capacity for one at all.**
    ///
    /// **There are two boundaries now and there used to be one.** The old rule asked only
    /// about density, because a node was both the capacity and the density and a list of
    /// zero nodes answered the capacity question implicitly. Splitting them makes *dense
    /// ground with no capacity* a state that can be written, so it is a case.
    #[test]
    fn a_spare_hand_exists_exactly_when_a_food_extractor_yields_two() {
        let cases: [(&[(Resource, u32, u32)], bool, &str); 6] = [
            (&[], false, "no food at all is no population and no labor"),
            (
                &[(Resource::Food, 1, 1)],
                false,
                "one citizen, working the one extractor that feeds it",
            ),
            (
                &[(Resource::Food, 3, 1)],
                false,
                "three of them, and each still eats what it gathers - territory 5",
            ),
            (
                &[(Resource::Food, 1, 2)],
                true,
                "two fed, one extractor worked, one hand spare",
            ),
            (
                &[(Resource::Food, 0, 6)],
                false,
                "the second boundary: dense ground with no capacity feeds nobody, and it \
                 could not be written before capacity was a number of its own",
            ),
            (
                &[(Resource::Metal, 1, 9), (Resource::Food, 1, 1)],
                false,
                "metal it cannot reach does not feed anyone",
            ),
        ];
        for (deposits, expected, why) in cases {
            let territory = offering(deposits);
            assert_eq!(territory.can_build_extractors(), expected, "{why}");
        }
        assert_eq!(cases.len(), 6, "six cases, and two boundaries between them");
    }

    /// The most of a resource one turn can yield.
    ///
    /// **This was a search and is now a formula, and `P-290` is what collapsed it.** It
    /// maximised over how many food extractors were worked, because with a density per node
    /// the best split could be at neither end. One density per territory per resource means
    /// every food extractor buys the same `density - 1` spare hands, so working one more is
    /// never worse and the answer is always at the end: **all the food capacity worked, the
    /// hands left over spent on the resource asked for, up to its capacity.**
    ///
    /// The case that used to demonstrate the interior split is kept and its numbers are
    /// unchanged - capacity 2 at density 2 feeding three metal extractors is still 18 - but
    /// it demonstrates the formula now rather than the search.
    #[test]
    fn the_most_in_one_turn_is_the_spare_hands_against_the_capacity() {
        let cases: [(&[(Resource, u32, u32)], Resource, u32, &str); 6] = [
            (
                &[(Resource::Food, 1, 4)],
                Resource::Metal,
                0,
                "no metal capacity, no metal",
            ),
            (
                &[(Resource::Food, 1, 4), (Resource::Metal, 1, 9)],
                Resource::Metal,
                9,
                "four fed, one holds food, three hands spare and room for one metal extractor",
            ),
            (
                &[(Resource::Food, 1, 4), (Resource::Metal, 2, 9)],
                Resource::Metal,
                18,
                "three spare hands reach both metal extractors, and the third has nothing to work",
            ),
            (
                &[(Resource::Food, 1, 1), (Resource::Metal, 1, 9)],
                Resource::Metal,
                0,
                "one citizen, and it is working the food",
            ),
            (
                &[(Resource::Food, 2, 2), (Resource::Metal, 3, 9)],
                Resource::Metal,
                18,
                "two food extractors at density two feed four and cost two, so two spare - \
                 the capacity for three metal is what is not reached",
            ),
            (
                &[(Resource::Food, 2, 4)],
                Resource::Food,
                8,
                "asked for food, the answer is what the citizens gathered",
            ),
        ];
        for (deposits, resource, expected, why) in cases {
            let territory = offering(deposits);
            assert_eq!(territory.most_in_one_turn(resource), expected, "{why}");
        }
        assert_eq!(
            cases.len(),
            6,
            "six cases: two where the hands run out, two where the capacity does, one with \
             no capacity at all and one asking for food itself"
        );
    }

    /// Ground with no food capacity answers zero rather than underflowing.
    ///
    /// **`Q-58`, declined with this.** The lens read the saturating subtraction's comment,
    /// which gave two reasons for it, and observed that one of them - density zero - has no
    /// case in the release's territory table or in the case table above. That is true of
    /// both tables and the conclusion does not follow: `Territory::empty` has no deposits at
    /// all, `create planet` makes twelve of them before `set resource` fills any in, and
    /// `is_fully_exploited` asks `can_hold_yard` about whatever is standing there.
    ///
    /// **Checked by making the change rather than by arguing about it.** With plain
    /// subtraction the whole suite stays green - the lens was right about that - and this
    /// panics with *attempt to subtract with overflow*. So the case is real, no test covered
    /// it, and the finding was worth more than the fix would have been.
    #[test]
    fn ground_with_no_food_at_all_produces_nothing_rather_than_underflowing() {
        let bare = Territory::empty(TerritoryId(1), Biome::Grassland);
        assert_eq!(bare.deposit(Resource::Food), Deposit::default());
        for resource in Resource::ALL {
            assert_eq!(
                bare.most_in_one_turn(resource),
                0,
                "no food capacity is no population, so nothing is produced"
            );
        }
        assert!(!bare.can_hold_yard(), "and nothing can be built on it");
    }

    /// A Yard needs fifteen metal to be holdable at once, which is stores plus one turn.
    ///
    /// **Third derivation of one rule, and the premise moved under it twice** - `C-29`.
    /// `C-9` said *fifteen in one turn*, correct while every store was discarded at a turn's
    /// end. `C-11` had metal carry to a flat twenty and it became *any metal at all*, which
    /// qualified ten territories. `P-258` then moved the bound onto the things in a
    /// territory and `P-270` made what is in nothing spendable the turn it is made, so it is
    /// now **what the stores can hold, plus what one turn makes**.
    ///
    /// The last case is the one that changed answer and is why the count is eight rather
    /// than ten: capacity for one metal extractor is one store is ten, and ten plus a slow
    /// turn never reaches fifteen. A territory can produce metal every turn of the game and
    /// never afford a Yard, which the previous rule could not express.
    #[test]
    fn a_yard_needs_metal_to_be_reachable_rather_than_reachable_at_once() {
        let cases: [(&[(Resource, u32, u32)], bool, &str); 5] = [
            (
                &[(Resource::Food, 1, 4)],
                false,
                "no metal capacity, so never",
            ),
            (
                &[(Resource::Food, 1, 1), (Resource::Metal, 1, 20)],
                false,
                "metal in the ground with nobody free to dig it is no metal",
            ),
            (
                &[(Resource::Food, 1, 4), (Resource::Metal, 3, 4)],
                true,
                "capacity for three, so three stores and thirty; twelve a turn on top",
            ),
            (
                &[(Resource::Food, 1, 4), (Resource::Metal, 1, 5)],
                true,
                "one store of ten and five a turn is exactly fifteen - the boundary",
            ),
            (
                &[(Resource::Food, 1, 4), (Resource::Metal, 1, 4)],
                false,
                "one store of ten and four a turn is fourteen, and it is never fifteen however long the game runs. Territories 8 and 10 are this case",
            ),
        ];
        for (deposits, expected, why) in cases {
            let territory = offering(deposits);
            assert_eq!(territory.can_hold_yard(), expected, "{why}");
        }
        assert_eq!(
            cases.len(),
            5,
            "five cases, and two of them either side of fifteen"
        );
    }
}
