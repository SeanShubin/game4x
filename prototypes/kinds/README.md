# kinds

[Prototypes](../../docs/prototypes/README.md) · [Architecture](../../docs/architecture.md) · [Root README](../../README.md)

**The question.** What do the inputs to the gameplay logic actually look like? The release
says it in two markdown tables, which is the right form for deciding it and the wrong one
for finding out whether it holds together. This is the same content as Rust data, so the
shape can be read and compiled before it is built into the model — which is cheaper than
reviewing it afterwards.

It does not play. No turn, no board, no rule, no state. Only what a thing is and what turns
into what.

```
cargo run -p kinds        # prints both tables back out, rendered from the data
cargo test -p kinds       # checks they are the release's tables, cell for cell
```

## The answer

`S-4` named four things it expected this to force into the open. All four turned out to be
real, and none of them could have been settled from prose.

**What type is a quantity?** Not a number. Twelve of the fifteen transformations give a
number everywhere; three do not, on four rows between them, and none of the four is missing
a figure — each is a figure known only when the transformation is applied. `work` yields the
*density* of the node being worked; `spoil` and `ready` take *any*, meaning however much is
there. So `Quantity` is `Exactly(n) | Density | Any`, and everything that reads this data
handles all three.

**How is a trait that varies per instance typed?** As a second axis, not as a kind and not
as a container. `move` takes *unit, here* and yields *unit, there* — the same unit, the same
kind, a different location — so location is a property of the instance, and a transformation
names the trait it requires and the trait it leaves behind. `Subject` is a `Kind` and an
optional `Trait`.

**`node, unworked` and `food, surplus`.** Kept as traits, with a flag saying they are
*derived*: a node is unworked when the extractors are fewer than the nodes, food is surplus
when it is more than the citizens. Neither is a fact anything stores — each is a comparison
between two counts. **Two rows out of fifty-odd**, which is what choosing between derived
kinds and comparisons costs, and the reason the cheaper answer wins here.

**Scope: a field, or two types?** A field. Ten transformations are `here` and five are
`every`, and nothing else about them differs — same ports, same quantities, same bounds — so
two types would duplicate the whole shape to carry one bit. What it costs is not in the data
but in whatever runs it: `here` needs to be told where, and `every` does not.

### And a fifth, which was not asked for

**Three of the nouns are not things but families.** `work` outputs a `resource` without
saying which; `ready` readies a `thing` whatever kind it is; `move` moves a `unit`. The table
quantifies over kinds, so `Kind` is either a leaf or a family and an implementation has to
know which. Writing the enum out is what made it obvious; reading the table it is invisible,
because a reader supplies the generality without noticing.

**It costs nothing to accommodate, which is the good news.** A family is not a parent class:
membership is a list, so `unit` is a trait an Ark and a Pioneer both carry, and a
transformation naming it matches anything carrying it. `FAMILIES` is that list and
`Kind::covers` is the whole mechanism. That matters because `spec/invariants.md` has every
kind of thing be data, and a parent class would be the one shape that is not.

**And then the gap turns out to be one noun rather than three.** Two families can be read
straight off the release — only the Ark and the Pioneer have cells and a move, and the
resources are the three the biome table has columns for. The third cannot:

> **The release names `thing` and never says what one is.** `ready` — the transformation
> that puts the whole planet back on its feet every turn — takes `thing, exhausted` and
> yields `thing, ready`, and nothing anywhere says which kinds can be exhausted. The only
> kind the release shows exhausted is a citizen, in `spend readiness`.

`FAMILIES` leaves it empty rather than guessing. `crates/game-model` readies extractors,
garrisons and labor as well — but that is the model's answer, not the release's, and showing
the difference is what this crate is for. The test asserts the gap and says to delete itself
when the release fills it, because otherwise nobody would think to come back and look.

## Why there is a test

Two copies of fifteen transformations would be one copy and one guess. The test renders this
data back into the release's two tables and compares them with `releases/first-release.md`
on disk, so neither can move without the other — the habit
[`quotations.rs`](../../crates/game-console/tests/quotations.rs) already has of reading the
document at test time rather than trusting a copy of it.

**Cells are compared, not bytes.** `tools/pad-tables` owns the column widths and rewrites
them whenever anything else in the file changes, so a byte comparison would fail on
whitespace nobody wrote.

It earns itself immediately: it is wired to the gate, which tests every crate that does not
link an engine, so a figure changed in one place and not the other is a failed build rather
than a discrepancy somebody eventually notices.

## What this is not

**Not the model.** `crates/game-model` is the shipped one and this has no bearing on it.
Nothing depends on this crate and nothing should — if the shape here turns out to be right,
what happens next is that the model is written to match it, not that the model imports it.

**Not a proposal.** The figures are the release's. If a figure here looks wrong, the release
is where it is wrong, and that is a proposal rather than an edit.
