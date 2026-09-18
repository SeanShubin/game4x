# Working with an assistant

**Derived.** Written by Claude, 2026-09-18, from one long session on `prototypes/thin-engine`. **The
method is Sean's and the analysis is Claude's**, which is the honest split: he stated the two rules
and applied them, and what follows is a record of where they bit and why they worked.

[Documentation map](README.md) · [The process](process.md) · [Root README](../README.md)

## The two rules

Sean, 2026-09-18:

> My two rules as a human are, "never trust anything an ai says", and "never let the ai make a
> decision".

**On the surface those make an assistant useless.** If nothing it says can be trusted, every claim
must be checked, and checking is the work. If it may not decide, then every fork comes back to the
human, and the human is the bottleneck again.

**Neither consequence follows, and the reason is that he does not use it as an oracle.** His own
statement of how he gets around them:

> I get around that by questioning AI claims until they match what I already know, and instead of
> choosing among alternatives the AI gives me, I push back with the relevant principles until the AI
> is forced to eliminate all but the one correct option. I wait until I see what I expect to see
> that I already know is right, so that is how I never believe the ai and never let it make a
> decision, yet still end up with a conclusion that the AI allowed me to reach faster than I could
> on my own.

Two moves, and they are different from each other. **Questioning claims** is about facts. **Pushing
back with principles** is about decisions. The first stops him believing something false; the second
stops him choosing from a menu somebody else wrote.

## What questioning claims caught, in one session

Each of these is a claim the assistant stated as fact and Sean rejected without checking the code -
he rejected it because it did not match what he already knew.

| The claim                                                                         | What he said                                                                               | What was actually wrong                                                                                     |
| --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| `from` is derivable, so `{move what:scout to:territory-2}` is the minimal command | *if it is true it is only true in a coincidental sense and not a general sense*            | Generalised from a world holding one scout, one commit after proving that you cannot                        |
| The specification contradicts itself about a deposit's density                    | *what made anything think density was part of deposits key in the first place?*            | No contradiction existed; it was manufactured by reading a legal state as illegal. Item filed and withdrawn |
| An input's name is ambiguous across rules, so references must be qualified        | *Wouldn't the invariant still be fine unless one rule took 2 wheres?*                      | A limitation of the translator, presented as a property of the model                                        |
| A scout cannot also be a refreshable                                              | *Why not? Many languages have multiple inheritance... Some languages have duck typing*     | A consequence of one line the assistant had written, presented as a law                                     |
| Six hundred rows, and every test would need re-reviewing                          | *how many tests need to be rewritten should not be a metric used to judge a change as bad* | A cost that is not a cost in a prototype, used as an argument                                               |
| `build-extractor` matches labor with a literal                                    | (caught by the quality lens, not by Sean)                                                  | True until two commits earlier; the example had been deleted and the assistant kept citing it               |

**Five of the six are the same failure.** Not a wrong answer - in most of them the conclusion was
right - but **a wrong reason attached to a right answer**. The assistant reconstructed a
justification that sounded like the sort of thing that would be true, and in each case the
reconstruction was stale, manufactured, or an artefact of its own implementation.

## Why no check catches that

`CLAUDE.md` already says it, and this session demonstrated it three times in an hour:

> A check cannot catch this because a check is the thing that has the predicate.

**Every one of the six was found by a person re-deriving a claim that arrived finished.** None was
found by a test, and the suite was green throughout. Two were found by the quality lens checking the
code lane; four were found by Sean reading a sentence and knowing it was wrong.

**The asymmetry that makes the method work is this**: the assistant is fast at deriving, enumerating
and transcribing, and unreliable at knowing which of its own statements are load-bearing knowledge
and which are plausible reconstruction. It cannot tell those apart from the inside. **A human who
holds the model can tell instantly**, because the false ones do not match what he already knows.

So *never trust anything an AI says* is not scepticism. It is a division of labour: the assistant
produces, the human is the only instrument that can tell reconstruction from knowledge.

## What pushing back with principles did

The second rule is the one that looks most like a bottleneck and is not, because he never picks from
the menu. Three times in this session the assistant offered alternatives; three times he answered
with a principle instead, and the principle eliminated all but one.

**The trilemma.** Offered three notations for a berth allowance. He did not choose. He wrote down
what he wanted - *not specify anything more if there only happens to be 1 choice; the option to have
more than one choice; the notation uniform regardless of number of choices* - and the assistant then
had to show that the three map onto a single question and that one of the three has to go. **He
dropped the one the specification did not need.** The decision was his and the elimination was work
he did not do.

**Separate and grouped commands.** Offered four ways to parameterise refresh. He declined to choose
and gave a principle: *I should be able to declare separate things with separate commands, as well
as explicitly declare group commands.* Two of the four died immediately - neither could say *just
scouts* - and a third turned out to be a special case of the first. **One option survived, and he
never picked it.**

**Duplication.** The assistant recommended declaring something derivable and gave a reason. He
supplied the principle instead - *inconsistency can be mitigated by automated checks; simplicity is
more important from the expression side that I audit* - and the assistant, applying it, found its own
reason was circular and produced a different one that actually holds. **The recommendation survived
and its justification did not**, which is the same failure as the six above, caught by a principle
rather than by a memory.

**Why this is stronger than choosing.** Picking one of the assistant's alternatives accepts its
framing, including whatever the enumeration silently assumed - the failure
[every option was the same move](postmortems/every-option-was-the-same-move.md) records. A principle
is outside the enumeration, so applying it can eliminate the whole menu, and *has*: twice in this
session the right answer was not among the options first offered.

**And the elimination is checkable in a way a choice is not.** When the assistant says *C and D
cannot say just scouts*, that is a claim about the model which can be tested. When it says *I
recommend B*, there is nothing to check but its judgement.

## The part that is easy to miss: auditing is a design constraint

**The method only works while the human can still hold the model**, and that is not free - it is
bought, repeatedly, at the cost of the artifact.

Sean, 2026-09-16, on why the notation must stay small:

> The primary reason for these conciseness and simplicity requirements are so that I can maintain
> executive control as a human. I felt I was losing control from the spec instance so now I am
> redoing everything incrementally from the ground up.

And 2026-09-17, on where numbers must live:

> There are certain things I always want to see in tests because I need to compute the tests in my
> head. So something like the berth system, I need see what the actual numbers are, not some hidden
> default.

**That second one overturned a layer boundary.** The assistant had put an allowance in the ruleset,
where a test could not show it; keeping the tests auditable was worth more than keeping the layer
clean, and `prototypes/thin-engine/layers.md` now records the precedence.

**The review mechanism is the same constraint made mechanical.** Every test carries a copy of the
version he last read, and it says *drifted* the moment the test changes. He named its value himself
while rejecting a design: *I am glad we developed a review system for the test because I don't like
the current berth system and it is good to have an option to keep it out by not approving any of
these tests.* **Not approving is a veto that costs him nothing and the assistant cannot route
around.**

So the loop is: the human can only audit what stays small, so he spends decisions on keeping it
small, which keeps him able to audit. **An assistant left to optimise for its own convenience would
spend that budget elsewhere** - on fewer rows, on a cleaner layer, on not rewriting tests - and each
of those was proposed in this session and rejected.

## What it costs

**It is slower per exchange**, and visibly so. Several decisions here took four or five rounds that
one instruction would have settled.

**It requires him to already know the answer, or enough of it to recognise the answer.** *I wait
until I see what I expect to see that I already know is right* is only available to someone who
holds the model. The method does not transfer to a domain where the human has no priors - there, the
first rule leaves nothing to stand on.

**And it does not remove the need to check.** Sean rejected six claims here; the quality lens caught
a seventh that he had no reason to doubt, because it was about a file he had not read. **Two
independent readers found different things**, and neither found all of them.

## Open questions

**Does the method survive the human not reading everything?** It has not been tested at a scale
where he cannot. The review mechanism exists partly to find out - it will say how much has drifted
unread, which is a number nobody has yet had to look at.

**Is *never let the AI decide* the same as *never let the AI eliminate*?** He lets it eliminate
constantly, and elimination under a principle he supplied is how most of these decisions were
reached. Whether that is the rule holding or the rule bending is his to say.
