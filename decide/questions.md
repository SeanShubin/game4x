# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-571 - the generated regression tests need a home and a rule saying they are not the specification

**to** sean · **status** open · **raised** 2026-09-26 · **asks** a decision · **kind** entailed · **into** where the generated tests live, and `spec/README.md` rule 3

**You asked which lane owns what. Almost all of it is the code lane's** - `scenario/main.4x`, the
generator, the tests, the expectations and the check are all in its column. **Two things are
not**, and both are yours.

## The size, so *excessive* has a number

```
34 commands in main.4x's {when} block -> 34 tests
10 distinct command names
21 of the 34 are two commands repeated - work 13 times, toil 8
```

**So the clutter you want to avoid is 34 items, not hundreds**, and half of it is two commands
seen in different states.

## The first decision - where they live, and one file or many

```
G1  one file per command name       10 files, work.4x holding its 13 in order.
                                    Browsing by mechanic; a change to work touches one file
G2  one file per command            34 files, named for the command and its turn.
                                    A change shows as one file in a diff and nothing else moves
G3  one file                        everything in order, read top to bottom like the scenario
```

**`G2` makes a diff say the most** - one changed file names the one command that moved - and it
is the most files. **`G1` groups the way you would browse.** `G3` is the scenario again with
expectations attached.

## The second - a rule, because these are not `spec/tests/`

**`CLAUDE.md` already has the sentence**: *a check that pins the present state cannot report a
gap against what should be... it says nothing about what it ought to do.* **These say what the
game does; `reviewed/` says what it must.**

**The risk is a later reader taking a generated test for a statement of intent**, which
`spec/README.md` rule 3 exists to prevent and does not currently cover, because nothing like this
existed when it was written. **Whether that needs a sentence in rule 3 is yours**; this lane
thinks it does and will draft it once you have said where they live, since the sentence names the
place.

## And one collision to decide before it is built

**Deleting a file means opposite things in your two systems.**

```
reviewed/<name>.4x   deleted by `u` in the review application = I have NOT read this
an expectation       deleted by you                           = I approve what it does now
```

**Same gesture, opposite meaning.** Nothing breaks today because they are far apart, but **a tool
that swept both would read approval as its negation.** Worth either keeping them visibly
separate, or giving the regression pattern a different gesture - and this lane will not choose
which, because the pattern is yours and you use it elsewhere.
