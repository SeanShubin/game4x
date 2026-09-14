# Postmortems

**Derived.** Written by Claude. Not binding - see [the specification](../../spec/README.md) for what
was decided. One failure per file, written once the failure is understood rather than while it is
being fixed.

[Documentation map](../README.md) · [Root README](../../README.md)

## What makes one

A postmortem is not a note. A note records analysis, an option, or a measurement; a postmortem
records **a failure that already happened, with the dates and the numbers that show how**.

Four things, and one missing any of them makes it a note instead:

|                                  |                                                                                        |
| -------------------------------- | -------------------------------------------------------------------------------------- |
| **What was supposed to be true** | the rule, the check or the item that was meant to hold                                 |
| **What actually happened**       | dated, with the commits, so it can be re-derived rather than believed                  |
| **Why nobody noticed**           | the mechanism - not *we were careless*, which explains every failure and predicts none |
| **What would have caught it**    | and honestly whether that thing exists, could exist, or cannot                         |

The fourth is the one that earns the file. `CLAUDE.md` says a check earns its place by a failure it
could have produced, and **a postmortem whose last section is *be more careful* has not found the
mechanism yet.**

## A postmortem is not a habit

**`CLAUDE.md` holds the habits and this directory holds the incidents.** A habit written from one
incident explains that incident by construction and could not have come out otherwise - which is why
`CLAUDE.md` says a habit is written as *where to look* until it has caught something it did not come
from. The incident lives here with its numbers; the habit, if one is warranted, is proposed
separately and says which it is.

## The postmortems

| What                                                | When       | The failure in one line                                                                                        |
| --------------------------------------------------- | ---------- | -------------------------------------------------------------------------------------------------------------- |
| [Tracked and still lost](tracked-and-still-lost.md) | 2026-09-13 | An invariant was tracked by three items for ten days and died at the handoffs, each one narrower than the last |
