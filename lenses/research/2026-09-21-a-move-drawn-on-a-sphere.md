# A move drawn on a sphere, and what the established practice already settles

2026-09-21. **Addressed to `code`**, carrying `X-38`. Sean, asking for something to hand the code
lane: *see what modern games do to clearly represent movement intentions, especially if they
handle curvature of a sphere.*

[Research](README.md) · [Root README](../../README.md) ·
[the prototype](../../prototypes/goldberg-move/README.md)

**Answered 2026-09-23 at `fec232b`, and one thing below is wrong.** The code lane built the arc,
the width, the dash and the eased recentre, and measured the number this report left open. **The
instrument this report named for it - `PlanetMesh::deepest()` - is the wrong call**, and the
section *The measurement* still says to make it. `deepest()` is `0.9797`, the deepest point
anywhere, which is the middle of the widest panel; a step's low point is over a **border**, where
the surface is `0.9881`. So the comparison this report recommended says the step is clear and it
is not. The answer, re-derived independently and split exactly by border class, is in `X-38`'s
closing note; the 0.0005 that did not reproduce is `X-39`.

## What the prototype draws today

Read from `prototypes/goldberg-move/src/main.rs` at `4c5e3f27`, not remembered.

| Part                | How it is drawn                                                                 |
| ------------------- | ------------------------------------------------------------------------------- |
| the route           | `gizmos.line(from, to, ..)` per step - a **straight chord** in view space       |
| each step's marker  | `gizmos.circle(..)`, radius `0.09`, yellow                                      |
| the origin's marker | `gizmos.circle(..)`, radius `0.11`, white                                       |
| how far above       | `ABOVE = 1.035`, one constant for every marker and every line                   |
| the refusal         | a sentence: *territory 30 is 4 steps away by 3 different routes*                |
| recentring          | `centre_on` writes `orbit.0.yaw` and `.pitch` **directly** - a jump cut         |
| line width, joints  | never configured, so Bevy's defaults: **2 px**, `perspective: false`, no joints |

`draw_the_route` is `main.rs:241-265` and the chord itself is `main.rs:257`; `centre_on` is
`main.rs:328-332`; the recentre fires on every `Reached` and every `Moved` at `main.rs:216-217`.
The defaults are `GizmoLineConfig::default`
in `bevy_gizmos-0.19.1/src/config.rs:267-276` - `width: 2.0`, `perspective: false`, `style: Solid`,
`joints: None` - with `depth_bias: 0.0` beside them in `GizmoConfig::default`.

## The measurement: every segment is drawn under the planet

**A chord between two points at radius `r` passes closest to the centre at `r · cos(θ/2)`**, where
`θ` is their angular separation. At `r = 1.035` that stays outside the unit sphere only while
`θ ≤ 29.886°`.

On `GP(2, 0)` it never does:

| quantity                                           | value       |
| -------------------------------------------------- | ----------- |
| neighbour separations counted                      | **240**     |
| neighbour separation, smallest                     | **31.717°** |
| neighbour separation, largest                      | **36.000°** |
| separation at which a chord at `1.035` breaks even | **29.886°** |
| chord midpoint radius, best case                   | **0.9956**  |
| chord midpoint radius, worst case                  | **0.9843**  |
| deepest below the arc it stands for                | **0.0507**  |

**All two hundred and forty, with no margin at either end.** The line that stands for one step sags
to between `0.9843` and `0.9956` of a radius while the disks it joins float at `1.035` - up to
**5.1% of a radius below the arc it represents**, and below the sphere itself for the middle of
every segment.

**The population is asserted rather than trusted.** A geodesic polyhedron of triangulation number
`T` has `30T` edges and `T = 4` here, so 120 edges counted from both ends is 240 - which is what
the script gets, and it would not land if the neighbour ring had been picked wrongly. The centres
are re-derived from the icosahedron rather than read out of `sphere-tessellation`, so this is a
second derivation and not a copy of the thing it checks. Reproduce with
`python tools/research/chord-sag.py`.

**Measured: the chord is under radius 1.0 for every step of every route. Not measured, and one
call away: whether it is under the *drawn panel* too.** The panels are flat and dip inside the
sphere as well, which `PlanetMesh::deepest()` already computes at
`crates/planet-render/src/mesh.rs:81`; that crate's own test records about `0.93` at twelve
territories and above `0.965` at ninety-two, and forty-two is between them. **I think the sag is
visible and I have not shown it** - the prototype has no `--shot`, so nothing here was
photographed.

**The repair is one substitution and the dependency already ships it.**
`Gizmos::short_arc_3d_between` - `bevy_gizmos-0.19.1/src/arcs.rs:223` - takes a centre and two
points and draws the shorter arc between them, subdividing by angle. The globe's transform is a
rotation about the origin, so the centre is `Vec3::ZERO`, and both endpoints are already at
exactly `1.035`.

```rust
gizmos.short_arc_3d_between(Vec3::ZERO, from, to, Color::srgb(1.0, 0.95, 0.4));
```

**One caution, because it will not fail loudly.** It takes its radius from `from` alone -
`let radius = center.distance(from)`, `arcs.rs:303` - so the arc lands on `to` only while both ends
share a radius. They do today. A later elevation profile, or a marker lifted to show selection,
breaks it silently.

## What the practice settles

Six things, each checked in session rather than recalled.

## Great-circle interpolation is settled, not a refinement

`deck.gl` ships this as a layer rather than as advice: `GreatCircleLayer` renders arcs along the
great circle joining each pair of endpoints, and since v8.2 it *is* `ArcLayer` with
`greatCircle: true` and `getHeight: 0`. The chord-versus-geodesic question was settled by folding
one into the other as a flag. **A path on a globe is a geodesic, and nobody argues it.**

Its documented gotcha does not bite here but is worth knowing: under `GlobeView` the arcs can
vanish from some angles, because that view enables back-face culling by default. Bevy's gizmos are
not culled that way. Noted and not.

## Occlusion has a recipe, and it is three settings

The mistake and the fix are both recorded in the open. Orbit guides drawn with `depthTest: false`
and a render order **before** the opaque bodies lose the whole arc wherever it crosses a body's
silhouette - near side included, which for a close orbit is all of it. The fix is **render after
the bodies, `depthTest: true`, `depthWrite: false`**, so a body hides its far-side arc alone; and
leaving depth writes off is what stops one segment of a wide line occluding the next.

Here the near equivalent is `GizmoConfig::depth_bias`, default `0.0`, which is already the
depth-tested behaviour. **The second half is the one that starts mattering the moment the line
stops being 2 px**: a wide line that writes depth eats its own corners.

## Direction belongs along the line, not only at its end

`KSP 1.1` fades the orbital trajectory line to show which way the craft is travelling. A gradient
along the whole path states direction everywhere the path is visible; an arrowhead states it once,
and on a sphere the end is frequently the part over the horizon.

**Bevy already has the cheapest version of this**: `GizmoLineStyle::Dashed { gap_scale, line_scale }`,
`config.rs:46-52`. Animating the phase gives a flowing dash; not animating it still gives a dashed
line, which reads as *intended* where a solid one reads as *drawn*. That distinction is the whole
of what Sean asked for by *movement intentions*.

## A 2 px line is a debug gizmo; screen-space width is what makes it a path

`three.js` ships `Line2` and `LineMaterial` for one reason: `linewidth` on `LineBasicMaterial` is
always 1 on most platforms, so wide lines have to be built as meshes carrying the viewport
resolution into the shader. The lesson outlives the library. **A path's width is a screen-space
quantity**, because a ribbon lying on a sphere foreshortens to nothing exactly at the limb - which
is where a long move is most often heading.

Bevy gives both without a mesh: `GizmoLineConfig::width`, in pixels, with `perspective: false`,
which is already screen-space. It is simply never set, so the route is two pixels wide.

## Ease the recentre; this is where the evidence is loudest

`centre_on` assigns yaw and pitch, which is a jump cut, and it fires on nearly every click. Heer
and Robertson, *Animated Transitions in Statistical Data Graphics*, IEEE TVCG 2007, ran two
controlled experiments and found animated transitions significantly improve graphical perception,
with staged animation - separating the stages of one change - better still. Google Earth and
Cesium implement exactly this case: **incremental rotation along the great circle between the two
coordinates.**

Highest-frequency item on the list, and the only one that is not about the line at all.

## Vertices at the boundary, not only at the centres

The practice for a path over a tiled map is to give the line its points as **the cell centre, then
the midpoint of the edge it crosses**, alternating. It halves the departure from the surface
before any interpolation, and it makes the path visibly *cross* each boundary rather than skip
between centres - which is what turns a polyline into *a route through these territories*.

Subsumed by the arc fix as far as geometry goes; kept because the crossing is a separate piece of
information from the curve.

## The refusal, which is a decision and not a defect

**Nothing this lane found refuses an ambiguous destination.** The shipped answer is to commit a
canonical route and let the player steer it - `XCOM 2` marks intermediate tiles with `Ctrl` held,
which is the same gesture as an RTS's shift-click queue. **The population searched was strategy
and tactics titles with a visible movement preview**, so the claim is *none of the ones reached*,
not *none*.

**That is not an argument against Sean's rule**, which this prototype exists to test and which its
README states as the whole question. What the comparison gives is one observation about the rule's
*presentation* rather than its content:

**the refusal is a sentence, and every shipped route UI is a picture.** *Territory 30 is 4 steps
away by 3 different routes* asks the player to hold three unseen alternatives in his head. The
same rule shown as **three translucent arcs fanning out from where the move has reached** answers
*which intermediate do I click* by pointing at it. `Board` already computes the routes and
`draw_the_route` already draws one, so this is the same function called more than once -
`Reach::Many` would have to carry them rather than count them.

Noted rather than filed as work: it changes what the prototype asks, and that is Sean's.

## Whether

**Four now, and they are small.** The arc, the width, the dash and the eased recentre are the whole
of *modern and clear* for this drawing, and not one of them adds a concept the prototype does not
already have. Estimated at some tens of lines; not counted, because this lane does not write them.

**Two eventually.** Boundary vertices; and the far side, where recentring is one of three shipped
answers and the others are an off-screen marker and a ghosted far-side arc. The second only starts
mattering once a move outruns the visible hemisphere - which on forty-two territories it already
can.

**One noted and deliberately not**: drawing the ambiguity instead of counting it.

## Sources

- [GreatCircleLayer, deck.gl](https://deck.gl/docs/api-reference/geo-layers/great-circle-layer)
- [ArcLayer, deck.gl](https://deck.gl/docs/api-reference/layers/arc-layer)
- [Orbit guides: render order, depth test, depth write](https://github.com/hugogu/orbit/pull/33)
- [KSP orbital line fade](https://forum.kerbalspaceprogram.com/topic/150055-orbital-line-fade/)
- [Heer and Robertson, Animated Transitions in Statistical Data Graphics, TVCG 2007](https://idl.cs.washington.edu/files/2007-AnimatedTransitions-InfoVis.pdf)
- [three.js `Line2`, and the wide-line limitation](https://threejs.org/docs/pages/Line2.html)
- [XCOM 2: marking tiles to steer a path](https://steamcommunity.com/app/268500/discussions/0/135507548128972223/)
- [Red Blob Games, hexagon tiling of a sphere](https://www.redblobgames.com/x/1640-hexagon-tiling-of-sphere/)
- [Pathfinding on a hexagonal grid: line points at centres and edges](https://azelito.itch.io/surviving-ragnarok/devlog/738954/pathfinding-on-a-hexagonal-grid)
