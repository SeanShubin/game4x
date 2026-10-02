# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-611 - Three states, one vocabulary, and only one of the two kinds clears on change

**to** sean · **status** open · **asks** approval · **raised** 2026-10-01 · **revised** 2026-10-01, after he corrected the case half and said he expects a case and a test to be interchangeable · **kind** recovered · **shape** text · **into** `spec/README.md` -> rule 3

**Your three, and the application shows four for a test and none for a case.**

```
I haven't looked at it yet            no record / no row  -- and `drifted`
I looked at it and approved it        approved
I looked at it and know it is wrong   denied              -- and a note in reviewed/asked.md
```

**Replacing *No record means I have not looked; a record saying `approved` means the code is bound by
it; a record saying `denied` means it is not, and that I owe the specification a statement of what I
want instead*:**

>    **There are three states and no others, for a test and for a case alike.** I have not looked at
>    it; I have looked and approved it; I have looked and know it is wrong. **No record and no row are
>    the first**, `approved` is the second and means the code is bound where a test is concerned, and
>    `denied` is the third.
>
>    **Denied is where I say what I want instead**, when I have words for it. That is the same state
>    whether I have said it or not: *this is wrong* and *this needs changing* are one thing, and the
>    words are an annotation rather than a state of their own.
>
>    **A test whose rows have changed since I read it is in the first state**, because somebody edited
>    it and my approval was of what it said. **A case whose rows have changed is not** - the game
>    changed rather than the file, and *I looked at this and said it was wrong* is worth most at
>    exactly the moment it changes again. **The suite reports a stale case; it does not clear my
>    verdict on one.**

## Why the two differ, which is the only asymmetry left

**Only one of them has a hand in it.** A test is written by hand and can be reworded under me; a case
is generated and cannot change unless the generator does. **So a changed test means my words are
gone and a changed case means the game moved** - and those want opposite treatment.

## What a denied case is waiting for, stated because it is not obvious

**Sean, 2026-10-01**: *the regression test denial is something I expect to fix by creating a future
unit test and regenerating the regression tests.* **So a denied case is an item for the specification
lane** - write the test that says what should happen - and the case moves when the code obeys it and
somebody regenerates. **It is not waiting on the case and it is not waiting on me pressing anything.**

## And the interchangeability he is aiming at is nearly true already

**Sean**: *I am hoping the regression tests and unit tests can actually have the same contents when
everything is inlined... I could actually copy all of the regression tests and make them unit tests.*

```
a case  {test} {load} {given} {when} {then}        world.4x holds 25 rows
a test  {test}        {given} {when} {then}        the same kinds, inline
```

**A case is a test with its invariant rows factored out.** Inline the `{load}` and the shapes are
identical - which is why the same three states fit both, and why this proposal says *for a test and
for a case alike* rather than stating the rule twice.

**Not offered as a rule**, because *could be copied* is a property to check rather than a thing to
declare, and nothing yet checks it.
