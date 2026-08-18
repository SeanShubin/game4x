# Game 4X

A 4X game written in Rust. 4X is the genre known as **e**xplore, **e**xpand, **e**xploit, **e**xterminate.

This repository currently holds **design documentation only**. Code lands as the
prototypes below get built.

## Start here

| Document | What it covers |
| --- | --- |
| [Vision](docs/vision.md) | What the game is, the design constraints, and the non-negotiables |
| [Architecture](docs/architecture.md) | Module boundaries, the composition root, dependency rules |
| [Prototypes](docs/prototypes/README.md) | Standalone programs demonstrating one aspect of the game each |
| [Theory](docs/theory/README.md) | Background research the design leans on |
| [Documentation map](docs/README.md) | Every document in the repo, and the rules for adding one |

## The short version

- The programming language is Rust.
- The game is organized into multiple modules.
- A thin composition root module ties everything together.
- The game logic module uses **whole numbers only** — no floating point.
- A separate module renders the game.
- Other modules exist as needed to keep the code clean.
- Many prototypes demonstrate individual aspects of the game in isolation.
- Documentation stays organized: **everything is discoverable from this file.**

## Theory documents

The two pieces of background research the planet view depends on:

- [Splitting a sphere into regions](docs/theory/region-splitting.md) — why a perfect
  hex grid on a sphere is impossible, which tessellations exist, and which one this
  project uses.
- [Coloring regions](docs/theory/region-coloring.md) — the minimum number of colors
  needed so no two adjacent regions match, and how to pick colors humans can actually
  tell apart.

## Prototypes

- [Planet view](docs/prototypes/planet-view.md) — 3D and 2D views of a sphere divided
  into regions, plus a simplified view showing only what the game mechanics see.

## Repository layout

```
README.md            you are here — the entry point to everything
Cargo.toml           workspace root; crates get added as members
docs/
  README.md          documentation map and documentation rules
  vision.md          what the game is
  architecture.md    how the code is organized
  prototypes/        one document per prototype
  theory/            background research, independent of implementation
```
