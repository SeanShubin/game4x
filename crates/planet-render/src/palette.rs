//! Colors for the simplified view.
//!
//! These are labels, not terrain. The palette follows the perceptual guidance in
//! `docs/theory/region-coloring.md`: four colors at clearly different lightness
//! values, spread around the hue circle, avoiding a red-green opposition. Because the
//! lightness values are well separated, the map stays readable in grayscale — which
//! is the cheap proxy for staying readable under any color vision deficiency.
//!
//! Approximate luma of the first four: 53, 103, 139, 213 out of 255.

/// Indices 0 to 3 are what the four-coloring actually uses. The last two exist only
/// for the greedy fallback, which should never fire on a planar graph.
pub const REGION_COLORS: [u32; 6] = [
    0x1B3A5C, // dark blue
    0x8B4A9C, // purple
    0x4FA88F, // teal green
    0xE8D98A, // pale gold
    0xC4573C, // burnt orange
    0xF2F2EC, // off white
];

pub const BACKGROUND: u32 = 0x0C0E12;
pub const BORDER: u32 = 0x05070A;
pub const CURSOR: u32 = 0xFFFFFF;
pub const HUD_TEXT: u32 = 0xE6E9EF;
pub const HUD_DIM: u32 = 0x8A93A3;
pub const HUD_PANEL: u32 = 0x0A0C10;
pub const LABEL: u32 = 0xFFFFFF;
pub const LABEL_SHADOW: u32 = 0x000000;

/// How much of the original color a duplicate keeps.
pub const DUPLICATE_STRENGTH: f64 = 0.34;

/// One colour per player, for tinting owned regions.
///
/// Deliberately unlike [`REGION_COLORS`]: those are arbitrary labels for the graph
/// colouring, these carry meaning about the world. Keeping the two palettes apart is
/// the rule in `docs/theory/region-coloring.md`.
pub const PLAYER_COLORS: [u32; 6] = [
    0xE5484D, // red
    0x3E9BFF, // blue
    0x30A46C, // green
    0xF5D90A, // yellow
    0xE58C2C, // orange
    0xB07DE8, // violet
];

pub fn player_color(player: u16) -> u32 {
    PLAYER_COLORS[player as usize % PLAYER_COLORS.len()]
}

/// How strongly an owner shows through the region's own colour.
pub const OWNER_TINT: f64 = 0.55;

pub fn region_color(index: u8) -> u32 {
    REGION_COLORS[index as usize % REGION_COLORS.len()]
}

pub const fn split(color: u32) -> (u32, u32, u32) {
    ((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF)
}

pub const fn join(red: u32, green: u32, blue: u32) -> u32 {
    (red << 16) | (green << 8) | blue
}

/// Mixes two colors. `amount` of 0 gives `from`, 1 gives `to`.
pub fn mix(from: u32, to: u32, amount: f64) -> u32 {
    let amount = amount.clamp(0.0, 1.0);
    let (fr, fg, fb) = split(from);
    let (tr, tg, tb) = split(to);
    let channel = |a: u32, b: u32| (a as f64 + (b as f64 - a as f64) * amount).round() as u32;
    join(channel(fr, tr), channel(fg, tg), channel(fb, tb))
}

/// Fades a color toward the background, for a duplicate of an already-drawn region.
pub fn dimmed(color: u32) -> u32 {
    mix(BACKGROUND, color, DUPLICATE_STRENGTH)
}

/// Brightens a color, for the region under the cursor.
pub fn highlighted(color: u32) -> u32 {
    mix(color, 0xFFFFFF, 0.30)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luma(color: u32) -> f64 {
        let (r, g, b) = split(color);
        0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64
    }

    #[test]
    fn mix_hits_both_ends() {
        assert_eq!(mix(0x102030, 0xA0B0C0, 0.0), 0x102030);
        assert_eq!(mix(0x102030, 0xA0B0C0, 1.0), 0xA0B0C0);
    }

    #[test]
    fn dimming_always_darkens() {
        for color in REGION_COLORS {
            assert!(
                luma(dimmed(color)) < luma(color),
                "{color:06X} did not darken"
            );
        }
    }

    #[test]
    fn highlighting_always_brightens() {
        for color in REGION_COLORS {
            assert!(luma(highlighted(color)) > luma(color));
        }
    }

    /// The property the palette exists to have: the four colors a planar graph
    /// actually needs must be separable without any hue information at all.
    #[test]
    fn the_four_working_colors_are_readable_in_grayscale() {
        let mut lumas: Vec<f64> = REGION_COLORS[..4].iter().map(|&c| luma(c)).collect();
        lumas.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for pair in lumas.windows(2) {
            assert!(
                pair[1] - pair[0] > 25.0,
                "lightness steps too small: {lumas:?}"
            );
        }
    }

    /// The duplicate cue is read per region — same hue, obviously darker — not by
    /// absolute brightness. It has to be, because a palette that spans a wide
    /// lightness range for grayscale readability will inevitably dim a pale color to
    /// something still brighter than a dark one at full strength. So the property
    /// worth asserting is that every color separates clearly from *itself*.
    #[test]
    fn every_color_separates_clearly_from_its_own_dimmed_form() {
        for color in REGION_COLORS {
            let gap = luma(color) - luma(dimmed(color));
            assert!(gap > 25.0, "{color:06X} only drops {gap:.1} when dimmed");
        }
    }
}
