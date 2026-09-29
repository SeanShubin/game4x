//! The play commands, read from the rules rather than written here.
//!
//! # Why these are not in `grammar.rs` with the others
//!
//! **`releases/rules-become-data.md`, `D-1`**: *I change a recipe by editing a data file, with no
//! Rust edited, and the game fires the changed rule.* A console with `build-yard` written into its
//! grammar cannot do that - the rule would exist and nothing could say it.
//!
//! **So the design phase has a written grammar and play has a read one.** Before `{start}` a
//! player states a world, and what may be stated is fixed by what a world is; after it a player
//! fires a rule, and what may be fired is whatever `spec/data/rules.4x` declares. **A rule added
//! tomorrow is a command tomorrow.**
//!
//! # The eleven verbs this replaces, and what happened to them
//!
//! **`spec/console.md` never specified them.** It states the design phase, `run`, `show`, `help`,
//! `history` and `start`, and stops - so the play verbs were the code lane's own, and four of them
//! had drifted from the rule they fire: `build-store` against `build-bin`, `create-labor` against
//! `toil`, `mine-energy` against `gather`, `found-by-land` and `deploy-ark` both against `deploy`.
//!
//! **One of those was not a rename.** `found-by-land` was *send a pioneer onto adjacent unclaimed
//! ground and found it there* - two rules in one command, and the data has them as two. It is
//! `{move ...}` and then `{deploy ...}`, which is what `scenario/main.4x` already types.
//!
//! # What a rule's inputs become
//!
//! **An input is typed as a relation and that decides how it is written.** `of:place` is a place
//! and a place is a number; anything else is a kind and a kind is a word. `{gather where:1}` and
//! `{move what:ark from:4 to:2}` are both what the foundation tests already hold.

use std::collections::BTreeMap;

use command_language::{Form, Kind, Term};
use game_model::notation::Row;

const RULE: &str = "rule";
const INPUT: &str = "input";
const NAME: &str = "name";
const ID: &str = "id";
const SEQ: &str = "seq";
const OF: &str = "of";
const PART: &str = "part";
const IS: &str = "is";

/// One rule a player may fire, and what it asks for.
pub struct Playable {
    pub name: String,
    /// Its inputs in the order they are declared, each with the relation it is typed as.
    pub inputs: Vec<(String, String)>,
}

/// Every rule a player may choose, read from the foundation the engine carries.
///
/// **A rule that is only ever a part is not one of them.** `upkeep` and `breed` happen because
/// `end-turn` runs them, and `engine::offered` ranges over exactly the roots for the same reason -
/// one fact about what a player may choose, read in two places rather than written twice.
pub fn playable() -> Vec<Playable> {
    let rows = game_model::foundation::rows();
    of(&rows)
}

/// The same, over rows given rather than read - so a test can hand it a small ruleset.
pub fn of(rows: &[Row]) -> Vec<Playable> {
    let value = |row: &Row, key: &str| row.value(key).unwrap_or_default().to_string();

    let a_part: Vec<String> = rows
        .iter()
        .filter(|it| it.relation == PART)
        .map(|it| value(it, IS))
        .collect();

    let mut named: Vec<(String, String)> = rows
        .iter()
        .filter(|it| it.relation == RULE)
        .map(|it| (value(it, ID), value(it, NAME)))
        .filter(|(id, _)| !a_part.contains(id))
        .collect();
    named.sort_by_key(|(id, _)| id.parse::<u64>().unwrap_or(u64::MAX));

    // **The foundation names nothing twice, so an input says which rule by its id and which
    // type by a relation's id.** `{input rule:1 ... of:26}` is `move`'s `what`, typed as `unit` -
    // and resolving both is what makes this read the engine's own form rather than the friendly
    // one a person writes.
    let relation_named: BTreeMap<String, String> = rows
        .iter()
        .filter(|it| it.relation == "relation")
        .map(|it| (value(it, ID), value(it, NAME)))
        .collect();

    named
        .into_iter()
        .map(|(id, name)| {
            let mut inputs: Vec<(u64, String, String)> = rows
                .iter()
                .filter(|it| it.relation == INPUT && value(it, RULE) == id)
                .map(|it| {
                    let of = value(it, OF);
                    (
                        value(it, SEQ).parse::<u64>().unwrap_or(u64::MAX),
                        value(it, NAME),
                        relation_named.get(&of).cloned().unwrap_or(of),
                    )
                })
                .collect();
            inputs.sort_by_key(|(seq, _, _)| *seq);
            Playable {
                name,
                inputs: inputs.into_iter().map(|(_, name, of)| (name, of)).collect(),
            }
        })
        .collect()
}

/// How an input of this type is written.
///
/// **A place is a number and everything else is a word.** `{gather where:1}` names the place by
/// its id; `{move what:ark ...}` names a kind. That is the whole of the rule, and it is here
/// rather than in a list of input names so that a new input of a known type needs nothing.
fn written_as(of: &str) -> Kind {
    match of {
        "place" => Kind::Number,
        _ => Kind::Name,
    }
}

/// Every rule as a command the grammar can read.
pub fn forms() -> Vec<Form> {
    playable().into_iter().map(form_of).collect()
}

fn form_of(rule: Playable) -> Form {
    // **Leaked so the form can hold it for the program's life.** `Form` takes `&'static str`
    // because the written grammar is all literals; these come out of the foundation, which is
    // itself `include_str!`ed and so lives as long. **One leak per rule at startup**, and a
    // `Session` made twice makes them twice - which is why `Grammar` is built once and shared.
    let name: &'static str = Box::leak(rule.name.clone().into_boxed_str());
    let mut terms = vec![Term::Keyword(name)];
    for (input, of) in &rule.inputs {
        let input: &'static str = Box::leak(input.clone().into_boxed_str());
        terms.push(Term::required(input, written_as(of)));
    }
    // **`repeat` is the console's and not the rule's** - `P-323`: a count of firings rather than
    // an argument, so every rule takes it and none declares it.
    terms.push(Term::optional("repeat", Kind::Number));
    Form::new(name, terms, Box::leak(said(&rule).into_boxed_str()))
}

/// What a rule's command says of itself in `help`.
fn said(rule: &Playable) -> String {
    if rule.inputs.is_empty() {
        format!("fire the rule `{}`", rule.name)
    } else {
        let asks: Vec<String> = rule
            .inputs
            .iter()
            .map(|(name, of)| format!("{name} (a {of})"))
            .collect();
        format!("fire the rule `{}` on {}", rule.name, asks.join(" and "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **What a player may fire is what the data says, and nothing here lists it.**
    #[test]
    fn every_rule_that_is_not_a_part_is_a_command() {
        let found = playable();
        let names: Vec<&str> = found.iter().map(|it| it.name.as_str()).collect();

        // **A count over nothing is the same failure with the sign flipped.** An empty foundation
        // would make every assertion below true of nothing.
        assert!(
            found.len() >= 10,
            "only {} playable rule(s), so the foundation did not read",
            found.len()
        );
        for rule in ["move", "work", "gather", "deploy", "launch", "end-turn"] {
            assert!(names.contains(&rule), "`{rule}` is not offered");
        }
        // **`upkeep` is a part of `end-turn` and is not a player's to choose.** The same fact
        // `engine::offered` reads, which is why neither writes it down.
        for part in ["upkeep", "perish", "breed", "refresh", "discard-disorder"] {
            assert!(!names.contains(&part), "`{part}` is a part and is offered");
        }
    }

    /// **A rule's inputs come out in the order it declares them.**
    #[test]
    fn a_rule_asks_for_what_it_declares() {
        let found = playable();
        let move_rule = found
            .iter()
            .find(|it| it.name == "move")
            .expect("`move` is a rule");
        assert_eq!(
            move_rule.inputs,
            vec![
                ("what".to_string(), "unit".to_string()),
                ("from".to_string(), "place".to_string()),
                ("to".to_string(), "place".to_string()),
            ]
        );

        let end = found
            .iter()
            .find(|it| it.name == "end-turn")
            .expect("`end-turn` is a rule");
        assert!(end.inputs.is_empty(), "`end-turn` asks for nothing");
    }

    /// **A place is written as a number and a kind as a word.**
    #[test]
    fn an_input_is_written_as_its_type_says() {
        assert_eq!(written_as("place"), Kind::Number);
        assert_eq!(written_as("unit"), Kind::Name);
        assert_eq!(written_as("resource"), Kind::Name);
        assert_eq!(written_as("founder"), Kind::Name);
    }
}
