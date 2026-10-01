# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-597 - A generated case writes one row per firing and an authored test writes a map, and nothing says which `{given}` is

**to** sean · **status** **withdrawn** 2026-09-30, by him saying it is fine and by a test already deciding it · **raised** 2026-09-30 · **asks** nothing now · **kind** recovered · **source** Sean reading `regression/scenario/01/04-toil.4x`

**The same notation means two things and `spec/` does not say so.** Found by him asking why a
generated case writes two rows of one rather than one row of two.

```
regression/scenario/01/04-toil.4x    {citizen where:place-1 hungry:0 bearing:1 laboring:0} -> 1
  generated                          {citizen where:place-1 hungry:0 bearing:1 laboring:0} -> 1
                                     {labor where:place-1} -> 1
                                     {labor where:place-1} -> 1

reviewed/breeding-does-not-reach-    {citizen where:place-1 hungry:0 bearing:0 laboring:1} -> 2
  the-citizens-it-just-made          {citizen where:place-1 hungry:0 bearing:1 laboring:1} -> 2
  authored                           {food where:place-1} -> 3
```

**`toil` matched two citizens, so it fired twice, and the generated case records each firing.** The
authored test records a world. **Both use `{given}` and `{then}`.**

## What `spec/` says, and what it is about

**`spec/console.md`**: *What a thing contains is a map from a description to a quantity.* And: *Each
distinct description is its own entry, and an entry is never zero.* **A map has one entry per key**,
so the generated block is not a legal state - and may not be meant as one. Its own header says
*`{given}` is what the command took and `{then}` is what it made, which is the engine's own account
of it*, which is a log rather than a world.

**So this may be no contradiction at all, and that is the problem.** Nothing in `spec/` says what a
test's `{given}` is. The authored tests answer one way and the generator answers the other, and both
are reasonable readings of a sentence that was written about the console's display.

## Measured

```
36 scenario cases use {given}/{when}/{then}   108 blocks
13 of the 36 repeat a row                      26 blocks
the commands                                   toil 8, end-turn 5 - the rules that match many rows
the other 129 generated files                  types, primitives and rules use no {given} at all
```

**That population statement is corrected from a first pass that said zero of 165.** The instrument's
lookahead stopped at the first row of a block, so it read one row per block and found no duplicate
anywhere - a plausible zero, in a measurement of whether something was systematic.

## The choice, and both answers cost something

**Coalesce**, and a generated case is a legal state: the same notation means one thing everywhere, and
a reader diffing a case against a reviewed test is comparing like with like. **The cost is that `-> 2`
hides how many times the rule fired**, which is real information in a change detector.

**Keep it per firing**, and the case says `toil` fired twice rather than that two citizens ended up
alike. **The cost is that `{given}` means two things** and a reader has to know which file they are in.

**This lane is not choosing.** A regression case exists to be read by you and deleted by you, so what
it should show is yours - and either answer wants a sentence in `spec/console.md`, which is why it is
here rather than filed to the code lane as a defect.

## Withdrawn 2026-09-30 - the test already says it, so prose would be a copy

**Sean**: *now that I understand it I think it is fine.*

**And `reviewed/toil-works-the-unworked-citizens-of-one-place.4x` already decides it**, which is the
reason this is withdrawn rather than rewritten to ask approval:

```
given   {citizen where:place-1 hungry:1 bearing:0 laboring:1} -> 2
        {citizen where:place-1 hungry:1 bearing:0 laboring:0} -> 1
        {citizen where:place-2 hungry:1 bearing:1 laboring:1} -> 1
when    {toil where:place-1}
then    {citizen where:place-1 hungry:1 bearing:0 laboring:0} -> 3
        {labor where:place-1} -> 2
```

**One command, two matches, two labor** - and the other place untouched. **So *a rule reaches every
row that matches* is asserted by a test he has read**, and `CLAUDE.md` is explicit: *a fact already
asserted by a test does not belong in prose too. The test is the stronger statement, and a prose copy
can drift from it.*

**`spec/console.md`'s *a command without one fires once* is about firings and not about matches**, and
reads as a gap only until the test is found. **Nothing was missing from `spec/`; what was missing was
the connection between the generated form and the test**, and the test's own name carries it - *the
unworked citizens*, plural.

## What this proposal got wrong, which is worth more than what it got right

**It was drafted before looking for the test.** `docs/process.md` says a case he does not understand
is answered either by a correction or by an explanation, and *either way the correction lands in
`spec/` and in a unit test* - so the first move is to ask whether the unit test exists, and this
lane's first move was to draft the sentence. **The order is what makes `CLAUDE.md`'s rule fire**: look
for the test, then write prose only if there is none.

## Settled by `P-598`, and this lane's reason for withdrawing it was wrong

**It asked exactly the question `P-598` answers**, and offered the two options by name: *coalesce*,
and *keep it per firing*. **`P-598` picks coalesce** - *a state has one entry per description, so two
things alike are one quantified row and never a row per firing.*

**So the withdrawal's reason does not hold.** It said a test already decides it, and the test decides
the **game's behaviour** - a rule reaches every row that matches. **The format of a generated case is
not something any test states**, and it was genuinely open: he answered *it is fine* within the hour
and reversed it on reading a second case.

**The instrument answered a narrower question than the one asked**, inside this lane's own withdrawal:
*is this fact already asserted?* about behaviour, where the question was about format. **The right move
was to leave it open and ask**, and what made the difference was him reading
`regression/scenario/01/06-end-turn.4x` rather than anything this lane did.

**Left withdrawn rather than reopened**, because `P-598` has landed and nothing waits on it. The
record is corrected so that the reason does not read as a precedent.

### P-596 - All five prototypes link main code, and rule 18 cannot be kept by replicating

**to** sean · **status** open · **raised** 2026-09-30 · **deferred** 2026-09-30, by him: *I will come back to it after I am happy with the first release* - so it waits on the first release rather than on him reading it, and no lane is blocked: nothing is deleted, `S-226` stays held · **asks** a decision · **kind** recovered · **shape** text · **into** `docs/architecture.md` -> Rules · **source** `C-191`, against the rule you promoted the same day

**Rule 18 says a prototype replicates rather than links, and *it is almost always a smaller and
modified version*.** The code lane measured what replicating would actually cost, and the second half
of your sentence is what the measurement contradicts.

```
                 own lines    main-code lines linked
gap-view               717                 4,129
goldberg-move          888                 9,023
goldberg-view          181                 9,023
hex-torus-view       1,004                   367
planet-view            450                12,431
```

**Replicating what `planet-view` links copies 12,431 lines into a prototype of 450**, and about 35,000
lines across the five. **That is not a smaller and modified version; it is the mainline with a copy
date.** So rule 18 as written cannot be satisfied by replicating here - which is the rule working, not
failing: it says a prototype must not hold main code in place, and the honest reading of these numbers
is that four of these five are not prototypes any more.

## The choice, and only you can make it

**Delete, or keep and accept the breach.** `docs/prototypes/README.md` says *that answer is the
deliverable; the code is a byproduct*, and **three of the five have their answer recorded** - so
deleting takes the breach, `planet-ecs`, `planet-flat`, `planet-raster` and `C-187`'s 399-line rules
engine with it, and loses nothing written down.

**Against that is your own keep-rule**, stated the same day: *prototypes about experiments I will want
to keep until I have already implemented their results into code.* **`planet-view`'s note records an
open question** - *GPU is an open question, not a decision that has been made* - and `planet-raster` is
the apparatus for answering it. **So the keep-rule protects at least one of the five and the link-rule
condemns it**, and that is the contradiction rather than a cost to weigh.

## What is not being asked

**Not whether to delete `planet-view`.** The two rules disagree about it, so a lane choosing either
would be deciding which of your rules wins. **Three of the five may be a different answer from the
other two**, and which three is a fact the code lane has - the recorded answers - rather than a
judgement.

**And nothing has been deleted.** `C-191` says so plainly: *I have not deleted a prototype on my own
reading of one rule.* **`S-226` is held rather than refused** for the same reason - its resolution
moves 399 lines into a prototype that is itself in breach.
*Nothing is open. Everything filed has been decided.*
