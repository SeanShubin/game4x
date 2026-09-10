# gap-view

[Prototypes](../../docs/prototypes/README.md) · [Architecture](../../docs/architecture.md) · [Root README](../../README.md)

**The question.** Can a player set a destination anywhere on the world with **one mouse
gesture, without rotating anything**? On the globe the mouse does two jobs - drag to turn,
click to choose - and reaching the far side needs both, which Sean reports as clunky. A flat
map needs only the click, and the standing objection is that flattening a sphere distorts what
it draws.

**Sean's proposal, 2026-09-09**, which is what this builds: *keep the territory shapes the
same and use gaps between them in order to do the projection. We have to pay the distortion
tax somewhere, so I am considering doing it in between territories rather than stretching
territories.*

Run it with [`scripts/gap-view.ps1`](../../scripts/README.md), which writes
`drawing/index.html` - four worlds to look at, and the numbers beside them.

## The answer, so far

**The framing is right and the tax is a fixed quantity.** Descartes' theorem: the total
angular defect of a closed solid is `360 * chi` degrees, and `chi` is 2, so **720 degrees** -
at every planet size. Flattening rigid faces has to open that much angle somewhere, so a
layout never chooses *how much* to pay, only *where*. `the_total_defect_is_720_degrees_at_every_size`
computes `chi` from each of the five solids rather than taking the theorem's word.

**The projection that makes it work is azimuthal equidistant**, and the reason is a property
no other projection has here. Its principal scale factors are **1 along the radius** and
**`theta / sin(theta)` across it**. Radial scale of exactly 1 means rings keep their true
spacing, so rigid tiles are never driven into each other; tangential scale of at least 1 means
the space between them only ever opens. An equal-area placement compresses radially near the
rim and rigid tiles would collide there; a gnomonic one tears them apart before the horizon.

**The whole world fits on one page, so nothing has to be rotated to.** That is the question
answered, in the narrow sense. The rest is what it costs.

### What it costs

| the map reaches out to | share of the world shown | share of the page that is gap |
| ---------------------- | ------------------------ | ----------------------------- |
| 60 degrees             | 25%                      | 9%                            |
| 90 degrees             | 50%                      | 19%                           |
| 120 degrees            | 75%                      | 32%                           |
| 180 degrees, all of it | 100%                     | **60%**                       |

**So showing the whole world costs a page that is three fifths empty**, and showing half of it
costs a fifth. That is the trade this puts in front of you, and it is the thing to look at the
drawing for rather than the table.

### Three findings that were not in the question

- **The gap you want for looks is free.** Flattening a *spherical* polygon at its true radii
  makes it slightly larger than it was, so territories touching along the line to the focus
  cross. Measured: nothing at all at 12 territories, 0.1 per cent at 32 and 42, half a per
  cent at 92. Any gap chosen for appearance is an order of magnitude more, so it costs
  nothing. **This began as a claim that the forced gap is never zero, and that was wrong** -
  whether any pair lies near enough to that line is an accident of where the seeds fell.
- **The seam is a point, not a line.** Every other way of flattening a sphere cuts it along a
  path, and territories either side of the cut read as far apart when they are neighbours.
  Here the only territory the map cannot draw honestly is the one antipodal to the focus: it
  lands on the rim and its own neighbours spread around the whole rim circle. **Which
  territory pays that is chosen by choosing the focus**, so it can be put somewhere the player
  is not looking.
- **Touching stops being what shows adjacency**, and something has to replace it. Where the
  gaps are wide the tiles no longer meet, so the drawing draws a line between neighbouring
  centres. Whether that reads is a judgement and is the main thing the picture is for.

### What is not answered

**Whether it feels better than the globe**, which is the actual question and needs hands on
it. This draws; it does not let you click. The next step is the interactive one: click to
choose a destination, and re-centre the map on a territory to see the far side compactly.

**Whether the far quarter is good enough.** The drawing shows it sparse and spread out. It is
clickable - the tiles out there are isolated, which if anything makes them easier to hit - but
the relations between them are hard to read.

## Shortcuts, and why they are allowed

- **No terrain, no ownership, no units.** The question is about layout, and a coloured
  polygon is enough to see whether one reads.
- **Territories are the tessellation's, with `Params::default()`** - no jitter, no relaxation.
  Irregular cells are the harder case for a claim about shapes not colliding, so this is the
  conservative choice rather than a flattering one.
- **The drawing is SVG rather than the engine.** Every other prototype here drives Bevy; this
  one answers a question about a static arrangement, and a picture answers it. **The
  interactive follow-up will need the engine** and does not exist yet.

## Where the reasoning lives

`src/lib.rs` carries the geometry and the argument for it; `tests/layout.rs` carries the
claims, each asked of every territory of every planet size with the population's size
asserted. The load-bearing one is `no_tile_overlaps_another`, over 8,165 pairs: if rigid tiles
collided the proposal would be unbuildable rather than merely ugly.

## One thing this crate is not yet

**A workspace member.** `tools/outbox/tests/architecture.rs` requires every member to have a
row in `docs/architecture.md`, and `docs/` is the specification lane's column - so adding the
member before that row exists reddens the gate for everybody, including a deploy. It declares
its own `[workspace]` until the row lands, which is filed as `C-76`.
