# Everything that says what the game does

Generated. Every reference is a link, every page has a markdown sibling to diff, and no page needs JavaScript to be read - `R-9`.

[The index](index.md)

## Pages  (7)

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
- [unused-rows.md](unused-rows.md)
  rows that can be deleted and nothing a read test asserts would notice - written by `cargo test -p game-model -- --ignored`
- [unused-values.md](unused-values.md)
  values that can be changed and nothing a read test asserts would notice - written by `cargo test -p game-model -- --ignored`
