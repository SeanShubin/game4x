# Interface tests

[The tests directory](../README.md) · [The specification](../../README.md)

**Empty, and that is the state it is in rather than a mistake.** Sean, 2026-10-01: *we are going to
have two sets of unit tests, one for the game rules and one for the user interface.* This is the
second set and nothing is in it yet.

**What goes here is a test about what the interface shows**, not about what the game does - which
menu items are displayed, whether each is active, and whether one has the user's attention. The
first set is [`../rule/`](../rule/).

**An interface test is written in the same form as a rule test.** Rows in, one command, rows out -
and the `then` is the whole of what the interface shows, so **an item that is not in it is not
displayed**, and how many items there are is asserted by the rows being all of them.

**`active` is a column and `displayed` is not.** An item is displayed by being in the `then`, so a
column saying so could hold only one value. An item that is shown and unusable says `active:0`.

**Attention is one row naming at most one item**, rather than a column on each - `{attention
of:new-game}`. *Has the user's attention* is a property of the interface, and a column would let
two items hold it at once.

**What this form cannot say is order.** The rows are a multiset, so no test can require *new game*
above *exit*. **A known limitation rather than an oversight**, and what settles it arrives when a
test needs it.

