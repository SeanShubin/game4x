# Questions

**Choices only you can make.** Not words to approve - a question here is open because no wording can
be final until it is answered. Nothing else is in this file.

[What is here](README.md) · [Proposals](proposals.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**An item lives in one file at a time.** When the last question in it is answered it becomes a
proposal and moves to [`proposals.md`](proposals.md); the reasoning stays behind in
[`docs/notes/decisions.md`](../docs/notes/decisions.md).

## Open

### P-565 - `L2` answered, `D-5` already carries it, and what `spec/tests/` is remains open

**to** sean · **status** open · **raised** 2026-09-26 · **half answered** 2026-09-26, `L2` · **asks** a decision · **kind** entailed · **into** `spec/README.md` rule 3, and whether `spec/tests/` exists

**You answered the lock and not the directory.** *The old scenario test where we conquer the whole
planet is gone, we will need a new scenario test to handle the smaller expand then launch* - that
is `L2`, a new scenario rather than the reviewed tests as the lock.

## You were right, and this lane was wrong about it twice

**It existed and it was a requirement rather than a file.** `R-6`'s *vetted when* asked for **a
fully exploited planet** until `P-422` changed the target on 2026-09-12 - `crates/outbox.md`:
*the vetted when no longer asks for a fully exploited planet*. `R-6`'s own text still carries the
scar: *these numbers were taken against an older vetted when that asked for a fully exploited
planet, which the release no longer requires.*

**This lane first answered that no scenario conquers the planet**, having counted territories in
the seven files of `scenario/commands/`. **That was true and it was not the question** - you said
*scenario test*, and the thing you remembered was what the scenario test was checked against.
Same shape as the three errors already recorded today.

**So nothing is lost and nothing needs hunting.** What you described as the replacement is what
the release should say, and the only question is whether `D-5` says it.

## And `D-5` already is that new scenario test

**You promoted it in `P-564` an hour ago.** *A main scenario exists over the reviewed ruleset and I
have watched it run: a first territory taken, a second taken by land, and an Ark launched from the
second.* **So `L2` needs no new capability** - it needs rule 3 to point at that instead of at the
scenario being deleted, which is a rewrite for you to read and this lane will bring it once you
settle the other half.

## What is still open, and it is one question

```
T1  delete spec/tests/       reviewed/ is canonical; a byte-identical copy of it is
                             clutter. CLAUDE.md names spec/tests/, so this needs your
                             approval there too
T2  keep it, and say what    spec/tests/ is where a test is written, reviewed/ is where
    it is                    one you have read lands. Rule 3 gains a sentence
```

**`T3` is withdrawn** - it proposed deleting `reviewed/`, and you have since said the approved
tests are canonical, which settles it.

**This lane would say `T2`**: the two are a before and an after rather than a duplicate, and they
are identical today only because you have read all 54.
