//! Rendering one frame to a PNG, with no window and no graphics engine.
//!
//! This exists because it is how the view gets checked without a human at the window,
//! and it only works because `planet-render` has no engine in it.

use crate::options::Options;
use planet_render::camera::Projection;
use planet_render::PlanetView;
use std::error::Error;

pub fn write_frame(options: &Options, path: &str) -> Result<(), Box<dyn Error>> {
    let mut planet = PlanetView::new(options.spec(), options.width, options.height);
    if options.globe {
        planet.view_mut().projection = Projection::Globe;
    }
    planet.view_mut().radius =
        planet_render::camera::default_radius(options.width, options.height) * options.zoom;
    aim(&mut planet, options.turn_right, options.turn_up);

    let mut pixels = vec![0u32; planet.pixel_count()];
    let started = std::time::Instant::now();
    planet.draw(&mut pixels, None);
    let elapsed = started.elapsed();

    write_png(path, &pixels, options.width, options.height)?;
    println!(
        "wrote {path} ({}x{}) in {:.1} ms/frame, {} regions, {} colors",
        options.width,
        options.height,
        elapsed.as_secs_f64() * 1000.0,
        planet.world().tessellation.region_count(),
        planet.world().coloring.color_count
    );
    Ok(())
}

/// Turns the sphere by the given angles, as a drag would.
fn aim(planet: &mut PlanetView, right: f64, up: f64) {
    let step = planet.view().radians_per_pixel();
    if right != 0.0 {
        planet.drag(right / step, 0.0);
    }
    if up != 0.0 {
        // Dragging the surface down moves the view north, so turning the view up is
        // a positive drag.
        planet.drag(0.0, up / step);
    }
}

fn write_png(path: &str, pixels: &[u32], width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let mut data = Vec::with_capacity(width * height * 3);
    for &pixel in pixels {
        data.push((pixel >> 16) as u8);
        data.push((pixel >> 8) as u8);
        data.push(pixel as u8);
    }
    let file = std::fs::File::create(path)?;
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header()?.write_image_data(&data)?;
    Ok(())
}
