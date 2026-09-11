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

### P-390 - The check decides which traits are a capacity, and that is a fact about the game

**to** sean - **status** open - **raised** 2026-09-11 - **kind** entailed, from `S-93` built - **asks** a decision - **into** `releases/first-release.md` -> Traits, and `spec/invariants.md` -> Nothing comes back round with more

**Your invariant is decided mechanically now** - `reports/nogain.md`, 43 rules ground from 24
blocks, a weighting solved for rather than declared, and no rule gaining. **This lane verified the
arithmetic rather than relaying it**: the tightest rule is `work (food x6)`, which nets +6 food at 4
against one draw on the planet at 20 and a labor and a readiness at 2 each. Exactly zero.

**It rests on one list that is written in code and nowhere else.**
`crates/game-console/src/nogain.rs` declares `CAPACITIES` as two trait pairs - `ready`/`not ready`
and `fertile`/`spent`. **That list decides which rules draw on time**, and therefore whether a
weighting exists at all.

**The generalisation is sound and you have not approved it.** `P-388` says *anything that **exhausts**
is a **readiness** extractor for a turn.* The check reads that as *any capacity spent by acting*, so
that `renew` draws on time the way `refresh` does. **Its ground is the release's own words** -
fertility is *a citizen's capacity to raise one more, spent by raising one and renewed each turn* -
so this is a fair reading rather than an invention. **It is still a rule that exists only in Rust.**

**Two questions, and the second is the one that lasts.**

- **Does `P-388`'s *readiness* mean any capacity that is spent by acting and refilled?** If yes, one
  word in `spec/invariants.md` carries what the check already does. If no, `renew` needs its own
  sentence and this lane will draft it
- **Should the release name which trait pairs are a capacity?** It is a fact about the game, and
  `spec/invariants.md` says *what the game is made of lives in a data file*. Today the code decides
  it, so a third capacity is a Rust edit rather than a release edit

**Not urgent, and the reason is worth having.** A capacity the list misses is never charged to time,
so a rule refilling it reads as a gain and **the check fails loudly rather than passing quietly.**
The failure mode is the safe one. **What it costs is that the answer is partly believed** - which is
the one thing you said this check was for.

**One smaller thing travels with it.** The release's *Traits* table names `spent`, a citizen, yes or
no. The recipes say `fertile` and `spent` as if they were two values of one trait. **The check
bridges that** and nothing states it. Whichever way the second question goes, that bridge stops being
the code's guess.

