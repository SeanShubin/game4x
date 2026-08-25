//! The adjacency graph of `GP(m, n)`, derived rather than measured.
//!
//! Everything here is integer arithmetic. No coordinates, no distances, no floating
//! point — which region touches which is a fact about a lattice, and a lattice is
//! countable.
//!
//! # Why this matters
//!
//! The alternative is to place seeds on a sphere and measure which of them share a
//! border, which is what [`crate::adjacency`] does. That works, but it decides adjacency
//! by comparing an `atan2` against a threshold, and a border of nearly zero length can
//! fall either side of it depending on the platform's maths library. `docs/layers.md`
//! records the consequence: worlds might not be reproducible across machines, so they
//! would have to be shipped as data or built on exact predicates.
//!
//! Deriving the graph instead removes the question. `(m, n)` in, adjacency out, bit for
//! bit identical everywhere, forever.
//!
//! # How it works
//!
//! The triangular lattice is the **Eisenstein integers**: `a + b*w` where `w` is a
//! sixth turn. Every operation the construction needs is exact in that ring:
//!
//! | Operation | In Eisenstein integers |
//! | --- | --- |
//! | rotate a sixth turn | `(a, b) -> (-b, a + b)` |
//! | conjugate | `(a, b) -> (a + b, -b)` |
//! | multiply | `(a, b) * (c, d) -> (ac - bd, ad + bc + bd)` |
//! | norm | `a^2 + ab + b^2`, which is `T` |
//!
//! Each icosahedral face carries a lattice patch whose corners are the origin, the
//! `(m, n)` walk, and that walk turned a sixth. A lattice point `p` lies inside the
//! patch exactly when `p * conj(z)` has non-negative parts summing to at most `T` — and
//! those parts *are* the barycentric coordinates, as integers over `T`.
//!
//! Points on a shared edge are reached from both faces, so each is given a canonical
//! name that both agree on. Those names are then sorted and numbered, which is what
//! makes the region ids deterministic.

use std::collections::{BTreeMap, BTreeSet};

/// A point of the triangular lattice: `a + b*w`, where `w` is a sixth turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lattice {
    pub a: i64,
    pub b: i64,
}

impl Lattice {
    pub const fn new(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    /// A sixth turn anticlockwise. `w^2 = w - 1`, so this stays in the ring.
    pub fn turned(self) -> Self {
        Self::new(-self.b, self.a + self.b)
    }

    pub fn conjugate(self) -> Self {
        Self::new(self.a + self.b, -self.b)
    }

    pub fn times(self, other: Self) -> Self {
        Self::new(
            self.a * other.a - self.b * other.b,
            self.a * other.b + self.b * other.a + self.b * other.b,
        )
    }

    /// `a^2 + ab + b^2`. For the walk `(m, n)` this is the triangulation number `T`.
    pub fn norm(self) -> i64 {
        self.a * self.a + self.a * self.b + self.b * self.b
    }

    /// The six lattice neighbours, in order.
    pub fn neighbours(self) -> [Self; 6] {
        [
            Self::new(self.a + 1, self.b),
            Self::new(self.a, self.b + 1),
            Self::new(self.a - 1, self.b + 1),
            Self::new(self.a - 1, self.b),
            Self::new(self.a, self.b - 1),
            Self::new(self.a + 1, self.b - 1),
        ]
    }
}

/// What a lattice point is, named so that two faces sharing it agree.
///
/// Ordered, so that sorting gives the same region numbering on every machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Site {
    /// An icosahedron vertex. Shared by five faces; becomes a pentagon.
    Corner(usize),
    /// On the edge between two icosahedron vertices, at a position measured from the
    /// lower-numbered one. Shared by two faces.
    Edge {
        from: usize,
        to: usize,
        along: i64,
    },
    /// Strictly inside one face.
    Inside {
        face: usize,
        a: i64,
        b: i64,
    },
}

/// Barycentric coordinates as integers over `T`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Barycentric {
    first: i64,
    second: i64,
    third: i64,
}

/// The adjacency graph of `GP(m, n)`, with regions numbered deterministically.
pub struct Topology {
    pub neighbours: Vec<Vec<u32>>,
    /// What each region is, in region order. Kept so that geometry can be attached
    /// afterwards without re-deriving anything.
    pub sites: Vec<Site>,
}

/// Derives the adjacency graph of `GP(m, n)` from the twenty icosahedral faces.
///
/// `faces` must be triples of icosahedron vertex indices, all wound the same way. That
/// is the only input, and it is a fixed combinatorial fact about the icosahedron.
///
/// # Class I only, for now
///
/// This is correct for `n = 0` and wrong for everything else, and the reason is worth
/// recording because it is not obvious.
///
/// Adjacency here is collected by walking each face and joining lattice points that are
/// lattice neighbours *within the same patch*. That is sound when the lattice lines up
/// with the patch edges, which is exactly the class I case.
///
/// When `n > 0` the lattice is turned relative to the patch, and some lattice edges
/// cross a patch boundary instead of staying inside it. `GP(1,1)` shows it plainly: the
/// vertex at a face's centre should have six neighbours — the three corners of its own
/// face and the three centres of the neighbouring faces — but the latter three lie in
/// other patches, so it comes out with three. Collecting them needs the lattice
/// continued across each shared edge, with the right turn rather than a reflection,
/// which is the substantive part of the Goldberg–Coxeter construction and is not built.
///
/// Until it is, class II and III worlds get their adjacency measured geometrically by
/// [`crate::adjacency`], which is correct but not portable — see `docs/layers.md`.
pub fn build(m: usize, n: usize, faces: &[[usize; 3]]) -> Topology {
    let walk = Lattice::new(m as i64, n as i64);
    let scale = walk.norm();
    assert!(scale > 0, "GP(0,0) is not a polyhedron");
    let conjugate = walk.conjugate();

    // Which lattice points fall inside a patch, with their barycentrics. The patch is
    // the same shape on every face, so this is computed once.
    let patch = patch_points(m, n, conjugate, scale);

    // Name every point, then sort so numbering does not depend on iteration order.
    let mut names: BTreeSet<Site> = BTreeSet::new();
    for (face, triple) in faces.iter().enumerate() {
        for &(_, bary) in &patch {
            names.insert(name_of(face, triple, bary, scale));
        }
    }
    let sites: Vec<Site> = names.into_iter().collect();
    let number: BTreeMap<Site, u32> = sites
        .iter()
        .enumerate()
        .map(|(index, &site)| (site, index as u32))
        .collect();

    // Two points are adjacent when they are lattice neighbours within some face. Every
    // edge of the graph lies inside at least one face, so walking all twenty finds them
    // all.
    let mut neighbours: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); sites.len()];
    let inside: BTreeMap<Lattice, Barycentric> = patch.iter().copied().collect();
    for (face, triple) in faces.iter().enumerate() {
        for (point, bary) in &patch {
            let here = number[&name_of(face, triple, *bary, scale)];
            for step in point.neighbours() {
                if let Some(&other) = inside.get(&step) {
                    let there = number[&name_of(face, triple, other, scale)];
                    if here != there {
                        neighbours[here as usize].insert(there);
                        neighbours[there as usize].insert(here);
                    }
                }
            }
        }
    }

    Topology {
        neighbours: neighbours
            .into_iter()
            .map(|set| set.into_iter().collect())
            .collect(),
        sites,
    }
}

/// Every lattice point inside one patch, with its barycentric coordinates.
fn patch_points(
    m: usize,
    n: usize,
    conjugate: Lattice,
    scale: i64,
) -> Vec<(Lattice, Barycentric)> {
    // The patch spans at most `m + n` steps in any direction.
    let reach = (m + n) as i64 + 1;
    let mut found = Vec::new();
    for a in -reach..=reach {
        for b in -reach..=reach {
            let point = Lattice::new(a, b);
            // Dividing by the walk gives the barycentric coordinates directly, and
            // multiplying by the conjugate is that division scaled by the norm.
            let mapped = point.times(conjugate);
            let (second, third) = (mapped.a, mapped.b);
            let first = scale - second - third;
            if first >= 0 && second >= 0 && third >= 0 {
                found.push((
                    point,
                    Barycentric {
                        first,
                        second,
                        third,
                    },
                ));
            }
        }
    }
    found
}

/// The canonical name of a point, which both faces sharing it will agree on.
fn name_of(face: usize, triple: &[usize; 3], bary: Barycentric, scale: i64) -> Site {
    let Barycentric {
        first,
        second,
        third,
    } = bary;
    let zeros = (first == 0) as u8 + (second == 0) as u8 + (third == 0) as u8;

    match zeros {
        // A corner of the patch is an icosahedron vertex.
        2 => {
            let corner = if first != 0 {
                triple[0]
            } else if second != 0 {
                triple[1]
            } else {
                triple[2]
            };
            Site::Corner(corner)
        }
        // On an edge, named from the lower-numbered end so both faces agree.
        1 => {
            let (from, to, along) = if third == 0 {
                (triple[0], triple[1], second)
            } else if first == 0 {
                (triple[1], triple[2], third)
            } else {
                (triple[2], triple[0], first)
            };
            if from < to {
                Site::Edge { from, to, along }
            } else {
                Site::Edge {
                    from: to,
                    to: from,
                    along: scale - along,
                }
            }
        }
        // Strictly inside, so it belongs to this face alone.
        _ => Site::Inside {
            face,
            a: second,
            b: third,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icosahedral::{icosahedron_faces, icosahedron_vertices};

    fn faces() -> Vec<[usize; 3]> {
        icosahedron_faces(&icosahedron_vertices())
    }

    /// Only class I is derivable so far — see the note on [`build`].
    fn class_one_up_to(limit: usize) -> Vec<(usize, usize)> {
        crate::goldberg::arrangements_up_to(limit)
            .into_iter()
            .filter(|&(_, n)| n == 0)
            .collect()
    }

    #[test]
    fn eisenstein_arithmetic_is_exact() {
        let point = Lattice::new(2, 1);
        assert_eq!(point.norm(), 7, "T = m^2 + mn + n^2");
        // Six turns is a full circle.
        let mut turned = point;
        for _ in 0..6 {
            turned = turned.turned();
        }
        assert_eq!(turned, point);
        // A number times its conjugate is its norm.
        assert_eq!(point.times(point.conjugate()), Lattice::new(7, 0));
    }

    #[test]
    fn every_point_has_six_lattice_neighbours() {
        let point = Lattice::new(3, -2);
        let around = point.neighbours();
        assert_eq!(around.len(), 6);
        for step in around {
            assert!(step.neighbours().contains(&point), "neighbours must be mutual");
        }
    }

    /// The graph is right: the correct number of regions, twelve pentagons, everything
    /// else a hexagon, and Euler satisfied.
    #[test]
    fn the_derived_graph_is_a_goldberg_polyhedron() {
        let faces = faces();
        println!("\n  GP(m,n) | regions | pentagons | hexagons");
        for (m, n) in crate::goldberg::arrangements_up_to(400)
            .into_iter()
            .filter(|&(_, n)| n == 0)
        {
            let built = build(m, n, &faces);
            let expected = crate::goldberg::region_count(m, n);
            assert_eq!(built.neighbours.len(), expected, "GP({m},{n}) region count");

            let mut pentagons = 0;
            let mut hexagons = 0;
            for list in &built.neighbours {
                match list.len() {
                    5 => pentagons += 1,
                    6 => hexagons += 1,
                    other => panic!("GP({m},{n}) produced a cell with {other} neighbours"),
                }
            }
            println!("  GP({m},{n})   | {expected:>7} | {pentagons:>9} | {hexagons:>8}");
            assert_eq!(pentagons, 12, "GP({m},{n})");
            assert_eq!(hexagons, expected - 12, "GP({m},{n})");

            let edges: usize = built.neighbours.iter().map(|l| l.len()).sum::<usize>() / 2;
            assert_eq!(edges, 3 * expected - 6, "GP({m},{n}) Euler");
        }
        println!();
    }

    /// The pentagons are the icosahedron's twelve vertices, by construction rather than
    /// by luck — and no two of them touch.
    #[test]
    fn the_pentagons_are_the_corners_and_are_isolated() {
        let faces = faces();
        for (m, n) in class_one_up_to(200) {
            let built = build(m, n, &faces);
            for (region, list) in built.neighbours.iter().enumerate() {
                if list.len() != 5 {
                    continue;
                }
                assert!(
                    matches!(built.sites[region], Site::Corner(_)),
                    "GP({m},{n}): a pentagon that is not an icosahedron vertex"
                );
                if built.neighbours.len() > 12 {
                    for &other in list {
                        assert_ne!(
                            built.neighbours[other as usize].len(),
                            5,
                            "GP({m},{n}): two pentagons touching"
                        );
                    }
                }
            }
        }
    }

    /// Derived with integers, measured with geometry: the same graph. This is the test
    /// that lets the measurement be retired.
    #[test]
    fn the_derived_graph_matches_the_measured_one() {
        let faces = faces();
        for (m, n) in class_one_up_to(200) {
            let derived = build(m, n, &faces);
            let measured = crate::adjacency::adjacency(&crate::goldberg::seeds(m, n));

            // Both describe the same shape, but number their regions differently, so
            // compare the graphs rather than the labels: sort each region's neighbour
            // degrees, then sort the whole lot.
            let signature = |lists: &Vec<Vec<u32>>| {
                let mut rows: Vec<Vec<usize>> = lists
                    .iter()
                    .map(|list| {
                        let mut row = vec![list.len()];
                        let mut around: Vec<usize> =
                            list.iter().map(|&o| lists[o as usize].len()).collect();
                        around.sort_unstable();
                        row.extend(around);
                        row
                    })
                    .collect();
                rows.sort();
                rows
            };
            assert_eq!(
                signature(&derived.neighbours),
                signature(&measured),
                "GP({m},{n}): derived and measured graphs differ"
            );
        }
    }

    /// The whole point: same inputs, same graph, with no floating point involved.
    #[test]
    fn deriving_the_same_graph_twice_gives_identical_output() {
        let faces = faces();
        for (m, n) in [(1usize, 0usize), (2, 0), (3, 0)] {
            let first = build(m, n, &faces);
            let second = build(m, n, &faces);
            assert_eq!(first.neighbours, second.neighbours);
            assert_eq!(first.sites, second.sites);
        }
    }
}
