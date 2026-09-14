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

### P-497 - `spec/data/` normalized, and two things the normalizing found

**to** sean · **status** open · **raised** 2026-09-13 · **kind** invented · **shape** an instruction · **asks** a decision · **into** `spec/data/`, and `spec/README.md` -> rule 7

**You chose `F4`**: normalize `spec/data/` as a whole rather than folding one table at a time.
**This is the schema, derived from what is on disk rather than sketched**, with every count
re-derivable by the script in the working notes.

**It asks a decision rather than approval for one reason**: the relation *names* are this lane's
invention, and so is the decision to give a recipe block an identity. The shapes are forced by the
data; the words are not.

## What the relations are, with a real row of each

```
{carries      kind:pioneer  trait:fuel}                       45 rows
{member       kind:ark      family:unit}                       7 rows
{limit        container:territory contained:garrison n:1}      5 rows
{block        id:refresh-1  recipe:refresh  owner:world}      36 rows
{line         block:refresh-1 seq:1 role:put qty: kind:unit place:}   93 rows
{constraint   block:refresh-1 seq:1 trait:moving compare:at-its-maximum}   26 rows
{for          block:build-extractor-1 seq:4 kind:food}         7 rows
```

**`carries` is the one that answers your own test.** Today `ark` and `pioneer` carry **byte-identical
seven-trait lists** on two lines of `kinds.4x`, and `binding` and `metal-in-it` are each written six
times. As rows, every `(kind, trait)` fact is stated once and the duplication is visible as what it
is rather than as two long lines that look different.

## One - a recipe name is not a key, and nothing said so

**36 blocks over 26 distinct names.** `refresh` opens **six** blocks, `discard` **five**, `stow`
**two**. So `recipe -> rows` is not a function and a row cannot be addressed by its recipe name
alone.

**The release has always worked this way and no artifact named it** until
`docs/designing-rules.md` wrote *ground from 32 blocks of recipe rows* - a word that exists in a
report and nowhere in the specification. **Normalizing is what turned an accident of layout into a
thing with a name**, and the id above is the part this lane invented rather than found.

## Two - the Traits column holds three different relations

**Counted: 34 filled cells, of which 26 decompose into `<trait> <comparison>` and eight do not.**
The eight are not malformed; they are **different kinds of fact sharing a column**:

| The cell                                         | What it actually is                                                                         |
| ------------------------------------------------ | ------------------------------------------------------------------------------------------- |
| `food`, `metal`, `` `$resource` `` - **7 cells** | which resource the extractor or store is *for* - hence `for`, and not a `constraint` at all |
| `joined to `$from` by an edge the unit crosses`  | a relationship between two places - prose under rule 7                                      |
| `whose upkeep is unpaid`                         | a trait test in words rather than in the form the other 26 use                              |

**This is the finding, and no reader of the table could have had it.** One column, three relations,
and they read alike because a markdown cell has no type. **`free energy at least 1` and `food` sit
in the same column and are not the same kind of thing at all.**

## What it does not settle, and these are the decision

- **The names are settled, 2026-09-13.** `member`, `limit`, `constraint` and `for` replace this
  lane's first four; `block`, `line` and `carries` stand. **That each is a kind was never a
  question** - `P-443` leaves no other option for the first word of a row, and this item previously
  wrote that consequence as though it were a choice
- **`carries` was queried and kept, on the specification's own usage.** `spec/logistics.md` uses
  *contain* and *hold* for contents seventeen times and *carries* twice, both for traits - *per kind
  carrying a particular value of a trait*, *a place carries an `id`*. `spec/console.md` says *one
  kind and carries every trait of that thing*, and `spec/planet.md` has a section called *What a
  territory carries*. **The split is already there and deliberate: things hold contents, kinds carry
  traits.** This lane's worry was that `carries` would collide with cargo in a game about
  containment; measured, it does not, because the spec never uses it that way
- **Whether a block's id is written or derived.** `refresh-1` is a name nobody has chosen. The
  alternative is position, which is what the table does today and is what normalizing is removing
- **`whose upkeep is unpaid`** wants to become `unpaid at least 1` and that is a rule's wording,
  not a transcription

## What this is, said plainly

**This is `C-114`'s restructuring approached from the data side.** `spec/invariants.md` says every
recipe is data rather than code, and 93 rows of `line` is what that sentence has always meant.
**The engine reading them is the other half and is not in this item.**

## How to tell it was carried out

**Every count above is re-derivable, and that is the check.** When this lands, the files in
`spec/data/` hold **45** `carries` rows, **7** `member`, **5** `limit`, **36** `block`, **93**
`line`, **26** `constraint` and **7** `for` - and each one equals what the same relation
derives from `releases/first-release.md` and today's `kinds.4x`. **A migration that loses a row
fails a count rather than being noticed later.**

**And the stronger half: no fact is stated twice.** `(kind, trait)` appears once for each of the 45
pairs, where `kinds.4x` today writes `binding` six times and `metal-in-it` six times. **That is the
property you said you review for**, and it is the one a count of rows does not show - so it is
asserted separately, as a uniqueness check on every relation's key.
*Nothing is open.*
