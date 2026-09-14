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

### P-499 - `P-497` refused: two columns it treats as atomic are not, and the counts passed anyway

**to** sean · **status** open · **raised** 2026-09-13 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `P-497`, before it is promoted

**You said promote `P-497` and this lane refused it rather than landing a mangled migration.**
`CLAUDE.md` gives two ways out of a promotion that breaks something, and this is the first: say what
has to be decided first.

**It was found by carrying the migration out.** All seven files were written, **every count matched
`P-497` exactly - 45, 7, 5, 37, 96, 29, 6 - and the natural key was unique over the 37 blocks.**
The check the instruction carries passed in full, and two of the relations were wrong.

**That is the failure this repository keeps recording, committed by the check written to prevent
it.** A row count asks *did every row arrive*; it cannot ask *is each row right*. The instrument
answered a narrower question than the one asked and returned a plausible number.

## One - a constraint that names a kind has nowhere to put it

`refuel`'s qualifier is `free energy at least 1`, which **you chose this morning** so that a bin
could say *free of what*. Decomposed by `P-497`'s schema, which has `trait` and `compare` and
nothing else:

```
{constraint block:refuel seq:2 trait:free compare:energy-at-least-1}
```

**The kind has been swallowed into the comparison.** `energy-at-least-1` is not a comparison; it is
a kind and a comparison run together, and a reader cannot get `energy` back out without parsing a
string. **The E1 form has three parts and the relation has two.**

**What it wants is a fourth column** - `{constraint block:refuel seq:2 trait:free kind:energy
compare:at-least n:1}` - which also splits `at-least-1` into a comparison and a number, and those
are two facts as well. **Whether `n` is its own column is the same question one level down.**

## Two - `Where` holds three different things, and this lane slugified them

**Seven distinct values, and they are not one kind of thing:**

| The cell                                   | What it is                                      |
| ------------------------------------------ | ----------------------------------------------- |
| `` `$where` ``, `` `$from` ``, `` `$to` `` | a reference to something the command bound      |
| `that unit`                                | a reference to an ingredient of this same block |
| `the orbit above `$where``                 | a place **derived** from another place          |
| `a store for energy`, `a store for metal`  | a place **described** by what it holds          |

The migration wrote the third as `place:the-orbit-above-where`, **which is prose flattened into an
identifier and means nothing**. The same for `a store for metal`.

**This is the Traits column's own finding, one column over**, and this item did not look for it -
it found the Traits column by decomposing and took `Where` on trust.

## What this lane is not doing

**Not guessing the shape.** Both are the same question - *what are the parts of this cell* - and
`P-497` answered it for Traits by measurement and for `Where` by assumption. **The measurement for
`Where` is above; the shape is yours**, as the notation is.

## What it does not block

**The five relations that came out clean** - `carries` 45, `member` 7, `limit` 5, `block` 37,
`for` 6 - are unaffected by either question. **`line` is affected only in its `place` column**, and
`constraint` only in how a qualifier splits. So this is two columns, not a rethink.
*Nothing is open.*
