# Recipes

**Generated. Do not edit.** `cargo run -p kinds -- recipes`, or `scripts/kinds.sh recipes`.

Every recipe the release declares, with its own lines gathered under it. The release states
these across seven columns, one row per line and the name written only on the first - which
is the right shape for a table and the wrong one for answering *what does this recipe do*.

15 recipes, 53 lines between them.

## deploy ark

Run by the **player**.

- **require** 1 territory, in `$where`
- **consume** 1 ark, in the orbit above `$where`
- **limit** 0 garrison
- **produce** 1 garrison
- **produce** 2 citizen
- **produce** 1 extractor, food
- **produce** 1 extractor, metal

## move

Run by the **player**.

- **require** 1 place, in `$from`
- **require** 1 place, joined to `$from` by an edge the unit crosses, in `$to`
- **consume** 1 unit, ready, in `$from`
- **consume** 1 energy, in that unit
- **produce** 1 unit, not ready, in `$to`

## found by land

Run by the **player**.

- **consume** 1 pioneer
- **limit** 0 garrison
- **produce** 1 garrison
- **produce** 2 citizen
- **produce** 1 extractor, food
- **produce** 1 extractor, metal

## build extractor

Run by the **player**.

- **consume** 1 labor
- **consume** 1 metal
- **produce** 1 extractor, `$resource`

## build yard

Run by the **player**.

- **consume** 1 labor
- **consume** 15 metal
- **produce** 1 yard

## produce pioneer

Run by the **player**.

- **consume** 3 metal
- **consume** 6 energy
- **consume** 2 citizen
- **produce** 1 pioneer

## produce ark

Run by the **player**.

- **consume** 3 metal
- **consume** 12 energy
- **consume** 2 citizen
- **require** 1 yard
- **produce** 1 ark

## create labor

Run by the **player**.

- **consume** 1 citizen, ready
- **produce** 1 citizen, not ready
- **produce** 1 labor

## work

Run by the **player**.

- **require** 1 territory, in `$where`
- **consume** 1 labor
- **consume** 1 extractor, ready
- **produce** 1 extractor, not ready
- **produce** resource — `$where`'s density for that resource

## upkeep

Run by the **world**.

- **require** 1 thing, with upkeep
- **consume** food — the thing's upkeep

## grow

Run by the **world**.

- **consume** 1 food, surplus
- **require** 1 thing, houses
- **produce** 1 citizen

## perish

Run by the **world**.

- **consume** 1 thing, whose upkeep is unpaid
- **produce** metal — the thing's metal

## spoil

Run by the **world**.

- **consume** 1 food, keeps 0

## age

Run by the **world**.

- **consume** 1 food, keeps at least 1
- **produce** 1 food, keeps one less

## refresh

Run by the **world**.

- **consume** 1 thing, not ready
- **produce** 1 thing, ready
