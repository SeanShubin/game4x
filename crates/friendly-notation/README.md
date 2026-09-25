# friendly-notation

The bridge between the form the engine runs and the form a person writes.

Sean, 2026-09-15: *the foundation is the logical model, friendly is a bridge from the user to the
logical model.* Both forms say the same rows; one says them with ids and the other with names.

```
{scout where:place-1 moving:1} -> 1     friendly - names, and a quantity after an arrow
{scout id:1 where:1 moving:1}            foundation - ids, and the quantity in its column
```

## Why it is a crate of its own

**The engine may not hold it.** `crates/thin-engine/tests/isolation.rs` asserts that the engine
names no noun the game has, and that its `src/` is eight modules none of which reads a file or
depends on another crate. This names a scout, a territory and a place in the first fifty lines, so
it was never a candidate for living there — which is why it spent its life in `tests/`.

**And a `#[path]` include is not a direction.** `examples/report.rs` and `examples/review-web.rs`
reached it as a file rather than as a dependency. `review-web.rs` is the review application, which
`CLAUDE.md` puts beyond every lane it judges — so the one property that has to hold is that **a
record's meaning cannot change because a rendering changed.** A dependency states that and a file
include leaves it to whoever edits next.

**The direction is one way and Cargo holds it.** This depends on `thin-engine`; `thin-engine` does
not depend on this. Its `[dependencies]` table is still empty and still deliberate.

## What is in it

|                     |                                                                              |
| ------------------- | ---------------------------------------------------------------------------- |
| `fold`              | reads friendly text, folding `-> n` back into the relation's quantity column |
| `Names`             | the name of every thing in a world, built from its rows                      |
| `Names::row`        | one foundation row, written friendly                                         |
| `Names::foundation` | one friendly row, written as the engine reads it                             |
| `Names::parse`      | one friendly line, read                                                      |
| `compared`          | two worlds, differenced, for a report that shows what a rule did             |

`rule 3` of [`spec/README.md`](../../spec/README.md) says which of the two forms is the source: **a
test is stated in the friendly form, and the foundation form is a rendering of it.** So
`Names::foundation` is the direction that generates, and `Names::row` is the one that explains.
