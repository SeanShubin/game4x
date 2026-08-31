# The turn

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

The order in which a turn resolves. Every other document assumes this order.

## Order of operations

- A turn has three parts: **producing**, which is the player acting, then **consuming** and
  **transforming**, which are what ending it does
- Producing happens in any order. Anything that can be used is ready or exhausted; using it
  exhausts it, and a thing created during a turn begins ready and may be used at once. **When
  everything is exhausted there is nothing left to do**
- Ending a turn: everything that eats, eats; then a population grows on surplus food or starves
  for want of it; **food that is not eaten is lost, while metal and energy remain where they
  are**; and everything becomes ready again.
- What a territory can keep is bounded. Anything above the bound is lost when the turn ends.

## Open questions
