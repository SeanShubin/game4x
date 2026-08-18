# Documentation Map

Every document in this repository, and the rules for adding another one.

The entry point is the [root README](../README.md). If a document is not reachable
by following links from there, it does not exist as far as the project is concerned.

## The map

```
README.md ......................... entry point, links to everything below
docs/
  README.md ....................... this file: the map and the rules
  vision.md ....................... what the game is and what it must not become
  architecture.md ................. module boundaries and dependency rules
  prototypes/
    README.md ..................... index of prototypes and their shared conventions
    planet-view.md ................ sphere divided into regions; 3D, 2D, simplified views
  theory/
    README.md ..................... index of background research
    region-splitting.md ........... dividing a sphere surface into regions
    region-coloring.md ............ minimum-color assignment and perceptual palettes
```

## What goes where

| Kind of content | Home |
| --- | --- |
| What the game is, and why | [vision.md](vision.md) |
| How the code is split up | [architecture.md](architecture.md) |
| A thing we are building to learn something | `prototypes/<name>.md` |
| Background that would be true even if we used another language | `theory/<name>.md` |
| How a specific crate works internally | that crate's own `README.md`, linked from `architecture.md` |

The split that matters most: **theory documents describe the problem space, prototype
documents describe what we are building.** A theory document should still be correct
and useful if this project is abandoned. A prototype document is allowed to say "we
chose X because it was easy."

## Rules for adding a document

1. Add the link before you add the content. A document nobody can reach is worse than
   no document.
2. Every document links back up to its index, and its index links back to the root
   README.
3. One topic per file. If a section grows a table of contents of its own, split it.
4. State decisions as decisions. "We use a Voronoi tessellation" beats "we could
   perhaps use a Voronoi tessellation." Record rejected alternatives and the reason.
5. Prefer prose over bullet fragments when explaining *why*; prefer bullets and tables
   for lists of facts.
6. Anything undecided goes in an explicit **Open questions** section at the bottom of
   the document, not scattered through the text.
