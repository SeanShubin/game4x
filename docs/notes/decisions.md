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

### P-352 - `CLAUDE.md` names the staging hazard correctly and gives an instruction that cannot close it

**to** sean - **status** open - **raised** 2026-09-07 - **kind** the research lens's `X-7` - **asks**
a decision - **into** `CLAUDE.md` -> Perspectives, the staging bullet

**Only you can approve a change to `CLAUDE.md`, which is why this is an item rather than an edit.**

**The bullet's diagnosis is right, and I checked that rather than relaying it.** It already says
`git commit` commits the index rather than the caller's changes, and already says *this is not a
caution about your own carefulness; the hazard is someone else*. **The code lane's account of it -
that it reads as a caution about being careful - is wrong**, and I would have repeated that if I had
not opened the file.

**What is missing is only the remedy, and that gap is real.** The instruction is *stage by name*.
**Staging by name bounds what you add, not what gets committed**, so it cannot close the hazard the
bullet correctly describes. It fired again today: the research lens staged by name, exactly as
written, and 21 lines of the code lane's file went into the research lens's commit. Twenty-six lines
a fortnight ago, 21 today, content intact both times.

**Checking for the lock first does not help either** - the window is after the check.

**A carrier already exists and nothing points at it.** `git commit -- <paths>` commits the named
paths from the working tree and never the index, and `hooks/post-commit` was wired in `a60def3` for
the residue that leaves. **The word `pathspec` appears in that hook and in the quality lens's own
files, and nowhere in `CLAUDE.md` or `docs/process.md`.** The code lane has used the form for four
commits today. So this is `P-327`'s principle finding its first case: **the carrier was built and
the sentence was never changed.**

**The choice, and it is a real one.**

- **Tell every lane to commit by pathspec.** The hazard stops being reachable rather than being
  warned about. **The cost is a second git idiom** every lane has to know, in the one file every
  lane reads first
- **Correct the bullet without adding the idiom** - say that staging by name bounds what you add and
  not what you commit, so the race is survivable rather than preventable. **Cheaper, and it leaves a
  known hazard open** on the argument that it has fired twice with the content intact both times

**I have drafted no words**, because a proposal that asks a decision carries none until you have
made it. Say which and the wording comes back as an approval.

### P-351 - Two words in the data file that the release does not declare, and the exception list is full

**to** sean - **status** open - **raised** 2026-09-07 - **kind** the code lane's `C-46`, which has
carried both since 2026-09-06 - **asks** a decision - **into** `releases/first-release.md` ->
*Kinds* and *Traits*

**My delay, not theirs.** `C-46` put both of these to me on the 6th and I have not put them to you
until now.

**Both are named exceptions in `crates/game-console/tests/vocabulary.rs`**, which fails if either is
repaired and fails if a third appears - so neither can outlive itself. That file's own warning is
why this is not indefinitely deferrable: *past about two, a list of exceptions is the thing being
checked written twice.* **It is at two.**

**1. Is `game` a kind?** It is the root of every data file - `{game phase:play ...}` - and
`spec/logistics.md` needs a thing that is in nothing for containment to be a tree. **The word is the
specification's own**, used by `spec/console.md` and `spec/invariants.md`. The release's *Kinds*
table declares no `game`.

- **A kind**, and it gets a row like any other
- **Not a kind**, and then something has to say what the root is instead, because the file still
  writes the word and the check still has to admit it

**2. Is `manned` a *Traits* row or a deletion?** `Trait::Manned` is *citizens working here this
turn*, kept on a garrison by the model since `P-276` and declared by no row.

**The deletion argument is that no rule reads it** - `held_force` stopped when a garrison's own
force went to zero - and `thing.rs` records the general form: **an unread representation cannot
diverge detectably.**

**The argument is weaker than `C-46` states it, which I found by looking rather than by taking it.**
`manned` **is** read, by `crates/game-console/src/report.rs:149` and `:280`, which print it. It is
read by a presentation and by no rule. **So deleting it changes what the report shows**, and that is
the actual cost rather than nothing.

- **A row**, and the trait is declared and stays printed
- **A deletion**, and the report loses a line it currently carries

### P-350 - Filing this study as an item is what makes it unrunnable, and that is the finding

**to** sean - **status** open - **raised** 2026-09-07 - **rewritten** 2026-09-07, and again on
the 7th - **kind** the
research lens's `X-5`, lifted - **asks** a decision - **into** nothing; it authorises a study or it
does not

**The question, in one line.** On some future restart, should one instance be given one task as
ordinary work - *what are a lens's jobs?* - with a record kept of which files it opened before
answering?

**Everything else about this item has collapsed into that.** Two of the three tasks now have
observational evidence against them from the crash, so they can only confirm; `P-315` is unrunnable
because the sentence that caused it was repaired. **One task is left.**

- **What it tests.** The lens's own README restates its jobs and is four days behind
  `docs/process.md`, which owns them. `X-3` predicts a cold instance **fails** - reaches the README -
  and that a long-running one might not. **If it reaches `docs/process.md` unprompted, `X-3` loses
  its central claim** that clearing context makes two of the phenomena worse
- **Why it is worth a person's attention at all.** The outbox architecture assumes an instance is
  replaceable by a fresh one that reads the files. **That assumption is why `pending.md` and the
  items are worth keeping, and it has never been tested**
- **When it can run.** Only after you answer. While this item is open, `pending.md` names it and the
  instance reads that first - which is the finding below
- **What it costs.** A few minutes of one restart, and no writes. **Declining costs nothing**:
  `X-3`'s answer stands as a derivation and says so

**Time-sensitive, and that is the only reason it is in front of you today.** `X-5` has said *worth
doing eventually rather than now* since 2026-09-06, correctly, because it needed an instance started
for the purpose. **You are starting instances anyway**, and the study needs a reader that has read
the files and nothing else - which is what a restarting instance is for its first few minutes.

**The ask is three ordinary tasks, not a study.** `X-3` answered `S-57` by **classifying** its cases
rather than measuring them, and said so. The design is
[the cold-instance study](../../lenses/research/2026-09-07-the-cold-instance-study.md), and its
shape is the part worth your attention: **the instance is given its ordinary lane prompt and three
pieces of ordinary work, and is told nothing about `S-57`, the report, or a study.** Each task can be
done correctly only by opening one file, and each has a plausible wrong answer available without
opening it. What is recorded is which files it opened before answering.

**Run as the quality lane, because none of the three tasks writes anything** - so a wrong answer
costs a paragraph rather than a commit.

**Only one of the three can cost anybody their answer, and that is what makes it a study.** The
predictions are written down before the run, which matters because `X-3` is that lens's own answer
and a study designed after seeing it would agree with it.

- Two of the tasks `X-3` predicts a cold instance **passes**. They can confirm and cannot refute
- The third - *what are a lens's jobs?*, where the lens's own README restates them and is four days
  behind `docs/process.md`, which owns them - `X-3` predicts a cold instance **fails**, and a
  long-running one might not. **If it reaches the owning document unprompted, `X-3` loses its
  central claim** that clearing context makes two of the phenomena worse

**What it would settle** is `S-57`'s third question: *does clearing context actually fix it, and what
does that cost?* **The outbox architecture assumes an instance is replaceable by a fresh one that
reads the files** - that assumption is why `pending.md` and the items are worth keeping, and it has
never been tested.

**I checked the four cases that are my lane's, from the record rather than from memory**, since that
is the failure under study. `P-320` and `S-56` are characterised correctly - `spec/console.md:45`
does say *nothing states its container*, and the code lane's `C-44` independently confirms `S-56`'s
work was in the tree before the item existed. `P-310` and `P-312` are correct and produced a rule,
promoted as `P-316`.

**And one case is gone, which is the better news in this item.** I filed this yesterday saying
`P-315` was mis-scored - that its own withdrawal blames a misread imperative rather than an unread
file, so a cold reader could reproduce it for a reason that has nothing to do with memory. **The
research lens checked that and found something neither of us had**: `b1d12c9`, the commit that
withdrew `P-315`, **also rewrote the sentence**. `CLAUDE.md` no longer contains the imperative with
no subject. So the case is not mis-scored, it is **unrunnable** - a cold instance reading today's
document gets the unambiguous version and passes for a reason that says nothing.

**The process had already repaired the thing the study would have measured, in the same commit that
recorded the mistake.** That is the mechanism in `CLAUDE.md` working, and it is worth more than the
data point it cost.

**What it costs you**: the first few minutes of one instance you are restarting for other reasons.
**What declining costs**: nothing today. `X-3`'s answer stands and says it is a derivation; what
stays unavailable is the ability to say **how much of a long-running instance's belief is memory**,
which no further classifying will produce.

## What changed while it sat here, and it is the answer to *have we learned anything*

**Two things, and the second is worth more than the study was.**

**The window closed.** This item's whole urgency was that you were restarting instances anyway and a
cold reader was free for a few minutes. They have been running for twenty-four minutes and are
working. **That version of the offer is gone until the next restart**, and nothing was lost by
missing it, because of what follows.

**The study cannot be blind while it is an open item, and I verified this rather than reasoned it.**
The design says: run it as the quality lane, give it **the ordinary lane prompt unchanged**, then
three tasks, and say **nothing about `S-57`, this report, or a study.** The ordinary quality prompt
in `docs/process.md` -> *Starting the instances* ends:

> Start by telling me what is open and addressed, read from the files rather than remembered.

**Doing that reads the index, and the index names this item.** `pending.md:11` carries it in full,
and so does `tools/outbox`. So a cold quality instance, following the unchanged prompt as its
**first action**, reads a line saying that an instance being restarted is about to be given three
tasks - before it is given them.

**That is not bad luck and no wording fixes it.** *Nothing open means nothing outstanding* works
because every open item is visible to every lane; `to` is what makes an instance's reading list a
query. **A measurement whose validity depends on the subject not knowing about it cannot be an open
item in a system built on every item being visible.** Filing it is what broke it, and filing it was
correct - `X-5` sat in a report's prose for a day precisely because it had not been filed.

**So the decision in front of you is a different one now**, and simpler:

- **Decline.** `X-3`'s answer stands as a derivation and says so. Nothing waits on it
- **Answer it, and let it be run on a later restart.** A closed item leaves the index, so an
  instance restarted after that reads no line about it. **This is the only order that works** - the
  study runs *after* the decision, never while it is the decision

**One thing today produced that bears on it, and it is not evidence.** Two instances cold from the
crash - the code lane and me - each committed the plausible-number failure within their first
twenty minutes. They read `grep -c 'P-322'` and got 3 where the absent heading was the answer; I
grepped whole commit messages and got 32 citing commits where the tool said 3. **Neither was blind,
neither was pre-registered, and both of us have read the reports these failures are named in** - so
this confirms nothing and the study would not accept it. **I record it because it is the shape of
what a study would ask, arriving unasked**, and because leaving it out would be choosing which
observations to mention.

## What the crash itself produced, asked 2026-09-07

**Yes, and it is the closest thing to the study that will exist without running one.** Three
instances went cold at once and worked for an hour. **Six failures of the study's own shape
appeared, unasked**, and I verified three of them myself rather than taking the report.

| #   | Lane     | What happened                                                                          | Whose shape |
| --- | -------- | -------------------------------------------------------------------------------------- | ----------- |
| 1   | code     | `grep -c 'P-322'` returned 3; the **absent heading** was the answer                    | `C-34`      |
| 2   | spec     | grepped whole commit messages, got 32 citing commits where the tool said 3             | `C-34`      |
| 3   | spec     | took `C-65`'s claim and rewrote `S-49` **without opening either file**                 | `C-34`      |
| 4   | code     | called the staging bullet a carefulness caution; it **explicitly disclaims being one** | `C-34`      |
| 5   | research | filed `X-7` asking the code lane to build a mechanism **that already existed**         | `S-56`      |
| 6   | code     | read a gate's exit code from `tail`, got a green `0` that meant nothing                | `C-34`      |

**Row 5 is the one that matters, because it is a prediction failing.** `X-3` predicts a cold
instance **passes** `S-56`'s shape, and names the refutation exactly: *cold files without looking,
which would move this case out of `never read` and into `nothing prompts a read`.* **That is what
happened**, and the research lens found it the same way `S-56` was found - by reading the hook its
own commit had just printed a message from. Its own words: `S-56`'s shape in a report about `S-56`'s
shape.

**Row 3 is mine and is the worst of the six**, because it wrote a wrong claim into the document a
fresh instance is supposed to trust.

**What this is not.** Not blind, not controlled, not pre-registered, and **self-reported** - each
lane is the only witness to its own failures, so the count is of failures *noticed*, and the true
number is larger by an unknown amount. **No row touches the third prediction**, the stale-README
one, which is the only row that can cost `X-3` its central claim.

**And the confound runs the wrong way for `X-3`, which is what makes this worth writing down.**
Every one of these lanes had read the taxonomy - `S-57`, `C-28`, `C-33` are in the files they open
first. **Knowing the failure by name did not prevent it six times in an hour.** A designed study
would have to argue that its subject was not primed; this one has the opposite problem and still
produced the failures.

**What it changes about the decision.** Two of the three predictions now have observational evidence
against them, so **the study's remaining value is concentrated almost entirely in the third task**.
If you authorise it, the honest version is one task, not three.

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

**to** sean - **status** open - **cited** `0361e25` - **raised** 2026-09-07 - **kind** the quality lens's `Q-68` - **asks**
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
