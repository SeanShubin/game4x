//! Divides the surface of a sphere into hex-dominant regions.
//!
//! The theory, including why a perfect hex grid on a sphere is impossible and why
//! icosahedral subdivision is rejected, is in `docs/theory/region-splitting.md`.
//!
//! This crate produces two things from a seed and a region count:
//!
//! - an **adjacency graph** of integer region identifiers, which is all the game logic
//!   ever sees, and
//! - the **seed positions**, which the renderer uses and the game logic does not.
//!
//! All floating point stays on this side of that boundary.

pub mod adjacency;
pub mod goldberg;
pub mod icosahedral;
pub mod lattice;
pub mod quality;
pub mod topology;
pub mod rng;
pub mod vec3;

pub use adjacency::{adjacency, degree_histogram, edge_count, is_connected, is_symmetric};
pub use icosahedral::{Verification, truncated_icosahedron_seeds};
pub use lattice::{fibonacci_lattice, mean_spacing, nearest_index, nearest_two};
pub use quality::Quality;
pub use rng::Rng;
pub use vec3::{Direction, Vec3};

/// Everything that determines a world. Same parameters, same world, every time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    /// How many regions to produce. Works from 1 upward.
    pub region_count: usize,
    /// Seed displacement as a fraction of mean spacing. 0.2 to 0.4 looks natural.
    pub jitter: f64,
    /// Lloyd relaxation passes. 2 to 4 keeps cells compact without going regular.
    pub relaxation: usize,
    /// The world seed.
    pub seed: u64,
}

impl Default for Params {
    /// **All randomness off, for now.** Jitter is zero, so the seed does nothing and
    /// generation is completely deterministic; relaxation is zero, so nothing is
    /// smoothed either. This is a deliberate starting point, not a tuned setting.
    ///
    /// Thirty-two is the truncated icosahedron's face count. Note that these
    /// parameters do **not** produce one: the Fibonacci lattice is a golden spiral,
    /// not the icosahedral arrangement, and unrelaxed at 32 points it gives
    /// `4:4 5:8 6:16 7:4`. A perfect truncated icosahedron comes from
    /// [`Tessellation::soccer_ball`], which is constructed rather than generated.
    ///
    /// For reference, the tuned settings that made *generated* 32-region worlds come
    /// out cleanly were jitter 0.20 with 16 relaxation passes.
    fn default() -> Self {
        Self {
            region_count: 32,
            jitter: 0.0,
            relaxation: 0,
            seed: 1,
        }
    }
}

/// A generated world: where the regions are, and which of them touch.
#[derive(Clone, Debug)]
pub struct Tessellation {
    pub params: Params,
    /// One unit vector per region. Rendering only.
    pub seeds: Vec<Vec3>,
    /// Sorted neighbour lists. This is the part the game logic sees.
    pub neighbours: Vec<Vec<u32>>,
}

impl Tessellation {
    pub fn generate(params: Params) -> Self {
        let mut seeds = lattice::fibonacci_lattice(params.region_count);

        let mut rng = Rng::new(params.seed);
        lattice::jitter(&mut seeds, params.jitter, &mut rng);

        if params.relaxation > 0 && params.region_count > 1 {
            let samples =
                lattice::fibonacci_lattice(lattice::recommended_sample_count(params.region_count));
            lattice::relax(&mut seeds, params.relaxation, &samples);
        }

        let neighbours = adjacency::adjacency(&seeds);
        Self {
            params,
            seeds,
            neighbours,
        }
    }

    /// A soccer ball: the truncated icosahedron, 12 pentagons and 20 hexagons, with
    /// no two pentagons touching.
    ///
    /// This is the icosahedral subdivision the design rejects for real worlds, kept as
    /// a reference. See [`icosahedral`].
    pub fn soccer_ball() -> Self {
        let seeds = icosahedral::truncated_icosahedron_seeds();
        let neighbours = adjacency::adjacency(&seeds);
        Self {
            params: Params {
                region_count: seeds.len(),
                jitter: 0.0,
                relaxation: 0,
                seed: 0,
            },
            seeds,
            neighbours,
        }
    }

    /// Measures this tessellation against the truncated icosahedron, reporting every
    /// quantity rather than just a verdict. See [`icosahedral::Verification`].
    pub fn verify_truncated_icosahedron(&self) -> icosahedral::Verification {
        icosahedral::verify(&self.seeds, &self.neighbours)
    }

    /// True when this is a soccer ball: every cell a pentagon or hexagon, twelve and
    /// twenty of them, and no two pentagons sharing a border.
    pub fn is_soccer_ball(&self) -> bool {
        if self.region_count() != 32 {
            return false;
        }
        let histogram = self.degree_histogram();
        if histogram.get(5).copied().unwrap_or(0) != 12
            || histogram.get(6).copied().unwrap_or(0) != 20
            || histogram.iter().sum::<usize>() != 32
        {
            return false;
        }
        self.neighbours.iter().all(|list| {
            list.len() != 5
                || list.iter().all(|&other| self.neighbours[other as usize].len() != 5)
        })
    }

    pub fn region_count(&self) -> usize {
        self.seeds.len()
    }

    pub fn edge_count(&self) -> usize {
        adjacency::edge_count(&self.neighbours)
    }

    pub fn degree_histogram(&self) -> Vec<usize> {
        adjacency::degree_histogram(&self.neighbours)
    }

    /// How good this is as territory: equal areas, six neighbours, compact cells,
    /// short borders. See [`quality`].
    pub fn quality(&self) -> Quality {
        Quality::measure(&self.seeds, &self.neighbours)
    }

    /// Generates a world and keeps relaxing until it is good territory, or until
    /// `max_passes` is spent.
    ///
    /// Relaxation improves all four measures at once, so there is one dial rather than
    /// four — but how far it needs turning depends strongly on the region count, and a
    /// fixed number is either wasteful or not enough. This asks the tessellation
    /// instead of guessing.
    ///
    /// Stops as soon as the result is playable, because further relaxation buys
    /// diminishing evenness at the cost of the irregularity that makes a world look
    /// natural rather than computed.
    pub fn generate_balanced(params: Params, max_passes: usize) -> (Self, usize) {
        // Where to start matters more than how long to relax. Relaxation is geometric:
        // it evens out areas and rounds off cells, but it will not move a pentagon that
        // has ended up next to another pentagon. A golden spiral is a poor topological
        // starting point at small counts, and 32 has a known best answer — so start
        // from it, and let jitter supply the irregularity.
        // A canonical arrangement is already the answer, and relaxation would only
        // damage it: Lloyd moves each seed to its cell's centroid and renormalizes,
        // which throws away the plane distances that make the edges equal. So when one
        // exists and there is nothing to perturb, hand it back untouched.
        if params.jitter <= 0.0 {
            if let Some(seeds) = icosahedral::canonical_seeds(params.region_count) {
                let neighbours = adjacency::adjacency(&seeds);
                return (
                    Self {
                        params: Params { relaxation: 0, ..params },
                        seeds,
                        neighbours,
                    },
                    0,
                );
            }
        }

        let mut seeds = icosahedral::canonical_seeds(params.region_count)
            .unwrap_or_else(|| lattice::fibonacci_lattice(params.region_count));
        let mut rng = Rng::new(params.seed);
        lattice::jitter(&mut seeds, params.jitter, &mut rng);

        if params.region_count < 4 {
            let neighbours = adjacency::adjacency(&seeds);
            return (Self { params, seeds, neighbours }, 0);
        }

        let samples =
            lattice::fibonacci_lattice(lattice::recommended_sample_count(params.region_count));

        // Relax in small batches and re-measure, rather than measuring every pass:
        // the measurement costs more than the pass does.
        const BATCH: usize = 4;
        let mut spent = 0;
        loop {
            let neighbours = adjacency::adjacency(&seeds);
            if Quality::measure(&seeds, &neighbours).is_playable() || spent >= max_passes {
                return (
                    Self {
                        params: Params { relaxation: spent, ..params },
                        seeds,
                        neighbours,
                    },
                    spent,
                );
            }
            lattice::relax(&mut seeds, BATCH, &samples);
            spent += BATCH;
        }
    }

    /// The direction a region's seed points in.
    ///
    /// Seeds themselves are not unit vectors — they carry a plane distance — so anything
    /// angular must go through this rather than using the seed directly. See
    /// [`Direction`].
    pub fn direction(&self, region: usize) -> Direction {
        Direction::of(self.seeds[region])
    }

    pub fn directions(&self) -> Vec<Direction> {
        self.seeds.iter().map(|&seed| Direction::of(seed)).collect()
    }

    /// Which region contains the given direction.
    pub fn region_at(&self, direction: Vec3) -> usize {
        lattice::nearest_index(&self.seeds, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic() {
        let params = Params {
            region_count: 30,
            jitter: 0.3,
            relaxation: 2,
            seed: 12345,
        };
        let first = Tessellation::generate(params);
        let second = Tessellation::generate(params);
        assert_eq!(first.seeds, second.seeds);
        assert_eq!(first.neighbours, second.neighbours);
    }

    /// The seed drives jitter and nothing else, so with randomness switched off it is
    /// inert by construction. Worth pinning both halves: silence now, and effect the
    /// moment jitter comes back.
    #[test]
    fn the_seed_matters_exactly_when_jitter_does() {
        let base = Params {
            region_count: 30,
            ..Params::default()
        };
        assert_eq!(base.jitter, 0.0, "defaults are meant to have no randomness");

        let quiet = Tessellation::generate(base);
        let also_quiet = Tessellation::generate(Params { seed: 999, ..base });
        assert_eq!(
            quiet.seeds, also_quiet.seeds,
            "with jitter at zero the seed must have no effect"
        );

        let jittered = Params { jitter: 0.3, ..base };
        let first = Tessellation::generate(jittered);
        let second = Tessellation::generate(Params { seed: 999, ..jittered });
        assert_ne!(
            first.seeds, second.seeds,
            "with jitter on the seed must have an effect"
        );
    }

    /// The small counts the theory document calls out as degenerate for a
    /// polygon-based pipeline. Nothing here may panic.
    #[test]
    fn degenerate_region_counts_are_handled() {
        for region_count in [1, 2, 3, 4, 5] {
            let tessellation = Tessellation::generate(Params {
                region_count,
                ..Params::default()
            });
            assert_eq!(tessellation.region_count(), region_count);
            assert!(is_symmetric(&tessellation.neighbours));
            let probe = Vec3::from_lon_lat(0.7, 0.2);
            assert!(tessellation.region_at(probe) < region_count);
        }
    }

    #[test]
    fn the_default_world_forms_a_sane_graph() {
        let tessellation = Tessellation::generate(Params::default());
        let count = Params::default().region_count;
        assert_eq!(tessellation.region_count(), count);
        assert!(is_connected(&tessellation.neighbours));
        assert!(is_symmetric(&tessellation.neighbours));
        // Euler's formula for a triangulation of the sphere.
        assert_eq!(tessellation.edge_count(), 3 * count - 6);
    }

    #[test]
    fn every_direction_lands_in_exactly_one_region() {
        let tessellation = Tessellation::generate(Params::default());
        let mut seen = vec![false; tessellation.region_count()];
        for step in 0..2_000 {
            let probe = lattice::fibonacci_lattice(2_000)[step];
            seen[tessellation.region_at(probe)] = true;
        }
        assert!(seen.iter().all(|&hit| hit), "some region is unreachable");
    }
}
