# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-573 - the ark cannot move, and the question is what a rule does with a column it does not name

**to** sean · **status** open · **raised** 2026-09-27 · **asks** a decision · **kind** entailed · **into** the notation, and `spec/logistics.md` or `spec/invariants.md`

**Your next edit is refused by the engine.** `{move what:ark from:place-2 to:place-4}` fails with
`` `move`.`4` binds nothing to `gathering` ``.

**Why, measured from `spec/data/` rather than from the failure.** `move` operates on the family
`unit`, whose four members do not have the same columns:

```
scout      where moving quantity
transport  where moving quantity
pioneer    where moving quantity
ark        where moving quantity  gathering
```

**`move` binds `where` and `moving` and says nothing about `gathering`**, so the clause that writes
the moved thing cannot build an ark's row. **Three of four members move; the fourth is refused.**

## Two of the code lane's three answers are already closed by `spec/units.md`

**It offered three and said `M3` - an ark genuinely cannot move - may well be right. It is not**,
and neither is `M2`, that an ark is not a `unit` for moving:

> A mobile unit contributes room for fuel to the place it is in, and **moving spends a unit of
> energy from the place it leaves**... **A mobile unit that moves in orbit gathers its own energy
> from the sun.**

**That sentence is about a unit that moves in orbit and gathers, which is an ark and nothing else.**
`gathering` exists *because* an ark moves. So the refusal is a defect and not a rule.

## What is actually open, and it is about the notation rather than the ark

```
M1  a rule carries through the columns it does not name, unchanged - an ark that
    moves keeps whatever it was gathering, and no rule ever lists a column it does
    not care about
M4  a rule binds every column of every member it applies to, and `move` gains a
    clause for `gathering` - omission stays an error, and the error arrives when
    the rule is written rather than when a player fires it
```

**`M1` makes rules short and makes a new column safe**: adding one to a kind never breaks a rule
that ignores it. **`M4` makes rules complete and makes a new column loud**: adding one breaks every
rule over that family until somebody says what happens to it.

**This is the same choice as `P-568`'s**, one level up - whether silence means *nothing happens* or
means *nobody has said*. You chose *nobody has said* for capacity this morning.

## The class the code lane named, which outlives whichever you pick

**A rule written for a family holds for every member or it does not hold.** `P-373` makes a rule
whose subject is a family a rule for each member, **and nothing checks that each member can satisfy
it.** This is the first instance and a person asking for a command found it, not a check.

**The check exists either way and the code lane has not built it**: for every rule over a family,
every member's columns are accounted for by some clause. **Under `M4` it is the rule; under `M1` it
is unnecessary** - which is why it waits on you.
