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

**And a player recipe**, which is the shape `work` already has:

> | **mine energy**     | player | require | 1 | territory |                    | `$where`       |
> |                     |        | require | 1 | ark       | working at least 1 | above `$where` |
> |                     |        | put     |   | ark       | working one less   | above `$where` |
> |                     |        | produce | 1 | energy    |                    | above `$where` |

**The Ark's `Readies` cell gains `working 1`**, beside the `moving 1` it has, and
`spec/data/carries.4x` gains `{carries kind:ark trait:working}`.

## Why the numbers agree with a rule written weeks earlier

**Room for 1, one a turn, one a move**: an Ark moves once per turn. **The Units table already says
`moving 1`** - so the fuel economy and the move limit meet on a number neither mentions, and
neither is derived from the other.

## A recipe, and the sentence that makes it one is already promoted

**This lane read *automated* as a world recipe and you corrected it**: *the automated part was
thematic. The user interface layer may automate a lot to remove busywork from the player, but as
far as the engine is concerned every transformation takes a recipe.*

**`spec/interface.md` already says the first half of that**, which is why the correction costs
nothing: *what is offered, and what is done unasked, is a layer above the rules that writes the
commands a player would have written.* **So the interface firing `mine energy` every turn is the
rule working rather than an exception to it** - the same layer that already works every extractor
with somewhere to put what it makes.

**And `work` is the shape to copy rather than invent.** It is a player recipe, it readies on
`working`, and the interface fires it unasked. **Mining differs in three cells and no structure**:
no labor, no density, and the place is the orbit above `$where` rather than `$where` itself.

## Why the Ark needs `working 1` rather than nothing

**A player recipe with no readying fires as often as it is asked.** `work` is bounded by
`working at least 1` and `working one less`, which is what makes it once per extractor per turn.
**Your *every turn* needs the same bound**, or an Ark can mine repeatedly within one turn.

**`working` is already declared** - `{trait name:working admits:number kept:thing}` - and carried
by the extractor alone. **Giving it to the Ark adds no vocabulary**, which is why this proposal
does not invent a `mining` trait.

## What it costs and what it does not

**The gate goes red until the code lane follows**: a new recipe, two trait cells, a row in
`spec/data/carries.4x`, and `scenario/expected/play.4x` gains an energy line in orbit 1 after the
launch. **Said in the same breath as the rule.**

**And nothing here changes the surface.** `move` already consumes one energy at `$from`, and the
release already says a tank gives room rather than holds - `P-512`. **The ground bullet above only
makes `spec/units.md` say what the release has said since then**, which is `S-170`'s other half
left where it is.

