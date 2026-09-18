//! What relations there are, what columns each has, and which columns point at another relation.
//!
//! **This module is the answer to *be clear about the data structure*.** The first test was typed
//! in relational notation - `residency (what fk thing, where fk territory)` - and everything here
//! is that made explicit enough for the engine to enforce.
//!
//! # It is read from the same rows as everything else
//!
//! `{relation ...}`, `{column ...}` and `{reference ...}` are rows in `data/given.4x` beside
//! `{territory id:1}`, and they are declared there too, so **the structure describes itself and is
//! checked against its own description**. That is what makes *everything is data* a thing a test
//! can fail rather than a thing to say.
//!
//! # Three things a schema decides
//!
//! **What may be stated.** A row whose relation is not declared, or whose columns are not exactly
//! the declared ones, is refused. There is no open row.
//!
//! **What a value means.** A `{reference column:residency.what to:thing}` row says the value in
//! that column is a `thing`'s key, and the engine checks such a row exists. **A relation's key is
//! its first column** - a convention rather than a dependency, which is why there is no `key`
//! relation.
//!
//! **What order a row is written in.** The order is the relation's own, stated at `seq`, and a row
//! reads back in it rather than alphabetically. That is `P-513`'s question from the data side.

use std::collections::BTreeMap;

use crate::notation::Row;
use crate::store::Store;

const RELATION: &str = "relation";
const COLUMN: &str = "column";
const REFERENCE: &str = "reference";
const ID: &str = "id";
const QUANTITY: &str = "quantity";
const NAME: &str = "name";
const SEQ: &str = "seq";
const TO: &str = "to";
const ATTRIBUTE: &str = "attribute";
const LIMIT: &str = "limit";
const HELD: &str = "held";
const BY: &str = "by";
const FAMILY: &str = "family";
const MEMBER: &str = "member";
const KIND: &str = "kind";
const SUPPLY: &str = "supply";
const PROVIDES: &str = "provides";
const CONSUMES: &str = "consumes";
const WHAT: &str = "what";
const PER: &str = "per";

/// One column: its id, what it is called, and what it points at if anything.
///
/// **The id is how a `binding` names a column**, which is why a column has one at all - `name` is
/// unique only within its relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Column {
    pub id: String,
    pub name: String,
    pub references: Option<String>,
}

/// One relation: its name, and its columns in the order it declares them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    pub name: String,
    pub columns: Vec<Column>,
    /// Columns that are neither the key nor the quantity: facts about the row rather than what
    /// tells it from another.
    ///
    /// **The default is that there are none**, which is why they are marked one at a time rather
    /// than the key being declared. `spec/console.md` - *a description is a kind and every trait
    /// of that thing* - is still what a key is, and this is the exception saying so out loud.
    pub attributes: Vec<String>,
}

impl Relation {
    /// The column a reference names this relation's rows by - always the first.
    ///
    /// **This is identity, not uniqueness**, and the two parted company when a quantity arrived.
    /// A reference needs one value to carry, so a relation something points at is identified by
    /// one column. **A relation nothing points at need not be**, which is what [`Relation::key`]
    /// is about.
    pub fn identity(&self) -> &str {
        &self.columns[0].name
    }

    /// The column holding how many things each row stands for, where there is one.
    ///
    /// **Named rather than declared**, exactly as `id` already is. Sean, 2026-09-15: *it would
    /// make no sense to have both an id and a quantity in the same logical model* - they are one
    /// slot, how a relation tells its rows apart, and `id` had occupied it by name since the
    /// beginning. A table saying which column is the quantity would have stated one half of an
    /// exclusive pair as data and left the other half a convention.
    pub fn quantity(&self) -> Option<&str> {
        self.columns
            .iter()
            .find(|it| it.name == QUANTITY)
            .map(|it| it.name.as_str())
    }

    /// The columns that together tell one row from another.
    ///
    /// **Every column but the quantity, where there is one** - `spec/console.md`: *a description
    /// is a kind and every trait of that thing* [...] *no trait of the thing may be left out*. So
    /// the key is not a subset anybody chooses, and a column added to the relation joins it by
    /// existing. **Where there is no quantity the key is the first column**, as it always was.
    pub fn key(&self) -> Vec<&str> {
        match self.quantity() {
            None => vec![self.identity()],
            Some(quantity) => self
                .columns
                .iter()
                .map(|it| it.name.as_str())
                .filter(|name| *name != quantity)
                .filter(|name| !self.attributes.iter().any(|it| it == name))
                .collect(),
        }
    }
}

/// Every way the data can fail to fit the structure, said about the data and not about the reader.
#[derive(Debug, PartialEq, Eq)]
pub enum Malformed {
    /// A `{column ...}` row naming a relation nothing declares.
    ColumnOfNothing { relation: String, column: String },
    /// A `{reference ...}` row naming a column nothing declares.
    ReferenceOfNothing { column: String },
    /// A relation declared with no columns at all, so it has no key.
    NoColumns { relation: String },
    /// Columns whose `seq` is not 1, 2, 3 and so on.
    BadOrder { relation: String, seq: Vec<String> },
    /// A reference pointing at a relation nothing declares.
    ReferencesNothing { column: String, to: String },
    /// A row of a relation nothing declares.
    NoSuchRelation { relation: String },
    /// A row whose columns are not the declared ones.
    WrongColumns {
        relation: String,
        wanted: String,
        given: String,
    },
    /// Two rows of one relation with the same key.
    ///
    /// **A reference names a row by its key, so a key naming two rows names neither.** Nothing
    /// checked this until it was looked for: two `{thing id:1 ...}` rows were accepted, and
    /// `{residency what:1}` then pointed at both of them.
    TwoWithOneKey {
        relation: String,
        /// Each key column and the value this row carries in it, in declared order.
        ///
        /// **A list because a key may be several columns.** One entry reads exactly as it did
        /// when a key was always one column, so the message did not change for the case that
        /// already existed.
        key: Vec<(String, String)>,
    },
    /// A relation declaring both an `id` and a `quantity`.
    ///
    /// **Sean, 2026-09-15**: *it would make no sense to have both an id and a quantity in the
    /// same logical model.* They are one slot - whether a row is one thing or a count of them -
    /// so carrying both says a row is identified and counted at once, and nothing can be.
    IdAndQuantity { relation: String },
    /// More of a held thing somewhere than there is room for it.
    ///
    /// **One variant for both ways of having no room**, because a row at quantity zero is not
    /// written: a deposit that is full and a deposit that does not exist differ only in the
    /// number, and `wanted` says which by naming the row that would have had to be there.
    Overfull {
        held: String,
        by: String,
        /// The row of `by` that would have had to exist, written out.
        wanted: String,
        /// How much room there actually is, which is `0` where the row is absent.
        room: String,
    },
    /// More of a supply consumed in a place than is provided there.
    ///
    /// **One number for many kinds**: two transports and two scouts are six berths, and the
    /// refusal says so rather than naming whichever row was read last.
    Crowded {
        supply: String,
        /// The place it was counted in, as its key.
        place: String,
        /// The providing that would have had to be there, written out.
        wanted: String,
        /// What is provided.
        room: String,
    },
    /// A relation belongs to a family and does not declare one of the family's columns.
    UnlikeShape {
        family: String,
        member: String,
        column: String,
    },
    /// A limit between two relations whose keys are not the same columns.
    ///
    /// **Held and holder are compared key for key**, so a limit between relations that do not
    /// agree about what a row is keyed by has nothing to compare.
    CannotLimit { held: String, by: String },
    /// A value in a column that points at a row nothing states.
    NoSuchRow {
        relation: String,
        column: String,
        value: String,
        to: String,
    },
}

impl std::fmt::Display for Malformed {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Malformed::ColumnOfNothing { relation, column } => {
                write!(
                    out,
                    "`{column}` is a column of `{relation}`, which is not declared"
                )
            }
            Malformed::ReferenceOfNothing { column } => {
                write!(
                    out,
                    "a reference names the column `{column}`, which is not declared"
                )
            }
            Malformed::NoColumns { relation } => {
                write!(out, "`{relation}` declares no columns, so it has no key")
            }
            Malformed::Overfull {
                held,
                by,
                wanted,
                room,
            } => {
                write!(
                    out,
                    "`{held}` needs {wanted} and there is room for {room} in `{by}`"
                )
            }
            Malformed::Crowded {
                supply,
                place,
                wanted,
                room,
            } => {
                write!(
                    out,
                    "`{place}` consumes {supply} enough for {wanted} and is provided {room}"
                )
            }
            Malformed::UnlikeShape {
                family,
                member,
                column,
            } => {
                write!(out, "`{member}` is a `{family}` and declares no `{column}`")
            }
            Malformed::CannotLimit { held, by } => {
                write!(
                    out,
                    "`{held}` is limited by `{by}` and the two are not keyed alike"
                )
            }
            Malformed::BadOrder { relation, seq } => {
                write!(
                    out,
                    "`{relation}` numbers its columns {seq:?}, and they run from 1"
                )
            }
            Malformed::ReferencesNothing { column, to } => {
                write!(out, "`{column}` points at `{to}`, which is not declared")
            }
            Malformed::NoSuchRelation { relation } => {
                write!(out, "nothing declares a relation `{relation}`")
            }
            Malformed::WrongColumns {
                relation,
                wanted,
                given,
            } => {
                write!(
                    out,
                    "`{relation}` is ({wanted}) and this row gives ({given})"
                )
            }
            Malformed::IdAndQuantity { relation } => {
                write!(
                    out,
                    "`{relation}` declares both an `id` and a `quantity`, and a row is one or the other"
                )
            }
            Malformed::TwoWithOneKey { relation, key } => {
                let said = key
                    .iter()
                    .map(|(column, value)| format!("`{column}` of `{value}`"))
                    .collect::<Vec<String>>()
                    .join(" and ");
                write!(
                    out,
                    "two `{relation}` rows have {said}, so it names neither"
                )
            }

            Malformed::NoSuchRow {
                relation,
                column,
                value,
                to,
            } => {
                write!(
                    out,
                    "`{relation}`.`{column}` is `{value}`, and no `{to}` has that key"
                )
            }
        }
    }
}

/// Every relation there is.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Schema {
    relations: BTreeMap<String, Relation>,
    /// A column's id to the relation it belongs to and what it is called there.
    by_id: BTreeMap<String, (String, String)>,
    /// A family's name to the names of the relations that belong to it.
    ///
    /// **A family is an abstract relation: columns and no rows.** The columns are the shape its
    /// members share and the members are the set - Sean, 2026-09-17, having considered both an
    /// exists/not-exists trait and a set of kinds: *they are the same mechanism*, and
    /// `spec/data/families.4x` writes it as one.
    ///
    /// **Kept here rather than worked out twice.** Both the engine, deciding what an input ranges
    /// over, and the reference check, deciding whether a value is of the right kind, ask the same
    /// question - so it is answered in the one place that already turns ids into names.
    families: BTreeMap<String, Vec<String>>,
    /// Which relation is held by which: `(extractor, deposit)` says there cannot be more
    /// extractors somewhere than there are deposits to hold them.
    ///
    /// **A constraint on the world rather than on a rule.** Sean, 2026-09-17: *We can't place an
    /// extractor if there are no available deposits* - and *the situation should be detectible and
    /// therefore preventable*. Detectable is this; preventable follows, because every rule already
    /// refuses the world it would leave if that world does not fit.
    limits: Vec<(String, String)>,
}

impl Schema {
    /// Read a schema from the `{relation ...}`, `{column ...}` and `{reference ...}` rows among
    /// whatever else is there.
    pub fn of(rows: &[Row]) -> Result<Schema, Malformed> {
        // **A relation is named by its id everywhere except in its own declaration.** The rows
        // that follow say `relation:16`, not `relation:residency`, so this book is what turns one
        // into the other - and it is read first because everything else depends on it.
        let mut named: BTreeMap<String, String> = BTreeMap::new();
        let mut relations: BTreeMap<String, Relation> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == RELATION) {
            let name = row.value(NAME).unwrap_or_default().to_string();
            named.insert(row.value(ID).unwrap_or_default().to_string(), name.clone());
            relations.insert(
                name.clone(),
                Relation {
                    name,
                    columns: Vec::new(),
                    attributes: Vec::new(),
                },
            );
        }

        // **What each column points at, gathered before the columns are built**, so that a
        // reference to a column that is declared later is not an error of ordering.
        let mut points_at: BTreeMap<String, String> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == REFERENCE) {
            let to = row.value(TO).unwrap_or_default();
            points_at.insert(
                row.value(COLUMN).unwrap_or_default().to_string(),
                named.get(to).cloned().unwrap_or_else(|| to.to_string()),
            );
        }

        let mut numbered: BTreeMap<String, Vec<(String, Column)>> = BTreeMap::new();
        let mut by_id: BTreeMap<String, (String, String)> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == COLUMN) {
            let of = row.value(RELATION).unwrap_or_default();
            let of = named.get(of).cloned().unwrap_or_else(|| of.to_string());
            let name = row.value(NAME).unwrap_or_default().to_string();
            let id = row.value(ID).unwrap_or_default().to_string();
            if !relations.contains_key(&of) {
                return Err(Malformed::ColumnOfNothing {
                    relation: of,
                    column: name,
                });
            }
            by_id.insert(id.clone(), (of.clone(), name.clone()));
            numbered.entry(of).or_default().push((
                row.value(SEQ).unwrap_or_default().to_string(),
                Column {
                    id: id.clone(),
                    name,
                    references: points_at.get(&id).cloned(),
                },
            ));
        }

        for column in points_at.keys() {
            if !by_id.contains_key(column) {
                return Err(Malformed::ReferenceOfNothing {
                    column: column.clone(),
                });
            }
        }

        // **Which columns are facts about a row rather than part of what it is.** Read before the
        // columns are attached, because `key()` asks the relation and the relation has to know.
        let mut attributes: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == ATTRIBUTE) {
            let column = row.value(COLUMN).unwrap_or_default();
            let Some((of, name)) = by_id.get(column) else {
                return Err(Malformed::ReferenceOfNothing {
                    column: column.to_string(),
                });
            };
            attributes.entry(of.clone()).or_default().push(name.clone());
        }

        // **A family and its members, resolved to names here** so that nothing downstream has
        // to turn an id into a relation again.
        let mut families: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == FAMILY) {
            let of = row.value(RELATION).unwrap_or_default();
            if let Some(name) = named.get(of) {
                families.entry(name.clone()).or_default();
            }
        }
        for row in rows.iter().filter(|row| row.relation == MEMBER) {
            let (Some(kind), Some(family)) = (row.value(KIND), row.value(FAMILY)) else {
                continue;
            };
            // **Members are kept as ids, because that is what a value carries.** A command says
            // `what:28`, a reference says `to:27`, and every other value in the data is an id -
            // so a family that answered in names would be the one place that did not.
            let Some(family) = named.get(family) else {
                continue;
            };
            if let Some(members) = families.get_mut(family) {
                members.push(kind.to_string());
            }
        }

        let mut limits: Vec<(String, String)> = Vec::new();
        for row in rows.iter().filter(|row| row.relation == LIMIT) {
            let held = row.value(HELD).unwrap_or_default();
            let by = row.value(BY).unwrap_or_default();
            limits.push((
                named.get(held).cloned().unwrap_or_else(|| held.to_string()),
                named.get(by).cloned().unwrap_or_else(|| by.to_string()),
            ));
        }

        for (of, mut columns) in numbered {
            columns.sort_by(|left, right| left.0.cmp(&right.0));
            let seq: Vec<String> = columns.iter().map(|it| it.0.clone()).collect();
            let wanted: Vec<String> = (1..=columns.len()).map(|it| it.to_string()).collect();
            if seq != wanted {
                return Err(Malformed::BadOrder { relation: of, seq });
            }
            let relation = relations.get_mut(&of).expect("declared above");
            relation.columns = columns.into_iter().map(|it| it.1).collect();
            relation.attributes = attributes.get(&of).cloned().unwrap_or_default();
        }

        for relation in relations.values() {
            if relation.columns.is_empty() {
                return Err(Malformed::NoColumns {
                    relation: relation.name.clone(),
                });
            }
            // **Identified or counted, never both.** Checked here rather than left to the key
            // computation, which would otherwise quietly drop `id` out of the key and go on.
            if relation.quantity().is_some()
                && relation.columns.iter().any(|column| column.name == ID)
            {
                return Err(Malformed::IdAndQuantity {
                    relation: relation.name.clone(),
                });
            }

            for column in &relation.columns {
                if let Some(to) = &column.references
                    && !relations.contains_key(to)
                {
                    return Err(Malformed::ReferencesNothing {
                        column: column.id.clone(),
                        to: to.clone(),
                    });
                }
            }
        }

        // **Every member declares the columns its family does.** That is what makes a family a
        // shape rather than only a set: a clause whose relation comes from an argument binds the
        // family's columns, and it can only do that if every member has them.
        for (family, members) in &families {
            let Some(shape) = relations.get(family) else {
                continue;
            };
            for member in members {
                let Some(member) = named.get(member) else {
                    continue;
                };
                let Some(declared) = relations.get(member) else {
                    continue;
                };
                for column in &shape.columns {
                    if !declared.columns.iter().any(|it| it.name == column.name) {
                        return Err(Malformed::UnlikeShape {
                            family: family.clone(),
                            member: member.to_string(),
                            column: column.name.clone(),
                        });
                    }
                }
            }
        }

        Ok(Schema {
            relations,
            by_id,
            families,
            limits,
        })
    }

    /// The relations belonging to `family`, or `None` where it is not a family.
    pub fn members(&self, family: &str) -> Option<&[String]> {
        self.families.get(family).map(Vec::as_slice)
    }

    pub fn relation(&self, name: &str) -> Option<&Relation> {
        self.relations.get(name)
    }

    pub fn names(&self) -> Vec<&str> {
        self.relations.keys().map(String::as_str).collect()
    }

    /// The relation a column id belongs to, and what it is called there.
    pub fn column(&self, id: &str) -> Option<(&str, &str)> {
        self.by_id
            .get(id)
            .map(|(relation, name)| (relation.as_str(), name.as_str()))
    }

    /// A row written back in its relation's declared column order.
    ///
    /// **Not alphabetically, which is what the notation would do on its own.** A `BTreeMap` holds
    /// the values by key and has no idea what order the relation wants them in; this is the only
    /// place that does.
    pub fn write(&self, row: &Row) -> String {
        let Some(relation) = self.relation(&row.relation) else {
            return crate::notation::write(row);
        };
        let mut out = String::from("{");
        out.push_str(&row.relation);
        for column in &relation.columns {
            if let Some(value) = row.value(&column.name) {
                out.push(' ');
                out.push_str(&column.name);
                out.push(':');
                out.push_str(value);
            }
        }
        out.push('}');
        out
    }

    /// Whether one row fits: a declared relation, and exactly its columns.
    ///
    /// **Exactly, rather than at least.** A row with a column nobody declared is as wrong as one
    /// missing a column, and both are the data saying something the structure does not allow.
    pub fn fits(&self, row: &Row) -> Result<&Relation, Malformed> {
        let Some(relation) = self.relation(&row.relation) else {
            return Err(Malformed::NoSuchRelation {
                relation: row.relation.clone(),
            });
        };
        let wanted: Vec<&str> = relation.columns.iter().map(|it| it.name.as_str()).collect();
        let given: Vec<&str> = row.values.keys().map(String::as_str).collect();
        let mut sorted = wanted.clone();
        sorted.sort_unstable();
        if sorted != given {
            return Err(Malformed::WrongColumns {
                relation: row.relation.clone(),
                wanted: wanted.join(" "),
                given: given.join(" "),
            });
        }
        Ok(relation)
    }
}

/// Every row fits its relation, its key is its own, and every reference points at a row.
pub fn check(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
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
            // **A reference to a family is a reference to its members.** A family has no rows, so
            // asking whether one of them carries this key would refuse everything; what the value
            // names is a relation, and the question is whether that relation belongs.
            if let Some(members) = schema.members(to) {
                if members.iter().any(|it| it == value) {
                    continue;
                }
                return Err(Malformed::NoSuchRow {
                    relation: row.relation.clone(),
                    column: column.name.clone(),
                    value: value.to_string(),
                    to: to.clone(),
                });
            }
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

    // **Last, because breaking a reference breaks this too.** Point a deposit's `where` at a key
    // nothing has and the extractors over it are suddenly over nothing - so a check that ran
    // first would answer *too many extractors* to a question about a dangling reference, and
    // `tests/mutation.rs` said exactly that. **The narrower fault is the one to report.**
    held_within_what_holds_it(schema, rows)?;
    nothing_crowds_a_place(schema, rows)?;
    Ok(())
}

/// No place consumes more of a supply than is provided there, each kind counting at its own rate.
///
/// **Sean, 2026-09-17**: *there are certain things I always want to see in tests because I need to
/// compute the tests in my head.* So the amounts are rows of the world, stated in a test's `given`
/// and read where the test is read - and a layer that would have kept them out of a scenario is a
/// layer hiding what the test is about. **Comprehension wins and the layer bends**, which is his
/// ruling rather than an inference.
///
/// ```text
/// {provides kind:territory what:berth} -> 6
/// {consumes kind:scout     what:berth} -> 1
/// {consumes kind:transport what:berth} -> 2
/// ```
///
/// **A provider either is the place or is in one.** A territory provides berths at itself; a store
/// would provide room at the territory it stands in, and a place's capacity is then the sum of what
/// is in it that provides - which is `spec/logistics.md`'s sentence, reached without another idea.
///
/// **The place is declared, not guessed.** `{supply ... per:territory}` says what a supply is
/// measured in, so nothing has to work out which relation every provider and consumer has in
/// common.
///
/// **No rule mentions any of it**, as with every limit here: a command that would overfill a place
/// leaves a world that does not fit, and every rule already refuses that.
fn nothing_crowds_a_place(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
    let supplies: Vec<&Row> = rows
        .rows()
        .iter()
        .filter(|it| it.relation == SUPPLY)
        .collect();
    if supplies.is_empty() {
        return Ok(());
    }
    let by_id: BTreeMap<&str, &str> = rows
        .rows()
        .iter()
        .filter(|it| it.relation == RELATION)
        .filter_map(|it| Some((it.value(ID)?, it.value(NAME)?)))
        .collect();

    for supply in supplies {
        let (Some(id), Some(named), Some(per)) =
            (supply.value(ID), supply.value(NAME), supply.value(PER))
        else {
            continue;
        };
        let Some(of_place) = by_id.get(per).map(|it| it.to_string()) else {
            continue;
        };

        // **How much each side is worth, per kind.** A kind named by neither is not in the
        // arithmetic at all, which is how everything that is not a vehicle stays out of it.
        let amounts = |relation: &str| -> BTreeMap<String, i64> {
            let mut found = BTreeMap::new();
            for row in rows.rows().iter().filter(|it| it.relation == relation) {
                if row.value(WHAT) != Some(id) {
                    continue;
                }
                let (Some(kind), Some(rate)) = (row.value(KIND), row.value(QUANTITY)) else {
                    continue;
                };
                let Some(kind) = by_id.get(kind) else {
                    continue;
                };
                found.insert(kind.to_string(), rate.parse().unwrap_or(0));
            }
            found
        };

        // **Rows of a kind, counted into the place each sits in.** A kind that *is* the place is
        // one of itself in itself; a kind that references the place is however many it says.
        let counted = |kinds: &BTreeMap<String, i64>| -> BTreeMap<String, i64> {
            let mut total: BTreeMap<String, i64> = BTreeMap::new();
            for (kind, rate) in kinds {
                let Some(declared) = schema.relation(kind) else {
                    continue;
                };
                if *kind == of_place {
                    for row in rows.rows().iter().filter(|it| it.relation == *kind) {
                        if let Some(at) = row.value(declared.identity()) {
                            *total.entry(at.to_string()).or_default() += rate;
                        }
                    }
                    continue;
                }
                let (Some(place), Some(quantity)) = (
                    declared
                        .columns
                        .iter()
                        .find(|it| it.references.as_deref() == Some(of_place.as_str())),
                    declared.quantity(),
                ) else {
                    continue;
                };
                for row in rows.rows().iter().filter(|it| it.relation == *kind) {
                    let (Some(at), Some(how_many)) = (row.value(&place.name), row.value(quantity))
                    else {
                        continue;
                    };
                    *total.entry(at.to_string()).or_default() +=
                        how_many.parse::<i64>().unwrap_or(0) * rate;
                }
            }
            total
        };

        let provided = counted(&amounts(PROVIDES));
        let consumed = counted(&amounts(CONSUMES));
        let providers = amounts(PROVIDES);

        for (place, taken) in consumed {
            let room = provided.get(&place).copied().unwrap_or(0);
            if taken <= room {
                continue;
            }
            // **The refusal names the providing that would have had to be there**, which is the
            // answer every limit here gives: not *this is too many* but *nothing provides this
            // much*.
            let of_kind = providers
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| of_place.clone());
            let by_name: BTreeMap<&str, &str> = by_id.iter().map(|(a, b)| (*b, *a)).collect();
            let mut wanted = BTreeMap::new();
            wanted.insert(
                KIND.to_string(),
                by_name.get(of_kind.as_str()).unwrap_or(&"").to_string(),
            );
            wanted.insert(WHAT.to_string(), id.to_string());
            wanted.insert(QUANTITY.to_string(), taken.to_string());
            return Err(Malformed::Crowded {
                supply: named.to_string(),
                place,
                wanted: schema.write(&Row {
                    relation: PROVIDES.to_string(),
                    values: wanted,
                }),
                room: room.to_string(),
            });
        }
    }
    Ok(())
}

/// No more of a held thing anywhere than there is room for it.
///
/// **This is a reference with a number on it.** An ordinary reference asks whether the row it
/// points at exists; this asks whether it exists *and has room*, and the two are the same question
/// where the room is one. **Both halves come out of the same comparison**, because a row at
/// quantity zero is never written - so a deposit that is full and a deposit that is not there at
/// all differ only in what the number is.
///
/// **No rule says any of this.** `build-extractor` adds an extractor and nothing else; every rule
/// already refuses the world it would leave when that world does not fit, so a rule written
/// tomorrow is bound by this without knowing it exists.
fn held_within_what_holds_it(schema: &Schema, rows: &Store) -> Result<(), Malformed> {
    for (held, by) in &schema.limits {
        let (Some(holder), Some(holds)) = (schema.relation(held), schema.relation(by)) else {
            return Err(Malformed::CannotLimit {
                held: held.clone(),
                by: by.clone(),
            });
        };
        let (Some(counted), Some(room_in)) = (holder.quantity(), holds.quantity()) else {
            return Err(Malformed::CannotLimit {
                held: held.clone(),
                by: by.clone(),
            });
        };
        // **The container's key must be part of the held thing's.** A deposit is keyed by
        // `(where, what)` and an extractor by `(where, what, working)`, and the extractors of one
        // deposit are every row that agrees on the deposit's columns - so a subset rather than
        // equality, and a limit between relations that share no key at all still says so.
        let key = holds.key();
        if !key.iter().all(|column| holder.key().contains(column)) {
            return Err(Malformed::CannotLimit {
                held: held.clone(),
                by: by.clone(),
            });
        }

        // **The held rows are summed over the container's key.** They were matched key for key
        // until an extractor gained a `working` column: a deposit holds an extractor whatever
        // state it is in, so one spent and one fresh are two rows of the same deposit. **The sum
        // is what stops two groups each fitting while together they do not.**
        let mut taken: BTreeMap<Vec<String>, i64> = BTreeMap::new();
        for row in rows.rows().iter().filter(|it| it.relation == *held) {
            let Some(values) = key
                .iter()
                .map(|column| row.value(column).map(str::to_string))
                .collect::<Option<Vec<String>>>()
            else {
                continue;
            };
            *taken.entry(values).or_default() += row
                .value(counted)
                .and_then(|it| it.parse::<i64>().ok())
                .unwrap_or(0);
        }

        for (values, how_many) in taken {
            let there = rows
                .rows()
                .iter()
                .filter(|it| it.relation == *by)
                .find(|it| {
                    key.iter()
                        .zip(&values)
                        .all(|(column, value)| it.value(column) == Some(value.as_str()))
                });
            let room: i64 = there
                .and_then(|it| it.value(room_in))
                .and_then(|it| it.parse().ok())
                .unwrap_or(0);
            if how_many <= room {
                continue;
            }
            // **The refusal names the row that would have had to be there**, which is what a test
            // can state and what a reader can act on: not *this is too many* but *there is no
            // deposit with room for this many*.
            let mut wanted = BTreeMap::new();
            for (column, value) in key.iter().zip(&values) {
                wanted.insert(column.to_string(), value.clone());
            }
            wanted.insert(room_in.to_string(), how_many.to_string());
            return Err(Malformed::Overfull {
                held: held.clone(),
                by: by.clone(),
                wanted: schema.write(&Row {
                    relation: by.clone(),
                    values: wanted,
                }),
                room: room.to_string(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation::read;

    /// **A relation whose columns are not in alphabetical order**, which the relations the data
    /// declares all happen to be - `tests/schema.rs` asserts that they are, so this exists for a
    /// reason a reader can check rather than one they have to take.
    const BACKWARDS: &str = "\
{relation name:pair}
{column id:pair.zero relation:pair seq:1 name:zero}
{column id:pair.also relation:pair seq:2 name:also}
";

    #[test]
    fn a_row_is_written_in_the_order_its_relation_declares() {
        let schema = Schema::of(&read(BACKWARDS).expect("a schema")).expect("read");
        let row = read("{pair also:2 zero:1}").expect("a row")[0].clone();

        assert_eq!(
            crate::notation::write(&row),
            "{pair also:2 zero:1}",
            "the notation on its own writes the keys in the order they sort"
        );
        assert_eq!(
            schema.write(&row),
            "{pair zero:1 also:2}",
            "and the schema writes them in the order the relation declares"
        );
    }

    /// Every way a schema can fail to be one, and the count so that none is untested.
    #[test]
    fn a_schema_that_is_not_one_says_what_about_it() {
        let refused = [
            (
                "{relation name:pair}\n{column id:a relation:other seq:1 name:a}\n",
                Malformed::ColumnOfNothing {
                    relation: "other".to_string(),
                    column: "a".to_string(),
                },
            ),
            (
                "{relation name:pair}\n",
                Malformed::NoColumns {
                    relation: "pair".to_string(),
                },
            ),
            (
                "{relation name:pair}\n{column id:a relation:pair seq:2 name:a}\n",
                Malformed::BadOrder {
                    relation: "pair".to_string(),
                    seq: vec!["2".to_string()],
                },
            ),
            (
                "{relation name:pair}\n{column id:a relation:pair seq:1 name:a}\n\
                 {reference column:a to:nowhere}\n",
                Malformed::ReferencesNothing {
                    column: "a".to_string(),
                    to: "nowhere".to_string(),
                },
            ),
            (
                "{relation name:pair}\n{column id:a relation:pair seq:1 name:a}\n\
                 {reference column:elsewhere to:pair}\n",
                Malformed::ReferenceOfNothing {
                    column: "elsewhere".to_string(),
                },
            ),
        ];
        for (stated, expected) in &refused {
            let why = Schema::of(&read(stated).expect(stated)).expect_err(stated);
            assert_eq!(&why, expected, "{stated}");
        }
        assert_eq!(refused.len(), 5, "five ways, and each is checked");
    }

    #[test]
    fn a_row_gives_exactly_the_columns_its_relation_declares() {
        let schema = Schema::of(&read(BACKWARDS).expect("a schema")).expect("read");
        schema
            .fits(&read("{pair zero:1 also:2}").expect("a row")[0])
            .expect("both columns, and no others");

        for wrong in [
            "{pair zero:1}",
            "{pair zero:1 also:2 spare:3}",
            "{other zero:1}",
        ] {
            let row = read(wrong).expect(wrong)[0].clone();
            schema.fits(&row).expect_err(wrong);
        }
    }
}
