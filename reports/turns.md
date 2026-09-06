# Every turn of `scenario/commands/play.4x`

**Generated. Do not edit.** One section per `end turn` in the scenario - 13 of them.
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
- {extractor territory:1 node:5 resource:metal ready:yes}
- {garrison territory:1 force:0}

**changed** (15)

- game phase:play · turn: 1 → 2
- game phase:play · units: 1 → 0
- kind id:ark · in-play: 1 → 0
- kind id:citizen · in-play: 0 → 4
- kind id:extractor · in-play: 0 → 2
- kind id:garrison · in-play: 0 → 1
- kind id:metal · in-play: 0 → 3
- labor territory:1 · made: 0 → 4
- labor territory:1 · left: 0 → 4
- store territory:1 resource:metal · amount: 0 → 3
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
| 1         | food     | 5        | 6       | 1     |
| 1         | metal    | 2        | 3       | 1     |
| 1         | energy   | 1        | 3       | 0     |
| 2         | food     | 5        | 6       | 0     |
| 2         | metal    | 2        | 3       | 0     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |

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
| metal     | 3       |
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
build store 1 food
create labor 1 1
build extractor 1 food
end turn
```

## what changed

**new** (1)

- {extractor territory:1 node:1 resource:food ready:yes}

**changed** (10)

- game phase:play · turn: 2 → 3
- kind id:citizen · in-play: 4 → 6
- kind id:extractor · in-play: 2 → 3
- kind id:metal · in-play: 3 → 1
- labor territory:1 · made: 4 → 6
- labor territory:1 · left: 4 → 6
- store territory:1 resource:metal · amount: 3 → 1
- structure territory:1 structure:extractor · count: 2 → 3
- territory id:1 · citizens: 4 → 6
- territory-resource territory:1 resource:food · built: 1 → 2

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 3    | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 6        | 0           | 0     |
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
| 1         | food     | 5        | 6       | 2     |
| 1         | metal    | 2        | 3       | 1     |
| 1         | energy   | 1        | 3       | 0     |
| 2         | food     | 5        | 6       | 0     |
| 2         | metal    | 2        | 3       | 0     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 1      |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |

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
| 1         | 6    | 0     | 6    |
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
| citizen   | 6       |
| labor     | 0       |
| food      | 0       |
| metal     | 1       |
| energy    | 0       |
| extractor | 3       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 3

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
build store 1 food
end turn
```

## what changed

**changed** (7)

- game phase:play · turn: 3 → 4
- kind id:citizen · in-play: 6 → 12
- kind id:metal · in-play: 1 → 3
- labor territory:1 · made: 6 → 12
- labor territory:1 · left: 6 → 12
- store territory:1 resource:metal · amount: 1 → 3
- territory id:1 · citizens: 6 → 12

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 4    | 12          | 0     |

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
| 1         | food     | 5        | 6       | 2     |
| 1         | metal    | 2        | 3       | 1     |
| 1         | energy   | 1        | 3       | 0     |
| 2         | food     | 5        | 6       | 0     |
| 2         | metal    | 2        | 3       | 0     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |

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
| metal     | 3       |
| energy    | 0       |
| extractor | 3       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 4

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
build extractor 1 food
create labor 1 1
build extractor 1 metal
create labor 1 1
build store 1 food
end turn
```

## what changed

**new** (2)

- {extractor territory:1 node:2 resource:food ready:yes}
- {extractor territory:1 node:6 resource:metal ready:yes}

**changed** (7)

- game phase:play · turn: 4 → 5
- kind id:extractor · in-play: 3 → 5
- kind id:metal · in-play: 3 → 0
- store territory:1 resource:metal · amount: 3 → 0
- structure territory:1 structure:extractor · count: 3 → 5
- territory-resource territory:1 resource:food · built: 2 → 3
- territory-resource territory:1 resource:metal · built: 1 → 2

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
| 1         | food     | 5        | 6       | 3     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 0     |
| 2         | food     | 5        | 6       | 0     |
| 2         | metal    | 2        | 3       | 0     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |

5 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 5     |
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
| metal     | 0       |
| energy    | 0       |
| extractor | 5       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 5

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
build extractor 1 energy
create labor 1 1
build store 1 energy
create labor 1 1
build store 1 metal
end turn
```

## what changed

**new** (1)

- {extractor territory:1 node:7 resource:energy ready:yes}

**changed** (10)

- game phase:play · turn: 5 → 6
- kind id:citizen · in-play: 12 → 18
- kind id:extractor · in-play: 5 → 6
- kind id:metal · in-play: 0 → 3
- labor territory:1 · made: 12 → 18
- labor territory:1 · left: 12 → 18
- store territory:1 resource:metal · amount: 0 → 3
- structure territory:1 structure:extractor · count: 5 → 6
- territory id:1 · citizens: 12 → 18
- territory-resource territory:1 resource:energy · built: 0 → 1

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 6    | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 18       | 0           | 0     |
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
| 1         | food     | 5        | 6       | 3     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 0     |
| 2         | metal    | 2        | 3       | 0     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |

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
| 1         | 18   | 0     | 18   |
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
| citizen   | 18      |
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

11 row(s)

# Turn 6

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 energy
create labor 1 1
build extractor 1 food
end turn
```

## what changed

**new** (1)

- {extractor territory:1 node:3 resource:food ready:yes}

**changed** (8)

- game phase:play · turn: 6 → 7
- kind id:energy · in-play: 0 → 3
- kind id:extractor · in-play: 6 → 7
- kind id:metal · in-play: 3 → 8
- store territory:1 resource:energy · amount: 0 → 3
- store territory:1 resource:metal · amount: 3 → 8
- structure territory:1 structure:extractor · count: 6 → 7
- territory-resource territory:1 resource:food · built: 3 → 4

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 7    | 12          | 0     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 18       | 0           | 0     |
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
| 1         | food     | 5        | 6       | 4     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 0     |
| 2         | metal    | 2        | 3       | 0     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 8      |
| 1         | energy   | 3      |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 3    | food     | yes   |

7 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 7     |
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
| 1         | 18   | 0     | 18   |
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
| citizen   | 18      |
| labor     | 0       |
| food      | 0       |
| metal     | 8       |
| energy    | 3       |
| extractor | 7       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 7

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 energy
create labor 1 1
produce pioneer 1
end turn
```

## what changed

**new** (1)

- {unit id:1 kind:pioneer place:territory-1 fuel:2 ready:yes}

**changed** (11)

- game phase:play · turn: 7 → 8
- game phase:play · units: 0 → 1
- kind id:citizen · in-play: 18 → 23
- kind id:energy · in-play: 3 → 0
- kind id:metal · in-play: 8 → 11
- kind id:pioneer · in-play: 0 → 1
- labor territory:1 · made: 18 → 23
- labor territory:1 · left: 18 → 23
- store territory:1 resource:energy · amount: 3 → 0
- store territory:1 resource:metal · amount: 8 → 11
- territory id:1 · citizens: 18 → 23

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 8    | 12          | 1     |

1 row(s)

### territory

| id  | biome     | nature | citizens | labor-spent | yards |
| --- | --------- | ------ | -------- | ----------- | ----- |
| 1   | grassland | 1      | 23       | 0           | 0     |
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
| 1         | food     | 5        | 6       | 4     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 0     |
| 2         | metal    | 2        | 3       | 0     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 11     |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 3    | food     | yes   |

7 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 7     |
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
| 1         | 23   | 0     | 23   |
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
| citizen   | 23      |
| labor     | 0       |
| food      | 0       |
| metal     | 11      |
| energy    | 0       |
| extractor | 7       |
| garrison  | 1       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 1       |
| territory | 12      |

11 row(s)

# Turn 8

## commands

```
found by land 2
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 energy
create labor 1 2
work 1 extractor 2 food
end turn
```

## what changed

**gone** (1)

- {unit id:1 kind:pioneer place:territory-1 fuel:2 ready:yes}

**new** (3)

- {extractor territory:2 node:0 resource:food ready:yes}
- {extractor territory:2 node:5 resource:metal ready:yes}
- {garrison territory:2 force:0}

**changed** (20)

- game phase:play · turn: 8 → 9
- game phase:play · units: 1 → 0
- kind id:citizen · in-play: 23 → 16
- kind id:energy · in-play: 0 → 3
- kind id:extractor · in-play: 7 → 9
- kind id:garrison · in-play: 1 → 2
- kind id:metal · in-play: 11 → 17
- kind id:pioneer · in-play: 1 → 0
- labor territory:1 · made: 23 → 12
- labor territory:1 · left: 23 → 12
- labor territory:2 · made: 0 → 4
- labor territory:2 · left: 0 → 4
- store territory:1 resource:energy · amount: 0 → 3
- store territory:1 resource:metal · amount: 11 → 17
- structure territory:2 structure:extractor · count: 0 → 2
- structure territory:2 structure:garrison · count: 0 → 1
- territory id:1 · citizens: 23 → 12
- territory id:2 · citizens: 0 → 4
- territory-resource territory:2 resource:food · built: 0 → 1
- territory-resource territory:2 resource:metal · built: 0 → 1

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 9    | 12          | 0     |

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
| 1         | food     | 5        | 6       | 4     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 1     |
| 2         | metal    | 2        | 3       | 1     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 17     |
| 1         | energy   | 3      |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 3    | food     | yes   |
| 2         | 0    | food     | yes   |
| 2         | 5    | metal    | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 7     |
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
| metal     | 17      |
| energy    | 3       |
| extractor | 9       |
| garrison  | 2       |
| yard      | 0       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 9

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 energy
create labor 1 1
build yard 1
create labor 1 2
work 1 extractor 2 food
end turn
```

## what changed

**changed** (12)

- game phase:play · turn: 9 → 10
- kind id:citizen · in-play: 16 → 18
- kind id:energy · in-play: 3 → 6
- kind id:metal · in-play: 17 → 2
- kind id:yard · in-play: 0 → 1
- labor territory:2 · made: 4 → 6
- labor territory:2 · left: 4 → 6
- store territory:1 resource:energy · amount: 3 → 6
- store territory:1 resource:metal · amount: 17 → 2
- structure territory:1 structure:yard · count: 0 → 1
- territory id:1 · yards: 0 → 1
- territory id:2 · citizens: 4 → 6

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 10   | 12          | 0     |

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
| 1         | food     | 5        | 6       | 4     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 1     |
| 2         | metal    | 2        | 3       | 1     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 2      |
| 1         | energy   | 6      |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 3    | food     | yes   |
| 2         | 0    | food     | yes   |
| 2         | 5    | metal    | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 7     |
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

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 18      |
| labor     | 0       |
| food      | 0       |
| metal     | 2       |
| energy    | 6       |
| extractor | 9       |
| garrison  | 2       |
| yard      | 1       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 10

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 energy
create labor 1 2
work 1 extractor 2 food
end turn
```

## what changed

**changed** (5)

- game phase:play · turn: 10 → 11
- kind id:energy · in-play: 6 → 9
- kind id:metal · in-play: 2 → 5
- store territory:1 resource:energy · amount: 6 → 9
- store territory:1 resource:metal · amount: 2 → 5

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 11   | 12          | 0     |

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
| 1         | food     | 5        | 6       | 4     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 1     |
| 2         | metal    | 2        | 3       | 1     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 5      |
| 1         | energy   | 9      |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 3    | food     | yes   |
| 2         | 0    | food     | yes   |
| 2         | 5    | metal    | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 7     |
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

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 18      |
| labor     | 0       |
| food      | 0       |
| metal     | 5       |
| energy    | 9       |
| extractor | 9       |
| garrison  | 2       |
| yard      | 1       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 11

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 energy
create labor 1 2
work 1 extractor 2 food
end turn
```

## what changed

**changed** (5)

- game phase:play · turn: 11 → 12
- kind id:energy · in-play: 9 → 10
- kind id:metal · in-play: 5 → 8
- store territory:1 resource:energy · amount: 9 → 10
- store territory:1 resource:metal · amount: 5 → 8

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 12   | 12          | 0     |

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
| 1         | food     | 5        | 6       | 4     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 1     |
| 2         | metal    | 2        | 3       | 1     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 8      |
| 1         | energy   | 10     |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 3    | food     | yes   |
| 2         | 0    | food     | yes   |
| 2         | 5    | metal    | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 7     |
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

*(empty) 0 rows*

### kind

| id        | in-play |
| --------- | ------- |
| citizen   | 18      |
| labor     | 0       |
| food      | 0       |
| metal     | 8       |
| energy    | 10      |
| extractor | 9       |
| garrison  | 2       |
| yard      | 1       |
| ark       | 0       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 12

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 metal
create labor 1 1
work 1 extractor 1 energy
create labor 1 1
produce ark 1
create labor 1 2
work 1 extractor 2 food
move ark 2
end turn
```

## what changed

**new** (1)

- {unit id:1 kind:ark place:territory-2 fuel:1 ready:yes}

**changed** (5)

- game phase:play · turn: 12 → 13
- game phase:play · units: 0 → 1
- kind id:ark · in-play: 0 → 1
- kind id:energy · in-play: 10 → 1
- store territory:1 resource:energy · amount: 10 → 1

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 13   | 12          | 1     |

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
| 1         | food     | 5        | 6       | 4     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 1     |
| 2         | metal    | 2        | 3       | 1     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 8      |
| 1         | energy   | 1      |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 3    | food     | yes   |
| 2         | 0    | food     | yes   |
| 2         | 5    | metal    | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 7     |
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
| metal     | 8       |
| energy    | 1       |
| extractor | 9       |
| garrison  | 2       |
| yard      | 1       |
| ark       | 1       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

# Turn 13

## commands

```
create labor 1 1
work 1 extractor 1 food
create labor 1 1
work 1 extractor 1 food
create labor 1 2
work 1 extractor 2 food
launch ark
end turn
```

## what changed

**changed** (2)

- game phase:play · turn: 13 → 14
- unit id:1 · place: territory-2 → orbit

## what is there now

### game

| phase | turn | territories | units |
| ----- | ---- | ----------- | ----- |
| play  | 14   | 12          | 1     |

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
| 1         | food     | 5        | 6       | 4     |
| 1         | metal    | 2        | 3       | 2     |
| 1         | energy   | 1        | 3       | 1     |
| 2         | food     | 5        | 6       | 1     |
| 2         | metal    | 2        | 3       | 1     |
| 2         | energy   | 1        | 3       | 0     |
| 3         | food     | 5        | 6       | 0     |
| 3         | metal    | 2        | 3       | 0     |
| 3         | energy   | 1        | 3       | 0     |
| 4         | food     | 1        | 3       | 0     |
| 4         | metal    | 5        | 7       | 0     |
| 4         | energy   | 2        | 3       | 0     |
| 5         | food     | 1        | 3       | 0     |
| 5         | metal    | 5        | 7       | 0     |
| 5         | energy   | 2        | 3       | 0     |
| 6         | food     | 6        | 6       | 0     |
| 6         | metal    | 1        | 2       | 0     |
| 6         | energy   | 1        | 2       | 0     |
| 7         | food     | 6        | 6       | 0     |
| 7         | metal    | 1        | 2       | 0     |
| 7         | energy   | 1        | 2       | 0     |
| 8         | food     | 5        | 6       | 0     |
| 8         | metal    | 2        | 3       | 0     |
| 8         | energy   | 1        | 3       | 0     |
| 9         | food     | 1        | 3       | 0     |
| 9         | metal    | 5        | 7       | 0     |
| 9         | energy   | 2        | 3       | 0     |
| 10        | food     | 2        | 4       | 0     |
| 10        | metal    | 3        | 4       | 0     |
| 10        | energy   | 5        | 6       | 0     |
| 11        | food     | 5        | 6       | 0     |
| 11        | metal    | 2        | 3       | 0     |
| 11        | energy   | 1        | 3       | 0     |
| 12        | food     | 1        | 2       | 0     |
| 12        | metal    | 3        | 5       | 0     |
| 12        | energy   | 1        | 2       | 0     |

36 row(s)

### store

| territory | resource | amount |
| --------- | -------- | ------ |
| 1         | food     | 0      |
| 1         | metal    | 8      |
| 1         | energy   | 1      |
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

| territory | node | resource | ready |
| --------- | ---- | -------- | ----- |
| 1         | 0    | food     | yes   |
| 1         | 5    | metal    | yes   |
| 1         | 1    | food     | yes   |
| 1         | 2    | food     | yes   |
| 1         | 6    | metal    | yes   |
| 1         | 7    | energy   | yes   |
| 1         | 3    | food     | yes   |
| 2         | 0    | food     | yes   |
| 2         | 5    | metal    | yes   |

9 row(s)

### structure

| territory | structure | count |
| --------- | --------- | ----- |
| 1         | extractor | 7     |
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
| metal     | 8       |
| energy    | 1       |
| extractor | 9       |
| garrison  | 2       |
| yard      | 1       |
| ark       | 1       |
| pioneer   | 0       |
| territory | 12      |

11 row(s)

