# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-575 - `N5`: two notation pieces, one relation, five rules, five tests

**to** sean · **status** open · **raised** 2026-09-27 · **answered** 2026-09-27, `N5` · **asks** approval · **kind** entailed · **shape** an instruction · **into** `spec/data/schema.4x`, `spec/data/rules.4x`, and `spec/tests/`

**You chose `N5`.** Designing stays made of the same rules as playing, `spec/console.md` keeps the
sentence it already has, and **a world a player makes is a history that replays** - which is the
property `N2` would have cost.

## The three pieces, and only one is an invention

```
a constant at the part and command layer   the missing half of a pair the schema
                                           already claims is "deliberately the same
                                           shape" - clause has binding AND literal,
                                           part has argument and nothing
an id minted as the next unused            new. It makes an id unique by construction,
                                           which is P-559's own distinction
a `biome` relation                         a row. P-541 decided it and nothing carried
                                           it into this schema
```

## Two names this lane is choosing, said out loud rather than slipped in

**Neither is settled by anything, and one word from you changes either.**

```
{constant part:P input:i value:6}    the part-layer twin of {literal clause:C column:N value:1}
{minted clause:C column:N}           says an add clause's column takes the next unused id
```

**`constant` mirrors `literal` without colliding with it**, since `literal` is keyed by `clause`
and this is keyed by `part`. **`minted` says what it does rather than how.** If you would rather
they were `value` and `fresh`, or anything else, say so with the promotion.

## The five rules

```
add-territory   a territory and its two places, surface and orbit    minted ids
add-crossing    a crossing from one territory to another             minted id
set-resource    a deposit of one resource, with a density            a constant
set-biome       a territory's biome                                  the new relation
add-to-orbit    a unit in the orbit of a territory                   no phase guard
```

**`add-to-orbit` carries no *before play begins*,** because you put that near the interface and
there is no `phase` in the schema. **Said here so it is not a surprise**: nothing in the engine
stops a unit being added to an orbit mid-game, and the interface is what will.

## How to tell it was carried out

```
spec/data/schema.4x     gains `biome`, `constant` and `minted`, and nothing else
spec/data/rules.4x      gains five rules
spec/tests/             five new tests, one per rule, each stating a world, firing one
                        command, and asserting what it made
the review application  shows all five as unread
the suite               runs the five tests against the five rules and they pass
reviewed/               unchanged - this lane writes tests and never the record
```

## What is not in this

**`create planet <size>`**, which is the console writing these commands and is the code lane's.
**Unblocked by this rather than part of it** - the commands it writes do not exist until these
rules do.

**And the engine work.** `minted` and `constant` are rows this lane can write; **making the engine
honour them is the code lane's**, and it is filed when this lands.
