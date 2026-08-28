# game-console

[Architecture](../../docs/architecture.md) · [Layers](../../docs/layers.md) · [Root README](../../README.md)

The command language bound to the game. **The only door into the model.**

`spec/invariants.md`: *every change to game state is representable and executable as a
console command.* This crate is what makes that true — it is where a word meets a rule,
and `Session::run` is the only path from one to the other.

```text
  text
    |  command-language     grammar and parser, no game nouns
  Utterance
    |  binding              the one place a word meets a rule
  Transition
    |  game-model           the one function
  Game
```

A question asked of the game comes back through the same door and changes nothing, because
the type it produces has no way to say otherwise.

## Public surface

| Item                                            | What it is                                                        |
| ----------------------------------------------- | ----------------------------------------------------------------- |
| `Session`                                       | A game and its history. `run` is the only way state moves         |
| `Outcome`                                       | `Changed`, `Said(String)`, or `Nothing`                           |
| `Problem`                                       | Everything that can go wrong, in the order the layers are crossed |
| `Library`, `NoLibrary`, `Embedded`              | Where `run <file>` finds a file                                   |
| `command_grammar`                               | The forms `spec/console.md` lists, as data                        |
| `interpret`, `Meaning`, `Subject`, `Misreading` | The binding table and what it produces                            |
| `Entry`                                         | One entity and its components, for the data browser               |

## The three failures

Each is reported by the layer that found it, in that layer's own terms. `spec/console.md`
asks that a rejection be phrased in terms of the game rather than the parser, and this is
how both get said:

| Problem   | Found by         | Reads like                                     |
| --------- | ---------------- | ---------------------------------------------- |
| `Parse`   | command-language | `line 1 column 10: expected a number, found …` |
| `Misread` | the binding      | `there is no planet size called enormous`      |
| `Rule`    | game-model       | `there is no ark in orbit`                     |

`NoSuchFile` and `TooDeep` belong to `run`, and say which files there are and that files
are calling each other without end.

## Binding is a table

Adding a command is a row in `binding` and a form in `grammar`, and nothing makes the two
agree except a test. `command_language::disagreements` lists every form with no handler and
every handler with no form; without it, a command nobody wrote a handler for is an error
the first time a player types it, in a program that compiled and whose other tests passed.

## History is a save file

Only a command that changed something is recorded, and a file's contents are recorded
rather than the call to it. That is what lets a history be replayed on its own, with no
files at all, to rebuild the same game — which is the invariant *a game state is exactly
the result of applying every transition in order* stated in a form you can run.

## The acceptance test

[`tests/first_release.rs`](tests/first_release.rs) plays `releases/first-release.md` from a
designed world through to a working territory, and it **parses the release document
itself** — both the twelve-territory node table and the production costs. The coupling is
deliberate: retuning a number in that document fails this test until the model and the
command files are retuned to match. Nothing else keeps a constant in Rust and a figure in a
markdown table honest about each other.

## The command files

The `.4x` files in [`commands/`](../../commands) at the repository root are the release's
own world, written in the language this crate reads. `setup` calls `nodes` and `forces` as
subroutines, which is what `spec/console.md` means by a hierarchy of files. They are read
off disk by the test and carried in the binary by the front end, because a browser has no
filesystem.
