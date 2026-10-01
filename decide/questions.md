# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-604 - Which relations may a rule touch, now that the interface's are rows too

**to** sean · **status** open · **raised** 2026-10-01 · **asks** a decision · **kind** recovered · **into** `spec/invariants.md` -> The game is one function · **source** him asking whether the composition roots keep interface detail out of mechanics

**`P-603` made the interface state in the same form, and that removed the thing that was keeping them
apart.** Not a crate boundary - `S-227` holds that one - but a boundary inside the data.

```
relations the engine distinguishes by kind     0
rules in `spec/` about which relations a       0
  rule may read
```

**Before `P-603` a game rule could not read a menu item because there was no menu item to read.**
Now there will be, and `docs/architecture.md`'s *enforced by the compiler rather than by discipline*
does not reach inside one engine over one set of relations.

## The question, and the symmetric answer is wrong

**A menu has to show a territory's name**, so *no crossing in either direction* cannot be the rule as
stated. **And a game rule has no business reading what has my attention.** So the prohibition is not
symmetric, and which asymmetry it is decides what a check can say.

## Three readings, and the third may dissolve it

**Directional.** An interface relation may be read by a rule of the interface and by nothing else; a
game relation may be read by either. **Cheap to check and it permits a rule of the game to depend on
nothing the player is hovering over.**

**Reads yes, writes no.** Either side may read the other; only its own side may be written. **Weaker,
and it allows a rule of the game to branch on a menu**, which is the case this is being asked about.

**No state crossing at all, because there is none to make.** The interface **reads** game state to
display it and **affects** game state only by issuing a command - which is what *select the ark,
select the place, confirm the move* already describes. **Then the crossing is a command rather than a
read**, and the rule is about what may be written rather than what may be seen.

**That third reading also splits the interface's own state in two**, which is worth knowing before
choosing: *what is displayed* is a view of game state and **what has my attention is not** - nothing
in the game knows the cursor is over a menu. So interface relations are partly derived and partly
their own, and only the second part is unambiguously the interface's.

## What is not asked

**Not the wording.** No block is offered, because each reading is a different sentence and two of
them need a second one about commands. **Say which and the words follow in a day.**

**And not a check.** Whichever you pick is checkable over every rule rather than reviewed case by
case - which is the shape this repository keeps finding it needs - but building one now would be a
lane deciding this.
