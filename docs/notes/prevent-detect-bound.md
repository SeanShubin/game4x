# Prevent, Detect, Bound

**Derived.** Written by Claude from conversation, 2026-08-30. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Everything is matter](everything-is-matter.md) · [Control without tedium](control-without-tedium.md)

Sean noticed that three separate finiteness questions were all settled the same way - some graph is
acyclic - and asked whether there is an invariant or guideline underneath it that he intends without
having said. There is, and it is larger than the graphs.

## The three tiers

Every soundness rule in this project sits in one of three tiers, and they are ranked:

| Tier             | How the bad state is stopped                      | Cost                         |
| ---------------- | ------------------------------------------------- | ---------------------------- |
| **1. Prevented** | it cannot be constructed                          | free, once                   |
| **2. Detected**  | it can be constructed, and something fails loudly | a check that must stay wired |
| **3. Bounded**   | it can happen, and a number stops it              | a number nobody can justify  |

**The rule is: prevent if you can, detect if you cannot prevent, bound only if you cannot do
either.**

## Tier 1, and why it keeps looking like a DAG

The acyclic cases Sean spotted are the same move applied to different graphs:

- **Rule references are acyclic**, so a rule set cannot recurse and terminates. `P-117`
- **A type cannot appear twice on a containment path**, so capacity is finite and the depth bound is
  the number of container types rather than a chosen number.
- **The crate graph is acyclic and algorithm crates cannot name Bevy**, so the engine cannot leak
  into the model. `docs/architecture.md` rules 6, 7 and 10

But tier 1 is wider than acyclicity, and the other instances make the pattern clearer:

- **One writer per file**, so two instances cannot lose each other's edits - there is no lock because
  there is no contention. `CLAUDE.md`
- **A validating loader**, so a malformed unit definition cannot reach the engine.
- **`--workspace` with an exclusion list** rather than an enumeration, so a new crate is covered by
  default and omitting one is an explicit act.
- **A generated file cannot be hand-edited**, because an edit loses at the next commit.
- **The code lane cannot mark its own capability vetted**, because the file is not in its column.

**What they share is not graph theory. It is that the failure has no way to occur**, so nobody has
to remember anything, and no check has to stay wired.

## Tier 3, and the confession every number makes

**Every magic number in this design marks a place where a cycle survived.**

- **The turn budget** exists because `end turn` makes everything ready again, so the exhaustion
  argument that bounds a single turn does not bound a game. The cycle is real and could not be
  removed, so `P-120` pays a number for it.
- **Fifteen open proposals** exists because reading is not free and nothing structurally stops
  Claude filing.
- **Eight items open to one producer** exists because a lens reviews the code written to satisfy that
  lens's last finding - step 10 returns to step 3, and that cycle is the point of the process rather
  than a defect in it.

So a number is not a failure - it is what you pay when the cycle is *wanted*. **But finding yourself
choosing one is a signal**: ask what loop you failed to remove, and check that you could not have
removed it.

**The diagnostic works in reverse too.** Where the structure is acyclic, finiteness came free and
there is no number. Where it is not, there is a number. That is true of every case in this
repository so far, which is why Sean noticed the pattern from only the free ones.

## Tier 2, and the thing that makes it fail

Detection is the middle tier and it fails in one particular way: **a detector nobody wired to a
failure is not a detector.** The quality lens made the point about the CI gate - it enumerated crates
by name, went stale silently, and reported success while seven tests never ran. What fixed it was
moving to tier 1: coverage by default, omission explicit.

So when detection is the only option, the check has to end in something that stops the work.
`hooks/pre-commit` refusing, a test failing in the gate, `outbox --check` exiting non-zero. **A
warning that scrolls past is tier 3 with extra steps.**

## Where it applies, and where it does not

This is about **soundness** - rules that keep the system finite, consistent and honest. It says
nothing about rules that exist to make the game good: *no penalty for building infrastructure* and *a
drawing never betrays how it was made* are not of this kind and should not be forced into it.

**The test for whether a rule is in scope**: if you can ask *what stops this from being violated?*,
the tiers apply. If the rule is a statement about what the game should feel like, they do not.
