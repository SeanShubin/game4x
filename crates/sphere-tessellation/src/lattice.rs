//! Seed point generation: the Fibonacci lattice, jitter, and Lloyd relaxation.
//!
//! These are steps 1 through 3 of the pipeline in `docs/theory/region-splitting.md`.

use crate::rng::Rng;
use crate::vec3::Vec3;
use std::f64::consts::{PI, TAU};

/// Spreads `count` points over the sphere using the golden-spiral lattice.
///
/// Sampling z uniformly gives equal-area bands; advancing the angle by the golden
/// angle keeps the points from settling into rows. O(count), closed form, any count.
pub fn fibonacci_lattice(count: usize) -> Vec<Vec3> {
    if count == 0 {
        return Vec::new();
    }
    let golden_ratio = (1.0 + 5.0_f64.sqrt()) / 2.0;
    (0..count)
        .map(|index| {
            let z = 1.0 - (2 * index + 1) as f64 / count as f64;
            let radius = (1.0 - z * z).max(0.0).sqrt();
            let theta = TAU * index as f64 / golden_ratio;
            Vec3::new(radius * theta.cos(), radius * theta.sin(), z)
        })
        .collect()
}

/// Mean angular distance between neighbouring seeds, roughly `2/sqrt(count)`.
pub fn mean_spacing(count: usize) -> f64 {
    if count <= 1 {
        PI
    } else {
        2.0 / (count as f64).sqrt()
    }
}

/// Displaces each seed by a random tangential offset.
///
/// `fraction` is measured against the mean seed spacing. 0.0 leaves the lattice
/// untouched and looks computed; 0.2 to 0.4 is the intended range; beyond about 0.6
/// cells start to degenerate.
pub fn jitter(points: &mut [Vec3], fraction: f64, rng: &mut Rng) {
    if fraction <= 0.0 {
        return;
    }
    let max_offset = fraction * mean_spacing(points.len());
    for point in points.iter_mut() {
        let basis_u = point.any_perpendicular();
        let basis_w = point.cross(basis_u).normalized();
        let heading = rng.unit() * TAU;
        // The square root spreads offsets uniformly over the disc rather than
        // bunching them toward the centre.
        let distance = max_offset * rng.unit().sqrt();
        let tangent = basis_u
            .scaled(heading.cos())
            .add(basis_w.scaled(heading.sin()));
        *point = point.moved_along(tangent, distance);
    }
}

/// Moves each seed toward the centroid of its own Voronoi cell, `iterations` times.
///
/// The cells are approximated by a dense sample set rather than constructed as
/// polygons — much less code, and accurate enough given that we deliberately stop
/// well short of convergence. Two to four iterations keep cells compact without
/// undoing the irregularity that jitter bought.
pub fn relax(points: &mut [Vec3], iterations: usize, samples: &[Vec3]) {
    if points.len() < 2 || iterations == 0 {
        return;
    }
    let mut totals = vec![Vec3::ZERO; points.len()];
    let mut counts = vec![0u32; points.len()];
    for _ in 0..iterations {
        totals.iter_mut().for_each(|t| *t = Vec3::ZERO);
        counts.iter_mut().for_each(|c| *c = 0);
        for &sample in samples {
            let owner = nearest_index(points, sample);
            totals[owner] = totals[owner].add(sample);
            counts[owner] += 1;
        }
        for index in 0..points.len() {
            // A cell that captured no samples keeps its seed; moving it would be
            // guesswork, and it means the sample set is too sparse for this count.
            if counts[index] > 0 && totals[index].length() > 1e-12 {
                points[index] = totals[index].normalized();
            }
        }
    }
}

/// How many sample points `relax` should use for a given region count.
pub fn recommended_sample_count(region_count: usize) -> usize {
    (region_count * 400).max(8_192)
}

pub fn nearest_index(points: &[Vec3], direction: Vec3) -> usize {
    let mut best_index = 0;
    let mut best_dot = f64::NEG_INFINITY;
    for (index, &point) in points.iter().enumerate() {
        let dot = point.dot(direction);
        if dot > best_dot {
            best_dot = dot;
            best_index = index;
        }
    }
    best_index
}

/// The two closest seeds to `direction`, with their cosines, nearest first.
///
/// Cosines rather than angles because the caller usually only needs to compare them,
/// and `acos` is expensive when this runs once per pixel.
pub fn nearest_two(points: &[Vec3], direction: Vec3) -> (usize, f64, usize, f64) {
    let mut best = (0usize, f64::NEG_INFINITY);
    let mut second = (0usize, f64::NEG_INFINITY);
    for (index, &point) in points.iter().enumerate() {
        let dot = point.dot(direction);
        if dot > best.1 {
            second = best;
            best = (index, dot);
        } else if dot > second.1 {
            second = (index, dot);
        }
    }
    (best.0, best.1, second.0, second.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lattice_points_are_unit_vectors() {
        for count in [1, 2, 3, 7, 20, 137] {
            let points = fibonacci_lattice(count);
            assert_eq!(points.len(), count);
            for point in points {
                assert!((point.length() - 1.0).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn empty_lattice_is_allowed() {
        assert!(fibonacci_lattice(0).is_empty());
    }

    #[test]
    fn lattice_is_reasonably_even() {
        // No seed should be much closer to its nearest neighbour than the mean
        // spacing predicts. This is the property jitter and relaxation build on.
        let count = 200;
        let points = fibonacci_lattice(count);
        let expected = mean_spacing(count);
        for (i, &a) in points.iter().enumerate() {
            let closest = points
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, &b)| a.angle_to(b))
                .fold(f64::INFINITY, f64::min);
            assert!(
                closest > expected * 0.5,
                "seed {i} is crowded: {closest} vs expected {expected}"
            );
        }
    }

    #[test]
    fn jitter_is_bounded_and_deterministic() {
        let count = 60;
        let original = fibonacci_lattice(count);
        let limit = 0.35 * mean_spacing(count);

        let mut first = original.clone();
        jitter(&mut first, 0.35, &mut Rng::new(99));
        let mut second = original.clone();
        jitter(&mut second, 0.35, &mut Rng::new(99));

        assert_eq!(first, second, "same seed must give the same jitter");
        for (before, after) in original.iter().zip(first.iter()) {
            assert!(before.angle_to(*after) <= limit + 1e-9);
            assert!((after.length() - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn relaxation_evens_out_a_jittered_lattice() {
        let count = 80;
        let samples = fibonacci_lattice(recommended_sample_count(count));

        let mut jittered = fibonacci_lattice(count);
        jitter(&mut jittered, 0.5, &mut Rng::new(5));
        let before = nearest_neighbour_spread(&jittered);

        let mut relaxed = jittered.clone();
        relax(&mut relaxed, 4, &samples);
        let after = nearest_neighbour_spread(&relaxed);

        assert!(
            after < before,
            "relaxation should reduce spacing spread: {before} -> {after}"
        );
    }

    /// Ratio of largest to smallest nearest-neighbour distance.
    fn nearest_neighbour_spread(points: &[Vec3]) -> f64 {
        let distances: Vec<f64> = points
            .iter()
            .enumerate()
            .map(|(i, &a)| {
                points
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, &b)| a.angle_to(b))
                    .fold(f64::INFINITY, f64::min)
            })
            .collect();
        let max = distances.iter().cloned().fold(0.0, f64::max);
        let min = distances.iter().cloned().fold(f64::INFINITY, f64::min);
        max / min
    }

    #[test]
    fn nearest_two_orders_by_proximity() {
        let points = fibonacci_lattice(30);
        let probe = Vec3::from_lon_lat(1.1, 0.3);
        let (first, first_dot, second, second_dot) = nearest_two(&points, probe);
        assert!(first_dot >= second_dot);
        assert_ne!(first, second);
        assert_eq!(first, nearest_index(&points, probe));
    }
}
