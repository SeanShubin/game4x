# The game's data, edited by choosing

2026-09-12. For Sean, who asked for a prototype editor over every piece of the game's data,
where the only thing typed is a thing's name.

**The question it answers is not *can an editor be built*.** It is **what does the data have to
be like for one to be possible**, and the deliverable is the answer rather than the page.
Editability is becoming a constraint on the design, so the useful output is the list of places
where the present data will not submit to a selection.

## Running it

```
python tools/research/editor/extract.py     # regenerate game.json and game.js from the release
```

Then open `tools/research/editor/index.html` in a browser. There is no server, no build and no
dependency; `game.js` exists so that the page works from `file://`, where `fetch` does not.

## What it covers

Every table in `releases/first-release.md`: kinds, families, containment, traits, what bounds a
kind, units and structures, all 31 recipes, biomes, the twelve territories, the world's firing
order and the loop. Not the code.

## The rule, and how it is kept

**Only two field types put a caret in front of a person**, and both are drawn in a warning
colour with a dashed border: `name`, typed once when a thing is created, and `prose`. Every
other type renders a select, a stepper or a toggle.

**So "what still has to be typed" is a query over `editor.js`, not a thing to remember**, and
the count is on the Recipes page. A field that quietly became free text would appear there.

**Vocabularies are functions of the live state, never lists.** Add a kind and it is immediately
choosable in all 77 places a kind can be chosen - checked by doing it. That is what makes the
editor a test of the data rather than a picture of it.

**A phrase the release has never used is available and is marked.** The columns that read as
prose - a trait's subject, its value shape, a bound's reason, what a container holds - are
offered as the set of phrases the release itself uses, plus *something else*. Editing what
exists is therefore all selection, and inventing a subject or a value shape the game has no
word for is where a person meets the wall. **Meeting the wall is the feature**: it is the
moment the data format is telling you it needs a decision.

## What it found

**The recipes are almost entirely selectable. The tables that declare the vocabulary are not,
and that is the wrong way round from what you would guess.**

- **2 of the 29 conditions in the recipes cannot be offered as a choice**, and both turn out to
  be expressible with vocabulary the release already has. See `X-31`.
- **Everything else in every recipe is a selection** - role, quantity, kind, condition, place.
- **Four columns are prose because what they hold is written for a person** rather than run by
  the game: a kind's description, a territory's *what it exercises*, and the steps of the loop.
  Those are documentation and are not a defect.
- **Seven columns that read as prose are not**, once the release's own phrases are gathered:
  a trait's *Of* has 16 distinct values across 23 traits, its *Values* has 12, and both are
  small closed sets wearing sentences.

## What it does not do

**It does not write back.** Turning edited data into the release's markdown is a promotion,
which is Sean's, and a lens may not write `releases/`. Export produces JSON.

**It is not the game's editor.** `releases/first-release.md` says *the rule editor is not in
this release*. This is a research prototype in this lens's own column, built to measure the
data; if an editor ships it is the code lane's, and nothing here is a design for one.

**The numbers on the page are computed at load**, from the data and the schema. The numbers in
this file were read off the running page and will go stale; the page is the one to believe.
