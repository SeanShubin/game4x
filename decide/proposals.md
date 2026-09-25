# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-553 - The deploy comes first, and the launch is the winning act

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** text · **asks** approval · **into** `spec/control.md` -> Winning

**You answered by describing the loop**: *deploy the ark to the surface, develop the territory we
deployed to, expand to an adjacent territory, develop the adjacent territory, launch an ark from
there.* **Six steps in order, the launch last** - which is `W2`.

## What lands

**Replacing the Winning bullet:**

> - A player wins by launching an Ark from a territory other than one an Ark has been deployed to.

## What it does and does not add

**It makes the order explicit** where the old sentence named two acts and left it open. `R-6`'s
*vetted when* already reads this way - *victory takes a launch from a territory other than the one
the Ark deployed to* - so the release stops assuming more than the rule says.

**Two words of yours are deliberately not in it.** *Adjacent* is how expansion works and not a
condition on winning; *develop* is what a Yard costs and not a rule. **Putting either in the win
condition would make it say more than you meant.**

**And nothing changes in the code.** `S-151` built `won` to latch after either act; `W2` is a
condition added, which the code lane said is one `match` arm away.

## What this leaves open, and it is the larger half

**Nothing in the release vets a win.** Eleven capabilities, and `R-6` - *the loop can be played
through* - **deliberately does not win and says so**: *this scenario deploys to territory 1 and
launches from territory 1.*

**So the loop you just described and the loop `R-6` vets are different loops.** Measured in
`scenario/commands/play.4x`: it deploys to 1, builds its Yard in 1, founds 2 by land, launches from
1, and territory 2 gets one turn of `create-labor` and `work food` and nothing else. **Five of your
six steps, and the missing one is developing the second territory far enough to launch from it.**

**Filed as `P-554`** rather than folded in here, because whether the first release should show you
a win is a scope question and this one is a wording question.

*Nothing is open. Everything filed has been decided.*

