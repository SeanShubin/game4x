//! Writes `drawing/index.html` - twenty worlds to pick between, and the numbers beside them.
//!
//! **A page rather than a window**, which is `gap-view`'s shape and not `goldberg-view`'s. The
//! deliverable is the answer to a question, and a file Sean can open without a build reaches
//! him more reliably than a binary he has to run. The controls `X-32` asks for are all here;
//! they are key handlers in a page instead of key handlers in `bevy`.
//!
//! Pass a directory to write somewhere else.

fn main() {
    let mut arguments = std::env::args().skip(1);
    let into = arguments
        .next()
        .unwrap_or_else(|| format!("{}/drawing", env!("CARGO_MANIFEST_DIR")));
    std::fs::create_dir_all(&into).unwrap_or_else(|why| panic!("cannot make {into}: {why}"));

    let at = format!("{into}/index.html");
    std::fs::write(&at, hex_torus_view::page::page())
        .unwrap_or_else(|why| panic!("cannot write {at}: {why}"));
    println!("wrote {at}");
    println!();
    println!("Open it. At each size, zoomed out: exactly N hexes are undimmed, and every");
    println!("dimmed hex shows the id of an undimmed one. That is what `X-32` asks you to see.");
    println!();
    println!("The twenty are buttons under the heading - ten folded, then ten axis-aligned,");
    println!("labelled by how many territories each has. [ and ] step them, I toggles ids,");
    println!("drag or the arrows pan, the wheel zooms, R resets.");
}
