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
    let carries = carried(document);
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
        let mut row = named("kind", &name, families.get(&name).map(String::as_str));
        // **`P-470`: a kind declares which traits it has**, and they are written with no value
        // - `spec/console.md`: *a trait of the kind is written with its value and a stored one
        // with its name*. The names sort in with `family` and `name` rather than after them,
        // because a description is a map and the sort is over the trait.
        for carried in carries.get(&name).into_iter().flatten() {
            row.traits.insert(carried.clone(), String::new());
        }
        rows.push(row);
    }
    crate::state::declared(&rows)
}

/// The six kinds *whatever is built* names, which no longer derive from the release.
///
/// **`P-466` removed the column this was read from.** *Units and structures* had a **Binding**
/// cell for exactly these six and has seven columns now; `binding`'s own *Values* cell says
/// what it is instead - *derived: the metal the recipe that makes it consumes* - and by that
/// definition the set is the kinds a metal-consuming recipe produces, which is
/// `ark, energy, extractor, metal, pioneer, store, yard`. **That is not these six**: it gains
/// two resources and loses the garrison.
///
/// **The garrison is `P-467` surviving its own withdrawal**, and it is filed as `C-106`. No
/// recipe consuming metal produces one - `found by land` consumes a pioneer and `deploy ark`
/// consumes an ark - so the garrison's `binding` derives from nothing while its line in
/// `spec/data/kinds.4x` names it.
///
/// **Written down because it cannot be derived, and asserted so it cannot outlive that.**
/// [`built_is_still_underivable`] fails the day the derivation reproduces this list, which is
/// the day to delete both. `C-61`'s pattern.
pub const BUILT: [&str; 6] = ["garrison", "extractor", "yard", "store", "ark", "pioneer"];

/// Which traits each kind carries, from the release's *Of* column read backwards.
///
/// **`P-470` inverted it and `spec/console.md` is why**: *a kind declares which traits it has*,
/// and a trait *says nothing about which kinds carry it*. So the column is a fact about kinds
/// written on the traits' rows, and this is where it goes back.
///
/// **Inverting is mechanical and rendering back is not** - `C-105`. A cell resolves to a set of
/// kinds; the set does not resolve to the cell, because `laboring` is *a citizen* and `upkeep`
/// is *a thing with upkeep* and both sets are `{citizen}`. This is the direction that works,
/// and it is the only one built here.
///
/// **Every cell must resolve or this panics.** A cell nobody wrote a rule for would otherwise
/// contribute nothing and leave a kind's line short a trait - a plausible file, which is the
/// failure this repository has recorded three times.
fn carried(document: &str) -> std::collections::BTreeMap<String, Vec<String>> {
    let families = family_members(document);
    let kinds: Vec<String> = body_under(document, "## Kinds")
        .iter()
        .map(|row| plain(row.first().map(String::as_str).unwrap_or_default()))
        .filter(|name| !name.is_empty())
        .collect();
    assert!(
        !kinds.is_empty(),
        "no kinds, so every cell would resolve to nothing"
    );

    let mut out: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    let mut resolved = 0;
    for row in body_under(document, "## Traits") {
        let trait_name =
            plain(row.first().map(String::as_str).unwrap_or_default()).replace(' ', "-");
        let of = plain(row.get(1).map(String::as_str).unwrap_or_default());
        // A trait of every kind is on no kind's line - `spec/console.md`, and `traits` writes
        // the `of:thing` that says so.
        if of == "thing" {
            resolved += 1;
            continue;
        }
        for kind in of_cell(document, &of, &families, &kinds) {
            assert!(
                kinds.contains(&kind),
                "`{of}` resolves to `{kind}`, which the Kinds table does not declare"
            );
            out.entry(kind).or_default().push(trait_name.clone());
        }
        resolved += 1;
    }
    assert_eq!(
        resolved,
        body_under(document, "## Traits").len(),
        "a Traits row was skipped, so this inverted less than the column"
    );
    out
}

/// One *Of* cell, as the kinds it names.
///
/// **Six forms, and the release uses all six**: a family named bare (`thing`, handled by the
/// caller), a predicate over another column, a bare comma list, alternatives joined by *or*, an
/// article and a family, an article and a kind. **A form nobody wrote a rule for panics** rather
/// than resolving to nothing.
fn of_cell(
    document: &str,
    of: &str,
    families: &std::collections::BTreeMap<String, Vec<String>>,
    kinds: &[String],
) -> Vec<String> {
    // The predicates, each over a column of *Units and structures* - except the first, whose
    // column `P-466` removed. See [`BUILT`].
    if of == "whatever is built" {
        return BUILT.iter().map(|name| name.to_string()).collect();
    }
    if let Some(column) = match of {
        "a thing with upkeep" => Some("Upkeep"),
        "whatever moves" => Some("Movable"),
        _ => None,
    } {
        return filled_in(document, column);
    }
    let mut out = Vec::new();
    for part in of.split(" or ").flat_map(|part| part.split(',')) {
        let word = part.trim();
        let word = word
            .strip_prefix("an ")
            .or_else(|| word.strip_prefix("a "))
            .or_else(|| word.strip_prefix("the "))
            .unwrap_or(word)
            .trim();
        if word.is_empty() {
            continue;
        }
        if let Some(members) = families.get(word) {
            out.extend(members.iter().cloned());
        } else if kinds.iter().any(|kind| kind == word) {
            out.push(word.to_string());
        } else {
            panic!(
                "`{of}` names `{word}`, which is neither a kind nor a family - and guessing \
                 would leave a kind's line short a trait it carries"
            );
        }
    }
    assert!(!out.is_empty(), "`{of}` resolved to no kind at all");
    out
}

/// The things whose cell in one column of *Units and structures* is not blank.
///
/// **The column is found by its name rather than by counting**, because `P-466` has just
/// removed three of them and a position is what silently becomes a different column.
fn filled_in(document: &str, column: &str) -> Vec<String> {
    let heading = "## Units and structures";
    let at = header_of(document, heading)
        .iter()
        .position(|cell| plain(cell) == column)
        .unwrap_or_else(|| panic!("*Units and structures* has no `{column}` column"));
    body_under(document, heading)
        .iter()
        .filter(|row| {
            !row.get(at)
                .map(|cell| cell.trim().is_empty())
                .unwrap_or(true)
        })
        .map(|row| plain(row.first().map(String::as_str).unwrap_or_default()))
        .filter(|name| !name.is_empty())
        .collect()
}

/// Each family's members, from the *Families* table as it is written.
///
/// **`thing` is skipped**, as it is in [`families_of`] and for the same reason: *every kind
/// above* is a rule about the table rather than a list.
fn family_members(document: &str) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut out = std::collections::BTreeMap::new();
    for row in body_under(document, "## Families") {
        let family = plain(row.first().map(String::as_str).unwrap_or_default());
        let members = row.get(1).cloned().unwrap_or_default();
        if family.is_empty() || members.contains("every kind above") {
            continue;
        }
        out.insert(
            family,
            members
                .split(',')
                .map(plain)
                .filter(|member| !member.is_empty())
                .collect(),
        );
    }
    out
}

/// The header row of the table under a heading, which [`body_under`] drops.
fn header_of(document: &str, heading: &str) -> Vec<String> {
    let mut inside = false;
    for line in document.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == heading;
            continue;
        }
        let line = line.trim();
        if inside && line.starts_with('|') {
            return line
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect();
        }
    }
    panic!("`{heading}` has no table under it")
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
        let of = plain(row.get(1).map(String::as_str).unwrap_or_default());
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
        let mut traits = std::collections::BTreeMap::new();
        traits.insert("name".to_string(), name);
        traits.insert("admits".to_string(), admits(&values));
        traits.insert("kept".to_string(), kept.to_string());
        // **A trait of every kind says so, and it is the only trait that says anything about
        // which kinds carry it** - `spec/console.md`, `P-471`: *a trait of every kind is the
        // one exception, and says so with `of:thing`, because there is no kind for it to
        // belong to and no family that could hold it.* Every other *Of* cell is inverted onto
        // the kinds' lines by [`kinds`] and says nothing here.
        if of == "thing" {
            traits.insert("of".to_string(), "thing".to_string());
        }
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

/// What a trait admits, read from its Values cell.
///
/// **Four things a trait may admit**, and the release's own cell says which.
///
/// **A number**, however the cell describes what it counts. `a number` outright; `0 or 1` and
/// `yes or no`, which are a maximum showing through rather than the trait's shape - `P-457`,
/// and `spec/turn.md` is why: *each kind declares how many of each action a thing of it may
/// take in a turn*; and four cells that are a number with a sentence about what it counts.
///
/// **An identity**, which is `id` alone. `P-462`: two are equal or they are not, nothing
/// orders or sums or aggregates one, and a guard compares with `=`. Sean's reason is the third
/// test in *When a primitive earns its place* and it outranks the two that were there -
/// *unifying it would create a lie*. `max id of {territory}` was well-formed and meaningless.
///
/// **A family**, where the values are kinds and the family already declares them -
/// `one of the resources` is `resource`, `a place` is `place`.
///
/// **`value`**, where they are declared values rather than kinds. Not the trait's own name:
/// `{value name:ice of:biome}` already says which set it is, so repeating it on the trait's
/// line would be a rule stated twice - `P-458`, and the specification lane applied it to its
/// own draft.
fn admits(values: &str) -> String {
    let said = values.trim();
    if said == "an identity" {
        return "identity".to_string();
    }
    if said == "one of the biomes" || said == "design or play" {
        return "value".to_string();
    }
    if let Some(rest) = said.strip_prefix("one of the ") {
        return rest.trim_end_matches('s').to_string();
    }
    if said == "a place" {
        return "place".to_string();
    }
    // **One cell is read rather than derived, and it is named here so it cannot be forgotten.**
    // `control` says *held by a player, or unclaimed*, which describes a closed set of two -
    // and `spec/data/traits.4x` says `admits:number`. `P-457` landed *nothing in the game is
    // two-valued anywhere* and `P-465` corrected eight cells and two; this one states a range
    // in a sentence rather than in the `0 or 1` and `yes or no` those two had, so it was not
    // in the list and is still stating one.
    //
    // **This matches the file rather than deriving from the cell**, which is a reading, and
    // `C-106` is where it is filed. **Keyed on the exact words, so the day the cell changes
    // this stops matching and the assertion below says why** - a named exception that cannot
    // outlive its excuse, which is `C-61`'s shape.
    if said == "held by a player, or unclaimed" {
        return "number".to_string();
    }
    assert!(
        said == "0 or 1"
            || said == "yes or no"
            || said.contains("number")
            || said.contains("how much")
            || said.contains("per turn"),
        "`{said}` is a Values cell this does not read, and guessing at it would put a word in \
         the file that the release did not say"
    );
    "number".to_string()
}
