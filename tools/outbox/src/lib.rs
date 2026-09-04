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
//! # What is actually forbidden, since this file parses a table anyway
//!
//! `CLAUDE.md` states the rule as a prohibition - never put a table row in a match string -
//! and says neither why nor what to do instead. Both are worth having, because people keep
//! hitting the wall it describes:
//!
//! **Padding changes the whitespace between cells. It never changes the cells.** So
//! splitting a row on `|` and trimming each piece is pad-proof, while matching a row's
//! bytes is not. What is forbidden is depending on a row's *width*, not on a row.
//!
//! [`accepted`] reads the proposal queue that way, because the queue is not this lane's
//! file to reshape and reading it as it is beats asking somebody to change it.
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
    /// Everything under the heading, up to the next item.
    ///
    /// Kept so that a reader wanting more than the fields does not have to find the item
    /// again by hand. `S-11`: `tools/spec` needs the text a proposal proposes, and two
    /// things disagreeing about where a proposal's body ends would be worse than either
    /// parsing alone - so the body is cut once, here.
    pub body: String,
    /// Commits that cite this id and were read without closing it.
    ///
    /// A citation usually means the item was settled, and sometimes it means the commit
    /// answered part of it, or cited it while doing something else. Recording the hash is
    /// how an author says *I looked* - without it the reconciliation below would report the
    /// same item forever, and a signal that always fires is one nobody reads.
    pub cited: Vec<String>,
}

/// The statuses that mean somebody still has to do something.
///
/// **`built` is one of them, and leaving it out is how the index went quiet.**
/// `releases/first-release.md` gives a capability three states: `open` addressed `to code`,
/// then **`built`, addressed `to sean` - the code lane says it is done, and nobody has
/// looked**, then `vetted`. So `built` is not a finished state; it is the state of waiting
/// on a person, and it is where every capability vetted by looking will sit.
///
/// This read `status == "open"` alone, so a capability vanished from `pending.md` at exactly
/// the moment it began waiting on Sean. `CLAUDE.md` records that happening: five capabilities
/// could never move while the index reported that nothing needed deciding. The rule was
/// written into the release and never into the code that reads it.
pub const OUTSTANDING: [&str; 2] = ["open", "built"];

impl Item {
    /// Whether this item still needs somebody to act.
    ///
    /// Named for what it means rather than for one of its values: *nothing open means
    /// nothing outstanding* is the promise, and `open` is a status while outstanding is the
    /// question.
    pub fn is_outstanding(&self) -> bool {
        OUTSTANDING.contains(&self.status.as_str())
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
    // A release is an outbox too. Each capability carries an id, a `**to** code` line and
    // one observable sentence, which is the same shape as any other item - and it is the
    // work the release exists to order, so being invisible to `--to code` made the one
    // list that says what to build next the one list that did not say it.
    if let Ok(entries) = std::fs::read_dir(root.join("releases")) {
        let mut releases: Vec<PathBuf> = entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.extension().and_then(|end| end.to_str()) == Some("md"))
            .collect();
        releases.sort();
        found.extend(releases);
    }
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
        // Everything to the next heading of the same level, or to the end.
        let ends = lines[at + 1..]
            .iter()
            .position(|line| line.starts_with("### "))
            .map(|found| at + 1 + found)
            .unwrap_or(lines.len());
        items.push(Item {
            id: id.to_string(),
            title: title.to_string(),
            to,
            status: field(fields, "status").unwrap_or_else(|| "open".to_string()),
            outbox: outbox.to_string(),
            body: lines[at..ends].join(
                "
",
            ),
            cited: considered(fields),
        });
    }
    items
}

/// Why a proposal's text could not be read.
///
/// **Reported rather than guessed**, because the whole point of reading it here is that
/// there is one answer about where a proposal's body ends. A function that returned its
/// best effort would give `tools/spec` something to promote that Sean never approved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NoText {
    /// No blockquote at all: an item that proposes nothing.
    None,
    /// Several, so which one is the proposal is a guess.
    Several(usize),
}

impl std::fmt::Display for NoText {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoText::None => write!(out, "no blockquote, so it proposes no text"),
            NoText::Several(count) => {
                write!(
                    out,
                    "{count} blockquotes, so which one is proposed is a guess"
                )
            }
        }
    }
}

impl Item {
    /// The text this proposal proposes, as it would appear in the destination.
    ///
    /// **`S-11`.** `tools/spec`'s `promote` reads this, applies it, and asserts it appears
    /// once in the destination - which is what makes *approved text is shipped text*
    /// mechanical rather than asserted. The alternative was a second parser, and two things
    /// disagreeing about where a proposal's body ends would be worse than either alone.
    ///
    /// **The proposed text is the item's one blockquote.** `S-11` describes it as the
    /// blockquote between the field line and `**Basis**`, and this does not look for
    /// `**Basis**` - a blockquote is unambiguous on its own, and a marker that has to be
    /// found is a second thing to agree about. If a proposal ever carries a second
    /// blockquote for some other purpose, this reports rather than picks.
    ///
    /// The `> ` markers are stripped, because what Sean approves is the text and not its
    /// quoting.
    pub fn proposed_text(&self) -> Result<String, NoText> {
        let mut blocks: Vec<Vec<&str>> = Vec::new();
        let mut current: Vec<&str> = Vec::new();
        for line in self.body.lines() {
            let trimmed = line.trim_start();
            if let Some(rest) = trimmed.strip_prefix('>') {
                current.push(rest.strip_prefix(' ').unwrap_or(rest));
            } else if !current.is_empty() {
                blocks.push(std::mem::take(&mut current));
            }
        }
        if !current.is_empty() {
            blocks.push(current);
        }
        match blocks.len() {
            0 => Err(NoText::None),
            1 => Ok(blocks[0].join("\n").trim_end().to_string()),
            several => Err(NoText::Several(several)),
        }
    }
}

/// The word after `**name**` on a field line.
///
/// Stops at whitespace, so the separator between fields never matters - `·`, `|` and two
/// spaces all work, and changing it later breaks nothing.
/// Short hashes an item says it has already considered.
fn considered(line: &str) -> Vec<String> {
    whole_field(line, "cited")
        .into_iter()
        .flat_map(|value| {
            value
                .split([',', ' '])
                .map(|word| word.trim_matches(['`', '.', '*']).to_string())
                .filter(|word| word.len() >= 7 && word.chars().all(|c| c.is_ascii_hexdigit()))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// A field's whole value, up to the next field or the end of the line.
///
/// **`field` returns one word, and that is right for the fields that have one.** `to` and
/// `status` are single words, and stopping at whitespace is what keeps the separator
/// between fields from mattering.
///
/// `cited` is a list, and going through `field` meant it was handed one word and then split
/// on commas *and spaces* - so the space arm could never fire, and
/// `` **cited** `a1b2c3d`, `e4f5a6b` `` silently kept the first and dropped the rest. Each
/// half was right alone and together they lost data, which is why this is a second reader
/// rather than a change to the first.
///
/// Anything in the value that is not hash-shaped is ignored, so a note may sit beside the
/// hashes.
fn whole_field(line: &str, name: &str) -> Option<String> {
    let marker = format!("**{name}**");
    let after = line.split_once(&marker)?.1;
    // **Ends at the next field, not at a separator.** It used to stop at a middle dot,
    // which is what the outboxes here punctuate with - and the proposal queue punctuates
    // with ` - `, so the value ran on into whatever followed. Harmless while nothing after
    // it looked like a hash, and `**cited** `abc1234` - **source** `1234567abc`` returned
    // both. A field line is a run of `**name** value` pairs, so the next `**` ends this one
    // whatever sits between them.
    let value = after.split("**").next()?.trim();
    (!value.is_empty()).then(|| value.to_string())
}

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

/// The document that says what is pending, as markdown.
///
/// Sean: *as a human, I need one place to go to figure out what is currently pending.* A
/// command he has to remember is weaker than a document he opens, and every hand-held habit
/// in this repository has rotted while every generated artifact has held.
///
/// **Generated, never hand-edited.** That is what makes it worth trusting: derived from the
/// outboxes, it cannot disagree with them, and a hand-edited copy could. `hooks/pre-commit`
/// rewrites it beside `pad-tables`, which is what stops it going stale.
///
/// What must be decided comes first, because that is the only part addressed to him. Each
/// producer's backlog is underneath, because *what is outstanding anywhere* is the other
/// question one open document should answer.
pub fn pending(all: &Outboxes, settled: &[Unclosed]) -> String {
    let mut out = String::new();
    out.push_str("# Pending\n\n");
    out.push_str(
        "**Generated.** Written by `tools/outbox` from every outbox in the repository, and \
rewritten by\n`hooks/pre-commit`. **Do not edit this file** - it is derived, so an edit here \
is a claim that\ndisagrees with its source and loses at the next commit.\n\n",
    );
    out.push_str(&format!(
        "Read from: {}\n\n",
        all.files
            .iter()
            .map(|at| format!("`{at}`"))
            .collect::<Vec<_>>()
            .join(", ")
    ));

    let grouped = open_by_addressee(&all.items);
    let empty = Vec::new();

    out.push_str("## What must be decided\n\n");
    let sean = grouped.get("sean").unwrap_or(&empty);
    if sean.is_empty() {
        out.push_str(
            "Nothing. Every perspective has said so in its own outbox, and this is read from \
those\nfiles rather than from anybody's memory of them.\n\n",
        );
    } else {
        for item in sean {
            out.push_str(&format!(
                "- **{}** - {} · `{}`\n",
                item.id, item.title, item.outbox
            ));
        }
        out.push('\n');
    }

    if !settled.is_empty() {
        out.push_str("## Open, and a commit says otherwise\n\n");
        out.push_str(
            "An item is closed by whoever filed it and answered by somebody else, so the \
filer gets\nno signal. These are still marked `open`, and a commit that touched no part of \
their own\noutbox cites them - which usually means they were settled and nobody went back.\n\n",
        );
        for item in settled {
            out.push_str(&format!(
                "- **{}** - `{}` {} · still open in `{}`\n",
                item.id, item.hash, item.subject, item.outbox
            ));
        }
        out.push('\n');
    }

    out.push_str("## What is outstanding\n\n");
    let mut any = false;
    for (who, theirs) in &grouped {
        if who == "sean" {
            continue;
        }
        any = true;
        out.push_str(&format!("### To {who} ({})\n\n", theirs.len()));
        for item in theirs {
            out.push_str(&format!(
                "- **{}** - {} · `{}`\n",
                item.id, item.title, item.outbox
            ));
        }
        out.push('\n');
    }
    if !any {
        out.push_str("Nothing anywhere.\n\n");
    }

    out.push_str("## Sections that have taken more than one proposal\n\n");
    let flags = same_section(&all.landed);
    if flags.is_empty() {
        out.push_str("None.\n");
    } else {
        out.push_str(
            "Not a defect list. Several proposals in one section is what working on one topic \
looks\nlike; what this cannot tell you is whether they all still hold together, which is the \
question\nit exists to ask.\n\n",
        );
        for flag in &flags {
            out.push_str(&format!(
                "- {} - {}\n",
                flag.destination,
                flag.proposals.join(", ")
            ));
        }
    }
    out
}

/// One commit, as much of it as reconciliation needs.
#[derive(Clone, Debug)]
pub struct Commit {
    pub hash: String,
    pub subject: String,
    /// Every path the commit touched, relative to the repository root.
    pub touched: Vec<String>,
}

/// An item still `open` that a commit says was dealt with.
#[derive(Clone, Debug)]
pub struct Unclosed {
    pub id: String,
    pub outbox: String,
    pub hash: String,
    pub subject: String,
}

/// Items whose id a commit cites, and which nobody has closed.
///
/// **An item is closed by whoever filed it and answered by somebody else, and the filer
/// gets no signal.** `C-1`, `C-2` and `C-3` were settled by the specification lane and sat
/// here marked `open` for a day, so `pending.md` - the one document that says what is
/// waiting on Sean - named three questions that were not.
///
/// The signal already existed and nothing read it: the workflow has a producer cite the id
/// in the commit that acts on the item, and they do. This reads it.
///
/// A commit that touches the item's *own outbox* is skipped, because that is the shape of
/// filing it, mentioning it, or closing it - the citations that mean something are the ones
/// in commits about code or documents somewhere else.
pub fn unclosed(items: &[Item], commits: &[Commit]) -> Vec<Unclosed> {
    let mut found = Vec::new();
    for item in items.iter().filter(|item| item.is_outstanding()) {
        for commit in commits {
            if commit.touched.iter().any(|path| path == &item.outbox) {
                continue;
            }
            if !cites(&commit.subject, &item.id) {
                continue;
            }
            if item.cited.iter().any(|seen| commit.hash.starts_with(seen)) {
                continue;
            }
            found.push(Unclosed {
                id: item.id.clone(),
                outbox: item.outbox.clone(),
                hash: commit.hash.clone(),
                subject: commit.subject.clone(),
            });
            break;
        }
    }
    found
}

/// Whether a line names this id, and not one that merely starts the same way.
///
/// `C-1` must not match `C-12`, or every early item would look settled by every later one.
fn cites(text: &str, id: &str) -> bool {
    let mut from = 0;
    while let Some(at) = text[from..].find(id) {
        let start = from + at;
        let end = start + id.len();
        let before = text[..start].chars().next_back();
        let after = text[end..].chars().next();
        let boundary = |character: Option<char>| {
            character.is_none_or(|character| !character.is_ascii_alphanumeric() && character != '-')
        };
        if boundary(before) && boundary(after) {
            return true;
        }
        from = end;
    }
    false
}

/// The last few hundred commits, for reconciliation.
///
/// Shelling out to `git` rather than reading `.git` directly: the format is stable, the
/// tool is present wherever this runs, and a wrong answer here is a report rather than a
/// change.
pub fn history(root: &Path, depth: usize) -> Vec<Commit> {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args([
            "log",
            &format!("--max-count={depth}"),
            "--name-only",
            "--format=%x01%H%x02%s",
        ])
        .output();
    let Ok(output) = output else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout).to_string();
    let mut commits = Vec::new();
    for block in text.split('\u{1}').skip(1) {
        let Some((head, rest)) = block.split_once('\u{2}') else {
            continue;
        };
        let mut lines = rest.lines();
        let subject = lines.next().unwrap_or_default().to_string();
        let touched = lines
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect();
        commits.push(Commit {
            hash: head.chars().take(7).collect(),
            subject,
            touched,
        });
    }
    commits
}

/// Open items, grouped by who has to act, addressees in alphabetical order.
pub fn open_by_addressee(items: &[Item]) -> BTreeMap<String, Vec<&Item>> {
    let mut grouped: BTreeMap<String, Vec<&Item>> = BTreeMap::new();
    for item in items.iter().filter(|item| item.is_outstanding()) {
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

    fn item(id: &str, cited: &[&str]) -> Item {
        Item {
            id: id.to_string(),
            title: "whatever".to_string(),
            to: "spec".to_string(),
            status: "open".to_string(),
            outbox: "crates/outbox.md".to_string(),
            body: String::new(),
            cited: cited.iter().map(|hash| hash.to_string()).collect(),
        }
    }

    fn commit(hash: &str, subject: &str, touched: &[&str]) -> Commit {
        Commit {
            hash: hash.to_string(),
            subject: subject.to_string(),
            touched: touched.iter().map(|path| path.to_string()).collect(),
        }
    }

    /// The failure this exists for, with the real commits that caused it.
    ///
    /// `C-1`, `C-2` and `C-3` were settled by the specification lane and stayed `open` in
    /// the code lane's outbox for a day, so `pending.md` named three questions to Sean
    /// that were not waiting on him. Nobody was at fault: an item is closed by whoever
    /// filed it and answered by somebody else, and the filer gets no signal.
    #[test]
    fn an_item_a_commit_settled_is_reported_while_it_says_open() {
        let items = vec![item("C-1", &[]), item("C-2", &[]), item("C-4", &[])];
        let commits = vec![
            commit("6e3cd6c", "Settle C-1 as housekeeping", &["CLAUDE.md"]),
            commit(
                "2ca59d3",
                "C-2: rule 6 described the losing side",
                &["docs/architecture.md"],
            ),
        ];
        let found = unclosed(&items, &commits);
        let ids: Vec<&str> = found.iter().map(|item| item.id.as_str()).collect();
        assert_eq!(ids, ["C-1", "C-2"], "C-4 was never cited");
        assert_eq!(found[0].hash, "6e3cd6c");
    }

    /// Filing an item cites its own id, and so does closing it. Neither is an answer, and
    /// both touch the outbox the item lives in - which is what tells them apart.
    #[test]
    fn a_commit_that_edits_the_outbox_is_not_an_answer() {
        let items = vec![item("C-5", &[])];
        let commits = vec![commit(
            "aaaaaaa",
            "finding: C-5 filed - a new crate makes the table stale",
            &["crates/outbox.md", "pending.md"],
        )];
        assert!(unclosed(&items, &commits).is_empty());
    }

    /// A citation an author has read and left open is not reported again.
    ///
    /// Without this the reconciliation reports the same item on every run - `C-5` was cited
    /// by a commit that answered half of it - and a signal that always fires is one nobody
    /// reads, which is the failure it was built to prevent.
    #[test]
    fn a_citation_already_considered_is_not_reported_again() {
        let commits = vec![commit(
            "1d8c46f",
            "C-5 and C-6: what a dependency costs",
            &["docs/architecture.md"],
        )];
        assert_eq!(unclosed(&[item("C-5", &[])], &commits).len(), 1);
        assert!(unclosed(&[item("C-5", &["1d8c46f"])], &commits).is_empty());
    }

    /// `C-1` must not be settled by a commit about `C-12`.
    #[test]
    fn an_id_is_not_a_prefix_of_another_id() {
        let commits = vec![commit(
            "bbbbbbb",
            "C-12 acted",
            &["crates/game4x/src/main.rs"],
        )];
        assert!(unclosed(&[item("C-1", &[])], &commits).is_empty());
        assert_eq!(unclosed(&[item("C-12", &[])], &commits).len(), 1);
        // And a bare mention inside a word is not a citation either.
        let odd = vec![commit("ccccccc", "renamed ABC-1X", &["src/lib.rs"])];
        assert!(unclosed(&[item("C-1", &[])], &odd).is_empty());
    }

    /// Only open items. A closed one is cited by the commit that closed it, always.
    #[test]
    fn a_closed_item_is_never_reported() {
        let mut done = item("C-3", &[]);
        done.status = "answered".to_string();
        let commits = vec![commit(
            "ddddddd",
            "C-3: a prototype gets its instrument",
            &["docs/prototypes/README.md"],
        )];
        assert!(unclosed(&[done], &commits).is_empty());
    }

    /// The report reaches the document, not only the exit code.
    ///
    /// `pending.md` is the one place Sean is asked to look, and it is regenerated at every
    /// commit - so putting the reconciliation in it is what makes this something nobody has
    /// to remember to run.
    #[test]
    fn the_generated_document_carries_the_reconciliation() {
        let all = Outboxes {
            items: vec![item("C-1", &[])],
            ..Default::default()
        };
        let settled = vec![Unclosed {
            id: "C-1".to_string(),
            outbox: "crates/outbox.md".to_string(),
            hash: "6e3cd6c".to_string(),
            subject: "Settle C-1 as housekeeping".to_string(),
        }];
        let document = pending(&all, &settled);
        assert!(
            document.contains("## Open, and a commit says otherwise"),
            "{document}"
        );
        assert!(document.contains("6e3cd6c"), "{document}");
        assert!(
            document.contains("Settle C-1 as housekeeping"),
            "{document}"
        );
        // And says nothing when there is nothing to say.
        assert!(!pending(&all, &[]).contains("says otherwise"));
    }

    /// A release is an outbox too, read against the real repository.
    ///
    /// Its capabilities carry an id, a `**to** code` line and one observable sentence -
    /// the same shape as any other item - and they are the work the release exists to
    /// order. Until `S-1` the tool did not look in `releases/`, so the one list that says
    /// what to build next was the one list that did not say it.
    ///
    /// Against the real tree rather than a fixture, because the defect was that a
    /// directory was not walked, and a fixture root has no directories to walk.
    #[test]
    fn the_release_is_read_and_its_capabilities_are_addressed_to_code() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let all = read(&root);
        assert!(
            all.files.iter().any(|at| at.contains("releases/")),
            "no release was read: {:?}",
            all.files
        );
        let ordered: Vec<&Item> = all
            .items
            .iter()
            .filter(|item| item.outbox.contains("releases/"))
            .collect();
        assert!(
            ordered.len() >= 6,
            "only {} capabilities parsed",
            ordered.len()
        );
        // A capability changes hands: `open` is addressed to code, and `built` to the
        // person who has to look at it, because five of the six are vetted by looking and
        // the lane that built a thing cannot certify it.
        assert!(
            ordered
                .iter()
                .all(|item| item.to == "code" || item.to == "sean"),
            "a capability is addressed to somebody who cannot act on it"
        );
        assert!(
            ordered.iter().any(|item| item.to == "code"),
            "no capability is addressed to the lane that builds them"
        );
        assert!(ordered.iter().any(|item| item.id == "R-6"));
    }

    /// A `cited` list is read whole, however it is written.
    ///
    /// **`S-8`.** It used to go through `field`, which returns one word - so the list was
    /// handed a single hash and then split on commas and spaces, and the space arm could
    /// never fire. `` **cited** `a1b2c3d`, `e4f5a6b` `` kept the first and dropped the rest
    /// in silence, and the only form that worked was one nobody would choose.
    ///
    /// Each half was right alone: `field` stops at whitespace so the separator between
    /// fields never matters, and `considered` splits a list. Together they lost data.
    #[test]
    fn a_cited_list_is_read_however_it_is_punctuated() {
        let both = ["a1b2c3d", "e4f5a6b"];
        for line in [
            "**to** spec · **status** open · **cited** `a1b2c3d`, `e4f5a6b`",
            "**to** spec · **status** open · **cited** `a1b2c3d` `e4f5a6b`",
            "**to** spec · **status** open · **cited** `a1b2c3d`,`e4f5a6b`",
            "**to** spec · **status** open · **cited** a1b2c3d, e4f5a6b",
        ] {
            assert_eq!(considered(line), both, "`{line}`");
        }

        // A note may sit beside the hashes; anything not hash-shaped is ignored.
        assert_eq!(
            considered("**status** open · **cited** `a1b2c3d`, and see the note"),
            ["a1b2c3d"]
        );
        // And a field after it is not swallowed, whatever separates the two.
        //
        // **The outboxes here punctuate with a middle dot and the proposal queue with
        // ` - `**, and this used to stop at the dot - so against the queue's punctuation
        // the value ran on into whatever came after it. Unnoticed because nothing
        // following a `cited` field had looked like a hash yet.
        // The whole list, not its first element and an absence.
        //
        // Checking `read[0]` and that the next field's hash is missing would pass a version
        // that dropped the second hash, or kept a stray token between them. What makes the
        // trailing separator harmless is the hex filter below, and a filter doing
        // load-bearing work reads as tidiness unless something asserts it.
        for (line, expected) in [
            (
                "**cited** `a1b2c3d` · **raised** 2026-09-01",
                vec!["a1b2c3d"],
            ),
            (
                "**cited** `a1b2c3d` - **raised** 2026-09-01",
                vec!["a1b2c3d"],
            ),
            (
                "**cited** `a1b2c3d` - **source** `1234567abc`",
                vec!["a1b2c3d"],
            ),
            (
                "**cited** `a1b2c3d`, `e4f5a6b` - **source** `1234567abc`",
                vec!["a1b2c3d", "e4f5a6b"],
            ),
            (
                "**cited** `a1b2c3d`, `e4f5a6b`, `c7d8e9f`, `0a1b2c3` - **raised** 2026-09-02",
                vec!["a1b2c3d", "e4f5a6b", "c7d8e9f", "0a1b2c3"],
            ),
        ] {
            assert_eq!(considered(line), expected, "`{line}`");
        }
        assert!(considered("**to** spec · **status** open").is_empty());
    }

    /// The single-word fields still stop at whitespace, which is why `field` was left alone.
    #[test]
    fn a_single_word_field_is_still_one_word() {
        let line = "**to** spec · **status** open · **raised** 2026-09-01";
        assert_eq!(field(line, "to").unwrap(), "spec");
        assert_eq!(field(line, "status").unwrap(), "open");
    }

    /// The text a proposal proposes, in the shape `S-11` describes.
    ///
    /// Written from that description rather than from a live proposal, because the queue
    /// was empty when this was built and `**Basis**` had no instance anywhere in the
    /// repository. **That is stated rather than hidden**: the first real proposal is what
    /// confirms the shape, and if it differs this test is the thing that says so.
    #[test]
    fn a_proposal_offers_the_text_it_proposes() {
        let queue = "\
### P-200 - a one-line title

**to** sean · **status** open · **raised** 2026-09-02 · **into** `spec/turn.md` -> Order

Some reasoning nobody promotes.

> A turn has three parts: producing, consuming and transforming.
> Producing is the player acting.

**Basis** the reasoning that makes it right.

### P-201 - one that proposes nothing

**to** sean · **status** open · **raised** 2026-09-02

Only prose.
";
        let items = parse(queue, "docs/notes/proposals.md");
        assert_eq!(items.len(), 2);

        assert_eq!(
            items[0].proposed_text().unwrap(),
            "A turn has three parts: producing, consuming and transforming.\nProducing is the player acting."
        );
        // The prose around it is not the proposal, and neither is the Basis.
        assert!(!items[0].proposed_text().unwrap().contains("Basis"));
        assert!(
            !items[0]
                .proposed_text()
                .unwrap()
                .contains("nobody promotes")
        );

        assert_eq!(items[1].proposed_text(), Err(NoText::None));
    }

    /// The same, against the real queue rather than a fixture.
    ///
    /// **The fixture beside this was written from a description, because when it was built
    /// the queue was empty and `**Basis**` had no instance anywhere.** That test says so and
    /// says the first real proposal is what confirms it. Real proposals exist now, so this
    /// reads them off disk - which is the difference between a test that checks this crate
    /// against itself and one that checks it against something else.
    ///
    /// It asserts a shape rather than any particular proposal's words, because the queue is
    /// emptied as Sean approves things and a test naming `P-183` would fail for the best
    /// possible reason.
    #[test]
    fn every_real_proposal_offers_its_text_or_says_why_not() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let Ok(queue) = std::fs::read_to_string(root.join("docs/notes/proposals.md")) else {
            return;
        };
        let parsed = parse(&queue, "docs/notes/proposals.md");

        // **What this test does when the queue is empty, which is most of the time.** An
        // empty queue is the good state, so `open` proposals cannot be required to exist -
        // and the loop below then runs over nothing and passes having asserted nothing.
        // The rule itself is held by the unit tests above, over text written to be wrong;
        // this is the spot-check against the live file.
        //
        // What can still go quiet is the parser. If it stopped recognising a proposal
        // heading, every proposal would vanish from this list and the loop would pass for
        // the wrong reason - which is exactly how `S-8`'s hash went unread for half a day.
        // So the headings are counted a second way, by looking for the text, and the two
        // counts have to agree.
        let by_heading = queue
            .lines()
            .filter(|line| line.trim_start().starts_with("### P-"))
            .count();
        let by_parser = parsed
            .iter()
            .filter(|item| item.id.starts_with("P-"))
            .count();
        assert_eq!(
            by_parser, by_heading,
            "the file has {by_heading} proposal headings and the parser found {by_parser};              a proposal it cannot see is one this test cannot check"
        );

        // **A proposal that asks a decision offers no text, and must not be required to.**
        // `P-229` split the queue in two: proposals ready for Sean to approve, and proposals
        // drawing attention to a decision he has to make. The second kind carries `**asks**`
        // and no quotation, because no wording can be right until he chooses - `P-231` is
        // the first, asking whether `labor` is a kind.
        //
        // Requiring text of one would push whoever wrote it to invent a quotation for a
        // question, which is the opposite of what the split is for.
        let proposals: Vec<Item> = parsed
            .into_iter()
            .filter(|item| item.id.starts_with("P-") && item.is_outstanding())
            // **Two kinds of proposal offer no text, and neither is malformed.** One asks a
            // decision - `P-229`'s split, marked `**asks**` - and no wording can be right
            // until Sean chooses. The other is `shape instruction`: it describes a change to
            // make in several places, nothing lands verbatim, and it carries the assertion
            // that proves it was applied instead.
            //
            // `tools/outbox/tests/promotions.rs` learned the second of these days ago and
            // this check had not. The same rule, in two places, out of step - which is what
            // one place asking two questions looks like from the inside.
            .filter(|item| {
                !item.body.contains("**asks**") && !item.body.contains("**shape** instruction")
            })
            .collect();

        for proposal in &proposals {
            match proposal.proposed_text() {
                Ok(text) => {
                    assert!(!text.is_empty(), "{} proposes nothing", proposal.id);
                    // The prose around a proposal is not the proposal.
                    assert!(
                        !text.contains("**Basis"),
                        "{} swallowed its basis",
                        proposal.id
                    );
                    assert!(!text.starts_with('>'), "{} kept its quoting", proposal.id);
                }
                Err(why) => panic!("{} has no readable text: {why}", proposal.id),
            }
        }

        // A blank quoted line separates paragraphs and must not end the block. It is the
        // shape every proposal of more than one paragraph takes, and the one thing the
        // description would not have settled - so it is asserted where it occurs rather
        // than assumed.
        if let Some(several) = proposals
            .iter()
            .find(|proposal| proposal.body.contains("\n>\n") || proposal.body.contains("\r\n>\r\n"))
        {
            let text = several
                .proposed_text()
                .expect("a paragraph break is not an ending");
            assert!(
                text.contains("\n\n"),
                "{} lost the break between its paragraphs",
                several.id
            );
        }
    }

    /// The heading count and the parser agree over a queue that is not empty.
    ///
    /// **The live check above compares 0 with 0 whenever the queue is empty, which is the
    /// good state and most of the time.** That is the failure this repository keeps
    /// producing, so the agreement it relies on is demonstrated here instead, over eight
    /// proposals written out rather than read from a file that empties.
    ///
    /// The second half is the part with teeth: one heading is made unreadable, and the two
    /// counts have to stop agreeing. A comparison that cannot disagree is not a comparison.
    #[test]
    fn a_proposal_the_parser_cannot_see_makes_the_two_counts_differ() {
        let mut queue = String::from("# Proposals\n\n");
        for n in 1..=8 {
            queue.push_str(&format!(
                "### P-{n} - a proposal\n\n**to** sean · **status** open · **kind** shape\n\n> Text {n}.\n\n"
            ));
        }

        let headings = |text: &str| {
            text.lines()
                .filter(|line| line.trim_start().starts_with("### P-"))
                .count()
        };
        let parsed = |text: &str| {
            parse(text, "docs/notes/proposals.md")
                .into_iter()
                .filter(|item| item.id.starts_with("P-"))
                .count()
        };

        assert_eq!(headings(&queue), 8, "eight written");
        assert_eq!(parsed(&queue), 8, "and eight read");

        // Now hide one from the parser, in the way `CLAUDE.md` says an item goes invisible:
        // *without the `**to**` line the item is invisible to the index*. The heading is
        // untouched, so a reader still sees a proposal and the parser does not - which is
        // the whole reason the two counts are taken separately.
        //
        // Blinding both counters instead would prove nothing: the first attempt at this
        // broke the heading as well, both counts fell to seven together, and the assertion
        // below failed for the right reason.
        let blinded = queue.replace(
            "### P-4 - a proposal\n\n**to** sean · **status** open · **kind** shape",
            "### P-4 - a proposal\n\nto sean, status open, kind shape",
        );
        assert_eq!(headings(&blinded), 8, "a reader still sees eight");
        assert_ne!(
            headings(&blinded),
            parsed(&blinded),
            "a proposal the parser cannot see has to make the counts differ, or the \
             comparison in the live check is decoration"
        );
    }

    /// Two blockquotes are reported rather than picked between.
    ///
    /// A best effort here would hand `tools/spec` something to promote that Sean never
    /// approved, which is the one failure this function exists to make impossible.
    #[test]
    fn a_proposal_with_two_blockquotes_is_a_question_rather_than_an_answer() {
        let queue = "\
### P-202 - two of them

**to** sean · **status** open · **raised** 2026-09-02

> The first.

Something in between.

> The second.
";
        let items = parse(queue, "docs/notes/proposals.md");
        assert_eq!(items[0].proposed_text(), Err(NoText::Several(2)));
    }

    /// An item's body stops at the next item.
    #[test]
    fn a_body_ends_where_the_next_item_begins() {
        let queue = "\
### P-203 - the first

**to** sean · **status** open

> Mine.

### P-204 - the second

**to** sean · **status** open

> Not mine.
";
        let items = parse(queue, "docs/notes/proposals.md");
        assert_eq!(items[0].proposed_text().unwrap(), "Mine.");
        assert_eq!(items[1].proposed_text().unwrap(), "Not mine.");
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
