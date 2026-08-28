//! A territory: what it holds, what it can produce, and what it is worth in force.
//!
//! In this release a territory is self-contained. No resource and no citizen crosses a
//! boundary, which is why ending a turn can resolve every territory independently and in
//! any order - see [`crate::game::Game::after`].

use crate::identity::{Biome, Resource, TerritoryId};

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
    pub founded: bool,
    pub citizens: u32,
    /// Labor used this turn. A citizen provides one, and it is not restored until the
    /// turn ends.
    pub labor_spent: u32,
    /// What is here now. `spec/logistics.md`: there is no general inventory, so this is
    /// per territory and nothing crosses a boundary.
    pub stores: [u32; 3],
    pub garrison: Option<Garrison>,
    pub extractors: Vec<Extractor>,
    pub yards: u32,
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
            founded: false,
            citizens: 0,
            labor_spent: 0,
            stores: [0; 3],
            garrison: None,
            extractors: Vec::new(),
            yards: 0,
        }
    }

    pub fn store(&self, resource: Resource) -> u32 {
        self.stores[resource.index()]
    }

    pub fn add(&mut self, resource: Resource, amount: u32) {
        self.stores[resource.index()] += amount;
    }

    pub fn take(&mut self, resource: Resource, amount: u32) {
        self.stores[resource.index()] = self.stores[resource.index()].saturating_sub(amount);
    }

    /// Labor not yet spent this turn. A citizen provides one each turn.
    pub fn labor_available(&self) -> u32 {
        self.citizens.saturating_sub(self.labor_spent)
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
            None if self.citizens > 0 => CITIZEN_FORCE,
            None => 0,
        }
    }

    /// Makes everything ready again, which is what ending a turn does.
    pub fn make_ready(&mut self) {
        self.labor_spent = 0;
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
        self.founded = false;
        self.citizens = 0;
        self.garrison = None;
        self.extractors.clear();
        self.yards = 0;
        self.stores = [0; 3];
        self.labor_spent = 0;
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
        territory.citizens = 4;
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
        territory.citizens = 2;
        territory.labor_spent = 2;
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
        territory.founded = true;
        territory.citizens = 6;
        territory.garrison = Some(Garrison::from_founding_unit(2));
        territory.extractors.push(Extractor {
            node: 0,
            exhausted: false,
        });
        territory.add(Resource::Metal, 10);

        territory.lost_to_nature();
        assert!(!territory.founded);
        assert_eq!(territory.citizens, 0, "its entire population perishes");
        assert!(territory.garrison.is_none());
        assert!(territory.extractors.is_empty());
        assert_eq!(territory.store(Resource::Metal), 0);
        assert_eq!(territory.nodes.len(), 1, "the land itself remains");
    }
}
