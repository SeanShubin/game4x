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

use game_console::containment::Description;
use game_console::{declare, state};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

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
    // **Sixteen since `P-522`**, which cut the Biomes section, the force rule and every
    // recipe that produced, spent or swept a force - nine blocks under five names of
    // their own. Sean cut force from the first release; `spec/control.md` keeps it.
    assert_eq!(
        table.len(),
        16,
        "sixteen kinds in the release when this was written; it has {} ({table:?})",
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
        // **A kind's line is its name, its family, and every trait it carries** - `P-470`,
        // which inverted the *Of* column onto the kinds. This asserted `["name"]` or
        // `["family", "name"]`, which was the shape before that.
        //
        // **The prose column still stays prose**, which is what the assertion was really
        // guarding and what rule 7 says. So `What it is` must not appear, and the way to say
        // that without listing the traits here is that every key is `name`, `family`, or a
        // trait the release declares.
        let declared: std::collections::BTreeSet<String> =
            game_console::recipes::body_under(&document, "## Traits")
                .iter()
                .map(|row| {
                    row.first()
                        .map(String::as_str)
                        .unwrap_or_default()
                        .trim()
                        .trim_matches('*')
                        .trim()
                        .replace(' ', "-")
                })
                .collect();
        // **This test holds two of the three population counts, and that is worth knowing when
        // a promotion moves them.** `P-476` took the Traits table from twenty-four rows to
        // twenty-six; three literal `24`s had to move, two of them in this test - here, and at
        // the cross-check against `spec/data/traits.4x` below.
        //
        // **Only two failures were reported and there were three wrong numbers.** An assertion
        // after a failing one in the same test is not a check that passed: it is a check that
        // did not run. Demonstrated rather than reasoned - set both back to twenty-four and
        // only this one reports; fix this one and the other appears.
        //
        // **So a count of failing assertions is not a count of wrong assertions**, which is
        // the same shape as a reader nobody calls: absence of a failure is not evidence, and
        // the gate cannot tell the two apart.
        // **Twenty-three since `P-522`.**
        assert_eq!(
            declared.len(),
            23,
            "the Traits table is the population here"
        );
        for key in row.traits.keys() {
            assert!(
                key == "name" || key == "family" || declared.contains(key),
                "`{name}` carries `{key}`, which is neither its name, its family, nor a trait \
                 the release declares - `spec/console.md`: every word in a data file is a kind, \
                 a trait, or one of a trait's values"
            );
        }
        // **A trait is named and not valued**, which is the form `P-462` gave a kind's line and
        // `0130c0e` gave the reader. `name` and `family` carry values and nothing else does.
        for (key, value) in &row.traits {
            let valued = key == "name" || key == "family";
            assert_eq!(
                valued,
                !value.is_empty(),
                "`{name}` writes `{key}` {}, and a trait of the kind is written with its value \
                 while a trait of the thing is written with its name",
                if value.is_empty() { "bare" } else { "valued" }
            );
        }
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
    // **Sixteen since `P-522`**, which cut `garrison`, `nature` and `force`. The file lists
    // twenty names and four of them are the vocabulary's own - `kind`, `trait`, `family`,
    // `value` - which the filter above sets aside.
    assert_eq!(
        from_file.len(),
        16,
        "sixteen compared, and the count is here so that two empty sets cannot agree"
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
    // **The generator writes a kind's name and its family, and the file writes its traits
    // too.** `P-473` deleted the release's *Of* column, which is where the trait names were
    // inverted from - so `spec/data/kinds.4x` now states something the release does not, and
    // `declare::kinds` is a generator of the half that is still derivable.
    //
    // **So the comparison is over that half, exactly, rather than loosened to fit.** Each
    // line of the file is stripped of its bare traits and written back through the same
    // writer, and the result must be the generator's bytes. A name or a family differing
    // still fails here, which is what this check was for.
    let without_traits: Vec<Description> = read
        .iter()
        .map(|row| Description {
            kind: row.kind,
            traits: row
                .traits
                .iter()
                .filter(|(_, value)| !value.is_empty())
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
        })
        .collect();
    assert_eq!(
        declare::kinds(&document),
        state::declared(&without_traits),
        "`declare::kinds` and the names and families in `spec/data/kinds.4x` have parted"
    );

    // **And the half the release no longer states is checked against the file that does.**
    // Every bare name on a kind's line is a trait `spec/data/traits.4x` declares - which is
    // `spec/console.md`'s rule that every word in a data file is a kind, a trait, or one of a
    // trait's values, over the one relation that used to be checked by byte equality.
    let traits_file = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/traits.4x"),
    )
    .expect("spec/data/traits.4x");
    let declared_traits: BTreeSet<String> = state::declarations(&traits_file)
        .expect("the file of traits parses")
        .iter()
        .filter_map(|row| row.traits.get("name").cloned())
        .collect();
    assert_eq!(
        declared_traits.len(),
        23,
        "twenty-three traits are declared"
    );

    // **Read from `carries.4x` since `P-497`, and it is the same join by a different route.**
    // A kind's line held the traits it carries as bare words; they are one row each now, and
    // the rule *every word in a data file is a kind, a trait, or one of a trait's values* is
    // asked of both ends of the row rather than of a word on a kind's line.
    let carries_file = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/carries.4x"),
    )
    .expect("spec/data/carries.4x");
    let carries = state::declarations(&carries_file).expect("the file of carries parses");
    let kinds: BTreeSet<String> = read
        .iter()
        .filter_map(|row| row.traits.get("name").cloned())
        .collect();

    let mut mentions = 0;
    for row in &carries {
        let kind = row.traits.get("kind").expect("a carries row names a kind");
        let carried = row
            .traits
            .get("trait")
            .expect("a carries row names a trait");
        assert!(
            kinds.contains(kind),
            "`carries.4x` says `{kind}` carries `{carried}` and `kinds.4x` declares no `{kind}`"
        );
        assert!(
            declared_traits.contains(carried),
            "`{kind}` carries `{carried}` and `spec/data/traits.4x` does not declare it"
        );
        mentions += 1;
    }
    // **Forty-five since `P-476`, and the deposit is where the two arrived.** Twenty-six
    // traits; `keeps` is `of:thing` and is on no kind's line; the other twenty-five are
    // carried by between one and six kinds each. **The deposit names four where it named
    // two** - `capacity`, `density`, `free`, `occupied` - because `room` became the three
    // that `spec/logistics.md` says describe one bound.
    //
    // **A count, because every name being declared is satisfied by a file that names none.**
    // **Still forty-five after `P-497` moved them**, which is the normalization doing what it
    // claims: the same facts in a shape with no repeating group, and the number that says so
    // is this one being unchanged by a change that rewrote every line.
    //
    // **Thirty-seven since `P-522`**, which is eight rows: `defending` on a citizen, an ark and
    // a pioneer; `binding`, `metal-in-it` and `strength` on a garrison; `biome` on a territory;
    // and `met` on a nature. **Every one of those names a trait or a kind the release stopped
    // declaring**, so this is the same number doing the same job rather than a literal moved to
    // make a test pass.
    //
    // **It is the count that found them**, and by the slower of the two routes available.
    // Deleting `defending` from `traits.4x` left three rows here referring to it, and
    // `carries.4x` is the one file under `spec/data/` with no generator - so regenerating the
    // derived files caught none of it, and what said so was a kind's line naming a trait that
    // is not declared. Re-derived here before being written down: thirty-seven rows in the
    // file, thirty-seven `trait:` references, one per row.
    assert_eq!(
        mentions, 37,
        "thirty-seven trait names across `carries.4x`; this read {mentions}"
    );
    // **Seven memberships, in `member.4x` rather than on a kind's line** - `P-497` again, and
    // the count is the one that was here when `family:` was a key.
    let members_file = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/member.4x"),
    )
    .expect("spec/data/member.4x");
    assert_eq!(
        members_file.matches("family:").count(),
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

/// A kind's line may name a trait of the thing and not value it, and the file round-trips.
///
/// **`spec/console.md`: *a trait of the kind is written with its value and a trait of the
/// thing with its name*.** So a territory's line reads `{kind biome family:place id name:territory
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
    // **`name` leads, then `family`, then the traits with `id` first** - `P-483`. It read
    // `{kind biome family:place id name:territory nature}` until that landed, which was the
    // alphabet: `biome` led a line about `territory`.
    let file = "{kind name:territory family:place id biome nature}\n";
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
///
/// # The release cut its subject, and the cut is what is checked now
///
/// **`P-522` deleted the Biomes section**, so there is no table to hold the file against. Sean
/// cut biomes from the first release; `spec/planet.md` keeps every one of them, so this comes
/// back rather than being wrong.
///
/// **The cut is asserted whole rather than the test being deleted or pointed at `spec/`.**
/// Deleting it loses work that returns with the feature. Pointing it at `spec/` would assert
/// that this crate implements something the release says it does not, which is a different
/// failure wearing the same colour. So what is left is the statement the release actually
/// makes: **there is no Biomes table, and nothing anywhere in the release mentions a biome.**
///
/// **Half a cut is what an empty hand list hides**, and it is the thing this can still catch: a
/// release that deleted the table while leaving a trait saying *one of the biomes*, or a `biome`
/// column in *Kinds*, fails here. So does the day the table returns, which is when the rest of
/// this test is wanted again and is sitting in the history one commit back.
///
/// `spec/data/biomes.4x` is left alone, and deliberately. It is the specification lane's, its
/// generator refuses to run against an empty table - correctly - and a data file for a feature
/// that is out of scope for one release costs nothing by being there.
#[test]
fn the_biomes_file_carries_nature_and_leaves_the_guiding_numbers_out() {
    let document = release();
    let declared: Vec<String> = game_console::recipes::body_under(&document, "## Biomes")
        .iter()
        .map(|row| row[0].trim().trim_matches('*').trim().to_lowercase())
        .collect();
    if declared.is_empty() {
        // **The tables and not the prose.** A release that deleted `## Biomes` while leaving a
        // Traits row saying *one of the biomes*, or a `biome` kind, or a territory whose
        // description still names one, is half-cut - and every one of those is a table cell.
        //
        // **Its prose may still say the word and that is not a fault**: `R-4` is a capability
        // that was delivered and then put out of scope, and a release explaining why something
        // went is not the release stating it. Sweeping the prose too found eight such lines
        // and every one of them was a record rather than a rule.
        let left: Vec<&str> = document
            .lines()
            .filter(|line| line.trim_start().starts_with('|'))
            .filter(|line| line.to_lowercase().contains("biome"))
            .collect();
        assert!(
            left.is_empty(),
            "the release has no Biomes table and {} of its table rows still say `biome`, so \
             the section was cut and something that read from it was not: {left:?}",
            left.len()
        );
        return;
    }

    let at = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/biomes.4x");
    let file = std::fs::read_to_string(&at)
        .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
    let read = state::declarations(&file)
        .unwrap_or_else(|why| panic!("{} does not parse: {why}", at.display()));

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

    // **Twenty-six since `P-476`, and `room` becoming three is why.** `P-474` made the
    // deposit's bound the room left; `P-477` and `P-476` name all three - `capacity`,
    // `occupied` and `free` - because `spec/logistics.md` says **three names describe it and
    // there are two facts**, and any two give the third.
    //
    // **`P-470` is why it was twenty-four rather than twenty-one.** `P-457` declared a derived
    // trait only where a recipe named it, which left three out; then a kind's line gained the
    // traits it carries, and a kind may only name a declared trait.
    // **Twenty-three since `P-522`**, which cut `defending`, `biome` and `met` with the
    // force rule and the Biomes section.
    assert_eq!(
        read.len(),
        23,
        "twenty-three traits, one per row of the release's table; this read {}",
        read.len()
    );
    assert_eq!(
        read.len(),
        game_console::recipes::body_under(&document, "## Traits").len(),
        "the file and the table declare a different number of traits"
    );
    let named: Vec<&str> = read
        .iter()
        .filter_map(|row| row.traits.get("name"))
        .map(String::as_str)
        .collect();
    // **`kept:nothing` is gone and these five are why it was there.** They were the derived
    // traits, and `P-476` removed the distinction: `kept` says where a value belongs and never
    // whether one is held. So each of them now belongs somewhere, and the assertion is that
    // none of them is excused from saying where.
    //
    // **Named rather than counted**, because the point is which five stopped being a category
    // rather than how many there are.
    //
    // **`unpaid` left and `free` arrived, and the swap is `P-498` rather than a correction
    // here.** `unpaid` was derived - the count of citizens `upkeep` could not feed - and is
    // `paid` now, a mark a rule puts, which is the whole of what that promotion did. `free`
    // was always derived and was missing from this list; the release's own *Values* cells are
    // where both were read from, and the five below are exactly the cells carrying a colon.
    let mut arrived = 0;
    for derived in ["surplus", "metal-in-it", "control", "binding", "free"] {
        assert!(
            named.contains(&derived),
            "`{derived}` is derived, a kind's line names it, and a kind may only name a \
             declared trait"
        );
        let row = read
            .iter()
            .find(|row| row.traits.get("name").map(String::as_str) == Some(derived))
            .expect("just asserted present");
        let kept = row.traits.get("kept").map(String::as_str);
        assert!(
            kept == Some("thing") || kept == Some("kind"),
            "`{derived}` says `kept` {kept:?}, and `P-476` left two: a value belongs to each \
             thing or to the kind"
        );
        arrived += 1;
    }
    assert_eq!(
        arrived, 5,
        "the five that were `kept:nothing`, each checked for both things"
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
        // **Three keys, and a fourth only for a trait of every kind** - `P-471`:
        // *a trait of every kind is the one exception, and says so with `of:thing`, because
        // there is no kind for it to belong to and no family that could hold it.* Asserted as
        // the pair rather than as a range, so a stray `of` on an ordinary trait fails here
        // instead of widening the rule.
        let name = row
            .traits
            .get("name")
            .map(String::as_str)
            .unwrap_or_default();
        let of = row.traits.get("of").map(String::as_str);
        assert_eq!(
            (row.traits.len(), of),
            if name == "keeps" {
                (4, Some("thing"))
            } else {
                (3, None)
            },
            "`{name}` carries {} keys and `of` {of:?}, where a trait's line is its name, what              it admits and how it is kept - plus `of:thing` where it is of every kind",
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

    // **Twenty-three of the twenty-four lines, and the twenty-fourth named.** `P-473` deleted
    // the release's *Of* column, which is where `of:thing` was read from - so `keeps` is the
    // one line the generator cannot write and the file states on its own. **The difference is
    // asserted rather than tolerated**: a byte comparison with one exception is a byte
    // comparison, and a byte comparison with a tolerance is not.
    //
    // **This is the direction reversing, seen from the code.** Until tonight the file was
    // derivable from the release and the diff said the promotion was faithful. It is now the
    // release that is short a fact, and what is checked is how far the derivation still
    // reaches - twenty-three lines, and the reader can see which one it does not.
    let written = declare::traits(&document);
    let generated: Vec<&str> = written.lines().collect();
    let stated: Vec<&str> = file.lines().collect();
    assert_eq!(
        generated.len(),
        stated.len(),
        "the generator writes {} lines and the file has {}",
        generated.len(),
        stated.len()
    );
    let differing: Vec<(&str, &str)> = generated
        .iter()
        .zip(&stated)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| (*a, *b))
        .collect();
    assert_eq!(
        differing,
        [(
            "{trait name:keeps admits:number kept:thing}",
            "{trait name:keeps of:thing admits:number kept:thing}"
        )],
        "`spec/data/traits.4x` and what the release still states differ in more than `keeps`'s \
         `of:thing`, which is the one fact `P-473` left the release unable to say"
    );

    // **The counts the release's own column gives**, so a generator inventing a `kept` would
    // fail here rather than at the shape.
    let kept_by = |what: &str| {
        read.iter()
            .filter(|row| row.traits.get("kept").map(String::as_str) == Some(what))
            .count()
    };
    // **Sixteen and seven since `P-522`**: `defending`, `biome` and `met` were all kept by
    // each thing, so the kind's seven are untouched and only the first number moved.
    assert_eq!(
        (kept_by("thing"), kept_by("kind"), kept_by("nothing")),
        (16, 7, 0),
        "sixteen belong to each thing, seven to the kind, and **none to nothing** - `P-476`          removed the third, because `kept` says where a value belongs and never whether one          is held. The zero is asserted rather than dropped, so a `nothing` reaching the          file fails here"
    );
}

/// Every word in every data file is a kind, a trait, or one of a trait's values.
///
/// **`spec/console.md` states this of a data file and it was only ever asked of one.** The
/// check beside it reads `spec/data/kinds.4x`, because when it was written that was the file
/// with bare words on its lines. `P-497` made `spec/data/` twelve relations and the rule did
/// not follow, so eleven files went unswept - and the sweep finds something in the twelfth.
///
/// # What it finds, and why a vocabulary check nearly missed it
///
/// A cell of `line.4x` can hold a quantity that is a sentence:
///
/// ```text
/// {line block:work seq:5 role:produce qty:`$where`'s density for that resource kind:resource}
/// ```
///
/// **A key takes one token**, so this reads ``qty:`$where`'s`` and leaves `density`, `for`,
/// `that` and `resource` as bare words. The quantity the release states has become one token
/// of itself, and nothing said so.
///
/// **Two of the four fragments are real trait names.** `density` and `resource` are declared,
/// so a check asking *is this token a declared trait* passes on half the sentence and fails on
/// the other half. **Had the sentence used only words that happen to be traits, this would be
/// green and wrong**, which is why the count below is of bare words rather than of failures.
///
/// **There were eight until `P-522`**, two each in `muster` and `stand` - *that citizen's
/// strength* and *that unit's strength* - and the cut took both rules with the force rule. The
/// four that are left are all one row's.
///
/// `C-120` carries it. The rows are the specification lane's; the sweep is this lane's.
#[test]
fn every_bare_word_in_every_data_file_is_a_declared_trait() {
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data");
    let declared: BTreeSet<String> = state::declarations(
        &std::fs::read_to_string(data.join("traits.4x")).expect("spec/data/traits.4x"),
    )
    .expect("the file of traits parses")
    .iter()
    .filter_map(|row| row.traits.get("name").cloned())
    .collect();
    assert_eq!(
        declared.len(),
        23,
        "twenty-three traits are declared; this read {declared:?}"
    );

    let mut files: Vec<PathBuf> = std::fs::read_dir(&data)
        .expect("spec/data")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|it| it == "4x").unwrap_or(false))
        .collect();
    files.sort();
    assert_eq!(
        files.len(),
        12,
        "twelve relations since `P-497`; this swept {files:?}"
    );

    let mut bare = 0;
    let mut wrong: Vec<String> = Vec::new();
    for path in &files {
        let name = path
            .file_name()
            .and_then(|it| it.to_str())
            .unwrap_or_default();
        let text = std::fs::read_to_string(path).unwrap_or_else(|why| panic!("{name}: {why}"));
        for row in state::declarations(&text).unwrap_or_else(|why| panic!("{name}: {why}")) {
            for (word, value) in &row.traits {
                if !value.is_empty() {
                    continue;
                }
                bare += 1;
                if !declared.contains(word) {
                    wrong.push(format!(
                        "{name}: a `{}` row carries bare `{word}`",
                        row.kind
                    ));
                }
            }
        }
    }

    // **The population is bare words and not files**, because eleven of the twelve have none
    // and a sweep over them would agree with anything. **Four since `P-522`**: `muster` and
    // `stand` carried two each and went with the force rule, so what is left is `work`'s four
    // - all of them in `line.4x`, and the other eleven relations still have none.
    assert_eq!(
        bare, 4,
        "four bare words across `spec/data/`, which is the population this counted against"
    );
    // **The four are excepted by name and the exception is the failing half of the report** -
    // `C-61`'s pattern, and the reason it is a set rather than a count is that both directions
    // have to bite. A fifth appearing fails here; the four being repaired fails here too, and
    // that is when the exception comes out rather than being widened.
    //
    // **They are one cell of `line.4x` and not two words of it.** *`$where`'s density for that
    // resource* is a quantity the release states as a phrase, and a key takes one token.
    //
    // **`citizen's` and `unit's` were excused here until `P-522`**, from *that citizen's
    // strength* and *that unit's strength* in `muster` and `stand`. Both rules went with the
    // force rule, so two of the four came out - the exception shrinking because the release
    // moved rather than because anything was repaired, which is worth the difference in words. `C-120` is open on what the
    // data should say instead, which is a rule and therefore the specification lane's.
    let excused: BTreeSet<String> = [
        "line.4x: a `line` row carries bare `for`",
        "line.4x: a `line` row carries bare `that`",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let found: BTreeSet<String> = wrong.into_iter().collect();
    assert_eq!(
        found, excused,
        "a word in a data file is neither a kind, a trait, nor a trait's value - \
         `spec/console.md` says every word is one of the three, and the two in the second set \
         are `C-120`'s, excused until the rows say a quantity in one token"
    );
}

/// `spec/data/block.4x` is what the release implies, byte for byte.
///
/// **`P-497` derived the four release-derived relations with a script that cannot be re-run.**
/// It read a kind's traits from `spec/data/kinds.4x`, and `P-497` itself made that file bare
/// declarations - so it now derives no `carries` row at all and asserts out. A migration whose
/// output cannot reproduce its own input is `C-123`'s ninth in a different place: the answer
/// exists only as the bytes somebody once produced.
///
/// # Bytes, because counts cannot ask whether a row is right
///
/// **The original asserted that every row arrived and every count matched, and passed in full
/// while two of the seven relations were wrong** - the specification lane's own account of it.
/// A count is a statement about how many, and being wrong about a row does not change how many
/// there are.
///
/// So this compares the generated text with the committed file and nothing else. **A row whose
/// id, recipe or owner differs fails**, and so does a row in the wrong place, because a
/// relation's file has an order even though a relation does not.
///
/// **The other three are covered by `every_release_derived_relation_round_trips`**, which is
/// the sentence that used to say they were not. Naming a gap is what closes it: a partial
/// generator silent about its gaps reads exactly like a whole one.
#[test]
fn the_blocks_the_release_implies_are_the_blocks_in_the_file() {
    let document = release();
    let generated = declare::blocks(&document);
    let committed = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/data/block.4x"),
    )
    .expect("spec/data/block.4x");

    // **Both sides non-empty before comparing**, or two failures to read agree with each other.
    assert_eq!(
        generated.lines().count(),
        27,
        "twenty-seven blocks since `P-522`; the generator wrote {}",
        generated.lines().count()
    );
    assert_eq!(
        generated, committed,
        "`declare::blocks` and `spec/data/block.4x` have parted"
    );

    // **And the ids are unique and derived**, which `gathered` asserts for itself - the repeat
    // here is deliberate, because that assertion lives in the generator and this is the check
    // that the generator was run at all.
    let ids: BTreeSet<String> = declare::gathered(&document)
        .into_iter()
        .map(|block| block.id)
        .collect();
    assert_eq!(ids.len(), 27, "two blocks share an id");
    // `refresh` is the name that needs all three parts, and `discard` the one that needs two.
    assert!(
        ids.contains("refresh-extractor-working"),
        "a repeated name is spelled the same way for every one of its blocks"
    );
    // **`discard-force` was the example here until `P-522` cut the block.** `discard` still
    // sweeps four kinds, so the rule it demonstrates is unchanged and only the witness moved -
    // which is the shape of a test whose subject a release cut, in the one case where the
    // subject survived and the example did not.
    assert!(
        ids.contains("discard-fertility"),
        "a name repeated over kinds is qualified by the kind"
    );
    assert!(
        ids.contains("work"),
        "a name stated once keeps its own name"
    );
}

/// All four release-derived relations are what the release implies, byte for byte.
///
/// **This covers what `the_blocks_the_release_implies_are_the_blocks_in_the_file` named as not
/// covered**, and that naming is why it got covered: a partial generator silent about its gaps
/// reads exactly like a whole one, which is the same failure as a count reading like a check.
///
/// **Four relations and four counts, asserted before the comparison** - not as the check, but
/// so that two failures to read cannot agree with each other. The comparison itself is bytes.
#[test]
fn every_release_derived_relation_round_trips() {
    let document = release();
    let at = |name: &str| {
        std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../spec/data")
                .join(name),
        )
        .unwrap_or_else(|why| panic!("cannot read spec/data/{name}: {why}"))
    };

    // **The shape `P-522` left.** Each is the population its relation is derived over, and
    // each is written down rather than counted from the generator it checks.
    for (name, generated, expected) in [
        ("block.4x", declare::blocks(&document), 27),
        ("line.4x", declare::lines(&document), 68),
        ("constraint.4x", declare::constraints(&document), 18),
        ("for.4x", declare::fors(&document), 6),
    ] {
        assert_eq!(
            generated.lines().count(),
            expected,
            "`{name}` should have {expected} rows and the generator wrote {}",
            generated.lines().count()
        );
        assert_eq!(
            generated,
            at(name),
            "`declare` and `spec/data/{name}` have parted"
        );
    }

    // **And the four relations `carries`, `member`, `limit` and `above` are not derived from
    // the release at all**, which is said here because a reader of this test would otherwise
    // take `spec/data/` to be generated whole. They state what no table in the release does -
    // which kinds carry which traits, and which orbit is above which territory - and the
    // release lost the columns they came from. Nothing here can check them.
    for name in ["carries.4x", "member.4x", "limit.4x", "above.4x"] {
        assert!(
            !at(name).is_empty(),
            "`spec/data/{name}` is empty, and it is not derived from the release so nothing \
             here would have noticed"
        );
    }
}
