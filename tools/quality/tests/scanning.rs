//! What the scanner finds, and what it deliberately does not.
//!
//! **Each case is a pair.** A shape that should be reported, beside the same shape with the
//! denominator added, which should not be - because a scanner that reports everything and a
//! scanner that reports the right things pass every one-sided test alike.
//!
//! The fixtures are named `tests/` so `scan` treats them as test code, which is the only thing
//! the path is used for.

use quality::{
    Missing, bound_from_a_literal, cannot_be_empty, population_of, scan, states_a_denominator,
    vacuous_tests,
};

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

/// **The second false positive, and the same cause as the first.** `let sources = [..]` is a
/// literal with a name on it, and three correct tests were reported until this followed the
/// binding - one of them `against_the_release.rs`, which already asserts its own floor and says
/// in a comment that a parser finding nothing would agree with anything.
#[test]
fn a_population_bound_from_a_literal_is_not_reported() {
    let source = r#"
#[test]
fn no_floating_point_anywhere() {
    let sources = [("lib.rs", "a"), ("world.rs", "b")];
    for (name, text) in sources {
        assert!(!text.contains("f32"), "{name}");
    }
}
"#;
    assert_eq!(
        scan(AT, source),
        vec![],
        "a population bound from a literal was reported"
    );

    assert!(bound_from_a_literal(
        "let sources = [(\"a\", 1)];",
        "sources"
    ));
    assert!(bound_from_a_literal("let rows = vec![1, 2];", "rows"));
    assert!(
        !bound_from_a_literal("let rows = parse(text);", "rows"),
        "a call is not a literal, and is the case this exists to keep"
    );
}

/// **The third false positive, found by the code lane, and the same cause again.**
///
/// A nested loop's *enclosing* loop asserts outside it. Blanking every loop in the test said
/// the test asserted nowhere, so `the_derived_graph_is_a_goldberg_polyhedron` was reported -
/// and it asserts its region count before the inner loop and its pentagon count after, both
/// inside the outer one, so an empty collection fails it at once.
#[test]
fn an_enclosing_loops_assertions_are_outside_the_inner_one() {
    let source = r#"
fn class_one_up_to(limit: usize) -> Vec<(usize, usize)> {
    let all = arrangements_up_to(limit);
    assert!(all.len() >= 4, "only {} arrangements", all.len());
    all
}

#[test]
fn the_derived_graph_is_a_goldberg_polyhedron() {
    for (m, n) in class_one_up_to(400) {
        let built = build(m, n);
        assert_eq!(built.neighbours.len(), expected, "region count");
        let mut pentagons = 0;
        for list in &built.neighbours {
            match list.len() {
                5 => pentagons += 1,
                other => panic!("{other} neighbours"),
            }
        }
        assert_eq!(pentagons, 12, "GP({m},{n})");
    }
}
"#;
    // `vacuous_tests` rather than `scan`: the loop genuinely has no denominator, and what
    // makes it harmless is the enclosing loop's assertions, which is the narrowing's question.
    assert_eq!(
        vacuous_tests(AT, source),
        vec![],
        "an inner loop was reported although the enclosing loop asserts around it"
    );

    // The control: with the enclosing assertions gone, the inner loop is reported again.
    let stripped = source
        .replace(
            "        assert_eq!(built.neighbours.len(), expected, \"region count\");\n",
            "",
        )
        .replace("        assert_eq!(pentagons, 12, \"GP({m},{n})\");\n", "");
    assert!(
        stripped.len() < source.len(),
        "the control removed nothing, so it is not a control"
    );
    assert_eq!(
        vacuous_tests(AT, &stripped).len(),
        1,
        "with nothing asserted around it the inner loop should be reported"
    );
}

/// **The shape the code lane named: a filter, not an emptiness.**
///
/// The collection can be full and the filter still match nothing, so asserting its length says
/// nothing. `the_pentagons_are_the_corners_and_are_isolated` passed over a graph with no
/// pentagons - a test named for the pentagons, checking none.
#[test]
fn a_body_wholly_behind_a_continue_is_reported() {
    let filtered = r#"
#[test]
fn the_pentagons_are_isolated() {
    let built = build();
    for (region, list) in built.neighbours.iter().enumerate() {
        if list.len() != 5 {
            continue;
        }
        assert!(matches!(built.sites[region], Site::Corner(_)), "not a vertex");
    }
}
"#;
    let found = scan(AT, filtered);
    assert_eq!(
        found.len(),
        1,
        "the filtered loop was not reported: {found:?}"
    );
    assert_eq!(
        found[0].why,
        Missing::FilteredAway,
        "reported for the wrong reason, so the two shapes are not being told apart"
    );

    // The control: counting what got through and asserting it is the repair.
    let counted = filtered.replace(
        "        assert!(matches!(built.sites[region], Site::Corner(_)), \"not a vertex\");
    }
",
        "        assert!(matches!(built.sites[region], Site::Corner(_)), \"not a vertex\");
        seen += 1;
    }
    assert_eq!(seen, 12, \"ran over the wrong population\");
",
    );
    assert!(
        counted.len() > filtered.len(),
        "the control changed nothing"
    );
    // `vacuous_tests`, because the repair sits *after* the loop: the body is still wholly
    // behind the `continue`, and what makes it harmless is the count that follows it.
    assert_eq!(
        vacuous_tests(AT, &counted),
        vec![],
        "a filtered loop that counts what got through is the repair, not the defect"
    );
    assert_eq!(
        vacuous_tests(AT, filtered).len(),
        1,
        "and without the count it is still reported"
    );
}

/// A comment is prose and may name what it describes.
///
/// The code lane's repair of the pentagons test *describes* the `continue` it removed, and
/// reading that word reported the fix as the defect. `planet-model`'s source guard already
/// says it: only code counts.
#[test]
fn the_word_continue_in_a_comment_is_not_a_filter() {
    let source = r#"
#[test]
fn t() {
    let rows = build();
    assert!(!rows.is_empty());
    for row in &rows {
        // Every assertion below used to sit behind a `continue`, and no longer does.
        assert!(row.ok(), "bad row");
    }
}
"#;
    assert_eq!(
        scan(AT, source),
        vec![],
        "a comment naming `continue` was read as a filter"
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
    // A bound is visible when it is written in the source: a plain number, one carrying a
    // type suffix or a separator, or a named constant. All four were false positives.
    for bound in [
        "0..3",
        "0..8u64",
        "1..=8u64",
        "0..PENTAGON_COUNT",
        "0..2_000",
    ] {
        assert!(cannot_be_empty(bound), "`{bound}` is visible in the source");
    }
    assert!(cannot_be_empty("0..3"));
    assert!(cannot_be_empty("[\"a\"]"));
    assert!(
        !cannot_be_empty("0..files.len()"),
        "a range over a length can be empty and must still be asked about"
    );
    assert!(!cannot_be_empty("values()"));
}
