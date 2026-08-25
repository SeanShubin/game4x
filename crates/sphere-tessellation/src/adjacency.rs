//! Turning seed points into the adjacency graph.
//!
//! Two regions are adjacent when their Voronoi cells share a border of positive
//! length. Corner contact is not adjacency — see the corner-touching caveat in
//! `docs/theory/region-coloring.md`, which depends on exactly this definition.
//!
//! The method here solves each candidate pair exactly rather than sampling.
//!
//! Points equidistant from seeds `i` and `j` form a great circle. Parametrize it by an
//! angle `t`. Every other seed `k` rules out the points that are closer to it than to
//! `i`, and that condition — `v . (s_i - s_k) >= 0` — is linear in `v`, so on the
//! circle it works out to `A cos t + B sin t >= 0`. That is exactly a half-circle of
//! `t`. Intersecting half-circles always leaves a single arc, so the shared border can
//! be computed by narrowing one interval, with no discretization anywhere.
//!
//! An earlier version sampled the circle at 512 points instead, and silently dropped
//! short borders — enough to break Euler's formula by 120 regions. Sampling cannot fix
//! that, only push it to smaller edges.
//!
//! Cost is O(n) per candidate pair. That is O(n^3) overall before the distance cutoff
//! below prunes it, which is fine into the hundreds. Past that the right answer is a
//! Delaunay triangulation via 3D convex hull, per step 4 of
//! `docs/theory/region-splitting.md`.

use crate::lattice::mean_spacing;
use crate::vec3::Vec3;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// A shared border must be longer than this to count, so that regions meeting at a
/// single point are not adjacent. Corner contact produces an arc of exactly zero
/// length, so this only has to clear floating point noise. See the corner-touching
/// caveat in `docs/theory/region-coloring.md`.
const MINIMUM_ARC: f64 = 1e-9;

/// A seed whose direction is this close to the bisector normal constrains nothing:
/// it ties with the pair everywhere on the circle rather than beating them anywhere.
const DEGENERATE: f64 = 1e-12;

/// Builds the neighbour lists for a set of seeds. Each list is sorted ascending.
pub fn adjacency(seeds: &[Vec3]) -> Vec<Vec<u32>> {
    let count = seeds.len();
    let mut neighbours = vec![Vec::new(); count];
    if count < 2 {
        return neighbours;
    }

    // Seeds further apart than this cannot share a border. Generous, because being
    // wrong here silently drops edges; it is only an optimization.
    let cutoff = (mean_spacing(count) * 4.0).min(PI);

    for first in 0..count {
        for second in (first + 1)..count {
            if seeds[first].angle_to(seeds[second]) > cutoff {
                continue;
            }
            if shares_a_border(seeds, first, second) {
                neighbours[first].push(second as u32);
                neighbours[second].push(first as u32);
            }
        }
    }

    for list in neighbours.iter_mut() {
        list.sort_unstable();
    }
    neighbours
}

/// The length of the border shared by two regions, in radians, or zero if they do not
/// share one.
pub fn shared_border_length(seeds: &[Vec3], first: usize, second: usize) -> f64 {
    // Points equidistant from both seeds are exactly the unit vectors perpendicular
    // to the difference between them.
    let normal = seeds[first].sub(seeds[second]).normalized();
    let basis_u = normal.any_perpendicular();
    let basis_w = normal.cross(basis_u).normalized();

    // The surviving arc, as a start angle and a width. It begins as the whole circle
    // and only ever narrows.
    let mut start = 0.0;
    let mut width = TAU;
    let mut constrained = false;

    for (index, seed) in seeds.iter().enumerate() {
        if index == first || index == second {
            continue;
        }
        let difference = seeds[first].sub(*seed);
        let along_u = difference.dot(basis_u);
        let along_w = difference.dot(basis_w);
        if along_u.hypot(along_w) < DEGENERATE {
            continue;
        }

        // This seed permits the half-circle centred on its own direction.
        let permitted_start = along_w.atan2(along_u) - FRAC_PI_2;

        if !constrained {
            start = permitted_start;
            width = PI;
            constrained = true;
            continue;
        }

        // Intersect [start, start + width] with [permitted_start, permitted_start +
        // PI], working relative to `start`. Width is at most PI by now, which is what
        // makes the wrapped case collapse to a single piece.
        let offset = (permitted_start - start).rem_euclid(TAU);
        let (low, high) = if offset + PI <= TAU {
            (offset, width.min(offset + PI))
        } else {
            (0.0, width.min(offset + PI - TAU))
        };
        if high <= low {
            return 0.0;
        }
        start += low;
        width = high - low;
    }
    width
}

fn shares_a_border(seeds: &[Vec3], first: usize, second: usize) -> bool {
    shared_border_length(seeds, first, second) > MINIMUM_ARC
}

/// Solid angle covered by each region, in steradians. The whole sphere is `4*pi`.
///
/// Measured by sampling rather than by integrating cell polygons, for the same reason
/// relaxation samples: far less code, and accurate enough to compare regions against
/// each other.
pub fn region_areas(seeds: &[Vec3], samples: usize) -> Vec<f64> {
    use crate::lattice::{fibonacci_lattice, nearest_index};
    let mut counts = vec![0usize; seeds.len()];
    let probes = fibonacci_lattice(samples);
    for probe in &probes {
        counts[nearest_index(seeds, *probe)] += 1;
    }
    let whole_sphere = 4.0 * PI;
    counts
        .iter()
        .map(|&count| whole_sphere * count as f64 / probes.len() as f64)
        .collect()
}

/// Counts each undirected edge once.
pub fn edge_count(neighbours: &[Vec<u32>]) -> usize {
    neighbours.iter().map(|list| list.len()).sum::<usize>() / 2
}

/// How many regions have each neighbour count, indexed by that count.
pub fn degree_histogram(neighbours: &[Vec<u32>]) -> Vec<usize> {
    let widest = neighbours.iter().map(|list| list.len()).max().unwrap_or(0);
    let mut histogram = vec![0usize; widest + 1];
    for list in neighbours {
        histogram[list.len()] += 1;
    }
    histogram
}

/// True when every region can be reached from region zero.
pub fn is_connected(neighbours: &[Vec<u32>]) -> bool {
    if neighbours.is_empty() {
        return true;
    }
    let mut seen = vec![false; neighbours.len()];
    let mut stack = vec![0usize];
    seen[0] = true;
    let mut reached = 1;
    while let Some(current) = stack.pop() {
        for &next in &neighbours[current] {
            if !seen[next as usize] {
                seen[next as usize] = true;
                reached += 1;
                stack.push(next as usize);
            }
        }
    }
    reached == neighbours.len()
}

/// True when the lists agree with each other in both directions.
pub fn is_symmetric(neighbours: &[Vec<u32>]) -> bool {
    neighbours.iter().enumerate().all(|(index, list)| {
        list.iter()
            .all(|&other| neighbours[other as usize].contains(&(index as u32)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::fibonacci_lattice;

    #[test]
    fn a_single_region_has_no_neighbours() {
        let neighbours = adjacency(&fibonacci_lattice(1));
        assert_eq!(neighbours.len(), 1);
        assert!(neighbours[0].is_empty());
        assert_eq!(edge_count(&neighbours), 0);
    }

    #[test]
    fn two_regions_are_adjacent_to_each_other() {
        let seeds = vec![Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0)];
        let neighbours = adjacency(&seeds);
        assert_eq!(neighbours[0], vec![1]);
        assert_eq!(neighbours[1], vec![0]);
    }

    #[test]
    fn three_regions_are_mutually_adjacent() {
        let seeds = fibonacci_lattice(3);
        let neighbours = adjacency(&seeds);
        assert_eq!(edge_count(&neighbours), 3);
        for list in &neighbours {
            assert_eq!(list.len(), 2);
        }
    }

    #[test]
    fn adjacency_is_symmetric_and_connected() {
        for count in [4, 7, 20, 60] {
            let neighbours = adjacency(&fibonacci_lattice(count));
            assert!(is_symmetric(&neighbours), "asymmetric at {count}");
            assert!(is_connected(&neighbours), "disconnected at {count}");
            assert!(
                neighbours.iter().all(|list| !list.is_empty()),
                "isolated region at {count}"
            );
        }
    }

    #[test]
    fn no_region_is_its_own_neighbour() {
        let neighbours = adjacency(&fibonacci_lattice(40));
        for (index, list) in neighbours.iter().enumerate() {
            assert!(!list.contains(&(index as u32)));
        }
    }

    /// Euler's formula is the strongest end-to-end check available: for a planar
    /// graph drawn on a sphere, V - E + F = 2. Faces are the Voronoi vertices, and
    /// with three cells meeting at each of them, F = 2E/3 when the graph is a
    /// triangulation. So V - E + 2E/3 = 2, giving E = 3V - 6.
    #[test]
    fn a_relaxed_lattice_satisfies_eulers_formula() {
        use crate::lattice::{recommended_sample_count, relax};
        let count = 60;
        let mut seeds = fibonacci_lattice(count);
        relax(
            &mut seeds,
            3,
            &fibonacci_lattice(recommended_sample_count(count)),
        );
        let neighbours = adjacency(&seeds);
        assert_eq!(
            edge_count(&neighbours),
            3 * count - 6,
            "degrees: {:?}",
            degree_histogram(&neighbours)
        );
    }

    /// The twelve pentagons of the theory document, observed rather than assumed.
    /// Summing (6 - degree) over every region must come to exactly 12 for any
    /// triangulation of the sphere, which is Euler's formula in another guise.
    #[test]
    fn total_degree_deficit_is_always_twelve() {
        use crate::lattice::{recommended_sample_count, relax};
        for count in [20, 50, 120] {
            let mut seeds = fibonacci_lattice(count);
            relax(
                &mut seeds,
                3,
                &fibonacci_lattice(recommended_sample_count(count)),
            );
            let neighbours = adjacency(&seeds);
            let deficit: i64 = neighbours.iter().map(|list| 6 - list.len() as i64).sum();
            assert_eq!(deficit, 12, "count {count}");
        }
    }

    /// The same invariant across the parameter space the prototype actually uses.
    /// The sampling version of this module passed the unjittered cases and failed
    /// here, so breadth matters more than depth for this particular check.
    #[test]
    fn the_deficit_holds_across_seeds_jitter_and_counts() {
        use crate::{Params, Tessellation};
        for region_count in [4, 8, 20, 37, 90, 150] {
            for seed in [1, 2, 7] {
                for jitter in [0.0, 0.25, 0.45] {
                    let tessellation = Tessellation::generate(Params {
                        region_count,
                        jitter,
                        relaxation: 3,
                        seed,
                    });
                    let deficit: i64 = tessellation
                        .neighbours
                        .iter()
                        .map(|list| 6 - list.len() as i64)
                        .sum();
                    assert_eq!(
                        deficit, 12,
                        "count {region_count}, seed {seed}, jitter {jitter}: {:?}",
                        tessellation.degree_histogram()
                    );
                }
            }
        }
    }

    /// Borders have length; corners do not. This is the distinction the whole
    /// four-color guarantee rests on.
    #[test]
    fn neighbours_share_a_border_and_others_do_not() {
        use crate::{Params, Tessellation};
        let tessellation = Tessellation::generate(Params {
            region_count: 40,
            ..Default::default()
        });
        for first in 0..40 {
            for second in (first + 1)..40 {
                let length = shared_border_length(&tessellation.seeds, first, second);
                let listed = tessellation.neighbours[first].contains(&(second as u32));
                assert_eq!(
                    listed,
                    length > MINIMUM_ARC,
                    "{first}-{second} listed={listed} length={length}"
                );
            }
        }
    }

    /// The regions themselves must be near enough equal in area, because area is a
    /// game resource. Any unevenness a player sees on the 2D map is the projection's
    /// doing, not the tessellation's — this test is what separates the two claims.
    #[test]
    fn regions_are_close_to_equal_area_on_the_sphere() {
        use crate::{Params, Tessellation};
        for region_count in [20, 60, 150] {
            let tessellation = Tessellation::generate(Params {
                region_count,
                ..Default::default()
            });
            let areas = region_areas(&tessellation.seeds, 400_000);
            let largest = areas.iter().cloned().fold(0.0, f64::max);
            let smallest = areas.iter().cloned().fold(f64::INFINITY, f64::min);
            let mean = 4.0 * PI / region_count as f64;
            println!(
                "{region_count:>4} regions: area ratio {:.2}, largest {:.1}% of mean, \
                 smallest {:.1}% of mean",
                largest / smallest,
                100.0 * largest / mean,
                100.0 * smallest / mean
            );
            assert!(
                largest / smallest < 2.0,
                "{region_count} regions: area ratio {:.2} exceeds the target of 2.0",
                largest / smallest
            );
        }
    }

    #[test]
    fn hexagons_dominate_once_there_are_enough_regions() {
        use crate::lattice::{recommended_sample_count, relax};
        let count = 120;
        let mut seeds = fibonacci_lattice(count);
        relax(
            &mut seeds,
            3,
            &fibonacci_lattice(recommended_sample_count(count)),
        );
        let histogram = degree_histogram(&adjacency(&seeds));
        let hexagons = histogram.get(6).copied().unwrap_or(0);
        assert!(
            hexagons * 2 > count,
            "expected mostly six neighbours, got {histogram:?}"
        );
    }
}
