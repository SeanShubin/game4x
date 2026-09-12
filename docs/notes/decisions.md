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

### P-445 - a cell holding several values, written four ways

**to** sean - **status** open - **raised** 2026-09-12 - **rewritten** 2026-09-12, when the fourth option was found and then left buried - **kind** entailed, from the code lane's `C-98` - **asks** a decision - **into** `spec/console.md` -> The language, and `spec/data/`

**`unit` has two members and the notation gives a key one value.** Here is the *Families* table
written four ways. **Each shows the three families that are lists** - `unit`, `resource`, `place` -
and **`thing` is held back to a table of its own below**, because it is eighteen lines under A and
one under C, and side by side that is the whole decision.


**`D` was added after you read the first three and is now beside them.** It was buried under *what
each costs* for an hour, so the file said three and this lane said four.


## Option A - a line per pair

```
{family name:unit member:ark}
{family name:unit member:pioneer}
{family name:resource member:food}
{family name:resource member:metal}
{family name:resource member:energy}
{family name:place member:territory}
{family name:place member:orbit}
```

## Option B - the values joined into one

```
{family name:unit members:ark-pioneer}
{family name:resource members:food-metal-energy}
{family name:place members:territory-orbit}
```

## Option C - a rule instead of a list

```
{family name:unit rule:movable}
{family name:resource rule:extracted}
{family name:place rule:holds-things}
```

## Option D - a list where it is a list, a rule where it is a rule

```
{family name:unit member:ark}
{family name:unit member:pioneer}
{family name:resource member:food}
{family name:resource member:metal}
{family name:resource member:energy}
{family name:place member:territory}
{family name:place member:orbit}
{family name:thing rule:every-kind}
```

**`unit` is a list and `thing` is a rule, and they are different sorts of thing.** A kind added
tomorrow is a `thing` by definition and is not a `unit` unless someone says so. **D says that;
A and C each say only half of it.**

**Whether that is unification or the exception `P-428` warns against is yours to judge**, and it is
the reason this is a decision rather than words to approve. The form is uniform - a family
declaration - and the trait differs. **Three families need no rule and one needs no list**, so
neither A nor C is wrong about all four.

**And the fourth family is what separates them.** `thing` is written *every kind above* - a rule
about the table, not a list:

|       | `thing` under each                                                                       |
| ----- | ---------------------------------------------------------------------------------------- |
| **A** | **eighteen lines**, one per kind, repeating `kinds.4x` entirely                          |
| **B** | one line, `members:citizen-garrison-extractor-yard-store-ark-...`, eighteen words joined |
| **C** | one line, `{family name:thing rule:every-kind}`                                          |
| **D** | one line, the same as C - D and C differ on the other three, never on this one           |

## The same choice on the other two cells

**A trait's `Of`.** `strength` is of citizen, garrison, ark and pioneer:

```
A   {trait name:strength of:citizen}  and three more lines
B   {trait name:strength of:citizen-garrison-ark-pioneer}
C   {trait name:strength of:has-strength}   - a family, which is option A one level up
D   {trait name:strength of:citizen}  and three more lines - a list, so D writes it as one
```


**A trait's `Values`.** `biome` admits six:

```
A   {trait name:biome admits:ice}  and five more lines
B   {trait name:biome admits:ice-desert-grassland-jungle-mountain-ocean}
C   {trait name:biome admits:biome}   - the values are a kind's members
D   {trait name:biome admits:ice}  and five more lines - a list, so D writes it as one
```

**So `D` is `A` everywhere except `thing`.** Those are the only two cells in the eight tables where
the several values are a **rule** rather than an enumeration, and `thing` is the only one of them
this proposal has found. **If there turns out to be a second, `D` is the option that already
accommodates it and `A` is the one that quietly writes it out longhand.**


## What each costs, revised 2026-09-12 when Sean read A and B as a matter of style

**A and B are not the same size, and this proposal presented them as though they were.**

**B breaks the rule you chose to keep two days ago.** *Every word in a data file is a kind, a trait,
or one of a trait's values.* The words in `{family name:unit members:ark-pioneer}` are `family`,
`name`, `unit`, `members` and **`ark-pioneer`** - **which is not a kind, not a trait, and not
sensibly a value**, because making it one means declaring every combination of members that could
ever be written. Under A the last word is `ark`, which is a kind.

**So B costs the property that a file can be checked word by word against the vocabulary.** A reader
would first have to know which traits hold lists, and split those and not the others. **That is not
visual style; it is a second category of value arriving by the back door** - which is precisely what
option B of `P-442` was and you declined it.

**A's cost is larger than *eighteen lines*, and it is the one that has already happened.** `thing` is
not a list. It is written *every kind above*, and `C-71` is what that cost: both joins split on
commas, missed it, and **the world's five recipes named nothing at all** - a computation that ran and
was wrong. **Writing `thing` as eighteen lines is the same copy in a new form**, and it drifts the
first time a kind is added and a line is not.

**C is what you said it is**, and there is a sharper way to put its advantage than *explicitly
declaring patterns*: **a derived thing cannot disagree with what it derives from.** That is your own
move in `spec/logistics.md` - *nothing records the total, so nothing can disagree with it*. `thing`
under C cannot drift because it is not stored.

**And C's plumbing is real but smaller than it looks.** The rule names have to be declared values of
a `rule` trait, which keeps the word rule intact; whatever reads the file has to know what
`every-kind` means, which is one function.

## Why this is yours and not the code lane's

**They refused to pick and said why**, which is `C-49`'s rule working: *a shape invented here and
transcribed into the specification is the promotion by the wrong lane that item was about.*

**Nothing is blocked.** `Biomes` and `Units and structures` hold one value per cell and need no
answer to this - they are next either way, and they raise a different question, because they declare
**facts about kinds** rather than vocabulary and `P-443` settled only the vocabulary case.

