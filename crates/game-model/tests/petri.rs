//! `R-10` - I can read a generated drawing in the theme I use.
//!
//! **The clauses are the capability's own words**, and each has a check that fails when it is
//! broken rather than a sentence asserting it holds:
//!
//! ```text
//! nothing_in_a_drawing_declares_a_colour_the_theme_does_not_supply
//! every_node_in_every_drawing_carries_its_own_name
//! every_part_says_what_it_leaves_out
//! what_a_part_leaves_out_is_computed_from_the_arcs
//! the_committed_drawing_is_what_the_generator_writes
//! ```
//!
//! **The fourth is the one worth arguing for.** A part could say what it leaves out by carrying a
//! list somebody wrote, and every other check here would pass - so that one re-derives the list
//! from the net and requires the page to agree, which a pasted list fails and a computed one
//! does not. `C-99` is where that pattern comes from, and `R-10` asks for it by name: *computed
//! from the arcs, so a recipe added tomorrow appears in the parts it touches with nobody
//! maintaining a list.*

use std::collections::BTreeSet;

/// The generator, borrowed rather than run as a subprocess - see `tests/browsable.rs`.
#[path = "../examples/petri.rs"]
#[allow(dead_code)]
pub mod petri;

fn reports() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../reports")
}

fn page() -> String {
    std::fs::read_to_string(reports().join("petri.html"))
        .expect("reports/petri.html - run `scripts/reports.sh`")
}

/// **Nothing in it declares a colour the theme does not supply** - `R-10`'s first clause.
///
/// **Asked of every colour rather than of a sample.** A drawing legible in one theme and not the
/// other is one that names a colour directly, so what this refuses is a literal: a hex triple, or
/// any of the keywords a drawing reaches for first. Every `fill` and `stroke` has to be a custom
/// property, which `report.css` defines twice - once for each scheme.
#[test]
fn nothing_in_a_drawing_declares_a_colour_the_theme_does_not_supply() {
    let page = page();

    let mut declared = 0;
    let mut rest = page.as_str();
    while let Some(at) = rest.find("fill=\"").or_else(|| rest.find("stroke=\"")) {
        let from = rest[at..].find('"').expect("an opening quote") + at + 1;
        let end = rest[from..].find('"').expect("a closing quote") + from;
        let value = &rest[from..end];
        assert!(
            value.starts_with("var(--"),
            "a drawing declares the colour `{value}`, which the theme does not supply - so it \
             cannot be legible in both a light and a dark reader"
        );
        declared += 1;
        rest = &rest[end..];
    }

    // **Assert both populations.** With no colours found, every colour is theme-supplied
    // vacuously - and a drawing that declared none at all would inherit a browser default,
    // which is the illegibility this clause is about.
    assert!(
        declared >= 200,
        "only {declared} colour(s) were declared across the drawings, so this checked almost \
         nothing - or the drawings are leaving their colours to the browser"
    );

    // **And no literal anywhere in the page**, which catches a colour written somewhere this
    // walk does not look - a `style` attribute, a gradient, a shadow.
    for literal in [
        "#0", "#1", "#2", "#3", "#4", "#5", "#6", "#7", "#8", "#9", "#a", "#b", "#c", "#d", "#e",
        "#f", "black", "white", "gray", "grey", "rgb(",
    ] {
        assert!(
            !page.contains(literal),
            "`reports/petri.html` contains the colour literal `{literal}`"
        );
    }
}

/// **Every node carries its own name** - `R-10`'s second clause.
///
/// **Asked of the net rather than of the page.** Counting labels in the page would say how many
/// were written; this asks, for every place and every transition the reader found, whether the
/// page names it - so a node that gained a name and lost it fails here.
#[test]
fn every_node_in_every_drawing_carries_its_own_name() {
    let read = petri::nogain::read();
    let page = page();

    let mut nodes = 0;
    for one in &read.ground {
        assert!(
            page.contains(&escaped(&one.rule.name)),
            "the drawing does not name the rule `{}`",
            one.rule.name
        );
        nodes += 1;
        for place in one.took.keys().chain(one.made.keys()) {
            assert!(
                page.contains(&escaped(&place.label())),
                "`{}` reaches the place `{}` and no drawing names it, so a reader meets a circle \
                 it cannot say the meaning of",
                one.rule.name,
                place.label()
            );
            nodes += 1;
        }
    }
    assert!(nodes >= 100, "only {nodes} node(s) were checked");

    // **A circle without a name would pass the loop above**, because that asks whether each name
    // is present and not whether each shape has one. So the shapes are counted too: a place is a
    // circle, a transition a rectangle, and every one of them is drawn beside a `<text>`.
    let circles = page.matches("<circle").count();
    let rects = page.matches("<rect").count();
    let texts = page.matches("<text").count();
    assert!(
        texts >= circles + rects,
        "{circles} circle(s) and {rects} rectangle(s) are drawn against {texts} label(s), so at \
         least one node carries no name"
    );
}

fn escaped(said: &str) -> String {
    said.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// **Shown in parts, and it says what each part leaves out** - `R-10`'s third clause.
#[test]
fn every_part_says_what_it_leaves_out() {
    let read = petri::nogain::read();
    let page = page();

    let parts = page.matches("<svg").count();
    assert_eq!(
        parts,
        read.ground.len(),
        "the net has {} rules and the page draws {parts} part(s)",
        read.ground.len()
    );

    let says = page.matches("Leaves out").count();
    assert_eq!(
        says, parts,
        "{parts} part(s) are drawn and {says} of them say what they leave out"
    );
}

/// **What a part leaves out is computed from the arcs**, not carried as a list.
///
/// **This is the check the others cannot stand in for.** A page with a hand-written list would
/// satisfy every clause above and go stale the first time a rule was added. So the list is
/// re-derived here from the net and the page is required to agree with it, over every part - and
/// the count is asserted, because a re-derivation over no parts agrees with anything.
#[test]
fn what_a_part_leaves_out_is_computed_from_the_arcs() {
    let read = petri::nogain::read();
    let page = std::fs::read_to_string(reports().join("petri.md")).expect("reports/petri.md");

    let mut checked = 0;
    let mut sets: BTreeSet<String> = BTreeSet::new();
    for (at, one) in read.ground.iter().enumerate() {
        let (rules, _) = petri::leaves_out(&read, at);
        let said = section(&page, &one.rule.name);
        for other in &rules {
            assert!(
                said.contains(&format!("`{other}`")),
                "`{}` reaches a place `{other}` also reaches, and its part does not name it",
                one.rule.name
            );
        }
        // **And it names no rule that reaches nothing of its own**, which is the half that
        // catches a list of every rule pasted under every part.
        for other in &read.ground {
            if other.rule.name == one.rule.name || rules.contains(&other.rule.name) {
                continue;
            }
            assert!(
                !said.contains(&format!("`{}`", other.rule.name)),
                "`{}`'s part names `{}`, which reaches none of the places it does - so the list \
                 is not computed from the arcs",
                one.rule.name,
                other.rule.name
            );
        }
        sets.insert(rules.join(","));
        checked += 1;
    }

    assert_eq!(checked, read.ground.len());
    assert!(checked >= 30, "only {checked} part(s) were checked");
    // **A pasted list would be the same under every part.** It is not: the parts differ.
    assert!(
        sets.len() >= 10,
        "only {} distinct leaves-out list(s) across {checked} parts, which is what a pasted list \
         would look like",
        sets.len()
    );
}

/// The lines of `petri.md` under one rule's heading.
fn section<'a>(page: &'a str, rule: &str) -> &'a str {
    let head = format!("\n## {rule}\n");
    let from = page
        .find(&head)
        .unwrap_or_else(|| panic!("no section for `{rule}`"))
        + head.len();
    let rest = &page[from..];
    match rest.find("\n## ") {
        Some(end) => &rest[..end],
        None => rest,
    }
}

/// **The committed drawing is what the generator writes**, so a stale net fails the gate.
#[test]
fn the_committed_drawing_is_what_the_generator_writes() {
    let before: Vec<(std::path::PathBuf, String)> = ["petri.html", "petri.md"]
        .iter()
        .map(|name| {
            let at = reports().join(name);
            let text = std::fs::read_to_string(&at).unwrap_or_default();
            (at, text)
        })
        .collect();
    for (at, text) in &before {
        assert!(
            !text.is_empty(),
            "`{}` is missing or empty - run `scripts/reports.sh`",
            at.display()
        );
    }

    petri::write_report();

    let moved: Vec<String> = before
        .iter()
        .filter(|(at, was)| std::fs::read_to_string(at).unwrap_or_default() != *was)
        .map(|(at, _)| at.display().to_string())
        .collect();
    assert!(
        moved.is_empty(),
        "the committed drawing is not what the rules now say - run `scripts/reports.sh` and \
         commit the result:\n  {}",
        moved.join("\n  ")
    );
}
