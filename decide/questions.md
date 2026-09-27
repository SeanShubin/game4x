# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-575 - the reviewed ruleset can play a game and cannot make one, so the switch is not one pass

**to** sean · **status** open · **raised** 2026-09-27 · **asks** a decision · **kind** measured · **into** `spec/tests/`, and `releases/rules-become-data.md`

**You asked whether the switch needs any new information or is just mechanical.** Most of it is
mechanical. **One part is not, and it is a hole rather than a difficulty.**

## The two rulesets, side by side

```
reviewed (15)  breed build-bin build-extractor build-pioneer deploy discard-disorder
               end-turn gather launch move perish refresh toil upkeep work

old (15)       AddUnitToOrbit Build BuildStore CreateLabor CreatePlanet FoundByLand
               Land Launch MineEnergy Move ProducePioneer SetBiome SetForceOfNature
               SetResource Work
```

**Ten of the old fifteen map across**, as a rename or a merge: `Land` is `deploy`, `BuildStore` is
`build-bin`, `MineEnergy` is `gather`, `CreateLabor` is `toil`, `FoundByLand` folded into `deploy`
when the literal was taken away. **`SetForceOfNature` goes with force, which you cut on
2026-09-20.**

**Four have no counterpart and they are all the same kind of thing:**

```
CreatePlanet     make a planet and its territories
SetResource      give a territory its capacity and density
SetBiome         give a territory its biome
AddUnitToOrbit   put a unit in orbit before play begins
```

**The reviewed ruleset can play a game and cannot make one.**

## Why that is a hole and not an oversight

**`spec/console.md` says design is made of rules**, not of something outside them: *a game state
changes only by a transition... and designing is therefore made of the same rules as playing. The
design commands are the player's recipes, offered only while the phase is design.*

**So by the specification those four must be rules, and they are not among the fifteen you have
read.** Measured: **no reviewed test builds a world with a command** - all fifty-four state their
world in `{given}` and start from it. **Nothing has ever exercised world-making under the new
model, which is why nothing has noticed.**

## What that means for a single pass

**Delete the old model and the application cannot start a game.** `create planet`, `set resource`,
`set biome` and `add <unit> orbit` are how a world comes to exist today, and `generate-planet` -
the one replacement the specification names - **is specified at `spec/console.md:243` and
implemented nowhere.**

## The decision

```
N1  write the four as rules, review them, and the switch waits on your reading.
    Design becomes data like everything else, which is what spec/console.md says
N2  build `generate-planet` instead - one thing outside the ruleset that makes a
    world, and design commands stop existing. Fewer rules, and it contradicts
    "designing is made of the same rules as playing" unless that sentence changes
N3  the switch ships without world-making and the app starts from a stated world
    until one of the above lands. Playable from a file, not from a new game
```

**This lane would say `N1`**, because it is the only one that needs nothing else changed, and the
four rules are small - each writes rows nobody computes. **`N2` is the larger idea** and may be the
one you want, since a generated planet is what a player actually starts from and no player wants to
type `set resource` twelve times.

**What is not in question**: the other ten, which are mechanical, and the deletion, which you have
confirmed.
