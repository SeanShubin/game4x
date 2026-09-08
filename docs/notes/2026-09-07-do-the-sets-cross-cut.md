# Do the sets cross-cut? Measured on the release as it stands

**Derived.** Written by the specification lane, 2026-09-07, answering Sean's question: *based on what
we know about the game so far, do we have enough information to make an informed decision regarding
how we should model the data?*

[Notes index](README.md) · [Decisions](decisions.md) · [The release](../../releases/first-release.md)

## The question, made decidable

Different transformations apply to different sets of entities, and two such sets may overlap without
either containing the other. **That is the property that decides the representation**: if every pair
of sets is disjoint or nested, a single hierarchy holds them all and the candidate paradigms are
indistinguishable on this data. **One crossing pair is a counterexample a hierarchy cannot hold.**

So the question *do we have enough information* becomes: **does the release contain a crossing
pair?** That is countable rather than arguable.

## How this was measured

`releases/first-release.md` -> *Units and structures* is a matrix: one row per kind, one column per
capability, a non-blank cell meaning the kind has it. **Membership is read from which cells are
filled**, over the nine columns after *Thing*. Re-runnable from the table; no judgement in the
extraction.

**Two instruments were wrong first and are named so the number is not trusted twice.** The first
expanded traits whose carrier is self-referential - *a thing with upkeep* - into every kind, so
everything nested and it reported **0 crossings**. The second dropped every continuation row of the
*Recipes* table, so each recipe contributed only its first ingredient. Both returned plausible
numbers about the wrong question, which is `C-28`'s shape twice in one measurement.

## The result

Nine capability columns, **six distinct sets**, 36 pairs, **five of them cross-cutting**.

| Capability           | Kinds that have it                             |
| -------------------- | ---------------------------------------------- |
| **Costs to produce** | ark, extractor, garrison, pioneer, store, yard |
| **Binding**          | ark, extractor, garrison, pioneer, store, yard |
| **Force**            | ark, citizen, garrison, pioneer                |
| **Readies**          | ark, citizen, extractor, pioneer               |
| **Fuel**             | ark, pioneer                                   |
| **A move**           | ark, pioneer                                   |
| **Crosses**          | ark, pioneer                                   |
| **Upkeep**           | citizen                                        |
| **Requires**         | ark                                            |

**The clean counterexample is *Force* against *Readies*.** They share ark, citizen and pioneer;
**garrison has force and does not ready**; **extractor readies and has no force.** Neither set
contains the other.

**And it is not one accident.** Take *is built* - the set both *Binding* and *Costs to produce* name
- against those two, and three kinds each carry a different pair:

- **citizen** has force and readies, and is **not built**
- **garrison** is built and has force, and **does not ready**
- **extractor** is built and readies, and has **no force**
- **ark and pioneer** have all three

**That is a lattice and not a tree.** No single-inheritance hierarchy places citizen, garrison and
extractor consistently, because each is missing a different one of the three.

## What this does and does not decide

**It eliminates one family of answers on evidence already in the tree.** A hierarchy of kinds - the
object-oriented answer - is refuted by the release as it stands, not by a future the game might have.

**It does not choose among what survives.** Structural typing, a trait a thing carries, and a
parameter over a set of kinds all express a lattice. **Nothing measured here distinguishes them**,
so the data is sufficient to rule out and insufficient to select.

**And the surviving answer is already in use, which is the part worth noticing.** *Units and
structures* **is** a capability matrix: a column per capability, a blank meaning *does not have it*.
The release has been representing cross-cutting sets as data since it was written. So the open
question is narrower than the paradigms make it sound - **whether a capability that already exists
as a column should also be a named trait**, rather than which paradigm to adopt.

## Two things the measurement found that nobody was looking for

**Three columns name one set.** *Fuel*, *A move* and *Crosses* all hold exactly `ark, pioneer`.
`P-346` deletes *A move*, taking it to two. **This is the duplication Sean said he wants to refactor
as aggressively as in a programming language**, and it is measurable rather than anticipated.

**Two more name another.** *Binding* and *Costs to produce* hold the same six kinds - *what is
built* - under two names.

**So of nine columns, six sets.** Whatever the representation ends up being, **it has three existing
duplications to justify itself against**, which is what the earlier note asked for when it said an
abstraction should be invented after its instances rather than before them.

## The example that prompted the question does not decide it

Sean's own pair - *things that can move* against *things that require food* - **does not cross-cut
today.** Moving is `ark, pioneer`; upkeep is `citizen`. **Disjoint**, so that pair alone would have
justified a hierarchy. The evidence for the lattice comes from force, readying and being built
instead.
