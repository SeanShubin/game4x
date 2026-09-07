# Review of `b61938f..1716f6d` - 2026-09-07

Requested by the code lane, who named four places they thought they were most likely to be wrong.
Two are answered here; two were not examined and this says so.

**Gate, by exit code.** `cargo test --workspace --no-fail-fast` exit 0, `cargo clippy --workspace --
-D warnings` exit 0, `tools/pad-tables` exit 0. **`tools/outbox` exits 101**, on
`a_promotion_lands_what_was_approved`, naming `P-339`, `P-342`, `P-343` and `P-344` - all
specification-lane promotions. **Their attribution is right and I checked it rather than accepting
it**: the same test fails at `b61938f~1`, before any of their work.

---

## 1. Their second question, and it is a finding

**They asked**: `signatures.rs` asserts the agreeing-pair count is zero over 105 pairs, so the
equality half of the equivalence has never run - *is a check that names its own untested half still
a check that passes for the wrong reason?*

**Yes, and not for the reason they gave.** Naming an untested half is honest and does not test it.
But the specific hole is sharper than *untested*.

**The equivalence assertion cannot fail, in either direction.** `signatures()` builds its groups
**by** `key()` equality - `out.iter_mut().find(|(_, seen, _)| seen.key() == mine.key())` - and the
test computes `same_key` with the same comparison. So `same_key == same_group` is one reading
checked against itself. That is *a self-check may share inputs; it may not share the computation*.

**What does the work is `agreeing == 0`, and it is one-sided.** A degenerate key that merged
everything would make `agreeing` 105 and fail loudly. **A key that separates everything passes.**
And over-separation is the direction `R-8` exists to guard: the whole point of a signature is to
find kinds that are the same.

**Demonstrated rather than argued.** Poisoning `signature()` so a kind's own name is part of its
signature - the definitive over-separation bug, after which no two kinds can ever agree - leaves
**one** test failing: `the_committed_catalog_is_what_the_release_generates`, which compares the
generated text against the committed file. **Running `cargo run -p kinds -- catalog`, which is what
anyone making the change would do, turns the suite fully green.**

So a signature that can never collide is reachable with every check passing, and `R-8` would be
permanently vacuous and silently so.

**Whether.** Worth one synthetic case, using machinery they already wrote. `with_row` and `mapping`
build modified release documents; a document in which two kinds carry identical traits and identical
`(recipe, role)` pairs would exercise the equality half where it must hold, and would fail under the
poison above. Filed as `Q-69`.

**What is already well built, said so it is not swept up:**
`the_key_moves_with_the_traits_and_with_nothing_else` pins what is in the key and what is not, with
a control in each direction - a yard-only trait moves the yard's key alone, every quantity moved at
once moves none, and every `consume` turned to `produce` moves one. That is a better test than most
of what it guards.

## 2. Their first question: the cut is right, and they already made it

**They asked** whether a column can be a reference in one table and not in another - *which
`reference()` allows and `UNLINKED` assumes away*.

**`UNLINKED` does not assume it away.** It is `[(&str, &str); 23]` - pairs of table and column, not
columns. Both sides of the partition are keyed the same way, so the worry describes a design they
did not build.

**And the pair is necessary rather than merely permitted, which `id` proves.** `id` appears in three
tables and means three things: a territory in `territory`, a unit in `unit`, and **a kind's name**
in `kind`. `reference()` already distinguishes them - `("territory", "id")` links to a territory,
`("kind", "id")` to a kind, and `("unit", "id")` is deliberately in `UNLINKED`.

**This is `P-254` one level up.** That proposal renamed a self-identifier to `id` because
`territory` meant *in territory 1* in nine tables and *is territory 1* in one. The fix moved the
ambiguity rather than removing it, and the pair-keyed rule is what absorbs it.

**The one place the cut genuinely does not reach, and they saw it.** `in-id`'s target depends on a
**sibling cell** - `in-kind` says whether it is a territory's id or an orbit's - so `(table, column)`
is not enough and it would need the row. `browse.rs:80` returns `None` with the reason stated. That
is the correct answer to a case the partition cannot express, and it is the only one.

## 3. What I did not examine

**Their third and fourth questions.** `style.rs`'s CSS scraper reading something that is not a
selector, and the counts moved in `b61938f` - 18 commands, 2 shared transitions, 10 tables. Neither
was looked at, and neither should be read as checked.

**Two things they declared uncovered and did not assert a zero for**: `R-9`'s filtered-view clause,
which has no filtered view to be true of, and `P-212`'s nested command values, which are `C-65`.
Recorded because *declared uncovered* and *not looked at* are different states and only one of them
is theirs.
