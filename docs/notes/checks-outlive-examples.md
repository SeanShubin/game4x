# Checks outlive examples

**Derived, 2026-09-01.** Written by Claude from a day's work in which the same failure appeared three
times wearing three different faces. Not binding.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## The shape

The code lane's statement, and it is the whole note in one line:

> An example demonstrates a rule until the code changes, and then it demonstrates nothing while still
> going green.

Three instances in one day, none of which failed loudly:

| What broke                                                                                                          | How it looked                                                                                              |
| ------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `add_node_is_not_mistaken_for_adding_a_unit_called_node` demonstrated prefix ordering with `add` against `add node` | `add node` was removed, the test could be renamed, and it would have gone on passing while showing nothing |
| The quotation guard listed attributing verbs, and `said` was among them                                             | Every other verb was present tense, so one past-tense record of changed wording read as a stale quotation  |
| `P-143`'s territory traits lived in a *Places* column                                                               | `P-148` rewrote the section around bins and the column stopped existing, taking four traits with it        |

## Why none of the three failed loudly

**In all three the artifact still read correctly on its own.** What stopped holding was its
relationship to something else: a test to the grammar it demonstrates, a verb list to the tense of
the claims it checks, a declaration to the recipes that use it. **A check that compares two things is
the only kind that catches that**, which is also why `pending.md` is generated from the outboxes
rather than written beside them.

## The half that is easy to miss

**A universal check over an empty set passes.** Replacing the ordering example with *for every pair
of forms sharing an opening word, a hole may not precede a keyword* is the right move, and on
2026-09-01 the grammar contained **no colliding pair at all** - so the check was true of nothing and
green either way.

Two things make it real, and the code lane did both:

- **Assert the count**, so the check has to have looked at something
- **Demonstrate the mechanism separately**, on a fixture built to collide, both ways round, so the
  rule survives a release in which nothing real exercises it

**Without the count it is the rotted example again**, one level up and harder to notice.

## Where this already applies

`CLAUDE.md` says a quality improvement's evidence is **a test that would have failed before it**.
That is right and this refines it: the test should check the rule over the whole artifact rather
than over one example of it, and should assert that the whole artifact was not empty.

`Q-8` is the pattern done correctly - one test comparing two derivations **at every planet size**,
not at one. The qualifier cross-check the code lane built on 2026-09-01 is the same move applied to
the release: every qualifier in the recipes names a declared trait, or is reported.
