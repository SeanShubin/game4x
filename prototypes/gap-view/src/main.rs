//! Draws the gap projection at several planet sizes and gaps, onto one page to look at.
//!
//! `cargo run -p gap-view` - or `scripts/gap-view.ps1`, which is the one to remember.

use gap_view::{Layout, Point, draw, flatten, predicted_gap, seam, snug_scale};
use sphere_tessellation::{Direction, Params, Tessellation, solid};

/// One drawing: a planet size, a gap, and a sentence saying what it is for.
struct Panel {
    size: usize,
    /// What every territory is scaled by. The gap is one minus this.
    scale: f64,
    what: &'static str,
}

const PANELS: [Panel; 4] = [
    Panel {
        size: 42,
        scale: 1.0,
        what: "True size, no gap chosen. Only the space the projection itself opens, which is \
               nothing at the focus and everything at the rim.",
    },
    Panel {
        size: 42,
        scale: 0.88,
        what: "The same world with a twelve per cent gap chosen for looks. This is the one to \
               judge: it is what a player would see.",
    },
    Panel {
        size: 92,
        scale: 0.88,
        what: "The largest planet the game has, same gap. More territories means each is \
               smaller, so the rim carries more of them.",
    },
    Panel {
        size: 12,
        scale: 0.88,
        what: "The smallest planet. Twelve territories and 720 degrees of tax means 60 \
               degrees each, which is why so little of it looks flat.",
    },
];

fn main() {
    // **Beside the crate rather than beside the caller.** A relative default is resolved
    // against the working directory, so running this from the repository root and from the
    // crate wrote to two different places - and the second one was inside the first.
    let out = std::env::args().nth(1).unwrap_or_else(|| {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("drawing")
            .to_string_lossy()
            .into_owned()
    });
    std::fs::create_dir_all(&out).expect("somewhere to write the drawings");

    let mut page = String::from(
        "<!doctype html>\n<meta charset=\"utf-8\">\n<title>gap-view</title>\n\
         <style>\n\
         body { margin: 0; padding: 2rem; background: #0b0c11; color: #d7dbe4;\n\
                font: 15px/1.6 system-ui, sans-serif; }\n\
         h1 { font-size: 1.4rem; margin: 0 0 .3rem; }\n\
         .lede { max-width: 46rem; color: #9aa3b4; margin: 0 0 2rem; }\n\
         figure { margin: 0 0 3rem; }\n\
         figcaption { max-width: 46rem; color: #9aa3b4; margin: .6rem 0 0; }\n\
         b { color: #e8ecf5; }\n\
         img { width: min(100%, 760px); height: auto; background: #12131a; border-radius: 6px; }\n\
         table { border-collapse: collapse; margin: 0 0 2.5rem; }\n\
         td, th { padding: .25rem 1.25rem .25rem 0; text-align: left; }\n\
         th { color: #e8ecf5; font-weight: 600; }\n\
         </style>\n",
    );
    page.push_str("<h1>The gap projection</h1>\n");
    page.push_str(
        "<p class=lede>Every territory is drawn at its own shape, wherever it lands. What \
         stretches is the space between territories. <b>The whole world is on one page</b>, \
         so no destination needs the globe turned to reach it - which is the thing this is \
         for. The dot is the focus; the dashed circle is where the focus's antipode lands; \
         the territory outlined in red is the one the projection cannot draw honestly.</p>\n",
    );

    page.push_str(
        "<table><tr><th>from the focus</th><th>gap between territories</th>\
         <th>share of the world within</th></tr>\n",
    );
    for degrees in [15.0_f64, 30.0, 60.0, 90.0, 120.0, 150.0] {
        let radians = degrees.to_radians();
        page.push_str(&format!(
            "<tr><td>{degrees:.0} degrees</td><td>{:.0}%</td><td>{:.0}%</td></tr>\n",
            predicted_gap(radians) * 100.0,
            (1.0 - radians.cos()) / 2.0 * 100.0
        ));
    }
    page.push_str("</table>\n");

    for panel in &PANELS {
        let tessellation = Tessellation::generate(Params {
            region_count: panel.size,
            ..Params::default()
        });
        let built = solid(&tessellation.seeds, &tessellation.neighbours);
        let focus = Direction::of(tessellation.seeds[panel.size / 2]);
        let layout = flatten(&built, &tessellation.seeds, focus, panel.scale);

        let forced = snug_scale(&built, &tessellation.seeds, focus);
        let name = format!("gap-view-{}-{:.0}.svg", panel.size, panel.scale * 100.0);
        let drawing = draw::svg(&layout, &tessellation.neighbours, &draw::Style::default());
        std::fs::write(std::path::Path::new(&out).join(&name), &drawing)
            .unwrap_or_else(|why| panic!("cannot write {name}: {why}"));

        page.push_str(&format!(
            "<figure><img src=\"{name}\" alt=\"{} territories\">\n\
             <figcaption><b>{} territories, {:.0} per cent gap.</b> {} \
             Flattening on its own forces a gap of {:.1} per cent here; everything beyond \
             that is chosen. The territory that pays the seam is {}.</figcaption></figure>\n",
            panel.size,
            panel.size,
            (1.0 - panel.scale) * 100.0,
            panel.what,
            (1.0 - forced) * 100.0,
            seam(&layout),
        ));

        println!(
            "{:>3} territories, gap {:>3.0}% - wrote {name} (forced gap {:.1}%, seam {})",
            panel.size,
            (1.0 - panel.scale) * 100.0,
            (1.0 - forced) * 100.0,
            seam(&layout)
        );
    }

    let index = std::path::Path::new(&out).join("index.html");
    std::fs::write(&index, page).expect("the page");
    println!("\nopen {}", index.display());

    // A cheap fact worth printing beside the drawings: how far the furthest territory is from
    // the focus, in the units the map is drawn in.
    let tessellation = Tessellation::generate(Params {
        region_count: 42,
        ..Params::default()
    });
    let built = solid(&tessellation.seeds, &tessellation.neighbours);
    let focus = Direction::of(tessellation.seeds[21]);
    let layout: Layout = flatten(&built, &tessellation.seeds, focus, 0.88);
    println!(
        "the map reaches {:.2} radians from its centre, against pi = {:.2}",
        layout
            .tiles
            .iter()
            .map(|tile| tile.centre.distance_to(Point::ORIGIN))
            .fold(0.0, f64::max),
        std::f64::consts::PI
    );
}
