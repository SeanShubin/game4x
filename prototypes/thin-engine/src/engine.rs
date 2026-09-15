//! The whole of the engine: read a command out of the data, check its rule, apply what it changes.
//!
//! # Everything it reads is a row, including what it is
//!
//! `data/before.4x` holds the structure, the structure's own structure, the roles, the rule, the
//! command and the world, all as rows of declared relations. **The engine is handed that one list
//! and nothing else.** There is no separate schema file, no separate rule file and no command
//! typed at it - a command is `{command id:1 rule:move}` with `{argument ...}` rows beside it.
//!
//! # The words it knows
//!
//! `relation`, `column` and `reference`, in [`crate::schema`]; and `role`, `rule`, `input`,
//! `clause`, `binding`, `command` and `argument` here. **Every one is about how a thing is written
//! down** - `territory`, `residency` and `move` appear in `data/` and in the tests and in no line
//! of `src/` that runs, which `tests/isolation.rs` checks.
//!
//! # Why a clause and a binding are two relations
//!
//! A clause says *require this relation*; a binding says *this column of it takes that input*.
//! **They are split because a relation cannot have columns that change with what it points at.**
//! An earlier version wrote `{effect relation:residency what:$what where:$from}`, whose columns
//! are `residency`'s - so `effect` had no fixed columns and was not a relation at all. That is the
//! whole of what *fully normalized* cost here, and it is four rows becoming twelve.
//!
//! # An input is typed, and that is where *no such place* is caught
//!
//! `of:territory` says the value bound to an input is a `territory`'s key, so a command naming
//! territory 9 is refused by the structure before the rule is looked at. **No rule has to say that
//! the destination exists.**

use std::collections::BTreeMap;

use crate::notation::Row;
use crate::schema::{Malformed, Schema};
use crate::store::Store;

const ROLE: &str = "role";
const RULE: &str = "rule";
const INPUT: &str = "input";
const CLAUSE: &str = "clause";
const BINDING: &str = "binding";
const COMMAND: &str = "command";
const ARGUMENT: &str = "argument";
const ID: &str = "id";
const NAME: &str = "name";
const SEQ: &str = "seq";
const OF: &str = "of";
const RELATION: &str = "relation";
const COLUMN: &str = "column";
const VALUE: &str = "value";
const REQUIRE: &str = "require";
const REMOVE: &str = "remove";
const ADD: &str = "add";

/// Why a command did not happen, said in terms of the data rather than of the engine.
#[derive(Debug, PartialEq, Eq)]
pub enum Refused {
    /// Nothing states a `{command id:...}` with that id.
    NoSuchCommand { id: String },
    /// The command's rule declares an input the command gives no argument for.
    Missing { rule: String, input: String },
    /// An argument's value is not a key of the relation its input is typed as.
    ///
    /// **This is *no such place*, and it arrives from the structure rather than from a rule.**
    WrongType {
        rule: String,
        input: String,
        value: String,
        of: String,
    },
    /// A clause names a role that is not `require`, `remove` or `add`.
    NoSuchRole {
        rule: String,
        clause: String,
        role: String,
    },
    /// A clause has a column bound to no input, so the row it wants cannot be built.
    Unbound {
        rule: String,
        clause: String,
        column: String,
    },
    /// Everything was bound and the world does not agree.
    NotSo { rule: String, wanted: String },
    /// The rule removes something no row matches, so the rule contradicts itself.
    NothingToRemove { rule: String, wanted: String },
    /// The rule left a world that does not fit the structure.
    Broke { rule: String, why: Malformed },
}

impl std::fmt::Display for Refused {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refused::NoSuchCommand { id } => write!(out, "no command is stated with id `{id}`"),
            Refused::Missing { rule, input } => write!(out, "`{rule}` wants `{input}`"),
            Refused::WrongType {
                rule,
                input,
                value,
                of,
            } => {
                write!(
                    out,
                    "`{rule}`.`{input}` is `{value}`, and no `{of}` has that key"
                )
            }
            Refused::NoSuchRole { rule, clause, role } => {
                write!(
                    out,
                    "`{rule}`.`{clause}` has the role `{role}`, which is not one"
                )
            }
            Refused::Unbound {
                rule,
                clause,
                column,
            } => {
                write!(out, "`{rule}`.`{clause}` binds nothing to `{column}`")
            }
            Refused::NotSo { rule, wanted } => write!(out, "`{rule}` needs {wanted} and it is not"),
            Refused::NothingToRemove { rule, wanted } => {
                write!(out, "`{rule}` removes {wanted} and nothing matched")
            }
            Refused::Broke { rule, why } => write!(out, "`{rule}` would leave a world where {why}"),
        }
    }
}

/// Every row there is, and the structure read out of them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    schema: Schema,
    rows: Store,
}

impl Game {
    /// **The rows are checked against the structure they themselves declare** before anything is
    /// run on them, so a game that exists is one where every row fits and every reference points
    /// at something.
    pub fn of(rows: Vec<Row>) -> Result<Game, Malformed> {
        let schema = Schema::of(&rows)?;
        let rows = Store::of(rows);
        check(&schema, &rows)?;
        Ok(Game { schema, rows })
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn rows(&self) -> &Store {
        &self.rows
    }

    /// Every row, in each relation's declared column order, sorted.
    ///
    /// **Sorted because the rows are a set and the order they are held in is nobody's**, so two
    /// games holding the same rows are the same game however they were built.
    pub fn shown(&self) -> Vec<String> {
        let mut out: Vec<String> = self
            .rows
            .rows()
            .iter()
            .map(|row| self.schema.write(row))
            .collect();
        out.sort();
        out
    }

    /// The `name` of the row of `relation` whose id is `id`.
    ///
    /// **The engine branches on names and the data references by ids**, so this is where the two
    /// meet. An id is arbitrary - `role:2` means nothing until the row is looked up - so a value
    /// that selects a code path is resolved here first. **That makes `role.name` and
    /// `relation.name` load-bearing where every other `name` is decoration.**
    pub fn named(&self, relation: &str, id: &str) -> Option<&str> {
        self.rows
            .rows()
            .iter()
            .find(|row| row.relation == relation && row.value(ID) == Some(id))
            .and_then(|row| row.value(NAME))
    }

    fn of_relation(&self, relation: &str) -> Vec<&Row> {
        self.rows
            .rows()
            .iter()
            .filter(|row| row.relation == relation)
            .collect()
    }

    /// Whether any row of `relation` has `value` as its key.
    fn has_key(&self, relation: &str, value: &str) -> bool {
        let Some(declared) = self.schema.relation(relation) else {
            return false;
        };
        self.rows
            .rows()
            .iter()
            .any(|row| row.relation == relation && row.value(declared.key()) == Some(value))
    }
}

/// Every row fits its relation, its key is its own, and every reference points at a row.
fn check(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
    // **A key names one row.** A reference is a key, so a key naming two rows is a reference that
    // names neither - and nothing checked it until it was looked for.
    let mut taken: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    for row in rows.rows() {
        let Some(relation) = schema.relation(&row.relation) else {
            continue;
        };
        let key = relation.key();
        let Some(value) = row.value(key) else {
            continue;
        };
        let seen = taken.entry((relation.name.as_str(), value)).or_default();
        *seen += 1;
        if *seen > 1 {
            return Err(Malformed::TwoWithOneKey {
                relation: row.relation.clone(),
                key: key.to_string(),
                value: value.to_string(),
            });
        }
    }

    for row in rows.rows() {
        let relation = schema.fits(row)?;
        for column in &relation.columns {
            let Some(to) = &column.references else {
                continue;
            };
            let value = row.value(&column.name).unwrap_or_default();
            let declared = schema
                .relation(to)
                .expect("checked when the schema was read");
            let there = rows
                .rows()
                .iter()
                .any(|it| it.relation == *to && it.value(declared.key()) == Some(value));
            if !there {
                return Err(Malformed::NoSuchRow {
                    relation: row.relation.clone(),
                    column: column.name.clone(),
                    value: value.to_string(),
                    to: to.clone(),
                });
            }
        }
    }
    Ok(())
}

/// Run one of the commands the data states, and give back the game it leaves.
///
/// **A new game rather than an edit in place**, so a refusal half way through applying leaves
/// nothing half applied. The caller either has the game after the command or the game before it,
/// and never one in between. **The signature is the whole of that guarantee**, so a test asserting
/// it would pass without the property; `tests/first_test.rs` asserts the observable half and says
/// so.
pub fn run(game: &Game, command: &str) -> Result<Game, Refused> {
    let Some(stated) = game
        .of_relation(COMMAND)
        .into_iter()
        .find(|row| row.value(ID) == Some(command))
    else {
        return Err(Refused::NoSuchCommand {
            id: command.to_string(),
        });
    };
    let of_rule = stated.value(RULE).unwrap_or_default().to_string();
    // **The id identifies and the name is what a refusal says.** Every filter below is by id;
    // this is only ever read into a message.
    let rule = game.named(RULE, &of_rule).unwrap_or(&of_rule).to_string();

    // **An argument per declared input, each of the input's declared type.** A value the structure
    // cannot place is refused here, which is why no rule says *the destination exists*.
    let arguments: BTreeMap<&str, &str> = game
        .of_relation(ARGUMENT)
        .into_iter()
        .filter(|row| row.value(COMMAND) == Some(command))
        .filter_map(|row| Some((row.value(INPUT)?, row.value(VALUE)?)))
        .collect();

    let mut inputs: Vec<&Row> = game
        .of_relation(INPUT)
        .into_iter()
        .filter(|row| row.value(RULE) == Some(of_rule.as_str()))
        .collect();
    inputs.sort_by_key(|row| row.value(SEQ).unwrap_or_default().to_string());

    let mut bound: BTreeMap<String, String> = BTreeMap::new();
    for input in inputs {
        let id = input.value(ID).unwrap_or_default();
        let named = input.value(NAME).unwrap_or_default().to_string();
        let Some(given) = arguments.get(id) else {
            return Err(Refused::Missing { rule, input: named });
        };
        // `of` is a relation's id; `has_key` wants its name.
        let of = input.value(OF).unwrap_or_default();
        let of = game.named(RELATION, of).unwrap_or(of).to_string();
        if !game.has_key(&of, given) {
            return Err(Refused::WrongType {
                rule,
                input: named,
                value: (*given).to_string(),
                of,
            });
        }
        bound.insert(id.to_string(), (*given).to_string());
    }

    let mut clauses: Vec<&Row> = game
        .of_relation(CLAUSE)
        .into_iter()
        .filter(|row| row.value(RULE) == Some(of_rule.as_str()))
        .collect();
    clauses.sort_by_key(|row| row.value(SEQ).unwrap_or_default().to_string());

    // **Requiring happens before anything is applied**, which is what makes the two passes below
    // safe to write as two rather than one.
    let mut after = game.rows.clone();
    for pass in [REQUIRE, REMOVE] {
        for clause in &clauses {
            let role = clause.value(ROLE).unwrap_or_default();
            let role = game.named(ROLE, role).unwrap_or(role);
            if role != pass {
                continue;
            }
            // **A pattern, not a whole row.** `require` and `remove` match on what they name,
            // so a clause can ask for *an adjacency from here to there* without naming which one.
            // **`add` is the other case and must name every column**, because a row that does not
            // fit the structure cannot be put into the world - including the `id` it will be
            // known by.
            let wanted = row_of(game, clause, &bound, &rule, PATTERN)?;
            match role {
                REQUIRE => {
                    if !game.rows.holds(&wanted) {
                        return Err(Refused::NotSo {
                            rule,
                            wanted: game.schema.write(&wanted),
                        });
                    }
                }
                _ => {
                    if after.remove(&wanted) == 0 {
                        return Err(Refused::NothingToRemove {
                            rule,
                            wanted: game.schema.write(&wanted),
                        });
                    }
                }
            }
        }
    }
    for clause in &clauses {
        let role = clause.value(ROLE).unwrap_or_default();
        match game.named(ROLE, role).unwrap_or(role) {
            REQUIRE | REMOVE => continue,
            ADD => after.add(row_of(game, clause, &bound, &rule, WHOLE)?),
            other => {
                return Err(Refused::NoSuchRole {
                    rule,
                    clause: clause.value(ID).unwrap_or_default().to_string(),
                    role: other.to_string(),
                });
            }
        }
    }

    let schema = Schema::of(after.rows()).map_err(|why| Refused::Broke {
        rule: rule.clone(),
        why,
    })?;
    check(&schema, &after).map_err(|why| Refused::Broke { rule, why })?;
    Ok(Game {
        schema,
        rows: after,
    })
}

/// The row a clause is about, with every column taking the value its binding names.
/// Whether a clause must name every column of its relation, or only the ones it constrains.
const WHOLE: bool = true;
const PATTERN: bool = false;

fn row_of(
    game: &Game,
    clause: &Row,
    bound: &BTreeMap<String, String>,
    rule: &str,
    whole: bool,
) -> Result<Row, Refused> {
    let relation = clause.value(RELATION).unwrap_or_default();
    let relation = game
        .named(RELATION, relation)
        .unwrap_or(relation)
        .to_string();
    let id = clause.value(ID).unwrap_or_default();
    let bindings: Vec<&Row> = game
        .of_relation(BINDING)
        .into_iter()
        .filter(|row| row.value(CLAUSE) == Some(id))
        .collect();

    let declared = game.schema.relation(&relation);
    let mut values = BTreeMap::new();
    for binding in bindings {
        let column = binding.value(COLUMN).unwrap_or_default();
        let Some((_, name)) = game.schema.column(column) else {
            continue;
        };
        let input = binding.value(INPUT).unwrap_or_default();
        let Some(value) = bound.get(input) else {
            continue;
        };
        values.insert(name.to_string(), value.clone());
    }

    // **Every column of the relation has to be bound**, said here rather than left to the
    // structure check - a half-built row would otherwise be reported as one whose columns are
    // wrong, which names the symptom instead of the clause.
    if let Some(declared) = declared
        && whole
    {
        for column in &declared.columns {
            if !values.contains_key(&column.name) {
                return Err(Refused::Unbound {
                    rule: rule.to_string(),
                    clause: id.to_string(),
                    column: column.name.clone(),
                });
            }
        }
    }
    Ok(Row { relation, values })
}

/// The roles a clause may have, so that a test can assert the data uses all of them and no others.
pub fn roles() -> [&'static str; 3] {
    [REQUIRE, REMOVE, ADD]
}
