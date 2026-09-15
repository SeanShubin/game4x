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

/// Several edits, from one file, applied in order.
///
/// **`C-129`. The carrier lost to the failure it prevents, because it was harder to use.** An edit
/// in three parts needed six files and three invocations of `replace`; a throwaway script doing
/// `str.replace` needed one file and one command. So the script won every time, and every failure
/// it is a carrier for came back - an anchor drafted against a file a formatter had since reflowed,
/// a regex assembled inside a shell string, a replacement that silently matched nothing.
///
/// **This makes the right thing the smaller thing**: one file, one command, however many edits.
///
/// # The format
///
/// **The first line is the marker**, chosen by whoever writes the file, and every section after it
/// begins `<marker> anchor` or `<marker> replace`. A marker is chosen rather than fixed so that
/// text containing one can still be edited - the same reason a heredoc takes a word.
///
/// ```text
/// @@
/// @@ anchor
/// the words to find
/// @@ replace
/// the words to put there
/// @@ anchor
/// ...
/// ```
///
/// **Each edit is applied to the result of the one before**, so an anchor may name text an earlier
/// edit wrote. Every refusal [`find`] makes applies to each in turn, and the index says which.
pub fn edits(text: &str, spec: &str, strip: Option<&str>) -> Result<String, Refused> {
    let mut lines = spec.lines();
    let marker = lines.next().unwrap_or_default().trim();
    if marker.is_empty() {
        return Err(Refused {
            at: 0,
            why: Problem::EmptyAnchor,
            what: "the first line of the edits names the marker and is empty".to_string(),
        });
    }

    // Sections, in order, each a heading and the lines under it.
    let anchor_at = format!("{marker} anchor");
    let replace_at = format!("{marker} replace");
    let mut sections: Vec<(bool, Vec<&str>)> = Vec::new();
    for line in lines {
        if line.trim_end() == anchor_at {
            sections.push((true, Vec::new()));
        } else if line.trim_end() == replace_at {
            sections.push((false, Vec::new()));
        } else if let Some((_, body)) = sections.last_mut() {
            body.push(line);
        }
    }

    if sections.is_empty() || !sections.len().is_multiple_of(2) {
        return Err(Refused {
            at: 0,
            why: Problem::NotFound,
            what: format!(
                "{} sections, and an edit is an `{anchor_at}` followed by a `{replace_at}`",
                sections.len()
            ),
        });
    }

    let mut out = text.to_string();
    for (at, pair) in sections.chunks(2).enumerate() {
        let (is_anchor, anchor) = &pair[0];
        let (is_replace, with) = &pair[1];
        if !is_anchor || *is_replace {
            return Err(Refused {
                at: at + 1,
                why: Problem::NotFound,
                what: format!("edit {} is not an anchor followed by a replacement", at + 1),
            });
        }
        let anchor = anchor.join("\n");
        let with = with.join("\n");
        out = replace(&out, &anchor, &with, strip).map_err(|why| Refused {
            at: at + 1,
            what: format!("edit {}: {}", at + 1, collapse(&anchor)),
            why,
        })?;
    }
    Ok(out)
}

/// Which edit was refused, and why.
///
/// **The index is the point.** A batch that says only *the anchor is not in the file* leaves
/// whoever wrote it to find which of six edits meant it, which is the cost that sent them back to
/// the script in the first place.
#[derive(Debug, PartialEq, Eq)]
pub struct Refused {
    pub at: usize,
    pub why: Problem,
    pub what: String,
}

impl std::fmt::Display for Refused {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "{}: {}", self.what, self.why)
    }
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
