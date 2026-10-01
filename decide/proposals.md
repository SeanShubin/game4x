# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-603 - The application is one function over normalized state, and the interface is not an exception

**to** sean · **status** open · **raised** 2026-10-01 · **answered** 2026-10-01, by him stating the general strategy · **asks** approval · **kind** recovered · **shape** text · **into** `spec/invariants.md` -> The game is one function

**You answered it with something larger than the question.** The question was whether the friendly
form stretches to an interface test; the answer is that there was never a second kind of thing for it
to stretch to.

**Offered as a block at the end of that section:**

> **This is not special to the game.** All application state can be represented as normalized data,
> and the application is a function from that state and a command to a new state. **The user
> interface is state like any other** - a menu item is a row, what is displayed is a row, what has my
> attention is a row - so an interface test is given rows, one command, then rows, in the same form.
>
> **Which is why the tests look the way they do.** `{given}` is old state as normalized rows,
> `{when}` is a command as a row, `{then}` is new state as normalized rows. **The shape is not a
> convention of the test format; it is the shape of the thing being tested.**

## What this settles and what it leaves

**It settles rule 3.** *A test is stated in the friendly form* is now true of both sets rather than
quietly true of one, because there is one kind of state and one kind of transition. **No cleanup is
needed and the contradiction dissolves rather than being resolved.**

**And it decides a question nobody had asked yet**: whether the interface needs an engine of its own.
It does not - **the same function over different relations.**

## Why it lands in that section rather than the two it generalises

**Two sections already state these halves about the game.** *The game is one function* says *a game
state and a transition yield a new game state*; *The data is a normalized relational model* says *the
game's data is a set of fully normalized relations*. **So the new content is only the reach**, and it
is offered in the first because the function shape is what your statement leads with.

**If you would rather it sat in the normalized-model section, say so** - that is a placement rather
than a wording, and the specification lane may move it afterwards without asking.

*Nothing is open. Everything filed has been decided.*

