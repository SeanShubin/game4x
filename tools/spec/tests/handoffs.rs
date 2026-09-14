//! What a handoff is, and what only looks like one.
//!
//! **The failure is in `docs/postmortems/tracked-and-still-lost.md`.** An invariant was carried
//! by `C-16`, then by `S-30`, then by `C-102`, over ten days with no gap in the chain - and
//! each hop asked a narrower question than the one before, so what the first one carried was
//! gone by the third and nothing said so.
//!
//! **`spec chains` is a detector and not a check.** It cannot compare the scope of two English
//! sentences, which is `P-245`'s wall. What it can do is say a chain exists and where to look,
//! and the whole of its value is that it says so about few enough chains to read.
//!
//! **So these tests drive it both ways**, which is the lesson the postmortem itself records:
//! three instruments written that day each returned a plausible wrong answer, and every one was
//! caught by deriving the answer a second way. A test that only shows a handoff being found
//! would have passed for all three of them.

use spec::handed_to;

/// The cue and the successor in one sentence - `C-16`'s own shape.
#[test]
fn a_successor_named_beside_a_cue_is_a_handoff() {
    let body = "**Closed, and the gap is unchanged.** `S-30` is the item that carries it.";
    assert_eq!(handed_to("C-16", body), vec!["S-30".to_string()]);
}

/// **The cue in a heading and the successor in the paragraph under it - `S-30`'s own shape.**
///
/// A paragraph-scoped version of this returned a plausible zero and missed the one chain that
/// mattered, because `## Where the live half is` carries the cue and names nothing.
#[test]
fn a_cue_in_a_heading_reaches_the_paragraph_under_it() {
    let body = "## Where the live half is\n\nWhich of the four tables is data is `C-102`, open.";
    assert_eq!(handed_to("S-30", body), vec!["C-102".to_string()]);
}

/// **`C-30`'s shape, and it is not a handoff.** It names `S-30` inside a table of counts.
///
/// A first version counted any id mentioned anywhere in a closed body and produced this edge.
#[test]
fn an_id_in_a_table_row_is_a_count_and_not_a_handoff() {
    let body = "## What is tracked by whom\n\n| `proposals.md` | **7** - `S-44`, `S-30`, `S-29` |";
    assert!(
        handed_to("C-30", body).is_empty(),
        "a table row stated a count and was read as a handoff"
    );
}

/// A section with no cue hands nothing off, however many ids it names.
#[test]
fn a_mention_without_a_cue_is_not_a_handoff() {
    let body = "## Three things\n\n`S-30`'s data file waits on `C-49`, already filed.";
    assert!(
        handed_to("C-87", body).is_empty(),
        "an id mentioned in passing was read as a handoff"
    );
}

/// An item never hands off to itself, and a promotion is a landing rather than a successor.
#[test]
fn neither_itself_nor_a_proposal_is_a_successor() {
    let body = "**Closed, tracked by `P-305`.** `C-16` is the item that carries it.";
    assert!(
        handed_to("C-16", body).is_empty(),
        "read its own id or a proposal as a successor"
    );
}

/// **The detector must never count over nothing**, which is the other half of the same failure:
/// *zero occurrences proves something only against a population that is not also zero.*
#[test]
fn the_cues_are_exercised_by_the_real_outboxes() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repository root");
    let all = outbox::read(&root);
    assert!(
        all.items.len() > 100,
        "only {} items read - the outboxes moved and this counts over nothing",
        all.items.len()
    );
    let closed: Vec<&outbox::Item> = all.items.iter().filter(|i| i.status != "open").collect();
    let handoffs: usize = closed.iter().map(|i| handed_to(&i.id, &i.body).len()).sum();
    assert!(
        handoffs > 0,
        "{} closed items and no handoff found at all - the cues have stopped matching",
        closed.len()
    );
}

/// **A cue in one section must not reach an id in another, and nothing tested this.**
///
/// Found by poisoning: removing the section split and running these tests left all of them
/// green, because every negative case above has no cue anywhere in it. So the split was
/// unverified and the suite said otherwise - which is `docs/postmortems/` all over again, a
/// check answering a narrower question than the one asked.
#[test]
fn a_cue_does_not_reach_across_a_heading() {
    let body = "## What this closed on\n\n\
                The gap is now tracked by `C-49`.\n\n\
                ## Three other things, none of them handed anywhere\n\n\
                `S-30`'s data file is already filed, and `X-12` is open to the research lens.";
    assert_eq!(
        handed_to("C-87", body),
        vec!["C-49".to_string()],
        "a cue in one section reached ids in another"
    );
}

/// **An item that describes a handoff must not be read as making one.**
///
/// Found in the gate within a minute of the detector landing there: closing `S-129`, whose body
/// explains the chain the postmortem is about, made the detector report `S-129` as handing off
/// to `S-30`. **The item documenting the defect was reported as committing it** - `CLAUDE.md`'s
/// *quoting a thing and doing it are the same bytes*.
///
/// The carrier is `tools/outbox`'s and not a new one: a double-backticked span is shown rather
/// than said, and is dropped before the rest is read.
#[test]
fn an_id_being_shown_is_not_a_successor() {
    let body = "## What it catches\n\n\
                `` `C-16` `` carried it, closed handing it to `` `S-30` ``, now tracked by nobody.";
    assert!(
        handed_to("S-129", body).is_empty(),
        "ids being displayed as an example were read as a handoff"
    );
}

/// And the carrier must not swallow an ordinary single-backticked successor beside it.
#[test]
fn a_shown_id_does_not_hide_a_real_one_on_the_same_line() {
    let body = "## Closing\n\n\
                Unlike `` `S-30` ``, this one is tracked by `C-49` and stays open.";
    assert_eq!(handed_to("C-87", body), vec!["C-49".to_string()]);
}
