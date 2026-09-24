# The carrier fires on the wrong event, and the rule points at an instrument that cannot answer it

**Derived.** 2026-09-24, on the code lane reporting that `C-120` read *three* for ten days and that
`crates/outbox.md`'s `**derived from**` convention did not save it. Not binding.

[Quality](README.md) · [The outbox tool](../../tools/outbox/src/lib.rs)

## The code lane's diagnosis is wrong in a way that matters

They wrote: *nothing reads a `derived from` looking for rows that are gone*. **Something reads
`derived from`.** `tools/outbox/src/lib.rs:656`, `sharing_a_rule`, is its only consumer, reached by
`outbox --closing`. Its own doc names what it is: `P-250`'s second half, given a mechanism by
`S-41`.

**It matters because the conclusion invites rebuilding something that works.** The convention is
carried, and it is carried by a real instrument with an honest doc comment - *this makes the failure
findable, not found*.

## What it actually does, and why `C-120` was never in its path

```rust
for closed in closing {
    let Some(rule) = &closed.derived_from else { continue };
    // open items whose derived_from string equals this one's
}
```

**Two conditions, and `C-120` met neither.**

- **The trigger is an outbox item closing.** What invalidated `C-120` was `P-522`, a **promotion**,
  which closes no item. Nothing fired because nothing closed.
- **The match is one item's rule string against another's.** It relates items to items. **It never
  looks at the data**, so no version of it could have noticed that two of the three rows `C-120`
  counted had stopped existing.

**So the carrier is sound and its trigger is the wrong event for this failure.** `C-9` - the case
`P-250` was written from - went stale when `C-11` **closed**, which is exactly the event
`sharing_a_rule` watches. `C-120` went stale when a promotion deleted rows, which nothing watches.
**This is the first instance under the other trigger.**

## And the promotion side names an instrument that cannot answer its question

**Where.** `CLAUDE.md:693`.

**What.** *After promoting, check the index for open items that cite the destination file -
`outbox` lists them - and tell their owner.*

**`outbox` does not list them.** Its modes are `--places`, `--to`, `--count`, `--check`,
`--orphans`, `--waiting`, `--closing`, `--settled`, `--write`, `--sections` and `--item`. **None
takes a file.** The plain listing gives an id, a one-line subject and which outbox the item lives
in - nothing about what a body cites - so a reader cannot tell from it which items name
`spec/data/line.4x` without opening each of the 53 open ones.

**Whether the sentence is wrong or merely loose, the effect is the same**: the promotion-side
staleness check is a manual scan nobody performs, and `C-120` is what that costs. **Ten days, and
it was found by its own author writing a second reader** - not by the convention, not by the rule,
and not by any check.

## The size of what is uncovered, counted rather than guessed

**Items carrying a `**derived from**` line**: 101 in `crates/outbox.md`, 15 in
`docs/notes/proposals.md`, 2 in `lenses/quality/outbox.md`, 2 in `lenses/research/outbox.md`, 0 in
`decide/proposals.md` - `grep -c`. **One consumer**, on one trigger.

**The count is the population and not the defect.** Nothing here says any of the other 119 is
stale; what it says is that the event that made `C-120` stale is unwatched for all of them.

## Whether

**Worth doing eventually, and the cheap half is a query rather than a check.** A mode taking a path
and listing open items whose body mentions it would make `CLAUDE.md:693` performable in a second.
It verifies nothing and will fire on items that merely mention the file - which is the same bargain
`--settled` already makes, and `--settled` earns its place.

**What is not available is a semantic version**, and the reason is `P-245`'s wall: `derived from` is
a rule stated as prose, so nothing can ask whether a deleted row is one the prose was about.
**A grep that a person reads is the honest ceiling here**, and saying so is better than leaving the
document promising a query that does not exist.

**The other repair is the sentence**, and that is the specification lane's rather than this lens's:
`CLAUDE.md:693` could say what the reader actually has to do instead of naming a tool that will not
do it.
