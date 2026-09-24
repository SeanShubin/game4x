# The turn

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

The order in which a turn resolves. Every other document assumes this order.

## Order of operations

- A turn has three parts: **producing**, which is the player acting, then **consuming** and
  **transforming**, which are what ending it does
- Producing happens in any order. **What a thing can do is a count it carries as a trait**, rather
  than something it contains: an **action** is named, and each kind declares how many of each
  action a thing of it may take in a turn. A recipe names the action it spends, and firing it
  lowers that count by one. **Two recipes naming the same action spend the same count**, which is
  how it is said that a thing must choose between them; two recipes naming different actions never
  compete. A thing created during a turn begins at its full count and may act at once. **When no
  thing has a count left, there is nothing left to do**
- Ending a turn: everything with upkeep pays it; then a population grows on surplus food or
  starves for want of it; **what expires expires, and what was not kept in order is lost**; and
  **time restores every count to the number that thing's kind declares**
- What a territory can keep is bounded. Anything above the bound is lost when the turn ends.
- Where two effects cannot both happen, they compete. Competing effects are gathered and resolved
  together, so nothing gains an advantage by being considered first
- What settles them is a deterministic mechanic of the game, and therefore something a person wrote
  and a player can change
- Effects compete when they arise together: several rules firing, or a sweep at the end of a turn.
  A player acting twice in sequence is not competition - the second act sees what the first did

## Open questions
