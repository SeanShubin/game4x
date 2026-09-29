//! Every number this lane's documents state, re-derived from what it is about.
//!
//! **Sean, 2026-09-12**: *ideally any staleness is fixed before I notice it.* This is the
//! mechanism for that, and it covered the numbers `docs/` states about
//! `releases/first-release.md`, which is where all four of the day's stale figures were.
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
//! # The subject was deleted on 2026-09-27, and this is what that costs
//!
//! **`c7bcd95c` deleted the *Recipes* table**, on Sean's word and under `D-4`. Every number
//! this file re-derived came from it, so **there is nothing left to re-derive** - and a check
//! that keeps its old shape over a gone population is the failure `CLAUDE.md` calls a count
//! over nothing, which is the same failure with the sign flipped.
//!
//! **So the predicate changes rather than the file being deleted.** What is checkable today is
//! that the document says its numbers are of a ruleset the game no longer plays, and that it
//! has not quietly gone on stating them as current. **That is weaker and it is not nothing**,
//! and it says out loud which it is.
//!
//! **The strong form returns with `S-208`**, which re-derives the counts against
//! `spec/data/rules.4x` - fifteen rules where the table stated twenty-one. When those numbers
//! land, they land here at the same time, and the list below fills again.
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

/// A number `docs/` states, and what it is a count of.
struct Stated {
    document: &'static str,
    says: &'static str,
    derived: usize,
}

/// Every stated number agrees with the thing it is about.
///
/// **The list is empty, and what is asserted is the reason rather than the emptiness.**
///
/// **An `assert!(stated.is_empty())` was written here first and clippy refused it** -
/// `const_is_empty`, *this expression always evaluates to true*. It was right and the refusal is
/// worth keeping: the length of a `[Stated; 0]` is a compile-time fact, so asserting it is a
/// sentence about the source rather than a check of anything. **The checkable reason is below** -
/// the table is gone and the document says its figures are of it - and that is what a reader of
/// this file needs to know is still true.
#[test]
fn every_number_the_documents_state_is_the_number_that_is_there() {
    let data = read("spec/data/rules.4x");
    let count = |kind: &str| {
        data.lines()
            .filter(|line| line.starts_with(&format!("{{{kind} ")))
            .count()
    };
    let role = |name: &str| data.matches(&format!("role:{name} ")).count();
    let stated = [
        Stated {
            document: "docs/designing-rules.md",
            says: "sixteen rules",
            derived: count("rule"),
        },
        Stated {
            document: "docs/designing-rules.md",
            says: "fifty-five clauses",
            derived: count("clause"),
        },
    ];
    // **Both roles asserted where the document names them**, because a count alone cannot tell that
    // the document still says it - which is how an older message in this file went on naming `hold`.
    assert_eq!(role("put"), 1, "`put` clauses in spec/data/rules.4x");
    assert_eq!(
        role("keep"),
        1,
        "`keep` clauses, which the document says is used once"
    );

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
                s if s.contains("sixteen") => Some(16),
                s if s.contains("fifty-five") => Some(55),
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

    let release = read("releases/first-release.md");
    assert!(
        !release.contains("## Recipes"),
        "the release has a Recipes section again, so a document counting it would have two sources"
    );
    // **The numbers are derived from the data now, which is `S-208` closed.** The document counted
    // the release's *Recipes* table until `c7bcd95c` deleted it; `spec/data/rules.4x` is where the
    // rules live since `D-1`, so that is what the figures are re-derived from and what the list
    // above reads.
    let rules = read("docs/designing-rules.md");
    assert!(
        rules.contains("The table this counted until 2026-09-29 had 73 role cells"),
        "docs/designing-rules.md no longer says where its old figures came from, so a reader meeting          73 anywhere has nothing to date it by"
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
/// it. Every figure left in that file was derivable from `releases/first-release.md`, which this
/// lane could see - until `D-4` deleted it.
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
