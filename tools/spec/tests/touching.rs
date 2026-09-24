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

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the repository root")
        .to_path_buf()
}

/// **A `built` capability is outstanding, and `touching` has to see it.**
///
/// `R-6` is the case that matters and the reason this test names it: it is `built`, addressed to
/// Sean, and its `In` line quotes `spec/control.md`'s win condition - **the sentence `P-520`
/// replaced.** A promoter who ran this tool then would not have been shown the one capability
/// whose account of itself the promotion broke.
#[test]
fn an_outstanding_capability_is_found_and_an_open_item_still_is() {
    let all = outbox::read(&root());

    // **Both populations are asserted before either answer is trusted.** A repository with no
    // `built` item would pass this test by having nothing to miss, in the same words as one
    // where the predicate works.
    let built = all
        .items
        .iter()
        .filter(|item| item.status.contains("built"))
        .count();
    let open = all
        .items
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

    let (found, looked) = spec::touching(&all.items, "spec/control.md");
    assert!(
        looked >= built + open,
        "looked at {looked} where {built} built and {open} open items exist"
    );

    let ids: Vec<&str> = found.iter().map(|item| item.id.as_str()).collect();
    assert!(
        ids.contains(&"R-6"),
        "R-6 is `built` and its In line quotes spec/control.md, and touching returned {ids:?}"
    );
}
