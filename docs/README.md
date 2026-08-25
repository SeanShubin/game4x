# Documentation Map

Every document in this repository, and the rules for adding another one.

The entry point is the [root README](../README.md). If a document is not reachable
by following links from there, it does not exist as far as the project is concerned.

## The map

```
README.md ......................... entry point, links to everything below
spec/
  README.md ....................... the specification index and its rules
  planet.md ....................... the sphere, its regions, what a territory carries
  economy.md ...................... resources, structures, labor, citizens
  units.md ........................ the unit types and what each one does
  console.md ...................... the command language and what it must cover
  combat.md ....................... ranges, weapons, resolution
  orbit.md ........................ the orbital layer
releases/
  README.md ....................... what each delivery includes; files are deleted once vetted
  first-release.md ................ one tiny planet, colonize through to launching a colonizer
docs/
  README.md ....................... this file: the map and the rules
  vision.md ....................... what the game is and what it must not become
  architecture.md ................. module boundaries and dependency rules
  layers.md ....................... old world + events -> new world, and what that demands
  prototypes/
    README.md ..................... index of prototypes and their shared conventions
    planet-view.md ................ sphere divided into regions; 3D, 2D, simplified views
  notes/
    README.md ..................... index of derived records; Claude's, not normative
    region-schemes.md ............. every scheme for dividing a sphere, measured
  theory/
    README.md ..................... index of background research
    region-splitting.md ........... dividing a sphere surface into regions
    region-coloring.md ............ minimum-color assignment and perceptual palettes

scripts/
  README.md ...................... one script per prototype, and the conventions
tools/
  pad-tables/ .................... markdown table aligner; outside the workspace
hooks/
  pre-commit ..................... pads staged markdown; opt in with core.hooksPath
crates/
  sphere-tessellation/README.md .. seeds, adjacency, and how the crate is tested
  graph-coloring/README.md ....... the coloring ladder and why the fallback is a bug
  planet-model/README.md ......... the fold: old world + intents -> new world
  planet-ecs/README.md ........... regions as entities; systems that only gather and apply
  planet-render/README.md ........ camera, rasterizer, and the engine boundary
  planet-bevy/README.md .......... the Bevy adapter, and why Bevy at all
prototypes/
  planet-view/README.md .......... the composition root, in three small files
```

Crate READMEs cover how a crate works internally and are linked from
[architecture.md](architecture.md). Documents under `docs/` cover why it works that way.

## What goes where

| Kind of content                                                | Home                                                                                |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| What the game **is**, normatively                              | `spec/<name>.md` - Sean's words only, see [CLAUDE.md](../CLAUDE.md)                 |
| A record of analysis or a rejected path                        | `docs/notes/<name>.md` - derived, dated, not binding                                |
| What the game is, and why                                      | [vision.md](vision.md)                                                              |
| How the code is split up                                       | [architecture.md](architecture.md)                                                  |
| Why it is split that way, and what must be reproducible        | [layers.md](layers.md)                                                              |
| A thing we are building to learn something                     | `prototypes/<name>.md`                                                              |
| How to actually run one                                        | `scripts/<name>.ps1` and `.sh`, listed in [scripts/README.md](../scripts/README.md) |
| Background that would be true even if we used another language | `theory/<name>.md`                                                                  |
| How a specific crate works internally                          | that crate's own `README.md`, linked from `architecture.md`                         |

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
