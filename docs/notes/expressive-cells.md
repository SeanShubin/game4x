# Expressive cells

**Derived**, 2026-09-03, brainstorming Sean's *I like the tables, but I also like extra
expressiveness in a single cell of a table*. Not binding.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## It is already happening, without a notation

`entities.md`'s territory table has **seventeen columns**, and two of its cells are already carrying
structure with nothing to express it in:

```
| garrison                      |  ->  force 1 multiplier 1 manned 0
| food nodes                    |  ->  4, 4, 4
```

**`force 1 multiplier 1 manned 0` is a node written by hand.** And six of the seventeen columns -
`food nodes`, `food extractors`, `metal nodes`, `metal extractors`, `energy nodes`, `energy
extractors` - are **two facts times three resources, flattened into the header.** Add a fourth
resource and the table grows two more columns.

With `P-212`'s notation in a cell:

```
| id | founded | citizens | labor | force of nature | resources | garrison | yards |
| 1  | true    | 12       | 0     | 1               | {food capacity:3 density:4 built:3} {metal ...} | {garrison force:1 multiplier:1 manned:0} | 0 |
```

**Seventeen columns become eight, and a new resource adds no column at all.**

## The rule: normalize what you compare, nest what you do not

**A nested cell destroys the one thing a table is for.** You read `state.md` by scanning a column -
territory 1's food against territory 3's - and a value inside braces cannot be scanned, sorted or
lined up.

So the test is not *is this value structured*, it is **would you ever compare it across rows**.

- **`garrison force:1 multiplier:1 manned:0`** - you would not read a column of garrison forces.
  **Nest it.**
- **capacity, density and built per resource** - **this is exactly what you compare.** `3 x 4`
  against `2 x 6` against `6 x 2` is the whole reason the twelve territories are designed as they
  are. **Do not nest it.**

**And `state.md` already got that second one right, by a better means than columns.** It has a
`territory resource` table with **one row per territory and resource** - which is normalization, and
it beats both a nested cell and six columns.

## Which is the split he already has

**The two views differ in precisely this, and it is not a coincidence.**

| View                    | Cells                    | Because                                                |
| ----------------------- | ------------------------ | ------------------------------------------------------ |
| `state.md`, relational  | **scalar, never nested** | that is what normalization means                       |
| `entities.md`, physical | **may be nodes**         | it groups by entity, so a thing's parts are in one row |

**A nested cell in the relational view is a normalization failure with a nicer syntax.** A flattened
node in the physical view is seventeen columns. **Each view's discomfort is the other view's job**,
which is why both exist.

## The same question one level up, and it is the interesting one

**The release's own cells are already doing this, in prose.**

```
| traits     | next to `$from`                             |
| where      | the orbit above `$where`                    |
| bounded by | a capacity of 2, and the food produced here |
```

**Those are nodes written as English**, in tables `P-199` says the game **loads**. Something already
parses `next to $from`, and `prototypes/kinds` carries a qualifier type to do it.

**So there is a real choice here that has nothing to do with the reports**: whether the release's
compound cells become `{next-to $from}` and `{above $where}`.

- **Prose is readable and unparseable.** Sean writes and approves these tables; English is what he
  approves in.
- **Nodes are parseable and terser.** The loader stops guessing, and *what was expected* becomes
  sayable for a malformed cell.

**The honest answer is that this one is not urgent and the reports one is not either**, because the
rule above resolves it: **the release is a document a person writes, so prose; the reports are
generated, so nodes.** The release's cells become nodes only if and when the loader needs them to -
which is `S-21`'s territory, not this note's.
