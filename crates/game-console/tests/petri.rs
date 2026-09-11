//! What the Petri net view claims about the release - `S-87`.
//!
//! **The requirement that is not about drawing is the one checked hardest.** A diagram can be
//! ugly and still be true; a diagram that silently omits four of sixteen recipes is a picture
//! of a game that is not this one, and nothing about looking at it would say so.

use game_console::petri::{Role, net};
use game_console::petri_draw::unreached;
use std::path::Path;

fn release() -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../releases/first-release.md"),
    )
    .expect("the release")
}

/// Every recipe the release declares is either drawn or named as not drawn.
///
/// **This is `S-87`'s one requirement about content**, and it is a partition: a recipe that
/// fell out of the parse entirely would be in neither list, and the counts would still look
/// reasonable. So the two are summed against the number of recipes rather than checked apart.
#[test]
fn every_recipe_is_either_drawn_or_named_as_not_drawn() {
    let net = net(&release());

    assert_eq!(
        net.recipes, 16,
        "the release declares sixteen recipes and the parse found {}",
        net.recipes
    );
    assert_eq!(
        net.transitions.len() + net.excluded.len(),
        net.recipes,
        "{} drawn and {} excluded do not account for {} recipes - a recipe in neither list is \
         one nothing on the page mentions",
        net.transitions.len(),
        net.excluded.len(),
        net.recipes
    );
    assert!(
        !net.excluded.is_empty(),
        "no recipe is excluded, which would mean either the release changed or the test for \
         a constant weight stopped working - and the second looks exactly like the first"
    );
    assert!(
        net.transitions.len() > net.excluded.len(),
        "more of the release is undrawable than drawable, which is not a diagram worth \
         publishing"
    );
}

/// A recipe is excluded exactly when one of its rows has no constant weight.
///
/// **Both directions, because one of them is the failure mode.** Excluding too much makes a
/// thin diagram, which a person notices; excluding too little means an arc was drawn with a
/// made-up weight, which nobody notices.
#[test]
fn exclusion_is_decided_by_the_quantity_and_nothing_else() {
    let document = release();
    let net = net(&document);

    // Re-read the table here rather than trusting the net's own account of it: this is the
    // second derivation, and a check that asked the net whether the net was right would be
    // asking one computation twice.
    let mut names = Vec::new();
    let mut current = String::new();
    let mut has_expression = std::collections::BTreeMap::new();
    for line in document.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() < 5 || cells.iter().all(|cell| cell.chars().all(|c| c == '-')) {
            continue;
        }
        let name = cells[0].trim_matches('*').trim();
        if !name.is_empty() {
            current = name.to_string();
            if !names.contains(&current) {
                names.push(current.clone());
            }
        }
        if current.is_empty() || !matches!(cells[2], "require" | "limit" | "consume" | "produce") {
            continue;
        }
        let numeric = cells[3].parse::<u32>().is_ok();
        *has_expression.entry(current.clone()).or_insert(false) |= !numeric;
    }

    let expected: Vec<&String> = names
        .iter()
        .filter(|name| has_expression.get(*name).copied().unwrap_or(false))
        .collect();
    let actual: Vec<&String> = net.excluded.iter().map(|out| &out.name).collect();

    assert_eq!(
        actual, expected,
        "the net excludes {actual:?} and the table says the ones with an expression are \
         {expected:?}"
    );
    assert_eq!(
        expected.len(),
        4,
        "four recipes have a quantity that is not a number, and these are {expected:?}"
    );
}

/// The exclusions cost places, and the page says which.
///
/// **This is the part a count of recipes hides.** Four rows leaving is four rows; what it
/// actually costs is that whole kinds stop appearing, and a reader looking at the drawing for
/// `food` would conclude the game has none.
#[test]
fn what_the_exclusions_cost_is_visible_rather_than_implied() {
    let document = release();
    let net = net(&document);
    let missing = unreached(&net);
    assert!(
        missing.is_empty(),
        "a place named by no arc is in the net's own list and should not be: {:?}",
        missing
            .iter()
            .map(|place| place.label())
            .collect::<Vec<_>>()
    );

    // `food` is the case. It is a kind of the release, and every recipe that moves it -
    // `work`, `upkeep`, `grow` - has a state-dependent amount, so the drawn net never names
    // it. The page has to say so, and this is what says the page has to.
    let names: Vec<String> = net.places.iter().map(|place| place.label()).collect();
    assert!(
        !names.contains(&"food".to_string()),
        "food appears as a place, so the sentence on the page explaining that it cannot is \
         now false and should be removed"
    );
    assert!(
        names.contains(&"metal".to_string()),
        "metal does appear, which is what makes food's absence a fact about the exclusions \
         rather than about resources"
    );

    let page = game_console::petri_page::markdown(&document);
    assert!(
        page.contains("not drawn"),
        "the page never says anything is not drawn"
    );
    for excluded in &net.excluded {
        assert!(
            page.contains(&excluded.name),
            "`{}` is excluded and the page does not name it",
            excluded.name
        );
    }
}

/// The two zero tests are found, and drawn as something other than an ordinary arc.
///
/// **They are the reason the picture is worth more than the table.** A plain Petri net has
/// decidable reachability and one with inhibitor arcs does not, so which arcs these are is the
/// single most consequential thing on the page.
#[test]
fn the_zero_tests_are_marked_as_what_they_are() {
    let document = release();
    let net = net(&document);

    let inhibitors = net.inhibitors();
    assert_eq!(
        inhibitors.len(),
        2,
        "the release has two `limit 0` rows and the net found {}",
        inhibitors.len()
    );
    for arc in &inhibitors {
        assert_eq!(arc.role, Role::Limit);
        assert_eq!(arc.weight, 0);
        assert_eq!(
            net.places[arc.place].kind, "garrison",
            "both zero tests are on a garrison, which is bounded by a capacity of 1 - that is \
             what makes them affordable, and a zero test somewhere unbounded would not be"
        );
    }

    let drawing = game_console::petri_draw::svg(&net);
    assert!(
        drawing.contains("url(#o)"),
        "no arc is drawn with the inhibitor head, so the two are indistinguishable from \
         ordinary requirements"
    );
}

/// The drawing needs no script, which `R-9` requires and every diagram library would break.
///
/// **Asserted here as well as in `browsable.rs`** because the two are about different things:
/// that one asks it of every generated page, and this asks it of the one that would most
/// plausibly acquire one. A Petri net is exactly what somebody reaches for a renderer to draw.
#[test]
fn the_drawing_arrives_drawn() {
    let net = net(&release());
    let drawing = game_console::petri_draw::svg(&net);
    for forbidden in [
        "<script",
        "javascript:",
        " onclick=",
        "mermaid",
        "<foreignObject",
    ] {
        assert!(
            !drawing.contains(forbidden),
            "the drawing contains {forbidden:?}, and `R-9` says no page needs JavaScript to \
             be read"
        );
    }
    assert!(
        drawing.starts_with("<svg") && drawing.trim_end().ends_with("</svg>"),
        "the drawing is not one SVG element"
    );
}

/// Generating twice gives the same bytes, which is what lets it be committed.
///
/// **The layout is the risk.** A force-directed placement would be a floating-point loop whose
/// last digit no library promises across platforms, and `dumps_are_current` compares this file
/// byte for byte against what CI regenerates. This is why the ordering is integer arithmetic.
#[test]
fn the_same_release_gives_the_same_bytes() {
    let document = release();
    assert_eq!(
        game_console::petri_page::markdown(&document),
        game_console::petri_page::markdown(&document)
    );
    assert_eq!(
        game_console::petri_page::page(&document),
        game_console::petri_page::page(&document)
    );
}
