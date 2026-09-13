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

/// The copies to draw: the domain itself, and the translates that surround it without a gap.
///
/// **How many echoes it takes depends on the shape of the domain, and it is not always six.**
/// Six is right for a hexagonal domain - the folded family reduces to the lattice's Voronoi
/// cell, and its six nearest translates close round it. **It is wrong for a parallelogram.**
/// The axis-aligned family's domain is the square `0..C × 0..C` and the offset family's is
/// `0..W × 0..H`, and a parallelogram is surrounded by the **eight** translates of a three by
/// three block: `±a`, `±b`, `±(a+b)` and `±(a−b)`. Take the six and the corners are left open.
///
/// **Nobody noticed for the whole life of the axis-aligned family.** It landed on 2026-09-12
/// drawing six echoes round a square, and the picture had two notched corners in every one of
/// its ten sizes - visible in every screenshot taken of it, including the ones sent to Sean.
/// `X-37`'s offset family made it obvious rather than new: that lattice is sheared far enough
/// that the gaps are wedges rather than notches.
///
/// **No check could have caught it and none was missing.** `the_copies_do_not_overlap` asserts
/// that no two copies draw the same cell, which was true; *and leave no gap* is the other
/// direction, and is asserted now. This is `Q-87`'s shape again - the one direction that could
/// not fail was the one nobody wrote.
pub fn copies(torus: &Torus) -> Vec<(Cell, bool)> {
    let [a, b] = torus.generators();
    let around: Vec<(i32, i32)> = match torus.family {
        crate::Family::Folded => NEIGHBOURS.to_vec(),
        crate::Family::AxisAligned | crate::Family::Offset { .. } => (-1..=1)
            .flat_map(|m| (-1..=1).map(move |n| (m, n)))
            .filter(|step| *step != (0, 0))
            .collect(),
    };
    let mut out = vec![((0, 0), true)];
    for (m, n) in around {
        out.push(((m * a.0 + n * b.0, m * a.1 + n * b.1), false));
    }
    out
}

/// How many copies this family draws - one bright and the rest echoes.
///
/// **Stated here so a check can read it rather than assume seven**, which is what every count
/// in `tests/drawing.rs` did until the offset family arrived.
pub fn copies_drawn(torus: &Torus) -> usize {
    match torus.family {
        crate::Family::Folded => 7,
        crate::Family::AxisAligned | crate::Family::Offset { .. } => 9,
    }
}

/// How far round, said in the shortest way that is still true.
///
/// **One number where the six agree and three where they do not.** Two families are isotropic
/// and the third is not - `X-37` - so a single figure would be a claim rather than a reading.
pub fn around(torus: &Torus) -> String {
    let closes = torus.circumferences();
    if closes.iter().all(|n| *n == closes[0]) {
        return closes[0].to_string();
    }
    // The six come in opposite pairs, so three numbers say all of it.
    format!("{}, {}, {}", closes[0], closes[1], closes[2])
}

/// One size, as an SVG group: every hex of all seven copies, with its id and colour.
///
/// **Every polygon says which territory it is, and all seven copies of one say the same
/// thing.** That is what makes `X-36`'s hover possible, and the hover is a distance
/// instrument: a territory is drawn in seven places and only one of them is near the middle,
/// so two that look far apart in the bright region may be adjacent through a wrap. **A page
/// without this does not merely fail to show distance, it misleads about it** - Sean's
/// observation, and the reason the echoes are load-bearing rather than decorative.
///
/// `partner` is the world at the same circumference in the other family, if there is one.
pub fn group(torus: &Torus, colours: &[u8], partner: Option<usize>) -> String {
    let domain = torus.domain();
    let mut out = String::new();
    out.push_str(&format!(
        "<g class=\"size\" data-cells=\"{}\" data-around=\"{}\" data-family=\"{}\" data-partner=\"{}\">\n",
        torus.cells(),
        // **All six, because one of the three families does not have one number.** `X-37`:
        // the offset family closes in `2W`, `H`, `2W`, and a page that showed the shortest
        // would say `12` about a world where half the walks take 24 - which is the claim the
        // measurement overturned, restated by the drawing.
        around(torus),
        torus.family.name(),
        // **`-1` rather than an absent attribute**, so the page reads one thing in both cases
        // and nothing has to tell *no partner* from *a partner at index zero*.
        partner.map_or(-1, |which| which as i64)
    ));
    for (shift, bright) in copies(torus) {
        for (at, (q, r)) in domain.iter().enumerate() {
            let (q, r) = (q + shift.0, r + shift.1);
            let (full, dim) = COLOURS[colours[at] as usize % COLOURS.len()];
            let fill = if bright { full } else { dim };
            let (cx, cy) = centre(q, r);
            // **The resting fill travels with the hex** - lighting one sets `fill` to the
            // bright colour and letting go sets it back to this, so a dimmed echo returns to
            // dim and the bright copy returns to bright without the page tracking which was
            // which.
            out.push_str(&format!(
                "<polygon data-cell=\"{at}\" data-fill=\"{fill}\" data-full=\"{full}\" points=\"{}\" fill=\"{fill}\" stroke=\"{}\" stroke-width=\"0.6\"/>\n",
                corners(q, r),
                if bright { "#22303f" } else { "#b8c3ce" }
            ));
            out.push_str(&format!(
                "<text class=\"id\" data-cell=\"{at}\" x=\"{cx:.2}\" y=\"{:.2}\" fill=\"{}\">{at}</text>\n",
                cy + 3.0,
                if bright { "#ffffff" } else { "#7d8a96" }
            ));
        }
    }
    out.push_str("</g>\n");
    out
}

/// How wide and tall the copies reach, so the page can frame them.
pub fn extent(torus: &Torus) -> (f64, f64, f64, f64) {
    extent_turned(torus, false)
}

/// **A flat-top drawing of this grid is the pointy-top one turned thirty degrees.** Exactly,
/// with no scale: taking the pointy-top placement `x = √3(q + r/2), y = 1.5r` into the flat-top
/// one `x = 1.5q, y = √3(r + q/2)` is left-multiplication by a matrix whose determinant is 1
/// and whose angle is 30°, measured rather than reasoned.
///
/// **So the orientation toggle `X-37` asks for needs no second copy of any geometry.** One SVG
/// `rotate(30)` on the group turns the whole drawing, and nothing in the lattice moves - every
/// adjacency, every id and every wrap is identical. What changes is which walks read as
/// natural, which is why it is not merely cosmetic to Sean even though nothing abstract moves:
/// **his *twelve up* and *twelve right* are flat-top readings**, and neither is a straight walk
/// in a pointy-top picture.
///
/// The one thing that does need saying twice is the frame, because turning the drawing changes
/// what rectangle contains it.
pub const TURN: f64 = 30.0;

/// The frame, in whichever orientation the page is showing.
pub fn extent_turned(torus: &Torus, turned: bool) -> (f64, f64, f64, f64) {
    let domain = torus.domain();
    let angle = if turned {
        std::f64::consts::PI / 180.0 * TURN
    } else {
        0.0
    };
    let (cos, sin) = (angle.cos(), angle.sin());
    let (mut left, mut right, mut top, mut bottom) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for (shift, _) in copies(torus) {
        for (q, r) in &domain {
            let (x, y) = centre(q + shift.0, r + shift.1);
            let (x, y) = (x * cos - y * sin, x * sin + y * cos);
            left = left.min(x - SIZE);
            right = right.max(x + SIZE);
            top = top.min(y - SIZE);
            bottom = bottom.max(y + SIZE);
        }
    }
    (left, top, right - left, bottom - top)
}
