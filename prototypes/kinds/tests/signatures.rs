//! `R-8`: a kind's signature, and the grouping that is supposed to follow from it.
//!
//! **The thing being checked is an equivalence, so it is checked over every pair.** A test
//! that showed two kinds grouping would go on passing after the pair that made it true was
//! edited away - `docs/notes/checks-outlive-examples.md` is three checks that did exactly
//! that. The grouping is instead re-derived a second way and compared against the first over
//! all 120 pairs, and the pair count is asserted, because an equivalence over no pairs holds
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
        16,
        "the release declares sixteen kinds; every count below is against that population"
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
        16 * 15 / 2,
        "every pair of the sixteen kinds is compared, and there are 120 of them"
    );

    // **The population this check ran against, said out loud.** Every assertion above is
    // satisfied by a signature that separates everything - and today one does, so `agreeing`
    // is zero and the equality half of the equivalence was never exercised. **A zero here is
    // not a pass**, it is the check reporting what it could not test, which is `C-64`.
    assert_eq!(
        agreeing, 0,
        "no two of the sixteen kinds share a signature, so nothing above tested two kinds \
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

/// Two kinds the release says the same things about land in one group.
///
/// **`Q-69`, and the quality lens is right about why the other checks could not catch this.**
/// `a_pair_shares_a_group_exactly_when_it_shares_a_signature` compares the grouping against
/// `key()` equality - and `signatures()` builds the groups *by* `key()` equality, so it is one
/// reading checked against itself. A self-check may share inputs; it may not share the
/// computation. What was doing the work there was `agreeing == 0`, and that is one-sided: a
/// key that merged everything fails loudly, and **a key that separates everything passes**.
///
/// Over-separation is the direction `R-8` exists to guard, because the point of a signature is
/// to find the kinds that are the same. The lens demonstrated it rather than arguing it: make a
/// kind's own name part of its signature - after which no two kinds can ever agree - and one
/// test fails, the committed-catalog comparison, which regenerating turns green. A signature
/// that cannot collide was reachable with the whole suite passing.
///
/// **So the equality half needs a case where equality must hold**, and the release has none -
/// which is `C-64`, and is a fact about the release rather than something to fix here. This
/// builds one: two kinds nothing else in the document distinguishes.
#[test]
fn two_kinds_the_release_says_the_same_things_about_share_a_signature() {
    let document = release();
    let before = every_kind(&document).len();

    // Two kinds, one trait covering both, and one recipe naming each in the same role. The
    // recipe rows go in last-first, because each insert goes directly under the separator -
    // so `alpha` ends up above `beta`, and `alpha` carries the recipe's name while `beta`
    // inherits it, which is how the table says two rows belong to one recipe.
    //
    // **No hyphen in either name, and it is not arbitrary.** `catalog::mentions` splits the
    // Traits table's *Of* column on every non-alphanumeric character, so a kind called
    // `alpha` is two words there and matches nothing - while `catalog::containers` splits
    // the same way but keeps `-`, so the two disagree about what a name is. No kind in the
    // release has a hyphen, so nothing is wrong today; the first one to have a hyphen would
    // silently carry no traits. Found by trying to name a kind `alpha` here.
    let mut poisoned = with_row(&document, "## Kinds", &["**alpha**", "one of a pair"]);
    poisoned = with_row(&poisoned, "## Kinds", &["**beta**", "the other"]);
    poisoned = with_row(
        &poisoned,
        "## Traits",
        &["**pairing**", "alpha, beta", "a number", "stored"],
    );
    poisoned = with_row(
        &poisoned,
        "## Recipes",
        &["", "player", "produce", "1", "beta", "", ""],
    );
    poisoned = with_row(
        &poisoned,
        "## Recipes",
        &["**pair up**", "player", "produce", "1", "alpha", "", ""],
    );

    let kinds = every_kind(&poisoned);
    assert_eq!(
        kinds.len(),
        before + 2,
        "the two kinds are in the document; without them nothing below is about anything"
    );

    // **The premise, stated rather than assumed.** If the two were not equal here, the
    // assertion after it would be checking that unequal things are apart - which is what the
    // rest of this file already does, and not what this test is for.
    let a = signature(&poisoned, "alpha");
    let b = signature(&poisoned, "beta");
    assert_eq!(
        a.key(),
        b.key(),
        "the document says the same things about both, so their keys are equal"
    );
    // `keeps` is declared *of thing*, and *thing* is the family whose members are every kind
    // above - so every kind carries it. It reached none of them until `C-71`, because
    // `every kind above` is a membership rather than a comma-separated list and both joins
    // split on commas. The synthetic trait is what this case is about; `keeps` is here
    // because leaving it out would be asserting the bug.
    assert_eq!(
        a.traits,
        vec![
            String::from("keeps"),
            String::from("kind"),
            String::from("pairing")
        ]
    );

    // The world's five recipes name `thing`, so they reach every kind - and they reached none
    // until `C-71`, for the same reason `keeps` did not. `pair up` is the one this case adds.
    assert!(
        a.pairs.contains(&String::from("pair up produce")),
        "the synthetic recipe is what this case is about: {:?}",
        a.pairs
    );
    assert_eq!(
        a.pairs,
        vec![
            String::from("age consume"),
            String::from("age produce"),
            String::from("pair up produce"),
            String::from("perish consume"),
            String::from("refresh consume"),
            String::from("refresh produce"),
            String::from("spoil consume"),
            String::from("upkeep require")
        ]
    );

    let found = signatures(&poisoned);
    let group_of = |kind: &str| -> String {
        found
            .iter()
            .find(|(_, _, members)| members.iter().any(|member| member == kind))
            .map(|(name, _, _)| name.clone())
            .unwrap_or_else(|| panic!("`{kind}` is in no group"))
    };
    assert_eq!(
        group_of("alpha"),
        group_of("beta"),
        "two kinds with one signature are shown together, which is what `R-8` asks for"
    );

    let together: Vec<&Vec<String>> = found
        .iter()
        .filter(|(_, _, members)| members.len() > 1)
        .map(|(_, _, members)| members)
        .collect();
    assert_eq!(
        together.len(),
        1,
        "exactly one group holds more than one kind, and it is the pair this built: {together:?}"
    );
    // Sorted, because a group keeps the order the Kinds table gives - and `with_row` inserts
    // at the top, so the second insert is the first row. That order is a fact about the
    // fixture rather than about the grouping, and asserting it here would make this test
    // fail for a reason it is not about.
    let mut held = together[0].clone();
    held.sort();
    assert_eq!(
        held,
        vec![String::from("alpha"), String::from("beta")],
        "and the group holds both of them and nothing else"
    );

    // **The rest of the document is untouched by the pair.** Without this, a signature that
    // merged everything would satisfy every assertion above.
    let real: Vec<String> = every_kind(&document)
        .iter()
        .map(|kind| group_of(kind))
        .collect();
    let mut distinct = real.clone();
    distinct.sort();
    distinct.dedup();
    assert_eq!(
        distinct.len(),
        real.len(),
        "the sixteen real kinds are still in sixteen groups, so the pair grouping is the \
         signature agreeing rather than the signature collapsing"
    );
}
