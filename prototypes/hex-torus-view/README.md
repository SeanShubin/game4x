# hex-torus-view

[Prototypes](../../docs/prototypes/README.md) · [Architecture](../../docs/architecture.md) · [Root README](../../README.md)

**The question.** At the sizes the game would use, does a flat isotropic hex torus read as a
world, and how visible is the wrapping? [`goldberg-view`](../goldberg-view/README.md) asks the
same thing of spheres, and these two are meant to be run against each other.

**Sean asked for it directly** on 2026-09-12, and chose the size list. Filed as `X-32` by the
research lane.

## What it draws

The plane tiles outward forever. **Exactly `N` hexes are drawn bright - one complete copy of
the planet - and every other hex is dimmed**, so that zoomed out the bright island is the whole
world and the rest is echo.

**Every hex carries the id of the bright hex it repeats.** That is the detail that does the
work: turn the ids on and `7` appears in every direction, which is what makes an echo legible
as the same territory rather than as more world. Ids are on by default here, unlike
`goldberg-view`, because the prototype does not do its job with them hidden.

## The requirement that produced two of the three families is refuted

**Sean asked for equal circumference in all six directions** on 2026-09-12, and the folded and
axis-aligned families were built to satisfy it. On 2026-09-13 he measured Solium Infernum
instead of reasoning about it: *twelve up returns to start; twelve right - alternating
up-right, down-right - returns to start.* Those two walks are `(0, 12)` and `(12, −6)`, which
generate a lattice of 144 cells that closes in **24, 12, 24**. One axis takes twice as long as
another, and **he did not notice until he went looking.**

So isotropy was never what made the pathing sensible. **What does is that the wrap happens in
the coordinates a person thinks in** - rows and columns - so *twelve up* and *twelve right* are
both twelve while the straight axes are 24 and 12 and 24. `X-37`, filed by the research lane.

The offset family is that lattice, at ten sizes. **`W` must be even**, because the generator is
`(W, −W/2)` and half a cell is not a cell; that is the standard offset-coordinate constraint
and is probably why the map he measured is twelve wide.

**The bright world is the `W × H` array itself**, indexed by column and offset row. The axial
rectangle `0..W × 0..H` is an equally valid set of representatives and draws as a
parallelogram - the lattice is identical either way and only which copy is called bright
moves - but the picture would then say *sheared lattice* where the whole claim is *the
rectangle a 2D array holds*. **Found by drawing it and looking**, which is what the prototype
is for.

|                       | at 144 territories           | isotropic |
| --------------------- | ---------------------------- | --------- |
| offset `12 × 12`      | closes in 24, 12, 24         | no        |
| axis-aligned `C = 12` | closes in 12                 | yes       |
| folded `k = 4`        | 48 territories, closes in 12 | yes       |

**Folded has no world at 144** - `3k² = 144` wants `k² = 48` and 48 is not a square - so the
three-way comparison is two at 144 and the third at the same circumference. Asserted in
`the_three_families_meet_where_they_can` rather than left for a reader to work out.

**The colour count is an output rather than a setting.**
[`crates/graph-coloring`](../../crates/graph-coloring/) climbs 2, then 3, then 4 and reports
which succeeded. **Three at all ten folded sizes; three or four in the axis-aligned family**,
three where `3 ∣ C` and four otherwise - `C = 4, 5, 7, 8, 10, 11`, six of its ten. That
difference is real and the page shows it without being told, which is the point of the count
being an output.

**This sentence said *three at every size* until `Q-87`.** It was true when the crate had one
family, and the axis-aligned family landed under it in `5ddc371` without it being re-read.

## Using the page

**Pointing at a hex lights every copy of that territory**, bright and dimmed alike, and the
heading says how many places it was drawn in. That is a distance instrument rather than a
decoration: a territory appears seven times, and **two territories that look far apart in the
bright region may be adjacent through a wrap**. The drawing can only place each one somewhere,
so without this it does not merely fail to show distance - **it misleads about it**. With every
copy lit, the nearest one is the one that decides. Sean's observation, `X-36`.

**All thirty are buttons under the heading**, grouped by family and labelled by how many
territories each has. They were reachable only by `[` and `]` for one commit, and in that
commit Sean opened the page, saw `folded, 12 territories`, and concluded the axis-aligned
family had not landed - when it had. **A correct page showing one of twenty under a header
promising twenty reads as *nothing changed***, which is worse than a missing feature because it
is a false report. `X-35`.

**`T` jumps to the world at the same circumference in the other family.** That pairing is the
only informative comparison the two families support - same distance around, three times the
territories - so **the toggle shows the folding itself**, where a toggle by list position would
compare two unrelated worlds. **Six of the thirty have a partner - three pairs**: 12↔36,
27↔81, 48↔144. The other twenty-four disable the control and say so. **The offset family has no
partner and should not**: it shares a shortest circumference with both of the others and is a
folding of neither, and matching on the number alone paired sixteen of the thirty and taught
nothing about any of them. **Whether to re-cut the axis-aligned
ladder to `C = 6, 9 … 33` so all ten pair is Sean's** - it buys the comparison at every size and
costs the small end, since the smallest world becomes 36 territories rather than 9.

**This said *three of the twenty* until `Q-87`, and it was not stale.** The denominator moved
from ten to twenty and the numerator did not: three is the number of *pairs*, and a pair is two
worlds. `partner_of`'s docstring says *three of the ten pair today* and is exactly right. A fact
restated over a larger population without rescaling is a new shape, and worth a second's
attention next time a count in this repository changes its denominator.

**`O` turns the drawing thirty degrees, between pointy-top and flat-top.** It is drawing
only: the lattice, every adjacency, every id and every wrap are identical, and what changes is
which walks read as straight. **That is why it is not merely cosmetic** - Sean's *twelve up*
and *twelve right* are flat-top readings, and neither is a straight walk in a pointy-top
picture, so the measurement `X-37` rests on is unreadable in the other orientation.

**Flat-top is the pointy-top drawing rotated by exactly thirty degrees**, measured rather than
assumed: taking `x = √3(q + r/2), y = 1.5r` into `x = 1.5q, y = √3(r + q/2)` is a matrix of
determinant 1 at 30°. So the page does it with one SVG `rotate(30)` and no second copy of any
geometry. The ids are turned back so they stay upright, and each world carries a second frame,
because turning the drawing changes which rectangle contains it.

`I` toggles the ids, drag or the arrows pan, the wheel zooms, `R` resets.

**The drawing gets the window and everything else is one line or folded away.** Sean,
2026-09-12: the picture was wedged between a block of prose above it and a table below. Both are
still there - the prose says what the page is for and the table is the thirty as numbers - each
behind a one-line `<details>`, and the stage takes whatever is left. **The picker does not
fold**, because a control nobody can see is `X-35` again.

## Two families, because Sean asked what a square grid has that a hex grid cannot

**Nothing.** Wrap each axial coordinate on its own - `q mod C`, `r mod C` - and the shifts are
`(±C, 0)`, `(0, ±C)`, `(±C, ∓C)`. The wrap is **coordinate-wise**, which is the whole of why
square-grid pathing is easy, and it is isotropic: all six directions close in `C`.

**And it is the `3k²` lattice unfolded.** The axis-aligned lattice at `C = 3k` sits inside the
`3k²` one with index exactly **three** - same circumference, a third of the territories. So
`3k²` is this torus folded into three, **and the folding is what destroys the coordinate-wise
wrap. Hexes were never the cause.**

|                        | circumnavigates            | wraps at C=6 | colours                             |
| ---------------------- | -------------------------- | ------------ | ----------------------------------- |
| folded, `N = 3k²`      | 105% of a sphere that size | 33% of steps | three, always                       |
| axis-aligned, `N = C²` | 61%, at every size         | 21% of steps | three where `3 ∣ C`, four otherwise |

**The axis-aligned family wins on pathing and on wrap frequency and loses one row**, and that
row is the one the `3k²` family was chosen for. Which the game should use is not settled here.
**Both are drawn**, folded first, because seeing them against each other is what answers the
question - and deleting a mode later is one line where deciding for him is not.

The colour count is an output rather than a setting, so the page reports the difference without
being told: four colours at `C = 4, 5, 7, 8, 10, 11`.

## The sizes, ten in each family and thirty in all

The folded family: `N = 3k²` territories with a circumference of `3k`, for `k = 2..=11`.
The axis-aligned family is `N = C²` for `C = 3..=12` - 9, 16, 25, 36, 49, 64, 81, 100, 121, 144.
The offset family is `N = W·H` for `W = H = 4, 6 .. 22` - 16, 36, 64, 100, **144**, 196, 256,
324, 400, 484. **`C = 1` and `2` are dropped as degenerate**, the way `k = 1` is, and the
offset ladder is square because Sean's measurement was `12 × 12` and a second free dimension
would multiply the page without answering anything he asked.

**The table below is the folded ten**, which is the ladder Sean chose; the axis-aligned ten are
the ten squares listed in the line above.

|     | territories | circumference | a sphere that size |
| --- | ----------- | ------------- | ------------------ |
| 1   | 12          | 6             | 5.7                |
| 2   | 27          | 9             | 8.6                |
| 3   | 48          | 12            | 11.4               |
| 4   | 75          | 15            | 14.3               |
| 5   | 108         | 18            | 17.1               |
| 6   | 147         | 21            | 20.0               |
| 7   | 192         | 24            | 22.9               |
| 8   | 243         | 27            | 25.7               |
| 9   | 300         | 30            | 28.6               |
| 10  | 363         | 33            | 31.4               |

**Why this family.** A wrapping of the hex plane is an ideal of the Eisenstein integers, so the
sizes that admit one are `a² + ab + b²`. Those fall into families by circumference-to-area, and
a sphere's ratio is 2.72. The `3k²` family sits at exactly 3 - it circumnavigates `1.73√N`
where a sphere takes `1.65√N`, **5% apart**. The square family sits at 1.0, which is 61% of a
sphere's circumference and the worst available.

**Sean chose this list over every isotropic size** - 3, 4, 7, 9, 12, 13, 16, 19, 21, 25 - which
is the stricter parallel to `goldberg-view` and whose first seven rungs are too small to ship.

## Coordinates, because the same family is written two ways

This crate uses **axial hex coordinates with 60° between the axes**: neighbours at
`(1,0) (0,1) (-1,1) (-1,0) (0,-1) (1,-1)`, norm `q² + qr + r²`, and the lattice generated by
`(k, k)` and its sixth turn `(-k, 2k)`.

**`X-32` first stated the generator in the 120° convention**, where the norm is `a² − ab + b²`
and the generator is `k(2+ω)`. Both describe one lattice, and the size list is identical either
way because the two forms represent the same integers. **What differs is the generator**: taken
into the other convention's norm it gives `7k²`, which is 28 cells where 12 were wanted at
`k = 2`. Corrected in `X-32` at `943c87f`; written here because the two look alike on the page
and the size list looks right under both.

## What is checked, and what a person still has to look at

**Vetted when** - at each of the ten sizes, zoomed out, exactly `N` hexes are undimmed and
every dimmed hex shows the id of an undimmed one. That is a person looking, and nothing below
replaces it.

The checks are what a picture cannot be trusted about - whether *exactly N bright* is true or
merely looks true. **Each runs over every case and asserts how many cases there were.** The
population is not the same for all of them - some are properties of any quotient and run over
all thirty, some are properties of the `3k²` lattice and run over its ten, and the other two
families' own are in `tests/families.rs` - so **each bullet says which**, because that is the
number a reader would otherwise assume.

**This paragraph has now been wrong twice**, both times because a count outlived its
denominator: it said the grid properties run over the folded ten an hour after two of them
stopped doing so, and it said twenty the day a third family landed. The numbers in the checks
are asserted and could not drift; prose has no such carrier, which is why it is always the half
that goes.

- **`exactly_n_cells_are_bright_and_every_cell_echoes_one_of_them`** - all thirty. **Three
  directions**: a domain of the right size proves nothing if a cell reduces outside it, every
  cell reducing into the domain proves nothing if the domain is the whole plane, and **both of
  those together still allow a domain cell nothing ever reduces to** - a bright hex that is no
  territory. That third direction could not fail for the folded family, whose domain is built
  by reducing, and can for the axis-aligned family, whose domain is the square `0..C × 0..C`
  written down independently. Found by poisoning the axis-aligned reduction while widening this
  check from ten worlds to twenty. **Four tests reddened elsewhere and this one stayed green** -
  three in `tests/families.rs` and one in this file - while `tests/drawing.rs` and
  `tests/colouring.rs` stayed green too, which is what says the new direction reaches something
  nothing else does. **The count was first written as six, sourced from nothing**: the quality
  lens had measured five under a *different* poison, and six was neither number. Re-measured at
  `5221933` by applying the poison and reading the four suites
- **`reducing_and_turning_a_sixth_commute`** and
  **`the_drawn_region_is_not_six_fold_symmetric_and_cannot_be`** - the folded ten, because the
  second is a fact about `3k`-squared and the first is checked for the other family by
  `the_axis_aligned_family_wraps_one_coordinate_at_a_time`. These replace a test called
  `the_bright_region_has_the_six_fold_symmetry_of_the_grid`, which **could not fail and named
  something impossible**. It rotated each cell and reduced the result, and a reduction maps
  every cell to its canonical representative, so the set came back whatever shape it had. And
  a set symmetric about the origin has size `1 mod 6`, which `3k²` never is. `X-34`, found by
  the research lane while asking why the picture looked lopsided when a green test said
  otherwise
- **`every_direction_wraps_in_the_same_number_of_steps`** - the folded ten. The definition of
  the family, and the thing a wrong generator breaks quietly.
  **`the_axis_aligned_family_is_isotropic`** asserts the same of the other ten
- **`every_cell_has_six_distinct_neighbours`** - all thirty. The wrap is what makes this true
  at the edge, and a cell neighbouring itself is a wrap folded onto itself whichever lattice
  folded it
- **`the_two_walks_sean_measured_are_the_offset_generators`** - the ten offset sizes, **walked
  rather than summed**: the alternating rightward walk is stepped one hex at a time and the sum
  is compared against `(W, −W/2)`, because writing it as `6a + 6b` is already half the
  conversion the check exists to verify
- **`the_offset_family_closes_in_two_w_h_two_w_and_is_not_isotropic`** - the ten offset sizes,
  and **it asserts the family fails the property the other two were built for**. A family that
  quietly became isotropic would delete `X-37`'s finding without deleting a line of it. The
  other twenty are asserted still isotropic in the same test, so a red there is about this
  family rather than about the instrument
- **`the_two_routes_to_a_circumference_agree`** - six directions at each of thirty worlds.
  `circumferences` solves `n·d = αa + βb` over the integers and never calls `reduce`;
  `circumnavigations_agree` steps and reduces. **Independent routes to one fact**, which is the
  habit that re-derived `X-37` rather than taking it
- **`three_colours_suffice_at_every_size`** - the folded ten. `Exact(3)`, and the colouring
  checked proper besides, because a method reporting success while leaving two neighbours
  matching would pass the first assertion.
  **`the_axis_aligned_family_needs_four_colours_unless_three_divides_c`** is the other family's,
  and asserts the split is four of ten and six rather than letting a run where every size agreed
  read as a clean result
- **`no_size_is_two_colourable`** - the folded ten, by finding an actual triangle at each,
  because `Exact(3)` alone is satisfied by a graph that only ever needed two
- **`turning_thirty_degrees_is_the_flat_top_layout`** - 169 cells of a patch, asserting that
  the rotation the page applies *is* the flat-top placement rather than something that looks
  like it. The flat-top formula is written in the test and not in `draw`, because `draw` does
  not need it - two routes to one point
- **`the_page_can_turn_the_drawing`** - one frame per world in each orientation, and **that the
  two frame arrays differ**, because a rotation that framed identically would mean the rotation
  is not happening and the count alone would pass against a copy
- **`the_offset_domain_is_the_array_a_game_would_store`** - the ten offset sizes, both
  directions: every column and offset row of the `W × H` array appears exactly once, and every
  cell of a patch four times the map reduces onto it
- **`the_copies_tile_without_overlap_or_gap`** - all thirty, and **the gap half was missing
  for the whole life of the axis-aligned family**. Six echoes close round a hexagonal domain
  and leave the corners of a parallelogram open, so that family shipped with two notched
  corners at all ten of its sizes, through a quality review and several screenshots sent to
  Sean. `X-37`'s offset lattice is sheared far enough that the notches became wedges, which is
  what made it visible. The parallelogram families draw **nine** copies now - `±a`, `±b`,
  `±(a+b)`, `±(a−b)` - and the check is *every neighbour of a bright cell is drawn*, which is
  what a person actually sees. No count of copies can say it and neither can no-overlap
- **`the_page_states_the_number_of_worlds_it_draws`** - the whole page. The header's spelled
  number against the
  groups actually in the page, and the per-family counts besides. Those are two independent
  things in one artifact, and `X-35` was exactly that they disagreed
- **`every_copy_of_a_territory_carries_the_same_cell_id`** - all thirty. Two marks per copy
  at each, a hex and an id, seven hexes and seven ids, which is what the hover selects on
- **`the_toggle_pairs_one_circumference_across_the_two_families`** - all thirty. Six of them have a
  partner, the pairing is mutual, and the bigger world has exactly three times the territories.
  A run where every size paired would fail here rather than read as a clean result

**Three of these read the page and not the grid, and two of them read the script text**, which
is a weaker claim than it sounds: whether a browser runs the script is what the *vetted when*
above asks a person to look at. What they catch is a control being dropped, or its handler
removed while the element stays - the shape of `X-35`, because the keys went on working the
whole time and nothing looked broken.

## How the wrapping actually works

**A step that leaves the drawn region comes back translated by one of six vectors** - `±a`,
`±b`, `±(a−b)`, the six shortest in the lattice - and each is used exactly `2k` times. Three
opposite edge-pairs, each a **constant translation**, the same everywhere along its edge. The
share of steps that wrap at all is `2/(3k)`: a third at `k = 2`, 13% at `k = 5`.

**That is a rule, and it is learnable.** The seam is hard to *see* because the drawn region is
a blob, not because the rule varies.

**It was first reported as the opposite** - *no single wrap rule, nothing for a player to
learn* - from id offsets, which are indices into a list sorted by position and say nothing
about geometry. Both lanes repeated it before anyone measured it. `X-33`, corrected.

**And it is why the region is a hexagon**, which is not the reason the hexagon was chosen. A
rhombic domain over the same lattice subtracts `±a`, `±b`, `±(a+b)`, uses its diagonal pair
**once** and the other four `4k−1` times each, and wraps about a third more often at every
size. So the hexagon wraps less and wraps evenly; the rhombus would give a player two seams
they meet once in a game and four they meet constantly.

## What this settles, and what it does not

**It settles whether the wrapping is legible. It settles nothing about whether a torus should
be the game's shape.** A circumnavigation on a torus separates nothing, northward paths never
converge, and no cell has a unique antipode - none of which is affected by how well this
renders. **If it comes out looking good, that is not evidence for the geometry.**

Written before it was built rather than after, by the lane that specified it.

**And one thing the colouring costs.** A 3-colouring of a hex grid is essentially rigid -
diagonal bands that wrap - where a Goldberg's four-colouring looks irregular. So the colouring
makes the repeating structure *more* visible, not less. That is right for a prototype about
wrapping and wrong for anything shipped.
