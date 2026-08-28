# Parser and Assembler Architecture

**Derived.** Written by Claude from conversation, 2026-08-25. Not binding - a review of the
predecessor's design, kept so the Rust rewrite can take the good parts deliberately.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Reviews the parser in [`../game-4x`](game-4x-predecessor.md), whose `prototype4` module is
the most evolved version.

> **This is not a porting guide, 2026-08-26.** Sean: *"I don't want you to replicate the old
> game-4x version, I want you to create a proper version of that."* An earlier line here said he
> wanted *the same shape in Rust*, and that was wrong. **What survives is the seam** - a generic
> parser that knows no game nouns, and a small table binding grammar rules to meaning. Everything
> under *what is weak* below is a **requirement for the new one**, not a defect to reproduce, and
> `spec/console.md` states the ones a player can observe.

## The pipeline

```
IndexedCursor            generic input, char by char
  |
CommandSyntax            the grammar, 31 lines of declarative combinators
  |                      Expression.consume(cursor) -> Success(cursor, Tree<Char>) | Failure
Tree<Char>               parse tree, every node labelled with its grammar rule name
  |
AssemblerImpl            35 lines, generic, dispatches on node NAME via a map
  |
Call / Item / primitives typed AST
  |
CommandLookupImpl        name -> Command.  14 lines.  The only game-specific file.
```

The notation it parses:

```
add {colonizer}
add {citizen activated=false} 10
add {node resource=food activated=false density=6} 6
```

```
call      := name value*
value     := item | primitive
item      := "{" name? attribute* "}"
attribute := alias | name "=" primitive
primitive := alias | number | boolean | string
alias     := "$" name
```

## What is good

**The name-keyed assembler map is the strongest idea in it.** `AssemblerImpl` knows nothing
about any grammar. The bridge from syntax to semantics is one table:

```kotlin
class AssemblerImpl(private val assemblerMap: Map<String, AssembleFunction>) : Assembler
typealias AssembleFunction = (parts: List<Any>) -> Any?
```

Add a rule to the grammar, add a row to the map. Two declarative tables and one generic
bottom-up walker, instead of a visitor hierarchy that grows a method per node type.

**The layer separation is real.** `CommandSyntax` contains no game nouns - no `add`, no
`citizen`, no `food`. Those are just names it happens to parse. The entire game-specific
surface is a 14-line `Map<String, Command>`. The seam Sean wants already exists here.

**Discarding punctuation is a semantic choice, not a parser concept.** `nullAssembler`
returns null for `{`, `}`, `=` and whitespace, and `assembleBranch` uses `mapNotNull`, so
they vanish. There is no "significant token" flag anywhere in the parser.

**`delegateToNestedAssembler` (`{ it[0] }`) handles the `OneOf` wrapper problem** in one
line, everywhere it occurs.

## What is weak

**Type safety is abandoned at the seam.** `(List<Any>) -> Any?` everywhere, three
`@Suppress("UNCHECKED_CAST")` helpers at the bottom of `CommandAssemblers`, and
`assembler.assemble(tree) as Call` in `ParserImpl`. A statically typed language running a
dynamically typed pipeline. A grammar/assembler mismatch is a `ClassCastException` with no
indication of which rule produced it.

In Rust this costs nothing to fix - one enum keeps the dynamic flexibility while making
every case exhaustive and every cast checked.

**Assemblers index their children by position.** `parts[0] as String`, `parts[1]`. The
contract is the order and arity of a `SeqOf`, which is implicit and unstated. Inserting a
term into a rule silently shifts every index in its assembler. It survives today only
because whitespace assembles to null and gets dropped.

**Nothing checks that the two tables agree.** `CommandSyntax` defines rule names;
`assemblerMap` supplies handlers. A missing handler is
`RuntimeException("No assembler defined for 'x'")`, thrown the first time that rule fires at
runtime. A test asserting every reachable rule name has an assembler is cheap and would make
this a compile-time-ish guarantee.

**Failures carry no position.** `Result.Failure` is a bare object. `ParserImpl` recovers a
message by diffing cursors - "parsed this portion / but this portion remains" - which is
clever but can only report *where it stopped*, never *what it expected*. `IndexedCursor`
knows the index and it is discarded. For a language a player types, "expected `}` at column
17" is the difference between usable and not.

**Two failure styles.** `ParseResult` is a proper sealed type; `Assembler.assemble` throws.
The same pipeline models failure as data in one layer and as an exception in the next.

**Ordered choice is load-bearing and unremarked.** `primitive = OneOf(alias, number,
boolean, string)`. Since `string` is `OneOrMore(wordChar)` and `wordChar` includes letters,
`true` matches `string` too - only the ordering makes it parse as a boolean. That is PEG
behaviour and it is fine, but it is an invisible constraint on a line that looks
order-independent.

**Duplication.** `language/Expressions.kt` and `prototype4/CommandSyntax.kt` are identical
grammars in two modules, and `AssemblersOld.kt` sits beside `Assemblers.kt`. Prototype
churn; do not carry it across.

**No recursion or precedence story.** The grammar is flat. Fine for a command language. If
the DSL grows arithmetic, conditionals or nesting, ordered choice and the absence of
left-recursion handling will need addressing deliberately.

## The property that makes "describe everything" work

Every game object in the predecessor is a bag of attributes:

```kotlin
Thing("name" to "citizen", "activated" to false)
```

and the language's `Item` is literally that same bag. That is why one literal syntax covers
every entity and why `add {citizen activated=false} 10` needs no per-entity grammar. The
language is universal because **the data model is uniform**.

**This is the tension to resolve before building.** If the Rust rewrite gives each entity a
distinct typed struct - which is Rust's grain, and which
[architecture](../architecture.md)'s ECS rules point toward - the DSL loses its
universality and needs a grammar rule per entity type. Either the game keeps a uniform
attribute-bag representation at its edges, or the scripting language stops being simple.

Nothing about this is decided. It belongs in [the spec backlog](spec-backlog.md).
