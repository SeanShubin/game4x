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
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .scan((String::new(), String::new()), |carried, mut cells| {
            // **The Recipe and Owner cells are written once and left blank down the rest of
            // that recipe's rows**, so a row on its own does not say which recipe it belongs
            // to or who owns it. Carried forward because *four of the six are player
            // recipes* is a count over the owner of a `put` row, and a row that has lost its
            // owner would count as neither.
            if cells[0].is_empty() {
                cells[0] = carried.0.clone();
                cells[1] = carried.1.clone();
            } else {
                *carried = (cells[0].clone(), cells[1].clone());
            }
            Some(cells)
        })
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
    let one_less: Vec<&&Vec<String>> = puts
        .iter()
        .filter(|cells| cells[5].contains("one less"))
        .collect();
    let player_one_less = one_less.iter().filter(|cells| cells[1] == "player").count();
    let reads_a_trait = rows
        .iter()
        .filter(|cells| !cells[3].is_empty() && cells[3].parse::<u32>().is_err())
        .count();

    let stated = [
        Stated {
            document: "docs/designing-rules.md",
            says: "73 role cells",
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
            says: "60 cells carry a quantity",
            derived: rows.len() - blank.len(),
        },
        Stated {
            document: "docs/designing-rules.md",
            says: "The thirteen rows split six, six and one",
            derived: puts.len(),
        },
        // **`25 recipes` was here and is gone.** It matched a sentence recording what
        // `P-494`, `P-495` and `C-115` took the table to - history, compared against the
        // present, which is the one thing this list must not do. The document states no live
        // recipe count, by choice: *how many rules that comes to is in the report and
        // deliberately not here.*
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
                // **Spelled-out numbers, added one at a time as documents use them.** A
                // missing word fails loudly - *no number in "fifteen `put` rows"* - rather
                // than reading as zero, which is the only property this list needs.
                s if s.contains("eleven") => Some(11),
                s if s.contains("seventeen") => Some(17),
                s if s.contains("fifteen") => Some(15),
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
    // **Five `refresh`'s and `upkeep`'s `paid`** - `P-522` cut two `refresh`'s with
    // `defending` and `hold` entire, taking eight to five, and `88b38801` added the Ark's
    // `working` to take five to six. The old message named `hold`, which stopped existing,
    // and the count is what said so - three times now, in the same assertion.
    assert_eq!(
        maxima, 6,
        "`put ... at its maximum` rows: five `refresh`'s and `upkeep`'s `paid`"
    );
    // **The other two thirds of the same sentence, which nothing re-derived until now.**
    // `P-552` moved all three at once and the loop above covered only the first, so *five,
    // five and one* stayed on the page while the count beside it was corrected. A number
    // stated in prose and not in this file is a number that goes stale in silence.
    assert_eq!(
        one_less.len(),
        6,
        "`put ... one less` rows: move, create labor, mine energy, work, bear and age"
    );
    assert_eq!(
        player_one_less, 4,
        "`put ... one less` rows a player owns: move, create labor, mine energy and work"
    );
    // **The release states its own table's numbers back in prose, and nothing reached that
    // until now.** `R-8` diagnosed it in place - *the check that covers what `docs/` says
    // about the release does not reach what the release says about itself* - and on
    // 2026-09-25 the code lane found the same shape three times in its own columns, one of
    // them four lines above the comment explaining the change that made it stale.
    //
    // **The cost of a founding unit is the case, because it is stated twice by design**: the
    // Recipes table charges it and the prose under *What the release leaves open* explains
    // what it means. `P-489` moved the pioneer's energy from six to two.
    let release = read("releases/first-release.md");
    for (recipe, says, want) in [
        (
            "produce pioneer",
            "**3 metal, 2 energy and 2 citizens**",
            [("metal", 3u32), ("energy", 2), ("citizen", 2)],
        ),
        (
            "launch ark",
            "3 metal, 12 energy and 2 citizens",
            [("metal", 3), ("energy", 12), ("citizen", 2)],
        ),
    ] {
        assert!(
            release.contains(says),
            "releases/first-release.md no longer says {says:?} of `{recipe}` - if the wording \
             changed, change it here too"
        );
        let charged: Vec<(String, u32)> = rows
            .iter()
            .filter(|cells| cells[0].trim_matches('*') == recipe && cells[2] == "consume")
            .map(|cells| {
                (
                    cells[4].clone(),
                    cells[3].parse().expect("a consume carries a number"),
                )
            })
            .collect();
        // **Poisoned from the other side too**: a recipe that stopped consuming anything would
        // make every comparison below vacuous.
        assert_eq!(
            charged.len(),
            want.len(),
            "`{recipe}` has {} consume rows and the prose names {}",
            charged.len(),
            want.len()
        );
        for (kind, quantity) in want {
            let found = charged.iter().find(|(k, _)| k == kind).unwrap_or_else(|| {
                panic!("`{recipe}` consumes no {kind}, and the prose says it does")
            });
            assert_eq!(
                found.1, quantity,
                "the prose says {says:?} and the Recipes table charges `{recipe}` {} {kind}",
                found.1
            );
        }
    }

    // **Each wording asserted where the number is**, because a standalone count cannot tell
    // that the document still says it - which is how the message above went on naming `hold`.
    let rules = read("docs/designing-rules.md");
    for says in [
        "**Six write *one less***",
        "**Six write *at its maximum*** - five `refresh`'s",
        "four of the six are player recipes",
    ] {
        assert!(
            rules.contains(says),
            "docs/designing-rules.md no longer says {says:?} - if the wording changed, change it here too"
        );
    }
    // **One since `P-522`**, which cut `muster` and `stand` and took the two strength reads
    // with them. What is left is `work`'s density.
    assert_eq!(
        reads_a_trait, 1,
        "quantities that read a trait rather than being a number"
    );
    assert_eq!(
        blank.len(),
        puts.len(),
        "a blank quantity and a `put` row are the same set, which is what makes `a put has no \
         quantity` true"
    );
}
/// **A document points at a generated report and does not restate its counts.**
///
/// **This asserts the opposite of the check it replaces**, which required
/// `docs/designing-rules.md` to quote `reports/nogain.md`'s figures exactly. Those figures went
/// stale three times in three days - `C-121` - and the third occurrence is what made the cause
/// exact: **it is provenance, not care.**
///
/// `reports/nogain.md` regenerates only when the code lane runs. **So the specification lane
/// cannot see the number in order to correct it, and the code lane cannot correct the document,
/// which is not theirs.** Neither could fix it alone, and the old check made it worse by failing
/// unless the stale pair was present in both places.
///
/// **What replaces it is the rule that stops it recurring**: point at the report, do not restate
/// it. Every figure left in that file is derivable from `releases/first-release.md`, which this
/// lane can see.
#[test]
fn no_document_restates_a_generated_report_s_counts() {
    let doc = read("docs/designing-rules.md");
    assert!(
        !doc.contains("rules, ground from"),
        "docs/designing-rules.md quotes `reports/nogain.md`'s counts again, which is the shape          C-121 recurred on three times - point at the report instead"
    );
    // **Counted over a population that is not zero**, or this passes for the wrong reason: the
    // document must still be talking about the report at all.
    assert!(
        doc.contains("reports/nogain.md"),
        "docs/designing-rules.md no longer mentions the report, so this checks nothing"
    );
}
