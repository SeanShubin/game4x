//! Play the main scenario and print what happened, command by command.
//!
//! **`D-5`**: *a main scenario exists over the reviewed ruleset and I have watched it run.* This is
//! the watching. `scenario/main.4x` is the world and the act; this plays it against the foundation
//! and prints what each command took and made, then the state it left.
//!
//! ```text
//! cargo run -p game-model --example scenario
//! ```
//!
//! **Nothing here decides anything.** The ruleset is `spec/data/`, the world and the commands are
//! `scenario/main.4x`, and the only thing this file contributes is the order to read them in - so a
//! rule that fires differently tomorrow shows here without this changing.
//!
//! **It reads the friendly form and the engine runs the foundation**, which is the same chain every
//! test runs on: `friendly_notation::fold` resolves a name to an id and the engine never sees a
//! name. `spec/invariants.md`: *the engine reads the form whose references are ids.*

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use friendly_notation::{Names, fold};
use game_model::engine::{Effect, Game, fire};
use game_model::foundation;
use game_model::notation::{Row, write};
use game_model::schema::Schema;

/// The repository root, from this crate.
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

/// The scenario's two sections, folded into rows.
///
/// **`{given}` is the world and `{when}` is the act**, and a scenario has no `{then}` - which is
/// what makes it a scenario rather than a test. The markers are read the way `examples/report.rs`
/// reads a test's, by looking for a lone `{word}` line.
pub fn sections(foundation: &[Row], schema: &Schema) -> (Vec<Row>, Vec<Row>) {
    let at = root().join("scenario").join("main.4x");
    let text = std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));

    let (mut given, mut when) = (String::new(), String::new());
    let mut into: Option<&mut String> = None;
    for line in text.lines() {
        match line.trim() {
            "{given}" => {
                into = Some(&mut given);
                continue;
            }
            "{when}" => {
                into = Some(&mut when);
                continue;
            }
            _ => {}
        }
        if let Some(target) = into.as_deref_mut() {
            target.push_str(line);
            target.push('\n');
        }
    }
    assert!(!given.is_empty(), "scenario/main.4x states no world");
    assert!(!when.is_empty(), "scenario/main.4x states no act");

    // **Two steps and they are different jobs.** `fold` reads the friendly `-> n` back into the
    // relation's quantity column; `Names::foundation` resolves every reference from a name to an
    // id and drops a generated name the foundation has nowhere to keep. **`Names` is built over
    // both sections together**, because a command in the `when` names a place the `given` declared.
    let folded = |what: &str, text: &str| {
        fold(text, schema).unwrap_or_else(|why| panic!("scenario/main.4x `{what}`: {why}"))
    };
    let (given, when) = (folded("given", &given), folded("when", &when));

    // **Built over the foundation as well as the scenario**, because `Names` reads its schema out
    // of the rows it is handed - so without the foundation it knows no relation, treats every row
    // as a rule-named command, and leaves a generated `name` on a `territory` that has nowhere to
    // keep one. **Measured the hard way**: `Game::of` then refused *`territory` is (id) and this
    // row gives (id name)*, which is the schema catching what the translator had not been told.
    let mut all = foundation.to_vec();
    all.extend(given.clone());
    all.extend(when.clone());
    let names = Names::of(&all);
    let resolve = |what: &str, rows: &[Row]| -> Vec<Row> {
        rows.iter()
            .map(|row| {
                names
                    .foundation(row)
                    .unwrap_or_else(|why| panic!("scenario/main.4x `{what}`: {why}"))
            })
            .collect()
    };

    (resolve("given", &given), resolve("when", &when))
}

/// The game the scenario starts in, and the commands it plays.
pub fn opening() -> (Game, Vec<Row>) {
    let foundation = foundation::rows();
    let schema = Schema::of(&foundation).expect("the foundation states a schema");
    let (given, when) = sections(&foundation, &schema);

    let mut all = foundation;
    all.extend(given);
    let game = Game::of(all).unwrap_or_else(|why| panic!("scenario/main.4x `given`: {why}"));
    (game, when)
}

/// Every rule the ruleset declares, by name.
pub fn every_rule() -> BTreeSet<String> {
    foundation::rows()
        .iter()
        .filter(|row| row.relation == "rule")
        .filter_map(|row| row.value("name").map(str::to_string))
        .collect()
}

/// What each command took and made, and what the world looked like afterwards.
///
/// **One entry per command, in the order they were played.** A refusal stops the run and is
/// returned, because a command that was refused leaves no state to have arrived at - which is
/// `engine::play`'s own shape, reproduced here so the effects can be printed as they arrive rather
/// than only at the end.
#[allow(clippy::type_complexity)]
pub fn played() -> (Game, Vec<Effect>, Option<String>) {
    let (mut game, commands) = opening();
    let mut history = Vec::new();
    for command in &commands {
        match fire(&game, command, 1) {
            Ok((next, effect)) => {
                history.push(effect);
                game = next;
            }
            Err(why) => return (game, history, Some(format!("{why:?}"))),
        }
    }
    (game, history, None)
}

fn main() {
    let (after, history, refused) = played();
    let rules = every_rule();

    println!("# The main scenario\n");
    println!(
        "{} rules in the ruleset, {} commands.\n",
        rules.len(),
        history.len()
    );

    let mut fired: BTreeMap<String, usize> = BTreeMap::new();
    for effect in &history {
        // **`effect.fired` and not `command.relation`**, which is `D-5`'s *measured by what fired*.
        // A command names one rule and `end-turn` runs eight of them through `{part ...}`, so
        // counting the command counts a narrower population and reports a plausible six of
        // fifteen. **This lane wrote that version first** and the number looked reasonable.
        for rule in &effect.fired {
            *fired.entry(rule.clone()).or_default() += 1;
        }
        println!("## {}", write(&effect.command));
        println!("  fired {}", effect.fired.join(", "));
        for row in &effect.took {
            println!("  took  {}", write(row));
        }
        for row in &effect.made {
            println!("  made  {}", write(row));
        }
        println!();
    }

    if let Some(why) = refused {
        println!("REFUSED after {} command(s): {why}\n", history.len());
    }

    // **What fired rather than what the file says** - `D-5`'s own words. A command's relation is
    // the rule it fires, so this counts rules and not lines.
    println!("## What fired\n");
    for rule in &rules {
        match fired.get(rule) {
            Some(times) => println!("  {rule:18} {times}"),
            None => println!("  {rule:18} -"),
        }
    }
    let missing: Vec<&String> = rules.iter().filter(|it| !fired.contains_key(*it)).collect();
    println!(
        "\n{} of {} rules fired; {} did not: {missing:?}",
        rules.len() - missing.len(),
        rules.len(),
        missing.len()
    );

    println!("\n## The state it left\n");
    // **The world and not the ruleset.** Every row is in one store, so the rules the game plays by
    // sit in `after` beside the places and the citizens - and somebody watching a game played does
    // not want eighty `{binding ...}` rows. **`{state relation:N}` already says which relations a
    // world is made of**, which is the same question `tests/first_test.rs` asks to hold the
    // scenario layer apart from the ruleset.
    let state = state_relations(&after);
    assert!(
        state.len() > 10,
        "only {} state relations, so this would print almost nothing",
        state.len()
    );
    let names = Names::of(after.rows().rows());
    let mut shown: Vec<String> = after
        .rows()
        .rows()
        .iter()
        .filter(|row| state.contains(&row.relation))
        .map(|it| names.row(it))
        .collect();
    shown.sort();
    for line in &shown {
        println!("  {line}");
    }
    println!(
        "\n{} row(s) of world, out of {} in the store.",
        shown.len(),
        after.rows().rows().len()
    );
}

/// Every relation the schema marks as state, by name.
///
/// **Read out of the game rather than listed**, so a relation that becomes state shows here without
/// this changing - which is the whole of what `{state relation:N}` is for.
pub fn state_relations(game: &Game) -> BTreeSet<String> {
    let named: BTreeMap<String, String> = game
        .rows()
        .rows()
        .iter()
        .filter(|row| row.relation == "relation")
        .filter_map(|row| Some((row.value("id")?.to_string(), row.value("name")?.to_string())))
        .collect();
    game.rows()
        .rows()
        .iter()
        .filter(|row| row.relation == "state")
        .filter_map(|row| row.value("relation"))
        .filter_map(|it| named.get(it).cloned())
        .collect()
}
