# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-552 - An Ark holds one energy, mines one a turn, and spends one to move

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `spec/units.md`, and `releases/first-release.md` -> Units and structures, Recipes

**Your words**: *ark has a storage container for 1 energy, ark can mine energy every turn.
Thematically energy comes from the sun via solar panels... it is self contained and automated, no
need to feed civilians or for civilians to generate labor. One move costs 1 energy.*

**That is `F1` with the numbers filled in**, and it resolves the contradiction rather than choosing
between its halves: pooling holds, and the Ark contributes the room while the orbit holds the
energy.

## What lands

**Replacing `spec/units.md`'s two fuel bullets**, which are the half that said a unit holds its own:

> - A mobile unit contributes room for fuel to the place it is in, and moving spends a unit of
>   energy from the place it leaves. **One with no energy where it stands cannot move.**
> - **A mobile unit that moves in orbit gathers its own energy from the sun.** It mines one unit
>   each turn into the orbit it is in, needing no citizen and no labor - **it is self-contained and
>   automated**, which is what being above the surface buys it.

**The Ark's `Fuel` cell**, which is blank today, becomes `1`.

**And a world recipe**, beside the other automatic ones:

> | **mine energy**     | world  | require | 1 | ark    |  |  |
> |                     |        | produce | 1 | energy |  |  |

## Why the numbers agree with a rule written weeks earlier

**Room for 1, one a turn, one a move**: an Ark moves once per turn. **The Units table already says
`moving 1`** - so the fuel economy and the move limit meet on a number neither mentions, and
neither is derived from the other.

## The one thing your words leave open, and this lane's reading of it

**You said *can mine* and also *automated*.** This takes it as a **world** recipe - the world fires
it at every turn's end, like `upkeep` - because *self-contained and automated* is the stronger
clause and because a player recipe would appear in the command list as something to remember.
**Say so if you meant a player recipe**, and the row changes one word.

## What it costs and what it does not

**The gate goes red until the code lane follows**: a new recipe, a trait cell, and
`scenario/expected/play.4x` gains an energy line in orbit 1 after the launch. **Said in the same
breath as the rule.**

**And nothing here changes the surface.** `move` already consumes one energy at `$from`, and the
release already says a tank gives room rather than holds - `P-512`. **The ground bullet above only
makes `spec/units.md` say what the release has said since then**, which is `S-170`'s other half
left where it is.

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

