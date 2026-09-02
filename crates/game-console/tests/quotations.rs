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
//! A backticked spec path, a colon, and an italic run.
//!
//! **And the same claim written as a sentence**, which is how it is usually written:
//!
//! ```text
//! /// `spec/planet.md` says *the terrain of the realistic drawing is continuous.*
//! ```
//!
//! A colon is not what makes it a quotation - marking words as the specification's own is.
//! `realistic.rs` attributed a sentence to `spec/planet.md` that the specification did not
//! contain, in a file written *after* this guard landed; the guard read that file and said
//! nothing, because it was watching for a colon. A reader trusts *the specification says
//! this* at least as much as a colon, so both forms are checked.
//!
//! Prose *about* the spec is still not a quotation. `spec/x.md` asks for three surfaces is
//! a claim about meaning, which no comparison of text can settle. Only text the author
//! marked as the specification's own words is checked - which is exactly the text a reader
//! would otherwise trust without looking.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every directory the code lane writes in.
///
/// It used to be `crates/` alone, and the very first document written outside it -
/// `prototypes/goldberg-view/README.md` - quoted the specification in the checked form and
/// went unchecked. A guard whose coverage is one directory name will keep having that
/// problem.
///
/// It stops at this lane's own column deliberately. `spec/`, `releases/` and `docs/` belong
/// to the documentation lane, and a stale quotation there is real but is not this lane's to
/// repair - putting them in would red this lane's pre-push gate on a file it must not
/// touch, which is the trap `CLAUDE.md` warns about. Quotations found there get reported
/// instead.
const OURS: [&str; 5] = ["crates", "prototypes", "scripts", "tools", "hooks"];

fn sources() -> Vec<PathBuf> {
    let mut found = Vec::new();
    for directory in OURS {
        collect(&root().join(directory), &mut found);
    }
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
            Some("rs") | Some("md") | Some("html") | Some("sh") | Some("ps1")
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

/// Flattened, and with markup dropped.
///
/// Whether a word is written as code, or bolded, is the quoting medium's formatting rather
/// than the specification's wording: a doc comment marking up `/browser` where the spec did
/// not, or declining to repeat the spec's own **emphasis**, is not a misquotation.
///
/// Backticks and asterisks survive [`flattened`] because that is what the quotations are
/// *found* by, and are dropped here because that is not what they are *judged* by.
fn bare(text: &str) -> String {
    flattened(&text.replace(['`', '*'], " "))
}

/// The prose in a file, as runs, with code left out.
///
/// Only Rust needs separating: in markdown the whole file is prose. In `.rs` an asterisk is
/// a dereference or a multiplication far more often than it is emphasis, so flattening a
/// whole source file and hunting for `*…*` finds spans of code - thirteen of them here, and
/// every one nonsense. Reading only the comments removes that entire class rather than
/// filtering it out afterwards.
///
/// Runs rather than one string, so a mention at the end of one comment cannot attribute
/// words that appear at the start of the next.
fn prose(path: &Path, text: &str) -> Vec<String> {
    if path.extension().and_then(|e| e.to_str()) != Some("rs") {
        // The same paragraph rule, for a document whose every line is prose.
        let kept: Vec<String> = text
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    " . ".to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();
        return vec![flattened(&kept.join("\n"))];
    }
    let mut runs = Vec::new();
    let mut current = String::new();
    for line in text.lines() {
        if line.trim_start().starts_with("//") {
            current.push('\n');
            current.push_str(&marked(line));
        } else if !current.is_empty() {
            runs.push(flattened(&current));
            current.clear();
        }
    }
    if !current.is_empty() {
        runs.push(flattened(&current));
    }
    runs
}

/// A blank line inside a comment, kept as a full stop.
///
/// Flattening erases paragraph breaks, and without them a quotation followed by a new
/// paragraph that opens in bold reads as a second quotation in a list. **This guard found
/// that in itself**: it reported a doc comment as attributing the author's own sentence to
/// `spec/control.md`, one paragraph below a real quotation of it.
///
/// A full stop is the marker because both readers already refuse to cross one - the
/// emphasis scan stops there, and a list continuation allows only a comma, a space and the
/// word `and`.
fn marked(line: &str) -> String {
    let bare = line
        .trim_start()
        .trim_start_matches('/')
        .trim_start_matches('!')
        .trim();
    if bare.is_empty() {
        " . ".to_string()
    } else {
        line.to_string()
    }
}

/// Words that mark what follows as the specification's own, rather than as a claim about it.
///
/// **All present tense, and `said` is deliberately not among them.** This checks quotations
/// against the specification *as it stands*, so it can only judge a claim about what the
/// document says now. `spec/x.md` **said** *…* is a claim about what it used to say, and
/// reading the current file cannot settle that - a record of a wording that has since
/// changed is correct precisely because the file no longer matches it.
///
/// It cost a red gate to notice. `C-7` is a withdrawn finding whose text says
/// `spec/control.md` **said** *every structure that can be built*, which was true when it
/// was filed and stopped being true when `P-125` landed. The guard read the past tense as a
/// present claim and reported the record as a defect. The alternative - editing the
/// quotation to the current wording - would have made the record false, which is worse than
/// the red gate and much harder to notice.
///
/// Deliberately a list rather than a rule. An unrecognised verb leaves a quotation
/// unchecked, which is where this guard already was, and is a smaller failure than the
/// alternative: a rule loose enough to catch every verb also catches the author's own
/// emphasis in a sentence that merely mentions a file. Both were measured against this
/// repository. The loose version reported seven such spans - `**required, not a
/// convenience.**` among them - and every one was correct prose being called a
/// misquotation.
const ATTRIBUTING: [&str; 13] = [
    "says",
    "asks",
    "requires",
    "states",
    "puts",
    "fixes",
    "allows",
    "calls",
    "describes",
    "forbids",
    "names",
    "lists",
    "means",
];

/// What follows the lead, if this mention attributes words rather than discussing the file.
///
/// Three leads count: a colon, a possessive, and an attributing verb.
fn attributed(tail: &str) -> Option<&str> {
    if let Some(rest) = tail.strip_prefix(':') {
        return Some(rest);
    }
    if let Some(rest) = tail.strip_prefix("'s").or_else(|| {
        tail.strip_prefix('\u{2019}')
            .and_then(|r| r.strip_prefix('s'))
    }) {
        return Some(rest);
    }
    let word = tail.split_whitespace().next()?;
    ATTRIBUTING
        .contains(&word.trim_end_matches([',', ':']))
        .then_some(tail)
}

/// The emphasised run at the start of this text, and how far it reached.
///
/// It has to be close and unpunctuated. A sentence that has reached a full stop or a
/// semicolon has stopped being about the file it named, and emphasis after that belongs to
/// whatever came next.
fn emphasised(lead: &str) -> Option<(String, usize)> {
    let mut offset = None;
    for (index, character) in lead.char_indices() {
        if index > 60 || matches!(character, '.' | ';') {
            break;
        }
        if character == '*' {
            offset = Some(index);
            break;
        }
    }
    let opened = &lead[offset?..];
    let (opened, closer) = match opened.strip_prefix("**") {
        Some(rest) => (rest, "**"),
        None => (opened.strip_prefix('*')?, "*"),
    };
    let shut = opened.find(closer)?;
    let quoted = opened[..shut].trim().to_string();
    if quoted.is_empty() {
        return None;
    }
    // Past the closing marker, not up to it. Reading it as an opener made the next
    // asterisk anywhere in the file look like the end of a second quotation, and a whole
    // README came back attributed to `spec/planet.md`.
    Some((quoted, offset? + closer.len() * 2 + shut))
}

/// The next quotation in a list, attributed to the same document as the one before it.
///
/// One mention often carries several: `inspect.rs` names `spec/planet.md` and then quotes
/// three of its lines in a row. Without this only the first is checked, and it was the
/// second and third that a reader would be trusting just as much.
///
/// Only a comma, a space and the word `and` may separate them. Anything else is a sentence
/// that has moved on to its own words.
fn continued(rest: &str) -> Option<(String, usize)> {
    let trimmed = rest.trim_start_matches([',', ' ']);
    let trimmed = trimmed.strip_prefix("and ").unwrap_or(trimmed).trim_start();
    if !trimmed.starts_with('*') {
        return None;
    }
    let skipped = rest.len() - trimmed.len();
    emphasised(trimmed).map(|(quoted, used)| (quoted, skipped + used))
}

/// The quotations in one file: the spec file named, and the words attributed to it.
fn quotations(path: &Path, text: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for run in prose(path, text) {
        let mut rest = run.as_str();
        while let Some(at) = rest.find("`spec/") {
            let after = &rest[at + 1..];
            let Some(close) = after.find('`') else { break };
            let document = after[..close].to_string();
            let tail = &after[close + 1..];
            rest = tail;
            let Some(lead) = attributed(tail) else {
                continue;
            };
            let Some((quoted, consumed)) = emphasised(lead) else {
                continue;
            };
            found.push((document.clone(), quoted));
            rest = &lead[consumed..];
            while let Some((quoted, used)) = continued(rest) {
                found.push((document.clone(), quoted));
                rest = &rest[used..];
            }
        }
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

/// A record of what the specification used to say is not a claim about what it says.
///
/// The distinction this guard turns on, pinned so that adding a verb cannot quietly erase
/// it. Present tense is checkable against the file on disk; past tense is a statement about
/// a file that no longer exists, and reading the current one cannot settle it - a correct
/// record of a changed wording is *exactly* the case where the document will not match.
///
/// This came from a red gate. `C-7` is a withdrawn finding recording that
/// `spec/control.md` **said** *every structure that can be built*, which `P-125` changed.
/// The guard read the past tense as a present claim and reported the record as a defect.
#[test]
fn past_tense_is_a_record_and_present_tense_is_a_claim() {
    let here = Path::new("outbox.md");

    let claim = "`spec/control.md` says *every structure that can be built*";
    assert_eq!(
        quotations(here, claim).len(),
        1,
        "a present-tense attribution is a claim and is checked"
    );

    let record = "`spec/control.md` said *every structure that can be built*";
    assert!(
        quotations(here, record).is_empty(),
        "a past-tense attribution is a record of a wording that has since changed, and \
         the current file cannot settle it"
    );

    // The colon and the possessive stay present-tense claims, which is what they read as.
    assert_eq!(
        quotations(here, "`spec/turn.md`: *a turn has three parts*").len(),
        1
    );
    assert_eq!(quotations(here, "`spec/turn.md`'s *three parts*").len(), 1);
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
        for (document, quoted) in quotations(&path, &text) {
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
        checked >= 40,
        "only {checked} quotations found; the convention has probably changed \
         and this test has stopped watching anything"
    );
    assert!(
        documents.len() >= 2,
        "quotations of only {documents:?}; expected several spec documents"
    );
}
