//! `R-8`: a kind's signature, and the grouping that is supposed to follow from it.
//!
//! **The thing being checked is an equivalence, so it is checked over every pair.** A test
//! that showed two kinds grouping would go on passing after the pair that made it true was
//! edited away - `docs/notes/checks-outlive-examples.md` is three checks that did exactly
//! that. The grouping is instead re-derived a second way and compared against the first over
//! all 105 pairs, and the pair count is asserted, because an equivalence over no pairs holds
//! for the wrong reason.

use kinds::catalog::{signature, signatures};
use kinds::release::{body_under, plain, release};

fn every_kind(document: &str) -> Vec<String> {
    body_under(document, "## Kinds")
        .iter()
        .map(|row| plain(&row[0]))
        .collect()
}

/// Every kind is in exactly one group, and the groups hold every kind.
///
/// **Both directions, because either alone is satisfied by a mistake.** A kind missing from
/// the groups would leave `signature_of` panicking rather than this failing, but a kind
/// appearing in two would not, and neither would a group holding a name the Kinds table has
/// never had.
#[test]
fn the_groups_partition_the_kinds() {
    let document = release();
    let kinds = every_kind(&document);
    assert_eq!(
        kinds.len(),
        15,
        "the release declares fifteen kinds; every count below is against that population"
    );

    let mut grouped: Vec<String> = signatures(&document)
        .into_iter()
        .flat_map(|(_, _, members)| members)
        .collect();
    let placed = grouped.len();
    grouped.sort();
    grouped.dedup();

    assert_eq!(
        placed,
        grouped.len(),
        "a kind is in two groups at once, so the grouping is not a partition"
    );
    let mut expected = kinds.clone();
    expected.sort();
    assert_eq!(
        grouped, expected,
        "the groups and the Kinds table hold different kinds"
    );
}

/// Two kinds are in one group exactly when their signatures are equal - over every pair.
///
/// **The grouping is derived twice and the two are compared.** `signatures` groups by
/// walking the kinds and matching against the groups it has built so far; this asks each
/// pair directly whether its two keys are equal, which is the definition rather than the
/// implementation of it. A bug in the first would have to be reproduced in the second to go
/// unseen.
#[test]
fn a_pair_shares_a_group_exactly_when_it_shares_a_signature() {
    let document = release();
    let kinds = every_kind(&document);
    let found = signatures(&document);

    let group_of = |kind: &str| -> String {
        found
            .iter()
            .find(|(_, _, members)| members.iter().any(|member| member == kind))
            .map(|(name, _, _)| name.clone())
            .unwrap_or_else(|| panic!("`{kind}` is in no group"))
    };

    let mut pairs = 0;
    let mut agreeing = 0;
    for (at, one) in kinds.iter().enumerate() {
        for other in kinds.iter().skip(at + 1) {
            pairs += 1;
            let same_key = signature(&document, one).key() == signature(&document, other).key();
            let same_group = group_of(one) == group_of(other);
            assert_eq!(
                same_key,
                same_group,
                "`{one}` and `{other}`: their signatures are {} and their groups are {}",
                if same_key { "equal" } else { "different" },
                if same_group { "one" } else { "two" }
            );
            if same_key {
                agreeing += 1;
            }
        }
    }
    assert_eq!(
        pairs,
        15 * 14 / 2,
        "every pair of the fifteen kinds is compared, and there are 105 of them"
    );

    // **The population this check ran against, said out loud.** Every assertion above is
    // satisfied by a signature that separates everything - and today one does, so `agreeing`
    // is zero and the equality half of the equivalence was never exercised. **A zero here is
    // not a pass**, it is the check reporting what it could not test, which is `C-64`.
    assert_eq!(
        agreeing, 0,
        "no two of the fifteen kinds share a signature, so nothing above tested two kinds \
         grouping together. If this fails, two kinds now collide - which is `C-64` answered, \
         and the catalog's paragraph about there being nothing to scan goes with it"
    );
}

/// Where the table under a heading is: its separator line, and one past its last row.
///
/// **A poison built by string-replacing a row is the bug this repository has recorded four
/// times.** The padder rewrites the column widths, so a match string drafted while writing
/// the edit meets bytes that no longer contain it, `replace` finds nothing, and the poison
/// lands nowhere - green, and proving exactly as much as no poison at all. So everything
/// below locates the table structurally and rewrites whole cells, matching no width.
fn table_at(lines: &[&str], heading: &str) -> (usize, usize) {
    let at = lines
        .iter()
        .position(|line| line.trim() == heading)
        .unwrap_or_else(|| panic!("no `{heading}` heading"));
    let separator = lines
        .iter()
        .skip(at)
        .position(|line| line.trim_start().starts_with("| ---"))
        .map(|found| at + found)
        .unwrap_or_else(|| panic!("no table under `{heading}`"));
    let mut end = separator + 1;
    while end < lines.len() && lines[end].trim_start().starts_with('|') {
        end += 1;
    }
    assert!(
        end > separator + 1,
        "the table under `{heading}` has no rows, so a poison would land on nothing"
    );
    (separator, end)
}

/// The document with one row added to a table.
fn with_row(document: &str, heading: &str, row: &[&str]) -> String {
    let lines: Vec<&str> = document.lines().collect();
    let (separator, _) = table_at(&lines, heading);
    let mut out: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
    out.insert(separator + 1, format!("| {} |", row.join(" | ")));
    out.join("\n")
}

/// The document with one column of a table rewritten, cell by cell.
fn mapping(document: &str, heading: &str, column: usize, to: impl Fn(&str) -> String) -> String {
    let lines: Vec<&str> = document.lines().collect();
    let (separator, end) = table_at(&lines, heading);
    let mut out: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
    let mut touched = 0;
    for line in out.iter_mut().take(end).skip(separator + 1) {
        let mut cells: Vec<String> = line
            .trim()
            .trim_start_matches('|')
            .trim_end_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect();
        if let Some(cell) = cells.get_mut(column) {
            let now = to(cell);
            if &now != cell {
                *cell = now;
                touched += 1;
            }
        }
        *line = format!("| {} |", cells.join(" | "));
    }
    assert!(
        touched > 0,
        "column {column} of the table under `{heading}` was rewritten nowhere, so this poison \
         proves nothing"
    );
    out.join("\n")
}

/// A signature is made of the two things `R-8` names and nothing else.
///
/// **Poisoning the input rather than the report.** Adding a trait to one kind must move that
/// kind's key and no other; changing every quantity must move none at all, because `R-8` says
/// the *(recipe, role)* pair and not the amount. **The version of this that proves nothing**
/// is comparing the catalog against itself, which is green whatever the signature is made of.
#[test]
fn the_key_moves_with_the_traits_and_with_nothing_else() {
    let document = release();
    let key_by = |text: &str| -> Vec<(String, String)> {
        every_kind(text)
            .iter()
            .map(|kind| (kind.clone(), signature(text, kind).key()))
            .collect()
    };
    let before = key_by(&document);

    // A trait of the yard alone, which no kind carries today.
    let poisoned = with_row(&document, "## Traits", &["`shielding`", "yard", "a number"]);
    assert_eq!(
        body_under(&poisoned, "## Traits").len(),
        body_under(&document, "## Traits").len() + 1,
        "the poisoned document has one more trait than the release"
    );
    let moved: Vec<&String> = before
        .iter()
        .zip(&key_by(&poisoned))
        .filter(|((_, was), (_, now))| was != now)
        .map(|((kind, _), _)| kind)
        .collect();
    assert_eq!(
        moved,
        vec!["yard"],
        "a trait of the yard alone moves the yard's key and no other"
    );

    // Every quantity moved at once, which is the strongest form of *this is not part of it*.
    let quantities = mapping(&document, "## Recipes", 3, |cell| {
        match cell.parse::<u32>() {
            Ok(was) => (was + 100).to_string(),
            Err(_) => cell.to_string(),
        }
    });
    assert_ne!(quantities, document, "no quantity moved");
    assert_eq!(
        before,
        key_by(&quantities),
        "a quantity is not part of a signature, so changing every one of them moves no key"
    );

    // And the control in the other direction: a *role* is part of it, so moving one must
    // move a key. Without this, the test above is also passed by a signature that reads
    // nothing from the Recipes table at all.
    let roles = mapping(&document, "## Recipes", 2, |cell| match cell {
        "consume" => String::from("produce"),
        other => other.to_string(),
    });
    assert_ne!(
        before,
        key_by(&roles),
        "a role is part of a signature, so turning every consume into a produce moves a key"
    );
}
