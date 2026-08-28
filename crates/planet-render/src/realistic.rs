//! The realistic drawing: the world as ground rather than as a survey of it.
//!
//! `spec/planet.md` asks for two drawings sharing the camera and nothing else. The
//! practical one is [`crate::mesh`] - flat colours, inset panels, a groove at every
//! boundary, and every one of those is there to make adjacency legible. This is the other
//! one, and almost every decision is the opposite decision.
//!
//! | | Practical | Realistic |
//! | --- | --- | --- |
//! | Colour | One per region, flat | Sampled from the terrain, per vertex |
//! | Panels | Inset, so a groove shows the boundary | Meeting exactly, so no boundary shows |
//! | Normal | The region's centre, so the panel shades flat | The terrain's own slope |
//! | Detail | One triangle fan per region | Each fan triangle subdivided |
//!
//! # Why the normals come from the field and not from the geometry
//!
//! `spec/planet.md`: *nothing in the terrain reveals how the sphere was divided.*
//!
//! Regions do not share vertices - [`crate::mesh`] explains why, and this builder inherits
//! it. So a normal averaged from the triangles around a vertex would be averaged over a
//! *different* set of triangles on each side of a boundary, and the two answers would
//! differ by a hair. A hair is enough: a lighting discontinuity along every edge would
//! draw the tessellation in shadow, which is exactly the thing that must not be visible.
//!
//! Taking the normal from the elevation field instead makes it a function of direction
//! alone. Two vertices at the same point on the sphere get the same normal whichever
//! region owns them, so the seam cannot be lit differently from its surroundings because
//! there is nothing about it that differs. The same argument covers the colour, which is
//! also a function of direction, and the displacement, which is too.
//!
//! `docs/notes/depth-on-a-sphere.md` ranks this second among the things that make a sphere
//! read as solid, after the light direction, and calls it *the cheapest large win*.

use sphere_tessellation::{Solid, Vec3};

use crate::mesh::{PlanetMesh, RegionSpan};

/// The roughly constant number of sub-triangles a whole planet is cut into.
///
/// The subdivision is what samples the field, so it sets how finely the terrain is
/// resolved - and terrain resolution is a fact about *angles on the sphere*, not about
/// territories. A twelve-territory planet has faces over a radian across, and cutting each
/// fan triangle into six meant sampling the field about every seven degrees. The result was
/// a planet whose coastlines were visibly pentagonal, because the only structure fine
/// enough to see was the subdivision grid, and that grid is the tessellation.
///
/// So the count is chosen per planet from how many territories there are, to keep the
/// angular step and the total cost roughly fixed across every size. `spec/planet.md`:
/// *nothing in the terrain reveals how the sphere was divided.*
const SUB_TRIANGLES: f64 = 28_000.0;

/// Segments per fan-triangle edge, never fewer than this or more than that.
const FEWEST: usize = 8;
const MOST: usize = 40;

/// How finely to cut each fan triangle, for a planet of this many territories.
///
/// **One number for the whole planet, deliberately.** Two territories sharing an edge must
/// cut it into the same number of pieces, or their vertices land on different points and
/// every quantity taken from the field disagrees along the seam - which is the boundary
/// drawn in light, the exact thing this drawing must not have. Deriving the count from each
/// cell's own size would do that, because a pentagon and a hexagon are not the same size.
pub fn segments_for(regions: usize) -> usize {
    // Each region contributes about six fan triangles, each of which becomes `n^2`.
    let per_region = SUB_TRIANGLES / (regions.max(1) as f64 * 6.0);
    (per_region.sqrt().round() as usize).clamp(FEWEST, MOST)
}

/// How far the ground rises and falls, as a fraction of the planet's radius.
///
/// Real relief is invisible at this scale - Everest is a thousandth of Earth's radius - so
/// this is exaggerated, and deliberately. `docs/notes/depth-on-a-sphere.md`: a planet's
/// silhouette is a circle, and mountains do not break the horizon seen from space. The
/// point of displacing at all is the shading it produces, not the outline.
const RELIEF: f64 = 0.045;

/// How far apart the two samples of a central difference are, in radians.
///
/// Small enough to measure the slope where the vertex is, large enough that the difference
/// is not lost in the field's own precision.
const SLOPE_STEP: f64 = 0.004;

/// How much the measured slope tilts the normal away from straight up.
///
/// Separate from [`RELIEF`] on purpose: the displacement is what the silhouette and the
/// depth buffer see, and this is what the light sees. Tilting harder than the ground
/// actually rises is a deliberate exaggeration, and it is the one that makes a
/// two-percent slope read at all.
const SLOPE_TO_NORMAL: f64 = 2.6;

/// The colour of ground with these properties, in linear RGB.
///
/// Biome first, because that is the fact the model holds and `spec/planet.md` says the
/// drawing must show *the biome the model has*. Within a biome the colour still moves with
/// the field, which is what makes terrain visibly vary inside one territory rather than
/// filling it with a single flat wash - the thing that would make this the practical
/// drawing with different colours.
fn ground(sample: &planet_terrain::Sample) -> [f32; 4] {
    use game_model::Biome;

    // Two tones per biome, mixed by something that varies within it. The mixer is chosen
    // per biome to be a quantity that means something there: depth for water, height for
    // rock, moisture for anything growing.
    let (dark, light, mix) = match planet_terrain::biome_of(sample) {
        Biome::Ocean => (
            [0.012, 0.055, 0.150],
            [0.035, 0.170, 0.330],
            (sample.elevation / planet_terrain::sea_level()).clamp(0.0, 1.0),
        ),
        Biome::Ice => (
            [0.720, 0.780, 0.850],
            [0.940, 0.965, 0.990],
            sample.elevation.clamp(0.0, 1.0),
        ),
        Biome::Desert => (
            [0.560, 0.450, 0.250],
            [0.820, 0.720, 0.470],
            sample.moisture.clamp(0.0, 1.0),
        ),
        Biome::Grassland => (
            [0.220, 0.330, 0.140],
            [0.450, 0.540, 0.240],
            sample.moisture.clamp(0.0, 1.0),
        ),
        Biome::Jungle => (
            [0.055, 0.180, 0.070],
            [0.130, 0.330, 0.120],
            sample.drainage.clamp(0.0, 1.0),
        ),
        Biome::Mountain => (
            [0.300, 0.290, 0.280],
            [0.640, 0.630, 0.620],
            ((sample.elevation - 1.0) * 2.0).clamp(0.0, 1.0),
        ),
    };
    let blend = |at: usize| (dark[at] + (light[at] - dark[at]) * mix) as f32;
    [blend(0), blend(1), blend(2), 1.0]
}

/// How far off the sphere the ground sits at this point.
fn height(at: Vec3, seed: u64) -> f64 {
    let sample = planet_terrain::sample(at, seed);
    // Water is flat. A sea floor modelled as terrain would show through as ripples in the
    // ocean's surface, and an ocean's surface is the one part of a planet that really is
    // a sphere.
    let sea = planet_terrain::sea_level();
    if sample.elevation < sea {
        return 0.0;
    }
    (sample.elevation - sea) * RELIEF
}

/// The outward normal of the ground at this point, from the field rather than the mesh.
///
/// A central difference along two perpendicular tangents. `asset-creator` gives the same
/// formula, by way of `docs/notes/planet-appearance.md`.
fn slope_normal(at: Vec3, seed: u64) -> [f32; 3] {
    let up = at.normalized();
    let east = up.any_perpendicular().normalized();
    let north = up.cross(east).normalized();

    let along = |tangent: Vec3| {
        let ahead = up.add(tangent.scaled(SLOPE_STEP)).normalized();
        let behind = up.sub(tangent.scaled(SLOPE_STEP)).normalized();
        (height(ahead, seed) - height(behind, seed)) / (2.0 * SLOPE_STEP)
    };

    // Tilt away from the uphill direction: a surface rising to the east faces west.
    let tilted = up
        .sub(east.scaled(along(east) * SLOPE_TO_NORMAL))
        .sub(north.scaled(along(north) * SLOPE_TO_NORMAL))
        .normalized();
    [tilted.x as f32, tilted.y as f32, tilted.z as f32]
}

/// Where a vertex sits, once the ground has been raised.
fn surface(at: Vec3, seed: u64) -> [f32; 3] {
    let up = at.normalized();
    let out = up.scaled(1.0 + height(up, seed));
    [out.x as f32, out.y as f32, out.z as f32]
}

/// Builds the realistic drawing of a solid.
///
/// The regions are still walked one at a time, and each still owns a contiguous block of
/// vertices, so [`RegionSpan`] means the same thing here as in the practical drawing and
/// anything that reads spans works on either. What differs is that nothing about a
/// region's identity reaches the surface: no per-region colour, no inset, no shared
/// normal. A region is only the piece of ground this loop happens to be covering.
pub fn build(solid: &Solid, seed: u64) -> PlanetMesh {
    let mut mesh = PlanetMesh::default();
    let segments = segments_for(solid.cells.len());

    for cell in &solid.cells {
        let corners: Vec<Vec3> = cell
            .iter()
            .map(|&corner| solid.corners[corner as usize].vector())
            .collect();
        let centre = corners
            .iter()
            .fold(Vec3::ZERO, |total, &corner| total.add(corner))
            .normalized();

        let first_vertex = mesh.positions.len() as u32;
        for step in 0..corners.len() {
            let next = (step + 1) % corners.len();
            fan_triangle(
                &mut mesh,
                centre,
                corners[step],
                corners[next],
                seed,
                segments,
            );
        }
        mesh.regions.push(RegionSpan {
            first_vertex,
            vertex_count: mesh.positions.len() as u32 - first_vertex,
        });
    }

    mesh
}

/// Subdivides one triangle of a region's fan and appends it.
///
/// A barycentric grid: row `r` has `r + 1` points, and the point at `(r, c)` is the corner
/// weights `(n - r, r - c, c)` normalized onto the sphere. Both triangles sharing an edge
/// of the tessellation walk that edge with the same weights, so their vertices land on the
/// same directions and every quantity taken from the field agrees along it.
fn fan_triangle(mesh: &mut PlanetMesh, hub: Vec3, from: Vec3, to: Vec3, seed: u64, n: usize) {
    let base = mesh.positions.len() as u32;

    for row in 0..=n {
        for column in 0..=row {
            let toward_hub = (n - row) as f64;
            let toward_from = (row - column) as f64;
            let toward_to = column as f64;
            let at = hub
                .scaled(toward_hub)
                .add(from.scaled(toward_from))
                .add(to.scaled(toward_to))
                .normalized();

            mesh.positions.push(surface(at, seed));
            mesh.normals.push(slope_normal(at, seed));
            mesh.colors.push(ground(&planet_terrain::sample(at, seed)));
        }
    }

    // Row `r` starts at index `r(r + 1)/2`. Each row pairs with the one below it: one
    // upward triangle per column, and one downward triangle between them.
    let start = |row: usize| base + (row * (row + 1) / 2) as u32;
    for row in 0..n {
        for column in 0..=row {
            let top = start(row) + column as u32;
            let left = start(row + 1) + column as u32;
            let right = left + 1;
            mesh.indices.push(top);
            mesh.indices.push(left);
            mesh.indices.push(right);
            if column < row {
                let across = top + 1;
                mesh.indices.push(top);
                mesh.indices.push(right);
                mesh.indices.push(across);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sphere_tessellation::icosahedral::canonical_seeds;

    const SEED: u64 = 20260828;

    fn solid_of(count: usize) -> Solid {
        let seeds = canonical_seeds(count).unwrap();
        let neighbours = sphere_tessellation::adjacency(&seeds);
        sphere_tessellation::solid(&seeds, &neighbours)
    }

    /// Every region contributes geometry, and the spans cover the vertices exactly once -
    /// the same contract the practical drawing has, so anything reading spans works on
    /// either drawing.
    #[test]
    fn the_regions_span_the_whole_mesh() {
        let mesh = build(&solid_of(12), SEED);
        assert_eq!(mesh.regions.len(), 12);
        let mut at = 0;
        for span in &mesh.regions {
            assert_eq!(span.first_vertex as usize, at);
            assert!(span.vertex_count > 0);
            at += span.vertex_count as usize;
        }
        assert_eq!(at, mesh.vertex_count());
        assert!(
            mesh.indices
                .iter()
                .all(|at| (*at as usize) < mesh.vertex_count())
        );
    }

    /// `spec/planet.md`: *nothing in the terrain reveals how the sphere was divided.*
    ///
    /// The strongest form of that available to a test: everything a vertex carries is a
    /// function of where it is and of nothing else. Two vertices at the same point,
    /// belonging to different regions, must agree exactly - not nearly - because a
    /// difference in colour or normal along a shared edge is a drawn boundary.
    #[test]
    fn two_regions_meeting_at_a_point_agree_about_it() {
        let mesh = build(&solid_of(32), SEED);
        // Group vertices by position. Any position reached by more than one region is on a
        // boundary, which is exactly where a seam would show.
        let mut seen: std::collections::HashMap<[u32; 3], (usize, [f32; 4], [f32; 3])> =
            std::collections::HashMap::new();
        let mut shared = 0;
        for at in 0..mesh.vertex_count() {
            let key = mesh.positions[at].map(|value| value.to_bits());
            let here = (at, mesh.colors[at], mesh.normals[at]);
            if let Some((_, colour, normal)) = seen.get(&key) {
                shared += 1;
                assert_eq!(*colour, here.1, "two regions disagree about a shared point");
                assert_eq!(*normal, here.2, "a shared point is lit two ways");
            } else {
                seen.insert(key, here);
            }
        }
        assert!(
            shared > 100,
            "only {shared} shared points; the test is not reaching any boundaries"
        );
    }

    /// `spec/planet.md`: the terrain *varies within a single territory*.
    ///
    /// A region drawn in one flat colour would be the practical drawing wearing a
    /// different palette.
    #[test]
    fn the_ground_varies_inside_a_region() {
        let mesh = build(&solid_of(12), SEED);
        let mut varied = 0;
        for span in &mesh.regions {
            let first = span.first_vertex as usize;
            let colours = &mesh.colors[first..first + span.vertex_count as usize];
            if colours.iter().any(|colour| colour != &colours[0]) {
                varied += 1;
            }
        }
        assert!(
            varied >= 10,
            "only {varied} of 12 territories have any variation in them"
        );
    }

    /// The panels are not inset, so there is no groove to read a boundary from - the one
    /// thing the practical drawing does deliberately and this one must not.
    #[test]
    fn neighbouring_ground_meets_rather_than_leaving_a_groove() {
        let mesh = build(&solid_of(12), SEED);
        // Every corner of the solid should be reached by all three regions around it. If
        // the panels were inset, no two would ever land on the same point.
        let mut positions: std::collections::HashSet<[u32; 3]> = std::collections::HashSet::new();
        let mut repeats = 0;
        for at in 0..mesh.vertex_count() {
            if !positions.insert(mesh.positions[at].map(|value| value.to_bits())) {
                repeats += 1;
            }
        }
        assert!(
            repeats > 0,
            "no two regions share a point, so they do not meet"
        );
    }

    /// Every planet is cut about as finely as every other, in angle and in cost.
    ///
    /// A fixed count per triangle would sample a twelve-territory planet eight times more
    /// coarsely than a ninety-two-territory one, and it was that coarseness that made the
    /// terrain pentagonal.
    #[test]
    fn a_small_planet_is_cut_more_finely_than_a_large_one() {
        let counts: Vec<usize> = [12, 32, 42, 72, 92].into_iter().map(segments_for).collect();
        assert!(
            counts.windows(2).all(|pair| pair[0] >= pair[1]),
            "fewer territories should mean more segments each: {counts:?}"
        );
        // And the totals stay in the same order of magnitude, so no size is ruinous.
        for count in [12usize, 32, 42, 72, 92] {
            let triangles = count * 6 * segments_for(count).pow(2);
            assert!(
                (8_000..90_000).contains(&triangles),
                "{count} territories would be {triangles} triangles"
            );
        }
    }

    /// Water is flat, and land is not. A sea floor drawn as terrain would ripple the one
    /// part of a planet that really is a sphere.
    #[test]
    fn the_sea_is_level_and_the_land_is_not() {
        let sea = planet_terrain::sea_level();
        let mut heights = Vec::new();
        for step in 0..400 {
            let z = 1.0 - 2.0 * (step as f64 + 0.5) / 400.0;
            let radius = (1.0 - z * z).max(0.0).sqrt();
            let angle = step as f64 * 2.399_963_229_728_653;
            let at = Vec3::new(radius * angle.cos(), radius * angle.sin(), z);
            let raised = height(at, SEED);
            if planet_terrain::sample(at, SEED).elevation < sea {
                assert_eq!(raised, 0.0, "the sea floor was raised");
            }
            heights.push(raised);
        }
        assert!(
            heights.iter().any(|raised| *raised > 0.0),
            "nothing on the planet is above sea level"
        );
    }
}
