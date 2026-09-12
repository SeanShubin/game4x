//! Which recipe each command fired, and the flattened list of what actually ran.
//!
//! **`S-24`'s fourth artifact.** Sean's acceptance test for the whole reporting effort, in
//! his words: *I should be able to take the things, the recipes, the commands, and manually
//! derive the data dump. If I can do that as a human, I can be pretty sure that I can detect
//! if the game is working as I intend or not.*
//!
//! Three of the four exist - `catalog.md`, `recipes.md`, and `state.md` with `entities.md`.
//! The commands were the missing one, and they are missing in two separate ways.
//!
//! **They are not a file you can read.** `commands/setup.4x` opens with `run world`, so the
//! sequence is a hierarchy across several files and what executed is the flattening. Reading
//! `play.4x` shows you 73 lines of a run that is 700.
//!
//! **And deriving the dump by hand needs one thing nothing states: which recipe a command
//! fires.** `land ark 1` is `deploy ark`. `build extractor 1 metal` is `build extractor`.
//! `spec/console.md` lists commands, the release lists recipes, and no document connects
//! them - so a person holding all four artifacts still cannot begin. This is that
//! connection, which makes the artifact a record of the run rather than a copy of the input.
//!
//! # Every command names its recipe, and one of them did not
//!
//! `move` used to fire `move` or `found by land` depending on what was on the ground,
//! because the model looked rather than being told - so this had to read the recipe from
//! **what happened**, the target territory gaining its first citizen, rather than from the
//! words. **`P-214` split the command in two and that disambiguation is gone.** It was
//! written to be deleted and it was: a command names a recipe now, and the mapping below is
//! a lookup rather than an inference.

use game_model::{StructureKind, Transition, UnitKind};

/// The six recipes an `end turn` runs, in `spec/turn.md`'s order.
///
/// `spec/turn.md`: *everything with upkeep pays it; then a population grows on surplus food
/// or starves for want of it*, then what expires expires, and then everything becomes ready
/// again. The middle clause is quoted in this crate's outbox rather than here because it
/// carries emphasis, and `quotations.rs` compares flattened text that still has the
/// asterisks in it - so quoting it here would fail the guard for a difference in markup
/// rather than in words. Worth knowing before trusting the guard to have read everything.
///
/// **The names are the release's and the order is now stated rather than read** - `P-379`:
/// *`upkeep`, then `bear`, `breed` and `renew`, then `perish`, then `age`, then `spoil`, then
/// `stow` and `discard`, then `refresh`.*
///
/// **This used to be six names in an order nothing checked.** The sentence in `spec/turn.md`
/// named four moments against six recipes, so two of them were placed by a reading - `perish`
/// with `grow`, `age` with `spoil` - and a wrong placement would have gone unnoticed. The
/// release states the whole order now, so the order is quoted rather than inferred.
///
/// **Eleven since `P-414`**, which made force something the world musters rather than
/// something a reader computes: `muster` once per citizen where a garrison stands, `stand`
/// once per unit, and `discard` sweeping what is left at the turn's end.
///
/// **Nine since `P-399` deleted `renew`.** The token model took the whole fertile-and-spent
/// pair out: a citizen holds a readiness `for bearing`, `bear` spends it, and `refresh` puts
/// it back with every other token - so the rule that turned a spent citizen fertile again has
/// nothing left to do, and the rule that puts readiness back does its work.
///
/// **Ten, because the saturating rewrite split the three that varied.** `grow` is gone: it
/// consumed *the lesser of the surplus food and the citizens here*, which is a quantity read
/// from the state, and `P-373` says such a rule is written as a smaller one that fires as many
/// times as it can. `bear`, `breed` and `renew` are that rule, and `stow` and `discard` are
/// the same treatment of what used to be one capacity clamp.
///
/// `tests/fired.rs` holds the set against the release's own *Recipes* table, read at test
/// time, so a world recipe added or renamed fails here rather than quietly dropping out of
/// the artifact.
pub const ENDING_A_TURN: [&str; 11] = [
    "upkeep", "bear", "breed", "perish", "age", "spoil", "stow", "discard", "refresh", "muster",
    "stand",
];

/// What one command fired, if it fired anything.
pub struct Fired {
    /// The command as written, after the file that held it was flattened away.
    pub command: String,
    /// The recipes it ran, in order. Empty for a command that is not a recipe at all.
    pub recipes: Vec<&'static str>,
    /// Why it is empty, for the ones that are - so a blank cell never has to be guessed at.
    pub instead: &'static str,
    /// Which turn it ran in. Zero is before `start`, which is the design.
    pub turn: u32,
}

/// The recipe a transition fires, given the states either side of it.
///
/// **Read from the transition rather than from the words**, because the words are the
/// player's and the recipe is the game's. `build extractor 1 metal` and a later shorthand
/// for the same thing have to give the same answer, and only the transition is common to
/// both.
///
/// The design commands and `start` fire nothing, and `spec/console.md` is why rather than
/// this being an omission: `P-217` says the query commands and the design commands are
/// listed *because neither is a recipe*. `launch` fires nothing either, and that one is a
/// fact about the release rather than about the console - no recipe in it names an orbit.
pub fn fired(transition: &Transition) -> (Vec<&'static str>, &'static str) {
    match transition {
        Transition::Land { kind, .. } => match kind {
            UnitKind::Ark => (vec!["deploy ark"], ""),
            _ => (Vec::new(), "no recipe lands one of these"),
        },
        Transition::Move { .. } => (vec!["move"], ""),
        Transition::FoundByLand { .. } => (vec!["found by land"], ""),
        Transition::BuildStore { .. } => (vec!["build store"], ""),
        Transition::Build { structure, .. } => match structure {
            StructureKind::Extractor => (vec!["build extractor"], ""),
            StructureKind::Yard => (vec!["build yard"], ""),
            // The release has no recipe that builds one. A garrison arrives with a founding
            // and is manned, which `found by land` and `deploy ark` already account for.
            StructureKind::Garrison => (Vec::new(), "no recipe builds a garrison"),
        },
        Transition::Produce { kind, .. } => match kind {
            UnitKind::Pioneer => (vec!["produce pioneer"], ""),
            // An ark is produced by no recipe since `P-342`: `produce ark` became
            // `launch ark` and lost its `produce 1 ark` row. The only ark in a game is the
            // one design put in orbit.
            UnitKind::Ark => (
                Vec::new(),
                "no recipe produces an ark since `P-342` - `launch ark` consumes the cost \
                 and puts nothing into orbit",
            ),
        },
        Transition::CreateLabor { .. } => (vec!["create labor"], ""),
        Transition::Work { .. } => (vec!["work"], ""),
        Transition::EndTurn => (ENDING_A_TURN.to_vec(), ""),
        // **`P-342` gave launching a recipe, which answers the second half of `C-54`.** It
        // used to fire nothing the release declared - `produce ark` built the Ark and this
        // moved it, and no recipe named an orbit. There is one recipe now and it produces
        // nothing, so launching is not a move and nothing needs to name where it went.
        Transition::Launch { .. } => (vec!["launch ark"], ""),
        Transition::Start => (Vec::new(), "the game begins; `P-217`, not a recipe"),
        Transition::CreatePlanet { .. }
        | Transition::SetResource { .. }
        | Transition::SetForceOfNature { .. }
        | Transition::SetBiome { .. }
        | Transition::AddUnitToOrbit { .. } => (Vec::new(), "design; `P-217`, not a recipe"),
    }
}

/// Every command that actually ran, in order, with the recipe it fired.
///
/// **The flattening is the point.** `run setup` is followed here rather than recorded, so a
/// hierarchy of seven files becomes one list - which is what a person deriving the dump has
/// to work from, and what no file on disk contains.
///
/// A `run` line is not itself in the list. It is not a recipe and not a move in the game; it
/// is where the next commands are kept, and keeping it would be reporting the filing system.
pub fn ran(library: &dyn crate::Library) -> Vec<Fired> {
    let mut session = crate::Session::new();
    let mut out = Vec::new();
    for line in ["{run file:setup}", "{start}", "{run file:play}"] {
        walk(line, library, &mut session, &mut out, 0);
    }
    out
}

/// One line, and whatever it turns out to contain.
fn walk(
    line: &str,
    library: &dyn crate::Library,
    session: &mut crate::Session,
    out: &mut Vec<Fired>,
    depth: usize,
) {
    assert!(depth < 8, "`{line}` is nested deeper than any scenario is");
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return;
    }

    let grammar = crate::command_grammar();
    let Some(utterance) = command_language::parse_line(&grammar, line, 1)
        .unwrap_or_else(|why| panic!("`{line}` does not parse: {why}"))
    else {
        return;
    };
    let meaning = crate::interpret(&utterance).unwrap_or_else(|why| panic!("`{line}`: {why}"));

    match meaning {
        crate::Meaning::Run(name) => {
            let text = library
                .fetch(&name)
                .unwrap_or_else(|| panic!("`{line}` names a file that is not there"));
            for inner in text.lines() {
                walk(inner, library, session, out, depth + 1);
            }
        }
        crate::Meaning::Change(transition) => {
            let turn = session.game.turn;
            session
                .run(line, library)
                .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
            let (recipes, instead) = fired(&transition);
            out.push(Fired {
                command: line.to_string(),
                recipes,
                instead,
                // The turn it ran *in*, so an `end turn` belongs to the turn it ended
                // rather than to the one it started. Read before applying, not after.
                turn,
            });
        }
        // A query answers a question and moves nothing, so it is not part of the derivation.
        _ => {}
    }
}

/// The commands artifact, as markdown.
pub fn markdown(ran: &[Fired]) -> String {
    let mut out = String::from("# Commands\n\n");
    out.push_str("**Generated. Do not edit.** `cargo run -p game-console --bin dump-state`.\n\n");
    out.push_str(
        "Every command that ran, in order, with the recipe it fired. `S-24`: the third of \
         the four artifacts a person needs to derive the data dump by hand, the others being \
         `catalog.md`, `recipes.md` and `state.md`.\n\n",
    );
    out.push_str(
        "The hierarchy is flattened. `run setup` opens `setup.4x`, which opens others, and \
         a `run` line is not listed - it is where the next commands are kept rather than a \
         move in the game.\n\n",
    );

    let turns = ran.iter().map(|one| one.turn).max().unwrap_or(0);
    out.push_str(&format!(
        "{} commands over {turns} turn(s), and {} of them before the game began.\n\n",
        ran.len(),
        ran.iter().filter(|one| one.turn == 0).count()
    ));

    // **Padded here rather than by the hook.** `hooks/pre-commit` runs `tools/pad-tables`
    // over staged markdown, so a table written narrow is widened on the way into the commit
    // - and then the file on disk stops being what the generator produces. The currency
    // check fails on a difference nobody wrote, and regenerating unpads it again, so the two
    // take turns being wrong. Every other dump pads itself for this reason; this one did
    // not, and its first commit was red before it had finished being made.
    let columns: Vec<String> = ["#", "turn", "command", "fires"]
        .iter()
        .map(|name| name.to_string())
        .collect();
    let mut rows: Vec<Vec<String>> = Vec::new();
    for (at, one) in ran.iter().enumerate() {
        let turn = if one.turn == 0 {
            "design".to_string()
        } else {
            one.turn.to_string()
        };
        let fires = if one.recipes.is_empty() {
            format!("*{}*", one.instead)
        } else {
            one.recipes
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        rows.push(vec![
            (at + 1).to_string(),
            turn,
            format!("`{}`", one.command),
            fires,
        ]);
    }
    out.push_str(&crate::dump::padded_rows(&columns, &rows));
    out.push_str(&format!("\n{} row(s)\n", ran.len()));
    out
}
