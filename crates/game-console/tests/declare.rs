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
#[test]
fn the_file_of_kinds_and_the_release_declare_the_same_words() {
    let document = release();
    let file = declare::kinds(&document);
    let read = state::declarations(&file).expect("the file it writes is a file it can read");

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
