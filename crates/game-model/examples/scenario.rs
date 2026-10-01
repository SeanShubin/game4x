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
            // **`Display` and not `Debug`.** `Refused` says what happened in words - *`move`.`4`
            // wants a value for `gathering`* - and the derived form says
            // `Unbound { rule: "move", ... }`, which is the struct rather than the refusal.
            //
            // **And `told` and not `Display`, so the rows in it carry names.** The engine holds
            // one as `{deposit where:4 what:50}`, which is true and of no use to a reader of
            // `scenario/main.4x`, where the same row is `{deposit where:place-4 what:energy}`.
            // **`game` is the world as the refused command met it**, which is where the names
            // are, so this is the one place the rendering can happen at all.
            Err(why) => {
                let names = Names::of(game.rows().rows());
                let said = why.told(&|row| names.row(row));
                return (game, history, Some(said));
            }
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

    // **`P-573`'s report, and it is short on purpose.** Sean took a report over a refusal knowing
    // a report can be ignored, on the reasoning he gave for the regression suite - so its whole
    // value is that he reads it, which is why it is here rather than in a file he would have to
    // remember to open. **Its diff is what says a rule stopped naming a column.**
    out.push_str("## Columns a rule leaves as it found them\n\n");
    let carried = carried_through();
    if carried.is_empty() {
        out.push_str(
            "None - every `add` clause names every column of every member it acts on.\n\n",
        );
    } else {
        for (rule, family, member, columns) in &carried {
            out.push_str(&format!(
                "**`{rule}`** acts on `{family}`; `{member}` carries {} that no clause names.\n\n",
                columns
                    .iter()
                    .map(|it| format!("`{it}`"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
        out.push_str(
            "`spec/invariants.md`: *what it does not name it leaves as it found it.* These are\n\
             carried through rather than refused - `P-573`, which chose a report over making the\n\
             notation say so.\n\n",
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

/// One generated case: what it is called, the command it covers, and its whole text.
///
/// **The command is carried beside the name because the orphan check needs it.** `P-572`:
/// *the two are told apart by `scenario/main.4x` - a case whose command is still there is waiting
/// on Sean, and a case whose command is gone is waiting on nobody.* **So the question is what the
/// commands are, and a check that compared filenames answered a different one** - `S-195`.
pub struct Case {
    pub name: String,
    /// The command in the friendly form, exactly as `{when}` states it.
    pub command: String,
    pub text: String,
}

/// Where the generated suites live, one directory each.
///
/// **A directory of their own**, because Sean, 2026-09-27: *organized in a way that allows me to
/// browse them without clutter of other files.*
///
/// # `regression/` is beside `reviewed/` and out of `scenario/`, which is `D-6`
///
/// **The two directories are siblings in meaning**: `reviewed/` is the record of what Sean has
/// read and `regression/` of what he has accepted. Under `scenario/` the suite read as an
/// appendix to one scenario, and `docs/process.md` says there are four of these and only the
/// first is about a scenario at all - *every type of thing that is data has a generated suite of
/// its own*.
pub fn suites_at() -> PathBuf {
    root().join("regression")
}

/// The suite of one case per command of the main scenario.
pub fn regression_at() -> PathBuf {
    suites_at().join("scenario")
}

/// One generated test per command of the main scenario: the world before, the command, the world
/// after.
///
/// # Why one per command rather than one file
///
/// **Sean, 2026-09-27**: *for every single command, I want a generated given/when/then test that
/// focuses in that single command... This is an excessive amount of detail, but I won't be looking
/// at all of them, when something changes I will know exactly what changed.*
///
/// **So the unit is the command and the diff is the point.** A rule that changes what `work` does
/// moves every `work` case and nothing else; a rule that changes the first turn moves everything
/// after it, which is the truth about the change rather than noise.
///
/// # The shape is the one the reviewed tests use
///
/// **`{given}`, `{when}`, `{then}`**, in the friendly form, so a case reads like the tests Sean has
/// already approved - and `{given}` is the whole world rather than only what the command touches,
/// which is what makes each case a world that could be run on its own.
///
/// **State rows only.** The ruleset is in the same store as the world, and a case carrying eighty
/// `{binding ...}` rows is one nobody can read. `{state relation:N}` says which relations a world is
/// made of.
pub fn regression_cases() -> Vec<Case> {
    let mut game = opening().0;
    let (_, history, refused) = watched();

    // **A refused scenario is the most useful failure there is and it read as the least.** Sean,
    // 2026-09-27: *I should clearly see why the regression test failed.* What he got was
    // `Unbound { rule: "move", clause: "4", column: "gathering" }` out of a `Debug`, with nothing
    // saying which command was refused or where in the run it was.
    //
    // **`Refused` has a `Display` and nothing was using it.** The commands are said in the friendly
    // form, because the ids in them mean nothing to a reader of `scenario/main.4x`.
    if let Some(why) = &refused {
        let stood = history.len();
        let mut replay = opening().0;
        let mut last = String::from("(none - the first command was refused)");
        for (_, effect) in &history {
            last = friendly_command(&replay, &effect.command);
            replay = fire(&replay, &effect.command, 1)
                .map(|(next, _)| next)
                .unwrap_or(replay);
        }
        let refused_at = opening()
            .1
            .get(stood)
            .map(|it| friendly_command(&replay, it))
            .unwrap_or_else(|| "(past the end of the command list)".to_string());
        panic!(
            "scenario/main.4x does not play.\n\n  \
             {stood} command(s) stood, the last being\n      {last}\n\n  \
             and command {} was refused:\n      {refused_at}\n      {why}\n\n\
             Nothing is recorded, because a scenario that does not play has no behaviour to \
             record. Either the command is wrong or the rule is - read the refusal and decide \
             which.",
            stood + 1
        );
    }

    let mut out = Vec::new();
    let mut seen: BTreeMap<usize, usize> = BTreeMap::new();
    for (at, (turn, effect)) in history.iter().enumerate() {
        let command = friendly_command(&game, &effect.command);
        let after_game = fire(&game, &effect.command, 1)
            .map(|(next, _)| next)
            .unwrap_or_else(|why| panic!("command {}: {why:?}", at + 1));

        // **What this command took and what it made, and nothing else.** The whole world was the
        // first shape and it cascaded: one density changed from six to seven and **all thirty-four
        // files moved**, because every later case carried a `{given}` it had merely inherited.
        // Sean, 2026-09-27: *when something changes I will know exactly what changed* - and
        // thirty-four files is not knowing.
        let names = Names::of(after_game.rows().rows());
        let render = |rows: &[Row]| -> Vec<String> {
            let mut out: Vec<String> = rows.iter().map(|it| names.row(it)).collect();
            out.sort();
            out
        };
        let before = render(&effect.took);
        let after = render(&effect.made);

        // **A directory per turn, and a case named by its position in that turn.** So the listing
        // is the play order, which is how the file is read.
        //
        // # What this replaces, and why the cost it pays is the smaller one now
        //
        // **It was `t{turn}-{rule}-{k}`**, `k` counting repeats of one rule inside one turn - so
        // `t01-move-1` was the first `move` of turn one and said nothing about when it ran.
        // `S-195` chose that by measuring three schemes against Sean's next edit, inserting one
        // `{move}` into turn one:
        //
        // ```text
        // by position in the run   renames 33   the scheme before that
        // by rule+occurrence       renames  1   worst case 13, the count of `work`
        // by turn+rule+occurrence  renames  0   worst case  4, the largest (turn, rule) group
        // ```
        //
        // **Zero, and it cost play order in the filename** - which the comment there said in as
        // many words, and which is exactly what went wrong. Sean, 2026-09-27, reading the
        // directory: *why does `t01-deploy-1.4x` show a deploy when no earlier test shows a move?*
        // The move was `t01-move-1.4x`, the first command of the turn, sorting fourth of six.
        //
        // **A position inside a turn renumbers only its own turn.** Against the same probe it
        // renames 5, the rest of turn one, and its worst case is the largest turn rather than the
        // whole run - 9 today. **So the cascade `S-195` measured is bounded rather than gone**,
        // and it is paid on an edit rather than on every reading.
        //
        // **The occurrence counter is not needed and is gone**: `04-work` and `05-work` are
        // already distinct, so four `work`s read as four adjacent numbers rather than as a
        // counter.
        *seen.entry(*turn).or_insert(0) += 1;
        let at_turn = seen[turn];
        let stem = format!("{at_turn:02}-{}", effect.command.relation);
        let name = format!("{turn:02}/{stem}");
        // **The test's own name carries the turn and holds no slash**, because it is an identifier
        // rather than a path - `{test name:t01-03-deploy}`.
        let test = format!("t{turn:02}-{stem}");
        let mut text = String::new();
        // **The turn and the command, and no position.** This read `command 4 of 34`, which put the
        // very thing `S-195` took out of the filename back into the body: inserting one command in
        // turn one left the names alone and then reported **34 of 35 cases stale**, because every
        // header carried a position and a total that had both moved.
        //
        // **Measured, and it is why the naming alone was not the fix.** A case says which turn it is
        // in and what it does; where it sits in the run is `scenario/played.md`'s business.
        text.push_str(&format!("# Turn {turn}: {command}\n"));
        text.push_str(
            "#\n\
             # **Generated. Do not edit.** `scripts/regression.sh`, from `scenario/main.4x`.\n\
             # **Delete this file to accept what the scenario does now** - `docs/process.md`:\n\
             # *absent expected data means I accept what it does now, so the test writes it, and\n\
             # what I review is the diff in version control.*\n\
             #\n\
             # `{given}` is what the command took and `{then}` is what it made, which is the\n\
             # engine's own account of it. **The whole world was the first shape and cascaded**:\n\
             # one density changed from six to seven and all thirty-four files moved, because\n\
             # every later case carried a world it had merely inherited.\n\
             #\n\
             # `scenario/played.md` is where the whole world at each turn's end is.\n\n",
        );
        text.push_str(&format!("{{test name:{test}}}\n"));
        // **The reference, which is what makes the case executable** - `P-598`: *what never
        // changes is referred to rather than repeated, and the reference is what the runner
        // follows, not only what he clicks.*
        //
        // **`S-234` derived the store over a closed set of three rather than choosing it**:
        // `script` is the test script and `expected` is the comparison target, so `game` is
        // the only reading left, and `Failed::NoSuchStore` refuses a fourth.
        //
        // **The runner already follows it and nothing was invented here.** `{load}` is
        // `{primitive id:12 word:load}`, `crates/game-model/src/script.rs` implements it, and
        // `data/foundation/setup.4x` uses it four times today.
        text.push_str("{load file:setup.4x into:game}\n\n");
        text.push_str("{given}\n");
        for line in &before {
            text.push_str(&format!("{line}\n"));
        }
        text.push_str(&format!("\n{{when}}\n{command}\n\n"));
        text.push_str("{then}\n");
        for line in &after {
            text.push_str(&format!("{line}\n"));
        }

        out.push(Case {
            name: format!("{name}.4x"),
            command,
            text,
        });
        game = after_game;
    }
    assert!(
        out.len() > 10,
        "only {} case(s), which is not the main scenario",
        out.len()
    );
    out
}

/// The relations no rule ever writes, derived from the ruleset rather than listed.
///
/// **`P-598`**: *what never changes is referred to rather than repeated.* **What never changes
/// is derivable**, and deriving it is the difference between a premise that survives a seventh
/// structural relation being added and one that does not.
///
/// **Measured over `spec/data/rules.4x` as it stands**: 55 clauses, 41 of which write - `add`
/// 18, `remove` 22, `put` 1 - and the thirteen relations they name between them are all things.
/// So `territory`, `place`, `adjacency`, `capacity`, `provides` and `consumes` fall out as
/// invariant **because no clause names them**, not because anybody wrote the six down.
///
/// `S-234` found the arithmetic in this lane's first pass - 40 - and the conclusion did not
/// move, because it is a zero over the whole set rather than a proportion of it.
fn relations_a_rule_writes() -> BTreeSet<String> {
    // **The friendly source, because the foundation form names nothing twice.** A clause row in
    // `data/foundation/rules.4x` carries `relation:72`, an id, and comparing an id against a
    // rendered relation name drops nothing - **which is what happened**: `relations_a_rule_writes`
    // read the foundation and returned a set of numbers, so every row looked invariant.
    //
    // **Caught by this function's own `dropped > 0`** rather than by reading it. A set of numbers
    // that intersects nothing is the quietest possible failure here: the file would have been
    // written with the whole world in it and every case would still have run.
    let at = root().join("spec/data/rules.4x");
    let text = std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));
    let rules =
        game_model::notation::read(&text).unwrap_or_else(|why| panic!("{}: {why:?}", at.display()));
    let mut written = BTreeSet::new();
    for row in rules {
        if row.relation != "clause" {
            continue;
        }
        let role = row.values.get("role").map(String::as_str).unwrap_or("");
        if !matches!(role, "add" | "put" | "remove") {
            continue;
        }
        if let Some(relation) = row.values.get("relation") {
            written.insert(relation.clone());
        }
    }
    written
}

/// The scenario's invariant rows: its world, less everything a rule can touch.
///
/// **This is the file a case refers to instead of repeating.** `S-234` names the store:
/// `{load file:... into:game}`, derived over a closed set of three rather than chosen -
/// `script` is the test script and `expected` is the comparison target, so `game` is the only
/// reading left and `Failed::NoSuchStore` refuses a fourth.
///
/// **One entry per description**, which is what the rows already are: this writes them as the
/// renderer writes any row, so a diff here is a diff in the scenario.
pub fn invariant_rows() -> String {
    let written = relations_a_rule_writes();
    let (game, _, _) = played();

    // **The scenario's own rows, not the engine's.** The game's store holds the foundation as
    // well - schema, engine and ruleset - because that is what `{load}` put there, and
    // `first_test.rs` already loads all three before any test runs. **Writing them here would
    // publish 700 lines of the engine as though they were the scenario's world**, which is what
    // the first version of this did: 709 lines, 60 of them `{primitive}`.
    //
    // **Subtracted as rows rather than as text**, because the two sides render differently -
    // the foundation names relations by id and a rendering names them by word.
    let foundation: BTreeSet<String> = foundation::rows().iter().map(write).collect();
    let mine: Vec<Row> = game
        .rows()
        .rows()
        .iter()
        .filter(|row| !foundation.contains(&write(row)))
        .cloned()
        .collect();
    assert!(
        !mine.is_empty(),
        "the scenario's world is entirely the foundation's, which cannot be"
    );

    let names = Names::of(game.rows().rows());
    let mut lines: Vec<String> = mine.iter().map(|it| names.row(it)).collect();
    lines.sort();

    let mut kept: Vec<String> = Vec::new();
    let mut dropped = 0;
    for line in &lines {
        let relation = line
            .trim()
            .trim_start_matches('{')
            .split([' ', '}'])
            .next()
            .unwrap_or_default()
            .to_string();
        if written.contains(&relation) {
            dropped += 1;
            continue;
        }
        kept.push(line.clone());
    }

    // **Both populations, because an empty file would read as a world with no structure in it**
    // and every case referring to it would then fail for the wrong reason.
    assert!(
        !kept.is_empty(),
        "no invariant rows: every relation in the scenario's world is written by some rule"
    );
    assert!(
        dropped > 0,
        "nothing was dropped, so either the ruleset writes nothing or this read it wrong"
    );

    let mut text = String::new();
    text.push_str(
        "# The scenario's world, less everything a rule can change.\n\
         #\n\
         # **Generated. Do not edit.** `scripts/regression.sh`, from `scenario/main.4x`.\n\
         #\n\
         # **Every case under `regression/scenario/` refers to this rather than repeating it** -\n\
         # `docs/process.md`, from `P-598`: *what never changes is referred to rather than\n\
         # repeated, and the reference is what the runner follows, not only what he clicks.*\n\
         # A case opens `{load file:setup.4x into:game}` and the runner follows it, which is\n\
         # what makes a case executable as a test.\n\
         #\n\
         # **Which rows are here is derived rather than listed.** A relation is invariant when\n\
         # no clause of `spec/data/rules.4x` adds, puts or removes it - so a structural\n\
         # relation added tomorrow arrives here without anybody editing a list.\n\n",
    );
    for line in &kept {
        text.push_str(&format!("{line}\n"));
    }
    text
}

/// Every column a rule leaves as it found it, because no clause of it names one.
///
/// # `P-573` chose a report over a refusal, and that shapes this
///
/// **`spec/invariants.md`**: *a rule carries through the columns it does not name. A rule acting on
/// a family acts on members that may carry columns it never mentions, and what it does not name it
/// leaves as it found it.*
///
/// **Sean rejected making the notation say so**, and his reason is the whole design here: a line
/// that always says *carry through* is a line that gets pasted, and it reads the same whether it
/// was considered or not. **So this reports and refuses nothing**, and its value is entirely that
/// he reads it - which is why it goes into `scenario/played.md`, the file he already opens, rather
/// than into one he would have to remember.
///
/// # Only a clause that builds a row
///
/// **This lane's first instrument counted seven and the answer is one.** It walked every clause,
/// and `require` and `remove` match on a **pattern** - they name what they care about and nothing
/// else, so every column they do not name reads as carried through and none of it is.
///
/// **A row is built by `add`**, which is the one role that must produce every column, and the only
/// place `Refused::Unbound` could ever have come from. Restricting to it gives `move` acting on
/// `unit`, an ark carrying `gathering`, and nothing else.
pub fn carried_through() -> Vec<(String, String, String, Vec<String>)> {
    let rows = foundation::rows();
    let of = |relation: &str| -> Vec<&Row> {
        rows.iter().filter(|it| it.relation == relation).collect()
    };
    let named = |relation: &str| -> BTreeMap<String, String> {
        of(relation)
            .iter()
            .filter_map(|row| Some((row.value("id")?.to_string(), row.value("name")?.to_string())))
            .collect()
    };
    let (relations, roles, rules) = (named("relation"), named("role"), named("rule"));

    let mut column_of: BTreeMap<String, (String, String)> = BTreeMap::new();
    let mut columns_of: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in of("column") {
        let (Some(id), Some(relation), Some(name)) =
            (row.value("id"), row.value("relation"), row.value("name"))
        else {
            continue;
        };
        column_of.insert(id.to_string(), (relation.to_string(), name.to_string()));
        columns_of
            .entry(relation.to_string())
            .or_default()
            .push(name.to_string());
    }

    let input_of: BTreeMap<String, String> = of("input")
        .iter()
        .filter_map(|row| Some((row.value("id")?.to_string(), row.value("of")?.to_string())))
        .collect();
    let acts_on: BTreeMap<String, String> = of("relation-of")
        .iter()
        .filter_map(|row| {
            Some((
                row.value("clause")?.to_string(),
                row.value("input")?.to_string(),
            ))
        })
        .collect();
    let mut members: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in of("member") {
        let (Some(kind), Some(family)) = (row.value("kind"), row.value("family")) else {
            continue;
        };
        members
            .entry(family.to_string())
            .or_default()
            .push(kind.to_string());
    }

    // **Three ways a column gets a value and all three count as naming it** - a binding from what
    // the caller wrote, a literal from what the rule says, a reading from a row an earlier clause
    // matched. `engine::row_of` reads them in that order and carries through only what none of
    // them gave.
    let mut says: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for relation in ["binding", "literal", "reading"] {
        for row in of(relation) {
            let (Some(clause), Some(column)) = (row.value("clause"), row.value("column")) else {
                continue;
            };
            if let Some((_, name)) = column_of.get(column) {
                says.entry(clause.to_string())
                    .or_default()
                    .insert(name.clone());
            }
        }
    }

    let mut out = Vec::new();
    for clause in of("clause") {
        let (Some(id), Some(rule), Some(role), Some(relation)) = (
            clause.value("id"),
            clause.value("rule"),
            clause.value("role"),
            clause.value("relation"),
        ) else {
            continue;
        };
        if roles.get(role).map(String::as_str) != Some("add") {
            continue;
        }
        let target = match acts_on.get(id).and_then(|it| input_of.get(it)) {
            Some(typed) => typed.clone(),
            None => relation.to_string(),
        };
        let Some(members) = members.get(&target) else {
            continue;
        };
        for member in members {
            let quiet: Vec<String> = columns_of
                .get(member)
                .into_iter()
                .flatten()
                .filter(|name| !says.get(id).is_some_and(|it| it.contains(*name)))
                .cloned()
                .collect();
            if quiet.is_empty() {
                continue;
            }
            out.push((
                rules.get(rule).cloned().unwrap_or_else(|| rule.to_string()),
                relations
                    .get(&target)
                    .cloned()
                    .unwrap_or_else(|| target.clone()),
                relations
                    .get(member)
                    .cloned()
                    .unwrap_or_else(|| member.clone()),
                quiet,
            ));
        }
    }
    out.sort();
    out
}
