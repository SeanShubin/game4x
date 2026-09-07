# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-345 - `spec/orbit.md` says launching is a move, and `P-341` says it is a Yard's act

**to** sean - **status** open - **raised** 2026-09-07 - **kind** contradiction I created by
promoting - **asks** a decision - **into** `spec/orbit.md` -> Crossing between layers, or
`spec/structures.md` -> Yard

**I promoted `P-341` and `P-342` into a contradiction, and this is the report.** `spec/orbit.md`,
which I never opened, says:

- *An orbit is next to the territory below it, and next to the orbits above that territory's
  neighbours*
- *Launching, landing and crossing from one orbit to another are **moves**, and cost what any other
  move costs*

**Against `spec/structures.md`, as of `6c6f910`**: *Building an Ark and launching it are one act. An
Ark a Yard builds never stands on the planet.*

**They cannot both hold.** If launching is a move it carries a thing from a territory to the orbit
above it, so the Ark has to be standing there first - which is exactly what one act says never
happens.

**How I missed it.** I grepped `spec/` for `launch` before promoting. The file says **`Launching`**,
capitalised at the start of a sentence, and the search was case-sensitive. **A check that answers a
narrower question than the one asked, and returns a plausible zero rather than an error** - which is
the failure this repository already has three recorded cases of, and now a fourth.

**Three ways, and your last message is the third.**

1. **`spec/orbit.md`'s sentence loses launching.** *Landing and crossing from one orbit to another
   are moves.* Launching stays what you chose - the Yard's act, consuming the cost and putting
   nothing into orbit. Nothing else in the tree changes
2. **`P-341` and `P-342` come back out.** Launching is a move, a built Ark stands on the ground and
   flies up, and the release returns to `produce ark` plus a separate launch
3. **`move` stops being a category.** *I am not sure if move has to be special at all. It could
   simply be a recipe with certain constraints and certain parameters that need to be chosen.*

**The third explains why this contradiction existed**, which the other two only repair. `spec/orbit.md`
says three things *are moves*, and that sentence is a **taxonomy**: it asserts that launching belongs
to a class, and the class is what made your Yard's act look like a violation. **If `move` is one
recipe among sixteen rather than a kind of thing a recipe can be**, then the sentence has nothing to
say about launching, and there was never a rule to break.

**What the sentence was actually carrying** is worth keeping under any of the three: **what is next to
what**, which the release already derives, and **that these traversals cost fuel**. Neither needs the
word *moves*.

**What the third costs** is the `Crosses`, `Fuel` and `A move` columns, which exist to parameterise
exactly one recipe. Folding them into that recipe's own rows is a real change to the release's tables
and to the code that reads them - **more than this contradiction needs**, which is why it is a
direction rather than the repair. If you want it, it is its own proposal and this one takes 1 in the
meantime.

**It settles `P-344` either way, which is why that one is now held under this.** If crossing from one
orbit to another is a move, then an Ark **does** fire `move`, and `Fuel`, `A move` and `Crosses` are
all live - so **my recommendation there was wrong** and the empty cell is off the table. What remains
is only whether `ascent` survives, and under 1 it does not while under 2 it does.

**Two things next to this that I am not folding in**, because each is a separate answer:

- **Is landing a move?** `spec/orbit.md` says yes; the release's `deploy ark` is one recipe that
  consumes an Ark in orbit and produces ground things, which is not a move. Same shape as this
- **May an Ark land and stand there?** If landing is a move, an Ark can arrive on the ground without
  being taken apart, and `spec/unit-types.md` now says it is taken apart *when it deploys*

**Your constraint holds under both**, and is already built: *the ark can only deploy to the territory
it is in orbit above* is `deploy ark`'s own Where cell, *the orbit above `$where`*.

**And your other correction stands.** *A recipe makes a thing cease to be here and come to be there*
was too wide - `create labor` and `work` consume and produce in one place, and `launch ark` produces
nothing anywhere. **I withdraw the offer to write it**; what is true is narrower and about `move`
rather than about recipes, and nothing yet needs it written.

### P-344 - What an Ark crosses, now that nothing it does is a move

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07 - **kind** the
cleanup `P-341` promised, reframed by Sean - **asks** a decision - **into**
`releases/first-release.md` -> *Units and structures*

**Held under `P-345`, 2026-09-07.** `spec/orbit.md` says crossing from one orbit to another is a move, so an Ark does fire `move` and `Fuel`, `A move` and `Crosses` are all live - **the empty cell recommended below is wrong.** What an Ark crosses cannot be settled until the contradiction is.

**Your correction, and it inverts what I filed.** *Move is just a recipe that happens to have a
certain symmetry. Launching into orbit doesn't have that symmetry but it is still a recipe.*

**I asked whether deploying is a move, and that was the wrong question.** A recipe makes a thing
cease to be in one place and come to be in another; `move` is only the case where those are the two
ends of one edge. **Nothing needed deploying to be a move**, and I treated the category as though it
governed something.

**What it actually governs is one row.** `Crosses` is read **once** in the whole release - `move`'s
second requirement, *joined to `$from` by an edge the unit crosses*. Nothing else consults the
column. So `deploy ark` never reads it: it names *the orbit above `$where`* in its own Where cell and
needs no permission from a table.

**So the question is narrow: across what does an Ark ever fire `move`?**

- **`ascent`** - nothing ascends now. A Yard launches, and launching is a recipe rather than a
  journey. **Dead**
- **`orbit border`** - an Ark goes from orbit to ground by **deploying**, which never reads this
  column. **Dead for the same reason**, which is the part your correction makes visible
- **orbit to orbit** - the one movement you have described an Ark making, *move about in orbit,
  choose a landing zone*. **The table has no value for it**

**Two ways, and I recommend the first.**

**Empty, and `Fuel` and `A move` go with it.** An Ark never fires `move`. Choosing a landing zone is
choosing `$where` in `deploy ark`, which can already name the orbit above any territory - so **the
flying is already expressed** and a second mechanism would say it twice. Three cells go: `Fuel` 2,
`A move` 1 fuel, `Crosses`.

**Or a value naming an orbit-to-orbit edge**, so an Ark must fly to the orbit above its landing zone
and pay fuel to do it. That is a real constraint on where a first landing can be, and **it is a game
rule rather than a tidy-up** - which is why it is yours and not mine.

**One thing worth writing down if you want it.** *A recipe makes a thing cease to be here and come to
be there; `move` is the case where those are two ends of one edge* is a fact about the language that
this decision leans on, and `spec/console.md` does not say it. **A rule leaning on it is the test**
for whether it belongs in the specification, and this is one - so say the word and it becomes a
proposal.
