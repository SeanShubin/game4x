# Recipes

**Generated. Do not edit.** `cargo run -p game-console --bin dump-state`, or `scripts/dump-state.sh`.

Every recipe the release declares, with its own lines gathered under it. The release states
these across seven columns, one row per line and the name written only on the first - which
is the right shape for a table and the wrong one for answering *what does this recipe do*.

**Beside each rule is a worked example** - a state, the command that fires the recipe, and
the state after - in the notation `scenario/expected/play.4x` uses, holding only what that
recipe touched. Every one is a real command run against a real state: `R-7`.

16 recipes, 58 lines between them, 11 worked examples.

## deploy ark

Run by the **player**.

- **require** 1 territory, in `$where`
- **consume** 1 ark, in the orbit above `$where`
- **limit** 0 garrison
- **produce** 1 garrison
- **produce** 2 citizen
- **produce** 1 extractor, food
- **produce** 1 extractor, metal
- **produce** 1 store, food
- **produce** 1 store, metal

### An example

Before:

```
{game phase:play}
  {orbit id:1} -> 1
    {ark fuel:2 id:1 ready:yes} -> 1
  {territory biome:grassland id:1 nature:0} -> 1
    {deposit density:4 resource:food total-capacity:3} -> 1
    {deposit density:4 resource:metal total-capacity:3} -> 1
```

`{deploy-ark territory:1}`

After:

```
{game phase:play}
  {orbit id:1} -> 1
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {deposit density:4 resource:food total-capacity:3} -> 1
    {deposit density:4 resource:metal total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {extractor ready:yes resource:metal} -> 1
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
    {store resource:metal} -> 1
```

## move

Run by the **player**.

- **require** 1 place, in `$from`
- **require** 1 place, joined to `$from` by an edge the unit crosses, in `$to`
- **consume** 1 unit, ready, in `$from`
- **consume** 1 energy, in that unit
- **produce** 1 unit, not ready, in `$to`

### An example

moving spends one fuel and leaves the unit not ready.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
```

`{move unit:pioneer territory:2}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:1 id:1 ready:no} -> 1
```

## found by land

Run by the **player**.

- **consume** 1 pioneer
- **limit** 0 garrison
- **produce** 1 garrison
- **produce** 2 citizen
- **produce** 1 extractor, food
- **produce** 1 extractor, metal
- **produce** 1 store, food
- **produce** 1 store, metal

### An example

the pioneer is consumed and the ground it takes is furnished.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {deposit density:4 resource:food total-capacity:3} -> 1
    {deposit density:4 resource:metal total-capacity:3} -> 1
```

`{found-by-land territory:2}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 2
    {deposit density:4 resource:food total-capacity:3} -> 1
    {deposit density:4 resource:metal total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {extractor ready:yes resource:metal} -> 1
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
    {store resource:metal} -> 1
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
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {deposit density:4 resource:metal total-capacity:3} -> 1
    {garrison force:0 manned:0} -> 1
    {labor} -> 1
    {metal} -> 1
```

`{build-extractor territory:1 resource:metal}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {deposit density:4 resource:metal total-capacity:3} -> 1
    {extractor ready:yes resource:metal} -> 1
    {garrison force:0 manned:0} -> 1
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
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {deposit density:4 resource:metal total-capacity:3} -> 1
    {extractor ready:yes resource:metal} -> 1
    {garrison force:0 manned:0} -> 1
    {labor} -> 1
    {metal} -> 1
```

`{build-store territory:1 resource:metal}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {deposit density:4 resource:metal total-capacity:3} -> 1
    {extractor ready:yes resource:metal} -> 1
    {garrison force:0 manned:0} -> 1
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
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {labor} -> 1
    {metal} -> 15
```

`{build-yard territory:1}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {yard} -> 1
```

## produce pioneer

Run by the **player**.

- **consume** 3 metal
- **consume** 6 energy
- **consume** 2 citizen
- **produce** 1 pioneer

### An example

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {energy} -> 6
    {garrison force:0 manned:0} -> 1
    {metal} -> 3
```

`{produce-pioneer territory:1}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

## launch ark

Run by the **player**.

- **consume** 3 metal
- **consume** 12 energy
- **consume** 2 citizen
- **require** 1 yard

### An example

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {energy} -> 12
    {garrison force:0 manned:0} -> 1
    {metal} -> 3
    {yard} -> 1
```

`{launch-ark territory:1}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {garrison force:0 manned:0} -> 1
    {yard} -> 1
```

## create labor

Run by the **player**.

- **consume** 1 citizen, ready
- **produce** 1 citizen, not ready
- **produce** 1 labor

### An example

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
```

`{create-labor territory:1}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:no} -> 1
    {garrison force:0 manned:0} -> 1
    {labor} -> 1
```

## work

Run by the **player**.

- **require** 1 territory, in `$where`
- **consume** 1 labor
- **consume** 1 extractor, ready
- **produce** 1 extractor, not ready
- **produce** resource — `$where`'s density for that resource

### An example

the density is what one extractor yields, so four food from a `3 x 4`.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {garrison force:0 manned:0} -> 1
    {labor} -> 1
    {store resource:food} -> 1
```

`{work territory:1 resource:food}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 1
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:no resource:food} -> 1
    {food} -> 4
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
```

## upkeep

Run by the **world**.

- **require** 1 thing, with upkeep
- **consume** food — the thing's upkeep

### An example

**One ending, six recipes.** This same firing is the example for grow, perish, age, spoil, refresh as well - no command fires one of the world's alone.

five of the world's six in one ending, in the release's order. Territory 1 has four food for two citizens, so both eat and the two left over grow two more; territory 2 has none, so its citizen goes unpaid and perishes. What food is left is discarded and the worked extractor is ready again. **The pioneer in territory 2 starved too and the file cannot show it** - the model marks it unusable rather than consuming it, and `usable` is a trait the release does not declare, so it reads unchanged. `C-62`.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:no resource:food} -> 1
    {food} -> 4
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 4
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

## grow

Run by the **world**.

- **consume** food — the lesser of the surplus food and the citizens here, surplus
- **produce** citizen — the lesser of the surplus food and the citizens here

### An example

**One ending, six recipes.** This same firing is the example for upkeep, perish, age, spoil, refresh as well - no command fires one of the world's alone.

five of the world's six in one ending, in the release's order. Territory 1 has four food for two citizens, so both eat and the two left over grow two more; territory 2 has none, so its citizen goes unpaid and perishes. What food is left is discarded and the worked extractor is ready again. **The pioneer in territory 2 starved too and the file cannot show it** - the model marks it unusable rather than consuming it, and `usable` is a trait the release does not declare, so it reads unchanged. `C-62`.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:no resource:food} -> 1
    {food} -> 4
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 4
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

## perish

Run by the **world**.

- **consume** 1 thing, whose upkeep is unpaid
- **produce** metal — the thing's metal

### An example

**One ending, six recipes.** This same firing is the example for upkeep, grow, age, spoil, refresh as well - no command fires one of the world's alone.

five of the world's six in one ending, in the release's order. Territory 1 has four food for two citizens, so both eat and the two left over grow two more; territory 2 has none, so its citizen goes unpaid and perishes. What food is left is discarded and the worked extractor is ready again. **The pioneer in territory 2 starved too and the file cannot show it** - the model marks it unusable rather than consuming it, and `usable` is a trait the release does not declare, so it reads unchanged. `C-62`.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:no resource:food} -> 1
    {food} -> 4
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 4
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

## age

Run by the **world**.

- **consume** 1 thing, keeps at least 1
- **produce** 1 thing, keeps one less

### An example

**One ending, six recipes.** This same firing is the example for upkeep, grow, perish, spoil, refresh as well - no command fires one of the world's alone.

five of the world's six in one ending, in the release's order. Territory 1 has four food for two citizens, so both eat and the two left over grow two more; territory 2 has none, so its citizen goes unpaid and perishes. What food is left is discarded and the worked extractor is ready again. **The pioneer in territory 2 starved too and the file cannot show it** - the model marks it unusable rather than consuming it, and `usable` is a trait the release does not declare, so it reads unchanged. `C-62`.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:no resource:food} -> 1
    {food} -> 4
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 4
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

## spoil

Run by the **world**.

- **consume** 1 thing, keeps 0

### An example

**One ending, six recipes.** This same firing is the example for upkeep, grow, perish, age, refresh as well - no command fires one of the world's alone.

five of the world's six in one ending, in the release's order. Territory 1 has four food for two citizens, so both eat and the two left over grow two more; territory 2 has none, so its citizen goes unpaid and perishes. What food is left is discarded and the worked extractor is ready again. **The pioneer in territory 2 starved too and the file cannot show it** - the model marks it unusable rather than consuming it, and `usable` is a trait the release does not declare, so it reads unchanged. `C-62`.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:no resource:food} -> 1
    {food} -> 4
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 4
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

## refresh

Run by the **world**.

- **consume** 1 thing, not ready
- **produce** 1 thing, ready

### An example

**One ending, six recipes.** This same firing is the example for upkeep, grow, perish, age, spoil as well - no command fires one of the world's alone.

five of the world's six in one ending, in the release's order. Territory 1 has four food for two citizens, so both eat and the two left over grow two more; territory 2 has none, so its citizen goes unpaid and perishes. What food is left is discarded and the worked extractor is ready again. **The pioneer in territory 2 starved too and the file cannot show it** - the model marks it unusable rather than consuming it, and `usable` is a trait the release does not declare, so it reads unchanged. `C-62`.

Before:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 2
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:no resource:food} -> 1
    {food} -> 4
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {citizen ready:yes} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```

`{end-turn}`

After:

```
{game phase:play}
  {territory biome:grassland id:1 nature:0} -> 1
    {citizen ready:yes} -> 4
    {deposit density:4 resource:food total-capacity:3} -> 1
    {extractor ready:yes resource:food} -> 1
    {garrison force:0 manned:0} -> 1
    {store resource:food} -> 1
  {territory biome:grassland id:2 nature:0} -> 1
    {garrison force:0 manned:0} -> 1
    {pioneer fuel:2 id:1 ready:yes} -> 1
```
