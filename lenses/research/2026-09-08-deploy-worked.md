# `deploy ark`, worked, and the seven rows that are already duplicated

2026-09-08. For Sean, who asked to take a simple recipe and see how it comes out, and who now also
wants the engine minimal and **the recipes editable inside the game itself**.

**Worked against `releases/first-release.md:202` as it stands**, read for this report rather than
recalled. Everything below is a reading of his sketch, not a proposal.

## What is there now

`deploy ark` is nine rows:

| #   | Role    | Qty | Kind      | Traits | Where                    |
| --- | ------- | --- | --------- | ------ | ------------------------ |
| 1   | require | 1   | territory |        | `$where`                 |
| 2   | consume | 1   | ark       |        | the orbit above `$where` |
| 3   | limit   | 0   | garrison  |        |                          |
| 4   | produce | 1   | garrison  |        |                          |
| 5   | produce | 2   | citizen   |        |                          |
| 6   | produce | 1   | extractor | food   |                          |
| 7   | produce | 1   | extractor | metal  |                          |
| 8   | produce | 1   | store     | food   |                          |
| 9   | produce | 1   | store     | metal  |                          |

## His sketch, in the change/threshold decomposition

```
ark.deploy                             selection: an ark
  territory = ark.location.below       binding
  change   (this ark,          -1)     was row 2
  threshold(garrison, at most 0, SOFT) was row 3
  change   (garrison,          +1)     was row 4
  change   (citizen,           +2)     was row 5
  change   (extractor food,    +1)     was row 6
  change   (extractor metal,   +1)     was row 7
```

**Three things fall out immediately, and two of them are decisions rather than translations.**

### Row 1 disappears, and so does a `Where`

`require 1 territory @ $where` is not a threshold. It is a **parameter declaration** wearing a
threshold's clothes - it says *this recipe takes a territory* and, incidentally, that the territory
exists. **Once the territory is derived from the selected ark, there is no parameter to declare.**

And row 2's qualifier goes with it. *The orbit above `$where`* exists to keep two independently named
things consistent; when one is defined as a function of the other they cannot disagree. **Nine rows
become seven, and two `Where` expressions become none** - and that is the whole of `X-8`'s
relational-versus-functional point, arriving as an arithmetic saving.

### The soft garrison forces a decision about the other five lines

Under the current rules, `limit 0 garrison` is **hard**, so it gates the entire recipe: deploying onto
a territory that already has a garrison is refused, and rows 4 to 9 never fire twice.

**Make it soft and that gate is gone.** Deploying onto an existing colony now succeeds, destroys the
ark, and runs every remaining line - **a second garrison is skipped, and two more citizens, another
food extractor, another metal extractor and two more stores all land on a colony that already had
them.**

**That is almost certainly not what he wants, and his sketch does not say what he wants instead.**
Each of the remaining lines needs its own answer, and they are not the same answer:

| Line                            | Plausible intent                                            |
| ------------------------------- | ----------------------------------------------------------- |
| garrison                        | soft threshold - skip if present, as sketched               |
| extractor food, extractor metal | probably the same, if a territory holds at most one of each |
| citizen +2                      | probably unconditional - more people is not a duplicate     |
| store food, store metal         | unknown, and depends on whether stores stack                |

**This is the re-encoding detecting a problem in the present specification, which is what he
predicted it would do.** The current table hides the question: one hard gate on the garrison silently
decides the behaviour of six other rows. Splitting the gate is what exposes that those six were never
individually considered.

### His sketch and the specification differ by two rows

The sketch produces a garrison, two citizens and two extractors. **The specification also produces a
food store and a metal store.** Not translated in above, because dropping them is a change to the
game and is his to make. Flagged rather than reconciled.

## The finding worth more than the worked example

**`found by land` contains the same seven rows, verbatim.**

| `deploy ark`                        | `found by land`               |
| ----------------------------------- | ----------------------------- |
| require 1 territory `$where`        | -                             |
| consume 1 ark, orbit above `$where` | consume 1 pioneer             |
| **limit 0 garrison**                | **limit 0 garrison**          |
| **produce 1 garrison**              | **produce 1 garrison**        |
| **produce 2 citizen**               | **produce 2 citizen**         |
| **produce 1 extractor food**        | **produce 1 extractor food**  |
| **produce 1 extractor metal**       | **produce 1 extractor metal** |
| **produce 1 store food**            | **produce 1 store food**      |
| **produce 1 store metal**           | **produce 1 store metal**     |

**Seven rows, duplicated exactly, differing in nothing.** Counted from the table rather than
remembered.

**So the first real call site for nesting is already in the specification.** A `found-colony`
sub-recipe called by both takes sixteen rows to eleven - but the number is not the point.
**The point is that *what a new colony starts with* is currently two edits, and one of them can be
forgotten.** For a game whose recipes are meant to be edited by players, that is the difference
between changing a rule and changing a rule twice.

**It also answers the code lane's objection to the acyclicity check.** `C-75` correctly refused to
wire it because no recipe calls any recipe, so the check would be green over an empty population.
**This is the population**, the moment it lands.

## On the engine being minimal, and the game being user-editable

**That requirement changes the status of an earlier finding, and this is the part worth keeping.**

To run this decomposition, an engine has to do four things: **resolve a path** (`ark.location.below`),
**evaluate a threshold**, **apply a change**, and **ground the recipe set against the state** to
produce the menu. Everything else is data. That is a small engine, and it is small **because** the
primitive set is small - which is the goal stated as an architecture rather than an aesthetic.

**But once a player can edit recipes, the engine must judge what a player wrote.** *Is this recipe
valid? Does this menu terminate? Does this nesting bottom out?* - and with a Turing-complete rule
language **it cannot answer any of them.** `X-9` said unbounded zero tests and recursive nesting cost
decidability, and `C-75` measured that adopting the boundedness rule costs nothing today. **User
editing is what turns that from an elegance concern into a product requirement**: the editor has to
reject a bad recipe *at edit time, with a reason*, and only a decidable language lets it.

**So the two checks stop being internal gates and become editor validation** - which is a better home
for them, because a rejection a player sees has to explain itself, and a rule that must explain
itself tends to be a rule worth having.

**And binding is sugar by his own test.** `territory = ark.location.below` can be removed by
substituting the path at each use - linear growth, not exponential - so it does not earn a place as a
primitive. **Which means he can add it purely for readability without it costing anything**, and that
is the pleasant direction of that test rather than the restrictive one.

## What this lens is not saying

It does not say what a second deploy onto an existing colony should do, whether stores belong in the
sketch, or whether `found-colony` should be extracted. Those are three decisions, and the worked
example exists to make them visible rather than to take them.

## Method

`deploy ark`, `found by land` and the recipe table read from `releases/first-release.md` at
`d01dde4`. The seven-row duplication was counted by comparing the two blocks row by row, not
recalled from a prior reading of either.
