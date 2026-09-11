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

### P-390 - What to call a thing time refills, given there will be a third

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, when Sean said more than one resource will come from time - **kind** entailed, from `S-93` built - **asks** a decision - **into** `releases/first-release.md` -> Traits, and `spec/invariants.md` -> Nothing comes back round with more

**Your invariant is decided mechanically now** - `reports/nogain.md`, 43 rules, a weighting solved for
rather than declared, nothing gaining. **The arithmetic was verified here rather than relayed**: the
tightest rule is `work (food x6)`, netting +6 food at 4 against a draw on the planet at 20 and a labor
and a readiness at 2 each. Exactly zero.

**It rests on a list written in Rust.** `nogain.rs` declares two trait pairs - `ready`/`not ready`
and `fertile`/`spent` - and **that list decides which rules draw on time**, and so whether a weighting
exists at all. A third one is a code edit today.

## The two that exist are named two different ways

| Trait   | Named for           | Its other value | In the *Traits* table                               |
| ------- | ------------------- | --------------- | --------------------------------------------------- |
| `ready` | the **full** state  | `not ready`     | yes or no                                           |
| `spent` | the **empty** state | `fertile`       | yes or no - **and `fertile` appears nowhere in it** |

**A third has to copy one habit or the other and nothing says which.** That is the cost being paid
now, before any third exists.

## One word is not available

**`capacity` is containment's.** *Total capacity*, *used capacity*, *a capacity of 1*, *room* - nine
uses in two files. Using it for a per-turn allowance would give one word two unrelated meanings in one
release. **The check already collides with this**, calling these places *a capacity, spent by acting*
in a report that also prints containment capacities.

## Three ways, and they disagree about what changes

- **Name the pair.** A template declares, per allowance, the word for full and the word for empty;
  `ready`/`not ready` and `fertile`/`spent` become two rows of it. **Vocabulary changes, the model
  does not.** A third is a row
- **Name the resource.** Time offers resources the way the planet does, and a thing holding one is
  holding that resource. The states stop being named at all - full is *holding one*, empty is
  *holding none*. **The model changes and the vocabulary shrinks**
- **Make it a count.** A thing holds *n* of an allowance and acting spends one, so `ready` is `n = 1`
  and `not ready` is `n = 0`. **A unit that acts twice in a turn becomes expressible with no new
  rule** - and `refresh` becomes a soft line refilling up to the allowance, which `P-386` and `P-387`
  just made a shape the game has

**The first is smallest and the third is the one that buys something.** They are not exclusive: a
count needs a name, and the template is where an allowance would be declared either way.

## If it is a template, this is the shape

> | Allowance   | Full      | Empty       | Of                | Refilled |
> | ----------- | --------- | ----------- | ----------------- | -------- |
> | `readiness` | `ready`   | `not ready` | whatever readies  | 1 a turn |
> | `fertility` | `fertile` | `spent`     | a citizen         | 1 a turn |

**Offered as a shape rather than as words to approve**, because the column heading is the question
below and the table cannot be final until it is answered.

## The word itself

**Candidates, and what each costs.** None of them collides with anything in the repository - checked
rather than assumed.

| Word          | For it                                                                                 | Against it                                                                                     |
| ------------- | -------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| **allowance** | says *per turn* without being told; a plain English word a player would read correctly | faintly bureaucratic                                                                           |
| **faculty**   | precise - a power to act, held and spent; reads well as *a thing's faculties*          | formal, and unfamiliar as a game term                                                          |
| **charge**    | short, and *charged*/`discharged` is a ready-made pair                                 | reads as energy, which is a resource this game already has                                     |
| **vigor**     | fictional flavour, no technical baggage                                                | says nothing about where it comes from or that it is per-turn                                  |
| **readiness** | already `P-388`'s word, so nothing new is introduced                                   | it is also the name of **one** of them, so the general and the particular become the same word |

**This lane would take *allowance*, and that is a preference rather than a finding.** It is the only
candidate whose everyday meaning already carries *one per turn*, which is the half a reader has to be
told otherwise. **`readiness` is the one to avoid** for the reason `capacity` is: it would name both
the family and a member.
