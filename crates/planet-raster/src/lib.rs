//! A software rasterizer for a tessellated sphere: a camera, projections, a font, and one
//! frame at a time.
//!
//! Every pixel independently asks the sphere which region it lands in. That is slower than
//! a mesh and it is the point: there is no antimeridian split, no clipping at the rim, no
//! instancing for repeating rings, and no trouble at one or two regions. Fragments do not
//! consult their neighbours, so there are no seams to get wrong.
//!
//! [`app::PlanetView`] is the object an engine adapter talks to: a whole session — drag,
//! zoom, resize, regenerate, draw — runs in a unit test with no window open, and the
//! assertions are made on actual pixels.
//!
//! # Why it is a crate rather than four modules
//!
//! It was four modules of `planet-render`, and **the game never called any of them.**
//! `planet-render` was two crates wearing one name: world building and the mesh, which the
//! game draws with, and this, which only the flat-projection prototype uses. Every binary
//! paid for both — about 2,200 lines reachable from `game4x` through code it never calls.
//!
//! A dependency tree is worth having when it describes what a binary actually contains.
//! Splitting is what makes it an audit rather than a diagram.
//!
//! It sits *above* `planet-render` rather than beside it, because it draws the same worlds:
//! the palette and the world are shared, and the technique is not.

pub mod app;
pub mod camera;
pub mod font;
pub mod raster;

pub use app::{Command, PlanetView, Settings};
pub use camera::{GlobeView, Projection};
