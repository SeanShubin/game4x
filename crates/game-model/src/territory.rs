//! A territory: what it holds, what it can produce, and what it is worth in force.
//!
//! In this release a territory is self-contained. No resource and no citizen crosses a
//! boundary, which is why ending a turn can resolve every territory independently and in
//! any order - see [`crate::game::Game::after`].

use crate::Biome;
use crate::identity::{Resource, TerritoryId};
use crate::thing::{Kind, Thing};

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
    /// Force of its own. A founding unit becomes a structure with one less force than the
    /// unit, and both founding units in this release are force 2.
    pub force: u32,
    /// What a citizen working here produces in force.
    pub multiplier: u32,
    /// Citizens working here this turn.
    pub manned: u32,
}

impl Garrison {
    /// The garrison a founding unit of this force becomes.
    pub fn from_founding_unit(unit_force: u32) -> Self {
        Self {
            force: unit_force.saturating_sub(1),
            multiplier: 1,
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
    /// Whether the player controls it. A territory is founded by a unit taking it.

    /// Labor used this turn. A citizen provides one, and it is not restored until the
    /// turn ends.

    /// What is here now. `spec/logistics.md`: there is no general inventory, so this is
    /// per territory and nothing crosses a boundary.
    /// What is here now, as things rather than as three numbers.
    ///
    /// **`stores: [u32; 3]` could not carry a fourth resource** and, more to the point, made
    /// a unit of food something other than a thing in a place - which is what
    /// `spec/invariants.md` says the state is. Adding a resource added an array element;
    /// now it adds nothing, because a resource is a kind and a kind is already general.
    ///
    /// Held as one `Thing` per unit. Twelve food is twelve things, because *how many of
    /// each* is a fact the state is read for rather than a compression of it.
    pub held: Vec<Thing>,
    pub garrison: Option<Garrison>,
    pub extractors: Vec<Extractor>,
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
            garrison: None,
            extractors: Vec::new(),
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

    /// What a territory can keep, per kind.
    ///
    /// `releases/first-release.md` → *What bounds a kind in a territory*: twenty of each
    /// resource. A citizen is bounded by the food produced here, and labor by the citizens
    /// that make it, so neither is a number - those are enforced where they happen.
    const KEEPS: u32 = 20;

    /// End-of-turn losses: what expires, and what is above the bound.
    ///
    /// **`C-11`.** `spec/turn.md`: *what expires expires, and what was not kept in order is
    /// lost*, and *what a territory can keep is bounded. Anything above the bound is lost
    /// when the turn ends.* The model discarded **all three** resources every turn, which is
    /// neither of those rules. Food expires, because the release gives it *a capacity of 20,
    /// and it keeps for one turn*; metal and energy have a capacity and no expiry, so they
    /// carry.
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
            let over = self.count_of(kind).saturating_sub(Self::KEEPS);
            if over > 0 {
                self.remove(kind, over);
            }
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
    /// so `commands/play.4x` does not change and `expected/play.4x` stays a check on this
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
        self.extractors
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
        let taken: Vec<usize> = self.extractors.iter().map(|e| e.node).collect();
        self.nodes_of(resource)
            .into_iter()
            .filter(|(at, _)| !taken.contains(at))
            .max_by_key(|(at, node)| (node.density, std::cmp::Reverse(*at)))
            .map(|(at, _)| at)
    }

    /// The force the territory itself presents, before any unit standing on it.
    ///
    /// `spec/control.md`: organised force sums, unorganised force is the highest present.
    /// A garrison is what organises citizens, so with one the garrison's own force and
    /// every manned citizen's contribution add up; without one, the citizens present the
    /// highest among them rather than the total.
    pub fn held_force(&self) -> u32 {
        match self.garrison {
            Some(garrison) => garrison.force + garrison.manned * garrison.multiplier,
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
        for extractor in &mut self.extractors {
            extractor.exhausted = false;
        }
        if let Some(garrison) = &mut self.garrison {
            garrison.manned = 0;
        }
    }

    /// What nature does when it takes a territory back.
    ///
    /// `spec/control.md`: its entire population perishes and any ark on it becomes
    /// unusable. The ark is dealt with by the caller, which is the only place that knows
    /// where units are.
    pub fn lost_to_nature(&mut self) {
        self.garrison = None;
        self.extractors.clear();
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
        territory.extractors.push(Extractor {
            node: 1,
            exhausted: false,
        });
        assert_eq!(
            territory.best_free_node(Resource::Food),
            Some(2),
            "density 4"
        );
        territory.extractors.push(Extractor {
            node: 2,
            exhausted: false,
        });
        assert_eq!(
            territory.best_free_node(Resource::Food),
            Some(0),
            "density 2"
        );
        territory.extractors.push(Extractor {
            node: 0,
            exhausted: false,
        });
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

        territory.garrison = Some(Garrison {
            force: 1,
            multiplier: 1,
            manned: 0,
        });
        assert_eq!(territory.held_force(), 1, "the garrison's own force");
        territory.garrison.as_mut().unwrap().manned = 3;
        assert_eq!(
            territory.held_force(),
            4,
            "one of its own plus three manning it"
        );
    }

    /// `spec/unit-types.md`: the structure a founding unit becomes has one less force.
    #[test]
    fn a_founding_unit_becomes_a_garrison_one_weaker_than_itself() {
        assert_eq!(Garrison::from_founding_unit(2).force, 1);
        assert_eq!(Garrison::from_founding_unit(2).multiplier, 1);
    }

    #[test]
    fn ending_a_turn_makes_everything_ready_again() {
        let mut territory = with_nodes(&[(Resource::Food, 4)]);
        territory.set_count(Kind::Citizen, 2);
        territory.spend_labor(2);
        territory.extractors.push(Extractor {
            node: 0,
            exhausted: true,
        });
        territory.garrison = Some(Garrison {
            force: 1,
            multiplier: 1,
            manned: 2,
        });

        territory.make_ready();
        assert_eq!(territory.labor_available(), 2);
        assert!(!territory.extractors[0].exhausted);
        assert_eq!(territory.garrison.unwrap().manned, 0);
    }

    #[test]
    fn nature_taking_a_territory_back_leaves_nothing_of_it() {
        let mut territory = with_nodes(&[(Resource::Food, 4)]);
        // Six citizens is what makes it founded now - `S-19`, control derived rather than
        // stored. Setting a flag beside them was the thing that could disagree with them.
        territory.set_count(Kind::Citizen, 6);
        assert!(territory.founded(), "citizens are what holding it means");
        territory.garrison = Some(Garrison::from_founding_unit(2));
        territory.extractors.push(Extractor {
            node: 0,
            exhausted: false,
        });
        territory.add(Resource::Metal, 10);

        territory.lost_to_nature();
        assert!(!territory.founded());
        assert_eq!(territory.citizens(), 0, "its entire population perishes");
        assert!(territory.garrison.is_none());
        assert!(territory.extractors.is_empty());
        assert_eq!(territory.store(Resource::Metal), 0);
        assert_eq!(territory.nodes.len(), 1, "the land itself remains");
    }
}
