//! What is open, and addressed to whom.
//!
//! ```text
//! outbox                  every open item, grouped by addressee
//! outbox --to code        one addressee's inbox
//! outbox --check          exit 1 if anything is open and addressed
//! outbox --count          the aggregate, against the limit
//! ```
//!
//! Run from anywhere: the repository root is found from this file's own path, the way the
//! scripts in `scripts/` do it.

use std::path::{Path, PathBuf};

/// How far back to look for a commit that settled something. Long enough to span the
/// whole workflow so far, short enough that the walk costs nothing.
const DEPTH: usize = 400;

use outbox::{
    Item, LIMIT, Outboxes, duplicate_ids, history, open_by_addressee, pending, read, same_section,
    unclosed,
};

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let root = root();
    let all = read(&root);

    // Said before anything else, and always. `tools/pad-tables` exists because thirteen
    // rows went missing from a hand-edited table and nothing noticed; a tool that walks
    // directories should say how many it found, so a missing one is visible rather than
    // silently excluded from a count somebody trusts.
    report_what_was_found(&all);

    let complaints = complain(&all);
    for complaint in &complaints {
        eprintln!("{complaint}");
    }

    let code = match arguments.first().map(String::as_str) {
        None => {
            show(&all.items, None);
            show_same_section(&all);
            0
        }
        Some("--to") => {
            let who = arguments.get(1).cloned().unwrap_or_default();
            if who.is_empty() {
                eprintln!("--to needs an addressee, as in `outbox --to code`");
                2
            } else {
                show(&all.items, Some(&who));
                0
            }
        }
        Some("--count") => {
            let open = all
                .items
                .iter()
                .filter(|item| item.is_outstanding())
                .count();
            // The count and nothing more, until `Q-32` settles what the limit counts.
            //
            // This used to add "past the limit: reviewing now costs as much as writing".
            // The count is true and that verdict is not: the limit's justification is
            // Sean's reading time, and most of what is open costs a producer rather than
            // him. Printed anyway it did real harm - it is what led one lane to tell him he
            // was at fifteen against a limit of fifteen while his queue was empty.
            //
            // Saying nothing would hide that a fix is pending; saying the verdict repeats
            // the error on whoever reads it next. So: the number, which is a fact, and no
            // judgement, which is not this tool's to make yet.
            println!("{open} open across every outbox");
            println!("a limit is pending: {LIMIT} was written for Sean's queue,");
            println!("and most of these are a producer's backlog");
            0
        }
        Some("--check") => {
            let open: Vec<&Item> = all
                .items
                .iter()
                .filter(|item| item.is_outstanding())
                .collect();
            if open.is_empty() {
                println!("nothing open; every perspective knows of nothing outstanding");
                0
            } else {
                show(&all.items, None);
                1
            }
        }
        // Only the items a commit says were dealt with, so a hook can act on them without
        // failing on every ordinary open item.
        Some("--settled") => {
            let settled = unclosed(&all.items, &history(&root, DEPTH));
            if settled.is_empty() {
                println!("nothing open that a commit says was settled");
                0
            } else {
                for item in &settled {
                    // The count is what tells *answered and not closed* from *somebody is
                    // in the middle of it* - `Q-43`. One citation is the shape this was
                    // built for; nine is ongoing work, and says so without reading intent
                    // out of a commit message.
                    let many = if item.citations > 1 {
                        format!(" ({} commits cite it)", item.citations)
                    } else {
                        String::new()
                    };
                    println!(
                        "{} - still open in {}, but {} says: {}{many}",
                        item.id, item.outbox, item.hash, item.subject
                    );
                }
                1
            }
        }
        Some("--write") => {
            let path = arguments
                .get(1)
                .cloned()
                .unwrap_or_else(|| "pending.md".to_string());
            let at = root.join(&path);
            let settled = unclosed(&all.items, &history(&root, DEPTH));
            match std::fs::write(&at, pending(&all, &settled)) {
                Ok(()) => {
                    println!("wrote {path}");
                    0
                }
                Err(why) => {
                    eprintln!("cannot write {path}: {why}");
                    2
                }
            }
        }
        Some("--sections") => {
            show_same_section(&all);
            0
        }
        Some("--help" | "-h") => {
            println!("{}", usage());
            0
        }
        Some(other) => {
            eprintln!("there is no option {other}; try --help");
            2
        }
    };

    // A malformed outbox is worse than a failing check, because it makes every count a
    // guess. It fails whatever was asked for.
    std::process::exit(if complaints.is_empty() { code } else { 2 });
}

fn usage() -> &'static str {
    "\
outbox - what is open, and addressed to whom

    outbox                  every open item, grouped by addressee
    outbox --to WHO         one addressee's inbox
    outbox --check          exit 1 if anything is open and addressed
    outbox --count          the aggregate, against the limit
    outbox --sections       sections that have taken more than one proposal
    outbox --write [PATH]   write the pending document, default pending.md
    outbox --help           this"
}

/// The repository root, from this file's own path.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn report_what_was_found(all: &Outboxes) {
    println!(
        "{} outbox{} read: {}",
        all.files.len(),
        if all.files.len() == 1 { "" } else { "es" },
        all.files.join(", ")
    );
    if !all.missing.is_empty() {
        println!("not present: {}", all.missing.join(", "));
    }
    println!();
}

/// What is wrong with the outboxes themselves, rather than with what they say.
fn complain(all: &Outboxes) -> Vec<String> {
    let mut complaints = Vec::new();
    if all.files.is_empty() {
        complaints.push("no outbox anywhere; nothing could be read".to_string());
    }
    for (id, wheres) in duplicate_ids(&all.items) {
        complaints.push(format!(
            "the id {id} is used in {} - a cited id must resolve to one item",
            wheres.join(" and ")
        ));
    }
    complaints
}

/// Sections that have taken more than one proposal.
///
/// The trigger behind the rule Sean decided. Not a defect list: several proposals in one
/// section is what working on one topic looks like. What it says is that nobody has asked
/// whether they all still hold together.
fn show_same_section(all: &Outboxes) {
    let flags = same_section(&all.landed);
    if flags.is_empty() {
        println!("no section took more than one proposal in a day");
        return;
    }
    println!(
        "{} section(s) have taken more than one proposal - re-read each whole, and ask",
        flags.len()
    );
    println!("whether all of them can hold at once:");
    for flag in &flags {
        println!("  {} - {}", flag.destination, flag.proposals.join(", "));
    }
}

fn show(items: &[Item], only: Option<&str>) {
    let grouped = open_by_addressee(items);
    let mut shown = 0;
    for (who, theirs) in &grouped {
        if only.is_some_and(|wanted| wanted != who) {
            continue;
        }
        println!("to {who} ({}):", theirs.len());
        for item in theirs {
            println!("  {} - {}", item.id, item.title);
            println!("      {}", item.outbox);
        }
        println!();
        shown += theirs.len();
    }
    match only {
        Some(who) if shown == 0 => println!("nothing open addressed to {who}"),
        None if shown == 0 => println!("nothing open"),
        _ => println!("{shown} open"),
    }
}
