# Recipes

**Generated. Do not edit.** `cargo run -p game-console --bin dump-state`, or `scripts/dump-state.sh`.

Every recipe the release declares, with its own lines gathered under it. The release states
these across seven columns, one row per line and the name written only on the first - which
is the right shape for a table and the wrong one for answering *what does this recipe do*.

**Beside each rule is a worked example** - a state, the command that fires the recipe, and
the state after - in the notation `scenario/expected/play.4x` uses, holding only what that
recipe touched. Every one is a real command run against a real state: `R-7`.

29 recipes, 73 lines between them, 13 worked examples.

## deploy ark

Run by the **player**.

- **require** 1 territory, in `$where`
- **consume** 1 ark, in above `$where`
- **produce** 2 citizen
- **produce** 1 extractor, food
- **produce** 1 extractor, metal

### An example

Before:

```
{game phase:play}
  {orbit id:1} -> 1
    {ark id:1 moving:1 working:1} -> 1
  {territory id:1 biome:grassland} -> 1
    {deposit density:4 resource:food occupied:0 free:3 capacity:3} -> 1
    {deposit density:4 resource:metal occupied:0 free:3 capacity:3} -> 1
```

`{deploy-ark territory:1}`

After:

```
{game phase:play}
  {orbit id:1} -> 1
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {deposit density:4 resource:metal occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {extractor resource:metal working:1} -> 1
    {garrison} -> 1
```

## move

Run by the **player**.

- **require** 1 place, in `$from`
- **require** 1 place, joined to `$from` by an edge the unit crosses, in `$to`
- **require** 1 unit, moving at least 1, in `$from`
- **put** unit — , moving one less, in `$to`
- **consume** 1 energy, in `$from`

### An example

moving spends one energy where it stood and leaves the unit not ready.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {energy} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
```

`{move unit:pioneer from:1 to:2}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:0} -> 1
```

## found by land

Run by the **player**.

- **consume** 1 pioneer
- **produce** 2 citizen
- **produce** 1 extractor, food
- **produce** 1 extractor, metal

### An example

the pioneer is consumed and the ground it takes is furnished.

Before:

```
{game phase:play}
  {territory id:2 biome:grassland} -> 1
    {deposit density:4 resource:food occupied:0 free:3 capacity:3} -> 1
    {deposit density:4 resource:metal occupied:0 free:3 capacity:3} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{found-by-land territory:2}`

After:

```
{game phase:play}
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {deposit density:4 resource:metal occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {extractor resource:metal working:1} -> 1
    {garrison} -> 1
```

## build extractor

Run by the **player**.

- **consume** 1 labor
- **consume** 1 metal
- **produce** 1 extractor, `$resource`

### An example

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {deposit density:4 resource:metal occupied:0 free:3 capacity:3} -> 1
    {garrison} -> 1
    {labor} -> 1
    {metal} -> 1
```

`{build-extractor territory:1 resource:metal}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {deposit density:4 resource:metal occupied:1 free:2 capacity:3} -> 1
    {extractor resource:metal working:1} -> 1
    {garrison} -> 1
```

## build store

Run by the **player**.

- **consume** 1 labor
- **consume** 1 metal
- **produce** 1 store, `$resource`

### An example

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {deposit density:4 resource:metal occupied:1 free:2 capacity:3} -> 1
    {extractor resource:metal working:1} -> 1
    {garrison} -> 1
    {labor} -> 1
    {metal} -> 1
```

`{build-store territory:1 resource:metal}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {deposit density:4 resource:metal occupied:1 free:2 capacity:3} -> 1
    {extractor resource:metal working:1} -> 1
    {garrison} -> 1
    {store resource:metal} -> 1
```

## build yard

Run by the **player**.

- **consume** 1 labor
- **consume** 15 metal
- **produce** 1 yard

### An example

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {labor} -> 1
    {metal} -> 15
```

`{build-yard territory:1}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {yard} -> 1
```

## produce pioneer

Run by the **player**.

- **consume** 3 metal
- **consume** 2 energy
- **consume** 2 citizen
- **produce** 1 pioneer

### An example

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {energy} -> 6
    {garrison} -> 1
    {metal} -> 3
```

`{produce-pioneer territory:1}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {energy} -> 4
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## launch ark

Run by the **player**.

- **require** 1 territory, in `$where`
- **consume** 3 metal
- **consume** 12 energy
- **consume** 2 citizen
- **require** 1 yard
- **produce** 1 ark, in above `$where`

### An example

Before:

```
{game phase:play}
  {orbit id:1} -> 1
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {energy} -> 12
    {garrison} -> 1
    {metal} -> 3
    {yard} -> 1
```

`{launch-ark territory:1}`

After:

```
{game phase:play}
  {orbit id:1} -> 1
    {ark id:1 moving:1 working:1} -> 1
  {territory id:1 biome:grassland} -> 1
    {garrison} -> 1
    {yard} -> 1
```

## create labor

Run by the **player**.

- **require** 1 citizen, laboring at least 1
- **put** citizen — , laboring one less
- **produce** 1 labor

### An example

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
```

`{create-labor territory:1}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:0 paid:0} -> 1
    {garrison} -> 1
    {labor} -> 1
```

## mine energy

Run by the **player**.

- **require** 1 territory, in `$where`
- **require** 1 ark, working at least 1, in above `$where`
- **put** ark — , working one less, in above `$where`
- **produce** 1 energy, in above `$where`

### An example

Before:

```
{game phase:play}
  {orbit id:1} -> 1
    {ark id:1 moving:1 working:1} -> 1
```

`{mine-energy territory:1}`

After:

```
{game phase:play}
  {orbit id:1} -> 1
    {ark id:1 moving:1 working:0} -> 1
    {energy} -> 1
```

## work

Run by the **player**.

- **require** 1 territory, in `$where`
- **require** 1 extractor, working at least 1
- **put** extractor — , working one less
- **consume** 1 labor
- **produce** resource — `$where`'s density for that resource

### An example

the density is what one extractor yields, so four food from a `3 x 4`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {labor} -> 1
    {store resource:food} -> 1
```

`{work territory:1 resource:food}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 4
    {garrison} -> 1
    {store resource:food} -> 1
```

## upkeep

Run by the **world**.

- **require** 1 citizen
- **consume** 1 food
- **put** citizen — , paid at its maximum

### An example

**One ending, 10 recipes.** This same firing is the example for bear, breed, perish, age, spoil, stow, discard, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## bear

Run by the **world**.

- **require** 1 citizen, bearing at least 1
- **put** citizen — , bearing one less
- **produce** 1 fertility

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, breed, perish, age, spoil, stow, discard, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## breed

Run by the **world**.

- **consume** 1 fertility
- **consume** 1 food
- **produce** 1 citizen

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, perish, age, spoil, stow, discard, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

### An example

the way the population settles when **the citizens are the lesser**. Two citizens eat two of the eight food, leaving six - but `bear` made only two fertility, one per citizen, so `breed` fires **twice** and stops, with four food it has no fertility left to spend. That is the doubling the old `grow` expression bounded in a word and the `spent` trait bounds in a rule: a population grows by at most itself, however much food there is. **The four it did not spend are gone from the state after**, and that is `age` and `spoil` rather than `breed` - food keeps for one turn, so what a territory is still holding when the turn ends expires with it.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:0 free:3 capacity:3} -> 1
    {food} -> 8
    {garrison} -> 1
    {store resource:food} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 4
    {deposit density:4 resource:food occupied:0 free:3 capacity:3} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
```

## perish

Run by the **world**.

- **consume** 1 citizen, paid 0

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, age, spoil, stow, discard, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## age

Run by the **world**.

- **require** 1 thing, keeps at least 1
- **put** thing — , keeps one less

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, spoil, stow, discard, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## spoil

Run by the **world**.

- **consume** 1 thing, keeps 0

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, stow, discard, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## stow

Run by the **world**.

- **consume** 1 metal
- **produce** 1 metal

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, discard, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## stow

Run by the **world**.

- **consume** 1 energy
- **produce** 1 energy

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, discard, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## discard

Run by the **world**.

- **consume** 1 metal

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## discard

Run by the **world**.

- **consume** 1 energy

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## discard

Run by the **world**.

- **consume** 1 labor

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## discard

Run by the **world**.

- **consume** 1 fertility

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, refresh, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## refresh

Run by the **world**.

- **put** unit — , moving at its maximum

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, discard, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## refresh

Run by the **world**.

- **put** citizen — , laboring at its maximum

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, discard, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## refresh

Run by the **world**.

- **put** citizen — , bearing at its maximum

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, discard, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## refresh

Run by the **world**.

- **put** extractor — , working at its maximum

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, discard, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## refresh

Run by the **world**.

- **put** ark — , working at its maximum

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, discard, renew as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

## renew

Run by the **world**.

- **require** 1 citizen
- **put** citizen — , paid 0

### An example

**One ending, 10 recipes.** This same firing is the example for upkeep, bear, breed, perish, age, spoil, stow, discard, refresh as well - no command fires one of the world's alone.

the world's ten in one ending, in the release's order, and the way the population settles when **the food is the lesser**. Territory 1 has three food for two citizens: `upkeep` feeds both, `bear` turns each of them spent and leaves two fertility, and `breed` fires **once** rather than twice - there is one food left and each new citizen costs one. `renew` makes both parents fertile again. Territory 2 has no food, so its citizen goes unpaid, `breed` cannot fire there at all, and `perish` takes it. What food is left expires, the fertility nobody bred with is discarded, and the worked extractor is ready again. **The pioneer in territory 2 is untouched**, because nothing but a citizen eats - `P-339`. This note used to say it starved and that the file could not show it, which was true of an older rule and of a `usable` trait the release never declared; `P-367` removed the last thing that set that trait, so there is no state a unit can be in now that an artifact cannot show. `C-62`.

Before:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 2
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:0} -> 1
    {food} -> 3
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory id:1 biome:grassland} -> 1
    {citizen bearing:1 laboring:1 paid:0} -> 3
    {deposit density:4 resource:food occupied:1 free:2 capacity:3} -> 1
    {extractor resource:food working:1} -> 1
    {garrison} -> 1
    {store resource:food} -> 1
  {territory id:2 biome:grassland} -> 1
    {garrison} -> 1
    {pioneer id:1 moving:1} -> 1
```
