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

### P-442 - the notation says what a state file may contain, and six of the eight declare rather than describe

**to** sean - **status** open - **raised** 2026-09-12 - **kind** entailed, from the code lane's `C-97` - **asks** a decision - **into** `spec/console.md` -> The language

**The code lane declined to guess at this and it is right to.** `P-440` sends the game's data to
files in the notation, and the notation has a rule about what a file may say:

**`spec/console.md`**: *Every word in a data file is a kind, a trait, or one of a trait's values. A
file that uses any other word is wrong about the game rather than describing it.*

**That is a rule about a file which *uses* the vocabulary.** A state file says `{citizen bearing:1}`,
and every word in it is a kind, a trait or a value. **Six of the eight tables *declare* the
vocabulary instead** - what kinds there are, what a trait is of, what values it admits - and a
declaration is not a thing in a game state.

| Table                    | Declares or describes                               |
| ------------------------ | --------------------------------------------------- |
| **Kinds**                | declares what kinds exist                           |
| **Families**             | declares which kinds a family holds                 |
| **Traits**               | declares a trait, what it is of, and what it admits |
| **Where things are**     | declares what may contain what                      |
| **What bounds a kind**   | declares a bound                                    |
| **Recipes**              | declares rules                                      |
| **Biomes**               | **describes** - seven biomes and four numbers each  |
| **Units and structures** | **describes** - eight things and their numbers      |

**The last two need nothing new**, which is why the code lane says they are where to start. **The
other six have no form in the notation today.**

## The difficulty, stated rather than solved

**A declaration's first word names the thing being declared, and that word is not yet a kind.**
Writing `{kind name:citizen}` makes `kind` the opening word - so either `kind` is itself a kind, and
the vocabulary declares itself, or the rule above admits a second category of word.

**Neither is obviously wrong and this lane is not choosing.** Self-description is how a good many
data languages work and costs nothing but a reader's double-take; a second category is smaller to
state and leaves the notation with two kinds of file.

## What is not at stake

**Nothing is blocked.** The comparison check comes first, `Biomes` and `Units and structures` need no
new form, and the split across files and the loader are the code lane's. **This is the question that
arrives when it reaches the other six**, and it is filed now because it is a specification question
and they found it before this lane did.

**And `rule 7` is not in doubt.** *Relationships in prose, data in data files* **splits a table
rather than moving it** - `Kinds`'s *What it is* column is prose and stays prose, and what goes to a
file is the name. **So the six may need less of a form than the table suggests**, which is worth
knowing before the form is designed.

