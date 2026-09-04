# Invariants

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../docs/notes/proposals.md).

[Specification](README.md) · [Root README](../README.md)

Statements that are always true of the game. Every other document must not violate one, and
where an invariant and a specific rule appear to conflict, the conflict is a defect in the
specific rule.

## Everything is modelled

- Nothing in the game appears or disappears without a cause inside the model
- Every quantity has an owner, and every change to it has something that did it and a rule
  that says when
- Conservation is not required; for example, fiat money is created and destroyed by a government, and
  that is a model - there is an owner and there are rules
- What is forbidden is a quantity that changes because the game says so, with nothing in the
  world doing it

## Everything is expressible

- Every change to game state is representable and executable as a console command
- Anything the player can do through a surface can be done by typing. A surface may be quicker or
  clearer; it is never more capable.

## Control without tedium

- A player has complete control over every detail, and is never required to exercise it by hand
  more than once. Anything they can do by hand, they can instead specify a rule that does it
- Rules are specified through the interface. Playing the game never requires writing a program
- Every rule a player can use is composed of rules they can also use, down to single actions.
  Nothing is provided only as a whole
- A small change in what a rule does is a small change in how it is written
- Nothing plays itself. Every behaviour that acts on a player's behalf is a rule some person
  wrote, including any the game ships with
- Any rule can be read and changed by the player using it, whatever its origin
- A rule can be written for a kind of thing rather than for one thing, so that what a player
  decides once holds for everything of that kind, including what does not exist yet
- A rule is not part of any one game. It can be named, kept, used in a later game, and given to
  another player
- A rule does the same thing for whoever holds it
- A player's rules always finish. Nothing that can be built in the rule editor runs forever
- A rule carries the number of turns it may run, and stops when they are spent
- Every rule has a text form, and the text is the rule. Anything the rule editor can build can be
  written as text, and anything written as text can be opened in the editor

## No penalty for building infrastructure

- Infrastructure is never a liability; a structure costs nothing to keep, and no structure a
  player builds ever has to be removed to make room for something else
- Every setback comes from outside the player, another player or the world; none comes from
  the player's own infrastructure

## No step that is always taken

- No action has an intermediate step that is always taken. Where one would, the action is
  defined to reach the outcome directly

## The game is one function

- A game state and a transition yield a new game state. There is no other way for state to
  change
- This holds for designing the world as much as for playing it. Which phase a game is in is part
  of its state
- A game state is exactly the result of applying every transition in order to the starting state
- A rule is a source of transitions, not a kind of one. The history records what a rule did,
  exactly as if the player had done it by hand

## The game is data

- Every kind of thing, and every recipe that turns some things into others, is data rather than
  code
- **What the game is made of lives in a data file, not in code and not in markup.** Rust holds
  the engine and the hand-written page holds the shell; neither holds a kind, a recipe, a cost,
  a planet size or a surface. Nothing restates what a data file says; every other form of it is
  derived, and a derived form is generated rather than written. **A scenario is a file too**, so
  what a run exercises can be changed without changing the program.
- The data that runs the game lives in a data file. **Not a presentation file such as markdown or
  HTML, and not a programming language file such as Rust.** The format may be a standard one or one
  we invent. A data file may carry presentation data - commentary for a presentation layer to
  display. **The data may be replicated in the presentation layer, and no replication is
  canonical.**
- A recipe expresses inputs and outputs. Where its inputs are available, it can be used to create
  its outputs. Some inputs are consumed and some are not, and an input says whether its quantity
  is a least or a most
- A recipe belongs to the player or to the world. The player's are offered wherever their inputs
  are present, to take or to leave. The world's are not offered: a population eats, food spoils, an
  unpaid unit is lost
- Every cycle among recipes must spend readiness somewhere along it
- The definitions are part of the game state. Defining one is a transition like any other, so a
  game's history is a complete account of it, including what its rules were
- A definition arrives in one transition. There is no state in which a kind or a recipe is half
  defined
- A game's state is things, in places, and how many of each. A thing is a set of traits, and one of
  them names its kind
- Nothing in the state is special to a kind. Adding a kind adds no field and no case, and whatever
  reads the state reads it the same way whatever kind it holds
- A trait may be derived rather than stored, computed from what is there. Nothing can leave a
  derived trait wrong, because nothing writes one

## Open questions
