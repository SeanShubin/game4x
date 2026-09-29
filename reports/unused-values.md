# == Values the `.4x` suite would not miss ==

7307 mutations tried; 143 of them nothing noticed - 78 in the ruleset, 65 in the machinery and vocabulary around it.

## Ruleset rows no reviewed behaviour depends on  (78 row(s))

This is the one that goes to zero, and each row goes one of two ways: a test of the behaviour that needs it, or the row deleted because nothing needs it.

### 7 row(s) - nothing a reviewed test asserts depends on this part of `deploy`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:635  {input id:21 rule:14 seq:1 name:where of:48}
  rules.4x:636  {input id:22 rule:14 seq:2 name:what of:55}
  rules.4x:641  {clause id:47 rule:14 seq:2 role:1 relation:48}
  rules.4x:642  {clause id:48 rule:14 seq:3 role:2 relation:55}
  rules.4x:643  {clause id:49 rule:14 seq:4 role:3 relation:19}
  rules.4x:644  {clause id:50 rule:14 seq:5 role:3 relation:19}
  rules.4x:645  {clause id:51 rule:14 seq:6 role:3 relation:47}
```

### 7 row(s) - nothing a reviewed test asserts depends on this part of `work`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:109  {input id:8 rule:3 seq:1 name:where of:48}
  rules.4x:110  {input id:9 rule:3 seq:2 name:what of:27}
  rules.4x:112  {clause id:10 rule:3 seq:1 role:1 relation:18}
  rules.4x:113  {clause id:13 rule:3 seq:2 role:2 relation:19}
  rules.4x:114  {clause id:11 rule:3 seq:3 role:2 relation:29}
  rules.4x:115  {clause id:14 rule:3 seq:4 role:3 relation:19}
  rules.4x:116  {clause id:12 rule:3 seq:5 role:3 relation:27}
```

### 6 row(s) - nothing a reviewed test asserts depends on this part of `gather`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:526  {input id:19 rule:12 seq:1 name:where of:48}
  rules.4x:528  {clause id:39 rule:12 seq:1 role:1 relation:18}
  rules.4x:529  {clause id:35 rule:12 seq:2 role:1 relation:51}
  rules.4x:530  {clause id:36 rule:12 seq:3 role:2 relation:51}
  rules.4x:531  {clause id:37 rule:12 seq:4 role:3 relation:51}
  rules.4x:532  {clause id:38 rule:12 seq:5 role:3 relation:50}
```

### 6 row(s) - nothing a reviewed test asserts depends on this part of `upkeep`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:316  {input id:15 rule:8 seq:1 name:where of:48}
  rules.4x:316  {input id:15 rule:8 seq:1 name:where of:48}
  rules.4x:321  {clause id:28 rule:8 seq:1 role:1 relation:47}
  rules.4x:322  {clause id:21 rule:8 seq:2 role:2 relation:47}
  rules.4x:323  {clause id:22 rule:8 seq:3 role:2 relation:31}
  rules.4x:324  {clause id:23 rule:8 seq:4 role:3 relation:47}
```

### 5 row(s) - nothing a reviewed test asserts depends on this part of `breed`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:413  {input id:17 rule:10 seq:1 name:where of:48}
  rules.4x:413  {input id:17 rule:10 seq:1 name:where of:48}
  rules.4x:418  {clause id:25 rule:10 seq:1 role:2 relation:47}
  rules.4x:419  {clause id:26 rule:10 seq:2 role:2 relation:31}
  rules.4x:420  {clause id:27 rule:10 seq:3 role:3 relation:47}
```

### 5 row(s) - nothing a reviewed test asserts depends on this part of `build-bin`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:238  {input id:13 rule:6 seq:1 name:where of:48}
  rules.4x:239  {input id:14 rule:6 seq:2 name:what of:27}
  rules.4x:241  {clause id:17 rule:6 seq:1 role:2 relation:29}
  rules.4x:242  {clause id:18 rule:6 seq:2 role:2 relation:30}
  rules.4x:243  {clause id:19 rule:6 seq:3 role:3 relation:42}
```

### 5 row(s) - nothing a reviewed test asserts depends on this part of `build-extractor`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:89  {input id:4 rule:2 seq:1 name:where of:48}
  rules.4x:90  {input id:7 rule:2 seq:2 name:what of:27}
  rules.4x:92  {clause id:5 rule:2 seq:1 role:2 relation:29}
  rules.4x:93  {clause id:6 rule:2 seq:2 role:2 relation:30}
  rules.4x:94  {clause id:7 rule:2 seq:3 role:3 relation:19}
```

### 5 row(s) - nothing a reviewed test asserts depends on this part of `end-turn`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:211  {part id:1 of:5 is:4 seq:5}
  rules.4x:212  {part id:2 of:5 is:4 seq:6}
  rules.4x:214  {part id:8 of:5 is:4 seq:8}
  rules.4x:215  {part id:9 of:5 is:4 seq:9}
  rules.4x:216  {part id:10 of:5 is:4 seq:10}
```

### 5 row(s) - nothing a reviewed test asserts depends on this part of `launch`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:582  {input id:20 rule:13 seq:1 name:where of:48}
  rules.4x:587  {clause id:42 rule:13 seq:4 role:2 relation:29}
  rules.4x:588  {clause id:43 rule:13 seq:5 role:2 relation:30}
  rules.4x:589  {clause id:44 rule:13 seq:6 role:2 relation:50}
  rules.4x:590  {clause id:45 rule:13 seq:7 role:3 relation:51}
```

### 5 row(s) - nothing a reviewed test asserts depends on this part of `move`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:57  {input id:1 rule:1 seq:1 name:what of:26}
  rules.4x:58  {input id:2 rule:1 seq:2 name:from of:48}
  rules.4x:59  {input id:3 rule:1 seq:3 name:to of:48}
  rules.4x:64  {clause id:3 rule:1 seq:4 role:2 relation:26}
  rules.4x:65  {clause id:4 rule:1 seq:5 role:3 relation:26}
```

### 5 row(s) - nothing a reviewed test asserts depends on this part of `toil`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:458  {input id:18 rule:11 seq:1 name:where of:48}
  rules.4x:462  {clause id:31 rule:11 seq:1 role:1 relation:47}
  rules.4x:463  {clause id:32 rule:11 seq:2 role:2 relation:47}
  rules.4x:464  {clause id:33 rule:11 seq:3 role:3 relation:47}
  rules.4x:465  {clause id:34 rule:11 seq:4 role:3 relation:29}
```

### 4 row(s) - no test's `when` fires `perish` at all

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:359  {rule id:9 name:perish}
  rules.4x:361  {input id:16 rule:9 seq:1 name:where of:48}
  rules.4x:361  {input id:16 rule:9 seq:1 name:where of:48}
  rules.4x:366  {clause id:24 rule:9 seq:1 role:2 relation:47}
```

### 4 row(s) - nothing a reviewed test asserts depends on this part of `build-pioneer`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:692  {input id:23 rule:15 seq:1 name:where of:48}
  rules.4x:694  {clause id:52 rule:15 seq:1 role:2 relation:29}
  rules.4x:695  {clause id:53 rule:15 seq:2 role:2 relation:30}
  rules.4x:696  {clause id:54 rule:15 seq:3 role:3 relation:54}
```

### 4 row(s) - nothing a reviewed test asserts depends on this part of `build-yard`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:721  {input id:24 rule:16 seq:1 name:where of:48}
  rules.4x:723  {clause id:55 rule:16 seq:1 role:2 relation:29}
  rules.4x:724  {clause id:56 rule:16 seq:2 role:2 relation:30}
  rules.4x:725  {clause id:57 rule:16 seq:3 role:3 relation:58}
```

### 4 row(s) - nothing a reviewed test asserts depends on this part of `refresh`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:176  {input id:11 rule:4 seq:1 name:what of:1}
  rules.4x:177  {input id:12 rule:4 seq:2 name:trait of:38}
  rules.4x:179  {clause id:15 rule:4 seq:1 role:4 relation:26}
  rules.4x:183  {assigns id:1 clause:15 input:12 value:1}
```

### 1 row(s) - nothing a reviewed test asserts depends on this part of `discard-disorder`

(an earlier run saw rows of each of these kinds)

```text
  rules.4x:290  {clause id:20 rule:7 seq:1 role:5 relation:49}
```

## The machinery and vocabulary around them  (65 row(s))

Reported rather than enforced. A vocabulary row wants a test of whatever reads it: a territory of each biome would pin all six at once.

### 7 row(s) - `one-extractors-readiness-is-not-anothers` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/one-extractors-readiness-is-not-anothers.4x:16  {capacity of:18 for:19 what:27 per:48 quantity:1}
  tests/one-extractors-readiness-is-not-anothers.4x:17  {deposit where:1 what:31 density:6 quantity:1}
  tests/one-extractors-readiness-is-not-anothers.4x:18  {deposit where:1 what:30 density:2 quantity:1}
  tests/one-extractors-readiness-is-not-anothers.4x:19  {extractor where:1 what:31 working:1 quantity:1}
  tests/one-extractors-readiness-is-not-anothers.4x:19  {extractor where:1 what:31 working:1 quantity:1}
  tests/one-extractors-readiness-is-not-anothers.4x:20  {extractor where:1 what:30 working:1 quantity:1}
  tests/one-extractors-readiness-is-not-anothers.4x:20  {extractor where:1 what:30 working:1 quantity:1}
```

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

### 4 row(s) - `an-extractor-cannot-be-worked-twice-on-one-readiness` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x:16  {capacity of:18 for:19 what:27 per:48 quantity:1}
  tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x:17  {deposit where:1 what:31 density:6 quantity:1}
  tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x:18  {extractor where:1 what:31 working:1 quantity:1}
  tests/an-extractor-cannot-be-worked-twice-on-one-readiness.4x:18  {extractor where:1 what:31 working:1 quantity:1}
```

### 4 row(s) - `the-hungry-perish-after-upkeep-and-not-before` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/the-hungry-perish-after-upkeep-and-not-before.4x:28  {citizen where:1 hungry:1 bearing:1 laboring:1 quantity:3}
  tests/the-hungry-perish-after-upkeep-and-not-before.4x:28  {citizen where:1 hungry:1 bearing:1 laboring:1 quantity:3}
  tests/the-hungry-perish-after-upkeep-and-not-before.4x:30  {citizen where:2 hungry:1 bearing:1 laboring:1 quantity:2}
  tests/the-hungry-perish-after-upkeep-and-not-before.4x:30  {citizen where:2 hungry:1 bearing:1 laboring:1 quantity:2}
```

### 4 row(s) - `the-sun-reaches-an-ark-once-a-turn` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/the-sun-reaches-an-ark-once-a-turn.4x:20  {deposit where:1 what:50 density:3 quantity:1}
  tests/the-sun-reaches-an-ark-once-a-turn.4x:20  {deposit where:1 what:50 density:3 quantity:1}
  tests/the-sun-reaches-an-ark-once-a-turn.4x:21  {ark where:1 moving:1 gathering:1 quantity:1}
  tests/the-sun-reaches-an-ark-once-a-turn.4x:21  {ark where:1 moving:1 gathering:1 quantity:1}
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

### 3 row(s) - `nothing-moves-between-the-layers` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/nothing-moves-between-the-layers.4x:25  {adjacency id:1 from:1 to:2}
  tests/nothing-moves-between-the-layers.4x:26  {scout where:1 moving:1 quantity:1}
  tests/nothing-moves-between-the-layers.4x:26  {scout where:1 moving:1 quantity:1}
```

### 3 row(s) - `the-same-free-space-admits-one-kind-and-refuses-another` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/the-same-free-space-admits-one-kind-and-refuses-another.4x:28  {adjacency id:1 from:1 to:2}
  tests/the-same-free-space-admits-one-kind-and-refuses-another.4x:30  {transport where:2 moving:1 quantity:2}
  tests/the-same-free-space-admits-one-kind-and-refuses-another.4x:31  {scout where:2 moving:1 quantity:1}
```

### 3 row(s) - `the-scout-cannot-cross-where-there-is-no-border` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/the-scout-cannot-cross-where-there-is-no-border.4x:12  {place id:2 of:2 layer:surface}
  tests/the-scout-cannot-cross-where-there-is-no-border.4x:17  {scout where:1 moving:1 quantity:1}
  tests/the-scout-cannot-cross-where-there-is-no-border.4x:17  {scout where:1 moving:1 quantity:1}
```

### 2 row(s) - `a-citizen-eats-and-one-there-is-no-food-for-starves` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-citizen-eats-and-one-there-is-no-food-for-starves.4x:19  {citizen where:1 hungry:1 bearing:1 laboring:1 quantity:3}
  tests/a-citizen-eats-and-one-there-is-no-food-for-starves.4x:19  {citizen where:1 hungry:1 bearing:1 laboring:1 quantity:3}
```

### 2 row(s) - `a-deployment-places-what-the-ground-has-room-for` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-deployment-places-what-the-ground-has-room-for.4x:25  {ark where:2 moving:1 gathering:1 quantity:1}
  tests/a-deployment-places-what-the-ground-has-room-for.4x:25  {ark where:2 moving:1 gathering:1 quantity:1}
```

### 2 row(s) - `a-deployment-with-nowhere-to-mine-still-costs-the-ark` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-deployment-with-nowhere-to-mine-still-costs-the-ark.4x:29  {ark where:2 moving:1 gathering:1 quantity:1}
  tests/a-deployment-with-nowhere-to-mine-still-costs-the-ark.4x:29  {ark where:2 moving:1 gathering:1 quantity:1}
```

### 2 row(s) - `a-scout-cannot-move-where-every-berth-is-taken` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-scout-cannot-move-where-every-berth-is-taken.4x:17  {adjacency id:1 from:1 to:2}
  tests/a-scout-cannot-move-where-every-berth-is-taken.4x:19  {transport where:2 moving:1 quantity:3}
```

### 2 row(s) - `an-ark-deploys-and-becomes-a-settlement` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/an-ark-deploys-and-becomes-a-settlement.4x:35  {ark where:2 moving:1 gathering:1 quantity:1}
  tests/an-ark-deploys-and-becomes-a-settlement.4x:35  {ark where:2 moving:1 gathering:1 quantity:1}
```

### 2 row(s) - `an-ark-deploys-onto-the-ground-below-it` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/an-ark-deploys-onto-the-ground-below-it.4x:39  {ark where:2 moving:1 gathering:1 quantity:1}
  tests/an-ark-deploys-onto-the-ground-below-it.4x:39  {ark where:2 moving:1 gathering:1 quantity:1}
```

### 2 row(s) - `an-ark-holds-one-energy-and-the-rest-is-lost` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/an-ark-holds-one-energy-and-the-rest-is-lost.4x:26  {ark where:1 moving:1 gathering:1 quantity:1}
  tests/an-ark-holds-one-energy-and-the-rest-is-lost.4x:26  {ark where:1 moving:1 gathering:1 quantity:1}
```

### 2 row(s) - `an-extractor-cannot-be-built-where-the-deposits-are-taken` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/an-extractor-cannot-be-built-where-the-deposits-are-taken.4x:15  {deposit where:1 what:31 density:6 quantity:1}
  tests/an-extractor-cannot-be-built-where-the-deposits-are-taken.4x:16  {extractor where:1 what:31 working:0 quantity:1}
```

### 2 row(s) - `an-extractor-cannot-be-worked-without-labor` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/an-extractor-cannot-be-worked-without-labor.4x:14  {capacity of:18 for:19 what:27 per:48 quantity:1}
  tests/an-extractor-cannot-be-worked-without-labor.4x:15  {deposit where:1 what:31 density:6 quantity:1}
```

### 1 row(s) - `a-bin-cannot-be-built-where-the-capacity-is-taken` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-bin-cannot-be-built-where-the-capacity-is-taken.4x:20  {place id:1 of:1 layer:surface}
```

### 1 row(s) - `a-pioneer-settles-the-ground-it-is-standing-on` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-pioneer-settles-the-ground-it-is-standing-on.4x:32  {pioneer where:1 moving:1 quantity:1}
```

### 1 row(s) - `a-scout-arriving-does-not-lend-a-move-to-one-that-has-spent-its-own` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/a-scout-arriving-does-not-lend-a-move-to-one-that-has-spent-its-own.4x:22  {scout where:2 moving:0 quantity:1}
```

### 1 row(s) - `breeding-does-not-reach-the-citizens-it-just-made` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/breeding-does-not-reach-the-citizens-it-just-made.4x:27  {citizen where:1 hungry:0 bearing:1 laboring:1 quantity:2}
```

### 1 row(s) - `breeding-stops-when-the-food-does` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/breeding-stops-when-the-food-does.4x:21  {citizen where:1 hungry:1 bearing:1 laboring:1 quantity:3}
```

### 1 row(s) - `three-citizens-and-ten-food-become-six` reaches its ending without it

(an earlier run saw rows of each of these kinds)

```text
  tests/three-citizens-and-ten-food-become-six.4x:28  {citizen where:1 hungry:1 bearing:1 laboring:1 quantity:3}
```

### 1 row(s) - no `.4x` test reads it and no rule here says why (the word `supply.name` appears in isolation.rs, structure.rs)

(an earlier run saw rows of each of these kinds)

```text
  schema.4x:410  {supply id:1 name:berth per:48}
```
