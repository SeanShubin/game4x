# The menu is an information channel, and yesterday's answer did not account for it

2026-09-08. For Sean, who intends **fog of war to feature heavily**, intends to **isolate
non-determinism behind abstractions** - a seeded pseudorandom algorithm rather than true randomness -
and wants **every interaction selectable from finite alternatives without typing**, starting by
re-representing the current specification that way.

**This corrects [`X-8`](outbox.md), filed yesterday**, which said the interface's menu is the set of
actions applicable in the current state. **With fog of war that is not merely imprecise, it is
forbidden.** Citations checked against sources in the session that wrote this.

## 1. The constraint that changes the design

In an imperfect-information game, a player's indistinguishable states form an **information set**, and
there is a hard requirement on it: **all states in one information set must offer the same set of
available actions.** If two states look identical to the player but offer different menus, **the menu
itself reveals which state they are in** - the player infers what they were not meant to know.

This is not a design preference. It is a structural condition, and GDL-II builds it in: a player
cannot distinguish two histories when it has seen the same information along both **and its own
available actions are the same**.

**So the menu cannot be computed from the state.** It has to be computed from what the player knows.
Concretely, in this game: if *deploy ark here* appears only when the territory has no garrison, then
**the presence of the menu entry tells the player there is no garrison** - which is exactly the fact
fog of war exists to hide.

### Which makes `create-if-missing` required rather than merely preferable

Yesterday's report said a precondition makes an action **invisible** and a guard on an effect leaves
it **visible and skipped**, and treated the choice as a design question. **Under fog of war it stops
being a choice.** A precondition that tests hidden state may not gate visibility, because gating is
leaking. The condition has to move onto the effect, or the action has to be offered and fail.

**Three separate lines now converge on the sketch Sean already wrote.** It was the better interface
behaviour (`X-8`); it is the fact-versus-quantity split that keeps zero tests bounded (`X-9`); and it
is what the information-set requirement compels once fog of war is heavy. **This lane did not
engineer that convergence and does not entirely trust it** - three arguments agreeing is weaker
evidence than it feels, because the second and third share a premise about what a garrison is.

### And the failure message becomes part of the design

If actions are offered that may fail, **the failure is an information channel** and has to be
designed as one, deliberately. *Attempt in order to learn* is a legitimate and old mechanic -
scouting by probing - but it must be chosen rather than fallen into, because **what a rejection
discloses is exactly what the player could not otherwise see.** This repository already has a
`Rejection` type; under fog of war it stops being an error report and becomes a game mechanic.

## 2. Fog of war costs one predicate, not a redesign

**GDL-II is base GDL plus two things**: a predicate `sees(role, fact)` saying what each player
perceives, and a distinguished `random` role that moves by chance. With those, it formalises the
rules of arbitrary finite n-player games with randomness and incomplete knowledge, including partial
observability, information asymmetry and communicative actions.

**That is the reassuring half of the answer**: for *representation*, heavy fog costs about one
predicate. What it costs elsewhere is **reasoning** - a player must now reason about what others
know, which is a different and much harder problem. **Sean is building the game and not the opponent
AI**, so the cheap half is the half he needs first, and the expensive half is a decision he can defer
knowingly rather than discover.

## 3. His instinct on randomness is right, and better than GDL-II's, for a specific reason

GDL-II isolates non-determinism by making **nature a player**. Sean proposes making it a **seeded
pseudorandom function of state**. For this project the second is strictly better, and the reason is
already written down here:

`docs/process.md` requires that the transformation read **the state and the commands and nothing
else**, which is what makes the data dump derivable by hand. **Nature-as-a-player breaks that** - it
is an extra input, so the dump is derivable only if you are also told what nature did. **A seed in
the state does not break it**: the draw is a function of state, the transformation stays
`(state, commands) -> state`, and the dump remains derivable from the four artifacts.

So the abstraction he wants is not a workaround for lacking true randomness. **It is the thing that
keeps his verification model intact**, and it would be worth adopting even if true randomness were
free.

**Two things that bite later and are cheap to get right now**, both this lane's inference from the
above rather than citations:

- **The seed is hidden state.** If a player can see it, they can predict every draw, and the
  unpredictability that was the whole point is gone. So the seed lives in the same fog-of-war
  machinery as everything else - **visible to the verifier, invisible to the player**. That is a
  clean split, and it is the same split as the information-set requirement above: one state, two
  audiences.
- **One stream leaks across subsystems.** If every draw comes from a single advancing generator, a
  player can learn that *something* consumed randomness elsewhere - a battle they cannot see - by
  observing that their own next draw moved. The standard remedy is **separate streams**, derived
  from the master seed per subsystem or per entity, so that consumption in one is invisible in
  another.

## 4. On re-representing the current specification, and choosing the metric first

He expects the re-representation to simplify things greatly. **That prediction is measurable, and the
metric has to be chosen before the measurement**, because the obvious metric will report failure
whether or not it succeeds.

**What the precedent suggests will happen.** A description in a language of this kind uses a **tiny
fixed vocabulary** and pays for it in **many explicit instances**. So:

| What to count                                                    | Predicted direction                                                                     |
| ---------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| **Distinct constructs** - the size of the finite language itself | **Down sharply.** This is the simplification he is after, and the only one that matters |
| **Instances** - rows, nested calls, declared facts               | **Up**, possibly a lot                                                                  |
| **Total size** - bytes or lines                                  | **Ambiguous, and quite possibly up**                                                    |

**So counting bytes will make a success look like a failure.** The claim *unlimited complexity
through an insanely simple structure* is precisely a claim that the first row goes down while the
second goes up - **and a metric that adds them together cannot see it.** This repository already has
the general form of that warning: an instrument that answers a narrower question than the one asked
returns a plausible number and invites none.

**Suggested, and not more than that**: fix the three counts against the present specification before
re-representing anything, so the comparison exists. This lane can produce that baseline if asked; it
has not, because nobody has asked and it would be work addressed to nobody.

## What this lens is not saying

Nothing here says what the language should be. The information-set requirement is a constraint the
design must satisfy rather than a design; how to satisfy it - hidden preconditions becoming guards,
or actions offered and failing informatively - is Sean's, and the two are genuinely different games.

## Sources

- Information sets, and that all states in one must offer the same actions or the action set leaks -
  [Endriss, *Imperfect-Information Games*](https://staff.science.uva.nl/u.endriss/teaching/game-theory/slides/gt8.pdf),
  [Maryland CMSC 474, Imperfect-Information Games](https://www.cs.umd.edu/~hajiagha/474GT13/Lecture10222013.pdf),
  [Zielonka, MPRI notes](https://www.irif.fr/~zielonka/Enseignement/MPRI/2015/Extensive/part2.pdf)
- GDL-II: `sees`, the `random` role, and indistinguishability requiring the same local actions -
  [Schiffel and Thielscher, *Representing and Reasoning About the Rules of General Games With
  Imperfect Information*, JAIR](https://www.jair.org/index.php/jair/article/view/10862),
  [A General Game Description Language for Incomplete Information Games](https://cdn.aaai.org/ojs/7647/7647-13-11177-1-2-20201228.pdf)
