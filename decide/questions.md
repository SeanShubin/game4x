# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-588 - `spec/invariants.md` names three sources and the data names none, so everything is charged to the planet

**to** sean · **status** open · **raised** 2026-09-28 · **asks** a decision · **kind** measured · **from** `C-166`

**`spec/invariants.md`**: *a source is named, and a named source is not a gain. There are three
sources: the planet, the star, and time.* **The data names no source at all**, so the solver that
decides the no-gain property had to choose where a draw comes from, and it charged every density draw
to the planet.

**Measured in `reports/nogain.md`'s weighting**: the wells are **the planet** and **time**. **The
star is not there** - nothing draws on it.

**And `spec/units.md` says something else**: *a mobile unit that moves in orbit gathers its own energy
from the sun.* **So an ark's energy is the star's in the prose and the planet's in the arithmetic.**

## The verdict holds either way, which is why this is a question and not a defect

**Charging every draw to one well is the strict reading.** A weighting that survives everything
coming out of a single endless source survives it coming out of three, so *nothing comes back round
with more* is not in doubt - `S-219` stands whichever way this goes.

**What is wrong is only that the arithmetic and the prose disagree about which well**, and nothing
notices, because the answer is the same.

## The choice

- **The data names the source.** A clause that draws says which well it draws on, and the solver
  reads it rather than assuming. **Then `spec/units.md`'s sentence is checkable** and the star stops
  being a source nothing uses
- **The prose keeps it and the data does not.** `spec/invariants.md` names the three, the solver
  charges everything to one, and **`spec/units.md`'s sentence is about fiction rather than
  arithmetic** - which wants saying there, because today it reads like a rule

**This lane leans to the first and will not act on a lean**, because it adds a column to the notation
and that is an idea rather than a wording. **What the second costs is a sentence in `spec/units.md`
saying the sun is the fiction and the planet is the ledger**, which is cheap and slightly sad.
### P-587 - A citizen's `bearing` is `1` in every world the rules can reach, so the trait tells nothing apart

**to** sean · **status** open · **raised** 2026-09-28 · **asks** a decision · **kind** measured · **from** `C-169`

**Re-derived here rather than taken from the report.** Every literal on the `bearing` column in
`spec/data/rules.4x` is `1`, all three of them, and two readings carry the value through unchanged.
**No clause anywhere writes a nought.**

```
{literal id:30 clause:clause-25 column:133 value:1}   deploy
{literal id:34 clause:clause-27 column:133 value:1}   breed, the child
{literal id:74 clause:clause-51 column:133 value:1}   breed, the parent back
{reading  id:2 ... takes:133}                          upkeep, preserved
{reading  id:6 ... takes:133}                          toil, preserved
```

**So three things follow and none of them is a defect in behaviour.** `{refresh what:citizen
trait:bearing}` can never fire, because nothing is ever spent for it to restore. A citizen's
`bearing` distinguishes no two citizens. **And the cap on breeding is real but comes from somewhere
else** - `repeats`, not the trait.

## Why nobody noticed, and why the data reads as though they had

**The comments are written for the other arrangement.** `spec/data/rules.4x`, on `breed`: *eating has
to say what the fed citizen's `bearing` is... a free breeding to any citizen that ate with its
bearing already spent* - and a few lines down, a defence against *two citizen rows differing in
`bearing`*, which cannot arise. **`schema.4x` declares the trait and `carries kind:citizen
trait:bearing`**, so `refresh` ranges over it.

**The behaviour is safe either way**, which is exactly why it went unseen: `repeats` was doing the
work the trait looks like it is doing.

## The two answers

- **Spend it.** `breed` puts the parent back with `bearing:0`, so a citizen breeds once a turn and
  the turn's end restores it. **The trait then means something** and `refresh` has work; the cap
  stops resting on `repeats` alone, which is a second statement of the same limit
- **Drop it.** The column, the `{trait}`, the `carries` row and the comments go, and `repeats` is
  the only thing that says how often a citizen breeds. **One fact stated once**, which is
  `spec/invariants.md`'s own preference

**This lane recommends neither, and the reason is your own sentence.** *I test the behaviour that
depends on the rules, and make sure there are no unnecessary rules* - **this is a row nothing depends
on, and whether that means a missing behaviour or an unnecessary row is the judgement you said no
sweep can make.** It is the first instance of `C-163`'s fork with a name on it.
