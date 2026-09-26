# The invariant was already held upstream

**2026-09-26.** A review of the code lane's D-4 night, asked for by them on Sean's instruction, with
an eye on minimising code and removing what is no longer needed. Four commits: `f633864a` the
engine's move into `game-model`, `c438986a` the foundation loader, `c4de6e7c` `S-194` acted,
`17a9332d` the D-4 deletion.

**Everything below is measured at `17a9332d` and the working tree has already moved past it.** See
*The subject moved while it was being reviewed*, last.

## What was asked, and what this found

They asked four things in the order they thought mattered. This report keeps their order and
disagrees with it once: their third question has an answer that changes their first.

| Their question                                                         | What this found                                                                                                                                      | Whether         |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | --------------- |
| 1. Did the deletions go far enough, and is there a cheaper order?      | D-4 names its own search; run, it finds 33 files and 743 lines, and six of them are not D-4 work at all                                              | Worth doing now |
| 2. Are the three exception lists honest?                               | `BEING_REPLACED` and `SHARED` are a real countdown. `BESIDE` is permanent because the population is a directory, not because `foundation` is special | Noted           |
| 3. Is `no_rendered_table_has_an_empty_cell` sound, and are its floors? | It cannot fail while `game-model` loads. The floors tolerate a 49% narrowing and miss losing half the data                                           | Worth doing now |
| 4. Is any of the seven counts right for the wrong reason?              | No. Six cross-derive; the seventh is a panic in library code rather than a test                                                                      | Noted           |

## The check asks a rule that is already enforced, harder, one crate down

**Where.** `crates/game-console/src/relations.rs:234-294`, `no_rendered_table_has_an_empty_cell`;
against `crates/game-model/src/schema.rs:821-837`, `Schema::fits`.

**What.** The new check reads `spec/data/`, builds each relation's columns as the union of the keys
its rows carry, and fails on a cell no row supplies. **A cell is empty exactly when two rows of one
relation carry different key sets.** `Schema::fits` forbids that outright:

> **Exactly, rather than at least.** A row with a column nobody declared is as wrong as one
> missing a column, and both are the data saying something the structure does not allow.

`schema::check` applies it to **every** row - `for row in rows.rows() { let relation =
schema.fits(row)?; ... }` at `schema.rs:965-966` - and `reified` substitutes values into columns the
row already carries, so it cannot change a key set. `schema.4x` is self-describing, so its own
`{relation ...}` and `{column ...}` rows are validated too, rather than bootstrapped past the rule.

**So every row the engine loads carries exactly its relation's declared columns; every row of one
relation therefore carries an identical key set; a union of identical sets has no gap.** The
check's population is `spec/data/`, which is a subset of the engine's - `schema.4x` and `rules.4x`
of the three in `foundation::FOUNDATION`. **It is red only in states where `game-model`'s own load
is already red.**

**Measured**, by a second reader written for this report rather than by reading the test:

|                                      |                                                      |
| ------------------------------------ | ---------------------------------------------------- |
| relations                            | 26 - thirteen in `rules.4x`, thirteen in `schema.4x` |
| rows                                 | 569                                                  |
| cells                                | 2,004                                                |
| empty cells                          | 0                                                    |
| relations whose rows differ in shape | **0**                                                |

**Why this is worth saying rather than a compliment.** The check is a second derivation - a
different parser over the same bytes - and this lens's own rule says sharing the inputs is fine and
sharing the computation is what makes a check circular, so it is evidence and not decoration. **What
overstates it is the module doc**: *Keep the invariant, drop the proxy: `no_rendered_table_has_an_empty_cell`
asks the rule itself, over every cell.* The rule itself was already asked, one crate down and
harder. A reader who believes that sentence will read a green here as *the data has no nulls*, when
what it means is *the data has no nulls and a second parser agrees*.

**And one sentence above it claims the opposite of what the test is for.** `relations.rs:31-34`:
*A relation's columns are the keys its own rows carry, in the order first seen, so one table per
relation has no empty cell to render* - immediately followed by *each later shape's new columns
after them*. The first half says an empty cell is impossible by construction; the second half says
rows have different shapes. **Both cannot hold, and the test below exists because the first is
false.** A later reader with a tidying instinct has a sentence licensing them to delete it.

**Whether. Worth fixing now, and it is two sentences rather than a change to the test.** Say that
the invariant is enforced by `Schema::fits` and that this is the independent derivation of it. The
test earns its place either way; what does not earn its place is the claim that it is the only thing
asking.

## The floors, measured - one of them is 22 cells from being crossed

**Where.** `relations.rs:260-264` and `:290-293`, `found.len() > 10` and `cells > 1000`.

**What.** They said a floor they chose is a number nobody else has checked. Checked, by deriving
what each floor tolerates:

| Population        | relations | cells | `>10`      | `>1000`          | the sibling's `>20` |
| ----------------- | --------- | ----- | ---------- | ---------------- | ------------------- |
| both files        | 26        | 2,004 | passes     | passes           | passes              |
| `rules.4x` alone  | 13        | 1,026 | **passes** | **passes by 26** | fails               |
| `schema.4x` alone | 13        | 978   | **passes** | fails by 22      | fails               |

**Losing `schema.4x` - half the ruleset - passes both of this test's floors.** Losing `rules.4x`
is caught by `cells > 1000` with 26 cells to spare, which is one per cent.

**The honest bound on how much that matters**: a `.4x` file cannot actually vanish without the
compiler saying so, because `foundation.rs` carries both through `include_str!`, and
`browsable.rs:470-478` pins the engine's inputs at ten files. **So the floor is not guarding the
data; it is guarding the reader** - `read()` narrowing its extension filter, its directory, or its
line predicate. Against that, `cells > 1000` at 2,004 tolerates the reader silently losing
**forty-nine per cent** of what it should see.

**The sharper form, because it is in the same file over the same call.**
`a_relation_is_named_by_its_rows_and_not_by_its_file` reads the same `read(&data())` and floors the
same population at `> 20`, **and has an equality besides** - `differing.len() == found.len()`. Two
chosen floors on one population, differing by a factor of two, and **the looser one is on the test
that has no equality to fall back on.** Neither is wrong; they cannot both be the right number, and
nothing in either says which.

**Whether. Worth doing now, and there is a derived answer rather than a better guess.**
`game-console` already depends on `game-model`. `foundation::FOUNDATION` carries each file's name,
path and bytes, and `notation::read` parses them. **Comparing `relations::read`'s relation names and
row counts against `notation::read`'s over the two `spec/data/` entries replaces both floors with an
equality**, and it is the same two-derivations-one-input shape that makes the check worth having at
all. A reader that narrows then disagrees with a reader that did not.

**What is not available, checked before proposing it.** *Every declared relation has rows here* is
not an equality: `schema.4x` declares **49** relations and only 26 have rows in `spec/data/`, the
rest being the state's and the engine's. A floor derived from the declaration count would be wrong
in the other direction.

## D-4's *vetted when* names a search, and the search needs a predicate

**Where.** `releases/rules-become-data.md:56-60`.

**What.** D-4 is vetted when *a search for any recipe name the reviewed tests do not use finds
nothing outside the history*. That is an instrument, so this report ran it rather than offering an
opinion about order.

The reviewed ruleset's rules, from `spec/data/rules.4x`: `breed`, `build-bin`, `build-extractor`,
`build-pioneer`, `deploy`, `discard-disorder`, `end-turn`, `gather`, `launch`, `move`, `perish`,
`refresh`, `toil`, `upkeep`, `work`. The old scenario's verbs that are not among them:
`build-store`, `produce-pioneer`, `launch-ark`, `found-by-land`, `build-yard`, `mine-energy`,
`deploy-ark`, `create-labor`.

**33 files, 743 lines**, over the tracked tree. **Control: `build-extractor`, a name the reviewed
tests do keep, is in 64 files** - so a zero below would be a zero and not a broken pattern.

| Where                                                         | Files | What it is                                                        |
| ------------------------------------------------------------- | ----- | ----------------------------------------------------------------- |
| `crates/command-language`                                     | 6     | **Not D-4 work.** Fixture keywords in a generic parser            |
| `crates/game-console`                                         | 7     | D-4 work - `binding`, `grammar`, `lib`, `report`, and three tests |
| `reports/`                                                    | 4     | D-4 work - `commands` and `turns`, both pairs                     |
| `docs/notes`, `lenses/`, `crates/outbox.md`, `tools/research` | 9     | **Not D-4 work.** The history, which D-4 excludes by name         |
| `scenario/commands`                                           | 2     | D-4 work - `play.4x` and `spread.4x`, and 521 of the 743 lines    |
| `releases/first-release.md`                                   | 1     | D-4 work, and another lane's column                               |
| `crates/game-model`, `game-front`, `game-inspect`             | 3     | D-4 work - `backlog.md`, `console.rs`, `options.rs`               |

**The answer to *is there a cheaper order* is that the order falls out of the search once it has a
predicate, and not before.** Fifteen of the 33 are not what D-4 means. `crates/command-language`
writes `Term::Keyword("deploy-ark")` twenty-four times in `parse.rs` alone, testing that a parser
handles a keyword; it does not know the word is a recipe and deleting it would assert nothing about
the game. **A fixture keyword and a rule statement are the same bytes** - which is the class this
repository has already named, and it is what will make this *vetted when* argue with itself when
somebody runs it to close the item.

**Two things follow, and the second is the one that costs.** The cheap one: `command-language` was
not on their list of what remains, and it is the largest single directory the search names after
`game-console` - so it would have arrived as a surprise at vetting time rather than as work. The
expensive one: **D-4 cannot be closed by the search as written**, because a green requires deleting
nine files of history the item itself excludes and six fixtures it does not mean.

**Whether. Worth doing now**, and it is a change to how the item states its own test rather than to
any code: the search's population is *files that state a rule the game plays by*, which excludes
`lenses/`, `docs/notes/`, `crates/outbox.md` and any keyword used as a parser fixture. That is the
specification lane's to write, since `releases/` is theirs.

## `BESIDE` is permanent because the population is a directory

**Where.** `crates/game-model/tests/common/mod.rs:42-95`.

**What.** They asked whether `BESIDE` should exist, and said the alternative is a separate crate
costing `docs/architecture.md` a row.

**`BEING_REPLACED` and `SHARED` are well built and this report has nothing against them.** Every
check that excepts them goes through one function - `engine_modules()` - and that function asserts
`skipped == BEING_REPLACED.len() + SHARED.len() + BESIDE.len()`, so a module deleted from `src/` and
left in a list is red. It is a countdown and not a catalogue, exactly as its comment says. Verified:
17 modules in `src/`, ten excepted, `ENGINE_MODULES` asserted at 7, and the arithmetic holds.

**The observation is about why the third list exists at all.** The engine's rule is *reads no file
and names no noun the game has* - a statement about **what a module does**. The check's population
is *every `.rs` in `src/`* - a statement about **where a file lives**. `foundation` is not an
exception to the rule; it is an exception to the proxy standing in for it. **This lens has the shape
on file as `Q-83`**: a population defined by location is escaped by an ordinary refactor, and there
the escape was a cost leaving `game::cost` as a side effect of a correct sentence.

**The behavioural statement is available and is true today.** If `foundation` is beside the engine
rather than under it, nothing in the engine names it. Checked over all seven engine modules:
`engine`, `notation`, `refusal`, `script`, `store` and `view` say the word zero times, and
`schema.rs` says it once, at line 1042, **in a comment**. So *no engine module names `foundation` in
code* holds, is one grep, and is a claim about the layering rather than about the directory.

**Whether. Noted, and deliberately not.** A separate crate is the wrong price for this: it buys a
guarantee the grep above already gives, and it spends a row in another lane's document. **What is
worth keeping is the reason the list is permanent** - the population, not the module - because the
comment currently answers *why is `foundation` excepted* and the question a later reader will have is
*why does this list exist*.

## The seven counts

**Where.** `browsable.rs:74,103,478`; `dumps_are_current.rs:151,211,228,332`; `dump.rs:1384`;
`tools/outbox/tests/architecture.rs:236`.

**What. None of the seven is right for the wrong reason**, and they cross-derive, which is better
than each being individually defensible:

- `pages == 22` counts `.html`; `paired == 21` is the same set less `index.html`; 22 = 9 reports +
  12 territories + the index. Checked against the directory: 22 `.html`, 21 `.md`.
- `generated.len() == 19 + 2 + 12 * 2` is 45 = nine report pairs + the index + two stylesheets +
  twelve territory pairs. The same number is restated at three sites **on purpose**, and the file
  says why: *a helper that both read would make one number, and two checks over one number is one
  check*. That is right.
- `pages == 10 + 12` is 22 again, spelled differently.
- `tools/outbox`'s floor moved 13 → 11 with two readers deleted, and **the assertion's own message
  asked for exactly that move**, which is the cleanest of the seven.

**Two small things, neither worth an item.**

**One of the seven is not a test.** `assert_eq!(listed, 9, "nine reports are linked")` is in the
body of `pub fn index`, and `dump.rs` carries **ten** assertions with no `#[cfg(test)]` anywhere in
the file. It is exercised - `dumps_are_current` calls `generated()`, which calls `index()` - so the
count is checked; what is not checked is that it stays a count rather than becoming a panic
somebody's session meets. **`Q-100` already records this shape in `declare.rs`**, and `declare.rs`
is one of the files D-4 deleted, so one instance went and ten stayed in the module D-4 was editing.

**Their tally of seven counts expressions where the diff has nine edit sites** -
`dumps_are_current.rs` changed at four, which they reported as *the dumps at two places*. Both
numbers are true of different things and theirs is the more useful one. Recorded only because
asserting a count whose population is ambiguous is the thing that bit `Q-94`.

## What went too far: nothing found, and what this did not check

They deleted four tests rather than repointing them. `closed_sets.rs`, `petri.rs` and
`vocabulary.rs` all still stand with their remaining tests at `17a9332d`. **This report did not
establish that each deleted test's claim is covered elsewhere**, and says so rather than implying a
sweep it did not run: what it checked is that no file was emptied, which is the weaker claim and the
one the commit already makes.

## The subject moved while it was being reviewed

**This is not a finding about their work and it is why every number above carries a commit.**

While this review ran, the working tree gained eleven staged deletions - `nogain.rs`, `petri.rs`,
`petri_draw.rs`, `petri_page.rs`, `recipes.rs` and `worked.rs` from `src/`, and `first_release.rs`,
`nogain.rs`, `petri.rs`, `vocabulary.rs` and `worked.rs` from `tests/` - plus six reports. These are
the next chunk of D-4, the part they said they had not started.

**Two consequences, one of them this lens's own error caught in flight.** A `cargo test --workspace`
started at the beginning of this review was compiling a tree that was being deleted under it; it was
stopped rather than read, because whatever it reported would have been about no commit at all. And
`dump.rs` now reads `assert_eq!(listed, 6, "six reports are linked")` where `17a9332d` reads `9` - so
*the index at nine reports*, one of the seven counts under review, had already moved to six before
the review of it finished.

**The baseline rule in this lens's own README says take the range from the work rather than from a
clock.** It does not cover the case where the work is still happening, and the answer is the same
one: every claim here names `17a9332d`, and a claim about the working tree would have named nothing.

**And the deletions are staged rather than committed**, which `CLAUDE.md` says is the shared index:
whoever commits next takes them, under whatever message they are writing. Reported to them as a fact
about the tree, not as a criticism - it is `C-4`, and it is nobody's to fix from here.
