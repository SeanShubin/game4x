//! The solid itself: where the corners of the regions actually are.
//!
//! The rest of this crate works in seeds and adjacency. That is enough to know which
//! regions touch, and enough to shade a pixel by asking which seed is nearest - which is
//! all the flat projection ever needed. Drawing the world as a three dimensional object
//! needs the thing itself: polygons, with corners.
//!
//! # Where a corner is
//!
//! A region is the set of directions for which its seed wins an `argmax` of `x . m`, so
//! the border between regions `i` and `j` lies on the plane `x . (m_i - m_j) = 0`. That
//! plane passes through the origin, which is what keeps every border a great circle.
//! Three regions meet where two such planes cross:
//!
//! ```text
//!   x . (m_i - m_j) = 0
//!   x . (m_i - m_k) = 0
//! ```
//!
//! so `x` is perpendicular to both differences, and therefore
//! `x = +/- normalize((m_i - m_j) X (m_i - m_k))`. The sign is settled by requiring the
//! corner to lie on the same side of the sphere as the regions meeting there.
//!
//! This is exact and involves no search, no sampling, and no tolerance on the answer
//! itself. It also works whether or not the seeds are unit vectors, which matters here:
//! a canonical seed is `n / h`. See [`crate::icosahedral::truncated_icosahedron_seeds`].
//!
//! # Which triples actually meet
//!
//! Three mutually adjacent regions do not have to share a corner - the graph can carry a
//! triangle whose three borders never meet at a point. So a candidate is kept only if it
//! survives the test that defines a region in the first place: no *other* seed may be
//! closer to it. That is the same `argmax` the renderer uses, so a corner is part of the
//! solid exactly when the picture agrees that it is.

use crate::vec3::{Direction, Vec3};

/// How much closer another seed must be before a candidate corner is rejected.
///
/// At a true corner three seeds tie exactly, and the nearest rival loses by a wide
/// margin - on the order of the region spacing squared. The tolerance only has to
/// absorb rounding in the cross product.
const TIE: f64 = 1e-9;

/// A tessellation as a polyhedron: corners, and the polygon of each region.
#[derive(Clone, Debug)]
pub struct Solid {
    /// Every point where three regions meet, once each.
    pub corners: Vec<Direction>,
    /// For each region, the indices of its corners in counter-clockwise order seen
    /// from outside the sphere.
    pub cells: Vec<Vec<u32>>,
}

impl Solid {
    /// Every border counted once. Each is shared by exactly two regions.
    pub fn edge_count(&self) -> usize {
        self.cells.iter().map(|cell| cell.len()).sum::<usize>() / 2
    }

    /// `V - E + F`, which is two for any polyhedron. A cheap check that the solid
    /// actually closed up rather than merely producing plausible numbers.
    pub fn euler_characteristic(&self) -> i64 {
        self.corners.len() as i64 - self.edge_count() as i64 + self.cells.len() as i64
    }
}

/// Builds the polyhedron from seeds and their adjacency.
///
/// Both must come from the same tessellation; passing mismatched ones is a programming
/// error rather than a supported case.
pub fn solid(seeds: &[Vec3], neighbours: &[Vec<u32>]) -> Solid {
    assert_eq!(
        seeds.len(),
        neighbours.len(),
        "seeds and adjacency disagree"
    );

    let mut corners: Vec<Direction> = Vec::new();
    let mut cells: Vec<Vec<u32>> = vec![Vec::new(); seeds.len()];

    for first in 0..seeds.len() {
        let around = &neighbours[first];
        for (offset, &second) in around.iter().enumerate() {
            if (second as usize) < first {
                continue;
            }
            for &third in &around[offset + 1..] {
                if (third as usize) < first || !adjacent(neighbours, second, third) {
                    continue;
                }
                let (second, third) = (second as usize, third as usize);
                if let Some(corner) = meeting_point(seeds, first, second, third) {
                    let index = corners.len() as u32;
                    corners.push(corner);
                    cells[first].push(index);
                    cells[second].push(index);
                    cells[third].push(index);
                }
            }
        }
    }

    for (region, cell) in cells.iter_mut().enumerate() {
        order_around(Direction::of(seeds[region]), &corners, cell);
    }

    Solid { corners, cells }
}

fn adjacent(neighbours: &[Vec<u32>], of: u32, to: u32) -> bool {
    neighbours[of as usize].binary_search(&to).is_ok()
}

/// The corner where three regions meet, or `None` if they do not meet at one.
fn meeting_point(seeds: &[Vec3], first: usize, second: usize, third: usize) -> Option<Direction> {
    let normal = seeds[first]
        .sub(seeds[second])
        .cross(seeds[first].sub(seeds[third]));
    if normal.length() < 1e-12 {
        // The two border planes coincide, so they never cross in a single point.
        return None;
    }
    // Of the two solutions, take the one on the same side as the regions themselves.
    let candidate = normal.normalized();
    let candidate = if candidate.dot(seeds[first]) < 0.0 {
        candidate.scaled(-1.0)
    } else {
        candidate
    };

    // The three tie by construction; the corner is real only if nothing else wins.
    let tie = candidate.dot(seeds[first]);
    let beaten = seeds.iter().enumerate().any(|(region, &seed)| {
        region != first && region != second && region != third && candidate.dot(seed) > tie + TIE
    });
    (!beaten).then(|| Direction::of(candidate))
}

/// Sorts a region's corners into a polygon rather than an arbitrary heap of points.
///
/// The corners all lie around the region's own direction, so measuring each one's angle
/// in the tangent plane there and sorting by it walks the boundary. The basis is built
/// right-handed against the outward direction, so increasing angle is counter-clockwise
/// seen from outside.
fn order_around(centre: Direction, corners: &[Direction], cell: &mut [u32]) {
    let outward = centre.vector();
    let across = outward.any_perpendicular();
    let up = outward.cross(across);
    cell.sort_by(|&left, &right| {
        let angle = |index: u32| {
            let corner = corners[index as usize].vector();
            corner.dot(up).atan2(corner.dot(across))
        };
        angle(left)
            .partial_cmp(&angle(right))
            .expect("corner angles are finite")
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Params, Tessellation, goldberg};

    /// The truncated icosahedron is the case whose answer is known without measuring:
    /// sixty corners, ninety edges, thirty-two faces.
    #[test]
    fn the_soccer_ball_comes_out_as_the_solid_it_is() {
        let world = Tessellation::soccer_ball();
        let solid = solid(&world.seeds, &world.neighbours);

        assert_eq!(solid.corners.len(), 60, "vertices");
        assert_eq!(solid.edge_count(), 90, "edges");
        assert_eq!(solid.cells.len(), 32, "faces");
        assert_eq!(solid.euler_characteristic(), 2);

        let sides = |count: usize| solid.cells.iter().filter(|c| c.len() == count).count();
        assert_eq!(sides(5), 12, "pentagons");
        assert_eq!(sides(6), 20, "hexagons");
    }

    /// Every Goldberg polyhedron should close up, and its corner count is forced: three
    /// faces meet at each corner, so `V = 2F - 4`.
    #[test]
    fn every_goldberg_polyhedron_closes_up() {
        for (m, n) in goldberg::arrangements_up_to(200) {
            let seeds = goldberg::seeds(m, n);
            let neighbours = crate::adjacency::adjacency(&seeds);
            let solid = solid(&seeds, &neighbours);
            let faces = goldberg::region_count(m, n);

            assert_eq!(solid.cells.len(), faces, "GP({m},{n}) faces");
            assert_eq!(solid.corners.len(), 2 * faces - 4, "GP({m},{n}) corners");
            assert_eq!(solid.edge_count(), 3 * faces - 6, "GP({m},{n}) edges");
            assert_eq!(solid.euler_characteristic(), 2, "GP({m},{n}) Euler");
        }
    }

    /// A region's polygon must have exactly as many sides as it has neighbours. This is
    /// what ties the drawn solid to the graph the game logic sees: if the two ever
    /// disagreed, the picture would be of a different world than the one being played.
    #[test]
    fn every_polygon_has_one_side_per_neighbour() {
        for region_count in [12, 32, 42, 92, 100] {
            let world = Tessellation::generate_balanced(
                Params {
                    region_count,
                    ..Default::default()
                },
                24,
            )
            .0;
            let solid = solid(&world.seeds, &world.neighbours);
            for (region, cell) in solid.cells.iter().enumerate() {
                assert_eq!(
                    cell.len(),
                    world.neighbours[region].len(),
                    "{region_count} regions: region {region} has {} corners but {} neighbours",
                    cell.len(),
                    world.neighbours[region].len()
                );
            }
        }
    }

    /// Three regions meet at a corner - never two, never four.
    #[test]
    fn three_regions_meet_at_every_corner() {
        let world = Tessellation::soccer_ball();
        let solid = solid(&world.seeds, &world.neighbours);
        let mut shared = vec![0usize; solid.corners.len()];
        for cell in &solid.cells {
            for &corner in cell {
                shared[corner as usize] += 1;
            }
        }
        assert!(shared.iter().all(|&count| count == 3), "{shared:?}");
    }

    /// Ordering has to produce a polygon rather than a star: consecutive corners must be
    /// one side apart, so no step around the boundary may be longer than any diagonal.
    #[test]
    fn corners_are_ordered_around_the_boundary() {
        let world = Tessellation::soccer_ball();
        let solid = solid(&world.seeds, &world.neighbours);
        for cell in &solid.cells {
            let step = |from: usize, to: usize| {
                solid.corners[cell[from] as usize].angle_to(solid.corners[cell[to] as usize])
            };
            let longest_side = (0..cell.len())
                .map(|at| step(at, (at + 1) % cell.len()))
                .fold(0.0f64, f64::max);
            for from in 0..cell.len() {
                for to in (from + 2)..cell.len() {
                    if (from, to) == (0, cell.len() - 1) {
                        continue;
                    }
                    assert!(
                        step(from, to) > longest_side,
                        "corners {from} and {to} are closer together than a side"
                    );
                }
            }
        }
    }

    /// Corners are directions, so they must land on the sphere however odd the seeds are.
    #[test]
    fn corners_are_on_the_sphere() {
        let world = Tessellation::soccer_ball();
        let solid = solid(&world.seeds, &world.neighbours);
        for corner in &solid.corners {
            assert!((corner.vector().length() - 1.0).abs() < 1e-12);
        }
    }
}
