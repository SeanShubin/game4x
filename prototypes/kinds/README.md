# kinds

[Prototypes](../../docs/prototypes/README.md) · [Architecture](../../docs/architecture.md) · [Root README](../../README.md)

**The question.** What do the inputs to the gameplay logic actually look like? The release
says it in seven markdown tables, which is the right form for deciding it and the wrong one
for finding out whether it holds together. This is the same content as Rust data, so the
shape can be read and compiled before it is built into the model — which is cheaper than
reviewing it afterwards.

It does not play. No turn, no board, no rule, no state. Only what a thing is and what turns
into what.

```
cargo run -p kinds        # prints all seven tables back out, rendered from the data
cargo test -p kinds       # checks they are the release's tables, cell for cell
```

## The answer

`S-4` named four things it expected this to force into the open, and building it produced
three more. The release has since **declared its own vocabulary** — kinds, families,
capacities, traits and what a territory has total capacity for — so several of these are now
answered by the document rather than inferred from it. That is the outcome worth having: a prototype that
argues itself out of a job.

**What type is a quantity?** Not a number. A quantity is *written in the recipe, read from a
trait of one of the ingredients, or read from a trait of a named ingredient* — and the third
way exists because of this crate. `work` yields the territory's density while a territory is
not among its ingredients, so the sentence was false of a row three lines below it. `work`
names the territory `$where` now, which is how a recipe reaches past what it consumes.

**How is a trait that varies per instance typed?** As a second axis. `move` takes a unit in
`$from` and yields the same unit in `$to` — same unit, same kind, a different place. The
Traits table says it outright: `place` is a trait of every thing, and its value is the thing
it is in.

**Derived against stored.** The release marks its own, and the marks moved: `metal in it`
became derived from a thing's binding plus its parts, and `control` became derived from
whether a citizen of that player is there. This crate derives both rather than storing them,
so the two cannot disagree.

**Owner, not scope.** It was `here`/`every`, which said who a recipe belonged to while
reading as though it said where. A player asks for the first sort; the world runs the second
whether anyone asks or not.

### Consumed was worked out, and is written down again

For a while the column was gone: *an ingredient is consumed exactly when the same thing,
with the same traits, does not appear among the results*. It worked, and it cost four
recipes an echo row that existed only to say something survived — `upkeep` took a thing with
upkeep and handed it straight back.

The release now writes a **Role** on every line: `require`, `limit`, `consume`, `produce`.
Saying it once is the better trade, and it bought something the derived rule could not
express cleanly. *Unheld ground* was a garrison taken at most zero and given back — a
quantity of zero that was also a result — and is now simply `limit 0 garrison`.

**Where moved too**, out of the qualifiers and into its own column. `in $where` read as a
trait of the thing and was one, but a recipe that acts somewhere puts every line there, so
it was written once per line to say something true of the whole recipe.

### Families were a gap, and are not

This crate reported that `ready` named a `thing` the release never defined, and the test that
reported it said to delete itself the day the release answered. It has. Two kinds it had
inferred turned out not to be kinds at all: `node` became a trait of a territory, and `cell`
became fuel in a unit's tank. **Both were nouns the recipes used and nothing listed**, which
is exactly what a declaration is for.

### And the finding this crate produced about itself

On the day the release moved furthest, **seven of its eight tests passed against data that
had stopped matching hours earlier.** One asserted eighteen recipes while there were sixteen.
Another asserted that `revert` names a place, and `revert` no longer existed.

They passed because they read this crate and checked it against numbers written in this
crate. Self-consistent, and empty. **A test that reads one artifact can only tell you it has
not changed** — and the one test that reads the release caught all of it at once.

## Why there is a test

Two copies of seventeen recipes would be one copy and one guess. The test renders this
data back into all seven of the release's tables and compares them with `releases/first-release.md`
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
