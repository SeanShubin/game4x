# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-591 - A command that can be expressed as a row is not needed, and what saves `create-planet` is open

**to** sean · **status** open · **raised** 2026-09-29 · **asks** a decision · **kind** recovered · **from** `C-176`

**Your rule, 2026-09-29**: *we don't need a command that can be expressed as a row, but isn't
`create-planet` still needed because it generates multiple rows?* **The first half is a rule and the
second half is a question**, so this lane is not writing either until you say which criterion saves
it.

**`S-216` is superseded rather than contradicted.** On 2026-09-27 you said *we can keep set-biome and
set-resource **for now***, and this is two days later. The word *for now* is doing the work.

## What each criterion decides, measured over the six

```
                     rows it writes                              by count   by impossibility
create-planet        96 at tiny-12, 816 at huge-92               stays      stays
set-resource         a deposit and a capacity - two rows         stays      goes
set-biome            one cell on a row that exists               goes       goes
add-ark-orbit        one row                                     goes       goes
add-pioneer-orbit    one row                                     goes       goes
set-force            none - `P-522` cut force from the release   goes       goes
```

**The two criteria disagree about exactly one command**, and `set-resource` is it.

- **By count**: a command that writes more than one row is worth keeping. **`set-resource` stays.**
- **By impossibility**: what saves a command is that nobody could write the rows correctly by hand.
  `create-planet` runs a tessellation and derives a biome per territory from a terrain field, and
  **the adjacency of a ninety-two-face Goldberg polyhedron is not tedious to write, it is not
  possible to write correctly.** `set-resource` writes two rows anybody could type, so it **goes**

**The second is the code lane's sharpening of your sentence and not your words**, which is why it is
offered rather than promoted. **This lane finds it the better line** - the count is a proxy and
impossibility is the thing the proxy is for - and will not act on that.

## Your third framing, drawn to attention rather than offered as an answer

**Sean, 2026-09-29**: *Create planet is essentially a macro expansion, which results in rows. Set
resource could also be expressed as a nested structure within a territory, which when normalized
would result in rows.*

**Both criteria above take *command* as the category and this does not.** It says the two survivors
are the same kind of thing - a compact form that expands into rows - **differing in what does the
expanding**, which is a question about the notation rather than about the command list.

**The notation already carries the shape.** `spec/console.md`: *a command and a description of game
state are written in the same form, and both carry a tree*, and *the language carries the tree
whether or not a command uses one today.* **What is missing is anything that normalizes a tree into
rows** - measured: nothing in `spec/` says a tree is normalized and no rule reads one.

**So the line this suggests is normalization against computation.** A nested structure normalizes to
rows mechanically; a tessellation computes them. **`set-resource` becomes notation rather than a
command and `create-planet` stays code** - the same answer *impossibility* gives, reached by saying
what the difference is rather than how much work it saves.

**It is not costed here.** Normalizing a tree is a thing the notation cannot do today, so this route
disqualifies `set-resource` by promising a replacement rather than by deleting it - and until that
exists, removing the command removes the capability. **That is the one way it differs from
impossibility in practice**, and it is why it is drawn to attention rather than added as a third
column.

## What lands once you pick

**The rule into `spec/console.md`**, and the commands it disqualifies come out of the same section in
the same commit, with the count asserted. **`create-planet` needs no promotion either way**;
`spec/console.md` already names it.

