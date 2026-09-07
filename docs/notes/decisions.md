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

### P-323 - Three names the code lane needs before it can rewrite the commands file

**to** sean - **status** open - **raised** 2026-09-06 - **kind** gap - **asks** a decision -
**into** `spec/console.md` -> Commands

**You want a new `scenario/commands/play.4x` and `scenario/expected/play.4x` to review.** The
expected data needs no decision - `P-322` landed and the code lane regenerates it. **The commands
file cannot be rewritten yet**, and this is everything that stops it.

**`P-321` gave the form and not the names.** *A command is written `{name field:value ...}`. Its name
is the words that open it and its arguments are named.* Most of the rewrite falls out of that and of
*a command names a recipe and binds what that recipe leaves open*: the recipe supplies the name, and
the `$` placeholders supply the fields - `$where`, `$from`, `$to`, `$resource`, `$name`. **Three
things do not fall out.**

**1. What the place field is called.** The recipes say `$where`. The dump says `territory`. So
`build extractor 1 metal` becomes either `{build extractor where:1 resource:metal}` or `{build
extractor territory:1 resource:metal}`. **`spec/console.md` already says a field named for a kind is
a reference to one**, which argues for `territory` - and the recipes are written in `$where`, which
argues for the other. **Whichever you pick, it is one word in every command in the file.**

**2. Whether a command may carry a count, and what it is called.** `work 1 extractor 1 food` opens
with a repetition count. **A count is neither a place nor a `$` placeholder**, so `spec/console.md`
does not say a command may have one. Two ways: a field, say `{work extractor count:1 territory:1
resource:food}`; or **no count at all**, and firing a recipe four times is four commands.

**Measured, because it decides how much this costs**: every count in the file is **1**, across 46
`work` commands and every `create labor` and `build`. **The count is a form the grammar allows and
the scenario has never used.** Dropping it changes no line of the scenario.

**3. `land ark` fires the recipe `deploy ark`.** Every other player command is named for the recipe
it fires - `build extractor`, `build store`, `build yard`, `create labor`, `found by land`. **This
one alone differs**, and `spec/console.md` says there is one command for each recipe the player may
fire. So either the command becomes `{deploy ark ...}` and the file loses the word *land*, or
`land ark` stays as the one command whose name is not its recipe's.

**Two commands are not recipes at all and are unaffected**: `create planet` and `add ark orbit` are
design commands, which `P-217` already says are not recipes. `end turn` fires six world recipes and
is not one of them either.

**No recommendation on the third.** The first two I would take as `territory` and no count - the
first because the dump already reads that way and you review the dump, the second because it removes
a form nothing uses. **The third is a naming choice about a word you chose**, and I have no ground to
prefer either.
