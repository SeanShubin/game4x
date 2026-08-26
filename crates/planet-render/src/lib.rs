//! The planet view, with no graphics engine in it.
//!
//! Everything needed to turn a sphere into a screenful of pixels: the world, the
//! camera, the projections, and a software rasterizer. What is missing is deliberate —
//! there is no window, no input device, no engine. See [`app::PlanetView`] for the
//! boundary an engine adapter talks to.
//!
//! The layering, per `docs/architecture.md`:
//!
//! ```text
//! sphere-tessellation, graph-coloring   the model: pure, no rendering
//!               |
//!         planet-render                 this crate: pixels, no engine
//!               |
//!          planet-bevy                  the engine adapter
//!               |
//!          planet-view                  the composition root
//! ```

pub mod app;
pub mod camera;
pub mod font;
pub mod mesh;
pub mod palette;
pub mod raster;
pub mod world;

pub use app::{Command, PlanetView, Settings};
pub use camera::{GlobeView, Projection};
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
