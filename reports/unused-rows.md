# == Rows the `.4x` suite would not miss ==

1660 mutations tried; 86 of them nothing noticed - 49 in the ruleset, 37 in the machinery and vocabulary around it.

## Ruleset rows no reviewed behaviour depends on  (49 row(s))

This is the one that goes to zero, and each row goes one of two ways: a test of the behaviour that needs it, or the row deleted because nothing needs it.

### 9 row(s) - nothing a reviewed test asserts depends on this part of `breed`

(1 of a kind no earlier run saw)

```text
  rules.4x:424  {binding id:39 clause:26 column:82 input:17}
  rules.4x:425  {binding id:40 clause:27 column:130 input:17}
  rules.4x:426  {binding id:66 clause:59 column:130 input:17}
  rules.4x:428  {literal id:29 clause:25 column:131 value:0}
  rules.4x:429  {literal id:30 clause:25 column:133 value:1}
  rules.4x:432  {literal id:33 clause:27 column:131 value:0}
  rules.4x:435  {literal id:84 clause:59 column:131 value:0}
  rules.4x:436  {literal id:85 clause:59 column:133 value:1}
* rules.4x:440  {reading id:17 clause:27 column:137 of:25 takes:137}
```

### 8 row(s) - nothing a reviewed test asserts depends on this part of `launch`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:597  {binding id:53 clause:40 column:134 input:20}
  rules.4x:598  {binding id:54 clause:42 column:78 input:20}
  rules.4x:599  {binding id:55 clause:43 column:80 input:20}
  rules.4x:600  {binding id:56 clause:44 column:140 input:20}
  rules.4x:603  {literal id:56 clause:40 column:136 value:surface}
  rules.4x:605  {literal id:58 clause:42 column:79 value:1}
  rules.4x:606  {literal id:59 clause:43 column:81 value:1}
  rules.4x:607  {literal id:60 clause:44 column:141 value:1}
```

### 6 row(s) - nothing a reviewed test asserts depends on this part of `gather`

(1 of a kind no earlier run saw)

```text
  rules.4x:542  {binding id:49 clause:36 column:142 input:19}
  rules.4x:543  {binding id:50 clause:37 column:142 input:19}
  rules.4x:547  {literal id:50 clause:36 column:144 value:1}
  rules.4x:548  {literal id:51 clause:36 column:145 value:1}
  rules.4x:550  {literal id:53 clause:37 column:145 value:1}
* rules.4x:552  {reading id:8 clause:37 column:143 of:35 takes:143}
```

### 4 row(s) - nothing a reviewed test asserts depends on this part of `build-bin`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:245  {binding id:30 clause:17 column:78 input:13}
  rules.4x:246  {binding id:31 clause:18 column:80 input:13}
  rules.4x:251  {literal id:20 clause:17 column:79 value:1}
  rules.4x:252  {literal id:21 clause:18 column:81 value:1}
```

### 4 row(s) - nothing a reviewed test asserts depends on this part of `build-pioneer`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:703  {binding id:59 clause:52 column:78 input:23}
  rules.4x:704  {binding id:60 clause:53 column:80 input:23}
  rules.4x:707  {literal id:77 clause:52 column:79 value:1}
  rules.4x:708  {literal id:78 clause:53 column:81 value:1}
```

### 4 row(s) - nothing a reviewed test asserts depends on this part of `build-yard`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:732  {binding id:62 clause:55 column:78 input:24}
  rules.4x:733  {binding id:63 clause:56 column:80 input:24}
  rules.4x:736  {literal id:81 clause:55 column:79 value:1}
  rules.4x:737  {literal id:82 clause:56 column:81 value:1}
```

### 3 row(s) - nothing a reviewed test asserts depends on this part of `build-extractor`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:96  {binding id:9 clause:5 column:78 input:4}
  rules.4x:97  {binding id:10 clause:6 column:80 input:4}
  rules.4x:102  {literal id:4 clause:5 column:79 value:1}
```

### 3 row(s) - nothing a reviewed test asserts depends on this part of `toil`

(2 of a kind no earlier run saw)

```text
  rules.4x:476  {binding id:46 clause:33 column:130 input:18}
* rules.4x:486  {reading id:5 clause:33 column:131 of:31 takes:131}
* rules.4x:487  {reading id:6 clause:33 column:133 of:31 takes:133}
```

### 3 row(s) - nothing a reviewed test asserts depends on this part of `upkeep`

(2 of a kind no earlier run saw)

```text
  rules.4x:329  {binding id:36 clause:23 column:130 input:15}
* rules.4x:338  {reading id:2 clause:23 column:133 of:28 takes:133}
* rules.4x:339  {reading id:7 clause:23 column:137 of:28 takes:137}
```

### 3 row(s) - nothing a reviewed test asserts depends on this part of `work`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:120  {binding id:17 clause:10 column:51 input:8}
  rules.4x:128  {binding id:28 clause:14 column:55 input:8}
  rules.4x:129  {binding id:29 clause:14 column:56 input:9}
```

### 2 row(s) - nothing a reviewed test asserts depends on this part of `deploy`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:656  {binding id:58 clause:48 column:153 input:21}
  rules.4x:659  {literal id:66 clause:48 column:154 value:1}
```

## The machinery and vocabulary around them  (37 row(s))

Reported rather than enforced. A vocabulary row wants a test of whatever reads it: a territory of each biome would pin all six at once.

### 6 row(s) - nothing refers to this `biome`

(no earlier run saw rows of these kinds)

```text
* schema.4x:936  {biome id:1 name:ocean}
* schema.4x:937  {biome id:2 name:ice}
* schema.4x:938  {biome id:3 name:desert}
* schema.4x:939  {biome id:4 name:grassland}
* schema.4x:940  {biome id:5 name:jungle}
* schema.4x:941  {biome id:6 name:mountain}
```

### 4 row(s) - `the-scout-cannot-cross-where-there-is-no-border` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/the-scout-cannot-cross-where-there-is-no-border.4x:12  {place id:2 of:2 layer:surface}
  tests/the-scout-cannot-cross-where-there-is-no-border.4x:15  {adjacency id:1 from:1 to:2}
  tests/the-scout-cannot-cross-where-there-is-no-border.4x:16  {adjacency id:2 from:2 to:3}
  tests/the-scout-cannot-cross-where-there-is-no-border.4x:17  {scout where:1 moving:1 quantity:1}
```

### 3 row(s) - `a-territory-with-no-orbit-cannot-launch` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-territory-with-no-orbit-cannot-launch.4x:19  {labor where:1 quantity:1}
  tests/a-territory-with-no-orbit-cannot-launch.4x:20  {metal where:1 quantity:1}
  tests/a-territory-with-no-orbit-cannot-launch.4x:21  {energy where:1 quantity:1}
```

### 3 row(s) - `an-ark-cannot-launch-where-there-is-no-yard` reaches its ending without it

(no earlier run saw rows of these kinds)

```text
* tests/an-ark-cannot-launch-where-there-is-no-yard.4x:21  {labor where:1 quantity:1}
* tests/an-ark-cannot-launch-where-there-is-no-yard.4x:22  {metal where:1 quantity:1}
* tests/an-ark-cannot-launch-where-there-is-no-yard.4x:23  {energy where:1 quantity:1}
```

### 3 row(s) - `one-extractors-readiness-is-not-anothers` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/one-extractors-readiness-is-not-anothers.4x:16  {capacity of:18 for:19 what:27 per:48 quantity:1}
  tests/one-extractors-readiness-is-not-anothers.4x:19  {extractor where:1 what:31 working:1 quantity:1}
  tests/one-extractors-readiness-is-not-anothers.4x:20  {extractor where:1 what:30 working:1 quantity:1}
```

### 3 row(s) - no `.4x` test reads it and no rule here says why (the word `attribute` appears in isolation.rs, petri.rs, reviewed.rs, structure.rs)

(an earlier run saw rows of each of these kinds)

```text
  schema.4x:365  {attribute column:53 relation:18}
  schema.4x:366  {attribute column:66 relation:22}
  schema.4x:367  {attribute column:68 relation:23}
```

### 3 row(s) - no `.4x` test reads it and no rule here says why (the word `stands-in` appears in isolation.rs, structure.rs)

(an earlier run saw rows of each of these kinds)

```text
  schema.4x:840  {stands-in kind:51 per:48 layer:orbit}
  schema.4x:841  {stands-in kind:19 per:48 layer:surface}
  schema.4x:911  {stands-in kind:54 per:48 layer:surface}
```

### 2 row(s) - `a-scout-arriving-does-not-lend-a-move-to-one-that-has-spent-its-own` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-scout-arriving-does-not-lend-a-move-to-one-that-has-spent-its-own.4x:22  {scout where:2 moving:0 quantity:1}
  tests/a-scout-arriving-does-not-lend-a-move-to-one-that-has-spent-its-own.4x:25  {move what:28 from:1 to:2}
```

### 2 row(s) - `an-extractor-cannot-be-worked-twice-on-one-readiness` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x:16  {capacity of:18 for:19 what:27 per:48 quantity:1}
  tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x:18  {extractor where:1 what:31 working:1 quantity:1}
```

### 2 row(s) - `nothing-moves-between-the-layers` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/nothing-moves-between-the-layers.4x:25  {adjacency id:1 from:1 to:2}
  tests/nothing-moves-between-the-layers.4x:26  {scout where:1 moving:1 quantity:1}
```

### 2 row(s) - no `.4x` test reads it and no rule here says why (the word `member` appears in browsable.rs, engine.rs, isolation.rs, nogain.rs, regression.rs, structure.rs)

(an earlier run saw rows of each of these kinds)

```text
  schema.4x:342  {member kind:51 family:26}
  schema.4x:905  {member kind:54 family:26}
```

### 1 row(s) - `a-scout-that-has-moved-cannot-move-again` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-scout-that-has-moved-cannot-move-again.4x:22  {move what:28 from:1 to:2}
```

### 1 row(s) - `an-extractor-cannot-be-worked-without-labor` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/an-extractor-cannot-be-worked-without-labor.4x:14  {capacity of:18 for:19 what:27 per:48 quantity:1}
```

### 1 row(s) - `the-sun-reaches-an-ark-once-a-turn` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/the-sun-reaches-an-ark-once-a-turn.4x:21  {ark where:1 moving:1 gathering:1 quantity:1}
```

### 1 row(s) - nothing refers to this `column`

(an earlier run saw rows of each of these kinds)

```text
  schema.4x:855  {column id:139 relation:49 seq:2 name:quantity}
```
