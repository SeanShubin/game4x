# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-610 - Two ways the page can write, and they want different reach on your token

**to** sean · **status** open · **raised** 2026-10-01 · **asks** a decision · **kind** recovered · **into** `releases/marking-state.md` -> `E-4` · **source** the code lane reaching the hosting half and declining to choose with your credential

**The page can write two ways and the gestures are not alike.** Marking something is a file write; a
regeneration needs the generator run, which a browser cannot do.

```
a verdict        {verdict case:... state:denied}  - a row in a file
                 the page commits it through the API           needs  Contents: write
a regeneration   {regenerate case:...} and then the generator
                 the page dispatches review.yml, which runs it  needs  Actions: write
```

**So it is not a choice between two designs; it is a question about your token.** A regeneration
cannot be a direct commit - the browser has no cargo - and a verdict need not be a dispatch.

## The three shapes

**One token with both permissions.** Everything works from the page, and the token can commit to
`game4x` **and** start any workflow in it. **Simplest, widest reach.**

**One token with `Contents: write` only.** The page marks; you start a regeneration yourself from the
Actions tab. **The page can write records and cannot start anything**, and the gesture you lose is
one you make rarely.

**Two tokens.** The marking page holds the narrow one; the regeneration is behind the wider one you
paste when you want it. **Least reach at any moment, most to carry.**

## What decides it, and it is not convenience

**The token sits in `localStorage` on `seanshubin.github.io`**, which also serves the game and the
reports - so any script that reaches that origin can read it. **Adding `Actions: write` widens what a
read token can do** from writing files to starting workflows.

**The code lane has no preference and says so** - *it is his credential*. **Neither do I**, beyond
noting that the narrow token covers four of the five gestures and the fifth is the rare one.

## What this does not ask

**Not whether the owner gate stays.** `E-5` keeps it either way: a dispatch is gated on
`github.actor == github.repository_owner`, and a direct commit is signed by your token, which no lane
has.
### P-609 - `rules/`'s sixteen cases are in neither half of what you said about the suites

**to** sean · **status** open · **raised** 2026-10-01 · **asks** a decision · **kind** recovered · **into** `releases/marking-state.md` -> `E-4` · **source** the code lane building them markable by default and saying it was a default

**You named two suites and there are four.** Your instruction: *let's show them, but these are
informational only, no vetting capability need be implemented* - asked about `types/` and
`primitives/`, which are 53 and 60, the 113 we have both been quoting.

```
scenario     36    markable, and never in doubt
rules        16    in neither half of your sentence
types        53    shown only
primitives   60    shown only
```

**36 + 113 is 149 and there are 165.** The page offers a control on `rules/` because nothing singles
it out and `spec/README.md` rule 3 says *no suite is privileged* - **which the code lane flagged as a
default rather than a reading of anything.**

## What the suite is, so you are not deciding blind

**`regression/rules/` holds a case per rule** - sixteen rules, sixteen cases, one firing each. It is
the suite that answers *does this rule still do what it did*, where `scenario/` answers *does a game
still play*.

**So it is closer to a unit test than the other two are**, which is an argument for marking it; and it
is generated and observes rather than decides, which is an argument for the informational half. **Both
readings are available and that is why this is a question.**

## The choice

**Markable**, and the page offers a control on 52 cases rather than 36. **The code lane has already
built this**, so saying so costs nothing.

**Shown only**, and the informational half is *everything but `scenario/`* - 129 cases listed and
linked with nothing to press. **One constant entry**, their words.

## What this is not

**Not a question about rule 3.** *No suite is privileged* says what may be **recorded** -
`reviewed/cases.4x` takes a verdict for any case either way. **This asks only what the page offers**,
which `E-4` already distinguishes because you asked for exactly that distinction.
