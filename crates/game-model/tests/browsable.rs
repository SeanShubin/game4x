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
#[test]
fn no_page_carries_a_script() {
    let mut looked = 0;
    for entry in std::fs::read_dir(reports()).expect("reports/") {
        let path = entry.expect("an entry").path();
        if path.extension().and_then(|it| it.to_str()) != Some("html") {
            continue;
        }
        let name = path.file_name().and_then(|it| it.to_str()).unwrap_or("?");
        let text = std::fs::read_to_string(&path).expect("a page");
        for forbidden in ["<script", "javascript:", " onclick=", " onload="] {
            assert!(
                !text.contains(forbidden),
                "`{name}` carries `{forbidden}`, and `R-9` asks that no page need JavaScript \
                 to be read"
            );
        }
        looked += 1;
    }
    assert!(
        looked >= 5,
        "only {looked} page(s) were read, so this refused almost nothing"
    );
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
