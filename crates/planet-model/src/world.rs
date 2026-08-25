//! The world, and the one function that changes it.

use crate::{Intent, PlayerId, RegionId, Topology};

/// Everything the rules know.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World {
    topology: Topology,
    /// Indexed by [`RegionId`], so ownership is addressed by identity rather than by
    /// position in some iteration. `None` means unowned.
    owners: Vec<Option<PlayerId>>,
}

impl World {
    /// An empty world with nothing owned.
    pub fn new(topology: Topology) -> Self {
        let owners = vec![None; topology.region_count()];
        Self { topology, owners }
    }

    /// A world with ownership already decided, as gathered from somewhere else.
    ///
    /// The slice is indexed by [`RegionId`]. Anything shorter than the topology is
    /// treated as unowned, so a caller cannot silently produce a half-built world.
    pub fn with_owners(topology: Topology, owners: &[Option<PlayerId>]) -> Self {
        let mut world = Self::new(topology);
        for (index, owner) in owners.iter().enumerate().take(world.owners.len()) {
            world.owners[index] = *owner;
        }
        world
    }

    pub fn topology(&self) -> &Topology {
        &self.topology
    }

    pub fn region_count(&self) -> usize {
        self.topology.region_count()
    }

    pub fn regions(&self) -> impl Iterator<Item = RegionId> + '_ {
        self.topology.regions()
    }

    pub fn owner(&self, region: RegionId) -> Option<PlayerId> {
        self.owners.get(region.index()).copied().flatten()
    }

    pub fn owners(&self) -> &[Option<PlayerId>] {
        &self.owners
    }

    pub fn owned_by(&self, player: PlayerId) -> impl Iterator<Item = RegionId> + '_ {
        self.regions()
            .filter(move |&region| self.owner(region) == Some(player))
    }

    pub fn holds_anything(&self, player: PlayerId) -> bool {
        self.owners.iter().any(|owner| *owner == Some(player))
    }

    /// **The function.** `(old world, intent array) -> new world`.
    ///
    /// Runs in three phases, which is not an implementation detail but the whole
    /// argument for why this is safe to parallelise:
    ///
    /// 1. **Gather** — every intent is judged against the world *as it was at the start
    ///    of the turn*. No intent can see another's effect, so no ordering of this phase
    ///    can change what it produces.
    /// 2. **Resolve** — proposals that collide on a region are settled by the intent's
    ///    position in the array. That is data, not schedule, so the answer is fixed.
    /// 3. **Apply** — each region is written by exactly one decision.
    ///
    /// The array's order is an input and is free to matter. The order in which the work
    /// happens is not, and does not.
    pub fn advance(&self, intents: &[Intent]) -> World {
        let proposals = self.gather(intents);
        let decisions = resolve(proposals, self.region_count());
        self.apply(&decisions)
    }

    /// Phase one: judge each intent against the start-of-turn world, independently.
    fn gather(&self, intents: &[Intent]) -> Vec<Proposal> {
        intents
            .iter()
            .enumerate()
            .filter_map(|(order, &intent)| {
                let region = intent.region();
                if !self.topology.contains(region) {
                    return None;
                }
                let outcome = match intent {
                    Intent::Claim { region, player } => {
                        if self.owner(region).is_some() {
                            // Already taken at the start of the turn.
                            return None;
                        }
                        let adjacent_to_player = self
                            .topology
                            .neighbours(region)
                            .iter()
                            .any(|&near| self.owner(near) == Some(player));
                        if adjacent_to_player || !self.holds_anything(player) {
                            Some(player)
                        } else {
                            // Not reachable from anything this player holds.
                            return None;
                        }
                    }
                    Intent::Abandon { region } => {
                        self.owner(region)?;
                        None
                    }
                };
                Some(Proposal {
                    region,
                    owner: outcome,
                    order,
                })
            })
            .collect()
    }

    /// Phase three: one decision per region, written by identity.
    fn apply(&self, decisions: &[Option<Proposal>]) -> World {
        let mut next = self.clone();
        for decision in decisions.iter().flatten() {
            next.owners[decision.region.index()] = decision.owner;
        }
        next
    }
}

/// One intent's judged effect: what it would do, and where it sat in the array.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Proposal {
    region: RegionId,
    owner: Option<PlayerId>,
    order: usize,
}

/// Phase two: settle collisions.
///
/// Several proposals may target one region. The earliest in the intent array wins,
/// which is a total order derived entirely from the input — so this is a pure function
/// of the *set* of proposals and could be computed by a parallel reduction over `min`
/// without changing the answer.
fn resolve(proposals: Vec<Proposal>, region_count: usize) -> Vec<Option<Proposal>> {
    let mut decisions: Vec<Option<Proposal>> = vec![None; region_count];
    for proposal in proposals {
        let slot = &mut decisions[proposal.region.index()];
        let wins = match slot {
            None => true,
            Some(existing) => proposal.order < existing.order,
        };
        if wins {
            *slot = Some(proposal);
        }
    }
    decisions
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

    #[test]
    fn a_new_world_is_unowned() {
        let world = World::new(line());
        assert_eq!(world.region_count(), 4);
        assert!(world.regions().all(|region| world.owner(region).is_none()));
        assert!(!world.holds_anything(PlayerId(1)));
    }

    #[test]
    fn a_first_claim_can_land_anywhere() {
        let world = World::new(line()).advance(&[claim(2, 1)]);
        assert_eq!(world.owner(RegionId(2)), Some(PlayerId(1)));
        assert!(world.holds_anything(PlayerId(1)));
    }

    #[test]
    fn later_claims_must_be_next_to_something_you_hold() {
        let world = World::new(line()).advance(&[claim(0, 1)]);

        let reachable = world.advance(&[claim(1, 1)]);
        assert_eq!(reachable.owner(RegionId(1)), Some(PlayerId(1)));

        let unreachable = world.advance(&[claim(3, 1)]);
        assert_eq!(unreachable.owner(RegionId(3)), None, "region 3 is two hops away");
    }

    #[test]
    fn an_owned_region_cannot_be_claimed() {
        let world = World::new(line()).advance(&[claim(0, 1)]);
        let after = world.advance(&[claim(0, 2)]);
        assert_eq!(after.owner(RegionId(0)), Some(PlayerId(1)));
    }

    #[test]
    fn abandoning_frees_a_region() {
        let world = World::new(line()).advance(&[claim(1, 1)]);
        let after = world.advance(&[Intent::Abandon {
            region: RegionId(1),
        }]);
        assert_eq!(after.owner(RegionId(1)), None);
    }

    #[test]
    fn nonsense_intents_are_ignored_rather_than_fatal() {
        let world = World::new(line());
        let after = world.advance(&[
            claim(99, 1),
            Intent::Abandon {
                region: RegionId(2),
            },
        ]);
        assert_eq!(after, world, "nothing should have changed");
    }

    // ---- the properties the whole design exists to provide ------------------------

    /// Same inputs, same output. The requirement, stated directly.
    #[test]
    fn the_same_inputs_always_give_the_same_output() {
        let intents = [claim(0, 1), claim(2, 2), claim(1, 1)];
        let world = World::new(line());
        assert_eq!(world.advance(&intents), world.advance(&intents));
    }

    /// Folding an intent log reproduces the state it originally produced. This is the
    /// replay guarantee, asserted rather than hoped for.
    #[test]
    fn replaying_a_log_reproduces_the_world() {
        let log = [
            vec![claim(0, 1), claim(3, 2)],
            vec![claim(1, 1)],
            vec![claim(2, 2), claim(2, 1)],
        ];
        let fold = || {
            log.iter()
                .fold(World::new(line()), |world, turn| world.advance(turn))
        };
        assert_eq!(fold(), fold());
    }

    /// The intent array is an ordered input, so its order is *allowed* to matter — and
    /// where two intents genuinely collide, it does. Earlier wins.
    #[test]
    fn the_intent_order_settles_a_genuine_collision() {
        let world = World::new(line());
        let first = world.advance(&[claim(1, 1), claim(1, 2)]);
        let second = world.advance(&[claim(1, 2), claim(1, 1)]);

        assert_eq!(first.owner(RegionId(1)), Some(PlayerId(1)));
        assert_eq!(second.owner(RegionId(1)), Some(PlayerId(2)));
        assert_ne!(first, second, "a real collision is decided by the array order");
    }

    /// ...but intents that do *not* collide must be order-insensitive. This is the
    /// property that lets the whole array be resolved in parallel, and the one most
    /// likely to be broken by accident when a rule starts reading a partial result.
    #[test]
    fn reordering_intents_that_do_not_collide_changes_nothing() {
        let world = World::new(line());
        let forwards = world.advance(&[claim(0, 1), claim(3, 2)]);
        let backwards = world.advance(&[claim(3, 2), claim(0, 1)]);
        assert_eq!(forwards, backwards);
        assert_eq!(forwards.owner(RegionId(0)), Some(PlayerId(1)));
        assert_eq!(forwards.owner(RegionId(3)), Some(PlayerId(2)));
    }

    /// No intent may observe another's effect within the same turn. If claiming 0 let
    /// a later claim on 1 suddenly become adjacent-and-legal, the turn would depend on
    /// evaluation order and could not be parallelised.
    #[test]
    fn no_intent_sees_another_intents_effect_within_a_turn() {
        let world = World::new(line());
        let after = world.advance(&[claim(0, 1), claim(1, 1)]);
        assert_eq!(after.owner(RegionId(0)), Some(PlayerId(1)));
        assert_eq!(
            after.owner(RegionId(1)),
            Some(PlayerId(1)),
            "this is the player's opening move, so it is legal on its own"
        );

        // With a foothold already established elsewhere, the same pair must not chain.
        let established = World::new(line()).advance(&[claim(3, 1)]);
        let chained = established.advance(&[claim(2, 1), claim(1, 1)]);
        assert_eq!(chained.owner(RegionId(2)), Some(PlayerId(1)));
        assert_eq!(
            chained.owner(RegionId(1)),
            None,
            "region 1 was not adjacent to anything held at the start of the turn"
        );
    }

    /// The resolve phase must be a pure function of the *set* of proposals. Shuffling
    /// the order they are reduced in must not change which one wins.
    #[test]
    fn the_resolve_phase_does_not_care_what_order_it_reduces_in() {
        let proposals = vec![
            Proposal { region: RegionId(1), owner: Some(PlayerId(3)), order: 7 },
            Proposal { region: RegionId(1), owner: Some(PlayerId(1)), order: 2 },
            Proposal { region: RegionId(1), owner: Some(PlayerId(2)), order: 5 },
        ];
        let forwards = resolve(proposals.clone(), 4);
        let mut reversed = proposals.clone();
        reversed.reverse();
        let backwards = resolve(reversed, 4);
        let rotated = vec![proposals[1], proposals[2], proposals[0]];
        let third = resolve(rotated, 4);

        assert_eq!(forwards, backwards);
        assert_eq!(forwards, third);
        assert_eq!(forwards[1].unwrap().owner, Some(PlayerId(1)), "lowest order wins");
    }

    #[test]
    fn a_whole_turn_of_many_players_is_stable_under_repetition() {
        let topology = Topology::from_neighbour_lists(&[
            vec![1, 2],
            vec![0, 2, 3],
            vec![0, 1, 3],
            vec![1, 2],
        ]);
        let opening = World::new(topology).advance(&[claim(0, 1), claim(3, 2)]);
        let contested = [claim(1, 1), claim(1, 2), claim(2, 2), claim(2, 1)];
        let once = opening.advance(&contested);
        let twice = opening.advance(&contested);
        assert_eq!(once, twice);
        assert_eq!(once.owner(RegionId(1)), Some(PlayerId(1)));
        assert_eq!(once.owner(RegionId(2)), Some(PlayerId(2)));
    }
}
