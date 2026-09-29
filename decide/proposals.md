# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-589 - The planet carries the star's density itself, which is two words in two bullets

**to** sean · **status** open · **raised** 2026-09-29 · **answered** 2026-09-29, the fifth shape · **asks** approval · **kind** recovered · **shape** text · **into** `spec/planet.md` -> Shape and `spec/orbit.md` -> The orbital layer

**You chose the shape that needs nothing new**, so what is left is the two bullets that say *deposit*
where the number is now the planet's own. **Each replaces the bullet `P-588` landed four hours ago**;
everything else in them is unchanged, including your reason for the planet over the game.

**Replacing the last bullet of `spec/planet.md` -> Shape:**

> - **A planet is a thing, and its territories are in it.** It is what the sun shines on, so **the
>   planet carries the star's density itself** - one number for the whole world, the way a
>   territory's deposit carries one for a resource - and every orbit above it draws on that one row.
>   **A second planet would have its own**, which is why this is the planet's and not the game's.

**Replacing the last bullet of `spec/orbit.md` -> The orbital layer:**

> - **An orbit has no deposits of its own.** A surface's deposits are the planet's material and an
>   orbit reaches none of them; what an ark in orbit reaches is **the planet's own energy density,
>   which is the star's**. So an ark gathers what an extractor works for, from the world it is above
>   rather than the ground below - and `spec/units.md`'s *a mobile unit that moves in orbit gathers
>   its own energy from the sun* is that density named.

## What changed, word by word

```
was                                     now
the star is a deposit on the planet     the planet carries the star's density itself
the planet's own deposit, the star      the planet's own energy density, which is the star's
gathers by the same rule an             gathers what an extractor works for
  extractor works by
```

**The third is the one that is not cosmetic.** Under your fifth shape `gather` requires a planet and
`work` requires a deposit, so they are no longer the same rule over different ground - **and saying
they were would be false the day it landed.**

## What follows, and it is this lane's

**An `energy` column on `planet`, and `gather` requiring a planet instead of a deposit.** Both are
`spec/data/`, both follow the words rather than lead them, and the first thing to find out is whether
a clause that binds nothing works - **no clause in `rules.4x` does, counted over all of them.**

**The scenario's two orbit deposits then have nothing to be**, which is the code lane's, and the
defect that started this - three above one territory and nothing above the other - goes with them.

