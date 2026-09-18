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
    /// A `literal`'s id, and the relation its value is of - read from the column it binds.
    ///
    /// **The column says what the value is.** `{literal column:44 value:2}` binds `residency.what`,
    /// which references `thing`, so the `2` is a thing - and a reader should see `labor`. Without
    /// this the rules file states three category ids and names none of them.
    literal_of: BTreeMap<String, String>,
    /// A relation and a name, to the id of the row that carries it - the inverse of `names`, and
    /// what turns a friendly reference back into a foundation one.
    by_name: BTreeMap<(String, String), String>,
    /// Which relations declare a `name` column, so a generated name can be told from data.
    declares_name: std::collections::BTreeSet<String>,
    /// A rule's name and one of its input's names, to the relation that input is typed as.
    ///
    /// **This is what lets a command be a row of its own rule.** `{move what:scout …}` names no
    /// relation the schema declares, so nothing above can say what `scout` is - the rule's input
    /// says it, and this is that lookup.
    input_of: BTreeMap<(String, String), (String, String)>,
    /// An input's id, by the rule it belongs to and its own name.
    ///
    /// **A bare input name is read in the scope of a rule**, which is the one thing the flat
    /// `by_name` cannot do: two rules each have a `where`, and the clause says which is meant.
    input_id: BTreeMap<(String, String), String>,
    /// Which rule a clause belongs to, by the clause's id and by its generated name.
    rule_of_clause: BTreeMap<String, String>,
    /// A `part`'s id, and the name of the rule that part **invokes**.
    ///
    /// **Not the rule it belongs to, which is the other one.** An `{argument ...}` names an input,
    /// and the input it means is an input of the rule being called - so this follows `is` and not
    /// `of`. The two are the same shape and the wrong one would resolve silently to nothing.
    rule_of_part: BTreeMap<String, String>,
    /// The relations that are families, whose values are relations rather than rows of their own.
    families: std::collections::BTreeSet<String>,
}

/// Which rows of a test file are the game's: everything after a `{given}`, `{when}` or `{then}`.
///
/// **A merged test file spans two stores.** Its prologue is script rows and its sections are game
/// rows, and the two schemas number their relations independently - so one `Names` cannot read the
/// whole file and which one to use is a fact about where the row sits.
pub fn in_a_section(rows: &[Row]) -> Vec<bool> {
    let mut inside = false;
    rows.iter()
        .map(|row| {
            if matches!(row.relation.as_str(), "given" | "when" | "then") {
                inside = true;
                false
            } else {
                inside
            }
        })
        .collect()
}

/// Whether each row states part of a world, as against asserting something about one.
///
/// **A `{refused}` row is an assertion and not a world.** It names the row a rule needed and did
/// not find, which may be the ruleset's own - `{pool name:berth per:territory n:7}` says *there is
/// no berth allowance this big*, and no world has such a pool in it.
///
/// **Putting those in the store they are written against took a name away from everything.** Three
/// rows came to be called `berth` - the real allowance and two assertions - and nameable is all or
/// nothing per relation, so every reference to a pool started rendering as a bare id. The same
/// hazard is already written down one function along, about two rows named `scout`.
///
/// **Which `Names` renders a row is a different question**, and [`in_a_section`] still answers it:
/// a refused row names the game's relations and is written in the game's spelling. It is read by
/// that store without joining it.
pub fn states_a_world(rows: &[Row]) -> Vec<bool> {
    let mut inside = false;
    rows.iter()
        .map(|row| match row.relation.as_str() {
            "given" | "when" | "then" => {
                inside = true;
                false
            }
            "refused" => {
                inside = false;
                false
            }
            _ => inside,
        })
        .collect()
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
        // **An input's name is unique inside its rule, and that is enough to resolve it.** A
        // binding names its clause and a clause names its rule, so `input:where` on a binding of
        // `work` can only be `work`'s `where`. **Nothing about the model is ambiguous here** -
        // what was ambiguous was the uniqueness test below, which asks whether a name is unique
        // across every row of a relation and knows nothing about parents.
        //
        // **Sean, 2026-09-17**: *Wouldn't the invariant still be fine unless one rule took 2
        // wheres?* It would. The first version of this qualified every reference as `rule.name`,
        // on the strength of a limitation that was described as though it were the model.
        //
        // **The blast radius is what gives it away.** `move`'s inputs are `what`, `from` and `to`
        // and collide with nothing, and they fell back to ids too - because the test is
        // all-or-nothing per relation. So the names are scoped for the uniqueness test below and
        // kept bare everywhere else.
        let mut qualified: BTreeMap<String, String> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == "input") {
            let (Some(id), Some(name), Some(of_rule)) =
                (row.value("id"), row.value("name"), row.value("rule"))
            else {
                continue;
            };
            // **By id or by name**, for the same reason everything else here is: the friendly
            // source writes `rule:move` and the foundation writes `rule:1`.
            let rule = rows
                .iter()
                .find(|it| {
                    it.relation == "rule"
                        && (it.value("id") == Some(of_rule) || it.value("name") == Some(of_rule))
                })
                .and_then(|it| it.value("name"));
            if let Some(rule) = rule {
                qualified.insert(id.to_string(), format!("{rule}.{name}"));
            }
        }
        // **Scoped for the uniqueness test only.** Two rules each taking a `where` are two
        // distinct names once the rule is part of them, which is what the model already says.
        let scoped = |row: &Row| -> Option<String> {
            let name = row.value("name")?;
            if row.relation == "input" {
                return qualified.get(row.value("id")?).cloned();
            }
            Some(name.to_string())
        };

        let mut seen: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in rows {
            if let Some(name) = scoped(row) {
                seen.entry(row.relation.clone()).or_default().push(name);
            }
        }

        // **Which rule each clause belongs to, by the clause's id and by its generated name.** A
        // friendly binding says `clause:clause-9` and a foundation one says `clause:9`, and this
        // turns either into `work` - the scope a bare input name is read in.
        let mut rule_of_clause: BTreeMap<String, String> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == "clause") {
            let (Some(id), Some(of_rule)) = (row.value("id"), row.value("rule")) else {
                continue;
            };
            let rule = rows
                .iter()
                .find(|it| {
                    it.relation == "rule"
                        && (it.value("id") == Some(of_rule) || it.value("name") == Some(of_rule))
                })
                .and_then(|it| it.value("name"));
            if let Some(rule) = rule {
                rule_of_clause.insert(id.to_string(), rule.to_string());
                rule_of_clause.insert(format!("clause-{id}"), rule.to_string());
            }
        }

        let mut rule_of_part: BTreeMap<String, String> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == "part") {
            let (Some(id), Some(is)) = (row.value("id"), row.value("is")) else {
                continue;
            };
            let rule = rows
                .iter()
                .find(|it| {
                    it.relation == "rule"
                        && (it.value("id") == Some(is) || it.value("name") == Some(is))
                })
                .and_then(|it| it.value("name"));
            if let Some(rule) = rule {
                rule_of_part.insert(id.to_string(), rule.to_string());
                rule_of_part.insert(format!("part-{id}"), rule.to_string());
            }
        }

        // **An input's id, by its rule and its own name.** The flat `by_name` cannot hold these:
        // `("input", "where")` names two different inputs, and which one it means is the question
        // the clause answers.
        let mut input_id: BTreeMap<(String, String), String> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == "input") {
            let (Some(id), Some(name)) = (row.value("id"), row.value("name")) else {
                continue;
            };
            if let Some(whole) = qualified.get(id)
                && let Some((rule, _)) = whole.split_once('.')
            {
                input_id.insert((rule.to_string(), name.to_string()), id.to_string());
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
            // **An input's name is unique inside its rule and nowhere else.** `what` is an input
            // of four rules here, so an argument that looked one up by name alone would take
            // whichever was written first - and the type it read off it would be that rule's.
            // **The part says which rule**, through the rule it invokes.
            let called = row.value("part").and_then(|part| rule_of_part.get(part));
            let of = rows
                .iter()
                // **By id or by name**, for the same reason the relation lookup below is: in the
                // friendly source an argument names its input `what`, and in the foundation it
                // names it `2`.
                .find(|it| {
                    it.relation == "input"
                        && (it.value("id") == Some(input) || it.value("name") == Some(input))
                        && match (called, it.value("rule")) {
                            (Some(called), Some(of_rule)) => rows.iter().any(|rule| {
                                rule.relation == "rule"
                                    && rule.value("name") == Some(called.as_str())
                                    && (rule.value("id") == Some(of_rule)
                                        || rule.value("name") == Some(of_rule))
                            }),
                            _ => false,
                        }
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

        // **A rule's inputs, by the rule's name and the input's own.** Read *by id or by name* for
        // the same reason everything else here is: the friendly source writes `rule:move` and the
        // foundation writes `rule:1`, and one translator reads whichever it is handed.
        let mut input_of = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == "input") {
            let Some(of_rule) = row.value("rule") else {
                continue;
            };
            let named = rows
                .iter()
                .find(|it| {
                    it.relation == "rule"
                        && (it.value("id") == Some(of_rule) || it.value("name") == Some(of_rule))
                })
                .and_then(|it| it.value("name"));
            let of = row.value("of").and_then(|of| {
                rows.iter()
                    .find(|it| {
                        it.relation == "relation"
                            && (it.value("id") == Some(of) || it.value("name") == Some(of))
                    })
                    .and_then(|it| it.value("name"))
            });
            if let (Some(named), Some(name), Some(of)) = (named, row.value("name"), of) {
                input_of.insert(
                    (named.to_string(), name.to_string()),
                    (
                        of.to_string(),
                        row.value("seq").unwrap_or_default().to_string(),
                    ),
                );
            }
        }

        // **What each column points at, by the column's id.** The literal rows below are the only
        // place a value's type comes from the column rather than from an input.
        let mut points_at: BTreeMap<String, String> = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == "reference") {
            let Some(column) = row.value("column") else {
                continue;
            };
            let Some(to) = row.value("to") else { continue };
            let named = rows
                .iter()
                .find(|it| {
                    it.relation == "relation"
                        && (it.value("id") == Some(to) || it.value("name") == Some(to))
                })
                .and_then(|it| it.value("name"))
                .unwrap_or(to);
            points_at.insert(column.to_string(), named.to_string());
        }
        let mut literal_of = BTreeMap::new();
        for row in rows.iter().filter(|row| row.relation == "literal") {
            if let (Some(id), Some(column)) = (row.value("id"), row.value("column"))
                && let Some(of) = points_at.get(column)
            {
                literal_of.insert(id.to_string(), of.clone());
            }
        }

        // **A family's values are relations.** `{move what:scout ...}` carries a relation's id
        // because a kind is a relation now, so a reference whose target is a family is resolved
        // against `relation` - the family itself has no rows to be named by.
        let mut families: std::collections::BTreeSet<String> = Default::default();
        for row in rows.iter().filter(|row| row.relation == "family") {
            let of = row.value("relation").unwrap_or_default();
            if let Some(row) = rows.iter().find(|it| {
                it.relation == "relation"
                    && (it.value("id") == Some(of) || it.value("name") == Some(of))
            }) && let Some(name) = row.value("name")
            {
                families.insert(name.to_string());
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
            literal_of,
            by_name,
            declares_name,
            input_of,
            input_id,
            rule_of_clause,
            rule_of_part,
            families,
        }
    }

    /// What the row of `relation` with that key is called.
    ///
    /// **A value pointing at nothing keeps its id, marked**, so a rendering of data the engine
    /// would refuse still says something rather than panicking.
    /// Where a name for a value of this relation is to be looked for.
    ///
    /// **A family has no rows**, so a value typed by one names a relation and is looked up there.
    fn under(&self, relation: &str) -> String {
        if self.families.contains(relation) {
            return "relation".to_string();
        }
        relation.to_string()
    }

    pub fn name(&self, relation: &str, id: &str) -> String {
        let relation = &self.under(relation);
        self.names
            .get(&(relation.to_string(), id.to_string()))
            .cloned()
            .unwrap_or_else(|| id.to_string())
    }

    /// One row in the user-facing format.
    pub fn row(&self, row: &Row) -> String {
        let Some(relation) = self.schema.relation(&row.relation) else {
            // **A command is a row of its own rule**, so its relation is a rule's name and each
            // value is named for one of that rule's inputs. A row that is neither - a `{given}`
            // marker, say - carries nothing to rename and is written as it is.
            // **Written in the rule's input order**, which is the order a player would say it in
            // and not the alphabetical one a row of no relation would otherwise get.
            let mut shown: Vec<(String, String, String)> = row
                .values
                .iter()
                .map(
                    |(key, value)| match self.input_of.get(&(row.relation.clone(), key.clone())) {
                        Some((of, seq)) => (seq.clone(), key.clone(), self.name(of, value)),
                        None => (String::new(), key.clone(), value.clone()),
                    },
                )
                .collect();
            shown.sort();
            let body = shown
                .iter()
                .map(|(_, key, value)| format!(" {key}:{value}"))
                .collect::<Vec<String>>()
                .join("");
            return format!("{{{}{body}}}", row.relation);
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
                None if row.relation == "literal" && column.name == "value" => {
                    match self.literal_of.get(id) {
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
            // **The same rule-named row, read the other way.**
            let resolved: BTreeMap<String, String> = row
                .values
                .iter()
                .map(|(key, value)| {
                    let back = match self.input_of.get(&(row.relation.clone(), key.clone())) {
                        Some((of, _)) => self
                            .by_name
                            .get(&(self.under(of), value.clone()))
                            .cloned()
                            .unwrap_or_else(|| value.clone()),
                        None => value.clone(),
                    };
                    (key.clone(), back)
                })
                .collect();
            return Ok(Row {
                relation: row.relation.clone(),
                values: resolved,
            });
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
                    .or_else(|| {
                        (row.relation == "literal" && column == "value")
                            .then(|| self.literal_of.get(&id).cloned())
                            .flatten()
                    })
            });
            let resolved = match points_at {
                // **An input is named inside its rule**, so the clause this row names is what
                // says which rule to look in. A value that is already an id finds nothing here
                // and is left alone, exactly as every other reference is.
                Some(to) if to == "input" => row
                    .value("clause")
                    .and_then(|clause| self.rule_of_clause.get(clause))
                    // **Or the part, which names the rule being called.** A binding is scoped by
                    // its clause and an argument by its part, and those are the only two rows that
                    // point at an input.
                    .or_else(|| {
                        row.value("part")
                            .and_then(|part| self.rule_of_part.get(part))
                    })
                    .and_then(|rule| self.input_id.get(&(rule.clone(), value.clone())))
                    .cloned()
                    .unwrap_or_else(|| value.clone()),
                Some(to) => self
                    .by_name
                    .get(&(self.under(&to), value.clone()))
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
