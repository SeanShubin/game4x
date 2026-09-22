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
            "ark",
            "bin",
            "capacity",
            "citizen",
            "consumes",
            "deposit",
            "energy",
            "extractor",
            "food",
            "labor",
            "metal",
            "pioneer",
            "place",
            "provides",
            "scout",
            "territory",
            "transport",
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

const REFERENCES: usize = 67;

/// **References no test world can violate**, because nothing points at them there.
///
/// **It is empty, and the sentence that stood here is why.** `food.where` was the only entry: food
/// was produced by `work` and stated by nobody, appearing in a `then` and never in a `given`. *The
/// day a test starts with food already in a territory this list goes empty, and that is the whole
/// of what it is for.* **`upkeep` is that day** - a citizen eats food that is already there.
///
/// **An empty list is not the same as no list.** Every reference in the data is now violated by
/// some world, which is the strongest thing this can say; keeping the constant is what makes the
/// next unreachable one arrive as a failure rather than as a silence.
const UNREACHABLE: [&str; 0] = [];

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
/// Run one job per mutation, across as many threads as the machine has.
///
/// **Every mutation is independent**: clone the files, change one thing, ask whether anything
/// noticed. Nothing a job does is visible to another, so the only shared state is the list of
/// what survived - and that is gathered per thread and joined afterwards rather than locked.
///
/// **Sean, 2026-09-19**: *Go ahead and implement parallelism on the sweep at the next available
/// opportunity.* It had reached forty-five minutes and is quadratic - mutations grow with the data
/// and each one runs every test - so the constant factor is worth having. **Nothing about what the
/// sweep means changes**; it asks the same questions in a different order.
///
/// **No crate.** `std::thread::scope` borrows the originals without moving them, which is the
/// whole of what this needed, and the prototype stays a thing with no dependencies.
fn swept<Job, Made>(jobs: Vec<Job>, made: Made) -> Vec<String>
where
    Job: Sync,
    Made: Fn(&Job) -> Option<String> + Sync,
{
    let threads = std::thread::available_parallelism()
        .map(|it| it.get())
        .unwrap_or(4);
    let each = jobs.len().div_ceil(threads).max(1);
    let made = &made;
    let mut survived = Vec::new();
    std::thread::scope(|scope| {
        let mut running = Vec::new();
        for chunk in jobs.chunks(each) {
            running
                .push(scope.spawn(move || chunk.iter().filter_map(made).collect::<Vec<String>>()));
        }
        for one in running {
            survived.extend(one.join().expect("a sweep thread"));
        }
    });
    survived
}

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
    // **The control first, or this test lies.** When `check` fails on unmutated data every
    // mutation fails too, so nothing survives and the list comes back empty - which reads exactly
    // like *everything is load-bearing* and is nothing of the kind. **It has happened twice**, both
    // times a `{reference ...}` row added without `REFERENCES` being bumped, and both times the
    // empty list was the first thing a reader saw. `the_data_as_it_stands_passes_every_check` says
    // it plainly; this makes sure it is said here too, before the number that would mislead.
    check(&InMemory(files.clone())).expect("the data as it stands");

    // **The jobs are described rather than built.** Holding four thousand mutated copies of the
    // data at once is a lot of memory for no reason; a job is a file, a line, and what to call it.
    let jobs: Vec<(String, usize, String)> = files
        .iter()
        .flat_map(|(name, text)| {
            rows_of(text)
                .into_iter()
                .map(|(at, row)| (name.clone(), at, row.relation.clone()))
                .collect::<Vec<_>>()
        })
        .collect();
    let tried = jobs.len();
    let survived = swept(jobs, |(name, at, relation)| {
        let mut mutated = files.clone();
        mutated.insert(name.clone(), without(files.get(name)?, *at));
        check(&InMemory(mutated))
            .is_ok()
            .then(|| format!("{name} {relation}"))
    });

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
/// **And the loop test brought three more down, which is the clearest thing it has said.** Adding
/// `an-ark-lands-a-planet-is-developed-and-an-ark-leaves` made three literals load-bearing that
/// every single-rule test had left dead - **a world where one rule's leavings are another's
/// ingredients reads values that a world stopped at one command never reaches.** It is the
/// argument for an acceptance test, made by the instrument rather than by anyone's reasoning.
/// **And three tests state a readiness they do not spend.** Each is a refusal: two of them refuse
/// before reaching the readiness clause, and the third refuses on the readiness whether it was
/// stated as one or as none. **They are there so a reader can see the refusal is about the one
/// thing the test is named for** - the labor in one, the key in another - and this line is the
/// price of that, said rather than trimmed.
/// **Two `{move ...}` rows are deletable, and that is the cost Sean chose.** Each is the first of
/// two commands in a refusal test, and either command alone produces the same refusal - because
/// `{refused}` names what the rule wanted and not which command wanted it. He, on whether to add
/// the index: *I don't need to say which command was refused on a multi line command, don't want
/// to encorage too many lines in the test.* **So the test says less than it looks like it says**,
/// and what pins the behaviour is the test beside it where the command succeeds.
///
/// **The same shape accounts for the `scout` and `extractor` rows here.** A world that reaches the
/// refusal by a shorter route reaches it just the same.
/// **Two of the three `{attribute ...}` rows are deletable and the third is not**, which says
/// what each is doing. Marking `attribute.relation` and `relation-of.input` keeps those two
/// relations keyed by their first column as they were before the key rule changed - and no data
/// test states a second row that would collide, so only `tests/structure.rs` holds them.
/// **All three `{attribute ...}` rows are deletable now, and the deposit's `density` is the one
/// that changed.** It was load-bearing everywhere while the deposit limit compared extractor and
/// deposit key for key: without the mark, `density` rejoined the deposit's key and there was
/// nothing to compare. **A capacity reads the column and not the key**, so folding the limit into
/// the table on 2026-09-20 took the last `.4x` reader of that mark away.
///
/// **It is not decoration, and this is the third entry that has to say so**: `a_deposit_cannot_
/// have_two_densities` in `tests/structure.rs` is what holds it, and the sweep runs the `.4x`
/// tests and the reference checks rather than the Rust suite.
const DELETABLE: [&str; 23] = [
    "12 rules.4x binding",
    "12 rules.4x literal",
    "3 schema.4x attribute",
    // **`stock`'s `quantity`, and it is read by `tests/structure.rs` rather than by `data/`.** A
    // family's columns are what its members must have, so declaring `quantity` is what stops a
    // relation that is merely somewhere from being a stock - and
    // `a_member_has_every_column_its_family_declares` is the poison that says so.
    //
    // **The sweep cannot see that**, for the same reason it calls `perish`'s name unread: it runs
    // the `.4x` tests and the reference checks, not the Rust suite. **What this list means is *no
    // test in `data/` reads it***, and the check that does read it was written because this entry
    // appeared - the sweep pointing at a column and finding an unwritten test behind it.
    "1 schema.4x column",
    // **Two memberships nothing reads.** Which two is not measured here - the sweep counts and
    // does not name - and the candidates are `ark` in `unit`, `pioneer` in `unit`, and `energy` in
    // `resource`, each of which buys something no test has asked for yet: moving an ark, moving a
    // pioneer, or a transport's templated container covering fuel.
    //
    // **`pioneer` in `founder` and `ark` in `founder` are not among them**, because `deploy` takes
    // a founder as its argument and a member of no family is not one.
    "1 schema.4x member",
    // **All three `{stands-in ...}` rows, and only `tests/structure.rs` reads them.** A `.4x` test
    // states a world, and a world that breaks this rule is not one - so no test in `data/` can
    // exercise it, and `nothing_stands_where_its_kind_may_not` is where every half lives.
    //
    // **The pioneer's row is the one that carries an argument.** `deploy` says nothing about the
    // layer of the place it is given, because an ark may only be asked from an orbit and a pioneer
    // only from a surface - so *pioneer deploys from same territory* and *surface deploys to
    // surface* are consequences of these rows rather than rules of that one. **The claim rests on
    // a row this sweep calls dead**, which is why the Rust check was widened the day the pioneer
    // landed rather than left for later.
    //
    // **Same shape as `schema.4x column` above**: the sweep runs the `.4x` tests and the reference
    // checks, not the Rust suite.
    "3 schema.4x stands-in",
    "1 tests/a-scout-arriving-does-not-lend-a-move-to-one-that-has-spent-its-own.4x move",
    "1 tests/a-scout-arriving-does-not-lend-a-move-to-one-that-has-spent-its-own.4x scout",
    "1 tests/a-scout-that-has-moved-cannot-move-again.4x move",
    // **Everything the launch costs, in the test where it never gets that far.** `launch` requires
    // the surface place, then the orbit above it, and only then removes anything - so a territory
    // with no orbit is refused at the second clause and the three resources are never reached.
    //
    // **They are stated so that the refusal cannot be about them**, which is what the test is for
    // and is not something the engine reads. It is the same shape as the adjacency two lines down.
    "1 tests/a-territory-with-no-orbit-cannot-launch.4x energy",
    "1 tests/a-territory-with-no-orbit-cannot-launch.4x labor",
    "1 tests/a-territory-with-no-orbit-cannot-launch.4x metal",
    // **The capacity row in the three refusal tests, and the same sentence covers all three.** A
    // `{refused}` test states a `given` and asserts what the rule said about it; there is no `then`
    // to differ, so a row is read only if the refusal turns on it - and in these three the refusal
    // is about labour, or a readiness, or a key.
    //
    // **They are stated because a world with a deposit and an extractor in it says what a deposit
    // holds**, which is what the fold made every such world do. **The two tests where a deposit
    // being full *is* the refusal are not on this list**, which is what says the row does work
    // where there is work to do.
    "1 tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x capacity",
    "1 tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x extractor",
    "1 tests/an-extractor-cannot-be-worked-without-labor.4x capacity",
    // **The adjacency and the scout, in the test about layers**, and the clause order is why:
    // `move` requires the `from` place, then the `to` place - which takes its layer from the first
    // - and only then the adjacency and the unit. **The second clause is where this test stops**,
    // so nothing after it is read.
    //
    // **Both rows are the story rather than the mechanism.** A neighbouring territory and a scout
    // that could have moved are what make the refusal mean *layers do not cross* rather than
    // *there was nothing to move*, and the engine needs neither to say it.
    "1 tests/nothing-moves-between-the-layers.4x adjacency",
    "1 tests/nothing-moves-between-the-layers.4x scout",
    "1 tests/one-extractors-readiness-is-not-anothers.4x capacity",
    "2 tests/one-extractors-readiness-is-not-anothers.4x extractor",
    "2 tests/the-scout-cannot-cross-where-there-is-no-border.4x adjacency",
    // **A place nothing stands in and nothing points at.** The scout crosses from the first
    // territory to the third, and the second is there only to have no border with either - so the
    // place standing on it is a row the test never uses, exactly like the adjacency one line up.
    "1 tests/the-scout-cannot-cross-where-there-is-no-border.4x place",
    "1 tests/the-scout-cannot-cross-where-there-is-no-border.4x scout",
    // **The ark itself, in the test about an ark.** With no ark at all the *first* `gather` is
    // refused for exactly the reason the second one is - no ark has a `gathering` - so the refusal
    // is the same and the row leaves no trace.
    //
    // **The test still proves what it says**: delete the second `{gather ...}` and nothing is
    // refused. It is the given that cannot be pinned, and it is the third time this shape has
    // appeared - *what is destroyed leaves no trace*, said about an allowance rather than a row.
    "1 tests/the-sun-reaches-an-ark-once-a-turn.4x ark",
];

/// **Every value matters**: change any one of them and something fails.
#[test]
fn no_value_can_be_changed_without_breaking_something() {
    let files = originals();
    // **The control first, or this test lies.** When `check` fails on unmutated data every
    // mutation fails too, so nothing survives and the list comes back empty - which reads exactly
    // like *everything is load-bearing* and is nothing of the kind. **It has happened twice**, both
    // times a `{reference ...}` row added without `REFERENCES` being bumped, and both times the
    // empty list was the first thing a reader saw. `the_data_as_it_stands_passes_every_check` says
    // it plainly; this makes sure it is said here too, before the number that would mislead.
    check(&InMemory(files.clone())).expect("the data as it stands");

    // **One job per value**, carrying the two things to try in its place. **Counted as before**:
    // a job that tries two values is two attempts, so the floor below still means what it meant.
    let mut jobs: Vec<(String, usize, Row, String, Vec<String>)> = Vec::new();
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
                tried += instead.len();
                jobs.push((name.clone(), *at, row.clone(), column.clone(), instead));
            }
        }
    }
    let survived = swept(jobs, |(name, at, row, column, instead)| {
        let text = files.get(name)?;
        let noticed = instead.iter().any(|value| {
            let mut altered = row.clone();
            altered.values.insert(column.clone(), value.clone());
            let mut mutated = files.clone();
            mutated.insert(name.clone(), changed(text, *at, &altered));
            check(&InMemory(mutated)).is_err()
        });
        (!noticed).then(|| format!("{name} {}.{column}", row.relation))
    });

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

/// **The refresh survivor is gone**, and it is what this list is for. It said
/// `refresh-restores-a-spent-extractor` would pass with the extractor starting ready - so the test
/// showed that refresh and work compose rather than that refresh was needed. **The repair was not
/// the obvious one**: a no-op test cannot close it, because both worlds end identically. Spending
/// the extractor inside the test does, and makes its density load-bearing too.
///
/// **A `moving` that never moves is the other shape here.** The berth tests hold vehicles to count
/// them, not to move them, so what those rows say about moves is read by nothing.
const NOT_LOAD_BEARING: [&str; 61] = [
    // **The one `assigns` row's id is read by nothing.** There were two, and changing one id
    // to the other's collided on the key; with one row there is nothing to collide with.
    // **`assigns` may not need an `id` at all** - keyed by `(clause, input, value)` it could not
    // state two assignments of one input on one clause, which is the rule rather than a
    // restriction. That is in `backlog.md` rather than done here.
    "1 rules.4x assigns.id",
    "45 rules.4x clause.seq",
    // **A scoped input's name is read by nothing, and the other eleven are read by name.** A
    // command finds its argument by the input's name and so does a part; an input the engine fills
    // is looked up by neither, because nothing outside the engine ever names it. **So `upkeep`'s
    // `where` and `perish`'s are the two**, and what their names are for is the friendly notation
    // and a person reading the rule.
    "3 rules.4x input.name",
    "20 rules.4x input.seq",
    // **Five of `end-turn`'s ten steps are in an order nothing depends on, and that is a fact
    // about the turn rather than a gap.** Measured on 2026-09-20 by moving each part to the end on
    // its own: `upkeep`, `perish` and `breed` are read - a citizen must eat before it starves and
    // starve before it breeds - and the other seven commute, because `discard-disorder` takes what
    // is over capacity and a `refresh` restores a readiness, and neither can change what the other
    // finds.
    //
    // **The sweep holds two of those seven anyway**, by swapping a `seq` for another rather than
    // by moving one to the end, which is the second mutation earning its place.
    //
    // **It was four until the loop test shortened.** Measured: the count moved when
    // `an-ark-lands-a-planet-is-developed-and-an-ark-leaves` went from four turns to two. I think
    // the reason is that a shorter arc ends fewer turns and so pins fewer orderings, but the only
    // thing measured is the count.
    "5 rules.4x part.seq",
    // **`reading.id` was on this list until `upkeep` grew a second reading.** With one row there
    // was nothing for an id to collide with, so changing it to anything at all was unnoticed; with
    // two, taking the other's value is a key the structure refuses. **A thing that was decoration
    // stopped being decoration**, and nothing but the sweep would have said so.
    //
    // **`perish` is never fired by name, and it is the only rule that is not.** A rule's name is
    // read when a command names it, and `perish` is reached only through the turn - so renaming it
    // changes nothing any `.4x` test can see. **`breed` is fired by name** and `upkeep` is, which
    // is why this is one and not three.
    //
    // **It is not decoration, and the sweep cannot say so.** `tree.txt` prints it, and
    // `the_tree_is_what_the_file_says_it_is` compares that file - but the sweep runs the `.4x`
    // tests and the reference checks, not the Rust suite. **What this list means is *no test in
    // `data/` reads it***, which is narrower than *nothing reads it*.
    "1 rules.4x rule.name",
    "1 schema.4x supply.name",
    // **Two layers left of the twelve that were here, and `move` is what took the other ten.** The
    // `to` place takes its `layer` from the `from` place's, so every move in the suite now reads
    // both - and this list said it would: *these entries empty themselves when the move rules
    // land, and the sweep will say so without anyone editing this list.* **The prediction is the
    // check**, and it is recorded because it came out right rather than because it was made.
    //
    // **What is left is the two places no move touches.** A bin test has one place and no move in
    // it at all; `the-scout-cannot-cross-where-there-is-no-border` has three, and the third is the
    // one the scout never reaches - already on the deletable list a few lines up for the same
    // reason.
    "1 tests/a-bin-cannot-be-built-where-the-capacity-is-taken.4x place.layer",
    // **A `bearing` no world reads, in the two tests where nothing breeds.** Both run out of food
    // before `breed` reaches them, so whether their citizens could bear never comes up - and a
    // world row must name every column its relation declares, so the value has to be *something*.
    // **This is the schema forcing a value rather than a test stating one**, which is the same
    // reason a spent scout's quantity is further down this list.
    "1 tests/a-citizen-eats-and-one-there-is-no-food-for-starves.4x citizen.bearing",
    // **A `laboring` no world reads, in the five tests that end a turn without working.** Nothing
    // in them fires `toil`, so no rule looks at the trait - and the turn restores it last, so a
    // citizen ends every one of these with `laboring:1` whatever it started with. **It is the same
    // class as the `citizen.bearing` above it**, which is unread for the same reason in the tests
    // where nothing breeds.
    "1 tests/a-citizen-eats-and-one-there-is-no-food-for-starves.4x citizen.laboring",
    // **Every ark that deploys, in both of its readinesses.** `deploy` requires an ark and reads
    // neither `moving` nor `gathering`, because it destroys the one it takes - a rule that spends
    // a move on a thing that will not exist afterwards is spending nothing.
    //
    // **The values have to be something and the world has to say what**, which is the same reason
    // a citizen that never breeds still states a `bearing`. It is the schema forcing a value
    // rather than a test stating one.
    "1 tests/a-deployment-places-what-the-ground-has-room-for.4x ark.gathering",
    "1 tests/a-deployment-places-what-the-ground-has-room-for.4x ark.moving",
    "1 tests/a-deployment-with-nowhere-to-mine-still-costs-the-ark.4x ark.gathering",
    "1 tests/a-deployment-with-nowhere-to-mine-still-costs-the-ark.4x ark.moving",
    // **A pioneer's `moving`, for the reason an ark's is unread in every deployment**: `deploy`
    // spends what it is given rather than a move, and a thing that will not exist afterwards has
    // no move worth taking.
    "1 tests/a-pioneer-settles-the-ground-it-is-standing-on.4x pioneer.moving",
    "1 tests/a-scout-arriving-does-not-lend-a-move-to-one-that-has-spent-its-own.4x scout.quantity",
    "1 tests/a-scout-cannot-move-where-every-berth-is-taken.4x adjacency.id",
    "1 tests/a-scout-cannot-move-where-every-berth-is-taken.4x transport.moving",
    // **The ark that starts the win-condition arc, in both its readinesses.** `deploy` reads
    // neither, for the reason the four deployment tests above record: it destroys what it takes.
    "1 tests/a-second-settlement-launches-the-ark-the-first-could-not.4x ark.gathering",
    "1 tests/a-second-settlement-launches-the-ark-the-first-could-not.4x ark.moving",
    // **The quantities of what a launch costs, in the test that never reaches the removing.** The
    // rows themselves are on the deletable list above, with the clause order that explains both.
    "1 tests/a-territory-with-no-orbit-cannot-launch.4x energy.quantity",
    "1 tests/a-territory-with-no-orbit-cannot-launch.4x labor.quantity",
    "1 tests/a-territory-with-no-orbit-cannot-launch.4x metal.quantity",
    // **A `{refused}` test reads very little of its given**, and the ark ones are no exception: the
    // rule says what it said about the world, and there is no `then` world to differ from. So a
    // value is read only if some clause reads it, and `moving`, `gathering`, a density and a count
    // are none of them once the refusal has been reached.
    "1 tests/an-ark-deploys-and-becomes-a-settlement.4x ark.gathering",
    "1 tests/an-ark-deploys-and-becomes-a-settlement.4x ark.moving",
    "1 tests/an-ark-deploys-onto-the-ground-below-it.4x ark.gathering",
    "1 tests/an-ark-deploys-onto-the-ground-below-it.4x ark.moving",
    "1 tests/an-ark-holds-one-energy-and-the-rest-is-lost.4x ark.gathering",
    "1 tests/an-ark-holds-one-energy-and-the-rest-is-lost.4x ark.moving",
    "1 tests/an-ark-lands-a-planet-is-developed-and-an-ark-leaves.4x ark.gathering",
    "1 tests/an-ark-lands-a-planet-is-developed-and-an-ark-leaves.4x ark.moving",
    "1 tests/an-extractor-cannot-be-built-where-the-deposits-are-taken.4x deposit.density",
    "1 tests/an-extractor-cannot-be-built-where-the-deposits-are-taken.4x extractor.working",
    // **And their quantities, for the reason their rows are deletable.** The rows are a few lines
    // up with the sentence that covers both.
    "1 tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x capacity.quantity",
    "1 tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x deposit.density",
    "1 tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x extractor.quantity",
    "1 tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x extractor.working",
    "1 tests/an-extractor-cannot-be-worked-without-labor.4x capacity.quantity",
    "1 tests/an-extractor-cannot-be-worked-without-labor.4x deposit.density",
    "1 tests/breeding-does-not-reach-the-citizens-it-just-made.4x citizen.laboring",
    "1 tests/breeding-stops-when-the-food-does.4x citizen.laboring",
    // **The adjacency and the scout in the layer test, for the same reason their rows are
    // deletable**: `move` stops at the second clause, and nothing after it is read. **The layer
    // itself is not here**, which is the whole point of the test - it is the one value the refusal
    // turns on.
    "1 tests/nothing-moves-between-the-layers.4x adjacency.id",
    "1 tests/nothing-moves-between-the-layers.4x scout.moving",
    "1 tests/nothing-moves-between-the-layers.4x scout.quantity",
    "1 tests/one-extractors-readiness-is-not-anothers.4x capacity.quantity",
    "2 tests/one-extractors-readiness-is-not-anothers.4x deposit.density",
    "2 tests/one-extractors-readiness-is-not-anothers.4x extractor.quantity",
    "2 tests/one-extractors-readiness-is-not-anothers.4x extractor.working",
    "2 tests/the-hungry-perish-after-upkeep-and-not-before.4x citizen.bearing",
    "2 tests/the-hungry-perish-after-upkeep-and-not-before.4x citizen.laboring",
    "1 tests/the-same-free-space-admits-one-kind-and-refuses-another.4x adjacency.id",
    "1 tests/the-same-free-space-admits-one-kind-and-refuses-another.4x scout.moving",
    "1 tests/the-same-free-space-admits-one-kind-and-refuses-another.4x transport.moving",
    "1 tests/the-scout-cannot-cross-where-there-is-no-border.4x place.layer",
    "1 tests/the-scout-cannot-cross-where-there-is-no-border.4x scout.moving",
    "1 tests/the-scout-cannot-cross-where-there-is-no-border.4x scout.quantity",
    "1 tests/the-sun-reaches-an-ark-once-a-turn.4x ark.gathering",
    "1 tests/the-sun-reaches-an-ark-once-a-turn.4x ark.moving",
    "1 tests/the-sun-reaches-an-ark-once-a-turn.4x deposit.density",
    "1 tests/the-sun-reaches-an-ark-once-a-turn.4x deposit.quantity",
    "1 tests/three-citizens-and-ten-food-become-six.4x citizen.laboring",
];
