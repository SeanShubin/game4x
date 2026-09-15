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
//! **Sean**: *I don't consider the translation between user friendly format and foundational
//! format part of the engine. The engine should only know about the foundational format. The user
//! friendly format is for the test harness and debugging.*
//!
//! So this lives in `tests/`, where naming a game noun is allowed and no constant of it reaches
//! `data/engine.4x`. **The translator can be as thick as it likes and the engine does not grow.**

use std::collections::BTreeMap;

use thin_engine::notation::{Row, read};
use thin_engine::schema::Schema;

/// Read friendly text, folding `-> n` back into the relation's quantity column.
///
/// **The arrow is the friendly format's and not the notation's.** `src/notation.rs` reads one
/// `{…}` per line and does not learn one - Sean, 2026-09-15: *the foundation is the logical model,
/// friendly is a bridge from the user to the logical model*, and the arrow is the bridge's.
///
/// **It is what `scenario/expected/play.4x` already writes**, 114 times, under its own header: *a
/// line reads `{description} -> quantity`, and the description is the kind and every trait of that
/// thing.* So this is the game's notation rather than one invented for this prototype.
pub fn fold(text: &str, schema: &Schema) -> Result<Vec<Row>, String> {
    let mut out = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (shape, counted) = match line.split_once("->") {
            Some((left, right)) => (left.trim(), Some(right.trim().to_string())),
            None => (line, None),
        };
        let mut row = read(shape)
            .map_err(|why| format!("line {}: {why}", number + 1))?
            .pop()
            .ok_or_else(|| format!("line {}: no row", number + 1))?;
        if let Some(counted) = counted {
            let quantity = schema
                .relation(&row.relation)
                .and_then(|it| it.quantity())
                .map(str::to_string)
                .ok_or_else(|| {
                    format!(
                        "line {}: `{}` has no quantity, so `->` says nothing about it",
                        number + 1,
                        row.relation
                    )
                })?;
            row.values.insert(quantity, counted);
        }
        out.push(row);
    }
    Ok(out)
}

/// What each row is called, and what its columns point at.
pub struct Names {
    schema: Schema,
    names: BTreeMap<(String, String), String>,
    /// An `argument`'s id, and the relation its value is of - the one reference the schema cannot
    /// state, because it follows the input's `of` rather than a `{reference ...}` row.
    argument_of: BTreeMap<String, String>,
    /// A relation and a name, to the id of the row that carries it - the inverse of `names`, and
    /// what turns a friendly reference back into a foundation one.
    by_name: BTreeMap<(String, String), String>,
    /// Which relations declare a `name` column, so a generated name can be told from data.
    declares_name: std::collections::BTreeSet<String>,
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

        // **Whether a relation's names are all its own.** `column.name` is not a name for the
        // row - it is the token a row is keyed by - so sixteen columns are called `id`. Sean,
        // 2026-09-15, choosing this over giving `column` a second column: *binding and column are
        // machinery.*
        //
        // **All or nothing, per relation.** Some column names happen to be unique - `what`,
        // `where` - and taking those while falling back for the rest rendered one kind of thing
        // two ways: `column:what` beside `column:44`.
        let mut seen: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in rows {
            if let Some(name) = row.value("name") {
                seen.entry(row.relation.clone())
                    .or_default()
                    .push(name.to_string());
            }
        }
        let mut nameable: std::collections::BTreeSet<String> = Default::default();
        for (relation, mut all) in seen {
            let how_many = all.len();
            all.sort();
            all.dedup();
            if all.len() == how_many {
                nameable.insert(relation);
            }
        }

        // Which relations declare a `name` column at all. **One that does and whose names
        // collide gets no generated name either** - the row's `name` slot is taken by the token,
        // so a generated name would appear nowhere a reader could find it, and a reference to it
        // would be unresolvable. Those references stay ids.
        let declares_name: std::collections::BTreeSet<String> = schema
            .names()
            .iter()
            .filter(|name| {
                schema
                    .relation(name)
                    .expect("declared")
                    .columns
                    .iter()
                    .any(|column| column.name == "name")
            })
            .map(|name| name.to_string())
            .collect();

        let mut names = BTreeMap::new();
        for row in rows {
            let Some(relation) = schema.relation(&row.relation) else {
                continue;
            };
            let Some(id) = row.value(relation.identity()) else {
                continue;
            };
            // **The `name` the row already has, or one made from its relation and id.** Sean:
            // *I was thinking of having a generated name for the user friendly style, in this
            // case `territory-1`.*
            let name = if declares_name.contains(&row.relation) {
                nameable
                    .contains(&row.relation)
                    .then(|| row.value("name").unwrap_or_default().to_string())
            } else {
                referenced
                    .contains(&row.relation)
                    .then(|| format!("{}-{id}", row.relation))
            };
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
                // **By id or by name**, for the same reason the relation lookup below is: in the
                // friendly source an argument names its input `what`, and in the foundation it
                // names it `2`.
                .find(|it| {
                    it.relation == "input"
                        && (it.value("id") == Some(input) || it.value("name") == Some(input))
                })
                .and_then(|it| it.value("of"))
                // **By id or by name.** In the foundation `of` is a relation's id; in the
                // friendly source it is already its name. This is read from whichever it is
                // handed, which is what lets the same translator work in both directions.
                .and_then(|of| {
                    rows.iter().find(|it| {
                        it.relation == "relation"
                            && (it.value("id") == Some(of) || it.value("name") == Some(of))
                    })
                })
                .and_then(|it| it.value("name"));
            if let Some(of) = of {
                argument_of.insert(id.to_string(), of.to_string());
            }
        }

        let by_name = names
            .iter()
            .map(|((relation, id), name)| ((relation.clone(), name.clone()), id.clone()))
            .collect();

        Names {
            schema,
            names,
            argument_of,
            by_name,
            declares_name,
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
            .unwrap_or_else(|| id.to_string())
    }

    /// One row in the user-facing format.
    pub fn row(&self, row: &Row) -> String {
        let Some(relation) = self.schema.relation(&row.relation) else {
            return thin_engine::notation::write(row);
        };
        // **A counted relation has no key column to lead with**, and an identified one's key is
        // its first column - which is where writing it first and then skipping it in the loop
        // put it anyway. **So leading with the key was declared order written twice**, and
        // dropping it is the whole of what a compound key cost this renderer.
        let quantity = relation.quantity();
        let id = row.value(relation.identity()).unwrap_or_default();

        let mut out = format!("{{{}", row.relation);
        let mut said_name = false;
        for column in &relation.columns {
            if Some(column.name.as_str()) == quantity {
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
        if let Some(quantity) = quantity {
            out.push_str(&format!(" -> {}", row.value(quantity).unwrap_or_default()));
        }
        out
    }

    /// One friendly line, arrow and all, as a row.
    pub fn parse(&self, line: &str) -> Result<Row, String> {
        fold(line, &self.schema)?
            .pop()
            .ok_or_else(|| format!("`{line}` is no row"))
    }

    /// One friendly row turned back into its foundation form.
    ///
    /// **This is what authoring in the friendly format needs.** Sean, 2026-09-15: *I expect to be
    /// authoring tests in the friendly format and only debugging/vetting in the foundation
    /// format.*
    ///
    /// **Nothing is minted.** A friendly row carries its own `id`, so the only work is turning
    /// each reference from a name back into an id, and dropping the `name` where the relation
    /// does not declare one. **A value that is not a name is left alone**, which is what lets a
    /// reference to a row with no name stay an id.
    ///
    /// # It refuses a name it would otherwise drop
    ///
    /// **`{territory id:1 name:home}` is an error rather than `{territory id:1}`.** `territory`
    /// declares no `name`, so the foundation has nowhere to put one - and silently dropping it
    /// would lose an author's work in the format they author in. **The refusal is the whole of
    /// what makes the conversion reliable**; naming a territory needs somewhere in the foundation
    /// to keep it, which is a schema decision rather than a translator one.
    pub fn foundation(&self, row: &Row) -> Result<Row, String> {
        let Some(relation) = self.schema.relation(&row.relation) else {
            return Ok(row.clone());
        };
        let id = row
            .value(relation.identity())
            .unwrap_or_default()
            .to_string();

        let mut values = BTreeMap::new();
        for (column, value) in &row.values {
            let declared = relation.columns.iter().find(|it| it.name == *column);
            let points_at = declared.and_then(|it| it.references.clone()).or_else(|| {
                (row.relation == "argument" && column == "value")
                    .then(|| self.argument_of.get(&id).cloned())
                    .flatten()
            });
            let resolved = match points_at {
                Some(to) => self
                    .by_name
                    .get(&(to, value.clone()))
                    .cloned()
                    .unwrap_or_else(|| value.clone()),
                None => value.clone(),
            };
            values.insert(column.clone(), resolved);
        }

        // **A generated `name` goes**, because the relation never had one. A relation that does
        // declare `name` keeps it: there it is data.
        if !self.declares_name.contains(&row.relation)
            && let Some(given) = values.remove("name")
        {
            let generated = format!("{}-{id}", row.relation);
            if given != generated {
                return Err(format!(
                    "`{}` is named `{given}`, and `{}` has nowhere to keep a name -                      the generated one is `{generated}`",
                    shown(row),
                    row.relation
                ));
            }
        }
        Ok(Row {
            relation: row.relation.clone(),
            values,
        })
    }

    /// Every row, in the order given.
    pub fn all(&self, rows: &[Row]) -> String {
        rows.iter()
            .map(|row| self.row(row))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

/// A row as the notation writes it, for a message about it.
pub fn shown(row: &Row) -> String {
    thin_engine::notation::write(row)
}
