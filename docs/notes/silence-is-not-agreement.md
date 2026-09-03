# Silence is not agreement

**Derived**, 2026-09-02. Eight failures found on one day, all the same shape. Not binding.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Every one of these read correctly. None of them was wrong about itself. What had stopped holding was
a **relationship** to something else, and nothing in the repository indexes relationships.

| What                                               | Read as   | Actually                                         |
| -------------------------------------------------- | --------- | ------------------------------------------------ |
| `citations.rs`'s shallow-clone guard               | passing   | never ran - `HEAD` resolves in a shallow clone   |
| the same check, in CI                              | green     | skipping, because the checkout was shallow       |
| `every_real_proposal_offers_its_text`              | green     | looping over zero open proposals                 |
| `is_a_commit`                                      | satisfied | true of objects a clone would never receive      |
| `transition.rs` quoting `spec/planet.md`           | correct   | quoting a sentence `P-191` had rewritten         |
| `CLAUDE.md` citing `Q-1` as *correctly still open* | correct   | `Q-1` closed three days earlier                  |
| `prototypes/kinds` comparing tables cell by cell   | agreeing  | both sides said `territory`; neither declared it |
| `deploy ark` as the recipe for landing             | complete  | consumes an ark already on the ground            |

## The three faces of it

**A check that goes quiet looks exactly like one that passes.** Four of the eight. The cure that
worked each time was the same and is not "be careful": ask what the function actually asks rather
than what its name says. `is_a_commit` asks whether an object is in this database, which is not
whether a clone would have it. A guard asking whether `HEAD` resolves is not asking whether the
history is here.

**A count is what tells an empty check from a passing one** - and where the empty set is the *good*
state, the count cannot be forbidden. An empty proposal queue is the state to want. So the parser
gets counted a second way and the two counts must agree, which is a comparison rather than a
minimum.

**A deferral is characterised by what is still watching while you defer**, not by the decision to
defer. `C-16` leaves a duplicate under a check that catches drift. `C-11` leaves a live divergence
with nothing comparing the two sides. Both were being called *parked*, and one of those is much
worse than the other. The code lane's observation, 2026-09-02, and the sharpest thing said all day.

## What actually found them

Not review, and not care. **Six of the eight were found by asking a question with an answer**: a
count, a poison test, a join, a re-run. The two found by reading were found by reading *for a
different purpose* - the code lane checked `CLAUDE.md`'s factual claims because it had nothing
buildable, and the catalog's first run put every kind beside its recipes because that is what a
catalog is.

**The join is the one worth generalising.** `orbit` having no recipe was invisible to every
comparison between two tables, because every table was correct alone. One place per kind, with the
join done, showed it on the first run - which is the argument
[`P-193`](proposals.md) makes for generated views, arriving on the day the first one was built.

## What this does not say

It does not say the guards were a bad investment. **Every one of the eight was found**, most within
hours, and the ones that took longest were the ones nothing was watching. It says that a guard's
silence is evidence only when something has established the guard can speak - which is why a poison
test is worth as much as the check it poisons, and why the code lane's first attempt at one being
*decoration rather than poison* is recorded in its own comment.
