//! The release's declaring tables, written in the notation - `P-443`, answering `C-97`.
//!
//! **Rule 7 puts the game's data in the specification**: *state the game's data in several
//! files in a directory of their own, in the notation rather than in a table*. This is what
//! one of those files says, generated from the release so that the transcription is checked
//! rather than trusted - which is what `S-110` asked for first and what makes Sean's *go with
//! the data we have been using* a verifiable sentence.
//!
//! **`P-443` is what made it writable at all.** `C-97` asked whether a notation whose every
//! word is *a kind, a trait, or one of a trait's values* can carry a **declaration** - a file
//! that says which kinds there are is not a state of a game. The answer was that it needs no
//! second form: `kind`, `trait` and `family` are themselves kinds, so `{kind name:citizen}` is
//! an ordinary description and [`crate::state::declarations`] reads it with the one parser.
//!
//! # One table, and why not yet the other seven
//!
//! **`Kinds` is here because `P-443` wrote the example**, and nothing about its shape is this
//! lane's invention: a row is a name, and the name is the whole of what the notation carries.
//! The *What it is* column stays where rule 7 puts it - *relationships in prose* - so this
//! file is the vocabulary and the sentences remain sentences.
//!
//! **`Families` and `Traits` declare too and are not here**, because each has a cell holding
//! **several values** - a family's members, a trait's `Of` and `Values` - and the notation
//! gives one value to a key. Whether that is several lines, a joined name, or something else
//! is a decision rather than work, and it is filed rather than guessed at.

use crate::recipes::{body_under, plain};
use game_model::containment::Description;

/// The three words a file of kinds needs before it can use any of them.
///
/// **`P-443` puts them in the file rather than in the release's table**, and says why: under
/// `P-440` that table becomes a copy of the data file rather than its source, so adding them
/// there would be work done twice. **A file that used `kind` without declaring it would be
/// using a word it had not introduced**, which is the rule this notation has about every other
/// word.
pub const VOCABULARY: [&str; 4] = ["kind", "trait", "family", "value"];

/// Every kind the release declares, as the file that would declare them.
///
/// **Read from the release's *Kinds* table**, so this is the same data by a different route
/// rather than a second copy of it - which is the whole of why it can be compared.
pub fn kinds(document: &str) -> String {
    let families = families_of(document);
    let mut rows: Vec<Description> = Vec::new();
    for name in VOCABULARY {
        rows.push(named("kind", name, None));
    }
    for row in body_under(document, "## Kinds") {
        let name = plain(row.first().map(String::as_str).unwrap_or_default());
        assert!(
            !name.is_empty(),
            "a row of the Kinds table names no kind, so the file would declare a blank word"
        );
        rows.push(named(
            "kind",
            &name,
            families.get(&name).map(String::as_str),
        ));
    }
    crate::state::declared(&rows)
}

/// One line: a declaration of `what`, named `name`, optionally in a family.
///
/// **Built as the struct rather than through `Description::of`**, which takes a
/// `game_model::thing::Kind` - and `kind` is not one of the model's kinds. That is the
/// distinction `S-112` warned about before this file was written: `citizen` can be in a game
/// state and `kind` cannot, so a declaration names a word the model has no variant for. The
/// model is right not to have one; nothing in a game is ever a `kind`.
fn named(what: &'static str, name: &str, family: Option<&str>) -> Description {
    let mut traits = std::collections::BTreeMap::new();
    traits.insert("name".to_string(), name.to_string());
    if let Some(family) = family {
        traits.insert("family".to_string(), family.to_string());
    }
    Description { kind: what, traits }
}

/// Which family each kind declares, from the *Families* table read backwards.
///
/// **`P-448` inverted the declaration**: a kind declares which family it is in, and a family
/// declares only its name. So the table's `Members` cell, which held a list and is what `C-98`
/// stopped on, is read here and written one value to a line - `{kind name:ark family:unit}`.
///
/// **`thing` is skipped and writes nothing.** `spec/console.md`: *`thing` is the family every
/// kind is in, and no line says so kind by kind - a kind added tomorrow is a `thing` because it
/// is a kind.* The table spells it *every kind above*, which is a rule about the table rather
/// than a list, and a rule belongs in prose where rule 7 puts it.
///
/// **A kind in two families would have nowhere to put the second**, because a key takes one
/// value - so this refuses rather than picking. It cannot happen in this release and the
/// release is not what makes it true.
fn families_of(document: &str) -> std::collections::BTreeMap<String, String> {
    let mut out: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    for row in body_under(document, "## Families") {
        let family = plain(row.first().map(String::as_str).unwrap_or_default());
        let members = row.get(1).cloned().unwrap_or_default();
        if family.is_empty() || members.contains("every kind above") {
            continue;
        }
        for member in members.split(',') {
            let member = plain(member);
            if member.is_empty() {
                continue;
            }
            if let Some(already) = out.insert(member.clone(), family.clone()) {
                panic!(
                    "`{member}` is in `{already}` and in `{family}`, and a kind's line has one \
                     `family` to give - which family it declares would be a choice this file \
                     is not entitled to make"
                );
            }
        }
    }
    assert!(
        !out.is_empty(),
        "no kind is in any family, so the Families table parsed to nothing and every kind \
         would be written as a `thing` alone"
    );
    out
}

/// Every family the release declares, as the file that would declare them.
///
/// **A name and nothing else** - `P-448`. What a family contains is on its members' lines, so
/// this file is the shortest of the three and is the whole of what a family declares.
pub fn families(document: &str) -> String {
    let mut rows: Vec<Description> = Vec::new();
    for row in body_under(document, "## Families") {
        let name = plain(row.first().map(String::as_str).unwrap_or_default());
        assert!(
            !name.is_empty(),
            "a row of the Families table names no family, so the file would declare a blank \
             word"
        );
        rows.push(named("family", &name, None));
    }
    assert!(
        !rows.is_empty(),
        "the Families table parsed to nothing, so this would write an empty file and the \
         comparison against it would agree for the wrong reason"
    );
    crate::state::declared(&rows)
}

/// Every biome the release declares, as the file that would declare them.
///
/// **A biome is a value rather than a kind** - `P-451`: *a value declares which trait it is one
/// of*. So each line is a `value`, named, `of:biome`, and `value` is the fourth declaring kind
/// beside `kind`, `trait` and `family`.
///
/// **`nature` rides on the value's line**, because a biome's force of nature is a fact about
/// the biome - `P-454`'s fourth sentence, and the release says which column that is: **the
/// numbers here guide and do not bind; a territory's own are in *Territory resources*. Force
/// of nature is the one column that binds.**
///
/// **So the three resource columns are not written here and that is the release's own
/// sentence rather than a choice made in this file.** `5 x 6` is advice to whoever picks a
/// territory's numbers - two numbers in one cell, guiding - and what binds is in *Territory
/// resources*, which is **this planet's** and not the game's. `C-97` is where that
/// distinction was measured.
///
/// **Ocean carries no `nature`**, because the release says it *is not claimable and carries
/// nothing* and its every cell is `-`. A line with a name and its trait is the whole of what
/// is true of it.
pub fn biomes(document: &str) -> String {
    let mut rows: Vec<Description> = Vec::new();
    for row in body_under(document, "## Biomes") {
        let name = plain(row.first().map(String::as_str).unwrap_or_default()).to_lowercase();
        assert!(
            !name.is_empty(),
            "a row of the Biomes table names no biome, so the file would declare a blank word"
        );
        let mut traits = std::collections::BTreeMap::new();
        traits.insert("name".to_string(), name);
        traits.insert("of".to_string(), "biome".to_string());
        let nature = row.get(4).map(String::as_str).unwrap_or_default().trim();
        if nature != "-" && !nature.is_empty() {
            nature.parse::<u32>().unwrap_or_else(|_| {
                panic!("`{nature}` is a force of nature and is not a number, so the line would carry a word the notation has no place for")
            });
            traits.insert("nature".to_string(), nature.to_string());
        }
        rows.push(Description {
            kind: "value",
            traits,
        });
    }
    assert!(
        !rows.is_empty(),
        "the Biomes table parsed to nothing, so this would write an empty file and a \
         comparison against it would agree for the wrong reason"
    );
    crate::state::declared(&rows)
}

/// Every trait the release declares that a data file needs, as the file that would declare them.
///
/// **`P-451`: a trait says what it admits and whether it is stored, and says nothing about
/// which kinds carry it.** So the *Of* column is not here - it is inverted onto the kinds -
/// and each line is the trait's name, what it admits, and how it is kept.
///
/// **The two keys are `admits` and `kept`, and they are the specification lane's proposal
/// rather than a promoted rule** - `P-457`, open to Sean, and `C-100` is where this lane said
/// the shape was not its to choose. `admits` is the release's own verb: *where a trait admits
/// a closed set of values*. `kept` is not `held`, because in this game holding is containment
/// and a store holds metal. **If either name changes, this function changes and nothing else
/// does.**
///
/// # What `admits` says, by what the Values cell is
///
/// **A number, however the cell describes what it counts.** *A number* five times, and four
/// more that are a number with a sentence about what it counts - *how much energy its tank
/// holds*, *food per turn*, *the number of turns it will last*, and `id`'s *unique among
/// things of its kind*. The sentence is a relationship and rule 7 leaves it in prose.
///
/// **Or the name of whatever already declares the values.** A family where they are kinds -
/// `one of the resources` is the `resource` family, `a place` is the `place` family - and the
/// trait's own name where they are values, because `{value name:ice of:biome}` says it there.
///
/// **Or `???`, which is the one open cell.** `0 or 1` five times and `yes or no` three times
/// are the same two-valued set spelled twice, and what the notation calls it is `P-457`'s
/// question. **Written as a word that cannot be mistaken for an answer**, so a reader of the
/// generated file cannot take it for one.
///
/// # Which traits are here
///
/// **Twenty-one of twenty-three.** A derived trait that a recipe names must be declared, since
/// every word in a data file is a kind, a trait or one of a trait's values - and `surplus` and
/// `unpaid` are named once each. **`metal in it` and `control` are named by nothing**, counted
/// over the *Recipes* table, so they are in no data file at all.
pub fn traits(document: &str) -> String {
    let mut rows: Vec<Description> = Vec::new();
    for row in body_under(document, "## Traits") {
        let name = plain(row.first().map(String::as_str).unwrap_or_default()).replace(' ', "-");
        let values = plain(row.get(2).map(String::as_str).unwrap_or_default());
        let kept = plain(row.get(3).map(String::as_str).unwrap_or_default());
        assert!(
            !name.is_empty() && !values.is_empty() && !kept.is_empty(),
            "a row of the Traits table is missing a cell, so the line would be short a fact"
        );
        let kept = match kept.split(':').next().unwrap_or_default().trim() {
            "stored" => "thing",
            "of the kind" => "kind",
            "derived" => "nothing",
            other => panic!(
                "`{name}` is kept `{other}`, and the release has three: stored, of the kind, \
                 derived"
            ),
        };
        // A derived trait no recipe names is in no data file, so it is not declared either.
        if kept == "nothing" && !names_it(document, &name) {
            continue;
        }
        let mut traits = std::collections::BTreeMap::new();
        traits.insert("name".to_string(), name);
        traits.insert("admits".to_string(), admits(&values));
        traits.insert("kept".to_string(), kept.to_string());
        rows.push(Description {
            kind: "trait",
            traits,
        });
    }
    assert!(
        rows.len() > 15,
        "only {} traits, so the table parsed to nearly nothing",
        rows.len()
    );
    crate::state::declared(&rows)
}

/// Whether any recipe row names this trait, which is what makes a derived one declarable.
fn names_it(document: &str, name: &str) -> bool {
    let looking = name.replace('-', " ");
    body_under(document, "## Recipes")
        .iter()
        .any(|row| row.iter().any(|cell| cell.contains(&looking)))
}

/// What a trait admits, read from its Values cell.
///
/// **`0 or 1` is a maximum showing through, not the trait's shape.** `spec/turn.md`: *an action
/// is named, and each kind declares how many of each action a thing of it may take in a turn.*
/// So an action count admits a **number**, bounded by a maximum the kind declares, and `0 or 1`
/// is what that range looks like in a release where every maximum is one.
///
/// **That is read from a promoted rule rather than chosen here**, which is why it is not
/// `???`: a unit that one day takes two moves needs no change to this file. `P-457` carries
/// the same reading to Sean, and if he reads it the other way this function changes and
/// nothing else does.
///
/// **`yes or no` is still open.** `unpaid` and `movable` are not action counts and have no
/// maximum behind them - either numbers like the counts, or values declared the way a biome
/// is. Two cells, and `P-457` is where they are decided.
fn admits(values: &str) -> String {
    let said = values.trim();
    if said == "yes or no" {
        return "???".to_string();
    }
    if said == "design or play" {
        return "phase".to_string();
    }
    if let Some(rest) = said.strip_prefix("one of the ") {
        return rest.trim_end_matches('s').to_string();
    }
    if said == "a place" {
        return "place".to_string();
    }
    // Everything left is a number: `0 or 1` by the rule above, `a number` outright, and four
    // cells that are a number with a sentence about what it counts.
    assert!(
        said == "0 or 1"
            || said.contains("number")
            || said.contains("how much")
            || said.contains("per turn"),
        "`{said}` is a Values cell this does not read, and guessing at it would put a word in \
         the file that the release did not say"
    );
    "number".to_string()
}
