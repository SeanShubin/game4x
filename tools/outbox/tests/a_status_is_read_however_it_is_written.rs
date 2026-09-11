//! A status is read whether it is bolded or not, and *absent* is not a status.
//!
//! **Written because a lane asked this question with a regex and got a wrong answer.** On
//! 2026-09-11 the code lane checked eight items' statuses by grepping the outboxes with
//! `**status** \([a-z]*\)`. Every item in the proposal queue writes its status **bolded** -
//! `**status** **acted**` - so the pattern matched nothing and reported all eight as absent
//! from the queue.
//!
//! **That was a wrong answer to the right question which happened to be loud.** Eight of eight
//! missing is obviously an instrument failure and it was caught in one look. The same pattern
//! returning *seven* of eight - had one item been written unbolded - would have read as a real
//! finding about the eighth, and nobody would have looked at the instrument. `CLAUDE.md` names
//! that shape: *a wrong number invites a question; a right number about the wrong thing invites
//! none.*
//!
//! **The parser here was already right and was not being called.** `field` trims the asterisks
//! and has done since long before that grep; what was missing was a way for a lane to *ask*,
//! so one wrote its own reader instead. `P-327`: a rule that fires at a moment of confidence
//! needs a carrier rather than a better sentence. `outbox --item ID` is the carrier and these
//! are the cases it exists to get right.

use outbox::parse;

/// Both spellings, in one file, so neither can be read by a rule that only knows the other.
const BOTH_WAYS: &str = "\
### S-47 - bolded, as every proposal writes it

**to** code - **status** **acted** 2026-09-06 - **cited** `f2040fa`

Prose under it.

### C-50 - plain, as this lane's own outbox writes it

**to** spec · **status** answered · **raised** 2026-09-06

Prose under it.

### C-87 - still open

**to** spec · **status** open · **raised** 2026-09-11

Prose under it.
";

/// A bolded status and a plain one read the same, and both read as themselves.
///
/// **Over every spelling in use, with the count asserted.** A reader that handled only one
/// would pass a test written against only that one, which is how the grep passed review in
/// somebody's head before it was run.
#[test]
fn a_status_is_read_whether_or_not_it_is_bolded() {
    let items = parse(BOTH_WAYS, "test.md");
    assert_eq!(items.len(), 3, "three items, and these parsed: {items:?}");

    let status = |id: &str| {
        items
            .iter()
            .find(|item| item.id == id)
            .unwrap_or_else(|| panic!("{id} did not parse at all"))
            .status
            .clone()
    };
    assert_eq!(status("S-47"), "acted", "the bolded spelling");
    assert_eq!(status("C-50"), "answered", "the plain spelling");
    assert_eq!(status("C-87"), "open", "and one that is genuinely open");

    // **No asterisk survives into a value.** The failure was a status read as `**acted**`
    // or as nothing; either would compare unequal to every name a caller knows.
    for item in &items {
        assert!(
            !item.status.contains('*'),
            "`{}` kept its markup: {:?}",
            item.id,
            item.status
        );
    }
}

/// Outstanding is decided by the value, not by how it was written.
///
/// **This is what the grep was actually for**, and the reason a spelling difference mattered
/// at all: the question was *is this settled*, and an unparsed status answers it wrongly in
/// whichever direction the caller's default happens to point.
#[test]
fn how_a_status_is_written_does_not_change_whether_it_is_outstanding() {
    let items = parse(BOTH_WAYS, "test.md");
    let outstanding: Vec<&str> = items
        .iter()
        .filter(|item| item.is_outstanding())
        .map(|item| item.id.as_str())
        .collect();
    assert_eq!(
        outstanding,
        ["C-87"],
        "only the open one is outstanding; the bolded `acted` must not count as unparsed"
    );
}

/// An id that is nowhere is not an item with a status.
///
/// **The distinction the grep could not make.** It reported `ABSENT FROM THE QUEUE` for items
/// that were present and settled, and would have said the same for `S-34`, which really was
/// removed. Two different facts arriving as one string is what made the output unreadable.
#[test]
fn an_id_that_is_nowhere_is_not_found_rather_than_being_given_a_status() {
    let items = parse(BOTH_WAYS, "test.md");
    assert!(
        items.iter().all(|item| item.id != "S-34"),
        "the fixture has no S-34, so a lookup for it must find nothing"
    );
    // **Both halves, because *finds nothing* is only meaningful against a file that does
    // find things.** A parser that had stopped working would satisfy the assertion above.
    assert!(
        items.iter().any(|item| item.id == "S-47"),
        "and the same fixture must still find one that is there"
    );
}
