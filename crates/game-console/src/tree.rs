//! The containment tree, as a page that collapses.
//!
//! **`S-54`, and Sean asked for it in his own words**: he wanted to see the containers and
//! the capacities and how everything fits together. `spec/logistics.md` -> *Containment* is
//! the whole specification of what is drawn here.
//!
//! # Why it is a page with no markdown beside it
//!
//! Every other report has both, because `P-246` gives them jobs that do not overlap -
//! markdown is what a change is reviewed on, HTML is what things are browsed on. **This one
//! is only browsable.** Collapsing is the feature: twelve territories and twelve orbits are
//! twenty-five lines shut and several hundred open, and a markdown file is always open. A
//! markdown twin would be the same information in the form that made it unreadable, and a
//! second file to keep current.
//!
//! `<details>` and `<summary>` collapse natively, so there is **no JavaScript and no
//! library** - which is what makes it readable from `file://` and what stops the page
//! acquiring a dependency the rest of the reports do not have.
//!
//! # Generated from the scenario, never drawn
//!
//! **`S-54`'s trap, in its own words**: a hand-authored tree can show a structure the code
//! cannot produce, and that failed three times in one day in this repository - `houses`
//! requiring something no kind carries, `grow` publishing quantities the model does not
//! compute, `phase` printing a value the release never named. Each was a published artifact
//! describing behaviour that is not there.
//!
//! So this renders [`game_model::containment::tree`], the same projection the data file is
//! written from. A generated tree cannot lie about the model.

use game_model::containment::{Capacity, Entry, may_contain};

/// The tree as a page.
pub fn page(game: &game_model::Game, title: &str) -> String {
    let mut out = crate::dump::head(title);
    out.push_str(&format!("<h1>{}</h1>\n", crate::dump::escaped(title)));
    out.push_str(
        "<p class=\"note\">Generated. Do not edit. Every thing is inside what holds it, and \
         nothing states its container - <code>spec/console.md</code>. A number after a \
         description is how many there are; <code>used/total</code> after a container is what \
         it holds against what it may.</p>\n",
    );
    out.push_str(
        "<p class=\"note\">A territory's <strong>density</strong> is not here. It is a stored \
         trait per resource and this shows containment, which density is not part of - see \
         <code>state.md</code>'s <code>territory-resource</code> table, and <code>C-46</code> \
         for why it is in neither this nor the data file.</p>\n",
    );
    out.push_str("<ul class=\"tree\">\n");
    node(&mut out, &tree_root(game), 0);
    out.push_str("</ul>\n</body>\n</html>\n");
    out
}

/// The same tree, as an indented list - `R-9`'s diffable sibling.
///
/// **`S-54` argued for this and was refused, and `S-64` settles it with a precedent rather
/// than an argument.** The refusal reasoned that collapsing is what makes the tree readable
/// and a markdown file is always open, so a twin would be the same information in the form
/// that made it unreadable. That is true of *reading* it and beside the point of what this
/// is for: `graph.html` has `graph.txt` beside it in the model Sean pointed at, and a `.txt`
/// is not there to be read - it is there so that a change to a generated view shows up as a
/// diff. **A page is browsed and a sibling is diffed**, and no file does both.
///
/// So this is deliberately the flat form: every node, at its depth, always open. What made
/// it a bad page is exactly what makes it a good diff - one line moves when one thing moves.
pub fn markdown(game: &game_model::Game, title: &str) -> String {
    let mut out = format!("# {title}\n\n");
    out.push_str(
        "**Generated. Do not edit.** The diffable sibling of `containment.html`, which is the \
         one to read -\nthis is the same tree with nothing collapsed, so that a change to it \
         is one line of a diff.\n\n",
    );
    let mut lines = 0;
    walk(&mut out, &tree_root(game), 0, &mut lines);
    out.push_str(&format!(
        "\n{lines} things, counting every container and everything in one.\n"
    ));
    out
}

fn walk(out: &mut String, entry: &Entry, depth: usize, lines: &mut usize) {
    let quantity = if entry.quantity == 1 {
        String::new()
    } else {
        format!(" x {}", entry.quantity)
    };
    let bounds = summary_of(&entry.capacity);
    // The page marks what is full with a colour, which a diff cannot carry, so the sibling
    // says it in words. **Not a second rendering of the same fact** - `summary_of` produces
    // the text and the page adds the colour to it.
    let bounds = if bounds.is_empty() {
        String::new()
    } else {
        format!(
            " - {}",
            bounds
                .replace("<span class=\"bound full\">", "")
                .replace("<span class=\"bound\">", "")
                .replace("</span>", "")
        )
    };
    out.push_str(&format!(
        "{}- {}{quantity}{bounds}\n",
        "  ".repeat(depth),
        entry.description.written()
    ));
    *lines += 1;
    for held in &entry.contents {
        walk(out, held, depth + 1, lines);
    }
}

fn tree_root(game: &game_model::Game) -> Entry {
    game_model::containment::tree(game)
}

/// Whether a node at this depth is open when the page loads.
///
/// **The root alone, so the page opens on the shape and not on the contents.** The game and
/// every place in it are visible at once - twenty-five lines - and what is *in* a place is one
/// click away.
///
/// `S-54` asked for about fifteen lines collapsed and this is twenty-five, because the twelve
/// orbits are places in their own right. Each still says on its own summary line whether it is
/// full, which is what the collapsing is for.
fn opens(depth: usize) -> bool {
    depth == 0
}

fn node(out: &mut String, entry: &Entry, depth: usize) {
    let described = crate::dump::escaped(&entry.description.written());
    let quantity = if entry.quantity == 1 {
        String::new()
    } else {
        format!(" <span class=\"many\">&times; {}</span>", entry.quantity)
    };
    let bounds = summary_of(&entry.capacity);

    // **A container is a container whether or not anything is in it.** Deciding by what is
    // there would draw an empty orbit as a leaf - the same picture as a citizen, which can
    // never hold anything - and `spec/logistics.md` says those are different facts. The root
    // is a container by being the thing everything is in.
    let container = depth == 0
        || game_model::thing::Kind::ALL
            .into_iter()
            .any(|kind| kind.name() == entry.description.kind && may_contain(kind));
    if !container {
        out.push_str(&format!("<li class=\"leaf\">{described}{quantity}</li>\n"));
        return;
    }

    let open = if opens(depth) { " open" } else { "" };
    out.push_str(&format!(
        "<li><details{open}><summary>{described}{quantity}"
    ));
    if !bounds.is_empty() {
        out.push_str(&format!(" <span class=\"bounds\">{bounds}</span>"));
    }
    out.push_str("</summary>\n");

    if entry.contents.is_empty() {
        out.push_str("<p class=\"nothing\">nothing in it</p>\n");
    } else {
        out.push_str("<ul>\n");
        for held in &entry.contents {
            node(out, held, depth + 1);
        }
        out.push_str("</ul>\n");
    }
    out.push_str("</details></li>\n");
}

/// The `used/total` line for a container, and only where a total is declared.
///
/// **A capacity of zero is left off rather than printed as `0/0`.** `spec/logistics.md`: *a
/// kind that declares no capacity contains nothing, and never can* - so a territory with no
/// metal is not a container of metal that happens to be empty, and printing it as one would
/// say the opposite of what the rule says.
///
/// **What is full is marked.** The reason a summary carries these at all is that a collapsed
/// container which cannot say whether it is full defeats collapsing it.
fn summary_of(capacity: &[Capacity]) -> String {
    let mut parts = Vec::new();
    for bound in capacity {
        if bound.total == 0 {
            continue;
        }
        let named = bound
            .of
            .written()
            .trim_matches(|c| c == '{' || c == '}')
            .replace(' ', "&nbsp;");
        let full = if bound.available() == 0 { " full" } else { "" };
        parts.push(format!(
            "<span class=\"bound{full}\">{named}&nbsp;{}/{}</span>",
            bound.used, bound.total
        ));
    }
    parts.join(" ")
}

// The tree's rules live in `crate::style` with everything else - `R-9`. They were here
// because this page was the only one that used them, which is exactly the argument that put
// `.empty` in two stylesheets meaning two different things.

#[cfg(test)]
mod tests {
    use super::*;
    use game_model::identity::{Resource, TerritoryId};
    use game_model::thing::Kind;

    /// A world holding one of every containment relationship this release has.
    ///
    /// **`S-54`'s round-trip fixture.** It is a test asset rather than something Sean reads:
    /// the page he reads is generated from the main scenario, so that he observes the real
    /// game. This exists so that every relationship is exercised somewhere, including the
    /// ones the scenario happens not to reach - a unit standing on the ground, and a capacity
    /// at its bound.
    pub fn every_relationship() -> game_model::Game {
        let mut game = game_model::Game::new();
        game.phase = game_model::Phase::Play;
        for (id, biome) in [
            (1, game_model::Biome::Grassland),
            (2, game_model::Biome::Mountain),
        ] {
            let mut place =
                game_model::Territory::empty(TerritoryId(id), game_model::Biome::Grassland);
            place.biome = biome;
            place.force_of_nature = 1;
            place.deposits.insert(
                Resource::Food,
                game_model::Deposit {
                    capacity: 2,
                    density: 4,
                },
            );
            place.deposits.insert(
                Resource::Metal,
                game_model::Deposit {
                    capacity: 1,
                    density: 3,
                },
            );
            game.territories.push(place);
        }

        let first = &mut game.territories[0];
        first.put(Kind::Citizen, 3);
        first.set_garrison(Some(game_model::territory::Garrison {
            force: 0,
            manned: 0,
        }));
        first.put(Kind::Yard, 1);
        // Both food extractors, which puts one capacity at its bound.
        first.add_extractor(Resource::Food);
        first.add_extractor(Resource::Food);
        first.add_store(Resource::Food);
        first.add(Resource::Food, 5);
        first.add(Resource::Metal, 2);

        // A unit in orbit, and a unit on the ground.
        game.units.push(game_model::Unit::new(
            game_model::UnitId(1),
            game_model::UnitKind::Ark,
            TerritoryId(1),
        ));
        let mut landed = game_model::Unit::new(
            game_model::UnitId(2),
            game_model::UnitKind::Pioneer,
            TerritoryId(2),
        );
        landed.location = game_model::Location::On(TerritoryId(2));
        game.units.push(landed);
        game
    }

    /// Every containment relationship the release has appears in the fixture.
    ///
    /// **Over the relationships rather than on one of them, and the count with it.** A
    /// fixture that exercised five of the six would render a page that looks complete, and
    /// the sixth would be drawn by nothing.
    #[test]
    fn the_fixture_holds_one_of_every_containment_relationship() {
        let tree = game_model::containment::tree(&every_relationship());
        let mut seen: Vec<(String, String)> = Vec::new();
        fn walk(entry: &Entry, seen: &mut Vec<(String, String)>) {
            for held in &entry.contents {
                seen.push((
                    entry.description.kind.to_string(),
                    held.description.kind.to_string(),
                ));
                walk(held, seen);
            }
        }
        walk(&tree, &mut seen);

        let wanted: [(&str, &str); 10] = [
            ("game", "territory"),
            ("game", "orbit"),
            ("orbit", "ark"),
            ("territory", "pioneer"),
            ("territory", "citizen"),
            ("territory", "garrison"),
            ("territory", "yard"),
            ("territory", "extractor"),
            ("territory", "store"),
            ("territory", "food"),
        ];
        assert_eq!(
            wanted.len(),
            10,
            "ten relationships the fixture must exercise"
        );
        for (container, held) in wanted {
            assert!(
                seen.iter().any(|(a, b)| a == container && b == held),
                "the fixture has no {held} in a {container}, so the page never draws one"
            );
        }
        assert!(
            seen.len() >= wanted.len(),
            "only {} relationships in the fixture at all",
            seen.len()
        );
    }

    /// The fixture survives being written to a data file and read back.
    ///
    /// **`S-54` calls the small world the round-trip fixture and this is that.** The main
    /// scenario round trips too - `tests/expected_state.rs` - but it happens not to reach
    /// every relationship: nothing in it ever stands a unit on the ground, and no capacity in
    /// it is bounded at one. A round trip over a state that omits a relationship proves
    /// nothing about that relationship.
    #[test]
    fn the_fixture_round_trips_through_the_data_file() {
        let game = every_relationship();
        let direct = game_model::containment::tree(&game);
        let written = crate::state::write(&game, "the fixture");
        let read = crate::state::read(&written).unwrap_or_else(|why| panic!("{why}"));
        assert_eq!(
            read,
            direct.contained(),
            "what was written is not what was read"
        );
        assert_eq!(
            crate::state::written(&read),
            crate::state::written(&direct),
            "and writing it again is the same bytes"
        );
        assert!(
            direct.walk().len() >= 12,
            "only {} entries in the fixture, so the round trip covers almost nothing",
            direct.walk().len()
        );
    }

    /// At least one capacity is at its bound, so the page's *full* marking is exercised.
    #[test]
    fn the_fixture_puts_a_capacity_at_its_bound() {
        let tree = game_model::containment::tree(&every_relationship());
        let at_bound: Vec<String> = tree
            .walk()
            .into_iter()
            .flat_map(|entry| entry.capacity.iter())
            .filter(|bound| bound.total > 0 && bound.available() == 0)
            .map(|bound| format!("{} {}/{}", bound.of.written(), bound.used, bound.total))
            .collect();
        assert!(
            !at_bound.is_empty(),
            "nothing is full, so the page's `full` marking is drawn by nothing"
        );
        assert!(
            at_bound
                .iter()
                .any(|line| line.starts_with("{extractor resource:food}")),
            "the two food extractors fill the ground's capacity for them: {at_bound:?}"
        );
    }

    /// The page is one `<details>` per container, needs no script, and marks what is full.
    #[test]
    fn the_page_collapses_without_a_script_and_says_what_is_full() {
        let text = page(&every_relationship(), "containment");
        assert!(
            !text.contains("<script") && !text.contains("javascript"),
            "no script and no library - that is what makes it readable from file://"
        );
        // The game, two territories, two orbits and the store: every container in the
        // fixture, whether or not anything is in it.
        assert_eq!(
            text.matches("<details open>").count(),
            1,
            "only the game opens by default, so the page is the shape rather than the contents"
        );
        let containers = text.matches("<details").count();
        assert_eq!(
            containers, 6,
            "the game, two territories, two orbits and a store are containers here; \
             {containers} were drawn"
        );
        assert_eq!(
            containers,
            text.matches("</details>").count(),
            "every `details` is closed, or a browser draws the rest of the page inside it"
        );
        assert!(
            text.contains("class=\"bound full\""),
            "nothing is marked full, and a collapsed container that cannot say whether it is \
             full defeats collapsing it"
        );
        assert!(
            text.contains("2/2"),
            "the food extractors are two of two and the summary says so"
        );
        // A quantity is shown where there is more than one, and not where there is one.
        assert!(
            text.contains("&times; 3"),
            "three citizens are one entry at a quantity of three"
        );
        // An empty container says it is empty rather than looking like a leaf.
        assert!(
            text.contains("nothing in it"),
            "an orbit with no unit in it is a container that is empty, not a thing that \
             cannot hold - `spec/logistics.md` says those are different"
        );
    }

    /// An empty orbit is drawn as a container and a citizen is not.
    ///
    /// **The distinction `spec/logistics.md` makes, checked on the two cases that differ.**
    /// Both hold nothing; only one of them never could.
    #[test]
    fn what_can_hold_is_drawn_as_a_container_and_what_cannot_is_not() {
        assert!(
            may_contain(Kind::Orbit) && may_contain(Kind::Territory) && may_contain(Kind::Store)
        );
        let cannot: Vec<Kind> = Kind::ALL
            .into_iter()
            .filter(|kind| !may_contain(*kind))
            .collect();
        assert_eq!(
            cannot.len(),
            12,
            "four of the sixteen kinds may contain; {} may not",
            cannot.len()
        );

        let text = page(&every_relationship(), "containment");
        // The orbit above territory 2 holds nothing and is still a container.
        let empty_orbit = text
            .split("{orbit id:2}")
            .nth(1)
            .expect("the orbit above territory 2 is on the page");
        assert!(
            empty_orbit.starts_with("</summary>"),
            "an empty orbit opens a `details` and says it is empty"
        );
        // A citizen holds nothing and can never hold, so it is a leaf.
        assert!(
            text.contains("<li class=\"leaf\">{citizen"),
            "a citizen is a leaf, because no capacity is declared for one"
        );
    }

    /// A capacity of zero is left off rather than drawn as `0/0`.
    ///
    /// `spec/logistics.md`: *a kind that declares no capacity contains nothing, and never
    /// can* - so drawing it as an empty container says the opposite of the rule.
    #[test]
    fn a_kind_a_territory_cannot_hold_is_not_drawn_as_an_empty_container() {
        let game = every_relationship();
        let tree = game_model::containment::tree(&game);
        let energy: Vec<&Capacity> = tree
            .walk()
            .into_iter()
            .flat_map(|entry| entry.capacity.iter())
            .filter(|bound| bound.of.written().contains("energy"))
            .collect();
        assert!(
            !energy.is_empty(),
            "the fixture declares no energy anywhere, so this checks nothing"
        );
        assert!(
            energy.iter().all(|bound| bound.total == 0),
            "the fixture's ground offers no energy"
        );
        let text = page(&game, "containment");
        assert!(
            !text.contains("0/0"),
            "a capacity of nothing is drawn as a container of nothing"
        );
    }
}
