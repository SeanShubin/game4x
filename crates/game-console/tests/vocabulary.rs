//! Every word in a data file is a kind, a family, a trait, or one of a trait's values.
//!
//! **`P-284`, and `S-47`'s second half.** `spec/console.md`: *Every word in a data file is a
//! kind, a trait, or one of a trait's values, except the words the notation reserves for
//! itself. A file that uses any other word is wrong about the game rather than describing it.*
//!
//! **The exception is `P-482`'s and it is a clarification rather than a change.** The sentence
//! sat sixty-three lines above the one naming `name`, `admits`, `kept`, `of` and `family` as
//! the notation's own, so read alone it condemned every declaration in `spec/data/`. This test
//! has always admitted them; the rule says so now too.
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

    // **Read by name since `P-473`.** *Values* was cell 2 and the *Of* column going made it
    // cell 1, so this read *Stored or derived* and found no cell naming a closed set - three
    // became zero. **The guard in `nogain.rs` states the release's column order and passed**,
    // because it compares the release with a list written there and cannot see a reader
    // elsewhere holding a different one. Only taking the index out finds those.
    let rows = rows_under(document, "## Traits");
    if rows.is_empty() {
        // **A heading with no table under it reads as no traits**, which is what
        // `an empty table has to read as empty` drives - and asking `column_of` for a column
        // of a table that is not there panics, correctly, so the question is not asked.
        return BTreeMap::new();
    }
    let at = game_console::recipes::column_of(document, "## Traits", "Values");
    let mut out = BTreeMap::new();
    for row in rows {
        let (Some(name), Some(values)) = (row.first(), row.get(at)) else {
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
            // **Backticks are stripped before the words are judged** - `P-399` writes the
            // `for` trait's values as `` `move`, `labor`, `work` or `bearing` ``, and a rule
            // that admits an `or` list only of bare words read that as naming no set at all.
            // The release does declare one; the parser could not see through the markup, and
            // the data file's `for:labor` was then a word nothing admitted.
            //
            // **Stripping markup is not widening the rule.** `C-37` records this check being
            // widened wrongly once - splitting every Values cell into words, which admitted
            // `turn` because `upkeep` reads *food per turn*. A backtick is not a word.
            // **Split on the separators an alternative list uses, not on every space.** A
            // first attempt split on spaces too, and `control` - *held by a player, or
            // unclaimed* - became five single words and read as naming a closed set. That is
            // `C-37`'s failure exactly, reintroduced while fixing something else, and the
            // guard that catches it is the one below: an alternative that is a phrase means
            // the cell describes rather than lists.
            let alternatives: Vec<String> = values
                .replace(" or ", ",")
                .split(',')
                .map(|word| word.trim().trim_matches('`').trim().to_string())
                .filter(|word| !word.is_empty())
                .collect();
            let listed =
                alternatives.len() > 1 && alternatives.iter().all(|word| !word.contains(' '));
            if listed {
                Admits::OneOf(alternatives.into_iter().collect())
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
/// **It was empty, and `P-522` put five words in it.** Both original entries were `C-46`'s:
/// `game`, which `P-351` declared, and `manned`, which Sean deleted in `S-72`. **An empty list
/// is a claim** - that every word in the data file is one the release declares - and it held
/// until a release deferred part of the model rather than the model getting ahead of one.
///
/// # These five are a schedule, not a gap
///
/// Sean, 2026-09-21, choosing between cutting `garrison`, `biome` and `force` out of the model
/// and narrowing the checks: **the model keeps what `spec/` keeps**, `spec/planet.md` keeps
/// every biome and `spec/control.md` keeps force, and a release saying *not this one* is a
/// schedule. So the played state still stands up garrisons and gives territories biomes, and
/// the release declares neither.
///
/// **That is different in kind from the two this list used to hold.** Those were words the
/// data used and nothing declared anywhere - a real gap, closed by declaring one and deleting
/// the other. These are declared in `spec/` and deferred by a release, which is why the reason
/// column says so rather than saying *not yet declared*.
///
/// **Five is over the two the warning above names**, and deliberately: an exemption list long
/// enough to be the thing checked written twice is the hazard, and these five are one
/// promotion rather than five findings. **They come out together or not at all**, which is
/// what makes them one entry with five spellings rather than five exceptions.
// **`defending` was the fifth and is gone, which is this list doing its job.** `P-522` deferred
// force and the word stayed in the dump for two days; the dump stopped writing it on 2026-09-24,
// so the exception had nothing left to excuse and the assertion below said so. **An exception
// that outlives its gap is a lie in the other direction**, and that is the half of this list
// that caught something.
const UNDECLARED: [(&str, &str); 4] = [
    (
        "garrison",
        "`P-522` deferred it; `spec/control.md` keeps it",
    ),
    ("nature", "`P-522` deferred it with the force rule"),
    (
        "biome",
        "`P-522` cut the Biomes table; `spec/planet.md` keeps every biome",
    ),
    ("met", "the mark `hold` put on a nature, deferred with it"),
];

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
    // **Eighteen since `P-435` declared `force`**, which answers `C-93` - this lane asked
    // whether one word naming a trait and a thing at once was deliberate, and the answer was
    // that it was: the trait is `strength` now and the kind is `force`. Seventeen between
    // `P-411` taking `readiness` out and that.
    // **Sixteen since `P-522`**, which cut `garrison`, `nature` and `force` with the
    // sections that used them.
    assert_eq!(
        kinds.len(),
        16,
        "sixteen kinds; the release lists {kinds:?}"
    );
    assert_eq!(
        families.len(),
        4,
        "four families; the release lists {families:?}"
    );
    // **Twenty-six since `P-476`, and `room` becoming three is why.** `P-474` made the
    // deposit's bound the room left; `P-477` and `P-476` name all three - `capacity`,
    // `occupied` and `free` - because `spec/logistics.md` says **three names describe it and
    // there are two facts**, and any two give the third.
    //
    // **`P-399` made readiness a kind and took `ready`
    // and `spent` out; `P-411` undid that and put five counts in - `moving`, `laboring`,
    // `working`, `bearing` and `defending`, one per action, each `0 or 1`, where the two it
    // replaced were one flag between them. `P-417` took `kind` out in the same pass, because a
    // kind is not a trait. `P-461` added `binding` for the twenty-fourth: the *Binding* column
    // of *Units and structures* had been a number `metal in it` referred to and no trait
    // declared, which is the dangling reference this lane reported.
    // **Twenty-three since `P-522`**: `defending` went with the force rule, and `biome` and
    // `met` with the Biomes section and the nature it marked.
    assert_eq!(
        traits.len(),
        23,
        "twenty-three traits; the release lists {:?}",
        traits.keys().collect::<Vec<_>>()
    );
    let closed = traits
        .values()
        .filter(|a| matches!(a, Admits::OneOf(_)))
        .count();
    // **Three since `P-465`, and the eight that left are the whole of the difference.**
    // `resource`, `biome` and `phase` name their values or point at a table that does. The
    // five action counts, `surplus`, `unpaid` and `movable` said *0 or 1* or *yes or no*, and
    // `P-465` made all eight *a number* - which is `P-457`'s rule reaching the cells:
    // **nothing in the game is two-valued anywhere.**
    //
    // **A tripwire watched for exactly this and has been deleted.** `C-103` filed it because
    // `P-465` said no check caught the eight; it fired on the first run after `P-465` landed,
    // with the set empty, and its own doc said the day it fires is the day to delete it rather
    // than update it. **This count is what carries the fact now**, which is where it belongs:
    // a check about what a trait admits rather than a list of what was wrong once.
    //
    // **`control` is still the next candidate and still describes rather than naming**:
    // *held by a player, or unclaimed* names one value and describes the other, and
    // `spec/data/traits.4x` resolves it to `admits:number` - which is `C-106`.
    //
    // **What moved before that:** `P-399` took `ready` and `spent` out and added `for`;
    // `P-411` took `for` out again and added the five counts; `P-417` deleted `kind`, because
    // a kind is not a trait.
    // **Two since `P-522` cut `biome`**, which was the one naming a set written out in a
    // table of its own. `resource` and `phase` name theirs in the Families table and in the
    // turn's phases.
    assert_eq!(
        closed, 2,
        "two traits name a closed set - `resource` and `phase`; {closed} do"
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
    // **Four, and the number is asserted so that the list cannot grow quietly.** It was zero
    // and the claim it carried was that every word in the data file is declared; `P-522`
    // deferred five, and `defending` left when the dump stopped writing it. The claim is now
    // that these four and no others are deferred. **A fifth is a finding** - either the model has got ahead of the specification, which is
    // what this check has always been for, or another promotion deferred something and the
    // reason belongs beside it.
    //
    // **The loop above is the other direction** and is what stops this being a weakening: a
    // word that stops being wrong fails there, so the day the release declares `garrison`
    // again the entry has to come out.
    assert_eq!(
        UNDECLARED.len(),
        4,
        "four words are deferred; a fifth is a finding rather than an allowance"
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

/// Every trait the render order names is a trait the game actually declares.
///
/// **`P-479` states the order and `S-124` asks for this check**, and the reason it gives is the
/// mirror of the one that would guard a full list. A list of all twenty-six rots when a trait is
/// **added** and nobody places it. **This list rots when a trait is renamed**: the ordering goes
/// on naming something nothing has, that name simply never matches, the rule quietly stops
/// applying to anything, and every dump still looks fine.
///
/// **Not hypothetical.** `free` was called `room` the day before `P-479` landed. An ordering
/// written then would still say `room` today, `room` would rank in the middle with everything
/// else, `free` would sit wherever the alphabet put it, and nothing would be red.
///
/// **The names are read from `containment` rather than retyped here**, because a check holding
/// its own copy of the list is checking the copy - and the population is read from
/// `spec/data/traits.4x`, which is generated from the release, so this compares the ordering
/// against what the game declares rather than against a second opinion about it.
#[test]
fn every_trait_the_ordering_names_is_declared() {
    let text = std::fs::read_to_string(root().join("spec/data/traits.4x"))
        .expect("spec/data/traits.4x is generated");
    let declared: BTreeSet<String> = state::declarations(&text)
        .expect("the declarations parse")
        .iter()
        .filter_map(|it| it.traits.get("name").cloned())
        .collect();

    // **The population, named.** A check that every name is in an empty set passes for the
    // wrong reason, which is the failure with the sign flipped.
    // **Twenty-three since `P-522`**, which cut `defending`, `biome` and `met`.
    assert_eq!(
        declared.len(),
        24,
        "spec/data/traits.4x declares {} traits and this check was written against 24. If the \
         release gained or lost one, that is fine - say so here.",
        declared.len()
    );

    let named: Vec<&str> = game_console::containment::ORDERED_FIRST
        .iter()
        .chain(game_console::containment::ORDERED_LAST.iter())
        .copied()
        .collect();
    // **The rename guard first, because it is what this check is for.** It was written after
    // the restatement below and failed behind it: poisoning `free` to `room` tripped the
    // restatement, which is a second copy of the list, and the assertion that reads
    // `spec/data/traits.4x` was never reached. **A guard that shadows the check it guards is
    // the check having its own copy after all** - `C-110`, inside the test written to avoid it.
    for name in &named {
        assert!(
            declared.contains(*name),
            "the render order names `{name}` and `spec/data/traits.4x` declares no such trait, \
             so that end of the order applies to nothing and no dump would look wrong"
        );
    }

    // **And the order agrees with the sentence**, which the check above cannot say: every name
    // could be declared and still be in the wrong sequence. This one is a second copy of the
    // list deliberately, and it is a copy of `P-479`'s words rather than of the code's.
    assert_eq!(
        named,
        vec!["id", "occupied", "free", "capacity"],
        "the order `P-479` states is `id` first and `occupied`, `free`, `capacity` last"
    );

    // **And each is named once**, because a name at both ends would rank by whichever branch
    // ran first and the order would depend on the code rather than on the rule.
    let mut seen = BTreeSet::new();
    for name in &named {
        assert!(
            seen.insert(*name),
            "`{name}` is named at both ends of the order"
        );
    }
}

/// The notation's own words are not traits, which is what lets one order cover both.
///
/// **`P-481`:** *`name`, `admits`, `kept`, `of` and `family` are the notation's own words...
/// These are the two things in a data file that are not a kind, a trait, or one of a trait's
/// values... neither says anything about a game, and neither is declared.*
///
/// **`P-483` puts them ahead of every trait in one order over both**, and that only works while
/// they cannot also *be* traits. If `name` were ever declared, a state description could carry
/// it and it would rank first there - ahead of `id`, in a line about a thing, silently. So this
/// is the premise the single order rests on, asserted rather than left true by luck.
#[test]
fn no_notation_word_is_a_declared_trait() {
    let text = std::fs::read_to_string(root().join("spec/data/traits.4x"))
        .expect("spec/data/traits.4x is generated");
    let declared: BTreeSet<String> = state::declarations(&text)
        .expect("the declarations parse")
        .iter()
        .filter_map(|it| it.traits.get("name").cloned())
        .collect();
    assert!(
        !declared.is_empty(),
        "a count against nothing proves nothing"
    );

    let mut checked = 0;
    for word in game_console::containment::NOTATION_WORDS {
        assert!(
            !declared.contains(word),
            "`{word}` is one of the notation's own words and `spec/data/traits.4x` declares it \
             as a trait, so one word would rank in two places"
        );
        checked += 1;
    }
    assert_eq!(checked, 5, "`name`, `of`, `family`, `admits` and `kept`");

    // **And the order is the one `P-483` states**, which the check above cannot say: all five
    // could be absent from the traits and still be sequenced wrongly.
    assert_eq!(
        game_console::containment::NOTATION_WORDS.to_vec(),
        vec!["name", "of", "family", "admits", "kept"],
        "a declaration leads with `name`, then `of` and `family`, then the notation's others"
    );
}
