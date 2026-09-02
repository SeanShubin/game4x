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

`S-4` named four things it expected this to force into the open, and building it produced
three more. The release has since **declared its own vocabulary** — kinds, families, bins
and traits, four sections that did not exist when this crate was written — so several of
these are now answered by the document rather than inferred from it. That is the outcome
worth having: a prototype that argues itself out of a job.

**What type is a quantity?** Not a number. A quantity is *written in the recipe, or read
from a trait of one of the ingredients* — and three of the eighteen read one. The cut that
matters is not what is read but **whose trait it is**: `upkeep` reads the unit's upkeep and
`perish` reads the unit's metal, and in both the unit is an ingredient. `work` reads the
*territory's* density, and a territory is not among its ingredients — those are labor and an
extractor. So `Quantity` is `Exactly(n) | OfAnIngredient | OfThePlace`, and **`work` is the
only one of eighteen that reads past its own ingredients**. It does so because density moved
from a thing to a place. Reported as `P-151`.

**How is a trait that varies per instance typed?** As a second axis, not as a kind and not
as a container. `move` takes *unit, here* and yields *unit, there* — the same unit, the same
kind, a different place. The release's Traits table now says this outright: `place` is a
trait of every thing, and its value is the bin it is in.

**Derived against stored.** The release marks its own now — `surplus` and `unfed` are
derived, `arriving` is stored and cleared at end turn — so `Held` records what the document
says instead of what this crate worked out. Modelling `arriving` as a plain stored field
would have been wrong in a way nothing would have caught.

**Scope: a field, or two types?** A field. Ten recipes are `here` and eight are `every`, and
nothing else about them differs, so two types would duplicate the whole shape to carry one
bit.

### Families were a gap, and are not

This crate reported that `ready` named a `thing` the release never defined, and the test
that reported it said to delete itself the day the release answered. It has. **Families are
declared**, and a family is still a list rather than a parent class — `spec/invariants.md`
has every kind of thing be data, and a hierarchy would be the one shape that is not.

Two kinds this crate had inferred turned out not to be kinds at all: `node` became a trait
of a territory, and `cell` became fuel in a unit's tank. **Both were things the recipes
named and nothing listed**, which is exactly what a declaration is for.

### Three sorts of noun, and only two of them are things

The Thing column names a kind, or a family, or — once — a **territory**. The release's own
*Where things are* says every thing is in a bin, and a territory *is* a bin: it is where
things are, not a thing that is anywhere. `revert` is the only recipe that names one, and
`Noun::Territory` records it rather than pretending it is a kind.

### Two shapes for one idea

Every qualifier but two reads `kind, qualifier` — *ark, in orbit*, *food, surplus*. Two read
as English instead: *unit with upkeep*, *unit whose upkeep is unpaid*. Nothing about them
differs in kind, so the difference is punctuation, and `Qualifier::as_phrase` records it
rather than smoothing it over. Recorded and not corrected: the release is the specification,
and this crate holding what it says is the whole reason the comparison is worth anything.

## Why there is a test

Two copies of eighteen recipes would be one copy and one guess. The test renders this
data back into all six of the release's tables and compares them with `releases/first-release.md`
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
