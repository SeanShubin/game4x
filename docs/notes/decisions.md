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

### P-422 - `R-6` asks how much of the planet has to be played, and has pointed at an empty queue since 2026-09-05

**to** sean - **status** open - **raised** 2026-09-11 - **kind** recovered, from `R-6`'s own note and the code lane's `C-95` - **asks** a decision - **into** `releases/first-release.md` -> `R-6`

**`R-6` has carried the question in prose and never as an item.** Its third bullet reads *what is
now in question is not whether it can be played but how much of it has to be - see the proposal
queue*, and **the queue has never held it**: 0 hits for that question across `proposals.md` and
this file, searched for by concept as well as by wording, against a population of **418 rows in
the Accepted ledger** plus every open, rejected and withdrawn item. So the pointer has been
dangling for six days. It now points here.

**What made it urgent is that the answer stopped being hypothetical.** `C-95` measured the
committed scenario on 2026-09-11: **twelve claimable territories, two founded, none at maximum
output**, `is_fully_exploited` and `has_won` both false. It launches an Ark at line 164 of 133
commands, so the *vetted when*'s second half holds and its first does not. `tests/fully_exploited.rs`
derives the remaining bill as **57 buildings and 114 commands**. **This is not a near miss**, and
`R-6` cannot be vetted as it stands.

**Three ways out, and the choice is which you want to look at.**

1. **Commit the full scenario.** `play.4x` grows by **at least** the 114 commands the bill names -
   that figure counts labor-and-build pairs only, and founding the other ten territories, moving
   pioneers to them and ending the turns are all on top - and `R-6` is then vetted by reading a
   scenario nobody will read line by line
2. **Play a smaller planet.** A scenario on fewer territories finishes, and *fully exploited* is
   demonstrated on something a person can hold in their head - at the cost of the vetted planet not
   being the one the game ships
3. **Reword the capability.** `R-6` becomes what the current scenario already shows, and *reaching
   a fully exploited planet* moves to a later release

**This lane has no recommendation**, because the three differ in what you would be looking at when
you vet it, and that is the whole of the question. **What it can say** is that 1 and 2 both keep
`docs/process.md`'s *the definitions and the commands are enough to derive the data dump by hand*
literally true of a scenario you could work through, and 3 does not.

### P-421 - `put` is a role in twelve rows that no document defines, and `limit` is defined and used nowhere


**to** sean - **status** open - **cited** `d29dc6d`, where the code lane recorded that `C-88` stays open until this comes back - **raised** 2026-09-11 - **kind** entailed, from the code lane's `C-88` - **asks** a decision - **into** `releases/first-release.md` -> Recipes


**One sentence declares the roles and it is out of step with the table under it in both
directions.** `releases/first-release.md:186` reads *`Role` is one of `require`, `limit`, `consume`
or `produce`*, and then defines each of the four. Counted over the table's 81 role cells: `consume`
29, `produce` 26, `require` 14, **`put` 12**, **`limit` 0**.

**`put` arrived twice without a definition.** `P-399` gave it to `move`; `P-411` undid the rest of
`P-399` and left `put` behind on the readiness rows. Neither promotion said what the word means, so
twelve rows are read by inference - which the code lane did, and `C-88` states the assumption it
built under.

**What the twelve rows do, so the question is cheap to answer.** Eleven name a trait and a value -
*laboring one less*, *defending at its maximum* - and set it. The twelfth is `move`'s, which names
the trait **and** a `Where` of `$to`, so the unit ends at `$to` with `moving` one less. **All twelve
read as one thing**: a statement about the state the thing is left in, rather than a quantity
flowing anywhere. That is why its `Qty` cell is blank, and it is what distinguishes `put` from
`consume` and `produce`, which move quantities.

**Two questions, and the second is smaller.**

1. **Is that what `put` means** - the row states the state the thing is left in, its `Qty` always
   blank - and if so, what sentence declares it beside the other four?
2. **Does `limit` stay?** It is declared and defined in that sentence and no row uses it. Keeping a
   role with no instance is a reasonable thing to do deliberately, and nothing says it was.

**This lane has not written the sentence**, because what `put` means is the choice and a definition
offered here would resolve it quietly. **Nothing is blocked**: the code lane is built and green
under its stated assumption, and `reports/petri.md` and `reports/nogain.md` both read the `put` rows
rather than refusing them.


