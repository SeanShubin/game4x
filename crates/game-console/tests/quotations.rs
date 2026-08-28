//! Every attributed quotation of the specification is a quotation of what it says now.
//!
//! One sentence in `spec/console.md` changed four times in four days, P-91 through P-95,
//! and each change left quotations of the previous wording scattered through `crates/`.
//! Two hand-grepped cleanups each missed three of them. A comment that misquotes the
//! specification is worse than one that says nothing, because it is the thing a reader
//! checks *instead of* opening `spec/`.
//!
//! So this reads the specification off disk at test time and checks the code against it,
//! which is the habit the repository already has:
//! [`first_release.rs`](first_release.rs) parses the release's own tables, and
//! `game-front`'s `what_is_carried_is_what_is_on_disk` compares the embedded command files
//! with the ones in `commands/`. Coupling a test to a document is deliberate here - it is
//! what stops the document and the code drifting in silence.
//!
//! # What counts as a quotation
//!
//! The convention already in use throughout the code:
//!
//! ```text
//! /// `spec/console.md`: *a line beginning with `/` directs the front end.*
//! ```
//!
//! A backticked spec path, a colon, and an italic run. Anything written that way is
//! checked. Prose *about* the spec is not a quotation and is not checked - only text the
//! author marked as the specification's own words, which is exactly the text a reader
//! would otherwise trust without looking.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every `.rs`, `.md` and `.html` file under `crates/`, which is where this lane writes.
fn sources() -> Vec<PathBuf> {
    let mut found = Vec::new();
    collect(&root().join("crates"), &mut found);
    found.sort();
    found
}

fn collect(directory: &Path, into: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // `target` and `dist` hold generated copies of files that are checked at their
            // source. Checking them too would report every finding twice.
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if name != "target" && name != "dist" {
                collect(&path, into);
            }
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("rs") | Some("md") | Some("html")
        ) {
            into.push(path);
        }
    }
}

/// Whitespace collapsed to single spaces, and comment markers removed.
///
/// A quotation is wrapped across lines to fit a comment, and the specification wraps at a
/// different width, so neither can be compared as written. Collapsing both makes the
/// comparison about the words.
fn flattened(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let line = line.trim();
        let line = line
            .strip_prefix("//!")
            .or_else(|| line.strip_prefix("///"))
            .or_else(|| line.strip_prefix("//"))
            .unwrap_or(line);
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(line.trim());
    }
    // Lowercased, because a quotation legitimately lowercases its first word when it is
    // embedded in a sentence, and that is a fact about the sentence around it rather than
    // a misquotation of the specification.
    out.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Flattened, and with backticks dropped.
///
/// Whether a word is written as code is the quoting medium's formatting rather than the
/// specification's wording: a doc comment marking up `/browser` where the spec did not is
/// not a misquotation. Backticks survive [`flattened`] because that is what the quotations
/// are *found* by, and are dropped here because that is not what they are *judged* by.
fn bare(text: &str) -> String {
    flattened(&text.replace('`', " "))
}

/// The quotations in one file: the spec file named, and the words attributed to it.
fn quotations(text: &str) -> Vec<(String, String)> {
    let flat = flattened(text);
    let mut found = Vec::new();
    let mut rest = flat.as_str();
    while let Some(at) = rest.find("`spec/") {
        let after = &rest[at + 1..];
        let Some(close) = after.find('`') else { break };
        let document = after[..close].to_string();
        let tail = &after[close + 1..];
        rest = tail;
        // The quotation follows immediately, as `: *words*`. Anything else is prose that
        // merely mentions the file, and prose is not a claim about its wording.
        let Some(opened) = tail.strip_prefix(": *") else {
            continue;
        };
        let Some(shut) = opened.find('*') else {
            continue;
        };
        let quoted = opened[..shut].trim().to_string();
        if !quoted.is_empty() {
            found.push((document, quoted));
        }
        rest = &opened[shut..];
    }
    found
}

/// Punctuation a quotation may reasonably trim, and the ellipsis that marks a cut.
fn comparable(quoted: &str) -> Vec<String> {
    // A quotation may elide a middle with an ellipsis, in which case each part has to
    // appear rather than the whole.
    quoted
        .split('…')
        .flat_map(|part| part.split("..."))
        .map(|part| {
            bare(part)
                .trim_end_matches(['.', ',', ';', ':'])
                .trim()
                .to_string()
        })
        .filter(|part| !part.is_empty())
        .collect()
}

#[test]
fn every_quotation_of_the_specification_says_what_it_says_now() {
    let mut checked = 0usize;
    let mut wrong = Vec::new();
    let mut documents = BTreeSet::new();

    for path in sources() {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for (document, quoted) in quotations(&text) {
            let spec = root().join(&document);
            let Ok(source) = std::fs::read_to_string(&spec) else {
                wrong.push(format!(
                    "{}\n  quotes `{document}`, which is not a file",
                    path.display()
                ));
                continue;
            };
            documents.insert(document.clone());
            let flat = bare(&source);
            checked += 1;
            for part in comparable(&quoted) {
                if !flat.contains(&part) {
                    wrong.push(format!(
                        "{}\n  attributes to {document}: \"{part}\"\n  which {document} does not say",
                        path.display()
                    ));
                }
            }
        }
    }

    assert!(
        wrong.is_empty(),
        "{} quotation(s) of the specification are of wording it no longer has:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );

    // The test has to be able to fail. If the convention were ever renamed, this would
    // quietly check nothing and pass forever, which is the failure mode of every scanner.
    assert!(
        checked >= 8,
        "only {checked} quotations found; the convention has probably changed \
         and this test has stopped watching anything"
    );
    assert!(
        documents.len() >= 2,
        "quotations of only {documents:?}; expected several spec documents"
    );
}
