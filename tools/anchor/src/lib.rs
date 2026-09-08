//! Find a piece of text in a file without caring how it was wrapped, and replace it.
//!
//! **`C-55`.** `CLAUDE.md` states two rules that fire at a moment of confidence - when an edit
//! looks obvious - and neither had anything but attention behind it:
//!
//! - **Normalize both sides before comparing them**, rather than choosing a match string
//!   carefully. A string drafted as one sentence meets a file that wrapped it, matches nothing,
//!   and `str::replace` with no match is a no-op rather than an error.
//! - **Write a script to a file before running it**, because a file has one level of quoting.
//!
//! **A rule that fires at a moment of confidence needs a carrier rather than a better
//! sentence** - `P-327`. This is the carrier. It takes the anchor and the replacement as
//! *files*, which is the second rule made structural rather than remembered, and it compares
//! them normalized, which is the first.
//!
//! # What it refuses
//!
//! **Nothing silent.** An anchor that matches nothing is an error, and so is one that matches
//! twice - the case where taking the first is how the wrong line gets edited. The failure a
//! carrier exists to prevent is the edit that quietly does nothing, so doing nothing is never
//! a success here.

/// The text, with every run of whitespace collapsed to one space and the ends trimmed.
///
/// **Wrapping is the difference this exists to ignore.** A sentence written on one line and
/// the same sentence broken across two are the same words and different bytes, and every
/// failure `CLAUDE.md` records of this kind is that difference.
pub fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Where the anchor sits in the text, as a byte range, ignoring how either was wrapped.
///
/// **The offset is mapped back to the original.** Comparing normalized text and then editing
/// normalized text would rewrite the whole file's whitespace; this finds the match in the
/// normalized form and returns the bytes of the original that produced it.
///
/// **`strip` is folded into the same scan rather than applied first.** Removing a marker from
/// the text and then searching it would give offsets into a string that is not the file, and
/// mapping those back is a second map to get wrong. Here a marker is simply not emitted, so
/// every position still points into the original bytes.
pub fn find(text: &str, anchor: &str, strip: Option<&str>) -> Result<(usize, usize), Problem> {
    let wanted = collapse(anchor);
    if wanted.is_empty() {
        return Err(Problem::EmptyAnchor);
    }

    // `at[i]` is the byte offset in `text` of the character that begins normalized position
    // `i`, and `ends[i]` is one past the byte that ends it.
    let mut normalized = String::new();
    let mut at: Vec<usize> = Vec::new();
    let mut ends: Vec<usize> = Vec::new();

    for line in text.split_inclusive('\n') {
        // Where this line begins in the whole text.
        let line_at = line.as_ptr() as usize - text.as_ptr() as usize;
        let mut content = line;
        let mut content_at = line_at;
        if let Some(marker) = strip {
            let after_space = content.trim_start();
            let indent = content.len() - after_space.len();
            if let Some(rest) = after_space.strip_prefix(marker) {
                content_at = line_at + indent + marker.len();
                content = rest;
            }
        }
        for (offset, character) in content.char_indices() {
            let here = content_at + offset;
            if character.is_whitespace() {
                if !normalized.is_empty() && !normalized.ends_with(' ') {
                    normalized.push(' ');
                    at.push(here);
                    ends.push(here + character.len_utf8());
                }
                continue;
            }
            normalized.push(character);
            at.push(here);
            ends.push(here + character.len_utf8());
        }
    }
    while normalized.ends_with(' ') {
        normalized.pop();
        at.pop();
        ends.pop();
    }

    let found: Vec<usize> = normalized.match_indices(&wanted).map(|(i, _)| i).collect();
    match found.len() {
        0 => Err(Problem::NotFound),
        1 => {
            let start = found[0];
            // Byte positions in the normalized string are character positions only if every
            // character is one byte, so the map is indexed by counting characters up to the
            // match rather than by the byte offset itself.
            let from_char = normalized[..start].chars().count();
            let to_char = from_char + wanted.chars().count();
            Ok((at[from_char], ends[to_char - 1]))
        }
        n => Err(Problem::Ambiguous(n)),
    }
}

/// The text with the anchor replaced, verbatim.
pub fn replace(
    text: &str,
    anchor: &str,
    with: &str,
    strip: Option<&str>,
) -> Result<String, Problem> {
    let (from, to) = find(text, anchor, strip)?;
    let mut out = String::with_capacity(text.len());
    out.push_str(&text[..from]);
    out.push_str(with);
    out.push_str(&text[to..]);
    Ok(out)
}

/// Why an edit was refused. **Every one of these is a failure**, including finding nothing.
#[derive(Debug, PartialEq, Eq)]
pub enum Problem {
    /// The anchor is not in the file. The edit this would have made is the silent no-op.
    NotFound,
    /// The anchor is in the file more than once. Taking the first is how the wrong line is
    /// edited, so this refuses instead of choosing.
    Ambiguous(usize),
    /// An anchor of no words matches everywhere and means nothing.
    EmptyAnchor,
}

impl std::fmt::Display for Problem {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Problem::NotFound => write!(
                out,
                "the anchor is not in the file, wrapping ignored - nothing was changed"
            ),
            Problem::Ambiguous(n) => write!(
                out,
                "the anchor matches {n} places, so this refuses rather than taking the first"
            ),
            Problem::EmptyAnchor => write!(out, "the anchor has no words in it"),
        }
    }
}
