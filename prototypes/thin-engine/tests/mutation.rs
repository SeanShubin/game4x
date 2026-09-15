//! Every row and every value in `data/` is load-bearing.
//!
//! **Sean's requirement, in his words**: *there should not be a single value I can change or delete
//! that doesn't end up breaking something.*
//!
//! So this deletes each row in turn, and changes each value in turn, and asserts that something
//! notices. **A row that can be deleted without anything failing is a row nobody reads**, and a
//! value that can be changed without anything failing is a value nobody reads.
//!
//! # What counts as noticing
//!
//! [`check`] is everything the suite asserts about the data: the test script runs, the report says
//! the state is as expected, the report names the test, the title and the four relations it
//! compared, and `engine.4x` still matches the constants in `src/`. **A mutation survives if all of
//! that still holds**, which means the thing mutated changed nothing anybody looks at.
//!
//! # Two mutations, because one of them asks the wrong question of an id
//!
//! **Changing a value to `mutated` asks *is this value read at all*.** That is the right question
//! for most columns and the wrong one for a key: **an id's job is to be distinct, and `mutated` is
//! still distinct**, so every id in the data survived it and looked dead.
//!
//! **So a value is also swapped for another value from the same column.** An id swapped for
//! another row's id collides, and the key check refuses it. A `seq` swapped for another `seq`
//! reorders something, and whether that breaks anything is exactly the question worth asking.
//!
//! **A value is load-bearing if either mutation is noticed.** One asks whether it is read; the
//! other asks whether *this* value is the one that matters.

use std::collections::{BTreeMap, BTreeSet};

use thin_engine::engine::Game;
use thin_engine::notation::{Row, read, write};
use thin_engine::schema::Malformed;
use thin_engine::script::{Files, run_test};

mod common;
use common::mine;

/// Every file in `data/`, as text.
fn originals() -> BTreeMap<String, String> {
    let mut all = BTreeMap::new();
    for file in std::fs::read_dir(mine().join("data")).expect("data") {
        let file = file.expect("a file").path();
        if file.extension().map(|it| it != "4x").unwrap_or(true) {
            continue;
        }
        let name = file.file_name().and_then(|it| it.to_str()).expect("a name");
        all.insert(
            name.to_string(),
            std::fs::read_to_string(&file).expect("a file"),
        );
    }
    assert_eq!(all.len(), 8, "eight data files: {:?}", all.keys());
    all
}

struct InMemory(BTreeMap<String, String>);

impl Files for InMemory {
    fn read(&self, name: &str) -> Option<String> {
        self.0.get(name).cloned()
    }
}

/// Every string constant the engine compares a data value against - as `tests/engine.rs` reads it.
fn constants() -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for file in std::fs::read_dir(mine().join("src")).expect("src") {
        let file = file.expect("a module").path();
        if file.extension().map(|it| it != "rs").unwrap_or(true) {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("a module");
        for line in text.split("#[cfg(test)]").next().unwrap_or("").lines() {
            let Some(rest) = line.trim().strip_prefix("const ") else {
                continue;
            };
            let Some((_, value)) = rest.split_once(": &str = \"") else {
                continue;
            };
            let Some(value) = value.strip_suffix("\";") else {
                continue;
            };
            if !value.is_empty() {
                found.insert(value.to_string());
            }
        }
    }
    found
}

/// Everything the suite asserts about the data. `Err` means something noticed.
///
/// **This is the whole of what "breaking something" means here**, so a mutation that survives it
/// is one nothing in the suite looks at - which is the finding rather than a pass.
fn check(files: &InMemory) -> Result<(), String> {
    let script = files
        .read("test.4x")
        .ok_or_else(|| "no test.4x".to_string())?;
    let script = read(&script).map_err(|why| format!("test.4x: {why}"))?;

    let report = run_test(&script, files).map_err(|why| format!("{why}"))?;

    if !report.same() {
        return Err(format!("not as expected: {report}"));
    }
    if report.test != "the-scout-moves-to-an-adjacent-place" {
        return Err(format!("the test is named `{}`", report.test));
    }
    if report.title != "the-first-test" {
        return Err(format!("the report is titled `{}`", report.title));
    }
    if report.compared != ["adjacency", "residency", "territory", "thing"] {
        return Err(format!("compared {:?}", report.compared));
    }

    every_reference_forbids_something(files)?;

    // **The script store's references, counted rather than violated.** They live in the file the
    // first load fetches, which `every_reference_forbids_something` does not reach - it walks the
    // game. The count is what keeps them from being deletable, the same way the count keeps the
    // game's nineteen honest; generating a violation for each is the better check and is not
    // written yet.
    let declarations = read(&files.read("script.4x").unwrap_or_default())
        .map_err(|why| format!("script.4x: {why}"))?;
    let references = declarations
        .iter()
        .filter(|row| row.relation == "reference")
        .count();
    if references != 6 {
        return Err(format!(
            "{references} references in the script, and there are six"
        ));
    }

    // **Read out of what the script loaded, not out of the file.** Reading the file directly
    // made the step that loads it dead: `engine.4x` could be dropped from `test.4x` and this
    // still found the rows. The question is what the game was given, so this asks the game.
    let declared: BTreeSet<String> = loaded(files)?
        .iter()
        .filter(|row| row.relation == "primitive")
        .filter_map(|row| row.value("word").map(str::to_string))
        .collect();
    if declared != constants() {
        return Err("engine.4x is not the constants in src/".to_string());
    }
    Ok(())
}

/// The files `test.4x` loads into the game, read out of the script rather than listed.
fn loaded(files: &InMemory) -> Result<Vec<Row>, String> {
    let script = read(&files.read("test.4x").unwrap_or_default())
        .map_err(|why| format!("test.4x: {why}"))?;
    // **`into` is a store's id, so the stores have to be read before the loads can be.** They
    // are declared in whichever file the first load fetches, which is the bootstrap said from
    // the other side.
    let mut first: Vec<&Row> = script.iter().filter(|row| row.relation == "load").collect();
    first.sort_by_key(|row| {
        row.value("seq")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(0)
    });
    let declarations = first
        .first()
        .and_then(|row| row.value("file"))
        .and_then(|file| files.read(file))
        .unwrap_or_default();
    let declarations = read(&declarations).map_err(|why| format!("{why}"))?;
    let game_store = declarations
        .iter()
        .find(|row| row.relation == "store" && row.value("name") == Some("game"))
        .and_then(|row| row.value("id"))
        .ok_or("no store named `game`")?;

    let mut all = Vec::new();
    for step in script
        .iter()
        .filter(|row| row.relation == "load" && row.value("into") == Some(game_store))
    {
        let file = step.value("file").ok_or("a load with no file")?;
        let text = files.read(file).ok_or_else(|| format!("no file {file}"))?;
        all.extend(read(&text).map_err(|why| format!("{file}: {why}"))?);
    }
    Ok(all)
}

/// **Every `{reference ...}` row forbids something, and this is what makes it do so.**
///
/// A reference is a constraint, and a constraint is worth nothing in a run where nothing violates
/// it - so deleting one changed nothing anybody looked at, and all nineteen showed up as dead.
/// **They were not dead; the suite had no case that needed them.**
///
/// For each reference, this takes a real row of the relation it constrains, points the constrained
/// column at a key nothing has, and asserts the structure refuses it. Delete the reference and
/// that row becomes legal, which is what the mutation check then notices.
fn every_reference_forbids_something(files: &InMemory) -> Result<(), String> {
    let game = loaded(files)?;
    let references: Vec<Row> = game
        .iter()
        .filter(|row| row.relation == "reference")
        .cloned()
        .collect();
    // **The count is exact and that is what makes a reference load-bearing.** Written as a
    // floor first, and deleting a reference then simply meant one fewer was checked - the loop
    // below only ever tests the references that are there. A floor asks *are there enough*; the
    // question is *are they all still here*.
    if references.len() != 19 {
        return Err(format!(
            "{} references, and there are nineteen",
            references.len()
        ));
    }

    for reference in &references {
        let column = reference
            .value("column")
            .ok_or("a reference with no column")?;
        // Which relation the column belongs to, and what it is called there.
        let declaration = game
            .iter()
            .find(|row| row.relation == "column" && row.value("id") == Some(column))
            .ok_or_else(|| format!("no column {column}"))?;
        // **A column names its relation by id**, so the name the rows are written in has to be
        // looked up before they can be found.
        let of = declaration.value("relation").ok_or("a column of nothing")?;
        let of = game
            .iter()
            .find(|row| row.relation == "relation" && row.value("id") == Some(of))
            .and_then(|row| row.value("name"))
            .ok_or_else(|| format!("no relation with id {of}"))?;
        let named = declaration.value("name").ok_or("a column with no name")?;

        // A real row of that relation, pointed at a key nothing has.
        let Some(sample) = game.iter().find(|row| row.relation == of) else {
            return Err(format!(
                "`{of}` has no rows, so `{column}` cannot be violated"
            ));
        };
        let mut violating = sample.clone();
        violating
            .values
            .insert(named.to_string(), "nothing-has-this-key".to_string());
        // **The sample is replaced rather than joined**, which keeps its key and so keeps every
        // reference to it resolving. Adding a second row alongside broke the key's uniqueness
        // instead, and giving the copy a fresh key broke the key's own reference where the key is
        // one - `adjacency` is keyed by `from`, which points at `territory`.
        let mut with_violation: Vec<Row> =
            game.iter().filter(|row| *row != sample).cloned().collect();
        with_violation.push(violating.clone());
        match Game::of(with_violation) {
            Ok(_) => {
                return Err(format!(
                    "{} is allowed, so `{column}` points at nothing",
                    write(&violating)
                ));
            }
            // **Refused for the reference and not for something else.** Without this the check
            // asks *does anything complain*, which is a narrower question than *is this
            // constraint doing the work*.
            Err(Malformed::NoSuchRow { column: at, .. }) if at == named => {}
            // **Three references are backstopped by the schema builder**, which cannot use them:
            // it refuses a column of an undeclared relation, and a reference naming a column or a
            // relation nothing declares, while it is still working out what the relations are.
            // **So `column.relation`, `reference.column` and `reference.to` are not the only
            // thing forbidding their own violation** - the count above is what keeps them
            // load-bearing, not the forbidding.
            Err(Malformed::ColumnOfNothing { .. })
            | Err(Malformed::ReferenceOfNothing { .. })
            | Err(Malformed::ReferencesNothing { .. }) => {}
            Err(why) => {
                return Err(format!(
                    "{} was refused for {why}, which is not `{column}` doing the work",
                    write(&violating)
                ));
            }
        }
    }
    Ok(())
}

/// The rows of one file, each with the line it came from.
fn rows_of(text: &str) -> Vec<(usize, Row)> {
    text.lines()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with('{'))
        .map(|(at, line)| {
            let row = read(line.trim()).unwrap_or_else(|why| panic!("{line}: {why}"));
            (at, row[0].clone())
        })
        .collect()
}

fn without(text: &str, at: usize) -> String {
    text.lines()
        .enumerate()
        .filter(|(line, _)| *line != at)
        .map(|(_, line)| line)
        .collect::<Vec<&str>>()
        .join("\n")
}

fn changed(text: &str, at: usize, row: &Row) -> String {
    text.lines()
        .enumerate()
        .map(|(line, was)| {
            if line == at {
                write(row)
            } else {
                was.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// **The control**: unmutated, everything holds. Without this the two tests below could pass by
/// `check` failing for some reason of its own, which would make every mutation look load-bearing.
#[test]
fn the_data_as_it_stands_passes_every_check() {
    check(&InMemory(originals())).expect("the data as it stands");
}

/// **Every row matters**: delete any one of them and something fails.
#[test]
fn no_row_can_be_deleted_without_breaking_something() {
    let files = originals();
    let mut survived = Vec::new();
    let mut tried = 0;

    for (name, text) in &files {
        for (at, row) in rows_of(text) {
            let mut mutated = files.clone();
            mutated.insert(name.clone(), without(text, at));
            tried += 1;
            if check(&InMemory(mutated)).is_ok() {
                survived.push(format!("{name} {}", row.relation));
            }
        }
    }

    assert_eq!(tried, 219, "every row in `data/` was deleted in turn");

    let mut counted: BTreeMap<String, usize> = BTreeMap::new();
    for one in survived {
        *counted.entry(one).or_default() += 1;
    }
    let counted: Vec<String> = counted
        .iter()
        .map(|(what, how_many)| format!("{how_many} {what}"))
        .collect();

    assert_eq!(
        counted, DELETABLE,
        "the rows nothing reads are not the ones written down"
    );
}

/// **The rows nothing reads, and why each is waiting on a test rather than on a change.**
///
/// Eight bindings, all on a `require` or a `remove` clause. Deleting one makes the pattern weaker
/// and it is still satisfied - `require {residency id:1 where:1}` holds as surely as
/// `require {residency id:1 what:1 where:1}` does.
///
/// **They are not decoration; the suite has no command that lies.** The `what` and `where`
/// bindings on the first clause are what check the command's `what` and `from` against the world,
/// and nothing here sends a command with the wrong `from`. **That is the same shape as the
/// nineteen references**, which looked dead until a violation was generated for each - a
/// constraint is worth nothing in a run where nothing violates it.
///
/// **So the fix is a test and not an edit**, and it is the next thing worth doing here.
const DELETABLE: [&str; 1] = ["8 rules.4x binding"];

/// **Every value matters**: change any one of them and something fails.
#[test]
fn no_value_can_be_changed_without_breaking_something() {
    let files = originals();
    let mut survived = Vec::new();
    let mut tried = 0;

    for (name, text) in &files {
        let all = rows_of(text);
        for (at, row) in &all {
            for column in row.values.keys() {
                let was = row.value(column).unwrap_or_default().to_string();

                // **Every other value this column takes in this file**, so that one of them can
                // stand in for the one that is there.
                let elsewhere: Vec<String> = all
                    .iter()
                    .filter(|(_, other)| other.relation == row.relation)
                    .filter_map(|(_, other)| other.value(column).map(str::to_string))
                    .filter(|value| *value != was)
                    .collect();

                let mut instead = vec!["mutated".to_string()];
                if let Some(other) = elsewhere.first() {
                    instead.push(other.clone());
                }

                let mut noticed = false;
                for value in &instead {
                    let mut altered = row.clone();
                    altered.values.insert(column.clone(), value.clone());
                    let mut mutated = files.clone();
                    mutated.insert(name.clone(), changed(text, *at, &altered));
                    tried += 1;
                    if check(&InMemory(mutated)).is_err() {
                        noticed = true;
                    }
                }
                if !noticed {
                    survived.push(format!("{name} {}.{column}", row.relation));
                }
            }
        }
    }

    assert!(
        tried > 300,
        "only {tried} values were changed, which is too few for this to be a check"
    );

    let mut counted: BTreeMap<String, usize> = BTreeMap::new();
    for one in survived {
        *counted.entry(one).or_default() += 1;
    }
    let counted: Vec<String> = counted
        .iter()
        .map(|(what, how_many)| format!("{how_many} {what}"))
        .collect();

    assert_eq!(
        counted, NOT_LOAD_BEARING,
        "the values nothing reads are not the ones written down"
    );
}

/// **The values nothing reads, named and counted, so the list cannot grow quietly.**
///
/// **Two groups, and neither is an oversight.**
///
/// **The decorations**, which is what Sean has said they are: `rule.name` and `input.name` are
/// read into an error message and nowhere else, because everything references by id. They are
/// waiting on the user-facing style, which is the thing that will read them.
///
/// **`input.seq` and `clause.seq` order things whose order does not matter** - yet. Clauses are
/// applied in role passes, every `require` then every `remove` then every `add`, so two clauses of
/// the same role are interchangeable. **A second rule where two removes contend would change
/// that**, and this line is where to look when it does.
///
/// **Four ids on relations with one row.** `test`, `execute`, `compare` and `report` have a single
/// row each, so there is no other id to swap theirs for - **the instrument cannot ask whether a key
/// is distinct when there is nothing to be distinct from.** Every other id in the data is
/// load-bearing, and was not before keys had to be unique.
const NOT_LOAD_BEARING: [&str; 8] = [
    "4 rules.4x clause.seq",
    "4 rules.4x input.name",
    "4 rules.4x input.seq",
    "1 rules.4x rule.name",
    "1 test.4x compare.id",
    "1 test.4x execute.id",
    "1 test.4x report.id",
    "1 test.4x test.id",
];
