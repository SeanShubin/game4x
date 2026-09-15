//! Rendering the foundation format in the user-facing one.
//!
//! **Sean, 2026-09-15**: *I need to add the constraint that name must be unique, and the user
//! friendly format is:*
//!
//! ```text
//! {territory id:1 name:territory-1}
//! {thing id:1 name:scout}
//! {adjacency id:1 from:territory-1 to:territory-2}
//! {residency id:1 what:scout where:territory-1}
//! ```
//!
//! **It is the same notation and the same rows.** Same relations, same columns, same `id`. Two
//! differences and no others:
//!
//! 1. **Every row has a `name`**, generated as `<relation>-<id>` where the relation has none.
//! 2. **Every reference is written as the name** rather than as the id.
//!
//! # Keeping `id` is what makes the round trip exact
//!
//! An earlier version of this rendering dropped the id where a row had a name, which meant
//! translating back had to mint one. **This carries every id**, so foundation to friendly to
//! foundation is the identity and nothing is invented on the way.
//!
//! # It is not part of the engine
//!
//! **Sean**: *I don\'t consider the translation between user friendly format and foundational
//! format part of the engine. The engine should only know about the foundational format. The user
//! friendly format is for the test harness and debugging.*
//!
//! So this lives in `tests/`, where naming a game noun is allowed and no constant of it reaches
//! `data/engine.4x`. **The translator can be as thick as it likes and the engine does not grow.**

use std::collections::BTreeMap;

use thin_engine::notation::Row;
use thin_engine::schema::Schema;

/// What each row is called, and what its columns point at.
pub struct Names {
    schema: Schema,
    names: BTreeMap<(String, String), String>,
    /// An `argument`\'s id, and the relation its value is of - the one reference the schema cannot
    /// state, because it follows the input\'s `of` rather than a `{reference ...}` row.
    argument_of: BTreeMap<String, String>,
}

impl Names {
    pub fn of(rows: &[Row]) -> Names {
        let schema = Schema::of(rows).expect("a schema");

        // **Which relations something points at.** A row needs a name so that a reference can
        // write one; a row nothing references needs none - which is why Sean's example names the
        // territories and the thing, and leaves the adjacencies and the residency without one.
        let referenced: std::collections::BTreeSet<String> = schema
            .names()
            .iter()
            .flat_map(|name| schema.relation(name).expect("declared").columns.iter())
            .filter_map(|column| column.references.clone())
            .collect();

        let mut names = BTreeMap::new();
        for row in rows {
            let Some(relation) = schema.relation(&row.relation) else {
                continue;
            };
            let Some(id) = row.value(relation.key()) else {
                continue;
            };
            // **The `name` the row already has, or one made from its relation and id.** Sean:
            // *I was thinking of having a generated name for the user friendly style, in this
            // case `territory-1`.*
            let name = row.value("name").map(str::to_string).or_else(|| {
                referenced
                    .contains(&row.relation)
                    .then(|| format!("{}-{id}", row.relation))
            });
            if let Some(name) = name {
                names.insert((row.relation.clone(), id.to_string()), name);
            }
        }

        let mut argument_of = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == "argument") {
            let Some(id) = row.value("id") else { continue };
            let Some(input) = row.value("input") else {
                continue;
            };
            let of = rows
                .iter()
                .find(|it| it.relation == "input" && it.value("id") == Some(input))
                .and_then(|it| it.value("of"))
                .and_then(|of| {
                    rows.iter()
                        .find(|it| it.relation == "relation" && it.value("id") == Some(of))
                })
                .and_then(|it| it.value("name"));
            if let Some(of) = of {
                argument_of.insert(id.to_string(), of.to_string());
            }
        }

        Names {
            schema,
            names,
            argument_of,
        }
    }

    /// What the row of `relation` with that key is called.
    ///
    /// **A value pointing at nothing keeps its id, marked**, so a rendering of data the engine
    /// would refuse still says something rather than panicking.
    pub fn name(&self, relation: &str, id: &str) -> String {
        self.names
            .get(&(relation.to_string(), id.to_string()))
            .cloned()
            .unwrap_or_else(|| format!("{relation}-{id}?"))
    }

    /// One row in the user-facing format.
    pub fn row(&self, row: &Row) -> String {
        let Some(relation) = self.schema.relation(&row.relation) else {
            return thin_engine::notation::write(row);
        };
        let key = relation.key();
        let id = row.value(key).unwrap_or_default();

        let mut out = format!("{{{} {key}:{id}", row.relation);
        let mut said_name = false;
        for column in &relation.columns {
            if column.name == key {
                continue;
            }
            let Some(value) = row.value(&column.name) else {
                continue;
            };
            let shown = match &column.references {
                Some(to) => self.name(to, value),
                None if row.relation == "argument" && column.name == "value" => {
                    match self.argument_of.get(id) {
                        Some(of) => self.name(of, value),
                        None => value.to_string(),
                    }
                }
                None => value.to_string(),
            };
            if column.name == "name" {
                said_name = true;
            }
            out.push_str(&format!(" {}:{}", column.name, shown));
        }
        // **A generated name, where something references this relation.** A row nothing points
        // at gets none: there is no reference for it to be written into, so a name would be a
        // value nothing reads - which is the thing this prototype is meant not to have.
        if !said_name && let Some(name) = self.names.get(&(row.relation.clone(), id.to_string())) {
            out.push_str(&format!(" name:{name}"));
        }
        out.push('}');
        out
    }

    /// Every row, in the order given.
    pub fn all(&self, rows: &[Row]) -> String {
        rows.iter()
            .map(|row| self.row(row))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
