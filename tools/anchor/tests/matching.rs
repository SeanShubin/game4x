//! `C-55`: the two failures this carries, each shown happening.
//!
//! **These are not hypotheticals.** Every case below is a shape `CLAUDE.md` records losing an
//! edit to, and three of them happened again while this session was building other things.

use anchor::{How, Problem, find, replace};

/// An anchor written on one line finds text the file wrapped across two.
///
/// **This is the whole of the first rule.** A match string drafted while writing the edit and
/// a file that has been through the padder or `cargo fmt` are the same words and different
/// bytes, and `str::replace` with no match is a no-op rather than an error.
#[test]
fn wrapping_is_not_a_difference() {
    let file = "before\nthe quick brown\nfox jumps over\nafter\n";
    let anchor = "the quick brown fox jumps over";
    let (from, to) = find(file, anchor, None, How::exact()).expect("the words are there, wrapped");
    assert_eq!(&file[from..to], "the quick brown\nfox jumps over");

    let after = replace(file, anchor, "one line now", None, How::exact()).unwrap();
    assert_eq!(after, "before\none line now\nafter\n");
}

/// Indentation is whitespace, so an anchor need not carry it.
#[test]
fn indentation_is_not_a_difference() {
    let file = "fn main() {\n    let x = 1;\n    let y = 2;\n}\n";
    let (from, to) = find(file, "let x = 1; let y = 2;", None, How::exact()).unwrap();
    assert_eq!(&file[from..to], "let x = 1;\n    let y = 2;");
}

/// An anchor that is not there is an error, and never a quiet no-op.
///
/// **The failure being carried is the edit that did nothing.** Returning the text unchanged
/// would be the exact behaviour `CLAUDE.md` records losing thirteen table rows to.
#[test]
fn an_anchor_that_is_not_there_is_refused() {
    let file = "the quick brown fox\n";
    assert_eq!(
        find(file, "a slow green fox", None, How::exact()),
        Err(Problem::NotFound(How::exact()))
    );
    assert_eq!(
        replace(file, "a slow green fox", "x", None, How::exact()),
        Err(Problem::NotFound(How::exact()))
    );
}

/// An anchor that matches twice is refused rather than taking the first.
///
/// **Taking the first is how the wrong line is edited**, and it looks like success. `CLAUDE.md`
/// records a range deleted between two markers destroying two proposals that were between them.
#[test]
fn an_anchor_that_matches_twice_is_refused() {
    let file = "let x = 1;\nlet y = 2;\nlet x = 1;\n";
    assert_eq!(
        find(file, "let x = 1;", None, How::exact()),
        Err(Problem::Ambiguous(2, How::exact()))
    );
    assert_eq!(
        replace(file, "let x = 1;", "let x = 9;", None, How::exact()),
        Err(Problem::Ambiguous(2, How::exact()))
    );
}

/// An anchor of no words matches everywhere and is refused.
#[test]
fn an_empty_anchor_is_refused() {
    assert_eq!(
        find("anything", "   \n  ", None, How::exact()),
        Err(Problem::EmptyAnchor)
    );
}

/// The replacement goes in exactly as written, including its whitespace.
///
/// **Normalizing is for finding and never for writing.** A tool that rewrote the file in its
/// normalized form would reflow everything it touched, which is a much worse silent edit than
/// the one it set out to prevent.
#[test]
fn the_replacement_is_verbatim() {
    let file = "a\nthe quick brown\nfox\nz\n";
    let after = replace(
        file,
        "the quick brown fox",
        "one\n  two\n    three",
        None,
        How::exact(),
    )
    .unwrap();
    assert_eq!(after, "a\none\n  two\n    three\nz\n");
}

/// Every byte outside the match is left alone.
///
/// **Checked over the whole file rather than at the seam**, because an off-by-one in the
/// offset mapping would show up as one character eaten somewhere else.
#[test]
fn nothing_outside_the_match_moves() {
    let file = "\u{e9}\u{e9}\u{e9} start\nthe quick\nbrown fox\nend \u{e9}\u{e9}\u{e9}\n";
    let after = replace(file, "the quick brown fox", "X", None, How::exact()).unwrap();
    assert_eq!(
        after,
        "\u{e9}\u{e9}\u{e9} start\nX\nend \u{e9}\u{e9}\u{e9}\n"
    );
    assert!(
        after.starts_with("\u{e9}\u{e9}\u{e9} start\n")
            && after.ends_with("end \u{e9}\u{e9}\u{e9}\n"),
        "multi-byte characters outside the match were disturbed: {after:?}"
    );
}

/// A marker the file carries and the anchor does not is not a difference.
///
/// **This is the case that made the strip worth having.** Offered text in a proposal is always
/// quoted, so the file carries `> ` on every line and the words being matched do not - and a
/// whitespace-only normalize sees a different string on every line.
#[test]
fn a_line_marker_is_not_a_difference() {
    let file = "before\n> the quick brown\n> fox jumps over\nafter\n";
    let (from, to) = find(
        file,
        "the quick brown fox jumps over",
        Some(">"),
        How::exact(),
    )
    .unwrap();
    assert_eq!(&file[from..to], "the quick brown\n> fox jumps over");

    // Without the strip the same anchor is simply not there, which is the failure it carries.
    assert_eq!(
        find(file, "the quick brown fox jumps over", None, How::exact()),
        Err(Problem::NotFound(How::exact()))
    );

    // A Rust doc comment is the same shape one directory over.
    let doc = "/// one two\n/// three four\n";
    assert!(find(doc, "one two three four", Some("///"), How::exact()).is_ok());
}

/// Every way this refuses is reachable from `find`, and adding a fifth breaks the build.
///
/// **`Q-72`: the first version of this asserted `refusals.len() == 3` over a three-element
/// array literal**, which is three by construction and tied to nothing. The quality lens
/// planted a fourth variant with its `Display` arm and all nine tests passed.
///
/// **The exhaustive `match` below is what ties it.** Rust cannot count an enum's variants
/// without a macro, but it can refuse to compile a `match` that does not cover them - so a
/// fifth kind of refusal stops this file building until somebody writes the case that
/// produces it. The assertions after it are that each variant is reachable from `find`,
/// which constructing one by hand would not show.
#[test]
fn every_way_this_refuses_is_reachable() {
    let refusals = [
        find("abc", "xyz", None, How::exact()).unwrap_err(),
        find("a a", "a", None, How::exact()).unwrap_err(),
        find("abc", " ", None, How::exact()).unwrap_err(),
        // **A marker no line begins with**, which is a strip that stripped nothing.
        find("abc", "abc", Some(">"), How::exact()).unwrap_err(),
    ];
    for why in &refusals {
        // No wildcard, deliberately: this arm is the tie to the variant count.
        match why {
            Problem::NotFound(_)
            | Problem::Ambiguous(_, _)
            | Problem::EmptyAnchor
            | Problem::MarkerNotFound(_) => {}
        }
    }
    assert!(
        refusals
            .iter()
            .any(|why| matches!(why, Problem::NotFound(_))),
        "an anchor that is not there must be reachable"
    );
    assert!(
        refusals
            .iter()
            .any(|why| matches!(why, Problem::Ambiguous(_, _))),
        "an anchor that matches twice must be reachable"
    );
    assert!(
        refusals
            .iter()
            .any(|why| matches!(why, Problem::EmptyAnchor)),
        "an anchor of no words must be reachable"
    );
    assert!(
        refusals
            .iter()
            .any(|why| matches!(why, Problem::MarkerNotFound(_))),
        "a strip that strips nothing must be reachable"
    );

    for why in &refusals {
        assert!(
            !why.to_string().is_empty(),
            "a refusal that says nothing is a silent failure wearing a type"
        );
    }
}
