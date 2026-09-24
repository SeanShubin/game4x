# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-547 - `adapter` in rule 4 means the layer, and the only real violation is a plugin written in the composition root

**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** an instruction · **asks** a decision · **into** `docs/architecture.md` -> The layers and Rules

**This item offered three answers and two of them were answers to the wrong question.** Reading
`docs/architecture.md` -> The layers rather than only its Rules changed what is broken. **The
question left is one row of a table.**

## What the layer table already says

```
Layer              Knows about                   Does not know about
Supporting crates  Spheres, graphs, integers     Pixels, windows, engines
Rendering          Pixels, cameras, projections  Windows, input devices, engines
Engine adapter     Bevy, windows, input, vsync   How anything actually works
Composition root   All of the above, briefly     Nothing else; it holds no logic
```

**`Engine adapter` is a layer, not a crate.** So `planet-bevy` and `planet-flat` both sitting in
it was never a violation - the document calls both of them adapters because both are adapters, and
rule 4's *the adapter* is this row. **This lane read the rule and not the table**, and reported
four violations where the table already excused two.

## What is actually left, and it is two things of different sizes

**One: `game-globe` and `planet-ecs` are engine-side and the layer row does not fit them.**
`planet-ecs` holds ECS entities, which rule 6 says *exist where the engine needs something to draw
or to receive input* - engine-side by definition. `game-globe` is a Bevy plugin crate whose whole
job is to make the globe follow **the** game.

**So both know Bevy and both know how things work**, and the `Engine adapter` row says the layer
*does not know about how anything actually works*. **That sentence is what is wrong**, not the
crates.

**Two: `crates/game4x/src/inspect.rs` writes a plugin in the composition root.** 166 lines,
`InspectPlugin`, two systems taking `Res` and `ResMut`. **Rule 4 permits the root to assemble
plugins and not to write them**, and the layer table says the root *holds no logic*.

## The decision, which is smaller than what this item first asked

**`A1` - widen the layer's description.** `Engine adapter` knows *Bevy, windows, input, vsync, and
what the game is when a surface has to follow it*. One row, no code moves, and `planet-ecs` and
`game-globe` stop being violations because they never were.

**`A2` - split the layer in two.** `Engine adapter` stays engine-only for `planet-bevy`,
`planet-flat` and `planet-ecs`; a fifth row above it holds engine code that knows the game, which
today is `game-globe` alone. Sharper, and a layer with one crate in it.

## What this lane would say

**`A1`, and it is the smaller claim.** `A2` describes a distinction that is real - `planet-bevy`
knows nothing about a game and `game-globe` exists to know one - but a layer per crate is a table
that has stopped being a layering.

## And `inspect.rs` needs no decision from you

**The fix is a crate boundary, not a rewrite.** Its own header says what matters: *nothing here is
compiled differently from what ships - the same binary plays and poses*, and **that survives the
move intact.** A crate of its own that `game4x` adds as a plugin keeps every word of that true and
puts the systems in the adapter layer where they belong.

**Filed to the code lane in `S-160`**, which this lane will narrow to say so once you have answered
the row above - because *where* the plugin should go depends on whether there are four layers or
five.
