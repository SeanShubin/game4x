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

### P-322 - The round-trip check cannot hold, because two traits have nowhere to be written

**to** sean - **status** open - **raised** 2026-09-06 - **kind** contradiction - **asks** a decision
- **into** `releases/first-release.md` -> Where things are, or `spec/console.md` -> The language

**Filed the moment it was found.** The code lane's `C-46`, point three, and it collides with words
you promoted an hour ago.

**`P-320`, in the release:** *The check is that the dump reads back into the state it came from.*

**`spec/console.md`:** a description is *a kind and every stored trait that thing has* - **a flat
map from a trait name to one value.**

**A territory has three densities and several total capacities.** `density` is stored **per
resource** and `total capacity` **per kind**, so a territory carries several of each. **A flat map
cannot hold a repeated trait**, and no rule says how one is written. The code lane assumed neither
goes in the data file and said so rather than inventing a rule.

**So the round trip is text against tree, not text against the game.** Reading the file back rebuilds
what things contain and **cannot rebuild a territory's numbers, because they are not in it.** The
test says which half is proved rather than claiming the whole, which is the honest thing to have
done and is why this is a decision rather than a defect.

**Three ways, and they differ in what they make the data file be.**

- **A repeated trait gets a form**, so a description may carry `density` three times. **This changes
  what a description is** - it stops being a flat map - and everything that reads one follows
- **A deposit is a thing**, so a territory contains `{deposit resource:food density:4} -> 1` the way
  it contains anything else. **Nothing changes about descriptions**, the round trip closes, and
  `Territory.deposits` is already a `BTreeMap` in the model, so the code is closer to this than to
  what it writes. **The cost is a new kind**, and the *Kinds* table declares no `deposit`
- **The round trip is narrowed on purpose**, and the release says the file states what things
  contain rather than all of the state. **Cheapest, and it gives up the property `P-320` just
  landed** - which is the one that made the check worth having

**My recommendation is the second**, and I will say why rather than only that. It needs no change to
what a description is, it closes the round trip rather than narrowing it, and **the model already
holds deposits as a map keyed by resource** - so it is the shape the code arrived at independently.
What it costs is one row in the *Kinds* table, which is a smaller change than the other two.

**`total capacity` needs no decision either way.** `spec/logistics.md` makes it a fact about
containment keyed by kind, so it is computed from what a thing holds and shown as `used/total` -
never written as a trait. Only `density` is homeless.
