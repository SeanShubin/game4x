# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-606 - *The order this specification already gives them* names two orders, and the tree is in one of them

**to** sean · **status** open · **raised** 2026-10-01 · **asks** a decision · **kind** recovered · **into** `spec/README.md` -> rule 3 · **source** the code lane refusing to guess the bytes of your approvals, twice

**`P-600` is a phrase this lane wrote and you approved, and it is ambiguous.** It says two tests are
the same test when their rows say the same thing, *entries coalesced to one per description, traits
and entries in the order this specification already gives them*. **There are two such orders.**

```
spec/console.md            `id` first, then every other trait alphabetically, then
                           `occupied`, `free` and `capacity` last
spec/data/schema.4x        the order columns are declared in - citizen is
                           where, hungry, bearing, laboring, quantity
```

**Measured over all 57 records: 449 rows are not in alphabetical trait order and 51 are by
coincidence.** Every row in the tree is in the schema's order. So the phrase names the order the tree
is not in.

**And `console.md` is about something else.** Its order is for an entry in the nested state dump,
where *where a thing is, is where it appears* and *nothing states its container*. **A test row is flat
and states its container** - `where:place-1`. So the rule does not reach a test row by itself; applying
it is a decision that a test row is an entry of that kind.

## The two answers, and they cost very differently

**The schema's order.** Nothing changes: every record is already that shape, so no verdict clears and
the writer is a fold over rows.

**`console.md`'s order.** Every record's rows are rewritten, **so every verdict you have given
clears** and the 57 are read again. And the canonical ordering lives in
`crates/game-console/src/containment.rs`, which `crates/game-model` cannot reach because the
dependency runs the other way - it would move to `friendly-notation`, a refactor of a 712-line module.

## One fragility worth knowing before you pick

**Under the schema's order, renumbering `seq:` clears every verdict.** The `seq:` values are editable
and carry no meaning beyond order, so a tidy-up nobody thinks twice about would rewrite every record
and send all 57 back to you. **Alphabetical cannot do that**, because it depends on names, which
nobody renumbers.

**It is loud rather than silent** - the verdicts clear and you notice - which is the safe direction.
**But it is a trap laid for a future session**, and the kind this repository has been bitten by before.

## What the code lane is doing meanwhile, which is not a decision

**Writing in the schema's order, because that is what every record already is.** Producing bytes
identical to what exists is compatibility rather than a choice - and if you pick `console.md`'s order,
the rewrite is the same size whether they wrote the writer today or waited.
