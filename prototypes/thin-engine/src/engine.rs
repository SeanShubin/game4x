//! The whole of the engine: read a command out of the data, check its rule, apply what it changes.
//!
//! # Everything it reads is a row, including what it is
//!
//! `data/given.4x` holds the structure, the structure's own structure, the roles, the rule, the
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
const LITERAL: &str = "literal";
const ID: &str = "id";
const NAME: &str = "name";
const SEQ: &str = "seq";
const OF: &str = "of";
const RELATION: &str = "relation";
const STATE: &str = "state";
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

    /// The world as indented text: one block per relation the schema marks as state.
    ///
    /// **Sean, 2026-09-16**: *I also like to define an indented text form of the state.* This is
    /// the plainest one the data can say without being told anything new - **grouped, not nested**.
    ///
    /// **Nesting would need one thing nobody has declared**: which column of a relation holds the
    /// thing that contains it. `residency` has `what` and `where` and both are references, and
    /// nothing says `where` is the container - so a tree would be this lane guessing which of two
    /// columns to hang the row from.
    pub fn outline(&self) -> String {
        let mut out = String::from("- root\n");
        let mut of_state: Vec<&str> = self
            .rows
            .rows()
            .iter()
            .filter(|row| row.relation == STATE)
            .filter_map(|row| row.value(RELATION))
            .filter_map(|id| self.named(RELATION, id))
            .collect();
        of_state.sort_unstable();
        for relation in of_state {
            out.push_str(&format!("  - {relation}\n"));
            let mut written: Vec<String> = self
                .rows
                .rows()
                .iter()
                .filter(|row| row.relation == relation)
                .map(|row| self.schema.write(row))
                .collect();
            written.sort();
            for row in written {
                out.push_str(&format!("    - {row}\n"));
            }
        }
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

    /// Every value that identifies a row of `relation`, in the order the rows are held.
    ///
    /// **The identity rather than the key**, because this is what a reference would carry - a
    /// counted relation has no single value to be named by, so it offers none and an input typed
    /// as one has nothing to range over.
    fn keys_of(&self, relation: &str) -> Vec<String> {
        let Some(declared) = self.schema.relation(relation) else {
            return Vec::new();
        };
        if declared.quantity().is_some() {
            return Vec::new();
        }
        self.rows
            .rows()
            .iter()
            .filter(|row| row.relation == relation)
            .filter_map(|row| row.value(declared.identity()))
            .map(str::to_string)
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
            .any(|row| row.relation == relation && row.value(declared.identity()) == Some(value))
    }
}

/// Every row fits its relation, its key is its own, and every reference points at a row.
fn check(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
    // **A key names one row.** A reference is a key, so a key naming two rows is a reference that
    // names neither - and nothing checked it until it was looked for.
    let mut taken: BTreeMap<(&str, Vec<&str>), usize> = BTreeMap::new();
    for row in rows.rows() {
        let Some(relation) = schema.relation(&row.relation) else {
            continue;
        };
        // **A relation is identified or counted and never both**, so a key of several columns and
        // a key of one are read the same way here: `key()` says which columns, and this counts
        // what the row carries in them.
        let key = relation.key();
        let Some(values) = key
            .iter()
            .map(|column| row.value(column))
            .collect::<Option<Vec<&str>>>()
        else {
            continue;
        };
        let seen = taken
            .entry((relation.name.as_str(), values.clone()))
            .or_default();
        *seen += 1;
        if *seen > 1 {
            return Err(Malformed::TwoWithOneKey {
                relation: row.relation.clone(),
                key: key
                    .iter()
                    .zip(values)
                    .map(|(column, value)| (column.to_string(), value.to_string()))
                    .collect(),
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
                .any(|it| it.relation == *to && it.value(declared.identity()) == Some(value));
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

/// Fire a rule under a binding of its inputs, and give back the game it leaves.
///
/// **Split out of [`run`] so that [`offered`] can ask whether a binding would be refused without
/// a `{command ...}` row existing for it.** Reading a command and firing a rule were one function,
/// and only the first half needs a command. **No logic moved** - this is the second half of `run`
/// with its own name.
fn apply(
    game: &Game,
    of_rule: &str,
    rule: String,
    bound: &BTreeMap<String, String>,
    effect: &mut Effect,
) -> Result<Game, Refused> {
    let mut clauses: Vec<&Row> = game
        .of_relation(CLAUSE)
        .into_iter()
        .filter(|row| row.value(RULE) == Some(of_rule))
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
            let wanted = row_of(game, clause, bound, &rule, PATTERN)?;
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
                    if !take(game, &mut after, &wanted, effect) {
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
            ADD => put(
                game,
                &mut after,
                row_of(game, clause, bound, &rule, WHOLE)?,
                effect,
            ),
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

/// The column a relation counts by, where it counts.
fn counted(game: &Game, row: &Row) -> Option<String> {
    game.schema
        .relation(&row.relation)
        .and_then(|it| it.quantity())
        .map(str::to_string)
}

/// Whether `row` carries every value `description` names, in the same relation.
fn describes(row: &Row, description: &Row) -> bool {
    row.relation == description.relation
        && description
            .values
            .iter()
            .all(|(key, value)| row.value(key) == Some(value.as_str()))
}

/// The same row with its quantity left off - what is left is the description.
fn description(row: &Row, quantity: &str) -> Row {
    let mut values = row.values.clone();
    values.remove(quantity);
    Row {
        relation: row.relation.clone(),
        values,
    }
}

/// Take what `wanted` names out of the store.
///
/// **A counted relation is arithmetic and an identified one is a set.** `{residency what:1 where:1
/// quantity:1}` takes one scout from a territory that may hold five; `{adjacency from:1 to:2}`
/// takes the row, because there is nothing there to count.
///
/// **An entry is never zero** - `spec/console.md` - so a row taken down to nothing is removed
/// rather than written as `-> 0`.
///
/// **Taking more than there are is refused**, which is the same answer as taking from nothing:
/// neither leaves a world the rule described.
fn take(game: &Game, after: &mut Store, wanted: &Row, effect: &mut Effect) -> bool {
    let Some(quantity) = counted(game, wanted) else {
        if after.remove(wanted) == 0 {
            return false;
        }
        effect.took.push(wanted.clone());
        return true;
    };
    let Some(taking) = wanted
        .value(&quantity)
        .and_then(|it| it.parse::<i64>().ok())
    else {
        // **A pattern that names no quantity means the row**, which is what `remove` meant
        // before any relation counted, and what it still means for a clause that says nothing
        // about how many.
        if after.remove(wanted) == 0 {
            return false;
        }
        effect.took.push(wanted.clone());
        return true;
    };
    let description = description(wanted, &quantity);
    let Some(there) = after
        .rows()
        .iter()
        .find(|row| describes(row, &description))
        .cloned()
    else {
        return false;
    };
    let held: i64 = there
        .value(&quantity)
        .and_then(|it| it.parse().ok())
        .unwrap_or(0);
    if held < taking {
        return false;
    }
    after.remove(&there);
    if held > taking {
        let mut left = there;
        left.values
            .insert(quantity.clone(), (held - taking).to_string());
        after.add(left);
    }
    // **What the command took, not what the store now holds.** One scout of five leaving is one
    // taken, and the four are not an effect of anything.
    let mut took = description;
    took.values.insert(quantity, taking.to_string());
    effect.took.push(took);
    true
}

/// Put `row` into the store, joining what is already there where the relation counts.
///
/// **Two of a description are one entry**, so arriving where a scout stands makes two rather than
/// a second row - which the key would refuse - or nothing at all, which is what an identical row
/// meeting a set used to do.
fn put(game: &Game, after: &mut Store, row: Row, effect: &mut Effect) {
    let Some(quantity) = counted(game, &row) else {
        effect.made.push(row.clone());
        after.add(row);
        return;
    };
    let adding: i64 = row
        .value(&quantity)
        .and_then(|it| it.parse().ok())
        .unwrap_or(0);
    let description = description(&row, &quantity);
    let there = after
        .rows()
        .iter()
        .find(|it| describes(it, &description))
        .cloned();
    effect.made.push(row.clone());
    match there {
        None => after.add(row),
        Some(there) => {
            let held: i64 = there
                .value(&quantity)
                .and_then(|it| it.parse().ok())
                .unwrap_or(0);
            after.remove(&there);
            let mut joined = there;
            joined.values.insert(quantity, (held + adding).to_string());
            after.add(joined);
        }
    }
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

    // **A literal is a binding whose value is written in the rule rather than passed to it.**
    // An input is typed as a relation and carries one of its keys, so a plain number cannot be
    // one - and a quantity is a plain number. `releases/first-release.md` does the same thing by
    // writing **Qty** in the recipe, so this is the shape the game already has rather than a new
    // idea. **Read after the bindings and into the same map**, because a column takes its value
    // from one place or the other and never both.
    for literal in game
        .of_relation(LITERAL)
        .into_iter()
        .filter(|row| row.value(CLAUSE) == Some(id))
    {
        let column = literal.value(COLUMN).unwrap_or_default();
        let Some((_, name)) = game.schema.column(column) else {
            continue;
        };
        let Some(value) = literal.value(VALUE) else {
            continue;
        };
        values.insert(name.to_string(), value.to_string());
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

/// What one command took out of the world and what it made.
///
/// **Sean, 2026-09-16**: *I like to implement my programs as some form of (old-state, commands) ->
/// (new-state, effects), sometimes omitting effects depending on the architecture.* **This is the
/// effects half**, and it was there all along without a name - `take` and `put` knew what they had
/// done and nobody asked them.
///
/// **It says what the command did, not what the world now holds.** One scout of five leaving is
/// one taken; the four that stayed are nobody's effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    /// The command that caused it, as the player wrote it.
    pub command: Row,
    pub took: Vec<Row>,
    pub made: Vec<Row>,
}

/// Play a list of commands against a world: **(old state, commands) -> (new state, effects)**.
///
/// **The whole of what the engine does, in one signature.** Everything else here is how. A refusal
/// stops the fold and gives back neither a world nor a list, because a command that was refused
/// leaves no state to have arrived at.
///
/// **One effect per command, in the order they were played**, so reading the list beside the two
/// states is reading what happened rather than inferring it from the difference.
pub fn play(game: &Game, commands: &[Row]) -> Result<(Game, Vec<Effect>), Refused> {
    let mut after = game.clone();
    let mut effects = Vec::new();
    for command in commands {
        let (next, effect) = fire(&after, command, 1)?;
        after = next;
        effects.push(effect);
    }
    Ok((after, effects))
}

/// Fire a command written as a rule-named row: `{move what:1 from:1 to:2}`.
///
/// **The row is the command**, so there is no `{command ...}` and no `{argument ...}` to mint and
/// none to read. Its relation is the rule's name and each value is named for one of the rule's
/// inputs. **This is the same shape [`offered`] gives back**, so what the engine says a player may
/// do is what the player writes.
///
/// **`repeat` is how many times it fires** - `spec/console.md`: *A command may carry a `repeat`,
/// which is how many times it fires.* And: *a command without one fires once.*
pub fn fire(game: &Game, command: &Row, repeat: usize) -> Result<(Game, Effect), Refused> {
    let Some(rule) = game
        .of_relation(RULE)
        .into_iter()
        .find(|row| row.value(NAME) == Some(command.relation.as_str()))
    else {
        return Err(Refused::NoSuchCommand {
            id: command.relation.clone(),
        });
    };
    let of_rule = rule.value(ID).unwrap_or_default().to_string();
    let named = command.relation.clone();

    // **Bound by the input's name, which is what the row writes.** A value the structure cannot
    // place is refused here, exactly as it is when a command is read from rows.
    let mut bound: BTreeMap<String, String> = BTreeMap::new();
    for input in game
        .of_relation(INPUT)
        .into_iter()
        .filter(|row| row.value(RULE) == Some(of_rule.as_str()))
    {
        let id = input.value(ID).unwrap_or_default();
        let name = input.value(NAME).unwrap_or_default();
        let Some(given) = command.value(name) else {
            return Err(Refused::Missing {
                rule: named,
                input: name.to_string(),
            });
        };
        let of = input.value(OF).unwrap_or_default();
        let of = game.named(RELATION, of).unwrap_or(of).to_string();
        if !game.has_key(&of, given) {
            return Err(Refused::WrongType {
                rule: named,
                input: name.to_string(),
                value: given.to_string(),
                of,
            });
        }
        bound.insert(id.to_string(), given.to_string());
    }

    let mut after = game.clone();
    let mut effect = Effect {
        command: command.clone(),
        took: Vec::new(),
        made: Vec::new(),
    };
    for _ in 0..repeat {
        after = apply(&after, &of_rule, named.clone(), &bound, &mut effect)?;
    }
    Ok((after, effect))
}

/// Every command the player could fire right now, as rows in the friendly command form.
///
/// `spec/invariants.md`, of a recipe: *The player's are offered wherever their inputs are present,
/// to take or to leave.* And of a choice: *What may be chosen is whatever the game holds, and the
/// offering is derived rather than listed.* **This is that sentence, run.**
///
/// **Offerable means would not be refused**, which is the strongest reading and the cheapest: a
/// candidate is bound and fired, and kept if firing succeeds. **No second copy of what legal
/// means** - `docs/process.md` calls a check that reads a copy of the population a check of the
/// copy, and a predicate written beside [`apply`] would be exactly that.
///
/// **It adds no word to the engine.** Every row it reads - `rule`, `input`, `clause`, `binding` -
/// is vocabulary the engine already had, so the boundary `data/engine.4x` draws does not move.
///
/// **A row's relation is the rule's name and its values are the input names**, so what comes back
/// is what a player would type: `{move what:scout from:territory-1 to:territory-2}`.
///
/// **The cost is a product over every input's type**, which is fine at three territories and is
/// the thing to watch as a world grows. It is measured rather than guarded against, because what
/// this prototype is for is finding out where the data explodes.
pub fn offered(game: &Game) -> Vec<Row> {
    let mut out = Vec::new();
    for rule in game.of_relation(RULE) {
        let Some(of_rule) = rule.value(ID) else {
            continue;
        };
        let named = rule.value(NAME).unwrap_or(of_rule).to_string();

        let mut inputs: Vec<&Row> = game
            .of_relation(INPUT)
            .into_iter()
            .filter(|row| row.value(RULE) == Some(of_rule))
            .collect();
        inputs.sort_by_key(|row| row.value(SEQ).unwrap_or_default().to_string());

        // **Every key of the relation each input is typed as.** That is what *wherever their
        // inputs are present* ranges over, and the type is what bounds it.
        let choices: Vec<(String, String, Vec<String>)> = inputs
            .iter()
            .map(|input| {
                let of = input.value(OF).unwrap_or_default();
                let of = game.named(RELATION, of).unwrap_or(of).to_string();
                let keys = game.keys_of(&of);
                (
                    input.value(ID).unwrap_or_default().to_string(),
                    input.value(NAME).unwrap_or_default().to_string(),
                    keys,
                )
            })
            .collect();

        for bound in every_binding(&choices) {
            let by_id: BTreeMap<String, String> = choices
                .iter()
                .map(|(id, _, _)| id.clone())
                .zip(bound.iter().cloned())
                .collect();
            // **Offering does not care what a command would do**, only that it could - so the
            // effect it would have is built and dropped.
            let mut aside = Effect {
                command: Row {
                    relation: named.clone(),
                    values: BTreeMap::new(),
                },
                took: Vec::new(),
                made: Vec::new(),
            };
            if apply(game, of_rule, named.clone(), &by_id, &mut aside).is_ok() {
                out.push(Row {
                    relation: named.clone(),
                    values: choices
                        .iter()
                        .map(|(_, name, _)| name.clone())
                        .zip(bound)
                        .collect(),
                });
            }
        }
    }
    out.sort_by_key(|row| (row.relation.clone(), format!("{:?}", row.values)));
    out
}

/// Every way of choosing one value for each input, in declared order.
///
/// **Written with loops because the closure form needed the word `move`**, which is a Rust keyword
/// and also the name of the game's one rule. `tests/isolation.rs` reads `src/` for the game's nouns
/// and cannot tell a keyword from a noun - and should not try, because the day it does is the day
/// it stops catching the thing it is for. **The check was right and the code moved.**
fn every_binding(choices: &[(String, String, Vec<String>)]) -> Vec<Vec<String>> {
    let mut all: Vec<Vec<String>> = vec![Vec::new()];
    for (_, _, keys) in choices {
        let mut next = Vec::new();
        for so_far in &all {
            for key in keys {
                let mut one = so_far.clone();
                one.push(key.clone());
                next.push(one);
            }
        }
        all = next;
    }
    all
}

/// The roles a clause may have, so that a test can assert the data uses all of them and no others.
pub fn roles() -> [&'static str; 3] {
    [REQUIRE, REMOVE, ADD]
}
