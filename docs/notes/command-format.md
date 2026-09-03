# What a command should look like

**Derived**, 2026-09-03, answering Sean's question about command format. Not binding.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

He named three: traditional flags, `move --unit pioneer --from 10 --to 3`; structured data,
`{move unit:pioneer from:10 to:3}`; and a compact format such as Clojure's EDN.

## One correction first

**Nothing here uses flags.** `spec/console.md` has `move <unit> <territory>` - **positional keywords
and holes**, so today it is `move pioneer 3`. The `--unit` form is neither what exists nor what the
predecessor used.

## The constraint that decides it is not the commands we have

Every command today fits on a line with two or three arguments. **The format only has to change
because of what Sean has just asked for: defining a recipe with a command.**

A recipe is not flat. `move` is five rows of role, quantity, kind, traits and place. Under a flat
grammar that is either **five commands assembling one recipe by side effect** - which makes a
half-defined recipe a real state - or one command carrying a tree.

**`crates/command-language/src/grammar.rs` already says so, before anyone asked:**

> A form is flat: keywords and holes, in order. That is enough for a language of one command to a
> line... **If the language ever grows nesting or arithmetic, this is the file that has to grow a
> real expression type**, and the absence of left recursion will have to be faced deliberately
> rather than inherited by accident.

## Why not EDN

**A general data format cannot say what was expected.** `spec/console.md` requires that *a rejection
names what was wrong, where, and what was expected instead*, and `command-language` exists to deliver
it - the whole crate is built around `Failure::expected`. **A reader for a general notation can only
say *unexpected token***, because nothing told it which commands exist.

EDN also imports decisions this project has not made - keywords against symbols, lists against
vectors, tagged literals - and needs either a dependency or a hand-written reader for a language
richer than the game needs.

**Take one idea from it and leave the format**: *one notation for data and for code*. That idea is
worth everything here; the syntax is not.

## Recommendation: named fields, nesting, one notation

**`{move unit:pioneer to:3}`, and a value may be another node.**

**Named rather than positional, and the reason is Sean's own acceptance test.** He wants to derive
the dump by hand from the definitions and the commands. **`build extractor 1 metal` requires knowing
that 1 is the territory** - a fact no artifact states. `{build kind:metal-extractor where:1}` does
not.

**Nesting, because a recipe is a tree and so is everything else.** He settled this afternoon that a
thing is a node with a value and children; the state is that tree; a recipe describes changes to it.
**A notation that can only express a flat line cannot state a rule about a tree in one transition.**

**One notation, because the same shape then serves three jobs**: a command, a rule definition, and a
rendering of state. `entities.md` is already *every thing with its components*, which is
`{territory id:1 biome:grassland citizens:12 ...}` in a table's clothes.

**Keep the grammar.** This is a change to what a *value* may be, not a move to a general data
format. The forms still say which commands exist, so a failure can still name what was expected -
and the grammar can still be **handed in from outside as data**, which `grammar.rs` already promises
and `P-199` will need.

## The cost, and where it lands

**Typing suffers for the common case.** `move pioneer 3` is better at a console than
`{move unit:pioneer to:3}`, and the console is a surface a person uses.

**Two ways out, and they are not equal.** A short positional form beside the structured one is two
syntaxes, which is the special language Sean keeps rejecting. **A form whose fields have an order as
well as names** - so `{move pioneer 3}` and `{move unit:pioneer to:3}` are the same command - is one
grammar with an abbreviation, and it is what the current forms already are.

**That second option is the recommendation**, and it means the structured form is what a file
contains and what a report prints, while a person at a console may still type the short one.

## What has to be decided before this can be built

- **Whether a rule definition is one transition or several.** One nested command is atomic; several
  flat ones leave a half-defined recipe reachable. The tree argues for one.
- **Where a failure points inside a nested command.** `spec/console.md` says a script *stops at the
  first line that fails and says which*; a command spanning twenty lines needs a position inside it,
  which `every_word_knows_where_it_started` already provides and no failure currently uses.
