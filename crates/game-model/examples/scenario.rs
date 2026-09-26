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
    let (game, history, refused) = watched();
    (
        game,
        history.into_iter().map(|(_, it)| it).collect(),
        refused,
    )
}

/// The same run, with each command's turn number beside it.
///
/// **A turn is what a person reads a game in.** `{end-turn}` closes one, so the turn a command
/// belongs to is how many `end-turn`s came before it - which is the only thing this adds, and it is
/// the thing that makes 545 lines of trace into something that can be vetted by hand.
#[allow(clippy::type_complexity)]
pub fn watched() -> (Game, Vec<(usize, Effect)>, Option<String>) {
    let (mut game, commands) = opening();
    let mut history = Vec::new();
    let mut turn = 1;
    for command in &commands {
        match fire(&game, command, 1) {
            Ok((next, effect)) => {
                let closes = effect.command.relation == "end-turn";
                history.push((turn, effect));
                if closes {
                    turn += 1;
                }
                game = next;
            }
            Err(why) => return (game, history, Some(format!("{why:?}"))),
        }
    }
    (game, history, None)
}

/// What a place holds, as a person would say it.
///
/// **Grouped by place and summed**, because a store keeps two rows of four citizens apart and a
/// reader counting his population does not.
pub fn holdings(game: &Game) -> BTreeMap<String, BTreeMap<String, i64>> {
    let state = state_relations(game);
    let mut out: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
    for row in game.rows().rows() {
        if !state.contains(&row.relation) {
            continue;
        }
        let Some(place) = row.value("where") else {
            continue;
        };
        let count = row
            .value("quantity")
            .and_then(|it| it.parse::<i64>().ok())
            .unwrap_or(1);
        *out.entry(place.to_string())
            .or_default()
            .entry(row.relation.clone())
            .or_default() += count;
    }
    out
}

/// The whole playthrough as markdown, which is the thing to read.
///
/// # Why a file rather than a terminal
///
/// **Sean, 2026-09-26**: *I see a terminal pop up and a bunch of commands fly by too fast for me to
/// grok.* **He is right and the first version of this was the wrong shape.** `R-9` already says how
/// this repository is read - *I can browse the reports without a script running* - and
/// `scenario/expected/play.4x` was the precedent: a committed artifact of a playthrough, which
/// diffs when the game changes.
///
/// **So the whole thing goes in one file and the terminal gets nine lines.** The file is read at
/// leisure, in an editor, and its diff is what says a rule moved. **Nothing is behind a flag**,
/// because a committed file whose contents depend on how it was invoked is a file nobody can trust.
///
/// **Four parts, in the order they answer questions**: the turns, what fired, the world it left, and
/// then every command's takings for when a number looks wrong.
pub fn played_as_markdown() -> String {
    let mut game = opening().0;
    let (after, history, refused) = watched();
    let rules = every_rule();
    let turns = history.last().map(|(turn, _)| *turn).unwrap_or(0);

    let mut out = String::from("# The main scenario, played\n\n");
    out.push_str(
        "**Generated. Do not edit.** `scripts/scenario.sh`. The rules are `spec/data/`, the world\n\
         and the act are `scenario/main.4x`, and this file is what happened when they met.\n\n\
         **`spec/scenarios.md`: it is vetted by hand.** So this is written to be read rather than\n\
         to pass; `crates/game-model/tests/scenario.rs` is the part a gate holds.\n\n",
    );
    out.push_str(&format!(
        "{} rules in the ruleset, {} commands, {turns} turns.\n\n",
        rules.len(),
        history.len()
    ));
    out.push_str(
        "Every place is written the way `scenario/main.4x` writes it: `place-1` is the surface\n\
         landed on, `place-2` the orbit above it, `place-3` the surface taken by land, `place-4`\n\
         the orbit above that.\n\n",
    );

    let mut fired: BTreeMap<String, usize> = BTreeMap::new();
    let mut closed = 0;
    for (at, (turn, effect)) in history.iter().enumerate() {
        if at == 0 || history[at - 1].0 != *turn {
            out.push_str(&format!("## Turn {turn}\n\n"));
        }
        for rule in &effect.fired {
            *fired.entry(rule.clone()).or_default() += 1;
        }
        out.push_str(&format!(
            "    {}\n",
            friendly_command(&game, &effect.command)
        ));
        game = fire(&game, &effect.command, 1)
            .map(|(next, _)| next)
            .unwrap_or(game);

        if effect.command.relation == "end-turn" {
            out.push('\n');
            for (place, held) in holdings(&game) {
                let what: Vec<String> = held
                    .iter()
                    .filter(|(_, count)| **count > 0)
                    .map(|(kind, count)| format!("{count} {kind}"))
                    .collect();
                if !what.is_empty() {
                    out.push_str(&format!("  place-{place}: {}\n", what.join(", ")));
                }
            }
            out.push('\n');
            closed += 1;
        }
    }
    assert!(closed > 0, "no turn closed, so there is nothing to read");

    if let Some(why) = &refused {
        out.push_str(&format!(
            "**REFUSED** after {} command(s): {why}\n\n",
            history.len()
        ));
    }

    // **What fired rather than what the file says.** A command names one rule and `end-turn` runs
    // five more through `{part ...}`, so counting the command counts a narrower population: it
    // reported six of fifteen where the engine had applied nine.
    out.push_str("## What fired\n\n");
    for rule in &rules {
        match fired.get(rule) {
            Some(times) => out.push_str(&format!("    {rule:18} {times}\n")),
            None => out.push_str(&format!("    {rule:18} -   never\n")),
        }
    }
    let missing: Vec<&String> = rules.iter().filter(|it| !fired.contains_key(*it)).collect();
    out.push_str(&format!(
        "\n{} of {} rules fired; {} did not: {missing:?}\n\n",
        rules.len() - missing.len(),
        rules.len(),
        missing.len()
    ));
    // **The second half of `D-5`'s clause**: a rule that did not fire is named here with what it
    // needs, so an omission is read rather than noticed. **Asserted and not merely printed** - an
    // unfired rule with no entry is a rule that stopped firing and said nothing.
    if !missing.is_empty() {
        out.push_str("### What a typical game does not use\n\n");
        for rule in &missing {
            let needs = UNUSUAL
                .iter()
                .find(|(named, _)| named == rule)
                .map(|(_, needs)| *needs)
                .unwrap_or_else(|| {
                    panic!(
                        "`{rule}` fired nowhere and is not named in `UNUSUAL` - `D-5` asks that a \
                         rule that does not fire be named with the unusual situation it needs"
                    )
                });
            out.push_str(&format!("**`{rule}`** needs {needs}\n\n"));
        }
        out.push_str(
            "`spec/scenarios.md`: *a mechanic that only appears in an unusual situation belongs to\n\
             a scenario of its own. Those are not built until the main scenario satisfies its\n\
             reader.*\n\n",
        );
    }

    out.push_str("## The world it left\n\n");
    let state = state_relations(&after);
    assert!(
        state.len() > 10,
        "only {} state relations, so this would show almost nothing",
        state.len()
    );
    let names = Names::of(after.rows().rows());
    let mut rows: Vec<String> = after
        .rows()
        .rows()
        .iter()
        .filter(|row| state.contains(&row.relation))
        .map(|it| names.row(it))
        .collect();
    rows.sort();
    for line in &rows {
        out.push_str(&format!("    {line}\n"));
    }
    out.push_str(&format!(
        "\n{} row(s) of world, out of {} in the store.\n\n",
        rows.len(),
        after.rows().rows().len()
    ));

    // **Last, because it is the part to reach for rather than to read.** When a number above looks
    // wrong, this is what produced it.
    out.push_str("## What every command took and made\n\n");
    let mut game = opening().0;
    for (turn, effect) in &history {
        out.push_str(&format!(
            "### Turn {turn}: {}\n\n",
            friendly_command(&game, &effect.command)
        ));
        out.push_str(&format!("    fired {}\n", effect.fired.join(", ")));
        for row in &effect.took {
            out.push_str(&format!("    took  {}\n", write(row)));
        }
        for row in &effect.made {
            out.push_str(&format!("    made  {}\n", write(row)));
        }
        out.push('\n');
        game = fire(&game, &effect.command, 1)
            .map(|(next, _)| next)
            .unwrap_or(game);
    }
    out
}

/// Where the playthrough is written.
pub fn played_at() -> PathBuf {
    root().join("scenario").join("played.md")
}

fn main() {
    let text = played_as_markdown();
    let at = played_at();
    std::fs::write(&at, &text).unwrap_or_else(|why| panic!("{}: {why}", at.display()));

    let (_, history, refused) = watched();
    let rules = every_rule();
    let fired: std::collections::BTreeSet<String> = history
        .iter()
        .flat_map(|(_, effect)| effect.fired.iter().cloned())
        .collect();
    let missing: Vec<&String> = rules.difference(&fired).collect();

    // **Nine lines, because the file is the thing to read.** Anything longer here is the problem
    // this arrangement exists to fix.
    println!("Played scenario/main.4x against spec/data/.");
    println!();
    println!(
        "  {} turns, {} commands, {} of {} rules fired",
        history.last().map(|(turn, _)| *turn).unwrap_or(0),
        history.len(),
        rules.len() - missing.len(),
        rules.len()
    );
    if !missing.is_empty() {
        println!("  never fired: {missing:?}");
    }
    match refused {
        Some(why) => println!("  REFUSED: {why}"),
        None => println!("  nothing was refused"),
    }
    println!();
    println!(
        "Read it in scenario/played.md - {} lines.",
        text.lines().count()
    );
}

/// A command as the scenario file writes it, with ids put back to names.
///
/// **The trace showed `{deploy what:51 where:2}`** and the file says
/// `{deploy where:place-2 what:ark}`. A reader confirming that the scenario did what it says cannot
/// do it against relation ids, so the command is rendered back the way he wrote it.
fn friendly_command(game: &Game, command: &Row) -> String {
    Names::of(game.rows().rows()).row(command)
}

/// Rules a typical game does not use, each with the unusual situation it needs.
///
/// # `D-5` asks for two things and this is the second
///
/// **`D-5`, promoted 2026-09-26**: *every rule a typical game uses fires at least once while it
/// runs, measured by what fired rather than by what the file says, and **a rule that does not fire
/// is named with the unusual situation it needs** - so an omission is something I can read rather
/// than something I have to notice.*
///
/// **`spec/scenarios.md` is where the naming points**: *a mechanic that only appears in an unusual
/// situation belongs in a scenario of its own. Those are not built until the main scenario satisfies
/// its reader.* So an entry here names a scenario that does not exist yet, and the list empties as
/// those are built.
///
/// **One list, rendered into `scenario/played.md` and held by
/// `every_rule_fires_or_is_named_with_what_it_needs`.** The naming has to be something Sean reads,
/// which is why it reaches the file rather than living only in a test.
///
/// **The clause this serves replaced one that gave no signal for it.** The old one asked only that
/// every rule fire, so a rule that silently stopped firing read as fourteen-of-fifteen and nothing
/// said which or why.
pub const UNUSUAL: [(&str, &str); 1] = [(
    "perish",
    "a starvation - a settlement whose citizens are hungry when its food runs out. The main \
     scenario works its food every turn and sustains its people, so nobody starves in it.",
)];

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
