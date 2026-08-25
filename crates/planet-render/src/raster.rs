//! Rasterizes the planet view.
//!
//! Rather than projecting region polygons and clipping them, every pixel independently
//! asks the sphere which region it lands in. That is why this prototype has no
//! antimeridian special case, no polar special case, and no trouble with one, two, or
//! three regions: those are all artifacts of building polygons, and we build none.
//!
//! Rendering happens in two passes. The first resolves each pixel to a region and a
//! copy number. The second turns that into color, and finds borders by looking for
//! places where the region changes between neighbouring pixels. Detecting borders on
//! the resolved buffer rather than from distances means they come out an even width
//! everywhere, at any zoom, under either projection, with no per-projection maths.

use crate::font;
use crate::camera::GlobeView;
use crate::palette;
use sphere_tessellation::{Direction, Vec3, nearest_index};

/// Marks a pixel that shows no part of the sphere. Only possible when the ball is held
/// together rather than fanned out.
const SPACE: u32 = u32::MAX;

/// Above this many copies of a region on screen, labels are suppressed as clutter.
const LABEL_COPY_LIMIT: usize = 6;

pub struct Scene<'a> {
    /// The vectors the nearest-region search compares against. **Not unit vectors** —
    /// each carries its face's plane distance. Only ever used with a dot product.
    pub seeds: &'a [Vec3],
    /// Where each region's seed points. Used for anything angular, such as placing a
    /// label. See [`Direction`].
    pub directions: &'a [Direction],
    pub colors: &'a [u8],
    /// Who holds each region, indexed by region id. Empty means nobody holds anything,
    /// in which case the view is exactly as it was before ownership existed.
    pub owners: &'a [Option<u16>],
    pub hovered: Option<usize>,
    pub show_borders: bool,
    pub show_labels: bool,
    pub dim_duplicates: bool,
}

/// A pixel resolved to a region and which copy of the world it belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell {
    region: u32,
    copy: u32,
}

impl Cell {
    const NOTHING: Cell = Cell {
        region: SPACE,
        copy: 0,
    };
}

/// A pixel left untouched by [`render_overlay`]. The rasterizer only ever writes
/// `0x00RRGGBB`, so this cannot collide with a real colour.
pub const TRANSPARENT: u32 = u32::MAX;

/// Draws only what sits *on top of* the planet: labels and cursors. Everything else is
/// left [`TRANSPARENT`].
///
/// This exists so the sphere can be drawn by a shader while the parts that are still
/// easier on the CPU — bitmap text, crosshairs — are composited over it.
pub fn render_overlay(
    buffer: &mut [u32],
    view: &GlobeView,
    scene: &Scene,
    cursor: Option<(f64, f64)>,
) {
    buffer.fill(TRANSPARENT);
    if scene.seeds.is_empty() {
        return;
    }
    if scene.show_labels {
        draw_labels(buffer, view, scene);
    }
    if let Some((x, y)) = cursor {
        draw_cursors(buffer, view, x, y);
    }
}

pub fn render(buffer: &mut [u32], view: &GlobeView, scene: &Scene, cursor: Option<(f64, f64)>) {
    if scene.seeds.is_empty() {
        buffer.fill(palette::BACKGROUND);
        return;
    }
    let mut cells = vec![Cell::NOTHING; view.width * view.height];
    resolve(&mut cells, view, scene);
    shade(buffer, &cells, view, scene);
    if scene.show_labels {
        draw_labels(buffer, view, scene);
    }
    if let Some((x, y)) = cursor {
        draw_cursors(buffer, view, x, y);
    }
}

/// Pass one: which region, and which copy of the world.
fn resolve(cells: &mut [Cell], view: &GlobeView, scene: &Scene) {
    let width = view.width;
    let threads = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(4)
        .clamp(1, 16);
    let rows_per_chunk = view.height.div_ceil(threads).max(1);

    std::thread::scope(|scope| {
        for (index, chunk) in cells.chunks_mut(rows_per_chunk * width).enumerate() {
            let first_row = index * rows_per_chunk;
            scope.spawn(move || {
                for (offset, cell) in chunk.iter_mut().enumerate() {
                    let row = first_row + offset / width;
                    let column = offset % width;
                    *cell = match view
                        .screen_to_sample(column as f64 + 0.5, row as f64 + 0.5)
                    {
                        None => Cell::NOTHING,
                        Some(sample) => Cell {
                            region: nearest_index(scene.seeds, sample.direction.vector()) as u32,
                            copy: sample.copy,
                        },
                    };
                }
            });
        }
    });
}

/// Pass two: color, borders, dimming.
fn shade(buffer: &mut [u32], cells: &[Cell], view: &GlobeView, scene: &Scene) {
    let (width, height) = (view.width, view.height);
    let threads = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(4)
        .clamp(1, 16);
    let rows_per_chunk = height.div_ceil(threads).max(1);

    std::thread::scope(|scope| {
        for (index, chunk) in buffer.chunks_mut(rows_per_chunk * width).enumerate() {
            let first_row = index * rows_per_chunk;
            scope.spawn(move || {
                for (offset, pixel) in chunk.iter_mut().enumerate() {
                    let row = first_row + offset / width;
                    let column = offset % width;
                    let cell = cells[row * width + column];

                    if cell.region == SPACE {
                        *pixel = palette::BACKGROUND;
                        continue;
                    }

                    let mut color = palette::region_color(scene.colors[cell.region as usize]);
                    if let Some(Some(player)) = scene.owners.get(cell.region as usize) {
                        color = palette::mix(color, palette::player_color(*player), palette::OWNER_TINT);
                    }
                    if scene.hovered == Some(cell.region as usize) {
                        color = palette::highlighted(color);
                    }
                    if scene.show_borders && is_border(cells, width, height, column, row) {
                        color = palette::BORDER;
                    }
                    if scene.dim_duplicates && cell.copy > 0 {
                        color = palette::dimmed(color);
                    }
                    *pixel = color;
                }
            });
        }
    });
}

/// A pixel is on a border when one of its neighbours shows a different region, or
/// shows space. Copy number is deliberately ignored: the seam between a region and its
/// own dimmed repeat is not a border between two regions.
fn is_border(cells: &[Cell], width: usize, height: usize, x: usize, y: usize) -> bool {
    let here = cells[y * width + x].region;
    let neighbour = |dx: isize, dy: isize| -> bool {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
            return false;
        }
        cells[ny as usize * width + nx as usize].region != here
    };
    neighbour(1, 0) || neighbour(-1, 0) || neighbour(0, 1) || neighbour(0, -1)
}

fn draw_labels(buffer: &mut [u32], view: &GlobeView, scene: &Scene) {
    let scale = if view.radius > 700.0 { 2 } else { 1 };
    for region in 0..scene.seeds.len() {
        let copies = view.copies_on_screen(scene.directions[region]);
        if copies.len() > LABEL_COPY_LIMIT {
            continue;
        }
        let text = region.to_string();
        let half_width = font::text_width(&text, scale) as f64 / 2.0;
        let half_height = font::text_height(scale) as f64 / 2.0;
        for ((x, y), copy) in copies {
            if x < -half_width
                || y < -half_height
                || x > view.width as f64 + half_width
                || y > view.height as f64 + half_height
            {
                continue;
            }
            let (ink, shadow) = if copy == 0 {
                (palette::LABEL, palette::LABEL_SHADOW)
            } else {
                (
                    palette::mix(palette::BACKGROUND, palette::LABEL, 0.45),
                    palette::BACKGROUND,
                )
            };
            font::draw_label(
                buffer,
                view.width,
                view.height,
                (x - half_width).round() as i64,
                (y - half_height).round() as i64,
                &text,
                ink,
                shadow,
                scale,
            );
        }
    }
}

/// Draws the real cursor plus a dimmed ghost over every repeat of the same point.
fn draw_cursors(buffer: &mut [u32], view: &GlobeView, x: f64, y: f64) {
    let Some(sample) = view.screen_to_sample(x, y) else {
        return;
    };
    for ((copy_x, copy_y), _) in view.copies_on_screen(sample.direction) {
        let is_real = (copy_x - x).abs() < 2.0 && (copy_y - y).abs() < 2.0;
        draw_crosshair(buffer, view.width, view.height, copy_x, copy_y, is_real);
    }
}

fn draw_crosshair(buffer: &mut [u32], width: usize, height: usize, x: f64, y: f64, solid: bool) {
    let strength = if solid { 1.0 } else { 0.45 };
    let centre_x = x.round() as i64;
    let centre_y = y.round() as i64;
    for step in 3i64..=10 {
        for (dx, dy) in [(step, 0), (-step, 0), (0, step), (0, -step)] {
            blend_pixel(
                buffer,
                width,
                height,
                centre_x + dx,
                centre_y + dy,
                palette::CURSOR,
                strength,
            );
        }
    }
    if solid {
        blend_pixel(buffer, width, height, centre_x, centre_y, palette::CURSOR, 1.0);
    }
}

fn blend_pixel(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: i64,
    y: i64,
    color: u32,
    strength: f64,
) {
    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return;
    }
    let index = y as usize * width + x as usize;
    // On an overlay, anything not yet drawn has no colour to blend with.
    let beneath = if buffer[index] == TRANSPARENT {
        palette::BACKGROUND
    } else {
        buffer[index]
    };
    buffer[index] = palette::mix(beneath, color, strength);
}

/// Draws the readout in the top-left corner.
pub fn draw_readout(buffer: &mut [u32], width: usize, height: usize, lines: &[String]) {
    let scale = 1;
    let padding = 6i64;
    let line_height = (font::text_height(scale) + 3) as i64;
    let panel_width = lines
        .iter()
        .map(|line| font::text_width(line, scale))
        .max()
        .unwrap_or(0) as i64
        + padding * 2;
    let panel_height = line_height * lines.len() as i64 + padding * 2 - 3;

    for y in 0..panel_height {
        for x in 0..panel_width {
            blend_pixel(buffer, width, height, x, y, palette::HUD_PANEL, 0.82);
        }
    }
    for (index, line) in lines.iter().enumerate() {
        let color = if index + 1 == lines.len() {
            palette::HUD_DIM
        } else {
            palette::HUD_TEXT
        };
        font::draw_text(
            buffer,
            width,
            height,
            padding,
            padding + index as i64 * line_height,
            line,
            color,
            scale,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Projection;
    use sphere_tessellation::{Params, Tessellation};
    use std::collections::HashSet;

    struct Fixture {
        tessellation: Tessellation,
        colors: Vec<u8>,
        directions: Vec<Direction>,
    }

    fn fixture(region_count: usize) -> Fixture {
        let tessellation = Tessellation::generate(Params {
            region_count,
            ..Default::default()
        });
        let colors = graph_coloring::color_graph(&tessellation.neighbours).colors;
        let directions = tessellation.directions();
        Fixture {
            tessellation,
            colors,
            directions,
        }
    }

    fn scene(fixture: &Fixture) -> Scene<'_> {
        Scene {
            seeds: &fixture.tessellation.seeds,
            directions: &fixture.directions,
            colors: &fixture.colors,
            owners: &[],
            hovered: None,
            show_borders: true,
            show_labels: false,
            dim_duplicates: true,
        }
    }

    fn full_colors() -> HashSet<u32> {
        palette::REGION_COLORS.iter().copied().collect()
    }

    fn dim_colors() -> HashSet<u32> {
        palette::REGION_COLORS
            .iter()
            .map(|&color| palette::dimmed(color))
            .collect()
    }

    #[test]
    fn fanned_out_there_is_no_background_anywhere() {
        let fixture = fixture(20);
        let view = GlobeView::new(200, 140);
        let mut buffer = vec![0xDEAD_BEEF_u32; 200 * 140];
        render(&mut buffer, &view, &scene(&fixture), None);
        assert!(buffer.iter().all(|&pixel| pixel != 0xDEAD_BEEF));
        assert!(
            !buffer.iter().any(|&pixel| pixel == palette::BACKGROUND),
            "the repeats fill the plane, so nothing should show through"
        );
    }

    #[test]
    fn held_together_the_corners_are_space() {
        let fixture = fixture(20);
        let mut view = GlobeView::new(200, 200);
        view.projection = Projection::Globe;
        let mut buffer = vec![0u32; 200 * 200];
        render(&mut buffer, &view, &scene(&fixture), None);
        assert_eq!(buffer[0], palette::BACKGROUND);
        assert_ne!(buffer[100 * 200 + 100], palette::BACKGROUND);
    }

    /// Every region must be on screen, at full strength, in one glance. That is what
    /// fanning the ball out is for.
    #[test]
    fn every_region_is_visible_at_full_strength_at_once() {
        let fixture = fixture(20);
        let view = GlobeView::new(500, 500);
        let mut cells = vec![Cell::NOTHING; 500 * 500];
        resolve(&mut cells, &view, &scene(&fixture));

        let mut seen = vec![false; 20];
        for cell in &cells {
            if cell.region != SPACE && cell.copy == 0 {
                seen[cell.region as usize] = true;
            }
        }
        assert!(
            seen.iter().all(|&hit| hit),
            "some region never appeared: {seen:?}"
        );
    }

    #[test]
    fn the_repeats_are_dimmed_and_the_first_copy_is_not() {
        let fixture = fixture(20);
        let mut view = GlobeView::new(400, 400);
        view.radius = 90.0;
        let mut buffer = vec![0u32; 400 * 400];
        render(&mut buffer, &view, &scene(&fixture), None);

        let full = full_colors();
        let dim = dim_colors();
        assert!(buffer.iter().any(|pixel| full.contains(pixel)));
        assert!(buffer.iter().any(|pixel| dim.contains(pixel)));
    }

    /// Full strength appears only inside the world disc. Outside it is all repeats.
    #[test]
    fn full_strength_pixels_stay_inside_the_world_disc() {
        let fixture = fixture(20);
        let mut scene = scene(&fixture);
        scene.show_labels = false;

        for radius in [70.0, 110.0, 160.0] {
            let mut view = GlobeView::new(400, 400);
            view.radius = radius;
            let mut buffer = vec![0u32; 400 * 400];
            render(&mut buffer, &view, &scene, None);

            let full = full_colors();
            for y in 0..400 {
                for x in 0..400 {
                    if !full.contains(&buffer[y * 400 + x]) {
                        continue;
                    }
                    let distance =
                        ((x as f64 + 0.5 - 200.0).powi(2) + (y as f64 + 0.5 - 200.0).powi(2)).sqrt();
                    assert!(
                        distance <= radius + 2.0,
                        "radius {radius}: full-strength pixel at ({x}, {y}) is \
                         {distance:.1} out, past the world disc"
                    );
                }
            }
        }
    }

    #[test]
    fn turning_off_dimming_removes_the_duplicate_shading() {
        let fixture = fixture(20);
        let mut view = GlobeView::new(400, 400);
        view.radius = 90.0;
        let mut scene = scene(&fixture);
        scene.dim_duplicates = false;
        let mut buffer = vec![0u32; 400 * 400];
        render(&mut buffer, &view, &scene, None);
        let dim = dim_colors();
        assert!(!buffer.iter().any(|pixel| dim.contains(pixel)));
    }

    #[test]
    fn borders_appear_between_regions() {
        let fixture = fixture(20);
        let view = GlobeView::new(300, 300);
        let mut with = vec![0u32; 300 * 300];
        render(&mut with, &view, &scene(&fixture), None);
        assert!(with.iter().any(|&pixel| pixel == palette::BORDER));

        let mut without_scene = scene(&fixture);
        without_scene.show_borders = false;
        let mut without = vec![0u32; 300 * 300];
        render(&mut without, &view, &without_scene, None);
        assert!(!without.iter().any(|&pixel| pixel == palette::BORDER));
    }

    /// Borders are found from the resolved buffer, so they stay an even width however
    /// far in you zoom. A distance-based test would drift with the projection.
    #[test]
    fn borders_keep_their_width_at_every_zoom() {
        let fixture = fixture(20);
        for radius in [80.0, 200.0, 900.0, 4000.0] {
            let mut view = GlobeView::new(300, 300);
            view.radius = radius;
            let mut buffer = vec![0u32; 300 * 300];
            render(&mut buffer, &view, &scene(&fixture), None);

            // Measure runs of border pixels across many rows. A run is only a
            // reading of thickness when the border crosses the row roughly at right
            // angles; a border that happens to lie along the row makes an arbitrarily
            // long run and says nothing. So take the median across many rows, which
            // ignores those while still catching a border that genuinely thickens.
            let mut runs = Vec::new();
            for y in (20..280).step_by(4) {
                let mut run = 0usize;
                for x in 0..300 {
                    if buffer[y * 300 + x] == palette::BORDER {
                        run += 1;
                    } else if run > 0 {
                        runs.push(run);
                        run = 0;
                    }
                }
            }
            assert!(!runs.is_empty(), "radius {radius}: no borders drawn at all");
            runs.sort_unstable();
            let median = runs[runs.len() / 2];
            assert!(
                median <= 3,
                "radius {radius}: median border run was {median} pixels"
            );
        }
    }

    #[test]
    fn the_cursor_leaves_ghosts_on_the_repeats() {
        let fixture = fixture(20);
        let mut view = GlobeView::new(400, 400);
        view.radius = 80.0;

        let mut without = vec![0u32; 400 * 400];
        render(&mut without, &view, &scene(&fixture), None);
        let mut with = vec![0u32; 400 * 400];
        render(&mut with, &view, &scene(&fixture), Some((250.0, 200.0)));

        let changed = without.iter().zip(with.iter()).filter(|(a, b)| a != b).count();
        assert!(changed > 32 * 2, "only {changed} pixels changed");
    }

    #[test]
    fn a_single_region_covers_everything() {
        let fixture = fixture(1);
        let view = GlobeView::new(120, 90);
        let mut buffer = vec![0u32; 120 * 90];
        render(&mut buffer, &view, &scene(&fixture), None);
        let only = palette::region_color(fixture.colors[0]);
        let allowed = [only, palette::dimmed(only)];
        assert!(
            buffer.iter().all(|pixel| allowed.contains(pixel)),
            "a lone region should never produce a border"
        );
    }

    #[test]
    fn degenerate_region_counts_render() {
        for region_count in [1, 2, 3, 4, 7] {
            let fixture = fixture(region_count);
            for projection in [Projection::Fanned, Projection::Globe] {
                let mut view = GlobeView::new(120, 90);
                view.projection = projection;
                let mut buffer = vec![0u32; 120 * 90];
                render(&mut buffer, &view, &scene(&fixture), Some((60.0, 45.0)));
                assert!(buffer.iter().any(|&pixel| pixel != palette::BACKGROUND));
            }
        }
    }

    #[test]
    fn adjacent_regions_never_share_a_color() {
        let fixture = fixture(20);
        assert!(
            graph_coloring::find_conflict(&fixture.tessellation.neighbours, &fixture.colors)
                .is_none()
        );
    }

    /// Rotating to any orientation always yields a complete picture, with no seam and
    /// no empty patch. The flat map could not promise this near the poles.
    #[test]
    fn every_orientation_renders_a_complete_world() {
        let fixture = fixture(20);
        for step in 0..24 {
            let mut view = GlobeView::new(160, 160);
            view.drag(step as f64 * 37.0, step as f64 * 23.0);
            let mut cells = vec![Cell::NOTHING; 160 * 160];
            resolve(&mut cells, &view, &scene(&fixture));

            let mut seen = vec![false; 20];
            for cell in &cells {
                if cell.region != SPACE && cell.copy == 0 {
                    seen[cell.region as usize] = true;
                }
            }
            assert!(
                seen.iter().all(|&hit| hit),
                "orientation {step} hid a region: {seen:?}"
            );
        }
    }
}
