# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-537 - The window is mostly the hook's own run, and it is now closed

**to** sean · **status** open · **raised** 2026-09-21 · **kind** measured · **shape** text · **asks** approval · **into** `CLAUDE.md` -> Perspectives

**This is the last one and it can wait as long as you like.** Nothing depends on it and the
hazard it describes is already fixed in the hook. It is filed because the sentence below is now
telling a reader something that has stopped being true.

```
CLAUDE.md   the window is between your `git add` and your `git commit`, and checking for the
            lock falls before it
```

**True, and it hides where the seconds are.** Measured today, after the race swallowed twenty
lines of the code lane's work into a promotion commit of this lane's:

```
hooks/pre-commit   the column check   line 107
                   356 lines long
                   ~2 seconds on an empty index, longer with files staged
```

**So the index is read near the start and the commit is built after the end**, with the padder,
`tools/outbox` three times and `tools/spec` in between. **The window was not an instant - it was
most of a hook run**, and a `git add` from another lane landing in it goes into a commit the hook
has already approved.

**The code lane closed it**: the column check is now a function called twice, once before the
tools and once after, and it says which call refused, because *the index changed while this hook
was running* is a different thing for a reader to act on.

## What lands

The sentence becomes:

> **Staging by name bounds what you add and not what you commit**, so no amount of care closes
> it: the window runs from your `git add` to the moment git builds the commit, and most of it is
> `hooks/pre-commit`'s own run. **The hook checks the columns twice for that reason**, before its
> tools and after them, and `hooks/post-commit` reports the instant after it returns, which is
> the only part nothing can refuse.

## Why it is worth the read at all

**Because the old sentence says no amount of care closes it, and now most of it is closed.** A
lane reading the file today learns that the race is unavoidable and that staging by name is the
only defence. **It is neither of those things any more**, and the next instance would be an
instant rather than ten seconds.

### P-514 - Three rows of `spec/data/line.4x` are unwritable, and this lane's own check said there was one

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**Read against `P-530`, 2026-09-21.** **Same as `P-513`: right about `spec/data/`, and
`spec/data/` is in question.** The defect it reports is real either way - three rows cannot be
read back - so if those files are replaced this is fixed by the replacement rather than by this
proposal. **The third row it could not fix is the part worth keeping**, whatever notation wins.

**`P-497`'s migration wrote three rows that cannot be read back.** Found by the code lane building a
generator against the same data.

```
{line block:work seq:5 role:produce qty:`$where`'s density for that resource kind:resource}
{line block:muster seq:4 role:produce qty:that citizen's strength kind:force}
{line block:stand seq:3 role:produce qty:that unit's strength kind:force}
```

**A value with spaces in it cannot be told from the words after it.** `qty:that citizen's strength
kind:force` reads as a `qty` of `that`, then four words belonging to nothing. **The notation has no
multi-word value and these three rows assume one.**

## And this lane's check reported exactly one such cell

`P-497` asserted that precisely one cell of the release could not be represented - `move`'s *joined
to `$from` by an edge the unit crosses* - and named it so that a second would fail the run.

**It asked only about the Traits column.** Quantities were written straight through without being
classified at all, so three unrepresentable cells passed a check built to catch exactly that, in the
item that introduced the check. **A right answer about the wrong population**, inside the migration
whose whole argument was that counts cannot ask whether each row is right.

## What lands, and the shape is already in the file

`spec/console.md` already has the thing these quantities are: **a path, which reads a trait of
something a name is bound to.** And `P-497`'s `place-line:` already refers to another row of the
same block by its sequence.

> **A quantity that reads a trait names what it reads it from and what it reads.** Where a relation
> writes such a quantity it uses two columns rather than one - which row of the block the thing came
> from, and which trait of it is read. **A value is one word**, and a quantity that needs more than
> one word is more than one fact.

**So the three rows become:**

```
{line block:muster seq:4 role:produce qty-line:2 qty-trait:strength kind:force}
{line block:stand  seq:3 role:produce qty-line:1 qty-trait:strength kind:force}
```

## The third one is not the same and is not fixed here

**`$where`'s density for that resource reads a trait of a place, per resource.** It is not *the
strength of the thing at row two*; it is a density indexed by which resource the block is for.
**Two columns do not hold it**, and this lane is not inventing a third form for one row.

**It is named rather than fixed**, which is the thing `P-497`'s check was supposed to do and did not.

## What this costs you to read

**Nothing in the game changes.** The release's Qty cells are untouched; this is about how the
relational form writes what they already say, and about one row it still cannot.
### P-513 - A relation names its columns, and nothing says so

**to** sean · **status** open · **raised** 2026-09-14 · **kind** measured · **shape** text · **asks** approval · **into** `spec/console.md` -> The language

**Read against `P-530`, 2026-09-21.** **This may not survive the change in direction.** It says a
relation of `spec/data/` must declare its column order. The thin-engine states the same facts in
its own notation, and whether `spec/data/` survives at all is open - so this is right about the
file it names and the file may go. **Worth answering `P-530` first.**

**`spec/data/` is the source now and nothing states what order its words go in.** `spec/console.md`
fixes the order for a **description** - *`id` first, then every other trait alphabetically, then
`occupied`, `free` and `capacity` last* - and **that rule describes none of the eight relations**.

```
carries      kind trait                          alphabetical would be: kind trait
member       kind family                                                family kind
limit        container contained n                                      contained container n
above        orbit territory                                            orbit territory
block        id recipe owner                                            id owner recipe
line         block seq role qty kind place-bound                        block kind place-bound qty role seq
constraint   block seq trait compare n                                  block compare n seq trait
for          block seq kind                                             block kind seq
```

**Two of the eight happen to match and six do not** - `carries` and `above`, and nothing else -
which is worse than none matching: the rule appears to hold until it is relied on. **This lane first
wrote three**, from the table directly above it, and the code lane re-derived it to two.

> **A relation names its columns, and a row gives them in that order.** The order is the relation's
> own and is stated where the relation is declared. **A description's order is a different rule** -
> it ranks traits because a description has no declaration to name them in.

## What this is for, and it is the guarantee that is missing

`spec/console.md` already says **the same state is always the same bytes**. **That covers a
description and does not reach a relation row**, so two writers of `spec/data/` could disagree about
column order and both be right. **The code lane hit it building a generator**: `Description::ordered`
cannot write these files, because no single ranking gives both `carries` and `constraint` their
order.

## Where the measurement is weaker than the code lane's and stronger overall

**Their example no longer separates the two.** They cited `carries` ordering kind before trait
against `constraint` ordering trait before kind - and `constraint` has **no** `kind` column today,
since `P-511` deleted the only row that had one. **The finding survives the example dying**: the
eight orders above are measured from the files as they stand, and five of them the description rule
gets wrong.

## What it does not do

**It does not choose the orders.** Each relation's is whatever it is declared to be, and the eight
above are what the code lane's generator writes. **This says they must be declared**, not what they
should say.