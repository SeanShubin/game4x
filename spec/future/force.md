# Force

**Authored.** Sean owns every idea here. Claude may rephrase and reorganize what is already
present, reporting every change; a new idea is entered by Sean himself, whether he types it
or pastes it from a [proposal](../../docs/notes/proposals.md).

[Specification](../README.md) · [Root README](../../README.md)

**These rules are not built and are not abandoned.** They are here because the game wants them
once it can be played, and the first release cannot be played while it is building them.

## Force

- Force is the capacity for violence.
- Every territory has a force of nature, inherent to it. Nature has no quantity and no
  population that grows.
- Nature consumes nothing. An extractor is intentional exploitation, and nature does not
  exploit.

## Producing force

- **Force is mustered each turn and does not outlast it.** What a territory presents is what it
  mustered this turn, and nothing accumulates
- A citizen can fight but cannot organise. **It musters no force unless something coordinates it**
- A garrison coordinates the citizens of its territory, so that **each of them musters its
  strength each turn**. It has no strength of its own, and **it does this by existing** - nothing
  has to work it
- **What a citizen spends to muster is its own**, and is not what it spends to labor or to bear
- **A unit is organised force in itself**, and musters its own force needing nothing to coordinate
  it
- A territory has at most one garrison, because it represents the organisation of the whole
  territory rather than a presence in one part of it

## Gaining and holding ground

- Taking a territory takes force greater than the existing force, be that nature, a player, or
  anything else not already controlled by you
- Holding a territory takes force equal to its force of nature
- Should the force in a territory fall below its force of nature, nature takes it back. Its entire
  population perishes, and every unit on it is destroyed
- Several units brought to one place sum their force. Taking a territory uses the organised force
  brought to it, and several units may take together

## Garrison

- The structure through which the citizens of a territory apply force. A garrison has no strength
  of its own and is what allows the citizens of its territory to muster theirs, and it is what
  allows units that travel by land to be produced.

## From the console

- `set force <territory> <force>` - set a territory's force of nature

## From the turn

- **nature takes back what is no longer held**

## From the data

```
{limit container:territory contained:garrison n:1}
{value name:ice of:biome nature:1}
{value name:desert of:biome nature:1}
{value name:grassland of:biome nature:1}
{value name:jungle of:biome nature:2}
{value name:mountain of:biome nature:1}
```
