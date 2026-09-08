//! `Q-70`: a dash is part of a name, and this file once decided that three different ways.
//!
//! `spec/console.md`: *A name is one word. Where it needs more than one, the words are joined
//! with dashes.* So `end-turn` is one word and a matcher that splits on the dash finds neither
//! it nor its halves. `catalog.rs` asked *does this phrase name this thing* in three places and
//! answered it two ways; the two that split on the dash outvoted the one that did not.
//!
//! **The failure was silent in the direction `R-8` exists to guard.** An empty answer from a
//! matcher that found nothing reads exactly like an empty answer from a kind that holds nothing.
//! Measured against the code as it was rather than argued: hyphenating all fifteen kinds merged
//! `deposit` and `adjacency` onto one key, and `R-8` reported two unrelated kinds as behaving
//! alike. **Only kinds with no recipe pairs merged** - the family match in `recipe_rows` is an
//! exact string compare and survives a rename - so the cost was one false group, not all of them.
//!
//! **Checked over every kind rather than on one.** All sixteen are renamed in turn, because a
//! test that hyphenated one kind would go on passing after that kind was edited away - which is
//! the failure `docs/notes/checks-outlive-examples.md` records three of. The count is asserted,
//! and so is the size of what is being preserved: an equality between two empty signatures holds
//! for the wrong reason.

use kinds::catalog::{signature, signatures};
use kinds::release::{body_under, plain, release};

/// The document with every whole-word occurrence of `name` rewritten to `to`.
///
/// **A word here is what `names_it` says it is** - a maximal run of alphanumerics and dashes -
/// so this and the code under test agree about where a name ends. Anything else would make a
/// green result a fact about this helper.
///
/// **Headings are left alone.** `body_under` looks a table up by its heading text, so a heading
/// is an address rather than a name: rewriting `## What bounds a kind in a territory` would hide
/// the table from the code instead of testing it, and the test would fail for the wrong reason.
fn renaming(document: &str, name: &str, to: &str) -> String {
    let is_word = |c: char| c.is_alphanumeric() || c == '-';
    let mut out = Vec::new();
    let mut hits = 0;
    for line in document.lines() {
        if line.trim_start().starts_with('#') {
            out.push(line.to_string());
            continue;
        }
        let mut rebuilt = String::new();
        let mut rest = line;
        while let Some(at) = rest.find(name) {
            let before = rest[..at].chars().next_back();
            let after = rest[at + name.len()..].chars().next();
            let whole = !before.is_some_and(is_word) && !after.is_some_and(is_word);
            rebuilt.push_str(&rest[..at]);
            if whole {
                rebuilt.push_str(to);
                hits += 1;
            } else {
                rebuilt.push_str(name);
            }
            rest = &rest[at + name.len()..];
        }
        rebuilt.push_str(rest);
        out.push(rebuilt);
    }
    assert!(
        hits > 0,
        "`{name}` was rewritten nowhere outside the headings, so this rename tests nothing"
    );
    out.join("\n")
}

/// A kind whose name carries a dash carries everything it carried without one.
///
/// **The rename is the only difference**, so the signature must be identical: the traits it
/// carries and every *(recipe, role)* pair naming it are facts about the kind, not about how its
/// name is spelled. Before `Q-70` this failed on all sixteen - two of the three matchers split
/// the new name in half and found neither piece.
#[test]
fn every_kind_keeps_its_signature_when_its_name_carries_a_dash() {
    let document = release();
    let kinds: Vec<String> = body_under(&document, "## Kinds")
        .iter()
        .map(|row| plain(&row[0]))
        .collect();
    assert_eq!(
        kinds.len(),
        16,
        "the release declares sixteen kinds; the count below is against that population"
    );

    let mut checked = 0;
    let mut traits_seen = 0;
    let mut pairs_seen = 0;
    for kind in &kinds {
        let before = signature(&document, kind);
        let hyphenated = format!("{kind}-of-a-kind");
        let renamed = renaming(&document, kind, &hyphenated);
        let after = signature(&renamed, &hyphenated);

        // A recipe may be named after a kind - `build extractor` is one - so renaming the kind
        // renames the recipe with it. That is the rename working, not the matcher failing, so
        // the new name is mapped back before comparing rather than the case being excluded.
        let restore = |what: &Vec<String>| -> Vec<String> {
            what.iter().map(|s| s.replace(&hyphenated, kind)).collect()
        };
        let after = kinds::catalog::Signature {
            traits: restore(&after.traits),
            pairs: restore(&after.pairs),
        };

        assert_eq!(
            before.traits, after.traits,
            "`{kind}` loses traits when its name carries a dash: {:?} became {:?}",
            before.traits, after.traits
        );
        assert_eq!(
            before.pairs, after.pairs,
            "`{kind}` loses recipe pairs when its name carries a dash: {:?} became {:?}",
            before.pairs, after.pairs
        );

        traits_seen += before.traits.len();
        pairs_seen += before.pairs.len();
        checked += 1;
    }

    assert_eq!(
        checked, 16,
        "a kind was skipped, so the rule is unchecked on it"
    );
    assert!(
        traits_seen > 0 && pairs_seen > 0,
        "every signature compared was empty, so the equality above holds for the wrong reason: \
         {traits_seen} traits and {pairs_seen} pairs across {checked} kinds"
    );
}

/// Hyphenated kinds stay as distinct as they were, which is the harm rather than the cause.
///
/// **`Q-8`'s shape: the test above says the matcher is right, and this one says what being
/// wrong cost.** `R-8` exists to find kinds that behave alike, so the direction that matters is
/// over-collision - a signature reporting two unrelated kinds as the same. Measured against the
/// pre-`Q-70` code rather than argued, on 2026-09-07 when there were fifteen kinds: renaming
/// them all collapsed the grouping to fourteen, and `deposit` and `adjacency` merged on the key
/// `kind | ` - each stripped to one trait and no pairs, which is not a resemblance but two
/// matchers failing in the same way. **The number is the one it was measured at**, and `game`
/// arrived after it; what the test asserts is derived from the table rather than from this.
///
/// **The grouping is the assertion, not the traits**, because a reader of `reports/catalog.md`
/// sees the groups. A false group is a confident wrong answer from the instrument built to give
/// a true one.
#[test]
fn hyphenating_every_kind_merges_none_of_them() {
    let document = release();
    let kinds: Vec<String> = body_under(&document, "## Kinds")
        .iter()
        .map(|row| plain(&row[0]))
        .collect();
    let before = signatures(&document).len();
    assert_eq!(
        (kinds.len(), before),
        (16, 16),
        "sixteen kinds in sixteen groups is the baseline this compares against"
    );

    let mut hyphenated = document.clone();
    for kind in &kinds {
        hyphenated = renaming(&hyphenated, kind, &format!("{kind}-of-a-kind"));
    }
    assert!(
        !hyphenated.contains("| **citizen** |"),
        "the renamed document still holds an un-renamed kind, so it is not the document meant"
    );

    let after = signatures(&hyphenated).len();
    assert_eq!(
        after,
        before,
        "hyphenating the names merged {} group(s) that were distinct: `R-8` would report kinds \
         as behaving alike because a matcher failed on both, not because they resemble each other",
        before - after
    );
}
