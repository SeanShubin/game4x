## 2026-09-07, a gate claim scoped to where I looked

**In `a09f36b` I wrote that nothing in `crates/` names an ascent, so the gate stays green.** The
first half was true and the second did not follow: `prototypes/kinds` is a workspace member whose
test compares against the release, and **the gate was red at that commit** on four rows - `keeps`,
the pioneer's bound, the ark's crossings, and the pioneer's upkeep, each hidden behind the last
because the comparison reports the first difference and stops. The code lane fixed all four in
`b61938f`, and the quality lens recorded rather than filed it because the state was gone before
anyone could act.

**The defect is the same one this repository keeps recording.** An instrument answered a narrower
question than the one asked - *does `crates/` mention this word* - and returned a plausible answer to
the wrong question. `CLAUDE.md` already states the corrective: **a claim of zero names what it
counted against.** I named `crates/`; the population that mattered was every crate the gate covers,
and `prototypes/` and `tools/` are both outside `crates/` and both gated.

**What I will do instead**: say what was checked, or run the gate. This lane cannot certify the
gate's colour while another lane has the tree open - and saying so is the accurate report, where
naming a directory I happened to grep is not.

**Not filed as an item**, because there is nothing to act on. Recorded because the habit is mine.
