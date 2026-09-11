# Nothing comes back round with more

**Generated. Do not edit.** Read out of the release's *Recipes* table by `crates/game-console/src/nogain.rs`.

`spec/invariants.md` says there is a weighting of the kinds under which no sequence of rules ends holding more than it began with, and that **whether this holds is decided mechanically, from the rules alone**. This is that decision. `S-93`.

**Nothing here is declared.** The weighting below is solved for, not read from anywhere: the only inputs are the release's rules. So a number that looks wrong is a fact about the rules rather than about somebody's judgement of them - and one that is sound but says something you would reject is visible for that reason.

**43 rules**, ground from **24** blocks of recipe rows: a family becomes its members, a density becomes its cases. **21 of them draw on a source** - the planet, the star or time - and a source is a place they take from rather than an exemption from the arithmetic. That is what keeps them in the check instead of out of it.

## A weighting exists, so nothing comes back round with more

Every rule is non-increasing under the weighting below, so any sequence of them is too - a cycle included. **It may end with less**, which is disorder and is meant.

| Place                           | Weight | What it is                  |
| ------------------------------- | ------ | --------------------------- |
| ark                             | 20     | a thing                     |
| pioneer                         | 20     | a thing                     |
| the planet                      | 20     | an endless well             |
| the star                        | 12     | an endless well             |
| food                            | 4      | a thing                     |
| metal                           | 3      | a thing                     |
| ark, ready                      | 2      | a capacity, spent by acting |
| citizen                         | 2      | a thing                     |
| citizen, fertile                | 2      | a capacity, spent by acting |
| citizen, ready                  | 2      | a capacity, spent by acting |
| citizen, whose upkeep is unpaid | 2      | a state a thing is in       |
| energy                          | 2      | a thing                     |
| extractor                       | 2      | a thing                     |
| extractor, ready                | 2      | a capacity, spent by acting |
| fertility                       | 2      | a thing                     |
| food, keeps 0                   | 2      | a state a thing is in       |
| food, keeps at least 1          | 2      | a state a thing is in       |
| food, keeps one less            | 2      | a state a thing is in       |
| garrison                        | 2      | a thing                     |
| labor                           | 2      | a thing                     |
| pioneer, ready                  | 2      | a capacity, spent by acting |
| store                           | 2      | a thing                     |
| time                            | 2      | an endless well             |
| yard                            | 2      | a thing                     |

## Every rule, and what it nets

Made minus taken, per place. A `require` row moves nothing and is absent rather than entered as a zero - a zero would read as something the weighting had weighed.

| Rule                | Nets                                                                                                                          | Worth |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ----- |
| deploy ark          | +1 garrison · +2 citizen · +2 citizen, fertile · +2 citizen, ready · +2 extractor · +2 store · -1 ark · -1 ark, ready         | 0     |
| move (ark)          | -1 ark, ready · -1 energy                                                                                                     | -4    |
| move (pioneer)      | -1 energy · -1 pioneer, ready                                                                                                 | -4    |
| found by land       | +1 garrison · +2 citizen · +2 citizen, fertile · +2 citizen, ready · +2 extractor · +2 store · -1 pioneer · -1 pioneer, ready | 0     |
| build extractor     | +1 extractor · -1 labor · -1 metal                                                                                            | -3    |
| build store         | +1 store · -1 labor · -1 metal                                                                                                | -3    |
| build yard          | +1 yard · -1 labor · -15 metal                                                                                                | -45   |
| produce pioneer     | +1 pioneer · +1 pioneer, ready · -2 citizen · -2 citizen, fertile · -2 citizen, ready · -3 metal · -6 energy                  | -11   |
| launch ark          | +1 ark · +1 ark, ready · -12 energy · -2 citizen · -2 citizen, fertile · -2 citizen, ready · -3 metal                         | -23   |
| create labor        | +1 labor · -1 citizen, ready                                                                                                  | 0     |
| work (energy x2)    | +2 energy · -1 extractor, ready · -1 labor · -1 the star                                                                      | -12   |
| work (energy x4)    | +4 energy · -1 extractor, ready · -1 labor · -1 the star                                                                      | -8    |
| work (energy x5)    | +5 energy · -1 extractor, ready · -1 labor · -1 the star                                                                      | -6    |
| work (energy x6)    | +6 energy · -1 extractor, ready · -1 labor · -1 the star                                                                      | -4    |
| work (energy x8)    | +8 energy · -1 extractor, ready · -1 labor · -1 the star                                                                      | 0     |
| work (food x1)      | +1 food · -1 extractor, ready · -1 labor · -1 the planet                                                                      | -20   |
| work (food x2)      | +2 food · -1 extractor, ready · -1 labor · -1 the planet                                                                      | -16   |
| work (food x3)      | +3 food · -1 extractor, ready · -1 labor · -1 the planet                                                                      | -12   |
| work (food x4)      | +4 food · -1 extractor, ready · -1 labor · -1 the planet                                                                      | -8    |
| work (food x6)      | +6 food · -1 extractor, ready · -1 labor · -1 the planet                                                                      | 0     |
| work (metal x2)     | +2 metal · -1 extractor, ready · -1 labor · -1 the planet                                                                     | -18   |
| work (metal x3)     | +3 metal · -1 extractor, ready · -1 labor · -1 the planet                                                                     | -15   |
| work (metal x4)     | +4 metal · -1 extractor, ready · -1 labor · -1 the planet                                                                     | -12   |
| work (metal x5)     | +5 metal · -1 extractor, ready · -1 labor · -1 the planet                                                                     | -9    |
| work (metal x6)     | +6 metal · -1 extractor, ready · -1 labor · -1 the planet                                                                     | -6    |
| work (metal x8)     | +8 metal · -1 extractor, ready · -1 labor · -1 the planet                                                                     | 0     |
| upkeep              | -1 food                                                                                                                       | -4    |
| bear                | +1 fertility · -1 citizen, fertile                                                                                            | 0     |
| breed               | +1 citizen · +1 citizen, fertile · +1 citizen, ready · -1 fertility · -1 food                                                 | 0     |
| renew               | +1 citizen, fertile · -1 time                                                                                                 | 0     |
| perish              | -1 citizen · -1 citizen, fertile · -1 citizen, ready · -1 citizen, whose upkeep is unpaid                                     | -8    |
| age (food)          | +1 food, keeps one less · -1 food, keeps at least 1                                                                           | 0     |
| spoil (food)        | -1 food · -1 food, keeps 0                                                                                                    | -6    |
| stow                | nothing                                                                                                                       | 0     |
| stow                | nothing                                                                                                                       | 0     |
| discard             | -1 metal                                                                                                                      | -3    |
| discard             | -1 energy                                                                                                                     | -2    |
| discard             | -1 labor                                                                                                                      | -2    |
| discard             | -1 fertility                                                                                                                  | -2    |
| refresh (citizen)   | +1 citizen, ready · -1 time                                                                                                   | 0     |
| refresh (extractor) | +1 extractor, ready · -1 time                                                                                                 | 0     |
| refresh (ark)       | +1 ark, ready · -1 time                                                                                                       | 0     |
| refresh (pioneer)   | +1 pioneer, ready · -1 time                                                                                                   | 0     |

**A rule that nets nothing is kept rather than dropped.** `stow` moves a resource into a store, and a store is a container rather than a state - so at this granularity it does nothing, and a reader looking for it should find it saying so rather than find it missing.
