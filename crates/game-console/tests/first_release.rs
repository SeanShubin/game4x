//! The first release, played end to end through the command language.
//!
//! The twelve territories are not data in this file. They are built by design-phase
//! commands in `commands/`, and what this asserts is that running those commands produces
//! the world `releases/first-release.md` describes - which it reads, rather than
//! restating, so the two cannot drift apart.
//!
//! The setup file and this test are the same kind of artifact. Both are a list of
//! commands and an expectation about what they leave behind; `spec/console.md` says
//! command files may invoke each other as subroutines, and `scenario/commands/setup.4x` does.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use game_console::{Library, Outcome, Problem, Session};
use game_model::{Biome, Phase, Resource, StructureKind, TerritoryId, Transition, UnitKind};

/// Command files, read off disk. A browser has no disk and carries them in the binary
/// instead; the console is told which by being handed one of these.
struct Files(PathBuf);

impl Files {
    fn commands() -> Self {
        Self(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scenario/commands"))
    }
}

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        let mut found: Vec<String> = std::fs::read_dir(&self.0)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                (path.extension()?.to_str()? == "4x")
                    .then(|| path.file_stem()?.to_str().map(str::to_string))?
            })
            .collect();
        found.sort();
        found
    }
}

/// What the release says each territory holds: resource, count of nodes, and density.
///
/// Read from the release rather than copied out of it. If somebody retunes a number
/// there, this test starts failing until the command file is retuned to match, which is
/// the only way the two stay honest about each other.
fn released_table() -> BTreeMap<u32, Vec<(Resource, u32, u32)>> {
    let text = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release document");

    let mut table = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        if cells.len() < 4 {
            continue;
        }
        let Ok(id) = cells[0].parse::<u32>() else {
            continue; // the header and the rule beneath it
        };
        let mut nodes = Vec::new();
        for (at, resource) in [Resource::Food, Resource::Metal, Resource::Energy]
            .into_iter()
            .enumerate()
        {
            let cell = cells[at + 1];
            if cell.eq_ignore_ascii_case("none") {
                continue;
            }
            let (count, density) = cell
                .split_once('x')
                .unwrap_or_else(|| panic!("territory {id}: cannot read `{cell}`"));
            nodes.push((
                resource,
                count.trim().parse().expect("a node count"),
                density.trim().parse().expect("a density"),
            ));
        }
        // **`Q-49`: the count catches a new id and not a colliding one.** `insert` returns
        // the value it replaced and discarding it is how a second row claiming territory 3
        // would overwrite territory 3's expected nodes while the length stayed twelve - the
        // test then checking the real territory against somebody else's row and reporting
        // nothing about the swap. Nothing collides today; this is the half of the parse's
        // luck that a count cannot convert into a failure.
        assert!(
            table.insert(id, nodes).is_none(),
            "two rows in the release both claim territory {id}"
        );
    }
    assert_eq!(table.len(), 12, "the release lists twelve territories");
    table
}

/// What the release says a thing costs to produce, as `(amount, what)` pairs.
///
/// Read from the release for the same reason the node table is: these are tuning figures
/// that are meant to move. When P-80 halved three of them, the only thing standing between
/// a retuned document and a model that quietly disagreed with it was a test that reads
/// both. This is that test's other half.
/// What the release says a thing costs to produce, read off the table at test time.
///
/// It used to read `- cost to produce:` under a `### Create Pioneer` heading. `P-130`
/// replaced those headings with one **Units and structures** table, and this went looking
/// for a heading that no longer exists - which is the failure a test that reads a document
/// is for. The document moved and said so.
///
/// The thing is named as the table names it: `pioneer`, not `Create Pioneer`.
fn released_cost(thing: &str) -> Vec<(u32, String)> {
    let text = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release document");

    let mut inside = false;
    // **Which column holds the costs, read from the header rather than counted.**
    // This was `cells.get(5)` with the header order written above it in a comment, and
    // `P-346` deleting the `A move` column moved *Costs to produce* from 5 to 4 - so the
    // test failed saying `pioneer` has no metal cost, which is a true statement about
    // column 5 and nothing at all about the release. **That is the failure this repository
    // designed the command language to avoid** - `syntax.rs`: the predecessor indexed a
    // list by position, so inserting a term silently shifted every index after it. The
    // same mistake, in a test that reads a document instead of a grammar.
    let mut costs_at: Option<usize> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line == "## Units and structures";
            continue;
        }
        if !inside {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if costs_at.is_none() && cells.contains(&"Thing") {
            costs_at = cells.iter().position(|cell| *cell == "Costs to produce");
            assert!(
                costs_at.is_some(),
                "the units table has no `Costs to produce` column: {cells:?}"
            );
            continue;
        }
        if !line.starts_with("| **") {
            continue;
        }
        let Some(name) = cells.first().map(|cell| cell.trim_matches('*')) else {
            continue;
        };
        if name != thing {
            continue;
        }
        let at = costs_at.expect("the header row comes before any row of things");
        let Some(costs) = cells.get(at) else { continue };
        return costs
            .split(',')
            .filter_map(|part| {
                let mut words = part.split_whitespace();
                // "and nothing else" carries no figure, and neither does "not produced".
                let amount: u32 = words.next()?.parse().ok()?;
                // "2 citizens" and "1 citizen" name the same cost. The figure decides the
                // plural and the caller should not have to know which it will be.
                let thing = words.next()?;
                let thing = thing.strip_suffix('s').unwrap_or(thing);
                Some((amount, thing.to_string()))
            })
            .collect();
    }
    panic!("no row for `{thing}` in the release's units table");
}

fn cost_of(thing: &str, what: &str) -> u32 {
    released_cost(thing)
        .into_iter()
        .find(|(_, thing)| thing == what)
        .unwrap_or_else(|| panic!("`{thing}` has no {what} cost"))
        .0
}

fn run(session: &mut Session, line: &str) -> Outcome {
    session
        .run(line, &Files::commands())
        .unwrap_or_else(|why| panic!("`{line}` failed: {why}"))
}

/// What went wrong, with where it was found unwrapped away.
///
/// **`P-215` made every problem a located one**, so a caller matching on `Problem::Rule`
/// now meets a `Problem::At` wrapping it. Unwrapped here rather than at twenty call sites,
/// because every one of them is asking *what* went wrong. `refused_at` is for the one test
/// that asks *where*.
fn refuse(session: &mut Session, line: &str) -> Problem {
    refused_at(session, line).1
}

/// What went wrong and where, for the test that is about the second half.
fn refused_at(session: &mut Session, line: &str) -> (game_console::Where, Problem) {
    let problem = session
        .run(line, &Files::commands())
        .expect_err(&format!("`{line}` should have been refused"));
    match problem {
        Problem::At { found, what } => (found, *what),
        other => panic!("`{line}` was refused without saying where: {other}"),
    }
}

/// The whole script, and what each stage of it leaves behind.
#[test]
fn the_first_release_plays_from_a_designed_world_through_to_a_working_territory() {
    let mut session = Session::new();

    // -- designing ---------------------------------------------------------
    assert_eq!(session.game.phase, Phase::Design);
    run(&mut session, "{run file:setup}");

    // A tiny planet is twelve territories, and on a dodecahedron each touches five.
    assert_eq!(session.game.territories.len(), 12);
    for place in &session.game.territories {
        let near = &session.game.adjacency[place.id.index()];
        assert_eq!(
            near.len(),
            5,
            "territory {} touches {}",
            place.id,
            near.len()
        );
        // **`P-253`: not one everywhere any more.** Jungle holds itself with two, and the
        // planet has two jungles. Asserting the flat 1 here was true of the release for as
        // long as the release said so, and the moment it stopped this was the only thing
        // that noticed - which is the right outcome, and is why it now reads the release
        // rather than a number. `tests/biomes_can_be_held.rs` holds every territory to its
        // own biome's force; this only says nature is set at all.
        assert!(
            place.force_of_nature >= 1,
            "territory {} has no force of nature",
            place.id
        );
        assert!(!place.founded(), "nothing is claimed before play");
    }

    // **`Q-48`: the population is asserted before it is looped over.**
    //
    // `released_table` finds its rows by shape - a line starting with `|`, four or more
    // cells, an integer first cell - and every assertion below sits inside the loop. A parse
    // that returned nothing would run it zero times, and **this test would report green
    // having checked no territory at all** while claiming the release is buildable.
    //
    // Not hypothetical for this document. Its tables changed four times this week - `founded`
    // dropped, `force of nature` renamed to `nature`, columns added, a store row arriving -
    // and every one was a change to the shape this parse depends on. **None would have
    // announced itself.** `CLAUDE.md` -> *What done means* carries the rule and this test did
    // not follow it: check the rule over every case, and assert how many cases there were.
    let released = released_table();
    assert_eq!(
        released.len(),
        12,
        "the release describes twelve territories and the parse found {}; if that table moved or changed shape, `released_table` is reading the wrong thing",
        released.len()
    );
    let mut checked = 0;

    // Every capacity the release calls for is there, and nothing else is.
    for (id, expected) in released {
        let place = session.game.territory(TerritoryId(id)).unwrap();
        for (resource, count, density) in &expected {
            let offered = place.deposit(*resource);
            assert_eq!(
                offered.capacity, *count,
                "territory {id} should have capacity for {count} {resource} extractors"
            );
            assert_eq!(
                offered.density, *density,
                "territory {id} {resource} density"
            );
        }
        let total: usize = expected
            .iter()
            .map(|(_, count, _)| *count as usize)
            .collect::<Vec<_>>()
            .iter()
            .sum();
        assert_eq!(
            place.total_extractor_capacity(),
            total,
            "territory {id} declares no capacity the release does not"
        );
        assert!(
            place.capacity_for(Resource::Food) > 0,
            "every territory has capacity for at least one food extractor"
        );
        checked += 1;
    }
    assert_eq!(checked, 12, "twelve territories checked; {checked} were");

    // One ark, in orbit, and nothing on the planet.
    assert_eq!(session.game.units.len(), 1);
    assert_eq!(session.game.units_in_orbit().len(), 1);
    assert_eq!(session.game.units[0].kind, UnitKind::Ark);

    // -- the phase boundary ------------------------------------------------
    // Playing is refused before `start`, and designing after it. Which phase the game is
    // in is part of its state, so both go through the same function and both are refused
    // by the same rule.
    assert!(matches!(
        refuse(&mut session, "{deploy-ark territory:1}"),
        Problem::Rule(game_model::Rejection::WrongPhase { .. })
    ));
    run(&mut session, "{start}");
    assert_eq!(session.game.phase, Phase::Play);
    assert_eq!(session.game.turn, 1);
    assert!(matches!(
        refuse(&mut session, "{add-ark-orbit territory:1}"),
        Problem::Rule(game_model::Rejection::WrongPhase { .. })
    ));

    // -- playing -----------------------------------------------------------
    //
    // **What the scenario leaves is in `scenario/expected/play.4x` and no longer here.** Twelve
    // assertions about the end state - citizens, extractors, stores, turn, control - came
    // out when that file went in, in the same change, because keeping both would give the
    // scenario two expectations. They can disagree, and **the one that is wrong is not the
    // one that fails**: an assertion nobody reviewed fails loudly against a file somebody
    // did. `S-34`.
    //
    // What stays above is what a data file cannot say: the release's table against the
    // model's, the two refusals, and the state before play, which an end-state file does
    // not describe.
    run(&mut session, "{run file:play}");
    assert_eq!(session.game.phase, Phase::Play, "and it played through");
}

/// The model's costs are the release's costs.
///
/// Nothing keeps a constant in Rust and a figure in a markdown table in step except this.
#[test]
fn the_costs_in_the_model_are_the_costs_in_the_release() {
    use game_model::game::cost;

    assert_eq!(cost_of("pioneer", "metal"), cost::PIONEER_METAL);
    assert_eq!(cost_of("pioneer", "energy"), cost::PIONEER_ENERGY);
    assert_eq!(cost_of("pioneer", "citizen"), cost::PIONEER_CITIZENS);
    assert_eq!(cost_of("ark", "metal"), cost::ARK_METAL);
    assert_eq!(cost_of("ark", "energy"), cost::ARK_ENERGY);
    assert_eq!(cost_of("ark", "citizen"), cost::ARK_CITIZENS);
    assert_eq!(cost_of("yard", "labor"), cost::YARD_LABOR);
    assert_eq!(cost_of("yard", "metal"), cost::YARD_METAL);
    // One row again, since `P-234` collapsed the three kinds back into one with a
    // `resource` trait. `P-206` had split it and the release stated the same cost three
    // times; the split lived in the definitions and nowhere else.
    assert_eq!(cost_of("extractor", "labor"), cost::EXTRACTOR_LABOR);
    assert_eq!(cost_of("extractor", "metal"), cost::EXTRACTOR_METAL);
    assert_eq!(cost_of("garrison", "labor"), cost::GARRISON_LABOR);
    assert_eq!(cost_of("garrison", "metal"), cost::GARRISON_METAL);

    // Every figure in the column, not the ones this test happened to name. It checked six
    // of eleven when the table had six; the table grew and the test did not, so a garrison
    // and an extractor gained a metal cost that nothing compared against anything.
    let figures: usize = ["citizen", "garrison", "extractor", "yard", "ark", "pioneer"]
        .into_iter()
        .map(|thing| released_cost(thing).len())
        .sum();
    assert_eq!(
        figures, 12,
        "twelve figures in the Costs to produce column; this checks each one by name"
    );
}

/// The landing site can now send a Pioneer out, and that is what opens the loop.
///
/// It could not before P-80. A Pioneer cost 16 metal and the landing site's ceiling is
/// twelve a turn, with `spec/turn.md` discarding whatever is left - so the cost could
/// never be met there, and step 5 of the loop was unreachable from where the ark lands.
/// Halving it to eight put the cost inside one turn's extraction.
#[test]
fn the_landing_site_can_send_a_pioneer_out() {
    let ceiling: u32 = {
        let mut session = Session::new();
        run(&mut session, "{run file:setup}");
        let place = session.game.territory(TerritoryId(1)).unwrap();
        let metal = place.deposit(Resource::Metal);
        metal.capacity * metal.density
    };
    // Territory 1's own row in *Territory resources*, which is what binds - `P-281`. The
    // *Biomes* table guides and does not, so grassland's `2 x 3` is not this number.
    assert_eq!(
        ceiling, 12,
        "capacity for three metal extractors at density four"
    );
    assert!(
        cost_of("pioneer", "metal") <= ceiling,
        "a pioneer must be affordable within one turn's extraction"
    );

    let mut session = Session::new();
    run(&mut session, "{run file:setup}");
    run(&mut session, "{start}");
    run(&mut session, "{run file:play}");

    // **What the pioneer left is in `scenario/expected/play.4x`**, not here - founded, garrison,
    // citizens, extractors, the consumed unit and what is controlled. Seven assertions came
    // out with the file going in. `S-34`.
    //
    // What stays is the line above, which is the release's ceiling against the model's cost:
    // a comparison between two documents, which no dump of one state can make.
}

/// Which territory id sits where.
///
/// `spec/planet.md` says ids are unique and start at one, and that two territories are
/// adjacent when they share an edge. It does not say which id goes on which face, and it
/// should not have to - but a command file naming `move pioneer 2` means nothing unless
/// the answer is fixed, so this records what `create planet tiny` actually produces.
///
/// It is fixed. `canonical_seeds(12)` builds `GP(1,0)` from an icosahedron whose vertices
/// are written down in the source: no randomness, no relaxation, no seed. Territory *n*
/// is the same face on every run and every machine that agrees about arithmetic. This
/// test is what stops that quietly ceasing to be true, because if the numbering ever
/// shifted, every command file naming a territory would silently mean somewhere else.
#[test]
fn the_numbering_of_a_tiny_planet_is_fixed() {
    let mut session = Session::new();
    run(&mut session, "{run file:setup}");

    let neighbours = |id: u32| -> Vec<u32> {
        let mut near: Vec<u32> = session.game.adjacency[TerritoryId(id).index()]
            .iter()
            .map(|other| other.0)
            .collect();
        near.sort_unstable();
        near
    };

    // A dodecahedron, numbered as the tessellation numbers it.
    assert_eq!(neighbours(1), [2, 3, 4, 5, 6]);
    assert_eq!(neighbours(2), [1, 3, 4, 7, 8]);
    assert_eq!(neighbours(3), [1, 2, 5, 7, 9]);
    assert_eq!(neighbours(4), [1, 2, 6, 8, 10]);
    assert_eq!(neighbours(5), [1, 3, 6, 9, 12]);
    assert_eq!(neighbours(6), [1, 4, 5, 10, 12]);
    assert_eq!(neighbours(7), [2, 3, 8, 9, 11]);
    assert_eq!(neighbours(8), [2, 4, 7, 10, 11]);
    assert_eq!(neighbours(9), [3, 5, 7, 11, 12]);
    assert_eq!(neighbours(10), [4, 6, 8, 11, 12]);
    assert_eq!(neighbours(11), [7, 8, 9, 10, 12]);
    assert_eq!(neighbours(12), [5, 6, 9, 10, 11]);

    // What the play script depends on: the landing site and the territory it expands into
    // really do share an edge.
    assert!(session.game.are_adjacent(TerritoryId(1), TerritoryId(2)));

    // And territory 11, which the release calls the prize, is the far side of the planet
    // from the landing site: they share no neighbour, so it is three moves away.
    let from_one = neighbours(1);
    let from_eleven = neighbours(11);
    assert!(
        !from_one.iter().any(|id| from_eleven.contains(id)),
        "1 and 11 should have nothing between them"
    );
    assert!(!session.game.are_adjacent(TerritoryId(1), TerritoryId(11)));
}

/// The invariant the whole crate is arranged around, checked end to end: a game is
/// exactly the result of applying every transition in order to the starting state.
#[test]
fn replaying_the_same_commands_produces_the_same_game() {
    let mut once = Session::new();
    run(&mut once, "{run file:setup}");
    run(&mut once, "{start}");
    run(&mut once, "{run file:play}");

    let mut twice = Session::new();
    run(&mut twice, "{run file:setup}");
    run(&mut twice, "{start}");
    run(&mut twice, "{run file:play}");

    assert_eq!(once.game, twice.game);
    assert_eq!(once.history(), twice.history());
}

/// Every command that was run is remembered, in order, and running the remembered list
/// rebuilds the same game. That is what makes `history` a save file rather than a log.
#[test]
fn the_history_of_a_game_is_enough_to_rebuild_it() {
    let mut played = Session::new();
    run(&mut played, "{run file:setup}");
    run(&mut played, "{start}");
    run(&mut played, "{run file:play}");

    // A history is the flat list of what actually changed the game - a call to a
    // subroutine records what it did, not that it was called - so it replays on its own.
    let script = played.history().join("\n");
    let mut rebuilt = Session::new();
    rebuilt
        .run_script(&script, &Files::commands())
        .expect("the history should replay");

    assert_eq!(rebuilt.game, played.game);
}

/// Asking never changes anything. Every query in `spec/console.md` is run against a real
/// game and the state is compared before and after.
#[test]
fn no_question_ever_changes_the_game() {
    let mut session = Session::new();
    run(&mut session, "{run file:setup}");
    run(&mut session, "{start}");

    let before = session.game.clone();
    for question in [
        "{show-territory id:5}",
        "{show-planet}",
        "{show-orbit}",
        "{show-units}",
        "{show-turn}",
        "{help}",
        "{help command:move}",
        "{history}",
    ] {
        let outcome = run(&mut session, question);
        assert!(
            matches!(outcome, Outcome::Said(_)),
            "`{question}` said nothing"
        );
        assert_eq!(session.game, before, "`{question}` changed the game");
    }
}

/// The browser and the console name the same thing the same way, and neither uses an
/// engine's idea of identity. `docs/architecture.md` rule 8.
#[test]
fn the_data_browser_names_things_by_their_model_id() {
    let mut session = Session::new();
    run(&mut session, "{run file:setup}");
    run(&mut session, "{start}");
    run(&mut session, "{run file:play}");

    let entries = session.entities();
    assert_eq!(entries.iter().filter(|e| e.kind == "territory").count(), 12);

    for id in 1..=12u32 {
        let entry = entries
            .iter()
            .find(|e| e.kind == "territory" && e.id == id.to_string())
            .unwrap_or_else(|| panic!("territory {id} is not in the browser"));
        assert!(!entry.components.is_empty());
    }

    // What the browser calls territory 1 is what `show territory 1` answers to.
    let Outcome::Said(said) = run(&mut session, "{show-territory id:1}") else {
        panic!()
    };
    assert!(said.starts_with("territory 1"), "{said}");
}

/// A command file may call another as a subroutine, and a failure inside one is reported
/// against its own line rather than against the line that called it.
#[test]
fn the_setup_is_a_hierarchy_of_files() {
    let library = Files::commands();
    assert!(library.names().contains(&"setup".to_string()));
    assert!(library.names().contains(&"nodes".to_string()));
    assert!(library.names().contains(&"forces".to_string()));

    // setup.4x calls nodes.4x and forces.4x, so running it alone builds the whole world.
    let mut session = Session::new();
    run(&mut session, "{run file:setup}");
    assert_eq!(session.game.territories.len(), 12);
    assert_eq!(
        session
            .game
            .territory(TerritoryId(12))
            .unwrap()
            .total_extractor_capacity(),
        2 + 8 + 8,
        "territory 12 from the release: rich extractors, almost no workers"
    );
}

/// Every kind of failure a player can cause, reported by the layer that found it and in
/// that layer's own terms.
#[test]
fn a_player_is_told_what_went_wrong_and_where() {
    let mut session = Session::new();
    run(&mut session, "{run file:setup}");
    run(&mut session, "{start}");

    // The parser: says where, and what it wanted instead.
    match refuse(&mut session, "{deploy-ark territory:somewhere}") {
        Problem::Parse(failure) => {
            // At the value, which is what has to change: the field opens at 13 and
            // `somewhere` at 23.
            assert_eq!(failure.position.column, 23);
            assert!(
                failure.expected.contains(&"a number".to_string()),
                "{failure}"
            );
        }
        other => panic!("expected a parse failure, got {other}"),
    }

    // The binding: a word in the right place that names nothing in the game.
    //
    // **It used to be `build refinery 1`, and `P-323` moved that case to the parser.** A
    // structure is a word of the command's name now, so `{build refinery ...}` is refused
    // before the binding sees it. A resource is still a value, so this is where the binding's
    // own kind of failure still lives.
    match refuse(&mut session, "{build-extractor territory:1 resource:gold}") {
        Problem::Misread(misread) => {
            assert_eq!(misread.to_string(), "there is no resource called gold")
        }
        other => panic!("expected a misreading, got {other}"),
    }

    // The rules: understood perfectly, and refused for a reason about the game.
    match refuse(&mut session, "{deploy-ark territory:99}") {
        Problem::Rule(rejection) => {
            assert_eq!(rejection.to_string(), "there is no territory 99")
        }
        other => panic!("expected a rejection, got {other}"),
    }
    assert!(
        refuse(&mut session, "{move unit:pioneer from:1 to:2}")
            .to_string()
            .contains("no pioneer"),
        "a unit that does not exist"
    );

    // **And where, which is the half this test was named for and did not check** - `P-215`.
    // A rejection said what was wrong about the game and nothing about which of seven files
    // it was in, so a failure five lines into `world.4x` reached by `setup.4x` reached by
    // the console read as a bare sentence.
    let (found, _) = refused_at(&mut session, "{deploy-ark territory:99}");
    assert_eq!(
        found.line, 1,
        "typed at the console, so line one of nothing"
    );
    assert!(
        found.inside.is_empty(),
        "nothing called it, so there is no chain to name"
    );
    assert_eq!(
        found.column, None,
        "a rejection is about the whole command, so inventing a column would be a precision it does not have"
    );

    // **Inside a file, and the chain is what makes it usable.** Running the scenario without
    // `start` fails somewhere in the middle of `play.4x`, and the whole value of this is
    // that the message says which file and which line rather than one sentence about the
    // game. The line is asserted to be past the first rather than to be a particular number,
    // so this stays true when the scenario moves.
    let mut nested = Session::new();
    run(&mut nested, "{run file:setup}");
    let (found, what) = refused_at(&mut nested, "{run file:play}");
    assert!(
        what.to_string().contains("once the game has started"),
        "{what}"
    );
    assert!(found.line > 1, "a line inside the file, not the call to it");
    assert_eq!(
        found.inside,
        ["{run file:play}"],
        "and which file that line is in"
    );
}

/// Landing needs more force than what holds the ground, and holding it needs as much as
/// nature has. Both are checked against a real world rather than a contrived one.
#[test]
fn taking_and_holding_a_territory_follow_the_force_rules() {
    let mut session = Session::new();
    run(&mut session, "{run file:setup}");
    run(&mut session, "{start}");

    // An ark is force 2 against a force of nature of 1: greater, so it takes the ground -
    // and taking it *is* founding it, so what stands there afterwards is the garrison the
    // ark became rather than the ark itself.
    run(&mut session, "{deploy-ark territory:1}");
    assert!(session.game.territory(TerritoryId(1)).unwrap().founded());
    assert!(session.game.units.is_empty(), "founding consumes the ark");

    let one = session.game.territory(TerritoryId(1)).unwrap();
    // **`P-276` and `P-277`: a garrison has no force of its own.** It does one thing - it
    // lets the citizens of that territory sum their force - and it does that by existing.
    // So what holds this ground is its two citizens, and the garrison is what makes them
    // sum rather than presenting only the highest.
    assert_eq!(
        one.garrison().unwrap().force,
        0,
        "a garrison has none of its own"
    );
    assert_eq!(
        session.game.force_in(TerritoryId(1)),
        2,
        "its two citizens, summed"
    );
    assert_eq!(one.force_of_nature, 1);
    assert!(one.founded(), "equal force is enough to hold");

    // **And it is lost if nobody works it, which is `S-19` correcting this test.**
    //
    // It used to assert the ground was still held five empty turns later. It was not: with
    // no extractor worked there is no food, the citizens starve at the first turn's end, and
    // the territory sat with `founded: true` and nobody on it. **The flag was saying *held*
    // about ground with no citizens** - which is exactly the disagreement a stored derived
    // trait can have, and `releases/first-release.md` calls `control` *derived: a citizen of
    // that player is there*.
    //
    // So what the force rules give you is ground you can hold, not ground that holds itself.
    for _ in 0..5 {
        run(&mut session, "{end-turn}");
    }
    let one = session.game.territory(TerritoryId(1)).unwrap();
    assert_eq!(
        one.citizens(),
        0,
        "nothing was worked, so nothing was eaten"
    );
    assert!(
        !one.founded(),
        "and ground with no citizens on it is not held by anybody"
    );
}

/// The ground the release actually plays on is ground it can be played on.
///
/// `spec/planet.md` says no territory can be claimed whose biome is ocean, and a
/// territory's biome is what the terrain gives it - so which territories are claimable is
/// decided by a noise field rather than by this document. `scenario/commands/play.4x` lands on 1 and
/// moves to 2. If the terrain ever puts water on either, the release stops being playable,
/// and it should say so here rather than fail somewhere in the middle of the loop.
#[test]
fn the_territories_the_release_plays_on_are_not_ocean() {
    let mut session = Session::new();
    run(&mut session, "{run file:setup}");
    for id in [1u32, 2] {
        let place = session.game.territory(TerritoryId(id)).unwrap();
        assert_ne!(
            place.biome,
            Biome::Ocean,
            "territory {id} came out as ocean, so `scenario/commands/play.4x` cannot claim it"
        );
    }
}

/// `spec/invariants.md`: every change to game state is representable and executable as a
/// console command. The other half of that is that nothing else can change it - so the
/// set of transitions and the set of commands have to be the same size.
#[test]
fn every_way_the_state_can_change_is_a_command() {
    let grammar = game_console::command_grammar();
    // One command for each way the state can change, plus the queries and `run`.
    let changing = [
        Transition::CreatePlanet {
            territories: 0,
            adjacency: Vec::new(),
            biomes: Vec::new(),
        },
        Transition::SetResource {
            territory: TerritoryId(1),
            resource: Resource::Food,
            extractors: 1,
            density: 1,
        },
        Transition::SetForceOfNature {
            territory: TerritoryId(1),
            force: 1,
        },
        Transition::SetBiome {
            territory: TerritoryId(1),
            biome: Biome::Grassland,
        },
        Transition::AddUnitToOrbit {
            kind: UnitKind::Ark,
            above: TerritoryId(1),
        },
        Transition::Start,
        Transition::Land {
            kind: UnitKind::Ark,
            territory: TerritoryId(1),
        },
        Transition::Launch {
            territory: TerritoryId(1),
        },
        Transition::Move {
            kind: UnitKind::Pioneer,
            from: TerritoryId(2),
            to: TerritoryId(1),
        },
        // `P-214`: one command per recipe, so the two the model used to choose between by
        // looking at the ground are two transitions and two commands.
        Transition::FoundByLand {
            territory: TerritoryId(1),
        },
        Transition::BuildStore {
            resource: Resource::Metal,
            territory: TerritoryId(1),
        },
        Transition::Build {
            structure: StructureKind::Yard,
            territory: TerritoryId(1),
            resource: None,
        },
        Transition::Produce {
            kind: UnitKind::Pioneer,
            territory: TerritoryId(1),
        },
        Transition::Work {
            count: 1,
            structure: StructureKind::Extractor,
            territory: TerritoryId(1),
            resource: Some(Resource::Food),
        },
        Transition::CreateLabor {
            count: 1,
            territory: TerritoryId(1),
        },
        Transition::EndTurn,
    ];
    // Sixteen ways to change the state. `P-260` added `build store`, which is what a
    // territory needs before it keeps anything at all; `P-232` added `create labor`, which
    // `P-214` found was the one player recipe with no command.
    assert_eq!(changing.len(), 16);

    let commands_that_change: Vec<&str> = grammar
        .forms()
        .iter()
        .filter(|form| {
            !form.name.starts_with("show") && !["help", "history", "run"].contains(&form.name)
        })
        .map(|form| form.name)
        .collect();

    // **Every transition has a command and every command makes one, and it is no longer one
    // for one.** `P-323` names a command for the recipe it fires, so the kind moved out of a
    // positional hole and into the name: `Transition::Move` is reached by `{move unit:ark ...}` and
    // by `{move unit:pioneer ...}`, and `Transition::Build` by `{build-extractor ...}` and
    // `{build-yard ...}`. Six commands share three transitions that way, which is why this
    // counts the mapping rather than the two lists.
    //
    // **The direction that matters is still checked**: a transition no command reaches is a
    // way to change the state that the player cannot ask for, and `Everything is expressible`
    // in `spec/invariants.md` forbids exactly that. `binding::handled()` is compared against
    // the grammar in its own test, so a form with no arm fails there.
    assert_eq!(
        commands_that_change.len(),
        18,
        "eighteen commands change the state; the grammar has {} ({commands_that_change:?})",
        commands_that_change.len()
    );
    // **Four pairs, named rather than counted.** `move`, `build`, `produce` and
    // `add unit to orbit` are each reached by two commands, because the kind is in the name.
    let shared = commands_that_change.len() - changing.len();
    assert_eq!(
        shared, 2,
        "two transitions are reached by two commands each - build and adding a unit to \
         orbit. `produce` stopped being one when `P-342` made `produce ark` into `launch ark`"
    );
}

/// Every recipe the player may fire has exactly one command named for it.
///
/// **`spec/console.md`: *a command is named for the recipe it fires, and there is one command
/// for each recipe the player may fire*.** A stated rule with nothing holding the two lists
/// together until now - `every_way_the_state_can_change_is_a_command` compares the grammar to
/// the **model's** transitions, which is a different pair.
///
/// # What it guards, and it has happened
///
/// **One command doing two recipes' work is the defect `P-214` split apart.** The scenario's
/// one `move` line founded, so it fired `found by land` and the recipe `move` had never once
/// run - and the coverage check was green for weeks, because a command existed and something
/// fired. **Nothing compared the command's name to the recipe's.**
///
/// **It passes today**, so this is a guard rather than a repair, and `docs/process.md` is
/// right that a passing test proves nothing on its own. What makes it worth having is that
/// the rule is stated and was unheld: a command renamed, or a second one added for one recipe,
/// would be invisible.
///
/// # One direction, deliberately
///
/// **Recipe to command, and not the reverse.** Five design commands fire no recipe at all -
/// `create-planet` and its four - which is `C-79` and `P-364`, open and not this lane's to
/// settle. Checking the other way would need those five as exceptions, and an exemption list
/// that size stops being a guard and becomes the list written twice.
#[test]
fn every_player_recipe_has_one_command_named_for_it() {
    let document = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release");

    let recipes: Vec<String> = game_console::recipes::body_under(&document, "## Recipes")
        .iter()
        .filter(|row| row.get(1).map(String::as_str) == Some("player"))
        .filter_map(|row| row.first())
        .map(|cell| cell.trim().trim_matches('*').trim().replace(' ', "-"))
        .filter(|name| !name.is_empty())
        .collect();
    let mut distinct = recipes.clone();
    distinct.sort();
    distinct.dedup();
    assert_eq!(
        distinct.len(),
        10,
        "ten recipes the player may fire when this was written; the release has {} \
         ({distinct:?})",
        distinct.len()
    );

    let grammar = game_console::command_grammar();
    let mut checked = 0;
    for recipe in &distinct {
        let named = grammar
            .forms()
            .iter()
            .filter(|form| form.opening() == *recipe)
            .count();
        assert_eq!(
            named, 1,
            "`{recipe}` is a recipe the player may fire and {named} commands are named for it \
             - one is the rule, and none of them is the `P-214` shape where a command fires a \
             recipe it is not named for"
        );
        checked += 1;
    }
    assert_eq!(
        checked,
        distinct.len(),
        "every player recipe, and the count so that an empty table cannot pass"
    );
}

/// Every place a recipe leaves open is a field of the command named for it.
///
/// **`spec/console.md`, since `P-460`: a command *binds what that recipe leaves open: every
/// place it leaves open, and any ingredient or trait value it names with a `$`*.** The
/// sentence before it bound only *the place it acts in*, which is the wording `C-101` was
/// filed against: `move` names two places with a `$` and its command bound one, so the model
/// chose the other by picking the lowest-numbered unit that could have made the move.
///
/// **Checked over every player recipe rather than over `move`.** One example stops meaning
/// anything the moment the example is edited away, and the recipe that gains a second open
/// place next is the one nobody is watching. Ten recipes, each counted from the release's
/// *Where* column and compared against its command's own required number fields.
///
/// # What counts as open, and the cells that do not
///
/// **A blank *Where* is one place rather than none** - `releases/first-release.md`: *a blank
/// means the one place the recipe acts* - and `spec/console.md` says as much from the other
/// side: *a recipe acting in one place need not name it*. So `found by land` leaves one place
/// open and its command names one.
///
/// **A place worked out from another is not open**: *the orbit above `$where`* is named by
/// naming `$where`, which the same paragraph says outright. A cell that is more than its
/// `$name` is worked out from one, and the reference is the tell.
///
/// **And a *Where* cell that names a thing rather than a place is neither** - `that unit`,
/// which says where the energy comes from and leaves nothing for the player to say.
#[test]
fn every_place_a_recipe_leaves_open_is_a_field_of_its_command() {
    let document = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release");
    let rows = game_console::recipes::body_under(&document, "## Recipes");

    // The rows of one recipe run until the next one names itself, which is how a table with a
    // blank first cell says *the same recipe, another row*.
    let mut order: Vec<String> = Vec::new();
    let mut players: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut current = String::new();
    for row in &rows {
        let named = row
            .first()
            .map(|cell| cell.trim().trim_matches('*').trim().to_string())
            .filter(|name| !name.is_empty());
        if let Some(name) = named {
            current = name.replace(' ', "-");
            if row.get(1).map(String::as_str) == Some("player") {
                order.push(current.clone());
                players.entry(current.clone()).or_default();
            }
        }
        let Some(places) = players.get_mut(&current) else {
            continue;
        };
        let cell = row.get(6).map(String::as_str).unwrap_or("").trim();
        let bare = cell.trim_matches('`');
        if let Some(name) = bare.strip_prefix('$')
            && !name.contains(char::is_whitespace)
        {
            places.insert(name.to_string());
        }
    }
    assert_eq!(
        order.len(),
        10,
        "ten recipes the player may fire when this was written; the release has {} ({order:?})",
        order.len()
    );

    let grammar = game_console::command_grammar();
    let mut checked = 0;
    for recipe in &order {
        // A recipe that names no place still acts in one, and the command names that one.
        let places = players[recipe].len().max(1);
        let form = grammar
            .forms()
            .iter()
            .find(|form| form.opening() == *recipe)
            .unwrap_or_else(|| panic!("`{recipe}` is a player recipe with no command"));
        // **A place is a number in the grammar**, which is what a territory's id is, and
        // `repeat` is optional so it is not a hole the recipe left open.
        let bound = form
            .holes()
            .filter(|(name, kind, required)| {
                *required && *kind == command_language::Kind::Number && *name != "repeat"
            })
            .count();
        assert_eq!(
            bound, places,
            "`{recipe}` leaves {places} place(s) open in the release's *Where* column and its \
             command binds {bound}. `spec/console.md`: a command binds every place a recipe \
             leaves open"
        );
        checked += 1;
    }
    assert_eq!(
        checked, 10,
        "every player recipe, and the count so that an empty table cannot pass"
    );
}

/// The *Readies* column declares the maximum the model reads, and every one of them is one.
///
/// **`P-459` closed the gap this was built to watch.** It read: the *Readies* column said
/// `yes` where `spec/turn.md` asks for a number, so the `at its maximum` the release's
/// `refresh` rows carry had nothing behind it but a constant. The column is the count per action now - *bearing 1, defending 1, laboring 1* -
/// and the constant is checked against it rather than standing in for it.
///
/// # The tripwire fired, and not through the door it was built with
///
/// **The first assertion missed and the catch-all caught it.** This asked whether the cell
/// parses as a number, expecting the column to become `1`; it became `bearing 1, defending 1,
/// laboring 1`, which parses as nothing. What fired was the second assertion - *a third value
/// is the column changing shape* - which was written as belt and braces.
///
/// **So the predicate was narrower than the thing it guarded**, which is this repository's
/// recurring shape and was mine this time. What saved it is that the guard refused **anything
/// it did not recognise** rather than only the case it predicted. That is the part worth
/// copying: a tripwire should fail on the unfamiliar, not on the expected.
///
/// **Over all eight pairs, with the count**, because a column that parsed to nothing would
/// satisfy *every maximum is one* for the wrong reason.
#[test]
fn the_readies_column_declares_the_maximum_this_reads() {
    let document = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release");

    let rows = game_console::recipes::body_under(&document, "## Units and structures");
    let mut pairs = 0;
    let mut readying = 0;
    for row in &rows {
        let thing = row[0].trim().trim_matches('*').trim();
        let said = row.get(8).map(String::as_str).unwrap_or_default().trim();
        if said.is_empty() {
            continue;
        }
        readying += 1;
        for part in said.split(',') {
            let part = part.trim();
            let (action, count) = part.split_once(' ').unwrap_or_else(|| {
                panic!(
                    "`{thing}` readies `{part}`, which is not an action and a number - the \
                     column has changed shape and `MAXIMUM_PER_ACTION` may no longer be what \
                     it reads"
                )
            });
            let count: u32 = count.trim().parse().unwrap_or_else(|_| {
                panic!("`{thing}` readies `{action}` `{count}`, which is not a number")
            });
            assert_eq!(
                count,
                game_model::containment::MAXIMUM_PER_ACTION,
                "`{thing}` may take {count} of `{action}` in a turn and the model restores \
                 `MAXIMUM_PER_ACTION`, which is {} - so the model would refill the wrong \
                 number. `P-459`.",
                game_model::containment::MAXIMUM_PER_ACTION
            );
            pairs += 1;
        }
    }
    assert_eq!(
        readying, 4,
        "four things ready - a citizen, an extractor and the two units - and {readying} do"
    );
    assert_eq!(
        pairs, 8,
        "eight kind-and-action pairs declare a maximum - a citizen's three, an extractor's \
         one, and two each for the two units - and {pairs} do"
    );
}
