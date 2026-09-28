# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-583 - You were right: `launch ark` required a Yard, the new `launch` does not, and a test you approved launches without one

**to** sean · **status** open · **raised** 2026-09-27 · **asks** a decision · **kind** measured · **from** the code lane reporting `build-yard` as the only player command with no rule

**The requirement existed and was explicit.** The old ruleset's `launch ark` recipe, at `c7bcd95c^`:

```
| **launch ark** | player | require | 1  | territory |  | `$where`       |
|                |        | consume | 3  | metal     |  |                |
|                |        | consume | 12 | energy    |  |                |
|                |        | consume | 2  | citizen   |  |                |
|                |        | require | 1  | yard      |  |                |
|                |        | produce | 1  | ark       |  | above `$where` |
```

**The new rule has no such clause and there is no `yard` in `spec/data/` at all** - no kind, no
relation, no row. `launch` requires two places, removes a labor, a metal and an energy, and adds an
ark.

## The part that makes this a decision rather than a repair

**A test you have read launches without one.** `reviewed/a-second-settlement-launches-the-ark-the-first-could-not.4x`
fires `{launch where:place-3}` after `{toil}`, `{work metal}`, `{build-extractor energy}` and
`{work energy}`. **No yard in its `given`, none built in its `when`, none in its `then`.** So the
requirement was already gone when you approved it, and `spec/README.md` rule 3 says the test is the
primary statement and the prose is the defect.

**What is still stated, and all of it agrees with you rather than with the data**:

```
spec/structures.md:18     A Yard produces Arks
releases/first-release.md:45    build a Yard there, and launch an ark from it
releases/first-release.md:202   requires a Yard
crates/game-console/       `build-yard` is a live command in the old model's grammar
```

## What was recorded, and what was not

**That `build yard` had no engine counterpart was known** - `S-193` lists it among six, with `age`,
`bear`, `renew`, `spoil` and `stow`. **That `launch` had stopped requiring one was not recorded
anywhere.** The item was about which lane writes which half, so the count was of missing rules and
not of requirements that had gone with them.

## The two answers, and their costs are not alike

- **The Yard comes back.** `spec/data/` gains a `yard` kind, a `build-yard` rule and a `require`
  clause on `launch`. **And the reviewed test becomes wrong** - it launches without one, so it goes
  back through the review application, which is one of your fifty-four
- **The Yard goes.** `spec/structures.md` loses its Yard section, the two lines in the release go,
  and the console's `build-yard` goes with the old model under `D-1`. **Nothing you have approved
  changes**

**This lane is not recommending either**, and that is deliberate rather than reticent: the first is
what you said you understood the game to be, and the second is what every test you have read says it
is. **No wording can be final until you say which.**
