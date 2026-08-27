//! The world being looked at: a tessellation, a coloring, and a verification.
//!
//! Pure data. Nothing here knows about windows, input, or a graphics engine.

use graph_coloring::{Coloring, Method};

/// How long balanced generation may spend before settling for what it has. Large
/// worlds do not fully converge; see `sphere_tessellation::quality`.
const MAX_RELAXATION_PASSES: usize = 24;
use sphere_tessellation::{Direction, Params, Quality, Tessellation, Verification};

/// Where a world comes from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldSpec {
    pub params: Params,
    /// Use the exact truncated icosahedron instead of generating one. That is the
    /// icosahedral subdivision the design rejects for real worlds, kept because its
    /// answer is known exactly and makes a far better reference than any generated
    /// world.
    pub soccer: bool,
}

impl Default for WorldSpec {
    fn default() -> Self {
        Self {
            params: Params::default(),
            soccer: true,
        }
    }
}

/// A world, plus everything derived from it that the renderer needs.
#[derive(Clone, Debug)]
pub struct World {
    pub spec: WorldSpec,
    pub tessellation: Tessellation,
    pub coloring: Coloring,
    pub verification: Verification,
    /// How good this is as territory: equal areas, six neighbours, compact cells,
    /// isolated pentagons.
    pub quality: Quality,
    /// Where each region's seed points. Precomputed because seeds are not unit vectors
    /// and anything angular needs the direction rather than the seed.
    pub directions: Vec<Direction>,
}

impl World {
    pub fn build(spec: WorldSpec) -> Self {
        let tessellation = if spec.soccer {
            Tessellation::soccer_ball()
        } else {
            // Relax until the territory is good rather than for a fixed count: how much
            // is needed depends heavily on the region count. See `sphere_tessellation::quality`.
            Tessellation::generate_balanced(spec.params, MAX_RELAXATION_PASSES).0
        };
        let coloring = graph_coloring::color_graph(&tessellation.neighbours);
        debug_assert!(
            graph_coloring::find_conflict(&tessellation.neighbours, &coloring.colors).is_none()
        );
        let verification = tessellation.verify_truncated_icosahedron();
        let quality = tessellation.quality();
        let directions = tessellation.directions();
        Self {
            spec,
            tessellation,
            coloring,
            verification,
            quality,
            directions,
        }
    }

    /// How the coloring was arrived at, for the readout. A greedy fallback would mean
    /// something upstream is wrong, so it is shouted about.
    pub fn coloring_method(&self) -> String {
        match self.coloring.method {
            Method::Trivial => "trivial".to_string(),
            Method::Exact(count) => format!("exact {count}"),
            Method::GreedyFallback => "GREEDY FALLBACK".to_string(),
        }
    }

    pub fn degree_summary(&self) -> String {
        self.tessellation
            .degree_histogram()
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count > 0)
            .map(|(degree, count)| format!("{degree}:{count}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The five planet sizes have to be territory counts that actually exist. The list
    /// lives in `planet-model`, which has no geometry and cannot check itself; the
    /// geometry lives in `sphere-tessellation`, which has never heard of a planet size.
    /// This is the lowest crate that can see both, so the two are tied together here.
    #[test]
    fn every_planet_size_is_a_goldberg_polyhedron() {
        use planet_model::PlanetSize;
        use sphere_tessellation::goldberg;

        for size in PlanetSize::ALL {
            let count = size.territory_count();
            let arrangement = goldberg::arrangements_up_to(count)
                .into_iter()
                .find(|&(m, n)| goldberg::region_count(m, n) == count);
            let (m, n) = arrangement
                .unwrap_or_else(|| panic!("{} ({count}) is not a Goldberg count", size.name()));

            // And the world actually built for it has to be that solid, not a relaxed
            // approximation of it.
            let world = World::build(WorldSpec {
                params: Params {
                    region_count: count,
                    ..Default::default()
                },
                soccer: false,
            });
            assert_eq!(world.tessellation.region_count(), count, "{}", size.name());
            assert_eq!(
                world.quality.degrees[5],
                12,
                "{} is GP({m},{n}) and must have twelve pentagons: {}",
                size.name(),
                world.quality.summary()
            );
            if count > 12 {
                assert_eq!(
                    world.quality.adjacent_pentagon_pairs,
                    0,
                    "{}: pentagons must be isolated",
                    size.name()
                );
            }
        }
    }

    /// Twelve is the dodecahedron, where every territory is a pentagon and so every
    /// pentagon touches another. It is the one size where isolation is impossible, which
    /// is why the test above excludes it rather than the exclusion being an oversight.
    #[test]
    fn the_smallest_planet_is_all_pentagons() {
        use planet_model::PlanetSize;
        let world = World::build(WorldSpec {
            params: Params {
                region_count: PlanetSize::Tiny.territory_count(),
                ..Default::default()
            },
            soccer: false,
        });
        assert_eq!(world.quality.degrees[5], 12);
        assert_eq!(world.tessellation.region_count(), 12);
    }

    #[test]
    fn the_default_world_is_the_exact_solid() {
        let world = World::build(WorldSpec::default());
        assert_eq!(world.tessellation.region_count(), 32);
        assert!(world.verification.is_perfect());
        assert!(world.coloring.color_count <= 4);
    }

    /// With randomness off, generating 32 regions lands *exactly* on the solid — and
    /// that is correct, not a coincidence. For 32 points there is essentially one best
    /// arrangement, so "the best territory" and "the soccer ball" are the same object.
    /// Jitter is what moves deliberately away from it.
    #[test]
    fn without_jitter_generation_finds_the_solid_itself() {
        let world = World::build(WorldSpec {
            soccer: false,
            ..Default::default()
        });
        assert_eq!(world.spec.params.jitter, 0.0);
        assert!(
            world.verification.is_perfect(),
            "with nothing to perturb it, generation should reach the optimum: {}",
            world.quality.summary()
        );
    }

    #[test]
    fn jitter_moves_the_world_off_the_optimum_without_spoiling_it() {
        let world = World::build(WorldSpec {
            soccer: false,
            params: Params {
                jitter: 0.25,
                ..Default::default()
            },
        });
        assert!(
            !world.verification.is_perfect(),
            "jitter should have varied it"
        );
        // ...but it should still read as a soccer ball.
        assert_eq!(world.quality.adjacent_pentagon_pairs, 0);
        assert_eq!(world.quality.degrees[5], 12);
        assert_eq!(world.quality.degrees[6], 20);
    }

    #[test]
    fn every_world_is_validly_colored() {
        for region_count in [1, 2, 3, 20, 32, 60] {
            let world = World::build(WorldSpec {
                params: Params {
                    region_count,
                    ..Default::default()
                },
                soccer: false,
            });
            assert!(
                graph_coloring::find_conflict(
                    &world.tessellation.neighbours,
                    &world.coloring.colors
                )
                .is_none(),
                "{region_count} regions produced adjacent matching colors"
            );
        }
    }
}
