# Proposals

**Words for you to approve.** Every item here is open and waiting on you, and nothing else is in
this file. Say *promote P-n* and it lands; say what to change and it changes first.

[What is here](README.md) · [Questions](questions.md) · [The specification](../spec/README.md) · [Root README](../README.md)

**Where the rest went.** What has closed, what is addressed to the other perspectives, and the
ledger of everything accepted are in [`docs/notes/proposals.md`](../docs/notes/proposals.md). None
of it needs you.

## Open

### P-592 - `P-591` removed a command its own rule protects, and this lane put the wrong row in the table

**to** sean · **status** open · **raised** 2026-09-29 · **asks** approval · **kind** measured · **shape** text · **into** `spec/console.md` -> Commands

**The rule you promoted says a command goes *when the notation can say what it said, not before*.**
Nothing can say what `add <unit> orbit` says, so the rule protects it - **and the table this lane put
in front of you said it goes.** You approved the rule and the table together, and the table was wrong
about one row.

**Offered as a bullet restored to the design-phase list, after `set resource`:**

> - `add <unit> orbit` - place a unit in orbit before play begins

## The bootstrap, traced rather than asserted

```
launch        removes a labor
labor         comes only from toil - the one `add relation:labor` in the file
toil          requires a citizen
citizen       comes from deploy and from breed
breed         removes a citizen, so it cannot be first
deploy        removes a founder, and founder is {pioneer, ark}
pioneer       comes from build-pioneer, which needs a labor - round again
ark           comes from launch and from gather, and gather requires an ark
```

**So nothing in the sixteen rules can produce the first unit.** `add <unit> orbit` is the only way to
state an opening position, and with its line gone the specification has no way to say one.

**`generate-planet` does not rescue it**: `spec/console.md` says it makes *everything a designed one
needs*, and it is unbuilt - `world.rs` mentions it twice, both in comments about what it would do.

## What this lane got wrong, precisely

**It applied the first half of the rule and not the second.** *A command that can be expressed as a
row* was read as *a command that writes few rows*, and `add <unit> orbit` writes one - so it went into
the going column on a count, which is the criterion you had just declined. **The second half exists
to catch exactly this** and this lane wrote it, promoted it, and then did not use it.

**Found by the code lane declining the instruction** rather than carrying it out, which is what
`CLAUDE.md` means by a producer refuting a finding. `C-180` is theirs.

## Two things would retire it, and neither is anybody's to start unasked

- **`generate-planet` built.** It is specified to make everything a designed planet needs, which
  includes whatever stands in orbit at the start
- **The tree that normalizes into rows** - the same thing `S-223` says retires `set resource`

**They ask which and will build it.** It blocks nothing: the command works today.

