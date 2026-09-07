# Review of `c4a8621..0e5f8f4` - the map form, 2026-09-06

Requested by Sean through the code lane. Read from the tree at `0e5f8f4`.

**Gate confirmed independently, by exit code.** `cargo test --workspace` exit 0 - 53 targets, 533
passed, 0 failed. `cargo clippy --workspace -- -D warnings` exit 0. `tools/outbox` and
`tools/pad-tables` exit 0. `cargo fmt` is not this lens's to run, so that line is theirs alone.

**`Q-64` is acted, and the number moved a long way.** The data file's vocabulary went from **50
distinct words with 19 forbidden** to **28 with 2**, re-derived here by classifying the new file
against the release's tables rather than by trusting `vocabulary.rs`. The two are `game` and
`manned`, both filed as `C-46`. `in-kind` and `in-id` are gone, which is what the item asked for -
removed with the map form rather than declared as traits.

---

## 1. A false reason, in the wording they asked to have checked

**Where.** `crates/game-console/tests/expected_state.rs:77`.

**What.** The comment reads *because capacity is derived and a derived trait is never part of a
description*. **`density` and `total capacity` are `stored`**, both of them, in
`releases/first-release.md:120-121`. The derived one in that neighbourhood is `metal in it`.

**Why it costs something.** That sentence is the justification for comparing against
`direct.contained()` rather than `direct`, and it makes the omission sound **legitimate** - a
derived trait *should* be absent. The doc comment thirty lines above says the true thing: the tree
round trips and the `Game` does not, *because a territory's `density` and `total capacity` are not
in the file*, citing `C-46`. **So one test carries two accounts of the same absence** - one calling
it a limitation, one calling it correct by rule - and only the second is next to the assertion it
explains.

**Their own item already says stored.** `C-46`: *`density` is a stored trait per resource and
`total capacity` [is too]*. So the code comment contradicts the finding filed about it, in the same
lane, on the same day.

**`P-303` is the rule this lands on** - *a reason that is false is worse than one that is missing* -
and the reader it misleads is the one who comes back unsure whether anything is missing, which is
the person the comment is for.

**Whether.** Worth one sentence, now. The decision is right and only the account is wrong: say
*capacity and density are stored and are not in the file - `C-46`*, which is what the doc comment
already says.

## 2. What `tree` drops silently - probed, and the answer is nothing I could find

They asked for a second pair of eyes on this and named it as the thing they could not settle by
reading.

**Poisoned rather than read**: `tree` made to skip the first thing in every territory. **Seven
checks fail**, in four files.

    containment::tests::things_that_cannot_be_told_apart_are_one_entry_with_a_quantity
    containment::tests::a_trait_that_tells_two_things_apart_makes_two_entries
    every_committed_dump_is_what_the_scenario_produces
    every_turn_of_the_scenario_is_dumped
    every_word_in_the_data_file_is_one_the_release_declares
    the_reviewed_expectation_holds
    tree::tests::the_page_collapses_without_a_script_and_says_what_is_full

**So a dropped entry is loud.** The unit count assertion covers orphaned units; `held` is mapped one
to one and `group` only merges equal descriptions. **`Thing::children` is the one path that would
vanish** - `Entry::leaf` never looks at it - and it is dead rather than latent: declared,
initialised empty in `new()`, and **written by nothing anywhere in the tree**. The remaining absence
is `density` and `total capacity`, which is `C-46` and is known.

## 3. The closed-set count, checked independently

Their seven agrees with this lens's classification, arrived at separately on 2026-09-06: **`kind`,
`resource` and `biome` point at a table; `ready`, `surplus` and `unpaid` name their values; `phase`
names them since `P-308`.** `control` is correctly excluded - *held by a player, or unclaimed* names
one value and describes the other, and the described one is three words, which `P-252` forbids in a
data file regardless.

## 4. What I did not check

Their points 3 and 5 - `turns_reconstruct`'s turn-1 blind region and `C-48` - were taken as
reported rather than verified. **Turn 1 being reconciled by nothing is recorded by them and not
repaired**, which is the honest state and needs no finding from here. `C-48` is filed to spec with
nothing built against it.

## 5. This lens's own instrument, wrong again, in this review

Running the poison above, `cargo test -p game-model -p game-console` reported **one** failing test,
and this report nearly said *only the HTML report catches it*. **Cargo stops at the first failing
target**, so `expected_state.rs` never ran. `--no-fail-fast` gives seven.

Third time today an instrument here answered a narrower question than the one asked, and the second
time in the same shape as `Q-63`, which was filed against the code lane for exactly this. **The
remedy that worked was not care**: it was that a claim of *one* against a subject this well tested
looked wrong, and re-running cost nothing.
