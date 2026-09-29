# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-588 - A planet is a thing, the sun is a deposit on it, and the containment root comes back

**to** sean · **status** open · **raised** 2026-09-28 · **answered** 2026-09-28, planet · **asks** approval · **kind** recovered · **shape** text · **into** `spec/planet.md` -> Shape, `spec/orbit.md` -> The orbital layer, and `spec/logistics.md` -> Containment

**Three blocks, and the third is the sentence that was lost rather than a new idea.** Each says
where it goes. **No rule and no column changes**: `gather` requires a deposit at the ark's place
today and would require one at the planet that place is under, which is a `where` naming a different
thing rather than a different clause.

**Into `spec/planet.md` -> Shape, as a bullet at the end:**

> - **A planet is a thing, and its territories are in it.** It is what the sun shines on, so a
>   planet's deposits are the ones no territory owns - **the star is a deposit on the planet**, with
>   a density like any other, and every orbit above it draws on that one row. **A second planet
>   would have its own**, which is why this is the planet's and not the game's.

**Into `spec/orbit.md` -> The orbital layer, as a bullet at the end:**

> - **An orbit has no deposits of its own.** A surface's deposits are the planet's material and an
>   orbit reaches none of them; what an ark in orbit reaches is **the planet's own deposit, the
>   star**. So it gathers by the same rule an extractor works by, from a deposit above it rather
>   than below, and `spec/units.md`'s *a mobile unit that moves in orbit gathers its own energy from
>   the sun* is that deposit named.

**Into `spec/logistics.md` -> Containment, as the first bullet, before *a thing may contain
things*:**

> - **Every thing but the game is in another thing.** The game is the one thing nothing holds, and
>   everything else has exactly one holder - which is what makes *in at most one other thing* below
>   a statement about the game's shape rather than about a particular thing

## Why the third block is here

**It is yours and it was deleted by this lane.** `releases/first-release.md` -> Where things are said
it, `c7bcd95c` removed that section with the old ruleset's tables on your word, and **`spec/` has
never held it** - measured at zero. It is the rule `rooming`, `stands-in` and every capacity check
already keep, and the bullet it now sits above assumes it.

**Say so and it comes out**, and the first two stand without it.

## What this leaves for the code lane, and it is not in the offered text

**A `planet` relation, a row for the planet, and the sun's deposit naming it** - plus
`scenario/main.4x`, which has two orbits and one sun today, so the same star gives three above one
territory and nothing above the other. **That defect goes when the sun is stated once.**

