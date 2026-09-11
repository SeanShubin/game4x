# Three exemptions and a source

**2026-09-11.** Sean asked why he could see instantly that all three of my answers were wrong, and
saw the right one as obvious, when I could not see it at all.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

## What happened

`spec/invariants.md` says *nothing comes back round with more*, and Sean wants it enforced rather
than believed. I established that the obstacle is `refresh`: it is the only recipe that makes a thing
`ready`, and readiness is what `create labor` and `work` spend in order to make labor and resources.
Under any weighting, `refresh` gains.

I enumerated three ways out and asked him to choose:

1. **Exempt the recipe** - declare `refresh` as drawing on a named source
2. **Exempt the phase** - require the invariant of the player's ten recipes and not the world's
3. **Exempt the trait** - weigh readiness at nothing, which I showed does not work

His answer: *the planet provides infinite materials, the star provides infinite energy, time provides
infinite turns, and anything that exhausts is a readiness extractor for a turn.*

**There is no exemption in it.** Readiness is not an anomaly the check must be taught to ignore. It
is gathered from a third source exactly as metal is gathered from the first, and the rule then covers
every recipe with no carve-out at all.

## Why every option I produced was the same move

**All three were exemptions.** Three surfaces - a recipe, a phase, a trait - and one move underneath:
*carve a hole in the invariant and argue the hole is safe.* I produced three, eliminated one by
argument, and the work of eliminating it felt like rigour. **It was rigour inside an assumption**,
and the assumption went unexamined because nothing in the list disagreed with it.

**That is the tell, and it is checkable at the moment of writing.** When every option in an
enumeration is the same kind of move, the enumeration is a subtree rather than the tree. A real
enumeration has options that disagree about something. Mine disagreed only about where to cut.

This is `CLAUDE.md`'s recurring failure in new clothes - *the instrument answers a narrower question
than the one asked, and returns a plausible number rather than an error.* Here the instrument was a
proposal and the plausible number was three options. **A wrong answer invites a question; three
plausible answers invite a choice.**

## Four reasons I could not see it, in the order they bite

**1. I inherited a closure nobody stated.** The specification said *a star's energy and a planet's
material are endless.* That is two examples. I read it as the set of sources, and then asked what to
do about the thing outside the set. **Sean read the same sentence as an instance of a rule** - an
endless well behind a bounded pump - and applied the rule again. Nothing in the file said the list
was closed and nothing said it was open; I supplied closure for free, because a list of two reads
like an enumeration.

**2. I was in a debugging frame and he was in a modelling frame.** My question was *why will this not
verify.* Everything cheaply reachable from there is a repair to the check: exempt, scope, zero. His
question was *what is readiness, actually.* From there the answer is a sentence about the world that
happens to make the check work. **When a formal check fails, the nearest repairs are all to the
check, and the right repair is often to the model.** Nearness was doing the choosing, and it is not a
good chooser.

**3. I had both halves and never put them together.** Hours earlier I had written that `work` is
extraction from a source, quoting `spec/resources.md` - *an extractor, and the labor to work it,
bring matter out of its source.* Minutes earlier I had established that `refresh` is the only maker
of readiness. **The analogy was sitting in my own output twice and I did not run it**, because I had
filed the two halves under different headings: one was the resource economy, the other was the
invariant problem. **The filing is what stopped it.** A fact indexed under the problem it was found
in is not available to a different problem.

**4. The answer was a design act, and design is his.** I cannot add a source to the game; adding one
is inventing a rule, which this lane must not do. **So part of this is the process working as
intended** - and that is the half I do not get to claim, because it explains why I could not *decide*
it and not why I could not *see* it.

## What I actually owed him, which is smaller than the answer

**My job was to ask the question, not to answer it, and I asked the wrong question.** The right one
was one line: *readiness comes from somewhere - where?* He would have answered it in a sentence,
which is what he did anyway, after first having to reject my framing.

Instead I asked *how should we excuse readiness?* - which presumes it is an exception, offers three
ways to agree with that premise, and leaves no shape the real answer fits into. **A proposal that
asks a decision is supposed to state the choice rather than resolve it.** Mine resolved a prior
question silently - *is this an exception?* - and presented the leftovers as the choice.

## The rule worth keeping

**If every option in an enumeration is the same kind of move, the enumeration is inside an
assumption - name the assumption and ask about that instead.**

It is cheap to apply: read your own list and ask what its entries have in common. Here the answer
would have been *they all carve out an exception*, and the question behind it - *must readiness be an
exception at all?* - is one sentence, and reaches the right answer immediately.

**It fires at a moment of confidence**, which `docs/process.md` says is the kind that does not
survive as a habit and needs a carrier. The carrier available today is the shape of a proposal
itself: a proposal asking a decision states the choice, and **a choice whose options are all one move
is a choice that has not been stated yet.** `P-388` is the case, and this note is the record of it.
