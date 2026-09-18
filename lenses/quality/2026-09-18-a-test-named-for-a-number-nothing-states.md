# A test named for a number nothing states

**Derived.** 2026-09-18, on Sean calling
`a-transport-takes-two-berths-where-a-scout-takes-one` questionable. Not binding.

[Quality](README.md) · [The prototype](../../prototypes/thin-engine/README.md)

Sean, 2026-09-18: *It is certainly valid to test that different berth sizes work properly, but we
don't specifically care what those sizes are for what units.*

## The subject the name gives it is a fact about the game

**Where.** `prototypes/thin-engine/data/friendly/tests/a-transport-takes-two-berths-where-a-scout-takes-one.4x`
and its foundation copy - the `{test name:...}` line and the header comment.

**What.** The name states a relationship between two kinds: a transport is worth two scouts. **The
fact under test is about the engine** - the same free space admits one kind and refuses another,
because the rate belongs to the consumer rather than to the count. The comment says so in its
second sentence and the name does not.

**And the two numbers are sourced nowhere.** `grep -rn "berth\|scout\|transport" spec/ releases/`
returns nothing: none of the three exists outside the prototype. That is `S-136` working as
intended - Sean, 2026-09-14: *I intend to reuse the game notation with different values* - and it
is exactly what makes `2` and `1` arbitrary rather than wrong.

**Why it costs something.** A test named for a number reads as authoritative about that number, and
the prototype's whole isolation is so that its invented values are not mistaken for findings. When
a rate changes - and nothing anywhere states it, so it will - the test goes on passing under a name
that has become false. **A correct test with a false name is worse than a failing one**, because
nothing reports it.

**Whether.** Worth doing when the file is next touched: a rename and two sentences, no change to
any row. Something naming the mechanism - *the same free space refuses a heavier kind* - and a
comment saying the rates are arbitrary, stated in the `given`, and that what is checked is that the
rate is the consumer's own.

## And the argument is not in the test, which is the smaller half

**What.** The comment reads *the territory holds five berths' worth and admits a scout - **the test
beside this one says so*** - so the contrast that makes this test mean anything lives in
`a-scout-moves-where-there-is-a-berth-to-spare`. Same world at five of six used: a scout fits
there, a transport is refused here. **Nothing couples the two files.** Rename, renumber or retune
the neighbour and this comment is silently false, with both tests still green.

**The split is forced rather than chosen.** `Failed::BothEndings`, `src/script.rs:356`, refuses a
test that states both a `then` and a `refused`, so one file cannot carry both halves. **So the fix
is in the words** - naming the neighbour rather than gesturing at it, so a reader who follows the
pointer lands somewhere and a reader who renames the neighbour has something to grep.

## What is not wrong, so the finding is not read as wider than it is

**The test is correctly pinned, and the harness proves it rather than this lens asserting it.**
`tests/mutation.rs` reads every file under `data/foundation/` **and** every file under
`data/foundation/tests/`, so a test's own `given` rows are mutated too. Change
`{consumes kind:transport what:berth} -> 2` to `-> 1` and territory-2 holds three of six rather
than five; the arrival makes four, the move succeeds, and a test expecting a refusal fails.

**So the rate row is load-bearing and the objection is to what the test says it is about, not to
what it checks.** The two neighbouring berth tests are named for their mechanism and need nothing.
