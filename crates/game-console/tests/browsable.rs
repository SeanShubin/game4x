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
