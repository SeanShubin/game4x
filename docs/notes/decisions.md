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

### P-354 - Three decisions about how a thing says what it can do

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07, as three decisions
rather than one - **kind** recovered - **asks** a decision - **into**
`releases/first-release.md` -> *Traits* and *Recipes*

**Working in [do the sets cross-cut?](2026-09-07-do-the-sets-cross-cut.md)**, including why a
hierarchy is already refuted and why an ECS fits. **These are the three choices that survive it.**
They are independent: any answer to one works with any answer to the others.

---

**1. What says a recipe applies to a thing - the family it is in, or a trait it carries?**

|                |                                                                                                                                                       |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Today**      | the family. *Families* says `unit` is `ark, pioneer`; `move` consumes `1 unit`                                                                        |
| **Or**         | a trait. `move` consumes `1 thing movable`                                                                                                            |
| **Change**     | one row in *Traits*, and two cells in `move`                                                                                                          |
| **Legal now?** | **Yes, both.** *Kind is the kind or the family alone*, `thing` is a family, and *Traits are the constraints on it* - so no grammar changes either way |
| **Difference** | a family is per **kind**; a trait is per **thing**. Nothing in the release needs two things of one kind to differ                                     |

---

**2. Must a trait carry a number, or may it just be present?**

|                |                                                                                                                   |
| -------------- | ----------------------------------------------------------------------------------------------------------------- |
| **Today**      | it carries a number. `Thing.traits` is `BTreeMap<Trait, u32>`, and **`ready` already encodes *yes or no* as one** |
| **Or**         | a trait may appear with no value - `{ark movable}`                                                                |
| **Change**     | `Thing.traits` becomes trait to **optional** number, and the data file needs a form for a valueless trait         |
| **Difference** | notation. **No behaviour changes either way**                                                                     |

---

**3. When a thing moves, is it the same thing?**

|                |                                                                                                                                                                                                                |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Today**      | **no.** `move` **consumes** a unit at `$from` and **produces** one at `$to` - one thing destroyed, another made                                                                                                |
| **Or**         | **yes** - the thing relocates, carrying one less energy. Your formulation, and what an ECS does                                                                                                                |
| **Change**     | **a fifth role** beside `require`, `limit`, `consume` and `produce` - none of the four relocates anything - or a stated convention that consuming and producing one kind within one recipe preserves the thing |
| **Difference** | whether anything can refer to a **particular** unit across a move                                                                                                                                              |

**Not observable today**, which is worth knowing before spending a role on it: **no ark or pioneer
appears in the played state at all** - each is consumed in the turn it is made - and the only things
carrying an `id` in `scenario/expected/play.4x` are orbits. **It becomes observable the first time a
unit persists across a turn and is named.**

---

**The prices are not alike.** 1 and 2 are a row, some cells, and a convention. **3 adds a piece to
the recipe language**, which is the kind of thing your own earlier note said should be decided
deliberately rather than arrived at one step at a time.

