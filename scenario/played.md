# The main scenario, played

**Generated. Do not edit.** `scripts/scenario.sh`. The rules are `spec/data/`, the world
and the act are `scenario/main.4x`, and this file is what happened when they met.

**`spec/scenarios.md`: it is vetted by hand.** So this is written to be read rather than
to pass; `crates/game-model/tests/scenario.rs` is the part a gate holds.

15 rules in the ruleset, 35 commands, 5 turns.

Every place is written the way `scenario/main.4x` writes it: `place-1` is the surface
landed on, `place-2` the orbit above it, `place-3` the surface taken by land, `place-4`
the orbit above that.

## Turn 1

    {move what:ark from:place-4 to:place-2}
    {gather where:place-2}
    {deploy where:place-2 what:ark}
    {toil where:place-1}
    {work where:place-1 what:food}
    {end-turn}

  place-1: 4 citizen, 2 deposit, 2 extractor
  place-2: 1 deposit
  place-3: 3 deposit

## Turn 2

    {toil where:place-1}
    {work where:place-1 what:food}
    {work where:place-1 what:metal}
    {build-pioneer where:place-1}
    {move what:pioneer from:place-1 to:place-3}
    {end-turn}

  place-1: 6 citizen, 2 deposit, 2 extractor
  place-2: 1 deposit
  place-3: 3 deposit, 1 pioneer

## Turn 3

    {deploy where:place-3 what:pioneer}
    {toil where:place-1}
    {work where:place-1 what:food}
    {toil where:place-3}
    {work where:place-3 what:food}
    {end-turn}

  place-1: 6 citizen, 2 deposit, 2 extractor
  place-2: 1 deposit
  place-3: 4 citizen, 3 deposit, 2 extractor

## Turn 4

    {toil where:place-1}
    {work where:place-1 what:food}
    {work where:place-1 what:metal}
    {build-bin where:place-1 what:metal}
    {toil where:place-3}
    {work where:place-3 what:food}
    {work where:place-3 what:metal}
    {end-turn}

  place-1: 1 bin, 6 citizen, 2 deposit, 2 extractor, 5 metal
  place-2: 1 deposit
  place-3: 6 citizen, 3 deposit, 2 extractor

## Turn 5

    {toil where:place-1}
    {work where:place-1 what:food}
    {toil where:place-3}
    {work where:place-3 what:food}
    {work where:place-3 what:metal}
    {build-extractor where:place-3 what:energy}
    {work where:place-3 what:energy}
    {launch where:place-3}
    {end-turn}

  place-1: 1 bin, 6 citizen, 2 deposit, 2 extractor, 5 metal
  place-2: 1 deposit
  place-3: 6 citizen, 3 deposit, 3 extractor
  place-4: 1 ark

## What fired

    breed              8
    build-bin          1
    build-extractor    1
    build-pioneer      1
    deploy             2
    discard-disorder   5
    end-turn           5
    gather             1
    launch             1
    move               2
    perish             -   never
    refresh            30
    toil               36
    upkeep             32
    work               13

14 of 15 rules fired; 1 did not: ["perish"]

### What a typical game does not use

**`perish`** needs a starvation - a settlement whose citizens are hungry when its food runs out. The main scenario works its food every turn and sustains its people, so nobody starves in it.

`spec/scenarios.md`: *a mechanic that only appears in an unusual situation belongs to
a scenario of its own. Those are not built until the main scenario satisfies its
reader.*

## Columns a rule leaves as it found them

**`move`** acts on `unit`; `ark` carries `gathering` that no clause names.

`spec/invariants.md`: *what it does not name it leaves as it found it.* These are
carried through rather than refused - `P-573`, which chose a report over making the
notation say so.

## The world it left

    {adjacency id:1 from:territory-1 to:territory-2}
    {adjacency id:2 from:territory-2 to:territory-1}
    {ark where:place-4 moving:1 gathering:1} -> 1
    {bin where:place-1 what:metal} -> 1
    {capacity of:bin for:energy what:energy per:place} -> 10
    {capacity of:bin for:food what:food per:place} -> 10
    {capacity of:bin for:metal what:metal per:place} -> 10
    {capacity of:deposit for:extractor what:energy per:place} -> 1
    {capacity of:deposit for:extractor what:food per:place} -> 1
    {capacity of:deposit for:extractor what:metal per:place} -> 1
    {capacity of:place for:bin what:energy per:place} -> 2
    {capacity of:place for:bin what:food per:place} -> 2
    {capacity of:place for:bin what:metal per:place} -> 2
    {citizen where:place-1 hungry:1 bearing:1 laboring:1} -> 6
    {citizen where:place-3 hungry:1 bearing:1 laboring:1} -> 6
    {consumes kind:pioneer what:berth} -> 1
    {deposit where:place-1 what:food density:6} -> 1
    {deposit where:place-1 what:metal density:6} -> 1
    {deposit where:place-2 what:energy density:3} -> 1
    {deposit where:place-3 what:energy density:6} -> 1
    {deposit where:place-3 what:food density:6} -> 1
    {deposit where:place-3 what:metal density:6} -> 1
    {extractor where:place-1 what:food working:1} -> 1
    {extractor where:place-1 what:metal working:1} -> 1
    {extractor where:place-3 what:energy working:1} -> 1
    {extractor where:place-3 what:food working:1} -> 1
    {extractor where:place-3 what:metal working:1} -> 1
    {metal where:place-1} -> 5
    {place id:1 of:territory-1 layer:surface name:place-1}
    {place id:2 of:territory-1 layer:orbit name:place-2}
    {place id:3 of:territory-2 layer:surface name:place-3}
    {place id:4 of:territory-2 layer:orbit name:place-4}
    {provides kind:place what:berth} -> 6
    {territory id:1 name:territory-1}
    {territory id:2 name:territory-2}

35 row(s) of world, out of 662 in the store.

## What every command took and made

### Turn 1: {move what:ark from:place-4 to:place-2}

    fired move
    took  {ark moving:1 quantity:1 where:4}
    made  {ark gathering:1 moving:0 quantity:1 where:2}

### Turn 1: {gather where:place-2}

    fired gather
    took  {ark gathering:1 quantity:1 where:2}
    made  {ark gathering:0 moving:0 quantity:1 where:2}
    made  {energy quantity:3 where:2}

### Turn 1: {deploy where:place-2 what:ark}

    fired deploy
    took  {ark quantity:1 where:2}
    made  {extractor quantity:1 what:30 where:1 working:1}
    made  {extractor quantity:1 what:31 where:1 working:1}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:1}

### Turn 1: {toil where:place-1}

    fired toil, toil
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}

### Turn 1: {work where:place-1 what:food}

    fired work
    took  {extractor quantity:1 what:31 where:1 working:1}
    took  {labor quantity:1 where:1}
    made  {extractor quantity:1 what:31 where:1 working:0}
    made  {food quantity:6 where:1}

### Turn 1: {end-turn}

    fired end-turn, breed, breed, discard-disorder, refresh, refresh, refresh, refresh, refresh, refresh
    took  {citizen bearing:1 hungry:0 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen bearing:1 hungry:0 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {food quantity:4 where:1}
    took  {labor quantity:1 where:1}
    took  {energy quantity:3 where:2}
    took  {extractor quantity:1 what:31 where:1 working:0}
    took  {citizen bearing:1 hungry:0 laboring:1 quantity:4 where:1}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:1}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:1}
    made  {extractor quantity:1 what:31 where:1 working:1}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:4 where:1}

### Turn 2: {toil where:place-1}

    fired toil, toil, toil, toil
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}

### Turn 2: {work where:place-1 what:food}

    fired work
    took  {extractor quantity:1 what:31 where:1 working:1}
    took  {labor quantity:1 where:1}
    made  {extractor quantity:1 what:31 where:1 working:0}
    made  {food quantity:6 where:1}

### Turn 2: {work where:place-1 what:metal}

    fired work
    took  {extractor quantity:1 what:30 where:1 working:1}
    took  {labor quantity:1 where:1}
    made  {extractor quantity:1 what:30 where:1 working:0}
    made  {metal quantity:6 where:1}

### Turn 2: {build-pioneer where:place-1}

    fired build-pioneer
    took  {labor quantity:1 where:1}
    took  {metal quantity:1 where:1}
    made  {pioneer moving:1 quantity:1 where:1}

### Turn 2: {move what:pioneer from:place-1 to:place-3}

    fired move
    took  {pioneer moving:1 quantity:1 where:1}
    made  {pioneer moving:0 quantity:1 where:3}

### Turn 2: {end-turn}

    fired end-turn, upkeep, upkeep, upkeep, upkeep, breed, breed, discard-disorder, refresh, refresh, refresh, refresh, refresh, refresh
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen bearing:1 hungry:0 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen bearing:1 hungry:0 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {metal quantity:5 where:1}
    took  {labor quantity:1 where:1}
    took  {pioneer moving:0 quantity:1 where:3}
    took  {extractor quantity:1 what:31 where:1 working:0}
    took  {extractor quantity:1 what:30 where:1 working:0}
    took  {citizen bearing:1 hungry:0 laboring:0 quantity:2 where:1}
    took  {citizen bearing:1 hungry:0 laboring:1 quantity:4 where:1}
    took  {citizen bearing:1 hungry:1 laboring:0 quantity:2 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:1}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:1}
    made  {pioneer moving:1 quantity:1 where:3}
    made  {extractor quantity:1 what:31 where:1 working:1}
    made  {extractor quantity:1 what:30 where:1 working:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:2 where:1}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:4 where:1}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:2 where:1}

### Turn 3: {deploy where:place-3 what:pioneer}

    fired deploy
    took  {pioneer quantity:1 where:3}
    made  {extractor quantity:1 what:30 where:3 working:1}
    made  {extractor quantity:1 what:31 where:3 working:1}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:3}

### Turn 3: {toil where:place-1}

    fired toil, toil, toil, toil, toil, toil
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}

### Turn 3: {work where:place-1 what:food}

    fired work
    took  {extractor quantity:1 what:31 where:1 working:1}
    took  {labor quantity:1 where:1}
    made  {extractor quantity:1 what:31 where:1 working:0}
    made  {food quantity:6 where:1}

### Turn 3: {toil where:place-3}

    fired toil, toil
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}

### Turn 3: {work where:place-3 what:food}

    fired work
    took  {extractor quantity:1 what:31 where:3 working:1}
    took  {labor quantity:1 where:3}
    made  {extractor quantity:1 what:31 where:3 working:0}
    made  {food quantity:6 where:3}

### Turn 3: {end-turn}

    fired end-turn, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, breed, breed, discard-disorder, refresh, refresh, refresh, refresh, refresh, refresh
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen bearing:1 hungry:0 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen bearing:1 hungry:0 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {food quantity:4 where:3}
    took  {labor quantity:5 where:1}
    took  {labor quantity:1 where:3}
    took  {extractor quantity:1 what:31 where:1 working:0}
    took  {extractor quantity:1 what:31 where:3 working:0}
    took  {citizen bearing:1 hungry:0 laboring:0 quantity:6 where:1}
    took  {citizen bearing:1 hungry:0 laboring:1 quantity:4 where:3}
    took  {citizen bearing:1 hungry:1 laboring:0 quantity:6 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:3}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:3}
    made  {extractor quantity:1 what:31 where:1 working:1}
    made  {extractor quantity:1 what:31 where:3 working:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:6 where:1}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:4 where:3}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:6 where:1}

### Turn 4: {toil where:place-1}

    fired toil, toil, toil, toil, toil, toil
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}

### Turn 4: {work where:place-1 what:food}

    fired work
    took  {extractor quantity:1 what:31 where:1 working:1}
    took  {labor quantity:1 where:1}
    made  {extractor quantity:1 what:31 where:1 working:0}
    made  {food quantity:6 where:1}

### Turn 4: {work where:place-1 what:metal}

    fired work
    took  {extractor quantity:1 what:30 where:1 working:1}
    took  {labor quantity:1 where:1}
    made  {extractor quantity:1 what:30 where:1 working:0}
    made  {metal quantity:6 where:1}

### Turn 4: {build-bin where:place-1 what:metal}

    fired build-bin
    took  {labor quantity:1 where:1}
    took  {metal quantity:1 where:1}
    made  {bin quantity:1 what:30 where:1}

### Turn 4: {toil where:place-3}

    fired toil, toil, toil, toil
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}

### Turn 4: {work where:place-3 what:food}

    fired work
    took  {extractor quantity:1 what:31 where:3 working:1}
    took  {labor quantity:1 where:3}
    made  {extractor quantity:1 what:31 where:3 working:0}
    made  {food quantity:6 where:3}

### Turn 4: {work where:place-3 what:metal}

    fired work
    took  {extractor quantity:1 what:30 where:3 working:1}
    took  {labor quantity:1 where:3}
    made  {extractor quantity:1 what:30 where:3 working:0}
    made  {metal quantity:6 where:3}

### Turn 4: {end-turn}

    fired end-turn, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, breed, breed, discard-disorder, refresh, refresh, refresh, refresh, refresh, refresh
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen bearing:1 hungry:0 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen bearing:1 hungry:0 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {metal quantity:6 where:3}
    took  {labor quantity:3 where:1}
    took  {labor quantity:2 where:3}
    took  {extractor quantity:1 what:31 where:1 working:0}
    took  {extractor quantity:1 what:30 where:1 working:0}
    took  {extractor quantity:1 what:31 where:3 working:0}
    took  {extractor quantity:1 what:30 where:3 working:0}
    took  {citizen bearing:1 hungry:0 laboring:0 quantity:6 where:1}
    took  {citizen bearing:1 hungry:0 laboring:0 quantity:2 where:3}
    took  {citizen bearing:1 hungry:0 laboring:1 quantity:4 where:3}
    took  {citizen bearing:1 hungry:1 laboring:0 quantity:6 where:1}
    took  {citizen bearing:1 hungry:1 laboring:0 quantity:2 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:3}
    made  {citizen bearing:1 hungry:0 laboring:1 quantity:2 where:3}
    made  {extractor quantity:1 what:31 where:1 working:1}
    made  {extractor quantity:1 what:30 where:1 working:1}
    made  {extractor quantity:1 what:31 where:3 working:1}
    made  {extractor quantity:1 what:30 where:3 working:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:6 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:2 where:3}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:4 where:3}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:6 where:1}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:2 where:3}

### Turn 5: {toil where:place-1}

    fired toil, toil, toil, toil, toil, toil
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    took  {citizen laboring:1 quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:1}
    made  {labor quantity:1 where:1}

### Turn 5: {work where:place-1 what:food}

    fired work
    took  {extractor quantity:1 what:31 where:1 working:1}
    took  {labor quantity:1 where:1}
    made  {extractor quantity:1 what:31 where:1 working:0}
    made  {food quantity:6 where:1}

### Turn 5: {toil where:place-3}

    fired toil, toil, toil, toil, toil, toil
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    took  {citizen laboring:1 quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:1 where:3}
    made  {labor quantity:1 where:3}

### Turn 5: {work where:place-3 what:food}

    fired work
    took  {extractor quantity:1 what:31 where:3 working:1}
    took  {labor quantity:1 where:3}
    made  {extractor quantity:1 what:31 where:3 working:0}
    made  {food quantity:6 where:3}

### Turn 5: {work where:place-3 what:metal}

    fired work
    took  {extractor quantity:1 what:30 where:3 working:1}
    took  {labor quantity:1 where:3}
    made  {extractor quantity:1 what:30 where:3 working:0}
    made  {metal quantity:6 where:3}

### Turn 5: {build-extractor where:place-3 what:energy}

    fired build-extractor
    took  {labor quantity:1 where:3}
    took  {metal quantity:1 where:3}
    made  {extractor quantity:1 what:50 where:3 working:1}

### Turn 5: {work where:place-3 what:energy}

    fired work
    took  {extractor quantity:1 what:50 where:3 working:1}
    took  {labor quantity:1 where:3}
    made  {extractor quantity:1 what:50 where:3 working:0}
    made  {energy quantity:6 where:3}

### Turn 5: {launch where:place-3}

    fired launch
    took  {labor quantity:1 where:3}
    took  {metal quantity:1 where:3}
    took  {energy quantity:1 where:3}
    made  {ark gathering:1 moving:1 quantity:1 where:4}

### Turn 5: {end-turn}

    fired end-turn, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, upkeep, discard-disorder, refresh, refresh, refresh, refresh, refresh, refresh
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:1}
    took  {food quantity:1 where:1}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {citizen hungry:1 quantity:1 where:3}
    took  {food quantity:1 where:3}
    took  {metal quantity:4 where:3}
    took  {labor quantity:5 where:1}
    took  {labor quantity:1 where:3}
    took  {energy quantity:5 where:3}
    took  {extractor quantity:1 what:31 where:1 working:0}
    took  {extractor quantity:1 what:31 where:3 working:0}
    took  {extractor quantity:1 what:30 where:3 working:0}
    took  {extractor quantity:1 what:50 where:3 working:0}
    took  {citizen bearing:1 hungry:0 laboring:0 quantity:6 where:1}
    took  {citizen bearing:1 hungry:0 laboring:0 quantity:6 where:3}
    took  {citizen bearing:1 hungry:1 laboring:0 quantity:6 where:1}
    took  {citizen bearing:1 hungry:1 laboring:0 quantity:6 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:1}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {citizen bearing:1 hungry:0 laboring:0 quantity:1 where:3}
    made  {extractor quantity:1 what:31 where:1 working:1}
    made  {extractor quantity:1 what:31 where:3 working:1}
    made  {extractor quantity:1 what:30 where:3 working:1}
    made  {extractor quantity:1 what:50 where:3 working:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:6 where:1}
    made  {citizen bearing:1 hungry:1 laboring:0 quantity:6 where:3}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:6 where:1}
    made  {citizen bearing:1 hungry:1 laboring:1 quantity:6 where:3}

