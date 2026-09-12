# Every turn of `scenario/commands/play.4x`

**Generated. Do not edit.** One section per `end turn` in the scenario - 10 of them.
The turn numbers are the scenario's own boundaries, so they line up with its comments.

# Turn 1

## commands

```
{deploy-ark territory:1}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:metal}
{end-turn}
```

## what changed

**gone** (1)

- {orbit id:1} {ark defending:1 id:1 moving:1} -> 1

**new** (4)

- {territory biome:grassland id:1 nature:1} {citizen bearing:1 defending:1 laboring:1} -> 4
- {territory biome:grassland id:1 nature:1} {extractor resource:food working:1} -> 1
- {territory biome:grassland id:1 nature:1} {extractor resource:metal working:1} -> 1
- {territory biome:grassland id:1 nature:1} {garrison} -> 1

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 4        | 0           | 0     |
| 2   | grassland | 1      | 0        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 1     |
| 1         | metal    | 3        | 4       | 1     |
| 1         | energy   | 3        | 4       | 0     |
| 2         | food     | 2        | 6       | 0     |
| 2         | metal    | 2        | 4       | 0     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 0      |
| 1         | energy   | 0      |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |

1 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |

2 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 2     |
| 1         | garrison  | 1     |
| 1         | yard      | 0     |
| 2         | extractor | 0     |
| 2         | garrison  | 0     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 4    | 0     | 4    |
| 2         | 0    | 0     | 0    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind | in-kind | in-id | fuel | ready |
| --- | ---- | ------- | ----- | ---- | ----- |

*(empty) 0 rows*

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 4       |
| labor     | 0       |
| food      | 0       |
| metal     | 0       |
| energy    | 0       |
| extractor | 2       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 2

## commands

```
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{build-extractor territory:1 resource:food}
{create-labor territory:1}
{build-store territory:1 resource:food}
{end-turn}
```

## what changed

**new** (1)

- {territory biome:grassland id:1 nature:1} {store resource:food} -> 1

**changed** (1)

- {territory biome:grassland id:1 nature:1} {extractor resource:food working:1} · 1 → 2

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 4        | 0           | 0     |
| 2   | grassland | 1      | 0        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 2     |
| 1         | metal    | 3        | 4       | 1     |
| 1         | energy   | 3        | 4       | 0     |
| 2         | food     | 2        | 6       | 0     |
| 2         | metal    | 2        | 4       | 0     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 0      |
| 1         | energy   | 0      |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |

1 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |

3 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 3     |
| 1         | garrison  | 1     |
| 1         | yard      | 0     |
| 2         | extractor | 0     |
| 2         | garrison  | 0     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 4    | 0     | 4    |
| 2         | 0    | 0     | 0    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind | in-kind | in-id | fuel | ready |
| --- | ---- | ------- | ----- | ---- | ----- |

*(empty) 0 rows*

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 4       |
| labor     | 0       |
| food      | 0       |
| metal     | 0       |
| energy    | 0       |
| extractor | 3       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 3

## commands

```
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{build-store territory:1 resource:metal}
{end-turn}
```

## what changed

**new** (2)

- {territory biome:grassland id:1 nature:1} {metal} -> 3
- {territory biome:grassland id:1 nature:1} {store resource:metal} -> 1

**changed** (1)

- {territory biome:grassland id:1 nature:1} {citizen bearing:1 defending:1 laboring:1} · 4 → 8

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 8        | 0           | 0     |
| 2   | grassland | 1      | 0        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 2     |
| 1         | metal    | 3        | 4       | 1     |
| 1         | energy   | 3        | 4       | 0     |
| 2         | food     | 2        | 6       | 0     |
| 2         | metal    | 2        | 4       | 0     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 3      |
| 1         | energy   | 0      |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |

1 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |

3 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 3     |
| 1         | garrison  | 1     |
| 1         | yard      | 0     |
| 2         | extractor | 0     |
| 2         | garrison  | 0     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 8    | 0     | 8    |
| 2         | 0    | 0     | 0    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind | in-kind | in-id | fuel | ready |
| --- | ---- | ------- | ----- | ---- | ----- |

*(empty) 0 rows*

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 8       |
| labor     | 0       |
| food      | 0       |
| metal     | 3       |
| energy    | 0       |
| extractor | 3       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 4

## commands

```
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{build-extractor territory:1 resource:metal}
{create-labor territory:1}
{build-extractor territory:1 resource:food}
{create-labor territory:1}
{build-extractor territory:1 resource:energy}
{create-labor territory:1}
{build-store territory:1 resource:energy}
{end-turn}
```

## what changed

**new** (2)

- {territory biome:grassland id:1 nature:1} {extractor resource:energy working:1} -> 1
- {territory biome:grassland id:1 nature:1} {store resource:energy} -> 1

**changed** (2)

- {territory biome:grassland id:1 nature:1} {extractor resource:food working:1} · 2 → 3
- {territory biome:grassland id:1 nature:1} {extractor resource:metal working:1} · 1 → 2

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 8        | 0           | 0     |
| 2   | grassland | 1      | 0        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
| 1         | metal    | 3        | 4       | 2     |
| 1         | energy   | 3        | 4       | 1     |
| 2         | food     | 2        | 6       | 0     |
| 2         | metal    | 2        | 4       | 0     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 3      |
| 1         | energy   | 0      |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |

1 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | energy   | yes   |

6 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 6     |
| 1         | garrison  | 1     |
| 1         | yard      | 0     |
| 2         | extractor | 0     |
| 2         | garrison  | 0     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 8    | 0     | 8    |
| 2         | 0    | 0     | 0    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind | in-kind | in-id | fuel | ready |
| --- | ---- | ------- | ----- | ---- | ----- |

*(empty) 0 rows*

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 8       |
| labor     | 0       |
| food      | 0       |
| metal     | 3       |
| energy    | 0       |
| extractor | 6       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 5

## commands

```
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{work territory:1 resource:energy}
{create-labor territory:1}
{build-extractor territory:1 resource:metal}
{create-labor territory:1}
{build-extractor territory:1 resource:energy}
{end-turn}
```

## what changed

**new** (1)

- {territory biome:grassland id:1 nature:1} {energy} -> 4

**changed** (4)

- {territory biome:grassland id:1 nature:1} {citizen bearing:1 defending:1 laboring:1} · 8 → 12
- {territory biome:grassland id:1 nature:1} {extractor resource:energy working:1} · 1 → 2
- {territory biome:grassland id:1 nature:1} {extractor resource:metal working:1} · 2 → 3
- {territory biome:grassland id:1 nature:1} {metal} · 3 → 9

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 12       | 0           | 0     |
| 2   | grassland | 1      | 0        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
| 1         | metal    | 3        | 4       | 3     |
| 1         | energy   | 3        | 4       | 2     |
| 2         | food     | 2        | 6       | 0     |
| 2         | metal    | 2        | 4       | 0     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 9      |
| 1         | energy   | 4      |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |

1 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | energy   | yes   |
| 1         | metal    | yes   |
| 1         | energy   | yes   |

8 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 8     |
| 1         | garrison  | 1     |
| 1         | yard      | 0     |
| 2         | extractor | 0     |
| 2         | garrison  | 0     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 12   | 0     | 12   |
| 2         | 0    | 0     | 0    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind | in-kind | in-id | fuel | ready |
| --- | ---- | ------- | ----- | ---- | ----- |

*(empty) 0 rows*

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 12      |
| labor     | 0       |
| food      | 0       |
| metal     | 9       |
| energy    | 4       |
| extractor | 8       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 6

## commands

```
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{work territory:1 resource:energy}
{create-labor territory:1}
{work territory:1 resource:energy}
{create-labor territory:1}
{build-extractor territory:1 resource:energy}
{create-labor territory:1}
{build-store territory:1 resource:energy}
{end-turn}
```

## what changed

**changed** (4)

- {territory biome:grassland id:1 nature:1} {energy} · 4 → 12
- {territory biome:grassland id:1 nature:1} {extractor resource:energy working:1} · 2 → 3
- {territory biome:grassland id:1 nature:1} {metal} · 9 → 10
- {territory biome:grassland id:1 nature:1} {store resource:energy} · 1 → 2

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 12       | 0           | 0     |
| 2   | grassland | 1      | 0        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
| 1         | metal    | 3        | 4       | 3     |
| 1         | energy   | 3        | 4       | 3     |
| 2         | food     | 2        | 6       | 0     |
| 2         | metal    | 2        | 4       | 0     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 10     |
| 1         | energy   | 12     |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |

1 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | energy   | yes   |
| 1         | metal    | yes   |
| 1         | energy   | yes   |
| 1         | energy   | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 9     |
| 1         | garrison  | 1     |
| 1         | yard      | 0     |
| 2         | extractor | 0     |
| 2         | garrison  | 0     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 12   | 0     | 12   |
| 2         | 0    | 0     | 0    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind | in-kind | in-id | fuel | ready |
| --- | ---- | ------- | ----- | ---- | ----- |

*(empty) 0 rows*

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 12      |
| labor     | 0       |
| food      | 0       |
| metal     | 10      |
| energy    | 12      |
| extractor | 9       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 7

## commands

```
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{work territory:1 resource:metal}
{create-labor territory:1}
{work territory:1 resource:energy}
{create-labor territory:1}
{work territory:1 resource:energy}
{create-labor territory:1}
{work territory:1 resource:energy}
{create-labor territory:1}
{build-store territory:1 resource:metal}
{create-labor territory:1}
{produce-pioneer territory:1}
{end-turn}
```

## what changed

**new** (1)

- {territory biome:grassland id:1 nature:1} {pioneer defending:1 id:1 moving:1} -> 1

**changed** (3)

- {territory biome:grassland id:1 nature:1} {energy} · 12 → 18
- {territory biome:grassland id:1 nature:1} {metal} · 10 → 18
- {territory biome:grassland id:1 nature:1} {store resource:metal} · 1 → 2

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 1     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 12       | 0           | 0     |
| 2   | grassland | 1      | 0        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
| 1         | metal    | 3        | 4       | 3     |
| 1         | energy   | 3        | 4       | 3     |
| 2         | food     | 2        | 6       | 0     |
| 2         | metal    | 2        | 4       | 0     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 18     |
| 1         | energy   | 18     |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |

1 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | energy   | yes   |
| 1         | metal    | yes   |
| 1         | energy   | yes   |
| 1         | energy   | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 9     |
| 1         | garrison  | 1     |
| 1         | yard      | 0     |
| 2         | extractor | 0     |
| 2         | garrison  | 0     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 12   | 0     | 12   |
| 2         | 0    | 0     | 0    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind    | in-kind   | in-id | fuel | ready |
| --- | ------- | --------- | ----- | ---- | ----- |
| 1   | pioneer | territory | 1     | 2    | yes   |

1 row(s)

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 12      |
| labor     | 0       |
| food      | 0       |
| metal     | 18      |
| energy    | 18      |
| extractor | 9       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 1       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 8

## commands

```
{move unit:pioneer territory:2}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:energy}
{create-labor territory:1}
{build-yard territory:1}
{end-turn}
```

## what changed

**gone** (1)

- {territory biome:grassland id:1 nature:1} {pioneer defending:1 id:1 moving:1} -> 1

**new** (2)

- {territory biome:grassland id:1 nature:1} {yard} -> 1
- {territory biome:grassland id:2 nature:1} {pioneer defending:1 id:1 moving:1} -> 1

**changed** (2)

- {territory biome:grassland id:1 nature:1} {energy} · 18 → 20
- {territory biome:grassland id:1 nature:1} {metal} · 18 → 3

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 1     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 12       | 0           | 1     |
| 2   | grassland | 1      | 0        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
| 1         | metal    | 3        | 4       | 3     |
| 1         | energy   | 3        | 4       | 3     |
| 2         | food     | 2        | 6       | 0     |
| 2         | metal    | 2        | 4       | 0     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 3      |
| 1         | energy   | 20     |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |

1 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | energy   | yes   |
| 1         | metal    | yes   |
| 1         | energy   | yes   |
| 1         | energy   | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 9     |
| 1         | garrison  | 1     |
| 1         | yard      | 1     |
| 2         | extractor | 0     |
| 2         | garrison  | 0     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 12   | 0     | 12   |
| 2         | 0    | 0     | 0    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind    | in-kind   | in-id | fuel | ready |
| --- | ------- | --------- | ----- | ---- | ----- |
| 1   | pioneer | territory | 2     | 1    | yes   |

1 row(s)

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 12      |
| labor     | 0       |
| food      | 0       |
| metal     | 3       |
| energy    | 20      |
| extractor | 9       |
| garrison  | 1       |
| yard      | 1       |
| ark       | 0       |
| pioneer   | 1       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 9

## commands

```
{found-by-land territory:2}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:energy}
{create-labor territory:1}
{launch-ark territory:1}
{create-labor territory:2}
{work territory:2 resource:food}
{end-turn}
```

## what changed

**gone** (2)

- {territory biome:grassland id:1 nature:1} {metal} -> 3
- {territory biome:grassland id:2 nature:1} {pioneer defending:1 id:1 moving:1} -> 1

**new** (4)

- {territory biome:grassland id:2 nature:1} {citizen bearing:1 defending:1 laboring:1} -> 4
- {territory biome:grassland id:2 nature:1} {extractor resource:food working:1} -> 1
- {territory biome:grassland id:2 nature:1} {extractor resource:metal working:1} -> 1
- {territory biome:grassland id:2 nature:1} {garrison} -> 1

**changed** (1)

- {territory biome:grassland id:1 nature:1} {energy} · 20 → 12

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 12       | 0           | 1     |
| 2   | grassland | 1      | 4        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
| 1         | metal    | 3        | 4       | 3     |
| 1         | energy   | 3        | 4       | 3     |
| 2         | food     | 2        | 6       | 1     |
| 2         | metal    | 2        | 4       | 1     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 0      |
| 1         | energy   | 12     |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |
| 2         | 0     |

2 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | energy   | yes   |
| 1         | metal    | yes   |
| 1         | energy   | yes   |
| 1         | energy   | yes   |
| 2         | food     | yes   |
| 2         | metal    | yes   |

11 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 9     |
| 1         | garrison  | 1     |
| 1         | yard      | 1     |
| 2         | extractor | 2     |
| 2         | garrison  | 1     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 12   | 0     | 12   |
| 2         | 4    | 0     | 4    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind | in-kind | in-id | fuel | ready |
| --- | ---- | ------- | ----- | ---- | ----- |

*(empty) 0 rows*

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 16      |
| labor     | 0       |
| food      | 0       |
| metal     | 0       |
| energy    | 12      |
| extractor | 11      |
| garrison  | 2       |
| yard      | 1       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

# Turn 10

## commands

```
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:1}
{work territory:1 resource:food}
{create-labor territory:2}
{work territory:2 resource:food}
{end-turn}
```

## what changed

**changed** (2)

- {territory biome:grassland id:1 nature:1} {citizen bearing:1 defending:1 laboring:1} · 12 → 8
- {territory biome:grassland id:2 nature:1} {citizen bearing:1 defending:1 laboring:1} · 4 → 6

## what is there now

### game

| phase | territories | units |
| ----- | ----------- | ----- |
| play  | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 8        | 0           | 1     |
| 2   | grassland | 1      | 6        | 0           | 0     |
| 3   | grassland | 1      | 0        | 0           | 0     |
| 4   | mountain  | 1      | 0        | 0           | 0     |
| 5   | mountain  | 1      | 0        | 0           | 0     |
| 6   | jungle    | 2      | 0        | 0           | 0     |
| 7   | jungle    | 2      | 0        | 0           | 0     |
| 8   | grassland | 1      | 0        | 0           | 0     |
| 9   | mountain  | 1      | 0        | 0           | 0     |
| 10  | desert    | 1      | 0        | 0           | 0     |
| 11  | grassland | 1      | 0        | 0           | 0     |
| 12  | ice       | 1      | 0        | 0           | 0     |

12 row(s)

### deposit

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
| 1         | metal    | 3        | 4       | 3     |
| 1         | energy   | 3        | 4       | 3     |
| 2         | food     | 2        | 6       | 1     |
| 2         | metal    | 2        | 4       | 1     |
| 2         | energy   | 2        | 4       | 0     |
| 3         | food     | 6        | 2       | 0     |
| 3         | metal    | 2        | 4       | 0     |
| 3         | energy   | 2        | 4       | 0     |
| 4         | food     | 1        | 2       | 0     |
| 4         | metal    | 4        | 5       | 0     |
| 4         | energy   | 4        | 5       | 0     |
| 5         | food     | 3        | 1       | 0     |
| 5         | metal    | 8        | 8       | 0     |
| 5         | energy   | 8        | 8       | 0     |
| 6         | food     | 4        | 4       | 0     |
| 6         | metal    | 0        | 0       | 0     |
| 6         | energy   | 4        | 5       | 0     |
| 7         | food     | 4        | 4       | 0     |
| 7         | metal    | 4        | 5       | 0     |
| 7         | energy   | 0        | 0       | 0     |
| 8         | food     | 6        | 6       | 0     |
| 8         | metal    | 1        | 2       | 0     |
| 8         | energy   | 1        | 2       | 0     |
| 9         | food     | 2        | 3       | 0     |
| 9         | metal    | 6        | 8       | 0     |
| 9         | energy   | 1        | 2       | 0     |
| 10        | food     | 3        | 3       | 0     |
| 10        | metal    | 1        | 3       | 0     |
| 10        | energy   | 6        | 8       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 5        | 6       | 0     |
| 11        | energy   | 5        | 6       | 0     |
| 12        | food     | 2        | 2       | 0     |
| 12        | metal    | 8        | 8       | 0     |
| 12        | energy   | 8        | 8       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 0      |
| 1         | energy   | 12     |
| 2         | food     | 0      |
| 2         | metal    | 0      |
| 2         | energy   | 0      |
| 3         | food     | 0      |
| 3         | metal    | 0      |
| 3         | energy   | 0      |
| 4         | food     | 0      |
| 4         | metal    | 0      |
| 4         | energy   | 0      |
| 5         | food     | 0      |
| 5         | metal    | 0      |
| 5         | energy   | 0      |
| 6         | food     | 0      |
| 6         | metal    | 0      |
| 6         | energy   | 0      |
| 7         | food     | 0      |
| 7         | metal    | 0      |
| 7         | energy   | 0      |
| 8         | food     | 0      |
| 8         | metal    | 0      |
| 8         | energy   | 0      |
| 9         | food     | 0      |
| 9         | metal    | 0      |
| 9         | energy   | 0      |
| 10        | food     | 0      |
| 10        | metal    | 0      |
| 10        | energy   | 0      |
| 11        | food     | 0      |
| 11        | metal    | 0      |
| 11        | energy   | 0      |
| 12        | food     | 0      |
| 12        | metal    | 0      |
| 12        | energy   | 0      |

36 row(s)

### garrison

| territory | force |
| --------- | ----- |
| 1         | 0     |
| 2         | 0     |

2 row(s)

### extractor

| territory | resource | ready |
| --------- | -------- | ----- |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | metal    | yes   |
| 1         | food     | yes   |
| 1         | energy   | yes   |
| 1         | metal    | yes   |
| 1         | energy   | yes   |
| 1         | energy   | yes   |
| 2         | food     | yes   |
| 2         | metal    | yes   |

11 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 9     |
| 1         | garrison  | 1     |
| 1         | yard      | 1     |
| 2         | extractor | 2     |
| 2         | garrison  | 1     |
| 2         | yard      | 0     |
| 3         | extractor | 0     |
| 3         | garrison  | 0     |
| 3         | yard      | 0     |
| 4         | extractor | 0     |
| 4         | garrison  | 0     |
| 4         | yard      | 0     |
| 5         | extractor | 0     |
| 5         | garrison  | 0     |
| 5         | yard      | 0     |
| 6         | extractor | 0     |
| 6         | garrison  | 0     |
| 6         | yard      | 0     |
| 7         | extractor | 0     |
| 7         | garrison  | 0     |
| 7         | yard      | 0     |
| 8         | extractor | 0     |
| 8         | garrison  | 0     |
| 8         | yard      | 0     |
| 9         | extractor | 0     |
| 9         | garrison  | 0     |
| 9         | yard      | 0     |
| 10        | extractor | 0     |
| 10        | garrison  | 0     |
| 10        | yard      | 0     |
| 11        | extractor | 0     |
| 11        | garrison  | 0     |
| 11        | yard      | 0     |
| 12        | extractor | 0     |
| 12        | garrison  | 0     |
| 12        | yard      | 0     |

36 row(s)

### labor

| territory | made | spent | left |
| --------- | ---- | ----- | ---- |
| 1         | 8    | 0     | 8    |
| 2         | 6    | 0     | 6    |
| 3         | 0    | 0     | 0    |
| 4         | 0    | 0     | 0    |
| 5         | 0    | 0     | 0    |
| 6         | 0    | 0     | 0    |
| 7         | 0    | 0     | 0    |
| 8         | 0    | 0     | 0    |
| 9         | 0    | 0     | 0    |
| 10        | 0    | 0     | 0    |
| 11        | 0    | 0     | 0    |
| 12        | 0    | 0     | 0    |

12 row(s)

### unit

| id  | kind | in-kind | in-id | fuel | ready |
| --- | ---- | ------- | ----- | ---- | ----- |

*(empty) 0 rows*

### adjacency

| from | to  |
| ---- | --- |
| 1    | 2   |
| 1    | 3   |
| 1    | 4   |
| 1    | 5   |
| 1    | 6   |
| 2    | 3   |
| 2    | 4   |
| 2    | 7   |
| 2    | 8   |
| 3    | 5   |
| 3    | 7   |
| 3    | 9   |
| 4    | 6   |
| 4    | 8   |
| 4    | 10  |
| 5    | 6   |
| 5    | 9   |
| 5    | 12  |
| 6    | 10  |
| 6    | 12  |
| 7    | 8   |
| 7    | 9   |
| 7    | 11  |
| 8    | 10  |
| 8    | 11  |
| 9    | 11  |
| 9    | 12  |
| 10   | 11  |
| 10   | 12  |
| 11   | 12  |

30 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 14      |
| labor     | 0       |
| food      | 0       |
| metal     | 0       |
| energy    | 12      |
| extractor | 11      |
| garrison  | 2       |
| yard      | 1       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |
| adjacency | 30      |
| orbit     | 12      |
| deposit   | 34      |
| fertility | 0       |
| force     | 0       |

16 row(s)

