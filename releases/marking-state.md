# Release: Marking State From A Page

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Releases](README.md) · [Specification](../spec/README.md) · [Root README](../README.md)

## Goal

I can mark a rule test, an interface test or a regression case as approved, pending or denied from
a page I open anywhere, and the terminals see what I said.

## Scope

**The rules are already promoted and this release invents none.** `spec/README.md` rule 3 says what
an approval is about, what a record holds, how two tests are compared, and where a case's verdict
lives; `docs/process.md` says a case's verdict is a reminder and that authorizing a regeneration is
a separate gesture.

**The page is served from GitHub Pages and writes through the GitHub API with Sean's own token**,
which is what keeps `CLAUDE.md`'s rule intact: no lane writes a verdict, and the token is his.

**Interface tests are in scope and there are none.** `spec/tests/interface/` is empty, so the page
shows an empty set rather than a missing one - which is the state it is in rather than a defect.

## Capabilities

### E-1 - The comparison normalizes, and no approval rests on an editable number

**to** code · **status** open · **from** `P-606`

- **In** - `spec/README.md` rule 3, *one function does that normalizing and everything that compares
  delegates to it*
- **Vetted when** - renumbering a `seq:` in `spec/data/schema.4x` clears no approval, and every
  record is in the order `spec/console.md` states. **The 57 convert**, which is not a change of
  approval - Sean, 2026-10-01: *the order of the columns is not significant, so this should not make
  tests different*

### E-2 - I can deny a test, not only approve one

**to** code · **status** open · **from** `P-605`

- **In** - `spec/README.md` rule 3, *a record saying `denied` means it is not* bound
- **Vetted when** - I deny a test, the suite stops running it, and the record says so. **The format
  and the reader already exist** - `{verdict state:denied}` and `verdict_of` - so this is a gesture
  and not a mechanism

### E-3 - A case takes a verdict and a regeneration is authorized separately

**to** code · **status** open · **from** `P-607`, `P-608`

- **In** - `spec/README.md` rule 3, *a case's verdict is a row in `reviewed/cases.4x` and pins no
  behaviour*
- **Vetted when** - I mark a case denied and it stays marked across a run; I authorize a
  regeneration and the case is rewritten without my deleting anything. **165 cases**, and no suite
  is privileged

### E-4 - I can do all of it from a page, from anywhere

**to** code · **status** open · **from** him, 2026-09-30: *a hosted app on github would be fine*

- **In** - `docs/process.md`, *I insist that the AI make its work verifiable to a human*
- **Vetted when** - I open a page away from this machine, mark a rule test and a regression case,
  and a terminal here sees both without my touching git. **A page that lists 222 rows flat is not
  this capability met** - the rule tests, the interface tests and the four regression suites are
  distinguishable without my counting
- **Shown without a control.** Sean, 2026-10-01, asked whether `types/` and `primitives/` belong on
  the page at all: *let's show them, but these are informational only, no vetting capability need be
  implemented. We can even link to them if it helps with comprehensibility.* **So the 113 cases in
  those two suites are listed and linked and offer nothing to press.**
- **That is the interface declining to offer a control, not the notation forbidding one.**
  `spec/README.md` rule 3 says *no suite is privileged* and `reviewed/cases.4x` will take a verdict
  for any case, including one of those 113. **A reader comparing the page to the rule would see a
  contradiction and there is none** - one says what may be recorded and the other says what he has
  asked to be built


### E-5 - Nothing but me can write what I said

**to** code · **status** open · **from** `CLAUDE.md`, the record rule

- **In** - `CLAUDE.md`, *the record is the one artifact whose whole value is that nobody judged by
  it can touch it*
- **Vetted when** - a lane asking the page or the workflow to mark something is refused by the same
  gate a stranger is. **`review.yml` already holds this** with
  `github.actor == github.repository_owner`, and whatever ships keeps it
