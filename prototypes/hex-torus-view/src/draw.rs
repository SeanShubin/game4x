//! One copy bright and its six neighbours dimmed, as SVG.
//!
//! **Seven copies, which is the picture the vetted-when asks a person to look at.** One
//! complete world at full brightness and an echo in each of the six directions - so the
//! wrapping is visible in all six at once, and the bright island is plainly the whole of it.
//!
//! **The bright count is `N` by construction here**, because the drawing places one copy of
//! the domain and translates it. That is deliberately *not* where the count is checked:
//! `tests/wrapping.rs` derives the domain from the reduction and asserts its size, so a
//! reduction that was wrong would fail there rather than draw a tidy picture of nothing.

use crate::{Cell, NEIGHBOURS, Torus};

/// How far apart hex centres are, in SVG units, before the viewer zooms.
const SIZE: f64 = 10.0;

/// The three colours, as a bright and a dim pair.
///
/// **Dimmed rather than greyed**, so an echo keeps the colour of the territory it repeats -
/// a grey echo would say *not a territory* where the point is *this same territory again*.
pub const COLOURS: [(&str, &str); 3] = [
    ("#3d6b9c", "#c2cedd"),
    ("#9c6b3d", "#ddd0c2"),
    ("#4f8a5b", "#c8dccd"),
];

/// Where a hex's centre falls, pointy-top.
pub fn centre(q: i32, r: i32) -> (f64, f64) {
    let x = SIZE * 3.0_f64.sqrt() * (q as f64 + r as f64 / 2.0);
    let y = SIZE * 1.5 * r as f64;
    (x, y)
}

/// The six corners of a hex, as an SVG points list.
pub fn corners(q: i32, r: i32) -> String {
    let (cx, cy) = centre(q, r);
    (0..6)
        .map(|corner| {
            let angle = std::f64::consts::PI / 180.0 * (60.0 * corner as f64 - 30.0);
            format!(
                "{:.2},{:.2}",
                cx + SIZE * angle.cos(),
                cy + SIZE * angle.sin()
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The seven copies to draw: the domain itself, and one translate in each of the six
/// directions of the *lattice*.
///
/// **The lattice's own six neighbours, not the grid's.** Stepping one lattice vector moves a
/// whole world, so these are where the six echoes sit.
pub fn copies(torus: &Torus) -> Vec<(Cell, bool)> {
    let [a, b] = torus.generators();
    let mut out = vec![((0, 0), true)];
    for (m, n) in NEIGHBOURS {
        out.push(((m * a.0 + n * b.0, m * a.1 + n * b.1), false));
    }
    out
}

/// One size, as an SVG group: every hex of all seven copies, with its id and colour.
pub fn group(torus: &Torus, colours: &[u8]) -> String {
    let domain = torus.domain();
    let mut out = String::new();
    out.push_str(&format!(
        "<g class=\"size\" data-cells=\"{}\" data-around=\"{}\" data-family=\"{}\">\n",
        torus.cells(),
        torus.circumference(),
        match torus.family {
            crate::Family::Folded => "folded",
            crate::Family::AxisAligned => "axis-aligned",
        }
    ));
    for (shift, bright) in copies(torus) {
        for (at, (q, r)) in domain.iter().enumerate() {
            let (q, r) = (q + shift.0, r + shift.1);
            let (full, dim) = COLOURS[colours[at] as usize % COLOURS.len()];
            let (cx, cy) = centre(q, r);
            out.push_str(&format!(
                "<polygon points=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"0.6\"/>\n",
                corners(q, r),
                if bright { full } else { dim },
                if bright { "#22303f" } else { "#b8c3ce" }
            ));
            out.push_str(&format!(
                "<text class=\"id\" x=\"{cx:.2}\" y=\"{:.2}\" fill=\"{}\">{at}</text>\n",
                cy + 3.0,
                if bright { "#ffffff" } else { "#7d8a96" }
            ));
        }
    }
    out.push_str("</g>\n");
    out
}

/// How wide and tall the seven copies reach, so the page can frame them.
pub fn extent(torus: &Torus) -> (f64, f64, f64, f64) {
    let domain = torus.domain();
    let (mut left, mut right, mut top, mut bottom) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for (shift, _) in copies(torus) {
        for (q, r) in &domain {
            let (x, y) = centre(q + shift.0, r + shift.1);
            left = left.min(x - SIZE);
            right = right.max(x + SIZE);
            top = top.min(y - SIZE);
            bottom = bottom.max(y + SIZE);
        }
    }
    (left, top, right - left, bottom - top)
}
