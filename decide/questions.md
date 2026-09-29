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

## What lands once you pick

**The rule into `spec/console.md`**, and the commands it disqualifies come out of the same section in
the same commit, with the count asserted. **`create-planet` needs no promotion either way**;
`spec/console.md` already names it.
