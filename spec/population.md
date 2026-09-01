# Population

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

## Citizens

- A citizen is not one person. It is the smallest group that can sustain reproduction.
- A count of one is not one citizen. A territory is a wide stretch of land, and a count is a
  density across the whole of it. One is the least that makes its presence felt.
- A unit taken apart produces the citizen that starts a territory; an Ark, arriving from orbit,
  starts a planet's population.
- Nothing else produces citizens, and after that the number changes only by the rule below.
- Each turn the number of citizens may change depending on available food
  - If less food than citizens, each unfed citizen starves
  - If food equals citizens, no change
  - If food exceeds citizens, then generate 1 citizen for each citizen with extra food, so
    increases by the minimum of extra-food and total-citizens, at most doubling if there is
    plenty of food

## Labor

- A citizen provides labor each turn
- Labor is spent when it is used, and is not restored until the end of the turn

## Open questions
