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

use std::collections::BTreeMap;

use crate::notation::Row;

/// A pattern over rows, with `$name` standing for something a command bound.
///
/// **A pattern is a row whose values may be holes**, which is why it is the same type. The
/// notation does not distinguish them and neither does this: `{at thing:$it place:$from}` is a
/// row that happens to have two values beginning `$`.
pub type Pattern = Row;

/// What a pattern could not be turned into.
#[derive(Debug, PartialEq, Eq)]
pub enum Unbound {
    /// A `$name` the command did not give a value for.
    ///
    /// **Refused rather than matched against anything**, which is the thin choice and the one
    /// that keeps the next concept visible: a hole that matches anything is a *search*, and a
    /// search has to answer *which one when several match*. Nothing here needs one yet.
    Hole { key: String, name: String },
}

impl std::fmt::Display for Unbound {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unbound::Hole { key, name } => {
                write!(out, "`{key}` wants `${name}` and nothing bound it")
            }
        }
    }
}

/// A pattern with every `$name` replaced by what the bindings give it.
pub fn fill(pattern: &Pattern, bindings: &BTreeMap<String, String>) -> Result<Row, Unbound> {
    let mut values = BTreeMap::new();
    for (key, value) in &pattern.values {
        let filled = match value.strip_prefix('$') {
            None => value.clone(),
            Some(name) => bindings.get(name).cloned().ok_or_else(|| Unbound::Hole {
                key: key.clone(),
                name: name.to_string(),
            })?,
        };
        values.insert(key.clone(), filled);
    }
    Ok(Row {
        relation: pattern.relation.clone(),
        values,
    })
}

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

/// Every way `pattern` can be made true by one row, extending what is already bound.
///
/// **This is the other direction, and it is the concept the first three did without.** [`fill`]
/// puts a bound value into a pattern and refuses a hole nothing bound; this takes a hole nothing
/// bound and reads a value *out of* a row. A rule fired by a command never needs it - the command
/// binds everything - and a rule fired by a turn has no command, so it needs nothing else.
///
/// **Several rows may match and all of them are returned.** That is the question `README.md` said
/// a search would have to answer - *which one, when several match* - and the answer here is *all
/// of them, and the caller fires once per solution*. It is an answer rather than an evasion
/// because a turn wants every settlement and not one of them.
///
/// **A value already bound is a filter and not a rebinding**, which is what joins the clauses of
/// one rule together: `$where` bound by the first clause has to be the same `$where` in the
/// second.
pub fn solutions(
    store: &Store,
    pattern: &Pattern,
    bindings: &BTreeMap<String, String>,
) -> Vec<BTreeMap<String, String>> {
    let mut ways = Vec::new();
    for row in &store.rows {
        if row.relation != pattern.relation {
            continue;
        }
        let mut extended = bindings.clone();
        let mut fits = true;
        for (key, value) in &pattern.values {
            let Some(held) = row.value(key) else {
                fits = false;
                break;
            };
            match value.strip_prefix('$') {
                None => fits = value == held,
                Some(name) => match extended.get(name) {
                    Some(already) => fits = already == held,
                    None => {
                        extended.insert(name.to_string(), held.to_string());
                    }
                },
            }
            if !fits {
                break;
            }
        }
        if fits && !ways.contains(&extended) {
            ways.push(extended);
        }
    }
    ways
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

    fn bound(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }

    #[test]
    fn a_hole_takes_what_the_binding_gives_it() {
        let pattern = read("{at thing:$it place:$from}").expect("a pattern")[0].clone();
        let filled = fill(&pattern, &bound(&[("it", "scout"), ("from", "1")])).expect("filled");
        assert_eq!(crate::notation::write(&filled), "{at place:1 thing:scout}");
    }

    #[test]
    fn a_hole_nothing_bound_is_refused_by_name() {
        let pattern = read("{at thing:$it place:$from}").expect("a pattern")[0].clone();
        let why = fill(&pattern, &bound(&[("it", "scout")])).expect_err("`from` is unbound");
        assert_eq!(
            why,
            Unbound::Hole {
                key: "place".to_string(),
                name: "from".to_string()
            }
        );
    }

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
