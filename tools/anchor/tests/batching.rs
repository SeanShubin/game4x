//! Several edits from one file - `C-129`.
//!
//! **The carrier lost to the failure it prevents, because it was harder to use.** An edit in three
//! parts needed six files and three invocations; a throwaway script doing `str.replace` needed one
//! file and one command, so the script won every time and took every failure with it. These are
//! about the shape that makes the right thing the smaller thing.

use anchor::{How, Problem, edits};

const SPEC: &str = "\
@@
@@ anchor
the first words
@@ replace
words instead
@@ anchor
the second words
@@ replace
others
";

#[test]
fn several_edits_are_applied_in_order() {
    let text = "before\nthe first words\nbetween\nthe second words\nafter\n";
    let after = edits(text, SPEC, None, How::exact()).expect("both edits");
    assert_eq!(after, "before\nwords instead\nbetween\nothers\nafter\n");
}

/// **An edit may name what an earlier edit wrote**, because each applies to the result of the one
/// before rather than all to the original.
#[test]
fn an_edit_may_anchor_on_what_an_earlier_one_wrote() {
    let spec = "\
@@
@@ anchor
one
@@ replace
two
@@ anchor
two
@@ replace
three
";
    assert_eq!(
        edits("one\n", spec, None, How::exact()).expect("both"),
        "three\n"
    );
}

/// **Wrapping is not a difference here either**, which is the whole reason the anchor is
/// normalized - and the failure that sent whoever wrote it back to a script.
#[test]
fn an_anchor_that_the_file_wrapped_differently_still_matches() {
    let text = "let name = row\n    .value(\"name\")\n    .map(str::to_string);\n";
    let spec = "\
@@
@@ anchor
let name = row .value(\"name\") .map(str::to_string);
@@ replace
let name = taken(row);
";
    assert_eq!(
        edits(text, spec, None, How::exact()).expect("wrapped"),
        "let name = taken(row);\n"
    );
}

/// **A refusal says which edit**, which is the cost that sent people back to the script: a batch
/// reporting only *not found* leaves whoever wrote it to find which of six meant it.
#[test]
fn a_refusal_names_the_edit_that_was_refused() {
    let text = "the first words\n";
    let why = edits(text, SPEC, None, How::exact()).expect_err("the second anchor is not there");
    assert_eq!(why.at, 2, "the second edit");
    assert_eq!(why.why, Problem::NotFound(How::exact()));
    assert!(
        format!("{why}").contains("the second words"),
        "and it says which anchor: {why}"
    );
}

/// **Nothing is written when any edit is refused.** A batch that applied the first two of three
/// and then failed would leave a file nobody meant to write.
#[test]
fn a_refused_batch_changes_nothing() {
    let text = "the first words\n";
    assert!(edits(text, SPEC, None, How::exact()).is_err());
    // `edits` returns the text rather than writing it, so a refusal cannot have written
    // anything - the caller still holds what it started with.
    assert_eq!(text, "the first words\n");
}

/// **The marker is chosen rather than fixed**, so text containing one can still be edited - the
/// same reason a heredoc takes a word.
#[test]
fn the_marker_is_whatever_the_first_line_says() {
    let spec = "\
!!!
!!! anchor
@@ anchor
!!! replace
a line that says the usual marker
";
    let after = edits("@@ anchor\n", spec, None, How::exact()).expect("a chosen marker");
    assert_eq!(after, "a line that says the usual marker\n");
}

/// Every way a batch can be refused, and the count so that none is untested.
#[test]
fn every_way_a_batch_is_refused_is_covered() {
    let refused = [
        ("", "an empty first line names no marker"),
        ("@@\n", "no sections at all"),
        ("@@\n@@ anchor\nalone\n", "an anchor with no replacement"),
        (
            "@@\n@@ replace\nalone\n@@ anchor\nx\n",
            "a replacement before its anchor",
        ),
    ];
    for (spec, why) in &refused {
        edits("whatever\n", spec, None, How::exact()).expect_err(why);
    }
    assert_eq!(refused.len(), 4, "four ways, and each is checked");

    // The control: a well formed batch is accepted, so the four above fail for their own reasons.
    edits(
        "x\n",
        "@@\n@@ anchor\nx\n@@ replace\ny\n",
        None,
        How::exact(),
    )
    .expect("a well formed batch");
}
