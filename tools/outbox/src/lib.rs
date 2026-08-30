//! Reading every outbox in the repository, and answering what is open and addressed to whom.
//!
//! An outbox is the one file a perspective addresses work through. The guarantee each one
//! makes is that **if nothing in it is open, that perspective knows of nothing
//! outstanding** - a promise about the file rather than about the tree. This turns that
//! guarantee from a habit into a fact, by being able to answer the question mechanically
//! and across all of them at once.
//!
//! # Why the format is two lines and not a table
//!
//! `tools/pad-tables` rewrites column widths every commit, so a parser keyed to the bytes
//! of a table row breaks on the next pad. That is not hypothetical: thirteen rows once went
//! missing from a hand-edited table in this repository and nothing noticed. So an item is a
//! heading and a field line, and everything else in it is prose for a person.
//!
//! ```markdown
//! ### Q-3 - `planet-bevy` depends on `game-front`
//!
//! **to** code · **status** open · **raised** 2026-08-28 · **source** [report 3](...)
//! ```
//!
//! Only the two lines are parsed. The separator between fields, the order of the fields
//! after the first two, and every word of the prose are free to change.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One addressed item, from some outbox.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    pub title: String,
    /// Who is expected to act: `code`, `spec`, `sean`, or anything a future outbox invents.
    pub to: String,
    /// `open`, `acted`, `rejected`, `withdrawn`, `answered`, or likewise.
    pub status: String,
    /// Which outbox it came from, relative to the repository root.
    pub outbox: String,
}

impl Item {
    pub fn is_open(&self) -> bool {
        self.status == "open"
    }
}

/// Everything read, and where it was read from.
#[derive(Clone, Debug, Default)]
pub struct Outboxes {
    pub items: Vec<Item>,
    /// Every file that was found and read, in order.
    pub files: Vec<String>,
    /// Every place looked at that held no outbox. Reported rather than skipped in silence,
    /// because a missing outbox and an empty one are very different facts.
    pub missing: Vec<String>,
}

/// How many open items the workflow tolerates before reviewing costs as much as writing.
pub const LIMIT: usize = 15;

/// Where outboxes live.
///
/// `quality/outbox.md` is where the first lens still is; `lenses/*/outbox.md` is where the
/// workflow puts one. Both are looked for, so this works before and after that move rather
/// than needing to land in the same commit as it.
pub fn places(root: &Path) -> Vec<PathBuf> {
    let mut found = vec![
        root.join("docs/notes/proposals.md"),
        root.join("crates/outbox.md"),
        root.join("quality/outbox.md"),
    ];
    if let Ok(entries) = std::fs::read_dir(root.join("lenses")) {
        let mut lenses: Vec<PathBuf> = entries
            .flatten()
            .map(|entry| entry.path().join("outbox.md"))
            .collect();
        lenses.sort();
        found.extend(lenses);
    }
    found
}

/// Reads every outbox under a repository root.
pub fn read(root: &Path) -> Outboxes {
    let mut all = Outboxes::default();
    for path in places(root) {
        let shown = shorten(root, &path);
        match std::fs::read_to_string(&path) {
            Ok(text) => {
                all.items.extend(parse(&text, &shown));
                all.files.push(shown);
            }
            Err(_) => all.missing.push(shown),
        }
    }
    all
}

fn shorten(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Every item in one outbox's text.
///
/// A heading that is not followed by a field line is not an item - `## Open` and `## How to
/// read this` are headings too, and a document is allowed prose.
pub fn parse(text: &str, outbox: &str) -> Vec<Item> {
    let lines: Vec<&str> = text.lines().collect();
    let mut items = Vec::new();
    for (at, line) in lines.iter().enumerate() {
        let Some(heading) = line.strip_prefix("### ") else {
            continue;
        };
        let (id, title) = match heading.split_once(" - ") {
            Some((id, title)) => (id.trim(), title.trim()),
            None => (heading.trim(), ""),
        };
        // The field line is the next non-blank line, because a blank line between a heading
        // and its fields is how markdown is normally written.
        let Some(fields) = lines[at + 1..]
            .iter()
            .find(|line| !line.trim().is_empty())
            .filter(|line| line.trim_start().starts_with("**to**"))
        else {
            continue;
        };
        let Some(to) = field(fields, "to") else {
            continue;
        };
        items.push(Item {
            id: id.to_string(),
            title: title.to_string(),
            to,
            status: field(fields, "status").unwrap_or_else(|| "open".to_string()),
            outbox: outbox.to_string(),
        });
    }
    items
}

/// The word after `**name**` on a field line.
///
/// Stops at whitespace, so the separator between fields never matters - `·`, `|` and two
/// spaces all work, and changing it later breaks nothing.
fn field(line: &str, name: &str) -> Option<String> {
    let marker = format!("**{name}**");
    let after = line.split_once(&marker)?.1;
    let word = after.split_whitespace().next()?;
    let word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '-' && c != '_');
    (!word.is_empty()).then(|| word.to_string())
}

/// Open items, grouped by who has to act, addressees in alphabetical order.
pub fn open_by_addressee(items: &[Item]) -> BTreeMap<String, Vec<&Item>> {
    let mut grouped: BTreeMap<String, Vec<&Item>> = BTreeMap::new();
    for item in items.iter().filter(|item| item.is_open()) {
        grouped.entry(item.to.clone()).or_default().push(item);
    }
    grouped
}

/// Ids used more than once, with where each use was.
///
/// Worth asserting rather than assuming: a duplicated id is how a status silently stops
/// meaning anything, because a commit citing it no longer says which item it closed.
pub fn duplicate_ids(items: &[Item]) -> BTreeMap<String, Vec<String>> {
    let mut seen: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for item in items {
        seen.entry(item.id.clone())
            .or_default()
            .push(item.outbox.clone());
    }
    seen.retain(|_, wheres| wheres.len() > 1);
    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
# An outbox

## How to read this

Prose, and a heading that is not an item.

## Open

### Q-1 - The palette exists in three places

**to** code · **status** open · **raised** 2026-08-28 · **source** [report 1](x.md)

One line of what it is.

### Q-13 - Adopt the workflow

**to** spec · **status** open · **raised** 2026-08-30

## Resolved

### Q-16 - The picture never sees the biome

**to** code · **status** withdrawn · **raised** 2026-08-29
";

    #[test]
    fn an_item_is_a_heading_and_a_field_line() {
        let items = parse(SAMPLE, "quality/outbox.md");
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].id, "Q-1");
        assert_eq!(items[0].title, "The palette exists in three places");
        assert_eq!(items[0].to, "code");
        assert_eq!(items[0].status, "open");
        assert_eq!(items[0].outbox, "quality/outbox.md");
    }

    /// A document is allowed prose and section headings. Only a heading followed by a field
    /// line is an item.
    #[test]
    fn a_heading_without_fields_is_not_an_item() {
        let items = parse(SAMPLE, "x.md");
        assert!(items.iter().all(|item| item.id.starts_with("Q-")));
    }

    #[test]
    fn only_open_items_are_outstanding() {
        let items = parse(SAMPLE, "x.md");
        let open = open_by_addressee(&items);
        assert_eq!(open["code"].len(), 1, "the withdrawn one is not open");
        assert_eq!(open["spec"].len(), 1);
        assert_eq!(open.len(), 2);
    }

    /// The separator between fields is not part of the format. `tools/pad-tables` and a
    /// later change of taste must both leave the parser alone.
    #[test]
    fn the_separator_between_fields_does_not_matter() {
        for separator in ["·", "|", "  ", " - "] {
            let text = format!("### Q-9 - A title\n\n**to** code {separator} **status** open\n");
            let items = parse(&text, "x.md");
            assert_eq!(items.len(), 1, "with separator {separator:?}");
            assert_eq!(items[0].to, "code");
            assert_eq!(items[0].status, "open");
        }
    }

    /// A field line right under the heading, with no blank line, is the same item.
    #[test]
    fn a_blank_line_between_heading_and_fields_is_optional() {
        let text = "### Q-2 - A title\n**to** sean · **status** open\n";
        assert_eq!(parse(text, "x.md").len(), 1);
    }

    #[test]
    fn a_duplicated_id_is_reported_with_both_homes() {
        let mut items = parse(SAMPLE, "quality/outbox.md");
        items.extend(parse(SAMPLE, "lenses/second/outbox.md"));
        let duplicates = duplicate_ids(&items);
        assert_eq!(duplicates.len(), 3);
        assert_eq!(
            duplicates["Q-1"],
            ["quality/outbox.md", "lenses/second/outbox.md"]
        );
    }

    #[test]
    fn distinct_ids_are_not_duplicates() {
        assert!(duplicate_ids(&parse(SAMPLE, "x.md")).is_empty());
    }

    /// Both homes for a lens's outbox are looked for, so this works before and after the
    /// move the workflow describes.
    #[test]
    fn it_looks_where_a_lens_is_and_where_one_is_going() {
        let looked: Vec<String> = places(Path::new("/root"))
            .iter()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
            .collect();
        assert!(looked.iter().any(|at| at.ends_with("quality/outbox.md")));
        assert!(
            looked
                .iter()
                .any(|at| at.ends_with("docs/notes/proposals.md"))
        );
        assert!(looked.iter().any(|at| at.ends_with("crates/outbox.md")));
    }
}
