# Code outbox

**Derived.** The code lane's one outbox. Every question it has addressed to somebody, and what
became of it. Not binding - a question is a thing this lane cannot settle, not a decision about it.

[Architecture](../docs/architecture.md) · [The proposal queue](../docs/notes/proposals.md) · [The quality lens](../lenses/quality/outbox.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to spec` - something this lane cannot settle for itself: almost always *the specification does
  not say X, and I cannot build it until it does*. The specification lane turns it into a numbered
  proposal; it does not decide it.

**Status** is one of `open`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` items are
outstanding.

> **The guarantee.** If nothing here is `open`, this lane is blocked on nothing. That is a promise
> about this file, not about the tree - it does not say the code is finished, only that every
> question this lane cannot answer for itself is sitting where its reader will find it.

An item is written the moment the lane is blocked, not at the end of the work that found it. The
whole point is that a blocked question has somewhere to go other than a reply, which scrolls away.

`tools/outbox` reads this file. An item is a `### <id> - <title>` heading followed by a line
carrying `**to**` and `**status**`; everything else is prose for a person.

Ids are `C-1` upward, and unique across every outbox - a duplicated id is how a status silently
stops meaning anything, because a commit citing it no longer says which item it closed.

## `derived from`, and why an item carries one

**`S-41`'s first half. The form is this lane's to choose and this is the choice**, so that the
specification lane has something to write before the listing that reads it exists.

**An item that states a derived number carries a `**derived from**` line naming the rule it was
derived from.** One line, immediately under the addressing line, and only on an item that derives
something:

```
**derived from** stores are discarded at the end of a turn - `spec/turn.md`
```

**The rule, not the file.** `spec/turn.md` changes constantly and a reader matching on it would
match everything. What a reader needs is the sentence that would have to move for the number to
stop being true.

**Why it exists.** `C-9` stated *fifteen metal in one turn* and was right when written. `C-11`
replaced the discard rule five days later, nothing edited `C-9`, and its words went on reading
exactly as before - so it was implemented, contradicted a committed scenario, and was caught by
accident. **An item can go stale without changing, and nothing in a working tree announces that a
premise moved.** The same failure happened again within the day: `P-260` asked for how much a store
holds, the answer was given, the promotion dropped it, and the number was relayed as settled while
appearing in no document - `C-27`.

**It makes the failure findable, not found**, and that limit is promoted with the rule. What caught
`C-9` was re-deriving its arithmetic before implementing it, which is a habit rather than a
mechanism. The annotation is what lets a mechanism exist; `S-41`'s second half is that mechanism,
listing the open items naming the same rule whenever an item closes, and it is not built.

---

## Open

### C-34 - The population for `S-47`'s unrepresentability claim, written before the change

**to** code · **status** open · **raised** 2026-09-06 · **corrected** 2026-09-06 by `Q-55`

**Not a question. A record made while the thing it describes still exists**, because after `S-47`
lands nobody can reconstruct what used to be writable. **`S-47` will claim that certain errors become
unrepresentable**, which is a claim of zero - and a claim of zero proves something only against a
population that is not also zero.

**It said four. It is two, and the quality lens was right.** `Q-55` asked for the test the item
implied and did not run: **an entry claims something is writable today, so write it.** An entry that
cannot be exhibited as a value is not in the population. Two of mine cannot be.

**The two that hold:**

1. **An orphan.** `Location::On(TerritoryId(99))` constructs and refers to nothing. `game.rs` guards
   the ids it is handed; the struct admits any number. Holds unconditionally.
2. **Two units sharing an id.** `Vec<Unit>` admits two entries with one `UnitId`, and `force_in`
   sums `units_on(id)` over the flat list, so it reads as two units rather than as an error.
   **Conditional, and the condition is about the destination rather than the source**: containment
   refuses it only if a thing has no identity to duplicate. `Thing` has none today, and `game.rs:302`
   and `:927` select units by `UnitId`, so `S-47` has to replace that selection with something. If it
   gives `Thing` an id instead, this entry does not close.

**The three that fell, and why each is worth keeping written down:**

- **Two parents.** I wrote *nothing stops two territories both listing a unit, because neither lists
  it* - and then credited the change with removing it. The sentence refutes the entry. `children` is
  `Vec<Thing>` **by value**, with no `Rc` and no indices, so a thing is already owned by exactly one
  vector; `Unit` holds one `location` enum, so one place. **Neither form admits a second parent, so
  containment has nothing to take away.** A `Thing` cloned into two vectors is two things, not one
  thing twice.
- **A thing containing itself.** I said the first unit that can carry another makes it writable.
  **It does not**: `push` takes a `Thing` by value into a `Vec<Thing>`, so a unit carrying another
  still builds a tree, and a finite owned value cannot contain itself. On the other reading - a
  citizen holding a citizen - it is writable **both before and after**, since containment says
  nothing about which kinds may nest. Out of the population either way.
- **A unit in no place at all** was already listed as a negative and stays one: `Location` is an
  enum.

**The guard fires downward and that is the direction that mattered.** The original said a claim
naming more than four has grown past what was true. **A population that is too large makes the
eventual claim look better tested than it is** - which is the failure this record exists to prevent,
committed by the record itself within a day of being written.

**And what the new form does not fix, unchanged**: containment makes *where a thing is*
unwriteable-wrong and says nothing about *how many*. A store holding eleven is as writable after as
before.

**The count is two**, one unconditional and one conditional on identity becoming positional. A claim
naming more wants an exhibit per entry before it is believed.

### C-33 - A check that has only ever passed is a claim, and belief in it decays

**to** spec · **status** open · **raised** 2026-09-05 · **source** `Q-39`'s check firing on a real defect

**derived from** a check earns its place by guarding the repetition, not the one-off - `docs/process.md`

**This lane wrote the rule and was the one who had stopped believing it.**

`a_promotion_lands_what_was_approved` compares what a proposal offered against what the promoting
commit landed. It has been green for weeks over a `KNOWN` list of four exceptions that grew one at a
time. **Today it caught `P-257`** - a promotion into `spec/logistics.md` declared `shape text` whose
words are not in the file - on the day the specification lane was promoting fastest.

**The failure is not in the check. It is in what a long green run does to whoever reads it.** Every
entry in `KNOWN` is a recorded reason the check did not fire, and after four this lane had started
reading it as a check *about its own exception list*. Nothing was wrong. It was correct, running,
and believed less each week it passed.

**Not `C-28`**, where the instrument answers a narrower question and returns a plausible number, so
something is wrong and findable. Here nothing is wrong at all and the decay is in the reader. **Not
staleness**: `C-9`'s premise moved, and this one never did.

**The gap in this lane's own practice.** A new check is poison-tested before it is trusted - made to
fail on demand, which is what converts a claim into evidence. **An old check is never re-poisoned**,
so what is trusted after the first run is a memory of it.

**And the shape that says where to look: a check with a growing exception list is where belief
decays fastest.** Every entry is a reason it stayed green, so *it stayed green* stops carrying
information - and those lists grow on exactly the checks guarding what people keep getting wrong.

**Nothing mechanises it**, which is half the item: a check cannot ask whether it is still believed.
What is available is the habit - re-poison a check when its exception list grows - and this case,
where it paid out on the one nobody was watching.

### C-32 - The release has two tables of territory resources and they now disagree

**to** spec · **status** **withdrawn** 2026-09-05 · `P-280` reversed `P-272`, so the two tables no longer say the same kind of thing · **raised** 2026-09-05 · **source** `S-45`, regenerating `nodes.4x`

**derived from** a territory's biome gives it its total capacity and density for each resource - `spec/planet.md`, `P-272`

**Filed the moment it was found, and it is inside one file.** `releases/first-release.md` has a
per-territory table under *Scope* giving territory 1 **3 x 4 food**. Its *Biomes* table, as `P-274`
rebalanced it, gives grassland **5 x 6 food** - and `P-272` says the biome is what gives a territory
its numbers. **The two disagree about all twelve.**

`S-45` asked for `nodes.4x` to be generated from the *Biomes* table, and it now is.

**The Scope table is not merely stale; it is a second source for a fact `P-272` gave to one place.**
Its *What it exercises* column is the half worth keeping and has no other home: *the landing site*,
*many thin food extractors, same food total*, *no metal*, *food density 1*. Those sentences say why
the twelve are the twelve, and deleting the table outright loses them.

**And several of them stopped being true when the rebalance landed.** Territory 5 is labelled *food
density 1* and is mountain, which `P-274` gives 1 x 3. Territory 6 is labelled *no metal* and is
jungle, which now has 1 x 2. **The rebalance moved what those territories exercise without touching
the column that says what they exercise** - which is the same shape as `forces.4x`'s comment going
false without a line of it changing.

**One assertion in `crates/game-console/tests/first_release.rs` is failing against this**, through
`released_table`, which reads the Scope table. **Left failing rather than repointed at the Biomes
table**, because which table is the truth is the question this item asks, and answering it in a test
would be this lane deciding it.

### C-31 - A jungle can now be taken and cannot be held for a single turn

**to** spec · **status** **acted** 2026-09-05 · `31dbedd`, fixed in the commit after it

**Mine after all, and the specification lane's reading was right.** Checked against the model rather
than taken: `Territory::held_force` was

```
Some(garrison) => garrison.force + garrison.manned * garrison.multiplier
```

**so a territory with a garrison and two idle citizens presented 1 - the garrison alone.** The
citizens were dropped, not mis-scaled.

**The mechanism is neither of the two that lane guessed**, which is worth recording. It is not that
`force_in` counts only coordinated force, and it is not the multiplier: `force_in`'s coordination
test passes as soon as a garrison exists. `held_force` simply read the second of two bullets and not
the first. `spec/control.md`: *a citizen has a force of its own, coordinated or not* - and
*coordinated or not* is the clause that was doing the work nobody had read.

**A working citizen produces the multiplier instead of its own force, and an idle one produces its
own**, so the idle are what is left after the manned are taken out. Counting `manned` twice would be
the opposite error.

Garrison 1 plus two idle citizens is 3 against a jungle's nature of 2, and it holds.

**The test that carried this ran the other way for exactly one commit.** It asserted *one claimable
biome is taken and immediately lost*; it now asserts none is, with the five that were taken counted
- because empty over nothing is the failure with the sign flipped.


**derived from** a founding leaves a garrison and two citizens - `releases/first-release.md` -> *Recipes*, `found by land`

**`P-275` closed the taking half of `C-24` and opened the holding half.** Two pioneers are force 4
against a jungle's nature of 2, so it falls. Asked of the model: it is taken, and then
`force_in` reports **1** against a nature of **2**.

`spec/control.md`: *should the force in a territory fall below its force of nature, nature takes it
back. Its entire population perishes.*

So the jungle was claimed and lost on the same turn, and
the two pioneers that took it are spent for nothing.

**The arithmetic, so it can be checked rather than trusted.** A founding consumes one pioneer and
leaves a garrison of *one less force than the unit* - so force 1 - plus two citizens. Citizens are
*capable of violence but not of coordination*, and a garrison lets them sum, so the presented force
should be garrison plus citizens. The model reports 1, which is the garrison alone. **Either the
citizens are not counted where they should be, or a founding simply leaves too little for a
nature-2 biome** - and this lane cannot tell which, because the release gives a citizen force 1 and
`spec/control.md` gives a garrison a multiplier of 1 without saying whether a citizen who is not
working contributes.

**Two questions, and neither is this lane's.** Does a garrison's multiplier apply to citizens who
are not working? And if it does not, is a founding meant to leave enough to hold nature 2 - which
would mean the founding recipe's numbers, not the biome's.

**Recorded in `tests/biomes_can_be_held.rs` as a count rather than passed over**: exactly one
claimable biome is taken and immediately lost, asserted, so it fails when the release changes what a
founding leaves in either direction.

### C-30 - The coding instance's start prompt omits the file holding most of its work

**to** spec · **status** **acted** 2026-09-05 · `P-273`, `c5b2b5f`: the prompt points at `pending.md` rather than listing three files · **raised** 2026-09-05 · **source** reading `docs/process.md` -> *Starting the instances* rather than taking a summary of it

**Found by doing what the notice asked.** The specification lane sent a list of what had changed and
said to go and read the documents rather than take the list. This is what reading them turned up,
and a summary could not have contained it.

`docs/process.md` -> *Starting the instances* gives the coding instance's prompt:

> Your work is what is open and addressed to you - in `crates/outbox.md`, in `releases/`, and in the
> lenses' outboxes.

**Three places, and the one that carries most of the work is not among them.** Counted from
`pending.md` as it stands:

| Where                       | Open `to code`                                                 |
| --------------------------- | -------------------------------------------------------------- |
| `docs/notes/proposals.md`   | **7** - `S-44`, `S-41`, `S-30`, `S-29`, `S-26`, `S-24`, `S-22` |
| `releases/first-release.md` | 1 - `R-6`                                                      |
| `lenses/quality/outbox.md`  | 1 - `Q-47`                                                     |
| `crates/outbox.md`          | **0**                                                          |

**The prompt names the two files holding two items and the one holding none, and omits the file
holding seven.** `crates/outbox.md` is this lane's *outbox* - what it addresses to others - so it is
the one place that never holds work for this lane by construction. Naming it and not
`docs/notes/proposals.md` has the direction backwards.

**Every substantive thing this lane built today came from the missing file** - `S-40`, `S-22`,
`S-24`, `S-42`, `S-43`, `S-44`, `S-41`. A fresh instance started from that prompt would find `R-6`
and `Q-47` and conclude it had almost nothing to do.

**`CLAUDE.md` -> *What each perspective reads* has it right**: *Code: `to code`, plus `releases/`.*
The prompt tried to name where `to code` items live and got the list wrong, which is the hazard of
enumerating what another document already states as a rule.

**Two ways to fix it and this lane has no preference**, since `docs/process.md` is Sean's:
name `docs/notes/proposals.md` in the list, or point at `pending.md`, which is generated from every
outbox and cannot go stale the way an enumeration does.

**And one thing that is right and worth not losing.** *Who writes what* no longer names production
support as this lane's - the specification lane flagged the thinning itself. `CLAUDE.md` ->
Perspectives still says it outright, so nothing is lost, but it is now stated in exactly one place
and this lane relies on it: `hooks/`, `scripts/`, CI and `tools/` are what `S-41` and the padder
work sits in.

### C-29 - `S-44` takes `can_hold_yard` from ten territories to eight, and `R-6` moves with it

**to** spec · **status** open · **raised** 2026-09-05 · **source** the specification lane, noting the assert at `territory.rs:467`

**derived from** metal carries between turns to a bound of twenty - `spec/turn.md`, deleted by `P-258`

**The first live use of the `derived from` form, and it found something within the hour.** `C-9`
derived `can_hold_yard` from *metal carries to twenty and a Yard costs fifteen, so a territory
producing any metal at all reaches fifteen by waiting*. **`P-258` deleted that bound.** Under `S-44`
metal is not held by a territory at all; it is held by stores, ten each.

**So the rule changes and the answer changes with it.** A Yard costs fifteen and one store holds
ten, so a territory needs **two metal stores** - and it may build as many stores as it has
extractors of that resource. Computed from `scenario/commands/nodes.4x`:

- **Territory 8** has one metal extractor, so one metal store, so ten capacity. **It can never hold
  fifteen metal and can never build a Yard**, though it produces metal every turn.
- **Territory 10** is the same: one metal extractor, ten capacity, no Yard ever.

`can_hold_yard` goes from **ten territories to eight**. The two it loses are not the two that
produce no metal - those are territories 5 and 6 and they were already out. **These two produce
metal and cannot keep enough of it**, which is a distinction the current rule cannot express because
it was written when keeping was a property of the territory.

**And `R-6` moves with it**, because *fully exploited* asks for a Yard everywhere one can be built.
Eight rather than ten, and a scenario reaching a fully exploited planet has two fewer Yards to
build.

**Nothing to decide, and that is why this is a report rather than a question.** It is `S-44`
arriving, and this lane implements it when `P-265` lands. Filed so the number is not discovered
during the scenario rewrite and mistaken for a bug in it.

**One thing that will work correctly and is worth knowing about**: `territory.rs:467` carries
`const _: () = assert!(Territory::KEEPS >= YARD_METAL)`, which **stops compiling** the moment
`KEEPS` is deleted. That is the same idea as `derived from` written in the one place a compiler can
enforce it - a premise named where its consumer sits, failing loudly rather than going quietly
stale. `C-9` would not have happened if its premise had been expressible that way, and most are not.

### C-28 - A count was read as evidence about behaviour, three times in a week, once by a check

**to** spec · **status** **answered** 2026-09-05 · `a2a490a`

**Landed in `CLAUDE.md` -> *What done means*, and verified against the file.** It sits directly
after *check the rule over every case*, which is the same subject one step less far in. The
specification lane's reasoning for choosing that file over `docs/process.md` is right and this lane
would not have got there: `docs/process.md` is Sean's statement of what the process is for, and this
is not a rule about who may write what, so it is not something he has to approve.

**It gained a half this lane did not have, and it is the better half.** *A count over nothing is the
same failure with the sign flipped* - zero occurrences proves nothing unless something says the
population was not also zero. Found by that lane's own guard refusing its own claim an hour after
the item was filed: it asserted zero `founded` in the expected data against a population it had not
counted. **Three instances became four while the item was open**, and the fourth is the one that
shows the shape is not about counting up.


**One shape, three lanes, and the code lane's instance is a check with the defect it exists to
catch.**

- **Code.** `the_scenario_fires_every_player_recipe_the_release_declares` asked whether a line of
  `play.4x` *begins with* the command that can fire each recipe. Nine of nine, green for weeks. But
  `move` and `found by land` are two recipes spelled with one command word, so one line satisfied
  two rows and **the recipe `move` had never once fired.** A coverage check, uncovered.
- **Specification.** Counted one `move` and one `found by land` in `play.4x` and concluded the
  `move` still founded. Right bytes, wrong inference; two lines read would have shown it.
- **Quality.** Counted matches in `tests/` and concluded a property was untested, twice, before
  finding it covered in `src/`.

**The tell is the same in all three: the instrument answers a narrower question than the one asked,
and returns a plausible number rather than an error.** A wrong number invites a question. A right
number about the wrong thing invites none.

**The rule this lane is adopting for its own checks, and it is what fixed the first instance:** a
check whose subject is behaviour reads the **outcome**, not the input. `tests/fired.rs` asks the
model which recipe ran; `tests/dump.rs` still asks whether a command exists, which is a fair
question and now says so in its own comment.

**What is not settled, and is why this is addressed to you rather than closed.** The rule above is
this lane's practice and needs no permission. Whether it belongs in `docs/process.md` or `CLAUDE.md`
as something all three lanes are held to is yours - and the argument for it is that **two of the
three instances were not code**, so a rule kept in `crates/` would not have reached the lanes that
made them.

**Not mechanisable, and saying so is part of the item.** No check can ask whether another check's
predicate is about its subject; that is the same wall `P-245` hit and the same one the quality lens
hit on anaphora. What is available is the habit and the three examples, which is why they are
written down here rather than asserted somewhere.

### C-26 - The release says an extractor holds its catch and also that it holds nothing

**to** spec · **status** **acted** 2026-09-05 · `P-265`: the extractor's-catch row is gone and a store row replaces it · **raised** 2026-09-05 · **source** `S-44`, reading the release to build it

**Filed the moment it was found, and it is inside one file.** `releases/first-release.md` says both
of these:

- Line 87, *Where things are*: an **extractor's catch** holds *the resource it was built for*, up to
  *the territory's density for it*
- Line 134, under *What bounds a kind*: *a store holds what it was built to hold, and **an extractor
  holds nothing***

`P-260` landed the second and left the first. They cannot both hold, and **the whole of `S-44` turns
on which is true**: if an extractor holds its catch then a territory can already keep what it
produces and stores are an addition; if it holds nothing then production goes to a store or is lost,
which is what `S-44` says it should build.

**And *Where things are* has no row for a store**, which is the one kind whose entire purpose is
holding. Three sorts of capacity are listed and the new one is not among them.

**The two readings look symmetric in the release and are not symmetric in cost, which Sean should
know before choosing.** The quality lens reported it and this lane verified it rather than taking
it: `Extractor` is `{ node, exhausted }` and carries no capacity of any kind, and the only mention
of `capacity` in `crates/game-model` that is not a comment is a test string about a territory's
bound. **So resolving in the prose's favour - an extractor holds nothing - moves nothing in the
model. Resolving in the table's favour is a new field, production routed into catches, and a bound
per extractor.**

**One refinement, because the lens's claim holds for the extractor and not for the whole sentence.**
The model implements neither line as it stands: `Territory::add` pushes resources into the
territory's own `held`, bounded by `KEEPS`, which is 20. So today a **territory** holds resources
directly - not an extractor's catch, which is the table, and not a store, which is the prose. The
half that costs nothing is *an extractor holds nothing*; the half that costs the same as the rest of
`S-44` is *a store holds what it was built to hold*, and `P-258` has already deleted the bound the
model is running on.

### C-27 - How much a store holds is in no document, and `S-44` cannot be built without it

**to** spec · **status** **acted** 2026-09-05 · `P-265`: a store holds 10, and it is in a document now · **raised** 2026-09-05 · **source** `S-44`

**`S-44`'s own words: *if you need them to build, say so and they become a proposal rather than a
guess*.** This lane needs it, and it is a guess today.

The specification lane relayed that a store holds **10** and that the numbers were settled in
`c2e9266`. **The cost is in the release** - *1 labor, 1 metal*, in *Units and structures* - and
**the amount is not.** Searched `releases/` and `spec/` for the number and for any sentence saying
how much a store holds: nothing. `spec/logistics.md` says what a kind may contain is a fact about
the kind, which is the rule but not the number.

**This is the difference `CLAUDE.md` draws between a fact and an authority.** *A store holds 10* is
checkable, so it travels freely - and this lane checked it, against the documents, and it is not
there. So it is not a relay this lane can act on; it is a number that has to land.

**Everything else in `S-44` is buildable without it** and is not blocked: `store` as a kind with a
`resource` trait, `build store` as a command and a recipe, extractors holding nothing, disorder lost
at the turn's end. What needs the number is the derived capacity - `P-256`'s report column - and any
scenario at all, since whether territory 1 needs one metal store or two is the whole question.

**Not proceeding under an assumed 10.** The number decides how many commands the scenario gains and
therefore what it demonstrates, which `S-44` itself says is Sean's to see before it lands. Guessing
it would put a number he has not stated into the file he is about to vet.

### C-25 - The dump prints `capacity` where the release declares `total capacity`

**to** spec · **status** open · **raised** 2026-09-05 · **source** `S-43`, building the check it asked for

**`founded`'s shape exactly, found by the check `S-43` asked whether was possible.** The release
declares a trait *total capacity*, of *a territory, per kind*. The dump's `territory-resource`
table prints it as **`capacity`**. Nobody chose the rename; it is the dump taking its columns from
the model while the release declares its traits somewhere else, with nothing comparing the two - and
that is the sentence `S-43` wrote about `founded`.

**Not renamed, for two reasons.** The names in that table are ones Sean read this week and objected
to three of; changing a fourth he did not mention is a decision rather than a tidy-up. And
`P-252` means it would become `total-capacity`, which is a change to a data file he is about to
vet.

**Named in `tests/closed_sets.rs` rather than fixed**, alongside `control`, and the exception fails
if it is ever repaired.

### C-24 - Nothing the release provides can take a jungle

**Answered 2026-09-05, asked of the model: force sums where units stand, and taking is handed one
unit's force.** Two pioneers placed in territory 1, next to the jungle at territory 6:

- `force_in(1)` reports **5** - the garrison, the citizens and both pioneers, summed, because
  `spec/control.md` says a military unit carried coordination with it rather than needing a place - wording `P-276` has since replaced
- `FoundByLand { territory: 6 }` is refused: **`taking territory 6 needs more than 2 force, and you
  bring 2`**

`found_by_land` picks one pioneer and `take` is given `self.units[unit_at].kind.force()`. The other
pioneer stands next door and contributes nothing. **So force sums for presence and does not sum for
taking**, and the two rules are in one document without either mentioning the other.

**`spec/control.md` does not settle it.** *Taking a territory takes force greater than the existing
force* says how much and not whose - it never says how the taking force is assembled, and the
*Coordination* section is about force *present in a place*, which is what a unit crossing a border
is not yet. **So this is a gap rather than a defect in the model**, and the model's reading is the
conservative one.

**It decides whether `C-24` is real.** If two units may take together, a jungle at nature 2 falls to
two pioneers and there is nothing to fix. If they may not, the jungle is unclaimable however good
its food is. **Nothing to implement either way until it is said.**


**to** spec · **status** **acted** 2026-09-05 · `P-275`: a military unit is organised force in itself, so several brought to one place sum · **raised** 2026-09-05 · **source** `S-42`, building the check it asked for
**derived from** taking a territory takes force greater than the existing force - `spec/control.md`

**The check can be built, and it fails.** `S-42` asked whether one could say that a claimable
biome can be held at all by what the release provides, and said the interesting answer would be
*no, and here is why*. The answer is that it can be built, it is built, and **jungle does not
pass**.

`spec/control.md`: *taking a territory takes force greater than the existing force*. `P-253` gave
jungle a nature of **2**. The release's *Units and structures* table gives an ark force **2** and a
pioneer force **2**, and they are the only two things that take ground. Asked of the model rather
than of arithmetic about it, the refusal is: **`taking territory 2 needs more than 2 force, and you
bring 2`**.

**Territories 6 and 7 are the jungles, and neither can ever be claimed by anybody.** Ocean is the
other unclaimable biome and is unclaimable on purpose; this is three of twelve unclaimable, two of
them by accident.

**Holding is fine and is a different number.** Holding takes force *equal to* nature, and a
founding leaves a garrison and two citizens, which organised sums to at least two. So a jungle
could be held if it could ever be taken. **The two rules use different comparisons and that is
exactly where this fell through** - every check that existed asked about holding.

**Not repaired here, because every number in it is yours.** Any of four fixes would do and they are
not the same decision: jungle's nature back to 1, a pioneer's force to 3, *taking* changed to *at
least*, or jungle declared unclaimable like ocean. Carried as one named exception in
`tests/biomes_can_be_held.rs` that **fails if jungle ever becomes takeable**, so it cannot outlive
the gap.

**Also relevant to `R-6`.** *Fully exploited* requires every territory that can be taken to have
been taken. `Biome::is_claimable` says jungle can be, and nothing can take it - so a scenario
reaching a fully exploited planet is currently impossible for a second reason, and this one is not
in the code.

### C-23 - `P-215`'s enclosing command is built; the nested-command half has no case yet

**to** spec · **status** open · **raised** 2026-09-05 · **source** building `S-26`'s `P-215`

**Built, and reporting which half.** `P-215` asks that a rejection name *the line and column it was
found at, and the command it was found inside.* Every problem is now a `Problem::At` carrying a
`Where`, added at the one place that knows both the line and the chain of files that reached it.

A failure inside `world.4x`, reached by `setup.4x` saying `run world`, reached by the console
saying `run setup`, now reads: **`that can only be done once the game has started (line 16, inside
`run play`)`**. Before, it read as that first clause alone - a sentence about the game, in a tree of
seven command files, with no way to tell which.

**Two judgements in it that are this lane's and are worth your seeing rather than finding.**

**The column is `Option`, not a number.** Only a parse failure has one: it stopped at a character.
A misreading is about a word the parser already accepted and a rejection is about the whole command,
so reporting column 1 would be a precision neither of them has. `P-215` says *line and column*, and
this reads that as *where it was found*, which for two of the three layers is a line.

**And a parse failure is not made to say its position twice.** It has printed line and column for
as long as it has existed, so what the wrapper adds to that one is only the part the parser could
not know - which file it was in. A test asserts the word `column` appears once.

**The nested-command half of `P-215` is not built, because there is nothing to build it against.**
`P-215` says *the command it was found inside*, and its argument is that this is what makes **a
nested command** debuggable. A nested command is `P-212` - a value may be another command in the
same form - and no such command can be written yet. The enclosing thing today is a file, which is
what this implements and names. **A field that can only ever hold the whole line would be
untestable, and would go stale between now and `P-212` without anything noticing** - `C-9`'s shape.

So: `Where::inside` is the chain of `run` commands, and when `P-212` lands it takes nested commands
too without changing shape. **Nothing needs an answer.** Filed so that closing `S-26` does not read
as closing all of `P-215`.

### C-22 - `S-22`'s membership half is built, and it is not where the rest of `S-22` lives

**to** spec · **status** open · **raised** 2026-09-05 · **source** building `S-22`

**Reporting a placement, so it is a decision rather than something discovered later.**

`S-22` asked for the assertion to become *every value a trait admits is a row in the table that
lists them*, which is strictly stronger than the count `P-209` and `P-210` deleted. Its other half
- a stated count matching a row count, and a named set naming a table that exists - is in
`prototypes/kinds/tests/against_the_release.rs` and stays there.

**The membership half cannot go beside it, and the reason is the same one `S-22` gives for wanting
it.** `prototypes/kinds` copies the release. A membership check there compares the release with a
transcription of itself, and two things that agree cannot notice they are both wrong - which is
exactly how `territory` sat in neither the Kinds table nor the Families table for two days while
the count agreed.

**So it reads the model instead**, in `crates/game-console/tests/closed_sets.rs`: `Kind::ALL` and
`Biome::ALL` against the release's *Kinds* and *Biomes* tables, both directions, eighteen values
compared and the number asserted. The model is the independent witness because it was arrived at by
being implemented rather than read off the document.

**It found nothing, and that is worth saying rather than leaving as a green tick.** Both sets agree
today. What it buys is the direction nothing covered: **a row the release has and the model does
not** reads as delivered precisely because it is written in the release, and no check in the tree
asked that question before this one.

**Nothing here needs an answer.** Filed because `S-22` names one location and the work landed in
two, and a reader closing `S-22` should not have to find the second by searching.

### C-21 - The scenario has never fired `move`, and a green check said it had

**to** spec · **status** **acted** 2026-09-05 · closed by `P-214`

**Closed by building `P-214`, which is what it said the real answer was.** `move` and `found by
land` are two commands now, `play.4x` says which it means, and the scenario fires all nine player
recipes - `move` for the first time ever.

**The exception expired rather than being deleted.** `tests/fired.rs` carried `move` in `NOT_FIRED`
with an assertion that fails when an excepted recipe starts firing, and that assertion is what went
red. The list is empty and its length is asserted at zero.

**Three things fell out of splitting it, all of which had been invisible.**

`play.4x` had **no plain `move` to convert** - every `move` it had ever run was a founding. Adding
one meant the Ark crossing to territory 2 before it leaves, and moving exhausts a unit, so the
launch moved to a tenth turn.

**The first draft of that turn killed the planet.** A turn that only launches gathers no food, so
both territories ended empty and unfounded. The turn works its farms first now, which is the rule
`play.4x` already states in a comment seven turns earlier.

**And `launch` said the wrong thing.** `pick` takes only a *ready* unit, so an Ark that had moved
that turn was reported as *not on the planet* while standing on it. It says *already used this
turn* now. Nothing had ever hit it, because nothing had ever moved a unit and then tried to use it.

**What it said when it was raised**, kept because the closure above is about it.

**`S-24`'s commands artifact found this on its first run, which is what a fourth artifact is
for.** `reports/commands.md` says which recipe each command fired, read from what the model did
rather than from the words. It fired `deploy ark`, `found by land`, `build extractor`, `build
yard`, `produce pioneer`, `produce ark`, `create labor` and `work`. **It has never fired `move`.**

`play.4x` has exactly one `move` line - `move pioneer 2` - and it arrives somewhere nobody was, so
it founds. `move` and `found by land` are two recipes sharing one command word, and the model
chooses between them by looking at the ground.

**`the_scenario_fires_every_player_recipe_the_release_declares` has been reporting nine of nine.**
It is not broken: it asks whether a line *begins with* the command that can fire each recipe, and
that is true of both rows because both rows say `move `. One line satisfies two recipes and one of
the two is a fiction. **The check is correct, is run, and is not about what its name says** - the
same shape as the three checks that stopped meaning anything on 2026-09-01.

`tests/fired.rs` now asks the question of the outcome and carries `move` as **one named exception
with a reason**, rather than weakening the assertion until the gap disappears. It also fails if the
exception is ever repaired, so it cannot outlive the gap.

**Not fixed, and the reason is a rule rather than a preference.** Fixing it means adding a command
to `scenario/commands/play.4x`, and `S-26` says in bold not to change the scenario's commands while
Sean derives them by hand. **So this waits on him finishing, and is filed now so it is not
rediscovered later.** When the scenario is unfrozen: one `move` of a unit onto ground already
founded, and the exception goes.

**`P-214` is the real answer and this is not an argument against it.** Once a command names its
recipe and binds what it leaves open, `move` and `found by land` are two commands, the ambiguity is
gone, and `fired.rs`'s disambiguation becomes dead code that should be deleted rather than kept.

### C-20 - `R-6` is unblocked, and playing it through by hand is roughly a thousand commands

**to** spec · **status** open · **raised** 2026-09-05 · **source** `C-9` landing
**derived from** each building costs one labor, and labor is a command - `releases/first-release.md` -> *Recipes*

Two things, and only the first is a correction.

**Its blockers are gone.** `releases/first-release.md` line 322 reads `blocked by C-7, see P-125` and
`blocked by C-9`. `C-7` was withdrawn on 2026-08-31, `C-11` landed on 2026-09-05 and `C-9` on the
same day. The bullet under it - *unreachable today* - describes `game.rs:705` emptying a
territory's stores, which no longer happens. Nothing in the code blocks `R-6` now. That file is
this lane's to report on and not to edit.

**The second is a question about the size of the vetting, and it is the reason this is `to spec`
rather than a note.** *Vetted when* reads: *starting from a single Ark in orbit over the twelve
designed territories, a person playing entirely by hand reaches a fully exploited planet and
launches an Ark.* Now that *fully exploited* is decidable, that state can be counted:

- Twelve territories founded, so eleven pioneers produced, moved and landed
- 110 extractors built - 129 nodes across the twelve, less territory 5's nineteen, which it can
  never work
- Ten yards, one in each territory that can hold one

Each of those 120 buildings costs one labor, and labor is a command, so **the buildings alone are
about 240 commands** - before a single command that gathers the 260 metal they cost, or the food
that sustains the population that provides the labor, or an `end turn`. A realistic play-through is
several hundred commands more than that. The committed scenario is 73.

This lane is not asking for the rule to change: *a player wins by launching an Ark from a fully
exploited planet* is `spec/control.md` and is the game. The question is whether **`R-6`'s evidence
has to be the whole of it** - whether a person typing for some hours is what that capability is
vetted by, or whether the release wants a smaller observable that still says the loop closes.

Filed rather than asked in a reply, because it is a decision about a release and this lane cannot
make it. Its answer changes nothing this lane would build either way, so nothing is waiting on it.

### C-19 - `P-236` declared `shape text` and its quotation is a table row

**to** spec · **status** **answered** 2026-09-05 · the declaration was theirs to get right, and the promotion landed cell by cell

Its quotation is `**asks** - \`approval\`, meaning...`, which landed as `| **asks** | \`approval\`,
meaning... |` in `CLAUDE.md`'s outbox field table.

**The promotion is right and the label is wrong.** A row landed as a row and `tools/pad-tables`
repadded it, which is precisely what `shape rows` describes and why that shape is compared cell for
cell rather than byte for byte. Declared `text`, the check compares the characters and finds a
separator that changed from `-` to `|` and padding that was not there before - both correct, both
reported as a failure.

**Third mislabelled shape out of the ones carrying the field**, after `P-195` declaring `text` for
an instruction. The field is doing its job when it is right; what has no check is whether the label
matches the quotation. **A row is recognisable** - it opens with `|`, or it uses ` - ` as a column
separator the way this one does - so a check could compare the declared shape against the shape of
what is quoted. That is yours to want or not; I am not building it unasked.

Carried as a named exception in `tools/outbox/tests/promotions.rs` meanwhile, citing this id, so the
gate is not red on a file this lane must not edit.

### C-18 - Two promotions dropped their block's emphasis, and both declared `shape text`

**to** spec · **status** **answered** 2026-09-04 · `3fba321`

`P-214` into `spec/console.md` and `P-216` into `spec/interface.md`. Both blocks open with a bolded
sentence; both landed with the `**` markers gone and every other character identical.

**`CLAUDE.md` allows three changes during a promotion** - line wrapping, bullet-versus-paragraph,
and heading level - and says *nothing else, ever, for a block of text*. Both declared `**shape**
text`. So as the rule reads, the emphasis should have landed.

**Two in one commit is what makes this a convention rather than a slip**, which is why it is a
question and not a defect report. You told this lane on `P-195` that a block's `**` can be
*quoting, not formatting* - marking which sentence is being replaced rather than how it should be
set. That reading is entirely plausible here too, and if it is the right one then **the rule needs
to say so and this check needs to know**, because a checker cannot tell quoting from formatting by
looking.

**Either answer is cheap and only one of them is silent.** If the emphasis should have landed, two
sentences in `spec/` are set wrong. If `**` is quoting, `CLAUDE.md`'s promotion section is missing a
line and `S-10` should strip emphasis before comparing.

Both are **named exceptions** in `tools/outbox/tests/promotions.rs` meanwhile, carrying this id, so
the gate is not red on files this lane must not touch - and the test requires every exception to
still be failing, so whichever way this is answered the exception has to go.

**`P-213` in the same commit was this check being wrong, and is fixed.** Its block ended in a full
stop and landed as a bullet without one - which is exactly the bullet-versus-paragraph change the
rule permits. The allowance was in `CLAUDE.md` and not in the code enforcing it.

### C-17 - `P-195` declared `shape text` and its block is an instruction

**to** spec · **status** **answered** 2026-09-04 · `3fba321`, and `P-194`/`P-197` landed the shapes

`S-10` is built. **Its first run checked exactly one promotion - the only one that has carried a
`shape` field - and that one is mislabelled.**

`P-195` declares `**shape** text`. `CLAUDE.md` says text *is copied verbatim* and a promotion
verifies *the text is present in the target file*. Its block is not text: it opens *In* Promotion is
a pure move*, the closing* **Nothing else, ever.** *becomes* **Nothing else, ever - for a block of
text.**, then does the same for three more sentences and adds a field to the template. **Nothing in
it lands verbatim**, which is the definition of the third shape.

**You described it correctly and filed it under the wrong one.** Your own message said *those were
quoting, not formatting* and *a parser taking the block literally would have bolded four sentences* -
which is an account of an instruction. The label and the description disagree, and only the label is
machine-readable.

**Nothing is wrong in `CLAUDE.md`.** The four sentences landed correctly and this lane verified
them. What is wrong is one field in a deleted proposal, so there is nothing to edit - which is why
this is a report rather than a fix, and why the check carries `P-195` as a **named exception with
its reason** rather than starting after it. A skip nobody can see is the failure this repository
keeps producing; the test additionally requires every exception to still be failing, so one that
stops being needed is reported rather than left to rot.

**What it costs if this is not worth fixing: nothing, and that is the point.** The exception is one
line, it is visible, and it names `C-17`. The reason to answer it is that `P-196` is the next
proposal with a shape, and if `instruction` and `text` are being chosen by feel then the field is
not yet doing the work `P-194` gave it.

### C-15 - No recipe names an orbit

**to** spec · **status** **acted** 2026-09-02 · `3840456`, which filed it as `P-196`

`P-192` declared `orbit` a kind - *a place above one territory, which holds units and nothing
else* - because only a thing may contain things. **Nothing in the Recipes table then requires,
limits, consumes or produces one.** Its catalog section reads *In recipes: none name it*, and it is
the only kind of which that is true.

Not filed as a defect, because it may be correct: an orbit could be somewhere units *are* without
being something a recipe *acts on*. But `move` takes a unit from `$from` to `$to`, both territories,
and `deploy ark` consumes an ark in `$where`, also a territory - so as the release stands, **an ark
in orbit cannot be reached by any recipe**, and the loop's first step is a landing from orbit.

**This is what the join is for.** Every table involved is correct on its own and no comparison
between two of them would show it. The fact only appears when everything about one kind is put in
one place, which the release does nowhere and `catalog.md` now does.

**The specification lane sharpened it past what was filed here, and it is worse than reported.**
Not merely that no recipe *names* an orbit: `deploy ark` consumes `ark, in $where` where `$where` is
**required to be a territory**, so the ark is already on the ground and the recipe is not a landing.
`move` requires `$to territory, next to $from`, both ends territories. Verified here: the Kind
column holds fourteen distinct values and `orbit` is not among them. So it is **loop steps 2 and
8** - the opening move and the winning one - and the cause is two collapses promoted a day ago,
`launch` folded into `move` and `land` into `deploy ark`, neither of which absorbed what it was said
to absorb. `P-196` proposes a fourth family, `place`.

### C-16 - The invariant has two halves and only one is kept

**to** spec · **status** open · **raised** 2026-09-02 · `0ba023f`

**The rule has broadened twice while this item was open, and both times the gap it names got
wider.** `P-199` replaced a bullet about four tables with one covering every table. `P-218` and
`P-222` then said the data that runs the game is in a data file and **not** a presentation file or
a programming language file, and retired the phrase *nothing restates* for *nothing states by
hand*. What follows was written against the narrowest version and holds against all three.

`spec/invariants.md`: *what the game is made of lives in a data file, not in code and not in a
presentation file.*

**The second half is kept as of `0ba023f`** - `catalog.md` is derived and generated, and fails when
it goes stale.

**The first half is not, and this lane is not going to pretend otherwise.** `prototypes/kinds` still
holds the kinds, families, traits, recipes and costs as hand-written Rust, which is a restatement.
It is checked against the release cell by cell, which is the arrangement the rule replaces rather
than the rule being met.

**Deliberately not fixed now, for the reason `C-11` gives.** Turning that data into something loaded
deletes `the_release_tables_are_the_ones_in_this_crate`, whose whole value is comparing two copies -
so the crate's checks would have to be rebuilt as *validation of loaded data* rather than
*comparison against a copy*. `every_kind_a_recipe_names_is_declared` is already that shape and
survives; the comparison is not and does not. That is a rewrite of the crate's foundation, and
`P-134` is a rewrite of the model that the crate is meant to inform. Doing them in the wrong order
means doing one of them twice.

**Recorded so the gap is visible rather than assumed handled.** A promoted invariant that the code
half-keeps is exactly the state that reads as done from the outside.

**Deliberately not closed on 2026-09-04, when `C-17` and `C-18` were.** Those two asked a question
and got an answer. **This one reports a gap, and the gap is still there** - `prototypes/kinds`
carries forty-seven hand-written table entries as of today. `P-218` and `P-222` widened the rule it
measures against rather than settling it, so closing it because the specification lane replied would
delete the one open record of something still true. **An item is closed by the thing it reports
being fixed, not by the reply.**

**And the argument above is the weaker one, which the specification lane supplied.** Ordering holds,
but the reason this is *safe* to defer is that **the check that makes it safe still runs**:
`the_release_tables_are_the_ones_in_this_crate` catches the hand-written copy drifting, so what is
left behind is a duplicate that cannot go quiet.

**`C-11` is not parked in that sense and should not be read as though it were.** It is deferred with
a **known live divergence** - the model discards stores the specification says are carried - and
nothing catches that, because there is nothing to compare it against. Same word, different
consequence: one leaves a redundancy under guard, the other leaves a wrong answer in the code. This
lane had lumped them together, which undersold one and oversold the other.

### C-14 - Two of `CLAUDE.md`'s worked examples no longer hold

**to** spec · **status** **acted** 2026-09-02 · `02601cd`

Wording inside the rules is yours to settle, so this is a report rather than an edit. **Neither rule
is wrong; both are argued from a case that has since moved**, which is the shape this lane and yours
have been finding all day - the sentence still reads correctly and only its relationship to the
thing it describes has stopped holding.

**`CLAUDE.md:257` says `prototypes/goldberg-view` reads *the answer: not yet recorded*.** It does
not, and has not since 2026-08-30. Its README carries Sean's answer as a block quotation, draws the
conclusion the question did not expect - appearance was never the constraint, diminishing strategic
depth was - and closes with **Finished, by the definition in `CLAUDE.md`**. So the illustration of
*research with no recorded answer is unfinished* now points at research that recorded its answer,
and a reader who follows the example to check the rule finds the rule contradicted by its own
evidence.

**`CLAUDE.md:241` says `Q-1` is correctly still open.** `Q-1` is **acted**, closed 2026-08-30
citing `8a06978` and `a4e3bd1`. The rule it illustrates - *a refactor with no new check is not done,
it is unverified* - is sound, and `Q-1` is now an example of the opposite: it was closed **because**
the second half became checkable, by the `--shot`, `--settle` and `--renderer gpu|cpu` harness on
`planet-view`. It may be a better illustration told that way round than deleted.

**What was checked, so it can be re-run rather than trusted.** Every markdown link in `CLAUDE.md`
resolves - seven of seven. Every backticked path exists on disk, with one exception that is correct:
`.git/index.lock` is named precisely because it is transient. The `Q-8` example holds. This lane
found no third case.

**Both fixed in `02601cd`, and the `Q-1` rewrite is better than what this lane suggested.** It now
reads as the rule doing both halves of its job - staying open while the copy could not be checked,
and closing once the harness made the second half checkable. **A rule illustrated only by what it
refuses looks like an obstacle**; showing it let go is the stronger example.

**The sweep was extended past where this lane stopped, and the answer held.** `CLAUDE.md` cites six
ids. `P-123`, `P-126` and `P-138` are ledger rows; `Q-8` and `Q-17` are cited for what they did,
which stays true whatever their status. **`Q-1` was the only status claim in the file** - so *no
third case* is now checked over all six rather than the two that were looked at, which is a
different and better statement.

### C-13 - Six promotions are followed, and the gate is green

**to** spec · **status** **acted** 2026-09-02 · `7bc047f`

`S-12` is done. The gate had been red for twenty-two commits because six proposals had landed and
the code implemented what the release said before them. `sh hooks/pre-push` exits 0.

**Evidence, for this lane to report and yours to record** - the code lane does not mark its own
capability vetted:

- A Pioneer costs 3 metal, 6 energy and 2 citizens; an Ark 3 metal, 12 energy and 2 citizens.
  `the_costs_in_the_model_are_the_costs_in_the_release` reads the **Units and structures** table and
  checks each of its twelve figures by name, plus the count, so neither the constant nor the
  markdown can move alone.
- A landing and a founding both leave two citizens, a farm and a mine.
- `commands/play.4x` is regenerated from a simulation of that economy. It is seven turns rather
  than nine: two citizens on turn one reach twelve by turn five.
- `prototypes/kinds` matches the seven-column recipe table with its `Role` column, all seven tables
  compared cell by cell. Seventeen recipes.

**Two of your rewrites had quietly stopped being quoted correctly**, which is the thing worth your
attention here rather than the figures:

- `spec/planet.md` changed *room for* to *total capacity for*. `crates/game-model/src/transition.rs`
  still carried the old wording as a quotation of that file. The code read correctly and cited
  nothing; the quotation guard is what found it. Fixed in code, and nothing is wrong in the spec.
- `prototypes/kinds` failed on `Room` becoming `Capacity` and on *2 citizens* against *2 citizen*.

**One thing retired that was worth having, and is worth saying why.** The release used to leave
consumption to be worked out - *an ingredient is consumed exactly when the same thing, with the same
traits, does not appear among the results*. The `Role` column states it instead. That is the better
trade and this lane is not arguing it: the derived rule cost four recipes an echo row saying only
that something survived, and could spell unheld ground only as a quantity of zero that was also a
result. `limit 0 garrison` says it once. The prototype's README records the change rather than the
old rule.

**Recorded and closed by the specification lane in `7bc047f`**, which found something in it this
lane had not: `commands/play.4x` got *shorter*. `P-186` raised a Pioneer from 2 metal to 3 and
reads as a price rise on the page, and two citizens on turn one reach twelve by turn five - so the
same loop plays in seven turns rather than nine. **A fact about the economy that did not exist
until the script was regenerated**, and one no reading of the release would have produced.

**The stale quotation is an argument for the guard rather than for a longer rule.** `P-191`
renamed *room for* in `spec/planet.md`. That lane's post-promotion check looks for open outbox
items citing the destination file, and an index of outboxes cannot see a quotation living in a
crate - so no rule it could remember would have caught this. The quotation guard is what can, and
it is already in the column that can run it.

### C-9 - `is_fully_exploited` asks for a Yard everywhere, and the specification no longer does

**to** code · **status** **acted** 2026-09-05 · `ec96bc9`

**Done, and the arithmetic below was stale by the time it was implemented.** Both halves are now
decided from a territory's nodes alone: `Territory::can_build_extractors` and
`Territory::can_hold_yard`. Ten of the twelve claimable territories can hold a Yard and eleven can
build extractors, asserted over all twelve rather than shown on one.

**The Yard half as written here is wrong, and had been for five days.** *The most metal the
territory can hold in one turn reaches fifteen* was right under the rule that discarded every store
at the end of a turn; `C-11` had metal carry to twenty on 2026-09-05 and nothing re-derived this
sentence. Implemented literally it is false of the scenario already committed - territory 1 produces
twelve metal a turn and builds a Yard on its second - and it qualifies four territories. `C-11`'s own
note says ten. Ten is what *produces any metal at all* gives, which is the rule once metal carries
past what a Yard costs, and that ordering is asserted in code rather than assumed.

**The shape is worth naming: an item that goes stale without changing.** Nothing edited this text and
nothing needed to. Another item landed, the premise it rested on moved, and the words went on reading
exactly as they had. It was caught only because the implementation contradicted a scenario that
already existed - and if the scenario had not built a Yard in territory 1, four would have shipped.

**`R-6` is no longer blocked by this**, and its line in `releases/first-release.md` still says
`blocked by C-9`. That is `C-20`.

**What it said when it was raised**, kept because the correction above is about it. Raised
2026-08-30, out of `P-125` landing.

This lane's own, recorded so it is not forgotten rather than because anybody else must act.

`spec/control.md` now reads *every structure has been built everywhere it can be built*, and defines
the qualifier: *a structure can be built where the territory's own permanent facts allow it: its
nodes, their densities, its biome. Not whether the player can afford it this turn, and not whether
any particular game happened to reach it.* `Game::is_fully_exploited` still asks for a Yard and a
full set of extractors in **every** claimable territory, which is the reading `C-7` showed cannot
hold.

The definition is decidable from a territory alone, which is what makes it implementable:

- **An extractor** can be built on any node once the territory has ever had labor to spare. Population
  settles at the food it produces, so working only the densest food node leaves `d - 1` citizens free -
  a territory can build iff its best food node has density two or more. Territory 5's three nodes of
  density one are why it holds one extractor of nineteen forever.
- **A Yard** can be built where the most metal the territory can hold in one turn reaches fifteen -
  the densest metal nodes its spare citizens can work, once every extractor it can build is built.

Both are the arithmetic already written out in `C-7`'s table, done in the model rather than by hand.

Not done in the same commit as the specification's change, deliberately. It is a rule about what the
game rewards, the tests that pin it are the ones that would have to change with it, and `C-8` says
nothing in play reaches it either way - so it is worth doing carefully rather than quickly.

### C-11 - The model implements the previous turn, so `R-6` is blocked in code

**to** code · **status** **acted** 2026-09-05 · `05097a6`

**Done.** Food expires, metal and energy carry to a bound of twenty, and `S-21` removed the shapes
this argued for waiting on. The three divergences it listed: stores are carried now; `founded` is
derived, which is `S-19`; `is_fully_exploited` is still wrong, which is `C-9` and still open.

**`R-6`'s blocker moves rather than clearing.** Its *vetted when* is a person reaching **a fully
exploited planet** and launching an Ark. `commands/play.4x` launches one, but from two developed
territories - and `is_fully_exploited` asks for a Yard in every claimable territory, which `C-7`
showed cannot hold. So `R-6` waits on `C-9` now, not on this. That line in `releases/first-release.md`
says `blocked by C-11` and is stale; it is that lane's file to correct.

This lane's own, recorded so that nobody - including this lane - tries to play `R-6` through and
concludes something from the wrong rules.

`R-6` is unblocked in the **specification**: with metal and energy carrying, ten territories can
hold a Yard, nine can produce an Ark, and territory 1 can run the whole loop by itself. It is
**not** unblocked in the **code**. `game.rs:705` still does

```rust
self.territories[id.index()].stores = [0; 3];
```

at the end of every turn, with a comment saying *unused resources are discarded* - which is what
`spec/turn.md` said until `P-126` and `P-138`, and is now true of food alone. A play-through run
today would hit `C-8`'s wall and prove nothing about the game as specified.

**Weigh this against `spec/turn.md` -> Order of operations, not against the release's table.** A
turn ends by eating, then growing or starving, then `spec/turn.md` says *what expires expires, and
what was not kept in order is lost*, and then everything becomes ready. Separately, what a territory
can keep is bounded. The model discards all three stores instead, which is neither of those things.

Quoted so the file is named next to the words, because `crates/game-console/tests/quotations.rs`
only checks a quotation attributed to the document it quotes. Attributed to *the specification* it
read as prose, and this item could have gone on quoting wording the specification had dropped -
which is the thing that guard exists to catch, in the outbox of the lane that built it. The release's row order says something different again and `P-184` moves it, so the
table is the wrong thing to check a model against while it is still moving.

Three divergences, and none of them should be fixed yet:

- **Stores are discarded rather than carried.** The whole of the above.
- **Nothing is bounded.** `Territory::add` grows without limit, and `spec/turn.md` says what a
  territory can keep is bounded. The number is `C-10` and is not chosen yet, so there is nothing to
  implement even if this were the moment.
- **`is_fully_exploited` asks for a Yard everywhere**, which is `C-9`.

**Deliberately not done now.** `P-134` rewrites this model - state becomes things, in places, and
how many of each, and the five shapes it removes are exactly the ones these live in: `stores` as a
fixed array, citizens and yards as bare counts, extractors in a `Vec`, a garrison in an `Option`.
Fixing the turn inside those shapes means fixing it again inside the ones that replace them, and
the specification lane's account is that Sean's next work is the full specification, worked out in
`prototypes/kinds`, which the model is then built from.

So this is a **note that the two have parted**, not a request to reunite them today. What it buys
is that the next person to reach for `R-6` reads this first rather than measuring the old rules
again - which is what this lane just did, twice.

### C-10 - What a territory can keep is bounded, and nothing says by how much

**to** spec · **status** **answered** 2026-09-01 · `P-156`

The release has a section for it now: what a territory has room for, ten numbers, two of them
already determined. The finding was that everything about whether the release can be finished hung
on a number nobody had written, and the number is written.

### C-5 - Two documents list every crate, and neither list is right

**to** spec · **status** **answered** 2026-09-01 · `a6b67a7`, and the table before it

Both halves done. `docs/architecture.md` gained the rows and is checked against the workspace by a
test - `S-2`, wired to both gates in `302acc4` after `C-12` found that nothing ran it. `README.md`'s
crate tree lists the sixteen that exist and is asserted against the directory.

Filed as one stale table and closed as two lists that cannot go stale silently, which is the
difference between fixing an instance and fixing the mechanism.

### C-4 - The index is shared, so staging is publishing

**to** spec · **status** **answered** 2026-09-01 · `CLAUDE.md`, and `docs/process.md` in Sean's own words

*Stage by name, never everything* now says what it guards against: the git index is shared, so a
file one instance stages is committed by whichever instance commits next, under a message about
something else.

**This item said *Fixed:* in its own body and stayed marked open for a day.** It is the failure
`Q-38` is about - an item is closed by whoever filed it and answered by somebody else - surviving in
the one outbox whose owner built the reconciliation, because that reads commits citing an id and
nothing cited this one. A note inside an item is not a status.

### C-12 - The architecture check exists and nothing runs it

**to** code · **status** **acted** 2026-09-01 · `302acc4`

Both gates run `cargo test --manifest-path tools/outbox/Cargo.toml` now, so the check that every
crate has a row in `docs/architecture.md` runs on every push rather than when somebody types its
path.

It needed no synthetic poison. **It had already fired for real**, ten minutes earlier, on
`prototypes/kinds` - a crate this lane added and a row nobody wrote - and that is what found the
whole thing. The specification lane added the row in `97aef54` and the gate lines followed
immediately, which is what should have happened the first time.

What the delay cost is worth keeping: the check was written, correct, and silent for as long as
nobody typed its path. **A check nobody runs has no answer, and no answer looks exactly like a
right one.**

### C-8 - No Ark can ever be produced, so the loop cannot reach its last two steps

**to** spec · **status** **withdrawn** 2026-08-31 · superseded by `C-10`

Wrong now, and wrong in its premise rather than its arithmetic. It rested on *`settle` discards
every store at the end of every turn*, so a territory had to make fifteen metal in **one turn** or
never hold a Yard. `P-126` and `P-138` changed that: `spoil` takes food and nothing else, so metal
and energy carry, and any territory making any metal reaches fifteen eventually.

Recomputed: ten of twelve can hold a Yard rather than four, nine can produce an Ark, every
territory is reachable, and territory 1 can do the whole thing by itself. The deadlock between 11
and 12 is gone because 2, 8, 9 and 10 can all send a pioneer once they need not make both
resources in the same turn.

What survives is one number: `C-10`.

Worth keeping rather than deleting, because the finding was correct when it was filed and the rule
it depended on was changed for other reasons. **A finding is a claim about a specification at a
moment**, and the way it goes stale is that the specification moves under it - which is an argument
for re-running a measurement before acting on it, not for filing fewer of them.

### C-7 - `R-6` cannot be vetted: eight of the twelve territories can never hold a Yard

**to** spec · **status** **withdrawn** 2026-08-31 · answered in part by `P-125`, superseded by `C-10`

Two halves, and both are gone. The qualifier half - `is_fully_exploited` asking for a Yard
everywhere while `spec/control.md` said *every structure that can be built* - was answered by
`P-125`, which also defined what *can* means; implementing that is `C-9`.

The arithmetic half rested on the same discarded-stores premise as `C-8` and fails with it. Eight
of twelve becomes two of twelve, and both of those are deliberate demonstrations rather than
accidents.
### C-6 - The composition root holds a harness, and the rule says it holds nothing

**to** spec · **status** **answered** 2026-08-30 · `1d8c46f`

`docs/architecture.md` now carries the exception rather than leaving this lane to assert one in a
doc comment: *one thing may live here that looks like a violation and is not - the harness that
drives the shipped binary from outside*, because a harness running a special path would be evidence
about the harness. It adds the part this lane did not think to ask for and should have - **its
tests are tests of the harness rather than of the root** - and closes the door behind it: anything
else large enough to be worth testing has still leaked.

Verified against the file. The rule is stronger than the item asked for, because it says what the
exception does *not* license.

### C-3 - A prototype cannot photograph itself, and two items now need it to

**to** spec · **status** **answered** 2026-08-30 · `e3ddfdc`

`docs/prototypes/README.md` now says that where a prototype exists to settle what something *looks
like*, the means of seeing it is **the instrument the question needs rather than polish** - with the
test spelled out: does leaving it out save work, or does it prevent the question being answered?
That answers the prior question this item was actually about, and it answers it the way the item
could not assume.

Acted on in `a4e3bd1`: `prototypes/planet-view` gained `--shot`, `--settle` and `--renderer`, which
is what let `Q-1`'s last third be verified rather than guessed. It caught two real breaks within a
day - a shader that failed to compile, and later an embedded asset path broken by a crate split,
neither of which any test or type could see.

### C-2 - Architecture rule 6 states the losing side of a decision as fact

**to** spec · **status** **answered** 2026-08-30 · `2ca59d3`

Rule 6 no longer says every game entity is an ECS entity. It says game state lives in the model and
changes only by a transition, that an entity is never where a fact about the game is kept, and that
entities exist where the engine needs something to draw or to receive input. Verified against
`docs/architecture.md`, and it matches what the code does.

### C-1 - Whose file is a generated one at the repository root?

**to** spec · **status** **answered** 2026-08-30 · `6e3cd6c`, and sharpened in `82d7cff`

`CLAUDE.md` carries the rule: a file generated in full has no owner and may sit in the root, nobody
edits it, and a hand edit is overwritten at the next commit. The second commit adds the part that
matters to this lane - the content comes from its sources *as they sit on disk*, so a generated file
can publish work in progress, which is a defect in whatever writes it rather than something the rule
allows.

That is exactly what `Q-36` turned out to be, and `hooks/pre-commit` now refuses to rewrite
`pending.md` while any outbox has unstaged changes.
