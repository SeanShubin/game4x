//! The world as triangles: a view model for a graphics engine to upload.
//!
//! This is the same layer as [`crate::raster`] and the opposite technique. The raster
//! answers "what colour is this pixel" by asking which seed is nearest; this hands over
//! the polygons and lets the hardware answer that question. Both are views of one model,
//! and neither is allowed to know which engine consumes it - there are no engine types
//! in this file, only numbers.
//!
//! # Why each region gets its own vertices
//!
//! Regions do not share vertices even where they share a corner. Two reasons, and both
//! are about the region being the unit that matters:
//!
//! - A shared vertex would have to blend the colours of the three regions meeting there,
//!   and a region is one flat colour with a hard edge, not a gradient.
//! - Owning a contiguous block of vertices means a region can be recoloured later by
//!   overwriting that block - see [`RegionSpan`] - without rebuilding the mesh.
//!
//! # Insetting
//!
//! Each panel is shrunk slightly toward its own centre, so neighbouring panels no longer
//! touch and the gap between them reads as a groove. That is what makes the solid look
//! like a ball made of panels rather than a faceted sphere. The gap shows whatever is
//! drawn behind it, so the engine is expected to put a darker sphere just underneath.

use graph_coloring::Coloring;
use sphere_tessellation::{Solid, Vec3};

use crate::palette::REGION_COLORS;

/// How far each corner is drawn toward its region's centre, as a fraction of the way.
///
/// Enough to read as a seam at a glance, small enough that a region still looks its own
/// size. Pentagons and hexagons take the same fraction, so the seams stay even.
pub const INSET: f32 = 0.028;

/// Which block of vertices belongs to a region.
///
/// Recolouring is then a write over `positions[first..first + count]`'s colours rather
/// than a rebuild, which is what selection and ownership changes will want.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionSpan {
    pub first_vertex: u32,
    pub vertex_count: u32,
}

/// A triangle mesh of the whole world, ready to upload.
#[derive(Clone, Debug, Default)]
pub struct PlanetMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    /// Linear RGBA, **not** sRGB. Graphics pipelines take vertex colours as linear, and
    /// handing them sRGB is the washed-out-picture bug that already happened once in the
    /// fragment shader.
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    /// Indexed by region.
    pub regions: Vec<RegionSpan>,
}

impl PlanetMesh {
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    /// How far below the sphere the surface actually reaches, as a radius.
    ///
    /// A panel is flat, so it dips inside the sphere its corners sit on: a triangle with
    /// every vertex at radius one passes closest to the centre at its own plane. How far
    /// depends entirely on how wide the territory is, which is to say on how many there
    /// are - a twelve-territory planet sags to about 0.93 and a ninety-two-territory one
    /// barely at all.
    ///
    /// Anything drawn beneath the panels has to sit below this or it comes through them.
    /// Returning it means the caller can place that thing from the geometry rather than
    /// from a guess that happens to hold at one size.
    pub fn deepest(&self) -> f32 {
        let mut deepest = 1.0f32;
        for triangle in self.indices.chunks(3) {
            let point = |at: u32| {
                let p = self.positions[at as usize];
                Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)
            };
            let (a, b, c) = (point(triangle[0]), point(triangle[1]), point(triangle[2]));
            let normal = b.sub(a).cross(c.sub(a));
            if normal.length() < 1e-12 {
                continue;
            }
            deepest = deepest.min(normal.normalized().dot(a).abs() as f32);
        }
        deepest
    }

    /// Overwrites one region's colour in place. Used for selection and, later, ownership.
    pub fn recolor(&mut self, region: usize, color: [f32; 4]) {
        let span = self.regions[region];
        let first = span.first_vertex as usize;
        for slot in &mut self.colors[first..first + span.vertex_count as usize] {
            *slot = color;
        }
    }
}

/// Builds the mesh for a solid, colouring each region by the graph colouring.
///
/// A region becomes a fan of triangles from its centre out to its corners, so a pentagon
/// costs five triangles and a hexagon six. Every vertex of a region carries that region's
/// centre direction as its normal, which shades the panel flat - the panels are so nearly
/// planar that anything else would only reintroduce the sphere they came from.
pub fn build(solid: &Solid, coloring: &Coloring) -> PlanetMesh {
    let mut mesh = PlanetMesh::default();

    for (region, cell) in solid.cells.iter().enumerate() {
        let corners: Vec<Vec3> = cell
            .iter()
            .map(|&corner| solid.corners[corner as usize].vector())
            .collect();
        let centre = corners
            .iter()
            .fold(Vec3::ZERO, |total, &corner| total.add(corner))
            .normalized();

        let color =
            linear_rgba(REGION_COLORS[coloring.colors[region] as usize % REGION_COLORS.len()]);
        let normal = as_f32(centre);
        let first_vertex = mesh.positions.len() as u32;

        // The fan's hub, then its rim.
        mesh.positions.push(as_f32(centre));
        for &corner in &corners {
            mesh.positions.push(as_f32(inset_toward(corner, centre)));
        }
        let vertex_count = mesh.positions.len() as u32 - first_vertex;
        for _ in 0..vertex_count {
            mesh.normals.push(normal);
            mesh.colors.push(color);
        }

        // Corners are already counter-clockwise seen from outside, so a fan taken in
        // that order faces outward and needs no winding fix.
        for step in 0..corners.len() as u32 {
            let next = (step + 1) % corners.len() as u32;
            mesh.indices.push(first_vertex);
            mesh.indices.push(first_vertex + 1 + step);
            mesh.indices.push(first_vertex + 1 + next);
        }

        mesh.regions.push(RegionSpan {
            first_vertex,
            vertex_count,
        });
    }

    mesh
}

/// Moves a corner a little toward its region's centre, staying on the sphere.
fn inset_toward(corner: Vec3, centre: Vec3) -> Vec3 {
    corner
        .scaled(1.0 - INSET as f64)
        .add(centre.scaled(INSET as f64))
        .normalized()
}

fn as_f32(vector: Vec3) -> [f32; 3] {
    [vector.x as f32, vector.y as f32, vector.z as f32]
}

/// Splits `0xRRGGBB` into linear components.
///
/// The palette is written the way a colour picker shows it, which is sRGB, and the
/// transfer curve has to come off before the numbers mean anything to a renderer.
fn linear_rgba(packed: u32) -> [f32; 4] {
    let channel = |shift: u32| {
        let encoded = ((packed >> shift) & 0xFF) as f32 / 255.0;
        if encoded <= 0.04045 {
            encoded / 12.92
        } else {
            ((encoded + 0.055) / 1.055).powf(2.4)
        }
    };
    [channel(16), channel(8), channel(0), 1.0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use sphere_tessellation::Tessellation;

    fn soccer_ball() -> (PlanetMesh, sphere_tessellation::Solid) {
        let world = Tessellation::soccer_ball();
        let solid = sphere_tessellation::solid(&world.seeds, &world.neighbours);
        let coloring = graph_coloring::color_graph(&world.neighbours);
        (build(&solid, &coloring), solid)
    }

    /// A pentagon is five triangles and a hexagon six, so the soccer ball is
    /// `12 * 5 + 20 * 6`.
    #[test]
    fn the_soccer_ball_costs_one_triangle_per_side() {
        let (mesh, _) = soccer_ball();
        assert_eq!(mesh.triangle_count(), 12 * 5 + 20 * 6);
        assert_eq!(mesh.regions.len(), 32);
    }

    /// Every index must point at a real vertex. An off-by-one here draws garbage
    /// geometry rather than failing, so it is worth asserting.
    #[test]
    fn every_index_is_in_range() {
        let (mesh, _) = soccer_ball();
        assert!(
            mesh.indices
                .iter()
                .all(|&i| (i as usize) < mesh.vertex_count())
        );
        assert_eq!(mesh.normals.len(), mesh.vertex_count());
        assert_eq!(mesh.colors.len(), mesh.vertex_count());
    }

    /// The spans must tile the vertex buffer exactly: no gaps, no overlaps.
    #[test]
    fn the_region_spans_cover_every_vertex_once() {
        let (mesh, _) = soccer_ball();
        let mut next = 0;
        for span in &mesh.regions {
            assert_eq!(span.first_vertex, next, "spans must be contiguous");
            next += span.vertex_count;
        }
        assert_eq!(next as usize, mesh.vertex_count());
    }

    /// Every triangle must face outward, or half the world would be invisible from
    /// outside it. The test is that the outward normal agrees with the winding.
    #[test]
    fn every_triangle_faces_outward() {
        let (mesh, _) = soccer_ball();
        for triangle in mesh.indices.chunks(3) {
            let point = |at: u32| {
                let p = mesh.positions[at as usize];
                Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)
            };
            let (a, b, c) = (point(triangle[0]), point(triangle[1]), point(triangle[2]));
            let facing = b.sub(a).cross(c.sub(a));
            assert!(
                facing.dot(a) > 0.0,
                "a triangle is wound inward: {triangle:?}"
            );
        }
    }

    /// Insetting has to leave a gap, which means a panel's corners sit strictly inside
    /// the polygon the tessellation actually assigns to that region.
    #[test]
    fn panels_are_smaller_than_the_regions_they_stand_for() {
        let (mesh, solid) = soccer_ball();
        for (region, span) in mesh.regions.iter().enumerate() {
            let hub = span.first_vertex as usize;
            let centre = Vec3::new(
                mesh.positions[hub][0] as f64,
                mesh.positions[hub][1] as f64,
                mesh.positions[hub][2] as f64,
            );
            for (offset, &corner) in solid.cells[region].iter().enumerate() {
                let drawn = mesh.positions[hub + 1 + offset];
                let drawn = Vec3::new(drawn[0] as f64, drawn[1] as f64, drawn[2] as f64);
                let real = solid.corners[corner as usize].vector();
                assert!(
                    centre.angle_to(drawn) < centre.angle_to(real),
                    "region {region} corner {offset} was not drawn inside its region"
                );
                assert!(
                    (drawn.length() - 1.0).abs() < 1e-6,
                    "panels stay on the sphere"
                );
            }
        }
    }

    /// The defect this fixes: a flat panel dips inside the sphere its corners sit on, and
    /// how far depends on how wide the territory is. A fixed radius for whatever is drawn
    /// underneath held at ninety-two territories and failed at twelve, where the ball came
    /// through the middle of every panel.
    #[test]
    fn the_surface_sags_further_on_a_planet_of_fewer_territories() {
        use sphere_tessellation::{Params, Tessellation};

        let deepest_at = |count: usize| {
            let world = Tessellation::generate_balanced(
                Params {
                    region_count: count,
                    ..Default::default()
                },
                24,
            )
            .0;
            let solid = sphere_tessellation::solid(&world.seeds, &world.neighbours);
            build(&solid, &graph_coloring::color_graph(&world.neighbours)).deepest()
        };

        let tiny = deepest_at(12);
        let huge = deepest_at(92);
        assert!(
            tiny < huge,
            "wider territories sag further: {tiny} vs {huge}"
        );
        assert!(
            tiny < 0.965,
            "twelve territories sag past the old fixed radius: {tiny}"
        );
        assert!(
            huge > 0.965,
            "ninety-two do not, which is why it went unnoticed: {huge}"
        );
        // Whatever is drawn underneath has to clear the deepest point at every size.
        for count in [12, 32, 42, 72, 92] {
            let deepest = deepest_at(count);
            assert!(deepest * 0.985 < deepest, "{count}");
            assert!(deepest > 0.5, "{count}: sanity, a panel is not a spike");
        }
    }

    /// Adjacent regions must not be given the same colour, or the seams vanish and the
    /// whole point of colouring the graph is lost.
    #[test]
    fn neighbouring_panels_are_different_colours() {
        let world = Tessellation::soccer_ball();
        let solid = sphere_tessellation::solid(&world.seeds, &world.neighbours);
        let coloring = graph_coloring::color_graph(&world.neighbours);
        let mesh = build(&solid, &coloring);
        for (region, neighbours) in world.neighbours.iter().enumerate() {
            let mine = mesh.colors[mesh.regions[region].first_vertex as usize];
            for &other in neighbours {
                let theirs = mesh.colors[mesh.regions[other as usize].first_vertex as usize];
                assert_ne!(mine, theirs, "region {region} matches neighbour {other}");
            }
        }
    }

    /// Black stays black and white stays white whatever the transfer curve does in
    /// between; mid grey must move, because sRGB is not linear.
    #[test]
    fn the_palette_is_converted_out_of_srgb() {
        assert_eq!(linear_rgba(0x000000), [0.0, 0.0, 0.0, 1.0]);
        let white = linear_rgba(0xFFFFFF);
        assert!((white[0] - 1.0).abs() < 1e-6);
        let grey = linear_rgba(0x808080)[0];
        assert!(
            grey < 0.25,
            "mid sRGB grey is about 0.216 linear, got {grey}"
        );
    }
}
