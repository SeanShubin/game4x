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
    /// Proposals that have landed, from the queue's Accepted table.
    pub landed: Vec<Landed>,
    /// Every file that was found and read, in order.
    pub files: Vec<String>,
    /// Every place looked at that held no outbox. Reported rather than skipped in silence,
    /// because a missing outbox and an empty one are very different facts.
    pub missing: Vec<String>,
}

/// How many open items the workflow tolerates before reviewing costs as much as writing.
pub const LIMIT: usize = 15;

/// Where outboxes live.
pub fn places(root: &Path) -> Vec<PathBuf> {
    let mut found = vec![
        root.join("docs/notes/proposals.md"),
        root.join("crates/outbox.md"),
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
                if shown.ends_with("proposals.md") {
                    all.landed.extend(accepted(&text));
                }
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
        //
        // **An item without an address is not an item.** That is load-bearing rather than
        // tidy: anything whose field line does not carry `**to**` is skipped entirely, so a
        // format change that dropped the address would delete a whole outbox from this
        // index rather than report it empty - silently, because a file that parses to
        // nothing and a file with nothing in it look identical from here. The next person
        // to change an outbox's format needs to know that before they do it.
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

/// A proposal that has landed: where it went, and when.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Landed {
    pub id: String,
    pub destination: String,
    pub date: String,
}

/// Several proposals that landed in the same section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SameSection {
    pub destination: String,
    /// Every proposal that landed there, oldest first.
    pub proposals: Vec<String>,
}

/// The accepted proposals, from the queue's own table.
///
/// # Parsing a table after saying not to
///
/// The item format is two lines precisely so that `tools/pad-tables` cannot break it. This
/// reads a table anyway, and the distinction is worth stating rather than glossing: padding
/// changes the **whitespace between cells**, never the cells. Splitting on `|` and trimming
/// is therefore pad-proof, where matching the bytes of a row is not. What is forbidden is a
/// parser keyed to a row's exact width, and thirteen rows once went missing that way.
///
/// The queue is not this lane's file to reshape, so it is read as it is.
pub fn accepted(text: &str) -> Vec<Landed> {
    let mut landed = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            inside = heading.trim() == "Accepted";
            continue;
        }
        if !inside || !line.trim_start().starts_with('|') {
            continue;
        }
        let cells: Vec<String> = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect();
        if cells.len() < 3 {
            continue;
        }
        // The header and its rule are rows too, and neither names a proposal.
        let Some(id) = cells[0].split(',').next().map(str::trim) else {
            continue;
        };
        if !id.starts_with("P-") {
            continue;
        }
        landed.push(Landed {
            id: id.to_string(),
            destination: one_arrow(&cells[1]),
            date: cells[2].clone(),
        });
    }
    landed
}

/// The same destination however the arrow was typed.
///
/// The queue writes `->` on some days and the arrow character on others - the style
/// changed partway - so without this, one section written two ways is two sections, and a
/// group that should fire is split into two that do not.
fn one_arrow(destination: &str) -> String {
    destination.replace('→', "->")
}

/// Where more than one proposal landed in the same section on the same day.
///
/// The trigger behind the rule Sean decided: *when a proposal lands in a section another
/// proposal has already landed in, re-read that section whole and ask whether all of them
/// can hold at once.* A rule with nothing emitting its condition is a duty somebody has to
/// remember, and every hand-held duty in this repository has rotted.
///
/// **Not scoped to a day, deliberately.** It was, and that was wrong: a contradiction is
/// not scoped to a day either. `P-100` and `P-109` happened to land together so it fired,
/// but had `P-109` come a week later the collision would be identical and the flag silent.
/// Nothing about two proposals contradicting each other depends on their arriving together.
///
/// It is a **prompt to re-read, not a defect**. Six proposals in one section is ordinary -
/// that is what working on one topic looks like. What it cannot tell you is whether they
/// all still hold together, and that is the question it exists to ask.
///
/// The threshold is more than one, not more than two. Fitting it to the two collisions
/// actually seen would be overfitting, and the costs are wildly asymmetric: a false fire
/// costs re-reading ten bullets, and the contradiction that was missed cost two days.
pub fn same_section(landed: &[Landed]) -> Vec<SameSection> {
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for one in landed {
        grouped
            .entry(one.destination.clone())
            .or_default()
            .push(one.id.clone());
    }
    let mut flags: Vec<SameSection> = grouped
        .into_iter()
        .filter(|(_, proposals)| proposals.len() > 1)
        .map(|(destination, proposals)| SameSection {
            destination,
            proposals,
        })
        .collect();
    // Busiest first: the section most worth re-reading is the one most was said about.
    flags.sort_by(|a, b| {
        b.proposals
            .len()
            .cmp(&a.proposals.len())
            .then(a.destination.cmp(&b.destination))
    });
    flags
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

    /// The flag fires where the lens said it would, against the queue's real history.
    ///
    /// Three groups on 2026-08-28, and the third is the one that matters: `P-100` and
    /// `P-109` landed in the same section on the same day and contradict each other. That
    /// is the collision the whole trigger exists to have caught, and this is the evidence
    /// that it would have.
    #[test]
    fn the_flag_fires_on_the_collision_that_happened() {
        let queue = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/notes/proposals.md"),
        )
        .expect("the proposal queue is where the workflow says it is");
        let flags = same_section(&accepted(&queue));

        let group = |destination: &str| {
            flags
                .iter()
                .find(|flag| flag.destination == destination)
                .unwrap_or_else(|| panic!("no group for {destination}"))
        };

        // The collision the trigger exists for: two proposals that contradict each other,
        // in one section. They landed on one day, but the flag must not need them to.
        let carries = group("`spec/planet.md` -> What a territory carries");
        assert!(carries.proposals.contains(&"P-100".to_string()));
        assert!(carries.proposals.contains(&"P-109".to_string()));

        // And the other two sections the lens named, the busiest in the queue.
        assert!(group("`spec/planet.md` -> Presentation").proposals.len() >= 6);
        assert!(
            group("`spec/invariants.md` -> Control without tedium")
                .proposals
                .len()
                >= 5
        );
    }

    /// A contradiction is not scoped to a day, so neither is the flag.
    ///
    /// This is the case the date-scoped version missed completely: two proposals in one
    /// section, a week apart. Had `P-109` arrived a week after `P-100`, the collision would
    /// have been identical and nothing would have said so.
    #[test]
    fn two_proposals_a_week_apart_in_one_section_still_flag() {
        let landed = vec![
            Landed {
                id: "P-100".to_string(),
                destination: "`spec/planet.md` -> What a territory carries".to_string(),
                date: "2026-08-28".to_string(),
            },
            Landed {
                id: "P-109".to_string(),
                destination: "`spec/planet.md` -> What a territory carries".to_string(),
                date: "2026-09-04".to_string(),
            },
        ];
        let flags = same_section(&landed);
        assert_eq!(flags.len(), 1, "a week apart is still the same section");
        assert_eq!(flags[0].proposals, ["P-100", "P-109"]);
    }

    /// One section written two ways is one section. The queue uses both arrows, because the
    /// style changed partway through.
    #[test]
    fn the_arrow_style_does_not_split_a_section() {
        let landed = vec![
            Landed {
                id: "P-1".to_string(),
                destination: one_arrow("`spec/planet.md` → Shape"),
                date: "2026-08-25".to_string(),
            },
            Landed {
                id: "P-6".to_string(),
                destination: one_arrow("`spec/planet.md` -> Shape"),
                date: "2026-08-25".to_string(),
            },
        ];
        let flags = same_section(&landed);
        assert_eq!(flags.len(), 1, "two arrows made two sections");
        assert_eq!(flags[0].proposals, ["P-1", "P-6"]);
    }

    /// A single proposal in a section is not a flag. The trigger is a collision between
    /// proposals, not activity in a file.
    #[test]
    fn one_proposal_in_a_section_is_not_flagged() {
        let landed = vec![Landed {
            id: "P-1".to_string(),
            destination: "`spec/planet.md` -> Shape".to_string(),
            date: "2026-08-25".to_string(),
        }];
        assert!(same_section(&landed).is_empty());
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
    /// Every producer's outbox is looked for by name, and every lens's by walking.
    #[test]
    fn it_looks_where_every_outbox_lives() {
        let looked: Vec<String> = places(Path::new("/root"))
            .iter()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
            .collect();
        assert!(
            looked
                .iter()
                .any(|at| at.ends_with("docs/notes/proposals.md"))
        );
        assert!(looked.iter().any(|at| at.ends_with("crates/outbox.md")));
        // The lens has moved under `lenses/`, so the pre-move path is gone rather than
        // probed - a completed move was reading as a missing file.
        assert!(!looked.iter().any(|at| at.ends_with("/quality/outbox.md")));
    }
}
