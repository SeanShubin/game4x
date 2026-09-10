//! What the projection costs, printed rather than asserted, so the numbers can be read.
//!
//! `cargo run --example report`

use gap_view::{flatten, predicted_gap, seam, snug_scale};
use sphere_tessellation::{Direction, Params, Tessellation, solid};

const SIZES: [usize; 5] = [12, 32, 42, 72, 92];

fn main() {
    println!("The tax by distance from the focus, and it does not depend on planet size");
    println!(
        "{:>12} {:>8} {:>22}",
        "from focus", "gap", "share of world within"
    );
    for degrees in [15.0_f64, 30.0, 45.0, 60.0, 90.0, 120.0, 150.0] {
        let radians = degrees.to_radians();
        let within = (1.0 - radians.cos()) / 2.0;
        println!(
            "{degrees:>11}d {:>7.0}% {:>21.0}%",
            predicted_gap(radians) * 100.0,
            within * 100.0
        );
    }

    println!();
    println!("The gap that is not optional: what flattening one territory costs");
    println!(
        "{:>6} {:>14} {:>10} {:>26}",
        "size", "defect each", "shrink", "widest territory, across"
    );
    for size in SIZES {
        let tessellation = Tessellation::generate(Params {
            region_count: size,
            ..Params::default()
        });
        let built = solid(&tessellation.seeds, &tessellation.neighbours);
        let scale = snug_scale(
            &built,
            &tessellation.seeds,
            Direction::of(tessellation.seeds[size / 2]),
        );

        let widest = built
            .cells
            .iter()
            .enumerate()
            .map(|(territory, corners)| {
                let centre = Direction::of(tessellation.seeds[territory]);
                corners
                    .iter()
                    .map(|&corner| centre.angle_to(built.corners[corner as usize]))
                    .fold(0.0_f64, f64::max)
            })
            .fold(0.0_f64, f64::max);

        println!(
            "{size:>6} {:>13.1}d {:>9.1}% {:>25.1}d",
            720.0 / size as f64,
            (1.0 - scale) * 100.0,
            widest.to_degrees() * 2.0
        );
    }

    println!();
    println!("Where the seam lands, and how much of the world reads truthfully");
    println!(
        "{:>6} {:>10} {:>16} {:>18}",
        "size", "seam", "within 60d", "within 90d"
    );
    for size in SIZES {
        let tessellation = Tessellation::generate(Params {
            region_count: size,
            ..Params::default()
        });
        let built = solid(&tessellation.seeds, &tessellation.neighbours);
        let scale = snug_scale(
            &built,
            &tessellation.seeds,
            Direction::of(tessellation.seeds[size / 2]),
        );
        let focus = Direction::of(tessellation.seeds[size / 2]);
        let layout = flatten(&built, &tessellation.seeds, focus, scale);

        let within = |degrees: f64| {
            layout
                .tiles
                .iter()
                .filter(|tile| tile.from_focus <= degrees.to_radians())
                .count()
        };
        println!(
            "{size:>6} {:>10} {:>11} of {size:<3} {:>13} of {size:<3}",
            seam(&layout),
            within(60.0),
            within(90.0)
        );
    }
}
