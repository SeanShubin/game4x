//! Every reference in a generated report is a link that resolves, and every view has a sibling.
//!
//! **`R-9`, and the clauses are its own words**: *every reference in a report is a link I can
//! follow to the thing it names; every generated view has a **diffable sibling** beside it, as
//! `graph.html` has `graph.txt`; and **no page needs JavaScript to be read**.*
//!
//! # The old check of this name went with the reports it was about
//!
//! **`D-4` deleted it in `e40325c2`** along with the reports generated from the old ruleset's
//! scenario. **The clauses did not go with them** - Sean, 2026-09-28: *I am fine with the data
//! going away, but the organizational structure was still applicable.* So this is that check
//! again, over what the index points at now.
//!
//! **A link that resolves on the published site and not in a clone is what `R-11` records
//! confusing him.** These are checked against the repository, which is where a reader of a clone
//! follows them to.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// **The generator, borrowed rather than run as a subprocess** - see `write_all`.
#[path = "../examples/index.rs"]
#[allow(dead_code)]
mod index;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn reports() -> PathBuf {
    root().join("reports")
}

/// Every `href` of every generated page, with the page it came from.
fn links() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(reports()).expect("reports/") {
        let path = entry.expect("an entry").path();
        if path.extension().and_then(|it| it.to_str()) != Some("html") {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|it| it.to_str())
            .unwrap_or_default()
            .to_string();
        let text = std::fs::read_to_string(&path).expect("a page");
        let mut rest = text.as_str();
        while let Some(at) = rest.find("href=\"") {
            rest = &rest[at + 6..];
            let Some(end) = rest.find('"') else { break };
            out.push((name.clone(), rest[..end].to_string()));
            rest = &rest[end..];
        }
    }
    out
}

/// **Every reference is a link I can follow to the thing it names.**
#[test]
fn every_link_in_every_report_resolves() {
    let found = links();
    // **A count over nothing is the same failure with the sign flipped** - `CLAUDE.md`. With no
    // pages read, every link would resolve vacuously.
    assert!(
        found.len() > 100,
        "only {} link(s) across the reports, so this checked almost nothing",
        found.len()
    );

    let missing: Vec<String> = found
        .iter()
        .filter(|(_, at)| !reports().join(at).exists())
        .map(|(page, at)| format!("{page} -> {at}"))
        .collect();
    assert!(
        missing.is_empty(),
        "{} link(s) point at nothing, so a reader following one arrives nowhere:\n  {}",
        missing.len(),
        missing.join("\n  ")
    );

    // **And they reach outside `reports/`, which is the half a self-contained page would pass.**
    // An index of the repository that only ever linked its own directory would satisfy the
    // assertion above and index nothing.
    let outward = found.iter().filter(|(_, at)| at.starts_with("../")).count();
    assert!(
        outward > 50,
        "only {outward} link(s) leave `reports/`, so the index is not indexing the repository"
    );
}

/// **Every generated view has a diffable sibling beside it.**
#[test]
fn every_page_has_a_markdown_sibling() {
    let mut pages = BTreeSet::new();
    for entry in std::fs::read_dir(reports()).expect("reports/") {
        let path = entry.expect("an entry").path();
        if path.extension().and_then(|it| it.to_str()) == Some("html") {
            pages.insert(
                path.file_stem()
                    .and_then(|it| it.to_str())
                    .unwrap_or_default()
                    .to_string(),
            );
        }
    }
    assert!(
        pages.len() >= 5,
        "only {} page(s), so this is not about the reports",
        pages.len()
    );
    for page in &pages {
        let sibling = reports().join(format!("{page}.md"));
        assert!(
            sibling.is_file(),
            "`{page}.html` has no `{page}.md` beside it, and `R-9` asks that every generated \
             view have a diffable sibling"
        );
    }
}

/// **No page needs JavaScript to be read.**
///
/// **A stylesheet is not a script**, which is why `report.css` is allowed and nothing here looks
/// for it. What this refuses is a page that does something when opened.
///
/// # It read eight pages of three hundred and sixty-nine
///
/// `read_dir` does not recurse, so this looked at the top level of `reports/` and nothing below
/// it. **Measured when `S-254` found it: 8 read, 361 never looked at** - and the floor was
/// `looked >= 5`, over a population of 8, so it could not notice it was missing 98% of its
/// subject.
///
/// **The one scripted page among the 361 is the one this lane put there.** `S-249` moved the
/// review page under `reports/` and the flat read made it exempt by accident - and `S-249`'s
/// reason for reading flat was about *links*, which resolve relative to a page's own directory.
/// **That reason does not carry over to scripts**, and nobody noticed it was being borrowed.
///
/// **So the floor is derived now.** It counts the `.html` files on disk and asserts every one was
/// read, which is the only version of this that cannot go quiet when the tree grows a directory.
#[test]
fn no_page_carries_a_script() {
    // **Legal under `R-9`, exempt by name, with the reason written down.** `R-9` asks that no page
    // *need* JavaScript to be read, and the review page does not: with the `<script>` deleted it
    // still renders every test, every case and every verdict - the script adds the controls that
    // write, which is `E-4` and is not reading.
    //
    // **Named rather than skipped by shape**, so that a second scripted page is a failure and not
    // a precedent. The assertion below refuses an exemption that has stopped being needed.
    // **The review application's pages, which is a directory rather than a name** - `E-6` split
    // the one page into an index and a page per category, and a named exemption went stale the
    // moment it did: `review/index.html` stopped carrying test cards, so *renders its tests
    // without a script* became false of it and true of `review/rule.html`.
    //
    // **A list of six slugs would go stale the same way** the next time a suite is added, so the
    // exemption is the directory and **each page in it is checked individually**: it must render
    // something a reader can read without the script, or it is a page that needs one.
    const ALLOWED: &str = "review/";

    let mut pages: Vec<(String, String)> = Vec::new();
    collect_pages(&reports(), &reports(), &mut pages);

    let on_disk = pages.len();
    let mut read = 0;
    let mut used: Vec<String> = Vec::new();
    for (at, text) in &pages {
        if at.starts_with(ALLOWED) {
            // **An exemption is checked, not waved through.** The page must render something
            // without its script - items, cases or the list of categories - which is what `R-9`
            // actually asks: that no page *need* JavaScript to be read.
            let renders = ["data-test=", "data-case=", "data-category="]
                .iter()
                .any(|it| text.contains(it));
            assert!(
                text.contains("<body") && renders,
                "`{at}` is exempt from the script rule and renders nothing without one, so the                  exemption has stopped being true of it"
            );
            used.push(at.clone());
            read += 1;
            continue;
        }
        for forbidden in ["<script", "javascript:", " onclick=", " onload="] {
            assert!(
                !text.contains(forbidden),
                "`{at}` carries `{forbidden}`, and `R-9` asks that no page need JavaScript \
                 to be read"
            );
        }
        read += 1;
    }

    // **Every page on disk, not a floor somebody chose.** A written-down minimum is what let this
    // pass over eight of three hundred and sixty-nine.
    assert_eq!(read, on_disk, "a page on disk was not read");
    assert!(
        on_disk > 100,
        "only {on_disk} page(s) under reports/, so this refused almost nothing"
    );
    // **And an exemption nobody used is a stale exemption**, which would quietly permit a second
    // page the day somebody reused the name.
    // **The exemption matched something**, or it has stopped being about anything and a page
    // under `review/` could carry a script with nothing noticing.
    assert!(
        used.len() >= 2,
        "only {} page(s) under `{ALLOWED}`, so the exemption is about almost nothing: {used:?}",
        used.len()
    );
}

/// Every `.html` under `reports/`, as the path it has beneath it.
///
/// **Recursive, which the reading this replaced was not.** `reports/foundation/` alone holds 63
/// pages and `reports/spec/tests/rule/` holds more; neither was ever read by the check above.
fn collect_pages(root: &Path, at: &Path, into: &mut Vec<(String, String)>) {
    let Ok(entries) = std::fs::read_dir(at) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_pages(root, &path, into);
        } else if path.extension().and_then(|it| it.to_str()) == Some("html")
            && let Ok(under) = path.strip_prefix(root)
            && let Ok(text) = std::fs::read_to_string(&path)
        {
            into.push((under.to_string_lossy().replace('\\', "/"), text));
        }
    }
}

/// **The committed reports are what the generator writes**, so a stale page fails here.
///
/// **This is `dumps_are_current`'s shape and the reason the old reports rotted without it.** A
/// generated file nobody compares is one that describes whatever the tree looked like when
/// somebody last remembered to run something.
#[test]
fn every_committed_report_is_what_the_generator_writes() {
    let before: Vec<(PathBuf, String)> = std::fs::read_dir(reports())
        .expect("reports/")
        .filter_map(|it| it.ok())
        .map(|it| it.path())
        .filter(|it| {
            matches!(
                it.extension().and_then(|e| e.to_str()),
                Some("html") | Some("md") | Some("css")
            )
        })
        .map(|at| {
            let text = std::fs::read_to_string(&at).unwrap_or_default();
            (at, text)
        })
        .collect();
    assert!(
        before.len() >= 11,
        "only {} generated file(s) in `reports/`, so this compared almost nothing",
        before.len()
    );

    index::write_all();

    let moved: Vec<String> = before
        .iter()
        .filter(|(at, was)| std::fs::read_to_string(at).unwrap_or_default() != *was)
        .map(|(at, _)| said(at))
        .collect();
    assert!(
        moved.is_empty(),
        "{} committed report(s) are not what the generator writes - run \
         `scripts/reports.sh` and commit the result:\n  {}",
        moved.len(),
        moved.join("\n  ")
    );
}

fn said(at: &Path) -> String {
    at.file_name()
        .and_then(|it| it.to_str())
        .unwrap_or("?")
        .to_string()
}

/// **Nothing the engine reads is missing from the index** - `R-11`.
///
/// **Asked of the engine, not of a list here.** `foundation::READ_BY_A_RUN` is checked against
/// `data/foundation/` itself, so a sixth input fails this the day it is added rather than the day
/// somebody notices the page is short. `R-11`: *checked by listing the inputs rather than by
/// anybody remembering to add one.*
///
/// **It was `PATHS` and `PATHS` is three of the five** - `S-220`. Those three are what
/// `include_str!` carries; `setup.4x` is read every time a test runs and fetches `script.4x`. The
/// check passed, because it was a true answer about a narrower population than the clause names.
///
/// **And reading it is what following the link does.** Every input is reachable as a rendering,
/// which the browser shows, rather than as a `.4x` it would offer to download - which is the
/// clause Sean added in September after the published site handed him a file instead of a page.
#[test]
fn every_file_the_engine_reads_is_reachable_from_the_index() {
    let inputs = game_model::foundation::READ_BY_A_RUN;
    assert!(
        !inputs.is_empty(),
        "the engine reads nothing, so this checked nothing"
    );

    let pages: String = std::fs::read_dir(reports())
        .expect("reports/")
        .filter_map(|it| it.ok())
        .map(|it| it.path())
        .filter(|it| it.extension().and_then(|e| e.to_str()) == Some("html"))
        .map(|it| std::fs::read_to_string(it).unwrap_or_default())
        .collect();

    for at in inputs {
        let rendering = format!("crates/game-model/{at}.html");
        assert!(
            pages.contains(&rendering),
            "the engine reads `{at}` and no page links `{rendering}` - `R-11` asks that nothing \
             it reads be missing from the index"
        );
        assert!(
            reports().join(&rendering).is_file(),
            "`{rendering}` is linked and not there"
        );
    }

    // **And the rendering says what it is**, which is the other half of the clause: a copy that
    // does not say it is a copy is one a reader may take for the file.
    for at in inputs {
        let page = std::fs::read_to_string(reports().join(format!("crates/game-model/{at}.html")))
            .expect("a rendering");
        assert!(
            page.contains("Not canonical") && page.contains(at),
            "the rendering of `{at}` does not say it is generated and not canonical, and which \
             file it came from"
        );
    }
}

/// **Every line of the playthrough is one the renderer knows what to do with.**
///
/// `S-233` asked for `scenario/played.md` to have an HTML rendering, and it is rendered by a
/// markdown renderer of about eighty lines rather than by a library. **That is honest only
/// while the document uses nothing the renderer does not handle** - no fenced block, no
/// bullet, no table - so this asserts the closed set rather than trusting the count taken on
/// the day it was written.
///
/// **Each kind is asserted non-empty as well as the total.** A document that became one `<pre>`
/// would have every line in one bucket and pass a check that only summed them.
#[test]
fn every_line_of_the_playthrough_is_one_this_renderer_knows() {
    let text = std::fs::read_to_string(root().join("scenario/played.md"))
        .expect("scenario/played.md is generated by scripts/scenario.sh");
    let mut blank = 0;
    let mut heading = 0;
    let mut preformatted = 0;
    let mut prose = 0;
    for line in text.lines() {
        match index::kind_of(line) {
            index::Line::Blank => blank += 1,
            index::Line::Heading(_) => heading += 1,
            index::Line::Preformatted => preformatted += 1,
            index::Line::Prose => prose += 1,
        }
    }
    assert_eq!(
        blank + heading + preformatted + prose,
        text.lines().count(),
        "a line was counted twice or not at all"
    );
    for (what, how_many) in [
        ("blank", blank),
        ("heading", heading),
        ("preformatted", preformatted),
        ("prose", prose),
    ] {
        assert!(how_many > 0, "no {what} line, so that arm proves nothing");
    }

    // **The constructs the renderer would get wrong, asserted absent rather than assumed.**
    // A bullet renders as prose and loses its list; a fence renders as prose and loses its
    // block; a table renders as prose and loses every cell. **Each would look like text on
    // the page rather than fail**, which is why they are named here.
    for (what, found) in [
        (
            "a fenced code block",
            text.lines().any(|l| l.starts_with("```")),
        ),
        (
            "a bullet",
            text.lines()
                .any(|l| l.starts_with("- ") || l.starts_with("* ")),
        ),
        ("a table row", text.lines().any(|l| l.starts_with('|'))),
    ] {
        assert!(
            !found,
            "`scenario/played.md` now has {what}, which this renderer turns into prose - \
             see `S-233`"
        );
    }
}

/// **Every section of the playthrough is reachable from the top of its page.**
///
/// `S-233`: *a rendering that makes him scroll past four turns to reach `What fired` is worse
/// than the markdown he has.* It is 718 lines with five per-turn world dumps, so the contents
/// list is the thing that makes the page worth having - and a list that silently lost a
/// section would leave exactly the scroll the item is about.
#[test]
fn every_section_of_the_playthrough_is_linked_from_its_own_page() {
    let text =
        std::fs::read_to_string(root().join("scenario/played.md")).expect("scenario/played.md");
    let page = std::fs::read_to_string(reports().join("scenario/played.md.html"))
        .expect("the rendering is generated by `cargo run --example index`");

    let sections: Vec<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .map(|said| said.trim().to_string())
        .collect();
    assert!(
        sections.len() >= 5,
        "only {} section(s) parsed out of played.md; its shape has changed",
        sections.len()
    );

    let missing: Vec<&String> = sections
        .iter()
        .filter(|said| !page.contains(&format!(">{said}</a>")))
        .collect();
    assert!(
        missing.is_empty(),
        "sections of played.md that its page does not link: {missing:?}"
    );
    // **And every link reaches an anchor that is there**, which is the other half: a contents
    // list of nine entries pointing at nothing is the same scroll with extra steps.
    let anchors = page.matches("<h2 id=\"").count();
    assert_eq!(
        anchors,
        sections.len(),
        "{} contents entr(ies) and {anchors} anchor(s)",
        sections.len()
    );
}

/// **Nothing the markdown says is dropped on the way to the page.**
///
/// A renderer that quietly skipped a kind of line would still produce a page with a contents
/// list, and the two checks above would pass. **This compares the words.**
#[test]
fn the_playthrough_page_says_what_the_markdown_says() {
    let text =
        std::fs::read_to_string(root().join("scenario/played.md")).expect("scenario/played.md");
    let page =
        std::fs::read_to_string(reports().join("scenario/played.md.html")).expect("the rendering");
    // **The page with its markup taken off, rather than the lines that happen to have none.**
    // The first version skipped any line holding a backtick or an angle bracket and still
    // failed on 422 of them: an indented line is syntax-marked by `lit`, so `{adjacency ...}`
    // reaches the page as a dozen spans and no contiguous copy of itself. **Comparing the
    // text to the text is the question; comparing it to the markup was a narrower one.**
    let mut flat = String::new();
    let mut inside = false;
    for ch in page.chars() {
        match ch {
            '<' => inside = true,
            '>' => inside = false,
            _ if !inside => flat.push(ch),
            _ => {}
        }
    }
    let flat = flat
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&");

    let mut compared = 0;
    let mut absent: Vec<&str> = Vec::new();
    for line in text.lines() {
        let said = line.trim();
        // Emphasis marks do not survive, by design - `**x**` becomes a tag and the marks go.
        if said.is_empty() || said.starts_with('#') || said.contains(['`', '*']) {
            continue;
        }
        compared += 1;
        if !flat.contains(said) {
            absent.push(said);
        }
    }
    assert!(
        compared > 200,
        "only {compared} line(s) were comparable, so this proves little"
    );
    assert!(
        absent.is_empty(),
        "{} line(s) of played.md are not on its page, the first being {:?}",
        absent.len(),
        absent.first()
    );
}
