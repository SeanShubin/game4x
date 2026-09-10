# Maximum possible output

**Specification lane, 2026-09-10.** What each of the twelve territories can actually produce, why two
of them can produce very little, and why the answer does not depend on how the game was played.
Written for [`P-361`](proposals.md), which offers Sean's definition of *fully exploited* for
`spec/control.md`.

[Notes index](README.md) · [Winning](../../spec/control.md) · [The release](../../releases/first-release.md)

## The question this answers

`spec/control.md` said a structure can be built **where the territory's own permanent facts allow
it** - which counts what the ground has room for, and never asks whether anybody could staff it.
Under that reading the first release **cannot be won**, and two territories are why.

Sean, 2026-09-10: *exploiting the planet is about using labor to work the extractors … a fully
exploited planet does not mean every territory is fully exploited, it means that the planet is
producing maximum possible resource output.*

## How the numbers are derived

Four facts from `spec/` and the release, and nothing else:

- **A citizen yields one labor a turn**, and spends it at one structure - `spec/economy.md`
- **Working an extractor costs one labor and yields the territory's density** for that resource
- **Upkeep is one food per citizen**, and food does not survive the turn's end
- **No resource crosses a territory boundary** - `releases/first-release.md`, *Scope*

So a territory settles where the food it produces equals the citizens it feeds. With `F` food
extractors at density `d`, putting a citizen at every one gives `F x d` food, which supports
**`Cmax = F x d`** citizens. `F` of them are busy farming, so **`Cmax - F`** are free for metal and
energy, and the territory staffs `min(Cmax - F, metal + energy)` of them.

**Two prior steps matter and the first version of this note missed one of them.** Building an
extractor costs **one labor and one metal**, and metal cannot be imported. So a territory can only
build if **both** are reachable:

- **Metal capacity is not metal.** A territory with none can never build anything, and its ceiling is
  whatever founding left it - one food extractor
- **Nor is capacity labor.** Founding leaves **one** food extractor, so the population first settles
  at `d` and leaves `d - 1` spare. **At `d = 1` that spare is zero and stays zero**, so a territory
  can hold metal capacity it will never work

**Corrected 2026-09-10, by the code lane as `C-78`.** The first version asked whether a territory
**had** metal rather than whether it could ever **work** it, and gave territory 5 a `Cmax` of 3 -
capacity times density - while the prose beside it said one citizen. **The prose was right and the
cell was wrong.** Territory 6's *Others* also read 0 where every other row reads capacity; it now
reads 4, and its *Staffed* of 0 is unchanged. **No conclusion in this note moves.**

## The twelve

`Cmax` is the equilibrium population; `spare` is what is left after the food extractors are staffed.

| Terr | Food  | `Cmax` | Spare | Others | Staffed | What it means                                                |
| ---- | ----- | ------ | ----- | ------ | ------- | ------------------------------------------------------------ |
| 1    | 3 x 4 | 12     | 9     | 6      | **6**   | staffs everything                                            |
| 2    | 2 x 6 | 12     | 10    | 4      | **4**   | staffs everything                                            |
| 3    | 6 x 2 | 12     | 6     | 4      | **4**   | staffs everything                                            |
| 4    | 1 x 2 | 2      | 1     | 8      | 1       | one spare citizen, eight places to put them                  |
| 5    | 3 x 1 | 1      | **0** | 16     | **0**   | **frozen** - one food extractor, one citizen, no spare labor |
| 6    | 4 x 4 | 4      | 3     | 4      | **0**   | **no metal**, so it can never build any of them              |
| 7    | 4 x 4 | 16     | 12    | 4      | **4**   | staffs everything                                            |
| 8    | 6 x 6 | 36     | 30    | 2      | **2**   | staffs everything                                            |
| 9    | 2 x 3 | 6      | 4     | 7      | 4       | four of seven - a real choice                                |
| 10   | 3 x 3 | 9      | 6     | 7      | 6       | six of seven - a real choice                                 |
| 11   | 5 x 6 | 30     | 25    | 10     | **10**  | staffs everything                                            |
| 12   | 2 x 2 | 4      | 2     | 16     | 2       | two of sixteen - a real choice                               |

**Seven staff every extractor they have. Four have exactly the choice Sean described** - food first,
then split the remainder between metal and energy. **Two can do neither**, and they are the two the
old wording blocked the game on.

### Territory 5, frozen by arithmetic

Food density **1**. To feed itself and still have labor spare, a territory needs `w x d >= C` and
`C - w >= 1`; at `d = 1` that is `w >= C` and `w <= C - 1` at once. **It fails algebraically rather
than narrowly** - every other density in the release works from the second citizen onward.

Founding leaves two citizens and one food extractor. One food against two upkeep, so it starves to
**one citizen** on the first turn and holds there for ever: one labor, one food, one upkeep, nothing
spare. **Its maximum possible output is one food, eaten by the citizen producing it.** It reaches
that immediately and can never do more.

**So its `Cmax` is one, not three.** It has total capacity for three food extractors and can never
build the second: building costs labor it does not have. **Capacity it cannot reach is not a
ceiling**, which is the whole of what this note is for.

### Territory 6, stopped by having no metal

Food 4 x 4, so four citizens - but **no metal capacity at all**, and building costs metal that
cannot cross a boundary. Founding leaves it one food extractor and no metal extractor, because there
is no metal deposit to attach one to. **It can never build a second anything.** Four citizens eat the
four food its single farm makes, and three of them have nothing to do.

Its ceiling is reached on arrival, and it is the only territory whose ceiling is set by a resource it
does not have rather than by the one it does.

## Why this does not reward foreclosing

`P-125` rejected one reading of *fully exploited* precisely because it was path-dependent - *no play
**from here** adds anything* lets a player who wastes a territory's capacity finish sooner. **This
definition is not that**, and the table is the demonstration: **every number above comes from
`(capacity, density)` pairs**, read off *Territory resources*. No play history is an input.

Two mechanisms make it hold in practice as well as on paper:

- **Labor renews every turn**, so a one-time build cost is always eventually affordable wherever
  spare labor is not zero
- **Nothing lowers a ceiling irreversibly.** No recipe demolishes an extractor, and a starved
  population regrows to the same equilibrium

So a player who wastes capacity has **not finished**; the bar does not come down to meet them.

## What it costs

The old wording could be checked by looking. This one is **derived**: food capacity times density
gives the population, population less the food workers gives the staffing budget, and territory 6
needs the prior step of noticing that no metal means nothing is ever built.

It is small, decidable, and doable by hand - which is what
[`docs/process.md`](../process.md) asks - but it is arithmetic rather than a glance, and that is a
real difference from the sentence it replaces.

## What this note does not settle

- **Whether *every storage structure on it is full* still belongs** in the condition. It is untouched
  by `P-361` and is separately in question, because `store` is a kind the release has and `spec/`
  does not define - see [the overnight report](2026-09-09-incorporating-the-re-encoding.md)
- **Whether taking a territory that can produce nothing is still required.** *Every territory that
  can be taken has been taken* is untouched, and under a rule about output a territory contributing
  nothing does not change the total either way
