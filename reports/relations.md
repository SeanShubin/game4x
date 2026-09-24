# The rules, as relations

**Generated. Do not edit.** One table per relation in `spec/data/`, which is where the
rules are stated. `reports/state.md` is the same view over the state the scenario
leaves; this is the same view over the rules it played by.

**The relation is the word each row opens with, and it is not always the file name** -
`biomes.4x` holds `value`, `families.4x` holds `family`, `kinds.4x` holds `kind` and
`traits.4x` holds `trait`.

12 relations, 233 rows.

## Values that are sentences

`spec/console.md` gives a field one token and these carry several, so a reader has
to take the words up to the next `key:` as one value. **This is `C-120` and it is
shown rather than corrected here** - the data is not this report's to edit.

| Where                       | Field | Value                                |
| --------------------------- | ----- | ------------------------------------ |
| `spec/data/line.4x` line 41 | `qty` | `$where`'s density for that resource |

## above

From `spec/data/above.4x`. 12 row(s), 2 column(s).

| orbit | territory |
| ----- | --------- |
| 1     | 1         |
| 2     | 2         |
| 3     | 3         |
| 4     | 4         |
| 5     | 5         |
| 6     | 6         |
| 7     | 7         |
| 8     | 8         |
| 9     | 9         |
| 10    | 10        |
| 11    | 11        |
| 12    | 12        |

## value

From `spec/data/biomes.4x`. 6 row(s), 2 column(s).

| name      | of    |
| --------- | ----- |
| ocean     | biome |
| ice       | biome |
| desert    | biome |
| grassland | biome |
| jungle    | biome |
| mountain  | biome |

## block

From `spec/data/block.4x`. 27 row(s), 3 column(s).

| id                        | owner  | recipe          |
| ------------------------- | ------ | --------------- |
| deploy-ark                | player | deploy-ark      |
| move                      | player | move            |
| found-by-land             | player | found-by-land   |
| build-extractor           | player | build-extractor |
| build-store               | player | build-store     |
| build-yard                | player | build-yard      |
| produce-pioneer           | player | produce-pioneer |
| launch-ark                | player | launch-ark      |
| create-labor              | player | create-labor    |
| work                      | player | work            |
| upkeep                    | world  | upkeep          |
| bear                      | world  | bear            |
| breed                     | world  | breed           |
| perish                    | world  | perish          |
| age                       | world  | age             |
| spoil                     | world  | spoil           |
| stow-metal                | world  | stow            |
| stow-energy               | world  | stow            |
| discard-metal             | world  | discard         |
| discard-energy            | world  | discard         |
| discard-labor             | world  | discard         |
| discard-fertility         | world  | discard         |
| refresh-unit-moving       | world  | refresh         |
| refresh-citizen-laboring  | world  | refresh         |
| refresh-citizen-bearing   | world  | refresh         |
| refresh-extractor-working | world  | refresh         |
| renew                     | world  | renew           |

## carries

From `spec/data/carries.4x`. 37 row(s), 2 column(s).

| kind      | trait       |
| --------- | ----------- |
| citizen   | bearing     |
| citizen   | laboring    |
| citizen   | paid        |
| citizen   | strength    |
| citizen   | upkeep      |
| extractor | binding     |
| extractor | metal-in-it |
| extractor | resource    |
| extractor | working     |
| yard      | binding     |
| yard      | metal-in-it |
| store     | binding     |
| store     | metal-in-it |
| store     | resource    |
| ark       | binding     |
| ark       | fuel        |
| ark       | metal-in-it |
| ark       | movable     |
| ark       | moving      |
| ark       | strength    |
| pioneer   | binding     |
| pioneer   | fuel        |
| pioneer   | metal-in-it |
| pioneer   | movable     |
| pioneer   | moving      |
| pioneer   | strength    |
| food      | surplus     |
| territory | id          |
| territory | control     |
| orbit     | id          |
| deposit   | density     |
| deposit   | occupied    |
| deposit   | free        |
| deposit   | capacity    |
| adjacency | from        |
| adjacency | to          |
| game      | phase       |

## constraint

From `spec/data/constraint.4x`. 18 row(s), 5 column(s).

| block                     | compare    | n   | seq | trait    |
| ------------------------- | ---------- | --- | --- | -------- |
| move                      | at-least   | 1   | 3   | moving   |
| move                      | one-less   |     | 4   | moving   |
| create-labor              | at-least   | 1   | 1   | laboring |
| create-labor              | one-less   |     | 2   | laboring |
| work                      | at-least   | 1   | 2   | working  |
| work                      | one-less   |     | 3   | working  |
| upkeep                    | at-maximum |     | 3   | paid     |
| bear                      | at-least   | 1   | 1   | bearing  |
| bear                      | one-less   |     | 2   | bearing  |
| perish                    | exactly    | 0   | 1   | paid     |
| age                       | at-least   | 1   | 1   | keeps    |
| age                       | one-less   |     | 2   | keeps    |
| spoil                     | exactly    | 0   | 1   | keeps    |
| refresh-unit-moving       | at-maximum |     | 1   | moving   |
| refresh-citizen-laboring  | at-maximum |     | 1   | laboring |
| refresh-citizen-bearing   | at-maximum |     | 1   | bearing  |
| refresh-extractor-working | at-maximum |     | 1   | working  |
| renew                     | exactly    | 0   | 2   | paid     |

## family

From `spec/data/families.4x`. 4 row(s), 1 column(s).

| name     |
| -------- |
| thing    |
| unit     |
| resource |
| place    |

## for

From `spec/data/for.4x`. 6 row(s), 3 column(s).

| block           | kind      | seq |
| --------------- | --------- | --- |
| deploy-ark      | food      | 4   |
| deploy-ark      | metal     | 5   |
| found-by-land   | food      | 3   |
| found-by-land   | metal     | 4   |
| build-extractor | $resource | 3   |
| build-store     | $resource | 3   |

## kind

From `spec/data/kinds.4x`. 20 row(s), 1 column(s).

| name      |
| --------- |
| kind      |
| trait     |
| family    |
| value     |
| citizen   |
| extractor |
| yard      |
| store     |
| ark       |
| pioneer   |
| food      |
| metal     |
| energy    |
| labor     |
| territory |
| orbit     |
| deposit   |
| adjacency |
| game      |
| fertility |

## limit

From `spec/data/limit.4x`. 4 row(s), 3 column(s).

| contained | container | n   |
| --------- | --------- | --- |
| yard      | territory | 1   |
| ark       | orbit     | 2   |
| pioneer   | territory | 2   |
| resource  | store     | 10  |

## line

From `spec/data/line.4x`. 68 row(s), 7 column(s).

| block                     | kind      | place-bound | qty                                  | role    | seq | place-above |
| ------------------------- | --------- | ----------- | ------------------------------------ | ------- | --- | ----------- |
| deploy-ark                | territory | where       | 1                                    | require | 1   |             |
| deploy-ark                | ark       |             | 1                                    | consume | 2   | where       |
| deploy-ark                | citizen   |             | 2                                    | produce | 3   |             |
| deploy-ark                | extractor |             | 1                                    | produce | 4   |             |
| deploy-ark                | extractor |             | 1                                    | produce | 5   |             |
| move                      | place     | from        | 1                                    | require | 1   |             |
| move                      | place     | to          | 1                                    | require | 2   |             |
| move                      | unit      | from        | 1                                    | require | 3   |             |
| move                      | unit      | to          |                                      | put     | 4   |             |
| move                      | energy    | from        | 1                                    | consume | 5   |             |
| found-by-land             | pioneer   |             | 1                                    | consume | 1   |             |
| found-by-land             | citizen   |             | 2                                    | produce | 2   |             |
| found-by-land             | extractor |             | 1                                    | produce | 3   |             |
| found-by-land             | extractor |             | 1                                    | produce | 4   |             |
| build-extractor           | labor     |             | 1                                    | consume | 1   |             |
| build-extractor           | metal     |             | 1                                    | consume | 2   |             |
| build-extractor           | extractor |             | 1                                    | produce | 3   |             |
| build-store               | labor     |             | 1                                    | consume | 1   |             |
| build-store               | metal     |             | 1                                    | consume | 2   |             |
| build-store               | store     |             | 1                                    | produce | 3   |             |
| build-yard                | labor     |             | 1                                    | consume | 1   |             |
| build-yard                | metal     |             | 15                                   | consume | 2   |             |
| build-yard                | yard      |             | 1                                    | produce | 3   |             |
| produce-pioneer           | metal     |             | 3                                    | consume | 1   |             |
| produce-pioneer           | energy    |             | 2                                    | consume | 2   |             |
| produce-pioneer           | citizen   |             | 2                                    | consume | 3   |             |
| produce-pioneer           | pioneer   |             | 1                                    | produce | 4   |             |
| launch-ark                | territory | where       | 1                                    | require | 1   |             |
| launch-ark                | metal     |             | 3                                    | consume | 2   |             |
| launch-ark                | energy    |             | 12                                   | consume | 3   |             |
| launch-ark                | citizen   |             | 2                                    | consume | 4   |             |
| launch-ark                | yard      |             | 1                                    | require | 5   |             |
| launch-ark                | ark       |             | 1                                    | produce | 6   | where       |
| create-labor              | citizen   |             | 1                                    | require | 1   |             |
| create-labor              | citizen   |             |                                      | put     | 2   |             |
| create-labor              | labor     |             | 1                                    | produce | 3   |             |
| work                      | territory | where       | 1                                    | require | 1   |             |
| work                      | extractor |             | 1                                    | require | 2   |             |
| work                      | extractor |             |                                      | put     | 3   |             |
| work                      | labor     |             | 1                                    | consume | 4   |             |
| work                      | resource  |             | `$where`'s density for that resource | produce | 5   |             |
| upkeep                    | citizen   |             | 1                                    | require | 1   |             |
| upkeep                    | food      |             | 1                                    | consume | 2   |             |
| upkeep                    | citizen   |             |                                      | put     | 3   |             |
| bear                      | citizen   |             | 1                                    | require | 1   |             |
| bear                      | citizen   |             |                                      | put     | 2   |             |
| bear                      | fertility |             | 1                                    | produce | 3   |             |
| breed                     | fertility |             | 1                                    | consume | 1   |             |
| breed                     | food      |             | 1                                    | consume | 2   |             |
| breed                     | citizen   |             | 1                                    | produce | 3   |             |
| perish                    | citizen   |             | 1                                    | consume | 1   |             |
| age                       | thing     |             | 1                                    | require | 1   |             |
| age                       | thing     |             |                                      | put     | 2   |             |
| spoil                     | thing     |             | 1                                    | consume | 1   |             |
| stow-metal                | metal     |             | 1                                    | consume | 1   |             |
| stow-metal                | metal     |             | 1                                    | produce | 2   |             |
| stow-energy               | energy    |             | 1                                    | consume | 1   |             |
| stow-energy               | energy    |             | 1                                    | produce | 2   |             |
| discard-metal             | metal     |             | 1                                    | consume | 1   |             |
| discard-energy            | energy    |             | 1                                    | consume | 1   |             |
| discard-labor             | labor     |             | 1                                    | consume | 1   |             |
| discard-fertility         | fertility |             | 1                                    | consume | 1   |             |
| refresh-unit-moving       | unit      |             |                                      | put     | 1   |             |
| refresh-citizen-laboring  | citizen   |             |                                      | put     | 1   |             |
| refresh-citizen-bearing   | citizen   |             |                                      | put     | 1   |             |
| refresh-extractor-working | extractor |             |                                      | put     | 1   |             |
| renew                     | citizen   |             | 1                                    | require | 1   |             |
| renew                     | citizen   |             |                                      | put     | 2   |             |

## member

From `spec/data/member.4x`. 7 row(s), 2 column(s).

| family   | kind      |
| -------- | --------- |
| unit     | ark       |
| unit     | pioneer   |
| resource | food      |
| resource | metal     |
| resource | energy    |
| place    | territory |
| place    | orbit     |

## trait

From `spec/data/traits.4x`. 24 row(s), 4 column(s).

| admits   | kept  | name        | of    |
| -------- | ----- | ----------- | ----- |
| identity | thing | id          |       |
| number   | thing | moving      |       |
| number   | thing | laboring    |       |
| number   | thing | working     |       |
| number   | thing | bearing     |       |
| resource | thing | resource    |       |
| number   | kind  | strength    |       |
| number   | kind  | fuel        |       |
| number   | kind  | upkeep      |       |
| number   | kind  | binding     |       |
| number   | kind  | metal-in-it |       |
| number   | thing | density     |       |
| number   | thing | capacity    |       |
| number   | thing | occupied    |       |
| number   | thing | free        |       |
| number   | thing | control     |       |
| place    | thing | from        |       |
| place    | thing | to          |       |
| number   | thing | keeps       | thing |
| number   | kind  | surplus     |       |
| number   | thing | paid        |       |
| value    | thing | phase       |       |
| number   | kind  | movable     |       |
| value    | thing | biome       |       |

