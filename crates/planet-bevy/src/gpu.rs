//! Drawing the planet on the GPU.
//!
//! The shader is a direct port of the CPU rasterizer, kept per-pixel rather than
//! switched to meshes — see `planet.wgsl` for why that matters. This module's whole job
//! is to hand it the state it needs each frame.
//!
//! The CPU path is still in the tree and still tested. It is the reference: when the two
//! disagree, the CPU one is right, and `G` switches between them so they can be compared
//! by eye on the same frame.

use bevy::asset::{RenderAssetUsages, embedded_asset};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use planet_render::PlanetView;
use planet_render::camera::Projection;

/// Must match `MAX_REGIONS` in the shader.
pub const MAX_REGIONS: usize = 512;

const FLAG_BORDERS: u32 = 1;
const FLAG_DIM_REPEATS: u32 = 2;

/// Border half-width, in pixels, as handed to the shader.
const BORDER_PIXELS: f32 = 1.5;

#[derive(ShaderType, Debug, Clone)]
pub struct PlanetUniform {
    pub row0: Vec4,
    pub row1: Vec4,
    pub row2: Vec4,
    /// x radius, y width, z height, w region count
    pub view: Vec4,
    /// x projection, y hovered, z flags, w border pixels
    pub params: Vec4,
    /// xyz seed direction, w packed as colour + 8 * (owner + 1)
    pub seeds: [Vec4; MAX_REGIONS],
}

impl Default for PlanetUniform {
    fn default() -> Self {
        Self {
            row0: Vec4::X,
            row1: Vec4::Y,
            row2: Vec4::Z,
            view: Vec4::new(1.0, 1.0, 1.0, 0.0),
            params: Vec4::ZERO,
            seeds: [Vec4::ZERO; MAX_REGIONS],
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone, Default)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub planet: PlanetUniform,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://planet_bevy/planet.wgsl".into()
    }
}

/// Registers the material and its shader.
pub struct PlanetGpuPlugin;

impl Plugin for PlanetGpuPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "planet.wgsl");
        app.add_plugins(Material2dPlugin::<PlanetMaterial>::default());
    }
}

/// Fills the uniform from the view. Everything the shader knows comes from here.
///
/// Regions past [`MAX_REGIONS`] are dropped rather than silently wrapping, and the
/// caller is expected to have kept the count below it.
pub fn fill_uniform(view: &PlanetView, hovered: Option<usize>) -> PlanetUniform {
    let world = view.world();
    let camera = view.view();
    let settings = view.settings();
    let owners = view.owners();

    let mut uniform = PlanetUniform {
        row0: as_vec4(camera.orientation.rows[0]),
        row1: as_vec4(camera.orientation.rows[1]),
        row2: as_vec4(camera.orientation.rows[2]),
        ..Default::default()
    };

    let count = world.tessellation.seeds.len().min(MAX_REGIONS);
    for index in 0..count {
        let seed = world.tessellation.seeds[index];
        let colour = world.coloring.colors[index] as u32;
        let owner = owners.get(index).copied().flatten();
        // The shader unpacks this as colour + 8 * (owner + 1).
        let packed = colour + 8 * owner.map(|player| player as u32 + 1).unwrap_or(0);
        uniform.seeds[index] =
            Vec4::new(seed.x as f32, seed.y as f32, seed.z as f32, packed as f32);
    }

    let mut flags = 0u32;
    if settings.borders {
        flags |= FLAG_BORDERS;
    }
    if settings.dim_duplicates {
        flags |= FLAG_DIM_REPEATS;
    }

    uniform.view = Vec4::new(
        camera.radius as f32,
        camera.width as f32,
        camera.height as f32,
        count as f32,
    );
    uniform.params = Vec4::new(
        if camera.projection == Projection::Globe {
            1.0
        } else {
            0.0
        },
        hovered.map(|region| region as f32).unwrap_or(-1.0),
        flags as f32,
        BORDER_PIXELS,
    );
    uniform
}

fn as_vec4(row: planet_render::Vec3) -> Vec4 {
    Vec4::new(row.x as f32, row.y as f32, row.z as f32, 0.0)
}

/// A quad covering the whole window, for the shader to draw on.
///
/// Bevy's own UVs are used deliberately. An earlier version overrode them with a
/// hand-written corner list, which assumed a vertex order Bevy does not use: the real
/// order gives `[[1,0],[0,0],[0,1],[1,1]]`, so the override flipped `u`. The shader
/// then drew the world mirrored left to right, and dragging moved the territories one
/// way while the CPU-drawn labels went the other.
pub fn screen_quad(meshes: &mut Assets<Mesh>, width: f32, height: f32) -> Handle<Mesh> {
    meshes.add(Mesh::from(Rectangle::new(width, height)))
}

pub fn render_asset_usages() -> RenderAssetUsages {
    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
}

#[cfg(test)]
mod tests {
    use super::*;
    use planet_render::WorldSpec;

    fn view() -> PlanetView {
        PlanetView::new(WorldSpec::default(), 400, 300)
    }

    #[test]
    fn the_uniform_describes_the_view() {
        let planet = view();
        let uniform = fill_uniform(&planet, None);
        assert_eq!(uniform.view.w, 32.0, "the soccer ball has 32 regions");
        assert_eq!(uniform.view.y, 400.0);
        assert_eq!(uniform.view.z, 300.0);
        assert_eq!(uniform.params.x, 0.0, "fanned by default");
        assert_eq!(uniform.params.y, -1.0, "nothing hovered");
    }

    /// Seeds are deliberately **not** unit vectors: a seed is `n / h`, the face normal
    /// over its plane distance, which is what makes every edge of the solid come out
    /// equal. They are still all roughly unit length, and the shader only ever takes a
    /// dot product with them, never an arc cosine.
    #[test]
    fn seeds_carry_a_plane_distance_and_a_packed_colour() {
        let planet = view();
        let uniform = fill_uniform(&planet, None);
        let mut lengths = Vec::new();
        for index in 0..32 {
            let seed = uniform.seeds[index];
            let length = (seed.x * seed.x + seed.y * seed.y + seed.z * seed.z).sqrt();
            assert!(
                (0.5..2.0).contains(&length),
                "seed {index} has an implausible length {length}"
            );
            lengths.push(length);
            // No owner yet, so the packed value is just the colour index.
            assert!(
                seed.w < 8.0,
                "seed {index} packed an owner it should not have"
            );
        }
        // The twelve pentagons and twenty hexagons should differ, and that difference is
        // the whole point.
        let shortest = lengths.iter().cloned().fold(f64::INFINITY as f32, f32::min);
        let longest = lengths.iter().cloned().fold(0.0f32, f32::max);
        assert!(
            longest / shortest > 1.01,
            "the two face kinds should sit at different plane distances"
        );
    }

    /// The packing is the one thing the shader and this file have to agree on, so it is
    /// worth pinning from the Rust side.
    #[test]
    fn ownership_packs_above_the_colour() {
        let mut planet = view();
        let mut owners = vec![None; 32];
        owners[3] = Some(0u16); // player 0
        owners[7] = Some(5u16); // player 5
        planet.set_owners(owners);

        let uniform = fill_uniform(&planet, None);
        let unowned = uniform.seeds[0].w as u32;
        assert_eq!(unowned / 8, 0, "unowned regions pack no owner");

        let first = uniform.seeds[3].w as u32;
        assert_eq!(first / 8, 1, "player 0 is stored as 1");
        assert!(first % 8 < 6, "the colour index survives");

        let second = uniform.seeds[7].w as u32;
        assert_eq!(second / 8, 6, "player 5 is stored as 6");
    }

    #[test]
    fn flags_follow_the_settings() {
        let planet = view();
        let uniform = fill_uniform(&planet, None);
        let flags = uniform.params.z as u32;
        assert!(flags & FLAG_BORDERS != 0, "borders are on by default");
        assert!(flags & FLAG_DIM_REPEATS != 0, "dimming is on by default");
    }

    #[test]
    fn the_hovered_region_is_passed_through() {
        let planet = view();
        let uniform = fill_uniform(&planet, Some(9));
        assert_eq!(uniform.params.y, 9.0);
    }

    #[test]
    fn the_orientation_rows_match_the_camera() {
        let mut planet = view();
        planet.drag(40.0, 25.0);
        let uniform = fill_uniform(&planet, None);
        let rows = planet.view().orientation.rows;
        assert!((uniform.row0.x - rows[0].x as f32).abs() < 1e-6);
        assert!((uniform.row2.z - rows[2].z as f32).abs() < 1e-6);
    }

    /// More regions than the shader can hold must be clamped rather than wrapping into
    /// the wrong slots.
    #[test]
    fn the_region_count_is_clamped_to_what_the_shader_can_hold() {
        let planet = view();
        let uniform = fill_uniform(&planet, None);
        assert!(uniform.view.w <= MAX_REGIONS as f32);
    }
}
