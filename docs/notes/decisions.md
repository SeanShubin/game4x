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

### P-432 - `force` is a kind in three recipe rows and *Kinds* declares seventeen without it

**to** sean - **status** open - **raised** 2026-09-11 - **rewritten** 2026-09-11, when you asked for the options - **kind** entailed, from the code lane's `C-93` - **asks** a decision - **into** `releases/first-release.md` -> Kinds

**The situation, counted rather than recalled.** Three rows carry `force` in the `Kind` column -
`muster` produces *that citizen's force*, `stand` produces *that unit's force*, `discard` consumes 1.
*Kinds* declares **seventeen** kinds and none of them is `force`. The code lane cannot write a row
naming something neither declared as a kind nor as a family, so it built under a stated assumption
and `prototypes/kinds` carries a `Kind::Force` deliberately outside the declared set, with a test
asserting the undeclared set is exactly `["force"]`.

**Your `P-425` decision is what keeps this open.** Under the passive reading force was never a
thing and this dissolved; under the token model it is a thing, and a thing has to be declared.

## The four ways, and two of them are not really available

**1. Declare `force` as a kind, and leave the word doing two jobs.**

> | **force** | what a territory presents to hold or take ground; mustered each turn and swept at its end |

**It has two close neighbours already in the table**, and they are the argument: `labor` is *what
working a machine takes; a citizen provides it each turn*, and `fertility` is *a citizen's capacity
to raise one more, spent by raising one and renewed each turn*. **Both are transient things made
each turn and spent**, which is exactly what force now is. Declaring it makes three of a shape
rather than an oddity.

**2. Declare it and rename one of the two, so the word does one job.** `force` is a **trait** of
citizen, garrison, ark and pioneer - *a number*, marked *of the kind* - and would become a **kind**
as well. Renaming the kind (*mustered force*) or the trait (a citizen's *strength*) separates them.

**The precedent is yours and it went the other way.** On 2026-09-11 you chose the uniform naming
rule over a rule with one correct exception - *unification is a big deal for me* - and the exception
you declined was `laboring` sitting beside the kind `labor`. **That was a near-collision and this is
an exact one**, which is the only reason this is listed rather than settled by `P-428`.

**3. Change the rows so no recipe names `force` as a kind.** This is `P-425`'s way two - force
becomes a derived trait of a territory and leaves the recipe table. **You decided against it this
afternoon**, and it is listed because it is the only way the row is genuinely unnecessary rather
than merely unwritten.

**4. Declare it as a family.** A family is a set of kinds - `unit` is an ark and a pioneer. **Force
is not a set of anything**, so this is listed only so a reader knows it was considered.

## What this lane would do, said rather than done

**Way 1.** `P-428` points at it: three transient kinds of one shape is the uniform form, and a
rename is the exception. **What stops this lane simply proposing it** is that the collision is
exact rather than near, and you have never been asked about an exact one.

**Nothing is blocked either way.** The code lane is green under its assumption, and the day the row
lands its own test fails and says so.

