//! `spec touching <file>` is the promotion rule's other half, and it could not see a capability.
//!
//! **`CLAUDE.md`**: *after promoting, check the index for open items that cite the destination
//! file - `spec touching <file>` lists them, over every outbox - and tell their owner.* On
//! 2026-09-24 that sentence was corrected to name this tool, and within the hour the code lane
//! found that the tool asked a narrower question than the sentence: `status == "open"`, which a
//! `built` capability is not.
//!
//! **The population is the repository rather than a fixture**, which is this tool's own rule: a
//! check that reads a copy of the population is checking the copy.
//!
//! **This test was itself the failure it was written about, three hours after being written.**
//! It asserted that `touching("spec/control.md")` returns `R-6` - one case, named by its
//! present state. `P-556` moved that file to `spec/future/control.md` and `R-6`'s `In` line
//! moved with it, so the check went red **while the tool it checks was working perfectly.**
//! `CLAUDE.md` says why in advance: *a check that pins the present state cannot report a gap
//! against what should be*, and *check the rule over every case, not on one case*.
//!
//! **So it now derives the path from the item instead of naming one.** Every outstanding item,
//! every specification file its own body names, and the count of pairs asserted - because a
//! rule checked over every case passes for the wrong reason when there are no cases.

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the repository root")
        .to_path_buf()
}

/// Every `spec/...` or `releases/...` file the text names, in order, without duplicates.
///
/// **Read out of the prose rather than from a list**, so that an item repointed at a file that
/// did not exist when this was written is covered the day it is repointed.
fn files_named(text: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for prefix in ["spec/", "releases/"] {
        let mut rest = text;
        while let Some(at) = rest.find(prefix) {
            let tail = &rest[at..];
            let end = tail
                .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '/' | '.')))
                .unwrap_or(tail.len());
            let path = tail[..end].trim_end_matches('.').to_string();
            // A file rather than a directory, and only the forms this repository states in.
            if (path.ends_with(".md") || path.ends_with(".4x")) && !found.contains(&path) {
                found.push(path);
            }
            rest = &tail[end.max(1)..];
        }
    }
    found
}

/// **Every outstanding item is returned by `touching` on a file its own body names.**
///
/// The property, said the way the promotion rule needs it: a promoter who changes a file is
/// shown every outstanding item that rests on that file - **whatever its status**, which is
/// where the tool was narrow, and **whatever the file is called**, which is where this test was.
#[test]
fn every_outstanding_item_is_found_by_a_file_its_body_names() {
    let all = outbox::read(&root());

    // **Both populations are asserted before either answer is trusted.** A repository with no
    // `built` item would pass this test by having nothing to miss, in the same words as one
    // where the predicate works.
    let outstanding: Vec<&outbox::Item> = all
        .items
        .iter()
        .filter(|item| item.is_outstanding())
        .collect();
    let built = outstanding
        .iter()
        .filter(|item| item.status.contains("built"))
        .count();
    let open = outstanding
        .iter()
        .filter(|item| item.status == "open")
        .count();
    assert!(
        built > 0,
        "no `built` item in any outbox, so this checks nothing about capabilities"
    );
    assert!(
        open > 0,
        "no `open` item either, over {} items",
        all.items.len()
    );

    let mut pairs = 0;
    let mut from_built = 0;
    for item in &outstanding {
        for path in files_named(&item.body) {
            let (found, looked) = spec::touching(&all.items, &path);
            assert!(
                looked >= built + open,
                "looked at {looked} where {built} built and {open} open items exist"
            );
            let ids: Vec<&str> = found.iter().map(|found| found.id.as_str()).collect();
            assert!(
                ids.contains(&item.id.as_str()),
                "{} names {path} in its body and touching({path}) returned {ids:?}",
                item.id
            );
            pairs += 1;
            if item.status.contains("built") {
                from_built += 1;
            }
        }
    }

    // **The counts are what tell a rule that holds from a rule with nothing to hold over**,
    // and the second one is the case the tool was actually broken on.
    assert!(
        pairs > 0,
        "no outstanding item names a specification file, so nothing above was checked"
    );
    assert!(
        from_built > 0,
        "{pairs} pairs checked and none from a `built` capability, which is the status the tool \
         could not see - so this would pass with the bug back"
    );
}
