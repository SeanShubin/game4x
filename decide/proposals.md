# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-549 - An ark is never on the surface, which makes one row of the data wrong

**to** sean · **status** open · **raised** 2026-09-24 · **kind** recovered · **shape** an instruction · **asks** approval · **into** `spec/orbit.md` -> The layers, `spec/data/limit.4x`, and `releases/first-release.md` -> What bounds a kind

**You answered `K1` and added a rule.** *Yes launching leaves the ark in orbit. We spend materials
from the surface and end up with an ark in orbit. **An ark is never on the surface.*** That last
sentence is not in `spec/` and it makes something wrong that this item had not looked at.

## What lands

**Into `spec/orbit.md`, beside *an orbit admits no extractor and no citizen*:**

> **An ark is never on the surface.** Launching spends materials from the surface and ends with an
> ark in orbit; deploying spends the ark and ends with citizens and extractors on the surface. **A
> surface admits no ark**, which is the layer rule above said from the other side.

## And one row of the data is wrong, which is the instruction

```
is    {limit container:territory contained:ark n:2}
to    {limit container:orbit contained:ark n:2}
```

**`orbit` is a kind and a member of the `place` family**, so the row is expressible as it stands.
**And `releases/first-release.md` says *an ark: a capacity of 2* under what bounds a kind**, which
follows the container rather than stating a different number.

**Nothing else changes.** The release and `spec/data/line.4x` already produce an ark above
`$where`, which is what you confirmed - so `K1` needed no correction to either.

## How to tell it was carried out

**Three assertions.** The sentence is present in `spec/orbit.md`; `limit.4x` holds four rows with
the ark's container reading `orbit` and no row containing `contained:ark` under `territory`; and
the release's capacity row names the orbit.

**The gate goes red until the code follows**, because `spec/data/` is what the engine loads and a
container that moves changes where the engine looks. **Said in the same breath as the rule**, and
the lane that has to fix it is not the only lane the gate stops.

## What this leaves open, filed rather than noted

**Nothing checks what a layer admits.** `spec/orbit.md` has said *an orbit admits no extractor and
no citizen* for weeks with no data form, and this sentence joins it. **`limit` bounds how many, not
which** - a container with no row for a kind is silent rather than forbidding. Filed as `S-167`.

### P-550 - A check that pins the present state cannot report a gap against what should be

**to** sean · **status** open · **raised** 2026-09-24 · **kind** measured · **shape** text · **asks** approval · **into** `CLAUDE.md` -> What done means

**Two failures this week were the same sentence twice**, and the code lane found the sentence.
**It has already caught three cases it did not come from**, which is what `CLAUDE.md`'s own test
asks of a habit before it becomes a rule.

## What lands

After *a check whose subject is behaviour reads the outcome, not the input*:

> **A check that pins the present state cannot report a gap against what should be.** A test
> asserting the exact words a program produces is the strongest possible statement about what it
> does and **says nothing about what it ought to do** - so it runs green while a document the code
> is meant to obey goes unkept. **What tells the two apart is what the assertion names**: the
> output, or the rule the output owes.
>
> **Two instances in one week and they are one sentence.** `docs/architecture.md`'s rule 4 -
> engine types only in the adapter - was written down and held by nothing. `P-542` renamed the
> planet sizes and the code said `tiny` for three days, **held by a test asserting the old
> names**. Neither was found by a check failing; both were found by somebody re-deriving a claim.
>
> **So a rule a document states gets a check that asks the rule, over every case it covers.**
> `every_refusal_over_a_closed_set_says_what_was_expected` is the shape: four closed sets rather
> than the one an item named, each asserted to say what was expected, the count asserted, and the
> fifth excluded by name with its reason. **Fixing the case an item happened to name would have
> left three others breaking the same rule.**

## Why it is a rule and not a place to look

**`CLAUDE.md` says a habit earns its place by a case it caught, not a case it explains.** This one
was written off two incidents and then **found three more that neither incident mentioned** -
biome, resource and unit all refused without saying what was expected, and no item had named any
of them.

**And it is checkable in the one way that matters.** The code lane drove it rather than arguing it:
reverting the message to what it said that morning turns the new test red with *`spec/console.md`
asks for what was expected* - the exact state that had been green for three days.

## Whose it is

**The sentence is the code lane's.** This lane had the two instances and drew the wrong boundary
around them: it excluded the incident from the class because a dead variant sat beside the live
one, and *a branch that never runs returns no answer at all* is true and was not the point. **The
test that ran every commit and was wrong by a rule is the instance**, and they said so.


