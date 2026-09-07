# Research outbox

**Derived.** The research lens's one outbox. Every finding it has addressed to somebody, and what
became of it. Not binding - a finding is a claim about the tree, not a decision about it.

[Research](README.md) · [Reports](README.md#reports) · [The proposal queue](../../docs/notes/proposals.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to spec` - something for `spec/`, `releases/` or `docs/`. The specification lane turns it into a
  numbered proposal; **it does not decide it.**
- `to code` - a defect or a decision in `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`,
  `hooks/` or CI.
- **Unaddressed** research does not appear here at all. It lives in a dated report and is nobody's
  work until this file gives it a reader.

**Status** is one of `open`, `noted`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` is
outstanding; `noted` is *recorded so it is not re-found*, and is terminal.

> **The guarantee.** If nothing here is `open`, this lens knows of nothing outstanding. That is a
> promise about this file, not about the tree - it does not say the documents are right, only that
> everything this lens knows to be wrong is sitting where its reader will find it.

**A producer may decline a finding, and often should.** It says so in the commit that declines it,
citing the id, and this file records it. Check a rejection before defending it.

## Open

### X-1 - what makes the game checkable by hand is never stated

**to** spec · **status** open · **raised** 2026-09-06 · **source** [report](2026-09-06-what-the-theory-already-settles.md)

`docs/process.md:91` says the definitions, the transformations and the commands *"are enough to
derive the fourth by hand"*, and rests *how I know the application is right* on that. It is true only
if the transformation reads the state and the commands and nothing else - and the document never says
so. Grepping `determin|replay|nondeterm|random|clock|reproduc` over `docs/process.md` returns **0
hits across 459 lines and 24 headings**, so the zero is against a named population that is not also
zero.

**Why it costs something.** The precondition is load-bearing for the artifact Sean checks by hand,
and it fails silently: a clock, an entropy source or an undeclared file read makes the fourth
artifact underivable while every test still passes and the scenario diff he reviews becomes noise he
cannot tell from a real change. It also decides a question the code lane is about to meet from the
other side - whether the environment a command executes against may answer anything the four
artifacts do not declare.

**Whether.** Worth doing now, and it is one sentence rather than a section. This lens has
deliberately not drafted it: the words are what Sean approves, and they are the specification lane's
to write.

### X-2 - the default in `layers.md` does not satisfy the rule in `turn.md`

**to** spec · **status** open · **raised** 2026-09-06 · **source** [report](2026-09-06-what-the-theory-already-settles.md)

`spec/turn.md:24` requires that what settles competing effects be *"a deterministic mechanic of the
game, and therefore something a person wrote and a player can change"*. `docs/layers.md:207` proposes
that where two events collide *"the lower index wins"*, and `:210` makes that reading **the
default**. An array index is deterministic and reproducible, and it is not a mechanic of the game,
nobody wrote it as a rule, and no player can change it.

**Why it costs something.** `docs/layers.md` is the non-normative why-layer, so this is not a
contradiction inside `spec/`. It is worse in one specific way: it is the document the code lane
reads for guidance, it says *default to this*, and following it would build something the
specification forbids. Nothing would report the divergence, because each file is correct on its own
terms.

**Noted alongside, and deliberately not filed as its own item.** The same decision has a second face
in the code - whether a command executes against a shared mutable environment or returns its effects
for a later merge - and settling it in one place and not the other is how it gets made twice,
differently. Recorded here rather than addressed to `code`, because the answer is not determined
until this item is.

**Whether.** Worth doing eventually rather than now. Nothing is being built on it today, and the
question is a real one for Sean rather than a defect to repair.

## Closed

Nothing yet.
