# The README is the deliverable, and two of its counts are refuted by the crate

**2026-09-12.** Reviewing `prototypes/hex-torus-view` over `fe227c0~1..4e3c363`, the range the code
lane named. The three places they pointed at are sound; the finding is one directory up from them.

## Why a README counts as more than documentation here

[`docs/prototypes/README.md`](../../docs/prototypes/README.md): *that answer is the deliverable; the
code is a byproduct.* A prototype exists to answer a question, and the document is where the answer
is. **So a false count in this README is the same kind of defect as code contradicting `spec/`** -
which is the finding this lens rates highest, because nobody else is looking for it.

## The finding

**Where.** `prototypes/hex-torus-view/README.md:25` and `:46`.

**What.** Two counted claims, each contradicted by a test in the same crate and by the README's own
later paragraphs.

| README says                                | The crate says                                                             |
| ------------------------------------------ | -------------------------------------------------------------------------- |
| `:25` *it comes back three at every size*  | six of the ten axis-aligned sizes take **four** - `C = 4, 5, 7, 8, 10, 11` |
| `:46` *Three of the twenty have a partner* | **six** of the twenty have one; fourteen do not                            |

**Measured**, at `4e3c363`, by calling the crate's own functions:

```
worlds: 20
worlds WITH a partner: 6 -> ["12<->36", "27<->81", "48<->144", "36<->12", "81<->27", "144<->48"]
worlds WITHOUT: 14
axis-aligned sizes needing FOUR colours: 6 of 10 -> C = [4, 5, 7, 8, 10, 11]
```

**Both are already right further down the same file.** Line 66's table says *three where `3 ∣ C`,
four otherwise*, and line 75 says *four colours at `C = 4, 5, 7, 8, 10, 11`*. Line 141 lists
`three_colours_suffice_at_every_size` without saying it covers ten of the twenty. So the document
disagrees with itself, and the true half is the half a reader reaches second.

**Why.** **Two different causes, and only one of them is staleness.**

- `:25` was true when the crate had one family. The axis-aligned family landed in `5ddc371` and
  `435427f`, the later paragraphs were written for it, and the headline was not revisited. Nothing
  connects them - `docs/notes/nothing-removes.md`.
- `:46` is not stale at all. **The denominator moved and the numerator did not.** `partner_of`'s own
  docstring says *three of the ten pair today*, which is correct: three of the folded ten have a
  partner. The README changed *ten* to *twenty* and kept *three*. Three **pairs** is six **worlds**,
  and `the_page_carries_the_hover_and_the_pairing` asserts exactly that - `worlds.len() - 6` with no
  partner.

**Whether.** Worth doing now, and it is two sentences. Not because anything is broken - the code is
right and the tests are right - but because **a reader who stops after *What it draws* carries away a
false headline about the prototype's own result**, and this document is the result.

## What this lens checked and is not filing

**Their three, in order, and the answer is no to all three.**

**The script-text checks are honestly labelled and the join is counted.** `page.contains("pointerover")`
does read the input rather than the outcome, and the docstrings say so. What makes them worth keeping
is the half this lens went looking for and found already there: the script reads `el.dataset.full`,
and `the_page_draws_one_bright_world_and_six_echoes` asserts `data-full` appears `7 * cells` times
over `checked == 20`. **The read and the emit are both counted, so the defect that exists only in the
join is covered.** They catch deletion rather than breakage, which is what they claim.

**The spelled-number table is not a maintenance trap.** Any change to either ladder moves `drawn`
off the five keys, and the lookup panics with *`{drawn}` worlds and this test has no word for that
many* - which names the file, the number and the fix. **A trap is a check that goes quiet; this one
goes loud and says what to type.**

**The literal counts after loops are the correct trade**, and the reason is in this repository's own
history: `docs/notes/checks-outlive-examples.md`. Four tests reddening at once is the cost of the
property being checked over every case rather than one, and the alternative - deriving the count from
the thing under test - is what `Q-83` was filed about eight hours ago.

**And one hypothesis this lens had, poisoned and withdrawn.** `tests/wrapping.rs` and
`tests/colouring.rs` iterate `sizes()`, the folded ten, so five core properties are never asserted of
the axis-aligned family. That looked like the whole family arriving under a test file written for one.
**It is not**: poisoning `domain()` for the axis-aligned family - `1..=C` instead of `0..C`, so it
disagrees with `reduce` - reddens five tests, because the drawing tests iterate `both_families()`.
The population is narrower than it looks in one file and is covered in another. **Recorded because a
poison that goes red is the only reason this is not filed.**
