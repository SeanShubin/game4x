//! The dump names every table and every column, including the empty ones.
//!
//! **That is the requirement, not a nicety.** Sean is reading this to find bad names:
//! *something like that is the only way I am going to be able to actually identify the
//! problems with names.* A table that disappears when it holds nothing takes its column
//! names with it, and those are exactly the ones nobody has reviewed - because a thing that
//! is never present is a thing nobody has looked at.

use std::path::PathBuf;

use game_console::{Library, Session, dump};

/// Command files, read off disk, the way `first_release.rs` reads them.
struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

/// A game played through `scenario/commands/play.4x`, which is the state the dump is written for.
fn played() -> Session {
    let files = Files(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scenario/commands"));
    let mut session = Session::new();
    for command in ["{run file:setup}", "{start}", "{run file:play}"] {
        session
            .run(command, &files)
            .unwrap_or_else(|why| panic!("`{command}` failed: {why}"));
    }
    session
}

/// Every table has its columns, and every row has one cell per column.
///
/// The second half is what stops a table drifting into meaning something else: a row longer
/// than its header renders as a table with a hidden column, and markdown will not complain.
#[test]
fn every_table_names_its_columns_and_every_row_matches_them() {
    let session = played();
    let tables = dump::tables(&session.game);

    assert!(tables.len() >= 8, "only {} tables", tables.len());
    for table in &tables {
        assert!(
            !table.columns.is_empty(),
            "{} names no columns, so its rows say nothing",
            table.name
        );
        for (at, row) in table.rows.iter().enumerate() {
            assert_eq!(
                row.len(),
                table.columns.len(),
                "{} row {at} has {} cells and {} columns",
                table.name,
                row.len(),
                table.columns.len()
            );
        }
    }
}

/// A table with nothing in it still appears, with its columns and an explicit count.
///
/// **Poison-tested by construction**: this asserts against a game in the *design* phase,
/// where almost everything is empty, so it fails if emptiness ever means omission. Asserting
/// it only against a played game would prove nothing - a played game fills most tables, and
/// the ones it leaves empty are the ones this is about.
#[test]
fn an_empty_table_is_named_rather_than_omitted() {
    let fresh = Session::new();
    let tables = dump::tables(&fresh.game);
    let empty: Vec<&str> = tables
        .iter()
        .filter(|t| t.rows.is_empty())
        .map(|t| t.name)
        .collect();
    assert!(
        !empty.is_empty(),
        "a new game should leave tables empty; if it does not, this test has stopped \
         checking what it was written for"
    );

    let text = dump::markdown(&fresh.game, "a new game");
    for table in &tables {
        assert!(
            text.contains(&format!("## {}\n", table.name)),
            "{} is missing from the document",
            table.name
        );
        for column in table.columns {
            assert!(
                text.contains(column),
                "{} does not name its column {column:?}",
                table.name
            );
        }
    }
    assert!(
        text.contains("*(empty) 0 rows*"),
        "an empty table has to say so, the way sql.html does"
    );
}

/// The dump describes the state it is given, not the latest one.
///
/// **This is what makes browsing intermediate states a loop rather than a rewrite.** Sean
/// said he *may just have to add more reporting that allows me to browse intermediate
/// states*; a dump that reached for the final state would have to be rebuilt to do it.
#[test]
fn the_dump_describes_the_state_it_is_handed() {
    let before = Session::new();
    let after = played();
    assert_ne!(
        dump::markdown(&before.game, "before"),
        dump::markdown(&after.game, "after"),
        "two moments must not render the same"
    );

    // Asserted on the cells rather than on the rendered line. This test used to look for
    // `| play |` and broke the moment the columns were padded to a fixed width - matching
    // rendered bytes made it a test of the layout, which is not what it is about.
    let phase = |session: &Session| {
        dump::tables(&session.game)
            .into_iter()
            .find(|t| t.name == "game")
            .expect("every dump has a game table")
            .rows[0][0]
            .clone()
    };
    assert_eq!(phase(&before), "design", "a new game is in design");
    assert_eq!(phase(&after), "play", "a played game is in play");
}

/// The entity view names every kind, and says plainly when it can name no columns.
///
/// **The empty case is different here and the difference is worth stating.** In the
/// normalized view the columns are written down in code, so an empty table still names them.
/// In this view a column name *is* a component name and components live on entities - so a
/// kind with no instances has no columns to show. It says that, rather than printing an
/// empty header row and implying it knew what would have been there.
#[test]
fn the_entity_view_names_every_kind_and_admits_what_it_cannot_name() {
    let session = played();
    let tables = dump::entity_tables(&session.game);
    let kinds: Vec<&str> = tables.iter().map(|t| t.kind.as_str()).collect();
    assert_eq!(kinds, ["game", "territory", "unit"]);

    // **The table is there whatever the scenario leaves in it**, which is the whole point of
    // the renderer naming what is empty.
    //
    // It has been both, three times now. It left none; `S-14` extended it through `produce
    // ark` and `launch` and it left one; `P-342` made launching produce nothing so it left
    // none again; and `S-165` found that `P-342` never said that - the release's `launch ark`
    // has always carried a `produce 1 ark above $where` row, the model fired four of its five,
    // and Sean settled it in `P-549`. **So it leaves one Ark, in orbit above territory 1.**
    //
    // **The comment above this said what to assert and the assertion said something else.**
    // *What is asserted here is the table rather than the count, because the count is a fact
    // about how far the scenario goes and the table is a fact about the renderer* - and then
    // it asserted the count, which is why this test went red on a change to the rules rather
    // than to the renderer. **It now asserts what it said it was asserting**, and the count
    // that is a fact about the scenario is named rather than required to be zero.
    //
    // `an_empty_table_is_named_rather_than_omitted` shows the renderer against a fresh game,
    // where emptiness is not an accident of coverage, and that is where the empty case lives.
    //
    // **Two since `S-174`'s second half, and the second one is what `R-6` asks for.** Its
    // *vetted when* wants an Ark launched from the *second* territory, so the scenario runs
    // five turns further and territory 2 builds its own Yard: one Ark above territory 1 from
    // turn 9 and one above territory 2 from turn 15. **The pioneer is not among them** - it
    // was consumed by founding on turn 9 - so two units is two Arks.
    let unit = tables.iter().find(|t| t.kind == "unit").expect("listed");
    assert_eq!(
        unit.rows.len(),
        2,
        "the scenario launches an Ark from each of the two territories it holds, and both are          in orbit"
    );
    let text = dump::entities_markdown(&session.game, "after");
    assert!(text.contains("## unit"), "and it has a table regardless");

    // Every row is as wide as the columns, including entities that carry only some of them.
    for table in &tables {
        for row in &table.rows {
            assert_eq!(row.len(), table.columns.len(), "{}", table.kind);
        }
    }
}

/// The HTML is the same rows as the markdown, and carries no game data of its own.
///
/// `spec/invariants.md`: *what the game is made of lives in a data file, not in code and not
/// in a presentation file.* So the assertion is not that the markup looks right - it is that
/// every name
/// in it came from the state, and that the two renderings do not disagree.
#[test]
fn the_html_carries_the_same_rows_and_names_nothing_itself() {
    let session = played();
    let sections = dump::normalized_sections(&session.game);
    let page = dump::html(&sections, "after");

    for section in &sections {
        let (name, columns, rows) = (&section.name, &section.columns, &section.rows);
        assert!(page.contains(name), "the page is missing the {name} table");
        for column in columns {
            assert!(page.contains(column), "{name} is missing column {column:?}");
        }
        assert!(
            page.contains(&format!("{} row(s)", rows.len())),
            "{name} does not say how many rows it has"
        );
    }

    // **The content follows the state, which is the claim worth testing.** A page whose
    // vocabulary came from the markup would be the same page for any game. A fresh game has
    // no territories, so nothing can have told it about grassland.
    //
    // The first version of this asserted that `ark` was absent, and `ark` is legitimately
    // there - the `unit kind` table names every kind from the model's enumeration whether
    // any exist or not, which is the whole point of that table. Asserting a name is absent
    // tests the scenario; asserting the page changes with the state tests the renderer.
    let empty = dump::html(&dump::normalized_sections(&Session::new().game), "before");
    assert_ne!(empty, page, "two states must not render the same page");
    assert!(
        page.contains("grassland"),
        "the played game has grassland territories"
    );
    assert!(
        !empty.contains("grassland"),
        "and a game with no territories cannot have learned that word from the markup"
    );
    assert!(page.contains("<!doctype html>"), "and it is a page");

    // Nothing in the frame around the tables names anything in the game.
    let head = page.split("<body>").next().expect("a page has a head");
    for word in ["grassland", "citizen", "extractor", "pioneer", "territory"] {
        assert!(
            !head.contains(word),
            "the head names {word:?}, so the markup carries game data"
        );
    }
}

/// Every kind the model knows is a table, or a value in one.
///
/// **`S-25` is why this exists, and my first version of it did not work.** `labor` is one of
/// the release's kinds and the dump had no table for it - it lived as a `labor spent` column
/// inside `territory`, so a reader looking for labor found nothing. **An absent table is the
/// one thing that cannot be told from a wrong one**: a wrong number invites a question and a
/// missing name invites none.
///
/// The first attempt searched the rendered markdown for each kind's name. It passed with the
/// labor table deleted, because `labor spent` contains the word - **so it would have passed
/// on the very defect it was written for.** A column name mentioning a kind is not the same
/// as the kind being represented, which is the whole of what `S-25` says.
///
/// So this reads the structure: a kind is named when it **is** a table, or **is** a value in
/// one. Column names deliberately do not count.
///
/// It reads the model's own enumerations rather than a list written here, so a kind added to
/// the model and forgotten in the dump fails this rather than passing quietly. It cannot
/// read the release's fourteen - the model has no such vocabulary yet, and `S-21` is where
/// it gets one - so this checks what the code knows and grows when the code does.
#[test]
fn every_kind_the_model_knows_is_a_table_or_a_value_in_one() {
    use game_model::{Resource, StructureKind, UnitKind};

    let session = played();
    let tables = dump::tables(&session.game);

    let mut named: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for table in &tables {
        named.insert(table.name.to_string());
        for row in &table.rows {
            for cell in row {
                named.insert(cell.clone());
            }
        }
    }

    let mut wanted: Vec<String> = vec!["citizen".into(), "labor".into(), "territory".into()];
    wanted.extend(Resource::ALL.iter().map(|r| r.name().to_string()));
    wanted.extend(UnitKind::ALL.iter().map(|k| k.name().to_string()));
    wanted.extend(StructureKind::ALL.iter().map(|k| k.name().to_string()));

    let missing: Vec<&String> = wanted.iter().filter(|k| !named.contains(*k)).collect();
    assert!(
        missing.is_empty(),
        "{missing:?} is a kind the model knows and the dump neither has a table for nor \
         puts in a cell. A reader looking for it finds nothing, which reads exactly like a \
         kind that does not exist. Mentioning it in a column name does not count."
    );

    // Over every case, and how many there were: an empty enumeration would pass this
    // without looking at anything.
    assert!(
        wanted.len() >= 11,
        "only {} kinds checked; the model's enumerations have probably moved",
        wanted.len()
    );
}

/// `turns.md` has a section for every `end turn` the scenario contains.
///
/// **`S-27`, and the count is the whole point.** Sean is deriving these turns by hand, and a
/// loop that stopped early would produce a file that looks finished - he would derive
/// against a truncated run and find nothing wrong with it. A wrong number invites a
/// question; a missing turn invites none.
///
/// **The boundaries are the scenario's own `end turn` lines**, because his checkpoints are
/// `play.4x`'s comments - *Turn 2. Four citizens* - and a dump that counted turns its own
/// way would be worse than no dump at all.
#[test]
fn every_turn_of_the_scenario_is_dumped() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let scenario = std::fs::read_to_string(root.join("scenario/commands/play.4x"))
        .expect("scenario/commands/play.4x is the scenario");
    let boundaries = scenario
        .lines()
        .filter(|line| line.trim() == "{end-turn}")
        .count();
    assert!(boundaries > 1, "a scenario of one turn tests nothing here");

    let turns =
        std::fs::read_to_string(root.join("reports/turns.md")).expect("turns.md is generated");
    let sections = turns.lines().filter(|l| l.starts_with("# Turn ")).count();
    assert_eq!(
        sections, boundaries,
        "scenario/commands/play.4x ends {boundaries} turns and turns.md has {sections} sections. \
         Run `cargo run -p game-console --bin dump-state`."
    );

    // Numbered from one and consecutive, so a section cannot go missing from the middle
    // while the count still adds up.
    for at in 1..=sections {
        assert!(
            turns.contains(&format!("\n# Turn {at}\n")),
            "turns.md has {sections} sections and no `Turn {at}`"
        );
    }
}

/// Every command that consumes labor is preceded by one that makes it.
///
/// **`P-232`, choice 2, and the count is what makes a partial application fail.** `work` and
/// `build` used to make their own labor; now `create labor` does, so a scenario that spends
/// labor without saying where it came from would be refused at run time - but *some* of them
/// missing would only make the scenario shorter, and a shorter scenario still passes.
///
/// **The closure test is what this guards.** `docs/process.md` asks that the definitions and
/// the commands are enough to derive the dump by hand. A `work` with no `create labor` in
/// front of it is a citizen spent with nothing in the command list saying so.
#[test]
fn every_labor_consumer_is_preceded_by_a_create_labor() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut checked = 0usize;

    for name in ["play", "spread"] {
        let text = std::fs::read_to_string(root.join(format!("scenario/commands/{name}.4x")))
            .unwrap_or_else(|why| panic!("scenario/commands/{name}.4x: {why}"));
        let lines: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect();

        let mut consumers = 0usize;
        for (at, line) in lines.iter().enumerate() {
            let wants = line.starts_with("{work ") || line.starts_with("{build-");
            if !wants {
                continue;
            }
            consumers += 1;
            checked += 1;
            let before = at.checked_sub(1).and_then(|n| lines.get(n)).copied();
            let made = before.is_some_and(|line| line.starts_with("{create-labor "));
            assert!(
                made,
                "scenario/commands/{name}.4x line {}: `{line}` spends labor and the command before it \
                 is {before:?}, which does not make any",
                at + 1
            );

            // And it makes as much as the consumer spends. **The number is a `repeat` now**
            // - `P-323` - so it is read from a named field rather than from a position, and a
            // command without one fires once.
            let repeat = |line: &str| -> u32 {
                line.split_whitespace()
                    .find_map(|word| word.strip_prefix("repeat:"))
                    .map(|value| value.trim_end_matches('}'))
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(1)
            };
            let wanted: u32 = if line.starts_with("{work ") {
                repeat(line)
            } else {
                1
            };
            let made: u32 = before.map(repeat).unwrap_or(0);
            assert_eq!(
                made,
                wanted,
                "scenario/commands/{name}.4x line {}: `{line}` spends {wanted} and the line before \
                 makes {made}",
                at + 1
            );
        }
        assert!(
            consumers > 0,
            "scenario/commands/{name}.4x spends no labor at all"
        );
    }

    // Over every case, and how many there were: a scenario that stopped spending labor
    // would satisfy every assertion above by having nothing to check.
    // **Ninety-three since `S-174`'s second half**, where it was fifty-eight: five more turns
    // of `play.4x`, in which territory 2 builds two stores of each resource, a second mine, two
    // wells and a Yard, and works them - thirty-five more commands that spend labor, each with
    // its own `create labor` in front of it. **`spread.4x` is untouched at thirteen**, which is
    // why the two are named separately rather than summed.
    //
    // **Ninety-six since turn 16**, which adds three: both territories are fed while the Ark
    // crosses in orbit. **The crossing itself is not one of the three** - `move` spends energy
    // rather than labor, so the command this turn exists for is invisible to this count, which
    // is what it means for a check to be about labor and not about turns.
    assert_eq!(
        checked, 109,
        "ninety-six labor consumers in play.4x and thirteen in spread.4x; found {checked}"
    );
}

/// The scenario fires every player recipe the release declares, counted.
///
/// **`S-14` asked for a check by count and the coverage was guarded by nothing.** It was
/// true when written and would have stayed silent the day a recipe was added - which is what
/// `CLAUDE.md` calls unverified rather than done.
///
/// **The list comes from the release, not from here.** A hand-kept list of nine would pass
/// forever; reading the Recipes table means a tenth player recipe fails this until the
/// scenario fires it.
///
/// # What is not checked, and why
///
/// The release declares fifteen recipes and six are the world's - `upkeep`, `grow`,
/// `perish`, `spoil`, `age`, `refresh`. **Three fire every turn and cannot not fire**;
/// `perish` needs a thing whose upkeep is unpaid, which this scenario deliberately never
/// reaches because it feeds everybody; and `spoil` and `age` are not separable in the model,
/// which expires all food at a turn's end rather than counting `keeps` down. So *fifteen
/// recipes fired* is not assertable here, and claiming it by counting to fifteen some other
/// way would be the coverage-guarded-by-nothing this test exists to end.
#[test]
fn the_scenario_fires_every_player_recipe_the_release_declares() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let release = std::fs::read_to_string(root.join("releases/first-release.md"))
        .expect("the release is there");
    let scenario = std::fs::read_to_string(root.join("scenario/commands/play.4x"))
        .expect("the scenario is there");

    // Which command fires which recipe. Small, and checked for completeness below - a recipe
    // the release adds and this does not name fails rather than going unnoticed.
    // **This says a command that *can* fire each recipe is present, not that each recipe
    // ran.** `tests/fired.rs` asks the model what actually happened; this is the cheaper
    // question and it fails earlier - a recipe with no command at all is a hole in the
    // console, and that is what it is for.
    //
    // **`P-323` closed the ambiguity this used to carry.** `move` and `found by land` were
    // both matched by the prefix `move `, so one `move pioneer 2` satisfied two rows and the
    // recipe `move` had never fired while the check read nine of nine - `C-21`. A command is
    // named for its recipe now, so each prefix reaches exactly one of them.
    // **The command is the recipe's name with the spaces joined** - `P-328`. So this table
    // is a dashing rather than a mapping, and a recipe the release adds fails here by having
    // no row rather than by being unreachable.
    let fired_by: [(&str, &str); 11] = [
        ("deploy ark", "{deploy-ark"),
        ("build store", "{build-store"),
        ("move", "{move "),
        ("found by land", "{found-by-land"),
        ("build extractor", "{build-extractor"),
        ("build yard", "{build-yard"),
        ("produce pioneer", "{produce-pioneer"),
        ("launch ark", "{launch-ark"),
        ("mine energy", "{mine-energy"),
        ("create labor", "{create-labor"),
        ("work", "{work "),
    ];

    // **The scenario stopped moving anything, and this says so rather than passing.**
    // `S-66` removed the Ark's move: launching is not a move since `P-342`, so the one
    // `{move ...}` in the repository went with it. **`move` is still a declared player recipe
    // with a command**, and `spec/scenarios.md` wants the main scenario to touch everything a
    // typical game uses - so this is a hole rather than a rule that stopped applying.
    //
    // **Named rather than dropped from the list**, so it fails the moment a move comes back
    // and cannot be forgotten while it is out. `C-63`, and putting one in is Sean's: he is
    // about to derive this file by hand and a command he has not been told about is a change
    // under him.
    // **Empty, and `S-76` is what emptied it.** `move` fires now, and it fires because
    // the game needs it rather than because a case was written to exercise it: founding
    // requires the pioneer to be standing on the ground, so the scenario has to cross
    // before it founds. That is the distinction `C-54` exists to name, arriving as a
    // consequence. An empty list is the claim that every declared recipe is in the
    // scenario, and the count below states it.
    // **One, and `P-489` put it there.** `refuel` is declared and no command fires it, so it
    // cannot be in the scenario - not *has not been put there*, which is what every other
    // entry this list has ever held meant. `C-112` is open to spec on what binds it: a command
    // naming the territory has to choose when two units standing there both have room, and one
    // naming the unit makes it the player's choice.
    //
    // **The assertion below is what makes this expire.** It fails the moment a line in the
    // scenario begins with the command, so the exception cannot outlive the gap it describes.
    const NOT_IN_THE_SCENARIO: [(&str, &str); 1] = [(
        "refuel",
        "no command fires it - spec/console.md names none, and `C-112` is open on the binding",
    )];

    let mut declared: Vec<String> = Vec::new();
    let mut inside = false;
    for line in release.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == "## Recipes";
            continue;
        }
        if !inside || !line.trim().starts_with("| **") {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').collect();
        let name = cells.first().unwrap_or(&"").trim().trim_matches('*').trim();
        let owner = cells.get(1).unwrap_or(&"").trim();
        if owner == "player" {
            declared.push(name.to_string());
        }
    }

    assert_eq!(
        declared.len(),
        // **Eleven since `P-552`**, which added `mine energy` - the first player recipe that acts in an orbit. Ten from `P-511` until then, when it deleted `refuel`.
        11,
        "eleven player recipes were declared since `P-552`; the release now has {} \
         ({declared:?}). If one was added, name what fires it above and make the scenario \
         fire it.",
        declared.len()
    );

    for recipe in &declared {
        // **The exception is read before the command**, because a recipe nothing fires has no
        // command to look up and the lookup below panics rather than returning. That ordering
        // was fine while every entry meant *not put in the scenario yet*; `refuel` is the
        // first that means *cannot be*.
        let excepted = NOT_IN_THE_SCENARIO
            .iter()
            .find(|(named, _)| *named == recipe.as_str());
        let named_here = fired_by.iter().find(|(name, _)| name == recipe);
        let Some((_, command)) = named_here else {
            assert!(
                excepted.is_some(),
                "nothing here says what fires `{recipe}`"
            );
            continue;
        };
        let command = *command;
        if let Some((_, why)) = excepted {
            // **An exception that has been repaired is a lie in the other direction.**
            assert!(
                !scenario
                    .lines()
                    .any(|line| line.trim().starts_with(command)),
                "`{recipe}` is in the scenario now, so delete its exception: {why}"
            );
            continue;
        }
        assert!(
            scenario
                .lines()
                .any(|line| line.trim().starts_with(command)),
            "no command in the scenario could fire `{recipe}` - looked for a line beginning \
             `{command}`"
        );
    }
}

/// Every kind is named in the dump, and only two of those namings depend on the scenario.
///
/// **`Q-65`, and the population is the finding.** This was called
/// *the_scenario_touches_every_kind*, and it does not check that. `dump.rs` builds the `kind`
/// table by unconditional pushes - one row per kind whatever the state, deliberately,
/// because *an absent table is the one thing that cannot be told from a wrong one* - and the
/// set read below takes every cell. So **eleven of the thirteen are named by construction,
/// with nothing played at all**, and the check can only ever fail for `store` and `orbit`.
///
/// **Its one real catch was `orbit`**, when `S-55` changed how a unit's place is written. That
/// is inside the two, so it says nothing about the other eleven - a catch inside the sighted
/// region is the same shape as a poison aimed there.
///
/// **The fix is the population rather than the predicate**, which is the rule this repository
/// already has for every other count. The eleven are not a defect: the `kind` table naming
/// every kind is what a reader wants, and removing it to make this check bite would break the
/// thing it was built for. What was wrong was a name claiming thirteen over a reach of two.
#[test]
fn every_kind_is_named_and_only_two_namings_depend_on_the_scenario() {
    use game_model::thing::Kind;

    // **Eighteen, and the eighteenth is `force`.** `P-399` made `readiness` a kind and
    // `P-411` made it a count carried as a trait again, so that one went the way it came;
    // `P-435` declared `force`, answering `C-93`, which asked whether one word naming a
    // trait and a thing at once was deliberate. It was - the trait is `strength` now.
    // **Nineteen since `P-494` declared `nature`.** It was a trait of a territory, and
    // making it a thing a territory holds is what took the zero test out of the force rule.
    assert_eq!(
        Kind::ALL.len(),
        19,
        "nineteen kinds: `store` from `P-260`, `deposit` from `P-322`, `adjacency` from `P-334`, `game` from `P-351`, `fertility` from the saturating rewrite, `force` from `P-435` and `nature` from `P-494`; the model has {}",
        Kind::ALL.len()
    );

    let session = played();
    let named: std::collections::BTreeSet<String> = dump::tables(&session.game)
        .iter()
        .flat_map(|table| {
            // **A table name is not the scenario touching a kind** - the quality lens, in the
            // sweep of `ba9bd41..f3dcc1e`. `dump.rs` prints every table whether or not
            // anything is in it, deliberately, so a kind named only by its own empty heading
            // satisfied a check whose name says the scenario reached it. Not live when it was
            // noticed - nothing was empty - and it becomes live the first time a kind's table
            // is. So the name is only counted when the table has a row under it.
            let heading = (!table.rows.is_empty()).then(|| table.name.to_string());
            heading
                .into_iter()
                .chain(table.rows.iter().flatten().cloned())
        })
        .collect();

    let missing: Vec<&str> = Kind::ALL
        .iter()
        .map(|kind| kind.name())
        .filter(|name| !named.contains(*name))
        .collect();
    assert!(
        missing.is_empty(),
        "the dump never names {missing:?}, so a reader looking for one finds nothing"
    );

    // **What this can fail on, asserted** - `Q-65`. Everything the `kind` table enumerates is
    // named whatever happens, so the live population is the kinds that are not, and it is two.
    // A change that made it one would be this check quietly covering less, which is precisely
    // what it did for weeks while its name said thirteen.
    let fresh = game_model::Game::new();
    let by_construction: std::collections::BTreeSet<String> = dump::tables(&fresh)
        .iter()
        .flat_map(|table| {
            let heading = (!table.rows.is_empty()).then(|| table.name.to_string());
            heading
                .into_iter()
                .chain(table.rows.iter().flatten().cloned())
        })
        .collect();
    let live: Vec<&str> = Kind::ALL
        .iter()
        .map(|kind| kind.name())
        .filter(|name| !by_construction.contains(*name))
        .collect();
    assert_eq!(
        live,
        ["store"],
        "fourteen kinds are named with nothing played, so only this one depends on the \
         scenario having reached it. **The list shrinking is coverage growing** - `orbit` \
         left it when the `kind` table gained a row for it, which is a fact about the \
         planet rather than about one run. A list that *grows* is the check covering \
         less than it did",
    );
}

/// Every turn splits what changed in two, and says so even where a half is empty.
///
/// **`S-123`, from Sean:** *double the level of detail in each turn, there is the consequences
/// of player action, and the consequence of end turn action.* The split point needs nothing
/// decided because `{end-turn}` is the last command of every turn - asserted here of the
/// emitted report rather than assumed of the loop that writes it, since the loop's structure
/// and the file's contents are two different claims.
///
/// **Both headings are written even where a half is empty.** A turn where the player changed
/// nothing and a turn whose section was not generated are different facts, and an absent
/// heading makes them the same bytes.
#[test]
fn every_turn_splits_the_player_from_the_end_of_the_turn() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let turns =
        std::fs::read_to_string(root.join("reports/turns.md")).expect("turns.md is generated");
    let sections = turns.lines().filter(|l| l.starts_with("# Turn ")).count();
    assert!(sections > 1, "a report of one turn tests nothing here");

    for (heading, what) in [
        ("## what your commands did", "the player's half"),
        ("## what `end-turn` did", "the world's half"),
    ] {
        let found = turns.matches(heading).count();
        assert_eq!(
            found, sections,
            "{what} appears {found} times and there are {sections} turns - an absent heading \
             and an empty one are different facts"
        );
    }
    assert!(
        !turns.contains("## what changed"),
        "the merged heading is still there, so the split is additive rather than a split"
    );

    // **The premise, read off the report.** Every turn's command block ends with `{end-turn}`
    // and contains exactly one, which is what makes *before the last command* a split point
    // rather than a guess.
    let blocks: Vec<&str> = turns.split("## commands\n\n```\n").skip(1).collect();
    assert_eq!(blocks.len(), sections, "one command block per turn");
    let mut checked = 0;
    for (at, block) in blocks.iter().enumerate() {
        let commands: Vec<&str> = block
            .split("```")
            .next()
            .expect("a fenced block closes")
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        assert_eq!(
            commands.last().map(|line| line.trim()),
            Some("{end-turn}"),
            "turn {} does not end with `{{end-turn}}`, so the split point is not where this \
             report says it is",
            at + 1
        );
        assert_eq!(
            commands.iter().filter(|l| l.trim() == "{end-turn}").count(),
            1,
            "turn {} has more than one `{{end-turn}}`",
            at + 1
        );
        checked += 1;
    }
    assert_eq!(checked, sections, "every turn's commands read");

    // **This said zero yesterday and the assertion caught it saying so.** When the split was
    // two halves, no turn of the scenario had an empty one, and the *Nothing changed.* wording
    // was driven directly below because a check over a population of zero proves nothing.
    // `S-125` broke the world's half into its five phases and the population stopped being
    // zero the same hour - nature reclaims rarely and expiry only bites where something is
    // over a bound, so most phases say nothing on most turns.
    //
    // **The number is asserted rather than the fact that there is one**, because *some are
    // empty* would go on passing if fifty became one.
    // **Ten since `P-494` and `P-495`.** The nature phase used to do nothing on a turn
    // when nobody lost a territory; it marks and clears a `met` on every nature every
    // turn now, and `take` consumes one on the turn a pioneer stands on ground nobody
    // has founded - so one of the eleven empty sections stopped being empty.
    //
    // **Fifteen since `S-174`'s second half, and the scenario moved rather than a rule.**
    // `R-6` wants an Ark launched from the second territory, so `play.4x` runs fifteen turns
    // where it ran ten. **Which phase is empty is what says the number is the scenario's**:
    // fourteen of the fifteen are *nature takes back what is no longer held*, one per turn
    // but turn 8, where a pioneer stood on ground nobody had founded - and the fifteenth is
    // *what expires expires*, on the one turn nothing was over a bound. **Five more turns
    // added exactly five more of the first kind**, which is a scenario running longer rather
    // than a phase falling silent.
    //
    // **Sixteen since turn 16**, and it is the same one kind again: fifteen of the sixteen are
    // now *nature takes back what is no longer held*, one per turn but turn 8. **The turn that
    // moved an Ark in orbit reclaimed nothing**, which is the phase saying so rather than the
    // count saying nothing.
    let empty = turns.matches("*Nothing changed.*").count();
    assert_eq!(
        empty, 16,
        "{empty} sections say nothing changed and this was written when 16 did. If a rule or \
         the scenario moved, that is fine - say so here rather than widening this to `> 0`"
    );
    assert!(
        empty < sections * (1 + game_model::Game::END_OF_TURN_PHASES.len()),
        "every section of every turn is empty, so the report is saying nothing at all"
    );
    assert_eq!(
        game_console::state::Disagreement::default().as_a_turn(),
        "*Nothing changed.*\n\n",
        "an empty half has to say so, and no turn of this scenario makes one"
    );
}

/// The folded section is plain HTML, which is what keeps `R-9` true.
///
/// **`R-9` - *I can browse the reports without a script running* - is built and waiting on
/// Sean.** A collapse that needed JavaScript would redden a capability he has not vetted, so
/// this asserts what the page is made of rather than trusting that nobody adds one.
#[test]
fn the_reports_fold_without_a_script() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let page = std::fs::read_to_string(root.join("reports/turns.html")).expect("turns.html");
    let turns =
        std::fs::read_to_string(root.join("reports/turns.md")).expect("turns.md is generated");
    let sections = turns.lines().filter(|l| l.starts_with("# Turn ")).count();

    assert_eq!(
        page.matches("<details").count(),
        sections,
        "one fold per turn"
    );
    assert_eq!(
        page.matches("</details>").count(),
        page.matches("<details").count(),
        "the folds are not balanced"
    );
    assert_eq!(
        page.matches("<summary>what is there now</summary>").count(),
        sections,
        "the fold does not carry the heading it replaced, so the outline reads differently \
         open and closed"
    );

    // **Every report, not only this one.** The capability is about browsing the reports, so a
    // script anywhere in the directory costs it - and a check over the one file just edited
    // would be a count over the wrong population.
    let mut looked = 0;
    for entry in std::fs::read_dir(root.join("reports")).expect("reports/ is generated") {
        let path = entry.expect("a directory entry").path();
        if path.extension().is_none_or(|it| it != "html") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("a generated page");
        assert!(
            !text.contains("<script"),
            "{} carries a script, and `R-9` says the reports browse without one",
            path.display()
        );
        looked += 1;
    }
    assert!(
        looked >= 10,
        "only {looked} pages checked; the reports directory has probably moved"
    );
}

/// The turn report's phase headings are the specification's five, read from one list.
///
/// **`S-125`, and the point is that the sections *are* the rule rather than resembling it.**
/// `spec/turn.md` ends a turn with five things; `Game::END_OF_TURN_PHASES` is that sentence,
/// and both `end_turn` and this report read it. So **a phase added to the rule adds a section**,
/// and a section with no clause behind it is a defect a person can see rather than a drift
/// nobody notices.
///
/// **Reading the names from the model rather than retyping them** is what makes that true. A
/// test carrying its own five strings would pass while the report and the rule disagreed, which
/// is the failure this whole arrangement exists to make impossible.
#[test]
fn the_turn_report_names_the_five_phases_of_ending_a_turn() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let turns =
        std::fs::read_to_string(root.join("reports/turns.md")).expect("turns.md is generated");
    let sections = turns.lines().filter(|l| l.starts_with("# Turn ")).count();
    assert!(sections > 1, "a report of one turn tests nothing here");

    let phases = game_model::Game::END_OF_TURN_PHASES;
    assert_eq!(phases.len(), 5, "`spec/turn.md` names five");

    let mut checked = 0;
    for phase in phases {
        let found = turns.matches(&format!("### {phase}\n")).count();
        assert_eq!(
            found, sections,
            "`{phase}` heads {found} sections and there are {sections} turns - every turn runs \
             every phase, and one that did nothing still gets its heading"
        );
        checked += 1;
    }
    assert_eq!(checked, phases.len(), "every phase looked for");

    // **In the rule's order, in every turn**, because five headings in the wrong sequence would
    // read as the specification and describe something else.
    for (at, turn) in turns.split("\n# Turn ").skip(1).enumerate() {
        let end_of_turn = turn
            .split("## what `end-turn` did")
            .nth(1)
            .and_then(|rest| rest.split("## what is there now").next())
            .unwrap_or_else(|| panic!("turn {} has no end-turn section", at + 1));
        let order: Vec<&str> = end_of_turn
            .lines()
            .filter_map(|line| line.strip_prefix("### "))
            .collect();
        assert_eq!(
            order,
            phases.to_vec(),
            "turn {}: the phases are not in the order `spec/turn.md` gives them",
            at + 1
        );
    }
}
