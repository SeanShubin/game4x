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

## Open questions
