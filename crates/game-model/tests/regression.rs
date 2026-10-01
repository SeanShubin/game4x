//! A generated given/when/then for every command of the main scenario.
//!
//! **Sean, 2026-09-27**: *I want to generate a detailed regression test out of `scenario/main.4x`.
//! For every single command, I want a generated given/when/then test that focuses in that single
//! command... This is an excessive amount of detail, but I won't be looking at all of them, when
//! something changes I will know exactly what changed.*
//!
//! # The pattern is his and it is written down
//!
//! **`docs/process.md`**: *When I change my mind, I delete the expected data and run the scenario
//! again. **Absent expected data means I accept what it does now**, so the test writes it, and what
//! I review is the diff in version control. **Nothing else may write it**: an expectation that can
//! be edited in place is one that can be edited by accident, which is the thing it exists to
//! prevent.*
//!
//! So this does exactly three things and no fourth:
//!
//! ```text
//! absent      write it, and say so. Deleting a file is how he says "this is right now"
//! present     compare, and fail with the first line that differs
//! ever        never overwrite one that is there
//! ```
//!
//! **That last line is the whole of the guarantee.** `scenario/played.md` is a report and is
//! rewritten every run; these are expectations and are not. The two look alike and are opposites.
//!
//! # What is in each case, and the shape that was tried first
//!
//! **`{given}` is what the command took and `{then}` is what it made** - the engine's own account
//! of that one command, and nothing else.
//!
//! **The whole world was the first shape and it was wrong**, measured rather than argued: one
//! density changed from six to seven and **all thirty-four files moved**, because every case
//! carried a `{given}` it had merely inherited from the commands before it. Sean asked for this so
//! that *when something changes I will know exactly what changed*, and thirty-four files is not
//! knowing.
//!
//! **What is lost is that a case is no longer a world that could be run on its own.** That is the
//! trade and it is worth saying: `scenario/played.md` is where the whole world at each turn's end
//! lives, so nothing is unavailable - it is somewhere else.

use std::collections::{BTreeMap, BTreeSet};

#[path = "../examples/scenario.rs"]
#[allow(dead_code)]
mod scenario;

use scenario::Case;

/// **Every command's case is on disk and says what the scenario does now.**
///
/// **It writes what is missing and compares what is there**, which is `docs/process.md`'s pattern
/// and is why this is one test rather than a generator plus a check: the writing only ever happens
/// where Sean has deleted something, so the two halves cannot get out of step.
#[test]
fn every_command_has_an_expectation_and_it_is_current() {
    let cases = scenario::regression_cases();
    let at = scenario::regression_at();
    std::fs::create_dir_all(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));

    // **The rows every case refers to, written always rather than delete-to-accept** - `P-598`
    // and `S-234`. **It is an input and not an expectation**: a case says what the command did,
    // and this says what the world was made of before anything ran. Nothing here is a claim
    // about behaviour, so there is nothing for Sean to accept - the diff in version control is
    // the whole of the review, the way `scenario/played.md` is.
    //
    // **Which rows are in it is derived from the ruleset**, not listed: a relation is invariant
    // when no clause of `spec/data/rules.4x` adds, puts or removes it.
    let setup = at.join("world.4x");
    let said = scenario::invariant_rows();
    if std::fs::read_to_string(&setup).unwrap_or_default() != said {
        std::fs::write(&setup, &said).unwrap_or_else(|why| panic!("{}: {why}", setup.display()));
    }

    let (mut written, mut compared) = (Vec::new(), 0);
    // **The name travels beside the diff**, because the message below prints the deletion for
    // each grain and a deletion is composed from the path. Formatting it into one string first
    // and parsing it back out is the shape `S-195` is about.
    let mut stale: Vec<(String, String)> = Vec::new();
    for Case {
        name,
        text: produced,
        ..
    } in &cases
    {
        let path = at.join(name);
        match std::fs::read_to_string(&path) {
            // **Present: compared and never rewritten.** A difference is reported with the line, so
            // the message says what moved rather than that something did.
            Ok(committed) => {
                compared += 1;
                if committed != *produced {
                    let differs = committed
                        .lines()
                        .zip(produced.lines())
                        .enumerate()
                        .find(|(_, (was, now))| was != now);
                    let said = match differs {
                        Some((line, (was, now))) => format!(
                            "{name} line {}:\n      was  {was}\n      now  {now}",
                            line + 1
                        ),
                        None => format!(
                            "{name}: {} line(s) committed, {} produced",
                            committed.lines().count(),
                            produced.lines().count()
                        ),
                    };
                    stale.push((name.clone(), said));
                }
            }
            // **Absent: accepted.** His words - *absent expected data means I accept what it does
            // now, so the test writes it, and what I review is the diff in version control.*
            Err(_) => {
                // **The turn's directory is made if it is not there**, because a case's name is
                // now `NN/MM-rule.4x` and deleting a whole turn takes the directory with it.
                if let Some(turn) = path.parent() {
                    std::fs::create_dir_all(turn)
                        .unwrap_or_else(|why| panic!("{}: {why}", turn.display()));
                }
                std::fs::write(&path, produced)
                    .unwrap_or_else(|why| panic!("{}: {why}", path.display()));
                written.push(name.clone());
            }
        }
    }

    if !written.is_empty() {
        println!(
            "wrote {} expectation(s) that were absent; read the diff: {written:?}",
            written.len()
        );
    }

    // **The gesture the message is asking for is printed rather than described** - `S-206`, and
    // `D-6`: *when a failure names stale cases it prints the deletion for each grain, so I paste
    // it rather than compose it.* Sean, 2026-09-27: *I need it to be very clear to distinguish
    // between them so that I delete the correct directory.*
    //
    // **Three grains, because the suite has three.** One case, one turn, the whole suite. The
    // turn grain is listed only for turns that actually hold a stale case, so pasting the whole
    // block never accepts a turn nothing moved in.
    //
    // **Grouped by turn as well**, which `S-206` offered and nobody asked for: a rule changing
    // moves several turns at once, and a flat list of twelve is how somebody deletes more than
    // they meant.
    if !stale.is_empty() {
        let mut by_turn: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        for (name, said) in &stale {
            let turn = name.split('/').next().unwrap_or_default().to_string();
            by_turn.entry(turn).or_default().push(said);
        }
        let mut out = String::new();
        for (turn, said) in &by_turn {
            out.push_str(&format!("\n  turn {turn}\n"));
            for one in said {
                out.push_str(&format!("    {one}\n"));
            }
        }
        out.push_str("\n  accept one case:\n");
        for (name, _) in &stale {
            out.push_str(&format!("    Remove-Item regression/scenario/{name}\n"));
        }
        out.push_str("\n  accept a whole turn:\n");
        for turn in by_turn.keys() {
            out.push_str(&format!(
                "    Remove-Item -Recurse regression/scenario/{turn}\n"
            ));
        }
        out.push_str("\n  accept the whole suite:\n    Remove-Item -Recurse regression/scenario\n");
        panic!(
            "{} of {} expectation(s) no longer say what the scenario does.\n\
             **Delete what you are happy with, run again, and read the diff in version \
             control.**\n{out}",
            stale.len(),
            cases.len()
        );
    }

    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. On a fresh
    // checkout everything is written and nothing compared, which is correct and is not evidence.
    assert!(
        compared + written.len() == cases.len(),
        "every case was either compared or written"
    );

    // **A directory per turn, one level deep, and nothing else.** Sean, 2026-09-27: *what about
    // each turn being in a separate directory with a numeric prefix, and each file within a turns
    // directory having a numeric prefix.*
    //
    // # This asserted the opposite until today, and the constraint it held is his too
    //
    // **Sean, 2026-09-27, earlier the same day**: *if I am deleting multiple files rather than
    // one, they need to be in a single directory.* **That reading has not gone away** - it has
    // been traded. Accepting one turn's behaviour is now the contents of one directory, which is
    // better for it; accepting one case in each of five turns is five directories, which is
    // worse. **The gesture that made it worth trading is the one he made**: deleting the whole
    // suite, which under this is five directories rather than thirty-five files.
    //
    // **Checked both ways, because a name and a directory entry are different populations.** Every
    // case is named `NN/...` exactly one level down, and every entry at the top is a directory.
    let mut depths = 0;
    for Case { name, .. } in &cases {
        assert!(
            !name.contains('\\'),
            "`{name}` is written with a backslash, and a case name is a path with `/`"
        );
        assert_eq!(
            name.matches('/').count(),
            1,
            "`{name}` is not exactly one level down, so it is not a turn's directory"
        );
        depths += 1;
    }
    assert_eq!(
        depths,
        cases.len(),
        "every case was asked how deep it sits, and the count is what says so"
    );
    // **`setup.4x` is the one loose file and it is not a case** - `S-234`. Every case belongs
    // to a turn; the rows they all refer to belong to none of them, which is the whole reason
    // they are referred to rather than repeated.
    let loose: Vec<String> = std::fs::read_dir(&at)
        .unwrap_or_else(|why| panic!("{}: {why}", at.display()))
        .filter_map(|it| it.ok())
        .filter(|it| !it.path().is_dir())
        .filter_map(|it| it.file_name().to_str().map(str::to_string))
        .filter(|name| name != "world.4x")
        .collect();
    assert!(
        loose.is_empty(),
        "`regression/scenario/` holds {loose:?} outside any turn, and every case belongs to a turn"
    );

    // **A file is left behind when its command is gone, and that is asked of the command rather
    // than of the name.**
    //
    // **`P-572`**: *the two are told apart by `scenario/main.4x` - a case whose command is still
    // there is waiting on Sean, and a case whose command is gone is waiting on nobody.*
    //
    // # This compared filenames and said something false
    //
    // **It was `found.difference(&wanted)` over names.** `S-195` simulated Sean's next edit against
    // it: inserting one `{move}` renamed 33 of 34 files, so 33 names were on disk and not in the
    // produced set, and it failed with *33 file(s) are of commands the scenario no longer plays -
    // delete them.*
    //
    // **Every one of those commands was still played.** And under `P-572` deleting a case whose
    // command is still played is Sean's approval gesture - so **the message instructed the reader to
    // approve thirty-three behaviours it had not shown them.** That is worse than noise.
    //
    // **A scheme that renames less is not a scheme that renames never**, and the check has to be
    // right when one does - which is what keeps it asking the command rather than the name now
    // that a case's position in its turn is back in the filename and does renumber.
    //
    // **It walks the turn directories**, because the cases are one level down. A turn with no
    // directory contributes nothing, and a directory with no cases is reported by the count below
    // rather than passing quietly.
    let played: BTreeSet<String> = cases.iter().map(|it| it.command.clone()).collect();
    let mut gone: Vec<String> = Vec::new();
    let mut renamed: Vec<(String, String)> = Vec::new();
    let mut looked = 0;
    // **The names this run produced**, so a file on disk that is not one of them can be told from
    // one that is.
    let produced: BTreeSet<String> = cases.iter().map(|it| it.name.clone()).collect();
    let mut found: Vec<std::path::PathBuf> = Vec::new();
    for turn in std::fs::read_dir(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display())) {
        let turn = turn.expect("a readable entry").path();
        if !turn.is_dir() {
            continue;
        }
        for entry in
            std::fs::read_dir(&turn).unwrap_or_else(|why| panic!("{}: {why}", turn.display()))
        {
            found.push(entry.expect("a readable entry").path());
        }
    }
    for path in found {
        if path.extension().and_then(|it| it.to_str()) != Some("4x") {
            continue;
        }
        // **Named with its turn**, so a message about a case says where to find it.
        let name = path
            .parent()
            .and_then(|it| it.file_name())
            .and_then(|it| it.to_str())
            .map(|turn| {
                format!(
                    "{turn}/{}",
                    path.file_name().and_then(|it| it.to_str()).unwrap_or("?")
                )
            })
            .unwrap_or_default();
        let text = std::fs::read_to_string(&path).unwrap_or_else(|why| panic!("{name}: {why}"));

        // **The command a case covers is the line under its `{when}`**, which is where every case
        // states it and is the only thing that identifies what it is about.
        let covers = text
            .lines()
            .skip_while(|line| line.trim() != "{when}")
            .nth(1)
            .map(str::trim)
            .unwrap_or_default()
            .to_string();
        assert!(
            !covers.is_empty(),
            "`{name}` has no command under its `{{when}}`, so nothing says what it is about"
        );
        looked += 1;
        if !played.contains(&covers) {
            gone.push(format!("{name} covers {covers}"));
        } else if !produced.contains(&name) {
            // **A file that covers a played command and is not one of today's cases** is what a
            // renumber leaves behind: inserting a command into a turn shifts every case after it,
            // and the old names stay. **`S-195` predicted exactly this** and the naming was chosen
            // to make it rare rather than impossible.
            //
            // **Whether it is worth reading is the thing to say**, because the two look identical
            // on disk. The case that replaced it covers the same command, so either the content
            // matches - a pure rename, nothing to read - or it does not, and the difference is a
            // behaviour change he has not seen.
            // **In the same turn, because a command repeats across turns.** Matching on the
            // command alone said `05/09-end-turn.4x was renamed to 01/06-end-turn.4x` - true of
            // the command and useless, since every turn ends with one. **A turn is what bounds
            // the renumber**, so it bounds the search for what replaced a case too.
            let turn = name.split('/').next().unwrap_or_default();
            let candidates: Vec<&Case> = cases
                .iter()
                .filter(|it| it.command == covers && it.name.starts_with(&format!("{turn}/")))
                .collect();
            // **Compared without the line that must differ.** A case names itself after its own
            // position, so a renumber changes `{test name:...}` by construction - and comparing
            // the whole text called every pure rename a change, which is the opposite of the
            // thing this message exists to tell him.
            let without_its_name = |said: &str| -> String {
                said.lines()
                    .filter(|line| !line.trim_start().starts_with("{test name:"))
                    .collect::<Vec<&str>>()
                    .join("\n")
            };
            let said = match candidates[..] {
                [only] if without_its_name(&only.text) == without_its_name(&text) => format!(
                    "{name} was renamed to {}, and says the same thing - nothing to read",
                    only.name
                ),
                [only] => format!(
                    "{name} was renamed to {}, and what it says changed - read that diff",
                    only.name
                ),
                [] => format!("{name} covers {covers} and no case of this run does"),
                _ => format!(
                    "{name} covers {covers}, which turn {turn} plays {} times - which case \
                     replaced it is not decidable from the command alone",
                    candidates.len()
                ),
            };
            renamed.push((name.clone(), said));
        }
    }

    // **Named and printed rather than counted** - the assertion below said `38` against `36` and
    // left a reader to find which two and what to do about them. **The gesture is his**, so what
    // this owes him is the deletion and whether there is anything to read first.
    if !renamed.is_empty() {
        let mut out = String::new();
        for (_, said) in &renamed {
            out.push_str(&format!("    {said}\n"));
        }
        out.push_str("\n  delete the ones you are happy with:\n");
        for (name, _) in &renamed {
            out.push_str(&format!("    Remove-Item regression/scenario/{name}\n"));
        }
        panic!(
            "{} case(s) on disk are not cases of this run, because a command was inserted and \
             the turn renumbered.\n\
             **Their commands are still played, so deleting one is your approval and no lane may \
             do it.**\n\
             A lane seeing this files it `to spec` before finishing: Sean cannot be addressed \
             directly, and `pending.md` is generated from the outboxes - so until an item exists, \
             the file whose job is to say what waits on a person says nothing. `S-218`.\n{out}",
            renamed.len()
        );
    }

    assert!(
        gone.is_empty(),
        "{} case(s) cover a command `scenario/main.4x` no longer plays, so they are waiting on \
         nobody and any lane may remove them - `P-572`:\n    {}",
        gone.len(),
        gone.join("\n    ")
    );

    // **Both populations, because either being empty would make this vacuous.** No files on disk and
    // every command would be unplayed and none reported; no commands and every file would be.
    assert_eq!(
        looked,
        cases.len(),
        "every file on disk was asked about and every case has one"
    );
    assert!(!played.is_empty(), "the scenario plays no commands");
}

/// **Every case refers to a world, and the world has the rows a command needs.**
///
/// `S-235`, and this is its own test for a reason the first version got wrong: it lived inside
/// `every_command_has_an_expectation_and_it_is_current`, **after the staleness comparison** - so
/// while the thirty-six committed cases were stale it never ran at all. **A check behind a
/// failing assertion is a check nobody has**, and the property here does not depend on whether
/// the committed cases are current.
///
/// # What the check it replaced asserted, and what its message claimed
///
/// **It asserted `world.4x` is a file.** Its message was *the rows every case refers to are not
/// there, so no case can run* - **the message named the property and the assertion named the
/// file.** Zero cases referred to it, because the generator still wrote `setup.4x`, so the
/// property was false while the check was green.
///
/// **A boolean invites no question at all**, which is worse than a plausible number: `0 of 129`
/// makes a reader ask what the population was, and `true` makes a reader ask nothing.
///
/// **What left the generator naming the old file was a `str.replace` with no assertion** - a
/// no-op rather than an error, which `CLAUDE.md` names in as many words, and which a quoted
/// heredoc eating the backslashes of the match string is how it came about.
#[test]
fn every_case_refers_to_a_world_that_holds_what_a_command_needs() {
    let cases = scenario::regression_cases();
    let at = scenario::regression_at();
    assert!(cases.len() > 10, "only {} case(s)", cases.len());

    let mut referred: BTreeSet<String> = BTreeSet::new();
    for Case { name, text, .. } in &cases {
        let row = text
            .lines()
            .map(str::trim)
            .find(|line| line.starts_with("{load "))
            .unwrap_or_else(|| panic!("`{name}` carries no `{{load ...}}` row, so it cannot run"));
        let file = row
            .split_whitespace()
            .find_map(|part| part.strip_prefix("file:"))
            .unwrap_or_else(|| panic!("`{name}`'s load row names no file: {row}"))
            .trim_end_matches('}')
            .to_string();
        assert!(
            row.contains("into:game"),
            "`{name}` loads into something other than the game's store: {row}"
        );
        referred.insert(file);
    }
    assert_eq!(
        referred.len(),
        1,
        "the cases refer to {} different files, and there is one world: {referred:?}",
        referred.len()
    );

    let named = referred.iter().next().expect("one file");
    let said = std::fs::read_to_string(at.join(named))
        .unwrap_or_else(|why| panic!("every case loads `{named}` and {why}"));

    // **What is in it, not that it is there.** `{toil where:place-1}` needs a `place-1`, and a
    // file of the right name holding the wrong rows is the state this exists to refuse.
    for relation in ["place", "territory", "adjacency"] {
        let opens = format!("{{{relation} ");
        assert!(
            said.lines()
                .any(|line| line.trim_start().starts_with(&opens)),
            "`{named}` holds no `{relation}` row, so a command naming one cannot resolve it"
        );
    }
    assert!(
        said.lines()
            .filter(|line| line.trim_start().starts_with('{'))
            .count()
            > 10,
        "`{named}` holds almost nothing, which is not the scenario's world"
    );
}

/// **A case omits no row that can change, and says each description once.**
///
/// `P-598`, in Sean's words: *a regression case is written the way a test is written: the rows that
/// go in, the single command, and the rows that come out. **It omits no row that can change** - so
/// the flow from input to output is on the page and nothing is left for me to remember, which is
/// what a projection onto the columns a rule happened to name cost me.* And: ***a state has one
/// entry per description**, so two things alike are one quantified row and never a row per firing.*
///
/// **The format is what this asserts, not an example of it.** `S-236` found the generator still
/// emitting `effect.took` and `effect.made` - the projection - after `S-234` and `S-235` had put the
/// reference in, and **nothing said so**: the deletion was described as the step that produces the
/// new shape and it produced the old shape with a load row.
///
/// **Checked over every case rather than over the two he named.** His test was *the first test shows
/// gathering on input*; a check that asserted only that would pass on a generator that projected
/// everything else.
#[test]
fn every_case_holds_the_whole_mutable_state_once_per_description() {
    let cases = scenario::regression_cases();
    assert!(cases.len() > 10, "only {} case(s)", cases.len());

    let world = std::fs::read_to_string(scenario::regression_at().join("world.4x"))
        .expect("the invariant rows");
    let invariant: BTreeSet<String> = world
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('{'))
        .map(|line| {
            line.trim_start_matches('{')
                .split([' ', '}'])
                .next()
                .unwrap_or_default()
                .to_string()
        })
        .collect();
    assert!(invariant.len() >= 5, "{invariant:?} is not the world");

    let mut checked = 0;
    let mut sections = 0;
    for Case { name, text, .. } in &cases {
        for marker in ["{given}", "{then}"] {
            // **Stops at the next section marker, which the first version did not.** A section
            // ends at `{when}` or `{then}`, and **both start with `{`** - so a `take_while` on
            // *starts with a brace* read the whole file from `{given}` onward and counted every
            // description twice, once per section.
            //
            // **It reported `{energy where:place-2}` as said twice and that was true of the
            // file** - once in `{given}` and once in `{then}`, which is what a case is. The
            // instrument read a wider population than the one it was asked about, and the answer
            // it gave was about the file rather than about the section.
            const MARKERS: [&str; 4] = ["{given}", "{when}", "{then}", "{refused}"];
            let rows: Vec<&str> = text
                .lines()
                .map(str::trim)
                .skip_while(|line| *line != marker)
                .skip(1)
                .take_while(|line| !MARKERS.contains(line))
                .filter(|line| line.starts_with('{'))
                .collect();
            assert!(!rows.is_empty(), "`{name}`'s {marker} holds no rows at all");
            sections += 1;

            // **One entry per description**, which is the half `04-toil` asked for: it had two
            // identical `{citizen ...} -> 1` rows where the state holds two alike citizens.
            let mut descriptions: BTreeMap<&str, usize> = BTreeMap::new();
            for row in &rows {
                let description = row.rsplit_once(" -> ").map(|it| it.0).unwrap_or(row);
                *descriptions.entry(description).or_insert(0) += 1;
            }
            let twice: Vec<&&str> = descriptions
                .iter()
                .filter(|(_, how_many)| **how_many > 1)
                .map(|(description, _)| description)
                .collect();
            assert!(
                twice.is_empty(),
                "`{name}`'s {marker} says a description more than once, so a reader counts rows \
                 to learn a quantity: {twice:?}"
            );

            // **No invariant relation is repeated into the case.** What never changes is stated
            // once and referred to, so a `{place}` row here would be the thing `world.4x` exists
            // to stop.
            let repeated: Vec<&&str> = rows
                .iter()
                .filter(|row| {
                    let relation = row
                        .trim_start_matches('{')
                        .split([' ', '}'])
                        .next()
                        .unwrap_or_default();
                    invariant.contains(relation)
                })
                .collect();
            assert!(
                repeated.is_empty(),
                "`{name}`'s {marker} repeats rows that `world.4x` already holds: {repeated:?}"
            );
            checked += rows.len();
        }
    }
    assert_eq!(sections, cases.len() * 2, "two sections per case");
    assert!(
        checked > cases.len() * 2,
        "only {checked} row(s) over {sections} section(s), so the sections are one row each and \
         the projection is still there"
    );
}
