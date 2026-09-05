# Every turn of `scenario/commands/play.4x`

**Generated. Do not edit.** One section per `end turn` in the scenario - 9 of them.
The turn numbers are the scenario's own boundaries, so they line up with its comments.

# Turn 1

## commands

```
land ark 1
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
end turn
```

## what changed

**gone** (1)

- {unit id:1 kind:ark place:orbit fuel:2 ready:yes}

**new** (3)

- {extractor territory:1 node:0 resource:food ready:yes}
- {extractor territory:1 node:3 resource:metal ready:yes}
- {garrison territory:1 force:1}

**changed** (15)

- game phase:play · turn: 1 → 2
- game phase:play · units: 1 → 0
- kind id:ark · in-play: 1 → 0
- kind id:citizen · in-play: 0 → 4
- kind id:extractor · in-play: 0 → 2
- kind id:garrison · in-play: 0 → 1
- kind id:metal · in-play: 0 → 4
- labor territory:1 · made: 0 → 4
- labor territory:1 · left: 0 → 4
- store territory:1 resource:metal · amount: 0 → 4
- structure territory:1 structure:extractor · count: 0 → 2
- structure territory:1 structure:garrison · count: 0 → 1
- territory id:1 · citizens: 0 → 4
- territory-resource territory:1 resource:food · built: 0 → 1
- territory-resource territory:1 resource:metal · built: 0 → 1

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 2    | 12          | 0     |

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

### territory-resource

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
| 1         | metal    | 4      |
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
| 1         | 1     |

1 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |

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

| id  | kind | place | fuel | ready |
| --- | ---- | ----- | ---- | ----- |

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 4       |
| labor     | 0       |
| food      | 0       |
| metal     | 4       |
| energy    | 0       |
| extractor | 2       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 2

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
build extractor 1 food
create labor 1 1
build extractor 1 food
end turn
```

## what changed

**new** (2)

- {extractor territory:1 node:1 resource:food ready:yes}
- {extractor territory:1 node:2 resource:food ready:yes}

**changed** (6)

- game phase:play · turn: 2 → 3
- kind id:extractor · in-play: 2 → 4
- kind id:metal · in-play: 4 → 6
- store territory:1 resource:metal · amount: 4 → 6
- structure territory:1 structure:extractor · count: 2 → 4
- territory-resource territory:1 resource:food · built: 1 → 3

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 3    | 12          | 0     |

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

### territory-resource

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
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
| 1         | metal    | 6      |
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
| 1         | 1     |

1 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |

4 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 4     |
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

| id  | kind | place | fuel | ready |
| --- | ---- | ----- | ---- | ----- |

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 4       |
| labor     | 0       |
| food      | 0       |
| metal     | 6       |
| energy    | 0       |
| extractor | 4       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 3

## commands

```
create labor 3 1
work 3 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
end turn
```

## what changed

**changed** (7)

- game phase:play · turn: 3 → 4
- kind id:citizen · in-play: 4 → 8
- kind id:metal · in-play: 6 → 10
- labor territory:1 · made: 4 → 8
- labor territory:1 · left: 4 → 8
- store territory:1 resource:metal · amount: 6 → 10
- territory id:1 · citizens: 4 → 8

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 4    | 12          | 0     |

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

### territory-resource

| territory | resource | capacity | density | built |
| --------- | -------- | -------- | ------- | ----- |
| 1         | food     | 3        | 4       | 3     |
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
| 1         | metal    | 10     |
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
| 1         | 1     |

1 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |

4 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 4     |
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

| id  | kind | place | fuel | ready |
| --- | ---- | ----- | ---- | ----- |

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 8       |
| labor     | 0       |
| food      | 0       |
| metal     | 10      |
| energy    | 0       |
| extractor | 4       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 4

## commands

```
create labor 3 1
work 3 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
build extractor 1 metal
create labor 1 1
build extractor 1 metal
create labor 1 1
build extractor 1 energy
create labor 1 1
build extractor 1 energy
end turn
```

## what changed

**new** (4)

- {extractor territory:1 node:4 resource:metal ready:yes}
- {extractor territory:1 node:5 resource:metal ready:yes}
- {extractor territory:1 node:6 resource:energy ready:yes}
- {extractor territory:1 node:7 resource:energy ready:yes}

**changed** (9)

- game phase:play · turn: 4 → 5
- kind id:citizen · in-play: 8 → 12
- kind id:extractor · in-play: 4 → 8
- labor territory:1 · made: 8 → 12
- labor territory:1 · left: 8 → 12
- structure territory:1 structure:extractor · count: 4 → 8
- territory id:1 · citizens: 8 → 12
- territory-resource territory:1 resource:energy · built: 0 → 2
- territory-resource territory:1 resource:metal · built: 1 → 3

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 5    | 12          | 0     |

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

### territory-resource

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
| 1         | metal    | 10     |
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
| 1         | 1     |

1 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 4    | metal    | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 6    | energy   | yes   |
| 1         | 7    | energy   | yes   |

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

| id  | kind | place | fuel | ready |
| --- | ---- | ----- | ---- | ----- |

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 12      |
| labor     | 0       |
| food      | 0       |
| metal     | 10      |
| energy    | 0       |
| extractor | 8       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 5

## commands

```
create labor 3 1
work 3 extractor 1 food
create labor 3 1
work 3 extractor 1 metal
create labor 2 1
work 2 extractor 1 energy
create labor 1 1
build extractor 1 energy
end turn
```

## what changed

**new** (1)

- {extractor territory:1 node:8 resource:energy ready:yes}

**changed** (8)

- game phase:play · turn: 5 → 6
- kind id:energy · in-play: 0 → 8
- kind id:extractor · in-play: 8 → 9
- kind id:metal · in-play: 10 → 20
- store territory:1 resource:energy · amount: 0 → 8
- store territory:1 resource:metal · amount: 10 → 20
- structure territory:1 structure:extractor · count: 8 → 9
- territory-resource territory:1 resource:energy · built: 2 → 3

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 6    | 12          | 0     |

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

### territory-resource

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
| 1         | metal    | 20     |
| 1         | energy   | 8      |
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
| 1         | 1     |

1 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 4    | metal    | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 6    | energy   | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 8    | energy   | yes   |

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

| id  | kind | place | fuel | ready |
| --- | ---- | ----- | ---- | ----- |

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 12      |
| labor     | 0       |
| food      | 0       |
| metal     | 20      |
| energy    | 8       |
| extractor | 9       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 6

## commands

```
create labor 3 1
work 3 extractor 1 food
create labor 3 1
work 3 extractor 1 metal
create labor 3 1
work 3 extractor 1 energy
produce pioneer 1
end turn
```

## what changed

**new** (1)

- {unit id:1 kind:pioneer place:territory-1 fuel:2 ready:yes}

**changed** (9)

- game phase:play · turn: 6 → 7
- game phase:play · units: 0 → 1
- kind id:citizen · in-play: 12 → 11
- kind id:energy · in-play: 8 → 14
- kind id:pioneer · in-play: 0 → 1
- labor territory:1 · made: 12 → 11
- labor territory:1 · left: 12 → 11
- store territory:1 resource:energy · amount: 8 → 14
- territory id:1 · citizens: 12 → 11

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 7    | 12          | 1     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 11       | 0           | 0     |
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

### territory-resource

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
| 1         | metal    | 20     |
| 1         | energy   | 14     |
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
| 1         | 1     |

1 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 4    | metal    | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 6    | energy   | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 8    | energy   | yes   |

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
| 1         | 11   | 0     | 11   |
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

| id  | kind    | place       | fuel | ready |
| --- | ------- | ----------- | ---- | ----- |
| 1   | pioneer | territory-1 | 2    | yes   |

1 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 11      |
| labor     | 0       |
| food      | 0       |
| metal     | 20      |
| energy    | 14      |
| extractor | 9       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 1       |
| territory | 12      |

11 row(s)

# Turn 7

## commands

```
create labor 3 1
work 3 extractor 1 food
create labor 3 1
work 3 extractor 1 metal
create labor 3 1
work 3 extractor 1 energy
found by land 2
create labor 1 2
work 1 extractor 2 food
end turn
```

## what changed

**gone** (1)

- {unit id:1 kind:pioneer place:territory-1 fuel:2 ready:yes}

**new** (3)

- {extractor territory:2 node:0 resource:food ready:yes}
- {extractor territory:2 node:2 resource:metal ready:yes}
- {garrison territory:2 force:1}

**changed** (18)

- game phase:play · turn: 7 → 8
- game phase:play · units: 1 → 0
- kind id:citizen · in-play: 11 → 16
- kind id:energy · in-play: 14 → 20
- kind id:extractor · in-play: 9 → 11
- kind id:garrison · in-play: 1 → 2
- kind id:pioneer · in-play: 1 → 0
- labor territory:1 · made: 11 → 12
- labor territory:1 · left: 11 → 12
- labor territory:2 · made: 0 → 4
- labor territory:2 · left: 0 → 4
- store territory:1 resource:energy · amount: 14 → 20
- structure territory:2 structure:extractor · count: 0 → 2
- structure territory:2 structure:garrison · count: 0 → 1
- territory id:1 · citizens: 11 → 12
- territory id:2 · citizens: 0 → 4
- territory-resource territory:2 resource:food · built: 0 → 1
- territory-resource territory:2 resource:metal · built: 0 → 1

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 8    | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 12       | 0           | 0     |
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

### territory-resource

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
| 1         | metal    | 20     |
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
| 1         | 1     |
| 2         | 1     |

2 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 4    | metal    | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 6    | energy   | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 8    | energy   | yes   |
| 2         | 0    | food     | yes   |
| 2         | 2    | metal    | yes   |

11 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 9     |
| 1         | garrison  | 1     |
| 1         | yard      | 0     |
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

| id  | kind | place | fuel | ready |
| --- | ---- | ----- | ---- | ----- |

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 16      |
| labor     | 0       |
| food      | 0       |
| metal     | 20      |
| energy    | 20      |
| extractor | 11      |
| garrison  | 2       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 8

## commands

```
create labor 3 1
work 3 extractor 1 food
create labor 3 1
work 3 extractor 1 metal
create labor 3 1
work 3 extractor 1 energy
create labor 1 2
work 1 extractor 2 food
create labor 1 1
build yard 1
produce ark 1
move ark 2
end turn
```

## what changed

**new** (1)

- {unit id:1 kind:ark place:territory-2 fuel:1 ready:yes}

**changed** (12)

- game phase:play · turn: 8 → 9
- game phase:play · units: 0 → 1
- kind id:ark · in-play: 0 → 1
- kind id:citizen · in-play: 16 → 18
- kind id:metal · in-play: 20 → 14
- kind id:yard · in-play: 0 → 1
- labor territory:2 · made: 4 → 6
- labor territory:2 · left: 4 → 6
- store territory:1 resource:metal · amount: 20 → 14
- structure territory:1 structure:yard · count: 0 → 1
- territory id:1 · yards: 0 → 1
- territory id:2 · citizens: 4 → 6

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 9    | 12          | 1     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 12       | 0           | 1     |
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

### territory-resource

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
| 1         | metal    | 14     |
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
| 1         | 1     |
| 2         | 1     |

2 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 4    | metal    | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 6    | energy   | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 8    | energy   | yes   |
| 2         | 0    | food     | yes   |
| 2         | 2    | metal    | yes   |

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

| id  | kind | place       | fuel | ready |
| --- | ---- | ----------- | ---- | ----- |
| 1   | ark  | territory-2 | 1    | yes   |

1 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 18      |
| labor     | 0       |
| food      | 0       |
| metal     | 14      |
| energy    | 20      |
| extractor | 11      |
| garrison  | 2       |
| yard      | 1       |
| ark       | 1       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 9

## commands

```
create labor 3 1
work 3 extractor 1 food
create labor 1 2
work 1 extractor 2 food
launch ark
end turn
```

## what changed

**changed** (2)

- game phase:play · turn: 9 → 10
- unit id:1 · place: territory-2 → orbit

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 10   | 12          | 1     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 12       | 0           | 1     |
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

### territory-resource

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
| 1         | metal    | 14     |
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
| 1         | 1     |
| 2         | 1     |

2 row(s)

### extractor

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 3    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 4    | metal    | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 6    | energy   | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 8    | energy   | yes   |
| 2         | 0    | food     | yes   |
| 2         | 2    | metal    | yes   |

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

| id  | kind | place | fuel | ready |
| --- | ---- | ----- | ---- | ----- |
| 1   | ark  | orbit | 1    | yes   |

1 row(s)

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 18      |
| labor     | 0       |
| food      | 0       |
| metal     | 14      |
| energy    | 20      |
| extractor | 11      |
| garrison  | 2       |
| yard      | 1       |
| ark       | 1       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

