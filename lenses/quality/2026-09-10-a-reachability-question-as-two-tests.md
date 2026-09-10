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
