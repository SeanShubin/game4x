//! Goldberg polyhedra: the canonical hex-dominant tilings of a sphere.
//!
//! A Goldberg polyhedron `GP(m, n)` has twelve pentagons at the icosahedron's vertices
//! and hexagons everywhere else. The pair `(m, n)` says how to walk from one pentagon
//! to the next through the hexagon lattice — `m` steps one way, turn 60 degrees, `n`
//! more — and that walk determines the whole tiling.
//!
//! | Class | Condition | Handedness |
//! | --- | --- | --- |
//! | I | `n = 0` | achiral |
//! | II | `m = n` | achiral |
//! | III | both nonzero, `m != n` | **chiral**: `GP(m,n)` and `GP(n,m)` are mirror images |
//!
//! With `T = m^2 + mn + n^2`, the polyhedron has `10T + 2` faces, `30T` edges and `20T`
//! vertices, and exactly twelve of the faces are pentagons — a consequence of Euler's
//! formula, not of the construction.
//!
//! # Why these and not a relaxed point set
//!
//! Every Goldberg polyhedron is a *perfect* hex-dominant tiling: no cell has four or
//! seven neighbours, no two pentagons touch, and the cells are as compact as hexagons
//! can be. A relaxed random point set reaches that only sometimes and only at small
//! sizes. See `docs/theory/region-splitting.md`.
//!
//! # Construction
//!
//! A Goldberg polyhedron is the **dual** of a geodesic polyhedron, so its faces
//! correspond to that geodesic's vertices — which is exactly what a seed is here. The
//! job is therefore to place the geodesic's vertices.
//!
//! Each of the icosahedron's twenty triangular faces carries a patch of triangular
//! lattice, oriented by the `(m, n)` vector. Lattice points inside the patch are mapped
//! onto the sphere by spherical barycentric interpolation, and points on shared edges
//! are merged.

use crate::icosahedral::{icosahedron_faces, icosahedron_vertices};
use crate::vec3::Vec3;

/// How close two points must be to count as the same lattice point.
///
/// Loose on purpose: `acos` amplifies error near zero, so two points differing by one
/// bit per coordinate already read as `1.4e-8` radians apart. Distinct points are never
/// closer than about `1.1 / (m + n)` radians.
const SAME_POINT: f64 = 1e-6;

/// `T = m^2 + mn + n^2`, the triangulation number.
pub fn triangulation_number(m: usize, n: usize) -> usize {
    m * m + m * n + n * n
}

/// How many regions `GP(m, n)` has.
pub fn region_count(m: usize, n: usize) -> usize {
    10 * triangulation_number(m, n) + 2
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    One,
    Two,
    Three,
}

pub fn class_of(m: usize, n: usize) -> Class {
    if n == 0 {
        Class::One
    } else if m == n {
        Class::Two
    } else {
        Class::Three
    }
}

/// Whether `GP(m, n)` is chiral — that is, whether its mirror image is a genuinely
/// different solid rather than a rotation of itself.
pub fn is_chiral(m: usize, n: usize) -> bool {
    class_of(m, n) == Class::Three
}

/// Every `(m, n)` whose region count is at most `limit`, in ascending count order.
///
/// Some counts appear twice: 492 is both `GP(7,0)` and `GP(5,3)`, because
/// `49 = 7^2 = 5^2 + 5*3 + 3^2`. Those are different solids with the same number of
/// regions.
pub fn arrangements_up_to(limit: usize) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    for m in 1..=40 {
        for n in 0..=m {
            if region_count(m, n) <= limit {
                found.push((m, n));
            }
        }
    }
    found.sort_by_key(|&(m, n)| (region_count(m, n), m, n));
    found
}

/// Seeds for `GP(m, n)`, one per region.
pub fn seeds(m: usize, n: usize) -> Vec<Vec3> {
    assert!(m >= 1, "a Goldberg polyhedron needs m >= 1");
    let vertices = icosahedron_vertices();
    let faces = icosahedron_faces(&vertices);

    // The lattice patch covering one icosahedral face. Its first corner is the origin,
    // the second is the (m, n) walk, and the third is that walk turned 60 degrees.
    let corner_one = lattice_point(m as f64, n as f64);
    let corner_two = rotate_sixty(corner_one);

    let mut points = Vec::new();
    for [a, b, c] in faces {
        for (first, second, third) in lattice_barycentrics(m, n, corner_one, corner_two) {
            points.push(spherical_barycentric(
                vertices[a],
                vertices[b],
                vertices[c],
                first,
                second,
                third,
            ));
        }
    }
    deduplicate(points)
}

/// A point in the triangular lattice, in plane coordinates.
fn lattice_point(along: f64, across: f64) -> (f64, f64) {
    // Basis vectors sixty degrees apart.
    (along + across * 0.5, across * (3.0f64).sqrt() * 0.5)
}

fn rotate_sixty((x, y): (f64, f64)) -> (f64, f64) {
    let (sin, cos) = (60.0f64).to_radians().sin_cos();
    (x * cos - y * sin, x * sin + y * cos)
}

/// Barycentric coordinates of every lattice point inside the patch.
fn lattice_barycentrics(
    m: usize,
    n: usize,
    corner_one: (f64, f64),
    corner_two: (f64, f64),
) -> Vec<(f64, f64, f64)> {
    // The patch is spanned by two vectors of length ~ (m + n), so lattice indices
    // outside that range cannot land inside it.
    let reach = (m + n) as i64 + 1;
    let determinant = corner_one.0 * corner_two.1 - corner_one.1 * corner_two.0;

    let mut found = Vec::new();
    for along in -reach..=reach {
        for across in -reach..=reach {
            let point = lattice_point(along as f64, across as f64);
            // Solve point = second * corner_one + third * corner_two.
            let second = (point.0 * corner_two.1 - point.1 * corner_two.0) / determinant;
            let third = (corner_one.0 * point.1 - corner_one.1 * point.0) / determinant;
            let first = 1.0 - second - third;

            const EDGE: f64 = 1e-9;
            if first >= -EDGE && second >= -EDGE && third >= -EDGE {
                found.push((first, second, third));
            }
        }
    }
    found
}

/// Maps barycentric coordinates onto a spherical triangle.
///
/// Interpolating linearly in three dimensions and then normalizing is simpler, but it
/// bunches points toward the middle of each face: at `GP(7,0)` that showed up as an
/// area ratio of 1.73 between the largest and smallest region. Interpolating *along the
/// sphere* keeps the spacing even.
///
/// The construction is applied once per corner and averaged, so that no corner is
/// privileged and the result has the same symmetry as the triangle.
fn spherical_barycentric(
    a: Vec3,
    b: Vec3,
    c: Vec3,
    first: f64,
    second: f64,
    third: f64,
) -> Vec3 {
    let mut total = Vec3::ZERO;
    // The apex's own weight is implied: the three sum to one.
    for (apex, left, right, left_weight, right_weight) in [
        (a, b, c, second, third),
        (b, c, a, third, first),
        (c, a, b, first, second),
    ] {
        let opposite = left_weight + right_weight;
        if opposite <= 1e-12 {
            total = total.add(apex);
            continue;
        }
        // Where the ray from the apex crosses the opposite edge, then how far along it.
        let crossing = slerp(left, right, right_weight / opposite);
        total = total.add(slerp(apex, crossing, opposite));
    }
    total.normalized()
}

/// Interpolation along the great circle between two directions.
fn slerp(from: Vec3, to: Vec3, amount: f64) -> Vec3 {
    let angle = from.angle_to(to);
    if angle < 1e-12 {
        return from;
    }
    let sin = angle.sin();
    from.scaled(((1.0 - amount) * angle).sin() / sin)
        .add(to.scaled((amount * angle).sin() / sin))
        .normalized()
}

/// Faces meet along shared edges, so points there are generated once per face.
fn deduplicate(points: Vec<Vec3>) -> Vec<Vec3> {
    let mut unique: Vec<Vec3> = Vec::with_capacity(points.len());
    for point in points {
        if !unique.iter().any(|kept| kept.angle_to(point) < SAME_POINT) {
            unique.push(point);
        }
    }
    unique
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adjacency::{adjacency, degree_histogram, edge_count};
    use crate::Quality;

    #[test]
    fn the_counting_formula_matches_the_known_solids() {
        assert_eq!(region_count(1, 0), 12, "dodecahedron");
        assert_eq!(region_count(1, 1), 32, "truncated icosahedron");
        assert_eq!(region_count(2, 0), 42);
        assert_eq!(region_count(2, 1), 72);
        assert_eq!(region_count(3, 0), 92);
        assert_eq!(region_count(5, 3), 492);
        assert_eq!(region_count(7, 0), 492, "the same count, a different solid");
    }

    #[test]
    fn the_classes_are_named_correctly() {
        assert_eq!(class_of(2, 0), Class::One);
        assert_eq!(class_of(3, 3), Class::Two);
        assert_eq!(class_of(5, 3), Class::Three);
        assert!(!is_chiral(2, 0));
        assert!(!is_chiral(3, 3));
        assert!(is_chiral(5, 3), "class III solids come in mirror pairs");
    }

    /// The property that makes these worth deferring to: every one of them is a perfect
    /// hex-dominant tiling, across all three classes.
    #[test]
    fn every_arrangement_is_a_perfect_hex_tiling() {
        println!("\n  GP(m,n) | class | regions | pent | hex | touching | area | edges");
        for (m, n) in arrangements_up_to(220) {
            let seeds = seeds(m, n);
            let expected = region_count(m, n);
            assert_eq!(seeds.len(), expected, "GP({m},{n}) seed count");

            let neighbours = adjacency(&seeds);
            let histogram = degree_histogram(&neighbours);
            let pentagons = histogram.get(5).copied().unwrap_or(0);
            let hexagons = histogram.get(6).copied().unwrap_or(0);
            let quality = Quality::measure(&seeds, &neighbours);

            println!(
                "  GP({m},{n})   | {:>5?} | {expected:>7} | {pentagons:>4} | {hexagons:>3} | \
{:>8} | {:.2} | {:.2}",
                class_of(m, n),
                quality.adjacent_pentagon_pairs,
                quality.area_ratio,
                quality.border_ratio
            );

            assert_eq!(pentagons, 12, "GP({m},{n}) must have twelve pentagons");
            assert_eq!(hexagons, expected - 12, "GP({m},{n}) the rest must be hexagons");
            assert_eq!(edge_count(&neighbours), 3 * expected - 6, "GP({m},{n}) Euler");
            if expected > 12 {
                assert_eq!(
                    quality.adjacent_pentagon_pairs, 0,
                    "GP({m},{n}) pentagons must be isolated"
                );
            }
        }
        println!();
    }

    /// Class III is chiral, so `GP(m,n)` and `GP(n,m)` should be genuinely different
    /// solids rather than the same one rotated — while having identical counts.
    #[test]
    fn mirror_pairs_have_the_same_size_and_a_different_shape() {
        assert_eq!(region_count(2, 1), region_count(1, 2));
        let left = seeds(2, 1);
        let right = seeds(1, 2);
        assert_eq!(left.len(), right.len());
        for point in &left {
            assert!((point.length() - 1.0).abs() < 1e-9);
        }
        for point in &right {
            assert!((point.length() - 1.0).abs() < 1e-9);
        }
    }

    /// Two different solids with 492 regions each. This is why "one region count, one
    /// shape" is not a rule that scales.
    #[test]
    fn four_hundred_and_ninety_two_admits_two_solids() {
        let class_one = seeds(7, 0);
        let class_three = seeds(5, 3);
        assert_eq!(class_one.len(), 492);
        assert_eq!(class_three.len(), 492);
        assert_eq!(class_of(7, 0), Class::One);
        assert_eq!(class_of(5, 3), Class::Three);
    }
}
