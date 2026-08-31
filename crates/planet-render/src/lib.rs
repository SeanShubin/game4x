//! The planet view, with no graphics engine in it.
//!
//! A world, the mesh that draws it, and the palette both drawings use. What is missing is
//! deliberate — there is no window, no input device, no engine.
//!
//! The software rasterizer that used to live here is
//! [`planet-raster`](../planet_raster/index.html), and it moved because the game never
//! called any of it: this crate was two crates wearing one name, and every binary paid for
//! both.
//!
//! The layering, per `docs/architecture.md`:
//!
//! ```text
//! sphere-tessellation, graph-coloring   the model: pure, no rendering
//!               |
//!         planet-render                 this crate: a world and a mesh, no engine
//!               |
//!          planet-bevy                  the engine adapter for the globe
//!               |
//!            game4x                     the composition root
//! ```

pub mod mesh;
pub mod palette;
pub mod realistic;
pub mod world;

pub use mesh::{PlanetMesh, RegionSpan};
pub use world::{World, WorldSpec};

/// The integer adjacency graph for a world, which is all the model ever sees.
///
/// Lives here because this crate already knows how to build a world; the composition
/// root should not have to reach through a tessellation to find the graph.
pub fn topology_of(spec: WorldSpec) -> planet_model::Topology {
    let world = World::build(spec);
    planet_model::Topology::from_neighbour_lists(&world.tessellation.neighbours)
}

// Re-exported so that callers do not need a direct dependency on the model crates
// just to name the parameters of a world.
pub use sphere_tessellation::{Params, Tessellation, Vec3};
