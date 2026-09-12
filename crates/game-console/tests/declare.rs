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
        // **A kind's line is its name, and its family where it is in one** - `P-448`. This
        // asserted one trait, which was the shape before the inversion: a family declared its
        // members, so a kind's line had nothing to say but its own name.
        //
        // **The prose column still stays prose**, which is what the one-trait assertion was
        // really guarding and what rule 7 says. So what is checked is the key rather than the
        // count: a kind carries `name`, and `family` where the release puts it in one, and
        // nothing else - `What it is` is a sentence and is in no data file.
        let mut keys: Vec<&str> = row.traits.keys().map(String::as_str).collect();
        keys.sort();
        assert!(
            keys == ["name"] || keys == ["family", "name"],
            "`{name}` carries {keys:?}, and a kind's line is its name and its family - the \
             prose column stays prose, which is what rule 7 says"
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

    // **The generator writes the file, byte for byte.** `P-455` landed `spec/data/kinds.4x`
    // with its families and the `value` line, so the gap this stood in for is closed.
    //
    // **What was here was an exception and it came out rather than being widened** - `C-61`'s
    // pattern, and this is the day it says that happens. While the file was one promotion
    // behind, the assertion named both sides of the difference: seven lines losing their bare
    // form and one added. The moment the file had them, that list stopped being the
    // difference and the assertion failed, which is what an exception that cannot outlive its
    // excuse looks like from inside.
    //
    // **Byte equality is the strongest form and it is now true**, so the handover is checked
    // rather than described: `--example declared-kinds` prints what is in the specification.
    assert_eq!(
        declare::kinds(&document),
        file,
        "`declare::kinds` and `spec/data/kinds.4x` have parted, so the generator would promote \
         bytes that are not what is there"
    );
    assert_eq!(
        file.matches("family:").count(),
        7,
        "seven kinds are in a family the release names - two units, three resources and two \
         places - and `thing` is in none of them, because it is a rule rather than a list"
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

/// A kind's line may name a stored trait and not value it, and the file round-trips.
///
/// **`spec/console.md`: *a trait of the kind is written with its value and a stored one with
/// its name*.** So a territory's line reads `{kind biome family:place id name:territory
/// nature}` - `family` and `name` valued, `biome`, `id` and `nature` named - and the sort is
/// over the trait names whether a value follows or not.
///
/// **Built before a file uses it, because the rule is promoted and the file is not this lane's
/// to write.** `spec/data/kinds.4x` has no such line today; when the specification lane
/// proposes one, the reader is already the reader of it rather than the thing that has to
/// change first. What is asserted here is the form the rule states, not a guess at the file.
///
/// # The flag, and why a state does not get it
///
/// **A state values everything it names**, because a state is about things and the thing is
/// what holds the value. So `read` refuses a bare word and says why, and that refusal is
/// asserted here rather than left to the reader of the parser - a permission given to one
/// caller is only a permission if the other one still refuses.
#[test]
fn a_kind_may_name_a_trait_without_valuing_it() {
    let file = "{kind biome family:place id name:territory nature}\n";
    let read = state::declarations(file).expect("a kind naming three traits and valuing two");
    assert_eq!(read.len(), 1);

    let named: Vec<(&str, &str)> = read[0]
        .traits
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect();
    assert_eq!(
        named,
        [
            ("biome", ""),
            ("family", "place"),
            ("id", ""),
            ("name", "territory"),
            ("nature", ""),
        ],
        "three named and two valued, sorted together by name - the sort is over the trait \
         rather than over which of the two forms it is in"
    );

    // **The bytes come back**, which is what makes this a form the notation carries rather
    // than something the reader tolerates and the writer cannot produce.
    assert_eq!(state::declared(&read), file);

    // **A state refuses it, and says which rule it is on the wrong side of.** Without this the
    // flag is a permission nobody checks the other half of.
    let refusal =
        state::read("{territory biome} -> 1\n").expect_err("a state values what it names");
    assert!(
        refusal.contains("is not `trait:value`") && refusal.contains("this is a state"),
        "refused for the wrong reason: {refusal}"
    );

    // And the other three refusals still hold on a line that uses the new form, so admitting
    // a bare name did not admit a quantity, an indent or a quotation with it.
    let mut checked = 0;
    for (text, why) in [
        ("{kind biome name:territory} -> 1\n", "carries no quantity"),
        ("  {kind biome name:territory}\n", "is in nothing"),
        ("{kind biome name:\"a territory\"}\n", "is quoted"),
    ] {
        let refusal = state::declarations(text).expect_err(&format!("`{text}` must be refused"));
        assert!(
            refusal.contains(why),
            "`{text}` was refused for the wrong reason: {refusal}"
        );
        checked += 1;
    }
    assert_eq!(
        checked, 3,
        "the three refusals, over the form that was added"
    );
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
/// **It reads `spec/data/families.4x`, which `P-455` landed.** Until then it held the
/// generator against the release and **asserted the file's absence**, so that the day the file
/// arrived this failed and had to be pointed at it - the population rather than a copy of it,
/// which is what `docs/process.md` asks and what an absence assertion is for.
#[test]
fn the_families_file_names_every_family_and_invents_none() {
    let document = release();
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/families.4x");
    let file = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    let read = state::declarations(&file)
        .unwrap_or_else(|why| panic!("{} does not parse: {why}", at.display()));

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

    // **And the generator writes the file, byte for byte**, so the handover is checked rather
    // than described: the example that prints these bytes prints what is in the specification.
    assert_eq!(
        declare::families(&document),
        file,
        "`declare::families` and `spec/data/families.4x` have parted"
    );
}

/// The biomes file names every biome, and carries the one column the release says binds.
///
/// **It reads `spec/data/biomes.4x`, which `P-455` landed.** Until then it held the generator
/// against the release and asserted the file's absence, for the reason its sibling gives.
#[test]
fn the_biomes_file_carries_nature_and_leaves_the_guiding_numbers_out() {
    let document = release();
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/biomes.4x");
    let file = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    let read = state::declarations(&file)
        .unwrap_or_else(|why| panic!("{} does not parse: {why}", at.display()));

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

    // **And the generator writes the file, byte for byte.**
    assert_eq!(
        declare::biomes(&document),
        file,
        "`declare::biomes` and `spec/data/biomes.4x` have parted"
    );
}

/// The traits file declares what a data file needs, and nothing a recipe never names.
///
/// **Two derivations of one table, which is `Q-8`'s shape.** `P-457` carries these bytes
/// written by hand; this reads the release. Where they differ, one of them is wrong, and that
/// is worth more than either alone.
///
/// **It reads `spec/data/traits.4x`, which `P-457` and `P-461` landed.** Until then it held the
/// generator against the release and asserted the file's absence, so that the day the file
/// arrived this failed and had to be pointed at it - which is what happened.
#[test]
fn the_traits_file_declares_what_a_data_file_needs() {
    let document = release();
    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/traits.4x");
    let file = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    let read = state::declarations(&file)
        .unwrap_or_else(|why| panic!("{} does not parse: {why}", at.display()));

    // **Twenty-one of twenty-four, and which three is the whole of the arithmetic.** Four
    // traits are derived; a derived trait is declared only where a recipe names it, because
    // every word in a data file is a kind, a trait or one of a trait's values. `unpaid` is
    // named by `perish`; `surplus`, `metal in it` and `control` are named by no recipe row.
    //
    // **Twenty-four rather than twenty-three since `P-461`**, which declared `binding` - the
    // dangling reference this lane verified and the specification lane filed: `metal in it` is
    // *derived: its binding plus the metal in its parts*, and nothing declared `binding`.
    assert_eq!(
        read.len(),
        21,
        "twenty-one traits belong in a data file; this wrote {}",
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
    // **Nothing is open any more, and `P-457` is what closed it.** Seven cells waited on what
    // the notation calls a two-valued set, then two after `spec/turn.md` settled the five
    // action counts. Sean's answer was that nothing in the game is two-valued anywhere: `yes`
    // and `no` appear in no data file, and `movable:1` matches `{citizen defending:1}` which
    // the map form already writes.
    //
    // **Asserted at zero rather than deleted**, so a `???` reaching the specification's file
    // would be caught rather than written. The generator has no other placeholder, so this is
    // also what says it never invented one.
    assert_eq!(
        open, 0,
        "{open} cell(s) in `spec/data/traits.4x` say `???`, which is a placeholder reaching \
         the specification rather than an answer"
    );

    // **And the generator writes the file, byte for byte**, which is the third of the four
    // and the last one to arrive.
    assert_eq!(
        declare::traits(&document),
        file,
        "`declare::traits` and `spec/data/traits.4x` have parted"
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
        (15, 5, 1),
        "fifteen stored, five of the kind, and one derived that a recipe names"
    );
}
