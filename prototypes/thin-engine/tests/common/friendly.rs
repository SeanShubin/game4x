//! Rendering the foundation style in a form a person can read.
//!
//! **Sean, 2026-09-15**: *I don't consider the translation between user friendly format and
//! foundational format part of the engine. The engine should only know about the foundational
//! format. The user friendly format is for the test harness and debugging.*
//!
//! So this lives in `tests/`, and **that is what keeps it free**: `src/` may name no noun the game
//! has and every constant in it is a word the data delegates, which `data/engine.4x` lists. Here
//! neither applies. **The translator can be as thick as it likes without the engine growing a
//! line.**
//!
//! # What a row is called
//!
//! **Three rules, tried in order**, and each exists because the one before it was not enough:
//!
//! 1. **The `name`, if it names one row.** `{thing id:1 name:scout}` is `scout`.
//! 2. **The name qualified by what the row points at**, shortest first, taking the first form
//!    that names one row. `{column id:44 relation:16 name:id}` is `residency.id`, and
//!    `{input id:1 rule:1 name:it of:16}` is `move.it` - qualified by its rule, not by its rule
//!    *and* the relation it is typed as.
//! 3. **`<relation>-<id>`**, for a row with no name at all. `{territory id:1}` is `territory-1`.
//!
//! **The second rule was added after reading the output of the first two.** Columns came out as a
//! mixture - `what` and `where` were unique so they kept their names, while `id` and `to` were not
//! and became `column-44` and `column-43`. Same kind of thing, rendered two ways, which is worse
//! than either.
//!
//! # This renders and does not parse
//!
//! Going back the other way is a separate question and a harder one, because **ids are arbitrary**:
//! friendly to foundation has to mint them, so it can only reproduce the original up to renaming.
//! Nothing here attempts it.

use std::collections::BTreeMap;

use thin_engine::notation::Row;
use thin_engine::schema::Schema;

/// What each row of each relation is called.
pub struct Names {
    schema: Schema,
    labels: BTreeMap<(String, String), String>,
    /// An `argument`'s id, and the relation its value is of.
    ///
    /// **The one reference the structure cannot state.** `argument.value` points at whatever the
    /// input's `of` says, which is data rather than schema - so no `{reference ...}` row can
    /// describe it and the renderer has to follow the input itself. **A translator may know this;
    /// the engine may not**, which is why it lives here.
    argument_of: BTreeMap<String, String>,
}

impl Names {
    pub fn of(rows: &[Row]) -> Names {
        let schema = Schema::of(rows).expect("a schema");

        // How many rows of a relation share each `name`, so that a name naming two rows is not
        // used to name either.
        let mut how_many: BTreeMap<(String, String), usize> = BTreeMap::new();
        for row in rows {
            if let Some(name) = row.value("name") {
                *how_many
                    .entry((row.relation.clone(), name.to_string()))
                    .or_default() += 1;
            }
        }

        // **Rule one and rule three**, which is every label that does not need another row
        // labelled first.
        let mut labels = BTreeMap::new();
        for row in rows {
            let Some(relation) = schema.relation(&row.relation) else {
                continue;
            };
            let Some(id) = row.value(relation.key()) else {
                continue;
            };
            let unique = row
                .value("name")
                .filter(|name| how_many.get(&(row.relation.clone(), name.to_string())) == Some(&1))
                .map(str::to_string);
            labels.insert(
                (row.relation.clone(), id.to_string()),
                unique.unwrap_or_else(|| format!("{}-{id}", row.relation)),
            );
        }

        // **Rule two, which needs the first pass finished**: a name that names several rows,
        // qualified by what the row points at. Taken only where the qualified form names one row,
        // so a relation it does not settle keeps `<relation>-<id>`.
        let settled = Names {
            schema: schema.clone(),
            labels: labels.clone(),
            argument_of: BTreeMap::new(),
        };
        // Every candidate label for a row, shortest first: the name qualified by the first
        // thing the row points at, then by the first two, and so on.
        let mut candidates: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
        let mut how_many_qualified: BTreeMap<(String, String), usize> = BTreeMap::new();
        for row in rows {
            let Some(relation) = schema.relation(&row.relation) else {
                continue;
            };
            let Some(id) = row.value(relation.key()) else {
                continue;
            };
            let Some(name) = row.value("name") else {
                continue;
            };
            // **Qualified whenever the relation points somewhere, not only when the bare name
            // is ambiguous.** Taking the bare name where it happened to be unique left `column`
            // rendering as `residency.id` beside a plain `what` - one kind of thing shown two
            // ways, which is what rule two exists to stop.
            let mut by: Vec<String> = Vec::new();
            for column in &relation.columns {
                if let Some(to) = &column.references
                    && let Some(value) = row.value(&column.name)
                {
                    by.push(settled.label(to, value));
                }
            }
            if by.is_empty() {
                continue;
            }
            // **Shortest first**, so a column is `residency.id` and an input is `move.it` rather
            // than `move.residency.it`. Joining every reference qualified both of them past
            // readability - an input points at its rule *and* at the relation it is typed as, and
            // only the first of those tells a reader which input it is.
            let mut so_far: Vec<String> = Vec::new();
            let mut shortest: Vec<String> = Vec::new();
            for one in by {
                so_far.push(one);
                let whole = format!("{}.{name}", so_far.join("."));
                *how_many_qualified
                    .entry((row.relation.clone(), whole.clone()))
                    .or_default() += 1;
                shortest.push(whole);
            }
            candidates.insert((row.relation.clone(), id.to_string()), shortest);
        }
        for (what, tried) in candidates {
            // The first candidate that names one row. A row none of them settles keeps
            // `<relation>-<id>`, which rule three already gave it.
            for whole in tried {
                if how_many_qualified.get(&(what.0.clone(), whole.clone())) == Some(&1) {
                    labels.insert(what, whole);
                    break;
                }
            }
        }

        // Each argument's value is of the relation its input is typed as.
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
            labels,
            argument_of,
        }
    }

    /// What the row of `relation` with that key is called.
    ///
    /// **A value pointing at nothing keeps its id**, so a rendering of malformed data still says
    /// something rather than panicking. The engine is what refuses such data; this only reads it.
    pub fn label(&self, relation: &str, id: &str) -> String {
        self.labels
            .get(&(relation.to_string(), id.to_string()))
            .cloned()
            .unwrap_or_else(|| format!("{relation}-{id}?"))
    }

    /// One row, with every reference replaced by what it points at.
    ///
    /// **The key and the name are folded into the label** rather than repeated, because they are
    /// what the label is made of. Everything else is shown as `column=value`.
    pub fn row(&self, row: &Row) -> String {
        let Some(relation) = self.schema.relation(&row.relation) else {
            return thin_engine::notation::write(row);
        };
        let key = relation.key();
        let id = row.value(key).unwrap_or_default();
        let mut out = self.label(&row.relation, id);

        for column in &relation.columns {
            if column.name == key {
                continue;
            }
            if column.name == "name" && !out.starts_with(&row.relation) {
                continue;
            }
            let Some(value) = row.value(&column.name) else {
                continue;
            };
            let shown = match &column.references {
                Some(to) => self.label(to, value),
                None if row.relation == "argument" && column.name == "value" => {
                    match self.argument_of.get(id) {
                        Some(of) => self.label(of, value),
                        None => value.to_string(),
                    }
                }
                None => value.to_string(),
            };
            out.push_str(&format!("  {}={}", column.name, shown));
        }
        out
    }

    /// Every row, grouped by relation, in the order the schema declares its columns.
    pub fn all(&self, rows: &[Row]) -> String {
        let mut by_relation: BTreeMap<&str, Vec<String>> = BTreeMap::new();
        for row in rows {
            by_relation
                .entry(row.relation.as_str())
                .or_default()
                .push(self.row(row));
        }
        let mut out = String::new();
        for (relation, mut lines) in by_relation {
            lines.sort();
            out.push_str(&format!("{relation}\n"));
            for line in lines {
                out.push_str(&format!("  {line}\n"));
            }
            out.push('\n');
        }
        out
    }
}
