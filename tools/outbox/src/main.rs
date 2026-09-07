//! What is open, and addressed to whom.
//!
//! ```text
//! outbox                  every open item, grouped by addressee
//! outbox --to code        one addressee's inbox
//! outbox --check          exit 1 if anything is open and addressed
//! outbox --waiting        holds whose reason is over
//! outbox --closing        what closed since HEAD, and open items deriving from its rule
//! outbox --orphans        closed items whose closing line names a withdrawn proposal
//! outbox --places         the outboxes it reads, one per line, for a caller that guards them
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
    Item, LIMIT, Outboxes, duplicate_ids, history, misfiled_by_asks, open_by_addressee, pending,
    read, same_section, unclosed,
};

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let root = root();
    let all = read(&root);

    // **`--places` prints the list and nothing else**, because a caller parsing it is a
    // shell. Everything below writes to stdout, so the banner is skipped for that one mode
    // rather than made conditional everywhere.
    if arguments.first().is_some_and(|first| first == "--places") {
        for file in &all.files {
            println!("{file}");
        }
        return;
    }

    // Said before anything else, and always. `tools/pad-tables` exists because thirteen
    // rows went missing from a hand-edited table and nothing noticed; a tool that walks
    // directories should say how many it found, so a missing one is visible rather than
    // silently excluded from a count somebody trusts.
    report_what_was_found(&all);

    // **Said and never gated**, which is the distinction between this and `complain`: a
    // complaint exits 2, `hooks/pre-commit` runs under `set -e`, and three lanes share this
    // tree - so a complaint about one lane's file stops every other lane committing.
    //
    // **Learned by doing it.** This began as a complaint, and the two headings it names are
    // in the proposal queue, which this lane may not edit. It blocked its own commit and
    // would have blocked the specification lane's next one, on a file only they can repair.
    // `Q-60` says exactly that about `S-51`, in the change that introduced it.
    let complaints = complain(&all);
    for complaint in &complaints {
        eprintln!("{}", complaint.text());
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
            // The count and nothing more, until it is settled what the limit counts.
            //
            // **This deferred to `Q-32`, which closed `acted` on 2026-08-30 having deleted a
            // duplicate figure from `CLAUDE.md` and settled nothing about what the limit
            // counts.** So the code waited on an item that could not answer it, and said a
            // fix was pending against nothing pending - `S-50`.
            //
            // **The question is open as `P-300`**, which is words Sean has not approved. The
            // line below stays as it is until he does: what it becomes - distinguishing his
            // reading budget from an instance's own backlog - is one of three consequences
            // `S-50` states and explicitly says not to build yet. Naming the live proposal
            // rather than a closed item is the whole of the fix here.
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
            println!("a limit is pending as `P-300`: {LIMIT} was written for Sean's queue,");
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
        // **`P-250`'s second half, which had no mechanism until `S-41`.** *When an item
        // moves to `acted`, whatever lists the outboxes lists the open items naming the
        // same rule.* Printed here rather than anywhere else because this output is read -
        // it is what found the stale `R-6` blocker - and a listing nobody reads is worth
        // nothing.
        //
        // **It makes the failure findable, not found**, which is the limit promoted with
        // the rule. An item whose premise moved still reads correctly and only its
        // conclusion has stopped being true, so nothing can decide that for a reader. What
        // this does is put the two in front of each other at the moment one of them moves.
        // `S-51`. Reports and does not gate - `Q-60`: a gate reddens for whichever lane
        // commits next, and that may be one which must not repair it. `P-305`'s filing rule
        // is the mechanism; this is the backstop for when it is forgotten.
        Some("--orphans") => {
            let queue =
                std::fs::read_to_string(root.join("docs/notes/proposals.md")).unwrap_or_default();
            let (population, orphans, misfiled) = outbox::closed_on_withdrawn(&all.items, &queue);
            // **Reported, never treated as a withdrawal** - `Q-61`. Acting on one would file
            // work into another lane's outbox because of a typo in a table.
            // **Both sides of the classification, so it cannot start looking at nothing.**
            // The safety rests on no genuine withdrawal opening its reason with a backticked
            // filename, which holds over 26 rows today and is a small population that will
            // grow - a reason reading "`spec/planet.md` already says this" would be read as
            // a misfiled Accepted row and quietly leave the withdrawn set smaller. Printing
            // the count that remains is what makes that visible.
            let (genuine, _) = outbox::withdrawn(&queue);
            println!(
                "{} withdrawn proposal(s) read as genuine, {} row(s) read as an Accepted row \
                 in the wrong table",
                genuine.len(),
                misfiled.len()
            );
            if !misfiled.is_empty() {
                println!("  misfiled: {misfiled:?}");
            }
            // **Both numbers, because zero says nothing on its own.** A predicate that had
            // stopped matching anything would report no orphans and look identical to a
            // queue with none - `docs/process.md` -> What makes a check worth having.
            println!(
                "{} closed item(s) whose closing line names a proposal; {} name a withdrawn one",
                population,
                orphans.len()
            );
            for (item, outbox, proposal) in &orphans {
                println!(
                    "{item} in {outbox} closed on {proposal}, which is withdrawn - it closed \
                     into something nobody decided to drop"
                );
            }
            0
        }
        // **`S-61`: a hold whose reason is over.** Reports and does not gate - `Q-60` settled
        // that for `S-51` and the reasoning carries: a gate reddens for whichever lane commits
        // next, and that may be a lane which must not repair it.
        Some("--waiting") => {
            let queue =
                std::fs::read_to_string(root.join("docs/notes/proposals.md")).unwrap_or_default();
            let (population, over) = outbox::waiting(&all.items, &queue);
            // **Both numbers, because zero says nothing on its own.** *No stale waits* means
            // nothing unless the count of items carrying a wait is non-zero.
            println!(
                "{population} open item(s) say what they wait on; {} of those waits are over",
                over.len()
            );
            for held in &over {
                let became = match &held.became {
                    outbox::Wait::Promoted { destination, date } => {
                        format!("promoted into {destination} on {date}")
                    }
                    outbox::Wait::Withdrawn => "withdrawn".to_string(),
                    // `waiting` returns only waits that are over, so this cannot arrive.
                    // Said plainly rather than as an `unreachable!`: a filtering mistake in a
                    // reporting tool should print a wrong line, not stop the hook that runs it.
                    outbox::Wait::Holding => {
                        "still open, which this should not have reported".to_string()
                    }
                    outbox::Wait::Closed { status } => format!("closed as {status}"),
                    // **Not a satisfied wait** - `S-51`'s shape. A predicate that cannot tell
                    // *resolved* from *never was* reports both as fine.
                    outbox::Wait::Unknown => {
                        "nowhere - no outbox and no proposal has that id, so this wait names \
                         nothing and is a defect in the item rather than a wait that ended"
                            .to_string()
                    }
                };
                println!(
                    "{} in {} waits on {}, which was {became}",
                    held.item, held.outbox, held.on
                );
            }
            0
        }
        Some("--closing") => {
            let closing = outbox::closing(&root, &all);
            let sharing = outbox::sharing_a_rule(&closing, &all.items);
            println!(
                "{} item(s) closed since HEAD; {} of them name a rule another open item names",
                closing.len(),
                sharing.len()
            );
            for (closed, also) in &sharing {
                for item in also {
                    println!(
                        "{closed} closed, and {} in {} derives from the same rule - re-derive \
                         it or say it still holds",
                        item.id, item.outbox
                    );
                }
            }
            0
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
    std::process::exit(exit_code(&complaints, code));
}

fn usage() -> &'static str {
    "\
outbox - what is open, and addressed to whom

    outbox                  every open item, grouped by addressee
    outbox --to WHO         one addressee's inbox
    outbox --check          exit 1 if anything is open and addressed
    outbox --waiting        holds whose reason is over
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
/// Something the tool has to say, and whether it may stop the caller.
///
/// **Two channels, because one made *reports and does not gate* a thing to remember.**
/// `complain` returned strings and `main` exited 2 if there were any, so every future note
/// gated by default and the only way not to was to remember an `eprintln!` somewhere else.
/// `S-51` said in its own words that it reports and does not gate, and the change that built
/// it pushed onto this list - blocking its own commit, and the specification lane's next one,
/// over headings in a file this lane may not edit.
///
/// **The quality lens is why this exists rather than the rule being restated.** `C-42` first
/// concluded that the shape could only be stated and not automated. That is the answer to
/// *can a check ask whether a rule was applied* - which is no, and is the wrong question. The
/// answerable one is **what tool would have refused**, and here it is: a note now says which
/// channel it is on, and an advisory one cannot reach the exit code at all.
enum Note {
    /// Stops the caller. Only for something no lane can commit past.
    Blocking(String),
    /// Never stops the caller. **Three lanes share this tree**, so a note about one lane's
    /// file must not stop another lane committing - especially when the lane that must
    /// repair it is barred from editing that file.
    Advisory(String),
}

impl Note {
    fn text(&self) -> &str {
        match self {
            Note::Blocking(said) | Note::Advisory(said) => said,
        }
    }

    fn blocks(&self) -> bool {
        matches!(self, Note::Blocking(_))
    }
}

/// The exit code, which only a blocking note may change.
///
/// Separated from the printing so it can be tested, which is what makes the rule mechanical
/// rather than a comment: an advisory note that started gating would fail here.
fn exit_code(notes: &[Note], code: i32) -> i32 {
    if notes.iter().any(Note::blocks) {
        2
    } else {
        code
    }
}

fn complain(all: &Outboxes) -> Vec<Note> {
    let mut notes = Vec::new();
    if all.files.is_empty() {
        notes.push(Note::Blocking(
            "no outbox anywhere; nothing could be read".to_string(),
        ));
    }
    for (id, wheres) in duplicate_ids(&all.items) {
        notes.push(Note::Blocking(format!(
            "the id {id} is used in {} - a cited id must resolve to one item",
            wheres.join(" and ")
        )));
    }
    // Advisory for the same reason: both of Sean's files are the specification lane's, and
    // this lane must not stop them committing over one. `S-53`.
    let (asked, misfiled) = misfiled_by_asks(&all.items);
    if !misfiled.is_empty() {
        notes.push(Note::Advisory(format!(
            "{} of {} item(s) carrying an `asks` field are in the wrong one of the two files \
             addressed to Sean:\n  {}",
            misfiled.len(),
            asked,
            misfiled.join("\n  ")
        )));
    }
    // Advisory, and named as such rather than kept out of the list. The two headings it
    // reports today are in the proposal queue, which this lane may not edit.
    if !all.unparsed.is_empty() {
        notes.push(Note::Advisory(format!(
            "{} heading(s) name an item that does not parse as one, so they are invisible to \
             this index: {}\n  the first non-blank line under a heading must be its `**to**` \
             line - a note written above it removes the item silently",
            all.unparsed.len(),
            all.unparsed.join(", ")
        )));
    }
    notes
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

#[cfg(test)]
mod tests {
    use super::*;

    /// An advisory note cannot reach the exit code, and a blocking one always does.
    ///
    /// **This is the check `C-42` said was unavailable.** It cannot ask whether somebody
    /// applied a rule; it can make the unapplied path unreachable, which is what the quality
    /// lens pointed out was the answerable question.
    #[test]
    fn only_a_blocking_note_can_stop_the_caller() {
        let advisory = vec![
            Note::Advisory("a heading does not parse".to_string()),
            Note::Advisory("and another".to_string()),
        ];
        assert_eq!(
            exit_code(&advisory, 0),
            0,
            "two advisory notes, and the caller is not stopped - three lanes share this tree"
        );
        assert_eq!(
            exit_code(&advisory, 1),
            1,
            "nor is a caller's own code overridden"
        );

        let mut mixed = advisory;
        mixed.push(Note::Blocking("a duplicated id".to_string()));
        assert_eq!(
            exit_code(&mixed, 0),
            2,
            "one blocking note among advisory ones still stops the caller"
        );
        assert_eq!(exit_code(&[], 0), 0, "and nothing to say changes nothing");
    }
}
