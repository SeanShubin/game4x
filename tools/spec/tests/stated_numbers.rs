//! Every number this lane's documents state, re-derived from what it is about.
//!
//! **Sean, 2026-09-12**: *ideally any staleness is fixed before I notice it.* This is the
//! mechanism for that, and it is narrow on purpose: it covers the numbers `docs/` states
//! about `releases/first-release.md`, which is where all four of the day's stale figures
//! were.
//!
//! **The failure it catches is `C-9`'s and it has no other catcher.** A count is written into
//! prose, a promotion moves what it counted, and **nothing edits the sentence because nothing
//! has to** - the words go on reading exactly as they did. `docs/designing-rules.md` said
//! *81 role cells* and *twelve `put` rows* until 2026-09-12; `P-427` had made it 77 and
//! `P-431` had made it 13, and both were found by re-deriving rather than by reading.
//!
//! **Why a test rather than a rule.** A rule asks a person to remember at the moment they are
//! least likely to - while promoting something else. The gate does not have that problem, and
//! `docs/process.md` says it in Sean's own words: *a rule that fires at a moment of confidence
//! does not survive as a habit, and needs a carrier.* **This is the carrier.**
//!
//! **Adding a number to a document means adding it here.** That is the cost, and it is the
//! point: a figure nobody will re-derive is a figure that should not be stated.

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("tools/spec sits two below the root")
        .to_path_buf()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative)).unwrap_or_else(|why| panic!("{relative}: {why}"))
}

/// The rows of the release's *Recipes* table, as cells, without the header or the rule.
fn recipe_rows() -> Vec<Vec<String>> {
    let text = read("releases/first-release.md");
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.trim() == "## Recipes")
        .expect("the release has a Recipes section");
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, line)| line.starts_with("## "))
        .map(|(at, _)| at)
        .expect("a section follows Recipes");
    lines[start..end]
        .iter()
        .filter(|line| line.starts_with('|'))
        .map(|line| {
            line.trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|cells| cells.len() > 5)
        .filter(|cells| {
            let role = &cells[2];
            !role.is_empty() && role != "Role" && !role.starts_with("---")
        })
        .collect()
}

/// A number `docs/` states, and what it is a count of.
struct Stated {
    document: &'static str,
    says: &'static str,
    derived: usize,
}

/// Every stated number agrees with the thing it is about.
///
/// **This is the test that would have failed on the morning of 2026-09-12** and did not
/// exist, so two figures in `docs/designing-rules.md` were wrong for a day and were found by
/// a person re-counting.
#[test]
fn every_number_the_documents_state_is_the_number_that_is_there() {
    let rows = recipe_rows();
    let puts: Vec<&Vec<String>> = rows.iter().filter(|cells| cells[2] == "put").collect();
    let blank: Vec<&Vec<String>> = rows.iter().filter(|cells| cells[3].is_empty()).collect();
    let maxima = puts
        .iter()
        .filter(|cells| cells[5].contains("at its maximum"))
        .count();
    let names: std::collections::BTreeSet<&str> = rows
        .iter()
        .filter(|cells| cells[0].starts_with("**"))
        .map(|cells| cells[0].trim_matches('*'))
        .collect();
    let reads_a_trait = rows
        .iter()
        .filter(|cells| !cells[3].is_empty() && cells[3].parse::<u32>().is_err())
        .count();

    let stated = [
        Stated {
            document: "docs/designing-rules.md",
            says: "77 role cells",
            derived: rows.len(),
        },
        Stated {
            document: "docs/designing-rules.md",
            says: "13 are blank",
            derived: blank.len(),
        },
        Stated {
            document: "docs/designing-rules.md",
            says: "thirteen `put` rows",
            derived: puts.len(),
        },
        Stated {
            document: "docs/designing-rules.md",
            says: "64 cells carry a quantity",
            derived: rows.len() - blank.len(),
        },
        Stated {
            document: "docs/designing-rules.md",
            says: "21 recipes",
            derived: names.len(),
        },
    ];

    for Stated {
        document,
        says,
        derived,
    } in stated
    {
        let text = read(document);
        assert!(
            text.contains(says),
            "{document} no longer says {says:?} - if the wording changed, change it here too"
        );
        let number = says
            .split_whitespace()
            .find_map(|word| word.parse::<usize>().ok())
            .or(match says {
                s if s.contains("thirteen") => Some(13),
                s if s.contains("twelve") => Some(12),
                _ => None,
            })
            .unwrap_or_else(|| panic!("no number in {says:?}"));
        assert_eq!(
            number, derived,
            "{document} says {says:?}, and the release now gives {derived}"
        );
    }

    // Counted here rather than only in prose, so the two that are not a bare count still move
    // the test when they move.
    assert_eq!(
        maxima, 6,
        "`put ... at its maximum` rows, all of which should be `refresh`'s"
    );
    assert_eq!(
        reads_a_trait, 3,
        "quantities that read a trait rather than being a number"
    );
    assert_eq!(
        blank.len(),
        puts.len(),
        "a blank quantity and a `put` row are the same set, which is what makes `a put has no \
         quantity` true"
    );
}

/// The document's claim about `reports/nogain.md` is the report's own number.
///
/// **A second copy of a generated figure**, and the one this lane reached for first when
/// asked where staleness comes from: `docs/designing-rules.md` quotes the report, and the
/// report is regenerated by a lane that does not read the document.
#[test]
fn the_quoted_report_figures_are_the_report_s() {
    let doc = read("docs/designing-rules.md");
    let report = read("reports/nogain.md");
    for figure in ["50 rules", "31 blocks"] {
        assert!(
            doc.contains(figure),
            "designing-rules no longer quotes {figure:?}"
        );
        let normalised = report.replace("**", "");
        assert!(
            normalised.contains(figure),
            "designing-rules quotes {figure:?} and reports/nogain.md no longer says it"
        );
    }
}
