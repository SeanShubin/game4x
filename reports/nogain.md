# Nothing comes back round with more

**Generated. Do not edit.** Read out of the release's *Recipes* table by `crates/game-console/src/nogain.rs`.

`spec/invariants.md` says there is a weighting of the kinds under which no sequence of rules ends holding more than it began with, and that **whether this holds is decided mechanically, from the rules alone**. This is that decision. `S-93`.

**Nothing here is declared.** The weighting below is solved for, not read from anywhere: the only inputs are the release's rules. So a number that looks wrong is a fact about the rules rather than about somebody's judgement of them - and one that is sound but says something you would reject is visible for that reason.

**41 rules**, ground from **23** blocks of recipe rows: a family becomes its members, a density becomes its cases. **20 of them draw on a source** - the planet, the star or time - and a source is a place they take from rather than an exemption from the arithmetic. That is what keeps them in the check instead of out of it.

## A weighting exists, so nothing comes back round with more

Every rule is non-increasing under the weighting below, so any sequence of them is too - a cycle included. **It may end with less**, which is disorder and is meant.

| Place                           | Weight | What it is            |
| ------------------------------- | ------ | --------------------- |
| ark                             | 21     | a thing               |
| pioneer                         | 21     | a thing               |
| the planet                      | 18     | an endless well       |
| the star                        | 18     | an endless well       |
| food                            | 4      | a thing               |
| citizen                         | 3      | a thing               |
| citizen, whose upkeep is unpaid | 3      | a state a thing is in |
| energy                          | 3      | a thing               |
| extractor                       | 3      | a thing               |
| fertility                       | 3      | a thing               |
| food, keeps 0                   | 3      | a state a thing is in |
| food, keeps at least 1          | 3      | a state a thing is in |
| food, keeps one less            | 3      | a state a thing is in |
| garrison                        | 3      | a thing               |
| labor                           | 3      | a thing               |
| metal                           | 3      | a thing               |
| readiness, for `bearing`        | 3      | a state a thing is in |
| readiness, for `labor`          | 3      | a state a thing is in |
| readiness, for `move`           | 3      | a state a thing is in |
| readiness, for `work`           | 3      | a state a thing is in |
| store                           | 3      | a thing               |
| time                            | 3      | an endless well       |
| yard                            | 3      | a thing               |

## Every rule, and what it nets

Made minus taken, per place. A `require` row moves nothing and is absent rather than entered as a zero - a zero would read as something the weighting had weighed.

| Rule              | Nets                                                            | Worth |
| ----------------- | --------------------------------------------------------------- | ----- |
| deploy ark        | +1 garrison · +2 citizen · +2 extractor · +2 store · -1 ark     | 0     |
| move              | -1 energy · -1 readiness, for `move`                            | -6    |
| found by land     | +1 garrison · +2 citizen · +2 extractor · +2 store · -1 pioneer | 0     |
| build extractor   | +1 extractor · -1 labor · -1 metal                              | -3    |
| build store       | +1 store · -1 labor · -1 metal                                  | -3    |
| build yard        | +1 yard · -1 labor · -15 metal                                  | -45   |
| produce pioneer   | +1 pioneer · -2 citizen · -3 metal · -6 energy                  | -12   |
| launch ark        | +1 ark · -12 energy · -2 citizen · -3 metal                     | -30   |
| create labor      | +1 labor · -1 readiness, for `labor`                            | 0     |
| work (energy x2)  | +2 energy · -1 labor · -1 readiness, for `work` · -1 the star   | -18   |
| work (energy x4)  | +4 energy · -1 labor · -1 readiness, for `work` · -1 the star   | -12   |
| work (energy x5)  | +5 energy · -1 labor · -1 readiness, for `work` · -1 the star   | -9    |
| work (energy x6)  | +6 energy · -1 labor · -1 readiness, for `work` · -1 the star   | -6    |
| work (energy x8)  | +8 energy · -1 labor · -1 readiness, for `work` · -1 the star   | 0     |
| work (food x1)    | +1 food · -1 labor · -1 readiness, for `work` · -1 the planet   | -20   |
| work (food x2)    | +2 food · -1 labor · -1 readiness, for `work` · -1 the planet   | -16   |
| work (food x3)    | +3 food · -1 labor · -1 readiness, for `work` · -1 the planet   | -12   |
| work (food x4)    | +4 food · -1 labor · -1 readiness, for `work` · -1 the planet   | -8    |
| work (food x6)    | +6 food · -1 labor · -1 readiness, for `work` · -1 the planet   | 0     |
| work (metal x2)   | +2 metal · -1 labor · -1 readiness, for `work` · -1 the planet  | -18   |
| work (metal x3)   | +3 metal · -1 labor · -1 readiness, for `work` · -1 the planet  | -15   |
| work (metal x4)   | +4 metal · -1 labor · -1 readiness, for `work` · -1 the planet  | -12   |
| work (metal x5)   | +5 metal · -1 labor · -1 readiness, for `work` · -1 the planet  | -9    |
| work (metal x6)   | +6 metal · -1 labor · -1 readiness, for `work` · -1 the planet  | -6    |
| work (metal x8)   | +8 metal · -1 labor · -1 readiness, for `work` · -1 the planet  | 0     |
| upkeep            | -1 food                                                         | -4    |
| bear              | +1 fertility · -1 readiness, for `bearing`                      | 0     |
| breed             | +1 citizen · -1 fertility · -1 food                             | -4    |
| perish            | -1 citizen · -1 citizen, whose upkeep is unpaid                 | -6    |
| age (food)        | +1 food, keeps one less · -1 food, keeps at least 1             | 0     |
| spoil (food)      | -1 food · -1 food, keeps 0                                      | -7    |
| stow              | nothing                                                         | 0     |
| stow              | nothing                                                         | 0     |
| discard           | -1 metal                                                        | -3    |
| discard           | -1 energy                                                       | -3    |
| discard           | -1 labor                                                        | -3    |
| discard           | -1 fertility                                                    | -3    |
| refresh (move)    | +1 readiness, for `move` · -1 time                              | 0     |
| refresh (labor)   | +1 readiness, for `labor` · -1 time                             | 0     |
| refresh (work)    | +1 readiness, for `work` · -1 time                              | 0     |
| refresh (bearing) | +1 readiness, for `bearing` · -1 time                           | 0     |

**A rule that nets nothing is kept rather than dropped.** `stow` moves a resource into a store, and a store is a container rather than a state - so at this granularity it does nothing, and a reader looking for it should find it saying so rather than find it missing.
