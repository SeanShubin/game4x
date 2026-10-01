# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-604 - A rule already reaches only what it declares, so the question is what it may declare

**to** sean · **status** open · **raised** 2026-10-01 · **revised** 2026-10-01, after he said the single function is logical rather than architectural · **asks** a decision · **kind** recovered · **into** `spec/invariants.md` -> The game is one function

**Sean**: *conceptually the game is a single function, but that does not have to mean there is
literally a single function that has access to all state. In object oriented languages I break up the
state all over the place in different implementations behind interfaces, and implementations have
access to only the interfaces relevant to them. That is what I am trying to capture in the code
architecture. It is logically (old-state, command) -> (new-state), but that may be represented without
coupling everything to a global state.*

**That property is already true of a rule, and this proposal's first version missed it.** Measured in
`spec/data/rules.4x`:

```
clauses that declare their own relation     55 of 55
a `{reading}` names                         another clause of the same rule, never a relation
a `{relation-of}` takes the relation from   an input, so the caller picks within a family
anything that reaches unnamed state         none
```

**So a rule is handed its clauses' relations and nothing else.** There is no wildcard and nothing to
read all of state with - which is the OO property he describes, arrived at by the data model rather
than by interfaces.

## So the question is narrower and the earlier framing was wrong

**It is not *what may a rule read*. It is *what may a rule declare*.** Declaring a clause is free
today: nothing stops a rule of the game naming a relation that belongs to the interface, and the
moment such relations exist, one could.

**And the mechanism exists in miniature.** `unit` is a family and `{relation-of clause:clause-3
input:what}` lets `move` serve every member of it. **So *a rule over a group of relations* is already
expressible**, and what is missing is a group that bounds rather than a group that parameterises.

## The choice

**How relations are grouped, and whether a rule's group bounds what it may declare.** A family groups
relations that behave alike so one rule can serve them; this would group relations that belong
together so a rule *cannot* leave them - the same word for the opposite purpose, which is worth
deciding deliberately rather than by reusing `family`.

**Nothing is offered**, because the sentence depends on whether the grouping is a new relation, an
existing one reused, or a property of the rule rather than of the relations.

## What this drops from the first version

**The three readings about reads and writes.** They were asking how to police a crossing; **his
answer is that a rule should never be given the chance to make one.** A prohibition and an absent
capability look alike when they hold and differ entirely when something is added - which is `S-227`'s
lesson about `shell::` said about data instead of code.
