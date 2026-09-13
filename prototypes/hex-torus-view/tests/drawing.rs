//! What the page actually contains, which is a different question from what the grid says.
//!
//! **`tests/wrapping.rs` checks the grid and this checks the drawing.** They would both pass
//! with the bright and dim colours swapped, or with the seven copies placed at the wrong
//! translates, or with the ids taken from the position rather than from the cell - none of
//! which is a fact about the torus and all of which is a fact about the picture a person is
//! asked to vet.
//!
//! **Counted from the emitted markup rather than from the data that produced it.** A count
//! over `Torus::domain` would agree with itself; `X-32`'s vetted-when is about what is on the
//! screen, so this reads the string the page is made of.

use graph_coloring::color_graph;
use hex_torus_view::{draw, sizes};

/// The bright colours, which is how a hex says it is the copy rather than an echo.
fn bright_fills() -> Vec<&'static str> {
    draw::COLOURS.iter().map(|(full, _)| *full).collect()
}

/// Exactly `N` bright hexes, `7N` in all, and every id drawn exactly seven times.
#[test]
fn the_page_draws_one_bright_world_and_six_echoes() {
    let bright = bright_fills();
    let mut checked = 0;
    for torus in sizes() {
        let coloured = color_graph(&torus.adjacency());
        let svg = draw::group(&torus, &coloured.colors, None);
        let want = torus.cells();

        // **The leading space is load-bearing**: `data-fill="` ends in `fill="` too, and the
        // hover needs it. Without the space this counted 252 where 168 was right - the kind of
        // plausible number that invites no question, except that this check states its own.
        let fills: Vec<&str> = svg
            .match_indices(" fill=\"")
            .map(|(at, _)| &svg[at + 7..at + 14])
            .collect();
        let full = fills.iter().filter(|fill| bright.contains(fill)).count();
        // Every polygon carries a fill and so does every id, so the polygons are half of them.
        assert_eq!(
            fills.len(),
            2 * 7 * want,
            "k = {}: {} fills where seven copies of {want} hexes and their ids is {}",
            torus.k,
            fills.len(),
            2 * 7 * want
        );
        assert_eq!(
            full, want,
            "k = {}: {full} hexes are drawn bright and one world is {want}",
            torus.k
        );

        // **Every id appears exactly seven times** - once in the bright copy and once in each
        // echo. That is the detail `X-32` says does the work: turn the ids on and the same
        // number appears in every direction.
        let mut seen: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
        for (at, _) in svg.match_indices("class=\"id\"") {
            let text = &svg[at..];
            let from = text.find('>').expect("an id element closes its tag") + 1;
            let to = text.find("</text>").expect("an id element ends");
            *seen.entry(&text[from..to]).or_default() += 1;
        }
        assert_eq!(
            seen.len(),
            want,
            "k = {}: {} distinct ids drawn and the world has {want} territories",
            torus.k,
            seen.len()
        );
        assert!(
            seen.values().all(|times| *times == 7),
            "k = {}: an id is drawn {:?} times rather than seven - once bright and six echoes",
            torus.k,
            seen.values().collect::<Vec<_>>()
        );
        checked += 1;
    }
    assert_eq!(
        checked, 10,
        "ten sizes, and the count so one cannot become ten"
    );
}

/// The seven copies are one world and its six lattice neighbours, and no copy overlaps another.
///
/// **Overlap is the failure this picture would hide.** Two copies drawn on top of each other
/// look like one copy with a strange edge, and the bright count would still be right.
#[test]
fn the_seven_copies_do_not_overlap() {
    let mut checked = 0;
    for torus in sizes() {
        let domain = torus.domain();
        let mut placed: std::collections::BTreeSet<(i32, i32)> = std::collections::BTreeSet::new();
        let copies = draw::copies(&torus);
        assert_eq!(copies.len(), 7, "k = {}: one world and six echoes", torus.k);
        assert_eq!(
            copies.iter().filter(|(_, bright)| *bright).count(),
            1,
            "k = {}: exactly one copy is the bright one",
            torus.k
        );
        for (shift, _) in &copies {
            for (q, r) in &domain {
                assert!(
                    placed.insert((q + shift.0, r + shift.1)),
                    "k = {}: two copies both draw ({}, {})",
                    torus.k,
                    q + shift.0,
                    r + shift.1
                );
            }
        }
        assert_eq!(
            placed.len(),
            7 * torus.cells(),
            "k = {}: seven copies of {} cells",
            torus.k,
            torus.cells()
        );
        checked += 1;
    }
    assert_eq!(checked, 10, "ten sizes");
}

/// The page says how many worlds it draws, and it draws that many.
///
/// **This is `X-35`, and the defect was not in the picture.** The page drew one of twenty under
/// a header promising twenty, and the nineteen others were ten keypresses away behind a drawing
/// that reframes each size to the same screen area - so consecutive steps look alike. Sean
/// opened it, saw `folded, 12 territories`, and concluded the axis-aligned family had not
/// landed. It had. A correct page reporting one of twenty under a promise of twenty reads as
/// *nothing changed*, which is more expensive than a missing feature: it is a false report.
///
/// **So the check compares the number the page states against the number it contains.** Those
/// are two independent things in one artifact, and the failure was exactly that they disagreed.
#[test]
fn the_page_states_the_number_of_worlds_it_draws() {
    let page = hex_torus_view::page::page();
    // **Collapsed, because the header is wrapped prose and a sentence spans two lines.** A
    // match string drafted as one sentence meets a file that broke it, and matches nothing -
    // which is a green check about the wrong thing, not a red one.
    let said = page.split_whitespace().collect::<Vec<_>>().join(" ");
    let drawn = page.matches("<g class=\"size\"").count();
    assert_eq!(
        drawn,
        hex_torus_view::both_families().len(),
        "the page draws {drawn} groups and there are {} worlds",
        hex_torus_view::both_families().len()
    );

    // **Spelled, because that is how the header says it**, and a figure the header states is
    // the thing that went wrong. Change the sizes and this fails until the sentence is fixed.
    let spelled = [
        (10, "Ten"),
        (16, "Sixteen"),
        (20, "Twenty"),
        (24, "Twenty-four"),
        (30, "Thirty"),
    ];
    let word = spelled
        .iter()
        .find(|(count, _)| *count == drawn)
        .map(|(_, word)| *word)
        .unwrap_or_else(|| panic!("{drawn} worlds and this test has no word for that many"));
    let claim = format!("{word} worlds are drawn");
    assert!(
        said.contains(&claim),
        "the header does not say `{claim}`, and it draws {drawn}"
    );

    // **Per family too**, because the whole cost of `X-35` was one family being invisible.
    for (family, count) in [
        ("folded", hex_torus_view::sizes().len()),
        ("axis-aligned", hex_torus_view::axis_aligned_sizes().len()),
    ] {
        let labelled = page.matches(&format!("data-family=\"{family}\"")).count();
        assert_eq!(
            labelled, count,
            "{labelled} groups are labelled {family} and there are {count}"
        );
    }
    assert!(
        said.contains("ten folded and ten axis-aligned"),
        "the header does not name the two families and how many of each"
    );
}

/// Every world is reachable, labelled and listed - the three ways the page can name one.
///
/// **A drawing, a frame and a table row each**, and they are built separately. The frames array
/// indexes by the same number the groups do, so one shorter than the other would show a world
/// against another world's viewBox and look like a bad drawing rather than a wrong index.
#[test]
fn every_world_is_drawn_framed_and_listed() {
    let page = hex_torus_view::page::page();
    let worlds = hex_torus_view::both_families();
    assert_eq!(worlds.len(), 20, "ten of each family");

    let from = page
        .find("const frames = [")
        .expect("the page frames each world")
        + 16;
    let to = from + page[from..].find("];").expect("the frames array closes");
    let frames = page[from..to].matches('[').count();
    assert_eq!(
        frames,
        worlds.len(),
        "{frames} frames and {} worlds - a world would be drawn in another's viewBox",
        worlds.len()
    );

    let rows = page.matches("<tr><td>").count();
    assert_eq!(
        rows,
        worlds.len(),
        "{rows} rows in the table and {} worlds",
        worlds.len()
    );

    // Each group carries what the heading and the buttons read off it, so a missing attribute
    // shows as `undefined` on the page rather than as an error anywhere.
    for what in ["data-cells", "data-around", "data-family"] {
        assert_eq!(
            page.matches(what).count(),
            worlds.len(),
            "{what} is not on every one of the {} groups",
            worlds.len()
        );
    }
}

/// The twenty can be reached without knowing a key, which is what `X-35` cost.
///
/// **This reads the markup and the script text, and that is a weaker claim than it sounds.**
/// Whether a browser runs the script is not checkable here; it is what `X-32`'s vetted-when
/// asks a person to look at. What this does catch is the picker being dropped, or its handler
/// being removed while the element stays - which is the shape of the regression, because the
/// keys went on working the whole time and nothing looked broken.
#[test]
fn the_page_offers_the_twenty_without_the_keyboard() {
    let page = hex_torus_view::page::page();
    assert!(
        page.contains("<nav id=\"pick\"></nav>"),
        "the page has no element for the picker"
    );
    assert!(
        page.contains("document.getElementById('pick')"),
        "nothing fills the picker, so it renders empty"
    );
    assert!(
        page.contains("button.addEventListener('click'"),
        "the picker's entries do nothing when clicked"
    );
    assert!(
        page.contains("createElement('button')"),
        "the picker's entries are not buttons, so they do not look clickable"
    );
}

/// Every copy of a territory can be found from any one of them, which is what hover needs.
///
/// **`X-36`, and the reason it matters is not decoration.** On a torus a territory is drawn
/// many times, so two that look far apart in the bright region may be adjacent through a wrap.
/// The drawing shows the copy nearest the middle, so it does not merely fail to show distance -
/// it misleads about it. Lighting every copy at once makes the nearest one visible, and the
/// nearest one is the one that decides.
#[test]
fn every_copy_of_a_territory_carries_the_same_cell_id() {
    let mut checked = 0;
    for torus in hex_torus_view::both_families() {
        let coloured = graph_coloring::color_graph(&torus.adjacency());
        let svg = draw::group(&torus, &coloured.colors, None);
        let mut copies: std::collections::BTreeMap<usize, usize> =
            std::collections::BTreeMap::new();
        for cell in 0..torus.cells() {
            let marks = svg.matches(&format!("data-cell=\"{cell}\"")).count();
            copies.insert(cell, marks);
        }
        assert_eq!(
            copies.len(),
            torus.cells(),
            "{}: every territory is looked for",
            torus.cells()
        );
        // Seven polygons and seven ids - the hover lights both, so the number appears on every
        // copy even with the ids turned off.
        assert!(
            copies.values().all(|marks| *marks == 14),
            "{}: a territory is marked {:?} times rather than fourteen - seven hexes and \
             seven ids",
            torus.cells(),
            copies.values().collect::<Vec<_>>()
        );
        // **The resting fill and the bright fill are both on every hex**, because letting go of
        // a lit echo has to put it back to dim rather than to bright.
        assert_eq!(
            svg.matches("data-fill=\"").count(),
            7 * torus.cells(),
            "{}: not every hex carries the colour it rests at",
            torus.cells()
        );
        assert_eq!(
            svg.matches("data-full=\"").count(),
            7 * torus.cells(),
            "{}: not every hex carries the colour it lights to",
            torus.cells()
        );
        checked += 1;
    }
    assert_eq!(checked, 20, "both families, ten sizes each");
}

/// The toggle pairs by circumference, and the pairing is mutual and threefold.
///
/// **A toggle by list position would compare two unrelated worlds.** What is informative is one
/// circumference and two foldings: the same distance around, and one world with three times
/// the territories of the other. That is the folding itself, and it is the only comparison the
/// two families support.
#[test]
fn the_toggle_pairs_one_circumference_across_the_two_families() {
    let worlds = hex_torus_view::both_families();
    let paired: Vec<usize> = (0..worlds.len())
        .filter(|at| hex_torus_view::partner_of(&worlds, *at).is_some())
        .collect();
    // **Three of the ten folded worlds and three of the ten axis-aligned**, because the folded
    // ladder's circumferences are 6..=33 in threes and the axis-aligned ladder is C = 3..=12.
    // They meet at 6, 9 and 12. A run where every size paired would fail here rather than read
    // as a clean result - and so would one where none did.
    assert_eq!(
        paired.len(),
        6,
        "six of the twenty have a partner and these do: {paired:?}"
    );
    assert!(
        !paired.is_empty(),
        "a pairing over nothing is the same failure with the sign flipped"
    );

    let mut checked = 0;
    for at in paired {
        let other = hex_torus_view::partner_of(&worlds, at).expect("this one was filtered for");
        assert_ne!(
            worlds[other].family, worlds[at].family,
            "{at} is paired inside its own family, which compares nothing"
        );
        assert_eq!(
            worlds[other].circumference(),
            worlds[at].circumference(),
            "{at} and {other} are paired and do not share a circumference"
        );
        assert_eq!(
            hex_torus_view::partner_of(&worlds, other),
            Some(at),
            "{at} points at {other} and {other} does not point back"
        );
        let (bigger, smaller) = if worlds[at].cells() > worlds[other].cells() {
            (worlds[at].cells(), worlds[other].cells())
        } else {
            (worlds[other].cells(), worlds[at].cells())
        };
        assert_eq!(
            bigger,
            3 * smaller,
            "{at} and {other} share a circumference and the folding is not threefold: \
             {bigger} against {smaller}"
        );
        checked += 1;
    }
    assert_eq!(checked, 6, "three pairs, counted from both ends");
}

/// The page carries the pairing and the hover, so a person can reach both without a keyboard.
///
/// **This reads the markup and the script text, which is a weaker claim than it sounds.**
/// Whether a browser runs the script is what `X-32`'s vetted-when asks a person to look at.
/// What it catches is a control being dropped, or its handler removed while the element stays -
/// the shape of the regression, because nothing looks broken when that happens.
#[test]
fn the_page_carries_the_hover_and_the_pairing() {
    let page = hex_torus_view::page::page();
    let worlds = hex_torus_view::both_families();
    assert_eq!(
        page.matches("data-partner=\"").count(),
        worlds.len(),
        "not every world says what it pairs with"
    );
    assert_eq!(
        page.matches("data-partner=\"-1\"").count(),
        worlds.len() - 6,
        "the worlds with no partner are the fourteen that share no circumference"
    );
    for what in [
        "pointerover",
        "classList.add('lit')",
        "el.dataset.full",
        "id=\"reach\"",
    ] {
        assert!(page.contains(what), "the hover is missing `{what}`");
    }
    for what in ["dataset.partner", "id = 'fold'", "e.key === 'T'"] {
        assert!(page.contains(what), "the pairing is missing `{what}`");
    }
    // The header says what the hover is for, because a control that is not explained is one
    // nobody presses - which is the whole of `X-35`.
    let said = page.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        said.contains("every copy of that territory lights up"),
        "the header does not say what pointing at a hex does"
    );
    assert!(
        said.contains("same circumference in the other family"),
        "the header does not say what the toggle pairs"
    );
}
