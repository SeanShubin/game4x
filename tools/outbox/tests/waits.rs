//! A hold whose reason is over says so, and one whose reason never existed says something else.
//!
//! **`S-61`, checking `P-325`.** `docs/process.md`: *An item that cannot be acted on yet says
//! what it waits on in a field, never in prose - `**waits on** P-n`. When that id is no longer
//! open the wait is over, and whatever lists the outboxes says so.*
//!
//! **The rule exists because four holds went stale in one night** and the last of them stalled
//! the file Sean was waiting for. A hold written into a paragraph is invisible to every tool
//! and to every reader who does not re-read the whole item, so an item goes on telling a lane
//! not to start work that is no longer blocked.
//!
//! # What these tests are for
//!
//! `S-61` names three ways this could be decoration, and each is a test here rather than a
//! sentence in the item:
//!
//! - **The population, not only the offences.** *No stale waits* means nothing against a
//!   population that is also zero.
//! - **A wait on an id that does not exist is a defect, not a satisfied wait.** `S-51`'s
//!   shape: a predicate that cannot tell *resolved* from *never was* reports both as fine.
//! - **A wait that is genuinely holding is left alone**, or the check reports every hold and
//!   is noise.

use outbox::{Wait, parse, waiting};

/// A queue with an Accepted table and a Withdrawn table, as the real one has.
const QUEUE: &str = "\
# Spec Proposals

## Accepted

| Proposal | Into                   | When       |
| -------- | ---------------------- | ---------- |
| P-325    | `docs/process.md`      | 2026-09-06 |

## Withdrawn

| Proposal | Why                         | When |
| -------- | --------------------------- | ---- |
| P-311    | it contradicted the spec    |      |
";

fn items(text: &str) -> Vec<outbox::Item> {
    parse(text, "an-outbox.md")
}

/// Each of the four things a `waits on` can name, told apart.
///
/// **One test over all four rather than four**, because what matters is that they are
/// *distinguished*: a check reporting `Unknown` as `Withdrawn` would still find every stale
/// hold and would tell a reader the opposite of the truth about why.
#[test]
fn a_wait_ends_three_ways_and_a_fourth_never_began() {
    let text = "\
### C-1 - waiting on a proposal that landed

**to** spec · **status** open · **waits on** `P-325`

### C-2 - waiting on a proposal that was withdrawn

**to** spec · **status** open · **waits on** `P-311`

### C-3 - waiting on an item that has closed

**to** spec · **status** open · **waits on** `C-9`

### C-4 - waiting on nothing at all

**to** spec · **status** open · **waits on** `P-999`

### C-5 - waiting on an item that is still open

**to** spec · **status** open · **waits on** `C-6`

### C-6 - the thing C-5 is waiting on

**to** code · **status** open

### C-9 - the thing C-3 was waiting on

**to** code · **status** **acted** 2026-09-01
";
    let (population, over) = waiting(&items(text), QUEUE);

    // **Five items carry a wait**, and the sixth and seventh carry none. Without this the
    // assertions below could all hold over an empty walk.
    assert_eq!(population, 5, "five open items say what they wait on");

    let became: Vec<(&str, &Wait)> = over
        .iter()
        .map(|held| (held.item.as_str(), &held.became))
        .collect();
    assert_eq!(
        became.len(),
        4,
        "four of the five waits are over: {became:?}"
    );

    assert_eq!(
        became[0].0, "C-1",
        "the items come back in the order they were read"
    );
    assert!(
        matches!(became[0].1, Wait::Promoted { destination, date }
            if destination == "`docs/process.md`" && date == "2026-09-06"),
        "a promoted proposal says where it went and when: {:?}",
        became[0].1
    );
    assert_eq!(became[1], ("C-2", &Wait::Withdrawn));
    assert!(
        matches!(became[2].1, Wait::Closed { status } if status.contains("acted")),
        "a closed item says what it closed as: {:?}",
        became[2].1
    );
    // **The arm `S-51` says a naive predicate would get wrong.** `P-999` is in no table and
    // no outbox, and *resolved* and *never was* must not read the same.
    assert_eq!(became[3], ("C-4", &Wait::Unknown));

    // And the one that is genuinely holding is not reported at all.
    assert!(
        !over.iter().any(|held| held.item == "C-5"),
        "a wait on something still open is a wait, not a finding"
    );
}

/// A closed item's wait is not reported, however stale it is.
///
/// **The check is about holds, and a closed item is not held.** Without this the tool would
/// report every wait ever written the moment its subject moved, including on items nobody is
/// waiting for any more - which is how a signal stops being read.
#[test]
fn a_wait_on_an_item_that_has_itself_closed_is_not_reported() {
    let text = "\
### C-1 - closed, and it once waited on something

**to** spec · **status** **acted** 2026-09-01 · **waits on** `P-325`
";
    let (population, over) = waiting(&items(text), QUEUE);
    assert_eq!(population, 0, "a closed item carries no live hold");
    assert!(over.is_empty());
}

/// The field is read as one id, whatever punctuates the line around it.
///
/// **This is the bug the check found in itself on its first run.** The value was read with
/// `whole_field`, which runs to the next `**` - and the proposal queue punctuates fields with
/// ` - `, so `` **waits on** `P-325` - **source** ... `` came back as `` `P-325` - `` and
/// matched nothing. **The check reported its own first case as a wait on an id that does not
/// exist**, which is exactly the arm it was built to keep separate.
#[test]
fn the_id_is_read_the_same_however_the_line_is_punctuated() {
    let separators = [
        "**to** spec - **status** open - **waits on** `P-325` - **source** a long tail",
        "**to** spec · **status** open · **waits on** `P-325` · **source** a long tail",
        "**to** spec · **status** open · **waits on** P-325",
    ];
    assert_eq!(separators.len(), 3, "three ways the outboxes punctuate");
    for fields in separators {
        let text = format!("### C-1 - held\n\n{fields}\n");
        let (population, over) = waiting(&items(&text), QUEUE);
        assert_eq!(population, 1, "`{fields}`");
        assert_eq!(over.len(), 1, "`{fields}`");
        assert_eq!(over[0].on, "P-325", "`{fields}`");
        assert!(
            !matches!(over[0].became, Wait::Unknown),
            "`{fields}` read the id as something no table has"
        );
    }
}

/// Nothing is reported when nothing waits, and the population says which kind of nothing.
///
/// **The two zeroes are different and only one of them is good news.** No items carrying a
/// wait, and items carrying waits that are all holding, both report no offences - so the
/// population is what tells a reader which they are looking at.
#[test]
fn no_waits_and_no_stale_waits_are_told_apart_by_the_population() {
    let none = "### C-1 - nothing is held here\n\n**to** spec · **status** open\n";
    let (population, over) = waiting(&items(none), QUEUE);
    assert_eq!((population, over.len()), (0, 0), "nobody is waiting");

    let holding = "\
### C-1 - held on something open

**to** spec · **status** open · **waits on** `C-2`

### C-2 - the thing it waits on

**to** code · **status** open
";
    let (population, over) = waiting(&items(holding), QUEUE);
    assert_eq!(
        (population, over.len()),
        (1, 0),
        "somebody is waiting, correctly"
    );
}

/// The reader agrees with a plain scan of the real tree about which items carry the field.
///
/// **This replaces a test that asserted the population was not zero, and that test was wrong
/// in a way worth recording.** `S-61` carried the `waits on` field and said it was its own
/// first case; the moment the specification lane closed it - correctly, and partly because
/// this check reported it - the field went and the population became zero. **A test asserting
/// a fact that a correct action falsifies is a test that punishes the action.**
///
/// **So this compares two derivations rather than a number.** The tool reads the field with
/// `field`, which was where the bug was on the first run; this counts open items whose field
/// line merely *contains* the words. The two share the item parse and nothing else, so a
/// reader that stopped finding the field fails here whatever today's population happens to be.
///
/// **Zero on both sides is a real answer**, and it says the repository is holding nothing -
/// which is what it should say when nobody is waiting.
#[test]
fn the_field_reader_and_a_plain_scan_agree_about_the_real_tree() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let all = outbox::read(&root);
    let queue = std::fs::read_to_string(root.join("docs/notes/proposals.md")).unwrap_or_default();
    let (population, over) = waiting(&all.items, &queue);

    let by_scan: Vec<&str> = all
        .items
        .iter()
        .filter(|item| item.is_outstanding())
        .filter(|item| item.fields.contains("**waits on**"))
        .map(|item| item.id.as_str())
        .collect();
    assert_eq!(
        population,
        by_scan.len(),
        "the field reader found {population} holds and a plain scan found {} ({by_scan:?})",
        by_scan.len()
    );

    // And the items it read the tree from are really there, so the agreement above is not
    // two empty walks agreeing.
    assert!(
        all.items.len() > 20,
        "only {} items read from the tree",
        all.items.len()
    );
    // Every reported hold is one the scan also saw, which is the direction a wrong field
    // reader would break: reading a wait out of an item that has none.
    for held in &over {
        assert!(
            by_scan.contains(&held.item.as_str()),
            "{} was reported as holding and its field line does not say so",
            held.item
        );
    }
}
