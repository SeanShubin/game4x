//! `docs/architecture.md` names every crate, and the workspace decides which crates exist.
//!
//! `S-2`. That document enumerates the set twice - the table of layers and dependencies,
//! and rule 5's requirement that each crate's `README.md` be linked from it - and both have
//! gone stale twice: once when `planet-terrain` landed, and again when `planet-presentation`,
//! `game-globe`, `planet-raster` and `planet-flat` did.
//!
//! # Why a test rather than a generated table
//!
//! The same hazard in the pre-push gate was fixed the other way, by selecting crates with
//! `--exclude` so that coverage is the default. That was right **because the thing being
//! fixed was the gate**: a detector needs somewhere trustworthy to report to, and there was
//! nothing. A table check has no such problem - the gate is now the trustworthy thing, so a
//! test in it is wired to a failure by construction.
//!
//! Coverage by default is still the instinct, and it is satisfied here by what the test
//! iterates: the workspace, not a list. A crate that lands and is not written down fails
//! this, and nobody has to have remembered anything.
//!
//! The table stays hand-written because its other three columns are judgements - what layer
//! a crate is, and what it holds. Only the *set* is a fact, and only the set is checked.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every crate the workspace builds, as a path from the repository root.
fn members(manifest: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let Some(from) = manifest.find("members = [") else {
        panic!("the workspace manifest has no members list");
    };
    let rest = &manifest[from..];
    let Some(to) = rest.find(']') else {
        panic!("the members list is not closed");
    };
    for piece in rest[..to].split('"').skip(1).step_by(2) {
        found.insert(piece.trim_end_matches('/').to_string());
    }
    found
}

/// Every crate the document has a row for, and the README each row links to.
fn rows(document: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for line in document.lines() {
        let line = line.trim();
        if !line.starts_with("| [`") {
            continue;
        }
        // | [`crates/x`](../crates/x/README.md) | kind | deps | what it holds |
        let Some(name) = line
            .split_once("[`")
            .and_then(|(_, rest)| rest.split_once('`'))
            .map(|(name, _)| name.to_string())
        else {
            continue;
        };
        let Some(link) = line
            .split_once("](")
            .and_then(|(_, rest)| rest.split_once(')'))
            .map(|(link, _)| link.to_string())
        else {
            continue;
        };
        found.push((name, link));
    }
    found
}

#[test]
fn every_crate_has_a_row_and_every_row_has_a_crate() {
    let manifest = std::fs::read_to_string(root().join("Cargo.toml"))
        .expect("the workspace manifest is at the root");
    let document = std::fs::read_to_string(root().join("docs/architecture.md"))
        .expect("the architecture document is where the workflow says it is");

    let built = members(&manifest);
    let written: BTreeSet<String> = rows(&document).into_iter().map(|(name, _)| name).collect();

    // The test has to be able to fail. If the table were reformatted so that no row parsed,
    // this would report every crate as missing rather than passing in silence - but a table
    // that parsed to nothing and a workspace with nothing in it look identical from here.
    assert!(
        built.len() >= 10,
        "only {} workspace members parsed; the manifest's shape has changed",
        built.len()
    );
    assert!(
        !written.is_empty(),
        "no rows parsed out of docs/architecture.md; the table's shape has changed"
    );

    let missing: Vec<&String> = built.difference(&written).collect();
    let extra: Vec<&String> = written.difference(&built).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "docs/architecture.md and the workspace disagree about which crates exist\n\
         \n  in the workspace, with no row: {missing:?}\
         \n  has a row, not in the workspace: {extra:?}\n\
         \nThe table's other columns are judgements and are not checked. Only the set is a \
         fact.\nSee S-2."
    );
}

/// Nothing but a generator and a check may name `reports/`.
///
/// **`Q-47`, from the quality lens.** `docs/process.md` says presentations are never
/// canonical and are generated from data. Nothing enforced either, and it is the one
/// statement in that section whose failure is **silent**: a presentation read as a source
/// looks exactly like a presentation until the data changes underneath it.
///
/// **The distinction needs no semantics, only a path.** *Reading to verify* and *reading as
/// input* are the same operation; what tells them apart is who is doing it. A generator
/// lives in `src/bin/`, a check lives under `tests/`, and **nothing else may name the
/// directory**. A test reading a report is a test; production depending on one is the
/// failure this exists to catch.
///
/// **The trap is the spelling, and it caught this check too.** `Q-47` said two were in use -
/// `"reports/…"` and `.join("reports")` - and that searching for either alone finds some
/// readers and misses the rest. The first version of this matched a literal that *was*
/// `reports` or *opened* `reports/`, and **`Q-56` found a third**: `../../reports/…`, from a
/// crate manifest's own directory, in two files that were in the tree while it was written.
/// So the predicate matches the directory **wherever it sits in the path**, and the word in
/// a sentence - *all reports*, *six reports* - is still not a path.
///
/// **Two counts that share a computation are one count**, which is how it survived review on
/// both sides. The population was asserted at five, five is what it found, and five was the
/// figure in the `Q-47` report - already corrected to seven that morning. And the poison used
/// to prove it could fail was `reports/state.md`, a spelling inside the region it already
/// saw, so it could only ever confirm what already worked. **A failing probe has to be
/// aimed where the check is blind**, and this one was not.
///
/// **The population is asserted because a count over nothing proves nothing**, and it is
/// asserted at the real figure rather than a floor under it: `>= 5` tolerated losing two
/// readers in silence.
#[test]
fn only_a_generator_or_a_check_reads_a_report() {
    let root = root();
    let mut readers: Vec<String> = Vec::new();
    let mut trespass: Vec<String> = Vec::new();

    let mut stack = vec![root.clone()];
    while let Some(at) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&at) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                // `target/` holds built artifacts and `.git/` is not source.
                if !matches!(name.as_str(), "target" | ".git" | "node_modules") {
                    stack.push(path);
                }
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let relative = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            // **This file writes the literal it is looking for, so it finds itself.** Left
            // in, the population below can never be empty, and a predicate that had stopped
            // matching anything at all would still satisfy it - the count over nothing with
            // its sign flipped.
            if relative.ends_with("tools/outbox/tests/architecture.rs") {
                continue;
            }
            // **The directory, wherever it appears in the path.** `Q-56`: the first version
            // of this matched a literal that *was* `reports` or *opened* `reports/`, and two
            // files reach it as `../../reports/…` from a crate manifest's own directory - a
            // third spelling, in the tree at the time, in a check whose whole subject was
            // that searching for one spelling misses readers.
            let names_it = text.split('"').skip(1).step_by(2).any(|literal| {
                literal == "reports"
                    || literal.starts_with("reports/")
                    || literal.contains("/reports/")
            });
            if !names_it {
                continue;
            }
            readers.push(relative.clone());
            // **A crate's binary root counts as a generator, and that is a widening.**
            // `prototypes/kinds/src/main.rs` writes two of the reports and is the only
            // binary its crate has, so Cargo's default target is where it belongs -
            // `src/bin/` exists to hold the *second* binary. Moving it to satisfy a check
            // would be churn in the file rather than a correction in the rule.
            //
            // Said rather than done quietly, because it does widen what is permitted: a
            // shipping binary's `main.rs` is now inside the sighted region too, and the path
            // cannot tell a generator from a consumer. What holds that down is the
            // population below.
            // **An example is a generator here too, and this is the second widening.**
            // `crates/thin-engine/examples/report.rs` writes `report.html`, and the nine
            // `crates/game-console/examples/declared-*.rs` write the `spec/data/` files -
            // so an example that prints a generated artifact is this repository's ordinary
            // form for a generator with no reason to be a second binary.
            //
            // **It surfaced when the engine moved out of `prototypes/` on 2026-09-22.**
            // `S-153` put it under `crates/`, where this check looks; nothing about the file
            // changed. So the rule had a hole the whole time and a move is what walked into
            // it, which is worth more than the one line it costs to close.
            //
            // **Said rather than done quietly, for the reason the paragraph above says it**:
            // this widens the sighted region again, and what holds it down is still the
            // population below rather than the shape of a path.
            let generator = relative.contains("/src/bin/")
                || relative.ends_with("/src/main.rs")
                || relative.contains("/examples/");
            let check = relative.contains("/tests/");
            if !generator && !check {
                trespass.push(relative);
            }
        }
    }

    readers.sort();
    // **Eleven since `D-4`**, thirteen since `S-153`, and seven before that. The engine's own
    // report reader arrived with the move into `crates/`; two left when `D-4` deleted
    // `prototypes/kinds` and the `reports/catalog.md` it wrote. **Moved with the population
    // rather than left**, because a floor set for one population is a literal about a different
    // one the moment the population moves - and lowered deliberately, which is what this
    // assertion's own message asks for.
    assert!(
        readers.len() >= 11,
        "only {} file(s) name the reports directory, and there were seven - {readers:?}. \
         A predicate that finds nothing passes while checking nothing, and one that finds \
         most of them passes while missing the rest, which is `Q-56` exactly. If a reader \
         was deliberately removed, lower this with it and say so. It was thirteen until \
         `D-4` deleted `prototypes/kinds` and the catalog it wrote.",
        readers.len()
    );
    assert!(
        trespass.is_empty(),
        "a report is a presentation and is never a source - `docs/process.md`. These name \
         `reports/` and are neither a generator in `src/bin/` nor a check under `tests/`:\
         \n  {}\n\nAll {} reader(s): {readers:?}",
        trespass.join("\n  "),
        readers.len()
    );
}

/// Rule 5: each crate's `README.md` is linked from the document. A row that links to a file
/// that is not there satisfies the rule in form and not in fact.
#[test]
fn every_row_links_to_a_readme_that_exists() {
    let document = std::fs::read_to_string(root().join("docs/architecture.md"))
        .expect("the architecture document is where the workflow says it is");
    let mut broken = Vec::new();
    let mut checked = 0usize;
    for (name, link) in rows(&document) {
        // Links are relative to `docs/`, which is where the document lives.
        let target = root().join("docs").join(&link);
        checked += 1;
        if !Path::new(&target).exists() {
            broken.push(format!("{name} -> {link}"));
        }
    }
    assert!(checked >= 10, "only {checked} rows parsed");
    assert!(
        broken.is_empty(),
        "rows link to a README that is not there:\n  {}",
        broken.join("\n  ")
    );
}

/// Every `.rs` file under a directory, so a walk covers what is there rather than a list.
///
/// **Coverage by default**, which is this file's own argument: a crate that lands and is not
/// written down fails `every_crate_has_a_row_and_every_row_has_a_crate` because that test
/// iterates the workspace. Same instinct, same reason.
fn every_rust_file(under: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![under.to_path_buf()];
    while let Some(at) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&at) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                if !matches!(name.as_str(), "target" | ".git" | "node_modules") {
                    stack.push(path);
                }
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// Every field of `Game` that is written from outside `game-model`, and each name is derived.
///
/// **The names come from `pub struct Game` itself**, so a field added to the state is covered
/// without anybody remembering to add it here - which is the same reason
/// `every_crate_has_a_row_and_every_row_has_a_crate` iterates the workspace rather than a list.
fn fields_of_the_game(source: &str) -> BTreeSet<String> {
    let Some(from) = source.find("pub struct Game {") else {
        panic!("crates/game-model/src/game.rs has no `pub struct Game`");
    };
    let rest = &source[from..];
    let Some(to) = rest.find("\n}") else {
        panic!("`pub struct Game` is not closed");
    };
    let mut found = BTreeSet::new();
    for line in rest[..to].lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("pub ")
            && let Some((name, _)) = rest.split_once(':')
            && !name.contains('(')
        {
            found.insert(name.trim().to_string());
        }
    }
    found
}

/// Nothing outside `game-model` writes the game's state except a fixture.
///
/// # What this is, and the finding it comes from
///
/// **`Q-99`, from the quality lens on 2026-09-24.** `Game`'s fields are `pub`, so any crate
/// that can read the state can also build one no transition could produce - and
/// `Game::after` exists to be the only thing that moves it. Twenty-eight sites outside
/// `game-model` were writing those fields.
///
/// **Twelve were fixtures and thirteen were the shipped path.** The fixtures are in
/// `#[cfg(test)]` modules of `containment.rs` and `tree.rs` and are not the defect: a test
/// building a state directly is a test, and making the fields private would redden exactly
/// the population that is fine. **The thirteen were `crates/game-console/src/worked.rs`**,
/// 508 lines with no test module at all, which generates the worked examples `R-7` is vetted
/// by.
///
/// # What it actually cost, which is not an impossible state
///
/// **Nothing wrong was on disk.** `ground()` wrote `phase = Play` and `turn = 1`, which is
/// exactly and only what `Transition::Start` does - so it *matched*. The cost is that it was
/// a transition's body restated in another crate with nothing keeping the two in step, and
/// five sites minted `UnitId(1)` by hand where the model mints `units.len() + 1`.
///
/// **The file doing it is the one whose own header argues against it**: *a written example can
/// show behaviour the code does not have, and this repository has produced three of those in
/// one week ... all three sat in artifacts whose purpose was hand-derivation.*
///
/// # Why a check here rather than private fields
///
/// **Privacy reddens the fixtures, which are not the problem.** The rule that is actually
/// wanted is *the shipped path goes through the model*, and a `#[cfg(test)]` module is exactly
/// where the exception belongs - so the predicate is the boundary rather than the visibility.
///
/// **Inside a test module, not merely in a file that has one.** The weaker form - *a file with
/// no `#[cfg(test)]`* - is satisfied by adding one token test to `worked.rs`, which would
/// change nothing and silence this.
///
/// # What it does not check
///
/// **Only `Game`'s own fields.** `Territory::deposits` is public too and is written by these
/// same fixtures; that is a second boundary and this does not claim it. The population of
/// files walked is asserted, so a walk that found nothing cannot pass.
#[test]
fn only_game_model_and_a_fixture_write_the_games_state() {
    let model = std::fs::read_to_string(root().join("crates/game-model/src/game.rs"))
        .expect("crates/game-model/src/game.rs");
    let fields = fields_of_the_game(&model);
    assert!(
        fields.len() >= 6,
        "only {} fields read off `pub struct Game`, so this would look for almost nothing:          {fields:?}",
        fields.len()
    );

    let mut walked = 0;
    let mut offences = Vec::new();
    let mut fixtures = 0;
    for file in every_rust_file(&root().join("crates")) {
        let relative = file
            .strip_prefix(root())
            .unwrap_or(&file)
            .to_string_lossy()
            .replace('\\', "/");
        // `game-model` is where the state lives and is allowed to write it.
        if relative.starts_with("crates/game-model/") {
            continue;
        }
        let source = std::fs::read_to_string(&file).expect("a rust file this walk found");
        walked += 1;
        // Where the test module begins, if there is one. Everything from there down is a
        // fixture, which is the exception this rule has.
        let tests_begin = source
            .lines()
            .position(|line| line.trim().starts_with("#[cfg(test)]"))
            .unwrap_or(usize::MAX);

        for (at, line) in source.lines().enumerate() {
            let line = line.trim();
            // A comment naming a write is not a write - which is the *quoting a thing and
            // doing it are the same bytes* failure this repository tracks by name.
            if line.starts_with("//") {
                continue;
            }
            for name in &fields {
                let Some(after) = line.split_once(&format!(".{name}")) else {
                    continue;
                };
                let after = after.1;
                let writes = after.starts_with(".push(")
                    || after.starts_with(".insert(")
                    || after.starts_with(".remove(")
                    || after.starts_with(".clear(")
                    || after.starts_with(".pop(")
                    || (after.trim_start().starts_with('=')
                        && !after.trim_start().starts_with("=="));
                if !writes {
                    continue;
                }
                if at > tests_begin {
                    fixtures += 1;
                } else {
                    offences.push(format!("{relative}:{}: {line}", at + 1));
                }
            }
        }
    }

    assert!(
        walked > 40,
        "only {walked} rust files outside `game-model` were read, so the assertions below          would be about almost nothing"
    );
    // **The fixtures are the population this rule deliberately allows**, and a count of zero
    // offences means nothing beside a count of zero exceptions - there would be nothing for
    // the predicate to have distinguished.
    assert!(
        fixtures > 8,
        "only {fixtures} writes were found inside test modules, and this rule's whole content          is that those are allowed and others are not - with none of them, a clean result          would not show the predicate had run"
    );
    assert!(
        offences.is_empty(),
        "{} site(s) outside `game-model` write the game's state from the shipped path, where          `Game::after` is meant to be the only thing that moves it:
  {}",
        offences.len(),
        offences.join("
  ")
    );
}

/// **Every script has a row in `scripts/README.md` and every row has a script.**
///
/// # Why this exists, and it is the same shape one file over
///
/// **`S-197`**: Sean had to ask how to run the regression test. `scripts/README.md` opens with *one
/// script per thing you might want to run, so that running it never requires remembering a cargo
/// incantation* - and the running worked while the index did not.
///
/// **Measured when it was written**: seventeen scripts, eleven rows. Seven missing including the two
/// he needed, one listed that `D-4` had deleted, and `reviewed` described as being over a directory
/// that had moved two days earlier.
///
/// **This is `every_crate_has_a_row_and_every_row_has_a_crate` applied to a second index.** That one
/// has kept `docs/architecture.md` honest through two crate moves this week, and the case it names -
/// a row for something that no longer exists - is exactly the third line above.
///
/// # Both directions, because they rot differently
///
/// **A script with no row** is a thing nobody can find, which is what happened. **A row with no
/// script** is an instruction that fails when followed, which is worse: it reads as current.
#[test]
fn every_script_has_a_row_and_every_row_has_a_script() {
    let scripts = root().join("scripts");
    let on_disk: BTreeSet<String> = std::fs::read_dir(&scripts)
        .expect("scripts/")
        .filter_map(|it| it.ok())
        .filter_map(|it| it.file_name().to_str().map(str::to_string))
        .filter_map(|name| name.strip_suffix(".sh").map(str::to_string))
        .collect();

    // **`.ps1` and `.sh` are one entry, and both have to be there.** `scripts/README.md`:
    // *two of each, PowerShell and POSIX shell, doing the same thing... a script that only runs on
    // one machine is a trap for the next person.* So the pairing is checked here rather than being
    // a convention somebody remembers.
    let unpaired: Vec<&String> = on_disk
        .iter()
        .filter(|name| !scripts.join(format!("{name}.ps1")).is_file())
        .collect();
    assert!(
        unpaired.is_empty(),
        "these have a `.sh` and no `.ps1`: {unpaired:?} - a script that runs on one machine is a \
         trap for the next person"
    );

    let text = std::fs::read_to_string(root().join("scripts").join("README.md"))
        .expect("scripts/README.md");
    let listed: BTreeSet<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("| `"))
        .filter_map(|rest| rest.split('`').next())
        .filter_map(|name| name.strip_suffix(".ps1").map(str::to_string))
        .collect();

    // **A count over nothing is the same failure with the sign flipped.** An index that parsed to
    // nothing would report every script as missing; a directory that read as empty would report
    // every row as stale. Both would be true of the sets and false of the repository.
    assert!(
        on_disk.len() > 10,
        "only {} script(s) found, so the directory did not read",
        on_disk.len()
    );
    assert!(
        listed.len() > 10,
        "only {} row(s) parsed out of scripts/README.md, so its table has changed shape",
        listed.len()
    );

    let missing: Vec<&String> = on_disk.difference(&listed).collect();
    let stale: Vec<&String> = listed.difference(&on_disk).collect();
    assert!(
        missing.is_empty() && stale.is_empty(),
        "scripts/README.md and scripts/ disagree\n\
         \n  on disk, with no row: {missing:?}\
         \n  has a row, not on disk: {stale:?}\n\
         \nThe Notes column is a judgement and is not checked. Only the set is a fact. See S-197."
    );
}
