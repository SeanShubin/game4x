# Catching up with the spec: move is a recipe now - 2026-09-07

Sean removed the special-ness of move and asked this lens to catch up with the specification and
look for what that leaves behind.

**Gate green at `b61938f`, by exit code**: `cargo test --workspace --no-fail-fast` exit 0.

---

## What the change is

**`move` is a row in the recipe table**, in `releases/first-release.md` -> *Recipes*:

| Role    | Qty | Kind   | Traits                                        | Where     |
| ------- | --- | ------ | --------------------------------------------- | --------- |
| require | 1   | place  |                                               | `$from`   |
| require | 1   | place  | joined to `$from` by an edge the unit crosses | `$to`     |
| consume | 1   | unit   | ready                                         | `$from`   |
| consume | 1   | energy |                                               | that unit |
| produce | 1   | unit   | not ready                                     | `$to`     |

That is the symmetry: **the unit is consumed here and produced there**, and everything else about a
move is an ordinary ingredient. `spec/orbit.md` no longer uses the word *move* at all, and `P-344`
removed `ascent` from the Ark's crossings.

## The one leftover, and it is live

**Where.** `releases/first-release.md:156` - the `A move` column of *Units and structures*, reading
`1 fuel` for `ark` and `pioneer`. Modelled in `prototypes/kinds/src/lib.rs:1150`.

**What.** The recipe fixes the cost: `consume | 1 | energy | that unit`, a constant, for **any**
unit. **The column is per-thing, and the rule is no longer per-thing.** If a future unit's row said
`2 fuel`, the recipe would still consume one, and the recipe is the mechanism - so the column can no
longer express anything, and can only repeat.

**And the same fact is stated a third time.** `spec/units.md:17`: *A mobile unit has a bin for fuel.
Moving burns a unit of it, and a unit with none cannot move.* Three statements, one of which is now
the mechanism and two of which are prose beside it.

**What is not a leftover, and is worth saying so it is not confused with one:** the `Crosses`
column. The recipe **reads** it - *joined to `$from` by an edge the unit crosses* - so it is an
ingredient's trait rather than a second copy of the rule. `Fuel` is likewise the tank's size and
still does work.

**Whether.** Worth a decision rather than an edit, because deleting a column from the release is
Sean's. Filed as `Q-68`.

## A red gate that was already green by the time I finished

Measured at `a09f36b`, the workspace gate was **red**, and `prototypes/kinds` disagreed with the
release in at least four places - each hidden behind the last, because the comparison reports the
first difference and stops:

    Traits row 16              keeps is of `food`; the release says `thing`
    What bounds a kind row 7   pioneer keeps "and the food produced here"; the release dropped it
    Units and structures row 6 ark crosses "orbit border, ascent"; the release says "orbit border"
    Units and structures row 7 pioneer carries "1 food per turn"; the release leaves it blank

**`b61938f` fixed all of them while this review was running**, so none of it is a finding and none
of it is filed. It is recorded because *the gate was red and is now green* is a different thing from
*the gate was never red*, and only the first explains why `P-344` and its neighbours needed a
follow-up commit at all.

**One thing from it is worth keeping.** `P-344`'s message said *nothing in `crates/` names an
ascent, so the gate stays green.* That was true of `crates/` and the divergence was in
`prototypes/`, which is a workspace member whose test compares against the release. **The claim was
checked against a narrower population than the gate covers** - `Q-63`'s shape, in the other lane.
Not filed: it was true within a commit of being written and the state it described is gone.

## This lens's own instrument, twice

**I predicted the failure and the failure was something else.** Expecting `ascent`, the first
mismatch reported was `keeps`. Running it rather than asserting it is the only reason the other
three surfaced.

**And I reviewed a moving tree.** The working copy carried thirty-three uncommitted files when this
started and none when it ended. That is the baseline race from
[the README](README.md#a-baseline-recorded-mid-flight-fails-in-the-direction-that-looks-like-success)
in its other direction: not a range that came out empty, but findings that were true when measured
and repaired before they could be written. **The remedy is the same** - re-check against the tree
as it is at the moment of filing, which is what turned three findings into none.
