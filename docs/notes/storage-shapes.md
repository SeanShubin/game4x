# Storage: what the references have in common, and where the real choice is

**2026-09-05.** Sean asked to brainstorm a storage system, offering three references and one
principle. This is the thinking, not a decision. Nothing here is settled until it is in
[the specification](../../spec/README.md).

## His principle is already written down

> The per-type is really a re-statement of no building working against you, you only have to build
> more, you don't have to decide which.

[`spec/logistics.md`](../../spec/logistics.md) says it in almost those words: *a total capacity of
four extractors is a maximum of four, **so nothing a player builds ever crowds out something of
another kind**.* **Per-type capacity is not a new decision, it is a rule he already promoted**, and
the question is only how far it reaches.

## What the three references actually share

|                 | Limit is                      | Per type?     | Player organises? |
| --------------- | ----------------------------- | ------------- | ----------------- |
| **Zelda 1**     | fixed, upgraded at set points | yes, one each | no                |
| **Far Cry 4**   | fixed, upgraded by crafting   | yes, per pelt | no                |
| **Baldur's G3** | emergent from total weight    | **no**        | **yes**           |

**The one he hates is the one where the player does the organising**, and it is also the only one
where the limit is a single pool. Those are two different faults and they arrive together, which
makes them easy to confuse.

**The one nobody notices**: in all three, **capacity is a property you have, not a thing you build
and place.** Zelda's wallet, Far Cry's pouch - both are upgrades to a number, not objects competing
for room. **That cuts against bins as buildable things**, which is where the bin language was
heading.

## Where the real choice is

Not per-type against pooled - he has settled that. **It is where a capacity comes from**, and there
are four answers:

- **Given.** A territory simply has a capacity of 20 for each resource. **This is the release
  today**, and it is the Zelda model
- **Upgraded.** Capacity is given, and something raises it - a structure, a technology. Far Cry's
  pouch
- **Built and placed.** A bin is a thing you build that grants capacity. **This is what the bin
  language implies** and what none of his references do
- **Derived from production.** A territory's metal capacity is a function of its metal extractors.
  Storage follows production and is never a decision at all

## The argument against bins as things, which is the useful finding

**Keep the per-type principle strictly and a bin becomes invisible.** A bin that holds only metal,
in a territory with a per-type limit on metal bins, is **arithmetically identical to a larger metal
capacity**. The nesting adds a noun and no decision.

**Nesting is only interesting when something is shared** - when bins compete for a slot, or one bin
can hold two kinds. **And sharing is exactly the thing he does not want**, because it is what makes
a player decide which rather than build more.

So bins in a territory are either **redundant** or they **reintroduce the trade-off he dislikes**.
There is no third case.

## Where bins do earn their existence

**In transit.** A metal transport with a bin for fuel and a bin for metal is not redundant, because
**the bin moves**. Its capacity is not a property of any place; it is a property of the vehicle, and
it goes where the vehicle goes.

Which suggests a shape with very few degrees of freedom:

- **Capacity in a place is a property of the place**, per kind. No bins to build, nothing to arrange
- **Capacity in transit is a property of the vehicle**, per kind. A transport declares what it can
  carry and how much
- **`spec/logistics.md`'s containment rules stay exactly as they are** and are what makes the second
  one work - a vehicle is a thing that contains things and takes up capacity in whatever holds it

**The player never arranges storage. They choose what to move.**

## One thing to reconsider while deciding

**20, 20, 20 is arbitrary uniformity.** Food already differs - it has `keeps` 1, so its limit is
about spoilage while metal's is about room. **Three resources with identical numbers and
non-identical behaviour** is the kind of thing that reads as a placeholder, and it is worth knowing
whether it is one before building anything on top of it.
