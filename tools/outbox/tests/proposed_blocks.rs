//! A proposal's offered blocks, one per destination - `S-103`.
//!
//! **`CLAUDE.md` -> Promotion says the count is not fixed**: *a proposal that lands in more
//! than one file carries one quotation for each*, and within one file *two bullets of one
//! section is two quotations, and four tables across four sections is four*.
//!
//! **So a second blockquote is usually a second destination**, and `Item::proposed_text`
//! reporting `Several` for it was right about its own question and one block too narrow for
//! the queue. `proposed_blocks` answers the wider one; `proposed_text` still refuses, because
//! a caller that means one and finds several is still looking at something it cannot use.
//!
//! **Both read the same parser**, which is the point of it living here at all:
//! `docs/notes/tools-spec-design.md` called on 2026-09-02 that either `outbox` exposes this
//! or `tools/spec` grows a second parser and the two come to disagree about where a
//! proposal's body ends.

use outbox::{Item, NoText};

/// An item with this body, built the way the parser hands one over.
fn item(body: &str) -> Item {
    let text = format!(
        "### P-1 - a title

**to** sean · **status** open

{body}
"
    );
    let mut items = outbox::parse(&text, "docs/notes/proposals.md");
    assert_eq!(items.len(), 1, "one item in the fixture");
    items.remove(0)
}

/// One block reads the same through both, which is what stops them drifting apart.
#[test]
fn one_block_is_the_same_text_through_either_reader() {
    let one = item("Some prose.\n\n> The offered words.\n> On two lines.\n\nMore prose.");
    assert_eq!(
        one.proposed_blocks(),
        Ok(vec!["The offered words.\nOn two lines.".to_string()])
    );
    assert_eq!(
        one.proposed_text(),
        Ok("The offered words.\nOn two lines.".to_string()),
        "the two readers disagree about a single block, which is the drift this avoids"
    );
}

/// Several blocks are several destinations, in the order the proposal names them.
///
/// **The order is the contract**, because `CLAUDE.md` says the quotations come *in the order
/// the destinations are named* - so a caller pairs the nth block with the nth destination and
/// a reader who sorted them would land approved text in the wrong file.
#[test]
fn several_blocks_come_back_in_order_and_proposed_text_still_refuses() {
    let two =
        item("Into `spec/a.md`:\n\n> First block.\n\nAnd into `spec/b.md`:\n\n> Second block.\n");
    assert_eq!(
        two.proposed_blocks(),
        Ok(vec![
            "First block.".to_string(),
            "Second block.".to_string()
        ]),
        "two destinations, and the order is the pairing"
    );
    assert_eq!(
        two.proposed_text(),
        Err(NoText::Several(2)),
        "a caller that means one and finds two still has to be told"
    );

    // Four, because `CLAUDE.md` names four tables across four sections as a real case.
    let four = item("> One.\n\ntext\n\n> Two.\n\ntext\n\n> Three.\n\ntext\n\n> Four.\n");
    assert_eq!(four.proposed_blocks().map(|b| b.len()), Ok(4));
    assert_eq!(four.proposed_text(), Err(NoText::Several(4)));
}

/// No blockquote is no text, and both readers say so the same way.
///
/// **An instruction may carry no quotation at all** - `CLAUDE.md` - so this is an ordinary
/// answer rather than a malformed item, and the two readers must not differ about it.
#[test]
fn no_block_is_the_same_refusal_through_either_reader() {
    let none = item("This proposal describes a change rather than offering words.");
    assert_eq!(none.proposed_blocks(), Err(NoText::None));
    assert_eq!(none.proposed_text(), Err(NoText::None));
}

/// The markers are stripped and a blank quoted line does not end a block.
///
/// **`> ` and `>` both**, because a quoted empty line is written without the trailing space
/// and a parser that required one would split one block into two - which would read as two
/// destinations, which is the failure this whole function exists to get right.
#[test]
fn a_blank_quoted_line_keeps_one_block_together() {
    let one = item("> First paragraph.\n>\n> Second paragraph.\n");
    assert_eq!(
        one.proposed_blocks(),
        Ok(vec!["First paragraph.\n\nSecond paragraph.".to_string()]),
        "a quoted blank line is inside the block, not between two"
    );
    assert!(
        one.proposed_text().is_ok(),
        "so a two-paragraph offer is still one offer"
    );
}
