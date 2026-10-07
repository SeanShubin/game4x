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
/// **Every local crate a row names is one that crate's manifest declares, and the other way.**
///
/// `Q-104`: *for a composition root the dependency list is the architecture statement, and
/// `docs/architecture.md`'s column transcribes it rather than checking it, so a wrong manifest
/// propagates into the document a reader would check the manifest against.* **A transcription
/// nobody compares is a second copy**, and this repository's standing answer to a second copy is
/// to derive it or to check it.
///
/// # What it compares, and what it deliberately does not
///
/// **Local path dependencies only.** `bevy`, `png`, `wasm-bindgen` and the rest are judgements
/// about what is worth naming in a document for a reader, and the column says things like
/// *`wasm-bindgen` on web* that no manifest field holds. **The set of workspace crates an edge
/// points at is a fact**, and it is the part that went wrong: `game4x` declared `game-console`
/// and named it in no line of code, while `main.rs` names `game-inspect` four times and the row
/// did not have it.
///
/// **Dev-dependencies count as declared.** A row saying a crate is reached is right whether the
/// edge is for the build or for the tests, and `goldberg-move` had a dev edge to `planet-model`
/// that no code named.
#[test]
fn every_local_dependency_a_row_names_is_one_the_manifest_declares() {
    let document = std::fs::read_to_string(root().join("docs/architecture.md"))
        .expect("the architecture document is where the workflow says it is");
    // **Bare crate names, because `members` returns paths.** It yields `crates/game-console`
    // and a manifest writes `game-console`, so comparing the two filters everything out and
    // leaves two empty sets agreeing. **That is how the first version of this check passed over
    // the exact state `Q-104` found** - caught by driving it against that state rather than by
    // reading it, and the reason `edges` below is asserted.
    let workspace: BTreeSet<String> = members(
        &std::fs::read_to_string(root().join("Cargo.toml")).expect("the workspace manifest"),
    )
    .into_iter()
    .filter_map(|path| path.rsplit('/').next().map(str::to_string))
    .collect();

    let mut compared = 0;
    let mut edges = 0;
    let mut wrong: Vec<String> = Vec::new();
    for (name, _) in rows(&document) {
        let at = root().join(&name).join("Cargo.toml");
        let Ok(manifest) = std::fs::read_to_string(&at) else {
            continue;
        };
        // **Declared is any line opening `<crate>.workspace` or `<crate> = `**, over both
        // dependency tables, with comments dropped. The manifests here write a local edge one
        // of those two ways and nothing else.
        let declared: BTreeSet<String> = manifest
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with('#'))
            .filter_map(|line| line.split(['.', ' ']).next())
            .filter(|word| workspace.contains(*word))
            .map(str::to_string)
            .collect();
        let written: BTreeSet<String> = row_dependencies(&document, &name)
            .into_iter()
            .filter(|word| workspace.contains(word))
            .collect();
        compared += 1;
        edges += declared.len();

        let missing: Vec<&String> = declared.difference(&written).collect();
        let extra: Vec<&String> = written.difference(&declared).collect();
        if !missing.is_empty() || !extra.is_empty() {
            wrong.push(format!(
                "{name}\n    declared and not in the row: {missing:?}\
                 \n    in the row and not declared: {extra:?}"
            ));
        }
    }

    // **Both populations.** A table that stopped parsing would compare nothing and pass in
    // exactly these words, which is `CLAUDE.md`'s count over nothing with the sign flipped.
    assert!(
        compared >= 15,
        "only {compared} row(s) had a manifest to compare against; the table's shape has changed"
    );
    // **The edges, not only the rows.** Fifteen rows each comparing nothing to nothing is the
    // green this check already produced once.
    assert!(
        edges >= 30,
        "only {edges} local dependency edge(s) were found over {compared} row(s), so the          comparison below ran over almost nothing"
    );
    assert!(
        wrong.is_empty(),
        "docs/architecture.md's dependency column and the manifests disagree:\n  {}\n\n\
         Only local crates are compared - `bevy` and the rest are a judgement about what is \
         worth naming. See `Q-104`.",
        wrong.join("\n  ")
    );
}

/// The crates named in one row's dependency cell, by the row's own name.
///
/// **Located by the row's link rather than by matching the row**, because `tools/pad-tables`
/// rewrites the column widths every commit - `CLAUDE.md`: *never put a table row in a match
/// string*. The cell is split on `|` and stripped, which is pad-proof by construction.
fn row_dependencies(document: &str, name: &str) -> BTreeSet<String> {
    let opens = format!("| [`{name}`]");
    for line in document.lines() {
        let line = line.trim();
        if !line.starts_with(&opens) {
            continue;
        }
        let cells: Vec<&str> = line.split('|').collect();
        let Some(cell) = cells.get(3) else {
            return BTreeSet::new();
        };
        // **Every backtick span in the cell, rather than every comma-separated piece.** A
        // cell says `` `graph-coloring` in tests `` and `` `wasm-bindgen` on web ``, so the
        // qualifier travels with the name and splitting on commas keeps it. **The first
        // version did and reported four rows wrong that were right** - the name matched
        // nothing because `graph-coloring` in tests` is not a crate.
        return cell
            .split('`')
            .skip(1)
            .step_by(2)
            .map(|word| word.trim().to_string())
            .filter(|word| !word.is_empty())
            .collect();
    }
    BTreeSet::new()
}

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
    // **Four since the rest of `D-4`**, eleven before it, thirteen since `S-153`, and seven
    // before that. **Moved with the population rather than left**, because a floor set for one
    // population is a literal about a different one the moment the population moves - and
    // lowered deliberately, which is what this assertion's own message asks for.
    //
    // **Seven readers went when the old ruleset's reports did**: `game-console`'s `dump`,
    // `browse`, `relations`, `tree` and `style`, its `dump-state` binary, and the tests over
    // them. What is left reads `reports/foundation/`, which `R-12` writes out of the ruleset
    // the game plays by - so the rule still has a population and it is the surviving one.
    assert!(
        readers.len() >= 4,
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

/// **Every repository path a script runs against is a path that is there.**
///
/// # The check that was missing, and what it cost
///
/// `every_script_has_a_row_and_every_row_has_a_script` asks whether a script exists and never
/// what it names. So `scripts/reviewed.ps1` went on asking `git status` about
/// `crates/game-model/reviewed` for eight days after `P-532` moved the records to `reviewed/` at
/// the root - **and it passed every gate in that time, because the file was there and the row
/// described it.** `$pending` was always empty, so the script exited saying there was nothing to
/// record, which is what it says when it has worked. `S-222`.
///
/// **Four of the nine paths in that file did not exist**, measured against it before the fix.
///
/// # Why comments are excluded and it is not a weakening
///
/// **A comment may name a path that is gone, and usually has to.** The corrected script explains
/// itself by saying which directory it used to watch, and a check that refused that would make
/// the explanation unwritable. **An executable line has no such reason** - it is the thing that
/// runs, and a path in it that is not there is a script doing nothing or doing it elsewhere.
#[test]
fn every_path_a_script_runs_against_is_there() {
    // The top-level directories a repository path can begin with, read rather than listed, so a
    // new one is covered the day it appears.
    let tops: BTreeSet<String> = std::fs::read_dir(root())
        .expect("the repository root")
        .filter_map(|it| it.ok())
        .filter(|it| it.path().is_dir())
        .filter_map(|it| it.file_name().to_str().map(str::to_string))
        .filter(|name| !name.starts_with('.') && name != "target")
        .collect();
    assert!(
        tops.len() > 5,
        "only {} top-level director(ies), so this would match almost no path",
        tops.len()
    );

    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for under in ["scripts", "hooks"] {
        for entry in std::fs::read_dir(root().join(under)).expect("a directory of scripts") {
            let path = entry.expect("an entry").path();
            // **What runs, rather than what sits beside it.** `scripts/README.md` is the index
            // and prose, not an instruction - it names paths inside sentences and inside
            // trailing ellipses, and the other test in this file is the one that keeps it
            // honest. A hook has no extension at all, which is why this is a list of what to
            // take rather than a list of what to skip.
            let runs = matches!(
                path.extension().and_then(|it| it.to_str()),
                Some("sh") | Some("ps1") | None
            );
            if path.is_file() && runs {
                files.push(path);
            }
        }
    }
    assert!(files.len() > 15, "only {} script(s) were read", files.len());

    let mut checked = 0;
    let mut missing: Vec<String> = Vec::new();
    for file in &files {
        let text = std::fs::read_to_string(file).expect("a script");
        let name = file.file_name().and_then(|it| it.to_str()).unwrap_or("?");
        for (at, line) in text.lines().enumerate() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            for said in paths_in(line, &tops) {
                checked += 1;
                if !root().join(&said).exists() {
                    missing.push(format!("{name}:{} names {said}", at + 1));
                }
            }
        }
    }

    // **A count over nothing is the same failure with the sign flipped.** With no paths found,
    // every path exists vacuously - which is what a changed quoting style would produce.
    assert!(
        checked > 20,
        "only {checked} path(s) were found across {} script(s), so the reader is not reading them",
        files.len()
    );
    assert!(
        missing.is_empty(),
        "{} path(s) a script runs against are not there, so the script does nothing or does it \
         somewhere else:\n  {}",
        missing.len(),
        missing.join("\n  ")
    );
}

/// Every repository path in one line, by its first segment being a directory that exists.
///
/// **A path with a variable in it is skipped rather than guessed at.** `"$root/spec/tests/$name"`
/// names a file whose name the script computes, and resolving it would mean running the script.
/// The fixed part of such a path is still covered wherever the script writes it out in full.
fn paths_in(line: &str, tops: &BTreeSet<String>) -> Vec<String> {
    let mut found = Vec::new();
    for word in line.split(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == '(') {
        let word = word.trim_matches(|c| matches!(c, ',' | ';' | ')' | '`' | '\\'));
        let Some((first, _)) = word.split_once('/') else {
            continue;
        };
        if !tops.contains(first) {
            continue;
        }
        if word.contains('$') || word.contains('*') || word.contains('{') {
            continue;
        }
        found.push(word.trim_end_matches('/').to_string());
    }
    found
}

/// **Only the composition root reaches the one console; everything else is handed an
/// interface.**
///
/// `S-227`, from Sean: *I would have wrapped access to game state in an interface, hooked up
/// the implementation in the composition roots.* `C-188` measured the state before it: ten
/// sites in three crates reached `game_front::shell::` directly, so there was nothing to
/// substitute and no place that said which game was which.
///
/// **This is the rule that keeps it that way.** A crate that names the shell again has reached
/// past the seam, and nothing else would notice - the code would compile, the game would run,
/// and the interface would quietly stop being the only door.
#[test]
fn only_the_composition_root_reaches_the_one_console() {
    let mut reached: Vec<String> = Vec::new();
    let mut scanned = 0;
    for base in ["crates", "prototypes"] {
        for path in every_rust_file(&root().join(base)) {
            let shown = path
                .strip_prefix(root())
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            // The crate that owns the console may of course name it.
            if shown.starts_with("crates/game-front/") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            scanned += 1;
            for (at, line) in text.lines().enumerate() {
                let said = line.trim();
                // A doc comment naming it is prose, not a reach.
                if said.starts_with("//") {
                    continue;
                }
                // **`game_front::shell` without the trailing colons, and the exemption
                // removed from the line rather than tested against it** - `Q-109`, which drove
                // the predicate instead of the tree.
                //
                // **Three import forms defeated the old one and every call site after them.**
                // `use game_front::shell;`, `use game_front::shell as console;` and
                // `use game_front::{shell, library};` all pass a test for
                // `game_front::shell::`, and `shell::generation()` afterwards does not contain
                // the crate name at all. **The check asked *does a line spell this exact path*
                // where the rule is *does a crate outside `game-front` reach the one console*.**
                //
                // **And `ALLOWED` was a substring test over the whole line**, so a line naming
                // the allowed path anywhere in it - a trailing comment included - exempted every
                // other reach on that line. Removing the allowed text first leaves the rest of
                // the line to be judged on its own.
                if reaches_the_console(said) {
                    reached.push(format!("{shown}:{}  {said}", at + 1));
                }
            }
        }
    }

    // **Both populations**: a scan that found no files would report no reaches in exactly
    // these words.
    assert!(
        scanned >= 50,
        "only {scanned} file(s) scanned; this would prove little"
    );
    assert!(
        reached.is_empty(),
        "these reach the one console rather than being handed an interface - `S-227`:\n  {}\n\n\
         Outside `game-front` the admitted names are {ADMITTED:?}; anything else naming both the \
         crate and its shell is a reach, whatever the import grouping.",
        reached.join("\n  ")
    );
}

/// Every `game_front::` reference a crate outside that one may make.
///
/// **Measured over the tree rather than listed from memory**: the two interfaces, the
/// implementation the composition root constructs, and the terminal thread - which is platform
/// wiring rather than game state, like the window the root describes two functions later.
const ADMITTED: [&str; 4] = [
    "game_front::game_state::Watches",
    "game_front::game_state::Drives",
    "game_front::game_state::TheOneConsole",
    "game_front::shell::terminal::serve",
];

/// Whether a line reaches `game-front`'s shell, in any form that compiles.
///
/// # Three patches, a fourth escape, and why this one matches no shape
///
/// **Every version before this matched a shape and a shape escaped it.**
/// `game_front::shell::` missed three import forms. `game_front::shell` missed the braced group,
/// because the crate and the module are not adjacent in `use game_front::{shell, library};`.
/// Splitting the braced group missed a *nested* group before `shell`, because `split_once('}')`
/// takes the first closing brace rather than the matching one -
/// `use game_front::{library::{browse, page}, shell};`.
///
/// **The quality lens found all three, and the third by driving this function rather than reading
/// it.** `Q-109`, and its own proposed repair closed two of the first three - which is the lesson
/// twice over: a pattern has one more form than whoever wrote it thought of.
///
/// **So this matches no shape.** Every form that compiles names the crate and names the module,
/// whatever grouping sits between them - so with the admitted names removed, **both tokens
/// remaining is the whole of it**, and there is no nesting left to get wrong.
///
/// # What the comment strip costs, in both directions
///
/// **A trailing comment is dropped first**, so a line importing an interface and mentioning the
/// shell in prose beside it is not a reach.
///
/// **It cuts at the first `//` and that is not always a comment.** The quality lens pointed out
/// that this doc named only the direction that fires too often, and **the other direction is a
/// silent miss** - which is the one worth writing down:
///
/// ```text
/// fires needlessly   let said = "shell";  use game_front::game_state::Watches;
/// misses silently    let url = "https://x"; use game_front::shell;
/// misses silently    use front::shell;      if a manifest renamed the dependency
/// ```
///
/// **Both misses are contrived** - rustfmt does not leave two statements on one line, and no
/// manifest renames `game-front` - so neither is repaired. **What is repaired is the comment**:
/// a reader deciding how far to trust this step should not have to find the unsafe direction
/// themselves, and a doc that considered string literals and then named only the safe case is
/// worse than one that had not thought of them.
///
/// **The message names the admitted set**, so a needless fire costs one glance.
fn reaches_the_console(line: &str) -> bool {
    let code = line.split("//").next().unwrap_or(line);
    let mut rest: String = code.chars().filter(|it| !it.is_whitespace()).collect();
    for one in ADMITTED {
        rest = rest.replace(one, "");
    }
    rest.contains("game_front") && rest.contains("shell")
}

/// Every `.rs` file under a directory.
fn every_rust_file(under: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![under.to_path_buf()];
    while let Some(at) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&at) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().and_then(|it| it.to_str()) != Some("target") {
                    stack.push(path);
                }
            } else if path.extension().and_then(|it| it.to_str()) == Some("rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// **Every step that puts committed files into the artifact survives a failure, and so does
/// everything after the first one** - `X-43`, `X-44`, `P-613`.
///
/// `docs/process.md`: *the whole site publishes whether or not the run succeeded. A failing build
/// leaves a broken game published rather than nothing published, because deploying is how I verify
/// and a staging area that vanishes when it breaks is no use to me.*
///
/// # The failure this is written from
///
/// **`deploy` became `if: always()` and the step that copies the reports in did not.** A `cargo fmt`
/// failure then skipped that step while the upload still ran, so **the deploy succeeded and
/// published a site with no reports** - and because Pages replaces the whole site,
/// `reports/review/index.html` 404ed rather than going stale.
///
/// # And the first version of this check said it read what a step does
///
/// **It did not: the anchor was `name.contains("Copy the reports into the artifact")`.** `X-44`
/// probed it - a second assembling step placed *above* the anchor, with no condition, and the check
/// passed. **A lint failure would then publish the reports and no `scenario/`**, which is `S-230`'s
/// outward-link 404 rather than a new failure mode.
///
/// **The sharper half of that finding is what the old floor did.** It asserted *at least one step
/// before the anchor is skippable* - and the inserted step satisfied it, so **the guard read as
/// healthier at the moment it stopped holding.**
///
/// **So the predicate reads the script now**: a step is assembling if it copies something into
/// `crates/game4x/dist`. **`mentions dist` would have been wrong** and this file holds the
/// counter-example - `Write build provenance` writes `dist/build-info.json` and `sed -i`s
/// `dist/index.html`, and **must not survive**, because `index.html` is the game's bundle and does
/// not exist when `trunk` never ran.
///
/// **Live rather than hypothetical**: that one step does nine copies into seven destinations, and
/// `X-44` notes that splitting it is the natural next edit.
#[test]
fn every_step_that_assembles_or_publishes_survives_a_failure() {
    // **The composite action, because that is where the site is assembled now** - `P-619`. It was
    // the `gate` job, then the `deploy` job, and now one definition that `publish` and `republish`
    // both call: *a push that touched both gets the short one and then the long one.*
    //
    // **So the subject moved twice and the check followed twice**, which is the cost of a predicate
    // anchored to a place rather than to a property - and there is nowhere better to anchor it,
    // because *what assembles the site* is a fact about a file.
    let at = root().join(".github/actions/publish/action.yml");
    let text = std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));

    // **Every step of it, because a composite action is nothing but steps.** There is no job to
    // find the end of.
    let job = text.as_str();

    // Each step is a `- name:` or `- uses:` at the steps' indent, with whatever follows it: whether
    // it carries the condition, and the script it runs.
    struct Step {
        named: String,
        always: bool,
        script: String,
    }
    let mut steps: Vec<Step> = Vec::new();
    for line in job.lines() {
        let bare = line.trim();
        if bare.starts_with("- name:") || bare.starts_with("- uses:") {
            steps.push(Step {
                named: bare
                    .trim_start_matches("- name:")
                    .trim_start_matches("- uses:")
                    .trim()
                    .to_string(),
                always: false,
                script: String::new(),
            });
        } else if let Some(one) = steps.last_mut() {
            if bare == "if: always()" {
                one.always = true;
            }
            one.script.push_str(bare);
            one.script.push('\n');
        }
    }

    // **A floor on the parse rather than on the job.** It was `> 8`, calibrated for `gate`'s
    // fourteen steps, and `deploy` has six - so moving the assembly made a correct parse fail a
    // number chosen for a different job. **The population guard below is what has teeth**; this
    // only says the text was read at all.
    assert!(
        steps.len() >= 4,
        "only {} step(s) read from the deploy job, so the parse found almost nothing",
        steps.len()
    );

    // **Assembling means it copies something into the artifact**, which is what makes it need no
    // toolchain and makes it mandatory when one failed.
    let assembles = |it: &Step| {
        it.script
            .lines()
            .any(|line| line.starts_with("cp ") && line.contains("crates/game4x/dist"))
    };

    let building: Vec<&Step> = steps.iter().filter(|it| assembles(it)).collect();
    // **The population, or the assertion below is about nothing.** A workflow that stopped copying
    // committed files into the artifact would pass this vacuously.
    assert!(
        !building.is_empty(),
        "no step copies committed files into the artifact, so this checked nothing"
    );
    let skippable: Vec<&String> = building
        .iter()
        .filter(|it| !it.always)
        .map(|it| &it.named)
        .collect();
    assert!(
        skippable.is_empty(),
        "these steps put committed files into the artifact and would be skipped by a failing \
         lint, publishing a site without them: {skippable:?}"
    );

    // **And everything from the first one on**, which covers the status page and the upload - they
    // assemble nothing and must still run.
    let first = steps.iter().position(assembles).expect("one of them");
    let after: Vec<&String> = steps[first..]
        .iter()
        .filter(|it| !it.always)
        .map(|it| &it.named)
        .collect();
    assert!(
        after.is_empty(),
        "from `{}` on, every step must carry `if: always()`: {after:?}",
        steps[first].named
    );
}

/// **Every job that installs Bevy's Linux dependencies installs the same packages.**
///
/// # The failure this is written from
///
/// **`checks` installed two of four.** This lane wrote that job when `S-251` split the tests out of
/// `gate`, **copied the step's name and not its package list**, and left `libwayland-dev` and
/// `libxkbcommon-dev` behind. So `wayland-sys`'s `build.rs` failed in `pkg-config` - *Package
/// wayland-client was not found in the pkg-config search path* - and the job went red on three runs
/// while `Verify - full test suite`, which has all four, passed over the same commit in the same
/// run.
///
/// # Two wrong answers before the log, and both read the wrong population
///
/// **The specification lane guessed `wayland-sys` and named the right mechanism for the wrong
/// reason**: that `checks` installed a *plain* toolchain where `gate` installed Bevy's dependencies.
///
/// **This lane refuted that with `bevy deps: True` per job** - a true reading of *does the step
/// exist*, where the question was *which packages does it install*. **So the refutation was correct
/// about its own predicate and wrong about the thing**, and it sent both lanes to cargo's feature
/// resolution instead.
///
/// **Neither of us read the package list until the log forced it.** *The step is there* and *the
/// step installs what is needed* are different claims, and the first is what a grep for the step
/// name answers.
///
/// **And reading the whole list found one more than the log named.** `libxkbcommon-dev` was missing
/// too and would have failed next - which is the same habit paying out immediately: the log names
/// the first package to fail, not the set that is absent.
#[test]
fn every_job_that_links_bevy_installs_the_same_packages() {
    let text = std::fs::read_to_string(root().join(".github/workflows/pipeline.yml"))
        .expect(".github/workflows/pipeline.yml");

    // Each `apt-get install` line, with the job it is in - found by indentation, since a job
    // opens at two spaces and nothing inside one does.
    let mut lists: Vec<(String, Vec<String>)> = Vec::new();
    let mut job = String::new();
    for line in text.lines() {
        if let Some(named) = line.strip_prefix("  ")
            && !named.starts_with(' ')
            && !named.starts_with('#')
            && let Some(named) = named.strip_suffix(':')
        {
            job = named.to_string();
        }
        if line.contains("apt-get install") {
            let mut packages: Vec<String> = line
                .split_whitespace()
                .filter(|it| it.starts_with("lib") && it.ends_with("-dev"))
                .map(str::to_string)
                .collect();
            packages.sort();
            lists.push((job.clone(), packages));
        }
    }

    // **The population, or an equality over one list passes for the wrong reason.** Four jobs
    // install these today and the whole point is that four copies agreed with nothing comparing
    // them.
    assert!(
        lists.len() >= 3,
        "only {} job(s) install Bevy's dependencies, so comparing them says little: {lists:?}",
        lists.len()
    );
    // And each list is non-empty, or jobs installing nothing would agree with each other.
    for (job, packages) in &lists {
        assert!(
            packages.len() >= 4,
            "`{job}` installs {} package(s), which is fewer than the four every job needs: \
             {packages:?}",
            packages.len()
        );
    }

    let (first, wanted) = &lists[0];
    for (job, packages) in &lists[1..] {
        assert_eq!(
            packages, wanted,
            "`{job}` and `{first}` install different packages, so one of them builds Bevy without \
             what its build scripts need"
        );
    }
}

/// **A job that uses a local action checks out the repository first.**
///
/// # The failure this is written from
///
/// **`S-257`: run `37242601189` deployed nothing**, and every job but publishing passed.
///
/// ```text
/// Can't find 'action.yml' under .github/actions/publish
/// Did you forget to run actions/checkout before running your local action?
/// ```
///
/// **A local action lives in the repository, so the repository has to be on disk before it can be
/// found.** The first version of `.github/actions/publish` had `actions/checkout` as its own first
/// step - which cannot help, because the action is looked up before any of its steps run.
///
/// **It broke both callers identically, which is the argument for one shared definition rather than
/// against it.** One action used twice meant one missing step failed both the same way, and the fix
/// was one line in two places rather than a diagnosis.
///
/// # Why this is a check and `P-613` could not cover it
///
/// **`P-613` says a failing run publishes anyway.** Here the thing that failed *was* the publishing,
/// which is the one case that sentence cannot reach - **and a run where everything but publishing
/// succeeds looks, from outside, exactly like a run that published.** Sean had to ask.
///
/// **So the gate is where it has to be caught**, before a push rather than after one.
#[test]
fn every_job_using_a_local_action_checks_out_first() {
    let at = root().join(".github/workflows/pipeline.yml");
    let text = std::fs::read_to_string(&at).unwrap_or_else(|why| panic!("{}: {why}", at.display()));

    let mut job = String::new();
    let mut checked_out = false;
    let mut local: Vec<(String, String)> = Vec::new();
    let mut jobs = 0;
    for line in text.lines() {
        // A job opens at two spaces; nothing inside one does.
        if let Some(named) = line.strip_prefix("  ")
            && !named.starts_with(' ')
            && !named.starts_with('#')
            && let Some(named) = named.strip_suffix(':')
        {
            job = named.to_string();
            checked_out = false;
            jobs += 1;
            continue;
        }
        let bare = line.trim();
        if bare.starts_with("- uses:") || bare.starts_with("uses:") {
            let what = bare
                .trim_start_matches("- ")
                .trim_start_matches("uses:")
                .trim();
            if what.starts_with("actions/checkout") {
                checked_out = true;
            } else if what.starts_with("./") && !checked_out {
                local.push((job.clone(), what.to_string()));
            }
        }
    }

    assert!(
        jobs >= 5,
        "only {jobs} job(s) read from the workflow, so this said almost nothing"
    );
    // **Both populations.** A workflow using no local action would pass the assertion below
    // vacuously, and this check exists because two jobs use one.
    let using = text.matches("uses: ./.github/").count();
    assert!(
        using >= 2,
        "only {using} use(s) of a local action, so this is about almost nothing"
    );
    assert!(
        local.is_empty(),
        "these jobs use a local action without checking out first, so the action cannot be \
         found and the job fails having done nothing: {local:?}"
    );
}

/// **Every input a composite action declares is referenced by one of its steps.**
///
/// # The input was declared, documented twice, and passed to nothing
///
/// `S-261`. `.github/actions/publish/action.yml` took an `artifact` input *so two publishes in one
/// run do not collide*, said so in its header and again beside the job that calls it - **and
/// neither `upload-pages-artifact` nor `deploy-pages` was given it.** Both publishes uploaded under
/// the default name, and the second deploy refused:
///
/// ```text
/// Error: Multiple artifacts named "github-pages" were unexpectedly found for this
/// workflow run. Artifact count is 2.
/// ```
///
/// **The fast publish had already deployed**, so the site was current with yesterday's game and the
/// run was red - which is the designed fallback working and the thing it falls back from broken.
///
/// # Why a check rather than care
///
/// **An unused input is indistinguishable from a used one by reading the caller.** Both callers
/// passed `artifact:` and were correct to; nothing on that side could show that the value stopped
/// there. **And the comment described the intent**, which is `CLAUDE.md`'s *quoting a thing and
/// doing it are the same bytes* in the other direction: a sentence saying the artifact is named by
/// the caller was true of the design and false of the file, and read identically either way.
///
/// **This is the second defect in this action in four days and both were invisible locally.**
/// `S-257` was a checkout inside a local action; this is an input going nowhere. Nothing a lane can
/// run reaches either, so the check has to read the YAML.
#[test]
fn every_input_a_composite_action_declares_is_used() {
    let mut actions: Vec<std::path::PathBuf> = Vec::new();
    let root = std::path::Path::new("../../.github/actions");
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let file = entry.path().join("action.yml");
            if file.is_file() {
                actions.push(file);
            }
        }
    }
    assert!(
        !actions.is_empty(),
        "no composite action found under {}, so this checked nothing",
        root.display()
    );

    let mut checked = 0;
    for file in &actions {
        let text = std::fs::read_to_string(file).unwrap_or_else(|why| panic!("{file:?}: {why}"));
        let (head, body) = text
            .split_once("\nruns:")
            .unwrap_or_else(|| panic!("{file:?} declares no `runs:`, so it is not an action"));

        // **The input names, read from the `inputs:` block rather than from a list of my own.**
        let declared: Vec<String> = match head.split_once("\ninputs:") {
            None => Vec::new(),
            Some((_, rest)) => rest
                .lines()
                .take_while(|it| it.trim().is_empty() || it.starts_with("  "))
                .filter(|it| it.starts_with("  ") && !it.starts_with("    "))
                .filter_map(|it| it.trim().strip_suffix(':').map(str::to_string))
                .collect(),
        };
        assert!(
            !declared.is_empty(),
            "{file:?} declares no inputs, so this file proves nothing - delete it from the sweep \
             or give the check a reason to skip it"
        );

        for input in &declared {
            // **`inputs.<name>` in the steps, which is the only place a value can be spent.**
            let spent = format!("inputs.{input}");
            assert!(
                body.contains(&spent),
                "{file:?} declares the input `{input}` and no step uses `{spent}`, so the value \
                 stops at the boundary while both callers look correct - which is `S-261`"
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 2,
        "only {checked} input(s) across {} action(s), which is too few to be a check",
        actions.len()
    );
    println!(
        "{checked} declared input(s) across {} action(s), all used",
        actions.len()
    );
}

/// **Every job that deploys is one the push scripts watch, and every name they watch is such a
/// job.**
///
/// # The name had never been a job
///
/// `S-263`. Both scripts looked for a job called `Deploy to GitHub Pages`, which is a **step** inside
/// `.github/actions/publish`. No job has ever been called that, so nothing matched and every push
/// since the publish/republish split printed `NOT DEPLOYED` whatever happened.
///
/// **Run `37573381461` is the one that showed it**: `Republish` succeeded, a Pages deployment for
/// `273f4ac` landed inside the run's window, and the report said the site had not moved.
///
/// # Why reading it could not catch it
///
/// **The failing case and the working case print the same line.** A run that truly did not deploy
/// and a run whose deploy the script cannot see are one string, so the report was *right by
/// accident* on the previous run - where `Republish` had genuinely failed - which is the occasion
/// somebody looked at it.
///
/// **This is `S-261` in the other direction.** There a value was declared and never spent; here a
/// name was spent and never declared. Both were invisible from the side that looked correct.
///
/// # Both directions, because one of them is the cheap half
///
/// Asserting that each watched name exists would not have caught a *new* publishing job nobody
/// watches - which is the failure that arrives when the pipeline grows. So this asks both, and
/// asserts each population is not empty.
#[test]
fn every_job_that_deploys_is_one_the_push_script_watches() {
    let pipeline = std::fs::read_to_string("../../.github/workflows/pipeline.yml")
        .expect("the pipeline to read");

    // The jobs whose steps use the publish action, by the first word of their name - which is what
    // the scripts match on, because a full name carries punctuation two shells would have to quote.
    let mut deploying: Vec<String> = Vec::new();
    let mut seen_name: Option<String> = None;
    for line in pipeline.lines() {
        if let Some(rest) = line.strip_prefix("    name: ") {
            seen_name = Some(rest.trim().to_string());
        }
        if line.contains("uses: ./.github/actions/publish") {
            let name = seen_name
                .clone()
                .expect("a job to carry a name before its steps");
            let first = name
                .split_whitespace()
                .next()
                .expect("a name with a word in it")
                .to_string();
            if !deploying.contains(&first) {
                deploying.push(first);
            }
        }
    }
    assert!(
        deploying.len() >= 2,
        "found {} job(s) using the publish action, so this checked almost nothing: {deploying:?}",
        deploying.len()
    );

    // What each script watches, read from the script rather than restated here.
    let sh = std::fs::read_to_string("../../scripts/push.sh").expect("push.sh to read");
    let ps = std::fs::read_to_string("../../scripts/push.ps1").expect("push.ps1 to read");

    let watched_sh: Vec<String> = sh
        .lines()
        .find_map(|it| it.strip_prefix("DEPLOY_JOBS=\""))
        .map(|it| {
            it.trim_end_matches('"')
                .split_whitespace()
                .map(str::to_string)
                .collect()
        })
        .expect("push.sh to declare DEPLOY_JOBS");
    let watched_ps: Vec<String> = ps
        .lines()
        .find_map(|it| it.strip_prefix("$deployJobs = @("))
        .map(|it| {
            it.trim_end_matches(')')
                .split(',')
                .map(|piece| piece.trim().trim_matches('"').to_string())
                .filter(|piece| !piece.is_empty())
                .collect()
        })
        .expect("push.ps1 to declare $deployJobs");

    for (which, watched) in [("push.sh", &watched_sh), ("push.ps1", &watched_ps)] {
        assert!(
            !watched.is_empty(),
            "{which} watches nothing, so every push would report NOT DEPLOYED - which is `S-263`"
        );
        for job in &deploying {
            assert!(
                watched.contains(job),
                "`{job}` deploys and {which} does not watch it, so a push that moved the site \
                 will report that it did not: {watched:?}"
            );
        }
        for job in watched {
            assert!(
                deploying.contains(job),
                "{which} watches `{job}` and no job using the publish action is named that, so \
                 nothing will ever match it - which is `S-263` exactly: the name was \
                 `Deploy to GitHub Pages`, a step inside the action"
            );
        }
    }

    // **The two spellings agree**, because Sean runs the PowerShell one and the gate runs neither.
    let mut a = watched_sh.clone();
    let mut b = watched_ps.clone();
    a.sort();
    b.sort();
    assert_eq!(
        a, b,
        "the two push scripts watch different jobs, so they would report differently about the \
         same run"
    );
    println!(
        "{} deploying job(s), watched by both scripts: {a:?}",
        a.len()
    );
}
