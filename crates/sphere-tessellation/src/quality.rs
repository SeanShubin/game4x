//! Measuring whether a tessellation makes good territory.
//!
//! Four things are wanted of a territory, and they are not independent — Lloyd
//! relaxation improves all four at once, which is why there is one dial rather than
//! four:
//!
//! | Want | Measured by | Why it matters |
//! | --- | --- | --- |
//! | Roughly equal area | [`Quality::area_ratio`] | area is a game resource |
//! | Roughly six neighbours | [`Quality::hex_fraction`], [`Quality::odd_degrees`] | uniform movement, no diagonal ambiguity |
//! | Compact, not slivered | [`Quality::worst_compactness`] | slivers look wrong and behave wrong |
//! | Reasonably short borders | [`Quality::mean_perimeter`], [`Quality::shortest_border`] | long borders mean sprawling shapes; near-zero borders are numerically fragile |
//!
//! Two of these have hard floors that no amount of relaxation can beat, and it is worth
//! knowing them before reading the numbers:
//!
//! - **The mean degree is always almost exactly 6.** Euler's formula forces the total
//!   degree deficit to be exactly 12, so with `n` regions the mean is `6 - 12/n`. That
//!   is not a quality signal; the *distribution* is. Twelve pentagons are mandatory, so
//!   the target is "every cell is a pentagon or a hexagon", not "every cell is a
//!   hexagon".
//! - **Perfect compactness is impossible.** A regular hexagon scores about 0.907 on the
//!   isoperimetric quotient `4*pi*A / P^2`, and no tiling of anything does better with
//!   hexagons.
//!
//! # Equal areas and equal edges are mutually exclusive
//!
//! Worth stating plainly, because both are reasonable things to want and only one is
//! available at a time.
//!
//! A regular hexagon has 1.51 times the area of a regular pentagon with the same edge
//! length. So a solid whose ninety edges are all equal — the Archimedean truncated
//! icosahedron, the shape in every published picture of a Goldberg polyhedron — has
//! pentagons that are genuinely smaller than its hexagons. Measured on ours: hexagons
//! are 1.54 times the area of pentagons, and [`Quality::area_ratio`] reads about 1.54.
//!
//! Forcing the areas equal instead pushes the bisectors elsewhere and the edges come out
//! 45% apart, with every hexagon carrying three long sides and three short ones.
//!
//! The project currently chooses **equal edges** at the canonical counts, because that is
//! what makes the shape read correctly. [`Quality::area_ratio`] is therefore *expected*
//! to be around 1.5 on those worlds and is not a defect. Relaxation-built worlds, which
//! have no canonical form to preserve, still target equal areas.

use crate::adjacency::{region_areas, shared_border_length};
use crate::vec3::Vec3;
use std::f64::consts::PI;

/// The isoperimetric quotient of a regular hexagon — the practical ceiling for a
/// hex-dominant tessellation.
pub const REGULAR_HEXAGON: f64 = 0.9069;

/// How many sample points per region the area measurement uses.
const AREA_SAMPLES_PER_REGION: usize = 4_000;

#[derive(Clone, Debug, PartialEq)]
pub struct Quality {
    pub region_count: usize,

    /// Largest region area divided by smallest. 1.0 is perfect.
    pub area_ratio: f64,
    /// Largest area as a fraction of the mean.
    pub largest_area: f64,
    /// Smallest area as a fraction of the mean.
    pub smallest_area: f64,

    /// How many regions have each neighbour count, indexed by that count.
    pub degrees: Vec<usize>,
    /// Fraction of regions with exactly six neighbours.
    pub hex_fraction: f64,
    /// Regions that are neither pentagons nor hexagons. Twelve pentagons are mandatory,
    /// so this is the honest count of what went wrong.
    pub odd_degrees: usize,

    /// Isoperimetric quotient `4*pi*A / P^2`, averaged. See [`REGULAR_HEXAGON`].
    pub mean_compactness: f64,
    /// The worst single region. Below about 0.6 is a sliver.
    pub worst_compactness: f64,

    /// Mean perimeter, in radians.
    pub mean_perimeter: f64,
    /// Total length of all borders, counting each once.
    pub total_boundary: f64,
    /// The shortest single border. Near-zero borders are where four regions almost meet
    /// at a point, and they are numerically fragile.
    pub shortest_border: f64,
    /// Longest border divided by shortest.
    pub border_ratio: f64,

    /// Pairs of pentagons that share a border.
    ///
    /// This is what separates "the right census" from "looks like a soccer ball". The
    /// twelve pentagons are mandatory; what makes the shape read correctly is that they
    /// are *spread out*, each ringed by hexagons. Clustered pentagons look lumpy even
    /// when the counts are perfect.
    pub adjacent_pentagon_pairs: usize,
}

impl Quality {
    pub fn measure(seeds: &[Vec3], neighbours: &[Vec<u32>]) -> Self {
        let region_count = seeds.len();
        if region_count == 0 {
            return Self::empty();
        }

        let areas = region_areas(seeds, region_count * AREA_SAMPLES_PER_REGION);
        let mean_area = 4.0 * PI / region_count as f64;
        let largest = areas.iter().cloned().fold(0.0, f64::max);
        let smallest = areas.iter().cloned().fold(f64::INFINITY, f64::min);

        let mut perimeters = vec![0.0; region_count];
        let mut borders = Vec::new();
        for first in 0..region_count {
            for &second in &neighbours[first] {
                let length = shared_border_length(seeds, first, second as usize);
                perimeters[first] += length;
                if (second as usize) > first {
                    borders.push(length);
                }
            }
        }

        let compactness: Vec<f64> = (0..region_count)
            .map(|region| {
                let perimeter = perimeters[region];
                if perimeter <= 0.0 {
                    // A lone region has no border and is trivially a whole sphere.
                    1.0
                } else {
                    4.0 * PI * areas[region] / (perimeter * perimeter)
                }
            })
            .collect();

        let widest = neighbours.iter().map(|list| list.len()).max().unwrap_or(0);
        let mut degrees = vec![0usize; widest + 1];
        for list in neighbours {
            degrees[list.len()] += 1;
        }
        let hexagons = degrees.get(6).copied().unwrap_or(0);
        let pentagons = degrees.get(5).copied().unwrap_or(0);

        Self {
            region_count,
            area_ratio: if smallest > 0.0 { largest / smallest } else { f64::INFINITY },
            largest_area: largest / mean_area,
            smallest_area: smallest / mean_area,
            hex_fraction: hexagons as f64 / region_count as f64,
            odd_degrees: region_count - hexagons - pentagons,
            degrees,
            mean_compactness: mean(&compactness),
            worst_compactness: compactness.iter().cloned().fold(f64::INFINITY, f64::min),
            mean_perimeter: mean(&perimeters),
            total_boundary: borders.iter().sum(),
            shortest_border: borders.iter().cloned().fold(f64::INFINITY, f64::min),
            adjacent_pentagon_pairs: {
                let mut pairs = 0;
                for first in 0..region_count {
                    if neighbours[first].len() != 5 {
                        continue;
                    }
                    for &second in &neighbours[first] {
                        if (second as usize) > first && neighbours[second as usize].len() == 5 {
                            pairs += 1;
                        }
                    }
                }
                pairs
            },
            border_ratio: {
                let longest = borders.iter().cloned().fold(0.0, f64::max);
                let shortest = borders.iter().cloned().fold(f64::INFINITY, f64::min);
                if shortest > 0.0 { longest / shortest } else { f64::INFINITY }
            },
        }
    }

    fn empty() -> Self {
        Self {
            region_count: 0,
            area_ratio: 1.0,
            largest_area: 1.0,
            smallest_area: 1.0,
            degrees: Vec::new(),
            hex_fraction: 0.0,
            odd_degrees: 0,
            mean_compactness: 1.0,
            worst_compactness: 1.0,
            mean_perimeter: 0.0,
            total_boundary: 0.0,
            shortest_border: 0.0,
            border_ratio: 1.0,
            adjacent_pentagon_pairs: 0,
        }
    }

    /// How many cells may be neither a pentagon nor a hexagon and still count as
    /// "approximately six neighbours": two percent of them, and none at all below fifty
    /// regions.
    ///
    /// Demanding exactly zero is achievable at small counts and not at large ones.
    /// Lloyd relaxation converges *geometrically* — it evens out areas and rounds off
    /// cells — but it does not guarantee clean topology, and a stuck square or heptagon
    /// can survive any number of passes. Removing the last one needs a topological
    /// repair (flipping the offending edge), which is not built. Measured: 32 and 60
    /// regions reach zero in four passes; 150 still had one defect after eighty.
    pub fn allowed_odd_degrees(region_count: usize) -> usize {
        region_count / 50
    }

    /// Whether this is good enough to play on.
    ///
    /// The thresholds are deliberately modest. They describe "a believable world", not
    /// "the best achievable tessellation" — pushing further costs the irregularity that
    /// makes the world look natural rather than computed.
    pub fn is_playable(&self) -> bool {
        self.region_count < 4
            || (self.area_ratio < 1.3
                && self.odd_degrees <= Self::allowed_odd_degrees(self.region_count)
                && self.worst_compactness > 0.6
                && self.adjacent_pentagon_pairs == 0)
    }

    /// A single line, for a readout.
    pub fn summary(&self) -> String {
        format!(
            "area {:.2}x  hex {:.0}%  odd {}  compact {:.2}  touching pentagons {}",
            self.area_ratio,
            self.hex_fraction * 100.0,
            self.odd_degrees,
            self.mean_compactness,
            self.adjacent_pentagon_pairs
        )
    }
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Params, Tessellation};

    /// The reference solid should score close to perfect on all four counts. If it does
    /// not, the measurement is wrong rather than the solid.
    #[test]
    fn the_truncated_icosahedron_scores_well() {
        let solid = Tessellation::soccer_ball();
        let quality = Quality::measure(&solid.seeds, &solid.neighbours);
        println!("\n  soccer ball: {}", quality.summary());

        assert_eq!(quality.odd_degrees, 0, "every cell is a pentagon or hexagon");
        assert_eq!(quality.hex_fraction, 20.0 / 32.0);
        assert_eq!(quality.degrees[5], 12);
        assert_eq!(quality.degrees[6], 20);
        // Around 1.5, and that is correct rather than a defect: equal edges force the
        // pentagons to be smaller. See the note at the top of this module.
        assert!(
            (1.4..1.7).contains(&quality.area_ratio),
            "an equal-edge solid should show its pentagons as smaller: {}",
            quality.area_ratio
        );
        assert!(
            quality.worst_compactness > 0.7,
            "no slivers: {}",
            quality.worst_compactness
        );
    }

    /// The canonical solids are handed back untouched, because relaxing one would
    /// renormalize its seeds and throw away the plane distances that make the edges
    /// equal.
    #[test]
    fn canonical_worlds_are_not_relaxed() {
        use crate::Params;
        let (world, passes) = Tessellation::generate_balanced(
            Params { region_count: 32, jitter: 0.0, relaxation: 0, seed: 1 },
            24,
        );
        assert_eq!(passes, 0, "a canonical world needs no relaxation");
        assert!(
            world.verify_truncated_icosahedron().is_perfect(),
            "relaxation damaged the solid: {}",
            world.quality().summary()
        );
    }

    /// Relaxation is the one dial, and it should move all four measures the right way.
    #[test]
    fn relaxation_improves_every_measure() {
        let generate = |relaxation| {
            let world = Tessellation::generate(Params {
                region_count: 60,
                jitter: 0.25,
                relaxation,
                seed: 3,
            });
            Quality::measure(&world.seeds, &world.neighbours)
        };

        let raw = generate(0);
        let relaxed = generate(24);
        println!("\n  raw     : {}", raw.summary());
        println!("  relaxed : {}", relaxed.summary());

        assert!(
            relaxed.area_ratio < raw.area_ratio,
            "areas should even out: {} -> {}",
            raw.area_ratio,
            relaxed.area_ratio
        );
        assert!(
            relaxed.odd_degrees <= raw.odd_degrees,
            "degrees should tidy up: {} -> {}",
            raw.odd_degrees,
            relaxed.odd_degrees
        );
        assert!(
            relaxed.mean_compactness > raw.mean_compactness,
            "cells should get rounder: {} -> {}",
            raw.mean_compactness,
            relaxed.mean_compactness
        );
        assert!(
            relaxed.total_boundary < raw.total_boundary,
            "total border should shorten: {} -> {}",
            raw.total_boundary,
            relaxed.total_boundary
        );
    }

    /// The mean degree is fixed by Euler and is not a quality signal. Worth pinning so
    /// nobody later "improves" it.
    #[test]
    fn the_mean_degree_is_forced_by_euler_not_by_relaxation() {
        for region_count in [32usize, 60, 150] {
            let world = Tessellation::generate(Params {
                region_count,
                jitter: 0.2,
                relaxation: 12,
                seed: 1,
            });
            let quality = Quality::measure(&world.seeds, &world.neighbours);
            let total: usize = quality
                .degrees
                .iter()
                .enumerate()
                .map(|(degree, count)| degree * count)
                .sum();
            let mean_degree = total as f64 / region_count as f64;
            let forced = 6.0 - 12.0 / region_count as f64;
            assert!(
                (mean_degree - forced).abs() < 1e-9,
                "{region_count} regions: mean degree {mean_degree}, Euler says {forced}"
            );
        }
    }

    #[test]
    fn compactness_never_beats_a_regular_hexagon_by_much() {
        let world = Tessellation::generate(Params {
            region_count: 150,
            jitter: 0.2,
            relaxation: 20,
            seed: 1,
        });
        let quality = Quality::measure(&world.seeds, &world.neighbours);
        assert!(
            quality.mean_compactness < REGULAR_HEXAGON * 1.15,
            "compactness {} implausibly beats a regular hexagon",
            quality.mean_compactness
        );
    }

    #[test]
    fn an_unrelaxed_lattice_is_not_playable() {
        let world = Tessellation::generate(Params {
            region_count: 32,
            jitter: 0.0,
            relaxation: 0,
            seed: 1,
        });
        let quality = Quality::measure(&world.seeds, &world.neighbours);
        assert!(quality.odd_degrees > 0, "raw lattice has squares and heptagons");
        assert!(!quality.is_playable());
    }

    /// The three *geometric* measures are reached at every size. Relaxation is a
    /// geometric process, so this is what it can promise.
    #[test]
    fn balanced_generation_reaches_good_geometry_at_every_size() {
        println!("
  regions | passes | quality");
        for region_count in [32usize, 60, 150, 300] {
            let (world, passes) = Tessellation::generate_balanced(
                Params { region_count, jitter: 0.25, relaxation: 0, seed: 1 },
                80,
            );
            let quality = world.quality();
            println!("  {region_count:>7} | {passes:>6} | {}", quality.summary());

            assert!(
                quality.area_ratio < 1.35,
                "{region_count} regions: areas {}",
                quality.area_ratio
            );
            assert_eq!(
                quality.adjacent_pentagon_pairs, 0,
                "{region_count} regions: pentagons clustered, which reads as lumpy"
            );
            assert!(
                quality.worst_compactness > 0.6,
                "{region_count} regions: a sliver at {}",
                quality.worst_compactness
            );
            assert!(
                quality.mean_compactness > 0.85,
                "{region_count} regions: cells not round enough at {}",
                quality.mean_compactness
            );
        }
        println!();
    }

    /// The *topological* measure is a different matter, and this records the limit
    /// rather than pretending it is not there.
    ///
    /// Lloyd relaxation moves seeds toward the centroids of their cells. That evens out
    /// areas and rounds off shapes, both of which it does well at any size. What it does
    /// not do is change which cells are adjacent, so a square or a heptagon that forms
    /// early can survive any number of passes. Removing the last few needs a topological
    /// repair — flipping the offending edge — which is not built.
    ///
    /// Measured with jitter 0.25 and up to 80 passes:
    ///
    /// | regions | defects left | within the 2% budget |
    /// | --- | --- | --- |
    /// | 32 | 0 | yes |
    /// | 60 | 0 | yes |
    /// | 150 | 2 | yes (3 allowed) |
    /// | 300 | 22 | **no** (6 allowed) |
    #[test]
    fn relaxation_does_not_fix_topology_at_larger_sizes() {
        let small = Tessellation::generate_balanced(
            Params { region_count: 60, jitter: 0.25, relaxation: 0, seed: 1 },
            80,
        )
        .0
        .quality();
        assert_eq!(small.odd_degrees, 0, "small worlds do settle completely");

        let large = Tessellation::generate_balanced(
            Params { region_count: 300, jitter: 0.25, relaxation: 0, seed: 1 },
            80,
        )
        .0
        .quality();
        assert!(
            large.odd_degrees > Quality::allowed_odd_degrees(300),
            "if this now passes, a topological repair has been added and this test              should become an assertion that it works: {}",
            large.summary()
        );
        // Even so, the geometry is fine and most cells are hexagons.
        assert!(large.hex_fraction > 0.75);
        assert!(large.area_ratio < 1.35);
    }

    /// Thirty-two regions should come out looking like a soccer ball: twelve pentagons,
    /// twenty hexagons, and — the part that actually makes it read correctly — no two
    /// pentagons touching.
    ///
    /// This does not come from relaxation. It comes from starting at the truncated
    /// icosahedron instead of a golden spiral: relaxation will not move a pentagon that
    /// has landed beside another one, so the starting topology is the topology you get.
    /// Jitter then supplies the irregularity without disturbing which cells touch.
    #[test]
    fn thirty_two_regions_look_like_a_soccer_ball() {
        for seed in 1..=8u64 {
            let (world, _) = Tessellation::generate_balanced(
                Params { region_count: 32, jitter: 0.25, relaxation: 0, seed },
                40,
            );
            let quality = world.quality();
            assert_eq!(quality.degrees[5], 12, "seed {seed}: {}", quality.summary());
            assert_eq!(quality.degrees[6], 20, "seed {seed}: {}", quality.summary());
            assert_eq!(
                quality.adjacent_pentagon_pairs, 0,
                "seed {seed}: pentagons touching, so it will not read as a soccer ball"
            );
            assert!(world.is_soccer_ball(), "seed {seed}");
            // ...but irregular, not the exact solid: jitter has done its job.
            assert!(
                !world.verify_truncated_icosahedron().is_perfect(),
                "seed {seed} produced the exact solid; jitter should have varied it"
            );
        }
    }

    /// A cheap invariant that is identical for isomorphic graphs: for every region,
    /// its own degree followed by its neighbours' degrees, sorted, then all of those
    /// sorted together. Two worlds with different signatures are definitely different
    /// shapes; identical signatures are strong evidence they are the same shape up to
    /// relabelling and rotation.
    fn shape_signature(neighbours: &[Vec<u32>]) -> Vec<Vec<usize>> {
        let mut rows: Vec<Vec<usize>> = neighbours
            .iter()
            .map(|list| {
                let mut row = vec![list.len()];
                let mut around: Vec<usize> = list
                    .iter()
                    .map(|&other| neighbours[other as usize].len())
                    .collect();
                around.sort_unstable();
                row.extend(around);
                row
            })
            .collect();
        rows.sort();
        rows
    }

    /// Does "there is only one good answer" scale to any region count?
    ///
    /// It does not, and this measures where it stops. Relaxing from different random
    /// starts and counting how many genuinely different shapes come out:
    #[test]
    fn how_many_distinct_shapes_are_there_at_each_size() {
        use std::collections::HashSet;
        println!("
  regions | distinct shapes from 6 starts");
        let mut results = Vec::new();
        for region_count in [32usize, 42, 60, 92, 150] {
            let mut shapes = HashSet::new();
            for seed in 1..=6u64 {
                let (world, _) = Tessellation::generate_balanced(
                    Params { region_count, jitter: 0.35, relaxation: 0, seed },
                    40,
                );
                shapes.insert(shape_signature(&world.neighbours));
            }
            println!("  {region_count:>7} | {}", shapes.len());
            results.push((region_count, shapes.len()));
        }
        println!();

        // The 32 row is not evidence on its own: balanced generation *starts* from the
        // truncated icosahedron at that count, so of course every start agrees. What it
        // does show is that jitter of 0.35 perturbs the geometry without changing the
        // topology.
        assert_eq!(results[0].1, 1, "32 regions, seeded from the solid, stays one shape");
        assert!(
            results.last().unwrap().1 > 1,
            "large worlds should admit genuinely different arrangements"
        );
    }

    /// Left to itself, does relaxation *find* the unique best answer at 32?
    ///
    /// No. There is one optimal arrangement — the truncated icosahedron — but the
    /// energy landscape around it is full of local minima, and Lloyd relaxation from a
    /// golden spiral settles into whichever one it started nearest. So "there is a
    /// single best configuration" and "any process will converge on it" are different
    /// claims, and only the first is true.
    ///
    /// This is why balanced generation seeds 32 from the solid rather than searching
    /// for it.
    #[test]
    fn relaxation_does_not_find_the_optimum_on_its_own() {
        use std::collections::HashSet;
        let mut shapes = HashSet::new();
        let mut soccer_balls = 0;
        for seed in 1..=8u64 {
            let mut seeds = crate::lattice::fibonacci_lattice(32);
            let mut rng = crate::Rng::new(seed);
            crate::lattice::jitter(&mut seeds, 0.35, &mut rng);
            let samples = crate::lattice::fibonacci_lattice(
                crate::lattice::recommended_sample_count(32),
            );
            crate::lattice::relax(&mut seeds, 40, &samples);
            let neighbours = crate::adjacency::adjacency(&seeds);
            shapes.insert(shape_signature(&neighbours));

            let quality = Quality::measure(&seeds, &neighbours);
            if quality.adjacent_pentagon_pairs == 0 && quality.odd_degrees == 0 {
                soccer_balls += 1;
            }
        }
        println!(
            "
  32 regions from a golden spiral: {} distinct shapes from 8 starts, {soccer_balls} of them soccer balls
",
            shapes.len()
        );
        assert!(
            shapes.len() > 1,
            "relaxation from a spiral should land in several different local minima"
        );
    }

    #[test]
    fn degenerate_counts_do_not_panic() {
        for region_count in [1usize, 2, 3, 4] {
            let world = Tessellation::generate(Params {
                region_count,
                ..Default::default()
            });
            let quality = Quality::measure(&world.seeds, &world.neighbours);
            assert_eq!(quality.region_count, region_count);
        }
    }
}
