# Decisions

**Derived.** Written by Claude. Not binding, and **not the specification** - these are choices only
Sean can make, held where he will find them.

[Proposals](proposals.md) · [Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What is here, and what is next door

`docs/process.md` -> *What I read, and what I do* says it: **this file holds choices only Sean can
make; [`proposals.md`](proposals.md) holds words for him to approve.** An item lives in one at a
time. It sits here while any question in it is unanswered, and **moves to the proposals file when
the last one is answered** - so this file only ever holds live questions.

Questions about the specification's own content are not here. They stay at the bottom of the `spec/`
file they concern, where the context is.

Every item carries the same four fields as a proposal, and says `asks a decision` rather than
`asks approval`. The *Accepted* ledger stays in [`proposals.md`](proposals.md); nothing lands from
here without first becoming a proposal.

## Open

### P-348 - `R-8` is built and no two kinds behave alike, which may be the answer or the defect

**to** sean - **status** open - **raised** 2026-09-07 - **kind** the code lane's `C-64` - **asks**
a decision - **into** `releases/first-release.md` -> `R-8`

**`R-8` is built as written and its grouping is empty**: fifteen kinds, fifteen signatures, every
group holding one. The half of the capability that says *kinds with the same signature are shown
together* is satisfied by a report that never shows anything together.

**It is a fact and not a bug, and the check names its population** - which is the part I would want
to know before deciding. The agreeing-pair count is **zero over 105 pairs**, which is every pair of
fifteen kinds, so **the day two kinds collide the check fails** and the claim stops being true
quietly. And the traits alone *do* collide: **11 of the 15 carry exactly the traits another one
carries**, and every such pair is then separated by the recipes that name it.

**Three readings, and they are the code lane's words because they are the right three.**

1. **This is the right answer.** Fifteen distinct kinds is what a small release should have, and the
   report's job was to tell you so
2. **The signature is too fine.** Quantity is already excluded; excluding the recipe's name as well,
   so a signature is only *which roles a kind plays*, would group several. **That is a different
   definition of behaving alike**, and `R-8` states the current one in as many words
3. **The release is what should move.** If two kinds ought to behave alike and do not, the tables say
   something you did not intend, and the signature is what found it

**If it is 1, one thing still changes**, and it is presentation rather than rule: **fifteen groups of
one reads as a broken report.** *No two of the fifteen kinds behave alike, over 105 pairs* is the
same fact and reads as a finding. That is the code lane's to build once you say the finding is the
answer.

### P-347 - No scenario fires `move`, and `P-340` just made the missing case explicit

**to** sean - **status** open - **raised** 2026-09-07 - **kind** the code lane's `C-63` - **asks**
a decision - **into** `releases/first-release.md` -> Scenarios, if the answer is yes

**Checked rather than taken on report**: seven files in `scenario/commands/` and **not one `{move`
among them.** The Ark crossing to territory 2 was the only one in the repository, and `P-342`
removed the Ark that made it.

**Every other player recipe fires somewhere.** `move` is declared, has a command, and is exercised
only by the worked example `R-7` builds for it - **a case written to exercise it**, which is not the
same as a rule meeting the other rules. `C-54` is this from the other side: a coverage check read
what a file said rather than what ran, and stayed green for weeks while `move` had never fired.

**And the obvious case is one you promoted today.** `P-340` put into `spec/unit-types.md` that a
Pioneer is taken apart **when it founds**, that *moving is not founding*, and that it **may cross
ground its player already holds**. That rule is what makes a pioneer able to reach a frontier that
is not next to where it was built - and **nothing anywhere demonstrates it.**

**Two ways.**

- **A scenario fires `move`** - a pioneer crossing its own ground before founding. It exercises the
  recipe and shows `P-340`'s rule at the same time. **Which scenario is the question inside the
  question**: `play.4x` says it touches everything a typical game uses, and crossing held ground is
  typical on twelve territories; `spread.4x` is where spreading already lives
- **It stays unfired**, and the release says so rather than leaving it to be rediscovered

**The code lane has already made this safe either way**, which is worth knowing before choosing:
`fired.rs` names `move` as its **one** exception and asserts the list is exactly `["move"]` - so a
second unfired recipe fails the gate, and putting the move back fails until the exception is
deleted. **Neither answer can be half-done.**

### P-346 - Three statements fix what a move costs, and only one of them is a mechanism

**to** sean - **status** open - **raised** 2026-09-07 - **kind** the quality lens's `Q-68` - **asks**
a decision - **into** `releases/first-release.md` -> *Units and structures* and *Recipes*, and
`spec/units.md`

**The same fact is stated three times.** The quality lens found it looking for what `move` becoming
an ordinary recipe left behind, and I checked all three.

| Where                                       | What it says                                                  | Does it work?                   |
| ------------------------------------------- | ------------------------------------------------------------- | ------------------------------- |
| `move`, its energy row                      | `consume` `1` `energy` from *that unit*                       | **yes** - this is the mechanism |
| *Units and structures*, the `A move` column | `1 fuel`, for an ark and for a pioneer                        | **no**                          |
| `spec/units.md`                             | *Moving burns a unit of it, and a unit with none cannot move* | it states the rule              |

**The column cannot work, and that is the finding.** The recipe consumes a literal `1`, so **a row
saying `2 fuel` would change nothing** - the unit would still spend one. A per-thing column beside a
rule that is no longer per-thing can only repeat what the recipe already fixes.

**Two ways, and I recommend the second.**

**Delete the column.** Two statements remain: the specification states the rule and the release implements it, which is the ordinary relationship between them. Nothing in the game changes. **What it costs is the ability to say a unit moves for more** - traded for tidiness, and not recoverable without a later proposal.

**Make the recipe read the column**, so the cell becomes `the unit's move` rather than `1`. **This is not a new form: the release already declares it.** *A quantity is a whole number. It is written in the recipe, read from a trait of one of the ingredients, or read from a trait of a named ingredient.* The second of those three is exactly this, and `upkeep` is it in use - consuming *the thing's upkeep* in food. One cell changes, the column starts
doing work, and **a unit that costs more to move becomes expressible** without another decision
later.

**What the second costs**: `spec/units.md` would have to widen too, because *burns **a unit** of it*
fixes the number in prose. Something like *moving burns fuel* leaves the amount to the recipe, which
is where you put cost when you removed the line from `spec/orbit.md`.

**Two things the lens checked so they are not swept up with this**, and I confirmed both:

- **`Crosses` is read by the recipe** - *joined to `$from` by an edge the unit crosses*. It is an
  ingredient's trait, not a second copy of a rule
- **`Fuel` is the tank's size** and is a different fact from the cost of one move

**Nothing here is urgent.** Every mobile unit costs one today, so both ways describe the same game;
the difference is only whether a later unit can cost more without a proposal.
