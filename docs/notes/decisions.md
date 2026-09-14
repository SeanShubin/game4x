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

**They are in [`decide/questions.md`](../../decide/questions.md).** This file keeps the answered
ones, for their reasoning.

## Answered, kept for the reasoning

### P-507 - A description cannot say what a thing holds, so only an empty container can be named

**to** sean · **status** **answered** 2026-09-14 · `E4`, and `P-508` carries it concretely · **raised** 2026-09-14 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `spec/console.md` -> The language, and `P-501` before it is promoted

**You asked how `repeat:7` finds the empty transport, and then asked the question that breaks it.**
The first has an answer. The second does not, and `P-501` is held until it does.

## Why the first one works

`P-501` says a description used to name is **exact** - it leaves out no trait **and no content**. So
after

```
{stow into:{transport metal} kind:metal repeat:3}
```

that transport holds `{metal} -> 3` and **is no longer named by `{transport metal}`**, which states
no contents and therefore names things holding nothing. The second command has one candidate left.

## Why the second one does not

**One transport holding 1 metal, one holding 2, and you want 2 into the first and 5 into the
second.** Both are non-empty, so both must be named by their contents - and **the game notation has
no way to write contents inline.**

Containment is written by indentation, in a state file:

```
{transport resource:metal}
  {metal} -> 1
```

**A command is one line.** `P-212` lets a value be another command - `{a b:{c d:1}}` - but that is a
**named argument**, and contents are a map from a description to a quantity with no name to hang on.
**So there is no expression for *the transport holding one metal*.**

**`P-501` is therefore incomplete rather than wrong.** It works for exactly one case - the empty
container - and your example is the first that is not it.

## Three ways, and the choice is which

**`E1` - a command carries a tree by indentation, as a state does.**

```
{stow kind:metal repeat:2}
  into {transport resource:metal}
    {metal} -> 1
```

**One notation, and containment written the one way it is already written.** The cost: a command
stops being a line, and `scenario/commands/*.4x` is line-oriented today.

**`E2` - contents as a named argument.**

```
{stow into:{transport resource:metal holding:{metal}:1} kind:metal repeat:2}
```

Stays on one line. The cost: `holding` is a word the notation does not have, and a thing holding two
kinds needs it twice, which is a map wearing a field's clothes.

**`E3` - the entry form inline, in brackets.**

```
{stow into:{transport resource:metal [{metal} -> 1]} kind:metal repeat:2}
```

Stays on one line and reuses `->`. The cost: `[` and `]` are punctuation the notation does not have,
which is a second way of writing containment.

## `E4` - name the entry by its position, and write no contents at all

**Sean, 2026-09-14**: *for manual allocation we will likely be able to select individual transports,
which implies all we really need is a positional notation, to know what is in each transport, and to
be able to add/remove from a transport by position.*

```
{stow into:2 kind:metal repeat:2}
{stow into:3 kind:metal repeat:5}
```

**A position is an entry's index in the order the state already puts it in.** `spec/console.md`
already fixes that order - *entries are in the order their descriptions sort in* - so **nothing is
invented and no contents are written.** `P-502`, as corrected today, makes the order total where two
entries share a description.

**It is not a fourth identity, it is the second one indexed.** `id` names a thing for ever; a
description names a set; **a position names an entry in a container's own listing**, and it changes
when the state changes because the listing does.

## Why `E4` beats the other three, and it is a different kind of argument

**The other three invent a way to write contents. `E4` writes none.** `E1` makes a command
multi-line, `E2` adds a word, `E3` adds punctuation - and all three restate what a container already
displays. **A position points at it instead.**

**It is also the only one that does not grow with what a thing holds.** A transport holding four
kinds needs four contents clauses under `E2` or `E3`, and one number under `E4`.

## What Sean raised against it, and it is the real limit

**Stacking.** *How are we to have massive fleets if I have to make each one selectable by the user.*

**A position names an entry, not a thing**, so a million identical transports are **one entry at one
position**. Positions do not grow with the fleet. **They grow with the number of distinct
(description, contents) combinations**, which is the honest bound and is the one he named: *an
additional problem when we have more possible combinations of contents than can fit on a user
interface.*

**This lane has not bounded that number and does not know it.** What can be said is that it is the
number of distinct states, which `spec/logistics.md` already relies on being small - *a kind has few
states however many things of it there are.* **That sentence is load-bearing for `E4` and was
written before anything held cargo**, so it is a premise to check rather than a reassurance.

## And the simple algorithm he named needs no addressing at all

*Fill up each transport and move the ones that are full.* **That is `repeat` against a description
that names the not-yet-full ones**, and the set shrinks as they fill - `P-501`'s mechanism, no
position required. **So position is for manual allocation only**, which is the case it was proposed
for.

## What this lane would pick and why it is not confident

**`E4`, and this lane changed its mind on being given the position idea.** It argued for `E1` on the
grounds that *there is one notation* and a state writes containment by indentation. **`E4` is better
by that same rule**: it writes no containment at all, so there is nothing to write a second way.

`E1` remains the answer if a command should be able to name a thing that is **not** in a listing
anybody has - a hypothetical container, or one being created. **Nothing in the three scenarios needs
that.**

**The reason this is a decision and not an approval**: the cost of `E1` falls on files you derive by
hand this week, and whether a command may stop being one line is a judgement about your own reading
rather than about the notation.
*Nothing is open.*

### P-499 - `P-497` refused: two columns it treats as atomic are not, and the counts passed anyway

**to** sean · **status** **answered** 2026-09-14 · `J1` for the constraint, and `P-500` for the `Where` column · **raised** 2026-09-13 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `P-497`, before it is promoted

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
