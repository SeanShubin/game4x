# What the predecessor built, and what is worth taking from it

**2026-09-05.** A survey of `D:\keep\github\sean\game-4x`, the project this one succeeds, read at
Sean's request so the information is to hand rather than rediscovered. **Paths are given rather than
contents copied** - `docs/README.md` says a document that restates another links to it instead of
listing it, and the same holds for a document restating a repository.

A second note, [what it did with things and readiness](game-4x-on-things-and-readiness.md), covers
the `game/` module in detail; it was written for `P-231` and is not repeated here.

## What is in there

Kotlin, Maven, twelve modules, 245 source files. Four of the modules are successive prototypes and
the later ones supersede the earlier.

| Module       | Files | What it is                                                     |
| ------------ | ----- | -------------------------------------------------------------- |
| `prototype4` | 55    | **The most developed thing in the repository.** A working REPL |
| `language`   | 44    | A standalone parser-combinator library                         |
| `prototype`  | 41    | An earlier attempt, superseded                                 |
| `command`    | 37    | Command handling                                               |
| `prototype2` | 31    | Superseded                                                     |
| `game`       | 12    | The thing-and-place model - see the other note                 |
| `prototype3` | 11    | Superseded                                                     |
| `skirmish`   | 6     | Combat, in miniature                                           |
| `format`     | 5     | Table formatting and JSON mapping                              |
| `contract`   | 2     | A boundary around the filesystem                               |

**Read `prototype4` and `game` and ignore the rest** unless looking for something specific. The three
earlier prototypes are the same ideas less well expressed.

## The four things worth considering now

### 1. It already had the command notation, including the part we have not settled

`prototype4/.../CommandSyntax.kt` is a grammar, thirty lines, and it is very close to what `P-212`
promoted here:

- `{name attribute-list}` and also **`{attribute-list}` with no name**
- an attribute is `name=value` **or a bare `$alias`**
- a value is a primitive **or another item**, so items nest
- a call is `name parameter-list`, and a parameter is a value

**Two differences from `spec/console.md`, and both are live questions here.** It used `=` where this
project uses `:` - cosmetic. And **a `$alias` could stand where a whole attribute goes, not only
where a value goes** - which is the shape `P-234` needed for `build extractor` with `$resource`, and
which this project reached four months later by a different route.

**It has an unnamed form and this project does not.** No use for one has come up here; worth knowing
it exists if one does.

### 2. Its command signature is the state function, effects included

`prototype4/.../Command.kt`: `execute(state, parameters, environment)`, returning
`CommandResult(success, state, lines)`.

**That is `docs/process.md` → *What verification requires* → *State*, already built**:
`(old-state, commands) -> (new-state, effects)`, where the effects are the lines printed. The
`environment` parameter is how `load` reaches the filesystem, which is this project's `run <file>`.

`prototype4/.../ReplTest.kt` drives it end to end with a stubbed line reader and writer, asserting
that `add {colonizer}` prints `0 -> 1 {colonizer}`. **The whole application runs from a REPL and the
test proves it** - which is the *Console* requirement in the same section.

### 3. A dependency that provides a home gets an interface and a delegate

`contract/.../FilesContract.kt` is an interface mirroring `java.nio.file.Files`, with
`FilesDelegate` forwarding to the real one and `FilesContractUnsupportedOperation` throwing for
everything, so a test overrides only what it uses.

**This is `docs/process.md` → Dependencies, in code.** The filesystem decides where data lives, so it
is a home rather than operations, and it gets exactly one crate to have opinions in. The same pattern
is in `../code-structure`, so Sean has used it in two projects.

**Worth taking when this project first needs the filesystem from inside the model.** It does not yet.

### 4. Quantity is beside the thing, and the script says so

`prototype4/.../Items.kt` is `Map<Item, Int>` where `Item` is `Map<String, Any>`, and
`script/sample.txt` reads `add {citizen activated=false} 10`. **The count is an argument to the
command, not an attribute of the item** - which is what Sean settled here on 2026-09-04 when he said
quantity should not be a trait.

## What not to take

**`Items` deletes a row when its count reaches zero**, so a thing that exists in quantity zero and a
thing that never existed are the same state. That is fine for a prototype and breaks
`spec/interface.md`'s requirement that the normalized view name every table and column whether or not
anything is in it.

**Two parser libraries.** `language/` is standalone and `prototype4` has its own under
`prototype4/.../language/`. Nothing says which won. **Do not port either** - this project's
`crates/command-language` already exists and works; the value here is the grammar, not the machinery.

**The abandoned prototypes.** `prototype`, `prototype2` and `prototype3` are 83 files of earlier
attempts at what `prototype4` does. Nothing in them is known to be worth recovering, and this note
did not read them closely.

## Combat, since this project has not specified it

`skirmish/` is six files. `GameUnit(attack, armor)`, `fireVolley()` returning one hit per unit, and
`isDestroyedBy(hit) = hit >= armor`. **Whole numbers, no rolls, a unit dies when a single hit meets
its armor.** `spec/combat.md` is still a scaffold here, and this is the smallest thing that could
work if it ever needs filling.

## Where the predecessor kept the turn, 2026-09-06

Sean said the predecessor notably did not have the problem of deciding whether `turn` and `phase`
belong in the state. **It did not, and the reason is that neither was ever in it.** Measured in
`../game-4x`:

- **`Universe(val planets: List<Planet>)`** and nothing else. **`turn` and `phase` appear zero times
  in the whole `game` module.**
- **`phase` appears zero times in the entire repository**, across every module and prototype.
- **`turn` is a local variable in `CommandRunnerImpl`**, incremented by the loop that applies the
  transition. It reaches the outside world as a **file name** - `turn-$turn.json` - and never as a
  field inside one.

**And it is load-bearing rather than incidental.** The runner ends with

    while (!history.contains(current))

**It stops when the state repeats.** That check only works because the state does not carry the turn:
**a state containing its own turn number is never equal to any other state**, so the loop would never
close and every game would run to the turn limit.

**Sean, reading this: the argument is weaker than it looks.** *We don't actually need to compare
game states like that; detecting a stalled game is not viable in practice because once we get to
sufficient complexity the game state can change in vacuous ways.* **He is right**, and it demotes
this from a reason to a piece of evidence: **the code shows where its author put the turn, not that
keeping it out bought something.**

**What survives is his own reason and one measurement.** A turn is a **boundary between states**
rather than a fact inside one. And **no rule in the specification names an absolute turn number** -
every mention is relative, *each turn*, *per turn*, *keeps for one turn* - so a stored turn is
carried for the reader rather than for the rules.

`Land(val things: List<Pair<Thing, Int>>)` is the same file's other lesson - Sean's map form,
already built, where a `Thing` is a list of name-and-attribute pairs and the pair's second element is
the quantity. See [counted things and invented names](counted-things-and-invented-names.md).
