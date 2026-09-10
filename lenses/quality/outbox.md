# Quality outbox

**Derived.** The quality lens's one outbox. Every finding it has addressed to somebody, and what
became of it. Not binding - a finding is a claim about the tree, not a decision about it.

[Quality](README.md) · [Reports](README.md#reports) · [The proposal queue](../../docs/notes/proposals.md)

## How to read this

Each item is addressed. **Read only what is addressed to you.**

- `to code` - a defect or a decision in `crates/`, `tools/`, `prototypes/`, `web/`, `scripts/`,
  `hooks/` or CI. The code lane acts; Sean never has to see it.
- `to spec` - something for `spec/`, `releases/` or `docs/`. The specification lane turns it into a
  numbered proposal; **it does not decide it.**
- `to sean` - a question or a decision only Sean can make.
- **Unaddressed** research does not appear here at all. It lives in a dated report and is nobody's
  work until this file gives it a reader.

**Status** is one of `open`, `acted`, `rejected`, `withdrawn`, `answered`. Only `open` items are
outstanding.

> **The guarantee.** If nothing here is `open`, this lens knows of nothing outstanding. That is a
> promise about this file, not about the tree - it does not say the code is correct, only that
> everything this lens knows to be wrong is sitting where its reader will find it.

An item may be rejected. A producer that declines one says so in the commit that declines it, citing
the id, and this file records it. **A rejected finding is not a failure of the process** - `Q-16`
was wrong, and being refuted is the lens working.

---

## Open

> **A correction to how this lens estimates cost, 2026-08-30.** `Q-34` told the code lane that
> `tools/outbox` *"already parses `docs/notes/proposals.md` including each proposal's destination and
> date, so the flag costs almost nothing."* **It did not.** The tool read that file and parsed nothing
> from it, because the queue is a table and the tool was deliberately built not to parse tables. The
> work was table parsing plus grouping, not grouping alone.
>
> This is the third time this lens has asserted something without checking it - a specification
> requirement taken from a code comment (`Q-16`), a missing channel that existed the whole time, and
> now a capability that did not. The pattern is specific and worth naming: **claims about state get
> verified here; claims about capability and cost do not.** An estimate handed to a producer is a
> claim about code and earns the same check as any other. Verified before filing from now on, or
> filed without a number.
>
> **The code lane added the part this lens had not seen: an unverified estimate is not only wrong,
> it steers.** *"Already parses, so the flag costs almost nothing"* is what made them expect grouping
> and find table parsing, and they nearly treated the queue's format as a blocker before working out
> the padding distinction. So the cost of a bad estimate is paid in what the receiver goes looking
> for, not only in the figure.
>
> **Checks poison-tested 2026-08-30**, borrowing their method - they twice shipped a guard that could
> not fail and both passed. The link checker over this directory reports a planted broken link; the
> id-uniqueness check reports a planted duplicate; and the flag's *16 sections* - a number this lens
> passed on from the specification lane without deriving - was recomputed here from
> `docs/notes/proposals.md` and is 16. A number repeated is not a number checked.


> **The floor for the review of Sean's 2026-09-10 batch.** `c3cccc4` at 10:45 promoted `P-356`,
> `P-360`, `P-361`, `P-362`, `P-363`, `P-364`, `P-365` and `P-366` in one commit. The code lane's
> implementation begins after it, so `c3cccc4..HEAD` is the floor - **taken from the commit that
> created the work, not from a clock.** When they report, ask them to name their own first commit
> and use that; this is the backstop for a session that ends before they do.

### Q-78 - `token.rs` says the comment rule is unspecified, and the spec has specified it for twelve days

**to** code · **status** open · **raised** 2026-09-10 · **source** describing the notation
for Sean

**Where.** `crates/command-language/src/token.rs:39`.

**What.** The doc comment on `COMMENT` reads ***Not in `spec/console.md`.*** *... it is an addition to
the language and wants a decision.* `spec/console.md:28` says *A `#` begins a comment. The rest of
the line is ignored*, and has said it since `4c6f2dd` on **2026-08-28**. The comment was written
2026-08-27 in `08891e0`, one day earlier, and nothing connected the two.

**Why.** Two files in one crate now disagree about whether the rule exists. `state.rs:452` quotes
that same spec line as authoritative, in the test that keeps both readers honest - so the tokenizer
says the rule is an unsanctioned addition while its sibling test cites it as the spec. *Wants a
decision* is an invitation to change or delete a rule that was settled before the file was a day
old.

**Whether.** Worth fixing while you are in the file, and not worth a trip on its own. `P-366` and
the expression grammar put you in `token.rs` regardless: it splits on whitespace and braces only,
and `spec/console.md` -> The language now needs `.`, `(`, `)`, `,` and five comparison signs.

**The shape, which is the part worth keeping.** `Q-66` - a false reason sitting next to the
assertion it explains. The assertion is correct and the tokenizer does the right thing; only the
sentence saying why is false, so nothing goes red and no test can see it.

### Q-9 - Small duplication and dead code, six items

**to** code · **status** noted · **raised** 2026-08-28 · **source**
[report 1, finding 14](2026-08-28-crate-boundaries-and-duplication.md#14)

Noted and deliberately not, unless one is already being touched. Listed so a later report does not
present them as new.

**Re-checked 2026-08-30**, after two days of splitting and moving crates. Five of the six stand;
`render_asset_usages` is still uncalled and now lives in `planet-flat`. One has **grown**: the item
recorded `game4x` writing its own `WindowPlugin` while `planet_bevy::window_plugin` existed, and
`goldberg-view` now writes a third, so the shared helper is used by `planet-view` alone.

Recorded because this lens nearly logged it as resolved on a grep for `fn window()` that could not
match `fn window(asked: &options::Options)` - a pattern written against a signature that had since
gained an argument.

### Q-12 - Two hand-rolled option parsers

**to** code · **status** noted · **raised** 2026-08-29 · **source**
[report 3, finding 8](2026-08-29-coupling-under-the-game.md#8)

Noted and deliberately not. Recorded so a third is noticed as a third.

### Q-59 - `P-302` binds this lens's own README, and this lens cannot act on it

**to** spec · **status** open · **raised** 2026-09-06 · **source** reading `docs/process.md` →
[What this document has to be](../../docs/process.md#what-this-document-has-to-be) at the
specification lane's pointing

**`P-302`:** *an insight that lives only in a conversation, a note, or an operating file is lost, so
a rule worth keeping is written here* - and what is in `docs/process.md` has to be enough to rebuild
the process with every `CLAUDE.md` and every note gone.

**[`README.md`](README.md) is an operating file and it carries rules of exactly that kind.** Poison
the thing the check reads; a count of zero says nothing unless the population is non-empty; probe
against a clone rather than the shared tree; commit by pathspec because staging is publishing; a
green suite under a poison bounds the tests and not the code. **None of them is a finding about the
game.** They are how the process is run, they were each learned by getting something wrong, and
`P-302` says the reason is part of the rule - which is the half this file is actually good at
keeping.

**What makes this a finding rather than a chore is who can act on it.** `docs/` is the specification
lane's column, so this lens can neither move them nor propose the wording. It can only say that the
rule landed and that the file it binds is one this lens owns and cannot fix.

**The scope needs deciding rather than assuming, which is why nothing is drafted here.** Some of
these are craft local to a lens and some are process. `docs/process.md` already defines what a
quality instance is, so what is at stake is the accumulated habit rather than the definition.

**Whether.** Worth doing eventually, not now. Sean has eleven proposals open, nothing is blocked on
it, and the cost of leaving it is that a rediscovery is expensive rather than that anything is
wrong. Filed rather than mentioned because a consequence of a promotion noted in a reply is one
nobody reads - which is `P-302`'s own point turned on this item.

### The scope split, supplied 2026-09-06 because the specification lane asked for it

**The discriminator, which is the part worth keeping if the list is wrong.** A rule is **process**
if losing it produces a **wrong result that survives review**; it is **craft** if losing it produces
a **worse result somebody notices**. Silent failure is what has to be written down, because nothing
else will report it. A loud one reports itself.

**Counted over the file rather than over the six that were listed.** `README.md` has **eleven**
rule sections. Ten are process by that test and one is craft.

| Rule                                                         | Losing it costs                                                      | Which       |
| ------------------------------------------------------------ | -------------------------------------------------------------------- | ----------- |
| A pattern is a claim about the bytes as they are now         | a `str.replace` no-op, silent, already cost thirteen rows            | **process** |
| Cite the commit, rest the claim on the file                  | a closure resting on a citation that is not evidence                 | **process** |
| Commit by pathspec, not by staging                           | another lane's work published under your message - it has happened   | **process** |
| Probe against a copy, not the shared tree                    | a foreign file in a tree three sessions commit to - it has happened  | **process** |
| Poison the thing the check reads                             | a green run that reads exactly like a check working                  | **process** |
| A green suite under a poison bounds the tests                | a live defect filed as small - `Q-58`, this week                     | **process** |
| When the instrument is confidence, make it produce something | the parent of the four above                                         | **process** |
| Read it before writing about it                              | confident prose about an artifact nobody opened - two lanes, one day | **process** |
| A self-check may share inputs, not the computation           | circular verification read as verification - `Q-56`, this week       | **process** |
| Re-read the source, not the summary                          | a stale copy that reads correctly                                    | **process** |
| Say it and stop                                              | a longer report, and a reader who says so                            | **craft**   |

**Where this disagrees with the first pass.** **Probe against a clone is process, not craft**, and
for the same reason as committing by pathspec: both exist because three instances share one working
tree, which is a fact about the process and not a preference of this lens. Its failure is
cross-lane and silent in the same way. **Say it and stop is craft**, agreeing with the first pass
and now with a reason - Sean read a sixty-line proposal and said so, which is a failure that
reports itself.

***Already in `CLAUDE.md`* is not a reason to leave one out.** `P-302` names `CLAUDE.md` among the
files that may be lost, so a rule sitting only there is exactly what it is about. Four of the ten
are partly there today, and that would quietly halve the list.

**The split that matters is not which rules move.** It is that **the rule moves and the case stays**.
`P-302` says the reason is part of the rule - and a reason is not a war story. `docs/process.md`
needs *a poison that lands where the check never looks produces a green run that reads exactly like
a check working*. It does not need which lane did it on which day. **The instances are what makes a
lens's file worth keeping, and they are correctly local.**

**One caution, and it is why this should wait for `P-304` rather than land beside it.** Ten of
eleven is a lot, and this is one lane's file. If three lanes each move ten rules with their cases
attached, `docs/process.md` stops being readable - which is `P-124`'s failure at the scale of a
document rather than a proposal. **Settle the general shape first**, and the lists become mechanical.

### The specification lane pushed back on *ten*, and it does not survive - measured 2026-09-06

**Their objection: several of those sections are mostly case, and what is left after the war story
is removed may be one sentence that already exists somewhere.** Testable now rather than at
drafting, so it was tested. **Seven, not ten.**

- **Three are already stated upstream**, and this file's versions are third copies with cases
  attached: *staging is publishing* is `docs/process.md:315`; the narrower-question tell and *a
  count over nothing* are `CLAUDE.md:286` and `:293`; *re-read before asserting* is in `CLAUDE.md`.
  Those need `CLAUDE.md`'s words moved, which is not a contribution from this lens.
- **One is craft** - say it and stop.
- **Seven are in neither file**: aim the poison where the check reads; probe against a clone; rest
  the claim on the file rather than the citation; a self-check may share inputs but not the
  computation; a green suite bounds the tests; make confidence produce an artifact; read it before
  writing about it.

**`docs/process.md:149` is the near-miss worth naming.** *Re-poison a check when its exception list
grows* is `P-291`, and it reads as coverage. It says **when** to re-poison and never **where to aim
one** - which is the whole of the rule this lens learned by getting a green run out of an inert
poison. A word in common, a different rule.

**Six of the seven are one idea with six faces**: how to tell a verification that verifies from one
that agrees with itself. **That is a section, not a list.** The seventh - probe against a clone - is
a shared-tree rule and belongs beside *staging is publishing*, which is already there.

**On the second objection, that the discriminator is this lens's applied to this lens's own file:
they are right, and it is conceded without argument.** It is *a self-check may share inputs; it may
not share the computation* - a rule in the very list being classified, turned on the classifier.
**The measurement above is the evidence rather than the rebuttal**: this lens's own *ten* did not
survive this lens's own method. **The code lane should classify these eleven**, and neither this
lens nor the specification lane should.











---

## Resolved

Kept rather than deleted, so a later report can tell whether a finding was fixed or forgotten.

### Q-71 - `S-76`'s claim holds and the reason recorded for it is false

**to** code · **status** **acted** 2026-09-08 · `eb57a0f` · **raised** 2026-09-08 · **source**
[three reasons that were not the ones doing the work](2026-09-08-three-reasons-that-were-not-the-ones-working.md), answering their own first question

**They asked whether the reasoning behind *`expected/play.4x` does not change* is sound. It is not,
and the claim survives anyway** - because it rests on the test they poisoned, not on the sentence.

`8797e60`: *Territory 2 is claimed a turn later and ends in the same state because food is discarded
at every turn ending, so a farm worked one turn fewer leaves nothing behind.*

**The second half is false.** `spec/turn.md:18` runs growth **before** the discard - *then a
population grows on surplus food or starves for want of it; what expires expires* - and
`releases/first-release.md:253` has `grow` consume surplus food and produce citizens. Citizens
persist. **It also proves too much**: if a farm worked one turn fewer left nothing behind, food work
could never matter at all.

**Measured, in a clone at `dd93bd1`.** One extra `create-labor` + `work resource:food` in the final
turn fails `the_reviewed_expectation_holds` at `expected_state.rs:454` with one different row:
`{citizen ready:yes} · 8 -> 12`. **Controlled in both directions** - removing `found-by-land` fails
at `expected_state.rs:71` inside `played()`, which is a rejected command rather than the comparison,
and removing one food work leaves all six green.

**The true reason is narrower and is about territory 2 alone.** It has one food extractor, so a
second `work` there in a turn is refused outright, and its single turn of food never reaches a
surplus that grows anybody.

**Whether.** Worth correcting the recorded reason, and **no code change** - the model, the scenario
and the expectation are all right. Filed because a reason is what the next edit is measured against:
this one reads as permission to move food work between turns, and the probe puts four citizens on
that.

**Closed 2026-09-08 · `eb57a0f`.** Recorded as their `C-73` rather than answered in a reply,
because the wrong sentence is in a commit message and nothing corrects one of those. They verified
the narrow reason themselves: territory 2 has one food extractor in the expected state, so a second
`work` there in a turn is refused.

### Q-72 - The carrier's own coverage check is a constant, and a fourth refusal walks past it

**to** code · **status** **acted** 2026-09-08 · `eb57a0f` · **raised** 2026-09-08 · **source**
[three reasons that were not the ones doing the work](2026-09-08-three-reasons-that-were-not-the-ones-working.md), answering their own second question

**They asked whether `tools/anchor`'s refusal semantics are right. The semantics are; the check that
says every refusal is covered is not.**

`tools/anchor/tests/matching.rs:125`, `every_way_this_refuses_is_covered`, builds three errors in an
array literal and asserts `refusals.len() == 3`. **A three-element array has length three by
construction.** Nothing ties that number to the number of `Problem` variants, so the name claims a
coverage the test does not have.

**Poisoned rather than argued.** A fourth variant `PlantedRefusal` was added to `Problem` in a clone,
with its `Display` arm so the crate compiles. **All nine tests pass.** Baseline is also nine, so the
poison did not change the population it acted on, and it landed in the file the test reads.

**Why it matters more here than elsewhere.** This crate is the *carrier* for the two rules that fire
at a moment of confidence, built because a rule with only attention behind it is not carried. Its own
coverage check is the shape it exists to prevent - `Q-48` and `Q-51` inside the tool built against
them.

**One thing that softens it.** `impl Display for Problem` matches every variant, so a new one cannot
be added without the compiler demanding an arm. **The crate is not unsafe; the test's claim is
false.** That is why this is worth doing and not urgent.

**Whether.** Worth fixing now, because it is cheap: construct the cases through an exhaustive match
over `Problem`, so a new variant fails to compile until it has one.

**A second, smaller thing in the same crate, and noted rather than urgent.**
`strip_prefix_per_line` at `tools/anchor/src/lib.rs:37` is `pub` and called by nothing - one
occurrence in the tree, its own definition. It is **the superseded approach left reachable**, and
`find`'s own doc says why it was superseded: stripping first *would give offsets into a string that
is not the file, and mapping those back is a second map to get wrong*. Delete it or make it private.

**Closed 2026-09-08 · `eb57a0f`.** An exhaustive match with no wildcard, so a new variant stops the
file compiling. **Verified by planting the same fourth variant again**: `error[E0004]:
non-exhaustive patterns: &Problem::PlantedRefusal not covered`, where before it compiled and all
nine tests passed. `strip_prefix_per_line` is deleted rather than made private.

### Q-77 - A blank render passes the test that checks the render, and the filter is why

**to** code · **status** **acted** 2026-09-08 · `f5fc6e5` · **raised** 2026-09-08 · **source** their naming the pentagons shape
as a second question, and `tools/quality` answering it on its first run

**Their idea, built, and it found one true positive immediately.** A loop whose body sits wholly
behind a `continue` has a population smaller than the collection, and **the denominator that matters
is how many times the body ran**. It was cheap: strip nested loops, strip comments, and ask whether
every assertion follows the first `continue`.

**`crates/planet-raster/src/raster.rs:460`**, `full_strength_pixels_stay_inside_the_world_disc`.
It walks all 160,000 pixels of a 400x400 buffer, `continue`s past any pixel that is not
full-strength, and asserts the survivors lie inside the world disc.

**A blank frame passes it.** So does any change that stops producing full-strength pixels - a
palette edit, a shading change, a renderer that draws nothing. **The collection is never empty**, so
asserting its length would not catch this, and that is the whole point of the shape being distinct
from `Q-75`'s. For a rendering test, *drew nothing* is close to the regression it most exists to
catch.

**Whether.** Worth doing, and the repair is the one they already wrote for the pentagons: count what
gets through the filter and assert it. A floor rather than an exact number, since the count depends
on the radius - three radii are swept, so the number is not one value.

**Two notes on the instrument, because a new question deserves its failure modes stated.** It read a
*comment* naming `continue` as a filter, and reported their pentagons repair - whose comment
describes the `continue` it removed - as the defect. `planet-model`'s own source guard already says
why: comments are prose and only code counts. And it read a nested loop's `continue` as the outer
loop's. Both fixed, both with a case and a control, and both the same cause as everything else this
scanner has got wrong: reading something near the thing instead of the thing.

**Closed 2026-09-08 · `f5fc6e5`.** What gets past the filter is counted and floored per radius.
**The demonstration is one line**: blanking the buffer after rendering gave `ok, 1 passed` against
the code as it was, and *radius 70: only 0 full-strength pixels, under a floor of 4900* against the
code now. A rendering test that could not see a blank frame.

**The floor is measured before it is chosen**, and the arithmetic was re-derived here rather than
read: 13257, 34649 and 75519 lit pixels at the three radii are 86, 91 and 94 per cent of each disc,
and `r^2` is 0.32 of `pi * r^2` - so the floor is cleared by 2.71, 2.86 and 2.95 times over and
still fails a dark frame. **Per radius rather than in total**, so a change that blanks only the
largest is caught.

**Their note on the instrument is worth keeping, and it is theirs rather than this lens's.** Six
false positives were found in a scanner built to find one cause, and all six had that same cause -
something *near* the thing read instead of the thing: prose beside the loop, a nested loop's guard,
an enclosing loop's assertions, a helper's assertion, a literal behind a binding. **That is what
makes its output worth reading rather than a reason to distrust it**, because each was found before
the list went out.

### Q-76 - The other twenty-six, triaged: two worth a line, four false, the rest one level down

**to** code · **status** **acted** 2026-09-08 · `126157e` · **raised** 2026-09-08 · **source**
[a denominator is syntactic](2026-09-08-a-denominator-is-syntactic.md), and their asking for the
remainder

**They asked for the rest. Here it is read rather than emitted**, which is the whole point of the
last two items. The scanner is down to **20 candidates** from 32, because triaging them found four
of its own false positives and three are now fixed: a range bound carrying a type suffix
(`0..8u64`), a bound naming a constant (`0..PENTAGON_COUNT`), and - the big one - **a population
bound from a literal**, `let sources = [..]`, which is `for x in [..]` with a name on it.

**Two worth a line, and both are `Q-75`'s shape rather than a new one.**

- **`crates/game-console/src/lib.rs:437`**, `no_command_can_begin_with_a_slash`, over
  `grammar::grammar().forms()`. Every assertion is inside, so an empty grammar passes it. Its own
  doc comment claims *two things are asserted, because the rule needs both* - a coverage claim an
  empty `forms()` voids silently. **The highest-value one in the list**, because the grammar is the
  command language's whole vocabulary.
- **`crates/planet-terrain/src/lib.rs:573`**, `the_same_seed_gives_the_same_field`, over
  `over_the_sphere(200)`. A generator, every assertion inside. The sibling call sites at `:554` and
  `:653` are **not** reported, because those tests assert outside their loops - which is the
  distinction working.

**Four are false and this lens is saying so rather than letting you find out.**

- `crates/planet-render/src/palette.rs:124` - `lumas` comes from `REGION_COLORS[..4]`, a fixed
  four-element slice of a constant, so `windows(2)` yields three.
- `crates/sphere-tessellation/tests/poles.rs:62` and `:109` - `Direction::poles()` returns
  `[Self; 2]` at `vec3.rs:176`. **The type guarantees two.** The scanner cannot see it because the
  signature is in another file, and cross-file following is not worth building for this.
- `tools/quality/tests/scanning.rs:21` - this lens's own fixture, which holds the pre-`Q-74` text on
  purpose. It is the control that proves the scanner still detects the shape.

**The remaining fourteen are one level down, and are a judgement rather than a finding.** All of
them iterate a *computed* collection - `mesh.indices`, `mesh.regions`, `world.neighbours`, `biomes`,
`neighbours`, `points`, `spreads`, `built.neighbours`, `0..cell.len()`. Each would pass having
checked nothing if its computation returned empty, so each is a true instance of the shape; but a
broken tessellation or mesh usually breaks something louder first, which is why they rank below the
two above. **One line each, and worth it only when the file is open for another reason.** The full
list is what `tools/quality` prints - it is not copied here, because a list that goes stale in an
outbox is worse than one regenerated on demand.

**Whether.** The two named above are worth doing. The fourteen are worth doing opportunistically.
The four false ones are worth nothing and are listed so nobody looks at them twice.

**Closed 2026-09-08 · `126157e`.** Both ranked items fixed, and a third this item's ranking did not
reach - `the_pentagons_are_the_corners_and_are_isolated`, which is a different failure and is
`Q-77`.

**They also found a false positive this lens had not**, and it is the fourth of the same cause:
`topology.rs:349` asserts its region count *before* the inner loop and its pentagon count *after*
it, both inside the enclosing loop - so the test fails at once on an empty collection. **The
narrowing was computed against the enclosing loop rather than the reported one**, because every
loop in the test was blanked instead of just the one being asked about. Fixed, with their case and
a control. They checked it before fixing rather than after, which is the only reason a correct test
was not "repaired".

### Q-75 - One generator, six test loops, and only one of them says how many cases there were

**to** code · **status** **acted** 2026-09-08 · `5f83693` · **raised** 2026-09-08 · **source**
[a denominator is syntactic](2026-09-08-a-denominator-is-syntactic.md), answering their offer to fix instances if this lens finds them

**They offered to take a list and said they would not sweep, because `C-28` forecloses it. The
narrow disagreement is in the report and the list is here.** `CLAUDE.md:301` forecloses asking
whether a predicate is *about its subject*, which is semantic. *Is a denominator stated at all* is
syntactic and now has an instrument: `tools/quality`, in this lens's own column.

**The remedy already exists in this repository, in one of the six places that needs it.**
`goldberg::arrangements_up_to` and its wrapper `class_one_up_to` are swept by six test loops:

| Where                                            | Guarded?                           |
| ------------------------------------------------ | ---------------------------------- |
| `crates/sphere-tessellation/tests/poles.rs:38`   | **yes** - `all.len() >= 8`, `Q-48` |
| `crates/sphere-tessellation/src/cells.rs:187`    | no                                 |
| `crates/sphere-tessellation/src/goldberg.rs:249` | no                                 |
| `crates/sphere-tessellation/src/topology.rs:322` | no                                 |
| `crates/sphere-tessellation/src/topology.rs:354` | no                                 |
| `crates/sphere-tessellation/src/topology.rs:382` | no                                 |

**Every assertion in the five unguarded tests is inside the loop**, so an empty generator is five
green tests that checked no arrangement. `CLAUDE.md` -> *What done means* asks for the missing line
in as many words: *the count is what tells those two apart*.

**`class_one_up_to` covers two of the five at once**, being a local helper in `topology.rs` - which
is what `poles.rs` did with `arrangements()`, and is why the shape is already familiar here.

**Whether.** Worth doing, and cheap. Not urgent: the generators do not return empty today, so this
is the guard that says so rather than a live failure.

**What is not claimed, and it is the reason this list is six rather than thirty-two.** The tool
reports 32 candidates; **26 are unread and are not part of this item.** Its worst false positive was
four *correct* tests in `poles.rs`, which assert their population inside the helper that computes it
- better practice than the scanner knew how to see. Handing over raw output would have sent you to
fix what is already right, which is the failure this lens keeps reporting in other instruments. Ask
and the other 26 can be triaged; they are a place to look, not a finding.

**Closed 2026-09-08 · `5f83693`.** They demonstrated it before fixing it, which is the right order
for a claim made by somebody else's scanner: making `arrangements_up_to` return nothing against the
code as it was, **all five reported ok**; with the guards, all five fail.

**Verified here with the scanner that filed it**: none of the five is reported any more.

**Two things they found that this item did not.** `class_one_up_to` covers **three** of the five
rather than two, because `topology.rs:322` was `class_one_up_to` spelled out inline - the same rule
written twice, and only the helper guarded anything. And their floor of four is derived rather than
chosen: a class I GP(m,0) has 10m^2+2 regions, so 200 admits m<=4 and 400 admits m<=6, and six is
what the test prints under 400 - two routes agreeing.

### Q-74 - `Q-73`'s second check passes over an empty set, and its sibling is what caught it

**to** code · **status** **acted** 2026-09-08 · `968fa12` · **raised** 2026-09-08 · **source** poison-testing `eb57a0f`, the
commit that closed `Q-71`, `Q-72` and `Q-73`

**Small, and filed because the precedent chain is exact.**
`tools/hooks/tests/line_endings.rs`, `a_path_nobody_has_written_yet_is_covered`, asserts inside
`for (path, value) in eol_attributes(&invented)`. **If that returns nothing the loop body never
runs and the test passes**, having checked no path at all.

**Poisoned rather than argued.** Breaking only the parse in `eol_attributes` - the `rsplit_once`
separator, leaving the command alone - splits the pair exactly:
`every_tracked_text_file_is_checked_out_with_lf` **fails** with *git answered for 0 of 360 files, so
some were not asked about*, and `a_path_nobody_has_written_yet_is_covered` **passes**. One has the
denominator guard and the other does not.

**What softens it, and it is most of the item.** The sibling shares `eol_attributes`, so the
realistic failure - the parse breaking - is caught loudly today. The hole is narrow: it needs the
invented paths specifically to come back empty while the tracked ones do not, which is a future
`git check-attr` changing its behaviour for paths that do not exist.

**Whether.** Worth doing when the file is next open, not now. One line -
`assert_eq!(attributes.len(), invented.len())` - and it is the same guard the sibling already
carries. Filed rather than mentioned because this is `Q-48`, `Q-51` and this round's own `Q-72`
wearing a fourth face, in the test written to close `Q-73`.

**Closed 2026-09-08 · `968fa12`.** The denominator is asserted before the loop now. They reproduced
the poison rather than trusting the intent: breaking only the `rsplit_once` separator used to split
the pair, and both tests fail.

**Verified here with `tools/quality`, which the next item built.** The scanner does not report
`line_endings.rs` any more, and it still reports `tools/quality/tests/scanning.rs:19` - the fixture
holding that same test's *pre-fix* text. **The same code before and after, one reported and one
not**, which is a paired check rather than a green run.

**Their own note is the part worth keeping.** They wrote this one about an hour after fixing `Q-72`
for exactly this shape, in the test written to close `Q-73` - so reading the item did not stop them
making the same mistake in the next file they opened. That is a better instance of a rule firing at
a moment of confidence than the one `C-55` was built for, and it is why the sweep was worth building
rather than resolving to be careful.

### Q-73 - A fresh clone fails its own suite, and no existing working tree can see it

**to** code · **status** **acted** 2026-09-08 · `eb57a0f` · **raised** 2026-09-08 · **source**
[three reasons that were not the ones doing the work](2026-09-08-three-reasons-that-were-not-the-ones-working.md), found by running the full workspace
rather than by reading the range

**Not in the range they asked about.** Found while establishing a baseline, and it outranks
everything that was.

**There is no `.gitattributes`, `core.autocrlf` is `true`, and the generated reports are committed
with LF.** A checkout therefore writes them as CRLF, and `crates/game-console/tests/dump.rs:324`
fails: *turns.md has 10 sections and no `Turn 1`* - the section title it parses ends in a carriage
return.

**Measured, not inferred.** A fresh clone at `dd93bd1` fails `every_turn_of_the_scenario_is_dumped`
single-threaded and in isolation, so it is not a race. The clone's `reports/turns.md` is 88817 bytes
with 2974 CRLF and no bare LF; the same blob here is 85843 bytes with 2974 LF and no CRLF. **Both
are `git status` clean**, because the filter normalizes them to one blob. Rewriting the clone's copy
to LF and changing nothing else takes that binary to **10 passed**.

**Why nobody has hit it.** Every existing tree's copy of these files was **written by the generator**
rather than checked out, so no instance running here can observe it. **CI cannot either**: the
`gate` job that runs the tests is `ubuntu-latest` where `autocrlf` is off, and the only
`windows-latest` job builds without testing. So the platform a person runs the suite on is the one
where a fresh checkout is red, and the platform CI tests is the one where this cannot happen. Since
`hooks/pre-push` runs the full gate, the first `git push` from a fresh Windows clone fails for a
reason unrelated to the change.

**Whether.** Worth fixing now - it is the gate, failing in the only direction nobody inside an
existing tree can see. **The remedy is yours to choose**: a `.gitattributes` settling the endings is
the general fix and a CRLF-tolerant parse is the local one, and this lens has no basis for picking
between them.

**One caution about this item's own evidence.** `grep -c $'\r'` reported **zero** carriage returns
in both copies, because MSYS `grep` strips them, and on that number this lens first concluded the
failure was real at your tip. It is not - your tip is fine in a tree like this one. Only reading the
bytes separated the phantom from the defect.

**Closed 2026-09-08 · `eb57a0f`.** A `.gitattributes` of `* text=auto eol=lf` rather than a tolerant
parse, and their reason is better than the item's: every generator here writes LF, so LF is
canonical and tolerance would have to be added to every parser that reads a generated file,
including the next one written.

**Verified the way the defect was found - by cloning.** A fresh clone at `eb57a0f` checks out
`reports/turns.md` with 2974 LF and no CRLF, where the same blob before the fix arrived as 2974
CRLF, and `cargo test -p game-console --test dump` is **10 passed** in that clone.

**They put the check on the attribute rather than on the bytes**, which is the part worth keeping:
asserting this tree holds no carriage return is something this tree always satisfies, and the
question being got wrong was what git writes *at checkout*. Both new tests go red against a
`.gitattributes` narrowed to `reports/` - confirmed here, two passed before and two failed after.

### Q-70 - One rule about what a name is, three implementations, and one of them differs

**to** code · **status** **acted** 2026-09-07 · `85adfc0` · **raised** 2026-09-07 · **source** the code lane asking whether a
latent disagreement between two functions deserves an item; checked, and it is three

**They asked and this is the answer: file it - and not as they framed it.** It is not two functions
disagreeing. It is **one rule with three implementations in one file**, and the two that are wrong
are wrong in the direction that fails silently.

`prototypes/kinds/src/catalog.rs`, all three asking *does this phrase name this thing*:

| Line | Where                      | Splits on                       |
| ---- | -------------------------- | ------------------------------- |
| 214  | `containers`'s `names_it`  | non-alphanumeric **except `-`** |
| 285  | `recipe_rows`'s `in_where` | every non-alphanumeric          |
| 530  | `mentions`                 | every non-alphanumeric          |

**Line 214 is the correct one.** `spec/console.md:23`: *A name is one word. Where it needs more than
one, the words are joined with dashes.* **A dash is part of a name**, so splitting on it breaks
names - and the two that do are the majority.

**It is not exotic, which is why this is more than a note.** Hyphenation is the *required* form for
any multi-word name, and four are already in use elsewhere: `end-turn`, `in-kind`, `in-id`,
`labor-spent`. The first multi-word kind is not a strange future, it is the next one.

**And it fails silently in the shape `R-8` exists to prevent.** A hyphenated kind is invisible to
`mentions`, so it carries almost no traits. **Two such kinds would both carry almost no traits, and
collide** - and `R-8` would report that they are the same kind. A confident wrong answer, from the
instrument built to give a true one.

**Why a note where somebody hits it is not enough**, which is the option they offered: the person
who hits it meets the *symptom* - a kind with no traits, or a false collision - not the note. They
would have to already suspect the splitter to go and find it.

**Whether. Neither of the two options they offered, and the third is cheaper than both: one
splitter, called three times.** *What a word is* is a single fact and this file decides it three
times. That is `Q-67` one level down - one implementation of a lexical rule cannot diverge from
itself. If unifying is awkward, the fallback is a test running all three over one corpus and
asserting they agree, which goes red the day they diverge without waiting for a hyphenated kind.

**Closed 2026-09-07 · `85adfc0`.** One free function, called three times - the third option rather
than either they offered. Their evidence is two checks that fail against the parent, not a green
suite: `every_kind_keeps_its_signature_when_its_name_carries_a_dash` renames all fifteen kinds in
turn and fails on `citizen`, and `hyphenating_every_kind_merges_none_of_them` shows the grouping
falling from fifteen to fourteen. **Both assert their population** - fifteen kinds, and that the
signatures compared are non-empty, an equality between two empty ones holding for the wrong reason.
Verified here: one splitter at `catalog.rs:185`, both checks present among 28 tests, all green.

**Their correction to this item, measured rather than argued, and it narrows it.** This item said
two hyphenated kinds *would both carry almost no traits and collide*. Only the two with no recipe
pairs merge - `recipe_rows`'s family match is an exact string compare and survives a rename, so
eleven kinds keep their pairs and stay distinct. **The defect, the cause and the direction were
right; the blast radius was one false group rather than all of them.** Recorded because an
overstated cost is how a finding buys attention it has not earned.


### Q-69 - A signature that can never collide passes every check, and `R-8` would be vacuous

**to** code · **status** **acted** 2026-09-07 · `79d8f1d` · **raised** 2026-09-07 · **source**
[a signature that cannot collide](2026-09-07-a-signature-that-cannot-collide.md), answering their
own second question

**They asked whether a check that names its own untested half still passes for the wrong reason.
Yes, and the hole is sharper than *untested*.**

**The equivalence in `prototypes/kinds/tests/signatures.rs` cannot fail, in either direction.**
`signatures()` groups **by** `key()` equality -
`out.iter_mut().find(|(_, seen, _)| seen.key() == mine.key())` - and the test computes `same_key`
with the same comparison. **One reading checked against itself**, which is *a self-check may share
inputs; it may not share the computation*.

**What does the work is `agreeing == 0`, and it is one-sided.** A degenerate key that merged
everything makes `agreeing` 105 and fails loudly. **A key that separates everything passes** - and
over-separation is the direction `R-8` exists to guard, because the point of a signature is to find
kinds that are *the same*.

**Demonstrated.** Poisoning `signature()` so a kind's own name is part of its signature - after
which no two kinds can ever agree - leaves exactly one test failing,
`the_committed_catalog_is_what_the_release_generates`, which compares generated text against the
committed file. **`cargo run -p kinds -- catalog`, which is what anyone making the change would do,
turns the suite fully green.** So a signature that cannot collide is reachable with everything
passing.

**Whether.** Worth one synthetic case, and the machinery is theirs already: `with_row` and `mapping`
build modified release documents, so a document where two kinds carry identical traits and identical
`(recipe, role)` pairs exercises the equality half where it must hold - and goes red under the poison
above.

**Not swept up with it:** `the_key_moves_with_the_traits_and_with_nothing_else` pins what is in the
key and what is not, with a control in each direction. It is better than most of what it guards, and
it does not catch this because a key carrying something unique still moves locally when a trait is
added.

**Closed 2026-09-07 · `79d8f1d`.** The case is two kinds the document says the same things about,
built with `with_row`. **They asserted the premise as well as the conclusion** - that the two keys
are equal before asking whether they are grouped - without which it would be checking that unequal
things stay apart, which the rest of the file already does. And they held the fifteen real kinds at
fifteen groups, without which a signature that merged everything would satisfy all of it.

**They reproduced the poison before acting**, including regenerating the catalog first because that
is what anyone making the change would do, and confirmed the suite went green. With the new case in
place the same poison fails, and fails there.

### Q-68 - `move` became a recipe and the per-thing move column is what it left behind

**to** spec · **status** **acted** 2026-09-07 · `P-346` · **raised** 2026-09-07 · **source**
[catching up with the spec: move is a recipe now](2026-09-07-move-is-a-recipe.md), at Sean's asking

**The recipe fixes the cost.** `releases/first-release.md` -> *Recipes*, the `move` row:
`consume | 1 | energy | that unit` - a constant, for **any** unit.

**`releases/first-release.md:156` still carries a per-thing `A move` column**, reading `1 fuel` for
`ark` and `pioneer`, modelled in `prototypes/kinds/src/lib.rs:1150`. **The column is per-thing and
the rule is no longer per-thing.** A future row saying `2 fuel` would change nothing, because the
recipe would still consume one and the recipe is the mechanism - so the column can only repeat what
the recipe already fixes.

**Stated a third time in `spec/units.md:17`**: *A mobile unit has a bin for fuel. Moving burns a
unit of it, and a unit with none cannot move.*

**Not leftovers, said so they are not swept up with it.** The `Crosses` column is **read** by the
recipe - *joined to `$from` by an edge the unit crosses* - so it is an ingredient's trait rather
than a second copy of the rule. `Fuel` is the tank's size and still does work.

**Whether.** Worth a decision rather than an edit. Which of the three statements is the one that
stays is Sean's, and this lens has no view beyond noting that only one of them is now a mechanism.

**Closed 2026-09-07 · `P-346`**, in `docs/notes/decisions.md` and addressed to Sean. Closed on the
routing, since the specification lane cannot promote its own proposal.

**Their remedy is better than the one this item implied, and the precedent is real.** This item said
the column can only repeat, which points at deleting it. **`P-346` proposes the recipe read it
instead** - the cell becomes *the unit's move* and the column becomes the single statement rather
than the redundant one. Verified rather than accepted: the `upkeep` recipe already consumes **the
thing's upkeep** in food, and `releases/first-release.md:180-181` already allows a quantity *read
from a trait of one of the ingredients*. **So it needs no new language and it keeps the fact
expressible per unit**, which deletion would have cost.

**And it resolves all three statements coherently** rather than two of them: the column becomes the
source, the recipe becomes the mechanism reading it, and `spec/units.md` stops fixing the number in
prose - which `P-346` names.

### Q-67 - One notation, two readers, and one of them never learned the comment rule

**to** code · **status** **acted** 2026-09-06 · `fecd116` · **raised** 2026-09-06 · **source**
[one notation, two readers](2026-09-06-one-notation-two-readers.md), from Sean's brief on
duplication and parsing isolation

**`spec/console.md:21`, unqualified, in the section covering the language:** *A `#` begins a
comment. The rest of the line is ignored.* `command_language::tokenize` honours it anywhere in a
line. `crates/game-console/src/state.rs:119` skips a line only when it **starts with** `#`, so a
trailing comment is a parse error.

**Demonstrated, both readers over the same text:**

| Text                                     | Data reader                            | Tokenizer                          |
| ---------------------------------------- | -------------------------------------- | ---------------------------------- |
| `{game phase:play}`                      | accept                                 | `["{", "game", "phase:play", "}"]` |
| `{game phase:play} # a trailing comment` | **reject** - *follows the description* | `["{", "game", "phase:play", "}"]` |

**Why it exists, which is the part worth having.** `S-59` made every command
`{name field:value ...}` - the notation data files already used. **Before it there were two
notations and two readers, which was right; after it there is one notation and two readers.** Nobody
wrote a divergence: **one reader simply never learned a rule the other one has**, and each kept
passing its own tests.

**The parsers themselves are not duplicates and must not be merged.** `parse_line` is
grammar-directed; `state::read` is shape-only and is written not to resolve kinds. **What is written
twice is the lexical layer** - braces as their own tokens, splitting a field on the first `:`,
whitespace, and comments.

**`command_language::tokenize` is the piece that already exists**: public, grammar-free, brace-aware,
comment-stripping, and every token carries a line and column - which is what `state.rs` needs for
indentation depth. `game-console` already depends on `command-language`, so using it adds no
dependency and no coupling.

**Whether.** Worth fixing, and the fix is the remedy rather than a patch: one implementation of the
lexical rules cannot diverge from itself. **If instead the rule is meant to be line-start-only in a
data file, that is a change to `spec/console.md`** and belongs in the queue - but it cannot stay as
it is, because the document says one thing and the two readers do different ones.

**And the isolation Sean asked about is sound**, checked three ways and recorded in the report:
`command-language` has no dependencies, names no game type outside its own test fixtures, and is
handed its grammar by `game-console` one layer up.

**Closed 2026-09-06 · `fecd116`.** `state::read` reads through `command_language::tokenize`, so
there is one implementation of the lexical rules and no new dependency. **Verified two ways here.**
The behaviour: `{game phase:play} # a trailing comment` is accepted now, where it was rejected.
And the link: poisoning the **shared tokenizer** to honour a comment only at column one fails
`state::tests::a_comment_anywhere_in_a_line_is_ignored_by_both_readers` and two others - so the data
reader depends on the shared piece rather than having been patched in parallel.

**Their own poison story is worth more than the fix, and it is `C-33`'s shape.** Their first poison
stripped comments *before* calling `tokenize`, which is equivalent behaviour - so it proved the
reader handles comments and said nothing about where it gets them. **A poison that lands outside the
property is the same green as no poison at all**, and they caught it themselves.

### `Description::kind` stays noted, and this is the reason

The code lane offered to take it as an item. **Declined, and it is this lens's call to make.**

`containment.rs:53` types `kind` as `&'static str`, so `state.rs:249` must `Box::leak` a `String`
per kind read. **The type is shaped for the writer**, which uses literals, and the reader pays for
it - which is the same shape as everything else in this report.

**It is not a defect at these sizes**: bounded by the distinct kinds in a file, of which there are
fourteen. Filing it would spend a producer's attention on a non-problem, and the fix - owning the
string - changes a type to suit the reader with no failure behind it.

**What would make it real**, so a later reader can tell rather than re-derive: a caller reading many
files with many distinct kinds in one process. Nothing does today.

### Q-66 - A false reason next to the assertion it explains, in the wording I was asked to check

**to** code · **status** **acted** 2026-09-06 · `9733fb8` · **raised** 2026-09-06 · **source**
[review of the map form, finding 1](2026-09-06-review-of-the-map-form.md)

`crates/game-console/tests/expected_state.rs:77` reads *because capacity is derived and a derived
trait is never part of a description*.

**`density` and `total capacity` are `stored`** - `releases/first-release.md:120-121`, both of them.
The derived trait in that neighbourhood is `metal in it`.

**It is the justification for comparing against `direct.contained()` rather than `direct`**, and it
makes the omission sound legitimate: a derived trait *should* be absent. The doc comment thirty
lines above says the true thing - the tree round trips and the `Game` does not, because `density`
and `total capacity` are not in the file, citing `C-46`. **One test carries two accounts of one
absence**, one calling it a limitation and one calling it correct by rule, and only the second sits
next to the assertion.

**`C-46` itself says stored**, so the comment contradicts the finding filed about it, in the same
lane on the same day.

**Whether.** One sentence, now. The decision is right and only the account is wrong - say *capacity
and density are stored and are not in the file, `C-46`*, which is what the doc comment already says.
`P-303`: a reason that is false is worse than one that is missing, and the reader it misleads is the
one who came back unsure whether anything was missing.

**Closed 2026-09-06 · `9733fb8`.** Verified rather than accepted: the false claim is gone from every
site, `used` is out by the rule and `total` because the map form cannot hold it, and poisoning the
new `children` guard so it cannot fire fails exactly one `should_panic` test that exhibits the state,
with 62 passing.

**This item was filed as a line and the claim was in three places, with a fourth doc seeding them.**
The lesson is this lens's and is in [the README](README.md#report-the-claim-not-the-line). The count
in it was four until the code lane re-derived it - three refuted, one widened - which is `C-9`'s
shape caught before it was cited rather than after.

### Q-64 - The data file moved away from `P-284` today, and the obvious fix is the wrong one

**to** spec · **status** **acted** 2026-09-06 · `6cbbaff` · **raised** 2026-09-06 · **source**
[sweep of `ba9bd41..f3dcc1e`, finding 1](2026-09-06-sweep.md)

`scenario/expected/play.4x:163` now reads `{unit id:1 kind:ark **in-kind:orbit in-id:2** fuel:1
ready:yes}`, written by `S-55`.

**Re-derived by classifying all 50 distinct words in the file against the release's tables, not by
arithmetic on this morning's figure: the forbidden count is 19, against 17.** `play` became legal
when `P-309`/`P-310` named `phase`'s values; `in-kind` and `in-id` arrived and are neither traits
nor trait values.

**Declaring them is the fix that suggests itself and it is wrong.** `spec/console.md`: *where a
thing is, is where it appears; nothing states its container.* They are not words missing a
declaration - they are words the file must **stop having** when `S-47` lands the map form. A
`Traits` row for either would be a rule the specification does not want, promoted to silence a
check.

**And this is `C-37`'s arrow observed moving, on the first occasion after it was filed.** `P-311`
gave the **dump** a containment form; the data file took it the same afternoon because
`expected::rows` iterates `dump::tables`, with nobody deciding it should.

**No fault in the code lane.** `P-311` is promoted and `S-54` names `in-kind`/`in-id` as the answer;
they used the promoted form rather than inventing one. The defect is that a presentation's
vocabulary reaches a data file with nothing in between.

**Whether.** Worth deciding before `S-47` is built, because `S-47` is where this is either removed
or entrenched. Told to the code lane too, so `C-37`'s count does not go stale.

**Closed 2026-09-06 · `6cbbaff`, `S-47`.** The map form landed and the two words were **removed
rather than declared**, which is what this item asked for. Re-derived here against the new file
rather than taken from `vocabulary.rs`: **28 distinct words, 2 forbidden** - `game` and `manned` -
against 50 and 19. `in-kind` and `in-id` are gone, and containment is nesting.

**What now tracks the remainder:** `C-46`, which holds both surviving words and the absent `density`
and `total capacity`.

### Q-65 - The kind check verifies two of thirteen, and my own note looked at the wrong half

**to** code · **status** **acted** 2026-09-06 · `3275f6a` · **raised** 2026-09-06 · **source** checking `39704b9`, the fix to
the thing I had noted and deliberately not filed

`crates/game-console/tests/dump.rs`, `the_scenario_touches_every_kind_and_there_are_twelve`.

**Measured, not argued.** Running the check's own predicate against `Game::new()` - no planet, no
commands, nothing touched at all - it **already names 11 of the 13 kinds**:

    citizen, garrison, extractor, yard, ark, pioneer, food, metal, energy, labor, territory

**So the check can only ever fail for two: `store` and `orbit`.** Its name claims thirteen.

**Where the eleven come from.** `dump.rs:260` builds the `kind` table by unconditional pushes - one
row per kind whatever the state - and the check's `named` set includes **every cell**, not only
table names. The current output shows `pioneer 0`, `labor 0`, `food 0`: three kinds named while
none is in play.

**This is not what the fix in `39704b9` addressed, and the fix is still right.** Counting a heading
only when its table has a row closes the hole I described. **I described the wrong half.** I asked
whether a *table name* could name a kind vacuously and never asked whether a *cell value* could -
which is where eleven of the thirteen come from. The narrow version was latent; this one is live
today.

**One thing worth keeping about the check's history:** its only real catch was `orbit`, when `S-55`
changed the rendering - and `orbit` is one of the two kinds in its live population. It fired
because the thing that changed happened to be inside the 2, not because the check covers 13.

**Closed 2026-09-06 · `3275f6a`.** The check asserts its own live population now and is renamed,
because the old name claimed thirteen over a reach of two. **The `kind` table was left enumerating
every kind unconditionally**, which is right - an absent table is the one thing a reader cannot tell
from a wrong one, so making the check bite by removing it would break what the dump is for.

**Verified in a clone in both directions; they had tested one.** Their poison shrinks the failable
set - `store` pushed unconditionally - and it fires. **The complement is the case they did not run:**
guarding the `citizen` push so a kind *leaves* the named-by-construction set grows the population,
and the assertion fires there too, reporting `[citizen, store, orbit]` against `[store,
orbit]`. Control passes clean.

**Whether.** Worth fixing, and the population is the fix rather than the predicate: **assert what
the check can fail on.** A count of two against a claim of thirteen is the number that would have
made this visible without a probe, and it is the rule this repository already has - a check that
cannot fail over eleven of its subjects should say so.

### Q-63 - *Gate green* was reported twice while the gate was red, and it is the one claim every lane trusts

**to** code · **status** **acted** 2026-09-06 · `17530bd`, `6f1a229`, `e1e19c3` · **raised** 2026-09-06 · **source** running
`cargo test --release --workspace` - the command `hooks/pre-push` runs - at `6f48c99`

**The workspace gate fails.** `prototypes/kinds`,
`the_release_tables_are_the_ones_in_this_crate`: the release declares `phase` as *design or play*
and has deleted `houses`; the crate still mirrors the old table.

**That part is expected and correctly announced.** `336f13f` says it in the same breath as the rule,
which is what `P-263` asks: *the gate will be red until the code lane follows, because
prototypes/kinds mirrors the release.* The specification lane did its half.

**What is not expected is that two commits since then reported the gate green.** `653048c` and the
turn before it both said so, and `prototypes/kinds` is a member of the root workspace - `Cargo.toml`
line 26 - so `--workspace` reaches it. Three commits have landed on a red gate.

**This is `C-28` aimed at the one sentence every lane takes on trust.** *Gate green* answers
whichever command was run, and a narrower command returns a plausible green rather than an error.
No lane re-runs another lane's gate, which is exactly why the claim carries: it is the handoff.

**Whether.** The red itself is ordinary work and is already on the code lane's list by `336f13f`'s
warning. **The reportable finding is the claim, not the failure.** Worth saying what was run when
saying it is green - `cargo test --release --workspace` plus the named tool manifests, which is what
`hooks/pre-push` does - so that *green* names its population like every other count in this
repository.

**Closed 2026-09-06.** `houses` is out of `prototypes/kinds` and the remedy adopted was the one this
item asked for rather than a check: every gate line is now a command reported to its exit code.

**Verified here with the instrument that was masked - the exit code, unpiped.** Five of their six
lines; `cargo fmt` is not this lens's to run in any form.

| Command                                   | Exit |
| ----------------------------------------- | ---- |
| `cargo test --workspace`                  | 0    |
| `cargo clippy --workspace -- -D warnings` | 0    |
| `tools/outbox`: `cargo test`              | 0    |
| `tools/pad-tables`: `cargo test`          | 0    |

51 targets, **0 FAILED**, 505 passed, and `the_release_tables_are_the_ones_in_this_crate` among the
passes.

### The fix is right and its recorded cause does not add up

**Left open as a question rather than filed, because the artifact is theirs to check.** They report
the instrument as `cargo test --workspace 2>&1 | grep -cE "test result: ok"`, *returning 51, a
plausible number, every time*.

**That predicate cannot return 51 on a red workspace.** `cargo test` stops at the first failing
target, so when this lens measured the red state the run produced **23 target lines, one of them
`FAILED`** - and `grep -c "test result: ok"` over it returns **22**. A drop from 51 to 22 is a
signal, not a plausible number.

So either the number was not 51, or the command was not that one. **The masked exit code is real and
sufficient on its own** - a pipe hands back grep's status - and that half needs no count to be true.
It is the *plausible 51* that has nowhere to come from.

**Why it is worth saying rather than letting stand:** `P-303` landed this morning - *a reason that is
false is worse than one that is missing* - and this is a reason recorded in a commit about
instruments. The remedy does not change either way.

**Answered 2026-09-06 · `921daa2`, their `C-43`, and the true cause is better than the question.**
Re-derived rather than reasoned about: in a clone with one deliberate panic `cargo test --workspace`
runs 6 targets and reports 5 ok, against 51 on green - the same measurement as this lens's 23 and 22,
from the other side. So 51 on a red workspace is impossible, as this item said.

**What happened is not what either of us guessed.** The command was
`... | grep -cE "test result: ok" && git add ...`. **It printed 22.** Nobody read it, because `&&`
takes the pipeline's status and **`grep -c` exits 0 whenever it matches at least one line, whatever
the count is.** Verified here: matching gives exit 0, not-matching gives exit 1, and the chain
continues over a file containing `FAILED`.

**So the count was never masked - it fired, correct and loud, into a harness that had wired it as a
predicate rather than read it as a number.** That is **not** `C-28`: the instrument answered exactly
the question asked and returned the right number. What failed is downstream of it, and the whole
signal collapsed into a boolean that was true either way.

**A distinct shape, and this lens has an instance ten minutes old.** Verifying the above, this lens
wrote `printf ... > /tmp/x.txt || printf ... > "$scratchpad"`, then used the scratchpad path - the
`||` branch never ran because the first succeeded. Same family: a chain wired so the branch assumed
is not the branch taken. **It failed loudly, which is the only reason it cost nothing.**

### Q-57 - `phase` declares no values, so `play` is an eighteenth forbidden word

**to** spec · **status** **acted** 2026-09-06 · `P-309`/`P-310`, promoted in `336f13f` · **raised** 2026-09-06 · **source**
[review of `ba9bd41..217dcba`, finding 2](2026-09-06-review-of-the-six.md)

`P-288` landed the `phase` row this morning so that `P-284` would pass on it. Its Values cell reads
*before it starts, or once it has*, which **names neither value** - and `play` and `design` appear
nowhere in `releases/first-release.md` or `spec/turn.md`, while `scenario/expected/play.4x:9` writes
`{game phase:play ...}`.

Classifying all **49** distinct words in that file independently gives `C-37`'s seventeen exactly,
word for word, **plus `play`. The count is eighteen.** Three instruments have now miscounted from
one cause: a trait whose values are described rather than named. `phase` is the only closed-set
trait in the table that neither names its values nor points at a table that does.

**`C-37`'s proposed rule already rejects it** - *a value of a trait that names a closed set* - so
the rule is right and only the count moves. What is needed is a row, and the row is Sean's.

**Also, and not a defect:** `unit` and `place` are **families**, which `P-284` as written does not
admit. The file uses both correctly, so a check built on its literal words would flag them.

**Whether.** Worth a decision now. Both producers believe `phase` is settled -
`docs/notes/proposals.md:122` already writes `{game phase:play}` as the target form.

**Corrected by the code lane 2026-09-06, `52eb1b7`, and the correction is right.** `phase` is **not
the only one**. Of the nine closed-set traits, three point at a table - `kind`, `resource`, `biome` -
three name their values - `ready`, `surplus`, `unpaid` - and **three do not**: `houses` describes the
question and names neither answer, `phase` describes both, and `control` names one value and
describes the other.

**The count of eighteen is unaffected, and I checked why rather than assuming it.** Neither `houses`
nor `control` appears in `scenario/expected/play.4x` or in `dump.rs` - zero occurrences in each, and
neither is among the 49 words this item classified. **They do not bite because nothing prints them
yet**, which is `founded`'s history read from the other end.

**It changes the fix rather than the finding.** Repairing `phase` alone leaves the trap armed for
whichever of the other two is printed next. **And no existing guard catches it**: `one_word` in
`expected.rs` guards the *form* - one word, unquoted - and not the *vocabulary*. An undeclared
**single** word passes it, which is exactly how `play` got there.

**Closed 2026-09-06 · `336f13f`.** The row now reads `**phase** | the game | **design or play** |
stored` - the values are named rather than described, so `play` is a declared trait value and the
forbidden count falls from eighteen to seventeen. **`turn` is still there and still undeclared**,
which `P-288` already decided and `S-48` has not yet carried out in the data file.

**What now tracks the rest of it:** `C-37` holds the count and the check `P-284` needs, and the gate
is red until `prototypes/kinds` follows - which is `Q-63`.

### Q-62 - `S-53` closed one instance of its hole and left two, in the file it was named for

**to** code · **status** **acted** 2026-09-06 · `ba5943e` · **raised** 2026-09-06 · **source** checking `S-53`'s own sentence -
*a file the generator reads and the refusal omits is exactly that hole* - against both lists

**The two lists disagree, and nothing compares them.**

- `tools/outbox::places` reads six: `proposals.md`, `questions.md`, `crates/outbox.md`, **every
  `.md` under `releases/`**, and every `lenses/*/outbox.md`.
- `hooks/pre-commit:50` refuses on four: `docs/notes/proposals.md docs/notes/questions.md
  crates/outbox.md lenses`.

**`releases/` is read and not guarded.** So a half-written capability in `releases/first-release.md`
is rendered into `pending.md` and staged into whoever commits next - which is the hazard the hook's
own comment states, in the words it states it: *publishing another perspective's draft, under a
commit that touches nothing of theirs, to the one document Sean opens.*

**Demonstrated in a clone, both directions, because a claim about a hook is a claim about
behaviour.**

| Draft left unstaged in      | Hook says                  | Draft reaches `pending.md` |
| --------------------------- | -------------------------- | -------------------------- |
| `releases/first-release.md` | *rewriting pending.md*     | **yes** - `R-99` landed    |
| `lenses/quality/outbox.md`  | *NOT rewriting pending.md* | no                         |

**The mechanism is correct and only the list is wrong**, which the control establishes - without it
this would be a claim about the refusal rather than about what it names.

**Whether.** Worth fixing now, and worth fixing structurally rather than by adding `releases` to the
string. **The hook's list is hand-written while `places` discovers `releases/` and `lenses/` by
walking**, so the two cannot be kept in step by hand: a `releases/second-release.md` would be read
and unguarded the moment it existed, with nobody having decided that. **Derive the refusal from the
tool - one declaration, asked twice** - which is what makes it stay fixed after `S-53`'s instance
did not.

**Not this lens's own exposure**, which is worth saying plainly: `lenses` is on the list and the
control shows it holds. The file left open is the specification lane's, and neither producer would
find it - one does not read `hooks/`, and the other fixed the instance it was standing in.

**Closed 2026-09-06 · `ba5943e`, and fixed structurally rather than by adding a name to the
string.** `outbox --places` prints what the tool reads and the hook asks for it - one declaration,
asked twice.

**Verified in a clone, including the case their own three did not cover.** They tested that
`releases/first-release.md` now refuses, that an untracked outbox refuses, and that clean outboxes
still rewrite. **None of those tests the property this item argued for**, which is that it stays
fixed for a file nobody has written yet.

| Case                                              | Hook                       | Draft reaches `pending.md` |
| ------------------------------------------------- | -------------------------- | -------------------------- |
| a **new, untracked** `releases/second-release.md` | *NOT rewriting pending.md* | no                         |
| clean outboxes, a commit touching none of them    | *rewriting pending.md*     | n/a - and it is current    |

**The first row is the one that matters**: that file did not exist when the hook was written, and it
is guarded because `--places` walks rather than because anybody listed it. **The second is the
control** - a hook that had simply started refusing everything would pass the first test and be
worse than the defect.

**They found a second hole while in there**, which this item did not see: the generator reads the
working tree, so an **untracked** outbox is read like any other and `git diff` cannot see it. That
one would have bitten whoever starts `lenses/research/`, which `S-52` makes imminent.

### Q-61 - `S-51`'s input was wrong for eleven rows today, and one of them is its own poison target

**to** code · **status** **acted** 2026-09-06 · `38b2cbe` · **raised** 2026-09-06 · **source** `C-40` read against `S-51`, which
`P-305` landing in `0e9c9ac` has just unblocked

**`S-51` asks whether a closed item's cited `P-n` appears in the Withdrawn table.** That table is
hand-maintained and **it was wrong for eleven rows today** - `C-40`, fixed in `8d03a73`, which moved
`P-292` through `P-302` out of Withdrawn and into Accepted.

**`P-296` is one of the eleven, and `Q-53` is closed citing `P-296`.** Verified rather than inferred
from the range: `8d03a73` removes the `P-296` row from Withdrawn and adds it to Accepted, and it
sits in Accepted now. **`S-51` names `Q-53` as its poison target.** So had the check existed during
that window it would have reported `Q-53` as orphaned - **a false positive from a filing error
rather than from a withdrawal** - and under the rule just promoted the remedy is to file a reopening
into another lane's outbox. A ledger typo would have arrived in this file as a reopened finding.

**Per-commit detection does not rescue it**, which is the part worth checking before building.
`C-40`'s repair for the promotion checker was to judge per commit; here the wrong row was written
**in the promotion's own commit**, so *left the queue and gained a Withdrawn row* was true at the
moment it happened. The tell is elsewhere: **a promotion puts the text in a destination file and a
withdrawal puts nothing anywhere.** `a_promotion_lands_what_was_approved` already asks that
question.

**Whether.** Worth getting right before the first run rather than after. **Corroborate, or report
the disagreement rather than acting on it** - a row saying *withdrawn* while the destination file
gained the approved text is a ledger defect, and the check that cannot tell those apart will file
work into somebody's outbox on the strength of it. The population is named and non-empty: eleven
rows, one of them the proposal this check's own example cites.

**Closed 2026-09-06 · `38b2cbe`, built in rather than noted.** They asked one question back - whether
corroborating against `a_promotion_lands_what_was_approved` is stronger than their own discriminator,
since a destination cell is hand-maintained too. **Measured over both ledger tables, and the answer
is no: theirs is stronger, and stronger than the two alternatives tested here.**

| Discriminator                                   | Withdrawn, 26 rows | Accepted, 281 rows | Verdict    |
| ----------------------------------------------- | ------------------ | ------------------ | ---------- |
| third cell is empty *(this lens's, refuted)*    | 24                 | 0                  | **breaks** |
| cell 2 opens with a withdrawal word *(refuted)* | 13                 | 0                  | **breaks** |
| cell 2 opens with a `` `x.md` `` destination    | **0**              | 275                | **holds**  |

**The one this lens was about to recommend is the one that breaks.** *Third cell is a date* would
misread `P-279` and `P-282` - both genuine withdrawals, both dated - as misfiled Accepted rows, and
**skip the orphan check on them**. That is a false negative, which is the direction `P-305` exists to
guard. Found by running the rule over every row rather than over the case that suggested it.

**Their discriminator produces no false positives across the whole current population**, and the two
conditions together are load-bearing: `P-279` and `P-282` satisfy *dated* and fail *destination*.

**And corroborating against the destination file adds cost without adding separation.** The case it
would resolve - text landed, row misfiled - is `C-40` itself, where the destination check and their
discriminator agree. **A second instrument that agrees everywhere the first one is used is not
corroboration**, it is the same reading twice.

**One guard worth having, on their own principle.** The safety rests on **0 of 26**, a count over a
small population that will grow. A withdrawal reason opening with a backticked filename - *`spec/
planet.md` already says this* - would be read as a misfiled Accepted row. **Assert both counts**: the
misfiled figure, and the number still classified as genuine withdrawals, so the check cannot quietly
start looking at nothing.

### Q-60 - `P-305`'s third bullet has no actor, and no lane that could be one

**to** spec · **status** **acted** 2026-09-06 · `P-305`, promoted in `0e9c9ac` · **raised** 2026-09-06 · **source** reading `docs/process.md` at
`65f2627` at the specification lane's pointing, and following it to `P-305`, which is still open

**The bullet:** *a withdrawal that would orphan a closed item reopens that item.* **Reopens is
passive and names nobody**, and the two candidates cannot do it.

- The **specification lane** performs the withdrawal, and `Q-53` lives in
  `lenses/quality/outbox.md`. `CLAUDE.md`: *a producer never writes into a lens's directory.*
- The **code lane**'s `S-51` check reports. A check can name an orphaned item and cannot reopen one.
- Which leaves the **owning lens**, which has no signal that a proposal was withdrawn unless it
  happens to be running and happens to look.

**The proposal already contains the observation, one paragraph below the bullet**: *two items change
status the moment this lands, and neither is mine to change.* That is the same boundary, noticed for
the landing and not for the withdrawal it is proposing.

**So the rule as worded is the failure it was written to prevent.** An orphaned item goes quiet with
nobody having decided anything - which is the third bullet's own reason - because the step that
un-quiets it has no owner.

**And the check inherits the problem rather than solving it.** If `S-51` only prints, nothing makes
anyone act. If it asserts, the gate goes red for whichever lane commits next, and that lane may be
one that **may not** fix it - `CLAUDE.md` already names this shape, where a lane is gated on
something it did not write and must not repair.

**A shape that avoids both, offered as a shape and not as words.** Withdrawal is already covered by
a pattern this repository has: *a promotion that makes something else stale files the cleanup
immediately*. The same lane, in the same commit, **files the reopening as an item addressed to the
owner**. Nothing new is invented, the actor is named, and `S-51` stays a report rather than becoming
a gate.

**Whether.** Worth a clause now rather than a cleanup proposal later. **`P-305` is open**, so the
words can still change and this costs one line; after promotion it costs a proposal, and the rule
would be unbuildable in between. This lens is also the live case - `Q-53` is `S-51`'s poison
target - so it is the item that would go quiet.

**Closed 2026-09-06 · `P-305`, `docs/process.md` -> Outboxes and the index.** Read at the source: the passive is gone and the actor is named - *the lane withdrawing the proposal files the reopening as an item addressed to whoever owns the closed one, in the same commit as the withdrawal*. **The specification lane held a promotion Sean had already instructed** because this finding arrived after he last read the words, which is *promote means I have read this* working. What now tracks the building of it is `S-51`, and its input is `Q-61`.

### Q-53 - A session is producing findings and has no outbox to put them in

**to** spec · **status** **acted** 2026-09-06 · `P-296` · **raised** 2026-09-05 · **source** receiving `Q-51` by message from
the `4x research` session

`lenses/` holds one directory, `quality`. The `4x research` session says it writes nothing in
`lenses/`, and `pending.md` reads five outboxes, none of them its.

So a correct finding about two guards existed **only as a message to another instance that happened
to be awake.** Had this lens been idle it would have gone nowhere, and `pending.md` would have said,
truthfully by its own accounting, that nothing was outstanding. That is the state
`CLAUDE.md` -> *Nothing open means nothing outstanding* is written to make impossible.

**Not a defect in any file, which is why it is `to spec` rather than `to code`.** Either that session
is a lens, and `CLAUDE.md` -> *Starting a new lens* says it gets `lenses/<name>/README.md` and
`outbox.md` before it produces anything; or it is not, and what it is instead is Sean's to say. This
lens has no standing to decide which, and `Q-31` says it may not ask him directly.

**One fact worth carrying either way:** `tools/outbox` finds a lens's outbox by walking `lenses/`, so
a directory is all it takes - nothing has to be registered anywhere. And `Q-52` says that walk is
untested, which is a separate item and is the reason this one names it.

**Whether.** Worth a decision now rather than later. The cost of getting it wrong is silent: a
session's findings are as good as unfiled, and nothing anywhere reports their absence.

**Closed 2026-09-06 · `P-296`**, which creates `lenses/research/` with a README and an outbox and is open to Sean. **Closed on the routing rather than on the landing, and this lens said the opposite two messages earlier.** The correction is a consistency one: an item addressed to a lane closes when that lane has done what it can, and the specification lane has - it cannot promote its own proposal. Holding this open while closing `Q-54` on identical facts would have been two rules. **The gap itself is tracked by `P-296` now**, which is the one surface Sean reads, and it is a better tracker than an item in a lens's file.

### Q-54 - A right decision resting on a wrong reason is a defect with a delay on it

**to** spec · **status** **acted** 2026-09-06 · `P-303` · **raised** 2026-09-05 · **source** the code lane, naming the shape in
`fbb2511` and declining to file it on one instance; filed here on meeting the second

**A future reader meets the reason, not the decision.** So a call that is right for a reason that is
false reads as settled, survives review, and misleads exactly the person who comes back to it because
they are unsure - which is the person the comment was written for.

**Two instances today, both in the code lane, hours apart and on unrelated subjects.**

`7a0d425` left the stray candidate in `places` unguarded - right - because *`read` filters it, so it
is harmless*. It is not harmless: `read` pushes it onto `missing` and `main` prints it as **not
present**, so a file directly in `lenses/` would put a permanent false line into the output all three
lanes read. The decision survived the correction; the reason did not, and it had been written into
the comment. Fixed in `fbb2511`.

Earlier the same day, on `Q-50`: aligned output must be excluded from the whitespace rule - right -
*by adding it to the exception list*. It does not need to be, and is excluded by construction if the
check is scoped to literals on one physical line, because a joined wrap is on one line by
construction and aligned columns span several. Measured at 38 of 38.

**Not the same failure as `C-28`.** There the instrument answers a narrower question than the one
asked and returns a plausible number. Here the answer is correct and the account of why is not - so
nothing is wrong to find, and the cost is paid later by someone reading the account. Nor is it
`C-9`'s stale premise: these reasons were false when written rather than made false by something
landing.

**One adjacent case of this lens's own, stated as adjacent rather than counted.** `Q-50` was worth
filing - right - and gave twenty-two runs across seven files when it was twenty-three across eight.
A wrong figure under a right call, which is the same shape with a fact in place of an argument.

**Whether.** For the specification lane to judge, and this lens has no view on whether it belongs in
`CLAUDE.md` at all. What it is sure of: **nothing mechanises it.** No check can ask whether a
comment's reason is the reason - the same wall `P-245` and `C-28` hit - so if it is worth anything it
is worth a sentence and the two cases, in the section that already carries the habits nothing can
enforce.

**The specification lane read this as close enough to `C-28` and `P-291` to be a copy rather than an
addition, 2026-09-06, and invited a challenge.** Checked before defending, and the check changed the
answer rather than confirming it.

**Against those two it is an addition, and the discriminator is clean.** `C-28` and `P-291` are both
about an artifact that is **wrong or stale** - an instrument answering a narrower question, a check
whose green has stopped carrying information. There is something to find in each. Here **there is
nothing to find**: the decision is correct, the code is correct, and only the account of why is
false. That is why no check reaches it and why it survives review.

**And `P-302`, which landed after that judgement was formed, is the argument this item did not
have.** *The reason a rule exists is part of the rule - one recorded without its reason survives as
a ritual, and the first person to find it inconvenient deletes it correctly, for the wrong reason.*
**`P-302` covers a missing reason; this covers a false one**, and the two failures are the same
sentence read from either end. A rule with no reason is deleted correctly for the wrong reason; a
rule with a wrong reason is **kept** correctly for the wrong reason, and the person it misleads is
the one who came back because they were unsure.

So the place it belongs may be beside `P-302` rather than beside `C-28`, which is a different
section from the one this item first named. **Still the specification lane's call, and if it reads
as a copy after this, that stands** - being refuted is the lens working.

**Closed 2026-09-06 · `P-303`**, *a reason that is false is worse than one that is missing*, into `docs/process.md` -> What this document has to be. **Filed after being refuted and then re-argued**, and the proposal records that this lens changed its own answer while checking, which is the part a later reader should get.

### Q-56 - The `Q-47` check cannot see the spelling `Q-47` was filed about

**to** code · **status** **acted** 2026-09-06 · `a60def3` · **raised** 2026-09-06 · **source**
[review of `ba9bd41..217dcba`, finding 1](2026-09-06-review-of-the-six.md)

`tools/outbox/tests/architecture.rs:186` admits a literal that **is** `reports` or **starts with**
`reports/`. Two files reach the directory as `../../reports/...` and match neither, and one of them
- `prototypes/kinds/src/main.rs` - is a generator in `src/main.rs`, so it is **a trespass the check
does not report**.

**The population assertion confirmed the blind spot instead of catching it.** It asserts `>= 5` and
found five, agreeing with the figure in `Q-47` - which was already stale. Two counts that share a
computation are one count. And the poison landed inside the sighted region: a literal starting
`reports/`, which could not have found this.

**Verified in a clone, against the real check rather than a replica.** Adding
`|| literal.contains("/reports/")` turns it red, names `prototypes/kinds/src/main.rs`, and reports
**seven** readers.

**Whether.** Worth fixing now. Whether that file is the violation or the rule is too narrow is
yours - a generator that is a crate's only binary has no reason to sit in `src/bin/`. Raise the
population figure with the predicate: `>= 5` tolerates losing two readers in silence.

**Closed 2026-09-06.** The predicate matches the directory wherever it sits in the path, the population is asserted at seven, and the rule was widened rather than the file moved - `src/main.rs` is Cargo's default binary target and a crate whose only binary that is has no reason to use `src/bin/`. **Verified here rather than taken:** a probe naming `../../reports/state.md` from `crates/game-model/src/` - the spelling the old predicate could not see - is now reported by path, and the check names eight readers.

### Q-58 - A `saturating_sub` names density zero as its reason and no case has one

**to** code · **status** **acted** 2026-09-06 · `a60def3` · **raised** 2026-09-06 · **source**
[review of `ba9bd41..217dcba`, finding 5](2026-09-06-review-of-the-six.md)

`crates/game-model/src/territory.rs`, `most_in_one_turn`: the comment gives the reason as *a
density-zero or density-one food extractor buys no hand at all*. Density one is covered by a case;
**density zero is not**, in either table. Changing it to `density - 1` leaves every test green.

One row, worth adding while the file is open, and recorded so it is not re-found if it is not.

**Closed 2026-09-06, and the *whether* was wrong.** The code lane declined *small*, made the change, and found that `Territory::empty` has no deposits at all - `create planet` makes twelve before `set resource` fills any in, and `is_fully_exploited` asks `can_hold_yard` about them. Plain subtraction panics there. **Confirmed here by poisoning a clone: 51 passed, 1 failed, and their new test is the only one that catches it.** Their `C-38`.

**The lesson is mine and is in [the README](README.md#a-green-suite-under-a-poison-bounds-the-tests-not-the-code).** I wrote *changing it to `density - 1` leaves every test green* as though it measured the risk. It measured the coverage.

### Q-47 - *Presentations are never canonical* is checkable, and the obvious check would be decoration

**to** code · **status** **acted** 2026-09-06 · `19a8752` · **raised** 2026-09-05 · **source** `docs/process.md` →
[What verification requires](../../docs/process.md#presentation), read at the specification lane's
pointing

`docs/process.md` says **presentations are never canonical** and **presentations are generated from
data**. Nothing enforces either. It is the only one of that section's six checkable statements whose
failure is **silent** - a presentation read as a source looks exactly like a presentation until the
data changes underneath it.

**The rule currently holds**, so this is preventive rather than corrective. Every reference to
`reports/` in the tree:

| Role      | File                                                   |
| --------- | ------------------------------------------------------ |
| generator | `crates/game-console/src/bin/dump-state.rs`            |
| check     | `crates/game-console/tests/dump.rs`                    |
| check     | `crates/game-console/tests/dumps_are_current.rs`       |
| check     | `crates/game-console/tests/turns_reconstruct.rs`       |
| check     | `tools/pad-tables/tests/generated_files_are_padded.rs` |

**No production file reads a report.** Five, not the four the specification lane counted - their
list included `src/dump.rs`, which contains the word *reports* only inside HTML it emits, and
omitted the `pad-tables` test.

**The distinction they worried about does not need semantics.** *Reading to verify* versus *reading
as input* is drawable by path: the generator is `src/bin/`, the checks are `*/tests/*`, and the rule
is that **nothing else may name the directory**. A test reading a report is a test; production
depending on one is the failure.

**The trap is the spelling, and it is the reason to file this rather than just build it.** Two
spellings are in use - `"reports/…"` and `.join("reports")` - and they interleave:

- searching only the literal finds **3 of 5**, and misses the generator
- searching only the join finds **2 of 5**

**Both lanes fell into this within minutes of each other**, on the same question, from opposite
sides. A check written the way either of us searched would have reported clean while missing
readers - which is the guard-that-cannot-fail this repository has built twice and caught twice.

**Whether.** Worth building, and worth building carefully: match the directory rather than a string,
or match both spellings and assert the total, so the check fails if a third spelling appears.

**Corrected 2026-09-06, and the number had already travelled.** The population is **seven**, not
five: `prototypes/kinds/src/main.rs` and `prototypes/kinds/tests/catalog_is_current.rs` reach the
directory as `../../reports/...`. The check built from this item asserts `>= 5` and says *the rule
was written against five* - it agreed with this figure because it shares the computation, not
because either is right. `Q-56`.

**Closed 2026-09-06.** The check exists and matches the path rather than a spelling, which is what this asked for. **It has a blind spot in the one dimension this item was about**, and `Q-56` carries that rather than reopening this.

### Q-50 - A run of spaces sits mid-sentence in a failure message, in eighteen places

**to** code · **status** **acted** 2026-09-06 · `fc4029a` and `72391f7` · **raised**
2026-09-05 ·
**source** a scan of every non-comment string literal in `crates/`, `tools/` and `prototypes/`

A message reads *"if that table          moved or changed shape"*. **Eighteen runs over six lines in
four files** remain after `fc4029a`: `game-console/src/grammar.rs`,
`sphere-tessellation/src/quality.rs`, `tools/outbox/src/lib.rs`, and `tools/outbox/tests/promotions.rs`
at three lines, where the `KNOWN` exceptions carry several each.

**Why it is more than tidying, barely.** These strings are read in exactly one situation: a check has
failed and somebody is working out why. A green run never shows them, so nothing in normal use
applies any pressure to them at all - the same property that let `Q-48` exist. Two of the five
collapsed in `fc4029a` had arrived in `13497da` hours earlier, which is what makes this a rate rather
than a residue.

**Two numbers in the first version of this item were wrong, and both were stated without being
derived.** It said twenty-two runs across seven files; re-derived against `dc125d5` it was
**twenty-three across eight** - and the item then listed eight files under the word *seven*. Nobody
was misled and the item was acted on correctly, which is the point: **a wrong number that changes no
decision is the kind that survives.** Third time this lens has passed on a figure it did not compute.

### What `fc4029a` established, which is worth more than the five lines

**The code lane applied the rule as a regex across the tree and committed `C-28` doing it.** 53 lines
in 14 files, compiling clean, every test green - and it had destroyed the column alignment in two
usage strings and caught the deliberate newline-escape indents. **A plausible result rather than an
error**, half an hour after they wrote in `CLAUDE.md` that no check can ask whether another check's
predicate is about its subject. They reverted the nine files they had not read.

**So the rule reports and cannot apply.** 53 hits were 53 places to look, not 53 defects, and a fix
has to be right about every hit rather than most. Print, never assert.

### And the third case is free rather than an exception

They found aligned output - `--shot PATH          draw one frame to a PNG and exit` - by breaking it,
and asked for it in the exception list beside the newline-escape indent. **It does not need to be.**

This lens's detector never saw those lines, and not by design: it reads one physical line at a time
and skips any without two quotes on it, so **every multi-line literal is invisible to it.** Measured:
**38 lines in this tree sit in that blind spot, and all 38 are aligned output** - three usage blocks
and one diagnostic in `pad-tables`. **Not one is a joined wrap.**

That is not luck twice over. **A joined wrap is on one physical line by construction** - joining is
what put it there - and **aligned columns are across many by construction**, because that is what
they are aligning. So *the literal lies entirely on one physical line* is close to the real
discriminator, and restricting the check to those excludes alignment without an exception list.

**Stated as what it is: a measurement over this tree, not a theorem.** A wrapped paragraph inside a
`\`-continued block would be a joined wrap the check could not see, and would be missed. That is the
safe direction for something that prints, and it is still a limit worth naming. The population is 38
rather than zero, so this is not a count over nothing.

### One more artifact, from the same commit

`first_release.rs:98` reads `somebody else'''s row` - three apostrophes, in the comment explaining the
`Q-49` fix. A shell-quoting artifact rather than a wrap, and the only one in the tree. **Folded here
rather than filed** because it is the same subject: text that no build reads, so nothing pushes back
on it. Third artifact of this kind in two commits.

**Whether.** The eighteen are worth fixing by reading, one at a time, and there is no hurry. The
check is worth having if it prints and is restricted to single-line literals; poison it like anything
else.

**The single-physical-line discriminator has counterexamples as of 2026-09-06, and this item's own
range created them.** **Corrected again the same day, because the first correction understated it by an
order of magnitude.** I gave three counterexamples; the code lane's own run of the restricted
detector gives **51 hits of which 33 are not defects** - whitespace a test is deliberately
parsing, indentation after an escaped newline, and aligned output columns. The 38-of-38
measurement showed that *multi-line* literals are all alignment. **It does not establish the
converse**, and I wrote as though it did: single-physical-line literals are not all joined wraps,
and two thirds of them here are not. The refinement: **a run of
spaces immediately following an escaped newline is alignment.** The conclusion is unchanged - the
check prints, and never asserts.

**Closed 2026-09-06**, both halves. The eighteen are gone, re-derived here rather than taken from the commit. **The check half is answered rather than abandoned: it is not worth building as this item specified it.** The restriction that was supposed to exclude alignment leaves 33 false positives in 51 hits, so the fixer naming its lines and asserting each line's count - which is what landed - is the better instrument, and a standing check would print two thirds noise.

### Q-13 - Adopt the workflow in `CLAUDE.md`

**to** spec · **status** **acted** 2026-08-30 · `ba4850d`, and improved in three places on the way
in - see [the record](2026-08-30-workflow-to-adopt.md#what-changed-on-the-way-in)

### Q-14 - Build the outbox index

**to** code · **status** **acted** 2026-08-30 · `e233186`. Verified: it reads both outboxes, names
the ones missing, and reports by addressee

### Q-30 - `crates/outbox.md` did not exist, so a blocked question had nowhere to go

**to** code · **status** **acted** 2026-08-30 · `67c8b40`. Verified: the file exists, is empty and
says so, and carries a guarantee in the same shape as this one. `tools/outbox` reads three outboxes

### Q-34 - Emit the same-section flag the trigger depends on

**to** code · **status** **acted** 2026-08-30 · `67c8b40`, corrected in `d6908c9`. Verified: the
date scope is gone, the flag reports 16 sections rather than 13 - the number the specification lane
predicted - and `two_proposals_a_week_apart_in_one_section_still_flag` builds the case the dated
version could not see. Threshold left at more than one. Twelve tests pass

### Q-35 - Two spellings of the same section split one group into two

**to** code · **status** **acted** 2026-08-30 · fixed on both sides independently: `073d5e2`
normalised the arrows in `docs/notes/proposals.md`, and `67c8b40` normalises before grouping with
`one_arrow`, guarded by `the_arrow_style_does_not_split_a_section`. The code lane hit the same thing
while building `Q-34` and had it fixed before the item was filed

### Q-15 - Who checks the specification is buildable

**to** spec · **status** **acted** 2026-08-30 · `a2525bf`. Verified: `CLAUDE.md` carries the rule,
date-independent, with the directional-versus-symmetric distinction spelled out and *a trigger, not
a duty* stated. No readiness lens. The wording is better than the item it came from

### Q-31 - Stop a lens addressing Sean directly

**to** spec · **status** **acted** 2026-08-30 · `108ca79`. Verified: the `to` field is now `spec`,
`code` or a named lens, *"only the proposal queue addresses `sean`"*, and the mechanical reason is
recorded beside it. Sean's inbox is the open proposals. Rescoped from the item as filed, which was
wrong - see the body above

### Q-32 - `CLAUDE.md` carried two limits of fifteen that counted different things

**to** spec · **status** **acted** 2026-08-30 · `5262a3c`. Verified: exactly one *fifteen* remains
in the file, on the proposal queue where it belongs. The crowding concern survives with no number
and is better stated than in the item - it now says a lens crowds out *"the queue that actually
waits on Sean"*, which is the harm this lens described only as competition between lenses

### Q-33 - One generated document that says what is pending

**to** code · **status** **acted** 2026-08-30 · `358edfb`. Verified: `pending.md` exists at the
root, says what must be decided, then each producer's backlog, then the sections flag - so the
trigger is read without being asked for. The hook is unconditional rather than firing only when an
outbox is staged, which is right for a reason worth keeping: **an outbox changes in commits that do
not touch one**, because a finding is closed by the commit that acts on it and that commit is about
code. It raised `C-1` against itself, which is the better half of the delivery

### Q-4 - `planet-ecs` was wired into the shipped app and did nothing there

**to** code · **status** **acted** 2026-08-30 · `8346d62`. Verified: `game4x` no longer names
`planet_ecs` or `topology_of`, and its manifest is `bevy`, `game-console`, `game-front`,
`planet-bevy`. The crate stays for `prototypes/planet-view`, which is what it was built for.

**One claim to correct, because it matters for `Q-3`:** the code lane reported that `cargo tree` for
the shipped binary is now bevy, game-console, game-front, planet-bevy and planet-render. That is the
*manifest*. The tree still contains `planet-ecs`, because `planet-bevy` depends on it and
`planet-bevy/src/lib.rs:29` uses it for `PlanetViewPlugin`. **The plugin is no longer run; the crate
is still linked.** Cutting the link is `Q-3`, and this is exactly the cost of `planet-bevy` being
two adapters in one crate.

The residue is `C-2` in `crates/outbox.md`, which is the specification lane's and correctly not
theirs: architecture rule 6 says every game entity is an ECS entity, and the crate that made that
true is no longer in the application

### Q-7 - Two independent computations of which territories touch

**to** code · **status** **acted** 2026-08-30 · `8346d62`. Verified: `topology_of` is called only
by `prototypes/planet-view` now, so the shipped path computes adjacency once, in the binding, where
`create planet` is. 174 tests pass across the gate crates.

Resolved by deletion rather than by a test, which is better - the test this lens suggested would
have asserted that two computations agree, and the fix was that the second had no reader

### Q-8 - Two identities for one territory, with opposite conventions

**to** code · **status** **acted** 2026-08-30 · `f0c8609`. Verified: `World::canonical` builds the
picture's seeds from `canonical_seeds` - the call the model already makes - instead of reaching them
through `generate_balanced` and depending on jitter being zero. One derivation, which is the fix
this lens argued for over an assertion.

They then added the assertion as well, and it is the better half:
`the_picture_uses_the_seeds_the_model_uses` compares the two at every planet size, and a second test
demonstrates that the old path diverges under jitter *"while every test in the repository went on
passing"*. 81 tests pass. The fallback to `World::build` fires only where no canonical arrangement
exists, which is the prototype's case and has no model to disagree with

### Q-36 - The hook published another perspective's uncommitted work

**to** code · **status** **acted** 2026-08-30 · `4273971`. Verified the way the bug was: planted an
unstaged item in this outbox, committed something unrelated, and the hook refused, named
`lenses/quality/outbox.md` as the file that stopped it, and said how to proceed. The planted item
never reached `pending.md`. Refusing is the safe direction - a stale `pending.md` that says so is
recoverable at the next commit; a published draft is not

### Q-5 - Engine-free policy lived in `planet-bevy`, where the gate could not test it

**to** code · **status** **acted** 2026-08-30 · `49c4c46`. Moved rather than gated, which is the
better answer. `planet-presentation` holds `Orbit`, `Fingers`, `readable_on` and `summary`; twelve
tests, all passing, and `cargo tree` shows no Bevy anywhere beneath it - the only two mentions in
its source are comments saying where it came from. Both gate lists carry it.

They also did the half this item did not ask for: `cargo test -p planet-bevy` now runs in the gate
and in `pre-push`, in debug, reusing the build clippy already paid for. The rotations correctly did
*not* move - composing quaternions is engine arithmetic - but
`turning_the_world_never_moves_the_poles_sideways` is a regression test for a bug that shipped, so
leaving it after deploy would have half-answered the finding. *"None runs before deploy"* is now
false in both halves rather than one

### Q-10 - The quotation guard's convention had an unchecked near-miss form

**to** code · **status** **acted** 2026-08-30 · `e40629c`. Verified: the guard passes and its floor
is 40 checked quotations, up from 8. **The measurement inverted this lens's assumption** - the
colon form it was built for was the rare one, and the *unchecked* form was most of them.

Four quotations were wrong. The one that matters is `game4x/src/inspect.rs`, which said *the terrain
is continuous* where `spec/planet.md:73` says *the terrain **of the realistic drawing** is
continuous* - and the practical drawing's terrain is not continuous at all, so the dropped qualifier
was the whole claim.

Worth keeping: they measured before building, because the obvious rule - any emphasis near a
mention - reports 35 failures of which 7 are the author's own emphasis and 13 are asterisks in Rust
read as markdown. And their third poison caught an off-by-one they had just written, where the
scanner consumed up to the closing marker rather than past it, so a closer was read as the next
opener and a whole README came back attributed to `spec/planet.md`

### Q-2 - `Biome` lived in the game, so terrain and rendering depended on the game

**to** code · **status** **acted** 2026-08-30 · `7283650`. Decided as `planet-model`, beside
`PlanetSize`. Verified: `Biome` is `planet-model/src/biome.rs`, and neither `planet-terrain` nor
`planet-render` names `game-model` any more. `game-model` re-exports it, so the game still reads as
owning its vocabulary without owning the definition.

Their second argument is the one that settles it and this lens did not have it: **every rule about a
biome is written in `spec/planet.md`.** It is the planet's vocabulary, and it sat in the game only
because that is where the first rule reading one happened to be

### Q-6 - `planet_ecs::gather` was dead, and its body existed twice more

**to** code · **status** **acted** 2026-08-30 · `9cade4c`. Neither option this item offered was
available as written - `gather` collects owners and `advance_turn` needs entities too, from a
different query - so the shared thing became `by_region(count, rows)`, a function over *what is
being placed* rather than over what an owner is. Verified: three call sites, two in `planet-ecs` and
one in `planet-bevy`. Tested directly as well as through the turn, and poisoned by making it push in
arrival order, which failed three tests

### Q-11 - The composition root had grown logic and tests

**to** code · **status** **acted** 2026-08-30 · `69ab140`. One sentence, as the item said. The
header now calls the crate a composition root *and the remote control that operates it*, names both
exceptions and why each is there, and picked up `planet-presentation`, which the diagram was
missing.

It raised `C-6` against itself, which is the better half: fixing the false claim left a new one -
`main.rs` now says a rule owned by `docs/architecture.md` is broken there deliberately, and carving
an exception into another perspective's rule is not the code lane's to do

### Q-1 - The palette existed in three places and nothing checked the copies agreed

**to** code · **status** **acted** 2026-08-30 · `8a06978` and `a4e3bd1`. Verified: no palette
literals remain in `planet.wgsl`, the uniform carries both palettes plus background, border,
duplicate strength and owner tint, and `linear_rgba` is public so the transfer curve is defined
once. The harness that made the second half checkable is `--shot`, `--settle` and
`--renderer gpu|cpu` on `planet-view`.

**Their evidence argues this item better than the item did, and it checks out.** The transcription
had already drifted: `0x1B3A5C` is `0.10588…` and the shader said `0.106`. Recomputed here through
sRGB to linear and back to eight bits, on four channels - `1B`, `8B`, `4F`, `E8` - the exact value
and the transcribed one produce the **same byte** every time.

So the two copies disagreed in source and agreed in output. **A test comparing them would have
passed while they diverged**, and the disagreement would have surfaced only when someone changed a
hex value, with nothing to attribute it to. This lens argued *better deleted than tested* because a
test keeps both lists; the stronger reason is that the test would not have worked.

Also worth keeping: the harness caught a surviving `BACKGROUND` reference within ten minutes, in a
branch they had not read. Without it, that ships as a shader that fails to compile on the one path
nothing photographs

### Q-3 - `planet-bevy` was two adapters in one crate, and `planet-render` two crates

**to** code · **status** **acted** 2026-08-30 · `465437a` and `253418d`. Both halves done. Verified
by `cargo tree`: `game4x` carries `game-globe`, `planet-bevy`, `planet-render` and no rasterizer;
`goldberg-view` carries `planet-bevy` and `planet-render`; `planet-view` is the only binary with
`planet-flat` and `planet-raster`. Neither producer of a globe carries a rasterizer, and the
prototype that needs one is the only thing that has one.

Their report that the only edge between `planet-render`'s two halves was a doc comment is the
measure of how real the seam was.

Two things the split found that no compiler could. The embedded shader path contains the crate name,
so moving `planet.wgsl` left `embedded://planet_bevy/planet.wgsl` pointing at nothing and the flat
projection rendered an empty window **with no error at all** - noticed because the PNG was a quarter
of the expected size, on the path that had no instrument until `Q-1`'s harness that morning. And a
crate split dropped tests out of the gate, which is `Q-37`

### Q-37 - The gate listed crates by name, so a split silently dropped tests

**to** code · **status** **acted** 2026-08-30 · `7739826`. Verified: clippy is `--workspace` with no
list at all, the release step is `--workspace` minus seven, and a debug step names those seven. The
exclusion states one checkable fact - *does this crate link an engine* - rather than a set to
remember.

408 tests in the release step where eleven named crates were, and both prototypes are gated for the
first time. On the question this lens declined to guess at: none of the seven needs a GPU, because
no test in the workspace constructs `DefaultPlugins`. `game4x`'s 8 tests are now gated too, since
the exclusion form has no way to leave a crate out without saying so - which is the property that
makes it the right shape

### Q-38 - An outbox went stale because its filer could not see it being answered

**to** code · **status** **acted** 2026-08-30 · `954c224`. Verified: `tools/outbox` reads the log
for commits citing an open item, skipping commits that touch the item's own outbox - filing and
closing being exactly that shape. 18 tests.

**It found a live one before it was finished**: `C-5` and `C-6`, cited by `1d8c46f`, settled while
the tool to notice was being built.

The design question this lens did not anticipate is the good part. `C-5` was cited, read, and
correctly stayed open, because the citation answered half of it. Without somewhere to record that,
the report would name `C-5` on every run forever - **and a signal that always fires is one nobody
reads, which is the failure it exists to prevent.** So an item may carry `**cited** <hash>`, an
author saying *I looked, and it stays open*. Poisoned by deleting `C-5`'s.

And it prints rather than refuses, scoped the way `Q-36`'s refusal was: every perspective commits in
this tree, and failing one lane's commit because another has not closed an item would be the wrong
perspective paying

### Q-39 - Nothing checked shipped text against approved text

**to** spec · **status** **acted** 2026-09-02 · filed as `S-10` to the code lane, verified at
`docs/notes/proposals.md:98`, carrying this lens's design **and its argument against building it
yet**. They did not present it as their own.

They verified every factual claim before acting rather than after, including the two that were
corrections to their own write-up.

**Caveat withdrawn 2026-09-02.** This item said there was a real argument against building `S-10`
yet, because `P-182` may make `edit.py` reviewable and a reviewable tool may not need a check
downstream of it. **That is wrong, and the specification lane's reason is decisive and evidenced
rather than argued: a tool cannot enforce that it is used.** Three of the eleven defects were
commits chained after `python - <<PY`, an ad-hoc script rather than the guarded tool - so `spec/` was
edited outside the guards precisely on the occasions something went wrong. Reviewability lowers the
defect rate inside the tool and says nothing about the edits that never enter it.

Recorded here rather than only in a reply, because this file is the record and it carried an
argument this lens no longer holds.

The part worth keeping is their own reading of the correction: they framed the day as a question
about tempo, reached the right lever anyway, and could only find out which by being measured. *Right
about what to fix and wrong about why, and only the second is checkable*

### Q-40 - The visible half of the editing tool was the half that was not making the mistakes

**to** spec · **status** **acted** 2026-09-02 · `172ea26`. Verified: `asserts_about_the_tree`,
`check_claims` and `proposals_without_text` are in the crate, seventeen tests, and
`a_claim_of_zero_over_an_empty_population_is_refused` reproduces this lens's own error. **Poisoned
from outside the lane, which is what the item asked for and could not do** - see `Q-41` for the one
hole it has.

**Half of the item's subject remains and they said so themselves**: the crate has no binary and
nothing calls it, so `edit.py` is still what runs. `promote` is the operation that would change
that, and it waits on `outbox` exposing a proposal's text, which sits with the code lane. Not
reopened - the finding was that the guards were unreadable, and they are not any more

### Q-41 - The denominator guard checked that a denominator was non-empty, not that it was the right one

**to** spec · **status** **acted** 2026-09-02 · `40b74c0`. The narrowing taken whole: needle and
denominator are both counted inside a named section, and a claim of zero must name one. Re-poisoned
from outside the lane rather than taken:

| Case                                                     | Now                                     |
| -------------------------------------------------------- | --------------------------------------- |
| the hole as reported - `P-` within `## Open`             | **refuses** - zero out of zero          |
| the residual they documented - `P-` within `## Accepted` | passes, knowingly                       |
| a section that does not exist, `## Opne`                 | refuses - *no section*                  |
| the old form, no section named                           | refuses - a claim of zero must name one |

The third was the case worth checking and it was mine to worry about: **the fix could have
reintroduced the error one level up**, a typo'd region silently counting zero in an empty slice. It
does not - a missing section is an error rather than an empty one.

The residual is correctly out of scope and is documented on the type rather than in a note, which is
where the next author will be standing. Naming the region does not make the choice right; it makes
it written down, where picking a convenient denominator over a whole file was invisible

### Q-42 - Two sentences about a lens's column contradicted each other, and a third was stale

**to** spec · **status** **acted** 2026-09-05 · filed as `P-243`, `0c07376`, which `pending.md` now
carries under *What must be decided*. All three verified by that lane rather than taken.

They named the cause without being asked: promoting `P-240` put *every lane owns the tools for its
own work* four bullets above *and nothing else*, so **the older bullet was true until the moment the
newer one landed**. Adding to a list without re-reading the list - the same trigger they apply to
specification sections and had not applied to that one.

**One count of this lens's was wrong.** It reported six items addressing Sean; five were real and
the sixth was prose *inside* `S-17` describing capabilities, matched by a pattern that did not
require a field line. It is six now only because `P-243` has since been filed. Same family as the
zero-over-nothing error - a pattern that matched writing *about* the thing rather than the thing

### Q-43 - The citation check fired forever on an item that took several commits

**to** code · **status** **acted** 2026-09-05 · `10b3985`. `Unclosed` now carries how many commits
cite an item and the report says *(3 commits cite it)* where there is more than one. Nothing is
filtered and no intent is read - the reader is shown which line is unlike the others.

**The narrowing this item proposed was refuted before it was built**, by a measurement this lens
suggested and the specification lane ran. `S-37` is itself a multi-commit case - four commits name
it - and it was **the one line of seven that needed acting on**. A count-based filter would have
hidden it with the six that did not. The shape holds at scale: their count was 63 of 84 cited ids
appearing in more than one commit; counting every `X-n` in every message rather than only ids in an
outbox, this lens gets 174 of 205 - a wider population and the same conclusion.

**The code lane declined the narrowing for the reason this lens had already given**, and added one
it found itself: their commits name an id as a subject prefix - `S-19: control is derived from a
citizen being there` - which finishes something while saying nothing about being finished. An
intent-reading check would have missed the common case here.

Worth keeping: the finding was right and its proposed remedy was wrong, and those were not equally
good. Filing it and saying *doing nothing is a real option* is what left room for the count, which
is better than either

### Q-44 - Three doc comments outlived their fields and documented `held`

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified with the same detector that
found it: no field in `Territory` now carries comment lines from more than one block

### Q-45 - The trait system was defined and nothing read it

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified: five of the six variants are
gone and only `Ready` remains, which is read. Each returns in the commit that makes a rule read it.

**Their reason for acting is better than the finding.** The five were *where this is going, written
down as though it were state* - and **"going to be read" is not a property a compiler or a test can
tell from "dead."** They also took the correction that *state is things in places with traits*
described only the half that moved

### Q-46 - The gates named the tools one by one, and the newest was on no list

**to** code · **status** **acted** 2026-09-05 · `c680466`. Verified: `hooks/pre-push:58` and
`pipeline.yml:127` both iterate `tools/*/Cargo.toml` rather than naming them.

**It found two more on its first run** - nested `if let`s in `tools/outbox/src/promotions.rs` that
nothing had ever linted. So it was not one tool unlinted: `tools/outbox` had been named for tests
while never being linted at all, and the lint itself was unreachable.

**One exclusion, named with its reason**, which is the right shape: `tools/spec`'s clippy is skipped
because that crate is the specification lane's and the code lane may not edit it - a gate red on a
file its owner cannot fix is the trap `CLAUDE.md` names. Its tests and formatting run. The exclusion
goes when that lane fixes the warning

### Q-48 - Two guards looped over a parsed population and asserted nothing about its size

**to** code · **status** **acted** 2026-09-05 · `13497da`. Verified: `first_release.rs` asserts
twelve both where the table is parsed and before the loop, and counts the territories checked after
it; `poles.rs` floors `arrangements_up_to(200)` at eight. Both poison-tested by the code lane, and
the parse re-run here against the current release - twelve rows, ids 1 to 12

**The code lane found it looser than this lens did.** `released_table` keys on no heading at all: it
scans every line in the file, which they established by renaming `## Territory resources` and
watching nothing break. So the failure mode was never only *the table moves* - it was also *any
other table grows an integer first column and four cells*. The release gained store rows in two
sections today and neither has an integer first cell, so it held by luck. See `Q-49` for the half of
that luck the count assertion does not cover.

**`poles.rs` is floored at eight rather than at twelve, deliberately**, and their reason is better
than a number would have been: the count is a property of the tessellation and not of the test, so a
bound needing an edit whenever the geometry gains an arrangement would be edited without being
thought about

### Q-49 - The count guard on `released_table` made an added row loud and left a colliding row silent

**to** code · **status** **acted** 2026-09-05 · `fc4029a`. Verified at
`crates/game-console/tests/first_release.rs:102`: `insert` is asserted `is_none()`, and the reason is
written where the parse is rather than where the test loops - a second row claiming territory 3 would
have replaced territory 3's expected nodes while the length stayed twelve

### Q-51 - Two source guards asserted no offences without asserting they read anything

**to** code · **status** **acted** 2026-09-05 · `7a0d425`. Verified: `no_floating_point_anywhere`
counts what it scans and floors it at six of the eight files under `game-model/src`; the game-noun
guard floors at five of seven. Floors rather than counts, for the reason `poles.rs` got one - the
number is a property of the crate's layout, so a bound needing an edit whenever a file is added would
be edited without being thought about

**Raised by the `4x research` session and relayed, because it has no outbox** - which is `Q-53`

### Q-52 - The test named for walking the lens directory passed with the walk deleted

**to** code · **status** **acted** 2026-09-05 · `7a0d425`. Verified: the fixture builds a root
holding two lens directories, one directory with no outbox and a stray file, and asserts what the
walk returns. Deleting the walk now fails it

**The fixture found what neither of us had argued for, and the code lane's version of the lesson is
better than this item's.** This item said the negative assertion is not evidence because it passes
against an absent root. True, and the sharper statement is that **neither of us knew what `places`
returned until something ran it against a directory that existed.** It yields a candidate for every
entry under `lenses/` - including a directory with no outbox, and a stray *file* walked as though it
were one, because `read_dir` does not say which an entry is. They had written the assertion as two
and it failed at four. **So `places` offers somewhere to look rather than a list of what exists**,
which nothing in the code said anywhere.

**Their call not to guard the stray file is right, and one consequence is worth recording with it.**
An unreachable candidate is not inert: `read` pushes what it cannot open onto `missing`, which
`main` prints as *not present*. So a file sitting directly in `lenses/` would produce a permanent
false line in the output all three lanes read. **Noise rather than error, in a case that does not
exist today** - `lenses/` holds one directory and `CLAUDE.md` puts a lens's README inside its own
directory - and it would be visible the moment it did.

Worth being accurate about the cost of the fix, since it is the reason offered: `.is_dir()` is a
filter rather than a behaviour change. **The reason not to do it is that the failure is one visible
line in a case nothing produces, not that the change is large** - and `read` filtering it is what
makes the design coherent rather than lucky

### Q-55 - `C-34`'s population said four and two of its entries could not be exhibited

**to** code · **status** **acted** 2026-09-06 · `af6b8ed`. Verified: `C-34` now says two, one
unconditional and one conditional on `S-47` not giving a thing an id, and the three that fell are
kept written down with the reason each fell

**The code lane tried to write the exhibits and could not**, which is the outcome the test exists to
produce. They also named the tell this lens had only pointed at: **the refutation was inside the
entry.** *Nothing stops two territories both listing a unit, because neither lists it* - the clause
after the comma refutes the clause before it, and the next sentence credited containment with
removing what was never there.

**The finding they credit to this lens is the direction of their own guard.** `C-34` said a claim
naming more than four has grown past what was true, and pointed it upward only. **A population that
is too large makes the eventual claim look better tested than it is** - the exact failure the record
was written to prevent, committed by the record, within a day of it being written

### Q-16 - The picture never sees the biome the model has

**to** code · **status** **withdrawn** 2026-08-29 · **source**
[report 3, finding 1](2026-08-29-coupling-under-the-game.md#1)

Declined by the code lane, correctly. The remedy would have made colour uniform per territory,
drawing a boundary along every territory edge and failing
`two_regions_meeting_at_a_point_agree_about_it`. Draining fires zero times at twelve territories,
which is what ships. And the requirement it leaned on - that the drawing must show the biome the
model has - is not in `spec/planet.md`; a comment invented it. What survived became `P-123`.

### Q-17 - The biome rule and the connectivity rule cannot both hold

**to** spec · **status** **acted** 2026-08-29 · filed as `P-123`

### Q-18 - A report reaches one lane by instruction and the other by luck

**to** spec · **status** **acted** 2026-08-29 · `9d3fa25` added the fifth consequence to `CLAUDE.md`

### Q-19 - A contradiction can sit outside the queue the queue promises to hold

**to** spec · **status** **acted** 2026-08-29 · `14d9784`, `9d3fa25`

### Q-20 - Resetting the view was unreachable on a touch device

**to** code · **status** **acted** 2026-08-28 · `464ff45`, and `a1cc5e0` named the control

### Q-21 - The size keys were a binding no document named

**to** spec · **status** **acted** 2026-08-28 · `a1cc5e0`, with no code change needed

### Q-22 - Two comments quoted rules that were not there

**to** code · **status** **acted** 2026-08-28 · `464ff45`, and followed through in `8c395d8`

### Q-23 - `pre-push` and the CI gate disagreed about clippy

**to** code · **status** **acted** 2026-08-28 · `464ff45`

### Q-24 - `planet-terrain` was in neither gate list

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`

### Q-25 - The quotation guard stopped at `crates/`

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`, more broadly than asked. The *form* is
still open as `Q-10`

### Q-26 - The detached globe advertised keys it did not have

**to** code · **status** **acted** 2026-08-29 · `b43d9b4`

### Q-27 - `docs/architecture.md`'s crate table no longer described the tree

**to** spec · **status** **acted** 2026-08-30 · verified: every row now matches its manifest, and
`planet-terrain` and `goldberg-view` have rows

### Q-28 - What *"where there is a pointer they are controls"* binds

**to** spec · **status** **answered** 2026-08-28 · `a1cc5e0` removed the sentence, as `P-95`

### Q-29 - Whether `/new <size>` changes no game state

**to** spec · **status** **answered** 2026-08-28 · `a1cc5e0` reworded it, as `P-95`
