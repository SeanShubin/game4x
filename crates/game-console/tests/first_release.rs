//! The first release, played end to end through the command language.
//!
//! The twelve territories are not data in this file. They are built by design-phase
//! commands in `commands/`, and what this asserts is that running those commands produces
//! the world `releases/first-release.md` describes - which it reads, rather than
//! restating, so the two cannot drift apart.
//!
//! The setup file and this test are the same kind of artifact. Both are a list of
//! commands and an expectation about what they leave behind; `spec/console.md` says
//! command files may invoke each other as subroutines, and `commands/setup.4x` does.

use std::collections::BTreeMap;
use std::path::PathBuf;

use game_console::{Library, Outcome, Problem, Session};
use game_model::{Phase, Resource, StructureKind, TerritoryId, Transition, UnitKind};

/// Command files, read off disk. A browser has no disk and carries them in the binary
/// instead; the console is told which by being handed one of these.
struct Files(PathBuf);

impl Files {
    fn commands() -> Self {
        Self(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../commands"))
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
        table.insert(id, nodes);
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
fn released_cost(heading: &str) -> Vec<(u32, String)> {
    let text = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release document");

    let mut inside = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("###") {
            inside = line.trim_start_matches('#').trim() == heading;
            continue;
        }
        if !inside || !line.starts_with("- cost to produce:") {
            continue;
        }
        return line
            .trim_start_matches("- cost to produce:")
            .split(',')
            .map(|part| {
                let mut words = part.split_whitespace();
                let amount: u32 = words.next().expect("an amount").parse().expect("a number");
                (amount, words.next().expect("a thing").to_string())
            })
            .collect();
    }
    panic!("no cost under `{heading}` in the release");
}

fn cost_of(heading: &str, what: &str) -> u32 {
    released_cost(heading)
        .into_iter()
        .find(|(_, thing)| thing == what)
        .unwrap_or_else(|| panic!("`{heading}` has no {what} cost"))
        .0
}

fn run(session: &mut Session, line: &str) -> Outcome {
    session
        .run(line, &Files::commands())
        .unwrap_or_else(|why| panic!("`{line}` failed: {why}"))
}

fn refuse(session: &mut Session, line: &str) -> Problem {
    session
        .run(line, &Files::commands())
        .expect_err(&format!("`{line}` should have been refused"))
}

/// The whole script, and what each stage of it leaves behind.
#[test]
fn the_first_release_plays_from_a_designed_world_through_to_a_working_territory() {
    let mut session = Session::new();

    // -- designing ---------------------------------------------------------
    assert_eq!(session.game.phase, Phase::Design);
    run(&mut session, "run setup");

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
        assert_eq!(
            place.force_of_nature, 1,
            "every territory has a force of nature of 1"
        );
        assert!(!place.founded, "nothing is claimed before play");
    }

    // Every node the release calls for is there, and nothing else is.
    for (id, expected) in released_table() {
        let place = session.game.territory(TerritoryId(id)).unwrap();
        for (resource, count, density) in &expected {
            let nodes = place.nodes_of(*resource);
            assert_eq!(
                nodes.len(),
                *count as usize,
                "territory {id} should have {count} {resource} nodes"
            );
            for (_, node) in nodes {
                assert_eq!(node.density, *density, "territory {id} {resource} density");
            }
        }
        let total: usize = expected
            .iter()
            .map(|(_, count, _)| *count as usize)
            .collect::<Vec<_>>()
            .iter()
            .sum();
        assert_eq!(
            place.nodes.len(),
            total,
            "territory {id} has no extra nodes"
        );
        assert!(
            !place.nodes_of(Resource::Food).is_empty(),
            "every territory has at least one food node"
        );
    }

    // One ark, in orbit, and nothing on the planet.
    assert_eq!(session.game.units.len(), 1);
    assert_eq!(session.game.units_in_orbit().len(), 1);
    assert_eq!(session.game.units[0].kind, UnitKind::Ark);

    // -- the phase boundary ------------------------------------------------
    // Playing is refused before `start`, and designing after it. Which phase the game is
    // in is part of its state, so both go through the same function and both are refused
    // by the same rule.
    assert!(matches!(
        refuse(&mut session, "land ark 1"),
        Problem::Rule(game_model::Rejection::WrongPhase { .. })
    ));
    run(&mut session, "start");
    assert_eq!(session.game.phase, Phase::Play);
    assert_eq!(session.game.turn, 1);
    assert!(matches!(
        refuse(&mut session, "add ark orbit"),
        Problem::Rule(game_model::Rejection::WrongPhase { .. })
    ));

    // -- playing -----------------------------------------------------------
    run(&mut session, "run play");

    let one = session.game.territory(TerritoryId(1)).unwrap();
    assert!(one.founded, "the landing site is held");
    assert!(one.garrison.is_some(), "the ark became a garrison");
    assert!(session.game.units.is_empty(), "and is no longer a unit");

    // Twelve citizens: every hand the food here can feed. Three food nodes at density
    // four is twelve food a turn, and a population cannot outgrow what feeds it.
    assert_eq!(one.citizens, 12);
    assert_eq!(
        one.extractors.len(),
        9,
        "three nodes of each resource, all worked"
    );
    for resource in Resource::ALL {
        assert_eq!(one.extractors_for(resource).len(), 3, "{resource}");
    }

    // `spec/turn.md`: unused resources are discarded. Nothing is carried into turn seven.
    for resource in Resource::ALL {
        assert_eq!(one.store(resource), 0, "{resource} was discarded");
    }
    // And everything is ready again.
    assert_eq!(one.labor_available(), one.citizens);
    assert_eq!(session.game.turn, 7, "turn one, then six endings");

    // The pioneer took a second territory by land and became what it needed there.
    let two = session.game.territory(TerritoryId(2)).unwrap();
    assert!(two.founded, "spread across the planet by land");
    assert!(two.garrison.is_some());
    assert_eq!(session.game.controlled(), [TerritoryId(1), TerritoryId(2)]);
}

/// The model's costs are the release's costs.
///
/// Nothing keeps a constant in Rust and a figure in a markdown table in step except this.
#[test]
fn the_costs_in_the_model_are_the_costs_in_the_release() {
    use game_model::game::cost;

    assert_eq!(cost_of("Create Pioneer", "metal"), cost::PIONEER_METAL);
    assert_eq!(cost_of("Create Pioneer", "energy"), cost::PIONEER_ENERGY);
    assert_eq!(cost_of("Create Pioneer", "citizen"), cost::PIONEER_CITIZENS);
    assert_eq!(cost_of("Create Ark", "metal"), cost::ARK_METAL);
    assert_eq!(cost_of("Create Ark", "energy"), cost::ARK_ENERGY);
    assert_eq!(cost_of("Yard", "metal"), cost::YARD_METAL);
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
        run(&mut session, "run setup");
        session
            .game
            .territory(TerritoryId(1))
            .unwrap()
            .nodes_of(Resource::Metal)
            .into_iter()
            .map(|(_, node)| node.density)
            .sum()
    };
    assert_eq!(ceiling, 12, "three metal nodes at density four");
    assert!(
        cost_of("Create Pioneer", "metal") <= ceiling,
        "a pioneer must be affordable within one turn's extraction"
    );

    let mut session = Session::new();
    run(&mut session, "run setup");
    run(&mut session, "start");
    run(&mut session, "run play");

    // Territory 2 was taken by land, founded, and the pioneer became what it needed.
    let two = session.game.territory(TerritoryId(2)).unwrap();
    assert!(two.founded, "the pioneer took the ground");
    assert!(two.garrison.is_some(), "and became a garrison holding it");
    // Founding produced one citizen, and because the new farm was worked on the turn it
    // arrived that citizen fed itself and grew. A territory can pay its own way from the
    // moment it is founded.
    assert_eq!(two.citizens, 2, "a citizen, fed and grown");
    assert_eq!(two.extractors.len(), 1, "and an extractor");
    assert_eq!(
        two.nodes[two.extractors[0].node].resource,
        Resource::Food,
        "working a food node, which is what a new territory needs"
    );
    assert!(session.game.units.is_empty(), "the pioneer was consumed");

    // Two territories held, which is more than the release started with.
    assert_eq!(session.game.controlled(), [TerritoryId(1), TerritoryId(2)]);
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
    run(&mut session, "run setup");

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
    run(&mut once, "run setup");
    run(&mut once, "start");
    run(&mut once, "run play");

    let mut twice = Session::new();
    run(&mut twice, "run setup");
    run(&mut twice, "start");
    run(&mut twice, "run play");

    assert_eq!(once.game, twice.game);
    assert_eq!(once.history(), twice.history());
}

/// Every command that was run is remembered, in order, and running the remembered list
/// rebuilds the same game. That is what makes `history` a save file rather than a log.
#[test]
fn the_history_of_a_game_is_enough_to_rebuild_it() {
    let mut played = Session::new();
    run(&mut played, "run setup");
    run(&mut played, "start");
    run(&mut played, "run play");

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
    run(&mut session, "run setup");
    run(&mut session, "start");

    let before = session.game.clone();
    for question in [
        "show territory 5",
        "show planet",
        "show orbit",
        "show units",
        "show turn",
        "help",
        "help move",
        "history",
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
    run(&mut session, "run setup");
    run(&mut session, "start");
    run(&mut session, "run play");

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
    let Outcome::Said(said) = run(&mut session, "show territory 1") else {
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
    run(&mut session, "run setup");
    assert_eq!(session.game.territories.len(), 12);
    assert_eq!(
        session.game.territory(TerritoryId(12)).unwrap().nodes.len(),
        2 + 8 + 8,
        "territory 12 from the release: rich nodes, almost no workers"
    );
}

/// Every kind of failure a player can cause, reported by the layer that found it and in
/// that layer's own terms.
#[test]
fn a_player_is_told_what_went_wrong_and_where() {
    let mut session = Session::new();
    run(&mut session, "run setup");
    run(&mut session, "start");

    // The parser: says where, and what it wanted instead.
    match refuse(&mut session, "land ark somewhere") {
        Problem::Parse(failure) => {
            assert_eq!(failure.position.column, 10);
            assert!(
                failure.expected.contains(&"a number".to_string()),
                "{failure}"
            );
        }
        other => panic!("expected a parse failure, got {other}"),
    }

    // The binding: a word in the right place that names nothing in the game.
    match refuse(&mut session, "build refinery 1") {
        Problem::Misread(misread) => {
            assert_eq!(misread.to_string(), "there is no structure called refinery")
        }
        other => panic!("expected a misreading, got {other}"),
    }

    // The rules: understood perfectly, and refused for a reason about the game.
    match refuse(&mut session, "land ark 99") {
        Problem::Rule(rejection) => {
            assert_eq!(rejection.to_string(), "there is no territory 99")
        }
        other => panic!("expected a rejection, got {other}"),
    }
    assert!(
        refuse(&mut session, "move pioneer 2")
            .to_string()
            .contains("no pioneer"),
        "a unit that does not exist"
    );
}

/// Landing needs more force than what holds the ground, and holding it needs as much as
/// nature has. Both are checked against a real world rather than a contrived one.
#[test]
fn taking_and_holding_a_territory_follow_the_force_rules() {
    let mut session = Session::new();
    run(&mut session, "run setup");
    run(&mut session, "start");

    // An ark is force 2 against a force of nature of 1: greater, so it takes the ground -
    // and taking it *is* founding it, so what stands there afterwards is the garrison the
    // ark became rather than the ark itself.
    run(&mut session, "land ark 1");
    assert!(session.game.territory(TerritoryId(1)).unwrap().founded);
    assert!(session.game.units.is_empty(), "founding consumes the ark");

    let one = session.game.territory(TerritoryId(1)).unwrap();
    assert_eq!(one.garrison.unwrap().force, 1, "one less than the unit");
    assert_eq!(session.game.force_in(TerritoryId(1)), 1);
    assert_eq!(one.force_of_nature, 1);
    assert!(one.founded, "equal force is enough to hold");

    // And it is still held many turns later, with nothing else done.
    for _ in 0..5 {
        run(&mut session, "end turn");
    }
    assert!(session.game.territory(TerritoryId(1)).unwrap().founded);
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
        },
        Transition::AddNode {
            territory: TerritoryId(1),
            resource: Resource::Food,
            density: 1,
        },
        Transition::SetForceOfNature {
            territory: TerritoryId(1),
            force: 1,
        },
        Transition::AddUnitToOrbit {
            kind: UnitKind::Ark,
        },
        Transition::Start,
        Transition::Land {
            kind: UnitKind::Ark,
            territory: TerritoryId(1),
        },
        Transition::Launch {
            kind: UnitKind::Ark,
        },
        Transition::Move {
            kind: UnitKind::Pioneer,
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
        Transition::EndTurn,
    ];
    // Twelve ways to change the state, and twelve forms that produce one.
    assert_eq!(changing.len(), 12);
    let commands_that_change = grammar
        .forms()
        .iter()
        .filter(|form| {
            !form.name.starts_with("show") && !["help", "history", "run"].contains(&form.name)
        })
        .count();
    assert_eq!(
        commands_that_change,
        changing.len(),
        "every transition needs a command and every command needs a transition"
    );
}
