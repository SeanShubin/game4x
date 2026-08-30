# Prototypes

[Documentation map](../README.md) · [Root README](../../README.md)

Each prototype is a standalone program demonstrating **one** aspect of the game in
isolation. Prototypes exist to answer a question, and a prototype is finished when its
question is answered — not when it is polished.

| Prototype                                                 | Question it answers                                                                       | Status                                                 |
| --------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| [Planet view](planet-view.md)                             | Can we divide a sphere into hex-like regions and show them so the world reads as a world? | Built ([code](../../prototypes/planet-view/README.md)) |
| [Goldberg view](../../prototypes/goldberg-view/README.md) | Which territory counts read as a planet rather than a die or a fog of small cells?        | Built, answer not yet recorded                         |

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
