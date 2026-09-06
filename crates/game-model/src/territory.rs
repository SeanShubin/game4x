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

/// What one citizen is worth in violence. `releases/first-release.md`: Citizen, force 1.
pub const CITIZEN_FORCE: u32 = 1;

/// One deposit. `spec/planet.md`: a territory has zero or more nodes for each resource,
/// and each node has a density.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node {
    pub resource: Resource,
    pub density: u32,
}

/// A structure working one node.
///
/// `spec/structures.md`: once per turn it may take a unit of labor from a citizen and
/// produce that node's density in its resource. `exhausted` is that "once per turn" - the
/// extractor is not consumed by working, only used up until the turn ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extractor {
    /// Which of the territory's nodes this works, by position in [`Territory::nodes`].
    pub node: usize,
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
    /// What a citizen working here produces in force.
    /// Citizens working here this turn.
    pub manned: u32,
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
        Self {
            force: 0,
            manned: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Territory {
    pub id: TerritoryId,
    /// What the terrain gives this ground. `spec/planet.md` puts it under what a territory
    /// carries, beside the id and the nodes, rather than under presentation - the realistic
    /// drawing illustrates this fact rather than inventing one.
    pub biome: Biome,
    pub nodes: Vec<Node>,
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
            nodes: Vec::new(),
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
                manned: thing.trait_of(Trait::Manned).unwrap_or(0),
            })
    }

    pub fn set_garrison(&mut self, garrison: Option<Garrison>) {
        self.held.retain(|thing| thing.kind != Kind::Garrison);
        if let Some(garrison) = garrison {
            self.held.push(
                Thing::of(Kind::Garrison)
                    .with(Trait::Force, garrison.force)
                    .with(Trait::Manned, garrison.manned),
            );
        }
    }

    /// Set how many citizens are manning the garrison this turn.
    ///
    /// **`garrison()` returns a copy, so mutating what it hands back changes nothing.**
    /// That is the cost of the accessor and it needs an operation rather than a field: the
    /// test for this wrote `garrison().as_mut().unwrap().manned = 3` and silently mutated a
    /// temporary. Production never did - it went through `set_garrison` - but nothing would
    /// have said so if it had.
    pub fn man_garrison(&mut self, citizens: u32) {
        for thing in &mut self.held {
            if thing.kind == Kind::Garrison {
                thing.set(Trait::Manned, citizens);
            }
        }
    }

    /// Every extractor here, in the order they were built.
    pub fn extractors(&self) -> Vec<Extractor> {
        self.held
            .iter()
            .filter(|thing| thing.kind == Kind::Extractor)
            .map(|thing| Extractor {
                node: thing.trait_of(Trait::Works).unwrap_or(0) as usize,
                exhausted: !thing.is_ready(),
            })
            .collect()
    }

    /// Build one, working that node.
    pub fn add_extractor(&mut self, node: usize) {
        let resource = self.nodes[node].resource;
        self.held.push(
            Thing::of(Kind::Extractor)
                .with(Trait::Works, node as u32)
                .with(Trait::Resource, resource.index() as u32),
        );
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
    /// node count, since that is what bounds the extractors.
    pub fn store_capacity(&self, resource: Resource) -> usize {
        self.node_count(resource)
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

    /// Every node of one resource, with its position, in id order.
    pub fn nodes_of(&self, resource: Resource) -> Vec<(usize, Node)> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.resource == resource)
            .map(|(at, node)| (at, *node))
            .collect()
    }

    /// How many extractors this territory could have for a resource.
    ///
    /// `spec/economy.md`: the number of nodes determines the number of extractors that
    /// can be built for that resource there.
    pub fn node_count(&self, resource: Resource) -> usize {
        self.nodes_of(resource).len()
    }

    pub fn extractors_for(&self, resource: Resource) -> Vec<usize> {
        self.extractors()
            .iter()
            .enumerate()
            .filter(|(_, extractor)| self.nodes[extractor.node].resource == resource)
            .map(|(at, _)| at)
            .collect()
    }

    /// The next node of this resource with no extractor on it, densest first.
    ///
    /// Densest first so that building an extractor without saying which node takes the
    /// best one left, which is what a player would mean. Ties break on node position, so
    /// the choice never depends on iteration order.
    pub fn best_free_node(&self, resource: Resource) -> Option<usize> {
        let taken: Vec<usize> = self.extractors().iter().map(|e| e.node).collect();
        self.nodes_of(resource)
            .into_iter()
            .filter(|(at, _)| !taken.contains(at))
            .max_by_key(|(at, node)| (node.density, std::cmp::Reverse(*at)))
            .map(|(at, _)| at)
    }

    /// Whether this territory can ever build an extractor, from its nodes alone.
    ///
    /// `spec/control.md`: *a structure can be built where the territory's own permanent
    /// facts allow it: how many it has total capacity for, their densities, its biome. Not
    /// whether the player can afford it this turn, and not whether any particular game
    /// happened to reach it.*
    ///
    /// So the question is answered from `nodes` and nothing else - not from what is
    /// standing there, and not from how the game went.
    ///
    /// **Population settles at the food the territory produces.** A citizen yields one
    /// labor and eats one food, so working the `k` densest food nodes sustains `F(k)`
    /// citizens while costing `k` hands, leaving `F(k) - k` spare. Working only the best
    /// one leaves `d - 1`, and a node of density one adds a hand and eats it - so a spare
    /// hand exists for some allocation exactly when the best food node has density two or
    /// more.
    ///
    /// Territory 5's nineteen nodes are all density one, which is why it holds the one
    /// extractor it was founded with and can never build a twentieth.
    pub fn can_build_extractors(&self) -> bool {
        self.nodes_of(Resource::Food)
            .iter()
            .any(|(_, node)| node.density >= 2)
    }

    /// The most of one resource this territory could produce in a single turn.
    ///
    /// Every extractor it can build is built, and its hands are split between food, which
    /// is what sets how many hands there are, and the resource asked for. Maximised over
    /// how many food nodes are worked, because working one more food node buys `f - 1`
    /// spare hands and there is no reason the best split is at either end.
    ///
    /// A territory with no food node has no population and so produces nothing, whatever
    /// its other nodes say. That falls out rather than being a case: `F(0)` is zero, no
    /// hands, nothing worked.
    pub fn most_in_one_turn(&self, resource: Resource) -> u32 {
        let mut food: Vec<u32> = self
            .nodes_of(Resource::Food)
            .iter()
            .map(|(_, node)| node.density)
            .collect();
        food.sort_unstable_by(|a, b| b.cmp(a));

        let mut wanted: Vec<u32> = self
            .nodes_of(resource)
            .iter()
            .map(|(_, node)| node.density)
            .collect();
        wanted.sort_unstable_by(|a, b| b.cmp(a));

        // Food asked for is the one case where the two lists are the same list: the hands
        // working food are already producing it, so the answer is the largest `F(k)`.
        let mut best = 0;
        for worked in 0..=food.len() {
            let produced: u32 = food.iter().take(worked).sum();
            if resource == Resource::Food {
                best = best.max(produced);
                continue;
            }
            // Citizens are what the food sustains, and `worked` of them are holding food
            // nodes. Saturating because a territory can work more food nodes than it can
            // sustain hands for, and that allocation simply has nothing spare.
            let spare = produced.saturating_sub(worked as u32) as usize;
            best = best.max(wanted.iter().take(spare).sum());
        }
        best
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
        // Nothing at all in a turn means no hands to spare, so the stores would never fill.
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
        // and `garrison()` hand back owned values now, so `extractor.exhausted = false` and
        // `garrison.manned = 0` changed a temporary and vanished. The refresh above already
        // readies every extractor, because it names no kind; the manning needed an
        // operation rather than a field.
        self.man_garrison(0);
    }

    /// What nature does when it takes a territory back.
    ///
    /// `spec/control.md`: its entire population perishes and any ark on it becomes
    /// unusable. The ark is dealt with by the caller, which is the only place that knows
    /// where units are.
    pub fn lost_to_nature(&mut self) {
        self.set_garrison(None);
        self.held.retain(|thing| thing.kind != Kind::Extractor);
        // Everything held goes, which is the one place `clear` is the right verb: nature
        // takes the population, the stores and the yards together. Naming the kinds one by
        // one would be a list to keep in step with the kinds, which is the thing this shape
        // exists to stop.
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

    fn with_nodes(densities: &[(Resource, u32)]) -> Territory {
        let mut territory = Territory::empty(TerritoryId(1), Biome::Grassland);
        territory.nodes = densities
            .iter()
            .map(|(resource, density)| Node {
                resource: *resource,
                density: *density,
            })
            .collect();
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
    fn nodes_are_counted_per_resource() {
        let territory = with_nodes(&[
            (Resource::Food, 4),
            (Resource::Food, 6),
            (Resource::Metal, 3),
        ]);
        assert_eq!(territory.node_count(Resource::Food), 2);
        assert_eq!(territory.node_count(Resource::Metal), 1);
        assert_eq!(territory.node_count(Resource::Energy), 0);
    }

    #[test]
    fn building_without_naming_a_node_takes_the_densest_free_one() {
        let mut territory = with_nodes(&[
            (Resource::Food, 2),
            (Resource::Food, 6),
            (Resource::Food, 4),
        ]);
        assert_eq!(
            territory.best_free_node(Resource::Food),
            Some(1),
            "density 6"
        );
        territory.add_extractor(1);
        assert_eq!(
            territory.best_free_node(Resource::Food),
            Some(2),
            "density 4"
        );
        territory.add_extractor(2);
        assert_eq!(
            territory.best_free_node(Resource::Food),
            Some(0),
            "density 2"
        );
        territory.add_extractor(0);
        assert_eq!(territory.best_free_node(Resource::Food), None, "all worked");
    }

    /// Organised force sums; unorganised force is the highest present.
    #[test]
    fn a_garrison_lets_citizens_add_their_force_together() {
        let mut territory = with_nodes(&[]);
        territory.set_count(Kind::Citizen, 4);
        assert_eq!(
            territory.held_force(),
            1,
            "uncoordinated, the highest present"
        );

        // **`P-276` and `P-277`.** A garrison has no force of its own and does one thing:
        // it lets the citizens sum instead of presenting only the highest. It does that by
        // existing, so manning it changes nothing.
        territory.set_garrison(Some(Garrison {
            force: 0,
            manned: 0,
        }));
        assert_eq!(territory.held_force(), 4, "four citizens, summed");
        territory.man_garrison(3);
        assert_eq!(
            territory.held_force(),
            4,
            "still four - nothing has to work it"
        );
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
        let mut territory = with_nodes(&[(Resource::Food, 4)]);
        territory.set_count(Kind::Citizen, 2);
        territory.spend_labor(2);
        // Built, then worked - `extractors()` hands back a copy, so pushing to it changed a
        // temporary and left the territory with none. That is what made this fail with an
        // index out of bounds rather than with a wrong answer.
        territory.add_extractor(0);
        territory.exhaust_extractor(0);
        territory.set_garrison(Some(Garrison {
            force: 1,
            manned: 2,
        }));

        territory.make_ready();
        assert_eq!(territory.labor_available(), 2);
        assert!(!territory.extractors()[0].exhausted);
        assert_eq!(territory.garrison().unwrap().manned, 0);
    }

    #[test]
    fn nature_taking_a_territory_back_leaves_nothing_of_it() {
        let mut territory = with_nodes(&[(Resource::Food, 4)]);
        // Six citizens is what makes it founded now - `S-19`, control derived rather than
        // stored. Setting a flag beside them was the thing that could disagree with them.
        territory.set_count(Kind::Citizen, 6);
        assert!(territory.founded(), "citizens are what holding it means");
        territory.set_garrison(Some(Garrison::from_founding_unit(2)));
        territory.add_extractor(0);
        territory.add(Resource::Metal, 10);

        territory.lost_to_nature();
        assert!(!territory.founded());
        assert_eq!(territory.citizens(), 0, "its entire population perishes");
        assert!(territory.garrison().is_none());
        assert!(territory.extractors().is_empty());
        assert_eq!(territory.store(Resource::Metal), 0);
        assert_eq!(territory.nodes.len(), 1, "the land itself remains");
    }

    /// The extractor rule, either side of its one boundary and not on an example of it.
    ///
    /// `C-9`. A citizen yields one labor and eats one food, so the `k` densest food nodes
    /// sustain `F(k)` citizens while occupying `k` of them. A density-one node adds a hand
    /// and eats it; a density-two node adds a hand and half feeds another. So a spare hand
    /// exists for some allocation exactly when a food node has density two.
    #[test]
    fn a_spare_hand_exists_exactly_when_a_food_node_has_density_two() {
        let cases: [(&[(Resource, u32)], bool, &str); 6] = [
            (&[], false, "no food at all is no population and no hands"),
            (
                &[(Resource::Food, 1)],
                false,
                "one hand, holding its own node",
            ),
            (
                &[
                    (Resource::Food, 1),
                    (Resource::Food, 1),
                    (Resource::Food, 1),
                ],
                false,
                "three of them, and each still eats what it gathers - territory 5",
            ),
            (
                &[(Resource::Food, 2)],
                true,
                "two fed, one node worked, one spare",
            ),
            (
                &[(Resource::Food, 1), (Resource::Food, 2)],
                true,
                "the best node is what decides, not the first or the worst",
            ),
            (
                &[(Resource::Metal, 9), (Resource::Food, 1)],
                false,
                "metal it cannot reach does not feed anyone",
            ),
        ];
        for (nodes, expected, why) in cases {
            let territory = with_nodes(nodes);
            assert_eq!(territory.can_build_extractors(), expected, "{why}");
        }
        assert_eq!(
            cases.len(),
            6,
            "six cases, three either side of the boundary"
        );
    }

    /// The most of a resource one turn can yield, maximised over how the hands are split.
    ///
    /// **The split is not at either end**, which is why this is a search rather than a
    /// formula. Working one more food node costs a hand and buys `f` of them.
    #[test]
    fn the_most_in_one_turn_splits_the_hands_where_it_pays_best() {
        let cases: [(&[(Resource, u32)], Resource, u32, &str); 6] = [
            (
                &[(Resource::Food, 4)],
                Resource::Metal,
                0,
                "no metal node, no metal",
            ),
            (
                &[(Resource::Food, 4), (Resource::Metal, 9)],
                Resource::Metal,
                9,
                "four fed, one holds food, three spare and one metal node to work",
            ),
            (
                &[
                    (Resource::Food, 4),
                    (Resource::Metal, 9),
                    (Resource::Metal, 2),
                ],
                Resource::Metal,
                11,
                "three spare hands reach both metal nodes",
            ),
            (
                &[(Resource::Food, 1), (Resource::Metal, 9)],
                Resource::Metal,
                0,
                "one hand, and it is holding the food node",
            ),
            (
                &[
                    (Resource::Food, 2),
                    (Resource::Food, 2),
                    (Resource::Metal, 9),
                    (Resource::Metal, 9),
                    (Resource::Metal, 9),
                ],
                Resource::Metal,
                18,
                "working the second food node costs a hand and buys two - so two spare, not one",
            ),
            (
                &[(Resource::Food, 4), (Resource::Food, 3)],
                Resource::Food,
                7,
                "asked for food, the answer is what the hands gathered",
            ),
        ];
        for (nodes, resource, expected, why) in cases {
            let territory = with_nodes(nodes);
            assert_eq!(territory.most_in_one_turn(resource), expected, "{why}");
        }
        assert_eq!(
            cases.len(),
            6,
            "six splits, including one that is at neither end"
        );
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
    /// than ten: one metal node is one store is ten, and ten plus a slow turn never reaches
    /// fifteen. A territory can produce metal every turn of the game and never afford a
    /// Yard, which the previous rule could not express.
    #[test]
    fn a_yard_needs_metal_to_be_reachable_rather_than_reachable_at_once() {
        let cases: [(&[(Resource, u32)], bool, &str); 5] = [
            (&[(Resource::Food, 4)], false, "no metal node, so never"),
            (
                &[(Resource::Food, 1), (Resource::Metal, 20)],
                false,
                "metal in the ground with no hand free to dig it is no metal",
            ),
            (
                &[
                    (Resource::Food, 4),
                    (Resource::Metal, 4),
                    (Resource::Metal, 4),
                    (Resource::Metal, 4),
                ],
                true,
                "three nodes, so three stores and thirty; twelve a turn on top",
            ),
            (
                &[(Resource::Food, 4), (Resource::Metal, 5)],
                true,
                "one store of ten and five a turn is exactly fifteen - the boundary",
            ),
            (
                &[(Resource::Food, 4), (Resource::Metal, 4)],
                false,
                "one store of ten and four a turn is fourteen, and it is never fifteen however long the game runs. Territories 8 and 10 are this case",
            ),
        ];
        for (nodes, expected, why) in cases {
            let territory = with_nodes(nodes);
            assert_eq!(territory.can_hold_yard(), expected, "{why}");
        }
        assert_eq!(
            cases.len(),
            5,
            "five cases, and two of them either side of fifteen"
        );
    }
}
