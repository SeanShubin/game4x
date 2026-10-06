# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-621 - An unapproved test is the shape of an intention, and the divergence is the point

**to** sean · **status** open · **raised** 2026-10-06 · **kind** his reading, which his own rules already support · **shape** text · **asks** approval · **into** `spec/README.md` -> rule 3, after *My approval is about what a test says*

**Sean, 2026-10-06**, asked what a record should hold for a test the engine cannot fold:

```
An unapproved test is text in friendly format that is the shape of an intention. The code is
bound to tests I have approved. Once I approve a test the code does not implement, we have a
divergence, which I expect. From here on the code lane is compelled to resolve the divergence.
I don't see why relations have to exist for unapproved tests.
```

**Nothing in rule 3 requires them**, and this lane checked before offering words:

```
rule 3   "The behaviour is every {...} row, including a {load}, and nothing else"
rule 3   column order is "id first, then every other trait alphabetically", which
         "depends on names" - and must "not depend on anything editable"
```

**So the record is the rows, normalized by name.** The schema is the engine's business and the
record never needed it. **`record_for` goes through `friendly_notation::fold`, which does** - the
implementation is narrower than the rule, which is `S-259` and not a decision for you.

The paragraph offered:

> **An unapproved test is text in the friendly form, and it is the shape of an intention.** The code
> is bound to the tests I have approved. **So approving a test the code does not implement creates a
> divergence, which I expect** - and from there the code lane is compelled to resolve it. **Relations
> do not have to exist for a test I have not approved**, and a test whose relations do not exist yet
> is the normal way a thing I want becomes a thing that is built.

## What it settles that nothing else did

**The order of arrival.** `spec/tests/README.md` says *a test he has read is red until the code obeys
it*, which only means anything if a test can be read before the code obeys it. **This says that
outright**, where before it was implied by a sentence about redness.

**And it tells the code lane it may not decline.** *Compelled to resolve the divergence* is the half
no existing rule carries - today a red unread test is a notice, and nothing says what an
**approved** one obliges.

## What this lane did not fold in

**Nothing about how a divergence is resolved.** Implementing the relations, or coming back with a
reason the test is wrong, are both resolutions and the words above pick neither.

**And nothing about the schema's own route.** `{saves}`, `{open-menu}`, `{item}` and `{attention}`
have to exist in `spec/data/schema.4x` eventually, and whether that arrives by promotion or follows
from the test is a separate question nobody has asked yet.

