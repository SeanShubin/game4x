//! A 5x7 bitmap font, so the prototype can label regions and show a readout without
//! taking on a text rendering dependency.
//!
//! Glyphs are written as seven rows of five characters, `1` for ink, joined with `/`.

use std::collections::HashMap;
use std::sync::OnceLock;

pub const GLYPH_WIDTH: usize = 5;
pub const GLYPH_HEIGHT: usize = 7;
const SPACING: usize = 1;

const GLYPHS: &[(char, &str)] = &[
    ('0', "01110/10001/10011/10101/11001/10001/01110"),
    ('1', "00100/01100/00100/00100/00100/00100/01110"),
    ('2', "01110/10001/00001/00010/00100/01000/11111"),
    ('3', "11111/00010/00100/00010/00001/10001/01110"),
    ('4', "00010/00110/01010/10010/11111/00010/00010"),
    ('5', "11111/10000/11110/00001/00001/10001/01110"),
    ('6', "00110/01000/10000/11110/10001/10001/01110"),
    ('7', "11111/00001/00010/00100/01000/01000/01000"),
    ('8', "01110/10001/10001/01110/10001/10001/01110"),
    ('9', "01110/10001/10001/01111/00001/00010/01100"),
    ('A', "01110/10001/10001/11111/10001/10001/10001"),
    ('B', "11110/10001/10001/11110/10001/10001/11110"),
    ('C', "01110/10001/10000/10000/10000/10001/01110"),
    ('D', "11100/10010/10001/10001/10001/10010/11100"),
    ('E', "11111/10000/10000/11110/10000/10000/11111"),
    ('F', "11111/10000/10000/11110/10000/10000/10000"),
    ('G', "01110/10001/10000/10111/10001/10001/01111"),
    ('H', "10001/10001/10001/11111/10001/10001/10001"),
    ('I', "01110/00100/00100/00100/00100/00100/01110"),
    ('J', "00111/00010/00010/00010/00010/10010/01100"),
    ('K', "10001/10010/10100/11000/10100/10010/10001"),
    ('L', "10000/10000/10000/10000/10000/10000/11111"),
    ('M', "10001/11011/10101/10101/10001/10001/10001"),
    ('N', "10001/10001/11001/10101/10011/10001/10001"),
    ('O', "01110/10001/10001/10001/10001/10001/01110"),
    ('P', "11110/10001/10001/11110/10000/10000/10000"),
    ('Q', "01110/10001/10001/10001/10101/10010/01101"),
    ('R', "11110/10001/10001/11110/10100/10010/10001"),
    ('S', "01111/10000/10000/01110/00001/00001/11110"),
    ('T', "11111/00100/00100/00100/00100/00100/00100"),
    ('U', "10001/10001/10001/10001/10001/10001/01110"),
    ('V', "10001/10001/10001/10001/10001/01010/00100"),
    ('W', "10001/10001/10001/10101/10101/11011/10001"),
    ('X', "10001/10001/01010/00100/01010/10001/10001"),
    ('Y', "10001/10001/01010/00100/00100/00100/00100"),
    ('Z', "11111/00001/00010/00100/01000/10000/11111"),
    ('.', "00000/00000/00000/00000/00000/01100/01100"),
    (',', "00000/00000/00000/00000/01100/00100/01000"),
    ('-', "00000/00000/00000/11111/00000/00000/00000"),
    ('+', "00000/00100/00100/11111/00100/00100/00000"),
    (':', "00000/01100/01100/00000/01100/01100/00000"),
    ('/', "00001/00010/00010/00100/01000/01000/10000"),
    ('(', "00010/00100/01000/01000/01000/00100/00010"),
    (')', "01000/00100/00010/00010/00010/00100/01000"),
    ('[', "01110/01000/01000/01000/01000/01000/01110"),
    (']', "01110/00010/00010/00010/00010/00010/01110"),
    ('<', "00010/00100/01000/10000/01000/00100/00010"),
    ('>', "01000/00100/00010/00001/00010/00100/01000"),
    ('=', "00000/00000/11111/00000/11111/00000/00000"),
    ('#', "01010/01010/11111/01010/11111/01010/01010"),
    ('%', "11001/11010/00010/00100/01000/01011/10011"),
    ('*', "00000/10101/01110/11111/01110/10101/00000"),
    ('?', "01110/10001/00001/00010/00100/00000/00100"),
    ('!', "00100/00100/00100/00100/00100/00000/00100"),
    ('\'', "00100/00100/00000/00000/00000/00000/00000"),
];

type Glyph = [u8; GLYPH_HEIGHT];

fn table() -> &'static HashMap<char, Glyph> {
    static TABLE: OnceLock<HashMap<char, Glyph>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut table = HashMap::new();
        for &(character, rows) in GLYPHS {
            let mut glyph = [0u8; GLYPH_HEIGHT];
            for (index, row) in rows.split('/').enumerate() {
                let mut bits = 0u8;
                for (column, cell) in row.chars().enumerate() {
                    if cell == '1' {
                        bits |= 1 << column;
                    }
                }
                glyph[index] = bits;
            }
            table.insert(character, glyph);
        }
        table
    })
}

pub fn text_width(text: &str, scale: usize) -> usize {
    let characters = text.chars().count();
    if characters == 0 {
        0
    } else {
        (characters * (GLYPH_WIDTH + SPACING) - SPACING) * scale
    }
}

pub fn text_height(scale: usize) -> usize {
    GLYPH_HEIGHT * scale
}

/// Draws text with its top-left corner at (x, y). Off-screen pixels are clipped.
pub fn draw_text(
    buffer: &mut [u32],
    buffer_width: usize,
    buffer_height: usize,
    x: i64,
    y: i64,
    text: &str,
    color: u32,
    scale: usize,
) {
    let glyphs = table();
    let mut pen = x;
    for character in text.to_ascii_uppercase().chars() {
        if let Some(glyph) = glyphs.get(&character) {
            for (row, bits) in glyph.iter().enumerate() {
                for column in 0..GLYPH_WIDTH {
                    if bits & (1 << column) == 0 {
                        continue;
                    }
                    fill(
                        buffer,
                        buffer_width,
                        buffer_height,
                        pen + (column * scale) as i64,
                        y + (row * scale) as i64,
                        scale,
                        color,
                    );
                }
            }
        }
        pen += ((GLYPH_WIDTH + SPACING) * scale) as i64;
    }
}

/// Draws text with a one-pixel drop shadow, so labels stay legible over any fill.
pub fn draw_label(
    buffer: &mut [u32],
    buffer_width: usize,
    buffer_height: usize,
    x: i64,
    y: i64,
    text: &str,
    color: u32,
    shadow: u32,
    scale: usize,
) {
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        draw_text(
            buffer,
            buffer_width,
            buffer_height,
            x + dx * scale as i64,
            y + dy * scale as i64,
            text,
            shadow,
            scale,
        );
    }
    draw_text(buffer, buffer_width, buffer_height, x, y, text, color, scale);
}

fn fill(
    buffer: &mut [u32],
    buffer_width: usize,
    buffer_height: usize,
    x: i64,
    y: i64,
    size: usize,
    color: u32,
) {
    for dy in 0..size as i64 {
        for dx in 0..size as i64 {
            let px = x + dx;
            let py = y + dy;
            if px >= 0 && py >= 0 && (px as usize) < buffer_width && (py as usize) < buffer_height {
                buffer[py as usize * buffer_width + px as usize] = color;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_glyph_is_well_formed() {
        for &(character, rows) in GLYPHS {
            let lines: Vec<&str> = rows.split('/').collect();
            assert_eq!(lines.len(), GLYPH_HEIGHT, "{character} has wrong row count");
            for line in lines {
                assert_eq!(line.len(), GLYPH_WIDTH, "{character} has a wrong row width");
                assert!(line.chars().all(|c| c == '0' || c == '1'));
            }
        }
    }

    #[test]
    fn glyphs_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for &(character, _) in GLYPHS {
            assert!(seen.insert(character), "{character} appears twice");
        }
    }

    #[test]
    fn digits_and_letters_are_all_present() {
        let glyphs = table();
        for character in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            assert!(glyphs.contains_key(&character), "missing {character}");
        }
    }

    #[test]
    fn drawing_stays_inside_the_buffer() {
        let (width, height) = (40, 12);
        let mut buffer = vec![0u32; width * height];
        // Deliberately draw off every edge.
        draw_text(&mut buffer, width, height, -20, -5, "HELLO", 0xFFFFFF, 1);
        draw_text(&mut buffer, width, height, 35, 8, "WORLD", 0xFFFFFF, 2);
        assert_eq!(buffer.len(), width * height);
    }

    #[test]
    fn text_actually_marks_pixels() {
        let (width, height) = (60, 10);
        let mut buffer = vec![0u32; width * height];
        draw_text(&mut buffer, width, height, 1, 1, "A1", 0xFFFFFF, 1);
        assert!(buffer.iter().any(|&pixel| pixel == 0xFFFFFF));
    }

    #[test]
    fn width_matches_what_gets_drawn() {
        assert_eq!(text_width("", 1), 0);
        assert_eq!(text_width("A", 1), GLYPH_WIDTH);
        assert_eq!(text_width("AB", 1), GLYPH_WIDTH * 2 + 1);
        assert_eq!(text_width("AB", 2), (GLYPH_WIDTH * 2 + 1) * 2);
    }
}
