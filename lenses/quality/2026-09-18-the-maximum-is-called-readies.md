# The number refresh tops off to already exists, and it is called Readies

**Derived.** 2026-09-18, on Sean describing refresh as four allowances that should all be numbers.
Not binding.

[Quality](README.md) · [The release](../../releases/first-release.md) · [A fact is stated once](../../spec/invariants.md)

## What was asked and what the tree says

Sean, 2026-09-18: *the number of moves remaining to be topped off, amount of labor citizens can
generate to be topped off, the citizens ability to breed to be topped off, and the extractors
ability to operate to be topped off. These should all be numbers, although they all happen to be 1
at the moment.*

**All four numbers exist, are numbers, are per kind, and are all 1.** They are the **Readies**
column of *Units and structures* in `releases/first-release.md:173-181`:

- `citizen` - bearing 1, defending 1, laboring 1
- `extractor` - working 1
- `ark` - defending 1, moving 1
- `pioneer` - defending 1, moving 1

**Five allowances, not four**: `defending` is refreshed too, on citizens and on units, and
`spec/data/constraint.4x` has six `refresh` blocks to the release's six rows. The four named are
four of the five traits.

## The defect: the number is in a release and not in `spec/`

**Where.** `releases/first-release.md:173`, the **Readies** column; and `spec/`, which does not
have it.

**What.** `grep -rn "readies\|ready" spec/` finds nothing - no trait in `spec/data/traits.4x`, no
row in `spec/data/carries.4x`, no sentence in any `spec/*.md`. The per-kind maximum that every
`refresh` tops off to is stated **only** in a release.

**Why.** `CLAUDE.md` -> Releases: *A release spec never invents a rule. If a release needs one the
spec lacks, propose it into the spec first, then have the release refer to it.* And the release is
temporary - *deleted once vetted* - so the only statement of five of the game's numbers is in the
file with the shortest life. `refresh` in `spec/data/block.4x` is six blocks whose
`compare:at-maximum` has no maximum anywhere in `spec/` to point at.

**Whether.** Worth raising now, because it is the fact being designed against this week.

## And `movable` says one of the five a second time

**Where.** `spec/data/traits.4x` - `{trait name:movable admits:number kept:kind}` -
`spec/data/carries.4x` lines 23 and 30, and `releases/first-release.md:141` and the **Movable**
column at `:173`.

**What.** For `ark` and `pioneer`, `movable` is **1** and Readies says **moving 1**. Both are per
kind, both are numbers, both are 1. **Either they are one fact stated twice**, which
`spec/invariants.md` -> *A fact is stated once* forbids - *two statements of one fact can disagree,
and the disagreement is invisible to anyone reading either one alone* - **or they are two facts and
nothing anywhere says how they differ.**

**The weaker claim is certainly true and is enough**: `movable` has no definition in prose. It is
declared in a data file, given to two kinds, given a `Values` cell of *a number*, and defined
nowhere. `P-465` is recorded as having asked whether *a number* is the right word for it, and
`crates/outbox.md` says that question was the specification's rather than the code lane's.

**Why it matters here specifically.** Refresh is about to be built. If `movable` is the maximum,
then Readies duplicates it for two kinds and is the sole statement for the other four; if `movable`
is *whether this kind can move at all*, then it is a different trait that happens to hold the same
number, and a reader has no way to tell.

**Whether.** Worth deciding before refresh is built, and it is Sean's to decide rather than
anyone's to infer.

## What the prototype would need, which is unbuilt work rather than a defect

Recorded because it was measured while answering, not because anything is wrong.

- **The release already names the role.** *Role is one of `require`, `limit`, `consume`, `produce`
  or `put`* - `releases/first-release.md:188-194` - and every `refresh` row is a `put`. The
  prototype has three roles, `{role id:1 name:require}`, `remove` and `add`, and no `put`.
- **`remove` cannot express a top-off, and the reason is exact.** `Store::take`,
  `prototypes/thin-engine/src/store.rs:121-130`, returns `None` when less is held than is taken -
  `Refused::NothingToRemove` - and deletes the row when the two are equal. **So the allowance row
  is gone precisely when refresh needs to act**, and a rule written as remove-then-add refuses on
  the thing that most needs refreshing. This is why `put` is a role and not a pair of them.
- **The zero half Sean supposes is already right, and it works because of that deletion.** `work`
  removes one `working` - `data/friendly/rules.4x`, clause 13 with literal 1 - and at zero there is
  no row, so the next `work` is refused. `spec/data/constraint.4x` writes it as `at-least n:1`
  paired with `one-less`, 7 rows each.
- **`1` is load-bearing.** `{reading ...}` copies a value from a matched row -
  `prototypes/thin-engine/src/engine.rs:342-365`, a `values.insert` and no arithmetic. At a maximum
  of 1, refresh reads the extractor count straight into the `working` quantity and is exactly
  right. **At a maximum of 3 it needs a product**, and rule application has never multiplied - only
  the structure check has, in `counted` at `src/schema.rs:754-786`. So *numbers that happen to be
  1* and *numbers* are two concepts, and only the second costs arithmetic.
- **Neither existing cap can carry a rate at the right key.** `{limit held:working by:extractor}`
  compares quantity to quantity key for key at a fixed 1:1 - `held_within_what_holds_it`,
  `src/schema.rs:839-894` - and has no rate. `provides`/`consumes` has a rate but sums to the
  place, so a food extractor and a metal extractor in one territory would pool their allowances:
  `counted` finds the first column referencing the place and ignores `what`. **This is the same
  collision `Q-93` found for stores**, in a second place.
