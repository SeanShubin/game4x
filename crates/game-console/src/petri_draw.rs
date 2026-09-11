//! The net as a picture and as a page - `S-87`.
//!
//! **No script, because `R-9` forbids one and `tests/browsable.rs` enforces it.** That rules
//! out every client-side diagram renderer, so the SVG is generated here, inline, and arrives
//! drawn. It also means the page works on github.com, which is what Sean asked for: *browse it
//! from github after a deploy*.
//!
//! **The layout is integer arithmetic with no iteration.** A generated file has to come out
//! byte-identical every time or `tests/dumps_are_current.rs` fails, and a force-directed layout
//! is a floating-point loop whose last digit is a promise no library makes across platforms.
//! Two columns with a crossing-reduction pass over integer positions has no such problem.

use crate::petri::{Arc, Net, Place, Role};

/// Where a node sits, in grid units before they become pixels.
struct Placed {
    row: usize,
}

/// Two columns - places on the left, transitions on the right - ordered to cross less.
///
/// **The barycentre heuristic, four sweeps, ties broken by index.** It is the standard
/// two-layer ordering and it is deterministic, which is the property that matters here: the
/// same net gives the same file, so a diff shows a rule changing rather than a layout
/// settling somewhere else.
fn order(net: &Net) -> (Vec<Placed>, Vec<Placed>) {
    let mut places: Vec<usize> = (0..net.places.len()).collect();
    let mut transitions: Vec<usize> = (0..net.transitions.len()).collect();

    let barycentre = |of: usize, other: &[usize], place_side: bool| -> (usize, usize) {
        let mut total = 0;
        let mut count = 0;
        for arc in &net.arcs {
            let (mine, theirs) = if place_side {
                (arc.place, arc.transition)
            } else {
                (arc.transition, arc.place)
            };
            if mine != of {
                continue;
            }
            if let Some(at) = other.iter().position(|it| *it == theirs) {
                total += at;
                count += 1;
            }
        }
        // A node with no arcs keeps its index, which is why the tie-break is the index and
        // not something derived. `checked_div` rather than a guard: the guard and the
        // division were two statements saying one thing, which is what clippy objected to
        // and what would let them disagree.
        match (total * 100).checked_div(count) {
            Some(mean) => (mean, of),
            None => (usize::MAX, of),
        }
    };

    for _ in 0..4 {
        let mut keyed: Vec<(usize, usize)> = places
            .iter()
            .map(|at| barycentre(*at, &transitions, true))
            .collect();
        keyed.sort();
        places = keyed.into_iter().map(|(_, at)| at).collect();

        let mut keyed: Vec<(usize, usize)> = transitions
            .iter()
            .map(|at| barycentre(*at, &places, false))
            .collect();
        keyed.sort();
        transitions = keyed.into_iter().map(|(_, at)| at).collect();
    }

    let mut place_rows: Vec<Placed> = (0..net.places.len()).map(|_| Placed { row: 0 }).collect();
    for (row, at) in places.iter().enumerate() {
        place_rows[*at] = Placed { row };
    }
    let mut transition_rows: Vec<Placed> = (0..net.transitions.len())
        .map(|_| Placed { row: 0 })
        .collect();
    for (row, at) in transitions.iter().enumerate() {
        transition_rows[*at] = Placed { row };
    }
    (place_rows, transition_rows)
}

const ROW: usize = 34;
const TOP: usize = 40;
const LEFT: usize = 200;
const RIGHT: usize = 660;
const WIDTH: usize = 900;

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The whole net, drawn.
///
/// A place is a circle and a transition is a bar, which is the notation every text on Petri
/// nets uses; the point of keeping it is that a reader who knows the formalism recognises the
/// picture, and one who does not has the legend.
pub fn svg(net: &Net) -> String {
    let (places, transitions) = order(net);
    let rows = net.places.len().max(net.transitions.len());
    let height = TOP * 2 + rows * ROW;

    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {WIDTH} {height}\" \
         width=\"{WIDTH}\" height=\"{height}\" role=\"img\" \
         aria-label=\"the release's rules as a Petri net\">\n"
    ));
    out.push_str(
        "<defs>\
         <marker id=\"a\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"6\" \
         markerHeight=\"6\" orient=\"auto-start-reverse\">\
         <path d=\"M 0 0 L 10 5 L 0 10 z\" fill=\"currentColor\"/></marker>\
         <marker id=\"o\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"7\" \
         markerHeight=\"7\" orient=\"auto-start-reverse\">\
         <circle cx=\"5\" cy=\"5\" r=\"4\" fill=\"none\" stroke=\"#e0a33f\" \
         stroke-width=\"2\"/></marker>\
         </defs>\n",
    );

    // Arcs first, so the nodes sit on top of them.
    for arc in &net.arcs {
        let from = TOP + places[arc.place].row * ROW;
        let to = TOP + transitions[arc.transition].row * ROW;
        let (x1, y1, x2, y2) = if arc.role.into_transition() {
            (LEFT, from, RIGHT, to)
        } else {
            (RIGHT, to, LEFT, from)
        };
        let (stroke, marker, dash) = match arc.role {
            // **The zero test is drawn differently because it is different.** A plain Petri
            // net has decidable reachability; an inhibitor arc makes it Turing-complete, and
            // this release has two of them. A reader should be able to find them without
            // counting.
            Role::Limit if arc.weight == 0 => ("#e0a33f", "o", " stroke-dasharray=\"5 4\""),
            Role::Require => ("currentColor", "a", " stroke-dasharray=\"2 3\""),
            _ => ("currentColor", "a", ""),
        };
        let middle = (x1 + x2) / 2;
        out.push_str(&format!(
            "<path d=\"M {x1} {y1} C {middle} {y1}, {middle} {y2}, {x2} {y2}\" fill=\"none\" \
             stroke=\"{stroke}\" stroke-width=\"1.2\" stroke-opacity=\"0.75\"{dash} \
             marker-end=\"url(#{marker})\"/>\n"
        ));
    }

    for (at, place) in net.places.iter().enumerate() {
        let y = TOP + places[at].row * ROW;
        out.push_str(&format!(
            "<circle cx=\"{LEFT}\" cy=\"{y}\" r=\"7\" fill=\"none\" stroke=\"currentColor\" \
             stroke-width=\"1.5\"/>\n<text x=\"{}\" y=\"{}\" text-anchor=\"end\" \
             fill=\"currentColor\" font-family=\"system-ui, sans-serif\" font-size=\"13\">{}</text>\n",
            LEFT - 14,
            y + 4,
            escape(&place.label())
        ));
    }

    for (at, name) in net.transitions.iter().enumerate() {
        let y = TOP + transitions[at].row * ROW;
        out.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"6\" height=\"20\" fill=\"currentColor\"/>\n\
             <text x=\"{}\" y=\"{}\" fill=\"currentColor\" font-family=\"system-ui, sans-serif\" font-size=\"13\">{}</text>\n",
            RIGHT - 3,
            y - 10,
            RIGHT + 14,
            y + 4,
            escape(name)
        ));
    }

    out.push_str("</svg>\n");
    out
}

/// One recipe on its own: its places, its arcs, and nothing else.
///
/// **Sean asked for the global net and said this was optional.** It earns its place because
/// the global net answers *how do the rules connect* and cannot answer *what does this one
/// do* - at 49 arcs the eye cannot follow one transition out of the bundle.
pub fn recipe_svg(net: &Net, transition: usize) -> String {
    let arcs: Vec<&Arc> = net.arcs_of(transition);
    let height = TOP + arcs.len().max(1) * 28 + 16;
    let bar = 300;

    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 820 {height}\" width=\"820\" \
         height=\"{height}\" role=\"img\" aria-label=\"{} as a Petri net\">\n",
        escape(&net.transitions[transition])
    ));

    let middle = height / 2;
    out.push_str(&format!(
        "<rect x=\"{}\" y=\"{}\" width=\"7\" height=\"26\" fill=\"currentColor\"/>\n\
         <text x=\"{bar}\" y=\"{}\" text-anchor=\"middle\" fill=\"currentColor\" font-family=\"system-ui, sans-serif\" \
         font-size=\"13\">{}</text>\n",
        bar - 3,
        middle - 13,
        middle + 30,
        escape(&net.transitions[transition])
    ));

    for (at, arc) in arcs.iter().enumerate() {
        let y = 28 + at * 28;
        let inbound = arc.role.into_transition();
        let (x1, x2) = if inbound {
            (150, bar - 6)
        } else {
            (bar + 6, 470)
        };
        let (stroke, marker, dash) = match arc.role {
            Role::Limit if arc.weight == 0 => ("#e0a33f", "o", " stroke-dasharray=\"5 4\""),
            Role::Require => ("currentColor", "a", " stroke-dasharray=\"2 3\""),
            _ => ("currentColor", "a", ""),
        };
        let node = if inbound { 150 } else { 470 };
        let label_x = if inbound { node - 14 } else { node + 14 };
        let anchor = if inbound { "end" } else { "start" };
        out.push_str(&format!(
            "<path d=\"M {x1} {y} C {}, {y}, {}, {middle}, {x2} {middle}\" fill=\"none\" \
             stroke=\"{stroke}\" stroke-width=\"1.2\"{dash} marker-end=\"url(#{marker})\"/>\n\
             <circle cx=\"{node}\" cy=\"{y}\" r=\"7\" fill=\"none\" stroke=\"currentColor\" \
             stroke-width=\"1.5\"/>\n\
             <text x=\"{label_x}\" y=\"{}\" text-anchor=\"{anchor}\" \
             fill=\"currentColor\" font-family=\"system-ui, sans-serif\" font-size=\"12\">{} {}</text>\n",
            (x1 + x2) / 2,
            (x1 + x2) / 2,
            y + 4,
            if arc.is_inhibitor() {
                "0!".to_string()
            } else {
                arc.weight.to_string()
            },
            escape(&net.places[arc.place].label()),
        ));
    }

    out.push_str("</svg>\n");
    out
}

/// The place list, as a table for the markdown sibling.
pub fn places_table(net: &Net) -> Vec<Vec<String>> {
    let mut out = vec![vec![
        "Place".to_string(),
        "Container".to_string(),
        "Kind".to_string(),
    ]];
    for place in &net.places {
        out.push(vec![
            place.label(),
            place.container.clone(),
            place.kind.clone(),
        ]);
    }
    out
}

/// What is not drawn, as a table, because a count alone does not say which.
pub fn excluded_table(net: &Net) -> Vec<Vec<String>> {
    let mut out = vec![vec!["Recipe".to_string(), "Why not drawn".to_string()]];
    for excluded in &net.excluded {
        out.push(vec![excluded.name.clone(), excluded.because.clone()]);
    }
    out
}

/// Whether a place is named by any drawn arc, which is how a kind goes missing.
pub fn unreached(net: &Net) -> Vec<&Place> {
    net.places
        .iter()
        .enumerate()
        .filter(|(at, _)| !net.arcs.iter().any(|arc| arc.place == *at))
        .map(|(_, place)| place)
        .collect()
}
