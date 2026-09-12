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

    // **The generator declares the vocabulary before any line uses it.** `P-443` puts these in
    // the file rather than in the release's table, because that table becomes a copy of this
    // file; `P-451` added `value`, which is the fourth.
    assert_eq!(
        declare::kinds(&document)
            .lines()
            .take(declare::VOCABULARY.len())
            .map(|line| line.trim_start_matches("{kind name:").trim_end_matches('}'))
            .collect::<Vec<_>>(),
        declare::VOCABULARY,
        "a file must declare `kind`, `trait`, `family` and `value` before a line uses one"
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

    // **The generator is ahead of the file, and the gap is named line by line.** `P-448` puts
    // each kind's family on its own line and `P-451` adds `value` to the vocabulary;
    // `spec/data/kinds.4x` has neither yet, and this lane cannot promote it. Reddening the
    // gate for everyone while waiting on a promotion is not the way to say so.
    //
    // **So the difference is asserted as the whole of itself** rather than tolerated: seven
    // lines lose their bare form and gain a family, and one is added. Both sides are listed,
    // so anything drifting for any other reason still fails.
    //
    // **It cannot outlive its excuse** - `C-61`. The day the file lands with these lines, this
    // list stops being the difference, the assertion fails, and it comes out.
    let generated = declare::kinds(&document);
    let has: std::collections::BTreeSet<&str> = generated.lines().collect();
    let had: std::collections::BTreeSet<&str> = file.lines().collect();
    let dropped: Vec<&&str> = had.difference(&has).collect();
    assert_eq!(
        dropped,
        [
            "{kind name:ark}",
            "{kind name:energy}",
            "{kind name:food}",
            "{kind name:metal}",
            "{kind name:orbit}",
            "{kind name:pioneer}",
            "{kind name:territory}",
        ]
        .iter()
        .collect::<Vec<_>>(),
        "the file has lines the generator does not write, and they are not the seven that \
         gained a family"
    );
    let added: Vec<&&str> = has.difference(&had).collect();
    assert_eq!(
        added,
        [
            "{kind family:place name:orbit}",
            "{kind family:place name:territory}",
            "{kind family:resource name:energy}",
            "{kind family:resource name:food}",
            "{kind family:resource name:metal}",
            "{kind family:unit name:ark}",
            "{kind family:unit name:pioneer}",
            "{kind name:value}",
        ]
        .iter()
        .collect::<Vec<_>>(),
        "the generator and the file differ by more than the families `P-448` adds and the \
         `value` `P-451` adds"
    );
    assert_eq!(
        generated.matches("family:").count(),
        7,
        "seven kinds are in a family the release names - two units, three resources, two          places - and `thing` is in none of them because it is a rule rather than a list"
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

/// The families file names every family the release declares, and no other word.
///
/// **`spec/data/families.4x` does not exist yet**, so this holds the generator against the
/// release rather than against the file - and says so, because that is the weaker of the two
/// and `docs/process.md` is why: a check that reads a copy of the population is checking the
/// copy. **The moment the file lands, this reads it**, the way its sibling already does.
///
/// **What it can check today is still worth checking**: that the generator writes every
/// family and invents none, which is what a promotion of these bytes would be promising.
#[test]
fn the_families_file_names_every_family_and_invents_none() {
    let document = release();
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/families.4x");
    assert!(
        !at.exists(),
        "`spec/data/families.4x` exists now, so this must read it instead of the generator - \
         the file is the population and the generator is a copy of it"
    );

    let file = declare::families(&document);
    let read = state::declarations(&file).expect("the file it writes is a file it can read");

    let mut named: Vec<String> = Vec::new();
    for row in &read {
        assert_eq!(
            row.kind, "family",
            "a line of the families file declares a `{}`",
            row.kind
        );
        assert_eq!(
            row.traits.len(),
            1,
            "`P-448`: a family declares only its name, and this line carries {} traits",
            row.traits.len()
        );
        named.push(
            row.traits
                .get("name")
                .expect("a declaration names the word it declares")
                .clone(),
        );
    }

    let declared: Vec<String> = game_console::recipes::body_under(&document, "## Families")
        .iter()
        .map(|row| row[0].trim().trim_matches('*').trim().to_string())
        .collect();
    assert_eq!(
        named, declared,
        "the families file and the release's table name different families, or name them in a \
         different order"
    );
    assert_eq!(
        named.len(),
        4,
        "four families when this was written; the release declares {} ({named:?})",
        named.len()
    );

    // **`thing` is among them and carries no members**, which is the half `P-448` changed.
    // The table spells it *every kind above*; the file says only that the family exists, and
    // `spec/console.md` carries the rule that every kind is in it.
    assert!(
        named.contains(&"thing".to_string()),
        "`thing` is a family and the file must declare it: {named:?}"
    );
    assert!(
        !file.contains("every kind above"),
        "the file carries the table's rule as if it were data: {file}"
    );
}

/// The biomes file names every biome, and carries the one column the release says binds.
///
/// **`spec/data/biomes.4x` does not exist yet**, so this holds the generator against the
/// release and asserts the file's absence - the same shape as its sibling, and for the same
/// reason: the day it lands this fails and gets pointed at the population rather than at a
/// copy of it.
#[test]
fn the_biomes_file_carries_nature_and_leaves_the_guiding_numbers_out() {
    let document = release();
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/biomes.4x");
    assert!(
        !at.exists(),
        "`spec/data/biomes.4x` exists now, so this must read it instead of the generator"
    );

    let file = declare::biomes(&document);
    let read = state::declarations(&file).expect("the file it writes is a file it can read");

    let declared: Vec<String> = game_console::recipes::body_under(&document, "## Biomes")
        .iter()
        .map(|row| row[0].trim().trim_matches('*').trim().to_lowercase())
        .collect();
    assert_eq!(
        declared.len(),
        6,
        "six biomes when this was written; the release declares {} ({declared:?})",
        declared.len()
    );

    let named: Vec<String> = read
        .iter()
        .map(|row| {
            assert_eq!(
                row.kind, "value",
                "a biome is a value, not a `{}`",
                row.kind
            );
            assert_eq!(
                row.traits.get("of").map(String::as_str),
                Some("biome"),
                "`P-451`: a value declares which trait it is one of"
            );
            row.traits
                .get("name")
                .expect("a value names itself")
                .clone()
        })
        .collect();
    assert_eq!(
        named, declared,
        "the biomes file and the release's table name different biomes, or in a different order"
    );

    // **The one column that binds is carried, and the three that guide are not.** The release
    // says which is which: *the numbers here guide and do not bind ... force of nature is the
    // one column that binds*. So this is the release's sentence rather than a choice made in
    // the generator, and it is asserted in both directions.
    let natured = read
        .iter()
        .filter(|row| row.traits.contains_key("nature"))
        .count();
    assert_eq!(
        natured, 5,
        "five biomes have a force of nature and ocean has none, which is the release saying \
         it is not claimable and carries nothing"
    );
    for guiding in ["x", "food", "metal", "energy"] {
        assert!(
            !file.contains(guiding),
            "the file carries `{guiding}`, which is one of the three columns the release says \
             guide and do not bind - a territory's own numbers are in *Territory resources*"
        );
    }

    // **Jungle is the one that is not 1**, so a generator writing a constant would fail here.
    let jungle = read
        .iter()
        .find(|row| row.traits.get("name").map(String::as_str) == Some("jungle"))
        .expect("the release declares a jungle");
    assert_eq!(
        jungle.traits.get("nature").map(String::as_str),
        Some("2"),
        "a jungle's force of nature is two, and every other claimable biome is one - so a \
         constant would pass everything above this line"
    );
}

/// The traits file declares what a data file needs, and nothing a recipe never names.
///
/// **Two derivations of one table, which is `Q-8`'s shape.** `P-457` carries these bytes
/// written by hand; this reads the release. Where they differ, one of them is wrong, and that
/// is worth more than either alone.
///
/// **`spec/data/traits.4x` does not exist yet**, so this holds the generator against the
/// release and asserts the file's absence, the way its two siblings do.
#[test]
fn the_traits_file_declares_what_a_data_file_needs() {
    let document = release();
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/traits.4x");
    assert!(
        !at.exists(),
        "`spec/data/traits.4x` exists now, so this must read it instead of the generator"
    );

    let file = declare::traits(&document);
    let read = state::declarations(&file).expect("the file it writes is a file it can read");

    // **Twenty of the twenty-three, and which three is the whole of the arithmetic.** Four
    // traits are derived; a derived trait is declared only where a recipe names it, because
    // every word in a data file is a kind, a trait or one of a trait's values. `unpaid` is
    // named by `perish`; `surplus`, `metal in it` and `control` are named by no recipe row.
    assert_eq!(
        read.len(),
        20,
        "twenty traits belong in a data file; this wrote {}",
        read.len()
    );
    let named: Vec<&str> = read
        .iter()
        .filter_map(|row| row.traits.get("name"))
        .map(String::as_str)
        .collect();
    for gone in ["surplus", "metal-in-it", "control"] {
        assert!(
            !named.contains(&gone),
            "`{gone}` is derived and named by no recipe row, so it is in no data file"
        );
    }
    assert!(
        named.contains(&"unpaid"),
        "`unpaid` is derived and `perish` names it, so it is declared"
    );

    // **`surplus` is the one the two derivations disagreed about, so it is asserted by the
    // reason rather than by the name.** It appears once under `## Recipes` and that once is
    // the `In` line's prose quoting `spec/turn.md` - no row names it. A count over the
    // section finds it; a count over the rows does not, and the rows are what a recipe is.
    let rows_naming_surplus = game_console::recipes::body_under(&document, "## Recipes")
        .iter()
        .filter(|row| row.iter().any(|cell| cell.contains("surplus")))
        .count();
    assert_eq!(
        rows_naming_surplus, 0,
        "a recipe row names `surplus` now, so it belongs in the file after all"
    );

    // Every line carries the three facts and nothing else.
    let mut open = 0;
    for row in &read {
        assert_eq!(row.kind, "trait", "a trait is declared as a `{}`", row.kind);
        assert_eq!(
            row.traits.len(),
            3,
            "a trait's line is its name, what it admits and how it is kept - {} keys",
            row.traits.len()
        );
        let kept = row
            .traits
            .get("kept")
            .map(String::as_str)
            .unwrap_or_default();
        assert!(
            ["thing", "kind", "nothing"].contains(&kept),
            "`{kept}` is not one of the three the release has"
        );
        if row.traits.get("admits").map(String::as_str) == Some("???") {
            open += 1;
        }
    }
    // **Seven open cells, which is the one question `P-457` puts to Sean.** `0 or 1` five
    // times and `yes or no` three times are the same two-valued set spelled twice - eight
    // cells in the table, and **one of the three `yes or no` traits is `surplus`**, which is
    // in no data file. So dropping it drops an open cell with it, and the question is over
    // seven rather than eight.
    assert_eq!(
        open, 7,
        "seven cells wait on what the notation calls a two-valued set; {open} do"
    );

    // **The counts the release's own column gives**, so a generator inventing a `kept` would
    // fail here rather than at the shape.
    let kept_by = |what: &str| {
        read.iter()
            .filter(|row| row.traits.get("kept").map(String::as_str) == Some(what))
            .count()
    };
    assert_eq!(
        (kept_by("thing"), kept_by("kind"), kept_by("nothing")),
        (15, 4, 1),
        "fifteen stored, four of the kind, and one derived that a recipe names"
    );
}
