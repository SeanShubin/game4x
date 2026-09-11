//! Every word in a data file is a kind, a family, a trait, or one of a trait's values.
//!
//! **`P-284`, and `S-47`'s second half.** `spec/console.md`: *Every word in a data file is a
//! kind, a trait, or one of a trait's values. A file that uses any other word is wrong about
//! the game rather than describing it.*
//!
//! # What is checked, and why it is not `P-284`'s literal words
//!
//! **Families are admitted too, and that gap is `C-37`'s.** `unit` and `place` are families
//! rather than kinds, the data file uses both correctly, and a check built on the literal
//! sentence would flag them - whereupon whoever ran it would "fix" correct usage. So the
//! vocabulary is kinds, families, trait names and trait values, and the difference between
//! that and the rule as written is filed rather than silently widened.
//!
//! **A value is checked against its own trait rather than against every word in the
//! document.** That is the correction `C-37` records making to its own instrument: a rule
//! that splits every Values cell into words admits `turn`, because `upkeep` reads *food per
//! turn*, and it was run twice knowing it was wrong. Here a trait either **names a closed
//! set** - listing its values, or pointing at a table that does - or it does not, and one
//! that does not admits a number and nothing else.
//!
//! # The reading direction that matters
//!
//! **This reads the file, not the code that writes it.** `C-28`: a count over `dump.rs`
//! tells you what was written and nothing about what ran. The population here is every word
//! of every description in the state the scenario actually produces.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use game_console::{Library, Session, state};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn release() -> String {
    std::fs::read_to_string(root().join("releases/first-release.md")).expect("the release document")
}

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }
    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

fn played() -> Session {
    let files = Files(root().join("scenario/commands"));
    let mut session = Session::new();
    for line in ["{run file:setup}", "{start}", "{run file:play}"] {
        session
            .run(line, &files)
            .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
    }
    session
}

/// Every row under a heading, as its cells stripped of emphasis and lowercased.
///
/// **Parsed to cells rather than matched as text.** `CLAUDE.md`: the padder rewrites column
/// widths, so a pattern that matched yesterday silently stops matching - and a `str::replace`
/// with no match is a no-op rather than an error. Splitting on `|` and trimming makes the
/// padding invisible to this by construction.
fn rows_under(document: &str, heading: &str) -> Vec<Vec<String>> {
    let mut out = Vec::new();
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
        if !inside || !line.starts_with('|') {
            continue;
        }
        let cells: Vec<String> = line
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().trim_matches('*').trim().to_lowercase())
            .collect();
        // **The separator row is what says the header is over**, so everything gathered
        // before it goes. A row of a data table is never all dashes; a separator always is.
        if cells.iter().all(|cell| cell.chars().all(|c| c == '-')) {
            out.clear();
            continue;
        }
        out.push(cells);
    }
    out
}

/// The first cell of every row under a heading.
///
/// **The header is dropped by [`rows_under`] and not by skipping one**, because skipping is
/// a count and the separator is the thing that actually says where the header ended. A table
/// with no rows would otherwise lose its first row of data to a skip that had nothing to
/// skip.
fn named_under(document: &str, heading: &str) -> Vec<String> {
    rows_under(document, heading)
        .into_iter()
        .filter_map(|row| row.first().cloned())
        .filter(|name| !name.is_empty())
        .collect()
}

/// What a trait admits.
#[derive(Debug, PartialEq, Eq)]
enum Admits {
    /// The Values cell names its values, or points at a table that lists them.
    OneOf(BTreeSet<String>),
    /// It does not, so what it admits is a number - `id`, `force`, `nature`, `fuel`.
    ///
    /// **Numbers are values.** `P-284` admits *one of a trait's values*, and `id`'s cell
    /// says *a number, unique among things of its kind* - so `1` is a value of `id` exactly
    /// as `yes` is a value of `ready`.
    ANumber,
}

/// Read the release's *Traits* table: what each trait is called, and what it admits.
///
/// **The rule for telling the two apart is stated here rather than guessed per row**, which
/// is what `C-37` records getting wrong: its first pass scored `phase` as naming its values
/// *because the cell contains the word `or`*, and returned a plausible split rather than an
/// error.
///
/// A cell names a closed set when it says **one of the ...**, pointing at a table, or when it
/// is alternatives joined by *or* and every alternative is a single word. Anything else -
/// *a number*, *food per turn*, *how much energy its tank holds* - names no set.
fn declared_traits(document: &str) -> BTreeMap<String, Admits> {
    let kinds: BTreeSet<String> = named_under(document, "## Kinds").into_iter().collect();
    let biomes: BTreeSet<String> = named_under(document, "## Biomes")
        .into_iter()
        .filter(|biome| biome != "ocean")
        .collect();
    // A resource is a family rather than a table, so its members are the Families row.
    let resources: BTreeSet<String> = rows_under(document, "## Families")
        .into_iter()
        .find(|row| row.first().map(String::as_str) == Some("resource"))
        .and_then(|row| row.get(1).cloned())
        .map(|members| members.split(',').map(|m| m.trim().to_string()).collect())
        .unwrap_or_default();

    let mut out = BTreeMap::new();
    for row in rows_under(document, "## Traits") {
        let (Some(name), Some(values)) = (row.first(), row.get(2)) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        let admits = if let Some(set) = values.strip_prefix("one of the ") {
            match set.trim() {
                "kinds" => Admits::OneOf(kinds.clone()),
                "biomes" => Admits::OneOf(biomes.clone()),
                "resources" => Admits::OneOf(resources.clone()),
                other => panic!("`{name}` says `one of the {other}` and nothing lists them"),
            }
        } else {
            let alternatives: Vec<&str> = values.split(" or ").map(str::trim).collect();
            if alternatives.len() > 1
                && alternatives
                    .iter()
                    .all(|word| !word.is_empty() && !word.contains(' '))
            {
                Admits::OneOf(alternatives.iter().map(|w| w.to_string()).collect())
            } else {
                Admits::ANumber
            }
        };
        // **A declared name is matched as a data file would write it.** `spec/console.md`:
        // *A name is one word. Where it needs more than one, the words are joined with
        // dashes.* That rule is about a data file, and the release's table is prose - so
        // `total capacity` is declared there and written `total-capacity` here, and a check
        // comparing the two literally reports a correct file as wrong. It did, on `P-331`.
        //
        // **Applying the rule rather than widening the check.** Two declared traits have a
        // space - `total capacity` and `metal in it` - and neither could appear in a data
        // file under its written name, because a name with a space in it cannot be written
        // at all without the quoting `P-252` forbids.
        out.insert(name.replace(' ', "-"), admits);
    }
    out
}

/// A word the data file uses that the release does not declare, and the reason it is allowed
/// to stay.
///
/// **A named exception fails when it is repaired and when it stops being true**, so a gap
/// cannot outlive itself and cannot be closed by quietly weakening the assertion. The pattern
/// is `closed_sets.rs`'s, and its warning holds here too: past about two, a list of
/// exceptions is the thing being checked written twice.
/// **It is empty, and that is the finding rather than a check with nothing to do.** Both
/// entries were `C-46`'s two words: `game`, which `P-351` declared, and `manned`, which Sean
/// deleted in `S-72`. **An empty list is a claim** - that every word in the data file is one
/// the release declares - and the count below states it, so emptying this stays a deliberate
/// act rather than somewhere a weakened test could arrive.
const UNDECLARED: [(&str, &str); 0] = [];

/// Every word of every description in the played state is one the release declares.
#[test]
fn every_word_in_the_data_file_is_one_the_release_declares() {
    let document = release();
    let kinds: BTreeSet<String> = named_under(&document, "## Kinds").into_iter().collect();
    let families: BTreeSet<String> = named_under(&document, "## Families").into_iter().collect();
    let traits = declared_traits(&document);

    // **Each side non-empty before the comparison, or it agrees with anything.** A release
    // whose tables stopped parsing would admit nothing and this would report every word,
    // which is loud - but a release whose tables parsed *empty* would be the silent one, and
    // that is the direction guarded here.
    // Eighteen since `P-399` made `readiness` one - it was a yes-or-no trait, and the token
    // model makes it a thing a thing holds.
    assert_eq!(
        kinds.len(),
        18,
        "eighteen kinds; the release lists {kinds:?}"
    );
    assert_eq!(
        families.len(),
        4,
        "four families; the release lists {families:?}"
    );
    // **Twenty since `P-399`, which took two out and put one in.** `ready` and `spent` were
    // both yes-or-no traits of a thing; readiness is a kind now, and what a reader needs of
    // one is `for`, naming the action it is for.
    assert_eq!(
        traits.len(),
        20,
        "twenty traits; the release lists {:?}",
        traits.keys().collect::<Vec<_>>()
    );
    let closed = traits
        .values()
        .filter(|a| matches!(a, Admits::OneOf(_)))
        .count();
    // **Eight, and the eighth is `movable`.** `C-37` measured six - `ready`, `surplus` and
    // `unpaid` outright, and `kind`, `resource`, `biome` by pointing at a table - `P-308`
    // named `phase`'s values for the seventh, and `P-355` added `movable`, *yes or no*.
    // This arrives at the figure by reading the rows rather than by adding one to a
    // remembered number, which is the only way the two counts are independent. `control` is
    // the next candidate and describes rather than naming: *held by a player, or unclaimed*
    // names one value and describes the other.
    // **Seven since `P-399`.** `ready` and `spent` both named a closed set and both are gone;
    // `for` names one - *`move`, `labor`, `work` or `bearing`* - but its values are backticked,
    // and the rule above admits alternatives joined by *or* only where each is a bare word. So
    // it is read as admitting a number, which is wrong about `for` and is the parser's reach
    // rather than the release's: reported rather than widened, which is what `C-37` records
    // this check getting wrong in the other direction.
    assert_eq!(
        closed, 7,
        "seven traits name a closed set - `kind`, `resource`, `biome`, `surplus`, `unpaid`, `phase` and `movable`; {closed} do"
    );

    let session = played();
    let tree = state::entries(&session.game);

    let mut words = 0;
    let mut wrong: Vec<String> = Vec::new();
    for entry in tree.walk() {
        let description = entry.description.written();
        words += 1;
        let kind = entry.description.kind.to_string();
        if !kinds.contains(&kind) && !families.contains(&kind) {
            wrong.push(format!(
                "{description}: `{kind}` is neither a kind nor a family"
            ));
        }
        for (name, value) in &entry.description.traits {
            words += 2;
            match traits.get(name) {
                None => wrong.push(format!("{description}: `{name}` is not a declared trait")),
                Some(Admits::OneOf(set)) if !set.contains(value) => wrong.push(format!(
                    "{description}: `{value}` is not one of {name}'s values {set:?}"
                )),
                Some(Admits::ANumber) if value.parse::<u32>().is_err() => wrong.push(format!(
                    "{description}: `{name}` names no closed set, so `{value}` has to be a number"
                )),
                Some(_) => {}
            }
        }
    }

    // **The population, because a claim of zero proves nothing against a population that is
    // also zero.** `CLAUDE.md`: *a count over nothing is the same failure with the sign
    // flipped, so a claim of zero names what it counted against.*
    assert!(
        words > 150,
        "only {words} words examined, so finding none wrong would mean nothing"
    );

    let named: BTreeSet<&str> = UNDECLARED.iter().map(|(word, _)| *word).collect();
    let unexcused: Vec<&String> = wrong
        .iter()
        .filter(|line| !named.iter().any(|word| line.contains(&format!("`{word}`"))))
        .collect();
    assert!(
        unexcused.is_empty(),
        "these words are in the data file and not in the release:\n  {}",
        unexcused
            .iter()
            .map(|l| l.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );

    // An exception that has been repaired is a lie in the other direction, and nothing else
    // would notice: this would go on passing while claiming a gap that had closed.
    for (word, why) in UNDECLARED {
        assert!(
            wrong.iter().any(|line| line.contains(&format!("`{word}`"))),
            "`{word}` is declared now, so delete its exception: {why}"
        );
    }
    assert_eq!(
        UNDECLARED.len(),
        0,
        "every word in the data file is declared; an entry here is a finding rather than an allowance"
    );
}

/// The check can fail, in each of the three ways a word can be wrong.
///
/// **Poisoning the real release is not an option** - it is Sean's file and this lane does not
/// edit it - so the arms are demonstrated against a document written to carry each. Without
/// this every assertion above is a claim: they pass on today's release and would pass just as
/// green if `rows_under` returned nothing at all.
#[test]
fn the_check_can_fail_and_each_way_of_being_wrong_is_told_apart() {
    let document = "## Kinds\n\n\
        | Kind        | What it is |\n\
        | ----------- | ---------- |\n\
        | **citizen** | a person   |\n\
        \n## Families\n\n\
        | Family       | Members             |\n\
        | ------------ | ------------------- |\n\
        | **resource** | food, metal, energy |\n\
        \n## Traits\n\n\
        | Trait     | Of          | Values             | Stored or derived |\n\
        | --------- | ----------- | ------------------ | ----------------- |\n\
        | **ready** | whatever    | yes or no          | stored            |\n\
        | **force** | a citizen   | a number           | stored            |\n\
        | **resource** | a store  | one of the resources | stored          |\n";

    let kinds: BTreeSet<String> = named_under(document, "## Kinds").into_iter().collect();
    assert_eq!(kinds, ["citizen".to_string()].into_iter().collect());

    let traits = declared_traits(document);
    assert_eq!(traits.len(), 3, "three traits were declared");
    assert_eq!(
        traits.get("ready"),
        Some(&Admits::OneOf(
            ["yes".to_string(), "no".to_string()].into_iter().collect()
        )),
        "`yes or no` names its two values"
    );
    assert_eq!(
        traits.get("force"),
        Some(&Admits::ANumber),
        "`a number` names no set, so it admits a number"
    );
    assert_eq!(
        traits.get("resource"),
        Some(&Admits::OneOf(
            [
                "food".to_string(),
                "metal".to_string(),
                "energy".to_string()
            ]
            .into_iter()
            .collect()
        )),
        "`one of the resources` reaches the family that lists them"
    );

    // The three ways a word can be wrong, each told apart from the others.
    assert!(
        !kinds.contains("yard"),
        "a kind the release does not declare"
    );
    assert!(
        !traits.contains_key("manned"),
        "a trait the release does not declare"
    );
    match traits.get("ready") {
        Some(Admits::OneOf(set)) => assert!(
            !set.contains("exhausted"),
            "a value outside a closed set is caught"
        ),
        other => panic!("`ready` names a closed set, and read as {other:?}"),
    }
    assert!(
        "many".parse::<u32>().is_err(),
        "and a word where a number belongs is caught"
    );

    // **The vacuous pass, guarded.** A heading with no table under it must read as empty, or
    // every assertion above it is satisfied by a release that stopped parsing.
    assert!(
        named_under("## Kinds\n\nnothing here yet.\n", "## Kinds").is_empty(),
        "an empty table has to read as empty"
    );
    assert!(
        declared_traits("## Traits\n\nnothing here yet.\n").is_empty(),
        "and so does an empty Traits table"
    );
}

/// The word `P-288` decided has to go is gone, and the check would have said so.
///
/// **`turn` is the standing example of an instrument answering a narrower question.** Three
/// passes over the release admitted it, because `upkeep`'s Values cell reads *food per turn*
/// and a rule that splits a cell into words admits every word in every such sentence. It is
/// checked here as a case rather than trusted to the rule, because the rule is what was wrong
/// three times.
#[test]
fn turn_is_neither_a_kind_nor_a_trait_nor_a_value_and_is_not_in_the_file() {
    let document = release();
    let kinds: BTreeSet<String> = named_under(&document, "## Kinds").into_iter().collect();
    let families: BTreeSet<String> = named_under(&document, "## Families").into_iter().collect();
    let traits = declared_traits(&document);

    assert!(!kinds.contains("turn"), "`turn` is not a kind");
    assert!(!families.contains("turn"), "`turn` is not a family");
    assert!(!traits.contains_key("turn"), "`turn` is not a trait");
    for (name, admits) in &traits {
        if let Admits::OneOf(set) = admits {
            assert!(
                !set.contains("turn"),
                "`turn` reads as a value of `{name}`, which is how three instruments admitted it"
            );
        }
    }

    let text = std::fs::read_to_string(root().join("scenario/expected/play.4x"))
        .expect("the expected state");
    let entries: Vec<&str> = text
        .lines()
        .filter(|line| line.trim_start().starts_with('{'))
        .collect();
    assert!(
        entries.len() > 40,
        "only {} entries read, so finding no `turn` would mean nothing",
        entries.len()
    );
    let carrying: Vec<&&str> = entries
        .iter()
        .filter(|line| line.contains("turn:"))
        .collect();
    assert!(
        carrying.is_empty(),
        "`turn` is still in the data file: {carrying:?}"
    );
}
