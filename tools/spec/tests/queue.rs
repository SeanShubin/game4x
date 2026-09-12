//! The queue operations, run over the real `docs/notes/proposals.md`.
//!
//! **The population is the file rather than a fixture**, which is the rule this tool was
//! built to keep: `docs/process.md`, *a check that reads a copy of the population is checking
//! the copy.* A fixture written by the same hand as the parser agrees with the parser.
//!
//! Every count below is asserted rather than printed, and each says what it counted over -
//! `CLAUDE.md`: *a count over nothing is the same failure with the sign flipped.*

use std::path::{Path, PathBuf};

use spec::queue::{FILE_SECTIONS, NOTHING_OPEN, block_of, remove_block};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("tools/spec sits two below the root")
        .to_path_buf()
}

fn queue() -> String {
    std::fs::read_to_string(root().join("docs/notes/proposals.md")).expect("the queue is readable")
}

fn ids(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("### "))
        .filter_map(|rest| rest.split_once(" - "))
        .map(|(id, _)| id.trim().to_string())
        .collect()
}

/// Every item in the queue has a block, and no block swallows the next item.
///
/// **This is the shape the hand-written removal got wrong twice on 2026-09-11**, in opposite
/// directions on the same day: once it stopped short at a `##` sub-heading and left sixty-eight
/// lines behind, and the guard against the other direction is what `CLAUDE.md` records two
/// proposals being destroyed by.
#[test]
fn every_item_has_a_block_that_holds_itself_and_nothing_else() {
    let text = queue();
    let all = ids(&text);
    assert!(
        all.len() >= 10,
        "the queue holds {} items, which is too few for this to be measuring anything",
        all.len()
    );
    let lines: Vec<&str> = text.lines().collect();
    for id in &all {
        let (start, end) = block_of(&text, id).unwrap_or_else(|why| panic!("{id}: {why}"));
        assert!(
            lines[start].starts_with(&format!("### {id} - ")),
            "{id} starts at its heading"
        );
        let inside = &lines[start + 1..end];
        let others: Vec<&&str> = inside.iter().filter(|l| l.starts_with("### ")).collect();
        assert!(others.is_empty(), "{id}'s block swallowed {others:?}");
    }
}

/// A block that carries `##` sub-headings keeps them, and removing it takes them with it.
///
/// **Measured rather than assumed to exist**: if no item in the queue has a sub-heading, this
/// test proves nothing, so it says how many did.
///
/// **Counted rather than searched for, and the first version was searched for.** It asked
/// whether the heading's text appears anywhere in what is left, which is a wider question than
/// *did this block's lines go* - so `P-458`'s `## What lands with it, if it lands` failed it on
/// behalf of `P-455`'s `## What lands with it`, one being a prefix of the other. The file was
/// right and the instrument was wrong, which is `CLAUDE.md` -> *a right number about the wrong
/// thing invites none*. Counting whole lines asks the question that was meant: as many
/// occurrences leave as the block held, and any other occurrence is somebody else's.
#[test]
fn sub_headings_belong_to_their_item_and_leave_with_it() {
    let text = queue();
    let lines: Vec<&str> = text.lines().collect();
    let occurrences = |body: &str, heading: &str| body.lines().filter(|l| *l == heading).count();
    let mut with_sub_headings = 0;
    for id in ids(&text) {
        let (start, end) = block_of(&text, &id).expect("a block");
        let subs: Vec<&&str> = lines[start + 1..end]
            .iter()
            .filter(|l| l.starts_with("## ") && !FILE_SECTIONS.contains(&l.trim()))
            .collect();
        if subs.is_empty() {
            continue;
        }
        with_sub_headings += 1;
        let left = remove_block(&text, &id).expect("removable");
        for sub in subs {
            let inside = lines[start + 1..end].iter().filter(|l| l == &sub).count();
            let before = occurrences(&text, sub);
            let after = occurrences(&left, sub);
            assert_eq!(
                before - inside,
                after,
                "{id} left {} of its {inside} {sub:?} behind, which is the 2026-09-11 defect",
                after - (before - inside)
            );
        }
    }
    assert!(
        with_sub_headings > 0,
        "no item in the queue carries a sub-heading, so this test is passing over nothing"
    );
}

/// The queue's own sections are the ones the tool knows about.
///
/// **The boundary is a list, so the list going stale is the way this tool fails.** A section
/// added to the file and not to `FILE_SECTIONS` would read as an item's sub-heading, and the
/// item before it would swallow the rest of the file. This is what notices.
#[test]
fn the_files_sections_are_the_ones_the_tool_names() {
    let text = queue();
    let present: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("## "))
        .collect();
    let unknown: Vec<&&str> = present
        .iter()
        .filter(|line| !FILE_SECTIONS.contains(&line.trim()))
        .collect();
    let known: Vec<&&str> = present
        .iter()
        .filter(|line| FILE_SECTIONS.contains(&line.trim()))
        .collect();
    assert_eq!(
        known.len(),
        FILE_SECTIONS.len(),
        "every named section is in the file exactly once: found {known:?}"
    );
    // The rest are items' own sub-headings, and each must sit inside some item's block.
    let lines: Vec<&str> = text.lines().collect();
    let blocks: Vec<(usize, usize)> = ids(&text)
        .iter()
        .map(|id| block_of(&text, id).expect("a block"))
        .collect();
    for sub in &unknown {
        let at = lines
            .iter()
            .position(|line| *line == **sub)
            .expect("the line is in the file");
        assert!(
            blocks.iter().any(|(start, end)| at > *start && at < *end),
            "{sub:?} is at line {} and belongs to no item, so it is a section the tool does not know",
            at + 1
        );
    }
}

/// Nothing is left in `Open` when it says it is empty.
///
/// **This is the one that would have failed on 2026-09-11**, twice: the `Open` section said
/// *Nothing is open* while sixty-eight lines of two removed proposals sat under it.
#[test]
fn the_open_section_holds_only_items_or_says_it_is_empty() {
    let text = queue();
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.trim() == "## Open")
        .expect("the queue has an Open section");
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, line)| FILE_SECTIONS.contains(&line.trim()))
        .map(|(at, _)| at)
        .expect("a section follows Open");
    let inside: Vec<&&str> = lines[start + 1..end]
        .iter()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let items = inside.iter().filter(|l| l.starts_with("### ")).count();
    if items == 0 {
        assert_eq!(
            inside.len(),
            1,
            "an empty Open section holds one sentence and nothing else, and holds {inside:?}"
        );
        assert!(
            inside[0].starts_with('*'),
            "and that sentence says it is empty: {:?}",
            inside[0]
        );
    } else {
        let orphans: Vec<&&&str> = inside
            .iter()
            .filter(|l| l.starts_with("## ") && !FILE_SECTIONS.contains(&l.trim()))
            .collect();
        let blocks: Vec<(usize, usize)> = ids(&text)
            .iter()
            .map(|id| block_of(&text, id).expect("a block"))
            .collect();
        for orphan in orphans {
            let at = lines
                .iter()
                .position(|l| *l == **orphan)
                .expect("in the file");
            assert!(
                blocks.iter().any(|(s, e)| at > *s && at < *e),
                "{orphan:?} sits in Open under no item"
            );
        }
    }
}

/// No specification file this lane writes carries a carriage return.
///
/// **The defect the whitespace check could not see.** A comparison that collapses whitespace
/// treats `\r` as whitespace, so six of them sat in `spec/invariants.md` while the promotion's
/// own assertion passed.
#[test]
fn the_files_this_lane_writes_hold_no_carriage_return() {
    let root = root();
    let mut checked = 0;
    let mut dirty: Vec<String> = Vec::new();
    for directory in ["spec", "releases", "docs"] {
        let mut stack = vec![root.join(directory)];
        while let Some(at) = stack.pop() {
            for entry in std::fs::read_dir(&at).expect("readable") {
                let path = entry.expect("an entry").path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|it| it == "md") {
                    let text = std::fs::read_to_string(&path).expect("readable");
                    checked += 1;
                    if text.contains('\r') {
                        dirty.push(path.display().to_string());
                    }
                }
            }
        }
    }
    assert!(
        checked > 20,
        "only {checked} files checked, which is too few to mean anything"
    );
    assert!(
        dirty.is_empty(),
        "{} of {checked} files hold a carriage return: {dirty:?}",
        dirty.len()
    );
}

/// The sentinel is in the Open section exactly when the Open section is empty.
///
/// **The sentence is the promise, so a stale one is the promise broken.** `CLAUDE.md`: *if
/// nothing in any outbox is `open` and addressed, nothing known is outstanding* - and
/// `NOTHING_OPEN` is where the queue says that about itself.
///
/// **`say_if_empty` only ever writes it**, which is half of a rule. `P-455` was filed into an
/// Open section that already carried the sentinel, and for a day the file said nothing was
/// open with a proposal standing three lines above the sentence. Nothing removes it, because
/// filing is done by hand and no verb runs then - so the check runs at the gate instead, where
/// there is no moment to remember.
#[test]
fn the_sentinel_says_what_is_true_of_the_open_section() {
    let text = queue();
    let lines: Vec<&str> = text.lines().collect();
    let open = lines
        .iter()
        .position(|line| line.trim() == "## Open")
        .expect("the queue has an Open section");
    let next = lines
        .iter()
        .enumerate()
        .skip(open + 1)
        .find(|(_, line)| FILE_SECTIONS.contains(&line.trim()) && line.trim() != "## Open")
        .map(|(at, _)| at)
        .expect("a section follows Open");
    let inside = &lines[open + 1..next];
    let items: Vec<&&str> = inside
        .iter()
        .filter(|line| line.starts_with("### "))
        .collect();
    let says_empty = inside.iter().any(|line| line.trim() == NOTHING_OPEN);
    assert_eq!(
        says_empty,
        items.is_empty(),
        "the Open section holds {} item(s) and {} the sentinel - {:?}",
        items.len(),
        if says_empty {
            "carries"
        } else {
            "does not carry"
        },
        items
    );
    assert!(
        !inside.is_empty(),
        "the Open section is empty of everything, sentinel included, so this agreed for the \
         wrong reason"
    );
}

// **`every_cited_hash_is_a_commit` was here and is deleted rather than repaired.** `Q-86`:
// it asked `git cat-file -t`, which says whether an object is in the **local** database - and
// an amended commit still is, surviving in the reflog reachable from nothing while a clone
// never receives it. **So the gate built after the amending incident would not have caught the
// amending incident.**
//
// **And its poison could not have shown that.** `0000000` exists nowhere, so it lands where
// both predicates agree; the region where they differ - exists locally, reachable from
// nothing - is the only one that mattered and the one the poison never entered.
//
// **`tools/outbox/tests/citations.rs` already checks both**, over `docs/notes/proposals.md`,
// `docs/notes/decisions.md` and every release - the same three files, with `is_reachable`
// driven against an orphan commit and a separate message for each half. So this added no file
// and subtracted a predicate, which is *a fact is stated once* applied to a check: two
// instruments over one population, and the weaker one goes.
