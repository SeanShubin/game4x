# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-328 - A command's name is several words or one dashed word, and `spec/console.md` says both

**to** sean - **status** open - **raised** 2026-09-06 - **kind** contradiction - **asks** a decision
- **into** `spec/console.md` -> The language

**You found it by reading the file the code lane produced.** It blocks
`scenario/commands/play.4x`, which is the file you are waiting for, so it leads.

**`spec/console.md`, line 23:** *A name is one word. **Where it needs more than one, the words are
joined with dashes*** - `in-play`, not `"in play"`.

**`spec/console.md`, twenty-four lines later:** *A command is written `{name field:value ...}`. Its
**name is the words that open it***. And, from the same promotion: *a command is named for the
recipe it fires.*

**Ten recipes have names of more than one word** - `deploy ark`, `build extractor`, `build store`,
`build yard`, `create labor`, `found by land`, `produce ark`, `produce pioneer`. So a command's name
is either one token or several, and **the file says both.**

- **Dashed.** `{deploy-ark territory:1}`, `{found-by-land territory:2}`. The name is one token like
  every other name in every data file, and **there is no question of where the name ends**
- **Several words.** `{deploy ark territory:1}`, `{found by land territory:2}`. Reads closer to
  English, and needs a rule saying where the name stops - which nothing states, and which
  `found by land` makes three words wide

**My recommendation is dashed.** Line 23 is the older rule and the broader one: it governs **every
name in every data file**, and the command sentence is the newer one that did not mention it.
Dashing also removes the parsing question rather than answering it.

**Every description in the expected data already opens with one token** - `{deposit ...}`,
`{garrison ...}` - because no kind has a multi-word name. **Commands are the first place a
multi-word name is written**, which is why this surfaced now rather than earlier.

**One thing this does not touch.** Whether `deploy ark` is *a `deploy` command about an ark* is
answered either way: it is one name, because a command is named for the recipe it fires and
`deploy ark` is one recipe. **The question is only how that one name is spelled.**
