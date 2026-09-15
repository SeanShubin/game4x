//! The whole of the engine: bind a command, check what the rule needs, apply what it changes.
//!
//! # The four words it knows
//!
//! `rule`, `needs`, `drops` and `adds`. Nothing else. **They are the engine's own vocabulary and
//! not the game's**, which is the distinction the experiment turns on: a word here is about how a
//! rule is stated, and a word like `move` or `adjacent` is about what the game is. The second
//! kind appears in `data/` and in the tests and nowhere in `src/`.
//!
//! A rule is stated as rows:
//!
//! ```text
//! {rule name:move}
//! {needs rule:move relation:at       thing:$it   place:$from}
//! {needs rule:move relation:adjacent from:$from  to:$to}
//! {drops rule:move relation:at       thing:$it   place:$from}
//! {adds  rule:move relation:at       thing:$it   place:$to}
//! ```
//!
//! A command is a row too, and its relation names the rule: `{move it:scout from:1 to:2}` binds
//! `$it`, `$from` and `$to` to what it says.
//!
//! # The two keys a clause reserves
//!
//! A clause row is flat, so `rule` and `relation` are read by the engine and everything else is
//! the pattern. **A game relation with a column called `rule` therefore cannot be written**, and
//! that is a real limit rather than an oversight - the alternative is nesting, which the notation
//! does not have. Recorded because the day it bites, the fix is a decision and not a patch.

use std::collections::BTreeMap;

use crate::notation::{Row, write};
use crate::store::{Store, Unbound, fill};

const RULE: &str = "rule";
const NEEDS: &str = "needs";
const DROPS: &str = "drops";
const ADDS: &str = "adds";
const NAME: &str = "name";
const RELATION: &str = "relation";

/// Why a command did not happen, said in terms of the rule rather than of the engine.
#[derive(Debug, PartialEq, Eq)]
pub enum Refused {
    /// The command names something no `{rule name:...}` row declares.
    NoSuchRule { name: String },
    /// A clause wants a `$name` the command did not bind.
    Unbound { rule: String, why: Unbound },
    /// A clause is malformed - it says no `relation`.
    Unstated { rule: String, clause: String },
    /// Everything was bound and the world does not agree.
    NotSo { rule: String, wanted: String },
    /// The rule drops something no row matches, so the rule contradicts itself.
    ///
    /// **Not a game rule and not reachable from `data/`**: every `drops` in there is also a
    /// `needs`, so the check can only fire on a rule that is wrong. **A count over nothing is
    /// the same failure with the sign flipped** - `CLAUDE.md` - and a drop that removes nothing
    /// is exactly that, succeeding silently.
    NothingToDrop { rule: String, wanted: String },
}

impl std::fmt::Display for Refused {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refused::NoSuchRule { name } => write!(out, "no rule is named `{name}`"),
            Refused::Unbound { rule, why } => write!(out, "`{rule}`: {why}"),
            Refused::Unstated { rule, clause } => {
                write!(out, "`{rule}`: {clause} says no `relation`")
            }
            Refused::NotSo { rule, wanted } => write!(out, "`{rule}` needs {wanted} and it is not"),
            Refused::NothingToDrop { rule, wanted } => {
                write!(out, "`{rule}` drops {wanted} and nothing matched")
            }
        }
    }
}

/// Run one command against one world, and give back the world it leaves.
///
/// **A new store rather than an edit in place**, so a refusal half way through applying leaves
/// nothing half applied. The caller either has the world after the command or the world before
/// it, and never one in between.
///
/// **The signature is the whole of that guarantee, and there is deliberately no test for it.**
/// `&Store` in and a fresh `Store` out means a partial application is not a thing that can be
/// written here, so a test asserting the world is unchanged after a refusal would pass without
/// the property and is therefore not evidence of it - *passing tests prove nothing*,
/// `CLAUDE.md`. If this ever takes `&mut Store`, that test becomes necessary in the same commit.
pub fn run(store: &Store, rules: &[Row], command: &Row) -> Result<Store, Refused> {
    let name = command.relation.clone();
    let declared = rules
        .iter()
        .any(|row| row.relation == RULE && row.value(NAME) == Some(name.as_str()));
    if !declared {
        return Err(Refused::NoSuchRule { name });
    }

    let bindings: BTreeMap<String, String> = command.values.clone();
    let clauses = |kind: &str| -> Vec<&Row> {
        rules
            .iter()
            .filter(|row| row.relation == kind && row.value(RULE) == Some(name.as_str()))
            .collect()
    };
    let wanted = |clause: &Row| -> Result<Row, Refused> {
        let relation = clause
            .value(RELATION)
            .ok_or_else(|| Refused::Unstated {
                rule: name.clone(),
                clause: write(clause),
            })?
            .to_string();
        let mut values = clause.values.clone();
        values.remove(RULE);
        values.remove(RELATION);
        fill(&Row { relation, values }, &bindings).map_err(|why| Refused::Unbound {
            rule: name.clone(),
            why,
        })
    };

    // **Everything is checked before anything is applied**, which is what makes the two halves
    // below safe to write as two loops rather than one.
    for clause in clauses(NEEDS) {
        let row = wanted(clause)?;
        if !store.holds(&row) {
            return Err(Refused::NotSo {
                rule: name.clone(),
                wanted: write(&row),
            });
        }
    }

    let mut after = store.clone();
    for clause in clauses(DROPS) {
        let row = wanted(clause)?;
        if after.remove(&row) == 0 {
            return Err(Refused::NothingToDrop {
                rule: name.clone(),
                wanted: write(&row),
            });
        }
    }
    for clause in clauses(ADDS) {
        after.add(wanted(clause)?);
    }
    Ok(after)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation::read;

    /// A rule with no game in it, so that these tests do not restate the ones in `tests/`.
    const RULES: &str = "\
{rule name:swap}
{needs rule:swap relation:here what:$a}
{drops rule:swap relation:here what:$a}
{adds rule:swap relation:here what:$b}
";

    fn rules() -> Vec<Row> {
        read(RULES).expect("the rules")
    }

    fn one(text: &str) -> Row {
        read(text).expect(text)[0].clone()
    }

    #[test]
    fn a_rule_whose_needs_hold_drops_and_adds() {
        let store = Store::of(read("{here what:x}").expect("a world"));
        let after = run(&store, &rules(), &one("{swap a:x b:y}")).expect("x is here");
        let rows: Vec<String> = after.rows().iter().map(write).collect();
        assert_eq!(rows, vec!["{here what:y}".to_string()]);
    }

    /// Each way a command is refused, and the count so that none is untested.
    #[test]
    fn every_refusal_says_the_rule_and_what_about_it() {
        let store = Store::of(read("{here what:x}").expect("a world"));
        let refused = [
            (
                "{stroll a:x b:y}",
                RULES,
                Refused::NoSuchRule {
                    name: "stroll".to_string(),
                },
            ),
            (
                "{swap a:x}",
                RULES,
                Refused::Unbound {
                    rule: "swap".to_string(),
                    why: Unbound::Hole {
                        key: "what".to_string(),
                        name: "b".to_string(),
                    },
                },
            ),
            (
                "{swap a:q b:y}",
                RULES,
                Refused::NotSo {
                    rule: "swap".to_string(),
                    wanted: "{here what:q}".to_string(),
                },
            ),
            (
                "{swap a:x b:y}",
                "{rule name:swap}\n{drops rule:swap relation:here what:$b}\n",
                Refused::NothingToDrop {
                    rule: "swap".to_string(),
                    wanted: "{here what:y}".to_string(),
                },
            ),
            (
                "{swap a:x b:y}",
                "{rule name:swap}\n{needs rule:swap what:$a}\n",
                Refused::Unstated {
                    rule: "swap".to_string(),
                    clause: "{needs rule:swap what:$a}".to_string(),
                },
            ),
        ];
        for (command, stated, expected) in &refused {
            let why = run(&store, &read(stated).expect(stated), &one(command))
                .expect_err("this command cannot happen");
            assert_eq!(&why, expected, "{command}");
        }
        assert_eq!(refused.len(), 5, "five refusals, and each is checked");
    }
}
