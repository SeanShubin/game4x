//! The committed dump files are what the scenario produces now.
//!
//! **`S-29`'s free one, and it is free because it depends on nothing.** A generated file has
//! to be current whatever generates it, and five of the seven generated files in this
//! repository were held to nothing at all.
//!
//! **`S-40` made it nine, and `S-24`'s commands artifact eleven.** Every markdown report now has a page beside it, and a page is a
//! second thing that can go stale - one that is *harder* to notice, because nobody diffs
//! rendered HTML. The two `prototypes/kinds` writes are rendered here rather than there,
//! since nothing may depend on that crate, so their pages are held here too.
//!
//! **A generated file that nobody regenerates is worse than no generated file.**
//! `prototypes/kinds/tests/catalog_is_current.rs` says it first and it generalises: a
//! derived file reads as more authoritative than prose *because* it is derived, so it is
//! trusted harder while it goes stale. And it goes stale without changing, which is the same
//! shape as a check that stops running.
//!
//! It matters more than usual this week. Sean is deriving `scenario/commands/play.4x` by hand against
//! `state.md` and `turns.md`; a stale file would send him looking for an error in his
//! arithmetic that is really an error in the file.
//!
//! **Two of the seven are not checked here and both omissions are deliberate.**
//! `catalog.md` is `prototypes/kinds`', which already holds it. `pending.md` is written by
//! `hooks/pre-commit`, which **refuses** while any outbox has unstaged changes - so that it
//! never renders somebody's half-written finding. It can therefore be legitimately stale,
//! and requiring it to be current would fail on a correct refusal. Making the hook
//! unconditional to satisfy a test would be trading a real safeguard for a green light.

use std::path::{Path, PathBuf};

use game_console::{Library, dump};

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }

    fn names(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Every file carrying the generated marker, found on disk rather than listed here.
///
/// **This is the half a list cannot do.** A test that regenerates each *known* file and
/// compares it will pass while the program grows output nobody asked for, or while a file it
/// stopped producing sits on disk being read - because the list of known files is the thing
/// that went stale. So the set is discovered, and the marker is what makes discovery
/// possible: every generated file says `Generated. Do not edit.` in its own first lines.
///
/// `catalog.md` carries the marker and is excluded by name: `prototypes/kinds` produces it
/// and already holds it to being current. `pending.md` carries no marker, which is just as
/// well - `hooks/pre-commit` refuses to rewrite it while an outbox has unstaged changes, so
/// it can be correctly stale and does not belong in a currency comparison at all.
///
/// # Why only this directory
///
/// **`S-33`: the predicate is unbounded content matching, and the scope is what saves it.**
/// Any file containing that sentence matches - and `docs/notes/proposals.md` contains it
/// three times, because reporting a defect in a generated file's header means writing that
/// header down. The scan does not see it only because generated files all happen to live in
/// the repository root today, which is a fact nothing states.
///
/// So it is stated here, and the count below is what keeps it honest. **If a generated file
/// ever lands outside the root** - `P-224` says only *a file of its own* - widening this
/// scan picks up prose immediately, and the count fails rather than the extra file passing
/// as a dump. **If a hand-written root file ever quotes the marker**, the same. `README.md`
/// and `CLAUDE.md` are both in the root and neither carries it, but nothing stops one.
///
/// A marker that cannot be quoted in prose would be stronger and costs every generated file
/// a change. Not worth it while the count holds.
fn marked_on_disk(root: &Path) -> Vec<String> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(root.join("reports")) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        // `prototypes/kinds` produces `catalog.md` and holds it to being current itself. It
        // is excluded here rather than added, because this test is about the dumps and a file
        // belongs to whatever generates it. **`recipes.md` left this list with `R-7`**: it
        // carries a worked example under each rule now, which is a real command run against a
        // real state, so it is generated here and held here.
        if name == "catalog.md" {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        // Only the head, so a file *discussing* the marker is not mistaken for one
        // carrying it. **Eight lines, and it is a constant now rather than a measurement.**
        // Twelve found the three markdown dumps and silently missed both pages;
        // twenty-four then missed the three pages `S-40` added, whose marker fell at line
        // 27 under a stylesheet. Both times the window was tuned to where the marker
        // happened to land, and both times a page grew past it - the check narrowing while
        // looking unchanged, which is the failure this test exists to catch, arriving
        // inside the test itself. `dump::MARKER` now puts it on line two of every page, so
        // this number stops tracking anything and the class is closed.
        let head: String = text.lines().take(8).collect::<Vec<_>>().join(
            "
",
        );
        if head.contains("Generated. Do not edit.") {
            found.push(name);
        }
    }
    found.sort();
    found
}

/// The committed dumps are what the scenario produces, in all three directions.
///
/// **`missing`, `extra` and `different`, summed and asserted at zero.** Two of those a list
/// can find and one it cannot: `extra` is a file on disk that nothing produces any more, and
/// no amount of regenerating what is listed will notice it, because the list is what went
/// stale. That file goes on being read after it has stopped being true - and it reads as
/// authoritative, because it says it was generated.
#[test]
fn every_committed_dump_is_what_the_scenario_produces() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut generated = dump::generated(&Files(root.join("scenario/commands")));

    // **Thirty-nine, and the number moved for three reasons at once** - all of them `R-9`.
    // `containment.md` arrived, because every view has a diffable sibling now and that was
    // the one that did not. The twelve territories each got a page and a sibling, which is
    // twenty-four. And two stylesheets are generated files like any other, so that a stale
    // one fails the way a stale report does.
    //
    // **Stated as a sum rather than as a number**, because thirty-nine on its own says
    // nothing about which of the three moved when it next changes.
    assert_eq!(
        generated.len(),
        13 + 2 + 12 * 2,
        "twelve report files and the index, two stylesheets, and a page and a sibling for \
         each of the twelve territories; `dump::generated` returned {}",
        generated.len()
    );

    // **The two `prototypes/kinds` writes get their page held here**, because that crate
    // cannot use this renderer and nothing should depend on it. `catalog.md` is held to the
    // release by `prototypes/kinds/tests/catalog_is_current.rs`; this holds `catalog.html`
    // to `catalog.md`. Neither half alone says the page shows what the release says.
    for name in dump::RENDERED_ELSEWHERE {
        let at = root.join("reports").join(name);
        let markdown = std::fs::read_to_string(&at)
            .unwrap_or_else(|why| panic!("cannot read {}: {why}", at.display()));
        generated.push((
            dump::html_name(name).to_string(),
            dump::page(&markdown, name),
        ));
    }
    assert_eq!(
        generated.len(),
        13 + 2 + 12 * 2 + 1,
        "one more with `catalog.html`, which `prototypes/kinds` writes the markdown for"
    );

    let produced: std::collections::BTreeSet<String> =
        generated.iter().map(|(name, _)| name.to_string()).collect();
    let on_disk: std::collections::BTreeSet<String> = marked_on_disk(&root).into_iter().collect();

    let missing: Vec<&String> = produced.difference(&on_disk).collect();
    let extra: Vec<&String> = on_disk.difference(&produced).collect();

    let mut different = Vec::new();
    for (name, text) in &generated {
        let Ok(committed) = std::fs::read_to_string(root.join("reports").join(name)) else {
            continue; // counted as missing above
        };
        if &committed == text {
            continue;
        }
        let at_line = committed
            .lines()
            .zip(text.lines())
            .position(|(theirs, ours)| theirs != ours)
            .map(|n| n + 1);
        different.push(match at_line {
            Some(line) => format!(
                "{name} line {line}:\n      committed: {}\n      generates: {}",
                committed.lines().nth(line - 1).unwrap_or("").trim_end(),
                text.lines().nth(line - 1).unwrap_or("").trim_end()
            ),
            None => format!(
                "{name} is {} lines committed and {} generated",
                committed.lines().count(),
                text.lines().count()
            ),
        });
    }

    let wrong = missing.len() + extra.len() + different.len();
    assert_eq!(
        wrong,
        0,
        "the committed dumps and the scenario disagree:\n\n  \
         missing ({}): {missing:?}\n  \
         extra ({}): {extra:?} - on disk, marked generated, and produced by nothing\n  \
         different ({}):\n    {}\n\n\
         Run `cargo run -p game-console --bin dump-state` and commit the result in the same \
         commit as the change that caused it.",
        missing.len(),
        extra.len(),
        different.len(),
        different.join("\n    ")
    );

    // The set was discovered, so it can be empty for the wrong reason. This says it was not.
    assert_eq!(
        on_disk.len(),
        13 + 2 + 12 * 2 + 1,
        "the same population again, counted from the directory rather than from the \
         program - `catalog.md` is the one file excluded, and it is `prototypes/kinds`'. \
         Found {} ({on_disk:?})",
        on_disk.len()
    );
}

/// The scenario produces something, so the comparison above is not two empty strings.
///
/// **Every file being identical is the same green as every file being blank.** This is the
/// half that says the generator did any work, and it reads the content rather than the
/// count, because five empty files would satisfy a count of five.
#[test]
fn the_scenario_produces_tables_rather_than_empty_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut generated = dump::generated(&Files(root.join("scenario/commands")));
    for name in dump::RENDERED_ELSEWHERE {
        let markdown = std::fs::read_to_string(root.join("reports").join(name)).unwrap();
        generated.push((
            dump::html_name(name).to_string(),
            dump::page(&markdown, name),
        ));
    }
    assert_eq!(
        generated.len(),
        13 + 2 + 12 * 2 + 1,
        "the same population as the currency check above, and it is worth restating rather \
         than sharing: a helper that both read would make one number, and two checks over \
         one number is one check"
    );

    for (name, text) in &generated {
        // **Two shapes are short and both are correct.** A territory with nothing on it
        // carries a head, a heading, its neighbours and one table; and the reset is five
        // rules, because a reset that normalises everything is a dependency in disguise.
        // **A floor per shape rather than one lowered to fit**, which would stop the number
        // saying anything about the twenty files it was written for.
        let floor = match name.as_str() {
            "reset.css" => 4,
            other if other.starts_with("territory-") => 12,
            _ => 20,
        };
        assert!(
            text.lines().count() > floor,
            "{name} is {} lines, which is not a dump of anything",
            text.lines().count()
        );
        // **The index is a page of links, not of rows**, so *has it any content* is a
        // different question for it. Asking every HTML file for a `<td>` reported it empty
        // when it was full - a check whose one shape stopped fitting the moment a second
        // kind of page existed.
        // **The index is a page of links, not of rows**, so *has it any content* is a
        // different question for it. Asking every HTML file for a `<td>` reported it empty
        // when it was full - a check whose one shape stopped fitting the moment a second
        // kind of page existed. `catalog.html` is the third shape: prose and lists, and no
        // table anywhere in it, so `<td>` would have called it empty too.
        // `recipes.md` is the fourth shape and arrived with `R-7`: bullets for the rule and
        // fenced blocks for the worked example, and no table anywhere. What says it has
        // content is a heading per recipe.
        // **A stylesheet is a fifth shape** and has neither rows nor headings. What says
        // it has content is that it declares something.
        let marker = match name.as_str() {
            "index.html" => "<a href",
            "recipes.md" => "## ",
            "containment.md" => "- ",
            _ if name.ends_with(".css") => " { ",
            _ if name.ends_with(".html") => "<h1>",
            _ if name.starts_with("territory-") => "## ",
            _ => "| ",
        };
        assert!(
            text.contains(marker),
            "{name} contains no {marker:?}, so it has no rows in it"
        );
    }
}

/// Every page is a page: one `<head>`, closed, and one `<body>`, closed.
///
/// **This defect shipped and no check named it.** Extracting the shared stylesheet out of
/// `dump::html` carried `</head><body>` away with it, and `state.html` and `entities.html`
/// were written for a whole session with neither tag. They still rendered - a browser
/// recovers from that - so nothing looked wrong, and every content check went on passing
/// because every row was present and correct.
///
/// It was caught by `dump::tests` asserting that the *head* names no territory, which read
/// the head by splitting on `<body>`, found no `<body>`, and so compared the whole document.
/// That is a check reporting a true failure for a reason it was not written for, which is
/// luck rather than coverage. **A structural claim needs a structural check**, and this is
/// it: five pages, each asked directly.
#[test]
fn every_page_is_well_formed_enough_to_be_read_as_one() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut generated = dump::generated(&Files(root.join("scenario/commands")));
    for name in dump::RENDERED_ELSEWHERE {
        let markdown = std::fs::read_to_string(root.join("reports").join(name)).unwrap();
        generated.push((
            dump::html_name(name).to_string(),
            dump::page(&markdown, name),
        ));
    }

    let mut pages = 0;
    for (name, text) in &generated {
        if !name.ends_with(".html") {
            continue;
        }
        for tag in ["<html", "<head>", "</head>", "<body>", "</body>", "</html>"] {
            assert_eq!(
                text.matches(tag).count(),
                1,
                "{name} has {} of {tag:?}, and a page has exactly one",
                text.matches(tag).count()
            );
        }
        // Order, because six present tags in the wrong sequence is not a page either.
        let at = |tag: &str| text.find(tag).unwrap_or_default();
        assert!(
            at("<html") < at("<head>")
                && at("<head>") < at("</head>")
                && at("</head>") < at("<body>")
                && at("<body>") < at("</body>")
                && at("</body>") < at("</html>"),
            "{name} has the tags of a page in an order that is not one"
        );
        pages += 1;
    }
    // Over every case, and how many cases there were - or this passes on an empty list, as
    // it would have done for the whole session the defect above was live. `index.html` is a
    // page as much as the reports are, and it is one of the two that do not go through
    // `dump::page` - `containment.html` is the other, since `S-54` - so both are the
    // likeliest to drift from the rest.
    assert_eq!(
        pages,
        8 + 12,
        "eight pages and one per territory, and every one of them checked"
    );
}
