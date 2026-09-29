# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-588 - The sun is a deposit on a container above the territory, and that container does not exist yet

**to** sean · **status** open · **raised** 2026-09-28 · **asks** a decision · **kind** measured · **from** `C-166`

**The concept you remember is real and it is not in `spec/`.** *Every thing but the game is in
another thing* was written in `releases/first-release.md` -> Where things are, which `c7bcd95c`
deleted with the old ruleset's tables on your word. **It survives in `docs/notes/decisions.md`, which
is not binding, and in `crates/outbox.md`, which is a record.** `spec/` has never said it - measured,
zero occurrences.

**What `spec/logistics.md` does say is the half that fits it**: *a thing may contain things, and is
itself in at most one other thing.* **So the containment allows a thing above a territory; nothing
names one.**

```
{relation name:game}     does not exist
{relation name:planet}   does not exist
{store id:2 name:game}   exists - but a store is where rows live, not a thing in the world
{primitive word:game}    exists - an engine word, not a row
```

## So your question has a yes and a choice inside it

**Yes: a deposit on a container above the territory is the shape**, and it needs no new column. A
deposit already names *where* it is; what is missing is a thing for it to name. **`gather` would not
change at all** - it requires a deposit at the ark's place today, and would require one at the
planet the ark's place is under.

**The choice is which container**, and the two are not the same bet:

- **`planet`.** The sun shines on a planet, and `spec/planet.md` already exists to describe one.
  **It leaves room for a second planet** without the sun becoming ambiguous, which `game` does not
- **`game`.** It is the word the notation already uses for the root, it is the containment
  `docs/notes/decisions.md` describes, and **one row exists for the whole world by construction.**
  The first release's scope is *a single planet*, so today the two are the same thing - and the day
  they are not, a sun on the game is wrong

**This lane would take `planet`** - it costs nothing today, and it is the one of the two that cannot
become wrong. **It will not write either without you**, because a noun in the data model is decided
in `spec/invariants.md` and that is yours.

## And one thing needs doing whichever you pick

**The containment root belongs in `spec/`.** *Every thing but the game is in another thing* is a rule
the engine already keeps - `rooming`, `stands-in` and the capacity checks all rest on it - and it is
currently stated in a note and a deleted release. **Say the word and it is a proposal into
`spec/logistics.md` -> Containment**, whose first bullet is the half that assumes it.
