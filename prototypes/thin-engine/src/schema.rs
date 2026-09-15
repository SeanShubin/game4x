//! What relations there are, what columns each has, and which columns point at another relation.
//!
//! **This module is the answer to *be clear about the data structure*.** The first test was typed
//! in relational notation - `residency (what fk thing, where fk territory)` - and everything here
//! is that made explicit enough for the engine to enforce.
//!
//! # It is read from the same rows as everything else
//!
//! `{relation ...}`, `{column ...}` and `{reference ...}` are rows in `data/before.4x` beside
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

const RELATION: &str = "relation";
const COLUMN: &str = "column";
const REFERENCE: &str = "reference";
const ID: &str = "id";
const QUANTITY: &str = "quantity";
const NAME: &str = "name";
const SEQ: &str = "seq";
const TO: &str = "to";

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

        for (of, mut columns) in numbered {
            columns.sort_by(|left, right| left.0.cmp(&right.0));
            let seq: Vec<String> = columns.iter().map(|it| it.0.clone()).collect();
            let wanted: Vec<String> = (1..=columns.len()).map(|it| it.to_string()).collect();
            if seq != wanted {
                return Err(Malformed::BadOrder { relation: of, seq });
            }
            relations.get_mut(&of).expect("declared above").columns =
                columns.into_iter().map(|it| it.1).collect();
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

        Ok(Schema { relations, by_id })
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
