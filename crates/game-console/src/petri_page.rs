//! The two files `S-87` asks for: the net as a diff, and the net as a page.
//!
//! **One source, two renderings.** The markdown is written here and the page is that markdown
//! put through [`crate::dump::page`] with the drawings spliced in, which is how `recipes.md`
//! becomes `recipes.html` and means the two cannot say different things.
//!
//! **The markdown is not a lesser copy of the picture**, which is why it carries the whole
//! argument rather than a caption. `R-9` requires a diffable sibling for every view, and here
//! the sibling is the better artifact for two of the three jobs: the incidence matrix is what
//! the checks operate on, a changed weight is one changed cell where in the SVG it moves every
//! coordinate after it, and github.com renders it with no deploy at all.

use crate::petri::{Net, by_role, kinds_never_drawn, matrix, net};
use crate::petri_draw::{excluded_table, places_table, recipe_svg, svg};

/// A markdown table, already padded.
///
/// **Padded here rather than left to the hook.** `hooks/pre-commit` runs `tools/pad-tables`
/// over staged markdown, and `tests/dumps_are_current.rs` compares the committed file with
/// what this generates - so a table written unpadded is padded on the way in and stale the
/// moment it lands. `CLAUDE.md` states the rule directly: padding a generated file changes
/// nothing.
fn table(rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    let mut widths = vec![0usize; columns];
    for row in rows {
        for (at, cell) in row.iter().enumerate() {
            widths[at] = widths[at].max(cell.chars().count());
        }
    }

    let line = |row: &[String]| {
        let mut out = String::from("|");
        for (at, width) in widths.iter().enumerate() {
            let cell = row.get(at).map(String::as_str).unwrap_or("");
            out.push_str(&format!(
                " {}{} |",
                cell,
                " ".repeat(width - cell.chars().count())
            ));
        }
        out.push('\n');
        out
    };

    let mut out = line(&rows[0]);
    out.push('|');
    for width in &widths {
        out.push_str(&format!(" {} |", "-".repeat(*width)));
    }
    out.push('\n');
    for row in &rows[1..] {
        out.push_str(&line(row));
    }
    out
}

/// How the page opens - the counts `S-87` requires before anything is drawn.
///
/// **The accounting comes first deliberately.** A reader who sees the picture first has
/// already formed a view of what the game is, and a note underneath saying *four recipes are
/// missing* arrives too late to change it.
fn accounting(net: &Net, document: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "The release declares **{} recipes**. **{} are drawn** and **{} are not**, because a \
         Petri net arc carries a constant weight and those recipes have amounts that depend on \
         the state when they fire.\n\n",
        net.recipes,
        net.transitions.len(),
        net.excluded.len()
    ));
    out.push_str(
        "**That is the whole of what is missing, and it is counted rather than mentioned.** A \
         diagram that quietly left them out would be a picture of a game that is not this one, \
         and nothing on the page would say so.\n\n",
    );

    // **What the exclusions cost, which is more than four rows.** A page reporting places
    // with no arcs would always report nothing, because the net only makes a place when an
    // arc needs one. What is worth saying is which *kinds* the release declares that the
    // drawn net never mentions at all.
    let never = kinds_never_drawn(net, document);
    if !never.is_empty() {
        out.push_str(&format!(
            "**It costs more than four rows.** {} the release declares {} nowhere in the \
             drawn net: {}. **They are not absent from the game - they are absent from what \
             can be drawn of it**, and `food` is the one that matters: every recipe that \
             moves it is one of the four above, so a reader looking at the picture for food \
             would conclude the game has none.\n\n",
            if never.len() == 1 {
                "One kind".to_string()
            } else {
                format!("{} kinds", never.len())
            },
            if never.len() == 1 {
                "appears"
            } else {
                "appear"
            },
            never
                .iter()
                .map(|kind| format!("`{kind}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    let roles = by_role(net);
    out.push_str(&format!(
        "**{} places and {} arcs** between them: {}.\n\n",
        net.places.len(),
        net.arcs.len(),
        roles
            .iter()
            .map(|(role, count)| format!("{count} `{role}`"))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    let rooms = net.places.iter().filter(|place| place.room).count();
    out.push_str(&format!(
        "**{rooms} of those places are room rather than a count** - `P-374`. What a container \
         stores is the room left, not the total: used capacity is what is there, total \
         capacity is the two added, and nothing records the total so nothing can disagree \
         with it. Making a thing takes one of the room and destroying it gives one back. Room \
         is stored, so room is state, so it is drawn - a diagram showing the count and hiding \
         the room would be leaving out half of what containment is.\n\n"
    ));
    out.push_str(&format!(
        "**{} of these arcs are zero tests, and that number used to be two.** Reachability in \
         a plain Petri net is decidable and an inhibitor arc makes the net Turing-complete; \
         the release had two, both `limit 0 garrison`. `P-374` removed them without meaning \
         to: *there is no garrison here* is *the garrison's room is untouched*, which is an \
         ordinary requirement on an ordinary place. **That translation is exact only because \
         a garrison's capacity is one** - at two, *there is none* and *there is room for one* \
         are different claims - and the reader is refused rather than approximated if a \
         `limit 0` ever appears somewhere with more room than that.\n\n",
        net.inhibitors().len()
    ));
    out
}

/// The whole document, which the page is a rendering of.
pub fn markdown(document: &str) -> String {
    let net = net(document);
    let mut out = String::from(
        "# The rules as a Petri net\n\n\
         **Generated. Do not edit.** Read out of the release's *Recipes* table by \
         `crates/game-console/src/petri.rs`.\n\n",
    );
    out.push_str(
        "`S-87`, and the formalism is the research lens's: \
         `lenses/research/2026-09-08-simple-finite-and-decidable.md` establishes that a recipe \
         network **is** a Petri net rather than resembling one. The release's four roles are \
         the four arc kinds and nothing had to be invented to get from one to the other.\n\n",
    );
    out.push_str(&accounting(&net, document));

    out.push_str(
        "A **place** is a circle - somewhere a kind can be, which is a container and a kind \
         together, because energy in a tank is not energy in a territory and the first of \
         those is bounded while the second has no limit. A place named *room for* something \
         holds the room left in its container rather than the things themselves. A \
         **transition** is a bar: one recipe. An arc into a bar is `consume`, or `require` \
         when it is dotted and the thing is not taken; an arc out of a bar is `produce`.\n\n",
    );

    out.push_str("## The whole net\n\n");
    out.push_str(
        "The drawing is on the page beside this file; what follows is the same net in the form \
         a diff can show.\n\n",
    );

    out.push_str("## What is not drawn\n\n");
    out.push_str(&table(&excluded_table(&net)));
    out.push_str(
        "\n**Computed from the release, not listed.** `S-87` named five, taken from the \
         research lens's own re-encoding rather than from `releases/first-release.md`, and two \
         of that five - `refuel` and `end-of-turn losses` - are not recipes in the release at \
         all. Deriving the set from the table is what keeps this page from inheriting that.\n\n",
    );

    out.push_str("## Places\n\n");
    out.push_str(&table(&places_table(&net)));

    out.push_str("\n## The incidence matrix\n\n");
    out.push_str(
        "Places down, transitions across. `-n` is taken, `+n` is made, `rn` is required and not \
         taken, `0!` is the zero test. **This is what the checks operate on.**\n\n",
    );
    out.push_str(&table(&matrix(&net)));

    out.push_str("\n## One recipe at a time\n\n");
    out.push_str(
        "The net above answers how the rules connect and cannot answer what one of them does - \
         at this many arcs the eye cannot follow a single transition out of the bundle. Each is \
         drawn on its own on the page.\n\n",
    );
    for name in &net.transitions {
        out.push_str(&format!("### {name}\n\n"));
    }
    out
}

/// The page: the markdown above, with the drawings put where its headings promise them.
///
/// **Spliced rather than written separately.** Two generators for one document is two things
/// that can disagree, and the disagreement would be invisible - both would render.
pub fn page(document: &str) -> String {
    let net = net(document);
    let mut out = crate::dump::page(&markdown(document), "petri.md");

    out = splice(&out, &heading(2, "The whole net"), &svg(&net));
    for (at, name) in net.transitions.iter().enumerate() {
        out = splice(&out, &heading(3, name), &recipe_svg(&net, at));
    }
    out
}

/// The heading `dump::page` writes for this text, which is what a splice has to match.
///
/// **Built by the same function that builds it**, rather than typed. `dump::page` gives every
/// heading an id from `browse::slug`, and a literal `<h2>The whole net</h2>` matches nothing -
/// which is how the first version of this failed, loudly, as it was written to.
fn heading(level: usize, text: &str) -> String {
    format!(
        "<h{level} id=\"{}\">{text}</h{level}>",
        crate::browse::slug(text)
    )
}

/// Puts a drawing immediately after the heading that announces it.
fn splice(page: &str, after: &str, drawing: &str) -> String {
    let Some(at) = page.find(after) else {
        // **A panic rather than a silent miss.** A heading that stopped matching would leave
        // the page rendering perfectly with one picture gone, which is the failure this whole
        // report exists to make impossible for recipes.
        panic!("the page has no `{after}` to put a drawing after");
    };
    let at = at + after.len();
    format!("{}\n{drawing}{}", &page[..at], &page[at..])
}
