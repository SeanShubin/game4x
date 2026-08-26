//! Game entities, as ECS entities.
//!
//! Every thing in the world with identity and state lives here as a Bevy entity with
//! components. There is no second way of holding game state — no parallel `Vec` of
//! regions, no side-table of ownership.
//!
//! What is *not* here is any rule. The systems in this crate do exactly two things:
//!
//! - **gather** — read the ECS into plain data, keyed by [`RegionId`]
//! - **apply** — write plain data back into the ECS
//!
//! Between those two, they call [`planet_model`], which is a pure function and knows
//! nothing about Bevy. That is the whole division: entities are nouns, algorithms are
//! verbs, and systems are the glue with no opinions of their own. See
//! `docs/layers.md`.
//!
//! # Why gathering is safe
//!
//! Query iteration order is not a contract — it follows archetype layout, which follows
//! insertion history. Gathering therefore never *accumulates* in iteration order; it
//! writes into a vector indexed by `RegionId`. The result is identical however the
//! query happened to walk, which is what lets the rest of the schedule run in parallel
//! without changing the answer.

use bevy::prelude::*;
use planet_model::{Intent, PlayerId, RegionId, Topology, World};

/// A region of the world. The component carries the canonical identity.
///
/// `Entity` is a runtime handle: Bevy reuses ids and does not keep them stable across
/// runs or saves, so it is never serialised, never ordered, and never a tie-break.
/// `RegionId` is the identity that means the same thing in every run.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region(pub RegionId);

/// Who holds a region. Absent means unowned, which is what ECS optionality is for.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Owner(pub PlayerId);

/// The adjacency graph. Fixed when the world is made, so it is a resource rather than
/// something reconstructed each turn.
#[derive(Resource, Clone, Debug)]
pub struct WorldTopology(pub Topology);

/// Intents waiting to be folded into the world at the next turn.
///
/// An ordered array, because its order is a legitimate input — it settles collisions.
#[derive(Resource, Default, Debug)]
pub struct PendingIntents(pub Vec<Intent>);

impl PendingIntents {
    pub fn push(&mut self, intent: Intent) {
        self.0.push(intent);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Raised after a turn has been folded in, so other layers can react without polling.
#[derive(Message, Debug, Clone, Copy)]
pub struct TurnAdvanced;

/// The set the turn runs in, so a caller can order its own work around it.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdvanceTurn;

/// Spawns entities for a topology and installs the turn system.
pub struct PlanetEcsPlugin {
    pub topology: Topology,
}

impl PlanetEcsPlugin {
    pub fn new(topology: Topology) -> Self {
        Self { topology }
    }
}

impl Plugin for PlanetEcsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldTopology(self.topology.clone()))
            .init_resource::<PendingIntents>()
            .add_message::<TurnAdvanced>()
            .add_systems(Startup, spawn_regions)
            .add_systems(Update, advance_turn.in_set(AdvanceTurn));
    }
}

/// One entity per region, in ascending id order.
///
/// The spawn order is ascending only for tidiness; nothing may depend on it, and
/// [`gather`] is written so that nothing can.
pub fn spawn_regions(mut commands: Commands, topology: Res<WorldTopology>) {
    for region in topology.0.regions() {
        commands.spawn(Region(region));
    }
}

/// Reads the ECS into a plain [`World`], keyed by identity rather than by iteration.
pub fn gather(topology: &Topology, regions: &Query<(&Region, Option<&Owner>)>) -> World {
    let mut owners = vec![None; topology.region_count()];
    for (region, owner) in regions.iter() {
        if let Some(slot) = owners.get_mut(region.0.index()) {
            *slot = owner.map(|owner| owner.0);
        }
    }
    World::with_owners(topology.clone(), &owners)
}

/// Folds the pending intents into the world: gather, resolve, apply.
pub fn advance_turn(
    mut commands: Commands,
    topology: Res<WorldTopology>,
    mut pending: ResMut<PendingIntents>,
    regions: Query<(Entity, &Region, Option<&Owner>)>,
    mut advanced: MessageWriter<TurnAdvanced>,
) {
    if pending.is_empty() {
        return;
    }

    // GATHER: ECS to plain data, indexed by RegionId so query order cannot show.
    let mut owners = vec![None; topology.0.region_count()];
    let mut entities = vec![None; topology.0.region_count()];
    for (entity, region, owner) in regions.iter() {
        let index = region.0.index();
        if index < owners.len() {
            owners[index] = owner.map(|owner| owner.0);
            entities[index] = Some(entity);
        }
    }
    let before = World::with_owners(topology.0.clone(), &owners);

    // RESOLVE: a pure function, with no idea any of this exists.
    let intents = std::mem::take(&mut pending.0);
    let after = before.advance(&intents);

    // APPLY: plain data back to the ECS, one write per region.
    for region in after.regions() {
        let index = region.index();
        let Some(entity) = entities[index] else {
            continue;
        };
        if before.owner(region) == after.owner(region) {
            continue;
        }
        match after.owner(region) {
            Some(player) => {
                commands.entity(entity).insert(Owner(player));
            }
            None => {
                commands.entity(entity).remove::<Owner>();
            }
        }
    }

    advanced.write(TurnAdvanced);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A line of four regions: 0 - 1 - 2 - 3.
    fn line() -> Topology {
        Topology::from_neighbour_lists(&[vec![1], vec![0, 2], vec![1, 3], vec![2]])
    }

    fn claim(region: u32, player: u16) -> Intent {
        Intent::Claim {
            region: RegionId(region),
            player: PlayerId(player),
        }
    }

    /// Builds an app with the regions already spawned.
    fn app_with(topology: Topology) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(PlanetEcsPlugin::new(topology));
        app.update();
        app
    }

    /// Reads ownership out by identity, which is the only stable way to read it.
    fn ownership(app: &mut App) -> Vec<Option<PlayerId>> {
        let count = app.world().resource::<WorldTopology>().0.region_count();
        let mut owners = vec![None; count];
        let mut query = app.world_mut().query::<(&Region, Option<&Owner>)>();
        for (region, owner) in query.iter(app.world()) {
            owners[region.0.index()] = owner.map(|owner| owner.0);
        }
        owners
    }

    fn submit(app: &mut App, intents: &[Intent]) {
        let mut pending = app.world_mut().resource_mut::<PendingIntents>();
        for &intent in intents {
            pending.push(intent);
        }
        app.update();
    }

    #[test]
    fn every_region_becomes_an_entity() {
        let mut app = app_with(line());
        let mut query = app.world_mut().query::<&Region>();
        let mut ids: Vec<u32> = query.iter(app.world()).map(|region| region.0.0).collect();
        ids.sort_unstable();
        assert_eq!(ids, vec![0, 1, 2, 3]);
    }

    #[test]
    fn regions_start_unowned() {
        let mut app = app_with(line());
        assert_eq!(ownership(&mut app), vec![None; 4]);
    }

    #[test]
    fn a_turn_applies_the_intents() {
        let mut app = app_with(line());
        submit(&mut app, &[claim(1, 7)]);
        assert_eq!(ownership(&mut app)[1], Some(PlayerId(7)));
    }

    #[test]
    fn the_queue_is_emptied_by_the_turn() {
        let mut app = app_with(line());
        submit(&mut app, &[claim(1, 7)]);
        assert!(app.world().resource::<PendingIntents>().is_empty());
    }

    #[test]
    fn abandoning_removes_the_component_rather_than_blanking_it() {
        let mut app = app_with(line());
        submit(&mut app, &[claim(1, 7)]);
        submit(
            &mut app,
            &[Intent::Abandon {
                region: RegionId(1),
            }],
        );

        let mut query = app.world_mut().query::<(&Region, Option<&Owner>)>();
        let owned = query
            .iter(app.world())
            .filter(|(_, owner)| owner.is_some())
            .count();
        assert_eq!(owned, 0, "unowned means the component is absent");
    }

    /// The confluence property, at the ECS boundary. Entities are spawned in different
    /// orders — which changes archetype layout and therefore query iteration order —
    /// and the resulting ownership must be identical, because gathering is keyed by
    /// `RegionId` rather than accumulated in iteration order.
    #[test]
    fn spawn_order_does_not_change_the_outcome() {
        let intents = [claim(0, 1), claim(3, 2), claim(1, 1), claim(2, 2)];

        let mut results = Vec::new();
        for order in [
            vec![0u32, 1, 2, 3],
            vec![3, 2, 1, 0],
            vec![2, 0, 3, 1],
            vec![1, 3, 0, 2],
        ] {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins)
                .insert_resource(WorldTopology(line()))
                .init_resource::<PendingIntents>()
                .add_message::<TurnAdvanced>()
                .add_systems(Update, advance_turn);
            for id in order {
                app.world_mut().spawn(Region(RegionId(id)));
            }
            app.update();
            submit(&mut app, &intents);
            results.push(ownership(&mut app));
        }

        for (index, result) in results.iter().enumerate() {
            assert_eq!(
                *result, results[0],
                "spawn order {index} produced a different world"
            );
        }
        assert_ne!(
            results[0],
            vec![None; 4],
            "the turn should have done something"
        );
    }

    /// Entity ids are a runtime detail; identity is the `RegionId` the entity carries.
    /// Two runs must agree on the regions and on the outcome, whatever entities they
    /// happened to allocate.
    #[test]
    fn identity_is_the_region_id_not_the_entity() {
        let run = || {
            let mut app = app_with(line());
            submit(&mut app, &[claim(0, 1), claim(1, 1)]);
            let mut query = app.world_mut().query::<(&Region, Option<&Owner>)>();
            let mut by_identity: Vec<(u32, Option<u16>)> = query
                .iter(app.world())
                .map(|(region, owner)| (region.0.0, owner.map(|owner| owner.0.0)))
                .collect();
            // Sorting by the data-derived key, never by iteration position.
            by_identity.sort_unstable();
            by_identity
        };
        assert_eq!(run(), run());
        assert_eq!(run()[0], (0, Some(1)));
    }

    /// Running a turn with nothing pending must not disturb anything, so that the
    /// system is safe to leave in the schedule every frame.
    #[test]
    fn an_empty_turn_is_a_no_op() {
        let mut app = app_with(line());
        submit(&mut app, &[claim(2, 4)]);
        let before = ownership(&mut app);
        app.update();
        app.update();
        assert_eq!(ownership(&mut app), before);
    }

    /// The same sequence of turns, run twice, gives the same world. The replay
    /// guarantee, asserted through the ECS rather than only through the pure function.
    #[test]
    fn replaying_the_same_turns_reproduces_the_world() {
        let play = || {
            let mut app = app_with(line());
            submit(&mut app, &[claim(0, 1), claim(3, 2)]);
            submit(&mut app, &[claim(1, 1), claim(2, 2)]);
            submit(&mut app, &[claim(2, 1), claim(1, 2)]);
            ownership(&mut app)
        };
        assert_eq!(play(), play());
    }
}
