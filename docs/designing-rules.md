# Designing rules

What the recipe net buys, what it costs, and how to tell before you write a rule whether it is
expressible.

[Documentation map](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

The recipes are a **coloured Petri net**, and the difference from a plain one is load-bearing.

**A place holds a count, a recipe is a transition, and a row is an arc carrying a constant weight.**
That is not an analogy - it is the mapping `crates/game-console/src/petri.rs` builds and
`reports/nogain.md` computes over.

## The net is a means, and this is the end it is a means to

**Sean, 2026-09-11**: *They are a means to an end. I want it to be safe for a user to edit recipes
without having to worry about infinite resources on one turn.*

**Everything below is that sentence, worked out.** The formalism is not here because a net is
elegant; it is here because it is the thing that makes *safe to edit* checkable instead of hoped
for. **The person being protected is someone editing a recipe**, and the failure being prevented is
a rule that hands them more than it took.

**Which is why the refusals are refusals and not advice.** A construct that costs decidability does
not make the game wrong - it makes the editor unable to tell the player their rule is wrong. `X-9`
states the trade in one line: with a plain net, *an unintended infinite-resource loop is computable
rather than something you playtest for.*

**And it is why the scope matters.** The guarantee is wanted for the sublanguage **a player writes
in**. The world's rules are the game's own and nobody edits them, so they may be stronger - and they
are. That split is real, unstated, and the subject of this document's open question.


## Two nets, and the guarantees belong to the second

**What we write is coloured.** A rule whose subject is a **family** is one rule standing for several
- `discard` over the resources, `refresh` over the things that act. An amount **read from a trait**
is the same move: `work` makes a territory's density, which is one rule with a number per case.
**The family and the trait are the colours.**

**What the guarantees are about is the plain net you get by unfolding it.** `reports/nogain.md` says
so in its own words: *50 rules, ground from 31 blocks of recipe rows - a family becomes its members,
a density becomes its cases.* **Boundedness and termination are decidable for the unfolded net**,
and that is the net the weighting is solved over.


**So every colour set must be finite, and that is a real constraint rather than a formality.** A
family has a listed membership; a trait read as an amount has a listed set of values. **A rule
parameterised over something unbounded has no finite unfolding, and every guarantee in this document
is about a net that would not exist.**

**There is more than one legitimate unfolding.** The check grounds to places at `(kind, state)`; the
drawing grounds to `(container, kind)`. Both are correct readings of the same coloured net at
different granularities, which is why one can see a thing the other cannot - `work` nets to zero in
the drawing and not in the check.

## `put`, and why identity costs the net nothing

**A `put` row is the one that does not carry a weight**, and it is the clearest case of the colour
doing work. **Sean, 2026-09-11**: *put was there so we could move things with an identity without
destroying, then creating them.*

**In a plain net a token is fungible**, so moving a unit is `consume 1 unit` here and `produce 1
unit` there. That is arithmetically right and it is a lie about the thing: the unit that arrives is
not the unit that left, and any id, tank or spent count it carried is gone. **In a coloured net the
token carries a colour, and an arc that preserves the colour is an ordinary arc.** So `put` is not
an extension of the formalism - it is the coloured version of what `consume`/`produce` was
approximating badly, and it buys identity **at no cost to any guarantee in this document**.

**The twelve rows split six and six, and only one half is interesting.**

- **Six write *one less*** - `move`, `create labor`, `work`, `bear`, `muster`, `stand`. Each is
  preceded by a `require ... at least 1`, so the value is known before it is written. **An ordinary
  decrement arc**, and four of the six are player recipes
- **Six write *at its maximum*** - all of them `refresh`, which is a **world** recipe. **The test is
  what the amount is, not what the row looks like**: *at its maximum* is **how much is missing**, so
  the amount depends on the current marking. *One less* is a fixed decrement and reads nothing.
  **This is the reset arc, and it is confined to the one rule that restores what time gives back**

**Two rows look like marking reads and are not**, which is the same distinction from the other
side. `work` produces *`$where`'s density for that resource* and `muster` produces *that citizen's
force*. **A density is a trait of the place and a force is a trait of the kind - both constant, and
both colours.** What makes an amount illegal is depending on *how much is there*, not on being
written as a phrase instead of a number.


**That confinement is the whole reason `put` is free.** `spec/invariants.md` already names time as
one of three sources - *anything that exhausts draws on time for a turn: it spends a count it
carries, and only the turn's end restores that count.* **`refresh` is that sentence written as
rows**, and it is the only rule in the game that writes a count upward from nothing.

**Counted from the Qty and Traits columns of the *Recipes* table, over its 77 role cells**, so it
can be re-run: **64 cells carry a quantity and 13 are blank, and the thirteen blanks are exactly the
thirteen `put` rows.** Of the 64, three read a trait - *that citizen's strength*, *that unit's
strength*, *`$where`'s density for that resource* - which is a colour rather than a marking read.

**Corrected 2026-09-12, and both numbers went stale from promotions this lane made.** It read 81
cells and twelve `put` rows. **`P-427` took four `store` rows out of founding**, so 81 became 77;
**`P-431` made `age`'s second row a `put`**, so twelve became thirteen. **Nothing edited this
paragraph and nothing had to** - which is `C-9`'s shape, and it was found by re-deriving the figures
rather than by reading them.



**What breaks it is an arc the unfolding cannot flatten**: one that tests a place is **empty** - an
inhibitor arc - or one whose weight **reads the marking** rather than being a constant, which is a
reset or transfer arc. **Every such arc in this game is in a world recipe and none is in a
player's**, which is the property the open question at the bottom is about.

**The claim has been measured twice, and the two measurements are not of the same thing.** `X-29`
counted with tooling on 2026-09-10 and found ten, in `grow`, `refuel` and *end-of-turn losses*.
Counting the release by hand gives **six**, all of them `refresh`'s *at its maximum* rows.

**That is not a stale number and calling it one was this document's first mistake about it.** The
research lens re-ran the check on 2026-09-11 and it still says ten - because it reads
`tools/research/formulas/data.json`, **that lane's own re-encoding**, whose recipe names are `grow`,
`refuel` and *end-of-turn losses*. **The instrument answers a narrower question than the one asked
and returns a plausible number rather than an error**, which is the failure `CLAUDE.md` names. Six
and ten never disagreed; they are two populations, and only the six is about the release.

**Re-measured on 2026-09-11, against the release, and the six holds.** The research lens's check 17
parses the *Recipes* table rather than any copy of it: **81 arcs over 21 recipes - 61 ordinary, 14
thresholds, and 6 reading a marking**, all six `put ... at its maximum`, all six in `refresh`, all
world. **Three methods now agree** - two hand counts and one computed, by different routes.

**The classification was settled the opposite way from this document's first attempt at it**, and
the correction is worth carrying because the wrong version is the intuitive one. *At its maximum*
does **not** write a constant: reaching a maximum has to move **the difference between the maximum
and what is there**, so the weight depends on the marking. *One less* reads nothing. **The rule of
thumb is the amount, never the phrasing** - which is the same test stated above, arrived at twice
from opposite directions.




**This document exists so the constraints are known in advance.** The design space is wide, and
almost all of it is reachable; what is not reachable is small, specific, and worth recognising on
sight rather than discovering after a rule is written.

## What the net buys

**The no-gain invariant is decided rather than believed.** `spec/invariants.md` says *there is a
weighting of the kinds, and under it no sequence of rules ends holding more than it began with*, and
that **whether this holds is decided mechanically, from the rules alone.** That is a **place
invariant**: a weighting `w` where `w · (made - taken) <= 0` at every rule. `reports/nogain.md`
solves for one from the *Recipes* table and publishes the weighting it found, so a number that looks
wrong is a fact about the rules rather than about anybody's judgement of them.

**It says which rule shapes are legal before one is written.** The table below is the whole of it.
A shape that is not in it is not a restriction discovered late; it is a shape the formalism never
had.

**The rules stay data, because a net is rules-as-data by construction.** Anything expressible as a
net is expressible as rows of a table, which is what makes a rule something a player can edit rather
than something the program knows.

**A bad player-written rule becomes refusable at edit time, with a reason.** For the unfolded net,
boundedness and termination are decidable - so an editor can say *this rule never finishes* rather
than letting a game discover it. **A Turing-complete rule language cannot have this feature at all**,
which is why the constructs below are refused rather than merely discouraged.

**It is a second reading of the same table.** The check derives its places at `(kind, state)`; the
drawing derives its own at `(container, kind)`. Two derivations from one source, so a disagreement
between them is visible - which is how `work` was found netting to zero in the drawing.

## What it costs

Every one of those guarantees is bought with the same currency: **no rule may ask how much of
something is present, and no rule may test that something is absent.** Everything else refused
below is one of those two wearing different clothes. **The constraints are not a tax on the design.
They are the guarantee, stated in advance.**

## Deciding whether a formula is allowed

Ask two questions in order. **If both answer no, the formula is expressible.**

**1. Does the amount depend on how much is present?** `spec/invariants.md`: *a rule's amounts are
constants*, and *what one firing takes and makes does not depend on how much of anything is
present*.

**This is usually a yes that becomes a no**, because the same document gives the rewrite: *where a
rule would need a quantity that varies, it is written as a smaller rule that fires as many times as
it can*, and *the quantity is then how often it fired*.

**An amount read from a trait is not an amount that depends on what is present.** `work` produces a
territory's density; that is one rule with a number per case, and it is allowed.

**2. Does the rule need to know something is absent?** A zero test is an **inhibitor arc**, and with
one the net is Turing-complete: reachability stops being decidable and so does everything this
document promises.

**But the line is boundedness, not the zero test**, and this is the correction worth reading twice
because the obvious version of it is wrong. **`X-9`'s first draft said moving the test from a
precondition to a guard on an effect saves it. It does not** - a zero-guarded effect can record the
test's result elsewhere and the branch returns. What actually separates the safe case from the fatal
one is **what is being tested**:

- **A place bounded by a stated capacity can be zero-tested for free.** The standard
  complementary-place construction turns *is it empty* into *is the room full*, with no inhibitor
  arc and nothing lost. `limit 0 garrison` was always safe, because a garrison's capacity is 1
- **An unbounded place cannot.** *If there is no food*, *if the store is empty* - these are the
  cliff, and they are exactly the tests a resource game invites

**`C-75` measured the split over the eleven bound kinds**, and it is the same division `C-74` found
from the other side - a thing that is at most one against a thing that is counted:

| Zero test | Kinds                                      |
| --------- | ------------------------------------------ |
| **free**  | garrison, extractor, yard, ark, pioneer    |
| **fatal** | citizen, store, labor, food, metal, energy |

**No rule says this and no check enforces it.** The release has zero `limit` rows today - `P-385`
deleted both `limit 0 garrison` rows - so adopting the constraint is **vacuous now and is not
vacuous later**. Whether to adopt it is `P-423`, open to Sean.


**A comparison is an instance of the second, which is why there are two questions and not three.**
*The largest of them* is *this one, and nothing is greater* - and **nothing is greater** is a test
that a set of places is empty. **Max is not refused for being a comparison; it is refused for being
a zero test wearing a comparison's clothes.**

**Comparing two things you have named is a different act and is allowed.** `spec/logistics.md`:
*there is never a quantity of a thing with an `id`* - a named thing is one thing, and `P-396` lets a
rule name one and say what changes about it. **What cannot be done is ranking the contents of a
place**, because the contents are a count and finding the greatest means proving the rest are not.

## The folds, and what to do with each

| Fold                    | Allowed        | How it is written                                                                    |
| ----------------------- | -------------- | ------------------------------------------------------------------------------------ |
| **sum**                 | **free**       | it is the marking of a place. You never compute it                                   |
| **min**                 | **yes**        | a pairing - one rule that spends one of each, fired as many times as it can          |
| **at least n**          | **yes**        | an input arc of weight `n` - a `require` or a `consume` of `n`                       |
| **at most n**           | **yes**        | a capacity. `P-374`: what is stored is the room left, so *at most* is *room remains* |
| **a number per kind**   | **yes**        | an amount read from a trait - `P-376`                                                |
| **max**                 | **no**         | *nothing is greater* is a zero test. Over things you have named it is allowed        |
| **a branch on absence** | **it depends** | an inhibitor arc on an unbounded place. On a capacity-bounded one it is free         |


### `min` is a pairing, and the game already contains one

| Recipe    | Auto  | Role    | Qty | Kind      |
| --------- | ----- | ------- | --- | --------- |
| **breed** | world | consume | 1   | fertility |
|           |       | consume | 1   | food      |
|           |       | produce | 1   | citizen   |

It fires as many times as it can, which is **min(fertility, food)**. `P-379` replaced the old `grow`
- which read *the lesser of the surplus food and the citizens here* and was forbidden - with this.
**The min survived; the reading of it did not.**

### A worked example: force is `2 + min(guns, citizens)`

**Every kind here is invented for the example.** A `keep` is not a garrison and a `gun` is not in
this game - the point is the shape of the arithmetic, and borrowing a real name would make a reader
check it against the release and find a rule that is not there.

| Recipe   | Auto  | Role    | Qty | Kind    | Traits             |
| -------- | ----- | ------- | --- | ------- | ------------------ |
| **hold** | world | require | 1   | keep    | holding at least 1 |
|          |       | put     |     | keep    | holding one less   |
|          |       | produce | 2   | force   |                    |
| **arm**  | world | require | 1   | citizen | arming at least 1  |
|          |       | put     |     | citizen | arming one less    |
|          |       | require | 1   | gun     | arming at least 1  |
|          |       | put     |     | gun     | arming one less    |
|          |       | produce | 1   | force   |                    |

`hold` fires once per keep, making **2**. `arm` fires once per pair it can make, making
**min(guns, citizens)**. Every arc carries a constant weight. **The formula is expressible exactly as
written, and nothing about it strains the net.**

**What the real game does is different and is not this example.** A garrison has **no strength of
its own**; it is what lets the citizens of its territory muster theirs. The numbers here are chosen
to show a `min`, not to describe a rule.


### A worked counter-example, and the game has already left it behind

**The shape that cannot be had is a rule that picks between two folds.** Suppose force were *the sum
of the citizens' where a garrison stands, and the highest among them where none does.*

**Sum is free** - it is what a marking is. **And max here is degenerate**: every citizen has
strength 1, so *the highest among them* is *1 if any citizen is present*, which is an ordinary
threshold. **Both halves are individually legal.**

**What is not legal is choosing between them.** Two behaviours selected by whether a garrison exists
means two rules: one requiring a garrison, and one requiring that there is **none**. The second is a
zero test. **The fold was never the problem; the branch was.**

## And this is no longer what the game does, which is the part to read

**`P-416` took the *highest* case out of the game entirely**, so there is no second fold to choose
between. What is left is one rule: **`muster` requires a garrison**, fires once per citizen, and
produces that citizen's strength. **A territory with no garrison musters nothing from its citizens,
and no rule has to ask whether a garrison is absent to arrive there** - the rule simply does not
fire.

**So the garrison's behaviour is data and always could have been.** A `require` row is a test for
**presence**, which is an ordinary arc; it is `limit 0`, a test for **absence**, that costs
decidability. **The condition was never the dangerous shape - the fold was**, and the fold is gone.

**This is written down because the question keeps coming back.** Sean raised it on 2026-09-11 -
*garrison has an effect that allows citizen force to be summed instead of using the max; this is an
algorithm, not data* - and that was true of the design as it then stood. **It stopped being true the
same day**, by a change aimed at something else, and this document went on holding the garrison up
as the example of the illegal shape. **A counter-example naming a live part of the game reads as a
verdict on it.**


## How to rewrite a formula that fails the test

**A varying amount becomes a repeated firing.** Write the rule for one, and let it fire as many
times as its inputs allow.

**A branch on absence becomes a presence.** Ask what is true when the thing is missing, and require
*that* instead. A rule that should fire only without a garrison is usually a rule that should fire
when something else is there.

**Absence is sometimes a capacity, and sometimes only looks like one.** `P-374` makes *room for a
garrison* a count, so *there is no garrison* and *there is room for one* coincide - **but only where
the capacity is exactly one.** `spec/invariants.md` states the trap: *those differ wherever a
capacity is more than one, and agree only by accident where it is one.* Use it knowing which case
you are in.

**A comparison becomes an identity or a redesign.** If the things being compared carry an `id` they
are distinguishable and a rule may name one - `P-396`. If they do not, the comparison is asking a
question the state cannot answer, and the rule wants a different shape.

## Open questions

**Nothing states that the player's sublanguage is the ordinary one.** Every marking-reading arc in
the game is in a **world** recipe and none is in a player's, so the property holds **by accident**.
Until a rule says so and a check enforces it, the first player-authored recipe that empties a place
moves the editor into a class where none of this is decidable. **`X-29` is open with its evidence
re-derived from the release**, and the property now has three independent measurements rather than
a claim. **What is still missing is the rule and the check** - nothing states that a player's
recipes must stay ordinary, and nothing refuses one that does not.



**Nothing says which kinds may be zero-tested.** The boundedness split above is a finding and not a
rule: `C-75` measured that adopting it would cost nothing, and `P-385` has since removed the only
two rows it would have governed, so it is now free and proves nothing. **`P-423` asks whether to
adopt it**, and the three ways out are stated there.

**Nothing says which of two idioms a rule should use for *the same thing, changed*.** `work` writes
`require` then `put`; `age` writes `consume` then `produce`, for the same operation. Both are legal
and they differ only in whether identity survives - which is invisible while nothing that ages has
an id. **`P-424` asks it.**

**The worked example and the counter-example were both rewritten on 2026-09-12**, and the reason is
worth keeping. The example borrowed `stand` and `garrison` from the release and had a garrison
producing 2 force, which `spec/control.md` flatly denies - *it has no strength of its own*. The
counter-example held the garrison up as *the shape that cannot be had*, which stopped being true
when `P-416` removed the *highest* case. **Both now invent their kinds**, and the counter-example
says what the game does instead. **A document about what cannot be expressed must not use a live
rule as its example of the forbidden**, because a reader cannot tell the illustration from the
verdict - and Sean twice asked whether the garrison was an algorithm, reading this.


