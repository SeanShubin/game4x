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

use crate::engine::{Game, Refused, run};
use crate::notation::{Row, Unreadable, read};
use crate::schema::{Malformed, Schema};

const TEST: &str = "test";
const LOAD: &str = "load";
const EXECUTE: &str = "execute";
const COMPARE: &str = "compare";
const REPORT: &str = "report";
const STATE: &str = "state";
const SEQ: &str = "seq";
const NAME: &str = "name";
const FILE: &str = "file";
const INTO: &str = "into";
const COMMAND: &str = "command";
const TITLE: &str = "title";
const THIS: &str = "this";
const WITH: &str = "with";
const ACTUAL: &str = "actual";
const SCRIPT: &str = "script";
const RELATION: &str = "relation";
const GAME: &str = "game";
const EXPECTED: &str = "expected";

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
    /// A step whose `seq` is not a number, so the steps cannot be put in order.
    ///
    /// **Every value in this notation is a string**, so ordering by `seq` means deciding what a
    /// `seq` is. Sorted as text, `10` comes before `2` - which is exactly what happened the first
    /// time a tenth step existed, and every step from the second onwards ran in the wrong order.
    OutOfSequence { step: String, seq: String },
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
            Failed::OutOfSequence { step, seq } => {
                write!(out, "{step} is at `{seq}`, which is not a number")
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
    pub title: String,
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
        writeln!(out, "{}", self.title)?;
        writeln!(out, "  test      {}", self.test)?;
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

    // **Ordered across the step relations by `seq`**, so the file reads top to bottom.
    // **The `{test ...}` row is validated even though it is not a step.** Skipped entirely at
    // first, which meant its columns were declared in `script.4x` and checked by nothing - the
    // `name` column could be renamed there and nothing noticed.
    let mut named: Vec<&Row> = Vec::new();
    let mut steps: Vec<&Row> = Vec::new();
    for row in script {
        match row.relation.as_str() {
            TEST => named.push(row),
            LOAD | EXECUTE | COMPARE | REPORT => steps.push(row),
            _ => {
                return Err(Failed::BadStep {
                    row: crate::notation::write(row),
                });
            }
        }
    }
    // **Ordered by `seq` as a number and not as text.** Written the other way first, and the
    // tenth step is what found it: `10` sorts before `2`, so every step after the first ran in
    // the wrong order and `report` was reached before `compare`. A refusal rather than a silent
    // fallback, because a `seq` nobody can order is a script nobody can run.
    let mut ordered: Vec<(usize, &Row)> = Vec::new();
    for step in steps {
        let seq = step.value(SEQ).unwrap_or_default();
        let at = seq.parse::<usize>().map_err(|_| Failed::OutOfSequence {
            step: crate::notation::write(step),
            seq: seq.to_string(),
        })?;
        ordered.push((at, step));
    }
    ordered.sort_by_key(|(at, _)| *at);
    let steps: Vec<&Row> = ordered.into_iter().map(|(_, step)| step).collect();

    let mut declared: Vec<Row> = Vec::new();
    let mut game: Vec<Row> = Vec::new();
    let mut expected: Vec<Row> = Vec::new();
    let mut actual: Option<Game> = None;
    let mut found: Option<Difference> = None;

    for step in &steps {
        match step.relation.as_str() {
            LOAD => {
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
                match step.value(INTO) {
                    Some(SCRIPT) => {
                        declared.extend(rows);
                        // **Every step is checked once the declarations are here, loads
                        // included.** Checking only the steps that come after left `load`'s own
                        // columns undeclared in effect - the `into` column could be dropped from
                        // `script.4x` and nothing noticed, because no load was ever validated.
                        for earlier in steps.iter().chain(named.iter()) {
                            fits(earlier, &declared)?;
                        }
                    }
                    Some(GAME) => game.extend(rows),
                    Some(EXPECTED) => expected.extend(rows),
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
            EXECUTE => {
                fits(step, &declared)?;
                let command = step.value(COMMAND).ok_or_else(|| Failed::BadStep {
                    row: crate::notation::write(step),
                })?;
                let before = Game::of(game.clone()).map_err(|why| Failed::Malformed { why })?;
                actual = Some(run(&before, command).map_err(|why| Failed::Refused { why })?);
            }
            COMPARE => {
                fits(step, &declared)?;
                // **`this` and `with` are read rather than decorative.** They were written before
                // anything looked at them, and a column the data states and the code ignores is
                // exactly the thing this prototype is meant to make visible.
                let this = step.value(THIS).unwrap_or_default();
                let with = step.value(WITH).unwrap_or_default();
                if this != ACTUAL || with != EXPECTED {
                    return Err(Failed::NothingToCompare {
                        this: this.to_string(),
                        with: with.to_string(),
                    });
                }
                let Some(after) = &actual else {
                    return Err(Failed::OutOfOrder {
                        step: COMPARE.to_string(),
                        needs: EXECUTE.to_string(),
                    });
                };
                found = Some(compare(after, &expected)?);
            }
            _ => {
                fits(step, &declared)?;
                let Some(difference) = found.clone() else {
                    return Err(Failed::OutOfOrder {
                        step: REPORT.to_string(),
                        needs: COMPARE.to_string(),
                    });
                };
                return Ok(Report {
                    test: name,
                    title: step.value(TITLE).unwrap_or("(untitled)").to_string(),
                    compared: difference.compared,
                    missing: difference.missing,
                    extra: difference.extra,
                });
            }
        }
    }
    Err(Failed::OutOfOrder {
        step: "the test".to_string(),
        needs: REPORT.to_string(),
    })
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
    let mut of_state: Vec<String> = actual
        .rows()
        .rows()
        .iter()
        .filter(|row| row.relation == STATE)
        .filter_map(|row| row.value(RELATION).map(str::to_string))
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
