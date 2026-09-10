# Handoff to the specification lane

**Research, 2026-09-09. Addressed to `spec`.** What a day's re-encoding produced that is *not* in
the report or the outbox — which claims are computed and which are asserted, what Sean corrected,
and what would be expensive to learn the way this lane learned it.

[Research](README.md) · [The recipe report](formulas.html) · [Outbox](outbox.md)

## Read this before trusting anything on the page

**Ten checks run against the data, and each is poisoned** — made to fail on demand — because a green
that cannot be turned red means nothing. `python tools/research/formulas/check.py` prints the poison
results first, and if any says `POISON FAILED` the greens below it are worthless.

**But a check only covers what it reads.** Grade every claim before acting on it:

| Grade        | What it means                                                   | Which claims                                                                                                                    |
| ------------ | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| **Computed** | A check derives it from the data and would go red if it changed | conservation, boundedness, containment, reachability, the editor test, minimal control, the capture arithmetic, the trait drift |
| **Executed** | `play.py` fires the recipes and it happened                     | the loop closes; two pioneers breach and one does not                                                                           |
| **Asserted** | This lane wrote it down after reading                           | **the whole divergence table**, and every sentence of prose on the page                                                         |

**The divergence table is the thing you most need and the least computed.** It is fourteen rows of
this lane's judgement. Check 7 is the only mechanical comparison against the release, and it covers
the *Traits* table alone.

## Six things Sean corrected, each a fact worth having

Each of these was this lane being wrong in a way that reading the documents did not prevent.

1. **An ark is only ever in orbit.** It lands to deploy and stops being an ark. The release declares
   a territory capacity of **2 arks** and no recipe can reach it.
2. **An orbit is a place in its own right**, not a shadow of a territory. It carries no stored trait
   *in this release* and will hold solar collectors and starbases. This lane argued it might be
   derivable; **"no traits yet" is not "no traits ever"**, and no check over a first release can tell
   those apart.
3. **The force clash is at the boundary, and entering is the cost.** This lane had reasoned entering
   must be free because `found by land` consumes a pioneer *in* the territory. Wrong — and the right
   answer is better, because the contest belongs on `move`, which exists, and the ark stops being a
   special case.
4. **Force is `max` unorganized and `sum` organized.** A citizen makes 1 passively; a garrison is
   force **0**, which is the design — a thing that holds a territory and contributes no force of its
   own is an *organizer*. Units that move are organized by construction.
5. **`disorder` and `perish` were one recipe.** This lane wrote them separately, gave them different
   consequences for no reason, then filed the inconsistency as a design question. **The duplication
   was hiding a decision.**
6. **Every territory's biome and nature are published**, in `reports/territory-N.md`, generated from
   the game. This lane searched `spec/` and `releases/` twice and never looked in `reports/`.

## The failure mode that recurred four times in one day

**A family hides a kind, and the check returns a plausible number instead of an error.**

- Check 2 scored `work` at zero metal because it produced the family `resource` — the game's only
  metal source, invisible. Fixed by `X-17`.
- Check 1 had the identical hole and nothing exposed it until a recipe destroyed a `unit`; `move`
  takes a unit out of one place and puts it in another, so its two family lines cancel.
- Check 8 grounded families on one side only and reported two gaps it had invented.
- An ad-hoc reachability check read `work` as producing `resource` and declared three recipes
  blocked for want of energy.

**If you compute anything over this data, ground the families first.** `resource` is food, metal and
energy; `unit` is ark and pioneer; `place` is territory and orbit; `thing` is everything. Every one
of those four bugs produced a number that looked reasonable and answered a narrower question than
the one asked.

## What this lane would check first, in your position

1. **`X-21` and `X-23` are defects on any reading**, not design changes. A store that nothing can
   fill and a unit that nothing can fuel are broken however the game turns out. If you incorporate
   two things, incorporate those.
2. **`X-14` and `X-20` are Sean's decisions already taken**, unwritten. Cheapest rows on the list.
3. **The primitive set is the expensive one.** `change`/`require`/`set`/`each`/`some`/`call` is a
   different vocabulary from the release's four roles, and adopting it is not a correction — it is a
   rewrite of *Recipes* and everything that reads it.
4. **Do not adopt the prototype's deletions without asking.** `fuel`, `make-world` and a territory's
   ark capacity are gone because Sean said the prototype could remove things. That leave does not
   extend to `releases/`.

## What is still soft, and this lane says so plainly

- **`X-22`'s force rule is stated and unwritten.** The two-test reading is verified arithmetically
  (check 11) and is not in any recipe.
- **The one-in-sixty-six thing.** The two jungles are exactly the two territories missing a resource.
  `biomes_of` computes from geometry and *Territory resources* is authored by hand, so the two are
  independent. This lane cannot tell whether that is design or coincidence and has not guessed.
- **`play.py` is not the game.** It is this lane's evaluator over this lane's data. It shows a
  sequence of firings reaches a new ark. Whether the *game* plays that way is the code lane's.
- **Every recipe is now buildable from an editor** — 410 of 410 tokens selectable — and that does
  **not** mean the game is finite. Seven assumptions remain about the notation rather than the menus.

## Nobody checks the column they do not write

**Named by the specification lane on the night of 2026-09-09, after three instances in one day
across three lanes.** It belongs in whatever report Sean reads, and it is not this lane's to write
up — recorded here so the instances sit together.

| Claim                             | Verified against       | Never opened   | What was there                                  |
| --------------------------------- | ---------------------- | -------------- | ----------------------------------------------- |
| `X-19` — no territory has a biome | `spec/`, `releases/`   | **`reports/`** | every biome and nature, generated from the game |
| `X-23` — nothing fuels a unit     | `releases/`            | **`crates/`**  | `fuel` already built as a decrementing counter  |
| `S-48` — *node goes*              | `releases/`, `crates/` | **`spec/`**    | `spec/logistics.md:35`, *the node it works*     |

**The shape is the same three times and it is not carelessness.** Each verification named a
**proper subset** of the columns its claim covered, and each returned **a plausible number rather
than an error** — `0 occurrences`, `no recipe fuels a unit`, `no territory has a biome`. All three
were true about what they read.

`CLAUDE.md` already has the words: *the instrument answers a narrower question than the one asked*,
and *a right number about the wrong thing invites no question*. **What it does not yet say is which
narrower question keeps getting asked**, and it is always the same one: the columns the author
happens to write.

**Two of the three are this lane's and one is the specification lane's**, which is the only reason
it is visible at all. A lens checking a producer would have found the producer's; a producer
checking a lens would have found the lens's. **It took both directions on one night to see the
shape**, and neither lane could have named it alone.

**What would catch it.** For a claim of the form *X is gone* or *nothing does Y*, assert over all
four columns — `spec/`, `releases/`, `crates/`, `reports/` — rather than the ones that were open.
The specification lane has proposed building that check and this lane agrees it is the right one;
it is worth more than any single finding either of us made tonight.

## Reaching this lane

`SendMessage` to `4x research`. A message only arrives if the session is running, so anything that
must survive goes in [the outbox](outbox.md) first. **If you find a real problem this lane can
verify, say so and it will be fixed rather than argued** — that is Sean's instruction, and being
refuted is the lens working.
