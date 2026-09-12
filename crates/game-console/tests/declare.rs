//! The Kinds table written in the notation, and read back as the same set - `C-97`, `S-110`.
//!
//! **`S-110` asked for this before anything else**: *a comparison cell for cell, failing in
//! both directions, is what makes "go with the data we have been using" verifiable rather than
//! trusted*. Sean said the data is the data we already have, so the transcription invents
//! nothing - and this is what says so mechanically rather than on anyone's word.
//!
//! **Both directions, because they fail differently.** A kind in the release and not in the
//! file is a word that would go missing when the table is deleted. A kind in the file and not
//! in the release is a word this lane invented - which is the one thing `C-49` said a
//! transcription must not do, since a transcription that becomes canonical is a promotion by
//! the wrong lane.

use game_console::{declare, state};
use std::path::Path;

fn release() -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release")
}

/// Every kind the release declares is in the file, and every kind in the file is declared.
///
/// # It reads the specification's file, and it did not at first
///
/// **This compared `declare::kinds` against the release** - the emitter's output against the
/// table the emitter reads. Both sides came from this lane, so it could only ever have caught
/// the emitter disagreeing with itself. **`spec/data/kinds.4x` exists now**, and it is the
/// population that matters: it is what the specification states, and what a reader of the
/// game reads.
///
/// **That is the rule from `docs/process.md`** - *a check that reads a copy of the population
/// is checking the copy* - firing on a check written an hour before the file landed. The
/// emitter is still held, below, but as a **second** assertion rather than the subject.
#[test]
fn the_file_of_kinds_and_the_release_declare_the_same_words() {
    let document = release();
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/kinds.4x");
    let file = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    let read = state::declarations(&file)
        .unwrap_or_else(|why| panic!("{} does not parse: {why}", at.display()));

    // What the release says, read a second time by the route the rest of this crate uses.
    let table: Vec<String> = game_console::recipes::body_under(&document, "## Kinds")
        .iter()
        .map(|row| row[0].trim().trim_matches('*').trim().to_string())
        .collect();
    assert_eq!(
        table.len(),
        18,
        "eighteen kinds in the release when this was written; it has {} ({table:?})",
        table.len()
    );

    // Every line is a `kind` naming one word, so the file declares and does not describe.
    let mut named: Vec<String> = Vec::new();
    for row in &read {
        assert_eq!(
            row.kind, "kind",
            "a line of the file declares a `{}`, where a declaration is a `kind`",
            row.kind
        );
        let name = row
            .traits
            .get("name")
            .expect("a declaration names the word it declares");
        assert_eq!(
            row.traits.len(),
            1,
            "`{name}` carries {} traits, and a declaration carries only the name - the prose \
             column stays prose, which is what rule 7 says",
            row.traits.len()
        );
        named.push(name.clone());
    }

    // **The three the file introduces before it uses them** - `P-443` puts them here rather
    // than in the release's table, because that table becomes a copy of this file.
    assert_eq!(
        &named[..3],
        &declare::VOCABULARY,
        "the file must declare `kind`, `trait` and `family` before a line uses `kind`"
    );

    // Both directions, over the whole set.
    let from_file: std::collections::BTreeSet<&str> = named
        .iter()
        .map(String::as_str)
        .filter(|name| !declare::VOCABULARY.contains(name))
        .collect();
    let from_release: std::collections::BTreeSet<&str> = table.iter().map(String::as_str).collect();

    let missing: Vec<&&str> = from_release.difference(&from_file).collect();
    assert!(
        missing.is_empty(),
        "the release declares {missing:?} and the file does not, so deleting the table would \
         lose them"
    );
    let invented: Vec<&&str> = from_file.difference(&from_release).collect();
    assert!(
        invented.is_empty(),
        "the file declares {invented:?} and the release does not - a word this lane invented, \
         which is what `C-49` says a transcription may never do"
    );
    assert_eq!(
        from_file.len(),
        18,
        "eighteen compared, and the count is here so that two empty sets cannot agree"
    );

    // **And the generator still writes what landed**, byte for byte. This is the second
    // assertion rather than the first: what the specification says is the subject, and this
    // says the handover has not drifted - so `--example declared-kinds` can still be trusted
    // to produce the next version of the file rather than something that merely resembles it.
    assert_eq!(
        declare::kinds(&document),
        file,
        "`declare::kinds` and `spec/data/kinds.4x` have parted, so the generator would          promote bytes that are not what is there"
    );
}

/// A line the reader takes is a line the writer wrote, and neither accepts a state.
///
/// **The refusals are what make the reader a reader of declarations** rather than of anything
/// shaped roughly right. Each is asserted with its own reason, because a loop over inputs that
/// only checks `is_err` passes when they all fail for the same wrong one.
#[test]
fn a_declaration_carries_no_quantity_and_is_in_nothing() {
    assert!(
        state::declarations("{kind name:citizen}\n{kind name:garrison}\n").is_ok(),
        "two ordinary declarations"
    );
    assert!(
        state::declarations("# a comment\n\n{kind name:citizen}\n").is_ok(),
        "a comment and a blank line are not lines"
    );

    for (text, why) in [
        ("{kind name:citizen} -> 1\n", "carries no quantity"),
        ("  {kind name:citizen}\n", "is in nothing"),
        ("kind name:citizen\n", "not a `{...}` description"),
        ("{kind name:\"a citizen\"}\n", "is quoted"),
    ] {
        let refusal = state::declarations(text).expect_err(&format!("`{text}` must be refused"));
        assert!(
            refusal.contains(why),
            "`{text}` was refused for the wrong reason: {refusal}"
        );
    }
}

/// What the two sides say that the other does not, given a file and a table.
///
/// **Lifted out so both arms can be driven against a document written here**, which is
/// `closed_sets.rs`'s rule - and this lane broke it once. The first check that this bites was
/// done by appending a line to `spec/data/kinds.4x` and running the suite. That is a file this
/// lane may not write, and the other lanes read the working tree, so for as long as it took
/// the specification declared a kind nobody promoted.
///
/// **Restoring it afterwards is not what makes it safe.** The window is the thing, and the way
/// to have no window is to poison a document written here.
fn differing(file: &str, table: &[String]) -> (Vec<String>, Vec<String>) {
    let read = state::declarations(file).expect("a file of declarations");
    let from_file: std::collections::BTreeSet<String> = read
        .iter()
        .filter_map(|row| row.traits.get("name"))
        .filter(|name| !declare::VOCABULARY.contains(&name.as_str()))
        .cloned()
        .collect();
    let from_table: std::collections::BTreeSet<String> = table.iter().cloned().collect();
    (
        from_table.difference(&from_file).cloned().collect(),
        from_file.difference(&from_table).cloned().collect(),
    )
}

/// Both arms of the comparison fail, shown on a document written here.
///
/// **The real file is not poisoned to find this out.** `closed_sets.rs` puts it in its own
/// words: it is Sean's file and this lane does not edit it, so the two failures are
/// demonstrated against a document written to carry each.
#[test]
fn a_word_missing_from_either_side_is_reported_against_the_other() {
    let table = vec!["citizen".to_string(), "garrison".to_string()];
    let opening = "{kind name:kind}\n{kind name:trait}\n{kind name:family}\n";

    let agreeing = format!("{opening}{{kind name:citizen}}\n{{kind name:garrison}}\n");
    assert_eq!(
        differing(&agreeing, &table),
        (Vec::new(), Vec::new()),
        "the control: a file and a table that agree differ in nothing, so a failure below is \
         the arm rather than the fixture"
    );

    let short = format!("{opening}{{kind name:citizen}}\n");
    assert_eq!(
        differing(&short, &table).0,
        vec!["garrison".to_string()],
        "a kind the table declares and the file omits would go missing when the table goes"
    );

    let long = format!("{agreeing}{{kind name:invented}}\n");
    assert_eq!(
        differing(&long, &table).1,
        vec!["invented".to_string()],
        "a kind the file declares and the table does not is a word this lane invented"
    );
}
