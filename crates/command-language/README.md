# command-language

[Architecture](../../docs/architecture.md) · [Layers](../../docs/layers.md) · [Root README](../../README.md)

A grammar, a parser, and a typed syntax tree. **Contains no game nouns.**

`land`, `territory` and `metal` do not appear in this crate. A grammar is data handed in
from outside; what the words mean is decided a layer up, in
[`game-console`](../game-console/README.md). That seam is the whole point of the crate
being separate — the parser can be tested against an invented language, and the binding
can be tested without a parser.

## Shape

```text
  text
    |  tokenize          words, each carrying where it was
    |  parse_line        ordered choice over the grammar's forms
  Utterance              typed, arguments reached by name
    |  (a layer up)      a binding table gives the words meaning
```

## Public surface

| Item                              | What it is                                                     |
| --------------------------------- | -------------------------------------------------------------- |
| `Grammar`, `Form`, `Term`, `Kind` | The language, as data. A form is a keyword and its arguments   |
| `parse_line`, `parse_script`      | Text to `Utterance`, or a `Failure` saying where and why       |
| `Utterance`, `Argument`           | The tree. Arguments are read by name and every read is checked |
| `Failure`, `Position`, `Span`     | What went wrong, where it was, and what was expected instead   |
| `tokenize`, `Token`, `COMMENT`    | Words and their positions; whitespace is dropped here          |
| `agree`, `disagreements`          | Whether a grammar and a binding table cover the same set       |

## Why a failure is data

Nothing here panics on input, and there is only one failure style. A `Failure` carries a
`Position`, and every argument carries a `Span`, so a problem can always say *line 1
column 10: expected a number, found `orbit`*. `spec/console.md` asks for rejections that
name what was wrong, where, and what was expected; this is the layer that can answer the
first two, and the binding layer answers the third in the game's own words.

The predecessor reviewed in [parser-architecture](../../docs/notes/parser-architecture.md)
had failures with no position, no expectation, and two different styles of reporting. The
table at the top of `src/lib.rs` lists each of those against what is done here instead.

## Ordered choice

Forms are tried in the order the grammar lists them, and the first that matches wins. That
is load-bearing — `end turn` must be attempted before any shorter form beginning `end` —
so it is written down on `Grammar` rather than left as an accident of iteration, and there
is a test for it.

## Agreement

Two declarative tables define a language: the grammar, here, and the binding, above.
Nothing makes them agree except `disagreements`, which lists every form with no handler
and every handler with no form. Without it, a command nobody wrote a handler for is an
error the first time a player types it — in a program that compiled and whose other tests
passed.
