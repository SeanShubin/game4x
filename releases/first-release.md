# Release: First Release

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Releases](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

## Scope

- A single planet
- Tiny, which is 12 territories
- Each territory is self-contained. No resource and no citizen crosses a territory boundary
- A mobile unit may move across a boundary to start another self-contained territory
- The twelve territories and the thirty adjacencies between them are generated from the
  twelve-faced Goldberg polyhedron rather than stated by hand

### Territory resources

**In** - `spec/planet.md`, *for each resource, a territory has capacity for some number
of extractors, and a density that each of them yields*.

The twelve territories are fixed, each chosen to exercise a different consequence of the rules.
Every territory has capacity for at least one food extractor.

| Territory | Food  | Metal | Energy | What it exercises                          |
| --------- | ----- | ----- | ------ | ------------------------------------------ |
| 1         | 3 x 4 | 3 x 4 | 3 x 4  | The landing site. Everything works         |
| 2         | 2 x 6 | 2 x 4 | 2 x 4  | Few dense food extractors                  |
| 3         | 6 x 2 | 2 x 4 | 2 x 4  | Many thin food extractors, same food total |
| 4         | 1 x 2 | 4 x 5 | 4 x 5  | The minimum a territory can be             |
| 5         | 3 x 1 | 8 x 8 | 8 x 8  | Food density 1                             |
| 6         | 4 x 4 | none  | 4 x 5  | No metal                                   |
| 7         | 4 x 4 | 4 x 5 | none   | No energy                                  |
| 8         | 6 x 6 | 1 x 2 | 1 x 2  | Population without industry                |
| 9         | 2 x 3 | 6 x 8 | 1 x 2  | Rich metal, too few hands to work it       |
| 10        | 3 x 3 | 1 x 3 | 6 x 8  | An energy depot                            |
| 11        | 5 x 6 | 5 x 6 | 5 x 6  | The prize                                  |
| 12        | 2 x 2 | 8 x 8 | 8 x 8  | Rich extractors, almost no workers         |

## The loop

1. Start with an ark in orbit
2. Land it, and develop the territory it lands on
3. Reach a second territory, build a Yard there, and launch an ark from it

## Controls

- Rotation is bound to the arrow keys, and to dragging
- Zoom is bound to the wheel, and to pinching
- Reset is bound to `R`, and to a control
- The drawing is bound to `T`, and to a control for each drawing.
- The three surfaces in this release are reached by `F1`, `F2` and `F3`, by buttons on the page,
  and by `/game`, `/console` and `/browser` typed at the console
- Choosing a planet size abandons the current game and starts one on a planet of that size. It
  is bound to `1` through `5`, to a control for each size, and to `/new <size>`

## Capabilities

Each capability is an item in the same shape every outbox uses, so it appears in `pending.md` and in
`tools/outbox` beside what a lens has found. It moves through four states and changes hands once:

- **`open`, addressed `to code`** - not built yet
- **`built`, addressed `to sean`** - the code lane says it is done, and nobody has looked
- **`vetted`** - a person has observed the *vetted when* line and it held
- **`retired`** - the thing it was evidence about is being removed, so nobody will observe it.
  **Only Sean retires one**, and the reason is recorded where the status is. A retired capability
  is not a failed one: the work was done and reported, and what lapsed is the reading


**The code lane does not mark its own.** It reports the evidence and this lane records it, which is
what `docs/process.md` requires in Sean's own words - *so that the account of what has been
delivered is not kept by whoever built it*. It touches neither `built` nor `vetted`. **Five of the six below are vetted by a person looking**,
at a drawing or at a whole game played through, so `built` is where they will wait and Sean is the
only one who can move them.

### R-1 - Two drawings

**to** sean · **status** **vetted** 2026-09-03 · **evidence** both drawings exist and are photographed; ids on the practical one only, poles marked, camera shared

- **In** - `spec/planet.md`, *the planet is drawn either practically or realistically, and the
  user can change which*
- **Vetted when** - switching between them moves nothing: the planet is at the same rotation and
  zoom afterwards, and every territory covers the same pixels

### R-2 - Terrain that crosses boundaries

**to** sean · **status** **vetted** 2026-09-03 · **evidence** one continuous field sampled per point; coastlines cross territory boundaries in the photograph

- **In** - `spec/planet.md`, *the terrain of the realistic drawing is continuous*
- **Vetted when** - no line visible in the realistic drawing coincides with a territory boundary,
  and terrain visibly varies within a single territory

### R-3 - A division that cannot be seen

**to** sean · **status** **vetted** 2026-09-03 · **evidence** no seam and no boundary in the realistic drawing; `Drawn.labels` is zero there

- **In** - `spec/planet.md`, *nothing in the terrain reveals how the sphere was divided*
- **Vetted when** - a person who has not seen the tessellation is shown the realistic drawing and
  cannot mark where a five-neighbour territory is, beyond the two at the poles

### R-4 - A biome per territory

**to** sean · **status** **vetted** 2026-09-03 · **evidence** `biomes_of` gives every territory one, and `join_the_land` keeps land connected

- **In** - `spec/planet.md`, *each territory has a biome*, and *a territory's biome is what the
  terrain gives it*
- **Vetted when** - `show territory 5` names a biome, and no other biome covers more of that
  ground in the realistic drawing
- **Biomes were delivered before they were cut.** `P-522` puts them out of scope for this
  release, and this capability was observed on 2026-09-03 against a drawing that still exists.
  **The cut is about which rules are built and not about what was seen**, so the vetting stands.

### R-5 - Terrain resolved as finely as it is shown

**to** sean · **status** **vetted** 2026-09-03 · **evidence** 400,000 sub-triangles, blended in parameter space

- **In** - `spec/planet.md`, *a drawing never betrays how it was made. A viewer sees the planet,
  never the process*
- **Vetted when** - at the default camera, no facet, band or flat wash betrays how the surface was
  built, and the finest visible detail is terrain

### R-6 - The loop can be played through

**to** sean - **status** **retired** 2026-09-26, unread, `P-562` - Sean: *if these mechanics come back I will read them then* - **status was** **built** 2026-09-11 - **cited** `faafb5f`, `2f38241`, `53bd58d`, `58c8b4a`, `92786a9`, `d7ed1e8`, `d7e6469`, `3292266`, `5e97b79`, `731aceb`, `c3b9e39` - **the last of those is tooling and not evidence**: `spec touching` could not see a built capability and `R-6` was the case - **evidence reported by the code lane and recorded here rather than by the lane that built it**, and **every clause re-run by this lane rather than taken from the report.** `{deploy-ark territory:1}` at `play.4x:19`, `{found-by-land territory:2}` at `:154`, `{launch-ark territory:1}` at `:164`. **The scenario's commands changed by one line on 2026-09-12, and `S-26` says not to do that under you without saying so.** `P-460` gave `move` its second place, so turn 8 reads `{move unit:pioneer from:1 to:2}` in `scenario/commands/play.4x` and the same in `spread.4x`. **`scenario/expected/play.4x` does not change at all**, so what this capability rests on - the loop playing through, and every recipe firing - is untouched and was re-measured rather than assumed. **And its expected state moved again when `P-474` was built, by thirty-four lines and no others.** Every deposit entry carried `room` where it had carried `total-capacity`: `{deposit density:4 resource:food room:0}` in territory 1, where three food extractors stand on ground with capacity for three. **And it moved a third time when `P-475`, `P-476`, `P-477` and `P-478` were built**, again by thirty-four lines and no others. Territory 1's food deposit reads `{deposit capacity:3 density:4 free:0 occupied:3 resource:food}`. **Every clause re-run by this lane rather than taken from the code lane's report**: 34 deposit entries, `capacity - occupied = free` on all 34, `occupied` equal to the extractors standing in that territory on all 34, five deposits with anything on them, and no `room:` anywhere. `cargo test --workspace` is green, where two tests in `crates/game-console/tests/declare.rs` were red on a literal 24 against a count of 26. **Nothing about what the game does moved** - the same 34 deposits by territory, resource and density, and `free` equal to the old `room` everywhere. **Nothing about what the game does moved** - the pioneer that went is the pioneer that went, and the check counted thirty-four deposit lines changed and zero of anything else. **The code lane predicted the shape of that diff before making it** - twenty-nine deposits untouched by any extractor, five not - and the diff agreed, which is why it was willing to reseed the file at all.
`every_recipe_the_release_declares_fires_while_the_scenario_runs` passes, and
`the_committed_scenario_launches_an_ark_and_does_not_finish_the_planet` passes with it. **It does
not win, which is the last clause rather than a shortfall.**

**The fourth clause needed a test that did not exist, and the reason is worth keeping.** Two tests
already covered the halves - ten player recipes and eleven world - and **neither asked whether ten
and eleven are all of them.** A recipe whose `Owner` cell said anything else would have been in
neither population, both would have stayed green, and the clause this capability now turns on would
have been false with nothing saying so. The new test counts the declared set entire, asserts the two
owners partition it, and reads what fired rather than the scenario's text. **Poisoned by dropping
`muster` from what fires: it reports `["muster"]` rather than passing.**

**What it cost, visible in the reseeded data rather than argued.** `P-427` took the two stores out
of founding, so territory 2 has no stores at all, territory 1 has one fewer of each, and ten metal
that used to be kept is lost at a turn's end. `scenario/expected/play.4x` was reseeded under
`P-225`'s protocol and is **unreviewed, and says so**.


**The first clause is observed whole, 2026-09-25** - `c3b9e39`. **What witnesses it is the steps
rather than the final state, and this line said otherwise for a day.** It read *two Arks aloft over
orbits 1 and 2, and none on any surface* - on the premise that `P-549` admits no ark on a surface,
so the orbit an Ark sits in is the territory it launched from. **`cd5b8b2c` added a sixteenth turn
that crosses an Ark from orbit 1 to orbit 2**: both Arks are in orbit 2 now, the position witnesses
one launch where there were two, and **nothing about the game changed.**

**The commands are `{deploy-ark territory:1}` on turn 1, `{launch-ark territory:1}` on turn 9,
`{launch-ark territory:2}` on turn 15, and `{move unit:ark from:1 to:2}` on turn 16** - sixteen
`{end-turn}`s over 219 command lines. **The second launch is from ground taken by land**, which is
what the clause asks, and the Ark the game began with was consumed by `deploy-ark` on turn 1.

**The check behind this clause has read a proxy twice, and reads the step now.** It counted
`{launch-ark` lines in the scenario file, which a scenario launching twice from the same ground
satisfies just as well - *a check whose subject is behaviour reads the outcome, not the input*. Its
replacement read the final position and asserted `[1, 2]`, which held only while no Ark had moved;
turn 16 made it `[2, 2]`, one launch appeared to vanish, and the game was correct throughout. **It
asserts the step now**: a launch above a territory is a step after which that orbit holds one more
Ark **and the game holds one more unit**, so a crossing adds none and a landing consumes one.

**Both corrections came out of re-deriving a claim that had arrived finished, and the second one
arrived saying this line was unaffected.** It was not. **`a line is not a launch` and `where a
thing is now is not where it came from` are the same failure one level apart**, found a day apart,
in the same clause of the same capability.

**What territory 2 had to build, because `P-427` leaves a founding with no stores at all**: a metal
store, a second mine and the first well on turn 11; the second metal store that mine licenses, the
second well and the first energy store on 12; the second energy store on 13; the Yard on 14, for
fifteen of the seventeen metal saved; the launch on 15. **Recorded here and not certified** - the
code lane may build a thing and may not vet it, and neither may this lane.

- **In** - `spec/future/control.md`, *a player wins by launching an Ark from a territory other
  than one an Ark has been deployed to* - **a future plan since `P-556`**, and the requirement
  below is what this release is checked against instead
- **Vetted when** - the scenario takes a first territory from orbit, takes a second by land, and
  **launches an Ark from the second**; and every recipe in the release fires at least once while
  it runs, measured by what fired rather than by what the file says. **This is the requirement
  the first release is checked against and not a victory** - nothing ends, and the interface lets
  the player keep going.

- **Nothing in the code blocks it, as of 2026-09-05.** Earlier doubts were settled: a
  territory's stores carry, and the output a territory can reach is decidable from the
  territory alone. **What is now in question is not whether it can be played but how much of
  it has to be.**
- **Measured 2026-09-11.** Running `setup.4x`, `{start}` and `play.4x` and asking the model
  gives **twelve claimable territories, two founded, none at maximum output**. It launches an
  Ark at line 164. **These numbers were taken against an older *vetted when*** that asked for a
  fully exploited planet, which the release no longer requires.
- **The gap is not a near miss**, which is the part a summary loses. `tests/fully_exploited.rs:410`
  derives **57 buildings**, which is **114 commands and counts nothing else** - each building is the
  labor that pays for it and the building, read off the predicate at `:404`. **It is a floor**, and
  three things sit on top of it, each read from the release rather than recalled.
- **Ten territories to found, and each wants a founding unit rather than a pioneer.** `found by
  land` consumes 1 pioneer and `deploy ark` consumes 1 ark; `play.4x` uses one of each, territory 1
  by ark and territory 2 by pioneer. **Which one is the player's choice and the release leaves it
  open.**
- **A founding unit costs citizens, and that is the cost that matters.** `produce pioneer` consumes
  **3 metal, 2 energy and 2 citizens**; `launch ark` consumes 3 metal, 12 energy and 2 citizens and
  requires a Yard. **So founding competes with the population rather than costing resources beside
  it** - ten foundings is twenty citizens spent against a target of **144**, summed from the
  per-territory figures `DERIVED_BY_HAND` states. Read as metal and energy alone it looks like a
  cost paid out of production; it is paid out of the goal.
- **Distance costs turns, not only energy.** `moving` is **0 or 1** and `move` requires *moving at
  least 1*, puts the unit back with one less, and consumes 1 energy. **Only `refresh` restores it,
  at a turn's end** - so a unit moves once per turn, and a territory *n* steps from a founded one is
  *n* turns away. `play.4x` spends **10** `{end-turn}`s reaching two founded territories.
- **No total is estimated anywhere here**, against a scenario that is 133 commands and has founded
  two of twelve.

 **Re-run by this lane rather than taken from
  the report**: `the_committed_scenario_launches_an_ark_and_does_not_finish_the_planet` passes on
  `(12, 2, 0)`.
- **One thing this proved that nothing had asserted.** The scenario reaches a second
  settlement and launches anyway from the first, so a launch is not a victory by itself - and
  that is now a check rather than an observation.



### R-7 - Each recipe can be confirmed on its own

**to** sean - **status** **retired** 2026-09-26, unread, `P-562` - Sean: *if these mechanics come back I will read them then* - **status was** **built** 2026-09-08, **and its report moved under you on 2026-09-11** - **cited** `747de8a`, `025eecb`, `fe3dc9b`, `3bab70e`, `2e9a06e`, `dd93bd1`, `ca2309e`, `e42d37c` - **evidence reported by the code lane and recorded here rather than by the lane that built it.** `reports/recipes.md` shows every recipe with a state before, the command that fires it and the state after, in the scenario's notation and generated by running it; `tests/worked.rs` fails if a recipe has no example. **Verified in the report by this lane rather than taken from the report: 29 sections and 21 distinct names at `475127a8`**, where the report states of itself *29 recipes, 73 lines between them, 13 worked examples*. **This line said 24 until 2026-09-25, and the file gives that figure at no commit anybody has found** - 16 sections when the capability was built, 31 when the line was last edited. Found by the code lane re-deriving it before acting, `C-145`; the count is pinned to a commit now because it moves. **What you read on 2026-09-08 is not what is there now**, so the earlier reading does not carry. The saturating rewrite reached the model: `grow` is gone, so the two examples that showed its expression turning out both ways are now `breed`'s - food the lesser, and citizens the lesser. **The world's recipes are still shown once together on one `{end-turn}`, and that one ending now fires ten rather than six.** Your *vetted when* says *four of them cannot act alone at all*, which was written when there were six; **that number wants your eye while you re-read**, and it is yours to change or leave. `S-88`, `C-84` **One line of the report moved on 2026-09-12 and it is the line that made it ambiguous.** `P-460` made a command bind every place a recipe leaves open, so the scenario's move reads `{move unit:pioneer from:1 to:2}` where it read `{move unit:pioneer territory:2}`. **Every state before and after is byte-identical** - the pioneer that went is the one that always went, and the command now says which. **One of the three things this capability names is the command**, so it moved to be more correct rather than merely changing, and only one of the recipes is affected. This lane first reported `R-7` untouched by checking the Recipes table, **which is not what the capability names**; the code lane measured the report and handed back the diff. `C-101`

- **In** - `docs/process.md`, *the definitions and the commands are enough to derive the data dump
  by hand*, applied to one recipe rather than to a whole scenario
- **Vetted when** - `reports/recipes.md` shows, beside each recipe's rule, a state before it fires,
  the command that fires it, and the state after - **in the same notation as the scenario's
  expected data**, holding only what that recipe touches, and generated by running it. I can derive
  the after from the rule and the before by hand, and a recipe whose quantity is an expression
  shows one example for each way the expression turns out. **The world's recipes are shown once,
  together, on `{end-turn}`**, because no command fires one of them alone and four of them cannot
  act alone at all


### R-8 - I can see which kinds behave alike

**to** sean - **status** **retired** 2026-09-26, unread, `P-562` - Sean: *if these mechanics come back I will read them then* - **status was** **built** 2026-09-07 - **cited** `d938c8a`, `79d8f1d`, `14b02d2`, `dd93bd1`, `d617b9a`, `08a77e3`, `d5f565a`, `883901a`, `788bf59` - **the last of those raised the count below rather than building anything** - **evidence** a signature per kind computed from the tables, reported by the code lane. **Ready to vet again, 2026-09-08 - `14b02d2`.** `C-71` is fixed: a trait declared of a family now reaches its members, so `fuel` reaches ark and pioneer and `keeps` reaches all sixteen. **It closed a second defect in the half I had called correct** - `thing` is written *every kind above*, a membership rather than a list, so both joins split on commas and missed it, and the world's five recipes named nothing at all. **The conclusion is unchanged and now computed from the right inputs**: no two of the sixteen behave alike, over 120 pairs. `S-78`

**Its report moved under you again on 2026-09-12, and three of its numbers were already wrong.** `P-466` took three columns out of *Units and structures* - `Costs to produce`, `Binding` and `Requires`, each the Recipes table said twice - `P-465` changed eight `Values` cells and two `Movable` cells, and `P-470` gave every kind its trait names in `spec/data/kinds.4x`. **A signature is computed from those tables, so every signature is computed from different inputs than the ones you would have read.**

**And three numbers in this line were stale before today.** The release declares **eighteen** kinds, not sixteen, and eighteen make **153** pairs, not 120 - `fertility` and `force` arrived after 2026-09-08. **The world has eleven recipes, not five**: `P-414` added `muster`, `stand` and `discard`, and the saturating rewrite added the rest. **All three were true when written and have read the same ever since**, which is the failure `docs/notes/nothing-removes.md` is about, found in the item that note was written beside.

**Three stale numbers in one status line is a pattern rather than an accident.** A capability's status is prose about a moment, and a number in it is a second form of a fact the tables hold - `spec/invariants.md` -> *A fact is stated once*. **Nothing re-derives these**, and the check that covers what `docs/` says about the release does not reach what the release says about itself.

**The conclusion survives and was re-derived rather than carried over**: the traits each kind carries, joined to every *(recipe, role)* pair naming it, give **eighteen distinct signatures over eighteen kinds** - so no two behave alike, over 153 pairs. **Its evidence moved three times on 2026-09-12 and then settled.** `P-465`, `P-466` and `P-470` changed the tables a signature was computed from; `P-473` changed it to compute from `spec/data/` instead. **Between the first and the last, `reports/catalog.md` understated seven of the eighteen kinds** - an Ark showed four traits and carries eight - because four cells of the deleted *Of* column described rather than named. **This says what happened rather than what is**: a present tense about another lane's generated file goes stale the moment they regenerate it, which this line did twice in one day.

**Re-derive rather than trust this line: every kind's *Traits* line in `reports/catalog.md` should hold what `spec/data/kinds.4x` states for that kind, plus every trait declared `of:thing`.** When last run it held for all eighteen, an Ark showed eight, and the page said **seven** kinds collide on traits alone - **which is the number the code lane derived by hand from `kinds.4x` before the code produced it from the files.** A prediction made from one source meeting an implementation built from another is the strongest evidence anything got that day.

**And the count moved a third time, back to where it started, which is why this line now states
none.** `P-522` cut force, garrison and nature from the release on 2026-09-21 and the Kinds table
went from nineteen rows to sixteen - so *eighteen kinds and 153 pairs* above was true when written
and is not now, and the *sixteen* it corrected is true again. **The live figure is generated and
belongs there**: `reports/catalog.md` opens with *16 kinds, 4 families, 23 traits, 29 recipes* and
*no two of the 16 kinds behave alike, over all 120 pairs of them*, computed from `spec/data/`
rather than written by anyone. **Read the count there. This line asserts only the conclusion**,
which has survived every one of the three movements. `C-130` is withdrawn by that promotion rather
than answered - it counted nineteen on 2026-09-15 and was right, six days before the cut - and
**nobody said so at the time**, which is the half of the promotion rule this lane missed.

- **In** - `docs/process.md`, *I insist that the AI make its work verifiable to a human*, applied
  to a kind's behaviour rather than to a scenario's outcome
- **Vetted when** - `reports/catalog.md` gives each kind a **signature**: the traits it carries and
  every *(recipe, role)* pair that names it. **Kinds with the same signature are shown together**,
  and the signature is computed from `spec/data/` rather than written by anyone. I can scan
  the groups, see that two kinds behave alike, and have a name to grep for when I want the detail

### R-9 - I can browse the reports without a script running

**to** code · **status** open · **reopened** 2026-09-28 · **status was** **built** 2026-09-07 · **cited** `dc6d341` · **what was observed then, and no longer exists**: every reference a link, a diffable sibling for every view, no page carrying a script, two shared stylesheets, and a page plus a sibling for each of the twelve territories. `S-64` built with it. **`e40325c2` deleted all of it under `D-4`**, because those pages were generated from the old ruleset's scenario - `C-164`. **Sean, 2026-09-28, on what it must show again**: *strike the twelve territory pages from R-9*, so twelve was a fact about the first release's world rather than about being able to browse. **The three clauses below are unchanged and are what it is vetted by.**

- **In** - `docs/process.md`, *presentations are generated from data*, and *I insist that the AI
  make its work verifiable to a human*
- **Vetted when** - every reference in a report is a link I can follow to the thing it names;
  every generated view has a **diffable sibling** beside it, as `graph.html` has `graph.txt`; and
  **no page needs JavaScript to be read** - a view that filters is a page that was generated, so
  the filter is a URL rather than a click

### R-10 - I can read a generated drawing in the theme I use

**to** code · **status** open · **reopened** 2026-09-28 · **status was** **built** 2026-09-12 · **cited** `7b4761f`, `8fd18d9` · **`a8386450` deleted `reports/petri.*` under `D-4`**, so nothing it names exists - `C-164`. **Sean, 2026-09-28**: *I want the petri reports back, it gives me confidence that we would immediately detect an infinite resource glitch.* **The detection is `S-219` and is not this clause** - a drawing legible in both themes is a different property from the net being sound, and the sound half went with the same commit. **What was observed in September**: **evidence reported by the code lane and recorded here rather than by the lane that built it.** All three clauses hold. **Colour**: every label in `reports/petri.html` declares a fill, counted at 295 of 295. **Names**: every node in the net carries its own. **Parts**: `reports/petri.md` has had one drawing per recipe since it had the whole net - **what was missing was the rest of the clause**, *and it says what each part leaves out*, so a reader of one recipe met a drawing that looked like the whole of that recipe's connections.

**A part now names the recipes that reach the same places**, rather than *everything else*, which is
true and tells a reader nothing - `create labor` leaves out 30 others reaching `citizen` and `labor`,
and names them. **Computed from the arcs**, so a recipe added tomorrow appears in the parts it
touches with nobody maintaining a list. Checked over every part with the count, and **driven both
ways on one case so a pasted list fails where a computed one passes** - `C-99`.

**And the numbers in this line were stale.** It said 62 nodes and 195 arcs; the net is **31 places
and 45 transitions, 76 nodes, joined by 161 arcs** - `P-411`, `P-414`, `P-427` and `P-431` each moved
it and nothing re-counted. **The clause was never about the figure**, which is why the staleness cost
nothing here: it asks whether a reader can read it.


- **In** - `docs/process.md`, *I reject AI responses that do not read clearly and unambiguously to
  a human*, applied to a drawing rather than to prose
- **Vetted when** - every generated drawing is legible in **both** a light and a dark reader,
  because nothing in it declares a colour the theme does not supply. **Every node carries its own
  name**, and I can say what a node is without looking anything up. Where a drawing is too large
  to satisfy that whole, it is shown in parts that do, and it says what each part leaves out

### R-11 - I can reach the engine's inputs from the reports

**to** code · **status** open · **reopened** 2026-09-28 · **status was** **built** 2026-09-14 · **cited** `287e67f`, `3b210d8` · **`e40325c2` deleted the index its clause names**, and `09f628d7` built a new one that renders every `.4x` as a page rather than linking a `.txt` twin - `C-164`. **Sean, 2026-09-28**: *keep R-11 reworded*, so the intent stands and the mechanism in the clause does not. **The new wording is `P-586` and is his to read.** **What was observed in September**: **evidence reported by the code lane and recorded here rather than by the lane that built it**, and **the counts re-derived by this lane rather than taken from the report**. **Nineteen links over three directories** - eleven in `spec/data/`, seven in `scenario/commands/` and one in `scenario/expected/` - **twenty and twelve until `P-557` deleted `spec/data/above.4x` on 2026-09-25** - each to a `.txt` twin the pipeline writes into `crates/game4x/dist` and nothing commits, labelled with the `.4x` path the engine actually reads. **The rendering half is measured rather than inferred**, which this lane twice said it could not claim: `spec/data/kinds.4x` and `scenario/expected/play.4x` both answered `200 application/octet-stream`, and `reports/index.html` `text/html`. So GitHub Pages was the cause and the twin is the fix. **And building it found the capability unmet a second way, which the code lane reports against itself.** The inputs section listed `spec/data/` by reading the directory and listed the scenario as a hand-written pair, naming `play.4x` twice - so `setup.4x`, `world.4x`, `biomes.4x`, `nodes.4x`, `forces.4x` and `spread.4x` were reachable from nothing at all. **Half were listed and half were remembered, in the commit that claimed the clause about listing rather than remembering.** Re-derived here: `scenario/commands/` holds seven files and all seven are linked. **It cannot be observed until it is pushed, and that is not a defect.** The twins are made at deploy and nothing commits them, so **nothing is published to follow** - a 404 today reads *not pushed* and never *not built*. **The code lane says plainly that it has not measured a `.txt` rendering on this site**, because none exists there yet; what it measured is that Pages types by extension and serves what it knows - `reports/report.css` as `text/css`, `reports/nogain.md` as `text/markdown`. **`.txt` to `text/plain` is an inference with two neighbours as evidence**, and one fetch after a push settles it.
to need the inputs to the thin engine to be reachable from `reports/index.html` (direct links or
non-canonical generated copies).*

**This invents no rule.** `spec/invariants.md` -> *The game is data* already says the data that runs
the game lives in a data file, and that **the data may be replicated in the presentation layer, and
no replication is canonical**. `R-9` already requires every reference in a report to be a link a
reader can follow. This capability is those three sentences applied to `spec/data/`, which today no
report reaches at all.

**Why it is worth a capability of its own, and it is the thin engine's reason rather than a
convenience.** `P-493` makes complexity in the data a **reading** - *if the data structure explodes
in complexity, or the data itself explodes in complexity, that tells us something needs to be
unified*. A reading nobody can see is not a reading. The inputs are 58 lines today and the instrument
only becomes useful as they grow, so the time to make them visible is before they do.

**Either form satisfies it and the choice is the code lane's**, because it is a presentation
question. A direct link to `spec/data/kinds.4x` costs nothing and shows the canonical bytes; a
generated copy reads better and must then say it is generated and not canonical, as every other
generated view does.

- **In** - `spec/invariants.md`, *The data may be replicated in the presentation layer, and no
  replication is canonical.*
- **Vetted when** - from the reports' index I can reach **every file the engine reads as input**,
  in as many clicks as it takes to reach any other view, without knowing the paths beforehand, and
  **reading it is what following the link does** - it renders in the browser rather than
  downloading. A rendering rather than the file itself **says on the page that it is generated and
  not canonical**, and says which file it came from. **Nothing the engine reads is missing from
  that index**, which is checked by listing the inputs rather than by anybody remembering to add
  one.

  **The rendering clause is Sean's, 2026-09-14**, on finding that
  `seanshubin.github.io/game4x/spec/data/above.4x` downloads: *I want to be able to view these
  pages from the website without downloading them.* **It was always the intent and was not
  written**, which is why the capability could be built and still not deliver it. **It is about the
  published site and not the repository view** - *I don't necessarily need it to be rendered when I
  browse it as source*.


### R-12 - I can read the foundation form of a test without leaving the reports

**to** sean · **status** **built** 2026-09-25 · **from** `P-558` · **cited** `211652e`, `694424c`, `f947d15` · **evidence reported by the code lane and recorded here rather than by the lane that built it**, and **every clause re-derived by this lane rather than taken from the report.**

**`reports/foundation/` holds 54 files and 985 rows**, and its names are identical to `reviewed/`'s 54 - diffed, not counted, so a file present under one name and absent under another could not pass. **Generated from `reviewed/` and never from `spec/tests/`**: `examples/foundation.rs` reads `reviewed/` at line 55 and says at line 60 of the other directory *nothing is generated from here*.

**The unread are named and counted and are never an error**, which is the clause `C-141` corrected: line 278 reports them by name with a count, under a floor asserted **before anything is written** - *a run over almost nothing would write almost nothing and look finished*.

**The reversal is `what_the_engine_runs_is_what_the_record_generates`** in `crates/thin-engine/tests/generated.rs`. The check it replaces converts `spec/tests/` and asserts the committed foundation equals it - **the working copy is the expectation there, and a test Sean has not read is in it.** This one asserts the other way.

**And `render.rs`'s header had named the wrong source since 2026-09-21** - *`spec/tests/*.4x` read, and `data/foundation/tests/*.4x` written from it* - which is the plan written in the code being stale in exactly the way rule 3 exists to prevent. Corrected while building rather than filed.

- **In** - `spec/README.md`, rule 3: *a test is stated in the friendly form, and the foundation
  form is a rendering of it. The rendering is generated from `reviewed/` and never from
  `spec/tests/`, so that what the engine runs is derived from what has been read rather than
  compared with it*
- **Vetted when** - `reports/` holds the foundation form of **every test in `reviewed/`**,
  generated rather than written, so I can open one and read the numeric rows of a test I am
  debugging; **no test I have not reviewed is among them**; and **the generator says how many are
  unread and names them**, so that a test waiting on me is reported rather than merely absent

## The third part is a correction, filed by the code lane as `C-141`

**`R-12`'s observable said *absent* and absence is not loud.** The code lane read `S-149`'s
reasoning and asked whether an unreviewed test should stop the gate or be quietly not-yet-running,
and said it would build the refusing version unless told otherwise.

**Neither, and the answer was already built.** `crates/thin-engine/tests/common/mod.rs`,
`every_read_test`: **A test with no record is left out rather than failed**, which is the half of
the rule that is easy to get backwards. **Drafting a test is not an error**; it is a thing that
constrains nothing until he has read it - so this returns fewer files and **the runner says how
many and which.** And `first_test.rs` prints *N of M tests have not been read and did not run*
with their names, under a floor - `reading.len() > 40`, because **a count over nothing is the same
failure with the sign flipped.**

**So the generator mirrors that contract rather than inventing one.** Generate for the read tests,
name and count the unread, and keep a floor so a missing `reviewed/` cannot make the whole thing
vacuous. **Refusing would make drafting a test an error**, which is the one thing that doc says not
to do - and drafting is what an assistant does on Sean's direction, so it would put the gate red
every time a test is written and before he has had a chance to read it.

## Three things, and the third is the one that reverses the direction

**Generate the foundation form into `reports/` from `reviewed/`.** Sean, 2026-09-25: *foundation
lives in reports*. It is a generated file, so nobody edits it and padding it changes nothing.

**Move `crates/thin-engine/tests/common/friendly.rs` into production support.** Sean, 2026-09-25:
*go with your recommendation, move it.* **`examples/report.rs` already reaches it with
`#[path = "../tests/common/friendly.rs"]`**, and `examples/review-web.rs` - the review application
- depends on it, so it is production support in fact already. **844 lines, 561 of them code, three
imports and none of them test infrastructure**, so nothing travels with it.

**Reverse the comparison in `friendly.rs`.** It renders friendly from foundation and compares;
under rule 3 it asserts the generated foundation against the read friendly instead.
**`every_file_survives_the_round_trip` already asserts the identity in both directions**, so the
reversal is which side is the expectation rather than new machinery.

**What must not change**: Sean, 2026-09-15, *the engine should only know about the foundational
format*. The translator sits outside the engine before and after this.

## Out of scope

Whole areas of the specification this release does not touch, so the omission reads as deliberate.
**`spec/` keeps all four**; this is scheduling and not a change to the game.

Sean, 2026-09-20: *I am cutting nature and force from this prototype because I don't think they are
necessary to vet the core game loop: start with an ark -> develop a planet -> launch an ark.*

- **Nature** - `spec/future/force.md` -> Force, *every territory has a force of nature, inherent to it*.
  Nothing resists a claim in this release, so `reclaim`, `renew` and `take` are not built
- **Force** - `spec/future/force.md` -> Producing force, and *gaining and holding ground*. No garrison,
  no muster, and `deploy ark` and `found by land` neither require force nor produce a garrison.
  **`spec/turn.md`'s *nature takes back what is no longer held* goes with it**, so a turn in this
  release has four steps rather than five
- **Biomes** - `spec/planet.md`, *each territory has a biome*. A territory's numbers are stated
  directly in *Territory resources* rather than guided by a biome
- **The rule editor** - `spec/interface.md` -> Surfaces. It is for automating the game to the
  player's preference, and this release has nothing to automate

## Open questions
