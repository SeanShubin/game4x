# Prototypes

[Documentation map](../README.md) · [Root README](../../README.md)

Each prototype is a standalone program demonstrating **one** aspect of the game in
isolation. Prototypes exist to answer a question, and a prototype is finished when its
question is answered — not when it is polished.

| Prototype                                                 | Question it answers                                                                       | Status                                                 |
| --------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| [Planet view](planet-view.md)                             | Can we divide a sphere into hex-like regions and show them so the world reads as a world? | Built ([code](../../prototypes/planet-view/README.md)) |
| [Goldberg view](../../prototypes/goldberg-view/README.md) | Which territory counts read as a planet rather than a die or a fog of small cells?        | **Answered** 2026-08-30, below                         |

## Conventions

- One prototype per crate, each its own workspace member, each with a `README.md` linked
  from this index.
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

