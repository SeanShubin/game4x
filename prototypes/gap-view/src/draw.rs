//! The map as an SVG, because the question this prototype asks is about looking at it.
//!
//! **A drawing is the instrument here rather than polish.** Whether a world laid out this way
//! reads as a world, and whether a territory on the far side is something a person would
//! click, are judgements - `docs/prototypes/README.md`: *an appearance cannot be settled by a
//! measurement*. The numbers in [`crate`] say what the projection costs; only a picture says
//! whether that is worth paying.
//!
//! **Adjacency is drawn as a line between centres**, which the globe never needs. Where the
//! gaps are wide the tiles no longer touch, so touching can no longer be what shows two
//! territories are neighbours. That is the one thing this projection takes away and the line
//! is what gives it back - whether it is enough is the thing to look at.

use crate::{Layout, Point, seam};

/// How the map is drawn.
pub struct Style {
    /// Pixels per radian.
    pub scale: f64,
    /// Ink for the space outside the world.
    pub background: &'static str,
    /// Territory fill, nearest the focus and furthest from it; every tile is between them.
    pub near: (u8, u8, u8),
    pub far: (u8, u8, u8),
    /// Whether to draw a line between neighbouring centres.
    pub adjacency: bool,
    /// Whether to write each territory's id on it, which `spec/planet.md` asks of the
    /// practical drawing.
    pub ids: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            scale: 200.0,
            background: "#12131a",
            near: (0xf2, 0xe8, 0xd5),
            far: (0x4a, 0x5a, 0x78),
            adjacency: true,
            ids: true,
        }
    }
}

fn mix(near: (u8, u8, u8), far: (u8, u8, u8), how_far: f64) -> String {
    let how_far = how_far.clamp(0.0, 1.0);
    let channel = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * how_far).round() as u8;
    format!(
        "#{:02x}{:02x}{:02x}",
        channel(near.0, far.0),
        channel(near.1, far.1),
        channel(near.2, far.2)
    )
}

/// The whole world on one page.
///
/// `neighbours` is the adjacency the game logic sees, and is what the connecting lines are
/// drawn from - so a line on this drawing is a move the rules allow, not a guess made from
/// how close two tiles happen to look.
pub fn svg(layout: &Layout, neighbours: &[Vec<u32>], style: &Style) -> String {
    let reach = std::f64::consts::PI;
    let margin = 24.0;
    let size = (reach * 2.0 * style.scale + margin * 2.0).ceil();
    let middle = size / 2.0;
    let place = |point: Point| {
        (
            middle + point.x * style.scale,
            middle + point.y * style.scale,
        )
    };

    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {size} {size}\" \
         width=\"{size}\" height=\"{size}\">\n"
    ));
    out.push_str(&format!(
        "<rect width=\"{size}\" height=\"{size}\" fill=\"{}\"/>\n",
        style.background
    ));

    // The rim: where the antipode of the focus lands. Everything in the world is inside it.
    out.push_str(&format!(
        "<circle cx=\"{middle}\" cy=\"{middle}\" r=\"{}\" fill=\"none\" stroke=\"#2b2f3d\" \
         stroke-width=\"1\" stroke-dasharray=\"4 6\"/>\n",
        reach * style.scale
    ));

    if style.adjacency {
        out.push_str("<g stroke=\"#39405400\" stroke-width=\"1.25\">\n");
        for (territory, near) in neighbours.iter().enumerate() {
            for &other in near {
                let other = other as usize;
                if other < territory {
                    continue;
                }
                let (x1, y1) = place(layout.tile(territory).centre);
                let (x2, y2) = place(layout.tile(other).centre);
                // Faded with distance, so the crowded middle stays readable and the sparse
                // rim still shows what connects to what.
                let how_far = (layout.tile(territory).from_focus + layout.tile(other).from_focus)
                    / (2.0 * reach);
                let opacity = 0.15 + 0.5 * how_far;
                out.push_str(&format!(
                    "<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" \
                     stroke=\"#7f8ba6\" stroke-opacity=\"{opacity:.2}\"/>\n"
                ));
            }
        }
        out.push_str("</g>\n");
    }

    let seam = seam(layout);
    for tile in &layout.tiles {
        let points: Vec<String> = tile
            .outline
            .iter()
            .map(|corner| {
                let (x, y) = place(*corner);
                format!("{x:.1},{y:.1}")
            })
            .collect();
        let fill = mix(style.near, style.far, tile.from_focus / reach);
        let stroke = if tile.territory == seam {
            "#e0533f"
        } else {
            "#00000033"
        };
        let width = if tile.territory == seam { 2.5 } else { 1.0 };
        out.push_str(&format!(
            "<polygon points=\"{}\" fill=\"{fill}\" stroke=\"{stroke}\" \
             stroke-width=\"{width}\"/>\n",
            points.join(" ")
        ));
    }

    if style.ids {
        for tile in &layout.tiles {
            let (x, y) = place(tile.centre);
            // Dark on the pale near tiles, pale on the dark far ones, so the id is legible
            // the whole way out rather than only near the focus.
            let ink = if tile.from_focus / reach < 0.45 {
                "#1b1c22"
            } else {
                "#e8ecf5"
            };
            out.push_str(&format!(
                "<text x=\"{x:.1}\" y=\"{:.1}\" font-family=\"system-ui, sans-serif\" \
                 font-size=\"11\" text-anchor=\"middle\" fill=\"{ink}\">{}</text>\n",
                y + 4.0,
                tile.territory
            ));
        }
    }

    // The focus, which is the one place the map is exactly the globe.
    let (x, y) = place(Point::ORIGIN);
    out.push_str(&format!(
        "<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3\" fill=\"#e0a33f\"/>\n"
    ));

    out.push_str("</svg>\n");
    out
}
