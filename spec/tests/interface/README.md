# Interface tests

[The tests directory](../README.md) · [The specification](../../README.md)

**Empty, and that is the state it is in rather than a mistake.** Sean, 2026-10-01: *we are going to
have two sets of unit tests, one for the game rules and one for the user interface.* This is the
second set and nothing is in it yet.

**What goes here is a test about what the interface shows**, not about what the game does - which
menu items are displayed, whether each is active, and whether one has the user's attention. The
first set is [`../rule/`](../rule/).

**What they are written in is not settled.** The friendly form states game rows, and *displayed*,
*active* and *has attention* are not game rows - so the form a test here takes is an open question
and is recorded in [`docs/notes/spec-backlog.md`](../../../docs/notes/spec-backlog.md) rather than
decided.

**One thing already known about the shape.** *Has the user's attention* is a property of the
interface naming at most one item, not a flag on each item - so a test that models it per item can
assert two items hold it at once.
