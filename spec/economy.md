# Economy

**Authored.** Every line here is reviewed by Sean before it lands. Claude may reorganize and
rephrase, reporting every change; new content arrives only by Sean accepting a numbered
proposal from [proposals](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

## Structures and labor

- A territory's natural resource level sets the rate at which a structure built there can
  exploit it
- For example, a territory can have a **fertility** of 8, which means if a **farm** is
  constructed there, each **turn** a **citizen** may expend **labor** to generate an amount
  of **food** equal to the **fertility** level

## Citizens

- Each turn the number of citizens may change depending on available food
  - If less food than citizens, each unfed citizen departs
  - If food equals citizens, no change
  - If food exceeds citizens, then generate 1 citizen for each citizen with extra food, so
    increases by the minimum of extra-food and total-citizens, at most doubling if there is
    plenty of food

## Open questions
