# First Release Balance, Traced

**Derived.** Written by Claude from conversation, 2026-08-26. Not binding - see
[the specification](../../spec/README.md) for what was actually decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Sean wants food, metal and energy all useful, and *"to initially balance them so that I need about
the same amount of each."* This traces what the current figures actually produce.

## How this was produced

By hand-running the landed rules on the first release's own numbers: `6 food nodes at density 6,
4 metal at 8, 5 energy at 7` from [the release](../../releases/first-release.md), the growth rule
from [`spec/population.md`](../../spec/population.md), one labor per citizen per turn from
[Labor](../../spec/population.md), and `Create Pioneer: 1 metal, 1 citizen`. Re-runnable by
stepping the growth rule with `farming = min(pop, 6)` and `food = farming * 6`.

## One territory, turn by turn

| Turn | Citizens | Farming | Spare labor | Food |
| ---- | -------- | ------- | ----------- | ---- |
| 1    | 1        | 1       | 0           | 6    |
| 2    | 2        | 2       | 0           | 12   |
| 3    | 4        | 4       | 0           | 24   |
| 4    | 8        | 6       | 2           | 36   |
| 5    | 16       | 6       | 10          | 36   |
| 6    | 32       | 6       | 26          | 36   |
| 7    | 36       | 6       | 30          | 36   |

**A territory matures in seven turns** and then holds: 36 citizens, 6 of them farming, **30 spare
labor every turn, for ever.**

## What that means for each resource

**Food is exactly balanced, and that is structural rather than lucky.** Population settles at
whatever food throughput is, because the growth rule caps new citizens at leftover food and
starvation caps population at food produced. 36 produced, 36 eaten, nothing spare. **Food will
always be the perfectly-consumed resource no matter what number is chosen.**

**Metal and energy are not consumed at all after the build-out.** A territory's whole build list is
**15 extractors** - one per node. At one labor each that is fifteen labor-turns, which is *half of
one turn* of spare labor at equilibrium. After that, 32 metal and 35 energy arrive every turn with
nothing to buy, and
[`spec/turn.md`](../../spec/turn.md) discards whatever is unused.

**So the imbalance is not in the node ratios.** 36 / 32 / 35 is nearly equal already. The
imbalance is that **food has continuous demand and the other two have a finite build list.**
Changing node counts cannot fix that; only costs and something to spend on can.

## The sharper problem: the pioneer is far too cheap

A developed territory has 30 spare labor and 32 metal a turn. A Pioneer costs **1 metal and 1
citizen**. So a single mature territory can field **thirty pioneers in one turn**, and a tiny
planet has **eleven territories left to take**.

**The game is over the turn after the first territory matures.** Seven turns to grow, one turn to
conquer the planet. Everything after that is bookkeeping.

## What would actually balance it

The lever is **cost**, not nodes. For metal to bind as tightly as food does, per-turn demand has to
approach per-turn supply:

| Change                           | Effect                                                                               |
| -------------------------------- | ------------------------------------------------------------------------------------ |
| Pioneer costs ~16-32 metal       | One or two pioneers per mature territory per turn; expansion becomes a real schedule |
| Extractors cost metal at all     | Currently free - the build-out spends nothing, which is why it takes half a turn     |
| Energy cells cost energy to fill | Gives energy its only continuous demand - see P-66                                   |
| Storage, later                   | The real fix for lumpy demand meeting steady supply - see the backlog                |

**Storage is the structural answer, and food spoiling is the right asymmetry.** Food demand is
continuous and metal and energy demand is lumpy; the mismatch between lumpy demand and steady
supply is exactly what storage exists to absorb. So *food spoils, metal and energy store* is not
arbitrary flavour - it matches which resource actually needs buffering. Recorded in
[the backlog](spec-backlog.md).
