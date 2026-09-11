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

### P-390 - An allowance, and whether it is a trait pair or a kind time supplies

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, twice - Sean took *allowance*, asked for examples, and asked whether a count makes things fungible - **kind** entailed, from `S-93` built - **asks** a decision - **into** `releases/first-release.md` -> Kinds, Traits, Recipes

***Allowance* is taken** and is used throughout below.

## What the release already has, which nothing states

**The allowance and the thing it yields are two different objects, and the release already models
them separately.** `fertility` is a **kind** - a token `bear` makes and `breed` spends. `spent` is a
**trait** of a citizen. They are not two names for one idea.

| The thing         | Its allowance, today a trait pair | Spending it yields            |
| ----------------- | --------------------------------- | ----------------------------- |
| a citizen         | `fertile` / `spent`               | one `fertility`               |
| a citizen         | `ready` / `not ready`             | one `labor`                   |
| an extractor      | `ready` / `not ready`             | nothing - it lets `work` fire |
| an ark, a pioneer | `ready` / `not ready`             | nothing - it lets `move` fire |

**So `create labor` and `bear` are the same rule twice** - spend an allowance, get a token - and
`refresh` and `renew` are the same rule twice, refilling one from time. **Four allowances, two
habits of naming, and no statement anywhere that they are one thing.**

## Naming the resource, which is what you asked for an example of

**The allowance stops being a trait and becomes a kind the thing holds**, drawn from time the way
metal is drawn from the planet. `ready` and `spent` disappear; what replaces them is containment,
which the specification already has: *what a thing may contain is a maximum per kind.*

| Today                                                                                         | Naming the resource                                          |
| --------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `create labor`: consume 1 citizen `[ready]`, produce 1 citizen `[not ready]`, produce 1 labor | consume **1 readiness** in the citizen, produce 1 labor      |
| `bear`: consume 1 citizen `[fertile]`, produce 1 citizen `[spent]`, produce 1 fertility       | consume **1 fecundity** in the citizen, produce 1 fertility  |
| `work`: consume 1 extractor `[ready]`, produce 1 extractor `[not ready]`, ...                 | consume **1 readiness** in the extractor, ...                |
| `refresh`: consume 1 thing `[not ready]`, produce 1 thing `[ready]`                           | produce **1 readiness** into whatever can hold one, **soft** |
| `renew`: consume 1 citizen `[spent]`, produce 1 citizen `[fertile]`                           | produce **1 fecundity** into a citizen, **soft**             |

**A citizen's capacity for readiness is 1**, the way a store's capacity for food is 10 - the same
sentence in `spec/logistics.md`, not a new one. **Refilling is a soft produce into a bounded
container**, which is the shape `P-386` and `P-387` just gave the game. **Nothing here is a new
mechanism.** Two rows leave *Traits*, two kinds join *Kinds*, and `refresh` and `renew` become one
rule shape.

**And it explains a bug the check hit.** `nogain.rs` found that counting *not ready* made
`create labor` read as pure gain. Under this reading `not ready` is **room**, not contents - `P-374`
says *what is stored is the room left* - so it was counting the wrong half of a containment.

## Your fungibility question, which is about a design I did not mean

**You are right that it would be untenable.** A citizen with *two actions* to spend on labor or
fertility or one of each is a **shared pool**, and it makes two unrelated abilities interchangeable
by accident.

**What I meant is a count per allowance and they never mix.** A citizen holds one readiness and one
fecundity; they are different kinds, and no rule turns one into the other. **Saying a citizen's
capacity for readiness is 2 makes it work twice a turn and changes nothing about its fecundity.**

**So *make it a count* is not a third option.** It is what naming the resource gives you for free,
because containment is already *a maximum per kind*. **Two options, not three** - and the one you
called a last resort is the one that cannot express a count at all, since a trait pair is yes or no.

## The two that remain

- **Name the pair.** A template declares full and empty per allowance. **Smallest change, and it
  keeps four allowances as four vocabularies.** A count stays inexpressible
- **Name the resource.** One mechanism, no new words beyond the kinds themselves, and a count comes
  with it. **The larger change, and it is the unification you said you prefer**

## What this lane cannot tell you, and would check before you commit

**Whether a held allowance takes up room in the territory.** `spec/logistics.md` says *a thing that
contains things takes up capacity in whatever contains it, so capacity is not conserved*. A readiness
inside a citizen inside a territory may therefore need the territory to declare room for it, which
would be a real cost and is not obviously intended. **One sentence decides it and this lane has not
found the sentence.** Say the word and it is the next thing checked.
