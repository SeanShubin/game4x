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
        net.recipes, 23,
        "the release states twenty-three blocks of recipe rows and the parse found {}",
        net.recipes
    );
    assert_eq!(
        net.names, 19,
        "those blocks are stated under nineteen distinct names and the parse found {}",
        net.names
    );
    // **The deduplication has to remove something**, or a version that stopped deduplicating
    // would pass both assertions above the day the release stops repeating a name.
    assert!(
        net.names < net.recipes,
        "no name is stated twice, so the two counts are checking one thing rather than two"
    );
    // **The partition is over blocks, not over nodes** - `P-376` lets `work` be spelled out
    // as one transition per `(resource, density)`, so one block is sixteen nodes.
    assert_eq!(
        net.blocks_drawn + net.excluded.len(),
        net.recipes,
        "{} block(s) drawn and {} excluded do not account for {} - a block in neither list is \
         one nothing on the page mentions",
        net.blocks_drawn,
        net.excluded.len(),
        net.recipes
    );

    // **One block is excluded and it is `move`** - `P-399` gave it a `put` row, and the
    // release's column description names four roles without that one, so what a `put` does as
    // an arc is undeclared. `C-88`. `work` is still drawn, spelled out per density.
    assert_eq!(
        net.excluded.iter().map(|one| &one.name).collect::<Vec<_>>(),
        [&"move".to_string()],
        "the excluded blocks are not the one expected"
    );

    // **An empty exclusion list means something only because the unfolding is doing work.**
    // A version that silently dropped `work` would also report nothing excluded, and would
    // have fewer transitions than blocks rather than more.
    assert!(
        net.transitions.len() > net.recipes,
        "{} transitions from {} blocks - nothing was spelled out, so *nothing excluded* may \
         mean a block went missing rather than that it was unfolded",
        net.transitions.len(),
        net.recipes
    );
}

/// The one row without a constant weight is spelled out rather than excluded.
///
/// **The rule this test checked has been replaced, not relaxed.** It used to say a recipe is
/// excluded exactly when a row has no constant weight. `P-376` distinguishes two things that
/// look alike: *a rule that takes a thing's upkeep, or makes a territory's density, is one
/// rule with a number per case ... which whatever reads it may spell out*, while *a rule that
/// takes the lesser of the food and the citizens* measures what is present and is forbidden.
/// So a non-constant quantity is now a reason to unfold or a defect, and which one it is
/// depends on the form.
///
/// **Both directions still, because one of them is the failure mode.** Spelling out too
/// little leaves a thin diagram, which a person notices; spelling out with a made-up number
/// is an arc nobody can check, which nobody notices. So the cases are compared against
/// *Territory resources*, read here a second time.
#[test]
fn the_density_rule_is_spelled_out_against_the_planet_it_describes() {
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

    // **One, and it used to be four.** The saturating rewrite took the other three out:
    // `grow` is gone entirely - `P-379` - and the two capacity clamps became `stow` and
    // `discard`, which carry a constant weight. `work` is the last row in the release whose
    // quantity is read from a trait, *`$where`'s density for that resource*.
    assert_eq!(
        expected,
        [&"work".to_string()],
        "the table says these have a quantity that is not a number: {expected:?}"
    );

    // **`work` is drawn, and `move` is the one thing left out** - for a different reason,
    // which is why they are asserted apart. `P-399` gave `move` a `put` row and the release's
    // column description still names four roles, so what a `put` does as an arc is undeclared.
    // `C-88`. Guessing it would draw a game nobody specified.
    assert_eq!(
        net.excluded.iter().map(|one| &one.name).collect::<Vec<_>>(),
        [&"move".to_string()],
        "the excluded blocks are not the one expected"
    );
    assert!(
        net.excluded[0].because.contains("put"),
        "`move` is excluded and the reason does not name the role that caused it: {}",
        net.excluded[0].because
    );

    // **The cases are the planet's, read here from *Territory resources* a second time.** A
    // net that invented a density - a range, or the biome table's guiding numbers, which the
    // release says do not bind - would produce a plausible set of transitions for a planet
    // that does not exist.
    let mut offered: Vec<(String, u32)> = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") || trimmed.starts_with("### ") {
            if inside {
                break;
            }
            inside = trimmed == "### Territory resources";
            continue;
        }
        if !inside || !trimmed.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        if cells.len() < 4 || cells[0].parse::<u32>().is_err() {
            continue;
        }
        for (at, resource) in ["food", "metal", "energy"].iter().enumerate() {
            if let Some((_, density)) = cells[at + 1].split_once('x')
                && let Ok(density) = density.trim().parse::<u32>()
            {
                let pair = ((*resource).to_string(), density);
                if !offered.contains(&pair) {
                    offered.push(pair);
                }
            }
        }
    }
    offered.sort();
    assert_eq!(
        game_console::petri::densities(&document),
        offered,
        "the net's cases and the release's table disagree about what the planet offers"
    );

    // **A `none` cell is no case, not a case yielding nothing.** Territory 6 offers no metal
    // and territory 7 no energy, so a zero-weight arc for either would say `work` fires there
    // and produces nothing.
    assert!(
        offered.iter().all(|(_, density)| *density > 0),
        "a density of zero became a case: {offered:?}"
    );

    let spelled: Vec<&String> = net
        .transitions
        .iter()
        .filter(|name| name.starts_with("work ("))
        .collect();
    assert_eq!(
        spelled.len(),
        offered.len(),
        "{} case(s) drawn against {} the planet offers: {spelled:?}",
        spelled.len(),
        offered.len()
    );
    assert!(
        !offered.is_empty(),
        "the planet offers no density at all, so every count above is against nothing"
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

    // **What leaving `move` out costs, measured against its own rows rather than guessed.**
    // `place` and `unit` are the two families it names, and nothing drawn names either -
    // `move` is the only rule that takes a unit anywhere. So the drawing has no unit moving in
    // it, and the page has to say so rather than leave a reader to notice.
    let cost = game_console::petri::what_exclusion_costs(&net, &document);
    assert_eq!(
        cost,
        ["place".to_string(), "unit".to_string()],
        "what excluding {:?} costs the drawing is {cost:?}",
        net.excluded.iter().map(|one| &one.name).collect::<Vec<_>>()
    );
    assert!(
        !net.excluded.is_empty(),
        "nothing is excluded, so the cost above was counted against an empty population and \
         means nothing"
    );

    // **`resource` is no longer a place, and that is the unfolding rather than a loss.**
    // `work` named the family while it was one transition; spelled out, each case names the
    // member it makes. A family standing in for its members was the last thing in the drawing
    // that was not a thing a territory can hold.
    assert!(
        !names.contains(&"resource".to_string()),
        "the family `resource` is drawn as a place, so `work` is not spelled out after all"
    );

    assert!(
        !net.transitions.is_empty(),
        "the net has no transitions at all, so every emptiness asserted above is about a \
         failed parse rather than about the release"
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
    // **The page said the diagram was complete and it no longer does, which is the sentence
    // working.** It is written under `net.excluded.is_empty()`, so `move` falling out took the
    // claim with it rather than leaving a reassurance nobody re-checked. Asserted in the
    // direction that fails if the claim comes back while something is still missing.
    assert!(
        !page.contains("Nothing is left out"),
        "the page says nothing is left out while `{}` is",
        net.excluded[0].name
    );
    assert!(
        page.contains(&net.excluded[0].name),
        "`{}` is excluded and the page does not name it",
        net.excluded[0].name
    );
    assert!(
        page.contains("became") && page.contains("transitions"),
        "the page does not explain why there are more transitions than blocks"
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

/// The `limit` role is declared, no row carries it, and the net has no zero test.
///
/// **This test was about a translation that no longer exists, and the translation was a rule
/// Sean has since decided against.** It read the release's two `limit 0 garrison` rows as
/// requirements on a garrison's room - `P-374` - on the ground that *there is no garrison* and
/// *there is room for a garrison* say the same thing where one is the most a territory holds.
/// **Read that way a second deployment is refused for want of room.** `P-385` deleted both
/// rows because Sean decided the opposite: *repeated deployments are player choice, safe
/// because they are not capable of causing an infinite resource glitch.* `S-92`.
///
/// **So what is checked is the emptiness, by name.** A role that is declared and unused is a
/// population that went to zero, and `docs/process.md` says a zero means something only
/// against a population that is not also zero - so this asserts the population it counted
/// against, and that the deleted reading cannot return without the build stopping.
#[test]
fn the_limit_role_is_declared_and_no_row_carries_it() {
    let document = release();
    let net = net(&document);

    // **Counted over the table, not over the whole document**, so the column description -
    // which still lists `limit` among the four roles - is not mistaken for a row.
    let rows = document
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .filter(|line| line.contains("| limit "))
        .count();
    assert_eq!(
        rows, 0,
        "the release carries {rows} `limit` row(s); `P-385` deleted the only two, and this \
         file refuses to draw one rather than reviving the reading that refused a repeat"
    );

    // **The population that zero is counted against.** Without this the assertion above is
    // satisfied by a release whose Recipes table stopped parsing, or by one with no rows at
    // all - the sign-flipped failure `CLAUDE.md` names.
    let all: usize = document
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .filter(|line| {
            ["| require ", "| consume ", "| produce "]
                .iter()
                .any(|role| line.contains(role))
        })
        .count();
    assert!(
        all > 60,
        "only {all} rows carry any role at all, so *no limit rows* is a statement about an \
         empty table rather than about the release"
    );

    // The role is still a role, which is what makes the emptiness a fact about the release
    // rather than about this crate.
    assert_eq!(Role::Limit.name(), "limit");

    assert!(
        net.inhibitors().is_empty(),
        "{} arcs are zero tests, so the net is Turing-complete and the page's account of why \
         it is not is wrong",
        net.inhibitors().len()
    );

    // **Nothing is drawn as a requirement on a garrison's room any more**, which is the
    // deleted reading's visible trace. A version that kept the translation would put these
    // back and every other assertion here would still pass.
    let on_room = net
        .arcs
        .iter()
        .filter(|arc| net.places[arc.place].room && net.places[arc.place].kind == "garrison")
        .filter(|arc| arc.role == Role::Require)
        .count();
    assert_eq!(
        on_room, 0,
        "{on_room} arc(s) still require a garrison's room, which is the reading `P-385` \
         removed - a second deployment would be refused for want of room"
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

/// Every label declares a colour, so the drawing is legible in a dark reader as well as a light one.
///
/// **Sean could not read the labels and said so twice before anyone measured the right thing.**
/// The shapes declare `currentColor` and follow the theme; **SVG's initial `fill` is black, not
/// `currentColor`**, so a `<text>` with no fill is painted black whatever the theme - and
/// `report.css` sets `color-scheme: light dark`. In a dark reader every label was black on a
/// dark ground. Selecting the text paints a highlight behind the glyphs, which is how he
/// confirmed it: drag across the drawing and the labels appear.
///
/// **The instrument is what was wrong twice, and it is the usual failure with the sign
/// flipped.** Counting `<text>` elements found one per node and concluded the labels were
/// fine - a correct count answering a narrower question than the one asked, *do labels exist*
/// rather than *do labels render*. **So this counts the fills and not the elements**, which is
/// the question a reader is actually asking.
#[test]
fn every_label_declares_a_colour_the_theme_can_supply() {
    let net = net(&release());
    let drawing = game_console::petri_draw::svg(&net);

    let labels = drawing.matches("<text").count();
    let coloured = drawing.matches("<text").count()
        - drawing
            .split("<text")
            .skip(1)
            .filter(|element| {
                let opening = element.split('>').next().unwrap_or_default();
                !opening.contains("fill=")
            })
            .count();
    assert_eq!(
        coloured,
        labels,
        "{} of {labels} label(s) declare no fill, so a dark reader paints them black on a dark \
         ground - SVG's initial fill is black rather than `currentColor`",
        labels - coloured
    );

    // **The population, because every count here has been wrong once.** A drawing with no
    // labels at all would satisfy the equality above.
    assert!(
        labels >= net.places.len() + net.transitions.len(),
        "{labels} label(s) for {} places and {} transitions - a node with no name is one a \
         reader cannot ask about",
        net.places.len(),
        net.transitions.len()
    );

    // **And the shapes still follow the theme**, which is the half that was already right and
    // would be easy to break while fixing the other.
    assert!(
        drawing.matches("currentColor").count() > labels,
        "the shapes no longer declare `currentColor`, so the drawing has stopped following the \
         reader's theme in the direction that was working"
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
