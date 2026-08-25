//! Markdown table padding - align every table's columns so pipes line up in monospace.
//!
//! [`pad_tables`] is the single source of truth for this repository's table style. It is a
//! pure `&str -> String` transform so it can be unit tested and, if documentation is ever
//! generated from code, called by the generator directly. A generator that emits
//! already-padded tables and a walker that finds nothing to change cannot fight each other.
//!
//! The transform is idempotent: `pad_tables(&pad_tables(s)) == pad_tables(s)`, asserted in
//! the tests below.
//!
//! The padding algorithm is unchanged from `seans-arcade/examples/pad_tables.rs`, so every
//! repository using it formats tables identically.

/// Pad every markdown table in `content` so its columns align.
/// Lines that are not part of a table are returned untouched.
pub fn pad_tables(content: &str) -> String {
    let lines: Vec<&str> = content.split('\n').collect();
    let mut result: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        if parse_row(lines[i]).is_some() {
            let mut table_lines = Vec::new();
            while i < lines.len() && parse_row(lines[i]).is_some() {
                table_lines.push(lines[i]);
                i += 1;
            }
            // A run of pipe-delimited lines is only a table if the second row separates.
            if table_lines.len() >= 2 && is_separator(&parse_row(table_lines[1]).unwrap()) {
                result.extend(pad_table(&table_lines));
            } else {
                result.extend(table_lines.iter().map(|s| s.to_string()));
            }
        } else {
            result.push(lines[i].to_string());
            i += 1;
        }
    }

    result.join("\n")
}

/// Parse a markdown table row into trimmed cell contents.
/// Returns `None` if the line isn't a table row.
fn parse_row(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') || trimmed.len() < 2 {
        return None;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    let cells: Vec<String> = inner.split('|').map(|c| c.trim().to_string()).collect();
    Some(cells)
}

/// Check if all cells in a row are separator patterns like `---`, `:---`, `---:`, `:---:`.
fn is_separator(cells: &[String]) -> bool {
    if cells.is_empty() {
        return false;
    }
    cells.iter().all(|c| {
        let mut chars = c.chars();
        // Must have at least one character
        let first = match chars.next() {
            Some(ch) => ch,
            None => return false,
        };
        // Strip optional leading colon
        let rest_start = if first == ':' {
            match chars.next() {
                Some(ch) => ch,
                None => return false, // just ":"
            }
        } else {
            first
        };
        // Must have at least one dash
        if rest_start != '-' {
            return false;
        }
        // Remaining chars: dashes, then optional trailing colon
        let mut saw_colon = false;
        for ch in chars {
            if saw_colon {
                return false; // something after the trailing colon
            }
            if ch == '-' {
                continue;
            } else if ch == ':' {
                saw_colon = true;
            } else {
                return false;
            }
        }
        true
    })
}

/// Visual width of a string - count chars, not bytes.
/// All characters in this project's docs are single-width in Western monospace fonts.
fn visual_width(s: &str) -> usize {
    s.chars().count()
}

/// Format a separator cell preserving alignment markers (`:---`, `---:`, `:---:`).
fn format_separator_cell(original: &str, width: usize) -> String {
    let left = original.starts_with(':');
    let right = original.ends_with(':');
    let colon_width = if left { 1 } else { 0 } + if right { 1 } else { 0 };
    let dash_count = if width > colon_width {
        width - colon_width
    } else {
        1
    };
    let mut s = String::with_capacity(width);
    if left {
        s.push(':');
    }
    for _ in 0..dash_count {
        s.push('-');
    }
    if right {
        s.push(':');
    }
    s
}

/// Pad a table so all columns align.
fn pad_table(lines: &[&str]) -> Vec<String> {
    // Parse all rows
    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in lines {
        match parse_row(line) {
            Some(cells) => rows.push(cells),
            None => return lines.iter().map(|s| s.to_string()).collect(),
        }
    }
    if rows.len() < 2 {
        return lines.iter().map(|s| s.to_string()).collect();
    }

    // Normalize column count
    let max_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    for row in &mut rows {
        while row.len() < max_cols {
            row.push(String::new());
        }
    }

    // Calculate max width per column (skip separator row)
    let mut col_widths = vec![0usize; max_cols];
    for (i, row) in rows.iter().enumerate() {
        if i == 1 && is_separator(row) {
            continue;
        }
        for (j, cell) in row.iter().enumerate() {
            col_widths[j] = col_widths[j].max(visual_width(cell));
        }
    }

    // Minimum width of 3 so separators are at least `---`
    for w in &mut col_widths {
        if *w < 3 {
            *w = 3;
        }
    }

    // Format each row
    let mut result = Vec::with_capacity(rows.len());
    for (i, row) in rows.iter().enumerate() {
        let mut line = String::from("|");
        if i == 1 && is_separator(row) {
            for (j, cell) in row.iter().enumerate() {
                line.push(' ');
                line.push_str(&format_separator_cell(cell, col_widths[j]));
                line.push(' ');
                line.push('|');
            }
        } else {
            for (j, cell) in row.iter().enumerate() {
                line.push(' ');
                line.push_str(cell);
                let padding = col_widths[j] - visual_width(cell);
                for _ in 0..padding {
                    line.push(' ');
                }
                line.push(' ');
                line.push('|');
            }
        }
        result.push(line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::pad_tables;

    const RAGGED: &str = "| a | bbbb |\n| --- | --- |\n| cccccc | d |\n";

    #[test]
    fn columns_align() {
        let out = pad_tables(RAGGED);
        let widths: Vec<usize> = out.lines().map(|l| l.chars().count()).collect();
        assert!(
            widths.windows(2).all(|w| w[0] == w[1]),
            "rows differ in width:\n{out}"
        );
    }

    #[test]
    fn idempotent() {
        let once = pad_tables(RAGGED);
        assert_eq!(pad_tables(&once), once);
    }

    /// Colons are kept, and they come out of the dash budget rather than widening the
    /// column - so at the minimum width of 3, `:---` correctly becomes `:--`.
    #[test]
    fn alignment_markers_survive() {
        let out = pad_tables("| a | b | c |\n| :--- | ---: | :---: |\n| 1 | 2 | 3 |\n");
        let sep = out.lines().nth(1).unwrap();
        let cells: Vec<&str> = sep
            .trim()
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim())
            .collect();
        assert_eq!(cells, vec![":--", "--:", ":-:"], "markers lost: {sep}");
    }

    #[test]
    fn prose_is_untouched() {
        let text = "# Title\n\nA sentence with a | pipe in it.\n";
        assert_eq!(pad_tables(text), text);
    }

    #[test]
    fn pipe_run_without_a_separator_is_not_a_table() {
        let text = "| just |\n| some |\n";
        assert_eq!(pad_tables(text), text);
    }

    #[test]
    fn short_cells_reach_the_minimum_separator_width() {
        let out = pad_tables("| a |\n| - |\n| b |\n");
        assert_eq!(out.lines().nth(1).unwrap(), "| --- |");
    }

    #[test]
    fn missing_trailing_cells_are_filled() {
        let out = pad_tables("| a | b |\n| --- | --- |\n| c |\n");
        let last = out.lines().last().unwrap();
        assert_eq!(last.matches('|').count(), 3, "row not normalised: {last}");
    }
}
