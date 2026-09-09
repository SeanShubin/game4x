//! `R-9`'s three conditions, each asked of every generated file.
//!
//! **Vetted when** - every reference in a report is a link I can follow to the thing it
//! names; every generated view has a diffable sibling beside it; and no page needs JavaScript
//! to be read. Three claims about a set of files, so each is checked over the whole set with
//! the set's size asserted - a claim of *no page needs JavaScript* is satisfied by there being
//! no pages, and this repository has recorded that failure often enough to name it.
//!
//! **What this cannot check is the third condition's second half**: *a view that filters is a
//! page that was generated, so the filter is a URL rather than a click*. There is no filtered
//! view yet - `S-64` says to build one only where a population is unreadable, which today is
//! `turns.html` at a hundred tables - so the rule has nothing to be true of. Said here rather
//! than asserted as a zero, because a zero against an empty population is not evidence.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use game_console::{Library, browse, dump};

struct Files(PathBuf);

impl Library for Files {
    fn fetch(&self, name: &str) -> Option<String> {
        std::fs::read_to_string(self.0.join(format!("{name}.4x"))).ok()
    }
}

fn everything() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut out = dump::generated(&Files(root.join("scenario/commands")));
    for name in dump::RENDERED_ELSEWHERE {
        let markdown = std::fs::read_to_string(root.join("reports").join(name))
            .unwrap_or_else(|why| panic!("cannot read {name}: {why}"));
        out.push((
            dump::html_name(name).to_string(),
            dump::page(&markdown, name),
        ));
        // **And its markdown, which `dump::generated` does not return.** It is
        // `prototypes/kinds`' file, so this crate renders it and does not produce it - and
        // the first version of this test therefore reported `catalog.html` as a view with
        // no sibling and `index.html` as linking to a file nothing generates. Both were
        // true of the list and false of the directory. **A check whose population is
        // narrower than its claim** is the shape this repository has recorded five times,
        // and it caught itself here.
        out.push((name.to_string(), markdown));
    }
    out
}

/// No page needs a script, and none has one.
///
/// **Both, because they are different claims.** *Needs* is about whether the page works
/// without one, which is a fact about the design; *has* is a fact about the bytes, and it is
/// the one a check can hold. The tree collapses with `<details>` and the index filters
/// nothing, so having none is evidence for needing none.
#[test]
fn no_page_carries_a_script_or_a_handler() {
    let files = everything();
    let mut pages = 0;
    for (name, text) in &files {
        if !name.ends_with(".html") {
            continue;
        }
        pages += 1;
        for forbidden in [
            "<script",
            "javascript:",
            " onclick=",
            " onload=",
            " onchange=",
        ] {
            assert!(
                !text.contains(forbidden),
                "{name} contains {forbidden:?}, and `R-9` says no page needs JavaScript to \
                 be read"
            );
        }
    }
    assert_eq!(
        pages, 20,
        "twenty pages were asked, and a claim about no pages is not a claim"
    );
}

/// Every view has a diffable sibling beside it, and every sibling has its view.
///
/// **Both directions.** A page with no sibling is the gap `R-9` names; a sibling with no page
/// is a file nobody browses to, which is how `recipes.md` was listed twice on the index and
/// went unnoticed for a week.
#[test]
fn every_view_has_a_diffable_sibling() {
    let files = everything();
    let names: Vec<&String> = files.iter().map(|(name, _)| name).collect();
    let mut paired = 0;
    for name in &names {
        // The index is the page you browse *from* rather than a view of anything, and a
        // stylesheet is not a view either.
        if !name.ends_with(".html") || *name == "index.html" {
            continue;
        }
        let sibling = format!("{}.md", name.trim_end_matches(".html"));
        assert!(
            names.iter().any(|it| **it == sibling),
            "{name} has no {sibling} beside it, and `R-9` requires one of every view"
        );
        paired += 1;
    }
    assert_eq!(
        paired, 19,
        "nineteen views were asked - seven reports and twelve territories"
    );

    for name in &names {
        if !name.ends_with(".md") {
            continue;
        }
        let page = format!("{}.html", name.trim_end_matches(".md"));
        assert!(
            names.iter().any(|it| **it == page),
            "{name} is a sibling of nothing, so no page is diffed by it"
        );
    }
}

/// Every link a page makes lands on a file that exists, and on an anchor that is in it.
///
/// **The half that matters is the anchor.** A link to a file is checked by the file being
/// missing, which is loud; a link to `catalog.html#pioneer` in a file whose headings carry no
/// ids is silent - the browser opens the page and does nothing, which reads exactly like the
/// link working. Both sides of that are generated from `browse::slug`, and this is what says
/// they still agree.
#[test]
fn every_link_lands_on_something_that_is_there() {
    let files = everything();
    let mut checked = 0;
    for (name, text) in &files {
        if !name.ends_with(".html") {
            continue;
        }
        for (at, _) in text.match_indices("href=\"") {
            let rest = &text[at + 6..];
            let Some(end) = rest.find('"') else { continue };
            let target = &rest[..end];
            // The scenario's own files are linked from the index by relative path and are
            // not generated; they are checked by being on disk rather than in this list.
            if target.starts_with("../") {
                continue;
            }
            let (file, anchor) = match target.split_once('#') {
                Some((file, anchor)) => (file, Some(anchor)),
                None => (target, None),
            };
            let found = files
                .iter()
                .find(|(named, _)| named == file)
                .unwrap_or_else(|| panic!("{name} links to {file}, which nothing generates"));
            if let Some(anchor) = anchor {
                assert!(
                    found.1.contains(&format!("id=\"{anchor}\"")),
                    "{name} links to {target}, and {file} has no id {anchor:?}"
                );
            }
            checked += 1;
        }
    }
    assert!(
        checked > 200,
        "only {checked} links were followed, which is too few for this to be about the \
         reports rather than about one of them"
    );
}

/// Every column of every table is either linked or named as deliberately not.
///
/// **This is the half a link check cannot do.** Following the links that exist says nothing
/// about the reference that was never made one, and a column added tomorrow renders as plain
/// text and looks exactly like a column that was considered. So the columns are enumerated
/// against the two lists, and a new one fails until somebody decides which it is.
#[test]
fn every_column_is_either_a_reference_or_declared_not_to_be() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let files = Files(root.join("scenario/commands"));
    let mut session = game_console::Session::new();
    for line in ["{run file:setup}", "{start}", "{run file:play}"] {
        session
            .run(line, &files)
            .unwrap_or_else(|why| panic!("`{line}` failed: {why}"));
    }

    let sections = dump::normalized_sections(&session.game);
    assert!(
        sections.len() > 5,
        "{} tables, which is too few to be the state",
        sections.len()
    );

    let mut columns = 0;
    let mut linked = 0;
    let mut declared = 0;
    for section in &sections {
        for column in &section.columns {
            columns += 1;
            // A value is needed to ask, and any non-empty one gives the same answer -
            // `browse::reference` decides by the column and never by the cell.
            let reference = browse::reference(&section.name, column, "1").is_some();
            let named = browse::UNLINKED
                .iter()
                .any(|(table, it)| *table == section.name && it == column);
            assert!(
                reference != named,
                "`{}`.`{column}` is {}, and every column is exactly one of the two",
                section.name,
                if reference {
                    "both linked and named as not a reference"
                } else {
                    "neither linked nor named as not a reference - decide which it is, in \
                     `browse::reference` or in `browse::UNLINKED`"
                }
            );
            if reference {
                linked += 1;
            } else {
                declared += 1;
            }
        }
    }
    assert_eq!(
        columns,
        linked + declared,
        "every column is accounted for exactly once"
    );
    assert!(
        linked > 0 && declared > 0,
        "{linked} linked and {declared} not, and a partition with an empty side means the \
         rule was never exercised"
    );
    assert_eq!(
        declared,
        browse::UNLINKED.len(),
        "every column named as not a reference is a column that exists; the list has {} and \
         {declared} were found",
        browse::UNLINKED.len()
    );
}

/// Everything the reports link outward to, so publishing them can carry it - `S-80`.
///
/// **The pipeline copies `reports/` and `scenario/` into the Pages artifact**, side by side,
/// because two report links leave the directory: `../scenario/commands/play.4x` and
/// `../scenario/expected/play.4x`. Every other link is relative and inside, so any subpath
/// serves them.
///
/// **A third outward directory would 404 on the published page and work perfectly in a
/// clone**, which is the failure this exists to make loud - the deploy is the one place the
/// gate does not reach, and the gate is where this belongs instead.
///
/// It asserts the set rather than a count, because a count would go on passing if one
/// outward link were replaced by a different one.
///
/// # Paths rather than directories - `X-16`
///
/// **Whole paths, because two kinds of thing are published now.** `scenario/` is copied
/// whole; the research lens's three files are copied by name, so that publishing one file
/// out of `lenses/` does not publish a lens's working directory. A check on the first path
/// segment would have accepted `../lenses/quality/outbox.md` and let it 404, which is the
/// narrower-predicate failure this repository has recorded often enough to expect.
///
/// **Both directions, so a stale entry is as loud as a missing one.** Every path linked must
/// be published, and everything named as published must be linked and must be on disk -
/// otherwise a file that moved leaves a copy line copying nothing and a check agreeing.
#[test]
fn the_reports_point_outward_only_at_what_the_pipeline_publishes() {
    /// Directories the pipeline copies whole beside `reports/`.
    const PUBLISHED_WHOLE: [&str; 1] = ["scenario"];

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let files = everything();
    assert!(
        files.len() > 20,
        "only {} report files, so this would agree with anything",
        files.len()
    );

    let mut outward: BTreeSet<String> = BTreeSet::new();
    let mut links = 0;
    for (name, text) in &files {
        if !name.ends_with(".html") && !name.ends_with(".md") {
            continue;
        }
        for at in text.match_indices("../") {
            links += 1;
            // **The whole path, not its first segment.** `/` is deliberately not a
            // terminator: a file published by name is only published at the name it was
            // published under.
            let rest = &text[at.0 + 3..];
            let end = rest
                .find(['"', ')', '\'', ' ', '\n', '<'])
                .unwrap_or(rest.len());
            outward.insert(rest[..end].to_string());
        }
    }
    assert!(
        links > 0,
        "no report links outward at all, so this checked nothing - the two that should are \
         `../scenario/commands/play.4x` and `../scenario/expected/play.4x`"
    );

    // **The pipeline is read rather than trusted.** The list below used to carry a comment
    // saying that adding a name here was a promise the workflow copied it too, and a promise
    // is what goes quiet.
    //
    // **What is read is where each `cp` puts things, not whether the file mentions a path.**
    // The first version of this asked `pipeline.contains(path)` and stayed green with the
    // copy line deleted, because the same path is in the `test -f` line beside it and in the
    // comment above it - a right answer about the wrong question, which is the shape
    // `CLAUDE.md` records four times. Three mutations found it: deleting either copy left it
    // green, and only a link to an unpublished file failed.
    //
    // **It still cannot see the copy happen**; it sees a `cp` whose destination is the
    // artifact path. The `test -f` lines in that step are the half that runs.
    let pipeline = std::fs::read_to_string(root.join(".github/workflows/pipeline.yml"))
        .expect("the pipeline that publishes the artifact");
    let copied: BTreeSet<&str> = pipeline
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("cp "))
        .filter_map(|line| line.split_whitespace().last())
        .collect();
    assert!(
        !copied.is_empty(),
        "the pipeline copies nothing into the artifact, so either it changed shape or this \
         no longer reads it"
    );

    // **A path is under a directory published whole, or it is not.** The two halves are
    // checked differently because they are different promises: a directory published whole
    // carries whatever is in it, so any number of links may reach into it; a file published
    // by name carries only itself.
    let under = |target: &str| {
        PUBLISHED_WHOLE
            .iter()
            .any(|directory| target.starts_with(&format!("{directory}/")))
    };

    // **Set equality on the by-name half, in both directions at once.** A link to a file
    // nothing copies fails here, and so does a copy line outliving the link that wanted it -
    // the second being the one nothing else would notice, because a file copied and never
    // read looks exactly like a file copied and read.
    let linked_by_name: BTreeSet<String> = outward
        .iter()
        .filter(|target| !under(target))
        .cloned()
        .collect();
    let published_by_name: BTreeSet<String> = dump::RESEARCH
        .iter()
        .map(|(path, _)| (*path).to_string())
        .collect();
    assert_eq!(
        linked_by_name, published_by_name,
        "the reports link outward at {linked_by_name:?} outside the directories published \
         whole, and the pipeline publishes {published_by_name:?} by name - anything in the \
         first and not the second is a link that works in a clone and 404s on the published \
         page"
    );

    // **No count over the directory half, deliberately.** A third file linked inside
    // `scenario/` is already published and would fail a count, and the fix for that would be
    // to raise the number - which is how a duplicate on the index survived a week. What is
    // asserted instead is that each published directory is reached by something, so a copy
    // nobody links is as loud as a link nobody copies.
    for directory in PUBLISHED_WHOLE {
        assert!(
            outward
                .iter()
                .any(|target| under(target) && target.starts_with(&format!("{directory}/"))),
            "`{directory}` is published beside the reports and nothing links into it, so the \
             copy is carrying a directory no reader reaches"
        );
        let into = format!("crates/game4x/dist/{directory}");
        assert!(
            copied.contains(into.as_str()),
            "`{directory}` is named as published whole and no `cp` in \
             `.github/workflows/pipeline.yml` has `{into}` as its destination"
        );
    }

    for (path, _) in dump::RESEARCH {
        assert!(
            root.join(path).is_file(),
            "`{path}` is published and linked, and is not in the repository"
        );
        let into = format!("crates/game4x/dist/{path}");
        assert!(
            copied.contains(into.as_str()),
            "`{path}` is linked from the index and no `cp` in \
             `.github/workflows/pipeline.yml` has `{into}` as its destination, so it would \
             404 on the published page"
        );
    }

    assert!(
        !linked_by_name.is_empty() && outward.len() > linked_by_name.len(),
        "both halves have to be exercised or one of the two rules was never asked: \
         {outward:?} outward, of which {linked_by_name:?} are published by name"
    );
}
