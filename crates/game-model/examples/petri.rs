//! The rules as a net, drawn in parts a reader can actually read.
//!
//! `cargo run --example petri`, and `tests/petri.rs` is what holds it.
//!
//! **`R-10`, and the clauses are its own words**: *every generated drawing is legible in **both** a
//! light and a dark reader, because nothing in it declares a colour the theme does not supply.
//! **Every node carries its own name**, and I can say what a node is without looking anything up.
//! Where a drawing is too large to satisfy that whole, it is shown in parts that do, and it says
//! what each part leaves out.*
//!
//! # The net is the one the decision is made on
//!
//! **This draws what `nogain.rs` reads**, rather than reading the rules a second way. A drawing
//! that came from its own reader could disagree with the verdict beside it and neither would be
//! wrong about itself - which is the failure the repository keeps finding under a different name:
//! **the instrument answers a narrower question than the one asked.** Here there is one reader,
//! and the picture is of the thing that was decided.
//!
//! **What it draws is the arcs, not the fold.** The no-gain decision needs `made - taken`, and a
//! place a rule takes and remakes in equal measure disappears from that sum - `move` takes a
//! scout and makes a scout, and only `moving` survives it. **A drawing that showed the sum would
//! show `move` as not touching a scout at all**, so `Ground` carries both ends.
//!
//! # Why there is no drawing of the whole net
//!
//! **The clause allows parts and this takes it.** Thirty places and thirty-two transitions joined
//! by every arc is a picture whose nodes cannot each carry a readable name, so a whole-net drawing
//! would fail the second clause in the course of satisfying nothing. **What replaces it is a part
//! per rule that says what it leaves out** - and says it by naming the other rules that reach the
//! same places, computed from the arcs, rather than by saying *everything else*, which is true and
//! tells a reader nothing.

use std::collections::{BTreeMap, BTreeSet};

/// The reader, the net and the solver.
#[path = "nogain.rs"]
#[allow(dead_code)]
pub mod nogain;

use nogain::{Ground, Place, Read};

fn root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn escaped(said: &str) -> String {
    said.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// `**bold**` and `` `code` `` into spans, so one string serves both forms.
fn marked(said: &str) -> String {
    let mut out = escaped(said);
    for (mark, open, close) in [("**", "<strong>", "</strong>"), ("`", "<code>", "</code>")] {
        let mut made = String::new();
        let mut rest = out.as_str();
        let mut opening = true;
        while let Some(at) = rest.find(mark) {
            made.push_str(&rest[..at]);
            made.push_str(if opening { open } else { close });
            opening = !opening;
            rest = &rest[at + mark.len()..];
        }
        made.push_str(rest);
        out = if opening { made } else { out };
    }
    out
}

// -- What each part leaves out ------------------------------------------------------------------

/// Every place a rule touches, either end.
fn touches(one: &Ground) -> BTreeSet<Place> {
    one.took.keys().chain(one.made.keys()).cloned().collect()
}

/// The other rules that reach the same places, and which places those are.
///
/// **Computed from the arcs**, so a rule added tomorrow appears in the parts it touches with
/// nobody maintaining a list. **Named rather than counted**: `R-10` records that *everything
/// else* is true and tells a reader nothing.
pub fn leaves_out(read: &Read, at: usize) -> (Vec<String>, Vec<Place>) {
    let mine = touches(&read.ground[at]);
    let mut rules = Vec::new();
    let mut shared: BTreeSet<Place> = BTreeSet::new();
    for (other, one) in read.ground.iter().enumerate() {
        if other == at {
            continue;
        }
        let also: BTreeSet<Place> = touches(one).intersection(&mine).cloned().collect();
        if also.is_empty() {
            continue;
        }
        rules.push(one.rule.name.clone());
        shared.extend(also);
    }
    rules.sort();
    rules.dedup();
    (rules, shared.into_iter().collect())
}

// -- The drawing ---------------------------------------------------------------------------------

/// How wide a label is, near enough to lay out against.
fn wide(said: &str) -> f64 {
    said.chars().count() as f64 * 7.2
}

const ROW: f64 = 44.0;
const PAD: f64 = 16.0;
const DOT: f64 = 11.0;

/// One part of the net: what a single rule takes, does and makes.
///
/// **Every colour here is a custom property the stylesheet defines for both themes**, which is
/// `R-10`'s first clause said in the only way a drawing can satisfy it. There is no literal
/// colour anywhere below, and `tests/petri.rs` refuses one.
fn drawn(one: &Ground) -> String {
    let took: Vec<(&Place, &i64)> = one.took.iter().collect();
    let made: Vec<(&Place, &i64)> = one.made.iter().collect();
    let rows = took.len().max(made.len()) as f64;

    let left = took
        .iter()
        .map(|(place, _)| wide(&place.label()))
        .fold(60.0_f64, f64::max);
    let right = made
        .iter()
        .map(|(place, _)| wide(&place.label()))
        .fold(60.0_f64, f64::max);
    let bar = wide(&one.rule.name).max(90.0) + 24.0;

    // Columns: label | dot | arc | transition | arc | dot | label
    let dot_in = PAD + left + 10.0 + DOT;
    let bar_at = dot_in + DOT + 46.0;
    let dot_out = bar_at + bar + 46.0 + DOT;
    let width = dot_out + DOT + 10.0 + right + PAD;
    let height = rows.max(1.0) * ROW + PAD * 2.0;
    let middle = height / 2.0;

    let mut svg = format!(
        "<svg class=\"net\" viewBox=\"0 0 {width:.0} {height:.0}\" width=\"{width:.0}\" \
         height=\"{height:.0}\" role=\"img\" aria-label=\"{}\" \
         xmlns=\"http://www.w3.org/2000/svg\">\n",
        escaped(&format!("The rule {} as a net", one.rule.name))
    );

    // The transition: a rectangle carrying the rule's own name.
    let bar_top = PAD / 2.0;
    let bar_high = height - PAD;
    svg.push_str(&format!(
        "<rect x=\"{bar_at:.0}\" y=\"{bar_top:.0}\" width=\"{bar:.0}\" height=\"{bar_high:.0}\" \
         rx=\"3\" fill=\"var(--paper)\" stroke=\"var(--ink)\" stroke-width=\"2\"/>\n\
         <text x=\"{:.0}\" y=\"{:.0}\" text-anchor=\"middle\" fill=\"var(--ink)\" \
         font-size=\"13\" font-weight=\"600\">{}</text>\n",
        bar_at + bar / 2.0,
        middle + 4.0,
        escaped(&one.rule.name)
    ));

    let place_at = |count: usize, at: usize| -> f64 {
        let span = count as f64 * ROW;
        middle - span / 2.0 + ROW / 2.0 + at as f64 * ROW
    };

    for (at, (place, many)) in took.iter().enumerate() {
        let y = place_at(took.len(), at);
        svg.push_str(&node(dot_in, y, &place.label(), true));
        svg.push_str(&arc(dot_in + DOT, y, bar_at, middle, **many));
    }
    for (at, (place, many)) in made.iter().enumerate() {
        let y = place_at(made.len(), at);
        svg.push_str(&node(dot_out, y, &place.label(), false));
        svg.push_str(&arc(bar_at + bar, middle, dot_out - DOT, y, **many));
    }
    svg.push_str("</svg>\n");
    svg
}

/// A place: a circle, and **its own name beside it** - `R-10`'s second clause.
///
/// **Beside rather than inside.** `extractor, working 1` does not fit in a circle at any size a
/// page can hold, and a circle labelled `e1` with a key underneath is exactly the *looking
/// something up* the clause refuses.
fn node(x: f64, y: f64, label: &str, taken: bool) -> String {
    let (text_x, anchor) = if taken {
        (x - DOT - 10.0, "end")
    } else {
        (x + DOT + 10.0, "start")
    };
    format!(
        "<circle cx=\"{x:.0}\" cy=\"{y:.0}\" r=\"{DOT:.0}\" fill=\"var(--paper)\" \
         stroke=\"var(--ink)\" stroke-width=\"2\"/>\n\
         <text x=\"{text_x:.0}\" y=\"{:.0}\" text-anchor=\"{anchor}\" fill=\"var(--ink)\" \
         font-size=\"13\">{}</text>\n",
        y + 4.0,
        escaped(label)
    )
}

/// An arc, with its weight where it is more than one.
fn arc(x1: f64, y1: f64, x2: f64, y2: f64, many: i64) -> String {
    let head = 7.0;
    let run = x2 - x1;
    let rise = y2 - y1;
    let length = (run * run + rise * rise).sqrt().max(1.0);
    let (ux, uy) = (run / length, rise / length);
    let (tip_x, tip_y) = (x2, y2);
    let (base_x, base_y) = (x2 - ux * head, y2 - uy * head);
    let mut out = format!(
        "<line x1=\"{x1:.0}\" y1=\"{y1:.0}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"var(--ink)\" \
         stroke-width=\"1.5\"/>\n\
         <polygon points=\"{tip_x:.1},{tip_y:.1} {:.1},{:.1} {:.1},{:.1}\" \
         fill=\"var(--ink)\"/>\n",
        base_x,
        base_y,
        base_x - uy * head * 0.5,
        base_y + ux * head * 0.5,
        base_x + uy * head * 0.5,
        base_y - ux * head * 0.5,
    );
    if many > 1 {
        out.push_str(&format!(
            "<text x=\"{:.0}\" y=\"{:.0}\" text-anchor=\"middle\" fill=\"var(--quiet)\" \
             font-size=\"12\">{many}</text>\n",
            (x1 + x2) / 2.0,
            (y1 + y2) / 2.0 - 6.0
        ));
    }
    out
}

// -- The page --------------------------------------------------------------------------------

/// What a part says in the diffable sibling, where there is no picture.
fn as_text(one: &Ground, out_of: &(Vec<String>, Vec<Place>)) -> String {
    let side = |which: &BTreeMap<Place, i64>| -> String {
        if which.is_empty() {
            return "\u{2014}".to_string();
        }
        which
            .iter()
            .map(|(place, many)| {
                if *many == 1 {
                    place.label()
                } else {
                    format!("{many} \u{d7} {}", place.label())
                }
            })
            .collect::<Vec<_>>()
            .join(" \u{b7} ")
    };
    let (rules, shared) = out_of;
    let mut said = format!(
        "- takes: {}\n- makes: {}\n",
        side(&one.took),
        side(&one.made)
    );
    if rules.is_empty() {
        said.push_str("- leaves out: nothing - no other rule reaches any place this one does\n");
    } else {
        said.push_str(&format!(
            "- shares: {}\n- leaves out: the {} other rule(s) that reach them - {}\n",
            shared
                .iter()
                .map(|it| format!("`{}`", it.label()))
                .collect::<Vec<_>>()
                .join(" \u{b7} "),
            rules.len(),
            rules
                .iter()
                .map(|it| format!("`{it}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    said
}

/// Which rules fill a place and which empty it, over the whole net.
///
/// **This is the question a drawing in parts cannot answer by being looked at**, and the reason
/// the parts are not the whole of the page. A place nothing fills can only fall; a place nothing
/// empties can only rise, **which is the shape of an unbounded accumulation** - not a defect on
/// its own, because a structure that is built and never destroyed is a choice, but the thing to
/// be looking at when one is suspected.
fn ends(read: &Read) -> BTreeMap<Place, (Vec<String>, Vec<String>)> {
    let mut out: BTreeMap<Place, (Vec<String>, Vec<String>)> = BTreeMap::new();
    for one in &read.ground {
        for place in one.made.keys() {
            out.entry(place.clone())
                .or_default()
                .0
                .push(one.rule.name.clone());
        }
        for place in one.took.keys() {
            out.entry(place.clone())
                .or_default()
                .1
                .push(one.rule.name.clone());
        }
    }
    out
}

/// One bullet naming a place and the rules at the other end.
fn one_ended(place: &Place, rules: &[String], said: &str) -> String {
    let named = if rules.is_empty() {
        "nothing at all".to_string()
    } else {
        rules
            .iter()
            .map(|it| format!("`{it}`"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let source = if nogain::SOURCES.contains(&place.kind.as_str()) {
        " - **a named source**, so this is what it is meant to be"
    } else {
        ""
    };
    format!("- `{}` - {said} {named}{source}\n", place.label())
}

/// Write `reports/petri.html` and `reports/petri.md`, and say how many moved.
pub fn write_report() -> (usize, usize) {
    let read = nogain::read();
    let places: BTreeSet<Place> = read.ground.iter().flat_map(touches).collect();

    let title = "The rules as a net";
    let note = "Generated from `spec/data/rules.4x` by `scripts/reports.sh`. **Not canonical** - \
                the rules are, and this is a drawing of them.";
    let opening = format!(
        "**{} places and {} transitions**, drawn one rule at a time. A circle is a place - a kind \
         of thing in a state - and a rectangle is a rule. An arc into the rectangle is what the \
         rule takes; an arc out of it is what it makes, and a number on an arc is how many. \
         **There is no drawing of the whole net**, because one that held every arc could not give \
         every node a readable name - so each part says what it leaves out, by naming the other \
         rules that reach the same places.",
        places.len(),
        read.ground.len()
    );

    let mut html = format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <title>{}</title>\n<link rel=\"stylesheet\" href=\"report.css\">\n</head>\n<body>\n\
         <h1>{}</h1>\n<p class=\"note\">{}</p>\n\
         <p class=\"where\"><a href=\"index.html\">The index</a> \u{b7} \
         <a href=\"petri.md\">this page as markdown</a> \u{b7} \
         <a href=\"nogain.html\">whether it can come back round with more</a></p>\n\
         <p>{}</p>\n",
        escaped(title),
        escaped(title),
        marked(note),
        marked(&opening)
    );
    let mut markdown = format!(
        "# {title}\n\n{note}\n\n[The index](index.md) \u{b7} \
         [whether it can come back round with more](nogain.md)\n\n{opening}\n"
    );

    // **What the parts cannot show**, because it is a fact about every part at once.
    let at_the_ends = ends(&read);
    for (heading, said, want_filled) in [
        (
            "Places nothing fills",
            "These can only fall. **The planet and time are meant to be here** - they are the \
             endless wells the invariant names, and a well nothing refills is what an endless \
             well looks like in a net. **Anything else here is a kind the game can spend and \
             cannot make.**",
            true,
        ),
        (
            "Places nothing empties",
            "These can only rise. **That is the shape of an unbounded accumulation**, which is \
             not a defect on its own - a structure that is built and never taken down is a \
             choice - but it is where to look when one is suspected.",
            false,
        ),
    ] {
        let mut bullets = Vec::new();
        for (place, (fills, empties)) in &at_the_ends {
            let (mine, other) = if want_filled {
                (fills, empties)
            } else {
                (empties, fills)
            };
            if !mine.is_empty() {
                continue;
            }
            bullets.push(one_ended(
                place,
                other,
                if want_filled {
                    "nothing fills it; emptied by"
                } else {
                    "nothing empties it; filled by"
                },
            ));
        }
        html.push_str(&format!(
            "<h2>{} ({})</h2>\n<p>{}</p>\n<ul>\n",
            escaped(heading),
            bullets.len(),
            marked(said)
        ));
        for bullet in &bullets {
            html.push_str(&format!(
                "<li>{}</li>\n",
                marked(bullet.trim_start_matches("- ").trim_end())
            ));
        }
        html.push_str("</ul>\n");
        markdown.push_str(&format!(
            "\n## {heading} ({})\n\n{said}\n\n{}",
            bullets.len(),
            bullets.concat()
        ));
    }

    for (at, one) in read.ground.iter().enumerate() {
        let out_of = leaves_out(&read, at);
        html.push_str(&format!(
            "<h2>{}</h2>\n{}",
            escaped(&one.rule.name),
            drawn(one)
        ));
        // **Two lists with a word between them rather than a colon.** Read together they are
        // unparseable, because a place's own name has a comma in it - `scout, moving 0 ·
        // scout, moving 1: refresh (scout, moving)` is three things and looks like five.
        if out_of.0.is_empty() {
            html.push_str(
                "<p class=\"generated\"><strong>Leaves out nothing</strong> - no other rule \
                 reaches any place this one does.</p>\n",
            );
        } else {
            html.push_str(&format!(
                "<p class=\"generated\"><strong>Shares</strong> {}<br>\
                 <strong>Leaves out</strong> the {} other rule{} that reach{} them: {}</p>\n",
                out_of
                    .1
                    .iter()
                    .map(|it| format!("<code>{}</code>", escaped(&it.label())))
                    .collect::<Vec<_>>()
                    .join(" \u{b7} "),
                out_of.0.len(),
                if out_of.0.len() == 1 { "" } else { "s" },
                if out_of.0.len() == 1 { "es" } else { "" },
                out_of
                    .0
                    .iter()
                    .map(|it| format!("<code>{}</code>", escaped(it)))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        markdown.push_str(&format!(
            "\n## {}\n\n{}",
            one.rule.name,
            as_text(one, &out_of)
        ));
    }

    html.push_str("</body>\n</html>\n");

    let out = root().join("reports");
    std::fs::create_dir_all(&out).expect("reports/");
    let mut written = 0;
    for (name, text) in [("petri.html", html), ("petri.md", markdown)] {
        let at = out.join(name);
        if std::fs::read_to_string(&at).unwrap_or_default() != text {
            std::fs::write(&at, &text).unwrap_or_else(|why| panic!("{name}: {why}"));
            written += 1;
        }
    }
    (read.ground.len(), written)
}

fn main() {
    let (parts, written) = write_report();
    println!("reports/petri: {parts} part(s); {written} page file(s) rewritten");
}
