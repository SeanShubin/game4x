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
use crate::store::{Store, Unbound, fill, solutions};

const RULE: &str = "rule";
const NEEDS: &str = "needs";
const DROPS: &str = "drops";
const ADDS: &str = "adds";
const NAME: &str = "name";
const RELATION: &str = "relation";
/// **What fires a rule**, which is about how a rule is stated and not about what the game is - so
/// `by`, `command` and `turn` are the engine's own vocabulary, like `rule` and `needs` above.
const BY: &str = "by";
const COMMAND: &str = "command";
const TURN: &str = "turn";

/// Why a command did not happen, said in terms of the rule rather than of the engine.
#[derive(Debug, PartialEq, Eq)]
pub enum Refused {
    /// The command names something no `{rule name:...}` row declares.
    NoSuchRule { name: String },
    /// The command names a rule that is not fired by a command.
    ///
    /// **`by` has to mean something in both directions or it means nothing.** Without this a turn
    /// would skip `move` and a player could still type `{grow where:1}`, which is the rule being
    /// enforced against the engine and not against the game.
    ///
    /// **`by` is required rather than defaulted**, so this carries what the rule actually says -
    /// the turn, something the engine does not know, or nothing at all. A rule that does not say
    /// how it fires is one somebody has not finished writing, and guessing `command` for it is
    /// the engine deciding.
    NotByCommand { name: String, by: String },
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
            Refused::NotByCommand { name, by } => {
                write!(out, "`{name}` is fired by {by} and not by a command")
            }
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
    let Some(declared) = declaration(rules, &name) else {
        return Err(Refused::NoSuchRule { name });
    };
    if declared.value(BY) != Some(COMMAND) {
        let by = match declared.value(BY) {
            Some(TURN) => "the turn".to_string(),
            Some(other) => format!("`{other}`"),
            None => "nothing it states".to_string(),
        };
        return Err(Refused::NotByCommand { name, by });
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

fn declaration<'a>(rules: &'a [Row], name: &str) -> Option<&'a Row> {
    rules
        .iter()
        .find(|row| row.relation == RULE && row.value(NAME) == Some(name))
}

fn clauses_of<'a>(rules: &'a [Row], kind: &str, name: &str) -> Vec<&'a Row> {
    rules
        .iter()
        .filter(|row| row.relation == kind && row.value(RULE) == Some(name))
        .collect()
}

/// A clause as a pattern: its own two keys removed, and its holes left as holes.
fn pattern(clause: &Row, name: &str) -> Result<Row, Refused> {
    let relation = clause
        .value(RELATION)
        .ok_or_else(|| Refused::Unstated {
            rule: name.to_string(),
            clause: write(clause),
        })?
        .to_string();
    let mut values = clause.values.clone();
    values.remove(RULE);
    values.remove(RELATION);
    Ok(Row { relation, values })
}

/// Run every rule the turn fires, once for each way the world satisfies it.
///
/// **This is the concept the other three did without, and it is why `src/` grew.** A rule fired by
/// a command is handed its `$name` holes; a rule fired by a turn has no command, so the holes are
/// bound from the world by [`crate::store::solutions`] - the search `README.md` had been naming as
/// a concept with no owner since the first commit.
///
/// **One pass, against the world as the turn found it.** Solutions are worked out from `store` and
/// applied to a copy, so a rule cannot see what another firing of it has just done. That makes a
/// turn terminate by construction rather than by a rule about loops - and it is a choice rather
/// than the only option, because a fixpoint would keep firing until nothing changed and would not.
///
/// **What two firings that contend look like is not settled here.** No rule the data has both
/// drops and fires on a turn, so the case has not come up; when it does it is a concept with a
/// name, not a patch.
pub fn turn(store: &Store, rules: &[Row]) -> Result<Store, Refused> {
    let mut after = store.clone();
    for declared in rules
        .iter()
        .filter(|row| row.relation == RULE && row.value(BY) == Some(TURN))
    {
        let name = declared.value(NAME).unwrap_or_default().to_string();

        // Every way the world satisfies every `needs` clause at once. Starting from one empty
        // binding rather than from none: a rule with no clauses fires once, and a rule whose
        // first clause matches nothing fires not at all.
        let mut solved = vec![BTreeMap::new()];
        for clause in clauses_of(rules, NEEDS, &name) {
            let wanted = pattern(clause, &name)?;
            solved = solved
                .iter()
                .flat_map(|bound| solutions(store, &wanted, bound))
                .collect();
        }

        for bindings in solved {
            let filled = |clause: &Row| -> Result<Row, Refused> {
                let wanted = pattern(clause, &name)?;
                fill(&wanted, &bindings).map_err(|why| Refused::Unbound {
                    rule: name.clone(),
                    why,
                })
            };
            for clause in clauses_of(rules, DROPS, &name) {
                let row = filled(clause)?;
                if after.remove(&row) == 0 {
                    return Err(Refused::NothingToDrop {
                        rule: name.clone(),
                        wanted: write(&row),
                    });
                }
            }
            for clause in clauses_of(rules, ADDS, &name) {
                after.add(filled(clause)?);
            }
        }
    }
    Ok(after)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation::read;

    /// A rule with no game in it, so that these tests do not restate the ones in `tests/`.
    const RULES: &str = "\
{rule name:swap by:command}
{needs rule:swap relation:here what:$a}
{drops rule:swap relation:here what:$a}
{adds rule:swap relation:here what:$b}
";

    /// A rule that does not say how it fires. **Declared here rather than written inline**,
    /// because the edit that gave every other rule a `by` was a sweep over `{rule name:swap}` and
    /// would have given this one too.
    const NO_BY: &str = "{rule name:swap}";

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
                "{rule name:swap by:command}\n{drops rule:swap relation:here what:$b}\n",
                Refused::NothingToDrop {
                    rule: "swap".to_string(),
                    wanted: "{here what:y}".to_string(),
                },
            ),
            (
                "{swap a:x b:y}",
                "{rule name:swap by:command}\n{needs rule:swap what:$a}\n",
                Refused::Unstated {
                    rule: "swap".to_string(),
                    clause: "{needs rule:swap what:$a}".to_string(),
                },
            ),
            // **A rule that does not say how it fires is refused, and the refusal says which.**
            // `by` is reported back rather than defaulted, so a rule nobody finished writing
            // cannot be run as though somebody had.
            (
                "{swap a:x b:y}",
                NO_BY,
                Refused::NotByCommand {
                    name: "swap".to_string(),
                    by: "nothing it states".to_string(),
                },
            ),
        ];
        for (command, stated, expected) in &refused {
            let why = run(&store, &read(stated).expect(stated), &one(command))
                .expect_err("this command cannot happen");
            assert_eq!(&why, expected, "{command}");
        }
        assert_eq!(refused.len(), 6, "six refusals, and each is checked");
    }
}
