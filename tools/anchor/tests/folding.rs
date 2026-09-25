//! `X-40`: the half of the carrier that was missing, and the failure that names which half ran.
//!
//! **`C-113` failed twice in one search.** One sentence was missed because it was **wrapped**,
//! another because it was **capitalised**. `anchor` already carried the first - `collapse` joins
//! on whitespace - and had no case handling at all: the only occurrence of *case* in its source
//! was the English word, in a comment.
//!
//! **So the carrier covered one of the two halves, and the research lens's sentence is why that
//! matters more than it sounds**: *it was rational not to reach for it - half a carrier is not a
//! carrier for the half you need.*
//!
//! # Why case is not incidental here
//!
//! `spec/` capitalises kinds at the start of a sentence and in **every section heading** -
//! `## Pioneer`, `### Yard` - so the words most worth searching for are exactly the ones a
//! case-sensitive search misses. **Two lanes hit it in one week** and in both the answer came
//! back **zero** rather than an error, which defeats this tool's whole design principle: *doing
//! nothing is never a success here.*
//!
//! # The property being extended is the failure message, not only the comparison
//!
//! A refusal already says `wrapping ignored`, and that is the only reason one run was enough to
//! diagnose `C-113`. **A folded miss says `case folded, wrapping ignored`**, so a reader learns
//! which normalisations were in play **from the failure alone**, rather than from the flags they
//! remember passing.

use anchor::{How, Problem, find, replace};

/// Folding on, off, and what each finds.
fn folded() -> How {
    How { fold_case: true }
}

/// A heading found with folding and missed without it, which is `X-40`'s own case.
///
/// # The check that would have failed before this
///
/// **`anchor find` on `a put has no quantity` against `releases/first-release.md` returned
/// nothing**, while the file says `A put has no quantity` at line 183. The specification lane's
/// sweep of thirty-nine items and this lane's check of that sweep both reached that zero and
/// neither noticed, an hour apart.
#[test]
fn a_capitalised_heading_is_found_only_when_case_is_folded() {
    let file = "before\n## Pioneer\nafter\n";

    assert_eq!(
        find(file, "## pioneer", None, How::exact()),
        Err(Problem::NotFound(How::exact())),
        "exactly is still exactly, and the default has not moved"
    );

    let (from, to) = find(file, "## pioneer", None, folded()).expect("case folded, it is there");
    // **What comes back is the file, in the file's own case.** Every position a folded
    // character produces points at that character's own bytes, so the range is of the original.
    assert_eq!(&file[from..to], "## Pioneer");
}

/// Folding does not rewrite what is written, only what is compared.
///
/// **The replacement is verbatim and the surrounding text keeps its case**, which is the
/// property that would break if folding were done by lowercasing the text and editing that.
#[test]
fn folding_changes_what_is_compared_and_never_what_is_written() {
    let file = "Before\nThe Quick Brown\nFox Jumps Over\nAfter\n";
    let after = replace(file, "the quick brown fox jumps over", "X", None, folded())
        .expect("case folded and wrapped, it is there");
    assert_eq!(after, "Before\nX\nAfter\n");
}

/// A character whose lower case is more than one character does not break the map.
///
/// **This is the case that would silently corrupt an offset**, and it is why folding is done per
/// character on both sides rather than by lowercasing the two strings. `İ` lowercases to two
/// characters; lowercasing the whole text would change its length and leave the map from
/// normalized positions back to bytes indexed by nothing.
///
/// **Asserted on the bytes either side of the match rather than only on the match**, because an
/// off-by-one in the mapping shows up as a character eaten somewhere else.
#[test]
fn a_character_whose_lower_case_is_longer_does_not_move_the_map() {
    let file = "start \u{130}stanbul end\n";
    let (from, to) = find(file, "\u{130}stanbul", None, folded()).expect("it is there");
    assert_eq!(&file[from..to], "\u{130}stanbul");
    assert_eq!(&file[..from], "start ");
    assert_eq!(&file[to..], " end\n");

    let after = replace(file, "\u{130}stanbul", "X", None, folded()).unwrap();
    assert_eq!(after, "start X end\n");
}

/// A refusal names the normalisations that were in play.
///
/// **This is the property `X-40` asked to have extended**, and it is what made one run enough to
/// diagnose `C-113`: the message said `wrapping ignored`, so the reader knew which normalisation
/// had already been tried and which had not.
#[test]
fn a_refusal_says_which_normalisations_were_tried() {
    let exact = find("abc", "xyz", None, How::exact())
        .unwrap_err()
        .to_string();
    let loose = find("abc", "xyz", None, folded()).unwrap_err().to_string();

    assert!(
        exact.contains("wrapping ignored") && !exact.contains("case folded"),
        "an exact miss must not claim a normalisation it did not do: {exact}"
    );
    assert!(
        loose.contains("case folded, wrapping ignored"),
        "a folded miss must name both, or a reader learns nothing from the failure: {loose}"
    );

    // **And the same for matching twice**, because folding is a way of matching twice where an
    // exact search matched once - and a count of two is baffling until you know that.
    let twice = find("Ab ab", "ab", None, folded()).unwrap_err().to_string();
    assert!(
        twice.contains("case folded, wrapping ignored"),
        "an ambiguous folded match must name the normalisations too: {twice}"
    );
}

/// Folding can turn a working lookup into a refusal, loudly.
///
/// **That is the right direction and it is worth pinning.** An anchor matching two places is
/// refused rather than resolved, so a caller that folds gets told the search became ambiguous
/// instead of quietly getting the first of two.
#[test]
fn folding_refuses_rather_than_choosing_when_it_finds_more() {
    let file = "Ab\nab\n";
    assert!(
        find(file, "ab", None, How::exact()).is_ok(),
        "exactly, there is one"
    );
    assert_eq!(
        find(file, "ab", None, folded()),
        Err(Problem::Ambiguous(2, folded())),
        "folded, there are two, and two is a refusal"
    );
}
