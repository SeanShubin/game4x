//! Who quotes a file of the specification, and where - the carrier for a habit that is a grep.
//!
//! ```text
//! cargo run -p game-console --example quotes                       every document, with a count
//! cargo run -p game-console --example quotes -- spec/logistics.md  who quotes that one
//! ```
//!
//! # Why this exists rather than the grep it replaces
//!
//! **`S-202`, from the specification lane.** `CLAUDE.md` now says that before replacing text in a
//! file of the specification, grep for who quotes it - and this repository has a rule about
//! reminders: *a rule stated without its tool is a rule whose tool nobody reaches for*, written
//! after a lane spent a day reimplementing `tools/anchor` without knowing it existed.
//!
//! **The case is `P-568`.** It reworded three containment bullets and stranded four quotations in
//! `tree.rs` and `containment.rs`. The rule the promoting lane follows is `spec touching <file>`,
//! which lists open **items** naming a file - and a quotation in another lane's code is not an item.
//! **So the rule was kept and the thing still broke.**
//!
//! # It is a question, not a gate
//!
//! **Nothing here fails.** `tests/quotations.rs` is the gate and is unchanged; this asks that
//! checker what it already knows, before an edit rather than after it. `C-137` measures the same
//! walker over the prototype's data comments at eight false alarms, which is an argument against
//! widening what is swept and not against answering a question about what is swept already.
//!
//! # Borrowed from the test, which inverts the usual direction on purpose
//!
//! **Elsewhere the program is the home and the test borrows it** - `tests/scenario.rs` borrows
//! `examples/scenario.rs`, and `tests/directories.rs` borrows `examples/render.rs` - because there
//! the program is the deliverable and the test holds it to what it produces.
//!
//! **Here the checker is the deliverable and this is a view of it**, so the borrowing runs the other
//! way. What matters either way is that there is one derivation: a document this prints and a
//! document the gate checks cannot come apart, because the same functions answer both.

#[path = "../tests/quotations.rs"]
#[allow(dead_code, unused_imports)]
mod quotations;

use std::collections::BTreeMap;

use quotations::{blocks, quotations as inline, root, sources};

/// Every quotation in the tree: the document, where it was found, and the words.
///
/// **Inline and block quotations together**, because a reader about to reword a bullet does not
/// care which shape quotes it - and `P-568` stranded both kinds, four inline in `tree.rs` and one
/// block in `containment.rs`.
fn every_quotation() -> Vec<(String, String, String)> {
    let at = root();
    let mut all = Vec::new();
    for path in sources() {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let shown = path
            .strip_prefix(&at)
            .unwrap_or(&path)
            .display()
            .to_string()
            .replace('\\', "/");
        for (document, quoted) in inline(&path, &text) {
            all.push((document, shown.clone(), quoted));
        }
        for (document, quoted) in blocks(&path, &text) {
            all.push((document, shown.clone(), quoted));
        }
    }
    all
}

fn main() {
    let wanted: Option<String> = std::env::args().nth(1);
    let all = every_quotation();

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. An answer of
    // *nobody quotes it* means something only against a sweep that found quotations, so the sweep's
    // own total is printed whether or not anything matched.
    if all.is_empty() {
        eprintln!("the sweep found no quotations at all, so this answers nothing");
        std::process::exit(1);
    }

    let Some(wanted) = wanted else {
        let mut by_document: BTreeMap<&str, usize> = BTreeMap::new();
        for (document, _, _) in &all {
            *by_document.entry(document.as_str()).or_default() += 1;
        }
        println!(
            "{} quotation(s) of {} document(s):\n",
            all.len(),
            by_document.len()
        );
        for (document, count) in &by_document {
            println!("{count:5}  {document}");
        }
        println!("\nName one to see who quotes it.");
        return;
    };

    // **Matched on the bare name as well as the path**, so `logistics.md` and `spec/logistics.md`
    // both work - a caller who has just edited a file has its path in hand and a caller reading a
    // proposal has its name.
    let named = wanted.trim_start_matches("./").replace('\\', "/");
    let mine: Vec<&(String, String, String)> = all
        .iter()
        .filter(|(document, _, _)| *document == named || document.ends_with(&format!("/{named}")))
        .collect();

    println!(
        "{} quotation(s) of `{named}`, out of {} in the tree:\n",
        mine.len(),
        all.len()
    );
    let mut by_file: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (_, where_, quoted) in &mine {
        by_file.entry(where_).or_default().push(quoted);
    }
    for (file, quoted) in &by_file {
        println!("{file}");
        for one in quoted {
            let short: String = one.chars().take(96).collect();
            let ellipsis = if one.chars().count() > 96 { " ..." } else { "" };
            println!("    {short}{ellipsis}");
        }
        println!();
    }
    if mine.is_empty() {
        println!(
            "Nothing quotes it, so its wording is not held by anything in this lane's column."
        );
    }
}
