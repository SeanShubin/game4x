//! A promotion put in the target file what Sean approved.
//!
//! `S-10`, from the quality lens's `Q-39`. **`CLAUDE.md` promises that approved text is
//! byte-identical to shipped text, and after a promotion nothing could check it**: the
//! ledger keeps a one-line row, the proposal's body is deleted, and the approved text is
//! retained nowhere. The guarantee became unverifiable at the moment it was asserted, which
//! is why every defect in the promotions of 2026-09-01 was caught by a person.
//!
//! **It is buildable only from git**, and that is the whole trick: a promotion deletes the
//! proposal, so the proposal is still in the **parent** commit, with its `**shape**` and
//! `**into**` fields intact. No prose is parsed - both fields are structured.
//!
//! **Future promotions only, deliberately.** The 182 rows already in the ledger record their
//! destination as prose and would need a one-off audit of a back catalogue that has had a
//! week of readers. Here that is implemented as *an item that declares a shape*: the field
//! arrived in `P-195`, so anything older is skipped by having nothing to skip on.
//!
//! `P-194` gives three shapes and they are checked differently:
//!
//! - **text** is copied verbatim, so it must appear in the destination - compared with
//!   whitespace collapsed, because line wrapping is one of the three things a promotion may
//!   change.
//! - **rows** are table rows whose widths `tools/pad-tables` rewrites, so every cell must
//!   appear rather than every byte.
//! - **an instruction** lands nowhere verbatim and carries its own assertion, which the
//!   promoting commit runs. **This checks that it declared itself one and nothing else** -
//!   the weakest of the three arms, and named as weak rather than left to look complete.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).to_string())
}

/// A field's value, from the line that carries `**name**`.
///
/// Stops at the next `**`, so it does not matter whether the fields are joined by a middot
/// or by a hyphen, both of which appear in the queue.
pub fn field(body: &str, name: &str) -> Option<String> {
    let marker = format!("**{name}**");
    let at = body.find(&marker)? + marker.len();
    let rest = &body[at..];
    let end = rest.find("**").unwrap_or(rest.len());
    Some(
        rest[..end]
            .trim()
            .trim_end_matches(['\u{b7}', '-'])
            .trim()
            .to_string(),
    )
}

/// The file a proposal names, from an `**into**` field.
pub fn destination_file(into: &str) -> Option<String> {
    let start = into.find('`')? + 1;
    let end = into[start..].find('`')? + start;
    Some(into[start..end].to_string())
}

/// Whitespace collapsed, so that re-wrapping a paragraph is not a difference.
///
/// Wrapping is one of the three things `CLAUDE.md` allows a promotion to change, so a
/// comparison that saw it would report every correct promotion as wrong.
pub fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// What a promotion is checked to have done, for one proposal.
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Landed,
    /// Declared a shape this checker does not know.
    UnknownShape(String),
    Missing {
        what: String,
    },
    /// It did not land as approved, and the destination says so now.
    ///
    /// **A repaired promotion is not a standing failure.** This compares against the
    /// destination as it stood in the promoting commit, which is the precise question at
    /// the time - and no later commit can change what a past one contained, so a deviation
    /// caught and then fixed would otherwise stay red forever. `P-214` and `P-216` dropped
    /// their emphasis and `3fba321` restored it: the guarantee - *approved text is what is
    /// shipped* - holds again, and holding again is the outcome this exists to produce.
    Repaired,
}

/// The rule, over strings, so it can be run on documents written to be wrong.
pub fn check(shape: &str, block: &str, destination: &str) -> Verdict {
    match shape {
        "text" => {
            let there = flat(destination);
            let want = flat(block);
            // **A paragraph promoted as a bullet loses its full stop**, and
            // bullet-versus-paragraph is one of the three changes a promotion may make. So
            // the same text ending without its period is the same text. `P-213` landed
            // correctly and this reported it as missing until the allowance was written
            // down - the rule was in `CLAUDE.md` and not in the code that enforces it.
            let without = want.trim_end_matches('.').to_string();
            if there.contains(&want) || there.contains(&without) {
                Verdict::Landed
            } else {
                Verdict::Missing {
                    what: want.chars().take(70).collect(),
                }
            }
        }
        "rows" => {
            let flat_destination = flat(destination);
            for row in block.lines().filter(|line| line.trim().starts_with('|')) {
                for cell in row.trim_matches('|').split('|') {
                    let cell = cell.trim();
                    if cell.is_empty() || cell.chars().all(|c| c == '-') {
                        continue;
                    }
                    if !flat_destination.contains(&flat(cell)) {
                        return Verdict::Missing {
                            what: format!("cell {cell:?}"),
                        };
                    }
                }
            }
            Verdict::Landed
        }
        // Nothing lands verbatim, and the assertion it carries is prose that the promoting
        // commit runs. Checking the destination here would be checking the wrong thing.
        "instruction" => Verdict::Landed,
        other => Verdict::UnknownShape(other.to_string()),
    }
}

/// Promotions this check cannot pass and cannot fix, each with the reason.
///
/// **A named exception, because a silent skip is the disease.** A promotion is history and
/// history is not rewritten to make a check green, so the alternative to naming these is
/// scoping the check to start after them - which turns it off for a reason no reader can
/// see. One line each, and the test below requires every one of them to still be failing,
/// so an exception that has stopped being needed is reported rather than left to rot.
const KNOWN: &[(&str, &str)] = &[(
    "P-195",
    "declared `shape text` and its block is an instruction - it says what four sentences in      `CLAUDE.md` become and adds a template field, and nothing in it lands verbatim. The      first proposal to carry the field mislabelled its own shape, which is what this check      found on its first run. **No repair can clear this one**: the four sentences landed      correctly, so there is nothing in the destination to fix - the wrong thing is one field      in a deleted proposal. `C-17`, answered.",
)];

/// Every promotion since the `shape` field existed put its block where it said.
#[test]
fn a_promotion_lands_what_was_approved() {
    let root = root();
    if git(&root, &["rev-parse", "--is-inside-work-tree"]).is_none() {
        return;
    }
    // A shallow clone has one commit and therefore no parent to read a proposal from.
    if git(&root, &["rev-parse", "--is-shallow-repository"])
        .map(|out| out.trim() == "true")
        .unwrap_or(false)
    {
        return;
    }

    let log = git(
        &root,
        &[
            "log",
            "--format=%H",
            "-n",
            "80",
            "--",
            "docs/notes/proposals.md",
        ],
    )
    .unwrap_or_default();

    let mut checked = 0usize;
    let mut older = 0usize;
    let mut excepted = 0usize;
    let mut repaired = 0usize;
    let mut left_without_landing = 0usize;
    let mut wrong = Vec::new();

    for commit in log.lines() {
        let Some(before) = git(
            &root,
            &["show", &format!("{commit}^:docs/notes/proposals.md")],
        ) else {
            continue;
        };
        let Some(after) = git(
            &root,
            &["show", &format!("{commit}:docs/notes/proposals.md")],
        ) else {
            continue;
        };
        // **A promotion is located by the proposal disappearing, never by the ledger row
        // appearing.** `git log -S` on a row finds the commit where the padder last widened
        // that table, not the commit that added it: on `P-1`'s row, landed 2026-08-25, it
        // answers a commit from 2026-08-28 about naming surfaces. Every row looks added
        // whenever a column moves.
        //
        // **But a disappearance is not a promotion on its own** - a withdrawal removes an
        // item too, and would be checked here as though its text should have landed
        // somewhere. So the ledger has to have gained a row for it in the same commit.
        let landed_now: std::collections::BTreeSet<String> = outbox::accepted(&after)
            .into_iter()
            .map(|row| row.id)
            .collect();
        let landed_before: std::collections::BTreeSet<String> = outbox::accepted(&before)
            .into_iter()
            .map(|row| row.id)
            .collect();

        let gone: Vec<outbox::Item> = outbox::parse(&before, "docs/notes/proposals.md")
            .into_iter()
            .filter(|item| item.id.starts_with("P-") && item.is_outstanding())
            .filter(|item| !after.contains(&format!("### {} ", item.id)))
            .filter(|item| {
                let promoted = landed_now.contains(&item.id) && !landed_before.contains(&item.id);
                if !promoted {
                    // Withdrawn, rejected, or promoted without a ledger row. The three are
                    // not distinguishable from here, so this counts them rather than
                    // guessing which.
                    left_without_landing += 1;
                }
                promoted
            })
            .collect();

        for item in gone {
            let Some(shape) = field(&item.body, "shape") else {
                older += 1;
                continue;
            };
            let Some(into) = field(&item.body, "into").and_then(|i| destination_file(&i)) else {
                wrong.push(format!("{}: no readable **into** field", item.id));
                continue;
            };
            let Ok(block) = item.proposed_text() else {
                wrong.push(format!("{}: no readable block", item.id));
                continue;
            };
            let Some(destination) = git(&root, &["show", &format!("{commit}:{into}")]) else {
                wrong.push(format!("{}: {into} is not in {}", item.id, &commit[..7]));
                continue;
            };
            checked += 1;
            let mut verdict = check(&shape, &block, &destination);
            // If it did not land then, ask whether it has landed since. Only a `Missing` is
            // worth re-asking: an unknown shape is unknown at every commit.
            if matches!(verdict, Verdict::Missing { .. }) {
                if let Some(now) = git(&root, &["show", &format!("HEAD:{into}")]) {
                    if matches!(check(&shape, &block, &now), Verdict::Landed) {
                        verdict = Verdict::Repaired;
                    }
                }
            }
            if let Some((_, why)) = KNOWN.iter().find(|(id, _)| *id == item.id) {
                // The exception has to still be needed, or it is hiding a passing case and
                // will hide a failing one later.
                assert_ne!(
                    verdict,
                    Verdict::Landed,
                    "{} is excepted and now passes; delete the exception. It said: {why}",
                    item.id
                );
                excepted += 1;
                continue;
            }
            match verdict {
                Verdict::Landed => {}
                Verdict::Repaired => repaired += 1,
                Verdict::UnknownShape(what) => wrong.push(format!(
                    "{} declares shape {what:?}, which is not text, rows or an instruction",
                    item.id
                )),
                Verdict::Missing { what } => wrong.push(format!(
                    "{} promoted into {into} as {shape} at {}, and {what} is not there",
                    item.id,
                    &commit[..7]
                )),
            }
        }
    }

    // Said rather than asserted: zero checked and all correct are the same green, and an
    // empty queue is the good state, so a count cannot be required.
    println!(
        "{checked} promotion(s) checked, {repaired} repaired after the fact, {excepted} excepted by name; \n         {older} older than the shape field, {left_without_landing} left the queue without a ledger row"
    );
    assert!(
        wrong.is_empty(),
        "a promotion did not land what was approved:\n  {}\n\n\
         The proposal is in the promoting commit's parent. Read it there.",
        wrong.join("\n  ")
    );
}

/// The rule, over documents written to be wrong.
///
/// **The live test above walks real history and is green when history is clean**, which is
/// the same green as a rule that does nothing. Every arm is therefore made to fail here.
#[test]
fn each_shape_is_checked_differently_and_each_can_fail() {
    // Text: wrapping may change, and the words may not.
    let block = "The tables that define kinds are the data\nthe game loads.";
    let wrapped = "- The tables that define kinds\n  are the data the game loads.\n";
    assert_eq!(check("text", block, wrapped), Verdict::Landed, "re-wrapped");
    assert!(
        matches!(
            check("text", block, "- The tables are the data the game loads.\n"),
            Verdict::Missing { .. }
        ),
        "text that never arrived has to be caught"
    );

    // Rows: the padder rewrites widths, so cells travel and bytes do not.
    let rows = "| **orbit** | a place above one territory |";
    let padded = "| **orbit**   | a place above one territory   |\n";
    assert_eq!(check("rows", rows, padded), Verdict::Landed, "re-padded");
    assert!(
        matches!(
            check("rows", rows, "| **orbit** | a place |\n"),
            Verdict::Missing { .. }
        ),
        "a cell that never arrived has to be caught"
    );

    // An instruction lands nowhere verbatim, so the destination cannot answer for it.
    assert_eq!(check("instruction", block, ""), Verdict::Landed);

    // And a shape nobody defined is reported rather than ignored, which is how a typo in
    // the field would otherwise turn the check off for that proposal.
    assert_eq!(
        check("prose", block, ""),
        Verdict::UnknownShape("prose".to_string())
    );
}

#[test]
fn a_field_is_read_whatever_separates_it() {
    let middot = "**to** sean \u{b7} **kind** cleanup \u{b7} **shape** text \u{b7} **into** `CLAUDE.md` -> Promotion";
    let hyphen =
        "**to** sean - **kind** cleanup - **shape** rows - **into** `spec/planet.md` -> Shape";
    assert_eq!(field(middot, "shape").as_deref(), Some("text"));
    assert_eq!(field(hyphen, "shape").as_deref(), Some("rows"));
    assert_eq!(
        field(middot, "into").and_then(|i| destination_file(&i)),
        Some("CLAUDE.md".to_string())
    );
    assert_eq!(
        field(hyphen, "into").and_then(|i| destination_file(&i)),
        Some("spec/planet.md".to_string())
    );
    assert_eq!(field(middot, "nonesuch"), None);
}

/// A capability waiting on a person is outstanding, and shows up as such.
///
/// **`S-17`.** `releases/first-release.md` gives a capability three states, and the middle
/// one - `built`, addressed `to sean` - means *the code lane says it is done and nobody has
/// looked*. Reading only `open` made that item vanish from the index at exactly the moment
/// it started waiting on somebody. `CLAUDE.md` records the consequence: five capabilities
/// could never move while `pending.md` reported that nothing needed deciding.
///
/// **Written against text rather than against the live outboxes, deliberately.** Nothing
/// carries `built` today - all six capabilities are `vetted` or `open` - so a test reading
/// the real files would pass without exercising the case at all, and would go on passing
/// after somebody reintroduced the bug. The states are written out here instead.
#[test]
fn a_capability_that_is_built_is_still_waiting_on_somebody() {
    let release = "\
### R-9 - something not built yet

**to** code · **status** open · **vetted when** somebody looks

### R-10 - something built and unlooked-at

**to** sean · **status** **built** 2026-09-03 · **evidence** it exists

### R-11 - something a person has observed

**to** sean · **status** **vetted** 2026-09-03 · **evidence** it held
";
    let items = outbox::parse(release, "releases/first-release.md");
    assert_eq!(items.len(), 3, "three capabilities written, three read");

    let waiting: Vec<&str> = items
        .iter()
        .filter(|item| item.is_outstanding())
        .map(|item| item.id.as_str())
        .collect();
    assert_eq!(
        waiting,
        ["R-9", "R-10"],
        "a capability is outstanding while it is open and while it is built; only a person \
         setting it vetted ends that"
    );

    // The half that would have caught the original bug on its own.
    let built = items
        .iter()
        .find(|item| item.id == "R-10")
        .expect("written above");
    assert!(
        built.is_outstanding(),
        "`built` means nobody has looked yet, so it is the one state that most needs to be \
         visible - it is where every capability vetted by looking waits"
    );
    assert_eq!(
        built.to, "sean",
        "and it waits on a person, not on the code lane"
    );
}
