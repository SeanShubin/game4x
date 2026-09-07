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

### P-344 - What an Ark crosses, now that nothing it does is a move

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07 - **kind** the
cleanup `P-341` promised, reframed by Sean - **asks** a decision - **into**
`releases/first-release.md` -> *Units and structures*

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
