# Prototypes

[Documentation map](../README.md) · [Root README](../../README.md)

Each prototype is a standalone program demonstrating **one** aspect of the game in
isolation. Prototypes exist to answer a question, and a prototype is finished when its
question is answered — not when it is polished.

| Prototype | Question it answers | Status |
| --- | --- | --- |
| [Planet view](planet-view.md) | Can we divide a sphere into hex-like regions and render them well in both 3D and 2D? | Designed, not built |

## Conventions

- One prototype per crate, each its own workspace member, each with a `README.md` linked
  from this index.
- A prototype depends on real modules where that is the point, and fakes everything else.
  A rendering prototype does not need real game rules; feed it generated state.
- A prototype may take shortcuts the game may not, as long as the document says which
  ones and why.
- Every prototype document states its **question** up front and records the **answer**
  when it has one. That answer is the deliverable; the code is a byproduct.
