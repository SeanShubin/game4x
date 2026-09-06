# Review of `ba9bd41..217dcba`, 2026-09-06

Requested by the code lane after six commits. Read from the tree at `217dcba`, not from the
summary that came with the request.

**The gate is green independently.** `cargo test --workspace`: 51 targets, **504 passed, 0
failed**, working tree still clean afterwards. Seventeen targets report `ok. 0 passed`, which
is the shape this lens is required to distrust - all seventeen are binaries and libraries with
no unit tests of their own, and the 504 sit in the other thirty-four.

**My first instrument was wrong and is worth recording.** I ran the suite through `tail -40`,
read the eight `test result:` lines that survived, and had *8 targets, 0 passed* in front of me -
a plausible number about a narrower question, which is `C-28` exactly. Re-run without the pipe.

---

## 1. The `Q-47` check cannot see the spelling `Q-47` was filed about

**Where.** `tools/outbox/tests/architecture.rs:186` - the predicate in
`only_a_generator_or_a_check_reads_a_report`.

**What.** The predicate admits a literal that *is* `reports` or *starts with* `reports/`. Two
files reach the directory by relative path - `.join("../../reports/catalog.md")` - and match
neither. One of them, `prototypes/kinds/src/main.rs`, is neither a generator in `src/bin/` nor
a check under `tests/`, so it is a trespass the check does not report.

**Why.** The check passes for two compounding reasons: the predicate misses the file, and so the
file is never tested against the rule. `Q-47`'s whole content was *two spellings are in use and
searching for either alone finds some readers and misses the rest*. This is a third spelling,
and it was in the tree when the check was written.

**Why nobody caught it.** The population assertion is `readers.len() >= 5`, and the message says
*the rule was written against five*. It found five, which agreed with the number in my `Q-47`
report - **and that number was already stale.** I re-derived it this morning, before the review,
and said so: the population is seven. The two counts agreed because they share the computation,
not because either was right. The self-check rule already names this: sharing the inputs is
fine, sharing the computation is what makes it circular.

**The poison was inert in the direction that mattered.** The probe named `reports/state.md` from
`crates/game-model/src/` - a literal starting with `reports/`, inside the predicate's sighted
region. It proved the handled spelling is handled. Nothing in it could have found the blind spot.

**Verified, in a clone rather than the shared tree.** Adding one clause -
`|| literal.contains("/reports/")` - and running the real check:

```
test only_a_generator_or_a_check_reads_a_report ... FAILED
  prototypes/kinds/src/main.rs
All 7 reader(s): [... dump-state.rs, dump.rs, dumps_are_current.rs, turns_reconstruct.rs,
                  prototypes/kinds/src/main.rs, prototypes/kinds/tests/catalog_is_current.rs,
                  generated_files_are_padded.rs]
```

Seven readers, one trespass, and the trespass is a real generator sitting in `src/main.rs`
instead of `src/bin/`.

**Whether.** Worth fixing now, and the harder half is not the predicate. Whether
`prototypes/kinds/src/main.rs` is a violation or the rule is too narrow is the code lane's call:
a generator that is a crate's only binary has no reason to be in `src/bin/`. Widen the predicate
either way, and raise the population assertion to the number it then finds, because `>= 5`
tolerates losing two readers in silence.

## 2. `phase` declares no values, so `play` is an eighteenth forbidden word

**Where.** `releases/first-release.md:114` - the `phase` row - against
`scenario/expected/play.4x:9`, `{game phase:play ...}`.

**What.** `C-37` counts seventeen words in the data file that `P-284` forbids. I classified all
**49** distinct words in that file independently against the release's own tables and got the
same seventeen, word for word. There is an eighteenth: **`play`**.

`P-288` landed the `phase` row this morning, specifically so `P-284` would pass on it. Its
Values cell reads *before it starts, or once it has* - which names neither value. The words
`play` and `design` appear nowhere in `releases/first-release.md`, and nowhere in
`spec/turn.md`.

**Why it is the same defect as `turn`, one step down.** `C-37` already found that both its
instruments admitted `turn` because a values cell was prose - *food per turn*. `phase` is prose
in the other direction: it admits nothing, so the word actually in the file is undeclared. The
count went wrong three times, in three instruments, from one cause: **a trait whose values are
described rather than named.**

`phase` is the only closed-set trait in the table that neither names its values nor points at a
table that does. Every other one does one or the other - `yes or no`, `held by a player, or
unclaimed`, `one of the kinds`, `one of the resources`, `one of the biomes`.

**And `C-37`'s proposed rule already rejects it**, which is the good news: *a word is admitted if
it is a value of a trait that names a closed set*. `phase` names no closed set, so a check built
on that rule reports `play` on its first run. The rule is right; the count under it is eighteen.

**Two words that are neither, and are not a defect.** `unit` and `place` are **families**, and
`P-284` as written admits *a kind, a trait, or one of a trait's values* - not a family name. The
file uses both correctly. A check built on `P-284`'s literal words would flag them and the code
lane would then "fix" correct usage.

**Whether.** Worth a decision now, and it is not the code lane's: naming `phase`'s values is a
row in a table that belongs to Sean. Until it lands, `S-47`'s *three traits have to be declared*
is answered for `phase` in name and not in value, and both producers currently believe it is
settled - `docs/notes/proposals.md:122` already writes `{game phase:play}` as the target form.

## 3. Two corrections to my own open items, both found by this review

**`Q-47`'s population is seven, not five.** Stated in the item as five; re-derived today. The
figure had already propagated into the check's assertion and its failure message before I
corrected it, which is the cost of a number passed on rather than computed - the fourth time
this lens has done it.

**`Q-50`'s discriminator now has counterexamples, and this commit range created them.** I wrote
that *a joined wrap is on one physical line by construction, and aligned columns are across many
by construction*, measured **38 of 38** over the tree, and said explicitly it was a measurement
and not a theorem. `tools/outbox/tests/promotions.rs:502`, `:532` and `:540` are alignment -
continuation lines indented under a printed message - carried on one physical line by `\n`
escapes rather than real newlines. Three counterexamples, one day later.

So a check restricted to single-physical-line literals would now report these three as joined
wraps. The refinement is cheap and specific: **a run of spaces immediately following an escaped
newline is alignment.** It does not rescue the general claim, and the conclusion is unchanged
and now better supported - the check prints, and never asserts.

## 4. What I checked and found nothing wrong with

Recorded so the next report does not re-derive it, and so *nothing found* is distinguishable
from *nobody looked*.

**`S-48`, the request to check no case was lost.** The three re-stated tables keep their counts -
6, 6, 5 - and each asserts its own count against a sentence saying what the cases are for. Both
boundaries of `can_build_extractors` are exercised, including `(Food, 0, 6)`, which is the case
that could not be written in the old shape. Nothing was lost that is expressible: the old tables
turned on one resource having two densities, which the model no longer permits.

**`most_in_one_turn`'s closed form is correct.** Working *k* of *cap* food extractors leaves
`k(density - 1)` spare hands, which is monotone in *k*, so `k = cap` is optimal and the search it
replaced could not have found an interior maximum. The claim *working one more is never worse*
holds for the stated reason, which - given `C-37` and `Q-54` - is worth saying explicitly.

**`C-35`'s window report is accurate.** Run with `--nocapture`: *0 of 4 named exceptions were
inside it*, naming `P-214`, `P-216`, `P-236`, `P-195`, and the window reaches back one day. The
lane reported its own check's dead half rather than leaving it looking live, which is `C-33`
being applied rather than cited.

**`Q-50`'s eighteen are gone.** All six named lines in all four named files are clear, re-derived
rather than taken from the commit message. Nothing in the range reintroduced one.

## 5. Noted, and small

`crates/game-model/src/territory.rs` - `most_in_one_turn` uses `density.saturating_sub(1)` and
the comment gives its reason as *a density-zero or density-one food extractor buys no hand at
all*. Density one is covered by a case; **density zero is not**, in either table. The branch is
load-bearing under `u32` and changing it to `density - 1` leaves every test green. One row, while
the file is open.
