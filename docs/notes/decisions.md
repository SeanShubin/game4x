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

### P-407 - Five traits are declared stored and never vary, and the dump writes four of them nowhere

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, when counting turned one case into five - **kind** entailed, from the code lane's `C-92` - **asks** a decision - **into** `spec/console.md` -> The language, and `releases/first-release.md` -> Traits

**`C-92` found this about `force` and it is not about `force`.** Counted over the release's *Traits*
table against `scenario/expected/play.4x`: **nineteen traits are declared `stored`, and eight appear
in no description.**

| Declared stored, written nowhere              | Why it is absent                                                              |
| --------------------------------------------- | ----------------------------------------------------------------------------- |
| `fuel`, `upkeep`, `keeps`, `movable`, `force` | **one number per kind**, the same for every thing of that kind                |
| `moving`, `laboring`, `working`, `bearing`    | the code has not followed `P-411` yet - **not a defect**                      |
| `kind`                                        | it is the head of a description rather than a field in one - **not a defect** |

**So the pattern is five traits, not one**, and `force` is only the visible case because
`{garrison force:0}` writes it while `{citizen}` does not.

**`spec/console.md` says a description is a kind and every stored trait that thing has, and that no
trait may be left out.** Four of the five are left out of every description in the file.

## The two ways

|       | The rule                                                                                                         | What the dump becomes                                                                                                                      |
| ----- | ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| **1** | **A trait that is the same for every thing of its kind is a fact about the kind**, and no description carries it | `{garrison force:0}` loses a word. **Everything else is already correct**                                                                  |
| **2** | **All five are facts about each thing** and every description carries them                                       | `{citizen}` becomes `{citizen bearing:1 force:1 laboring:1 upkeep:1}`; `{food}` gains `keeps:1`; an ark gains `force:2 fuel:2 movable:yes` |

**`S-58` already drew this line once**, for bounds: `catalog.md` says which belong to a kind and
which to each one. **This is the same line asked of traits.**

**The arithmetic is one-sided and the choice is still yours.** Under **1** one line loses one word.
Under **2** every citizen in twelve territories carries three fields that never differ, and a reader
deriving the dump by hand copies them out every time.

**Whichever you take, `force` stays a trait.** `P-414` has `muster` and `stand` produce *that
citizen's force* and *that unit's force*, which is an amount read from a trait and is what `P-376`
allows. **What this decides is only whether such a trait appears in a description.**

