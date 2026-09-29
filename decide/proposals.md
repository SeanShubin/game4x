# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-586 - `R-11` reworded: the intent stands and the `.txt` twin comes out

**to** sean · **status** open · **raised** 2026-09-28 · **asks** approval · **kind** recovered · **shape** text · **into** `releases/first-release.md` -> R-11

**You said keep it reworded, and only one sentence of it is about a mechanism.** The clause asks
that you can reach every file the engine reads and that following the link **reads** it - and it
then names the `.txt` twin, which existed because GitHub Pages served `.4x` as
`application/octet-stream`. **`09f628d7` renders every `.4x` as a page instead**, which satisfies
the same intent and can say *generated, not canonical* on its own face rather than in a label.

**Offered as the replacement for the whole *Vetted when* bullet**:

> - **Vetted when** - from the reports' index I can reach **every file the engine reads as input**,
>   in as many clicks as it takes to reach any other view, without knowing the paths beforehand, and
>   **reading it is what following the link does** - it renders in the browser rather than
>   downloading. A rendering rather than the file itself **says on the page that it is generated and
>   not canonical**, and says which file it came from. **Nothing the engine reads is missing from
>   that index**, which is checked by listing the inputs rather than by anybody remembering to add
>   one.

## What changed and what did not

```
was                                    now
reports/index.html, by name            the reports' index, by role
each to a .txt twin                    a rendering, whatever form it takes
nineteen links over three directories  nothing the engine reads is missing
```

**The last line is the one that does the work and it is untouched.** It is the clause that caught
the code lane listing half the inputs and remembering the other half, in the very commit that
claimed it - so the count went and the check stayed.

**The nineteen is gone on purpose.** `P-557` had already made it twenty-and-twelve into
nineteen-and-eleven by deleting one file, and the number moved again when the old reports went.
**A count that changes whenever the data does is not what you are vetting.**

