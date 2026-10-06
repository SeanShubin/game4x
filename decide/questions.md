# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-621 - A test the engine cannot read yet cannot be approved, which is backwards

**to** sean · **status** open · **raised** 2026-10-06 · **kind** a gap his own rules leave, found by trying to use them · **asks** a decision · **into** `spec/README.md` -> rule 3, once decided

**Sean, 2026-10-06**: *I see the test at `.../reports/review/interface.html` but can not review it.*

**Traced, and the chain is four steps with no judgement in it:**

```
report.rs:1175   render::record_for(stem, source, "approved")
render.rs:831    friendly_notation::fold(text, &schema)?        -> Err: no such relations
report.rs:1176   .ok()                                          -> None, silently
report.rs:1179   .unwrap_or_default()                           -> no data-record attribute
the page         querySelectorAll('details[data-test][data-record]')  -> card not selected
```

**So there are no buttons**, because `{saves}`, `{open-menu}`, `{item}` and `{attention}` are in no
schema. **The card is there and reviewable-looking**; `rule.html` carries 63 `data-record=`
attributes and `interface.html` carries none.

## The gap, which is in your rules rather than in the code

**`P-605`: a record holds a verdict and the behaviour.** The behaviour is the test's rows in
foundation form - generated, normalized. **A test the engine cannot fold has no such form**, so
there is nothing for the record to hold.

**And `spec/tests/README.md` says the opposite has to work**: *a test he has read is red until the
code obeys it.* **That only means anything if he can read a test before the code obeys it** - which
is exactly the test he just tried to read.

```
what the process says   the test arrives, he approves it, the code is red until it obeys
what happens now        the code must obey first, then he may approve
```

**So the first interface test is unapprovable, and so is every future one** until its relations
exist - at which point approving it constrains nothing that was not already built.

## Three answers, and this lane has no view

```
one    the record holds the test's own bytes when it cannot be folded, and the
       foundation form appears later when it can
two    the page offers approval with no record body, and the record is written
       from the source rather than from the fold
three  an unfoldable test is not approvable, and the specification says so -
       the interface tests wait for a schema
```

**One and two differ on what `reviewed/` holds**, which `P-605` settled for the foldable case and
did not for this one. **Three is a real answer** and makes the order *schema first, test second*,
which is the opposite of how the 63 rule tests arrived.

**This lane will not pick**, because every answer changes what a record is - and that is the one
artifact whose whole value is that nobody judged by it can touch it.

## And one thing that is a defect whatever you choose

**`.ok()` swallows the reason.** The page shows a card with no controls and says nothing, so *this
test cannot be folded yet* and *this page is broken* look identical. Filed separately to the code
lane.
