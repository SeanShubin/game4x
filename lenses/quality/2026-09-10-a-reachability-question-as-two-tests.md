# A reachability question written as two capacity tests

**Derived**, 2026-09-10. Reviewing the code lane's burst for `c3cccc4`'s eight promotions, at
Sean's asking. Not binding.

[Quality](README.md) · [Outbox](outbox.md) · [Reports](README.md#reports)

**Range taken from the work, not from a clock.** The code lane named `e570e94`, `c764fe2`,
`81d26a1` and `8e49cb6`. The last is yesterday's prototype and unrelated to `c3cccc4`, so the
range is `e570e94..81d26a1` and the substance is `c764fe2`. `f387184` landed after they wrote and
is the specification lane's, fixing the note `C-78` reported.

**They asked for a view on their risk order before anything long, and got one.** Their order was
right. This report is the argument behind it.

## The finding

`crates/game-model/src/territory.rs:479`:

```rust
let can_ever_build = food_density >= 2 && self.capacity_for(Resource::Metal) >= 1;
```

**The food branch is guarded on capacity and density; the metal branch on capacity alone.** Four
lines above, `maximum_output` returns `(0, 0, 0)` when `food_capacity == 0 || food_density == 0`.
Nothing asks the same of metal.

A metal deposit with capacity 3 and density 0 yields nothing when worked. Nothing crosses a
territory boundary, so such a territory can never obtain a metal, so it can never build - which is
the case the `else` branch exists for. `can_ever_build` says true.

### Measured, against a clone

`Territory` with food 4 x 4 and energy 4 x 5 - territory 6's shape - run twice, once with metal
capacity 3 density 0 and once with no metal at all:

| Metal deposit             | `maximum_output` | Ceiling in fact |
| ------------------------- | ---------------- | --------------- |
| capacity 3, **density 0** | `(16, 4, 7)`     | `(4, 1, 1)`     |
| none at all               | `(4, 1, 0)`      | `(4, 1, 0)`     |

**The run reported `1 passed; 62 filtered out`**, so the population it acted on was not empty -
which is the half of a probe that is easy to skip and reads identically when it is missing.

**Reachable rather than hypothetical.** `Deposit` derives `Default`, `Territory::empty` starts every
deposit at zero, and `set resource <territory> <resource> <extractors> <density>` takes the two
numbers independently. It is the door `Q-58` came through.

### What it costs

An unreachable ceiling makes `at_maximum_output` false for ever. That makes the planet never fully
exploited, which makes the game unwinnable, and nothing goes red.

**That is the failure `P-361` was promoted to remove, surviving one cell over.** The old wording
counted ground nobody could staff; territory 5 and territory 6 could never finish. This is the same
shape reached through density instead of through capacity.

## Why it is worth more than the line it is on

**The code lane's own third observation is the right diagnosis, and this is its third instance.**
They corrected the arithmetic twice while writing it, both times from a check rather than from
reading, and said so: *under-constrained rather than careless.* This is a third correction to one
expression, also found by a check.

`can_ever_build` asks a **reachability** question - can this territory ever obtain a metal? - and
states it as a two-term conjunction over capacity. **Reachability does not factor into two capacity
tests**, which is why each pass finds one more term rather than finishing. Written as what it means
it closes, because founding is the only other source and nothing crosses a boundary:

```
metal obtainable  <=>  metal capacity >= 1
                  AND  metal density  >= 1
                  AND  food density   >= 2      (something must staff the extractor)
```

Each term comes from a sentence. The current version came from working the release's twelve.

## The check is good; its population is twelve

`the_release_reaches_the_output_the_specification_lane_derived` is better than most in this
repository. It asserts its population is twelve, counts its comparisons, and takes its expected
values from a hand-worked note rather than from the code.

**It is not circularity that hides this.** `maximum_output` is total over all territories and the
check runs over twelve hand-chosen ones, none of which has metal capacity with zero density. So
de-circularizing the two `game-model` tests - the thing the code lane was most worried about -
would cost effort and still not see it. **The lever is the population, not the shared computation.**

Said with care, because this lens drew that caution too wide once already: `Q-56` argued a delta
check would be circular, the code lane built it anyway, and it found a real bug on its first run. A
caution drawn too wide costs a check that would have worked.

## The fix cannot borrow the existing check's green

No territory in the release has metal capacity with zero density, so adding the term leaves all
twelve answers unchanged and `the_release_reaches...` passes either way. **A fix with no new case is
unverified**, which is `C-38` and the reason a green suite under a change bounds the tests rather
than the code.

## Separate: the specification names an input nothing reads

`spec/control.md:54`: *What that greatest output is follows from the territory's own permanent
facts: how many extractors it has total capacity for, their densities, **and its biome**.*

`maximum_output` reads no biome. Probed on one shape across three biomes:

| Biome     | `maximum_output` |
| --------- | ---------------- |
| Grassland | `(16, 4, 7)`     |
| Ice       | `(16, 4, 7)`     |
| Desert    | `(16, 4, 7)`     |

In `game-model`, `biome` reaches only `is_claimable`. **Either the sentence names an input that does
nothing, or something is missing**, and the first looks right: the biome clause reads like it
belongs to *every territory that can be taken has been taken*, which is the neighbouring clause of
the same rule. That makes it the specification lane's sentence rather than the code lane's defect,
which is why it is addressed there.

## Not re-found, at the code lane's request

`quotations.rs` finding italicised quotations and missing plain prose, and `C-79`'s table held twice
in two columns. Both already recorded by them. The prototype was not reviewed.

## Verified after the fix, 2026-09-10

**`Q-79` acted in `c63190a`, checked from the files rather than from the code lane's report.**

**The poison goes red on the right test.** `yields` reverted to `capacity >= 1` in a clone:
`a_deposit_with_room_and_no_density_is_not_a_source` fails with `left: (16, 4, 7)`,
`right: (4, 1, 0)` - the probe's own numbers. Run: **62 passed, 1 failed, 0 filtered**.

**And the release check does not.** Under the same poison,
`the_release_reaches_the_output_the_specification_lane_derived` is **4 passed, 0 failed, 0
filtered**. So the claim this report rested on - *the population is what misses this, not the
circularity* - is measured rather than argued.

### A number in this report was wrong

It gave the true ceiling as `(4, 1, 1)`, counting the metal extractor founding attaches to a
deposit that yields nothing. **By this report's own principle that is not somewhere a citizen
produces anything**, so it is `(4, 1, 0)`, which is what the code lane wrote. Their extension past
what was filed - filtering `yields` in the other branch too, so a zero-density *energy* deposit is
not counted as somewhere to put a citizen - is the same principle applied where this report did not
look.

### The fix reached one of two predicates

`can_build_extractors` at `territory.rs:589` answers the same question -
*whether this territory can ever build an extractor* - as
`food.capacity >= 1 && food.density >= 2`, and never asks whether a metal can be obtained.

| Case                          | `can_build_extractors` | `can_ever_build` |
| ----------------------------- | ---------------------- | ---------------- |
| t6: food 4x4, no metal        | true                   | false            |
| food 4x4, metal 3 x density 0 | true                   | false            |
| food 4x4, metal 3 x density 4 | true                   | true             |
| t5: food 3x1, metal 8x8       | false                  | false            |

Two disagreements in four. Over the release's twelve, **the predicate says eleven can build and the
corrected rule says ten** - the difference is territory 6, whose row in the release reads *No
metal*. Its test asserts the predicate against its own retyped body, and its doc's *eleven of the
twelve* became false when `Q-79` landed. `Q-81`.

### The gate's clippy skips every test target

`hooks/pre-push:20` runs `cargo clippy --workspace -- -D warnings`, which exits 0. **Line 68 of the
same file lints the tools with `--all-targets`.** The workspace line without it lints no `tests/`
target and no `#[cfg(test)]` module inside `src/`; with it, **11 errors** across eight files.

Measured identically at both ends of the burst - **16 at `c3cccc4`, 11 at `7c0501c`** - so this
burst reduced it and introduced none of it. `Q-82`, and it is a flag plus eleven fixes rather than
a flag.

## All three closed, 2026-09-10

`Q-78` and `Q-81` in `b0d43b3`, `Q-82` in `8996dc1`. Verified by running, not by reading the report.

### `Q-81`'s fix is better than this report asked for

The report asked for the missing term. **They removed the second predicate instead** -
`can_build_extractors` is the question now and `maximum_output` calls it. Poisoning the metal term
back out:

|                  | `the_release_reaches_the_output_the_specification_lane_derived` |
| ---------------- | --------------------------------------------------------------- |
| before `b0d43b3` | **4 passed, 0 failed** - fully green                            |
| after `b0d43b3`  | **FAILED**, 2 passed 2 failed                                   |

**That is the population problem this report diagnosed being fixed rather than worked around.**
Before, the term was held only by a constructed case; now it is under the check whose expected
values come from outside the code. Both terms were poisoned separately and both go red.

**What this report did not predict**, and they supplied: unifying the predicates forced the six
table cases into five plus four, because with three terms a case carrying no metal deposit answers
false for a reason its own test is not about. A case that fails for the wrong reason tests nothing -
this lens's rule, arriving from the other direction.

### `Q-82`, and a count this report got narrowly right

Both `hooks/pre-push:28` and `.github/workflows/pipeline.yml:115` carry `--all-targets`. They added
the pipeline half unprompted: **a hook the pipeline does not mirror is half a gate**, because the
hook takes `--no-verify` and the pipeline does not. This report named only the hook.

Measured here: strict clippy exits **0**, `fmt --check` exits **0**, workspace suite **574 passed,
0 failed**.

**This report said eleven errors and they fixed seventeen. Both are right and neither is the
population.** At `b0d43b3`: **8 visible, 4 crates aborted** in one run. Clippy stops at the first
failing crate, so a run shows only what compiles ahead of the abort. *Eleven* was what one run
displayed; *seventeen* is what removing them revealed. **An abort is a denominator that moves** -
the same failure this report is about, with the sign turned around, and committed by the report
itself.
