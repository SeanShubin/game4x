# What `../vote`'s scenario actually does

**Derived**, 2026-09-03, from reading `documentation/src/main/kotlin/.../Scenario.kt`, `Main.kt` and
`DocumentationRecorder.kt` in `../vote`. Not binding.

[Notes index](README.md) · [Documentation map](../README.md) · [Root README](../../README.md)

Sean: *this is precisely what the scenario test does in `../vote`*. It is close, and **the one
difference is the thing he has just invented.**

## The shape

`Scenario.comprehensive(ctx)` is thirteen lines of Kotlin: register four users, create an election,
add four candidates, cast three ballots. `Main` then runs **that same scenario four times**, against
four different backends, and dumps each:

| Run | Backend        | Produces        |
| --- | -------------- | --------------- |
| 1   | in-memory      | `scenario.html` |
| 2   | **MySQL**      | `sql.html`      |
| 3   | **DynamoDB**   | `dynamodb.html` |
| 4   | HTTP-recording | `events.html`   |

**The relational and physical dumps are not two renderings of one thing.** They come from two
running stores, and if they disagreed about what happened, an implementation would be wrong.
**Vote's pair is evidence; ours is presentation** - `state.md` and `entities.md` are both rendered
from one `Game`, so they cannot disagree.

## What transfers

**The scenario's stated purpose is the reports, not the logic.** Its own comment: *a small
comprehensive scenario that exercises the remaining domain surface... **so the generated HTML has
meaningful rows in each projection***. That is Sean's *touch every thing and recipe* exactly, and it
confirms the requirement is about the reports being informative rather than about test coverage.

**The scenario narrates itself.** `DocumentationRecorder` records **section markers, calls and events
in one chronological list**, and `markSection(title, description)` is what makes `scenario.html`
readable English instead of a dump. **`commands/play.4x` already carries its narration in comments** -
so the fourth artifact is the flattened history *with those comments kept*, not a bare list of lines.

**The index is generated first**, *so we know what files we're creating*. Our analogue is the *Start
here* table in `README.md`, which now lists all four generated files and asserts it.

## What does not transfer, and one place we are ahead

**Vote's scenario is code**; ours is a data file, which `P-199` requires - *a scenario is a file too,
so what a run exercises can be changed without changing the program*. **On that one point this
repository is ahead of the thing it is copying.**

**And there is no closure test in vote.** Nothing there checks that a person could derive the dumps
from the definitions. **Four backends agreeing is the check**, which is machine against machine.

## The insight worth keeping

**Sean's proposal is not what vote does; it is stronger, and in a different direction.**

- **Vote checks consistency**: four implementations of the same rules, and disagreement means a bug
- **Sean's check is intent**: *take the things, the recipes, the commands, and manually derive the
  data dump* - a person against a machine

**Consistency cannot catch a rule that is wrong in the same way everywhere.** Four backends would
have agreed perfectly that a territory holds eight citizens while the model let it hold twelve, if
all four had read the same constant. **A pencil would not have.**

**So the closure is worth having and is not borrowed.** What is borrowed is everything around it: one
scenario, several projections, narration recorded with the events, an index, and a stated purpose of
*meaningful rows in each projection*.
