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

use crate::petri::{Net, Role, by_role, kinds_never_drawn, matrix, net, what_exclusion_costs};
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
        "The release states **{} blocks of recipe rows** under **{} names**. **{} of the \
         blocks are drawn** and **{} are not**, as **{} transitions**.\n\n",
        net.recipes,
        net.names,
        net.blocks_drawn,
        net.excluded.len(),
        net.transitions.len()
    ));
    if net.excluded.is_empty() {
        out.push_str(
            "**Nothing is left out.** A Petri net arc carries a constant weight, and until \
             `P-376` the release had one row that did not: `work` produces *`$where`'s \
             density for that resource*. That rule is not a rule that measures what is \
             present - it is **one rule with a number per case**, and the specification says \
             whatever reads it may spell the cases out. So it is drawn as its cases rather \
             than counted as missing, and this page no longer has a list of what a reader \
             cannot see.\n\n",
        );
    }
    if net.transitions.len() > net.blocks_drawn {
        out.push_str(&format!(
            "**{} blocks became {} transitions**, which is that spelling out. The cases are \
             the ones *Territory resources* offers - a `(resource, density)` pair a territory \
             actually has - rather than a range or the biome table's numbers, which the \
             release says guide and do not bind. A density no territory offers would be a \
             transition for a planet that does not exist.\n\n",
            net.blocks_drawn,
            net.transitions.len()
        ));
    }
    if net.names < net.recipes {
        out.push_str(&format!(
            "**Blocks outnumber names because a rule whose subject is a family is a rule for \
             each of them** - `P-373`. `stow` and `discard` are each stated once per kind, and \
             in a net they really are separate transitions: `discard` metal takes metal and \
             `discard` labor takes labor. A repeated name is labelled with the kind it acts \
             on, so two of them are never one node on the page. {} names, {} blocks.\n\n",
            net.names, net.recipes
        ));
    }
    out.push_str(
        "**What is missing is counted rather than mentioned.** A diagram that quietly left \
         something out would be a picture of a game that is not this one, and nothing on the \
         page would say so - so the arithmetic above is printed whether or not it has anything \
         to report.\n\n",
    );

    // **What the exclusion costs, and what is missing for another reason entirely.** These
    // were one paragraph until `S-88`, and the saturating rewrite is what separated them:
    // `food` used to be absent because every recipe that moved it was excluded, and it is
    // drawn now. What is left absent is absent because no recipe names it, which would be
    // just as true of a net with nothing left out.
    let cost = what_exclusion_costs(net, document);
    if !net.excluded.is_empty() {
        out.push_str(&format!(
            "**What that costs the drawing is measured rather than assumed.** Asked against \
             the rows of the {} rather than against the list of kinds, because what a left-out \
             recipe costs is only whatever it was the sole way into: {}\n\n",
            if net.excluded.len() == 1 {
                "one recipe left out".to_string()
            } else {
                format!("{} recipes left out", net.excluded.len())
            },
            if cost.is_empty() {
                "**nothing**. Every place they touch is reached by some other recipe, so the \
                 drawing is short of those transitions and of no part of the state."
                    .to_string()
            } else {
                format!(
                    "**{}**, which nothing drawn names. A reader looking at the picture for \
                     {} would conclude the game has none.",
                    cost.iter()
                        .map(|kind| format!("`{kind}`"))
                        .collect::<Vec<_>>()
                        .join(", "),
                    cost.first().map(String::as_str).unwrap_or_default()
                )
            }
        ));
    }

    // A page reporting places with no arcs would always report nothing, because the net only
    // makes a place when an arc needs one. What is worth saying is which *kinds* the release
    // declares that the drawn net never mentions at all - and, since `S-88`, why.
    let never = kinds_never_drawn(net, document);
    if !never.is_empty() {
        out.push_str(&format!(
            "**{} the release declares {} nowhere in the drawn net**: {}. **This is not the \
             exclusion's doing** - no recipe names any of them, so they would be missing from \
             a drawing with nothing left out. They are where things are and how places relate, \
             rather than things a recipe moves.\n\n",
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
         the release had two, both `limit 0 garrison`. **`P-385` deleted both rows.** Sean, \
         2026-09-11: *repeated deployments are player choice, safe because they are not \
         capable of causing an infinite resource glitch* - so a second landing on a colony \
         you already hold is allowed, and nothing in the rules refuses it. The `limit` role \
         is still one of the four the release names and no row carries it.\n\n",
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
         drawn on its own on the page, and written out here.\n\n",
    );
    // **Written out as well as drawn, which it was not until the unfolding.** These headings
    // carried nothing in the markdown: the page splices a drawing under each, and the diffable
    // sibling was left with a run of empty sections. That was tolerable at twenty-four and is
    // not at thirty-nine - and `github.com` renders the markdown, which is where Sean said he
    // wanted to read this. A heading with nothing under it is also the one shape a reader
    // cannot tell from a transition that touches nothing.
    for (at, name) in net.transitions.iter().enumerate() {
        out.push_str(&format!("### {name}\n\n"));
        let arcs = net.arcs_of(at);
        for arc in &arcs {
            let place = net.places[arc.place].label();
            let said = match arc.role {
                Role::Consume => format!("takes {} {place}", arc.weight),
                Role::Produce => format!("makes {} {place}", arc.weight),
                Role::Require => format!("needs {} {place}, and does not take it", arc.weight),
                Role::Limit if arc.weight == 0 => format!("fires only where there is no {place}"),
                Role::Limit => format!("fires only at {} {place} or fewer", arc.weight),
            };
            let traits = arc.traits.trim();
            if traits.is_empty() {
                out.push_str(&format!("- {said}\n"));
            } else {
                out.push_str(&format!("- {said} - {traits}\n"));
            }
        }
        // **Said rather than left blank.** A transition with no arcs would otherwise look
        // exactly like one whose arcs the renderer dropped.
        if arcs.is_empty() {
            out.push_str("- touches no place at all, which no recipe in this release does\n");
        }
        out.push('\n');
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
