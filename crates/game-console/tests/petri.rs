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

    // **Two numbers, because the release states some recipes more than once.** `P-373` makes
    // a rule whose subject is a family a rule for each of them, so `stow` is stated twice and
    // `discard` four times - twenty-four blocks of rows under twenty names. A net draws the
    // blocks: `discard` metal and `discard` labor take different things and are different
    // transitions.
    assert_eq!(
        net.recipes, 24,
        "the release states twenty-four blocks of recipe rows and the parse found {}",
        net.recipes
    );
    assert_eq!(
        net.names, 20,
        "those blocks are stated under twenty distinct names and the parse found {}",
        net.names
    );
    // **The deduplication has to remove something**, or a version that stopped deduplicating
    // would pass both assertions above the day the release stops repeating a name.
    assert!(
        net.names < net.recipes,
        "no name is stated twice, so the two counts are checking one thing rather than two"
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
    // **One, and it used to be four.** The saturating rewrite took the other three out:
    // `grow` is gone entirely - `P-379` - and the two capacity clamps became `stow` and
    // `discard`, which carry a constant weight. `work` is the last row in the release whose
    // quantity is read from the state, *`$where`'s density for that resource*.
    assert_eq!(
        expected.len(),
        1,
        "one recipe has a quantity that is not a number, and these are {expected:?}"
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

    // **`food` is drawn now, and it was the whole of this test's point until `S-88`.** Every
    // recipe that moved it - `work`, `upkeep`, `grow` - once had a state-dependent amount, so
    // all three were excluded and the drawing had no food anywhere. `grow` is gone since
    // `P-379` and `upkeep` carries a constant weight. **Asserted in the direction that would
    // fail if the rewrite were undone**, rather than deleted: the claim the page used to make
    // is false and a reader has to be able to see when it stops being.
    let names: Vec<String> = net.places.iter().map(|place| place.label()).collect();
    for kind in ["food", "metal", "energy"] {
        assert!(
            names.contains(&kind.to_string()),
            "`{kind}` is not drawn, so some recipe that moves it has acquired a \
             state-dependent amount and the page's account of what is missing is now short"
        );
    }

    // **What the exclusion costs, asked against the excluded rows rather than the kinds.**
    // `work` produces a `resource`, which is a family - so a check reading the *Kinds* table
    // would find nothing missing and report a cost of zero. That is the narrow-predicate
    // failure, and a zero would have looked like good news.
    let cost = game_console::petri::what_exclusion_costs(&net, &document);
    assert_eq!(
        cost,
        vec!["resource".to_string()],
        "what the {} excluded recipe(s) cost the drawing is {cost:?}",
        net.excluded.len()
    );
    assert!(
        !net.excluded.is_empty(),
        "nothing is excluded, so the cost above was counted against an empty population and \
         means nothing"
    );

    // **And the page says so, which is the half a test alone does not achieve.** This
    // assertion existed before the page did: the accounting reported places with no arcs,
    // which is always empty, so the finding lived in this file and nowhere a reader goes.
    //
    // **What is never drawn is now a different fact and the page must not tell the old
    // story.** These four are absent because no recipe names them, which would be true of a
    // drawing with nothing left out at all - so the page has to say that rather than blame
    // the exclusion, and this asserts it does.
    let never = game_console::petri::kinds_never_drawn(&net, &document);
    assert_eq!(
        never,
        ["orbit", "deposit", "adjacency", "game"]
            .map(String::from)
            .to_vec(),
        "a different set of kinds is never drawn, and the page's explanation of why is \
         written for these four: {never:?}"
    );

    let page = game_console::petri_page::markdown(&document);
    assert!(
        page.contains("not drawn"),
        "the page never says anything is not drawn"
    );
    assert!(
        page.contains("not the exclusion's doing"),
        "the page does not separate what the exclusion costs from what no recipe names, and \
         a reader will read the second as the first"
    );
    assert!(
        page.contains("would conclude the game has none"),
        "the page does not tell a reader what the exclusion costs them"
    );
    for kind in &never {
        assert!(
            page.contains(kind.as_str()),
            "`{kind}` is never drawn and the page does not name it"
        );
    }
    for excluded in &net.excluded {
        assert!(
            page.contains(&excluded.name),
            "`{}` is excluded and the page does not name it",
            excluded.name
        );
    }
}

/// The two zero tests are gone, and room is what replaced them.
///
/// **`P-374` removed the only reason this net was Turing-complete.** Reachability in a plain
/// Petri net is decidable and an inhibitor arc destroys that; the release had two, both
/// `limit 0 garrison`. Capacity is stored as room left now, so *there is no garrison here*
/// became *the garrison's room is untouched* - an ordinary requirement on an ordinary place.
///
/// **The release still says `limit 0` twice**, which is why this checks both ends: the rows
/// are still there, and no arc is an inhibitor. A version that lost the rows instead would
/// pass a check that only counted inhibitors.
#[test]
fn the_zero_tests_became_claims_on_room() {
    let document = release();
    let net = net(&document);

    let rows = document
        .lines()
        .filter(|line| line.contains("| limit ") && line.contains("| 0 "))
        .count();
    assert_eq!(
        rows, 2,
        "the release states `limit 0` twice and this found {rows} - if it is now zero the \
         translation below is checking nothing"
    );

    assert!(
        net.inhibitors().is_empty(),
        "{} arcs are still zero tests, so the net is still Turing-complete and the page's \
         historical note is wrong",
        net.inhibitors().len()
    );

    // What they became: a requirement on the room for a garrison, which is the same claim
    // only because a garrison's capacity is one.
    let on_room: Vec<&game_console::petri::Arc> = net
        .arcs
        .iter()
        .filter(|arc| net.places[arc.place].room && net.places[arc.place].kind == "garrison")
        .filter(|arc| arc.role == Role::Require)
        .collect();
    assert_eq!(
        on_room.len(),
        2,
        "two `limit 0 garrison` rows should have become two requirements on a garrison's \
         room, and {} did",
        on_room.len()
    );
    assert_eq!(
        game_console::petri::stated_capacity("garrison"),
        Some(1),
        "the translation is exact only at a capacity of one, and this is what says the \
         release still declares one"
    );

    let drawing = game_console::petri_draw::svg(&net);
    assert!(
        !drawing.contains("url(#o)"),
        "an arc is still drawn with the inhibitor head and there are none left to draw"
    );
}

/// Room is spent when a thing is made and given back when it is destroyed - `P-374`.
///
/// **Both directions, over every bounded kind**, because a version that only took room would
/// drain every place to zero and a version that only gave it back would fill them, and either
/// looks plausible in one recipe.
#[test]
fn making_takes_room_and_destroying_gives_it_back() {
    let net = net(&release());

    let mut checked = 0;
    for arc in &net.arcs {
        let place = &net.places[arc.place];
        if place.room || !game_console::petri::bounded(&place.container, &place.kind) {
            continue;
        }
        let opposite = match arc.role {
            Role::Produce => Role::Consume,
            Role::Consume => Role::Produce,
            // A requirement takes nothing and makes nothing, so it moves no room.
            _ => continue,
        };
        assert!(
            net.arcs.iter().any(|other| {
                other.transition == arc.transition
                    && net.places[other.place].room
                    && net.places[other.place].kind == place.kind
                    && other.role == opposite
                    && other.weight == arc.weight
            }),
            "`{}` {} {} {} and no matching room arc goes the other way",
            net.transitions[arc.transition],
            arc.role.name(),
            arc.weight,
            place.label()
        );
        checked += 1;
    }
    assert!(
        checked > 10,
        "only {checked} arcs on bounded kinds were paired, which is too few for this to be \
         about the release rather than about one recipe"
    );

    // And nothing unbounded got room, which `P-372` requires: a territory declares no limit
    // for a resource, so there is no room to be short of.
    //
    // **In a territory**, which is the qualifier the first version of this left out - and it
    // was wrong in the same way the code was: energy in a territory has no limit and energy
    // in a unit's tank is bounded by that unit's fuel, so a check keyed on the kind alone
    // asks about neither.
    for (kind, _) in game_console::petri::UNBOUNDED {
        assert!(
            !net.places
                .iter()
                .any(|place| place.room && place.kind == kind && place.container == "a territory"),
            "`{kind}` has a room place in a territory and the release declares no limit for it"
        );
    }
}

/// Every kind the release bounds is classified, and a new row fails until somebody decides.
///
/// **This is the check that stops the list going quietly stale.** `bounded` is a written list
/// rather than a predicate over the words, because the bounds are prose - *a capacity of 1*,
/// *as many as the extractors of its resource* - and a rule matching on `capacity` would call
/// the store unbounded, which is the narrow-predicate failure this repository keeps recording.
/// A list is safe only with something asserting it covers the population.
#[test]
fn every_bound_the_release_states_is_classified() {
    let document = release();
    let mut rows = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        if line.starts_with("## ") {
            if inside {
                break;
            }
            inside = line.trim() == "## What bounds a kind in a territory";
            continue;
        }
        let line = line.trim();
        if !inside || !line.starts_with("| **") {
            continue;
        }
        let name = line
            .trim_matches('|')
            .split('|')
            .next()
            .unwrap_or_default()
            .trim()
            .trim_matches('*')
            .to_string();
        if !name.is_empty() {
            rows.push(name);
        }
    }

    // Twelve since `P-380` gave `fertility` its own row - *the citizens that make it, one
    // each per turn*, which is `labor`'s word for word. It was eleven, and the number is
    // written rather than counted from the same table the loop above counts, because a test
    // comparing a count with itself agrees with any release at all.
    assert_eq!(
        rows.len(),
        12,
        "the release bounds twelve kinds and this found {}: {rows:?}",
        rows.len()
    );

    for kind in &rows {
        let in_bounded = game_console::petri::BOUNDED
            .iter()
            .any(|(name, _)| name == kind);
        let in_unbounded = game_console::petri::UNBOUNDED
            .iter()
            .any(|(name, _)| name == kind);
        assert!(
            in_bounded != in_unbounded,
            "`{kind}` is {} - every kind the release bounds is exactly one of the two, and a \
             new row has to be decided rather than defaulting",
            if in_bounded {
                "in both lists"
            } else {
                "in neither list: decide whether a territory declares room for it, in \
                 `petri::BOUNDED` or `petri::UNBOUNDED`"
            }
        );
    }
    assert_eq!(
        game_console::petri::BOUNDED.len() + game_console::petri::UNBOUNDED.len(),
        rows.len(),
        "the two lists together name something the release does not bound"
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
