# The rules as a net

Generated from `spec/data/rules.4x` by `scripts/reports.sh`. **Not canonical** - the rules are, and this is a drawing of them.

[The index](index.md) · [whether it can come back round with more](nogain.md)

**32 places and 32 transitions**, drawn one rule at a time. A circle is a place - a kind of thing in a state - and a rectangle is a rule. An arc into the rectangle is what the rule takes; an arc out of it is what it makes, and a number on an arc is how many. **There is no drawing of the whole net**, because one that held every arc could not give every node a readable name - so each part says what it leaves out, by naming the other rules that reach the same places.

## Places nothing fills (3)

These can only fall. **The planet and time are meant to be here** - they are the endless wells the invariant names, and a well nothing refills is what an endless well looks like in a net. **Anything else here is a kind the game can spend and cannot make.**

- `citizen, bearing 0` - nothing fills it; emptied by `refresh (citizen, bearing)`
- `the planet` - nothing fills it; emptied by `work (metal)`, `work (metal), per unit of density`, `work (food)`, `work (food), per unit of density`, `work (energy)`, `work (energy), per unit of density`, `gather`, `gather, per unit of density` - **a named source**, so this is what it is meant to be
- `time` - nothing fills it; emptied by `refresh (scout, moving)`, `refresh (ark, moving)`, `refresh (transport, moving)`, `refresh (pioneer, moving)`, `refresh (extractor, working)`, `refresh (citizen, hungry)`, `refresh (citizen, bearing)`, `refresh (citizen, laboring)`, `refresh (ark, gathering)` - **a named source**, so this is what it is meant to be

## Places nothing empties (2)

These can only rise. **That is the shape of an unbounded accumulation**, which is not a defect on its own - a structure that is built and never taken down is a choice - but it is where to look when one is suspected.

- `bin` - nothing empties it; filled by `build-bin`
- `yard` - nothing empties it; filled by `build-yard`

## move (scout)

- takes: scout · scout, moving 1
- makes: scout · scout, moving 0
- shares: `scout, moving 0` · `scout, moving 1`
- leaves out: the 1 other rule(s) that reach them - `refresh (scout, moving)`

## move (ark)

- takes: ark · ark, moving 1
- makes: ark · ark, moving 0
- shares: `ark` · `ark, moving 0` · `ark, moving 1`
- leaves out: the 4 other rule(s) that reach them - `deploy (ark)`, `gather`, `launch`, `refresh (ark, moving)`

## move (transport)

- takes: transport · transport, moving 1
- makes: transport · transport, moving 0
- shares: `transport, moving 0` · `transport, moving 1`
- leaves out: the 1 other rule(s) that reach them - `refresh (transport, moving)`

## move (pioneer)

- takes: pioneer · pioneer, moving 1
- makes: pioneer · pioneer, moving 0
- shares: `pioneer` · `pioneer, moving 0` · `pioneer, moving 1`
- leaves out: the 3 other rule(s) that reach them - `build-pioneer`, `deploy (pioneer)`, `refresh (pioneer, moving)`

## build-extractor

- takes: labor · metal
- makes: extractor · extractor, working 1
- shares: `extractor` · `extractor, working 1` · `labor` · `metal`
- leaves out: the 12 other rule(s) that reach them - `build-bin`, `build-pioneer`, `build-yard`, `deploy (ark)`, `deploy (pioneer)`, `launch`, `refresh (extractor, working)`, `toil`, `work (energy)`, `work (food)`, `work (metal)`, `work (metal), per unit of density`

## work (metal)

- takes: extractor · extractor, working 1 · labor · the planet
- makes: extractor · extractor, working 0 · metal
- shares: `extractor` · `extractor, working 0` · `extractor, working 1` · `labor` · `metal` · `the planet`
- leaves out: the 16 other rule(s) that reach them - `build-bin`, `build-extractor`, `build-pioneer`, `build-yard`, `deploy (ark)`, `deploy (pioneer)`, `gather`, `gather, per unit of density`, `launch`, `refresh (extractor, working)`, `toil`, `work (energy)`, `work (energy), per unit of density`, `work (food)`, `work (food), per unit of density`, `work (metal), per unit of density`

## work (metal), per unit of density

- takes: the planet
- makes: metal
- shares: `metal` · `the planet`
- leaves out: the 12 other rule(s) that reach them - `build-bin`, `build-extractor`, `build-pioneer`, `build-yard`, `gather`, `gather, per unit of density`, `launch`, `work (energy)`, `work (energy), per unit of density`, `work (food)`, `work (food), per unit of density`, `work (metal)`

## work (food)

- takes: extractor · extractor, working 1 · labor · the planet
- makes: extractor · extractor, working 0 · food
- shares: `extractor` · `extractor, working 0` · `extractor, working 1` · `food` · `labor` · `the planet`
- leaves out: the 18 other rule(s) that reach them - `breed`, `build-bin`, `build-extractor`, `build-pioneer`, `build-yard`, `deploy (ark)`, `deploy (pioneer)`, `gather`, `gather, per unit of density`, `launch`, `refresh (extractor, working)`, `toil`, `upkeep`, `work (energy)`, `work (energy), per unit of density`, `work (food), per unit of density`, `work (metal)`, `work (metal), per unit of density`

## work (food), per unit of density

- takes: the planet
- makes: food
- shares: `food` · `the planet`
- leaves out: the 9 other rule(s) that reach them - `breed`, `gather`, `gather, per unit of density`, `upkeep`, `work (energy)`, `work (energy), per unit of density`, `work (food)`, `work (metal)`, `work (metal), per unit of density`

## work (energy)

- takes: extractor · extractor, working 1 · labor · the planet
- makes: energy · extractor · extractor, working 0
- shares: `energy` · `extractor` · `extractor, working 0` · `extractor, working 1` · `labor` · `the planet`
- leaves out: the 16 other rule(s) that reach them - `build-bin`, `build-extractor`, `build-pioneer`, `build-yard`, `deploy (ark)`, `deploy (pioneer)`, `gather`, `gather, per unit of density`, `launch`, `refresh (extractor, working)`, `toil`, `work (energy), per unit of density`, `work (food)`, `work (food), per unit of density`, `work (metal)`, `work (metal), per unit of density`

## work (energy), per unit of density

- takes: the planet
- makes: energy
- shares: `energy` · `the planet`
- leaves out: the 8 other rule(s) that reach them - `gather`, `gather, per unit of density`, `launch`, `work (energy)`, `work (food)`, `work (food), per unit of density`, `work (metal)`, `work (metal), per unit of density`

## refresh (scout, moving)

- takes: scout, moving 0 · time
- makes: scout, moving 1
- shares: `scout, moving 0` · `scout, moving 1` · `time`
- leaves out: the 9 other rule(s) that reach them - `move (scout)`, `refresh (ark, gathering)`, `refresh (ark, moving)`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `refresh (pioneer, moving)`, `refresh (transport, moving)`

## refresh (ark, moving)

- takes: ark, moving 0 · time
- makes: ark, moving 1
- shares: `ark, moving 0` · `ark, moving 1` · `time`
- leaves out: the 10 other rule(s) that reach them - `launch`, `move (ark)`, `refresh (ark, gathering)`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `refresh (pioneer, moving)`, `refresh (scout, moving)`, `refresh (transport, moving)`

## refresh (transport, moving)

- takes: time · transport, moving 0
- makes: transport, moving 1
- shares: `time` · `transport, moving 0` · `transport, moving 1`
- leaves out: the 9 other rule(s) that reach them - `move (transport)`, `refresh (ark, gathering)`, `refresh (ark, moving)`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `refresh (pioneer, moving)`, `refresh (scout, moving)`

## refresh (pioneer, moving)

- takes: pioneer, moving 0 · time
- makes: pioneer, moving 1
- shares: `pioneer, moving 0` · `pioneer, moving 1` · `time`
- leaves out: the 10 other rule(s) that reach them - `build-pioneer`, `move (pioneer)`, `refresh (ark, gathering)`, `refresh (ark, moving)`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `refresh (scout, moving)`, `refresh (transport, moving)`

## refresh (extractor, working)

- takes: extractor, working 0 · time
- makes: extractor, working 1
- shares: `extractor, working 0` · `extractor, working 1` · `time`
- leaves out: the 14 other rule(s) that reach them - `build-extractor`, `deploy (ark)`, `deploy (pioneer)`, `refresh (ark, gathering)`, `refresh (ark, moving)`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (pioneer, moving)`, `refresh (scout, moving)`, `refresh (transport, moving)`, `work (energy)`, `work (food)`, `work (metal)`

## refresh (citizen, hungry)

- takes: citizen, hungry 0 · time
- makes: citizen, hungry 1
- shares: `citizen, hungry 0` · `citizen, hungry 1` · `time`
- leaves out: the 13 other rule(s) that reach them - `breed`, `deploy (ark)`, `deploy (pioneer)`, `perish`, `refresh (ark, gathering)`, `refresh (ark, moving)`, `refresh (citizen, bearing)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `refresh (pioneer, moving)`, `refresh (scout, moving)`, `refresh (transport, moving)`, `upkeep`

## refresh (citizen, bearing)

- takes: citizen, bearing 0 · time
- makes: citizen, bearing 1
- shares: `citizen, bearing 1` · `time`
- leaves out: the 11 other rule(s) that reach them - `breed`, `deploy (ark)`, `deploy (pioneer)`, `refresh (ark, gathering)`, `refresh (ark, moving)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `refresh (pioneer, moving)`, `refresh (scout, moving)`, `refresh (transport, moving)`

## refresh (citizen, laboring)

- takes: citizen, laboring 0 · time
- makes: citizen, laboring 1
- shares: `citizen, laboring 0` · `citizen, laboring 1` · `time`
- leaves out: the 12 other rule(s) that reach them - `breed`, `deploy (ark)`, `deploy (pioneer)`, `refresh (ark, gathering)`, `refresh (ark, moving)`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (extractor, working)`, `refresh (pioneer, moving)`, `refresh (scout, moving)`, `refresh (transport, moving)`, `toil`

## refresh (ark, gathering)

- takes: ark, gathering 0 · time
- makes: ark, gathering 1
- shares: `ark, gathering 0` · `ark, gathering 1` · `time`
- leaves out: the 10 other rule(s) that reach them - `gather`, `launch`, `refresh (ark, moving)`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `refresh (pioneer, moving)`, `refresh (scout, moving)`, `refresh (transport, moving)`

## build-bin

- takes: labor · metal
- makes: bin
- shares: `labor` · `metal`
- leaves out: the 9 other rule(s) that reach them - `build-extractor`, `build-pioneer`, `build-yard`, `launch`, `toil`, `work (energy)`, `work (food)`, `work (metal)`, `work (metal), per unit of density`

## upkeep

- takes: citizen · citizen, hungry 1 · food
- makes: citizen · citizen, hungry 0
- shares: `citizen` · `citizen, hungry 0` · `citizen, hungry 1` · `food`
- leaves out: the 8 other rule(s) that reach them - `breed`, `deploy (ark)`, `deploy (pioneer)`, `perish`, `refresh (citizen, hungry)`, `toil`, `work (food)`, `work (food), per unit of density`

## perish

- takes: citizen · citizen, hungry 1
- makes: —
- shares: `citizen` · `citizen, hungry 1`
- leaves out: the 6 other rule(s) that reach them - `breed`, `deploy (ark)`, `deploy (pioneer)`, `refresh (citizen, hungry)`, `toil`, `upkeep`

## breed

- takes: citizen · citizen, bearing 1 · citizen, hungry 0 · food
- makes: 2 × citizen · 2 × citizen, bearing 1 · 2 × citizen, hungry 0 · 2 × citizen, laboring 1
- shares: `citizen` · `citizen, bearing 1` · `citizen, hungry 0` · `citizen, laboring 1` · `food`
- leaves out: the 10 other rule(s) that reach them - `deploy (ark)`, `deploy (pioneer)`, `perish`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `toil`, `upkeep`, `work (food)`, `work (food), per unit of density`

## toil

- takes: citizen · citizen, laboring 1
- makes: citizen · citizen, laboring 0 · labor
- shares: `citizen` · `citizen, laboring 0` · `citizen, laboring 1` · `labor`
- leaves out: the 14 other rule(s) that reach them - `breed`, `build-bin`, `build-extractor`, `build-pioneer`, `build-yard`, `deploy (ark)`, `deploy (pioneer)`, `launch`, `perish`, `refresh (citizen, laboring)`, `upkeep`, `work (energy)`, `work (food)`, `work (metal)`

## gather

- takes: ark · ark, gathering 1 · the planet
- makes: ark · ark, gathering 0 · energy
- shares: `ark` · `ark, gathering 0` · `ark, gathering 1` · `energy` · `the planet`
- leaves out: the 11 other rule(s) that reach them - `deploy (ark)`, `gather, per unit of density`, `launch`, `move (ark)`, `refresh (ark, gathering)`, `work (energy)`, `work (energy), per unit of density`, `work (food)`, `work (food), per unit of density`, `work (metal)`, `work (metal), per unit of density`

## gather, per unit of density

- takes: the planet
- makes: energy
- shares: `energy` · `the planet`
- leaves out: the 8 other rule(s) that reach them - `gather`, `launch`, `work (energy)`, `work (energy), per unit of density`, `work (food)`, `work (food), per unit of density`, `work (metal)`, `work (metal), per unit of density`

## launch

- takes: energy · labor · metal
- makes: ark · ark, gathering 1 · ark, moving 1
- shares: `ark` · `ark, gathering 1` · `ark, moving 1` · `energy` · `labor` · `metal`
- leaves out: the 16 other rule(s) that reach them - `build-bin`, `build-extractor`, `build-pioneer`, `build-yard`, `deploy (ark)`, `gather`, `gather, per unit of density`, `move (ark)`, `refresh (ark, gathering)`, `refresh (ark, moving)`, `toil`, `work (energy)`, `work (energy), per unit of density`, `work (food)`, `work (metal)`, `work (metal), per unit of density`

## deploy (pioneer)

- takes: pioneer
- makes: 2 × citizen · 2 × citizen, bearing 1 · 2 × citizen, hungry 0 · 2 × citizen, laboring 1 · 2 × extractor · 2 × extractor, working 1
- shares: `citizen` · `citizen, bearing 1` · `citizen, hungry 0` · `citizen, laboring 1` · `extractor` · `extractor, working 1` · `pioneer`
- leaves out: the 15 other rule(s) that reach them - `breed`, `build-extractor`, `build-pioneer`, `deploy (ark)`, `move (pioneer)`, `perish`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `toil`, `upkeep`, `work (energy)`, `work (food)`, `work (metal)`

## deploy (ark)

- takes: ark
- makes: 2 × citizen · 2 × citizen, bearing 1 · 2 × citizen, hungry 0 · 2 × citizen, laboring 1 · 2 × extractor · 2 × extractor, working 1
- shares: `ark` · `citizen` · `citizen, bearing 1` · `citizen, hungry 0` · `citizen, laboring 1` · `extractor` · `extractor, working 1`
- leaves out: the 16 other rule(s) that reach them - `breed`, `build-extractor`, `deploy (pioneer)`, `gather`, `launch`, `move (ark)`, `perish`, `refresh (citizen, bearing)`, `refresh (citizen, hungry)`, `refresh (citizen, laboring)`, `refresh (extractor, working)`, `toil`, `upkeep`, `work (energy)`, `work (food)`, `work (metal)`

## build-pioneer

- takes: labor · metal
- makes: pioneer · pioneer, moving 1
- shares: `labor` · `metal` · `pioneer` · `pioneer, moving 1`
- leaves out: the 12 other rule(s) that reach them - `build-bin`, `build-extractor`, `build-yard`, `deploy (pioneer)`, `launch`, `move (pioneer)`, `refresh (pioneer, moving)`, `toil`, `work (energy)`, `work (food)`, `work (metal)`, `work (metal), per unit of density`

## build-yard

- takes: labor · metal
- makes: yard
- shares: `labor` · `metal`
- leaves out: the 9 other rule(s) that reach them - `build-bin`, `build-extractor`, `build-pioneer`, `launch`, `toil`, `work (energy)`, `work (food)`, `work (metal)`, `work (metal), per unit of density`
