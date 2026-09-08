//! What the scanner finds, and what it deliberately does not.
//!
//! **Each case is a pair.** A shape that should be reported, beside the same shape with the
//! denominator added, which should not be - because a scanner that reports everything and a
//! scanner that reports the right things pass every one-sided test alike.
//!
//! The fixtures are named `tests/` so `scan` treats them as test code, which is the only thing
//! the path is used for.

use quality::{Missing, cannot_be_empty, population_of, scan, states_a_denominator};

const AT: &str = "crates/x/tests/fixture.rs";

/// `Q-74` itself, reduced: every assertion inside the loop, over a call's result.
const THE_Q74_SHAPE: &str = r#"
#[test]
fn a_path_nobody_has_written_yet_is_covered() {
    let invented = ["a.txt"].map(str::to_string);
    for (path, value) in eol_attributes(&invented) {
        assert_eq!(value, "lf", "{path} is not covered");
    }
}
"#;

/// The same test after `Q-74`, which must not be reported.
const THE_Q74_FIX: &str = r#"
#[test]
fn a_path_nobody_has_written_yet_is_covered() {
    let invented = ["a.txt"].map(str::to_string);
    let answered = eol_attributes(&invented);
    assert_eq!(answered.len(), invented.len(), "answered for fewer than asked");
    for (path, value) in answered {
        assert_eq!(value, "lf", "{path} is not covered");
    }
}
"#;

#[test]
fn the_shape_this_exists_for_is_reported() {
    let found = scan(AT, THE_Q74_SHAPE);
    assert_eq!(
        found.len(),
        1,
        "expected exactly one candidate, got {found:?}"
    );
    assert_eq!(found[0].why, Missing::NeverNamed);
    assert_eq!(found[0].in_fn, "a_path_nobody_has_written_yet_is_covered");
    assert!(
        found[0].over.contains("eol_attributes"),
        "the iterated expression is not carried: {}",
        found[0].over
    );
}

/// **The control, and the half that makes the other half mean something.** A scanner that
/// reported every loop would pass the test above.
#[test]
fn the_same_loop_with_its_denominator_stated_is_not_reported() {
    assert_eq!(
        scan(AT, THE_Q74_FIX),
        vec![],
        "the fix is still reported, so this cannot tell the defect from the repair"
    );
}

/// A loop over a literal cannot be empty, and reporting it would drown the list.
#[test]
fn a_population_visible_in_the_source_is_not_reported() {
    let cases = [
        (
            "for name in [\"a\", \"b\"] { assert!(name.len() > 0); }",
            "array literal",
        ),
        ("for i in 0..3 { assert_eq!(i, i); }", "range literal"),
        ("for i in 0..=9 { assert_eq!(i, i); }", "inclusive range"),
    ];
    assert_eq!(cases.len(), 3, "a case was lost");
    for (source, what) in cases {
        let wrapped = format!("#[test]\nfn t() {{\n    {source}\n}}\n");
        assert_eq!(scan(AT, &wrapped), vec![], "{what} was reported");
    }
}

/// A loop that asserts nothing is setup, not a check.
#[test]
fn a_loop_that_asserts_nothing_is_not_reported() {
    let source = "#[test]\nfn t() {\n    for x in values() {\n        out.push(x);\n    }\n}\n";
    assert_eq!(scan(AT, source), vec![]);
}

/// Only the test module of a `src/` file counts.
#[test]
fn production_code_outside_the_test_module_is_not_scanned() {
    let source = "fn real() {\n    for x in values() {\n        assert!(x > 0);\n    }\n}\n";
    assert_eq!(
        scan("crates/x/src/lib.rs", source),
        vec![],
        "production code was scanned"
    );

    let with_tests = format!("{source}\n#[cfg(test)]\nmod tests {{\n{source}}}\n");
    assert_eq!(
        scan("crates/x/src/lib.rs", &with_tests).len(),
        1,
        "the test module was not scanned, or the production copy was counted too"
    );
}

/// **The worst false positive this scanner had, kept as a case.**
///
/// `crates/sphere-tessellation/tests/poles.rs` asserts the size of its population *inside the
/// helper that computes it*, citing `Q-48`, which is better than asserting it at each use. Four
/// correct tests were reported until the scanner followed the call. A list that sends somebody
/// to fix what is already right costs more than it saves.
#[test]
fn a_population_asserted_where_it_is_computed_is_not_reported() {
    let source = r#"
fn arrangements() -> Vec<(usize, usize)> {
    let all = goldberg::arrangements_up_to(200);
    assert!(all.len() >= 8, "only {} arrangements", all.len());
    all
}

#[test]
fn both_poles_sit_at_the_centre_of_a_pentagon() {
    for (m, n) in arrangements() {
        assert_eq!(neighbours(m, n).len(), 5, "GP({m},{n})");
    }
}
"#;
    assert_eq!(
        scan(AT, source),
        vec![],
        "a population asserted in its helper was reported as unasserted"
    );

    // The control: the same shape with the helper's assertion removed must still be reported.
    let unguarded = source.replace(
        "    assert!(all.len() >= 8, \"only {} arrangements\", all.len());\n",
        "",
    );
    assert!(
        unguarded.len() < source.len(),
        "the control did not remove anything, so it is not a control"
    );
    assert_eq!(
        scan(AT, &unguarded).len(),
        1,
        "with the helper's assertion gone this should be reported again"
    );
}

#[test]
fn a_population_is_the_root_of_the_expression() {
    let cases = [
        ("files", Some("files")),
        ("files.iter()", Some("files")),
        ("&things", Some("things")),
        ("attributes.iter().filter(|x| x.0)", Some("attributes")),
        ("eol_attributes(&invented)", None),
        ("thing.method(x)", Some("thing")),
    ];
    assert_eq!(cases.len(), 6, "a case was lost");
    for (expression, want) in cases {
        assert_eq!(
            population_of(expression).as_deref(),
            want,
            "population of `{expression}`"
        );
    }
}

#[test]
fn a_denominator_is_an_assertion_about_size() {
    let yes = [
        "assert_eq!(answered.len(), 3);",
        "assert!(files.len() > 100, \"only {} files\", files.len());",
        "assert!(!rows.is_empty());",
    ];
    let no = [
        "let n = answered.len();",
        "assert_eq!(answered[0], other);",
        "assert!(files.iter().all(|f| f.ok()));",
    ];
    assert_eq!(yes.len() + no.len(), 6, "a case was lost");
    for text in yes {
        let name = text
            .split(['(', '!', '.'])
            .find(|w| ["answered", "files", "rows"].contains(&w.trim_start_matches('!').trim()));
        let name = name.unwrap_or("answered").trim_start_matches('!').trim();
        assert!(
            states_a_denominator(text, name),
            "`{text}` should state one for `{name}`"
        );
    }
    for text in no {
        assert!(
            !states_a_denominator(text, "answered") && !states_a_denominator(text, "files"),
            "`{text}` should not state a denominator"
        );
    }
}

#[test]
fn only_literal_bounds_make_a_range_safe() {
    assert!(cannot_be_empty("0..3"));
    assert!(cannot_be_empty("[\"a\"]"));
    assert!(
        !cannot_be_empty("0..files.len()"),
        "a range over a length can be empty and must still be asked about"
    );
    assert!(!cannot_be_empty("values()"));
}
