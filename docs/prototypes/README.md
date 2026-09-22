# Prototypes

[Documentation map](../README.md) · [Root README](../../README.md)

Each prototype is a standalone program demonstrating **one** aspect of the game in
isolation. Prototypes exist to answer a question, and a prototype is finished when its
question is answered — not when it is polished.

| Prototype                                                   | Question it answers                                                                                                                                                   | Status                                                          |
| ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| [Planet view](planet-view.md)                               | Can we divide a sphere into hex-like regions and show them so the world reads as a world?                                                                             | Built ([code](../../prototypes/planet-view/README.md))          |
| [Goldberg view](../../prototypes/goldberg-view/README.md)   | Which territory counts read as a planet rather than a die or a fog of small cells?                                                                                    | **Answered** 2026-08-30, below                                  |
| [Kinds](../../prototypes/kinds/README.md)                   | What do the inputs to the gameplay logic actually look like?                                                                                                          | **Answered** 2026-08-31, below                                  |
| [Gap view](../../prototypes/gap-view/README.md)             | Can a player set a destination anywhere on the world with one mouse gesture, without rotating anything?                                                               | Built ([code](../../prototypes/gap-view/README.md))             |
| [Hex torus view](../../prototypes/hex-torus-view/README.md) | At the sizes the game would use, does a flat isotropic hex torus read as a world, and how visible is the wrapping? **Not whether a torus should be the game's shape** | Being built ([code](../../prototypes/hex-torus-view/README.md)) |
| [Thin engine](../../prototypes/thin-engine/README.md)       | Can a rule run from rows of data, against an engine that names no game noun? **`C-114`'s open half, answered by existing rather than decided**                        | **Answered** 2026-09-14, below                                  |
| [Goldberg move](../../prototypes/goldberg-move/README.md)   | Can a player lay out a move of any length across a sphere by clicking, without the interface ever having to guess which way round he meant to go?                     | Being built ([code](../../prototypes/goldberg-move/README.md))  |

## Conventions

- One prototype per crate, each with a `README.md` linked from this index. **Most are workspace
  members and `thin-engine` is deliberately not** - `S-136` keeps it outside so the gate does not
  fire on a thing being grown one concept at a time, and so nothing it depends on can arrive
  without being asked for.
- Each prototype gets a run script in [`scripts/`](../../scripts/README.md), so running
  one never requires remembering a cargo incantation.
- A prototype depends on real modules where that is the point, and fakes everything else.
  A rendering prototype does not need real game rules; feed it generated state.
- A prototype may take shortcuts the game may not, as long as the document says which
  ones and why.
- A shortcut is something a prototype does not build. It is not something that stops the
  prototype answering its question. Where a prototype exists to settle what something
  **looks like**, the means of seeing it is the instrument the question needs rather than
  polish, because an appearance cannot be settled by a measurement. The test: does leaving
  it out save work, or does it prevent the question being answered?
- Every prototype document states its **question** up front and records the **answer**
  when it has one. That answer is the deliverable; the code is a byproduct.

## Thin engine: answered 2026-09-14

**Yes for one mechanic, and the reading it was built to take has not been taken.** `move` runs from
**eleven rows of data** against **232 lines of code that contain no game noun** - checked against a
word list read out of `data/` rather than written by hand. Sean's two tests pass: the scout moves
from territory 1 to 2, and does not move from 1 to 3.

**The number is not a ratio and must not be read as one.** 232 lines runs **one** mechanic;
`crates/game-model/src/` is 1,662 lines for **twenty-six**. *Seven times smaller* is not a reading
this supports. **The claim under test is that 232 does not grow when a mechanic is added**, and the
second mechanic is what tests it. **An instrument built, and taken once.**

**`C-114`'s three explosions get one reading each, and the third is unknown.** The code: no
explosion. The data structure: no explosion, and this is the one that surprised - **rules are
written in the same structure as facts**, so `{needs rule:move relation:at thing:$it place:$from}`
is a row exactly as `{at thing:scout place:1}` is, and there is one structure rather than two. The
data itself: **honestly unknown**, because eleven rows is too few to read anything off.

**The engine's whole vocabulary is four words** - `rule`, `needs`, `drops`, `adds` - and all four
are about how a rule is **stated** rather than about what the game **is**.

**Nothing in it is normative.** `S-136` required that and the code lane agrees: a second mechanic
could take it apart. A form that appears here and not in `spec/` is not a contradiction, and
anything worth keeping comes back as a proposal.

**Three absences are the design rather than gaps.** Territories are stated and read by nothing, so
*not adjacent* and *no such place* refuse identically - which is the next concept, and first because
it is the smallest one needing the engine to check a row it was not handed. Adjacency is
one-directional. And the notation is re-implemented with `crates/command-language` unused, which
`S-136` required and the prototype's own README argues, so a reader who finds the duplication finds
the reason with it.

**Fifteen tests, three of them poisoned before being trusted** - a game noun added to `src/`, a file
read added to `src/`, and `{adjacent from:1 to:3}` added to the world - each failing the run it
should have. **The isolation is asserted rather than promised.**

Recorded from the code lane's `C-126`; the argument is in
[`prototypes/thin-engine/README.md`](../../prototypes/thin-engine/README.md).

## Goldberg view: answered 2026-08-30

**The question was aimed at appearance, and appearance turned out not to be the constraint.** Sean
looked at all ten solids and reports that **the five sizes the game uses are fine as planets**, and
that everything past the fifth looks like a planet too.

Two things in the answer are worth keeping rather than the verdict alone.

**Twelve does not read as a planet in polyhedron form, and it does not matter.** A drawing of the
solid at that count looks like a die. But `spec/planet.md` says the planet *is* a sphere and the
tessellation is a division of it, so what is drawn represents a sphere either way - and if the
division ever needs to be visible at that size, the answer is to draw the boundaries **on a sphere**
rather than to draw the polyhedron. The prototype shows the practical drawing, which is the abstract
one, so this is a fact about the prototype rather than about the game.

**What actually limits planet size is strategic depth, not looks.** Sean: *two units with ranges of
5 or 6 versus 50 or 51 have different gameplay feels.* One step of range is a fifth of the
difference on a small planet and a fiftieth on a large one, so **the discrimination between adjacent
values collapses as the planet grows**. Every number that is counted in territories - range,
distance, movement, the reach of a weapon - loses resolution the same way. That is the diminishing
return, and it is why the list stops at five rather than continuing up a sequence that has twenty
members below 500.

**So the prototype answered the question it was built for and the question turned out to be the
smaller half.** It established the technical capability, which is what Sean says he was after: all
ten build, all ten draw, and stepping between them costs nothing.

## Kinds: answered 2026-08-31

**The question was whether the two tables hold together**, which markdown cannot tell you.
`prototypes/kinds` is the same content as Rust data - enums for the kinds, a struct per
recipe, figures hardcoded - with a test that renders it back into the release's tables and
compares them cell by cell. It does not play: no turn, no board, no state.

**`S-4` predicted four things it would force open. All four were real.** The prototype's own README
carries them in full; the shape of each:

- **A quantity is not a number.** Twelve of the fifteen give a number everywhere; three do not, on
  four rows, and none of the four is *missing* a figure - each is known only when applied. `work`
  yields the **density** of the node being worked; `spoil` and `ready` take **any**. So a quantity is
  `Exactly(n)`, `Density` or `Any`, and everything reading the data handles all three.
- **A trait is a second axis** - not a kind and not a container. `move` takes *unit, here* and yields
  *unit, there*: same unit, same kind, different location.
- **`node, unworked` and `food, surplus` stay traits**, flagged *derived*, since each is a comparison
  between two counts rather than anything stored. **Two rows out of fifty-odd**, which is what the
  choice between derived kinds and comparisons costs, and why the cheaper answer wins.
- **Scope is a field**: ten `here`, five `every`, nothing else differing.

### The fifth, which nobody asked for and is the most useful

**Three of the nouns are families rather than kinds.** `work` outputs a *resource* without saying
which, `ready` readies a *thing* whatever kind it is, `move` moves a *unit*. The code lane's account
of why that matters is the part worth keeping: **writing the enum out made it obvious, and reading
the table it is invisible, because a reader supplies the generality without noticing they are doing
it.**

**It costs nothing to accommodate, and that is worth saying beside it.** A family need not be a
parent class - `spec/invariants.md` says every kind is data, and under a trait bag *unit* is simply a
trait that Ark and Pioneer both carry. A recipe naming `{unit}` matches anything carrying it,
which is the predecessor's `isPartOf` and needs no hierarchy. **So the finding is a real gap in what
the tables say and not a gap in the shape** - which is the good kind, since Sean ruled out
inheritance on his own grounds weeks before this came up.

**Independently checked here rather than taken**: ten `here` against five `every`, four quantities
that are not plain numbers across three recipes, fifteen recipes, and the three
generic nouns - all read off `releases/first-release.md` and all matching. The code lane also reports
two of its own first-pass figures were wrong and its assertions caught them.

