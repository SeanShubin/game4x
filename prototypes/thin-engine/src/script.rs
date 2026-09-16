//! Running a test that is written down: load some files, execute a command, compare, report.
//!
//! # `load` exists and the engine still reads no file
//!
//! **`data/test.4x` says `{load seq:1 file:schema.4x into:game}`, and this module never opens
//! anything.** It is handed a [`Files`] - something that can turn a name into text - and asks it.
//! The harness in `tests/` is what actually reads the directory, which is where `std::fs` is
//! allowed to be.
//!
//! **That is not a dodge, it is the finding.** A command that loads a file cannot be pure data in
//! an engine that reads nothing, so the file system is the first thing the game has needed from
//! outside itself. It arrives as one method rather than as a dependency.
//!
//! # A test is rows, and its steps are five relations
//!
//! ```text
//! {test name:the-scout-moves-to-an-adjacent-place}
//! {load seq:1 file:schema.4x into:game}
//! {execute seq:6 command:1}
//! {compare seq:7 this:actual with:expected}
//! {report seq:8 title:the-first-test}
//! ```
//!
//! **`seq` orders the steps across the four step relations**, so the file reads top to bottom and
//! the relations stay normalized - each has fixed columns, which one `{step ...}` relation
//! carrying a file *or* a command could not.
//!
//! # What is not checked here, said rather than left to be found
//!
//! **The script's own vocabulary is not declared in `data/`.** `schema.4x` describes the game's
//! relations and its own; nothing describes `load`, `execute`, `compare` or `report`, so the list
//! below is the only place they are stated. **It is the one part of this prototype that is not yet
//! self-describing**, and it is a real gap rather than a simplification.

use std::collections::BTreeMap;

use crate::engine::{Game, Refused, fire};
use crate::notation::{Row, Unreadable, read};
use crate::schema::{Malformed, Schema};

const TEST: &str = "test";
const LOAD: &str = "load";
const STATE: &str = "state";

const NAME: &str = "name";
const FILE: &str = "file";
const INTO: &str = "into";
// **The three sections a test is written in.** They carry no values - the name is the whole of the
// row - so they are words the script knows rather than relations `script.4x` declares. A relation
// with no columns has no key and holds no data, and `Malformed::NoColumns` is right to refuse one.
const GIVEN: &str = "given";
const WHEN: &str = "when";
const THEN: &str = "then";
// **The fourth section, and the one that says nothing should happen.** A test either states the
// world the command leaves - `{then}` - or what the rule needed and did not find - `{refused}`.
const REFUSED: &str = "refused";

const SCRIPT: &str = "script";
const STORE: &str = "store";
const ID: &str = "id";
const RELATION: &str = "relation";
const GAME: &str = "game";

/// Somewhere to get a file from, by name.
///
/// **The one thing the engine needs from outside itself**, and the only way it gets it.
pub trait Files {
    fn read(&self, name: &str) -> Option<String>;
}

/// Why a test could not be run - about the test, not about the engine.
#[derive(Debug, PartialEq, Eq)]
pub enum Failed {
    /// A step naming a file nothing can provide.
    NoSuchFile { file: String },
    /// A file that is not rows.
    Unreadable { file: String, why: Unreadable },
    /// A row in the script that is not a step, or a step missing a column.
    BadStep { row: String },
    /// `into` naming somewhere that is not `script`, `game` or `expected`.
    NoSuchStore { into: String },
    /// A step that does not fit the structure `script.4x` declares for it.
    BadlyFormed { step: String, why: Malformed },
    /// `compare` naming something other than the two stores there are.
    NothingToCompare { this: String, with: String },
    /// A test stating both a `{then}` and a `{refused}`.
    ///
    /// **A command leaves a world or it is refused**, and a test saying both says two things about
    /// one command. Refused rather than resolved, because which one was meant is the author's.
    BothEndings { test: String },
    /// A step naming a command nothing fires.
    ///
    /// **A command is named by the rule it fires**, which `spec/invariants.md` calls *the offering
    /// is derived rather than listed* - the name is resolved against what the game holds rather
    /// than against a number somebody wrote down.
    NoSuchCommand { command: String },
    /// The rows loaded do not fit the structure they declare.
    Malformed { why: Malformed },
    /// The command did not happen.
    Refused { why: Refused },
    /// A step that needs an earlier one which did not run.
    OutOfOrder { step: String, needs: String },
}

impl std::fmt::Display for Failed {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Failed::NoSuchFile { file } => write!(out, "there is no file `{file}`"),
            Failed::Unreadable { file, why } => write!(out, "`{file}`: {why}"),
            Failed::BadStep { row } => write!(out, "{row} is not a step this knows"),
            Failed::NoSuchStore { into } => write!(out, "`{into}` is not somewhere to load into"),
            Failed::BadlyFormed { step, why } => write!(out, "{step}: {why}"),
            Failed::NothingToCompare { this, with } => {
                write!(out, "there is nothing to compare `{this}` with `{with}`")
            }
            Failed::BothEndings { test } => {
                write!(
                    out,
                    "`{test}` states a `then` and a `refused`, and a command does one"
                )
            }
            Failed::NoSuchCommand { command } => {
                write!(out, "nothing fires `{command}`")
            }
            Failed::Malformed { why } => write!(out, "{why}"),
            Failed::Refused { why } => write!(out, "{why}"),
            Failed::OutOfOrder { step, needs } => {
                write!(out, "`{step}` needs `{needs}`, which did not run")
            }
        }
    }
}

/// What a comparison found: the relations it looked at, and how the two differ.
///
/// **The relations are carried with the difference rather than worked out again**, so a report
/// can say what it compared. A report that cannot say that is one nobody can tell apart from a
/// report that compared nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Difference {
    compared: Vec<String>,
    missing: Vec<String>,
    extra: Vec<String>,
}

/// What the comparison found, and what the test was.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub test: String,
    /// The relations compared, which is every one the schema marks as state.
    pub compared: Vec<String>,
    /// Rows the expected state has and the actual one does not.
    pub missing: Vec<String>,
    /// Rows the actual state has and the expected one does not.
    pub extra: Vec<String>,
}

impl Report {
    pub fn same(&self) -> bool {
        self.missing.is_empty() && self.extra.is_empty()
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(out, "{}", self.test)?;
        writeln!(out, "  compared  {}", self.compared.join(", "))?;
        if self.same() {
            write!(out, "  result    as expected")
        } else {
            writeln!(out, "  result    NOT as expected")?;
            for row in &self.missing {
                writeln!(out, "  expected  {row}")?;
            }
            for (at, row) in self.extra.iter().enumerate() {
                if at + 1 == self.extra.len() {
                    write!(out, "  actual    {row}")?;
                } else {
                    writeln!(out, "  actual    {row}")?;
                }
            }
            Ok(())
        }
    }
}

/// Run a test written as rows.
pub fn run_test(script: &[Row], files: &dyn Files) -> Result<Report, Failed> {
    let name = script
        .iter()
        .find(|row| row.relation == TEST)
        .and_then(|row| row.value(NAME))
        .unwrap_or("(unnamed)")
        .to_string();

    // **Ordered by where they appear**, so the file reads top to bottom and says so by saying it
    // in that order. Sean, 2026-09-15: line order is a **temporal coupling** - it says when a
    // statement happens and never what an argument means, which stay named.
    //
    // **The `{test ...}` row is validated even though it is not a step.** Skipped entirely at
    // first, which meant its columns were declared in `script.4x` and checked by nothing - the
    // `name` column could be renamed there and nothing noticed.
    // **A prologue of steps, then three sections.** `{given}`, `{when}` and `{then}` are markers:
    // every row after one belongs to it, which is line order carrying grouping as well as
    // sequence. **The sections hold game rows and the prologue holds script rows**, which is why
    // the script store below is checked against the prologue alone.
    let mut named: Vec<&Row> = Vec::new();
    let mut steps: Vec<&Row> = Vec::new();
    let mut section: Option<&str> = None;
    let mut given: Vec<Row> = Vec::new();
    let mut when: Vec<Row> = Vec::new();
    let mut then: Vec<Row> = Vec::new();
    let mut refused: Vec<Row> = Vec::new();
    for row in script {
        let marker = matches!(row.relation.as_str(), GIVEN | WHEN | THEN | REFUSED);
        if marker {
            // **A marker carries nothing**, so anything beside its name is a mistake rather than
            // a value nobody reads.
            if !row.values.is_empty() {
                return Err(Failed::BadStep {
                    row: crate::notation::write(row),
                });
            }
            section = Some(match row.relation.as_str() {
                GIVEN => GIVEN,
                WHEN => WHEN,
                REFUSED => REFUSED,
                _ => THEN,
            });
            continue;
        }
        match (section, row.relation.as_str()) {
            (None, TEST) => named.push(row),
            (None, LOAD) => steps.push(row),
            (Some(GIVEN), _) => given.push(row.clone()),
            (Some(WHEN), _) => when.push(row.clone()),
            (Some(THEN), _) => then.push(row.clone()),
            (Some(REFUSED), _) => refused.push(row.clone()),
            _ => {
                return Err(Failed::BadStep {
                    row: crate::notation::write(row),
                });
            }
        }
    }
    // **`seq` is gone and the bug it carried is unwritable rather than fixed.** It was sorted as
    // text once, so `10` came before `2` and every step after the first ran in the wrong order -
    // `report` reached before `compare` had run. **Nothing can say a wrong order now**, because
    // nothing says an order at all except the order the lines are in.

    let mut declared: Vec<Row> = Vec::new();
    let mut game: Vec<Row> = Vec::new();
    // **Every step is a load now**, which is what dropping `execute`, `compare` and `report` left:
    // a test says what to read, and its three sections say everything else.
    for step in &steps {
        {
            {
                let file = step.value(FILE).ok_or_else(|| Failed::BadStep {
                    row: crate::notation::write(step),
                })?;
                let text = files.read(file).ok_or_else(|| Failed::NoSuchFile {
                    file: file.to_string(),
                })?;
                let rows = read(&text).map_err(|why| Failed::Unreadable {
                    file: file.to_string(),
                    why,
                })?;
                // **The first load is the declarations, by definition** - nothing has been
                // declared yet, so there is no row saying what `into:1` means. That is the same
                // bootstrap as the step itself not being validated until afterwards, and it is
                // closed the same way: once the declarations are in, this step's `into` is
                // resolved and refused if it was not the script store after all.
                let bootstrap = declared.is_empty();
                let into = if bootstrap {
                    SCRIPT
                } else {
                    store_named(&declared, step.value(INTO).unwrap_or_default())
                };
                match Some(into) {
                    Some(SCRIPT) => {
                        declared.extend(rows);
                        // **Every step is checked once the declarations are here, loads
                        // included.** Checking only the steps that come after left `load`'s own
                        // columns undeclared in effect - the `into` column could be dropped from
                        // `script.4x` and nothing noticed, because no load was ever validated.
                        // **The script store is a game, and is checked like one.** Every row
                        // fits its relation, every key names one row, and every reference points
                        // at something - which needs `script.4x` to declare `relation`, `column`
                        // and `reference` as well as its own five, and it does. **Checked by hand
                        // first**, which got the fits and missed the keys, so every id in the
                        // script store could be swapped for another and nothing noticed.
                        // **The declarations and the script together**, so the script's own rows
                        // are key-checked as well as fitted. Checking only the declarations left
                        // every id in `test.4x` free to be any value at all.
                        // **The prologue, not the whole file.** The sections hold game rows, and
                        // a `{territory ...}` checked against the script's schema is a row of a
                        // relation the script never declared. **They are checked where they are
                        // used** - `given` when the game is built, `then` when it is compared.
                        let mut whole = declared.clone();
                        whole.extend(named.iter().map(|row| (*row).clone()));
                        whole.extend(steps.iter().map(|row| (*row).clone()));
                        Game::of(whole).map_err(|why| Failed::BadlyFormed {
                            step: crate::notation::write(step),
                            why,
                        })?;
                        for earlier in steps.iter().chain(named.iter()) {
                            fits(earlier, &declared)?;
                        }
                        // **And the bootstrap load's own `into` is checked, late.** Without this
                        // the first load could name any store at all and nothing would notice,
                        // because nothing was there to notice at the time.
                        let said = step.value(INTO).unwrap_or_default();
                        if store_named(&declared, said) != SCRIPT {
                            return Err(Failed::NoSuchStore {
                                into: said.to_string(),
                            });
                        }
                    }
                    Some(GAME) => game.extend(rows),

                    Some(other) => {
                        return Err(Failed::NoSuchStore {
                            into: other.to_string(),
                        });
                    }
                    None => {
                        return Err(Failed::BadStep {
                            row: crate::notation::write(step),
                        });
                    }
                }
            }
        }
    }

    // **Given, when, then - and nothing says to execute or to compare.** The sections say it:
    // `given` is the state the game starts in, `when` is what the player does, `then` is the
    // state it should leave. **Three relations went with the three steps** - `execute`, `compare`
    // and the stores they named.
    game.extend(given);
    let before = Game::of(game).map_err(|why| Failed::Malformed { why })?;

    // **A test says what the world becomes, or what the rule could not find.** `{then}` is the
    // first and `{refused}` the second, and a test carrying both is saying two things about one
    // command.
    if !refused.is_empty() && !then.is_empty() {
        return Err(Failed::BothEndings { test: name });
    }

    let mut actual = before.clone();
    for command in &when {
        // **A command without a `repeat` fires once** - `spec/console.md`. Nothing here writes
        // one yet, and writing one means a column in the foundation and `-> n` in the friendly
        // form, exactly as a quantity is written.
        match fire(&actual, command, 1) {
            Ok(after) => actual = after,
            Err(why) => {
                // **A refusal is the answer when a test asked for one**, and the failure
                // otherwise. `{refused}` names the row the rule required and the world did not
                // have, which is what `Refused::NotSo` carries.
                if refused.is_empty() {
                    return Err(Failed::Refused { why });
                }
                let wanted = match &why {
                    Refused::NotSo { wanted, .. } => wanted.clone(),
                    other => format!("{other}"),
                };
                let said: Vec<String> = refused
                    .iter()
                    .map(|row| before.schema().write(row))
                    .collect();
                return Ok(Report {
                    test: name,
                    compared: vec!["the refusal".to_string()],
                    missing: said.iter().filter(|it| **it != wanted).cloned().collect(),
                    extra: if said.contains(&wanted) {
                        Vec::new()
                    } else {
                        vec![wanted]
                    },
                });
            }
        }
    }

    // **A test that asked to be refused and was not is not as expected**, and says what it wanted
    // rather than passing because nothing went wrong.
    if !refused.is_empty() {
        return Ok(Report {
            test: name,
            compared: vec!["the refusal".to_string()],
            missing: refused
                .iter()
                .map(|row| before.schema().write(row))
                .collect(),
            extra: vec!["nothing was refused".to_string()],
        });
    }

    let difference = compare(&actual, &then)?;
    Ok(Report {
        test: name,
        compared: difference.compared,
        missing: difference.missing,
        extra: difference.extra,
    })
}

/// The name of the store an id names.
///
/// **`into:2` means nothing until the row is looked up.** The script declares its stores as rows
/// like everything else, so the engine branches on `game` or `expected` and the data references
/// the row that says so. A store the script does not declare is left as its id, which then matches
/// none of the names and is refused by name.
fn store_named<'a>(declared: &'a [Row], id: &'a str) -> &'a str {
    declared
        .iter()
        .find(|row| row.relation == STORE && row.value(ID) == Some(id))
        .and_then(|row| row.value(NAME))
        .unwrap_or(id)
}

/// Whether a step fits what `script.4x` declares for its relation.
///
/// **A step is checked like any other row**, so a misspelt column is refused by the structure
/// rather than by a special case here. **Loads are the exception and cannot not be**: the step
/// that fetches the declarations runs before they exist, which is the one place the data cannot
/// describe itself.
fn fits(step: &Row, declared: &[Row]) -> Result<(), Failed> {
    let schema = Schema::of(declared).map_err(|why| Failed::BadlyFormed {
        step: crate::notation::write(step),
        why,
    })?;
    schema.fits(step).map_err(|why| Failed::BadlyFormed {
        step: crate::notation::write(step),
        why,
    })?;
    Ok(())
}

/// The relations the schema marks as state, and what differs in them.
///
/// **Only the state.** The actual holds the structure, the rule and the command as well, because
/// they were loaded - and none of them is something the test is about. `{state relation:...}` is
/// what says which is which, so the scope of the comparison is stated rather than inferred from
/// whatever `expected.4x` happens to mention.
fn compare(actual: &Game, expected: &[Row]) -> Result<Difference, Failed> {
    let schema = actual.schema();
    // **`{state relation:13}` names the relation by id**, so each is resolved to the name the
    // rows are actually written in.
    let mut of_state: Vec<String> = actual
        .rows()
        .rows()
        .iter()
        .filter(|row| row.relation == STATE)
        .filter_map(|row| row.value(RELATION))
        .map(|id| actual.named(RELATION, id).unwrap_or(id).to_string())
        .collect();
    of_state.sort();

    // **Every expected row fits the structure**, checked here rather than taken on trust - a
    // typo in `expected.4x` would otherwise read as the engine getting the answer wrong.
    for row in expected {
        schema.fits(row).map_err(|why| Failed::Malformed { why })?;
    }

    let shown = |rows: &mut Vec<String>, row: &Row| rows.push(schema.write(row));
    let mut mine: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    let mut theirs: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for relation in &of_state {
        let here = mine.entry(relation.as_str()).or_default();
        for row in actual
            .rows()
            .rows()
            .iter()
            .filter(|it| it.relation == *relation)
        {
            shown(here, row);
        }
        here.sort();
        let there = theirs.entry(relation.as_str()).or_default();
        for row in expected.iter().filter(|it| it.relation == *relation) {
            shown(there, row);
        }
        there.sort();
    }

    let mut missing = Vec::new();
    let mut extra = Vec::new();
    for relation in &of_state {
        let here = mine.get(relation.as_str()).cloned().unwrap_or_default();
        let there = theirs.get(relation.as_str()).cloned().unwrap_or_default();
        for row in &there {
            if !here.contains(row) {
                missing.push(row.clone());
            }
        }
        for row in &here {
            if !there.contains(row) {
                extra.push(row.clone());
            }
        }
    }
    Ok(Difference {
        compared: of_state,
        missing,
        extra,
    })
}
