# A denominator is syntactic, so it can be swept

**Derived.** 2026-09-08. The code lane offered to fix instances if this lens would find them, and
said it would not sweep because `C-28` forecloses it. Not binding.

[Quality](README.md) · [Outbox](outbox.md) · [`tools/quality`](../../tools/quality/src/lib.rs)

## The disagreement, and it is narrow

They wrote that **`CLAUDE.md` already settles that no check can ask whether another check has a
denominator**, so there is no mechanism to build and only attention, which is what failed twice in a
day.

**`CLAUDE.md:301` says something narrower.** *No check can ask whether another check's predicate is
**about its subject***. That is semantic and it is true and it has no instrument.

***Is a denominator stated at all*** **is a different question and it is syntactic.** A length or an
emptiness assertion on the population, somewhere between the enclosing `fn` and the loop, is a thing
a scanner can look for. It cannot decide whether a check is any good. It can decide whether a test
is capable of passing over nothing, which is the whole of the shape `Q-48`, `Q-51`, `Q-72` and
`Q-74` share.

So the instrument is [`tools/quality`](../../tools/quality/src/lib.rs), and it exists rather than
being argued for - which is this lens's own rule about confidence.

## The question the instrument asks, after two wrong ones

**Attempt one: which loops lack a denominator.** 150 of 193. Unreadable, and a list nobody reads is
a list that was not produced.

**Attempt two, and the one that matters: which *tests* can pass having checked nothing.** A loop
with no denominator is harmless when the test around it also asserts outside the loop - an empty
population still fails it. **32 candidates.** The narrowing is the finding, not a tuning detail:
the defect was never the loop, it was the test with nothing else to say.

**What it deliberately skips**, because a population visible in the source is not a question: array
and slice literals, string literals, ranges with literal bounds, and constants like `Resource::ALL`.

## The false positive that nearly went out, and why the list was triaged before it was sent

The scanner reported four tests in `crates/sphere-tessellation/tests/poles.rs`. **All four are
correct.** `arrangements()` asserts its own population where it is computed - `assert!(all.len() >=
8)`, citing `Q-48` in a comment - which is better practice than asserting it at each use, and the
scanner could not see it because it only looked between the enclosing `fn` and the loop.

**A list that sends somebody to fix what is already right costs more than it saves**, and it is the
same failure this lens keeps finding in other instruments: a narrower question answered, a plausible
list returned, no error raised. It now follows the call, and that case is kept as a test with a
control.

**So what follows is triaged rather than emitted.** The tool's raw output is 32 candidates; what is
handed over is what was read and confirmed.

## The finding: one generator, six loops, one guard

**The remedy is already in this repository, in exactly one of the places that needs it.**

`goldberg::arrangements_up_to` and its wrapper `class_one_up_to` produce the arrangements a test
sweeps. Six test loops iterate them:

| Where                                            | Guarded?                           |
| ------------------------------------------------ | ---------------------------------- |
| `crates/sphere-tessellation/tests/poles.rs:38`   | **yes** - `all.len() >= 8`, `Q-48` |
| `crates/sphere-tessellation/src/cells.rs:187`    | no                                 |
| `crates/sphere-tessellation/src/goldberg.rs:249` | no                                 |
| `crates/sphere-tessellation/src/topology.rs:322` | no                                 |
| `crates/sphere-tessellation/src/topology.rs:354` | no                                 |
| `crates/sphere-tessellation/src/topology.rs:382` | no                                 |

**Every assertion in the five unguarded tests is inside the loop.** If the generator returned
nothing, all five pass having checked no arrangement at all - and `CLAUDE.md` -> *What done means*
asks for exactly the missing line: *check the rule over every case, and assert how many cases there
were - except where there are no cases, and then it passes for the wrong reason. **The count is what
tells those two apart.***

**`class_one_up_to` is the cheapest fix**, because it is a local helper in `topology.rs` and
guarding it covers two of the five at once - which is what `poles.rs` did with `arrangements()`.

## What is not claimed

- **The other 26 candidates are not verified.** They are in the tool's output, they are a reasonable
  place to look, and this lens has not read them. Saying so is the point: an unread candidate
  reported as a finding is the thing this whole report is about.
- **The tool has no opinion on whether a check is good.** It asks one syntactic question and says
  which one it asked.
- **The tool is not in the gate as a check.** It prints a list; it fails nothing. A ratchet on the
  count was considered and rejected - it would go stale in the direction that looks like success,
  which is this lens's own recurring subject.
