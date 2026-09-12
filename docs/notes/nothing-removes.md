# Nothing removes

**Derived.** Written by Claude, 2026-09-12, from a day in which the same failure happened five
times in three lanes. Not binding - see [the specification](../../spec/README.md) for what was
decided.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Named by the code lane, after the third instance: **a thing only ever written says something true
once and then goes on saying it.**

## The five

Each of these was correct when it was written, and each went on being read after it stopped being
true. None of them was edited by anybody, which is the whole of the shape - there is no careless
act to point at.

| What                                               | True when written                                         | When it stopped                                        |
| -------------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------ |
| The queue's *Nothing is open* sentinel             | the last item had landed                                  | the next item was filed under it - twice in one day    |
| The code lane's absence assertions on `spec/data/` | `families.4x` did not exist, so a test read the generator | `P-455` created it                                     |
| Six read receipts, `to code · status open`         | each announced a promotion the code lane had not yet seen | the moment it read them, which was minutes later       |
| `S-97`, `status open`                              | the release had four roles and no way to name a thing     | `P-421` declared `put`, and it sat open another day    |
| `UNDECLARED_MAXIMUM`'s doc comment                 | nothing in the release declares a maximum                 | whenever `P-459` is answered, and nobody would be told |

**The asymmetry is structural rather than anybody's carelessness.** In all five the write is cheap
and automatic - it falls out of doing the thing - and the removal is expensive and manual, because
it requires noticing that a fact somewhere else has moved. A discipline cannot close a gap that
wide, because the moment the removal falls due is exactly the moment attention is elsewhere.

## The repair that works, and it is not a reminder

**Make the removal fail loudly rather than be remembered.** Four of the five are beaten this way,
and all four repairs have the same shape: something that is cheap to run asserts the statement, so
the day the statement stops being true a gate goes red and names it.

- **The sentinel** is derived from the section it describes rather than written into it -
  `say_if_empty` removes as well as adds, and `the_sentinel_says_what_is_true_of_the_open_section`
  holds the two against each other at the gate
- **The absence assertions** named both sides of the difference, so when `families.4x` arrived the
  assertion did not stop being needed - **it failed**, with the old list printed against the new
  reality. This is the case that taught the pattern to the other four
- **`UNDECLARED_MAXIMUM`** has
  `the_readies_column_states_no_maximum_for_the_constant_to_read`, which fails the day the
  `Readies` column carries a number, cites `P-459`, and refuses a third column shape too
- **`status open`** is beaten by `hooks/pre-commit`, and it has been running all along:
  `outbox --settled` names every open item cited by a commit that did not touch its outbox, and it
  prints on every commit until the item is closed or records the hash. **The code lane thought this
  one could not be mechanized** - *a person deciding to look IS the mechanism* - and the mechanism
  was already nagging both lanes about `P-456` and `Q-59` on the day it said so

## What is left, and why it is not a mechanism's job

**The blind spot in all of them is an item that describes its subject instead of naming it.**
`S-97` says *the release's recipe table* and no path, so `spec touching releases/first-release.md`
does not find it and no commit cited it. A check that asked *has this item's subject moved* would
have to know what the subject is, which is the question being asked - the wall `P-245` hit, the
wall the three constants hit, and the wall `docs/process.md` records as *no check can ask whether
another check's predicate is about its subject*.

**What remains is that each lane reads the other's outbox as work and finds out by hand.** That is
slow and it does work: `S-97` was found because the code lane went looking for something to do
while idle, and paid twenty minutes to discover the item was already answered.

## The other repair, which is to not write it

**A notice has no work in it and should never have carried `status open`.** Six items that say
*this landed, and it is not work for you* were read once and then cost the code lane attention
every time it listed its inbox - which is the same cost as a stale item, arrived at without
anything going stale. `answered` is a status the outbox format already has, and it is what a notice
is on the day it is filed.

## Sources

Everything here happened on 2026-09-12 and is in the history: `bb9bba4` (the sentinel derived),
`7c81656` (the absence assertions ending), `8c26439` (the read receipts and `S-97`), `958cd68`
(the doc comment), `e7c10ee` (`spec touching`). The naming is the code lane's, in
`crates/outbox.md` and in messages between the two sessions on the day.
