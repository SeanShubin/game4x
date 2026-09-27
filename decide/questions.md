# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-575 - the rule notation is built for playing and world-making is a different operation, measured four ways

**to** sean · **status** open · **raised** 2026-09-27 · **stopped** 2026-09-27, a second time and for a better reason · **asks** a decision · **kind** measured · **into** `spec/console.md`, and how a world comes to exist

**This lane started writing the five rules and stopped four times.** Each time the notation could
not say the thing. **They are not four difficulties; they are one fact seen four ways.**

```
1  no rule has ever added a row to a relation carrying an `id`
   what rules add to: ark bin citizen energy extractor labor pioneer resource unit
   every one keyed by `where` with a quantity. territory, place and adjacency all
   carry ids and no rule touches them

2  no rule takes a number
   every input is `of:` a relation - place, resource, unit, founder, trait, relation
   `set-resource <territory> <resource> <extractors> <density>` needs two numbers

3  there is no `phase`
   so "before play begins" cannot be said, and no design command can be distinguished
   from a play command

4  there is no `biome`
   `P-541` brought it back to the data and nothing carried it into this schema
```

**A rule changes a world that exists. World-making makes identified things with literal values,
which is the one thing the notation does not do.**

## And your own artifacts already say how a world arrives

**Every world is stated, never built.** All fifty-four reviewed tests state theirs in `{given}`;
`scenario/main.4x` states its own. **Nothing anywhere has ever made a world by firing rules** - the
thing `N1` asks for has no precedent in the model because the model does not work that way.

## So `spec/console.md` contains the contradiction, and it is one sentence

> A game state changes only by a transition, which phase a game is in is part of its state, and
> **designing is therefore made of the same rules as playing.**

**That sentence is what `N1` was faithful to.** Against it stands everything above, and the fact
that no phase exists to be part of any state.

## Your three answers, 2026-09-27, and two of them already exist

**Id minting.** *Create-territory just gives the next number not used by any other territory.* **New,
small, and it strengthens your own `P-559`**: an id becomes unique by construction where a
caller-supplied one would be unique by constraint. **Wall 1 falls.**

**A core for regular and meta commands.** **It is built.** `{part of:end-turn is:upkeep seq:1}`
composes a rule from rules; `{argument part:P input:i value:v}` supplies a value to a part's input;
`src/schema.rs` refuses a cycle and a second parent, and `examples/tree.rs` prints the whole tree.
**`end-turn` is exactly this, with five parts.** So the mechanism you asked for is there and
`create planet` does not even need it - **code writing commands is your other framing and it needs
no composition at all**, because the code emits as many `add-territory` commands as the size calls
for.

**Phase near the interface.** **Wall 3 dissolves and it fits what the spec already says** - *the
game knows nothing of the interface*. Nothing in the engine has to know a phase.

## One wall is left and it is the smallest

**An argument carries a word and never a number.** Measured over every `{argument}` in
`spec/data/rules.4x`: `ark bearing citizen extractor gathering hungry laboring moving unit
working` - ten values, all names of relations or traits. **`set-resource` needs a density.**

**So the extension is one mechanism widened rather than a new one invented**, which is a much
smaller thing than this item claimed an hour ago.

## The decision

```
N5   the notation grows two things and loses one worry: an id minted as the next
     unused, an argument that may be a number, and a `biome` relation. Phase goes
     to the interface. Then `create planet` is code writing commands, which is
     your macro framing, and the commands it writes exist
N2   a world is produced rather than played into being - `generate-planet` emits
     stated rows, design commands stop existing, and `spec/console.md`'s
     "designing is made of the same rules as playing" changes
```

**This lane now says `N5`, having argued `N2` an hour ago on a count of four that is a count of
two.** It keeps the sentence you already wrote, it keeps designing and playing the same thing, and
**every world a player makes is then a history that replays** - which is the property `N2` would
have had to give up.

**What is not in question**: the ten mechanical renames, and that the old model goes.
