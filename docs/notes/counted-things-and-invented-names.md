# Counted things and invented names

**2026-09-05.** Sean, reading `scenario/expected/play.4x`, asked what `labor-spent` means, then
described what the predecessor did instead: a map from a thing's description to a quantity, so
`[{citizen ready:true} -> 8, {citizen ready:false} -> 6]` is eight ready citizens and six
exhausted ones. He asked how it compares to the two choices in `P-282` and whether it suggests
others. **Nothing here is decided.**

## What the measurement says

`crates/game-console/src/dump.rs` declares **28** distinct column names across ten tables. **18 are
neither a declared trait nor a kind** - counted against the release's Traits and Kinds tables:

    amount, built, capacity, citizens, count, id, in-play, labor-spent, left, made,
    node, phase, spent, structure, territories, turn, units, yards

Some are structural and defensible - `id`, `node`, `territory`, `place`. Most are vocabulary the dump
invented. **So the six Sean has caught by reading - `founded`, `readiness`, `capacity`, `citizens`,
`yards`, `labor-spent` - are samples from eighteen**, not a list.

## How the three compare

**`P-282` choice A**, dropping `labor-spent`, removes one of eighteen and leaves the mechanism.

**`P-282` choice B**, declaring `citizens` and `yards` as derived traits, is worse than it looks. It
means the release grows a trait per count anyone wants to print - `citizens`, `yards`, `extractors`,
`stores`, `units` - **a name per query**, which is what `P-262` says not to do: many near-identical
declarations differing in one word.

**The map form removes the ability to invent a name.** `citizens:8` becomes `{citizen ready:yes} ->
8`; `yards:1` becomes `{yard} -> 1`; `labor-spent` is `{citizen ready:no}` and needs no name at
all. **Every count becomes a query over vocabulary that already exists**, so the release never
declares `citizens`, and the dump cannot invent one.

## Why it is not a new idea, which is the strongest thing about it

**`P-257` already draws the line it needs.** A thing that must be referred to individually has an
`id` and is listed; a thing that need not is fungible and collapses into a count. That is the
counted-versus-listed distinction from
[storage shapes](storage-shapes.md), and **the map form is what *counted* looks like when it is
written down.**

**`P-262` supplies the other half.** One kind with a trait beats several kinds differing in one word,
and one *query shape* beats several column names differing in one word for the same reason.

## What it does not settle

**Which traits form the grouping key.** `{citizen} -> 14` and `{citizen ready:yes} -> 8,
{citizen ready:no} -> 6` are both true. Grouping by every *stored* trait and never by a derived one
is the rule that seems to follow from the specification, but nothing says it. **This is the same
shape as the bins question**: full freedom is unusable and the organising principle has to be chosen.

**Whether a zero appears.** `{citizen ready:no} -> 0` present, or the row absent? Absent is quieter
to diff and makes *went to zero* indistinguishable from *the dump stopped emitting it*.

**Order.** Diffing needs a total order over descriptions, or the file churns.

## What it suggests that neither choice did

**The dump becomes the containment tree.** `P-257` says containment is a tree with the game at its
root. If each place lists what it contains - fungible things counted, identified things listed -
**the dump stops being ten tables and becomes one structure**, and `store`, `garrison`, `extractor`
and `structure` stop needing tables of their own. The territory row's `citizens`, `yards` and
`labor-spent` are three counts that a container's contents already give.

**And `spent` stops existing.** The `labor` table prints `made`, `spent` and `left`; `spent` is
history rather than state, and `spec/invariants.md` says the data is what is there. Under a map,
`{citizen ready:no} -> 6` says the same thing as a fact about now.

## The one part that holds whatever he decides

**Every word in a dump line should be a kind, a declared trait, or a trait value.** That check would
have caught all six he found and the twelve he has not, and it is worth having under any of the three
shapes. Filed as `P-284`.

## Testing his four restrictions, 2026-09-05

He proposed trying the map form under four rules: **stored traits only**, **no omitted traits**,
**every unique combination is its own key**, and **no zero entries**. Asked whether that stays
internally consistent. **It does, and three things follow from it that have to be settled first.**

The release declares **eighteen** traits, **fourteen stored** and four derived - `metal in it`,
`control`, `surplus` and `unpaid`. So rule 1 is a real restriction and rule 2 is applied to those
fourteen.

### It is consistent, and rule 3 earns its place

**Listed and counted stop being two things.** `P-257` says a thing that must be named individually
carries an `id` and cannot collapse into a count. Under rule 3 that is not a separate rule: **a thing
with an id has a unique combination, so its entry is 1, and a list is a map whose counts are all 1.**
The two cases become one mechanism, and the line between them moves to a single question - *does this
kind carry an id* - which is where `P-257` already put it.

### Three things that have to be settled before it can be tried

**1. `id` is not a declared trait.** Measured: the Traits table has no `id` row, and `P-254` made
`id` a thing's own identifier without declaring it. **Under rule 2 a key carries every stored trait,
so if `id` is not one, two units alike in every declared trait merge into one entry with a count of
two - and neither can be commanded.** So the rules require `id` to be declared as stored, of the
kinds that have one. That is inside `P-284` rather than new.

**2. `place` is stored and is of every thing, so rule 2 puts it in every key.** That is consistent
and it makes the map flat: one map for the whole game, every entry saying where it is. **The
alternative is nesting** - the containment tree, where position states the place and the key omits
it - **and that is rule 2 with one exception.** Flat is simpler to state and harder to read; nested
reads like the tree the specification already describes. **A choice, not a defect.**

**3. Two stored traits are not attributes.** `density` and `total capacity` are *per resource* and
*per kind*, so they are maps themselves; `adjacency` is *which places it touches, and by which kind of
edge*, which is a relation. **Under rule 2 all three go in a territory's key**, which makes every
territory unique and its key long. Consistent, and it means **the map compresses nothing for
territories** - correctly, because there are twelve of them and they differ.

### Where the compression actually lands

**Fungible in practice**: citizens split by `ready`, labor, food, metal and energy by place, stores
by resource, yards, garrisons. **A player never needs to name one metal store rather than another**,
so `{store place:1 resource:metal} -> 3` is the whole truth about them.

**Never fungible**: territories, and any unit that a command must name. **Extractors are the
interesting case** - `node` distinguishes them and `node` is not a declared trait either, so today
they would merge and `build extractor` would have nothing to aim at.

### The cost of rule 4, which is not an inconsistency

**Absence means zero, uniformly, so nothing is ambiguous about the state.** What it costs is
diffing: *went to zero* and *stopped being emitted* look identical in a diff, and the second is how a
dump bug hides. **A count of entries in the file makes that visible again** - the same fix as
counting the cases a check covers.

## What it implies for the physical structure, 2026-09-06

He asked: this is conceptually a tree presented as a normalized relational structure - what does that
imply for the physical one? **The short answer is that the presentation implies nothing and the tree
implies a great deal, and the model already contains the answer once and contradicts it twice.**

### The rendering constrains nothing, and that is already a rule

*Specification and presentation are different concerns and do not share a format*, and **a rendering
is generated and never canonical**. So *normalized relational* is a view. The physical structure owes
it one thing only: the ability to produce it.

### The model already has the shape, in one of three places

Measured in `crates/game-model/src`:

- **`Thing { kind, traits: BTreeMap<Trait, u32>, children: Vec<Thing> }`** - a tree node, with traits
  already a map and children already nested. **This is the target, and it exists.**
- **`Unit { id, kind, location: Location, ... }`**, held in `Game.units` beside the territories -
  **location as a field, in a flat list.** That is exactly what `P-285` says location is not.
- **`Territory`** - typed fields for `biome`, `nature` and `nodes`, plus `held: Vec<Thing>`. **Half
  record, half tree.**

**So the same concept has three physical forms**, and the disagreement between them is not stylistic:
one of them can express a state the specification forbids.

### The implication that matters: unrepresentable beats checked

**In `children: Vec<Thing>` a thing cannot have two parents, cannot contain itself, and cannot be an
orphan.** The structure has no way to say those things, so `P-257`'s three rules are free.

**In `Game.units` with a `location` field, all three are expressible** and only a check would notice.
Two units could claim the same id; a location could name a territory that does not exist; nothing
would be structurally wrong.

**This repository's own standard settles which is better.** *An operation that cannot fail loudly
will fail quietly*, and a quality improvement's evidence is *a check that did not exist before*.
**A structure that cannot express the error is stronger than a check that catches it**, because it
needs no maintenance and cannot itself go stale - which is what half of this year's findings have
been about.

### Rust rewards the same choice

Ownership in Rust is a tree. `children: Vec<Thing>` is the borrow checker enforcing containment for
nothing; a flat arena with parent ids re-implements by hand what the language gives away. **The form
that matches the concept is the form the language is easiest in**, which is rare enough to take.

### What the relational view then needs, and why `P-284`'s exceptions are not a wart

A normalized view needs a foreign key per row. **Under `P-285` there is no `place` trait to supply
one**, so the view must synthesise the container's reference from position. **That is exactly what
`P-284`'s *named few structural words* are for**: they are the join keys, they belong to the
rendering, and they are the reason the rule cannot simply say *every word is a trait*.

### Where the map form changes the physical structure rather than the rendering

`children: Vec<Thing>` lists. His map form says identical children collapse, so a container holds
**`BTreeMap<Description, u32>`** rather than a vector.

- **Mutation becomes a transfer between keys** - one off `{citizen ready:yes}`, one onto
  `{citizen ready:no}`. That is more faithful to *matter is conserved and its arrangement is not*
  than mutating a field, and **it makes conservation checkable as a sum** rather than as a property
  nobody can state.
- **Ordering comes free.** A tree has no order among siblings and a diffable file needs one.
  `BTreeMap` gives it; a hash map would churn `expected/play.4x` on every run. `Thing.traits` is
  already a `BTreeMap`, so the instinct is already in the code.
- **The cost is indexing.** Anything that says `extractors[3]` has nothing to index. Extractors are
  distinguished by `node`, so they would carry it in their key and never collapse - which requires
  `node` to become a declared trait, as `P-284` already says.

### One thing that is true today and is worth someone's attention

**`Unit` carries `location` and `Thing` does not.** The moment `P-285` lands, that field is a
statement the specification says nothing may make. It is the code lane's, and it should move with
the map form rather than before it.
