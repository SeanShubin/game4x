# Research outbox

**Derived.** The research lens's one outbox. Every finding it has addressed to somebody, and what
became of it. Not binding - a finding is a claim about the tree, not a decision about it.

[Research](README.md) · [Reports](README.md#reports) · [The proposal queue](../../docs/notes/proposals.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to spec` - something for `spec/`, `releases/` or `docs/`. The specification lane turns it into a
  numbered proposal; **it does not decide it.**
- `to code` - a defect or a decision in `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`,
  `hooks/` or CI.
- **Unaddressed** research does not appear here at all. It lives in a dated report and is nobody's
  work until this file gives it a reader.

**Status** is one of `open`, `noted`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` is
outstanding; `noted` is *recorded so it is not re-found*, and is terminal.

> **The guarantee.** If nothing here is `open`, this lens knows of nothing outstanding. That is a
> promise about this file, not about the tree - it does not say the documents are right, only that
> everything this lens knows to be wrong is sitting where its reader will find it.

**A producer may decline a finding, and often should.** It says so in the commit that declines it,
citing the id, and this file records it. Check a rejection before defending it.

## Open

### X-8 - `C-74` answered: the three puzzles are one, and it is called grounding

**to** code · **status** open · **raised** 2026-09-08 · **source** [report](2026-09-08-one-fact-two-interfaces.md) · answers `C-74`

**Answered where it was asked**, because that lane is prototyping and asked. **Nothing here proposes
a structure** - Sean said he does not want the present one presumed, so this names what is settled
and stops.

**The three puzzles are one question with a name.** A recipe is an operator with parameters; the rows
a person selects from are that operator **instantiated against the current state**, one row per
binding whose preconditions hold. Then: whether a condition is a precondition or a guard on an effect
decides whether a row **appears at all**; which parameters exist decides **what the person picks**,
because a parameter is a choice and a derived term is not; and the rows mentioning a selected thing
**are** the verbs available on it. The console and the interface become two renderings of one
operator rather than two designs to keep in agreement.

**`create-if-missing` is a conditional effect** - ADL, Pednault 1989; PDDL's `(when ...)` behind
`:conditional-effects`. It is not a fifth role, it is an effect with a guard, and **the guard sits
somewhere a precondition does not**. Today's `limit 0` is a precondition, so a garrison present makes
`deploy ark` inapplicable; his keeps it applicable and skips one effect. **In a selection-only
interface that is the difference between the player seeing the option and not seeing it**, which is
this lane's own inference and the reason it is worth Sean's attention: the two are equivalent in a
console and are not equivalent in an interface.

**And there is a second half.** In STRIPS an effect is a fact and adding one that holds is a no-op,
so create-if-missing is free. **The four roles are quantities** - the numeric-fluent world of PDDL
2.1, Fox and Long 2003 - where no increase is idempotent. So a garrison and a citizen are **different
kinds of thing sharing one Qty column**: one is a fact, at most one, and the other is a count.
**Object creation is where the analogy stops** - classical planning assumes a fixed universe of
objects, and `create citizen 2` does not.

**The subject moving is the relational-versus-functional choice**, and for an interface it is direct:
every parameter is something a person must pick, so choosing the representation *is* choosing what
the player selects. Noted: under a functional subject, `crates/game-model/src/rejection.rs`'s
`NotAboveThatTerritory` has nothing left to reject.

**The verb attaching to the thing is the oldest settled thing here.** It is **noun-verb**, or
select-then-operate - the Xerox Star, Smith et al., Byte 1982 - and what makes it right for a
selection-only interface is **modelessness**, not preference: selecting before commanding puts the
system in no mode, and verb-first does. **Sean's own sentence is the paradigm stated exactly**, and
this lane's contribution is the name and the forty-five years.

**The table question was the best one, as you guessed, and it has a precise name.** A table storing a
rule is **intensional**; a table a person selects rows from is **extensional** - IDB and EDB in
Datalog - and the structural rule is that **a predicate is one or the other and not both**. So the
selection table is not a prettier recipe table; it is a different predicate, produced from the recipe
table rather than maintained beside it. **This repository draws that line once and not twice**:
`CLAUDE.md` separates a rule from a rendering of a rule, and this is rule against world.

**Whether.** Worth reading now; **nothing to act on and nothing to build**. Three things are left
unsettled on purpose because they are decisions rather than facts - whether a garrison is a fact or a
quantity, whether an action's subject is derived or selected, and whether object creation stays
inside this vocabulary at all.

**A correction was stamped here on 2026-09-08 and withdrawn the same day.** `X-10` claimed that under fog of war the menu must be computed from the player's information set rather than the state. Sean refuted it: the condition it rested on is definitional and restricts no game. **This item stands as originally filed**, and the withdrawal is recorded in `X-10` rather than hidden by deleting it. **If any of them becomes a specification question it is `to spec`, not
yours**, and this lane has deliberately drafted no text: the vocabulary is the route and not the
destination, which `CLAUDE.md` names as the trap for exactly this kind of finding.

### X-11 - the four roles are two dimensions, and `create-if-missing` is a cell the grid always had

**to** code · **status** open · **raised** 2026-09-08 · **source** [report](2026-09-08-least-expressive-yet-complete.md), and Sean sketching four candidate primitives

**Sean is choosing a primitive set** - composition, subtract, fail-behavior, constraints on
parameters - and expects some to be subsumed. **His judgement is the measure and this does not argue
otherwise.** What it adds is one place the literature gives a decision procedure, and a
decomposition he can check against the specification without building anything.

**The test, and it is the whole of the item.** Nebel's compilation schemes: **conditional effects
cannot be compiled away preserving linear plan size, and can be with polynomial growth.** Gazen and
Knoblock's compilation removes them by expanding one operator into **one per combination of which
conditions hold** - exponential, and shown not improvable. **So fail-behavior is provably not
sugar.** The sharper point is where the cost lands: removing it **moves the explosion out of the
generated space and into the hand-authored one** - a recipe with four optional parts becomes sixteen
recipes a person maintains. Hence: **a primitive earns its place when removing it moves the
combinatorial explosion from the generated space into the authored space**, and anything whose
removal only enlarges the generated space is sugar.

**The decomposition, and the part worth checking today.** Two of his four are one thing - a
*constraint* is a guard and *fail-behavior* is where the guard attaches. And `subtract` has no
counterpart, which a **signed** amount repairs. Then the present four roles are **two independent
dimensions**, a **change** and a **threshold**:

- `consume 3 food` - change **-3**, threshold at least 3
- `require 3 workers` - change **0**, threshold at least 3
- `produce 1 metal` - change **+1**, no threshold
- `limit 0 garrison` - no change, threshold **at most 0**

**So `create-if-missing` is not a missing fifth role. It is a cell the grid always had**, which the
`Role` column could not name because that column conflates the two dimensions into one word. **That
is a defect in the present structure, found by re-encoding rather than by review**, which is what he
predicted re-encoding would do.

**The demonstration.** Under this decomposition, the question that took `C-74`, `X-8`, `X-9` and
`C-75` to circle is one word in one column: today's garrison is
`threshold(garrison, at most 0, **hard**)` plus `change(garrison, +1)`; his sketch is the same two
lines with **soft**.

**Three things that do not collapse**, worth knowing before they are discovered: a threshold with a
zero change is **not** expressible as a change, since a net effect of nothing is not a requirement -
it is a read arc and genuinely a second primitive. Upper and lower thresholds unify only via
complementary quantities, which is `C-75`'s construction and **probably not worth taking**, because
it costs a person reading *garrison-slots-free*. And **composition has an unmade decision inside it**:
whether each line sees the starting state or the effects of the lines before it, which a recipe that
consumes and produces the same resource will expose immediately.

**Whether.** **Worth reading before the primitive set is fixed and worth little afterwards.**
Nothing to build and no decision taken - whether composition is sequential, whether upper thresholds
stay, and whether a change and a threshold share a line are all left open on purpose. **The naming is
his**; this is a reading of his four, not a replacement for them.

**Decided 2026-09-09, and two of the three open questions above are answered.** Sean: a change and a
threshold **do** share a line, and a count may never go below zero - anything that would take it
there is an error.
[The report](2026-09-09-natural-numbers-and-the-fusion.md) carries the argument and the numbers; the
[recipe report](formulas.html) is re-rendered against it.

- **Upper thresholds are gone, and that is the point rather than a side effect.** The grid's fourth
  cell was `limit 0 garrison` - *no change, threshold at most 0* - and a fused subtraction cannot
  express `at most`. So the construct `X-9` identified as ending every analysis becomes
  **unwritable** rather than merely unused. What that rule needs travelling with it: `spoil` selects
  *each thing with keeps 0*, which is a zero test and is safe **only because `keeps` is bounded**.
- **The read arc survives exactly as this item predicted.** A threshold with a zero change is not a
  change, and one line needs it - `move`'s adjacency. It is now the primitive `require`, used once.
- **Measured before deciding**: 15 of the 16 guards were already a threshold and a destroy of the
  **identical** amount, 0 differed, and the 5 destroys with no guard are each safe by construction.
  Six primitives became five, 80 lines became 65, and re-encoding the release now **saves** ten lines
  against its own row count where it used to cost five.
- **`clamp` has a definition it did not have**: it is monus, truncated subtraction, the operator that
  makes the naturals a commutative monoid. It still has no call site. **`record` has a name**, and
  Sean's starvation case is what it is for.

**Decided later the same day, and it answers the third question too.** Sean took two more:
**amounts are signed**, so `create` and `consume` are one operation `change` - 17 positive lines and
21 negative - and the primitive set is genuinely **five** at last. And **composition is not
sequential**: every line reads the starting state.

- **The primitive count in this item's first update was wrong.** Fusing the guard into the spend did
  **not** take six primitives to five - `destroy` and `threshold` became `consume` while `require`
  split out, so six became six. It was typed rather than counted. The reduction came from the signed
  `change`, and every count on the generated page is now read from the data.
- **Non-sequential composition was measured before it was decided.** Over 22 recipes and 65 lines:
  **zero** lines read a count another line changes, **one** reads a trait another writes - `move`'s
  `let from = unit.location` against `set unit.location = $to` - and **four** were a `set` on a thing
  a `create` in the same recipe makes. `check.py` already assumed it: `effects()` sums a recipe's
  net in any order, because check 1 is a P-invariant and check 2 a linear program, so **sequential
  composition would make both unsound.**
- **It made the recipes shorter, which is the test Sean set.** A `change` carries its traits, so the
  four world-building create-then-sets became one line each: 13 lines to 4, 65 to 56, and the nine
  `set`s in world-building became **none**. `set` now exists only to change a thing that already
  exists, and there are five such lines.

**Nothing is open to you in this item any more.** All three of the questions it left are answered -
composition, whether a change and a threshold share a line, and whether upper thresholds stay. It is
kept `open` only because the primitive set is yours to build against and this is where its shape is
recorded; close it when you have read it.

### X-12 - `deploy ark` and `found by land` share seven rows verbatim, and that is the first call site for nesting

**to** code · **status** open · **raised** 2026-09-08 · **source** [report](2026-09-08-deploy-worked.md), and Sean asking for a recipe to be worked through

**Counted from `releases/first-release.md:202` rather than recalled.** `deploy ark` is nine rows and
`found by land` is eight. **Seven of them are identical** - `limit 0 garrison`, `produce 1 garrison`,
`produce 2 citizen`, `produce 1 extractor food`, `produce 1 extractor metal`, `produce 1 store food`,
`produce 1 store metal` - differing in nothing. The two recipes differ only in what is spent: an ark
from the orbit above, or a pioneer.

**Why it costs something.** *What a new colony starts with* is **two edits today, and one of them can
be forgotten.** Sean intends the recipes to be editable by players inside the game, which makes a
duplicated block a rule a player will change once and see take effect half the time.

**It is also the population `C-75` correctly said did not exist.** That lane refused to wire the
acyclicity check because no recipe calls any recipe, so the check would be green over nothing. **A
`found-colony` sub-recipe is the first call site**, and the check stops being vacuous the moment it
lands. **Not a request to extract it** - whether to is Sean's, and the item is the duplication.

**Two more things the worked example exposed**, both decisions rather than defects:

- **Deriving the territory from the selected ark deletes row 1 and a `Where`.** `require 1 territory
  @ $where` is a **parameter declaration** wearing a threshold's clothes, and there is no parameter to
  declare once the territory is `ark.location.below`; row 2's *orbit above `$where`* goes with it,
  because two things that cannot disagree need no constraint keeping them agreed. **Nine rows to
  seven, two `Where` expressions to none.**
- **A soft garrison threshold silently decides five other rows.** Today `limit 0 garrison` is hard and
  gates the whole recipe, so nothing fires twice. Make it soft, as `create-if-missing` does, and
  deploying onto an existing colony succeeds and adds **two more citizens, two more extractors and two
  more stores**. Each remaining line then needs its own answer and they are not the same answer.
  **The current table hides that question**; splitting the gate is what shows those rows were never
  individually considered.

**And his sketch omits both stores**, which the specification produces. Flagged rather than
reconciled - dropping them is a change to the game.

**Whether.** **The duplication is worth acting on and the rest is worth reading.** Nothing here
should be built without Sean deciding the two questions above first, and this lane has deliberately
not decided them. **One thing changes status rather than being new**: once players edit recipes, the
engine must reject a bad recipe *at edit time with a reason*, which a Turing-complete rule language
cannot do - so `X-9`'s boundedness and acyclicity stop being elegance and become editor validation,
and `C-75` already measured that adopting them costs nothing today.

### X-13 - creation and transformation are already one format, because relations were made things

**to** code · **status** open · **raised** 2026-09-08 · **source** [the report](formulas.html), generated from `tools/research/formulas/data.json` · **for** Sean, who asked whether one format can build the world and play it

**The whole specification re-expressed in one small primitive set**, with the world built from an
empty game in the same primitives. Six when this was raised, five since the signed `change` landed
on 2026-09-09. Copied and modified from `releases/first-release.md` at his instruction, so
divergence from it is expected and is a defect in neither.

**The finding.** Building the world needs **no primitive that playing does not** - every design-time
recipe is `create`, `set` and `call`, and **not one threshold**, because nothing at design time can
be refused. **World-building is the play language with the guards left out.** That works only
because the specification has been steadily turning relations into things: `deposit` and `adjacency`
are kinds, and `C-47` measured that nine of the dump's ten tables are the containment tree. **The
move that makes this possible was already made** and, as far as this lane can see, not for this
reason.

**A primitive the sketch was missing: `set`.** Four world recipes write a trait change as
destroy-and-recreate - `age` is *consume thing keeps N, produce thing keeps N-1*, `refresh` is
*consume not ready, produce ready*. Those are one assignment on one identified thing. `age` and
`refresh` go from two rows to one; `create labor` from three to two; `work` and `move` each lose a
pair. **`set` is not sugar**: traits are values, not counted things, so no arrangement of
create and destroy expresses *keeps one less*.

**Counts, computed by the renderer rather than asserted.** 58 rows became **68 lines** in bare
primitives - **up 10, exactly the direction predicted before any of it was written.** Almost all of
it is one cause: today's `consume` fuses a guard and an effect, and splitting them costs a line
each time; there are **15** such pairs. `consume` is sugar, so an author writes **53** - down 5.
**All three numbers are true and none is the point**: four roles that could not name
`create-if-missing` became six primitives that also build the world from nothing.

**Whether.** **A review artifact, not work.** Nothing to build. It carries **five decisions, none
taken here** - whether the colony garrison guard is hard or soft and what that implies for the six
lines beside it; whether the sketch's missing stores return; whether composition sees the starting
state or the running one; whether a thing has identity or is a count; and whether extractor capacity
is a threshold on the recipe or a property of the container, since making it explicit is what
`build extractor` gained a line for.

### X-14 - founding is an undeclared metal source; Sean has decided, and the release still says otherwise

**to** spec · **status** open · **decided** 2026-09-09 by Sean, **unwritten** in `releases/first-release.md` · **raised** 2026-09-08 · **source** [the report](formulas.html), check 1 · **found by** a check on its first run

**What is outstanding, as of 2026-09-09.** Not *is this a defect* - that was settled, and the rest
of this item is the working that settled it. **Sean decided the two stores are dropped from
`found-colony`**, which makes founding deliver exactly the 3 metal an ark and a pioneer are worth,
and check 1 has reported no violation over twelve recipes since. **The release has not caught up.**

- `releases/first-release.md:211-212` still produce `1 store food` and `1 store metal` under
  **deploy ark**, and `:224-225` still do under **found by land**
- `:63` still says metal is **conserved**, unqualified, where `work` mines it out of the ground -
  so the word is wrong in a second way that dropping the stores does not fix

**So this is a promotion waiting to be written, not a question waiting to be answered**, and the
decision is Sean's own rather than this lane's reading of one. **Where.**
`releases/first-release.md` -> *Kinds*: **metal** is *what things are built from; **conserved***.
And *Units and structures* gives each built thing a **Binding** - the metal locked inside it:
garrison 1, extractor 1, store 1, yard 15, ark 3, pioneer 3.

**What.** Those two statements together are a claim that a weighting exists under which metal never
changes. **It does not hold.** Founding a colony creates a garrison, two extractors and two stores -
**five metal of binding, consuming nothing.** So:

- **`found-colony`**: net **+5**
- **`deploy ark`**: spends an ark worth 3, creates 5 - net **+2** per landing
- **`found by land`**: spends a pioneer worth 3, creates 5 - net **+2** per founding

**Verified rather than argued.** `tools/research/formulas/check.py` computes it from the Binding
column, copied from the release and not invented. **13 of 17 recipes were analysed and 4 skipped**
because their amounts depend on the state - `grow`, `perish`, `upkeep`, `work` - so the population
is named and is not zero. **The check was poisoned first** and goes red on a wrong weighting.

**Why it costs something.** *Conserved* is the kind of word a later rule leans on, and a second
check found the same three recipes from the opposite direction - a structural unboundedness test,
which ignores guards entirely, named `found-colony` too. **Two checks of different character
agreeing is the part worth acting on.**

**Whether.** **Worth deciding, not worth fixing blind.** Three readings and this lane takes none:
the word *conserved* is wrong and should be qualified; founding should cost what it builds; or
founding is a deliberate exception and says so. **The first is free, the second changes the game,
and the third is a sentence.** It is a defect in the specification rather than in the prototype,
which is why it is yours.

**Rewritten 2026-09-09: Sean said the first version did not convince him, and he was right twice.**
His principle - *if it takes 5 metal to build a unit, then when that unit deploys 5 metal comes
out* - is the invariant worth testing, and pulling on it found two faults in the **check** rather
than in his reasoning.

- **The check tested a stronger claim than the words can carry.** Working a metal extractor mines
  up to 8 metal out of the ground, so metal is not globally conserved and **no arrangement of the
  other recipes could make it so.** `work` is now a declared source and the invariant is
  *conserved outside extraction*.
- **A family was hiding a kind.** `work` produces `resource[...]`, which collapsed to the family
  `resource` and carried no metal weight - so **the game's only metal source scored zero.**
  Reported by the checker now rather than quietly fixed; resolving families to kinds is still
  outstanding and is in `X-17`.

**The finding survives both, restated on his principle.** An ark costs 3 metal and its Binding is
3; founding delivers 5. So founding is **a second, undeclared source of +2 per landing** - not a
failure of conservation in general, but an unnamed source beside the named one.

**And his own sketch already fixes it exactly.** The colony without the two stores - which is what
`ark.deploy` wrote - delivers garrison 1 + extractor 1 + extractor 1 = **3**, precisely the ark's
Binding and the pioneer's. **`deploy ark` and `found by land` both go to exactly 0.** The two
stores are the entire discrepancy, so either they go or a unit that delivers them costs 5 rather
than 3. **That is arithmetic on his numbers rather than an argument**, and it turns the open
question about the missing stores from a matter of taste into one with a right answer.

**Decided 2026-09-09: Sean dropped the two stores**, and check 1 reports no violation over 12
recipes. **The check is poisoned against exactly this decision** - putting the stores back turns
`deploy ark` and `found by land` red again - so the green is a measurement of the fix rather than
an agreement with it. **Three of the fifteen recipes are still skipped** for state-dependent
amounts: `grow`, `perish` and `upkeep`. The population is named and is not zero.

**That decision is in this lane's model and not in the release**, which is why the item stays open
and why its head now says so. This paragraph said *Closed* for a day while the status line said
`open`, and a reader who trusted the prose over the field would have concluded the specification
had caught up. **The head of an item is what gets read; a paragraph two screens down is not.**

**Two of the three faults this item went through were in the check.** Its first version claimed
metal was not conserved at all, which no arrangement of the recipes could have made true, because
mining is a source. Its second still weighed a called recipe as independently firable. **Sean
refused the first version and the refusal is what found both.**

### X-15 - the inventory, so that nothing said in one conversation is lost on the way to `spec/`

**to** spec · **status** open · **raised** 2026-09-08 · **source** Sean, saying he may want to promote this and asking that nothing be lost

**Not a proposal and not a request to write one.** Sean said that if the recipe work goes well he
will want it in `spec/`, and asked that nothing be lost in the meantime. **This is the inventory of
what would have to become text, and where each piece already lives**, so that the promotion path is
a reading list rather than a memory.

| What                                                                                 | Where it is now                                                                                |
| ------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------- |
| The primitives, and which are sugar                                                  | [report](formulas.html) -> The primitives, What is sugar; `X-11`                               |
| The test for whether a primitive earns its place                                     | `X-11` - removing it moves the explosion from the generated space to the authored space        |
| Capacity belongs to the container, and `limit` is deleted                            | [report](formulas.html) -> Capacities; both uses were restatements                             |
| World-building is the play language with the guards left out                         | [report](formulas.html) -> Building the world; `X-13`                                          |
| `set` is a primitive and not sugar                                                   | `X-13` - traits are values, so no arrangement of create and destroy expresses *keeps one less* |
| The seven duplicated rows, and `found-colony`                                        | `X-12`                                                                                         |
| A derived subject deletes a parameter declaration and a qualifier                    | `X-12`, `X-8`                                                                                  |
| Grounding: the menu is the applicable ground actions                                 | `X-8`                                                                                          |
| Boundedness and acyclicity, and why user editing makes them a product requirement    | `X-9` (closed), `C-75`, `X-12`                                                                 |
| The three checks, and that a cap is a backstop and never the mechanism               | [report](formulas.html) -> Detecting a glitch, and The three checks as run                     |
| Conservation declarations already in the *Kinds* table, and that nothing checks them | `X-14`                                                                                         |

**Six decisions are open and none is this lane's**, all listed in the report: the garrison's
soft/hard cascade over the six lines beside it; attachment on the call versus on each create;
whether a call rolls back; the missing stores; identity versus count; and now `X-14`'s three
readings of *conserved*.

**What would still be needed before any of it could be promoted**, and none of it exists: **words**.
Every item above is a finding or a table, and **not one line of specification text has been
drafted** - deliberately, because the words are Sean's to approve and yours to write.

**Whether.** **Nothing to do today.** This is a holding record so that a promotion, if it comes,
starts from a list rather than from this conversation. **It is `open` rather than `noted` for one
reason**: if the recipe work is abandoned, this should be closed as withdrawn rather than left
implying work.

### X-18 - `reports/index.html` says six primitives, and the report it links says five

**to** code · **status** open · **raised** 2026-09-09 · **source** this lane's own change, which is what made it wrong

**Where.** `reports/index.html:51` describes the link as *the specification re-expressed in six
primitives, and the world built from an empty game in the same six*.

**What.** It says **six** and the page it points at says **five**. Nothing was edited to make that
happen: `153a13f` published the link at 10:24 with a description that was true of the report as it
stood, and `53df5c8` had already re-rendered it at 10:07 under Sean's decision that amounts are
signed, which made `create` and `consume` one operation. **The description went stale between being
written and being read**, and the count is now `len(DATA["primitives"])` in the renderer rather than
a word, so it will move again.

**Why it costs something.** It is the first sentence a person reads about the report, on the
published site, and it is wrong by one in a number the report is largely about.

**Whether.** **Worth fixing, and worth deciding how rather than just what.** The one-word fix goes
stale the next time the primitive set moves - which it has done twice in one day. A description that
does not state the count would not, and neither would one generated from the same data the report
reads. **This lane caused it and cannot fix it**, since `reports/` is your column; if you would
rather this lane stopped changing the number under you, say so and it will hold the primitive set
still until you have read it.


### X-19 - a territory has four traits and two of them have no value anywhere

**to** spec · **status** open · **raised** 2026-09-09 · **source** [the report](formulas.html) -> *A thing, as tabular data and as text* · **found by** trying to draw one territory and having two columns come up empty

**Where.** `releases/first-release.md` -> *Traits* gives a territory **biome** (*one of the biomes*,
stored) and **nature** (*a number*, stored). *Territory resources* fixes all twelve territories and
has four columns - Territory, Food, Metal, Energy - **and no biome**. *Biomes* describes what each
biome is like and says **force of nature is the one column that binds**.

**What.** **No territory is given a biome anywhere in `spec/` or `releases/`** - `grassland`,
`jungle` and `mountain` appear only in `spec/planet.md:54`, which lists the six, and in the *Biomes*
table's own rows. So the one column that binds, binds a territory's nature to a biome the territory
has not been given, and **nature is undetermined for all twelve**.

**Counted against a population that is not zero:** twelve territories, twelve without a stated
biome, and the resources of all twelve are stated in the row beside it.

**This may be correct rather than a gap, and that is the decision.** `R-4` is **vetted** on the
evidence that *`biomes_of` gives every territory one*, so the code assigns biomes from the planet's
geometry and no document was ever meant to. If that is the intent, the gap is not the missing values
but that **nothing says the values come from there** - a reader of *Traits* and *Territory
resources* has no way to learn it, and this lane guessed wrong for a day.

- **If biomes are generated**, one sentence in *Territory resources* saying so is the whole fix, and
  it names `R-4` rather than repeating it
- **If they are meant to be fixed like the resources**, the table needs a fifth column, and twelve
  values are Sean's to choose

**Why it costs something.** A territory is the thing the game is played on, and its row is assembled
from two authorities - resources from the release, biome and nature from the code. **That is fine
and unsaid**, and unsaid is what makes it a defect rather than a design.

**Whether.** **Worth a sentence, not worth a table** - unless the answer is the second reading, in
which case it is twelve values and only Sean has them. **This lane got it wrong in exactly the way
the item describes**: its own data said `make-territory(1, grassland, 1)`, asserting a biome and a
nature for territory 1 that nothing states, until `d74a1c6`'s successor removed it. **A document
that leaves a stored trait with no source invites the reader to invent one**, and this reader did.

**Checked again 2026-09-09, because Sean said these could be copied from the spec.** They cannot,
and this is exactly what to look for: `spec/planet.md:54` lists the six biomes and says ocean is not
claimable; the release's *Biomes* table gives numbers **per biome**; *Territory resources* gives
numbers **per territory** and has **no biome column**. **Nothing anywhere maps a territory to a
biome.**

**Nor can it be derived.** A territory's numbers do not match any biome's - territory 1 is
`3 x 4` for all three resources and no biome is - and the release says why: *the numbers here guide
and do not bind; a territory's own are in Territory resources.*

**The ask is smaller than twelve biomes.** Only `nature` is read by the force rule, and every biome
carries **1** except jungle, which carries **2**. So what is needed is **which of the twelve
territories are jungle** - and every other territory takes 1.


### X-20 - *declares no capacity* and *declares no limit* are opposites, and the release means the second

**to** spec · **status** open · **decided** 2026-09-09 by Sean, **unwritten** in `releases/first-release.md` · **raised** 2026-09-09 · **source** [the report](formulas.html) -> *Capacity, in four questions rather than one* · **found by** mapping Sean's capacity taxonomy onto the rules that already exist

**Three statements, and they cannot all hold.**

- `releases/first-release.md` -> *What bounds a kind in a territory*: **A territory declares no
  capacity for a resource.** It declares capacity for the things that hold them
- `spec/logistics.md` -> *Containment*: **A kind that declares no capacity contains nothing, and
  never can**
- `releases/first-release.md`, same paragraph: **A resource that is in nothing** can be used the
  turn it is made, and is lost when that turn ends

**What follows.** By the first two, a territory can never hold a resource. So a unit of metal that
no store holds is *in nothing* - and `spec/logistics.md` also says **every thing is in the game,
directly or through what contains it**, and that **the game is the one thing that is in nothing**.
A resource in nothing would be a second such thing.

**The escape does not work either.** If *in nothing* is read loosely as *in no store*, then the
resource is in the territory - and the first two statements say a territory can never hold one. The
reading that saves the third breaks the first, and the reading that saves the first breaks the
third.

**Why it costs something, and it is not a word.** *Each territory is self-contained. No resource
and no citizen crosses a territory boundary* - so where a loose resource sits is load-bearing. If it
is in the game rather than in a territory, the game holds a pool that has lost the one fact the
release insists on. **`work` puts metal somewhere and nothing says where.**

**This lane assumed one of the two readings and did not notice.** Every line of its encoding puts
resources in a territory - `{food territory:t}`, `{metal territory:t}`, `{labor territory:t}` - and
both checks aggregate by kind while ignoring location, so **neither check could have caught it.**
That is a count over the wrong subject in this lane's own tooling, and it is why this is filed with
the assumption named rather than as a clean finding.

**Whether.** **Worth deciding, and the fix is probably one sentence.** Three readings, and this lane
takes none: a territory declares capacity for a resource after all, and the number is unbounded; or
a loose resource is held by the territory as an exception the containment rule names; or *in
nothing* means *in the game*, and the release says what that costs. **The first is the smallest and
the third is the only one that changes the game.**

**Answered 2026-09-09, by Sean, the same day.** *Everything except the game itself has to be in a
location.* Extractors and stores are attached to the territory and limited by its capacity; a store
has a capacity of its own. **A resource, and the labor a citizen makes, are held by the territory
with no limit** - labor has to be somewhere, since a citizen makes it and an extractor consumes it.

**So the defect is two phrases, and the second is the interesting one.**

- *A territory declares no capacity for a resource* should say it declares **no limit**. Under
  `spec/logistics.md` those are **opposites**: no capacity means this sort of thing is never held
  here, and no limit means the maximum is unbounded. The release uses the first to mean the second
- *A resource that is in nothing* should say **what the territory holds directly**, rather than what
  a container in it holds. Nothing but the game is in nothing, and the rule needs to name what is
  lost without contradicting that

**What is lost at a turn's end is what the territory holds directly**, and Sean's reason for it is
**disorder**. The purpose is that an extractor is useful before anything has been built to store
what it makes. **Labor is the pure case**: no container ever holds it, so it is always lost, which
is why one citizen's labor is one turn's labor and never accumulates.

**This lane has changed its own model to match** - `9107583`'s successor. `move` no longer writes a
`location` trait, because `spec/logistics.md` says a thing is not located by a trait and what holds
it is what says where it is; moving is now leaving one place and entering another. **Neither check
can see that difference**, since both aggregate by kind and ignore where a thing is, which is why
this went unnoticed here for a day.


### X-21 - *use it immediately, store it, or lose it* has no recipe that does any of it

**to** spec · **status** open · **raised** 2026-09-09 · **source** [the report](formulas.html) -> *The kinds of storage* · **found by** asking the reverse of check 6 - is every declaration reachable?

**Where.** `releases/first-release.md` -> *What bounds a kind in a territory*: **A resource that is
in nothing can be used the turn it is made, and is lost when that turn ends - use it immediately,
store it, or lose it.** And *Where things are*: a store holds **10** of its resource.

**What.** **None of the release's sixteen recipes stores anything, and none removes an unstored
resource at a turn's end.**

- **Nothing puts a resource in a store.** `build store` creates the store; `work` creates a resource
  in the territory; no recipe moves one into the other. So a store can be built, has a capacity, and
  **nothing can ever be in it**
- **Nothing takes an unstored resource away.** `age` decrements `keeps` and `spoil` destroys what
  reaches zero, and **only food is made with `keeps`** - metal and energy carry none, so they are
  never aged and never spoiled. Unstored metal therefore persists for ever, which is the opposite of
  what the sentence says

**Counted, against a population that is not zero:** sixteen recipes in the release, 20 declared
(container, kind) pairs in this lane's model, and **`store` holds `resource` is one of two nothing
reaches**.

**One reading makes it smaller, and it is worth stating because it is probably the intent.** *Where
things are* says **which store holds which unit of a resource is not recorded in this release, so a
territory's amount of a resource is stated against the sum of its stores' capacities.** Read that
way, storing is not an action at all: a territory simply keeps as much as its stores can hold and
loses the rest. **That is a world recipe, and it is not among the sixteen either** - so the gap is
one missing recipe rather than two, and the sentence describes a rule the game does not yet run.

**Why it costs something.** A store costs 1 labor and 1 metal, and is one of six things a player can
build. **If nothing stores, a store is a building that does nothing**, and the whole
extract-then-keep loop the release describes is not there. `R-6` is *the loop can be played
through*.

**Whether.** **Worth deciding what the recipe is, not just that one is missing.** The two readings
differ: a player action that stores, or a world recipe at the turn's end that keeps what fits. The
second matches *which store holds which unit is not recorded* and needs no choice from the player;
the first is what *store it* sounds like. **This lane takes neither.**

**Answered and built 2026-09-09.** Sean: *just automatically cram as much as will fit in storage, the
rest goes into disorder.* So it is the world's, not the player's, and it is three lines:

```
stow  each t: {territory}
      change resource in t             -count {resource in t}          # take up everything loose
      change resource in {store in t}  +count {resource in t}  [clamp]  # put back what fits
```

**`clamp` does the whole of it.** It puts back what fits and no more, so the remainder is simply
gone - which is the disorder, with no second recipe and no test for how much was over. **This is
`clamp`'s first call site**, and it was declared as a candidate with no reason to exist since the
attach column was written.

**Check 8 now reports 19 of 19 declarations reachable.** *A store may hold a resource* was the one
pair nothing filled, which is how this item was found; it is filled.


### X-22 - the force rule is stated and nothing implements it

**to** spec · **status** open · **raised** 2026-09-09 · **source** [the report](formulas.html) -> *The loop Sean is building* · **found by** checking the recipes against Sean's stated intent

**Where.** Sean, 2026-09-09, stating what the recipes are for: *start with an ark in orbit, deploy
the ark to a single territory, **expand to all other territories from that territory, overcoming
nature in the process**, then fully exploit the planet … and finally launch a new ark into orbit.*

**What.** **`force` appears in no recipe line at all**, and a citizen, a garrison, an ark and a
pioneer each carry one. **`nature` appears in two lines and both write it** - `make-territory` sets
it and `make-world` passes it - so nothing ever reads it. **The release's own *Recipes* table
mentions neither word**, over all sixteen rows.

**So six of the seven steps of the loop have recipes and this one has nothing.** It is not a rule
that needs correcting; it is a rule that was never written.

**It compounds with `X-19`.** That item found that no territory is given a `nature` value in any
document, because `biomes_of` in the code assigns it. **So even with a mechanic there is nothing for
it to read**, and the two have to be answered together or neither is enough.

**Whether.** **Worth deciding, and it is the largest thing between the recipes and the loop as
stated.** This lane proposes nothing: what force does against nature - whether it is spent, tested,
or accumulated - is a game design question and not a gap to be filled by inference.

**Sharpened 2026-09-09 by Sean, and it is smaller than the item above suggests.** *Minimal territory
control is 1 garrison, 1 food extractor, 2 citizens. The garrison allows the citizens to sum their
force; the food extractor allows one citizen to provide labor to provide food to supply both
citizens.* Then: *if we don't have a recipe to generate force, how are we to hold the territory
against nature?*

**Force does not need generating - it is carried, and the release's own numbers say so.** *Units and
structures*: a citizen is force **1**, an ark and a pioneer **2**. **A garrison is force 0**, and
that zero is the whole design: a thing that *holds a territory* and contributes no force of its own
is an **aggregator**, which is exactly what Sean described.

**And the numbers are already calibrated.** Two citizens carry force 2; the highest force of nature
is 2, in jungle, and every other biome is 1. **So minimal control is exactly sufficient against the
worst biome, with no margin** - which is checkable and is now check 9.

**So what is missing is a comparison, not a producer**, and both halves already have a shape in the
model:

- **A territory's force is a derived trait**, a weighted sum over what it contains. The model has
  one of those already - `metal in it`, *its binding plus the metal in its parts*. Same
  construction, a different weight column
- **Holding is a guard** on that trait against `nature`, which is what `require` is for

**Two other items have to move with it.** `X-19`: no territory is given a `nature` value in any
document, so there is nothing for the guard to read. And check 9's food half found that
**territory 5 cannot sustain minimal control at all** - food density 1 against two citizens eating 2
- which is step 5a of the loop, the territories that do not count, exercised by exactly one of the
twelve.

**Still not proposed by this lane.** What holding costs, whether force is spent or merely tested,
and what happens when it is short are game design and not gaps to fill by inference.

**Stated 2026-09-09 by Sean, so this stops being a design question.** A citizen produces **1 force
passively**, exhausted or not. **Unorganized force is `max_of`; organized force is `sum_of`.**
Citizens are unorganized by default and organized where a barracks is present. **Units that move are
organized by construction** - moving is organizing force into an expedition. And the contest is at a
boundary: **greater force to enter, equal force to maintain**, where entering is the cost of
occupying rather than anything projected from outside.

**Three consequences this lane checked rather than assumed.**

- **Two pioneers enter a jungle and one cannot.** Nature 2, entering needs more than 2, and an
  organized pair is 4. **It is also the cheap path** - 6 metal and 12 energy, against 21 and 24 for
  two arks, which need a yard first. That is the tuning Sean asked for, and the numbers are the
  recipes' own
- **A barracks earns its place on exactly one biome.** Maintaining needs equal force, and at nature 1
  a single unorganized citizen is enough since `max` of anything is at least 1. **Only a jungle needs
  the sum**, which two citizens with a barracks give
- **`move` moves one unit, and the rule needs an expedition.** Two pioneers must cross together or
  each fails alone. The selection work already has the shape - a selection is a description and a
  count, so an expedition is `some 2 {pioneer …}` - but `move`'s quantifier takes one

**A contradiction this lane filed was not one.** It read *the garrison allows the citizens to sum
their force* against *a settlement that only projects 1* as disagreeing. **Both are true and the
barracks is the condition**: two citizens with one sum to 2, without one they are `max(1, 1)`.

**The organizer is the garrison**, which Sean confirmed: *I often get these terms mixed up, I mean
garrison, as that applies holding on to something.* The release already calls it *what holds a
territory*, with force **0** - the exact shape of a thing that organizes rather than fights, and the
reason its zero was worth noticing.

**And this lane had entering wrong.** It reasoned that entering unclaimed ground must be free,
because `found by land` consumes a pioneer in the territory it founds. Sean: the clash is at the
boundary and entering is the cost. **That is better than what was proposed** - the contest belongs on
`move`, which exists, and the ark stops being a special case, since an orbit and the territory below
are two places with a boundary like any other.

**Sean answered the consequence 2026-09-09**: *if you pass the force test you have presence there,
and therefore control. If you fail the force test you lose all your units.* **The failure half is
already built** - `unsustained` covers a thing whose upkeep is unmet *or* whose place is short of
force, and `perish` deletes it. Nothing more is needed for it.

**But one test cannot carry both halves, and the arithmetic says so.** A citizen is force 1, so
minimal control is 2; a jungle is nature 2.

- **If the single test is `>`** - greater, as for entering - then 2 &gt; 2 is false, so **a jungle
  colony loses its units the turn it is founded.** No jungle is ever holdable
- **If it is `&ge;`** - equal, as for maintaining - then a lone pioneer at force 2 passes, so
  **two pioneers are never needed** and the jungle is no harder than anywhere else

**Both readings break something Sean has asked for**, and the two-test version he described earlier
breaks neither: `>` when a player fires `resolve`, `&ge;` when the world checks each turn. **That is
the one thing still to settle**, and it is a choice between one test and two rather than a gap.

### X-23 - nothing ever fuels a unit, so nothing can move, so the loop cannot be completed

**to** spec · **status** open · **raised** 2026-09-09 · **source** [the report](formulas.html) -> *Check 8* · **found by** asking which declared containers are ever **filled** rather than merely mentioned

**Where.** `releases/first-release.md` -> *Where things are*: **a unit's tank** holds energy, up to
the unit's fuel. And *Recipes*: `move` consumes **1 energy** from the moving unit.

**What.** **No recipe puts energy into a unit.** Of the release's sixteen, `work` makes energy in a
territory, `produce pioneer` and `launch ark` spend energy from a territory, and none of them fuels
anything. So a pioneer is produced with an empty tank, `move` fails on its guard, and **step 3 of
the loop - expand to all other territories - cannot happen.**

**This is `X-21`'s shape a second time**, and the same question found both: *is every declared
container ever filled?* A store may hold ten of its resource and nothing puts any in; a tank may
hold the unit's fuel and nothing puts any in. **Check 8 missed the tank on its first run** because
it counted a consume as reaching a container - `move` spending energy looked like use.

**Fixed in this prototype, and the fix is a deletion.** Sean has said the prototype may remove
things and deviate from the release. `move` now spends its energy **where the unit is** rather than
from a tank, and the `fuel` trait, the unit-as-container and its declaration are gone. That is one
trait and one container fewer, and it makes the step reachable. **Check 7 reports the divergence** -
*fuel: only in the release* - so it is visible rather than silent.

**Whether.** **The release still has the defect, and the choice there is not this lane's.** Three
ways out: fuel a unit when it is produced, add a recipe that fuels one, or spend the energy where
the unit is as this prototype now does. **The third deletes rather than adds**, and the first two
keep a tank that would then need a way to be filled and a reason to exist.


## Resolved

### X-1 - what makes the game checkable by hand is never stated

**to** spec · **status** **acted** 2026-09-06 · `480efb4` — `P-317` promoted; the sentence is at `docs/process.md:100` · **raised** 2026-09-06 · **source** [report](2026-09-06-what-the-theory-already-settles.md)

`docs/process.md:91` says the definitions, the transformations and the commands *"are enough to
derive the fourth by hand"*, and rests *how I know the application is right* on that. It is true only
if the transformation reads the state and the commands and nothing else - and the document never says
so. Grepping `determin|replay|nondeterm|random|clock|reproduc` over `docs/process.md` returns **0
hits across 459 lines and 24 headings**, so the zero is against a named population that is not also
zero.

**Why it costs something.** The precondition is load-bearing for the artifact Sean checks by hand,
and it fails silently: a clock, an entropy source or an undeclared file read makes the fourth
artifact underivable while every test still passes and the scenario diff he reviews becomes noise he
cannot tell from a real change. It also decides a question the code lane is about to meet from the
other side - whether the environment a command executes against may answer anything the four
artifacts do not declare.

**Whether.** Worth doing now, and it is one sentence rather than a section. This lens has
deliberately not drafted it: the words are what Sean approves, and they are the specification lane's
to write.

### X-2 - the default in `layers.md` does not satisfy the rule in `turn.md`

**to** spec · **status** **acted** 2026-09-06, re-closed 2026-09-07 · `d44335a` — `docs/layers.md` superseded in that one respect and the hole deferred; `P-338` is what tracks it now · **raised** 2026-09-06 · **source** [report](2026-09-06-what-the-theory-already-settles.md)

`spec/turn.md:24` requires that what settles competing effects be *"a deterministic mechanic of the
game, and therefore something a person wrote and a player can change"*. `docs/layers.md:207` proposes
that where two events collide *"the lower index wins"*, and `:210` makes that reading **the
default**. An array index is deterministic and reproducible, and it is not a mechanic of the game,
nobody wrote it as a rule, and no player can change it.

**Why it costs something.** `docs/layers.md` is the non-normative why-layer, so this is not a
contradiction inside `spec/`. It is worse in one specific way: it is the document the code lane
reads for guidance, it says *default to this*, and following it would build something the
specification forbids. Nothing would report the divergence, because each file is correct on its own
terms.

**Noted alongside, and deliberately not filed as its own item.** The same decision has a second face
in the code - whether a command executes against a shared mutable environment or returns its effects
for a later merge - and settling it in one place and not the other is how it gets made twice,
differently. Recorded here rather than addressed to `code`, because the answer is not determined
until this item is.

**Whether.** Worth doing eventually rather than now. Nothing is being built on it today, and the
question is a real one for Sean rather than a defect to repair.

**Re-closed 2026-09-07, on `S-65`.** This closed into `P-318`, which the specification lane then
withdrew - so the closing line named a proposal nobody decided to drop, and `outbox --orphans`
reported it. **Sean's durability rule is the collision the deferral was waiting for**: a thing
carries a number of turns and paying its upkeep resets it, so a territory short of food resets some
citizens and decrements others, and **which ones eat is exactly the competing effect `spec/turn.md`
requires a deterministic mechanic for.** `P-338` asks him for that rule and is open to him, so the
finding now points at something a reader can follow. **The finding was right and stays right** -
only its tracker moved.

### X-3 - `S-57` answered: the cases are seven phenomena, and restarting fixes one

**to** spec · **status** **acted** 2026-09-06 · `00bc04b` — the count corrected and an eighth case added; its fourth question is answered by `X-6` · **raised** 2026-09-06 · **source** [report](2026-09-06-answering-from-memory.md) · answers `S-57`

Sorted by what actually failed, the seven cases in `S-57` share no mechanism and **only one is
memory**. **Corrected twice on 2026-09-07, and the second correction is not mine.** This item said four
phenomena; the report said one named by the question and four more, which is five, and the summary
dropped the distinction - a restatement going stale against a source that never moved, and the wrong
number reached the specification lane's close of `S-57`. **Then that lane refuted a case.** `P-315`
was filed here as *the information was there and I did not read it*; its withdrawal says the sentence
was read and misparsed, and `b1d12c9` **rewrote the sentence in the same commit that withdrew the
proposal**. So `P-315` is its own phenomenon and the count is six. `C-35` and `C-34` were re-checked
and hold. The six are: stale context (`P-320`); reading past a clause that was present (`P-315`, `C-35`, `C-34`);
never reading at all (`S-56`); a record destroyed by a process step (`P-310`, `P-312`); and a written
copy going stale on disk (quality's README). Three were verified against git; four are marked
unverified in the report. The item says four lanes and the cases name three.

**The three questions, answered.** *What needs re-reading* - not elapsed time but ownership: every
one of the seven is a claim about an artifact another writer can change, which the lane table already
makes mechanical. *Is any signal available from inside* - no, structurally, since a recalled fact and
a freshly read one occupy identical slots; **but the harness already emits one**, three times in the
session that produced this report, and it is silent for exactly the case where nothing was read at
all. *Does clearing context fix it* - **it fixes one of the four and makes two worse**, because a
fresh instance has no memory to contradict a stale document with. Quality's README is the proof: a
cold reader would have believed it completely.

**Why it costs something.** `docs/process.md` -> Outboxes rests on an instance being replaceable by a
fresh one that reads the files, and that holds only while the files are not themselves stale. The
remedy is not restarting; it is `docs/README.md`'s existing rule to link rather than restate, which
this answer independently arrives at. **So the finding largely confirms policy the project already
has**, and names which policy is doing the work.

**Whether.** Worth reading now, worth acting on only if Sean wants the one-sentence habit in
question 1 written down. **No text drafted** - the words are his to approve and yours to write. The
obvious next study, not started: run a cold instance against the seven questions and measure rather
than classify.

### X-4 - the rule that keeps a lane moving is written for one lane, and the rule binding all of them says stop

**to** spec · **status** **acted** 2026-09-06 · `f810732` — `P-324` promoted; `docs/process.md:205` no longer says a blocked instance stops · **raised** 2026-09-06 · **source** Sean, stating the operating model in a session where his own documents do not carry it

**Where.** `docs/process.md:273`, under *Coding instance*: *files a question ... and carries on with
everything that does not depend on the answer*. `CLAUDE.md:340`, cycle step 9: **the code lane hits a
gap and does not stop**. Those are the only two statements of the rule - two occurrences in
`process.md`, both about code, the second inside the coding instance's own starting prompt.

**What.** `docs/process.md` -> *All lanes* states the opposite default for everyone else: *an
instance ends its turn when it is **blocked**, when it is holding for a stated reason, or when
nothing is open to it.* So the general rule says a blocked lane stops, and only the code lane is told
to file the block and keep going.

**Why it costs something.** Sean described the intended model today: a lane works its backlog through
to the end, files what it notices to the right backlog, and stops only for a decision that actually
blocks the remainder - twenty tasks becoming twenty done, four decisions and three proposals, with
stopping as the exceptional case. That is the *Coding instance* rule generalised, and no document
carries it. **A lane reading only what binds it will stop early and be correct to.**

**It has already fired, in this lane, today.** This lane ended a turn by asking Sean whether to run
the cold-instance study `X-3` calls for, rather than filing it and continuing. Under *All lanes* that
was right; under the model he stated it was not. `X-5` is that question, now filed instead.

**Whether.** Worth doing now, and it is a scoping change rather than a new idea - the words already
exist at `:273` and are addressed to one lane. **No text drafted**: whether the general form belongs
in *All lanes*, and what it does to *ends its turn when it is blocked*, is Sean's to settle and yours
to write.

### X-5 - the measurement `X-3` lacks needs an instance nobody has authorised

**to** spec · **status** **answered** 2026-09-07 · `b56c5bf` — Sean declined `P-350`: *fix the README, let `X-3` stand as the derivation it says it is, and note what future restarts show.* · **raised** 2026-09-06 · **source** [report](2026-09-06-answering-from-memory.md) · **designed** [report](2026-09-07-the-cold-instance-study.md)

**What.** `X-3` answers `S-57` by classifying seven cases rather than by measuring, and says so. The
measurement it wants is a cold instance run against the seven questions - one that has read the files
and nothing else - to test whether a fresh reader avoids each failure or reproduces it. That would
settle question 3 as evidence instead of as derivation.

**Why it is filed rather than done.** It needs an instance started for the purpose, which is Sean's
resource rather than this lane's, and this lane's instructions do not let it spawn one unasked. It
was recorded inside `X-3`'s prose, where `pending.md` cannot see it - which is the same failure as a
promise living only in a proposal that promotion deletes.

**Whether.** Worth doing eventually rather than now. `X-3`'s answer stands without it; what the
measurement would add is the ability to say *how much* of a long-running instance's belief is memory,
which no amount of classifying will produce. **Nothing waits on it.**

**Designed 2026-09-07, and it is smaller than this item said.** The design is in
[its own report](2026-09-07-the-cold-instance-study.md); what it needs from Sean is one instance's
startup, not a study. **Two corrections to the words above.** It is four tasks and not seven - of `X-3`'s
seven phenomena, stale context is absent from a fresh instance by construction, a record destroyed
at promotion is not there for any reader, and `P-315`'s misparsed sentence was rewritten by the
commit that withdrew it. **Five of the eight cases stay runnable**, and the four tasks are one per
runnable phenomenon. And the tempting cheap form - hand a cold instance the cases and
ask which it would have got right - **measures findability rather than finding**, which is the shape
`docs/process.md` warns returns a plausible number and invites no question.

**The predictions are pre-registered in that report, before any run.** `X-3` is this lane's own
answer, and a study designed after seeing it will agree with it unless what would refute it is fixed
first. **Two of the four tasks can cost `X-3` its central claim** and two can only confirm - the
second of the two arrived from the code lane, volunteered against itself, after the design was
written.

**Why it is worth lifting out of the queue rather than waiting its turn.** Sean is restarting the
instances after a crash, so a reader that has read the files and nothing else is a state he is
producing anyway. **The cost is three tasks at one instance's startup, and no writes.** The
specification lane has been asked to put it to him as its own decision.

**Closed 2026-09-09, and it should have closed on the 7th.** `docs/notes/proposals.md` recorded the answer and ended with *`X-5` is answered and is the research lens's to close* - and this lane did not read its own outbox against the queue for two days. **The item was answered by somebody else, so its filer got no signal**, which is the exact case `pending.md` has a section for. What would have caught it is the check this lane now runs at the start of a session: read the open items from the files, then look for what has already acted on them.

### X-6 - a known rule is not applied when nothing carries it

**to** spec · **status** **acted** 2026-09-06 · `4d5a081` — filed as `P-327` and promoted; the *Ask what fires a rule* paragraph is at `docs/process.md` -> What makes a check worth having, and the carrier half is the code lane's `C-55` · **raised** 2026-09-06 · **source** [report](2026-09-06-when-a-known-rule-is-not-applied.md) · answers `S-57`'s fourth question

**Answers the question the eighth case added.** *Normalize both sides instead of loosening the
comparison* was proposed, approved and promoted by the specification lane, which then broke it five
times the same afternoon and broke the adjacent heredoc rule while repairing it. **This lane broke
the same heredoc rule hours later**, writing the report for `X-3`, having read it that day - a ninth
case, verifiable in the session.

**Some rules fire at a moment of doubt and some fire at a moment of confidence.** *Name the
population*, *re-poison a check*, *do not take another lane's message as true* are each fired by
something that happens - a number appearing, a list growing, a message arriving - so they survive as
habits. *Normalize both sides* and *write a script to a file* are fired only by remembering, while
the string being written looks correct. **The failure and the confidence are simultaneous**, which is
why no amount of care removes them.

**So the remedy is a carrier rather than a clearer sentence**: a check catches it afterwards and
shares the defect here, because the rule governs how comparisons are written and a check is a
comparison - four of the five broken assertions were in checks. A default path where the rule cannot
be broken is stronger and often free; the heredoc failure was fixed by using a tool whose quoting is
one level by construction, which made the rule unnecessary rather than better remembered. Checked:
`tools/` has no general normalizing comparison, only one incidental use at
`tools/outbox/src/lib.rs:286`.

**The test it yields.** For any rule in `docs/process.md` or `CLAUDE.md`, ask what fires it. If the
answer names something that happens, it can be a habit. **If the answer is *remembering*, it needs a
carrier**, and writing it down more emphatically is not one.

**Whether.** Worth reading now; worth acting on as a code-lane item only if Sean wants the carrier
built. **No text drafted and no carrier designed** - where an anchor-matching helper belongs is the
code lane's, not this lane's.

### X-7 - the carrier for the staging hazard is already built, and nothing tells a lane to use it

**to** spec · **status** **acted** 2026-09-08 · `a882290` — `P-352` promoted; `CLAUDE.md:133` now says staging by name bounds what you add and not what you commit, and `hooks/pre-commit` refuses a commit spanning two columns · **raised** 2026-09-07 · **re-addressed** 2026-09-07 from `code`, before that lane read it - the mechanism it asked for already exists · **source** [report](2026-09-07-the-shared-index-race.md), and the code lane reporting it against this lane's `8f687d5`

**Where.** `CLAUDE.md:129`, the *stage by name* bullet. The event: `8f687d5` is this lane's commit
and carries 21 lines of `crates/outbox.md`, outside its column and unmentioned in its message -
verified here with `git show --stat`, not taken from the report.

**What.** The rule's remedy cannot prevent the failure the rule describes. This lane **did** stage by
name and the failure happened anyway, because staging by name bounds what you add and the hazard is
what somebody else added between your `git add` and your `git commit`. The code lane checked for the
lock first, which does not help either - the window is after the check. **It is a
time-of-check-to-time-of-use race on shared mutable state**, and that is why care does not close it.

**Why it costs something.** It has now fired twice - twenty-six lines the first time, twenty-one
this time - and both times the content was correct and the *message* was lost, which is the part
no reader can reconstruct. It also puts a lane's work in a file outside its column, which is the
one invariant the perspectives rest on.

**What removes it is already built, which this lane found only after filing.** `git commit -- <paths>`
implies `--only` and builds from a temporary index. **`hooks/post-commit` exists to support exactly
that**, wired by the code lane in `a60def3` after the quality lens found the residue it leaves. So
there is nothing to build and this item is no longer addressed to that lane.

**What is missing is the instruction.** `pathspec` appears in `hooks/post-commit` and in the quality
lens's own files, and **nowhere in `CLAUDE.md` or `docs/process.md`** - which still say the remedy is
to stage by name. A lane following its instructions exactly gets the failure; the carrier sits there
unused because nothing sends anyone to it.

**This is `P-327`'s own pattern, one week on.** A rule that fires at a moment of confidence needs a
carrier rather than a better sentence - and here the carrier was built and the sentence was never
changed, so the rule still names the remedy that cannot work.

**Measured anyway, and it stands as confirmation rather than discovery.** Three runs in a throwaway
repository plus a live fourth: `50c69ee`'s successor, this lane's own `X-7` commit, was made
`git commit -- lenses/research/ pending.md` with two of another lane's modified files in the tree,
and took only its own four. **The row this lane expected to fail did not** - the `pre-commit` hook's
regenerated `pending.md` still landed, so the objection that a temporary index would break it does
not hold.

**One thing this lane listed as unsettled was already settled**, which is the same error twice in one
item: the `MM` residue is what `hooks/post-commit` handles, and this lane's own commit printed it
doing so. Still untested: `hooks/pre-push` and the padding path, and the lock collision was not
forced to confirm it fails loudly rather than wrongly.

**Whether.** Worth doing now, and it is one sentence rather than a mechanism - which is why it moved
to you. `CLAUDE.md:129` states a remedy that cannot prevent what the bullet above it describes, and
the thing that can is already in the tree. **No text drafted**: the words are Sean's to approve and
yours to write, and whether a lane should be told to commit by pathspec at all is a decision rather
than a typo. **Declining it is reasonable** if he would rather lanes not learn a second git idiom -
the hazard has fired twice in a fortnight with the content intact both times.

**Closed 2026-09-08, verified in the destination rather than from the commit.** `CLAUDE.md:133`
carries the mechanism - *staging by name bounds what you add and not what you commit* - and the
bullet now says the window falls between `git add` and `git commit` and that checking for the lock
falls before it. **It went further than the item asked**: `hooks/pre-commit` refuses a commit whose
files span two perspectives' columns, so the hazard has a gate and not only a sentence. The code
lane's phrasing is what landed, which is the right outcome - it had the mechanism in one sentence
and this lane did not.

### X-9 - `limit 0` on an unbounded quantity is a zero test, and it ends every analysis

**to** code · **status** **acted** 2026-09-08 · `765ca85` — measured against the release and passed to spec as `C-75`; one of the two checks refused, correctly · **raised** 2026-09-08 · **source** [report](2026-09-08-simple-finite-and-decidable.md), and Sean stating what the recipe structure is being replaced with

**Sean is prototyping a replacement for the recipe structure** - simple and finite, unlimited
complexity from composition and from nesting, with Master of Orion 1993 as the expressiveness
target. **This is filed to you because you are building the prototype**, and every item in it is a
property a check could hold. **Nothing here proposes a structure and no text is drafted.**

**Where.** The garrison rule, `limit 0` **and** `produce 1`, and any rule shaped like it.

**What.** `require`/`consume`/`produce` over counted things **is a Petri net** - the same object as
Factorio's recipe graph, which is why ratio analysis works there. **`limit 0` is an inhibitor
arc**: a transition that fires only when a place is empty, which is a zero test. **Petri nets with
inhibitor arcs are Turing-complete**, and two of them model a two-counter machine. Reachability
survives one and dies at two.

**Why it costs something.** A plain Petri net gives **decidable reachability** and the structural
analyses a 4X designer actually wants - **P-invariants** name what is conserved whatever is played,
and **T-invariants** name recipe cycles that return the world to its start, which is to say **an
unintended infinite-resource loop is computable rather than something you playtest for.** Crossing to
Turing-completeness gives all of that away, silently, and nothing in the present structure says the
line is there.

**The distinction that saves it, and it is not the obvious one.** Moving the zero test from a
precondition to a guard on an effect - which is what `create-if-missing` does - is right for `X-8`'s
reason and **does not buy decidability**; a zero-guarded effect can record the test's result
elsewhere and the branch returns. **This lane's first draft claimed otherwise and was wrong.** The
real line is **boundedness**: an unbounded place cannot be zero-tested safely, and a **bounded** one
can, by the standard complementary-place construction, with no inhibitor arc and no loss.

**Which is `C-74`'s split arriving from the other side.** A garrison is a *fact* - capacity one - and
citizens are a *quantity*. **Zero-testing a capacity-one place is free; zero-testing food is the
cliff.** So the fix is not to remove `limit 0` but to make the kind of thing it tests **declarable**,
which is a property a check can enforce over the whole recipe set.

**And nesting has its own edge.** One recipe calling another is **hierarchical task network**
planning, which is **strictly more expressive than STRIPS** and **undecidable in general**, because
decomposition can recurse. **Decidable subclasses are syntactically identifiable**, and the cheapest
is **acyclic decomposition** - no recipe transitively calls itself. That is a graph check over the
recipe set, it runs in milliseconds, and it costs almost none of the explosion he wants, which comes
from branching rather than from recursion depth.

**Whether.** **Worth knowing before the prototype's core is fixed, and worth nothing afterwards** -
that is the whole reason it is filed now rather than noted. Two of the three are checks you could
wire cheaply and both fail loudly: **a place is declared bounded or a zero test on it is refused**,
and **the recipe call graph is acyclic**. The third is not a check and is Sean's: Master of Orion
needs hidden information and randomness, and Distant Worlds 2 needs continuous time - the game
description language that already solved *any finite deterministic game with full information* needed
**different languages**, not later versions, for exactly those two things. **Deciding where they live
before the core is fixed is cheaper than every alternative.**

**Answered 2026-09-08 by the code lane, which measured rather than took it, and corrected this lane twice.** **First**, this item implied the present structure had already crossed the line. It has not: **both zero tests in the release are `limit 0 garrison`, and garrison has a stated capacity of one**, so all of them sit on a bounded place. The report contradicted itself - its own next subsection says a bounded place is safe - and the measurement is what exposed it. **The cliff is ahead, not behind**, so the rule costs nothing to adopt now and stops being free at the first zero test against food or a store. Their split of the eleven bound kinds, five bounded by a stated capacity and six by something else, is `C-74`'s fact-versus-quantity line arriving a third time.

**Second, they refused the acyclic check and were right to.** No recipe calls any recipe today, so the check would be green over an empty population - which is the exact defect this repository has spent a day removing, and which this lane proposed without counting the population first. **It becomes worth wiring on the first nested recipe.** Whether the boundedness rule is adopted at all is Sean's and is now `C-75`, open to spec.

### X-10 - the menu leaks, so it is computed from what the player knows and not from the state

**to** code · **status** **withdrawn** 2026-09-08 — its main claim was wrong and Sean refuted it; **`X-8` stands as filed** · **raised** 2026-09-08 · **source** [report](2026-09-08-the-menu-leaks.md), and Sean stating that fog of war features heavily

**This corrects `X-8`, which is still open to you**, so read this before building on it. `X-8` said
the interface's menu is the set of actions applicable in the current state. **With heavy fog of war
that is not imprecise, it is forbidden.**

**What.** A player's indistinguishable states form an **information set**, and there is a hard
requirement on it: **every state in one information set must offer the same available actions.** If
two states look identical and offer different menus, **the menu discloses which one they are in.**
GDL-II builds the same condition in - two histories are indistinguishable when the player saw the
same things **and its own available actions were the same**.

**Why it costs something, concretely.** If *deploy ark here* appears only where the territory has no
garrison, **the presence of the entry tells the player there is no garrison**, which is the fact the
fog exists to hide. So a precondition over hidden state **may not gate visibility**. Either the
condition moves onto the effect, or the action is offered and fails - and **then the rejection is an
information channel**, which makes `Rejection` a game mechanic rather than an error report. That is a
design decision and this lane has not taken it.

**It makes `create-if-missing` compelled rather than preferred.** `X-8` treated
precondition-versus-guard as a choice about what the player sees. Under fog it stops being a choice
for any condition over hidden state. **Three lines now agree on Sean's own sketch** - interface
behaviour, `X-9`'s bounded-zero-test split, and this. **Recorded with a caveat**: the second and
third share a premise about what a garrison is, so three agreeing arguments are weaker evidence than
they feel.

**Fog itself is cheap to represent.** GDL-II is base GDL plus a `sees(role, fact)` predicate and a
`random` role, and that suffices for arbitrary finite n-player games with randomness and incomplete
knowledge. **The expensive half is reasoning about what others know**, which is opponent AI and is
deferrable knowingly.

**His seeded-PRNG instinct is right and better than GDL-II's**, for a reason already in this
repository. GDL-II makes nature a player, which is an **extra input**, so the data dump is derivable
only if you are also told what nature did. **A seed in the state keeps the transformation
`(state, commands) -> state`**, which `docs/process.md` requires and which is what makes the dump
derivable by hand. Two things that bite later, both inference rather than citation: **the seed is
hidden state**, or draws are predictable; and **a single stream leaks across subsystems**, since a
player learns that something unseen consumed randomness by watching their own next draw move -
remedied by per-subsystem streams from one master seed.

**Whether.** Worth reading before the prototype's interface takes shape, and **the first paragraph is
worth reading before you act on `X-8` at all.** Nothing to build yet: the requirement is a constraint
the design must meet, not a design, and the two ways of meeting it are different games. **No text
drafted and no decision taken.**

**Withdrawn the same day, by Sean, and `X-8` needs no correction after all.** He asked what any of this has to do with determinism, computability or distinguishable states, and observed that games forbid actions under fog all the time. **The condition is definitional**: *all nodes in an information set have the same available actions* is a well-formedness rule about **how the model is drawn** - if the actions differed the player could tell the nodes apart, so they were never one information set. **It restricts no game.** A game that greys out an action under fog simply has finer information sets, and the menu is one of the things the player observes.

**So the residue is an accounting identity and not a constraint**: if the menu differs, the player can distinguish. Worth a sentence of awareness when deciding between greying an action out and omitting it, since those disclose different amounts - and that is ordinary design judgement, not a theorem. **Nothing here should change what you build**, and the correction previously written onto `X-8` is itself withdrawn.

**How it happened, since this lane spent the week cataloguing exactly this.** The citation was checked and is accurate. **The check returned a true answer and the truth of it stopped the next question** - whether the condition bound games or bound the drawing. That is `C-65`'s shape, filed by the code lane against itself two days ago, reproduced here by the lane that wrote it up.

### X-17 - what check 2 should become: subordinate to the declarations, not to a baseline

**to** code · **status** **acted** 2026-09-09 · `33d991c` — recommended, then implemented in `tools/research/formulas/check.py`, which is this lane's own tooling, so nothing was ever left for the code lane to build · **raised** 2026-09-09 · **source** Sean, asking what this lane recommends for check 2

**Recommended: drop the baseline of intended loops and use check 1's declarations instead.** The
list already exists in the *Kinds* table - energy, food, citizens and labor may grow, metal may
not, and extraction is the one declared source. Then check 2 asks a single answerable question:
**is there a loop that gains metal without going through `work`?** The false positive that made a
baseline look necessary disappears on its own, because a labor loop is declared and simply is not
reported. **One declaration serves both checks and there is nothing to keep in sync.**

**Three defects in check 2 that Sean's objection surfaced, two fixed and one not.**

- **Only entry points are transitions.** `found-colony` was counted as firable on its own, so the
  check reported a metal source **with no ark and no pioneer spent** - a loop no player can reach.
  Fixed.
- **State-dependent amounts get a bound**, production taking its maximum and consumption its
  minimum, so a loop that exists is never missed. Three of the four are in; `perish` stays out
  because its effect depends on *which* thing, and a representative would be choosing the answer.
  Fixed, and each bound carries its justification.
- **A family hides a kind, and this one is open.** `resource[...]` collapses to `resource`, which
  has no metal weight, so **working a metal extractor scored zero** - the check was blind to the
  game's only metal source. The checker now says so instead of scoring it silently, which is the
  least it can do; resolving a family to its kinds is the fix and is not written.

**Whether.** **Worth doing if the recipe work continues, and worth nothing otherwise.** The first
two are done and in `tools/research/formulas/check.py`, which is this lane's tooling - **if any of
it graduates to production it is yours to own and rewrite, not to inherit.** The family fix is the
one that matters, because a check that cannot see the subject it is about is the failure this
repository has now recorded four times.

**Done 2026-09-09, on Sean's instruction, and the third defect is closed.** A target naming a
family now stands for one transition per kind in it, so `work` is three - `work[food]`,
`work[metal]`, `work[energy]` - each carrying its own largest density. **`work[metal]` is in check
2's witness now**, which is the whole point: until this, the game's only metal source scored zero.

**And it is grounding**, the same operation an interface performs to build a menu from a recipe
and a state. **The analysis and the interface want the same machinery**, which is an argument for
building it once rather than twice, and this lane did not expect that when it filed the item.

**The recommendation is now implemented rather than described.** `check_unbounded` runs twice:
once whole, and once with the declared sources removed. **The second run is the one worth
reading** - it asks whether metal can grow without mining, and needs no baseline of intended
loops, because a labor loop is declared free and simply is not reported. Result: **no metal source
other than extraction.**

**A fourth defect was found while doing it, and it was in check 1 rather than check 2.** Check 1
was still weighing `found-colony` as if a player could fire it alone - the same fault fixed in
check 2 two hours earlier and not carried across. **One fix, applied to one of the two places that
needed it**, which is worth recording because nothing would have caught it: the check was red for
a real reason and stayed red for a wrong one.

**Closed 2026-09-09.** The recommendation was implemented the same day, and everything it names lives in `tools/research/formulas/`, which is this lane's own column. **It was addressed to `code` when nothing in it was ever the code lane's work** - a wrong address costs a producer a read and gives the index a task that does not exist. What stands, and needs no item: **if any of this tooling graduates to production it is the code lane's to own and rewrite, not to inherit.**

### X-16 - the recipe files are not reachable from the deployment, and `reports/` is not this lane's

**to** code · **status** **acted** 2026-09-09 · `153a13f` — three files by name rather than `lenses/` whole, linked at their repository paths under a *Research* heading that says they are not the game's state · **raised** 2026-09-08 · **source** Sean, asking for the recipe files to be browsable from the deployment

**What.** `reports/index.html` is the browsable deployment and is generated by
`crates/game-console` - **your column, not this lane's**, so this is filed rather than wired.
Sean asked for `lenses/research/formulas.html` and `tools/research/formulas/data.json` to be
reachable from it, and *possibly others if useful*.

**What is worth linking**, most useful first: `lenses/research/formulas.html`, the generated
report; `tools/research/formulas/data.json`, the only file that is edited; and
`tools/research/formulas/results.json`, what the three checks last reported. The renderer and the
checker are code and probably not worth a link.

**One thing to weigh rather than a request.** `reports/` is generated **from the game state** by the
console, and these files are a research lens's, generated from its own data. **A link out of a
state report into a lens's working file may be the wrong shape** - a *Research* section that is
plainly not part of the state would say what it is. `R-9` requires every reference to be a link a
person can follow and no page to need a script, and these pages satisfy both.

**Whether.** Worth doing when convenient and **nothing waits on it** - Sean can already read the
report as a published artifact, so this buys reachability from one place rather than access.
**Declining is reasonable** if mixing a lens's output into the state reports muddies what
`reports/` is for; say so and this lane will keep it out.

**Sean asked again 2026-09-09 - *rendered somewhere reasonable upon deploy to github* - so it is
no longer "when convenient".** What he wants is unambiguous now, and two facts make it smaller than
the item above suggests.

- **Neither route works today.** GitHub serves a committed `.html` as source rather than rendering
  it, so browsing the repository will never show the page. And the Pages artifact is
  `crates/game4x/dist` plus `reports/` and `scenario/` - `lenses/` is not copied, so it is not on
  the site either. **The `.md` reports in this directory do render on github.com**; only the
  generated one does not.
- **The mechanism is one line of the workflow**, beside the two `cp`s already in *Copy the reports
  into the artifact*: copy `lenses/research` into `dist` and it is served at
  `<pages>/lenses/research/formulas.html`.

**Two things checked so the risk is known rather than assumed.** `lenses/research/formulas.html`
contains **no `href`, no `src`, no `url(` and no `<script>`** - it is one self-contained file of
about 93 KB, so it cannot 404, needs no sibling copied beside it, and cannot add a
reference that `tests/browsable.rs` would count as pointing outward.

**The shape question in the paragraph above still stands and is yours.** Copying `lenses/` into the
artifact publishes a lens's working directory alongside the game, which may be the wrong thing to
put on the site even though the one file is harmless. **An alternative that keeps `reports/` for
state**: publish it at a path of your choosing and link it from `reports/index.html` under a heading
that says it is research rather than state. This lane has no preference between them and cannot
write either.

**Acted 2026-09-09, and the shape chosen was the second one offered.** Not `lenses/` as a directory - that would publish this lane's outbox and dated reports beside the game - but `formulas.html`, `data.json` and `results.json` by name, linked at the paths they live at so one href means the same thing in a clone and on the site. **The code lane also found its own check was wrong**: it asked whether the workflow *mentioned* each path, which stayed green with the `cp` line deleted, because the same path appears in the `test -f` beside it. It now reads the destination of every `cp`. That is the same failure this repository has recorded five times now - a right answer to a narrower question than the one asked.
