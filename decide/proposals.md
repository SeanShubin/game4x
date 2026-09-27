# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-573 - a rule carries through the columns it does not name, and a report says which those are

**to** sean · **status** open · **raised** 2026-09-27 · **answered** 2026-09-27, `M5` · **asks** approval · **kind** entailed · **shape** text · **into** `spec/invariants.md` -> The game is data

**You chose `M5`**: the notation carries through, and the loud event comes from a report rather
than from ceremony in every rule.

**What it fixes.** `{move what:ark ...}` is refused today with `` `move`.`4` binds nothing to
`gathering` `` - `move` acts on the family `unit`, and `ark` alone carries a fourth column that
`move` never mentions. **Three of four members move and the fourth does not.** Under this rule
`move` is unchanged and the ark moves, keeping whatever it was gathering.

## The words

**One bullet, into `spec/invariants.md` -> The game is data, after *nothing in the state is special
to a kind*:**

> - **A rule carries through the columns it does not name.** A rule acting on a family acts on
>   members that may carry columns it never mentions, and what it does not name it leaves as it
>   found it. **So giving a kind a new column does not break a rule that has nothing to say about
>   it** - which is *adding a kind adds no field and no case* said about a column rather than a
>   kind.

## What this does not say, and it is the half you chose it for

**Nothing here makes a new column loud.** That is the report, and it is the code lane's to build:
for every rule over a family, every member's column that no clause names. **It reports rather than
refuses** - `M4` would have made it an error, and you took `M5` because a line that always says
*carry through* is a line that gets pasted.

**It will be filed to the code lane when this lands**, not before, because a rule follows the words
rather than preceding them. `C-152` already scoped the mechanism.

## What you are giving up, stated so the report is not forgotten

**The report can be ignored and `M4` could not.** If it is never read, a rule that should have said
something about a new column silently will not - which is the one thing `M4` bought and this pays
for by other means. **It is the same bet as the regression suite**, and your reason there was that
a difference you look at becomes a test and a difference you do not is a scenario change.
