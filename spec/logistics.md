# Logistics

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

## Where materials are

- Every resource exists in a particular place; there is no general inventory

## Containment

- A thing may contain things, and is itself in at most one other thing. Nothing else holds
  anything
- What a thing may contain is a maximum **per kind, per family of kinds, or per kind carrying a
  particular value of a trait**. **Three names describe it and there are two facts**: its
  **capacity** for that kind, how much of that capacity is **occupied**, and how much is **free**.
  **Any two give the third, so only two are ever held** and nothing can disagree with anything. A
  capacity of four extractors is a maximum of four, so nothing a player builds ever crowds out
  something of another kind
- **Capacity is spent and given back.** Making a thing occupies one of the free capacity in
  whatever will hold it; **destroying it frees one**. The two never come apart, because the
  capacity is only ever their sum
- **When a thing that holds capacity is consumed, the place it stood in has that much less room.**
  Nothing falls loose, because nothing was inside it: what a place holds is the place's, and
  destroying a store leaves the resources where they already were.
- **A resource in a place is in that place, not in a container inside it.** What a place holds
  of a kind is one number. **The things in it that can hold that kind contribute capacity and
  hold nothing**, and at the turn's end what the place holds beyond that capacity is lost
- **A place's capacity for a kind is the sum of what is in it that can hold that kind**, and a
  place declares none of its own. **This holds of every place**: an orbit has room for the fuel
  its units carry and for nothing else, because that is what is in it
- **A bin is a thing.** What holds a kind is a store for that kind, and a thing with two bins
  holds two stores. **A store's capacity is what its kind declares**, and it holds nothing
- **A thing that leaves takes what it hauls.** A unit moving out of a place is given an amount
  of each kind, no more than its own capacity for that kind, and that amount joins the number
  the new place holds. **Allocation happens at the moment of leaving and at no other.**
- A thing that contains things takes up capacity in whatever contains it
- A thing says which of the things in it are next to which. That is a fact about the container
  rather than about its contents
- Nothing contains itself, directly or through anything else
- Every thing is in the game, directly or through what contains it. **The game is the one thing that
  is in nothing**, so containment is a tree rather than a scattering
- **A kind declares one of three things about what it may hold.** It may declare **no capacity**,
  and then it holds nothing of that sort and never can. It may declare a **limit**, and then it
  holds up to that many and may happen to be empty - so a thing holding nothing today is not
  thereby a thing that never could. Or it may declare **no limit**, and then it holds any number,
  and there is no free capacity to record because nothing can be short of it
- **The game declares no limit, for every kind.** It contains everything, there is no free
  capacity to record because nothing can be short of it, and it is the one thing that is in
  nothing - so the tree has a root that no rule has to except.
- **What a kind may contain is a fact about the kind and not about any one of them.** Nothing becomes
  a different sort of thing by picking something up
- **Containing is not referring.** A thing may name another without holding it - an adjacency names
  the two places it joins, and neither of them is inside it
- A thing is not located by a trait. **What holds it is what says where it is**, and nothing else
  does
- A thing may carry an **`id`**, and one that does is unique. **There is never a quantity of a
  thing with an `id`** - it is one thing, and anything that holds it holds exactly it
- **A place carries an `id`**, because movement and adjacency name one place rather than some
  place of its kind
- **A unit does not, and the reason is scale.** Fleets of any size are intended, and **acting on
  one unit and acting on a million are meant to be the same act** - which they are only while the
  million is one entry with a count. **An `id` makes every thing its own entry**
- **Things of a kind are grouped by their state**, and a kind has few states however many things
  of it there are. **So a fleet's size grows and its number of entries does not**

## Moving materials

## Paying a cost

- A cost may be made of anything the player controls: resources, citizens, units or structures
- Whatever pays a cost must be in the territory where the thing being paid for is built
- Building something that costs 8 metal and 5 energy requires 8 metal and 5 energy to be in
  that territory

## Open questions
