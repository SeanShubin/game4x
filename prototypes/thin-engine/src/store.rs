//! Every fact about the world, as rows.
//!
//! **There is no `Territory`, no `Unit` and no `Game` here**, and that is the experiment rather
//! than a style. The main tree's engine has a struct per game noun and a method per rule; this
//! has rows and a pattern. If a thin engine can run the game from data, the shape it has to
//! reach is one where the engine names no noun at all.
//!
//! **No relation the data names appears anywhere in this crate's code**, which
//! `tests/isolation.rs` checks by reading the word list out of `data/` rather than remembering it.
//! `scout`, `territory` and `move` are in the data, in the tests and in comments like this one,
//! and in no line that runs.

use crate::notation::Row;

/// The rows the world is made of.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Store {
    rows: Vec<Row>,
}

impl Store {
    pub fn of(rows: Vec<Row>) -> Self {
        Store { rows }
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// Every row that matches, in the order they are held.
    ///
    /// **A caller that needs the row itself rather than whether there is one.** `holds` answers a
    /// question and this hands back the answer, which is what a clause reading a value out of the
    /// world needs.
    pub fn matching(&self, wanted: &Row) -> Vec<&Row> {
        self.rows
            .iter()
            .filter(|row| matches(row, wanted))
            .collect()
    }

    /// Whether any row matches, where matching is *the same relation and at least these values*.
    ///
    /// **A subset rather than the whole row**, so a fact can gain a value without every pattern
    /// that reads it having to name the new one. `{at thing:scout place:1}` is matched by
    /// `{at thing:scout}`.
    pub fn holds(&self, wanted: &Row) -> bool {
        self.rows.iter().any(|row| matches(row, wanted))
    }

    /// Add a row, unless it is already there.
    ///
    /// **A store is a set and not a list**, because *the scout is at 2* is either true or not and
    /// cannot be true twice. Said here because the underlying `Vec` would happily hold it twice.
    pub fn add(&mut self, row: Row) {
        if !self.rows.contains(&row) {
            self.rows.push(row);
        }
    }

    /// Remove every row that matches, and say how many went.
    pub fn remove(&mut self, wanted: &Row) -> usize {
        let before = self.rows.len();
        self.rows.retain(|row| !matches(row, wanted));
        before - self.rows.len()
    }
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

impl Store {
    /// Take what `wanted` names out of the store, and say what was taken.
    ///
    /// **A counted relation is arithmetic and an identified one is a set.** `{residency what:1
    /// where:1 quantity:1}` takes one scout from a territory that may hold five; `{adjacency
    /// from:1 to:2}` takes the row, because there is nothing there to count.
    ///
    /// **An entry is never zero** - `spec/console.md` - so a row taken down to nothing is removed
    /// rather than written as `-> 0`. **Taking more than there are is refused**, which is the same
    /// answer as taking from nothing: neither leaves a world the rule described.
    ///
    /// **`quantity` is handed in rather than looked up.** Which column counts is a fact about the
    /// schema, and a store holds rows - so the caller asks the schema and this does the arithmetic.
    pub fn take(&mut self, wanted: &Row, quantity: Option<&str>) -> Option<Row> {
        let Some(quantity) = quantity else {
            return (self.remove(wanted) != 0).then(|| wanted.clone());
        };
        let Some(taking) = wanted.value(quantity).and_then(|it| it.parse::<i64>().ok()) else {
            // **A pattern that names no quantity means the row**, which is what taking meant
            // before any relation counted, and what it still means for a clause that says nothing
            // about how many.
            return (self.remove(wanted) != 0).then(|| wanted.clone());
        };
        let description = description(wanted, quantity);
        let there = self
            .rows()
            .iter()
            .find(|row| describes(row, &description))
            .cloned()?;
        let held: i64 = there
            .value(quantity)
            .and_then(|it| it.parse().ok())
            .unwrap_or(0);
        if held < taking {
            return None;
        }
        self.remove(&there);
        if held > taking {
            let mut left = there;
            left.values
                .insert(quantity.to_string(), (held - taking).to_string());
            self.add(left);
        }
        // **What was taken, not what is left.** One scout of five leaving is one taken, and the
        // four that stayed are nobody's effect.
        let mut took = description;
        took.values.insert(quantity.to_string(), taking.to_string());
        Some(took)
    }

    /// Put `row` in, joining what is already there where the relation counts.
    ///
    /// **Two of a description are one entry**, so arriving where a scout stands makes two rather
    /// than a second row - which the key would refuse - or nothing at all, which is what an
    /// identical row meeting a set used to do.
    pub fn put(&mut self, row: Row, quantity: Option<&str>) {
        let Some(quantity) = quantity else {
            self.add(row);
            return;
        };
        let adding: i64 = row
            .value(quantity)
            .and_then(|it| it.parse().ok())
            .unwrap_or(0);
        let description = description(&row, quantity);
        let there = self
            .rows()
            .iter()
            .find(|it| describes(it, &description))
            .cloned();
        match there {
            None => self.add(row),
            Some(there) => {
                let held: i64 = there
                    .value(quantity)
                    .and_then(|it| it.parse().ok())
                    .unwrap_or(0);
                self.remove(&there);
                let mut joined = there;
                joined
                    .values
                    .insert(quantity.to_string(), (held + adding).to_string());
                self.add(joined);
            }
        }
    }
}

fn matches(row: &Row, wanted: &Row) -> bool {
    row.relation == wanted.relation
        && wanted
            .values
            .iter()
            .all(|(key, value)| row.value(key) == Some(value.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation::read;

    /// Matching is the same relation and at least these values, and both halves are checked.
    #[test]
    fn a_pattern_matches_on_what_it_names_and_not_on_what_it_does_not() {
        let store = Store::of(read("{at thing:scout place:1}").expect("a world"));
        for wanted in [
            "{at thing:scout}",
            "{at place:1}",
            "{at thing:scout place:1}",
        ] {
            let row = read(wanted).expect(wanted)[0].clone();
            assert!(store.holds(&row), "{wanted} names only what is there");
        }
        for wanted in ["{at thing:scout place:2}", "{near thing:scout}"] {
            let row = read(wanted).expect(wanted)[0].clone();
            assert!(!store.holds(&row), "{wanted} names something that is not");
        }
    }

    #[test]
    fn a_store_holds_a_fact_once() {
        let mut store = Store::default();
        let row = read("{at thing:scout place:1}").expect("a row")[0].clone();
        store.add(row.clone());
        store.add(row.clone());
        assert_eq!(store.rows().len(), 1, "the scout is at 1, not at 1 twice");
        assert_eq!(store.remove(&row), 1);
        assert!(store.rows().is_empty());
    }
}
