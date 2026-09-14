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

### P-492 - The tank is already there; what `refuel` cannot say is which bin it means

**to** sean · **status** open · **raised** 2026-09-13 · **rewritten** twice on 2026-09-13, on your answer and then on your metal transport · **kind** recovered · **shape** rows · **asks** a decision · **into** `releases/first-release.md` -> Recipes

**Yes, it informs it, and it decides two things.** It also corrected this lane's last answer, which
called a tank *this game's first part*. It would not have been.

## What the release already says, which the previous version of this item missed

`releases/first-release.md` -> *Where things are* lists three containers, and the third is yours:

| Container     | Holds                         | Up to                           |
| ------------- | ----------------------------- | ------------------------------- |
| a territory   | that kind                     | its free capacity for that kind |
| a store       | the resource it was built for | 10                              |
| a unit's tank | energy                        | the unit's fuel                 |

And `fuel` is defined in the Traits table as **how much energy its tank holds**.

**So *the pioneer contains a fuel tank that can contain fuel* is not a change - it is what the
release says today.** `D2`, making `tank` a kind, is withdrawn: it would replace a container that
works with a kind that says the same thing, and a fact is stated once.

## What your transport decides, and this is the part no file could have told this lane

**A pioneer holds one kind, so `free at least 1` is unambiguous. A metal transport holds two, so it
is not.** *Up to 2 fuel and 10 metal* is two bins in one thing, and a qualifier that says `free`
without saying free **of what** stops meaning anything the day that unit exists.

**This lane was about to write `free at least 1`.** It reads correctly against every unit in this
release and would have had to be rewritten for the first unit that carries cargo - a right answer
about the wrong population.

## So the remaining question is the qualifier's form, and here are three

| Form                     | `refuel`'s Traits cell reads | A transport's cargo check would read |
| ------------------------ | ---------------------------- | ------------------------------------ |
| **`E1`** trait then kind | `free energy at least 1`     | `free metal at least 1`              |
| **`E2`** kind then trait | `energy free at least 1`     | `metal free at least 1`              |
| **`E3`** prose, as now   | `with room for 1 energy`     | `with room for 1 metal`              |

**`E1` matches the shape most of the table already uses, and the count this lane first gave was
wrong.** It said *all 27 distinct forms begin with the trait*. Re-derived: **26 distinct qualifiers,
of which 18 parse today as `<trait> <phrase>`** - `moving at least 1`, `defending at its maximum` -
**two more are counters of another shape** - `keeps 0`, `met at least 0` - and **six are neither**:
`food`, `metal`, `` `$resource` ``, and three written as prose, one of which is the `with room for
energy` this item exists to replace. The first count included the table's `---` separator as a
qualifier, which is this repository's own recurring failure committed inside the item describing it.

**This lane would pick `E1`**, and the reason is the 18 rather than taste.

## How each survives the Petri net constraint, measured in `crates/game-console`

**All three fail the parser identically today, and that answer is useless.** `nogain::count_in`
splits on the first space and matches three fixed phrases, so `free energy at least 1`, `energy free
at least 1` and `with room for 1 energy` all return `None` - and so does the `with room for energy`
already in the release.

**It does not currently matter, because the net never reads a qualifier on a `require` row.**
`petri.rs` builds a non-`put` arc against `Place { container, kind, room: false }`; the qualifier
rides along in the arc's `traits` field and selects nothing. **`refuel` is drawn as *requires a
unit*, full stop.** That is conservative rather than wrong - an ignored requirement over-approximates
what can fire, so the no-gain result still holds - but **the bin is not in the net.**

**So the question that discriminates is which form can become a place**, and there the three differ:

|          | Becomes                      | Costs                                                             |
| -------- | ---------------------------- | ----------------------------------------------------------------- |
| **`E1`** | the place `unit free energy` | `count_in` matches the trailing phrase rather than the first word |
| **`E2`** | the place `unit energy free` | the same one change                                               |
| **`E3`** | nothing                      | a fourth form in a three-form vocabulary, and it is prose again   |

**And whichever is chosen, the constraint is met for the same reason `met` meets it.** Free capacity
is held as positive tokens on a **bounded** place - a pioneer's tank is 2, a transport's would be 2
and 10 - so `free energy at least 1` is an ordinary read arc and **not an inhibitor**.
`spec/invariants.md` licenses exactly this: *a rule may ask whether something is absent only where
what would hold it declares a limit for it.* A unit's tank declares one. The same question asked of
something unbounded is the cliff, and **reachability survives one inhibitor arc and dies at two.**

**`E1` and `E2` tie on the net** - same change, same arc, one place per kind either way.

## What is not being asked

**Nothing about the transport itself.** It is not in this release, and a release never specifies
what it is not building. What it does is settle the notation now, so the row that adds it later adds
a row and not a rule.
