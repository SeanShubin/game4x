# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-604 - Two runs of the one function, and a command is the only crossing

**to** sean · **status** open · **raised** 2026-10-01 · **answered** 2026-10-01, he chose the third form · **asks** approval · **kind** recovered · **shape** text · **into** `spec/invariants.md` -> The game is one function

**Your choice, stated as a rule.** It goes after the block `P-603` landed, which says the interface is
state like any other - this says the two states are not one.

**Offered as a block at the end of that section:**

> **There are two runs of that function and they share no state.** The game's run holds the game's
> relations; the interface's run holds the interface's. **Neither can name the other's relations**, so
> a rule of the game reading a menu item is not something that is refused - it is not something that
> can be written.
>
> **The interface affects the game by issuing a command, and that is the only crossing there is.** It
> never writes a game relation.
>
> **And it holds its own rows for whatever game state it shows.** A menu naming a territory has a row
> of its own saying so; the game does not know the menu exists.

## Why this form rather than a refusal

**Because a prohibition and an absent capability look alike while they hold and differ when something
is added.** A `concern` column refuses a crossing somebody wrote; this makes the crossing unwritable.
**That is `S-227`'s lesson about `shell::` said about data** - the `thread_local` is still there and
stopped being load-bearing because the root hands out a handle instead.

**And the property is already most of the way true.** Measured: 55 of 55 clauses declare their own
relation, a `{reading}` names another clause rather than a relation, and nothing reaches unnamed
state. **A rule is handed its clauses' relations and nothing else** - what this adds is that the two
sets of relations are never in the same run to be named from.

## What it leaves open, and it is the cost you accepted

**What produces the interface's copy of game state.** The third paragraph says the interface holds its
own rows for what it shows and does not say what writes them. **It cannot be a rule of the game**,
which cannot name an interface relation, **and it cannot be a rule of the interface**, which cannot
name a game one.

**So something outside both runs reads one and writes the other.** That is a third thing, it has no
name, and it is the next question rather than this one. **Nothing is blocked by leaving it**: no
interface relation exists yet.

*Nothing is open. Everything filed has been decided.*

