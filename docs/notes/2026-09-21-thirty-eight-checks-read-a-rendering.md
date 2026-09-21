# Thirty-eight checks read a rendering as a source

**Derived.** Written by the specification lane, 2026-09-21, from the measurement that followed
promoting `P-521` to `P-526`. Not binding.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

`CLAUDE.md` states the rule and has for weeks:

> **Two kinds of fact are stated, and they do not share a format either.** A rule is stated in
> prose, in `spec/`. **The game's data is stated in a data file.** Markdown is a stating form for
> the first and a rendering for the second, so **a table of game data in markdown is a rendering
> and never a source.**

**`releases/first-release.md` carries five tables of game data in markdown** - Kinds, Traits, What
bounds a kind, Units and structures, and Recipes. **Thirty-eight checks across fifteen targets read
them as their source**, and nothing said so until one approved edit broke all of them in an
afternoon.

## The measurement

`P-522` cut nine recipe blocks, eight table rows and a whole section out of that file, on Sean's
approval. The suite before and after, both with `--no-fail-fast` so that every target ran:

|                       | targets run | targets failed | tests failed |
| --------------------- | ----------- | -------------- | ------------ |
| before, at `a432094c` | 74          | 1              | 1            |
| after, at `292a2018`  | 74          | 16             | 39           |

**The one that was already failing is `quotations`** and is unrelated. **Thirty-eight are the
promotion**, across `kinds` - signatures, columns_are_found_by_name, catalog_is_current,
against_the_release, a_name_may_carry_a_dash - and `game-console` - worked, vocabulary, petri,
nogain, first_release, fired, dumps_are_current, declare, closed_sets, biomes_can_be_held.

**The test names say what they read.** `every_word_in_the_data_file_is_one_the_release_declares`.
`the_release_tables_are_the_ones_in_this_crate`. `the_file_of_kinds_and_the_release_declare_the
_same_words`. **Each is a check that the data file agrees with the markdown table** - which reads
the rendering as the authority and the data file as the copy, exactly inverted.

## Why nobody knew the number

**Every one of the thirty-eight was individually reasonable.** A check that the data and the
release agree is a good check; it is the direction of the dependency that is wrong, and the
direction is invisible in any one of them. **The count is what makes it a finding**, and the count
did not exist until something broke.

**And the first attempt at the count was wrong.** `cargo test` stops at the first failing target,
so a fail-fast run reported **three** - true of everything up to `biomes_can_be_held` and of
nothing else. It was reported to Sean as three before being corrected against a baseline. **A
right number about a population nobody named**, which is the fifth instance of that shape in two
days - see [four counts in one day](2026-09-20-four-counts-in-one-day.md).

## What it argues, and what it does not

**It is the evidence for [`P-530`](../../decide/proposals.md) arriving as a measurement rather
than as reasoning.** Sean's stated reason for an executable specification is that the prose grew
past what one person could hold. This is the same fact from the other side: the prose was
load-bearing for thirty-eight checks, **and the only thing that noticed was the tests.** Not the
padder, not the outbox index, not two lanes reading the diff.

**It sharpens where the tests live**, which `P-530` deliberately leaves open. Named by the code
lane, 2026-09-21: *a test that reads a prose table is reading a rendering as a source, and
thirty-eight of them did.* **A test whose input is a rendering inherits the rendering's authority
problem**, so wherever the tests end up, what they read has to be a stating form.

**It does not argue that the thirty-eight are bad tests.** They caught the change, loudly and
immediately, which is what they are for. **What it argues is that they were pointed at the wrong
file**, and that thirty-eight of them being pointed the same wrong way was a fact about the
repository nobody held.
