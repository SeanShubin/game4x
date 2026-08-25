//! Which regions touch which. Fixed when the world is made, integers throughout.

use crate::RegionId;

/// The adjacency graph, and nothing else.
///
/// No coordinates, no geometry, no floating point. Where a region *is* on a sphere is
/// the view model's business; the model only knows what touches what. That boundary is
/// what keeps transcendental functions out of the deterministic core — see
/// `docs/layers.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Topology {
    neighbours: Vec<Vec<RegionId>>,
}

impl Topology {
    /// Builds a topology from sorted neighbour lists, as produced by world generation.
    ///
    /// Lists are sorted and de-duplicated on the way in, so that two topologies
    /// describing the same graph are equal regardless of how they were assembled.
    pub fn from_neighbour_lists(lists: &[Vec<u32>]) -> Self {
        let neighbours = lists
            .iter()
            .map(|list| {
                let mut list: Vec<RegionId> = list.iter().map(|&id| RegionId(id)).collect();
                list.sort_unstable();
                list.dedup();
                list
            })
            .collect();
        Self { neighbours }
    }

    pub fn region_count(&self) -> usize {
        self.neighbours.len()
    }

    /// Every region, in ascending order. The canonical iteration order for anything
    /// that must produce a stable result.
    pub fn regions(&self) -> impl Iterator<Item = RegionId> + '_ {
        (0..self.neighbours.len() as u32).map(RegionId)
    }

    pub fn neighbours(&self, region: RegionId) -> &[RegionId] {
        self.neighbours
            .get(region.index())
            .map(|list| list.as_slice())
            .unwrap_or(&[])
    }

    pub fn are_adjacent(&self, first: RegionId, second: RegionId) -> bool {
        self.neighbours(first).binary_search(&second).is_ok()
    }

    pub fn contains(&self, region: RegionId) -> bool {
        region.index() < self.neighbours.len()
    }

    /// Counts each undirected edge once.
    pub fn edge_count(&self) -> usize {
        self.neighbours.iter().map(|list| list.len()).sum::<usize>() / 2
    }

    /// True when the lists agree with each other in both directions. A one-way edge
    /// would make adjacency depend on which end you asked from.
    pub fn is_symmetric(&self) -> bool {
        self.regions().all(|region| {
            self.neighbours(region)
                .iter()
                .all(|&other| self.are_adjacent(other, region))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triangle() -> Topology {
        Topology::from_neighbour_lists(&[vec![1, 2], vec![0, 2], vec![0, 1]])
    }

    #[test]
    fn it_reports_its_own_shape() {
        let topology = triangle();
        assert_eq!(topology.region_count(), 3);
        assert_eq!(topology.edge_count(), 3);
        assert!(topology.is_symmetric());
        assert_eq!(
            topology.regions().collect::<Vec<_>>(),
            vec![RegionId(0), RegionId(1), RegionId(2)]
        );
    }

    #[test]
    fn adjacency_reads_the_same_from_either_end() {
        let topology = triangle();
        assert!(topology.are_adjacent(RegionId(0), RegionId(1)));
        assert!(topology.are_adjacent(RegionId(1), RegionId(0)));
        assert!(!topology.are_adjacent(RegionId(0), RegionId(0)));
    }

    /// The same graph described differently must produce the same topology, so that
    /// equality means "same world" and not "same construction history".
    #[test]
    fn input_order_does_not_change_the_result() {
        let ordered = Topology::from_neighbour_lists(&[vec![1, 2], vec![0, 2], vec![0, 1]]);
        let jumbled = Topology::from_neighbour_lists(&[vec![2, 1], vec![2, 0], vec![1, 0, 1]]);
        assert_eq!(ordered, jumbled);
    }

    #[test]
    fn an_unknown_region_has_no_neighbours_rather_than_panicking() {
        let topology = triangle();
        assert!(!topology.contains(RegionId(99)));
        assert!(topology.neighbours(RegionId(99)).is_empty());
    }

    #[test]
    fn an_empty_world_is_allowed() {
        let topology = Topology::from_neighbour_lists(&[]);
        assert_eq!(topology.region_count(), 0);
        assert_eq!(topology.edge_count(), 0);
        assert!(topology.is_symmetric());
    }
}
