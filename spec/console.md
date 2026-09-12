# Console

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

## Phases

- A game has two phases. In the first the world is designed. In the second it is played.
- `start` ends the first and begins the second.
- A command of one phase is refused in the other. The five design commands are refused once
  `start` has run, and every other command is refused before it.
- The rules of the game govern the second phase. In the first, the designer is the cause of what
  appears.

## The language

**There is one notation.** A command and a description of game state are written in the same form,
and both carry a tree.

**The language carries the tree whether or not a command uses one today.** Two notations would cost
more to maintain and more to read than one, and a command that takes a tree is expected rather than
hypothetical.

- Commands may be organized in a hierarchy of files, one file invoking another as a subroutine
- A `#` begins a comment. The rest of the line is ignored

A name is one word. **Where it needs more than one, the words are joined with dashes** - `in-play`,
not `"in play"`. **Nothing in a data file is quoted.**

**A thing's own identifier is `id`.** A field named for a kind is a reference to one - so
`{extractor territory:1}` is an extractor in territory 1, and `{territory id:1}` is the territory
itself.

**A field that refers to a thing is named for that thing's kind**, so what a value is can be read
from the key beside it. `where:1` does not say what kind of thing `1` is and `territory:1` does.
A thing's own identifier is `id`, which is the one field that names no kind.

**A field's value may name a kind rather than a thing.** `territory:1` refers to one particular
territory; `resource:food` says which resource, and means no particular food. **Which of the two a
field takes is a fact about that field**, so reading the key still tells you what its value is.

**A rule may name one thing and say what changes about it**, rather than taking a quantity of its
kind. What is named keeps its `id`, what it holds, and what it has already spent; **only what the
rule changes changes**. A quantity is for what is interchangeable, and **a thing carrying an `id`
is never a quantity**.

**A command is named for the recipe it fires**, and there is one command for each recipe the
player may fire.

**A command may carry a `repeat`**, which is how many times it fires. It is not an argument of the
recipe; it is a count of firings, and a command without one fires once.

**What a thing contains is a map from a description to a quantity.** A description is a kind and
**every trait of that thing**; a trait **of its kind** is not part of one, because naming the kind
has already said it, and a derived trait is never part of one. **No trait of the thing may be left
out** - `{citizen defending:1} -> 8` and `{citizen defending:0} -> 6`, never `{citizen} -> 14`.
**Each distinct description is its own entry, and an entry is never zero.** A thing carrying an
`id` has a description no other thing shares, so **its quantity is always one**. **Where a thing
is, is where it appears**; nothing states its container. **Entries are in the order their
descriptions sort in, and the traits inside a description sort too**, so the same state is always
the same bytes and a description is one string however it was built.


**Every word in a data file is a kind, a trait, or one of a trait's values.** A file that uses any
other word is wrong about the game rather than describing it.

**A file may declare the vocabulary rather than use it, and it is written in the same form.**
`kind`, `trait` and `family` are themselves kinds, so a line that declares one is a description
like any other and the rule above needs no exception. **A declaration is the third thing the one
notation carries**, beside a command and a state.

**A kind declares which family it is in; a family declares only its name.** **`thing` is the
family every kind is in**, and no line says so kind by kind - a kind added tomorrow is a `thing`
because it is a kind, and is in no other family unless it says so.

**A kind declares which traits it has, and a value declares which trait it is one of.** So a trait
says what it admits and whether it is stored, and says nothing about which kinds carry it; and a
trait whose values are kinds names their family instead, because they are already declared. **The
thing that belongs to something says so, and the something says only what is true of itself.**

**In a recipe the same form is a selector, not a description.** A selector may name a **family**
rather than a kind, and it may **leave traits out** - `{extractor resource:food}` selects every
extractor built for food, whatever else is true of it. **A description may do neither**: it names
one kind and carries every trait of that thing. **Where the form stands is what says which it
is** - a selector in a recipe, a description in a state.

**In a recipe, a quantity and either side of a guard may be an expression.** An expression is one
of these and nothing else:

- a number
- a **path**, which reads a trait of something a name is bound to, as `t.nature`
- **`holder of x`**, what holds `x` - which is how a thing's place is read, since a thing is not
  located by a trait
- **`<trait> of x`**, one thing's trait
- **`count {…}`**, how many things match a description
- **`sum <trait> of {…}`** and **`max <trait> of {…}`**, that trait aggregated over all of them
- **`min(a, b)`**, the lesser of two
- **`available <kind> of x`**, a container's total capacity for that kind less what it holds

**A guard compares two expressions**, with `=`, `<`, `≤`, `>` or `≥`.

**The words an expression is built from are the notation's own.** They are the one thing in a data
file that is not a kind, a trait, or one of a trait's values.

**A line may carry an attachment, written in brackets after its amount** - `-1 [soft]`. It says what
to do when the line cannot do all of what it says, and it is a fact about the operation rather than
about the thing operated on. **A line with no attachment does all of what it says, or the rule does
nothing.**

## Commands

- Commands to query the game state are available
- A sequence of commands may be run one at a time interactively, or run in full as a test

A command is written `{name field:value ...}`. **Its name is one word**, dashed where it needs
more, and its arguments are named. **A value is a word, a number, or another command in the same
form**, so a command may carry a tree.

**A command names a recipe and binds what that recipe leaves open**: the place it acts in, and any
ingredient or trait value it names with a `$`. **There is one command for each recipe the player may fire**, and
ending a turn fires the world's.

The commands are therefore not a list this document keeps. They are the recipes whose owner is the
player, and adding a recipe adds a command.

A command that changes the game names a recipe, and that holds while a world is being designed as
much as while it is played. A game state changes only by a transition, which phase a game is in is
part of its state, and designing is therefore made of the same rules as playing. The design
commands are the player's recipes, offered only while the phase is design. `show`, `help` and
`history` are listed here because they change nothing, and so name no recipe.

- `run <file>` - run the commands in a file, as though they had been typed in its place

And three that change nothing:

- `show <subject>` reports what is true of it and what can be done with it. For each action the
  rules permit on that subject, it says whether it is possible now, and when it is not, what is
  missing.
- `help [<command>]` - list every command, or give one command's syntax
- `history` - list every command executed so far, in order

Available only before `start`:

- `create planet <size>` - make a planet and its territories
- `set resource <territory> <resource> <extractors> <density>` - give a territory its
  total capacity and its density for one resource
- `set force <territory> <force>` - set a territory's force of nature
- `set biome <territory> <biome>` - give a territory its biome
- `add <unit> orbit` - place a unit in orbit before play begins
- `start` - end the design phase and begin play

A line beginning with `/` directs the front end rather than the game. `/game`, `/console` and
`/browser` choose a surface; `/new <size>` abandons the current game and starts one on a planet
of that size. `/save <file>` writes the history of the current game to a file, which `run` can
then execute. None of these is a command and none is a transition: history does not record them,
and help does not list them. A game's history begins when the game does.

## Errors

A command that cannot be run says why, and says it in terms of the game rather than the parser.
A rejection names what was wrong, where, and what was expected instead.
A rejection names the line and column it was found at, and the command it was found inside.

## Open questions
