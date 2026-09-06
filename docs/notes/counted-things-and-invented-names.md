# Counted things and invented names

**2026-09-05.** Sean, reading `scenario/expected/play.4x`, asked what `labor-spent` means, then
described what the predecessor did instead: a map from a thing's description to a quantity, so
`[{{citizen ready:true}} -> 8, {{citizen ready:false}} -> 6]` is eight ready citizens and six
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

**The map form removes the ability to invent a name.** `citizens:8` becomes `{{citizen ready:yes}} ->
8`; `yards:1` becomes `{{yard}} -> 1`; `labor-spent` is `{{citizen ready:no}}` and needs no name at
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

**Which traits form the grouping key.** `{{citizen}} -> 14` and `{{citizen ready:yes}} -> 8,
{{citizen ready:no}} -> 6` are both true. Grouping by every *stored* trait and never by a derived one
is the rule that seems to follow from the specification, but nothing says it. **This is the same
shape as the bins question**: full freedom is unusable and the organising principle has to be chosen.

**Whether a zero appears.** `{{citizen ready:no}} -> 0` present, or the row absent? Absent is quieter
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
`{{citizen ready:no}} -> 6` says the same thing as a fact about now.

## The one part that holds whatever he decides

**Every word in a dump line should be a kind, a declared trait, or a trait value.** That check would
have caught all six he found and the twelve he has not, and it is worth having under any of the three
shapes. Filed as `P-284`.
