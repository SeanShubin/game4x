# Everything that says what the game does

Generated. Every reference is a link, every page has a markdown sibling to diff, and no page needs JavaScript to be read - `R-9`.

[The index](index.md)

## Pages  (10)

- [tests.html](tests.html) - [as markdown](tests.md)
  every test, its foundation form, and whether he has read it
- [ruleset.html](ruleset.html) - [as markdown](ruleset.md)
  every rule, how many read tests fire it, and the data under them
- [scenario.html](scenario.html) - [as markdown](scenario.md)
  the main scenario, one case per command, by turn
- [types.html](types.html) - [as markdown](types.md)
  one case per relation the game declares
- [primitives.html](primitives.html) - [as markdown](primitives.md)
  one case per word the engine implements
- [review/index.html](review/index.html) - [as text](review/index.txt)
  every test with a control beside it, and every regression case - the page a                verdict is pressed on
- [nogain.html](nogain.html) - [as markdown](nogain.md)
  whether any sequence of rules can come back round with more - solved for, not declared
- [petri.html](petri.html) - [as markdown](petri.md)
  the rules as a net, one part per rule, and the places nothing fills or empties
- [unused-rows.md](unused-rows.md)
  rows that can be deleted and nothing a read test asserts would notice - written by `cargo test -p game-model -- --ignored`
- [unused-values.md](unused-values.md)
  values that can be changed and nothing a read test asserts would notice - written by `cargo test -p game-model -- --ignored`
