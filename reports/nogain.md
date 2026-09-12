# Nothing comes back round with more

**Generated. Do not edit.** Read out of the release's *Recipes* table by `crates/game-console/src/nogain.rs`.

`spec/invariants.md` says there is a weighting of the kinds under which no sequence of rules ends holding more than it began with, and that **whether this holds is decided mechanically, from the rules alone**. This is that decision. `S-93`.

**Nothing here is declared.** The weighting below is solved for, not read from anywhere: the only inputs are the release's rules. So a number that looks wrong is a fact about the rules rather than about somebody's judgement of them - and one that is sound but says something you would reject is visible for that reason.

**What this does not decide, because the title reads wider than the check.** The invariant is over the **kinds**, and the places below are kinds and the counts a thing carries. **A derived trait is not among them** - `metal in it` is *its binding plus the metal in its parts*, it appears in no recipe row, and nothing here reads it. So the metal bound up in what a recipe builds is invisible to this page, and a green run is not evidence that the release's *conserved* holds of it. `P-426` and `P-427` are where that question is being asked.

**50 rules**, ground from **31** blocks of recipe rows: a family becomes its members, a density becomes its cases. **24 of them draw on a source** - the planet, the star or time - and a source is a place they take from rather than an exemption from the arithmetic. That is what keeps them in the check instead of out of it.

## A weighting exists, so nothing comes back round with more

Every rule is non-increasing under the weighting below, so any sequence of them is too - a cycle included. **It may end with less**, which is disorder and is meant.

| Place                           | Weight | What it is               |
| ------------------------------- | ------ | ------------------------ |
| ark                             | 21     | a thing                  |
| pioneer                         | 21     | a thing                  |
| ark, defending                  | 18     | a count, spent by acting |
| citizen, laboring               | 18     | a count, spent by acting |
| extractor, working              | 18     | a count, spent by acting |
| pioneer, defending              | 18     | a count, spent by acting |
| time                            | 18     | an endless well          |
| citizen, defending              | 9      | a count, spent by acting |
| force                           | 9      | a thing                  |
| food                            | 4      | a thing                  |
| ark, moving                     | 3      | a count, spent by acting |
| citizen                         | 3      | a thing                  |
| citizen, bearing                | 3      | a count, spent by acting |
| citizen, whose upkeep is unpaid | 3      | a state a thing is in    |
| energy                          | 3      | a thing                  |
| extractor                       | 3      | a thing                  |
| fertility                       | 3      | a thing                  |
| food, keeps 0                   | 3      | a state a thing is in    |
| food, keeps at least 1          | 3      | a state a thing is in    |
| food, keeps one less            | 3      | a state a thing is in    |
| garrison                        | 3      | a thing                  |
| labor                           | 3      | a thing                  |
| metal                           | 3      | a thing                  |
| pioneer, moving                 | 3      | a count, spent by acting |
| store                           | 3      | a thing                  |
| the planet                      | 3      | an endless well          |
| the star                        | 3      | an endless well          |
| yard                            | 3      | a thing                  |

## Every rule, and what it nets

Made minus taken, per place. A `require` row moves nothing and is absent rather than entered as a zero - a zero would read as something the weighting had weighed.

| Rule                            | Nets                                                            | Worth |
| ------------------------------- | --------------------------------------------------------------- | ----- |
| deploy ark                      | +1 garrison · +2 citizen · +2 extractor · +2 store · -1 ark     | 0     |
| move (ark moving)               | -1 ark, moving · -1 energy                                      | -6    |
| move (pioneer moving)           | -1 energy · -1 pioneer, moving                                  | -6    |
| found by land                   | +1 garrison · +2 citizen · +2 extractor · +2 store · -1 pioneer | 0     |
| build extractor                 | +1 extractor · -1 labor · -1 metal                              | -3    |
| build store                     | +1 store · -1 labor · -1 metal                                  | -3    |
| build yard                      | +1 yard · -1 labor · -15 metal                                  | -45   |
| produce pioneer                 | +1 pioneer · -2 citizen · -3 metal · -6 energy                  | -12   |
| launch ark                      | +1 ark · -12 energy · -2 citizen · -3 metal                     | -30   |
| create labor (citizen laboring) | +1 labor · -1 citizen, laboring                                 | -15   |
| work (energy x2)                | +2 energy · -1 extractor, working · -1 labor · -1 the star      | -18   |
| work (energy x4)                | +4 energy · -1 extractor, working · -1 labor · -1 the star      | -12   |
| work (energy x5)                | +5 energy · -1 extractor, working · -1 labor · -1 the star      | -9    |
| work (energy x6)                | +6 energy · -1 extractor, working · -1 labor · -1 the star      | -6    |
| work (energy x8)                | +8 energy · -1 extractor, working · -1 labor · -1 the star      | 0     |
| work (food x1)                  | +1 food · -1 extractor, working · -1 labor · -1 the planet      | -20   |
| work (food x2)                  | +2 food · -1 extractor, working · -1 labor · -1 the planet      | -16   |
| work (food x3)                  | +3 food · -1 extractor, working · -1 labor · -1 the planet      | -12   |
| work (food x4)                  | +4 food · -1 extractor, working · -1 labor · -1 the planet      | -8    |
| work (food x6)                  | +6 food · -1 extractor, working · -1 labor · -1 the planet      | 0     |
| work (metal x2)                 | +2 metal · -1 extractor, working · -1 labor · -1 the planet     | -18   |
| work (metal x3)                 | +3 metal · -1 extractor, working · -1 labor · -1 the planet     | -15   |
| work (metal x4)                 | +4 metal · -1 extractor, working · -1 labor · -1 the planet     | -12   |
| work (metal x5)                 | +5 metal · -1 extractor, working · -1 labor · -1 the planet     | -9    |
| work (metal x6)                 | +6 metal · -1 extractor, working · -1 labor · -1 the planet     | -6    |
| work (metal x8)                 | +8 metal · -1 extractor, working · -1 labor · -1 the planet     | 0     |
| upkeep                          | -1 food                                                         | -4    |
| bear (citizen bearing)          | +1 fertility · -1 citizen, bearing                              | 0     |
| breed                           | +1 citizen · -1 fertility · -1 food                             | -4    |
| perish                          | -1 citizen · -1 citizen, whose upkeep is unpaid                 | -6    |
| age (food)                      | +1 food, keeps one less · -1 food, keeps at least 1             | 0     |
| spoil (food)                    | -1 food · -1 food, keeps 0                                      | -7    |
| stow                            | nothing                                                         | 0     |
| stow                            | nothing                                                         | 0     |
| discard                         | -1 metal                                                        | -3    |
| discard                         | -1 energy                                                       | -3    |
| discard                         | -1 labor                                                        | -3    |
| discard                         | -1 fertility                                                    | -3    |
| refresh (ark moving)            | +1 ark, moving · -1 time                                        | -15   |
| refresh (pioneer moving)        | +1 pioneer, moving · -1 time                                    | -15   |
| refresh (citizen laboring)      | +1 citizen, laboring · -1 time                                  | 0     |
| refresh (citizen bearing)       | +1 citizen, bearing · -1 time                                   | -15   |
| refresh (extractor working)     | +1 extractor, working · -1 time                                 | 0     |
| muster (citizen defending)      | +1 force · -1 citizen, defending                                | 0     |
| stand (ark defending)           | +2 force · -1 ark, defending                                    | 0     |
| stand (pioneer defending)       | +2 force · -1 pioneer, defending                                | 0     |
| refresh (citizen defending)     | +1 citizen, defending · -1 time                                 | -9    |
| refresh (ark defending)         | +1 ark, defending · -1 time                                     | 0     |
| refresh (pioneer defending)     | +1 pioneer, defending · -1 time                                 | 0     |
| discard                         | -1 force                                                        | -9    |

**A rule that nets nothing is kept rather than dropped.** `stow` moves a resource into a store, and a store is a container rather than a state - so at this granularity it does nothing, and a reader looking for it should find it saying so rather than find it missing.
