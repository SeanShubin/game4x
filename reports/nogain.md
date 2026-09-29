# Nothing comes back round with more

Generated from `spec/data/rules.4x` by `scripts/reports.sh`. **Not canonical** - the rules are, and this is what follows from them.

[The index](index.md)

**16** rules in `spec/data/rules.4x` ground to **32**, over **30** places. A rule over a family is one rule for each of its members, and a clause whose quantity is read gets a second row for the coefficient of what it reads.

**A weighting exists, so nothing comes back round with more.** Every rule below is non-increasing under it, so any sequence of them is too - a cycle included. That is the whole of the argument: it does not depend on which rules a player picks, or on how many times, because each one alone never raises the total.

## Every rule under the weighting  (32)

Made minus taken, weighed. **Every figure is at or below zero**, and **16** of them are exactly zero - the loops the weighting is tight around, where what comes back is worth precisely what went in. A rule that could gain would show a figure above zero, and none can, because the weighting was solved for rather than chosen.

| Rule                                 | Takes                                     | Makes                                                                                                                             | Weighed |
| ------------------------------------ | ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | ------- |
| `move (scout)`                       | scout, moving 1                           | scout, moving 0                                                                                                                   | 0       |
| `move (ark)`                         | ark, moving 1                             | ark, moving 0                                                                                                                     | 0       |
| `move (transport)`                   | transport, moving 1                       | transport, moving 0                                                                                                               | 0       |
| `move (pioneer)`                     | pioneer, moving 1                         | pioneer, moving 0                                                                                                                 | 0       |
| `build-extractor`                    | labor · metal                             | extractor · extractor, working 1                                                                                                  | -13     |
| `work (metal)`                       | extractor, working 1 · labor · the planet | extractor, working 0 · metal                                                                                                      | -1      |
| `work (metal), per unit of density`  | the planet                                | metal                                                                                                                             | 0       |
| `work (food)`                        | extractor, working 1 · labor · the planet | extractor, working 0 · food                                                                                                       | -1      |
| `work (food), per unit of density`   | the planet                                | food                                                                                                                              | 0       |
| `work (energy)`                      | extractor, working 1 · labor · the planet | energy · extractor, working 0                                                                                                     | -1      |
| `work (energy), per unit of density` | the planet                                | energy                                                                                                                            | 0       |
| `refresh (scout, moving)`            | scout, moving 0 · time                    | scout, moving 1                                                                                                                   | -1      |
| `refresh (ark, moving)`              | ark, moving 0 · time                      | ark, moving 1                                                                                                                     | -1      |
| `refresh (transport, moving)`        | time · transport, moving 0                | transport, moving 1                                                                                                               | -1      |
| `refresh (pioneer, moving)`          | pioneer, moving 0 · time                  | pioneer, moving 1                                                                                                                 | -1      |
| `refresh (extractor, working)`       | extractor, working 0 · time               | extractor, working 1                                                                                                              | -1      |
| `refresh (citizen, hungry)`          | citizen, hungry 0 · time                  | citizen, hungry 1                                                                                                                 | -1      |
| `refresh (citizen, bearing)`         | citizen, bearing 0 · time                 | citizen, bearing 1                                                                                                                | -10     |
| `refresh (citizen, laboring)`        | citizen, laboring 0 · time                | citizen, laboring 1                                                                                                               | 0       |
| `refresh (ark, gathering)`           | ark, gathering 0 · time                   | ark, gathering 1                                                                                                                  | 0       |
| `build-bin`                          | labor · metal                             | bin                                                                                                                               | -14     |
| `upkeep`                             | citizen, hungry 1 · food                  | citizen, hungry 0                                                                                                                 | -14     |
| `perish`                             | citizen · citizen, hungry 1               | —                                                                                                                                 | -2      |
| `breed`                              | food                                      | citizen · citizen, bearing 0 · citizen, hungry 0 · citizen, laboring 1                                                            | 0       |
| `toil`                               | citizen, laboring 1                       | citizen, laboring 0 · labor                                                                                                       | 0       |
| `gather`                             | ark, gathering 1 · the planet             | ark, gathering 0 · energy                                                                                                         | -1      |
| `gather, per unit of density`        | the planet                                | energy                                                                                                                            | 0       |
| `launch`                             | energy · labor · metal                    | ark · ark, gathering 1 · ark, moving 1                                                                                            | 0       |
| `deploy (pioneer)`                   | pioneer                                   | 2 × citizen · 2 × citizen, bearing 1 · 2 × citizen, hungry 0 · 2 × citizen, laboring 1 · 2 × extractor · 2 × extractor, working 1 | 0       |
| `deploy (ark)`                       | ark                                       | 2 × citizen · 2 × citizen, bearing 1 · 2 × citizen, hungry 0 · 2 × citizen, laboring 1 · 2 × extractor · 2 × extractor, working 1 | 0       |
| `build-pioneer`                      | labor · metal                             | pioneer · pioneer, moving 1                                                                                                       | 0       |
| `build-yard`                         | labor · metal                             | yard                                                                                                                              | -14     |

## The weighting  (30)

**Nothing here is declared.** These numbers are solved for, and the only input is `spec/data/rules.4x` - so they are a consequence of the rules rather than a statement about them. Any positive multiple of them would do as well; these are the smallest whole numbers the solver reached.

| Place                | Weight |
| -------------------- | ------ |
| ark                  | 14     |
| ark, gathering 0     | 13     |
| ark, gathering 1     | 14     |
| ark, moving 0        | 1      |
| ark, moving 1        | 1      |
| bin                  | 1      |
| citizen              | 1      |
| citizen, bearing 0   | 10     |
| citizen, bearing 1   | 1      |
| citizen, hungry 0    | 1      |
| citizen, hungry 1    | 1      |
| citizen, laboring 0  | 1      |
| citizen, laboring 1  | 2      |
| energy               | 14     |
| extractor            | 1      |
| extractor, working 0 | 1      |
| extractor, working 1 | 1      |
| food                 | 14     |
| labor                | 1      |
| metal                | 14     |
| pioneer              | 14     |
| pioneer, moving 0    | 1      |
| pioneer, moving 1    | 1      |
| scout, moving 0      | 1      |
| scout, moving 1      | 1      |
| the planet           | 14     |
| time                 | 1      |
| transport, moving 0  | 1      |
| transport, moving 1  | 1      |
| yard                 | 1      |

## Rules that move nothing  (2)

**Named rather than dropped**, so that the count above can be reconciled with the sixteen in the data. Neither is a gap: `end-turn` is exactly its parts, and each of those is a rule in its own right above; `discard-disorder` keeps what it matches and takes nothing away.

| Rule               |
| ------------------ |
| `end-turn`         |
| `discard-disorder` |

**What this reads is at the top of `crates/game-model/examples/nogain.rs`.** The short of it: a place is a kind, or a kind with one trait pinned, because a weighting over bare kinds would call `move` and `refresh` no-ops and report the whole economy as doing nothing. A `put` draws on time and a density reading draws on the planet, which is `spec/invariants.md`'s own sentence about what exhausts. **There are two wells here and not three**, and since `P-588` that is what the specification says rather than this reader's guess: the planet carries the star's density itself, so every orbit above it draws on that one row and there is no separate star to miss.
