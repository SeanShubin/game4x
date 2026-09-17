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
    // **The shared files, then the tests.** A test is one file in `tests/` and nothing else is,
    // so both are read rather than listed - a list here would be a second place to remember.
    let root = mine().join("data").join("foundation");
    for at in [root.clone(), root.join("tests")] {
        let under = at == root.join("tests");
        for file in std::fs::read_dir(&at).expect("data") {
            let file = file.expect("a file").path();
            if file.extension().map(|it| it != "4x").unwrap_or(true) {
                continue;
            }
            let name = file.file_name().and_then(|it| it.to_str()).expect("a name");
            let name = if under {
                format!("tests/{name}")
            } else {
                name.to_string()
            };
            all.insert(name, std::fs::read_to_string(&file).expect("a file"));
        }
    }
    // **Six shared files and at least one test**, rather than a number every new test would move.
    // Sean, 2026-09-15: *I intend to have one test per file.*
    let tests = all.keys().filter(|it| it.starts_with("tests/")).count();
    assert_eq!(all.len() - tests, 5, "five shared files: {:?}", all.keys());
    assert!(tests > 0, "no tests, so mutating proves nothing");

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
    // **Every test, not one.** `data/foundation/tests/` holds one test per file, so mutating a row
    // breaks something if any of them notices - and adding a test widens this without editing it.
    let script = read(&files.read("setup.4x").unwrap_or_default())
        .map_err(|why| format!("setup.4x: {why}"))?;
    let named: Vec<String> = files
        .0
        .keys()
        .filter(|name| name.starts_with("tests/"))
        .cloned()
        .collect();
    if named.is_empty() {
        return Err("no test".to_string());
    }
    for one in &named {
        let mut whole = script.clone();
        whole.extend(
            read(&files.read(one).unwrap_or_default()).map_err(|why| format!("{one}: {why}"))?,
        );
        let report = run_test(&whole, files).map_err(|why| format!("{why}"))?;
        if !report.same() {
            return Err(format!("not as expected: {report}"));
        }
        // **A test's name is its file's name**, which is checkable without a literal every new
        // test would have to add. **Dropping the literal left `{test name:...}` read by nothing**
        // and the mutation check said so immediately - both test rows became deletable.
        let stem = one
            .trim_start_matches("tests/")
            .trim_end_matches(".4x")
            .to_string();
        if report.test != stem {
            return Err(format!("`{one}` names the test `{}`", report.test));
        }

        // **A test compares a world or a refusal**, and either is a real comparison. What would
        // not be is comparing nothing, which is what this rules out.
        let state = [
            "adjacency",
            "deposit",
            "extractor",
            "food",
            "labor",
            "metal",
            "scout",
            "territory",
        ];
        if report.compared != state && report.compared != ["the refusal"] {
            return Err(format!("compared {:?}", report.compared));
        }
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
    if references != 4 {
        return Err(format!(
            "{references} references in the script, and there are four"
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
    let named = files
        .0
        .keys()
        .find(|name| name.starts_with("tests/"))
        .cloned()
        .ok_or("no test")?;
    loaded_from(files, &named)
}

/// The rows one named test loads, which is the script, the ruleset and that test's own world.
///
/// **Which test is named used to be *whichever sorted first*.** That was invisible and it moved:
/// adding a deposit test put a world with no `adjacency` at the front of the map, and the
/// reference check failed saying a relation it had always found rows for had none. **The check
/// was right and its world was arbitrary.**
fn loaded_from(files: &InMemory, named: &str) -> Result<Vec<Row>, String> {
    // **A test's script is what everything loads, then the test.** `setup.4x` holds the loads so
    // that a test file is its name and its three sections and nothing else.
    let mut script = read(&files.read("setup.4x").unwrap_or_default())
        .map_err(|why| format!("setup.4x: {why}"))?;
    script.extend(
        read(&files.read(named).unwrap_or_default()).map_err(|why| format!("{named}: {why}"))?,
    );
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
    // **And the state, which is no longer loaded from anywhere.** It is the `{given}` section of
    // the test, so a game built from the loads alone has a schema and a rule and no world - which
    // is what `adjacency has no rows` said when this was not here.
    let mut inside = false;
    for row in &script {
        if matches!(row.relation.as_str(), "given" | "when" | "then") {
            inside = row.relation == "given";
            continue;
        }
        if inside {
            all.push(row.clone());
        }
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
    // **Every test's world, not one of them.** No single world holds a row of every relation a
    // reference points at - a deposit test has no border and a movement test has no deposit - so
    // a reference is violated wherever there is something to violate it in, and what is asserted
    // is that all of them were violated somewhere.
    let worlds: Vec<String> = files
        .0
        .keys()
        .filter(|name| name.starts_with("tests/"))
        .cloned()
        .collect();
    if worlds.is_empty() {
        return Err("no test".to_string());
    }
    let mut violated: BTreeSet<String> = BTreeSet::new();
    let mut named: BTreeMap<String, String> = BTreeMap::new();
    for world in &worlds {
        every_reference_forbids_something_in(files, world, &mut violated, &mut named)?;
    }
    if named.len() != REFERENCES {
        return Err(format!(
            "{} references, and there are {REFERENCES}",
            named.len()
        ));
    }
    // **What no world can violate is named rather than counted.** A count said *36 of 37* and
    // left a reader to find which - and the one it could not reach turned out to be a fact worth
    // knowing: no test states food in its `given`, so food only ever appears as an outcome.
    let unreachable: Vec<String> = named
        .iter()
        .filter(|(id, _)| !violated.contains(*id))
        .map(|(_, name)| name.clone())
        .collect();
    if unreachable != UNREACHABLE {
        return Err(format!(
            "these references are violated in no world: {unreachable:?}"
        ));
    }
    Ok(())
}

const REFERENCES: usize = 37;

/// **References no test world can violate**, because nothing points at them there.
///
/// **`food.where` is the only one, and it says something about the tests rather than the data.**
/// Food is produced by `work` and stated by nobody: it appears in a `then` and never in a `given`.
/// **The day a test starts with food already in a territory this list goes empty**, and that is
/// the whole of what it is for.
const UNREACHABLE: [&str; 1] = ["food.where"];

fn every_reference_forbids_something_in(
    files: &InMemory,
    world: &str,
    violated: &mut BTreeSet<String>,
    named: &mut BTreeMap<String, String>,
) -> Result<(), String> {
    let game = loaded_from(files, world)?;
    let references: Vec<Row> = game
        .iter()
        .filter(|row| row.relation == "reference")
        .cloned()
        .collect();
    // **The count is exact and that is what makes a reference load-bearing.** Written as a
    // floor first, and deleting a reference then simply meant one fewer was checked - the loop
    // below only ever tests the references that are there. A floor asks *are there enough*; the
    // question is *are they all still here*.
    if references.len() != REFERENCES {
        return Err(format!(
            "{} references, and there are {REFERENCES}",
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
        let column_name = declaration.value("name").ok_or("a column with no name")?;
        named.insert(column.to_string(), format!("{of}.{column_name}"));
        let column_name = column_name.to_string();

        // A real row of that relation, pointed at a key nothing has.
        // **Nothing to violate is not a failure here**, because the caller asks the question
        // across every world rather than in one.
        let Some(sample) = game.iter().find(|row| row.relation == of) else {
            continue;
        };
        violated.insert(column.to_string());
        let mut violating = sample.clone();
        violating
            .values
            .insert(column_name.clone(), "nothing-has-this-key".to_string());
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
            Err(Malformed::NoSuchRow { column: at, .. }) if at == column_name => {}
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

    assert!(
        tried > 150,
        "only {tried} rows were deleted in turn, so a count proves nothing"
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
        counted, DELETABLE,
        "the rows nothing reads are not the ones written down"
    );
}

/// **The rows nothing reads, and why each is waiting on a test rather than on a change.**
///
/// **Bindings on a `require` or a `remove` clause.** Deleting one makes the pattern weaker and it
/// is still satisfied - `require {residency what:1}` holds as surely as
/// `require {residency what:1 where:1}` does. They are not decoration; a constraint is worth
/// nothing in a run where nothing violates it, which is the same shape as the references that
/// looked dead until a violation was generated for each.
///
/// **`build-extractor` arrived under-tested, and this is where that shows.** Its three bindings
/// and four of the literals can go without any test noticing: one test builds one extractor from
/// exactly one labor and one metal, so a `remove` that loses its `what` still takes something, and
/// one that loses its quantity falls back to taking the row - **which is the same answer when the
/// row holds one**. The masking is the fallback doing what it was written to do, and it wants a
/// test with two of something before it can be trusted.
///
/// **This said six until a test was written that refuses, and it says four now.**
/// `the-scout-cannot-cross-where-there-is-no-border` sends a `move` the world does not allow, and
/// two bindings that nothing had ever leaned on became load-bearing. **That is what an error
/// condition is for**: a test that only succeeds exercises no constraint, because a constraint is
/// what stops something.
///
/// **And two rows the other way, which is the cost of a given that reads.** The refusal test
/// states `{adjacency from:1 to:2}` and `{adjacency from:2 to:3}` and needs neither - what it
/// turns on is that nothing says 1 touches 3. **They are there so a reader can see there is a path
/// and it is not a direct one**, and this line is the price of that, said out loud rather than
/// trimmed away.
/// **Two rules later it says ten and five, and the reason is the same one.** `work` and
/// `build-extractor` each bind `where` and `for` on clauses whose pattern still matches without
/// them, because every deposit test has one territory and one or two deposits in it. **A binding
/// is load-bearing when something else could have matched**, and a world with one of everything
/// gives nothing else to match. The masking is the world's, not the rule's.
/// **The unification brought both numbers down.** Three literals named a kind - `value:labor`,
/// `value:metal` - and a clause's relation says that now, so they are gone rather than dead. Two
/// bindings went the same way: `move` no longer binds a `what` column, because what is moved is
/// the relation the clause is about.
const DELETABLE: [&str; 3] = [
    "8 rules.4x binding",
    "2 rules.4x literal",
    "2 tests/the-scout-cannot-cross-where-there-is-no-border.4x adjacency",
];

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
/// **The decorations are gone.** `input.name` was the last of them - read into an error message and
/// nowhere else - and a command that is a row of its own rule binds its inputs *by name*, so
/// `{move what:... from:... to:...}` reads it. **Both names this list used to hold now do work**:
/// `rule.name` when the section fires, `input.name` when it binds.
///
/// **`rule.name` was on that list and is not now**, which is what collapsing `test.4x` bought.
/// `{execute command:move}` resolves a command by the name of the rule it fires, so a decoration
/// became the thing a step is written in. **That is the direction to want**: a value nothing reads
/// is a question, and answering it by finding a reader beats answering it by deleting the column.
///
/// **`input.seq` and `clause.seq` order things whose order does not matter** - yet. Clauses are
/// applied in role passes, every `require` then every `remove` then every `add`, so two clauses of
/// the same role are interchangeable. **A second rule where two removes contend would change
/// that**, and this line is where to look when it does.
///
/// **One id on a relation with one row.** `literal` has a single row, so there is no other id to
/// swap its for - **the instrument cannot ask whether a key is distinct when there is nothing to be
/// distinct from.**
///
/// **Four more were on that list and the columns are gone.** `test`, `execute`, `compare` and
/// `report` were keyed by an id nothing referenced and ordered by a `seq` nothing needed, because
/// the steps are a sequence and the file already says what order they are in. **Nine columns went
/// and the list got shorter by four**, which is the cheaper of the two ways a dead value stops
/// being dead.
///
/// **And one that is none of those groups, which this instrument found rather than anybody
/// predicting where**: `residency.quantity` in `given.4x`. **The world says one scout is in
/// territory 1 and changing that to five breaks nothing**, because `move` never reads it - its
/// `require` and `remove` clauses match on `what` and `where` and leave the quantity unbound, and
/// the quantity that lands at the destination is the `literal` written in the rule.
///
/// **Every category's name, and it is the friendly side that reads them.** `things.4x` states four
/// and the mutation suite runs the foundation, where a thing is `1` and never `scout` - so renaming
/// all four changes nothing it looks at. **`tests/directories.rs` is what reads them**, and it is
/// not what this suite mutates.
///
/// **The quantities went live.** Three of these lines used to be a test's `residency.quantity`,
/// dead because `move` removed rows and added ones. `take` and `put` read them now, and only the
/// refusal test's remains - that test never moves anything, so nothing of its world is counted.
///
/// **And `thing.name` in the refusal test**, which is the same shape one level along: that test
/// never gets as far as moving anything, so the scout's name is never resolved. **A test that is
/// refused reads less of its own world than one that succeeds**, which is worth knowing before
/// reading a short list as a tidy one.
///
/// **That is the arithmetic gap, showing up as dead data rather than as an argument.** Moving one
/// scout out of a territory holding five should leave four, and nothing in `require`, `remove` and
/// `add` can say so - they are set operations over whole rows. **This line is the check that will
/// go red when quantities start being read**, which is the only reason it is worth writing down
/// rather than fixing by binding a column nothing needs yet.
/// **And `reading.id`, which is the newest and the plainest.** A `reading` row is found by the
/// clause it belongs to, exactly as a `literal` is, so nothing ever looks its id up. **It carries
/// one because every relation the structure declares is keyed**, which is the rule paying for
/// itself somewhere it is not needed.
/// **And a density in a test that is refused before anything reads it.** A deposit declares a
/// `density` column, so every deposit row carries one whether the test needs it or not - and a
/// test about running out of room never gets as far as working the deposit. **It is the refusal
/// test's shape rather than a spare value**, the same way that test's `thing.name` is: a command
/// that is refused reads less of its world than one that succeeds.
/// **And `things.4x` left the list by leaving.** Four category names were read only by the
/// friendly side; a kind is a relation now and its name is read by everything, so the entry is not
/// fixed - it is gone.
const NOT_LOAD_BEARING: [&str; 5] = [
    "11 rules.4x clause.seq",
    "7 rules.4x input.seq",
    "1 rules.4x reading.id",
    "1 tests/an-extractor-cannot-be-built-where-the-deposits-are-taken.4x deposit.density",
    "1 tests/the-scout-cannot-cross-where-there-is-no-border.4x scout.quantity",
];
